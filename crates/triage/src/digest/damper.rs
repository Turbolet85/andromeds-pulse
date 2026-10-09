//! L4 generation damper — gates generations on digest INPUT state.
//!
//! A digest whose condition projection matches the last SUCCESSFULLY
//! generated one for its condition identity is suppressed before the
//! inference runner is invoked. Suppression happens upstream of the
//! incident producer, so a suppressed re-emission never bumps
//! `updated_at` and the auto-resolve window runs out exactly once per
//! unchanged condition episode.
//!
//! The projection deliberately EXCLUDES model output (arch §Established
//! Decisions [Fault Identity] — model-authored values are constant under
//! the deterministic runner) AND retrieval context (`corpus_matches`,
//! `incident_refs`, `payload_summary` — the payload embeds both, and
//! corpus churn, including this damper's own convergence dynamics, would
//! self-perturb the key). What remains is first-hand condition state:
//! digest kind, workspace, the cue tuples, and the Q1-derived service
//! rows.

use std::sync::atomic::{AtomicU64, Ordering};

use dashmap::DashMap;

use crate::contract::{CueKind, Digest, DigestKind};

/// Eviction bound for CUE-BEARING condition identities. A live cue
/// condition re-arrives at the CueLatch refractory rate (60s), so a
/// 300s absence means the condition CLEARED — the entry dies and a new
/// episode of the same condition re-analyzes as first-seen, mirroring
/// the latch's cleared-condition eviction semantics.
pub const DAMPER_CUE_EVICTION_SECONDS: i64 = 300;

/// Eviction bound for interval-driven (no-cue) identities. The
/// reflection cadence arrives every 1800s, so its entry must survive
/// that gap; 3900s covers one missed interval plus margin.
pub const DAMPER_INTERVAL_EVICTION_SECONDS: i64 = 3900;

/// Why a digest was admitted to generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerateReason {
    /// No successful generation is recorded for this identity (truly
    /// first, post-eviction, or every prior attempt failed).
    FirstSeen,
    /// The condition projection differs from the last generated one.
    Changed,
    /// Lifecycle kinds (`ResolutionSummary` / `resolution_event`) are
    /// never damped — they are events, not re-analysis.
    BypassKind,
}

/// Bounded static label for [`GenerateReason`] — tracing-field value.
pub fn generate_reason_label(reason: GenerateReason) -> &'static str {
    match reason {
        GenerateReason::FirstSeen => "first_seen",
        GenerateReason::Changed => "changed",
        GenerateReason::BypassKind => "bypass_kind",
    }
}

/// One damper decision. `released_run_len` / `run_len` carry the length
/// of the suppression run so the subscriber can emit ONCE-per-transition
/// records (engage on the first suppression, release on the generate
/// that ends a run) — never per-decision records.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DamperVerdict {
    Generate {
        reason: GenerateReason,
        /// Non-zero when this generate ends a suppression run (emit the
        /// release transition record).
        released_run_len: u64,
    },
    Suppress {
        /// True on the FIRST suppression of a run (emit the engage
        /// transition record).
        engaged: bool,
        run_len: u64,
    },
}

/// Condition identity: the cue-identity tuple for cue-bearing digests
/// (the same `(kind, scope_id)` family the CueLatch and incident
/// coalescing key on), `(workspace, digest kind)` for interval-driven
/// digests. Keys stay in memory and are never logged.
type ConditionIdentity = (String, DigestKind, Option<(CueKind, String)>);

#[derive(Debug)]
struct DamperEntry {
    last_generated_projection: Option<String>,
    last_seen_nanos: i64,
    suppressed_since_generate: u64,
    has_cue: bool,
}

/// Per-identity unchanged-input gate with cumulative counters. Owned by
/// the binary boundary (constructed in `pulse-app/src/main.rs`, shared
/// by the L4 subscriber and the L4 heartbeat).
#[derive(Debug, Default)]
pub struct GenerationDamper {
    entries: DashMap<ConditionIdentity, DamperEntry>,
    suppressed_total: AtomicU64,
    run_total: AtomicU64,
}

impl GenerationDamper {
    pub fn new() -> Self {
        Self::default()
    }

    /// Cumulative generations suppressed by the damper.
    pub fn generations_suppressed_total(&self) -> u64 {
        self.suppressed_total.load(Ordering::Relaxed)
    }

    /// Cumulative generations admitted (including bypass kinds).
    pub fn generations_run_total(&self) -> u64 {
        self.run_total.load(Ordering::Relaxed)
    }

    /// Live condition identities currently tracked.
    pub fn tracked(&self) -> usize {
        self.entries.len()
    }

    fn is_bypass(digest: &Digest) -> bool {
        matches!(digest.kind, DigestKind::ResolutionSummary) || digest.resolution_event
    }

    fn identity(digest: &Digest) -> ConditionIdentity {
        let cue = digest
            .attention_cues
            .first()
            .map(|c| (c.kind, c.scope_id.clone().unwrap_or_default()));
        (digest.workspace.clone(), digest.kind, cue)
    }

    /// Decide whether `digest` warrants a generation. Pure w.r.t. the
    /// injected `now_nanos` (tests supply deterministic instants).
    pub fn decide(&self, digest: &Digest, now_nanos: i64) -> DamperVerdict {
        if Self::is_bypass(digest) {
            self.run_total.fetch_add(1, Ordering::Relaxed);
            return DamperVerdict::Generate {
                reason: GenerateReason::BypassKind,
                released_run_len: 0,
            };
        }
        self.evict_stale(now_nanos);
        let projection = project(digest);
        let mut entry = self
            .entries
            .entry(Self::identity(digest))
            .or_insert_with(|| DamperEntry {
                last_generated_projection: None,
                last_seen_nanos: now_nanos,
                suppressed_since_generate: 0,
                has_cue: !digest.attention_cues.is_empty(),
            });
        entry.last_seen_nanos = now_nanos;
        let unchanged = entry
            .last_generated_projection
            .as_deref()
            .is_some_and(|prev| prev == projection);
        if unchanged {
            entry.suppressed_since_generate += 1;
            let run_len = entry.suppressed_since_generate;
            self.suppressed_total.fetch_add(1, Ordering::Relaxed);
            return DamperVerdict::Suppress {
                engaged: run_len == 1,
                run_len,
            };
        }
        let reason = if entry.last_generated_projection.is_some() {
            GenerateReason::Changed
        } else {
            GenerateReason::FirstSeen
        };
        let released_run_len = entry.suppressed_since_generate;
        entry.suppressed_since_generate = 0;
        self.run_total.fetch_add(1, Ordering::Relaxed);
        DamperVerdict::Generate {
            reason,
            released_run_len,
        }
    }

    /// Record a SUCCESSFUL generation for `digest`. The caller invokes
    /// this only on a clean parse — a failed generation must never mark
    /// its content analyzed, or a failing kind would self-suppress after
    /// one attempt.
    pub fn record_generated(&self, digest: &Digest, now_nanos: i64) {
        if Self::is_bypass(digest) {
            return;
        }
        let mut entry = self
            .entries
            .entry(Self::identity(digest))
            .or_insert_with(|| DamperEntry {
                last_generated_projection: None,
                last_seen_nanos: now_nanos,
                suppressed_since_generate: 0,
                has_cue: !digest.attention_cues.is_empty(),
            });
        entry.last_seen_nanos = now_nanos;
        entry.last_generated_projection = Some(project(digest));
    }

    fn evict_stale(&self, now_nanos: i64) {
        self.entries.retain(|_, e| {
            let bound_seconds = if e.has_cue {
                DAMPER_CUE_EVICTION_SECONDS
            } else {
                DAMPER_INTERVAL_EVICTION_SECONDS
            };
            now_nanos.saturating_sub(e.last_seen_nanos)
                < bound_seconds.saturating_mul(1_000_000_000)
        });
    }
}

/// Length-prefix a field so no content byte can alias the record
/// structure (two different digests can never project equally by
/// embedding separators).
fn push_field(out: &mut String, field: &str) {
    out.push_str(&field.len().to_string());
    out.push(':');
    out.push_str(field);
    out.push(';');
}

/// Deterministic projection of a digest's first-hand condition state.
fn project(digest: &Digest) -> String {
    let mut out = String::with_capacity(256);
    push_field(&mut out, &format!("{:?}", digest.kind));
    push_field(&mut out, &digest.workspace);
    push_field(
        &mut out,
        if digest.active_incident_bypass {
            "bypass"
        } else {
            "direct"
        },
    );
    let mut cues: Vec<String> = digest
        .attention_cues
        .iter()
        .map(|c| {
            format!(
                "{:?}|{:?}|{:?}|{}|{}",
                c.kind,
                c.priority_tier,
                c.scope,
                c.scope_id.as_deref().unwrap_or(""),
                c.fingerprint.as_deref().unwrap_or(""),
            )
        })
        .collect();
    cues.sort();
    for cue in &cues {
        push_field(&mut out, cue);
    }
    let mut services: Vec<String> = digest
        .services
        .iter()
        .map(|s| {
            format!(
                "{}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}",
                s.service,
                s.rate_per_sec,
                s.rate_baseline_per_sec,
                s.error_rate,
                s.error_rate_baseline,
                s.p99_latency_ms,
                s.p99_baseline_ms,
            )
        })
        .collect();
    services.sort();
    for service in &services {
        push_field(&mut out, service);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract::{CueScope, DigestCueRef, DigestLwwMode, DigestServiceRow, PriorityTier};

    const T0: i64 = 1_700_000_000_000_000_000;
    const SEC: i64 = 1_000_000_000;

    fn cue(kind: CueKind, scope_id: &str, fingerprint: Option<&str>) -> DigestCueRef {
        DigestCueRef {
            kind,
            priority_tier: PriorityTier::Autonomous,
            summary: format!("{kind:?} scope_id={scope_id}"),
            scope: CueScope::Service,
            fingerprint: fingerprint.map(str::to_string),
            scope_id: Some(scope_id.to_string()),
        }
    }

    fn digest(kind: DigestKind, cues: Vec<DigestCueRef>) -> Digest {
        Digest {
            kind,
            token_count: 41,
            payload_summary: "payload".to_string(),
            incident_refs: Vec::new(),
            generated_at_unix_nano: T0,
            workspace: "D:/ws/project".to_string(),
            window_start_unix_nano: T0 - 60 * SEC,
            window_end_unix_nano: T0,
            services: Vec::new(),
            attention_cues: cues,
            corpus_matches: Vec::new(),
            lww_mode: DigestLwwMode::Default,
            active_incident_bypass: false,
            resolution_event: false,
        }
    }

    fn tier1_digest(scope_id: &str) -> Digest {
        digest(
            DigestKind::CadenceTier1,
            vec![cue(CueKind::ServiceWentSilent, scope_id, None)],
        )
    }

    #[test]
    fn first_seen_generates_then_unchanged_suppresses() {
        let damper = GenerationDamper::new();
        let d = tier1_digest("svc-a");
        assert_eq!(
            damper.decide(&d, T0),
            DamperVerdict::Generate {
                reason: GenerateReason::FirstSeen,
                released_run_len: 0
            }
        );
        damper.record_generated(&d, T0);
        assert_eq!(
            damper.decide(&d, T0 + 60 * SEC),
            DamperVerdict::Suppress {
                engaged: true,
                run_len: 1
            }
        );
        assert_eq!(
            damper.decide(&d, T0 + 120 * SEC),
            DamperVerdict::Suppress {
                engaged: false,
                run_len: 2
            }
        );
    }

    #[test]
    fn changed_projection_generates_and_reports_the_released_run() {
        let damper = GenerationDamper::new();
        let unchanged = tier1_digest("svc-a");
        damper.decide(&unchanged, T0);
        damper.record_generated(&unchanged, T0);
        damper.decide(&unchanged, T0 + 60 * SEC);
        damper.decide(&unchanged, T0 + 120 * SEC);
        let mut changed = tier1_digest("svc-a");
        changed.attention_cues[0].fingerprint = Some("deadbeef".to_string());
        assert_eq!(
            damper.decide(&changed, T0 + 180 * SEC),
            DamperVerdict::Generate {
                reason: GenerateReason::Changed,
                released_run_len: 2
            }
        );
    }

    #[test]
    fn state_updates_only_on_recorded_success() {
        let damper = GenerationDamper::new();
        let d = tier1_digest("svc-a");
        assert!(matches!(
            damper.decide(&d, T0),
            DamperVerdict::Generate {
                reason: GenerateReason::FirstSeen,
                ..
            }
        ));
        // No record_generated (the generation failed) — the same content
        // must be retried, never suppressed.
        assert!(matches!(
            damper.decide(&d, T0 + 60 * SEC),
            DamperVerdict::Generate {
                reason: GenerateReason::FirstSeen,
                ..
            }
        ));
    }

    #[test]
    fn bypass_kinds_are_never_suppressed() {
        let damper = GenerationDamper::new();
        let resolution = digest(DigestKind::ResolutionSummary, Vec::new());
        damper.record_generated(&resolution, T0);
        assert_eq!(
            damper.decide(&resolution, T0 + SEC),
            DamperVerdict::Generate {
                reason: GenerateReason::BypassKind,
                released_run_len: 0
            }
        );
        let mut resolution_event = digest(DigestKind::CadenceTier3, Vec::new());
        resolution_event.resolution_event = true;
        damper.record_generated(&resolution_event, T0);
        assert_eq!(
            damper.decide(&resolution_event, T0 + SEC),
            DamperVerdict::Generate {
                reason: GenerateReason::BypassKind,
                released_run_len: 0
            }
        );
    }

    #[test]
    fn cleared_cue_condition_evicts_so_a_new_episode_reanalyzes() {
        let damper = GenerationDamper::new();
        let d = tier1_digest("svc-a");
        damper.decide(&d, T0);
        damper.record_generated(&d, T0);
        assert!(matches!(
            damper.decide(&d, T0 + 60 * SEC),
            DamperVerdict::Suppress { .. }
        ));
        // Absent past the cue eviction bound — a recurrence is a NEW
        // episode and generates as first-seen.
        let later = T0 + 60 * SEC + (DAMPER_CUE_EVICTION_SECONDS + 1) * SEC;
        assert!(matches!(
            damper.decide(&d, later),
            DamperVerdict::Generate {
                reason: GenerateReason::FirstSeen,
                ..
            }
        ));
    }

    #[test]
    fn interval_identity_survives_the_reflection_gap() {
        let damper = GenerationDamper::new();
        let d = digest(DigestKind::Reflection, Vec::new());
        damper.decide(&d, T0);
        damper.record_generated(&d, T0);
        // One full reflection interval later the entry must still stand.
        assert!(matches!(
            damper.decide(&d, T0 + 1800 * SEC),
            DamperVerdict::Suppress { .. }
        ));
    }

    #[test]
    fn distinct_identities_damp_independently() {
        let damper = GenerationDamper::new();
        let a = tier1_digest("svc-a");
        let b = tier1_digest("svc-b");
        damper.decide(&a, T0);
        damper.record_generated(&a, T0);
        assert!(matches!(
            damper.decide(&b, T0 + SEC),
            DamperVerdict::Generate {
                reason: GenerateReason::FirstSeen,
                ..
            }
        ));
        assert!(matches!(
            damper.decide(&a, T0 + 2 * SEC),
            DamperVerdict::Suppress { .. }
        ));
        assert_eq!(damper.tracked(), 2);
    }

    #[test]
    fn changed_service_rows_generate() {
        let damper = GenerationDamper::new();
        let mut d = digest(DigestKind::CadenceTier3, Vec::new());
        d.services.push(DigestServiceRow {
            service: "checkout".to_string(),
            rate_per_sec: 1.0,
            rate_baseline_per_sec: 1.0,
            error_rate: 0.0,
            error_rate_baseline: 0.0,
            p99_latency_ms: 10.0,
            p99_baseline_ms: 10.0,
        });
        damper.decide(&d, T0);
        damper.record_generated(&d, T0);
        assert!(matches!(
            damper.decide(&d, T0 + 60 * SEC),
            DamperVerdict::Suppress { .. }
        ));
        d.services[0].error_rate = 0.5;
        assert!(matches!(
            damper.decide(&d, T0 + 120 * SEC),
            DamperVerdict::Generate {
                reason: GenerateReason::Changed,
                ..
            }
        ));
    }

    #[test]
    fn retrieval_context_is_outside_the_projection() {
        // payload_summary / corpus_matches / incident_refs churn with
        // corpus writes (including this damper's own convergence) — two
        // digests differing ONLY there are the same condition.
        let damper = GenerationDamper::new();
        let d = tier1_digest("svc-a");
        damper.decide(&d, T0);
        damper.record_generated(&d, T0);
        let mut retrieval_only = tier1_digest("svc-a");
        retrieval_only.payload_summary = "entirely different render".to_string();
        retrieval_only.corpus_matches = vec!["match line".to_string()];
        retrieval_only.incident_refs = vec!["7".to_string()];
        retrieval_only.generated_at_unix_nano = T0 + 60 * SEC;
        retrieval_only.window_start_unix_nano = T0;
        retrieval_only.window_end_unix_nano = T0 + 60 * SEC;
        assert!(matches!(
            damper.decide(&retrieval_only, T0 + 60 * SEC),
            DamperVerdict::Suppress { .. }
        ));
    }

    #[test]
    fn counters_track_runs_and_suppressions() {
        let damper = GenerationDamper::new();
        let d = tier1_digest("svc-a");
        damper.decide(&d, T0);
        damper.record_generated(&d, T0);
        damper.decide(&d, T0 + 60 * SEC);
        damper.decide(&d, T0 + 120 * SEC);
        assert_eq!(damper.generations_run_total(), 1);
        assert_eq!(damper.generations_suppressed_total(), 2);
    }
}
