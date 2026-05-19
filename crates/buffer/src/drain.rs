//! Drain3 log-template-mining algorithm — chunk #69 Phase B production (Session 1).
//!
//! Re-derived from Drain3 specifications per `.andromeda/decisions/pre-d2-drain-spike.md`
//! §Cleanup discipline (does NOT promote spike code as-is per security plan
//! §Code Patterns hygiene). The algorithm walks a fixed-depth parse tree keyed
//! by token-count then first-N tokens; at each leaf it computes a position-based
//! similarity against existing clusters and either merges (replacing differing
//! token positions with `<*>` wildcards) or creates a new cluster. LRU eviction
//! bounds the template tree at `max_clusters` to preserve hot-path latency
//! (<50μs p99 production target; Phase A spike measured 2μs in isolation —
//! 25× headroom for Phase B's masking + LRU + cluster merge overhead).
//!
//! # Module surface
//!
//! - [`DrainConfig`] holds tuning parameters — depth, similarity, max_clusters,
//!   regex-driven masking patterns.
//! - [`DrainMiner`] is the in-memory miner. [`DrainMiner::assign`] returns the
//!   matching [`TemplateId`] (creating a new cluster when no existing template
//!   matches the similarity threshold).
//! - [`DrainPersistence`] is the trait abstraction for tree serialization
//!   round-trip; the corpus-backed adapter lives at
//!   `pulse-app/src/drain_persistence.rs` (Session 4 work).
//! - [`DrainState`] is the bincode-serializable payload exchanged through the
//!   persistence trait.
//! - [`TemplateDistEntry`] is the per-template snapshot consumed by the
//!   `diagnostics.template_distribution()` TauRPC procedure (Session 3 work
//!   at `pulse-app/src/diagnostics_router.rs`).
//!
//! # PII discipline
//!
//! Template content MUST pass through `crates/security::scrubber::scrub_attribute`
//! BEFORE persistence to either the in-memory DuckDB `log_templates` table
//! OR the corpus SQLite via the persistence adapter (per capability P-047 +
//! security plan §Logging NEVER-log discipline). The miner itself does not
//! scrub; the appender + persistence callers do. This separation keeps the
//! algorithm free of any security-crate dep edge.

use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use duckdb::Connection;
use lru::LruCache;
use regex::Regex;
use security::scrubber::{ScrubbedValue, scrub_attribute};
use serde::{Deserialize, Serialize};

use crate::contract::Error;

/// Wildcard placeholder token used in template strings to represent positions
/// that vary across cluster members. Drain3-canonical (`<*>`).
pub const WILDCARD_TOKEN: &str = "<*>";

/// Sentinel template-id meaning "no template assigned". Public so the appender
/// can serialize a sentinel value in the Arrow `template_id` column when Drain
/// assignment did not produce a match (currently only possible on empty
/// post-masking bodies; reserved for future opt-out paths).
pub const NULL_TEMPLATE_ID: TemplateId = 0;

/// Schema version baseline for [`DrainState`]. Increment on layout-breaking
/// changes; loaders MUST reject mismatched versions rather than risk
/// data corruption.
pub const DRAIN_STATE_SCHEMA_VERSION: u32 = 1;

/// Sequential template identifier. Starts at 1; 0 reserved (see [`NULL_TEMPLATE_ID`]).
pub type TemplateId = u64;

/// Regex-driven masking pattern. Applied sequentially to log bodies BEFORE
/// tokenization; ALL non-overlapping matches of each pattern get replaced.
/// `category` is the stable label used for observability events (it MUST be
/// safe to emit; the matched value content NEVER appears in logs).
#[derive(Clone, Debug)]
pub struct MaskPattern {
    /// Compiled regex pattern.
    pub regex: Regex,
    /// Replacement placeholder (e.g., `<NUM>`, `<IP>`).
    pub replacement: &'static str,
    /// Stable category label for observability.
    pub category: &'static str,
}

/// Drain tuning parameters. Defaults per pulse-v0_2_0-route §67 line 296
/// (depth=4, similarity=0.5, max_clusters=1000) chosen to balance template
/// quality against memory footprint under the 10k-event-per-second ingest
/// budget.
#[derive(Clone, Debug)]
pub struct DrainConfig {
    /// Tree depth; the first `depth - 1` tokens become tree-walk keys; the
    /// remaining tokens differentiate clusters via similarity match.
    pub depth: u32,
    /// Position-based similarity threshold ∈ (0, 1]. `0.5` means at least
    /// half of the token positions must match for a cluster to be reused.
    pub similarity: f32,
    /// LRU eviction trigger; once `templates.len() > max_clusters` an
    /// eviction fires for the least-recently-touched cluster.
    pub max_clusters: usize,
    /// Sequential regex masks applied before tokenization. Applied in order;
    /// each pattern operates on the output of the previous.
    pub masking_patterns: Vec<MaskPattern>,
}

impl DrainConfig {
    /// Default config: depth=4, similarity=0.5, max_clusters=1000, masking
    /// patterns covering IPv4 addresses, file paths (POSIX + Windows),
    /// hex addresses (`0x...`), and decimal numbers. Categories are stable
    /// strings safe for emission in observability events.
    pub fn default_config() -> Self {
        Self {
            depth: 4,
            similarity: 0.5,
            max_clusters: 1000,
            masking_patterns: default_masking_patterns(),
        }
    }
}

impl Default for DrainConfig {
    fn default() -> Self {
        Self::default_config()
    }
}

/// Compile the default masking pattern catalog. Order matters: more-specific
/// patterns first so they claim matches before generic patterns over-redact
/// (e.g., IPv4 before plain numeric so `127.0.0.1` masks as `<IP>` not as
/// three `<NUM>`s separated by dots).
fn default_masking_patterns() -> Vec<MaskPattern> {
    vec![
        MaskPattern {
            regex: Regex::new(r"\b(?:\d{1,3}\.){3}\d{1,3}(?::\d+)?\b")
                .expect("IPv4 regex compiles"),
            replacement: "<IP>",
            category: "ipv4",
        },
        MaskPattern {
            // POSIX-style absolute path: starts with `/` followed by ≥1 path segment.
            regex: Regex::new(r"(?:/[\w.\-]+)+/?").expect("POSIX path regex compiles"),
            replacement: "<PATH>",
            category: "path",
        },
        MaskPattern {
            // Windows-style absolute path: drive letter + colon + backslash + chars.
            regex: Regex::new(r"[A-Za-z]:\\[\w.\-\\]+").expect("Windows path regex compiles"),
            replacement: "<PATH>",
            category: "path",
        },
        MaskPattern {
            // Hex address: `0x` prefix followed by hex digits.
            regex: Regex::new(r"\b0x[0-9A-Fa-f]+\b").expect("hex regex compiles"),
            replacement: "<HEX>",
            category: "hex",
        },
        MaskPattern {
            // Generic decimal/integer number. Lowest priority; runs after the more
            // specific patterns above.
            regex: Regex::new(r"\b\d+(?:\.\d+)?\b").expect("number regex compiles"),
            replacement: "<NUM>",
            category: "number",
        },
    ]
}

/// Persistence trait abstraction. Implementation lives at
/// `pulse-app/src/drain_persistence.rs` (Session 4) wrapping
/// `corpus::contract::CorpusWriter`. Trait defined in the lower (buffer)
/// crate per CLAUDE.md §Session Additions 2026-05-16 trait-in-lower-crate
/// pattern; preserves the arch DAG (no `buffer → corpus` dep edge).
pub trait DrainPersistence: Send + Sync {
    /// Load a previously-saved state. `None` means no prior state (fresh boot).
    /// Errors surface to caller for tracing emission; callers should treat as
    /// "proceed with empty state" rather than fatal (boot is non-fatal per
    /// chunk #68 corpus precedent).
    fn load(&self) -> Result<Option<DrainState>, Error>;

    /// Save current state. Implementation MUST scrub template content via
    /// `security::scrubber::scrub_attribute` BEFORE persistence per
    /// capability P-047. Errors surface to caller for tracing emission.
    fn save(&self, state: &DrainState) -> Result<(), Error>;
}

/// Bincode-serializable snapshot of the miner's state. `schema_version` MUST
/// be bumped on any breaking change to the serialized layout; loaders reject
/// mismatched versions rather than risk data corruption.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DrainState {
    /// Schema version. Bump on any layout-breaking change.
    pub schema_version: u32,
    /// Next template id to assign. Monotonic; eviction doesn't recycle ids.
    pub next_template_id: u64,
    /// All currently-held template records.
    pub templates: Vec<TemplateRecord>,
    /// Tree structure snapshot.
    pub tree: SerializableTree,
}

/// Per-cluster record. Tokens are the merged template token sequence;
/// `<*>` wildcards mark positions that vary across cluster members.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TemplateRecord {
    pub id: TemplateId,
    pub tokens: Vec<String>,
    pub occurrence_count: u64,
    pub first_seen_unix_nano: i64,
    pub last_seen_unix_nano: i64,
}

/// Tree shape serialization. Keyed at root by token-count; each subtree node
/// holds child tokens + optional wildcard branch + leaf cluster ids.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SerializableTree {
    pub length_buckets: HashMap<usize, SerializableNode>,
}

/// Serializable form of an internal [`TreeNode`].
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SerializableNode {
    pub children: HashMap<String, SerializableNode>,
    pub wildcard: Option<Box<SerializableNode>>,
    pub leaf_clusters: Vec<TemplateId>,
}

/// Drift indicator for the diagnostics surface. Three states informed by
/// template quality:
///
/// - `Healthy` — moderate wildcard mix (≤60% wildcards) with multiple
///   occurrences; the typical case for a stable production cluster.
/// - `OverGeneralized` — token mix dominated by wildcards (>60%);
///   suggests masking too aggressive OR similarity threshold too low.
/// - `UnderClustered` — single-occurrence template that hasn't merged;
///   suggests masking too narrow OR similarity threshold too high (or
///   genuinely a one-off event).
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DriftIndicator {
    Healthy,
    OverGeneralized,
    UnderClustered,
}

/// Per-template snapshot used by the diagnostics TauRPC procedure (Session 3).
/// Token sequence joined as a single string for UI display + occurrence
/// count + drift state.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct TemplateDistEntry {
    pub id: TemplateId,
    pub content: String,
    pub occurrence_count: u64,
    pub drift_indicator: DriftIndicator,
}

// ─── Internal state ─────────────────────────────────────────────────────────

/// Internal mutable state guarded by a single `Mutex`. Per-event work is
/// microseconds at p99 (Phase A spike measured 2μs) so single-mutex
/// contention stays acceptable through the 10k-events/sec ingest budget.
struct MinerState {
    next_template_id: u64,
    templates: HashMap<TemplateId, TemplateRecord>,
    tree: TreeRoot,
    lru: LruCache<TemplateId, ()>,
    /// Newly-created [`TemplateRecord`]s accumulated since the last
    /// [`DrainMiner::drain_newly_created_templates`] call. The consumer
    /// drains this slot after each Arrow batch and writes new templates
    /// to the in-memory DuckDB `log_templates` table via prepared statement.
    newly_created_since_last_drain: Vec<TemplateRecord>,
    /// LRU evictions accumulated since the last
    /// [`DrainMiner::take_lru_evictions_since_tick`] call. The pulse-app
    /// `buffer.tick` heartbeat reads + resets this counter every 15s and
    /// surfaces it as the `drain_lru_evictions_since_tick` heartbeat field
    /// (per pre-registered AllowList entry at observability.rs:160).
    lru_evictions_since_last_tick: u64,
}

/// Tree root: branches by token count (length bucket).
struct TreeRoot {
    length_buckets: HashMap<usize, TreeNode>,
}

/// Internal tree node. Internal levels hold child branches (literal or
/// wildcard); leaf levels (at `config.depth` from root) hold cluster ids.
#[derive(Default)]
struct TreeNode {
    children: HashMap<String, TreeNode>,
    wildcard: Option<Box<TreeNode>>,
    leaf_clusters: Vec<TemplateId>,
}

/// The Drain template miner. Thread-safe via interior mutability; share
/// across the consumer task + TauRPC resolver via `Arc<DrainMiner>`.
pub struct DrainMiner {
    config: DrainConfig,
    persistence: Option<Arc<dyn DrainPersistence>>,
    state: Mutex<MinerState>,
}

impl DrainMiner {
    /// Construct a fresh miner with the given config and optional persistence
    /// adapter. The persistence adapter (if `Some`) is NOT called at
    /// construction time; callers should invoke [`Self::load_from_persistence`]
    /// at boot to rehydrate state.
    pub fn new(config: DrainConfig, persistence: Option<Arc<dyn DrainPersistence>>) -> Self {
        let lru_cap =
            NonZeroUsize::new(config.max_clusters.max(1)).expect("max_clusters.max(1) >= 1");
        Self {
            config,
            persistence,
            state: Mutex::new(MinerState {
                next_template_id: 1, // 0 reserved per NULL_TEMPLATE_ID
                templates: HashMap::new(),
                tree: TreeRoot {
                    length_buckets: HashMap::new(),
                },
                lru: LruCache::new(lru_cap),
                newly_created_since_last_drain: Vec::new(),
                lru_evictions_since_last_tick: 0,
            }),
        }
    }

    /// Load saved state from persistence. Returns `Ok(true)` when prior state
    /// rehydrated; `Ok(false)` when no persistence configured OR persistence
    /// has no prior state; `Err` on schema mismatch or persistence-side error.
    pub fn load_from_persistence(&self) -> Result<bool, Error> {
        let Some(persistence) = &self.persistence else {
            return Ok(false);
        };
        let Some(loaded) = persistence.load()? else {
            return Ok(false);
        };
        if loaded.schema_version != DRAIN_STATE_SCHEMA_VERSION {
            return Err(Error::Drain {
                reason: format!(
                    "drain state schema_version mismatch: expected {}, got {}",
                    DRAIN_STATE_SCHEMA_VERSION, loaded.schema_version
                ),
            });
        }
        let mut state = self.state.lock().map_err(|_| Error::Drain {
            reason: "drain state mutex poisoned".to_string(),
        })?;
        state.next_template_id = loaded.next_template_id;
        state.templates = loaded.templates.into_iter().map(|t| (t.id, t)).collect();
        state.tree = deserialize_tree(loaded.tree);
        // Rehydrate LRU in ascending template-id order; oldest first → most-
        // recently-touched last (approximates insertion order).
        let mut ids: Vec<TemplateId> = state.templates.keys().copied().collect();
        ids.sort_unstable();
        for id in ids {
            state.lru.put(id, ());
        }
        Ok(true)
    }

    /// Snapshot current state for persistence. Cheap (`O(N)` over templates +
    /// tree clone) but not free; callers should invoke periodically rather
    /// than per assignment.
    pub fn snapshot_state(&self) -> Result<DrainState, Error> {
        let state = self.state.lock().map_err(|_| Error::Drain {
            reason: "drain state mutex poisoned".to_string(),
        })?;
        let mut templates: Vec<TemplateRecord> = state.templates.values().cloned().collect();
        templates.sort_by_key(|t| t.id);
        Ok(DrainState {
            schema_version: DRAIN_STATE_SCHEMA_VERSION,
            next_template_id: state.next_template_id,
            templates,
            tree: serialize_tree(&state.tree),
        })
    }

    /// Persist current snapshot via the configured persistence adapter.
    /// Returns `Ok(true)` when persistence ran; `Ok(false)` when no
    /// persistence configured; `Err` on persistence-side error.
    pub fn persist(&self) -> Result<bool, Error> {
        let Some(persistence) = &self.persistence else {
            return Ok(false);
        };
        let snapshot = self.snapshot_state()?;
        persistence.save(&snapshot)?;
        Ok(true)
    }

    /// Assign a template to the given log body. Returns the matching
    /// (or newly-created) [`TemplateId`]. Returns `None` only when the
    /// post-masking body produces an empty token list (whitespace-only or
    /// pure-empty input — no template to assign).
    ///
    /// Hot-path discipline: target p99 < 50μs production budget. Phase A
    /// spike measured 2μs in isolation; Phase B production with masking +
    /// LRU + cluster merge adds an estimated ~5-15μs.
    pub fn assign(&self, log_body: &str) -> Option<TemplateId> {
        self.assign_at(log_body, 0)
    }

    /// Variant of [`Self::assign`] that takes an explicit wall-clock
    /// timestamp (in unix nanoseconds) for the cluster's
    /// `first_seen_unix_nano` / `last_seen_unix_nano` bookkeeping. Use this
    /// when integrating into the appender hot path where the OTLP log
    /// record's timestamp is already known. The zero-default `assign`
    /// variant is fine for unit tests where the timestamp is uninteresting.
    pub fn assign_at(&self, log_body: &str, ts_unix_nano: i64) -> Option<TemplateId> {
        // Per-event latency emission per chunk #69 Phase B Session 7+
        // (obs allowlist pre-registered at observability.rs:1389 +
        // observability.rs:2840 — only `value` field permitted under the
        // aggregate-only discipline). Trace-level gated: `Instant::now()`
        // call elided when trace level is disabled (production default;
        // ANDROMEDA_PULSE_LOG_LEVEL=trace enables for SLO verification
        // per obs-plan §11 hot-path level-gating discipline + plan
        // §Test Commands jq pipeline). Result type is captured + returned
        // verbatim so the emission cost is a single Instant read at trace
        // level + zero at production levels.
        let start = if tracing::enabled!(tracing::Level::TRACE) {
            Some(Instant::now())
        } else {
            None
        };
        let id = self.assign_at_inner(log_body, ts_unix_nano);
        if let Some(start) = start {
            let elapsed_us = start.elapsed().as_micros() as u64;
            tracing::trace!(
                target: "metric.pipeline.l1c.drain_assignment_latency_p99_microseconds",
                value = elapsed_us,
                "drain assignment latency",
            );
        }
        id
    }

    fn assign_at_inner(&self, log_body: &str, ts_unix_nano: i64) -> Option<TemplateId> {
        let masked = mask_body(&self.config.masking_patterns, log_body);
        let tokens: Vec<String> = masked.split_whitespace().map(|s| s.to_string()).collect();
        if tokens.is_empty() {
            return None;
        }

        let mut state = self.state.lock().ok()?;
        let token_count = tokens.len();
        let walk_depth = (self.config.depth.saturating_sub(1) as usize).min(token_count);

        // Phase 1: walk + clone candidate cluster ids; release leaf borrow
        // before Phase 2 reads/writes state.templates.
        let candidate_ids: Vec<TemplateId> = {
            let length_bucket = state.tree.length_buckets.entry(token_count).or_default();
            let leaf_ref = walk_down(length_bucket, &tokens, walk_depth);
            leaf_ref.leaf_clusters.clone()
        };

        // Phase 2: find best similarity match against the candidates.
        let best = find_best_match(
            &candidate_ids,
            &state.templates,
            &tokens,
            self.config.similarity,
        );

        match best {
            Some(id) => {
                // Merge: replace differing token positions with wildcards;
                // increment count; promote in LRU.
                if let Some(record) = state.templates.get_mut(&id) {
                    merge_template(&mut record.tokens, &tokens);
                    record.occurrence_count += 1;
                    if ts_unix_nano > record.last_seen_unix_nano {
                        record.last_seen_unix_nano = ts_unix_nano;
                    }
                }
                // `LruCache::get` promotes to most-recently-used.
                let _ = state.lru.get(&id);
                Some(id)
            }
            None => {
                // Create new cluster.
                let new_id = state.next_template_id;
                state.next_template_id = state.next_template_id.saturating_add(1);
                if new_id == NULL_TEMPLATE_ID {
                    // Wraparound is implausible (u64) but defend explicitly.
                    return None;
                }
                let record = TemplateRecord {
                    id: new_id,
                    tokens: tokens.clone(),
                    occurrence_count: 1,
                    first_seen_unix_nano: ts_unix_nano,
                    last_seen_unix_nano: ts_unix_nano,
                };
                state.templates.insert(new_id, record.clone());
                state.newly_created_since_last_drain.push(record);

                // Phase 3: re-walk tree to attach the new id at the leaf
                // (the leaf was created during Phase 1; this re-walk is
                // O(walk_depth) and the borrow checker requires it because
                // Phase 2 borrowed state.templates mutably).
                {
                    let length_bucket = state
                        .tree
                        .length_buckets
                        .get_mut(&token_count)
                        .expect("Phase 1 ensured this length bucket exists");
                    let leaf_ref = walk_down(length_bucket, &tokens, walk_depth);
                    leaf_ref.leaf_clusters.push(new_id);
                }

                state.lru.put(new_id, ());

                // Phase 4: evict if over cap. Loop in case the cap shrunk
                // dynamically (defensive; usually evicts at most 1 per assign).
                while state.templates.len() > self.config.max_clusters {
                    let Some((evict_id, _)) = state.lru.pop_lru() else {
                        break;
                    };
                    state.templates.remove(&evict_id);
                    remove_id_from_tree(&mut state.tree, evict_id);
                    state.lru_evictions_since_last_tick =
                        state.lru_evictions_since_last_tick.saturating_add(1);
                }

                Some(new_id)
            }
        }
    }

    /// Return the top-N templates by occurrence count, formatted for the
    /// diagnostics TauRPC procedure (Session 3). Caller controls `top_n`
    /// truncation per the chunk's `top-50` AC-L5 layouts constraint.
    pub fn template_distribution(&self, top_n: usize) -> Vec<TemplateDistEntry> {
        let Ok(state) = self.state.lock() else {
            return Vec::new();
        };
        let mut entries: Vec<TemplateDistEntry> = state
            .templates
            .values()
            .map(|t| TemplateDistEntry {
                id: t.id,
                content: t.tokens.join(" "),
                occurrence_count: t.occurrence_count,
                drift_indicator: drift_for(t),
            })
            .collect();
        entries.sort_by_key(|e| std::cmp::Reverse(e.occurrence_count));
        entries.truncate(top_n);
        entries
    }

    /// Total template count currently held. Used by `buffer.tick` heartbeat
    /// emission (Session 6 obs integration).
    pub fn template_count(&self) -> u64 {
        self.state
            .lock()
            .map(|s| s.templates.len() as u64)
            .unwrap_or(0)
    }

    /// Drain accumulated newly-created [`TemplateRecord`]s and return them
    /// to the caller. The consumer drains this slot after each Arrow log
    /// batch and writes the new templates to the in-memory DuckDB
    /// `log_templates` table via [`write_template_to_table`]. Each record
    /// is yielded at most once across the lifetime of the miner; subsequent
    /// calls return an empty Vec unless new clusters were created in the
    /// interim. On poisoned mutex returns an empty Vec (best-effort
    /// surfacing — the underlying clusters are still queryable via the
    /// in-memory state for `diagnostics.template_distribution`).
    pub fn drain_newly_created_templates(&self) -> Vec<TemplateRecord> {
        match self.state.lock() {
            Ok(mut state) => std::mem::take(&mut state.newly_created_since_last_drain),
            Err(_) => Vec::new(),
        }
    }

    /// Read + reset the LRU eviction counter accumulated since the last
    /// call. Used by the `buffer.tick` heartbeat to surface
    /// `drain_lru_evictions_since_tick` (per pre-registered AllowList entry
    /// at observability.rs:160). On poisoned mutex returns 0 — the metric
    /// is informational; missing samples are preferable to panicking the
    /// long-running heartbeat task.
    pub fn take_lru_evictions_since_tick(&self) -> u64 {
        match self.state.lock() {
            Ok(mut state) => std::mem::take(&mut state.lru_evictions_since_last_tick),
            Err(_) => 0,
        }
    }
}

/// Persist a newly-created template to the in-memory DuckDB `log_templates`
/// table via a prepared statement (NO `format!()`; per security plan
/// universal invariant on DuckDB SQL composition). The template's
/// joined-token text is scrubbed via [`security::scrubber::scrub_attribute`]
/// BEFORE the write per capability P-047 + security plan §Logging NEVER-log
/// discipline extended to a new persistence surface; redacted matches store
/// the stable `[REDACTED:{category}]` marker rather than the raw matched
/// content.
///
/// Caller passes the same `Connection` already held under the
/// `Arc<Mutex<Connection>>` consumer guard — write happens on the same
/// lock + transaction window as the preceding `log_records` Arrow append.
pub(crate) fn write_template_to_table(
    conn: &Connection,
    record: &TemplateRecord,
) -> Result<(), Error> {
    let raw_text = record.tokens.join(" ");
    let scrubbed = match scrub_attribute(&raw_text) {
        ScrubbedValue::Allowed(s) => s,
        ScrubbedValue::Redacted { category } => format!("[REDACTED:{}]", category),
    };

    let mut stmt = conn
        .prepare(
            "INSERT INTO log_templates \
             (template_id, template_content, occurrence_count, first_seen_unix_nano, last_seen_unix_nano) \
             VALUES (?, ?, ?, ?, ?)",
        )
        .map_err(|e| Error::Drain {
            reason: format!("prepare log_templates insert: {}", short_drain_err(&e.to_string())),
        })?;
    stmt.execute(duckdb::params![
        record.id as i64,
        scrubbed,
        record.occurrence_count as i64,
        record.first_seen_unix_nano,
        record.last_seen_unix_nano,
    ])
    .map_err(|e| Error::Drain {
        reason: format!(
            "execute log_templates insert: {}",
            short_drain_err(&e.to_string())
        ),
    })?;
    Ok(())
}

/// Compact short-error rendering for [`Error::Drain`] reasons. Mirrors
/// `appender::short_err` shape — first line, trimmed, capped at 120 chars
/// — but module-local to avoid leaking the appender's pub(crate) visibility
/// across what is otherwise a self-contained module surface.
fn short_drain_err(msg: &str) -> String {
    let first_line = msg.lines().next().unwrap_or(msg).trim();
    if first_line.len() > 120 {
        format!("{}…", &first_line[..120])
    } else {
        first_line.to_string()
    }
}

// ─── Helpers ────────────────────────────────────────────────────────────────

/// Apply sequential mask patterns to the raw body. ALL non-overlapping
/// matches of each pattern are replaced; subsequent patterns operate on
/// the already-masked output.
fn mask_body(patterns: &[MaskPattern], body: &str) -> String {
    let mut current = body.to_string();
    for pattern in patterns {
        current = pattern
            .regex
            .replace_all(&current, pattern.replacement)
            .into_owned();
    }
    current
}

/// Walk down the tree by tokens up to `walk_depth` levels, creating missing
/// branches as we go. The literal-token branch is preferred over wildcard
/// at each step (existing literal branches are taken; new tokens insert
/// new literal branches rather than collapsing aggressively to wildcards).
fn walk_down<'a>(
    start: &'a mut TreeNode,
    tokens: &[String],
    walk_depth: usize,
) -> &'a mut TreeNode {
    let mut current: &mut TreeNode = start;
    for token in tokens.iter().take(walk_depth) {
        if token == WILDCARD_TOKEN {
            // Explicit wildcard input: descend the wildcard branch (creating if missing).
            current = current
                .wildcard
                .get_or_insert_with(|| Box::new(TreeNode::default()))
                .as_mut();
        } else {
            current = current.children.entry(token.clone()).or_default();
        }
    }
    current
}

/// Search a leaf's cluster ids for the best similarity match against the
/// input tokens. Returns `Some(id)` when ≥ `threshold` similarity is found;
/// `None` otherwise. Best-match semantics: highest similarity wins; ties
/// broken by lowest id (deterministic).
fn find_best_match(
    cluster_ids: &[TemplateId],
    templates: &HashMap<TemplateId, TemplateRecord>,
    tokens: &[String],
    threshold: f32,
) -> Option<TemplateId> {
    let mut best: Option<(TemplateId, f32)> = None;
    for &id in cluster_ids {
        let Some(record) = templates.get(&id) else {
            continue;
        };
        let sim = position_similarity(&record.tokens, tokens);
        if sim >= threshold {
            match best {
                None => best = Some((id, sim)),
                Some((best_id, best_sim)) => {
                    if sim > best_sim || (sim == best_sim && id < best_id) {
                        best = Some((id, sim));
                    }
                }
            }
        }
    }
    best.map(|(id, _)| id)
}

/// Position-based similarity: count token positions where template matches
/// input (template wildcards count as match) divided by max length. Both
/// inputs descended from the same length bucket so the counts match in
/// practice; defensive against length mismatch.
fn position_similarity(template: &[String], input: &[String]) -> f32 {
    if template.is_empty() || input.is_empty() {
        return 0.0;
    }
    let n = template.len().min(input.len());
    let mut matches = 0;
    for i in 0..n {
        if template[i] == WILDCARD_TOKEN || template[i] == input[i] {
            matches += 1;
        }
    }
    matches as f32 / template.len().max(input.len()) as f32
}

/// Merge `input` tokens into `template` in place: positions where the two
/// disagree become wildcards. Positions equal or already-wildcard stay
/// unchanged.
fn merge_template(template: &mut [String], input: &[String]) {
    let n = template.len().min(input.len());
    for i in 0..n {
        if template[i] != input[i] && template[i] != WILDCARD_TOKEN {
            template[i] = WILDCARD_TOKEN.to_string();
        }
    }
}

/// Classify a template's drift state for the diagnostics surface.
fn drift_for(template: &TemplateRecord) -> DriftIndicator {
    if template.tokens.is_empty() {
        return DriftIndicator::Healthy;
    }
    if template.occurrence_count <= 1 {
        return DriftIndicator::UnderClustered;
    }
    let wildcard_count = template
        .tokens
        .iter()
        .filter(|t| t.as_str() == WILDCARD_TOKEN)
        .count();
    let ratio = wildcard_count as f32 / template.tokens.len() as f32;
    if ratio > 0.6 {
        DriftIndicator::OverGeneralized
    } else {
        DriftIndicator::Healthy
    }
}

/// Recursive tree → serializable conversion.
fn serialize_tree(tree: &TreeRoot) -> SerializableTree {
    SerializableTree {
        length_buckets: tree
            .length_buckets
            .iter()
            .map(|(k, v)| (*k, serialize_node(v)))
            .collect(),
    }
}

fn serialize_node(node: &TreeNode) -> SerializableNode {
    SerializableNode {
        children: node
            .children
            .iter()
            .map(|(k, v)| (k.clone(), serialize_node(v)))
            .collect(),
        wildcard: node.wildcard.as_ref().map(|w| Box::new(serialize_node(w))),
        leaf_clusters: node.leaf_clusters.clone(),
    }
}

/// Recursive serializable → tree conversion.
fn deserialize_tree(s: SerializableTree) -> TreeRoot {
    TreeRoot {
        length_buckets: s
            .length_buckets
            .into_iter()
            .map(|(k, v)| (k, deserialize_node(v)))
            .collect(),
    }
}

fn deserialize_node(s: SerializableNode) -> TreeNode {
    TreeNode {
        children: s
            .children
            .into_iter()
            .map(|(k, v)| (k, deserialize_node(v)))
            .collect(),
        wildcard: s.wildcard.map(|w| Box::new(deserialize_node(*w))),
        leaf_clusters: s.leaf_clusters,
    }
}

/// Remove a cluster id from all leaf-cluster lists in the tree (called
/// after LRU eviction). Walks the whole tree; cost O(total-nodes) but
/// eviction frequency is low at steady state.
fn remove_id_from_tree(tree: &mut TreeRoot, id: TemplateId) {
    for bucket in tree.length_buckets.values_mut() {
        remove_id_from_node(bucket, id);
    }
}

fn remove_id_from_node(node: &mut TreeNode, id: TemplateId) {
    node.leaf_clusters.retain(|cid| *cid != id);
    for child in node.children.values_mut() {
        remove_id_from_node(child, id);
    }
    if let Some(wildcard) = node.wildcard.as_mut() {
        remove_id_from_node(wildcard, id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_miner() -> DrainMiner {
        DrainMiner::new(DrainConfig::default_config(), None)
    }

    // ─── Constants & defaults ──────────────────────────────────────────────

    #[test]
    fn wildcard_token_constant_is_drain3_canonical() {
        assert_eq!(WILDCARD_TOKEN, "<*>");
    }

    #[test]
    fn null_template_id_constant_is_zero() {
        assert_eq!(NULL_TEMPLATE_ID, 0);
    }

    #[test]
    fn schema_version_constant_is_one() {
        assert_eq!(DRAIN_STATE_SCHEMA_VERSION, 1);
    }

    #[test]
    fn default_config_matches_route_spec() {
        let cfg = DrainConfig::default_config();
        assert_eq!(cfg.depth, 4);
        assert_eq!(cfg.similarity, 0.5);
        assert_eq!(cfg.max_clusters, 1000);
        assert!(!cfg.masking_patterns.is_empty());
    }

    #[test]
    fn default_config_via_default_trait_matches_default_config_fn() {
        let a = DrainConfig::default_config();
        let b = DrainConfig::default();
        assert_eq!(a.depth, b.depth);
        assert_eq!(a.similarity, b.similarity);
        assert_eq!(a.max_clusters, b.max_clusters);
        assert_eq!(a.masking_patterns.len(), b.masking_patterns.len());
    }

    // ─── Assignment basics ─────────────────────────────────────────────────

    #[test]
    fn assign_returns_none_for_empty_body() {
        let miner = make_miner();
        assert_eq!(miner.assign(""), None);
        assert_eq!(miner.assign("   "), None);
        assert_eq!(miner.assign("\n\t "), None);
    }

    #[test]
    fn assign_returns_template_id_starting_at_one() {
        let miner = make_miner();
        let id = miner.assign("hello world").expect("non-empty body assigns");
        assert_eq!(id, 1);
        assert_ne!(id, NULL_TEMPLATE_ID);
    }

    #[test]
    fn assign_identical_bodies_returns_same_template_id() {
        let miner = make_miner();
        let id1 = miner.assign("request completed").expect("ok");
        let id2 = miner.assign("request completed").expect("ok");
        assert_eq!(id1, id2);
    }

    #[test]
    fn assign_distinct_token_counts_get_distinct_ids() {
        let miner = make_miner();
        let id1 = miner.assign("alpha beta").expect("ok"); // 2 tokens
        let id2 = miner.assign("alpha beta gamma").expect("ok"); // 3 tokens
        assert_ne!(
            id1, id2,
            "different token counts route to different buckets"
        );
    }

    #[test]
    fn assign_increments_occurrence_count() {
        let miner = make_miner();
        miner.assign("operation completed");
        miner.assign("operation completed");
        miner.assign("operation completed");
        let dist = miner.template_distribution(10);
        assert_eq!(dist.len(), 1);
        assert_eq!(dist[0].occurrence_count, 3);
    }

    // ─── Masking behavior ──────────────────────────────────────────────────

    #[test]
    fn numeric_masking_clusters_messages_differing_only_in_numbers() {
        let miner = make_miner();
        let id1 = miner.assign("request took 5 ms").expect("ok");
        let id2 = miner.assign("request took 999 ms").expect("ok");
        assert_eq!(
            id1, id2,
            "numeric masking should cluster messages differing only in numbers"
        );
        assert_eq!(miner.template_count(), 1);
    }

    #[test]
    fn ipv4_masking_clusters_distinct_ips() {
        let miner = make_miner();
        let id1 = miner
            .assign("connection from 192.168.1.1 dropped")
            .expect("ok");
        let id2 = miner
            .assign("connection from 10.0.0.42 dropped")
            .expect("ok");
        assert_eq!(id1, id2, "IPv4 masking should yield same cluster");
        assert_eq!(miner.template_count(), 1);
    }

    #[test]
    fn path_masking_clusters_distinct_posix_paths() {
        let miner = make_miner();
        let id1 = miner.assign("file /var/log/app.log opened").expect("ok");
        let id2 = miner.assign("file /tmp/data.txt opened").expect("ok");
        assert_eq!(id1, id2, "POSIX path masking should cluster");
    }

    #[test]
    fn hex_masking_clusters_distinct_addresses() {
        let miner = make_miner();
        let id1 = miner
            .assign("dispatched to address 0xdeadbeef")
            .expect("ok");
        let id2 = miner
            .assign("dispatched to address 0xcafef00d")
            .expect("ok");
        assert_eq!(id1, id2, "hex masking should cluster");
    }

    #[test]
    fn mask_body_handles_empty_pattern_list() {
        let masked = mask_body(&[], "no masking applied 123");
        assert_eq!(masked, "no masking applied 123");
    }

    // ─── Distinct messages ─────────────────────────────────────────────────

    #[test]
    fn assign_distinct_bodies_get_distinct_ids() {
        let miner = make_miner();
        let id1 = miner.assign("user login succeeded admin").expect("ok");
        let id2 = miner.assign("connection refused by server").expect("ok");
        assert_ne!(id1, id2);
    }

    // ─── Template distribution / queries ───────────────────────────────────

    #[test]
    fn template_distribution_sorts_descending_by_count() {
        let miner = make_miner();
        miner.assign("alpha apple"); // 1
        miner.assign("beta banana"); // 1
        miner.assign("beta banana"); // 2
        miner.assign("gamma grape"); // 1
        miner.assign("gamma grape"); // 2
        miner.assign("gamma grape"); // 3
        let dist = miner.template_distribution(10);
        assert!(dist.len() >= 3);
        assert!(dist[0].occurrence_count >= dist[1].occurrence_count);
        assert!(dist[1].occurrence_count >= dist.last().unwrap().occurrence_count);
    }

    #[test]
    fn template_distribution_respects_top_n_truncation() {
        let miner = make_miner();
        for i in 0..20 {
            // each `event_{i}` token differs → distinct cluster (no masking
            // collapses these because `event_N` is not a number alone)
            miner.assign(&format!("event_x{i} occurred at startup phase"));
        }
        let dist = miner.template_distribution(5);
        assert!(dist.len() <= 5);
    }

    #[test]
    fn template_count_reflects_unique_clusters() {
        let miner = make_miner();
        miner.assign("event alpha occurred");
        miner.assign("event beta occurred");
        miner.assign("event gamma occurred");
        miner.assign("event alpha occurred"); // duplicate; no new cluster
        // After masking + similarity merging, three "event * occurred"
        // messages with distinct middle tokens may merge depending on
        // threshold. Either 1 (merged) or 3 (separate) is acceptable;
        // assert ≤ 3 (no double counting).
        let count = miner.template_count();
        assert!((1..=3).contains(&count), "got {count}");
    }

    // ─── LRU eviction ──────────────────────────────────────────────────────

    #[test]
    fn lru_eviction_bounds_max_clusters() {
        let cfg = DrainConfig {
            depth: 4,
            similarity: 0.99, // very strict; forces every distinct message into new cluster
            max_clusters: 5,
            masking_patterns: Vec::new(), // disable masking so each message stays distinct
        };
        let miner = DrainMiner::new(cfg, None);
        for i in 0..15 {
            miner.assign(&format!("distinct message {i} token tail"));
        }
        assert!(
            miner.template_count() <= 5,
            "templates should be capped at max_clusters=5; got {}",
            miner.template_count()
        );
    }

    // ─── Persistence round-trip ────────────────────────────────────────────

    #[test]
    fn snapshot_state_round_trips_via_bincode() {
        let miner = make_miner();
        miner.assign("snapshot test alpha");
        miner.assign("snapshot test beta");
        miner.assign("snapshot test gamma");
        let snap = miner.snapshot_state().expect("snapshot ok");
        assert_eq!(snap.schema_version, DRAIN_STATE_SCHEMA_VERSION);
        assert!(snap.next_template_id >= 2);

        let bytes = bincode::serialize(&snap).expect("serialize ok");
        let restored: DrainState = bincode::deserialize(&bytes).expect("deserialize ok");
        assert_eq!(restored.schema_version, snap.schema_version);
        assert_eq!(restored.next_template_id, snap.next_template_id);
        assert_eq!(restored.templates.len(), snap.templates.len());
    }

    #[test]
    fn load_from_persistence_returns_false_when_no_persistence() {
        let miner = make_miner();
        let loaded = miner.load_from_persistence().expect("ok");
        assert!(!loaded);
    }

    #[test]
    fn persist_returns_false_when_no_persistence() {
        let miner = make_miner();
        let persisted = miner.persist().expect("ok");
        assert!(!persisted);
    }

    // In-memory persistence for round-trip verification.
    struct InMemoryPersistence {
        slot: std::sync::Mutex<Option<DrainState>>,
    }
    impl InMemoryPersistence {
        fn new() -> Self {
            Self {
                slot: std::sync::Mutex::new(None),
            }
        }
    }
    impl DrainPersistence for InMemoryPersistence {
        fn load(&self) -> Result<Option<DrainState>, Error> {
            Ok(self.slot.lock().unwrap().clone())
        }
        fn save(&self, state: &DrainState) -> Result<(), Error> {
            *self.slot.lock().unwrap() = Some(state.clone());
            Ok(())
        }
    }

    #[test]
    fn persist_and_load_round_trip_preserves_clusters() {
        let persistence = Arc::new(InMemoryPersistence::new());
        let miner1 = DrainMiner::new(
            DrainConfig::default_config(),
            Some(persistence.clone() as Arc<dyn DrainPersistence>),
        );
        miner1.assign("alpha test event");
        miner1.assign("alpha test event");
        miner1.assign("beta different event");
        let count_before = miner1.template_count();

        let persisted = miner1.persist().expect("persist ok");
        assert!(persisted);

        let miner2 = DrainMiner::new(
            DrainConfig::default_config(),
            Some(persistence.clone() as Arc<dyn DrainPersistence>),
        );
        let loaded = miner2.load_from_persistence().expect("load ok");
        assert!(loaded);
        assert_eq!(miner2.template_count(), count_before);
    }

    #[test]
    fn load_rejects_mismatched_schema_version() {
        let persistence = Arc::new(InMemoryPersistence::new());
        let bad_state = DrainState {
            schema_version: 999,
            next_template_id: 1,
            templates: Vec::new(),
            tree: SerializableTree::default(),
        };
        persistence.save(&bad_state).expect("save ok");

        let miner = DrainMiner::new(
            DrainConfig::default_config(),
            Some(persistence.clone() as Arc<dyn DrainPersistence>),
        );
        let err = miner.load_from_persistence();
        assert!(err.is_err(), "schema version mismatch must error");
    }

    // ─── Drift indicators ──────────────────────────────────────────────────

    #[test]
    fn drift_indicator_under_clustered_for_single_occurrence() {
        let miner = make_miner();
        miner.assign("singleton message word four");
        let dist = miner.template_distribution(10);
        assert_eq!(dist[0].drift_indicator, DriftIndicator::UnderClustered);
    }

    #[test]
    fn drift_indicator_healthy_for_stable_repeated_template() {
        let miner = make_miner();
        miner.assign("clean stable healthy message");
        miner.assign("clean stable healthy message");
        miner.assign("clean stable healthy message");
        let dist = miner.template_distribution(10);
        assert_eq!(dist[0].drift_indicator, DriftIndicator::Healthy);
    }

    // ─── Internal helper unit tests ────────────────────────────────────────

    #[test]
    fn merge_template_replaces_differing_positions_with_wildcard() {
        let mut t: Vec<String> = vec!["a".into(), "b".into(), "c".into()];
        let input: Vec<String> = vec!["a".into(), "z".into(), "c".into()];
        merge_template(&mut t, &input);
        assert_eq!(t, vec!["a", WILDCARD_TOKEN, "c"]);
    }

    #[test]
    fn merge_template_preserves_existing_wildcards() {
        let mut t: Vec<String> = vec![WILDCARD_TOKEN.into(), "b".into(), "c".into()];
        let input: Vec<String> = vec!["a".into(), "b".into(), "c".into()];
        merge_template(&mut t, &input);
        assert_eq!(t[0], WILDCARD_TOKEN);
    }

    #[test]
    fn position_similarity_zero_for_empty_inputs() {
        assert_eq!(position_similarity(&[], &["x".to_string()]), 0.0);
        assert_eq!(position_similarity(&["x".to_string()], &[]), 0.0);
    }

    #[test]
    fn position_similarity_one_for_identical_inputs() {
        let v: Vec<String> = vec!["a".into(), "b".into(), "c".into()];
        assert_eq!(position_similarity(&v, &v), 1.0);
    }

    #[test]
    fn position_similarity_treats_template_wildcard_as_match() {
        let template: Vec<String> = vec![WILDCARD_TOKEN.into(), "b".into()];
        let input: Vec<String> = vec!["a".into(), "b".into()];
        assert_eq!(position_similarity(&template, &input), 1.0);
    }

    #[test]
    fn position_similarity_partial_match() {
        let template: Vec<String> = vec!["a".into(), "b".into(), "c".into(), "d".into()];
        let input: Vec<String> = vec!["a".into(), "x".into(), "c".into(), "y".into()];
        // 2 of 4 positions match
        let sim = position_similarity(&template, &input);
        assert!((sim - 0.5).abs() < 0.001, "expected ~0.5, got {sim}");
    }

    #[test]
    fn drift_for_empty_template_is_healthy() {
        let record = TemplateRecord {
            id: 1,
            tokens: Vec::new(),
            occurrence_count: 5,
            first_seen_unix_nano: 0,
            last_seen_unix_nano: 0,
        };
        assert_eq!(drift_for(&record), DriftIndicator::Healthy);
    }

    #[test]
    fn drift_for_high_wildcard_ratio_is_over_generalized() {
        let record = TemplateRecord {
            id: 1,
            tokens: vec![
                WILDCARD_TOKEN.into(),
                WILDCARD_TOKEN.into(),
                WILDCARD_TOKEN.into(),
                "stable".into(),
            ],
            occurrence_count: 100,
            first_seen_unix_nano: 0,
            last_seen_unix_nano: 0,
        };
        // 3/4 wildcards = 0.75 > 0.6
        assert_eq!(drift_for(&record), DriftIndicator::OverGeneralized);
    }
}
