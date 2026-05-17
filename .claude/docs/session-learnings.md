# Session Learnings

_This file is curated by `/wrap-session`. Learnings captured here are too detailed or specific for CLAUDE.md but worth preserving as reference material for future sessions._

_Entries are added in reverse chronological order (newest first). Each entry has an ISO date, short title, and body._

_This file is entirely wrap-session's territory. `/setup-project` creates it if missing but NEVER regenerates it. Manual edits are preserved across all Andromeda skill runs._

---

## 2026-05-17 (session 82) — Bundled `--delta` commit when prior uncommitted refactor exists (confidence 0.75)

When `/andromeda-setup-project --delta` is invoked with a working tree that carries uncommitted work BEYOND the delta scope (e.g., a prior cosmetic refactor pass that wasn't committed yet, status updates from earlier planning, etc.), the strict protocol guidance "stage only delta-scoped files" doesn't map cleanly. Splitting via `git add -p` is technically possible but risky for compounded edits to the same file (e.g., arch.md had BOTH retroactive refactor edits AND new Type 6 amendment edits this session — both touched §Architecture Registry Updates but in different ways).

**Pragmatic pattern:** bundle into one commit with a comprehensive message that documents both streams (primary = delta-rerun; secondary = bundled prior work). The commit message body should clearly separate "delta-rerun (this session's primary work)" from "bundled work (this session, pre-/delta)". Project history precedent: `2dded9f` + `c836aae` both bundle multiple amendment cascades in single setup-project --delta commits.

**Trade-off:** deviates from the strict per-protocol "delta-scoped files only" discipline, but maintains audit-trail clarity via the comprehensive commit message. Surface the bundling explicitly in the post-Phase-9 report so the user can choose to split via `git reset HEAD~1 && git add -p ...` if they prefer cleaner two-commit history.

**When to split into separate commits instead:** when the prior uncommitted work touches DIFFERENT files than the delta scope (no shared file edits → clean `git add <specific files>` per commit; no interactive splitting needed). The current session bundled because arch.md had both stream edits — splitting required interactive staging which is error-prone.

Generalizes to any future `/andromeda-setup-project --delta` invocation where the working tree carries multi-stream uncommitted work. Companion to Proposal 10 in `docs/andromeda-improvements.md` (which proposes protocol-level enhancement for detection + guidance).

---

## 2026-05-17 (session 82) — Compact-format marker ↔ Decisions Log entry duality validated via dogfood (confidence 0.80)

P8 Phase 1 + P9 Phase 1 (landed in skill files at `~/.claude/skills/andromeda-evolve/` earlier this session) introduced compact Decisions Log entry templates: 5-content-line for Type 6 (arch.md §Architecture Registry Updates), 4-content-bullet for Type 7 (route.md §3). Verbose detail relocates from the inline Decisions Log entry to the amendment marker file at `.andromeda/runs/{ISO}-spec-amendment-{slug}/amendment.md`, with the Decisions Log entry citing the marker via `**Marker:**` field.

**Dogfood validation this session:** the retroactive refactor pass exercised compact form across 7 historical Type 6 entries + 8 historical Type 7 entries (a comprehensive replay of the templates against real content); then the new Type 6 amendment for `pulse://stream/restart-events` (chunk #63 D3 drift remediation) authored a fresh 8th compact entry going forward. All entries fit the canonical templates without ack-required deviations (Check 7.6 / Check 8.8 returned clean).

**Information-flow design:** marker file is the audit-trail snapshot (verbose Authority paragraph + multi-sentence Rationale + detailed Impact analysis + Check sub-results table); Decisions Log entry is the quick-scan summary (Section / Added / Rationale / Marker for Type 6; Insert / Why / Mechanical / Marker for Type 7). The duality is intentional — Decisions Log entries appear inline in canonical specs (arch.md / route.md) so they must be glanceable; marker files live in run-dirs so verbose detail doesn't bloat the specs.

**Reader pattern:** scanning the Decisions Log gives the gist + flag citation + chunk reference + marker path. Following `**Marker:**` to the marker file gives the full audit detail when needed. This pattern preserves the audit trail without polluting glance-readability. Apply to ANY future /andromeda-evolve amendment authored under the compact template (Type 6 via --allow-arch-registry, Type 7 via --allow-route-append).

**Skill-internal record:** the compact templates live in `~/.claude/skills/andromeda-evolve/references/output-templates.md` §Family default — Type 6 / Family default — Type 7. The verbose pre-P8/P9 templates are preserved in HTML comment blocks for historical reference but new entries MUST use the compact form.

---

## 2026-05-17 (session 79) — EWMA convergence in N-sample tests is misleading at production alpha (confidence 0.85)

When writing unit tests against `crates/triage/src/baseline/EwmaTracker` (alpha=0.00333, 5-min window), seeding strategies that assume "N errors in M samples → N/M error rate" produce wildly incorrect EWMA values at typical test scale (100 samples). With alpha=0.00333, a single initial error observation sets EWMA=1.0; 99 subsequent non-error observations decay it via `value = 0.99667 * value` к ~0.717 — STILL above any sub-50% threshold. Tests asserting "1 error in 100 → below 3% threshold" fail because actual EWMA is ~71% NOT 1%. Discovered at chunk #62 cue emitter tests (`evaluate_thresholds_low_error_rate_does_not_emit_cue` + `evaluate_thresholds_classification_flips_when_multiplier_raised` both failed on first run).

**Resolution patterns for cue/baseline emit tests:**

1. **Below threshold:** seed 0 errors. EWMA stays at exactly 0.0 (first observation = 0; all subsequent = 0). Reliable below-threshold without convergence wait.
2. **Above threshold + clearly classified:** seed HIGH error counts (50/100) — EWMA converges to ~85%; safely above any sub-100% threshold; classify by setting multiplier to suppress (e.g., multiplier=100 → threshold=100% → cue suppressed when EWMA=85%).
3. **Borderline cases:** AVOID — the 5-min window doesn't converge to 4%/10%/etc. в 100 samples regardless of seeding pattern. Use multipliers that flip Hard→Suggested transitions instead of magnitude-based tests.

Apply to ANY future test in `crates/triage/` that uses BaselineState. The 5-min EWMA window is calibrated for streaming production traffic, not 100-sample unit tests; alternating injection (every Nth sample is error) would converge but adds test complexity. Pre-emptively reach for option 1 (0 errors) or option 2 (high errors + multiplier flip) over magnitude-based assertions.

---

## 2026-05-17 (session 79) — Buffer consumer is the canonical baseline-tap point (not ingest hot-path) for cross-crate span observation (confidence 0.80)

When a downstream crate (chunk #62 `triage::BaselineState`) needs к observe every decoded OTLP span without taking a sibling dep on `ingest`, the buffer crate's `run_consumer` is the cleaner tap point than per-receiver wiring through `ingest/src/{grpc,http}.rs`. Rationale:

1. **Buffer already iterates decoded spans** (`build_spans_record_batch` walks ResourceSpans → ScopeSpans → Span for the Arrow record batch); adding a parallel `observe_spans_for_baseline` walk is trivial vs threading `Arc<dyn SpanObserver>` through 2 separate receiver handlers + their generated tonic code paths.
2. **Single tap point** covers all OTLP traffic regardless of transport (gRPC + HTTP).
3. **Buffer already depends on ingest** (sanctioned per arch §Module dependency direction for the Batch enum); no new workspace dep edge needed.
4. **Trait-in-lower-crate + impl-in-pulse-app preserved**: `SpanObserver` trait lives in `crates/ingest/src/observer.rs` (call site); `BaselineObserverAdapter` impl lives at `pulse-app/src/baseline_observer.rs` boundary wrapping `Arc<triage::BaselineState>`. Mirrors chunk #59 `ReceiverBindStatus` precedent.
5. **Trade-off:** observation happens BEFORE `spawn_blocking` for DuckDB write but AFTER the batch is constructed (already past invariant checks). Slightly later in pipeline vs per-receiver tap, but pre-spawn_blocking so doesn't block on DuckDB I/O. Acceptable for chunk #62 cadence (1s tick reads stable state regardless of mid-batch timing).

**Generalization:** any future cross-crate state delivery where the consumer needs decoded spans (cross-spec metric aggregators, custom counters, future incident detectors) should default к buffer's consumer tap rather than per-receiver wiring. The chunk #62 plan originally specified per-receiver tap but Phase 1 research surfaced buffer as the simpler home; the deviation was in-scope per Phase 2 §Bounded retry caps + strict scope classification.

---

## 2026-05-17 — Dogfood Andromeda improvements via the next pending cascade (confidence 0.85)

When landing improvements to user-level Andromeda skill files (`~/.claude/skills/andromeda-*/`), sequence them IMMEDIATELY before the next pending `/andromeda-evolve` invocation rather than as a standalone improvement-only session. The next pending cascade IS the live test — verifies mechanical operation under real conditions rather than synthetic ones. Validated session 78 by landing Proposals 5 (Type 7 cascade visibility) + 6 (Form 1 §1 mechanical update) immediately before the chunk #62 `--allow-route-append` cascade. Result: both proposals exercised end-to-end (evolve marker pre-populated, route §1 mechanically incremented, setup-project --delta grep-expansion found ZERO additional files because P5 pre-populated successfully). Pattern generalizes: bundle improvement landing + first dogfood cascade in the same session for maximum verification feedback. Caveat: only works when the next cascade exercises the improved code path. P7 (Type 6 narrative-cascade) was landed in same session 78 but NOT exercised because chunk #62 is Type 7, not Type 6 — P7 awaits next `--allow-arch-registry` invocation for live test. So "next pending cascade" must match the improved flag's classification; otherwise improvement lands without immediate validation and accumulates "pending live test" debt.

---

## 2026-05-17 — Cross-file consistency grep methodology when extending flag scope (confidence 0.80)

When extending the behavior or scope of an Andromeda flag (`--allow-arch-registry`, `--allow-route-append`, etc.) across reference files, grep ALL sibling reference files for citations of the OLD constraint BEFORE landing the change, and re-grep AFTER to verify staleness. Discovered during session 78 P6 work (Form 1 §1 mechanical update): the plan covered 5 target files (SKILL.md / output-templates / refuse-taxonomy / validation-checks / delta-rerun-protocol), but a final cross-file grep for `Form 2 only.*scope_summary` surfaced a stale citation in `classification-taxonomy.md:446` — `scope_summary_updates` was still labeled "Form 2 only" even though P6 made it always-present. Fixed in the same session via two additional edits. Generalization: for any flag-scope extension, run `grep -rnE 'Form X only' ~/.claude/skills/andromeda-{evolve,setup-project,wrap-session,new-session}/` (or the equivalent constraint phrasing) before declaring the change complete. The session 78 cross-file consistency review caught the drift before it shipped; without the grep step, classification-taxonomy.md would have drifted from the SKILL spec for the lifetime of the next /andromeda-evolve run. Apply this discipline to ANY future flag-scope extension (Type 6 / Type 7 / future Type 8 per Proposal 1).

---

## 2026-05-17 (session 77) — `#[allow(dead_code)]` impl-block pattern for chunk-substrate primitives consumed by future chunks

**Context:** chunk #61 implementation delivered three callable + testable algorithm primitives (`EwmaTracker`, `RollingWindow<T>`, `TDigestPair`) in `crates/triage/src/baseline/{ewma,rolling_window,tdigest_pair}.rs`. Each primitive type exposes `pub` accessor methods (`alpha()` / `samples()` / `last_update_nanos()` for EwmaTracker; `len()` / `capacity()` / `iter()` / `sum()` / `mean()` for RollingWindow; `samples_current()` / `centroid_count()` for TDigestPair) that are exercised by `#[cfg(test)] mod tests` blocks but NOT called by the lib (non-test) code path. `BaselineState`'s public API (`error_rate(service)` / `latency_percentile(service, op, q)` / `total_centroid_count()`) intentionally does NOT drill into the primitives' internal accessors — it exposes only aggregate query semantics for chunk #62 (attention cue emitter) к consume. Result: `cargo clippy --workspace --all-targets --all-features -- -D warnings` reported 5 `clippy::dead_code` errors across lib build (`methods samples, last_update_nanos, alpha never used` etc.) blocking the standard gate baseline.

**Discipline:** When a chunk delivers `pub` accessor methods on substrate types as **future-API surface** for a downstream consumer chunk (route number known + named, NOT speculative), add `#[allow(dead_code)]` к the **impl block** (not the type) with a comment naming the consuming chunk:

```rust
// Chunk #61 deliverable: callable + testable primitives for chunk #62
// attention cue emitter. Accessor methods (samples / last_update_nanos /
// alpha) exercised via tests; allow(dead_code) signals future API surface
// для emitter + percentile-snapshot consumers.
#[allow(dead_code)]
impl EwmaTracker { ... }
```

This is preferable к: (a) silently deleting unused methods (deletes verified-tested future API); (b) `#[allow(dead_code)]` at the type level (overscoped — applies to ALL items including private internals); (c) calling the methods from lib code with `let _ = x.alpha();` (creates false coupling that's harder к refactor).

**Pre-emptive method removal:** if a method is unused in BOTH lib AND tests, delete it outright (`RollingWindow::is_empty()`, `TDigestPair::samples()` + `last_swap_nanos()` were deleted at chunk #61 cleanup). The `#[allow(dead_code)]` exception applies only к the lib-vs-tests asymmetry — both consumer in tests + future-consumer-chunk-named.

**Verified:** chunk #61 `crates/triage/src/baseline/{ewma.rs,rolling_window.rs,tdigest_pair.rs}` — 3 impl-level `#[allow(dead_code)]` annotations + 3 method deletions cleared the gate. Total lib-side surface = methods used + future-chunk surface; both intentional, none accidental. Confidence 0.78 — pattern resolves a real-and-recurring clippy posture conflict; future infrastructure chunks (any chunk delivering primitives + persistence + state types for downstream chunks к consume) will encounter the same shape. Applies generally — not chunk-#61-specific.

**When applicable:** any chunk delivering substrate primitives (algorithm types, persistence layer, IPC contract types) where:
- Methods are public surface к support testability OR future-chunk consumption
- The current chunk's own lib code does NOT call those accessors (BaselineState-style aggregator-only API)
- Future chunk is route-named (not speculative; concretely chunk #N+1 in route §2)

Currently chunk #62 (attention cue emitter), chunk #63 (restart event detector), AND future v0.2.0 chunks #64-#88 that consume triage::baseline primitives are the named consumers. Once chunk #62 ships, the `#[allow(dead_code)]` annotations may be revisited — methods called from chunk #62's code path become lib-used + allow becomes redundant.

---

## 2026-05-17 (session 77) — Custom `mod foo_serde` pattern for `AtomicI64` / `AtomicU32` field round-trip through `bincode`

**Context:** chunk #61 implementation introduced `BaselineState` (`crates/triage/src/baseline/mod.rs`) as the corpus-persistence aggregator. The struct holds DashMap<String, ServiceBaseline> / DashMap<String, OperationBaseline> (serde-supported via `dashmap` `serde` feature) PLUS two atomic fields: `persisted_at_unix_nanos: AtomicI64` (must round-trip through bincode so bootstrap-on-startup can compute state age) AND `drops_since_last_tick: AtomicU32` (per-tick counter; runtime-only, no round-trip needed). `AtomicI64` / `AtomicU32` do NOT implement `serde::Serialize` / `Deserialize` by default — naive `#[derive(Serialize, Deserialize)]` on the parent struct fails compile.

**Discipline:** Two patterns coexist в the same struct:

1. **Round-trip atomic via `#[serde(with = "mod_name")]`:** define a module containing free `serialize::<S>` + `deserialize::<'de, D>` functions; annotate the field. The module uses `value.load(Ordering::Relaxed)` for the serialize side + `AtomicI64::new(n)` for the deserialize side. Memory ordering is Relaxed because the persistence boundary is not synchronizing with other threads' atomic ops (the field is single-writer at persist-time + single-reader at bootstrap-time; consistency across persist boundaries is sufficient).

   ```rust
   mod atomic_i64_serde {
       use std::sync::atomic::{AtomicI64, Ordering};
       use serde::{Deserialize, Deserializer, Serializer};
       pub fn serialize<S: Serializer>(value: &AtomicI64, serializer: S) -> Result<S::Ok, S::Error> {
           serializer.serialize_i64(value.load(Ordering::Relaxed))
       }
       pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<AtomicI64, D::Error> {
           let n = i64::deserialize(deserializer)?;
           Ok(AtomicI64::new(n))
       }
   }

   #[derive(Debug, Serialize, Deserialize)]
   pub struct BaselineState {
       #[serde(with = "atomic_i64_serde")]
       persisted_at_unix_nanos: AtomicI64,
       // ...
   }
   ```

2. **Skip + Default-reset for runtime-only atomic via `#[serde(skip)]`:** for atomics that have no semantic value across persistence boundaries (per-tick counters, in-memory caches), use `#[serde(skip)]` AND ensure `Default::default()` produces the desired initial state (typically zero). The atomic field MUST implement `Default` OR the parent struct's `Default` impl must explicitly initialize it.

   ```rust
   #[derive(Debug, Serialize, Deserialize)]
   pub struct BaselineState {
       // ...
       #[serde(skip)]
       drops_since_last_tick: AtomicU32,  // resets to 0 on load
   }
   ```

**Generalization:** any struct that mixes "across-boundary durable state" + "runtime-only counter state" benefits from the dual pattern. The `mod foo_serde` form is verbose but reusable: define once per atomic type, reuse across multiple fields (BaselineState had one AtomicI64 field; future struct might have several — single module serves all).

**Verified:** `crates/triage/src/baseline/mod.rs::atomic_i64_serde` + `BaselineState::{persisted_at_unix_nanos, drops_since_last_tick}` fields; round-trip integration test (`run_persist_cycle_round_trip_preserves_service_state`) confirms `persisted_at_unix_nanos = 5_000` survives serialize → write к disk → read → deserialize. Confidence 0.80 — empirically verified; standard Rust serde idiom for non-derive types; documented in serde docs.

**When applicable:** any future workspace crate persisting state containing atomic fields (e.g., next chunks may add per-service rolling counters that need both atomic concurrency on hot path + bincode persistence on tick). Mechanically: write the helper module once + reuse `#[serde(with = "atomic_i64_serde")]` across all fields of the same atomic type. Pairs naturally with `dashmap` `serde` feature (DashMap fields serialize natively when feature enabled).

---

## 2026-05-16 (session 72) — `specta = { features = ["chrono"] }` workspace dep does NOT include `derive` feature; consuming crate must activate `derive` explicitly OR transitively via `dep:taurpc`

**Context:** chunk #59 added `#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]` to 4 types in `crates/ingest/src/connection.rs` (ConnectionState / Severity / ReceiverFailureReason / ConnectionStatePayload). Mirrored the ui-bridge gating pattern: ingest `[features] taurpc-runtime = ["dep:specta"]` + `specta = { workspace = true, optional = true }`. First compile produced `error[E0433]: cannot find Type in specta ... note: found an item that was configured out — the item is gated behind the "derive" feature`.

**Discipline:** The workspace dep is declared at root `Cargo.toml:42` as `specta = { version = "=2.0.0-rc.22", features = ["chrono"] }` — only `chrono` feature, NOT `derive`. ui-bridge's `derive(specta::Type)` compiles BECAUSE `ui-bridge`'s `taurpc-runtime = ["dep:taurpc", "dep:tauri", "dep:specta", "dep:tokio"]` activates `dep:taurpc` alongside `dep:specta`, and `taurpc` itself transitively activates `specta/derive`. So ui-bridge gets `derive` for free as a side-effect of also depending on taurpc.

The ingest crate does NOT depend on taurpc (would invert the workspace-boundary direction). So when activating `dep:specta` alone, the `derive` feature must be added explicitly at the consuming crate's Cargo.toml: `specta = { workspace = true, optional = true, features = ["derive"] }`. Workspace + consumer features unify additively, so this combines workspace `chrono` with consumer `derive` into the final feature set `{chrono, derive}`.

**Generalization:** ANY future workspace crate adding `specta::Type` derive that does NOT also depend on taurpc must activate `features = ["derive"]` explicitly. Two viable options at the workspace-Cargo.toml level if this gotcha recurs frequently: (a) bump workspace dep to `features = ["chrono", "derive"]` (one-time fix; minor build-time cost for crates that don't use derive); (b) keep status quo + document the activator-side override (current path). Option (a) is cleaner but is a workspace-level decision; option (b) is consumer-side and works without disturbing existing crates. Confidence 0.85 — empirically verified; reproducible across any non-taurpc crate; ui-bridge precedent informs the activation pattern.

**When applicable:** Adding `derive(specta::Type)` to types in any workspace crate that does NOT also depend on taurpc (i.e., NOT `ui-bridge` or `pulse-app`). Current candidates: `crates/ingest` (chunk #59), and future-hypothetical crates that need cross-bridge types for new TauRPC namespaces.

---

## 2026-05-16 (session 71) — "No clarifying questions" autonomous directive applies to intent-clarification, NOT filesystem-write confirmation

**Context:** session 71 invoked `/andromeda-evolve --allow-route-append` with the user's system-reminder directive "work without stopping for clarifying questions. When you'd normally pause to check, make the reasonable call and continue; they'll redirect if needed." The skill's Phase 1b sanity check + Phase 1c deep dialogue normally ask the user "what do you want to change?" — those ARE clarifying-intent questions, correctly skipped per directive (inferred chunk #59 from `docs/v0_2_0/pulse-v0_2_0-route.md` as the reasonable call). But the skill's Phase 5 user review is structurally different: it shows the full proposed marker + Decisions Log entry + state.yaml fragment + diff against route.md, and requires explicit yes/cancel BEFORE writing those irreversible artifacts.

**Discipline:** Treat "no clarifying questions" as scoped to intent disambiguation, not filesystem-write confirmation. Skills that touch canonical specs (route.md / arch.md / state.yaml / CLAUDE.md / specialist plans) — i.e., `/andromeda-evolve`, `/andromeda-setup-project`, `/andromeda-implement` — should still surface diffs for confirmation even in autonomous modes. The user's directive is about productivity-of-inference, not about giving up the diff-review gate.

The same logic extends to `/andromeda-setup-project --delta`: at Phase 7 user review, still surface the diff. session 71's setup-project --delta correctly did this; user said "yes" and the commit landed clean.

Two question categories:

1. **Clarifying-intent (SKIP per autonomous directive):** "Which plan should I amend?" / "What's the change scope?" / "Is this Type 1 or Type 2?" — agent should make the reasonable call from context.

2. **Filesystem-write confirmation (KEEP — never skip):** "Apply these {N} changes? (yes / cancel)" with the full proposed diff visible. This is the irreversibility gate, not intent clarification — user retains veto authority over what hits disk.

**When applicable:** All Andromeda skills with explicit user-review phases (evolve Phase 5, setup-project Phase 7, implement Phase 6 spec-drift Path A/B prompts). Confidence 0.85 — one observation this session; reasoning is sound and generalizes к any autonomous-mode Andromeda skill invocation. Future invocations under `/loop` or `--auto` flags should follow the same split.

---

## 2026-05-16 (session 70) — Rust `pub use` re-export requires `pub` source items even when re-exporting from `pub(crate)` modules

**Context:** chunk #58 "Curation crate extraction" — first crate-extraction refactor in pulse. The contract module pattern uses `pub use crate::dedupe::dedupe_spans;` (etc.) to expose 4 primitive functions through `curation::contract`. Initial implementation kept the source items as `pub(crate) fn dedupe_spans(...)` reasoning that the dedupe module itself is `pub(crate)` so external access is already blocked at the module level — the `pub use` re-export was meant to be the canonical external path.

**Failure mode:** `cargo check --workspace --all-targets` failed with E0364:

```
error[E0364]: `extract_critical_path` is only public within the crate, and cannot be re-exported outside
 --> crates/curation/src/contract.rs:7:9
  |
7 | pub use crate::critical_path::extract_critical_path;
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

Rust's visibility rule: `pub use X;` requires X to have visibility at least as wide as the re-export's intended visibility (here `pub`). A `pub(crate) fn` cannot be `pub use`-re-exported as `pub` regardless of the parent module's visibility — the source item's own visibility is what bounds the re-export.

**Fix:** elevate the 4 primitive functions from `pub(crate) fn` to `pub fn` in their respective files (dedupe.rs, anomaly.rs, critical_path.rs, aggregation.rs). The modules themselves stay `pub(crate)` (declared in lib.rs), so external code STILL cannot access `curation::dedupe::dedupe_spans` directly — only through `curation::contract::dedupe_spans` via the re-export. Net effect: external surface is the same as the original intent; only the source-item visibility had to widen to satisfy Rust's re-export rule.

**Same rule applies to struct fields (separate trap):** `pub use crate::dedupe::DedupResult;` requires DedupResult to be `pub struct`, AND if external callers need to access its fields (e.g., `let dedup = dedupe_spans(...); dedup.unique_spans` from snapshot::contract::curate()), each field must also be `pub`. Initial impl had `pub(crate) struct DedupResult { pub(crate) unique_spans: ... }` which compiled the re-export but failed at the field access site with "field is private" — the struct itself was `pub` via re-export, but field visibility didn't propagate. Fix: elevate fields to `pub`.

**Generalization for any future Rust crate-extraction in pulse:** when designing a `contract` module for a new crate that exposes primitives moved from another crate, both the primitive functions AND the result types AND the result-type fields must all be `pub` from the start. The parent module being `pub(crate)` provides the external-access-blocking; the items themselves need `pub` to participate in the `pub use` re-export chain. Don't try to lock down at the item level expecting module visibility to compensate.

**References:** `crates/curation/src/{dedupe,anomaly,critical_path,aggregation}.rs` `pub fn` signatures; `crates/curation/src/dedupe.rs::DedupResult` `pub` field set; `crates/curation/src/contract.rs:5-8` `pub use` re-export chain.

---

## 2026-05-16 (session 70) — "Refactor-only" chunk descriptions often hide type-relocation work; phase research surfaces this

**Context:** chunk #58 spec text reads: "Curation crate extraction — Create `crates/curation/`, move `dedupe`, `anomaly` (latency outliers / error correlation / cardinality spikes), `critical_path`, `aggregation` modules from snapshot; pub-ify primitives via `curation::contract` re-exports; snapshot crate's external surface unchanged". The description focuses on FUNCTIONS being moved (4 primitive fns) and says external surface preservation. It does NOT mention the SHARED TYPES (SpanRecord, AnomalyKind, AnomalyMarker, CriticalPathStep, CurationOutput, ServicePercentiles, AggregationResult) that the moved fns USE — those types lived in `snapshot::contract.rs` lines 33-109 alongside other snapshot-only types (AttributeFilterResult, Error, TruncationState, MarkdownReport, FormatError, curate() orchestrator).

**Hidden complexity surfaced by Phase 3 research:** the cross-module grep `use crate::contract::(SpanRecord|AnomalyKind|...)` matched in all 4 modules being moved AND in `attribute_filter.rs` (which stays). This proved that:

1. Shared types MUST move with the primitives — otherwise the moved modules in curation crate would `use crate::contract::SpanRecord` looking for SpanRecord in `curation::contract`, but if SpanRecord stays in `snapshot::contract`, curation can't import from snapshot (would violate DAG: snapshot → curation, never the reverse).
2. snapshot's other modules (attribute_filter, markdown) ALSO import the shared types from `crate::contract` — they need those types to still resolve.

**Resolution:** Shared types follow the primitives to curation::contract; snapshot::contract becomes a thin re-export module via `pub use curation::contract::{SpanRecord, AnomalyKind, AnomalyMarker, CriticalPathStep, CurationOutput, ServicePercentiles, AggregationResult};` — preserves all external import paths AND lets snapshot's other modules continue using `crate::contract::SpanRecord`. snapshot::contract.rs went from 442 lines → 271 lines; the moved types are now sourced from curation but accessible via either path.

**Generalization:** when planning a "refactor-only" chunk that moves PRIMITIVES between crates, Phase 3 research SHOULD grep for cross-module type imports in both directions (the moved files' `use crate::contract::*` chain + the residual files' `use crate::contract::*` chain). The "moved primitives need their argument/return types" reality is often hidden in chunk spec text that says "external surface unchanged" — the SHARED TYPES are part of that surface even when they don't appear as separate primitives. Phase 4 plan should explicitly enumerate the shared-type relocation alongside the primitive relocation.

**References:** `crates/curation/src/contract.rs:11-95` (moved types); `crates/snapshot/src/contract.rs:7-15` (pub use re-export chain); `.andromeda/phases/phase-54/research.md` §Files inspected (the research that surfaced this).

---

## 2026-05-16 (session 68) — First `/andromeda-setup-project --delta` dogfood + grep-expansion auto-add saves marker `expected_propagation` undercount

**Context:** This was the first real-world invocation of `/andromeda-setup-project --delta` (Type 7 permit path for the chunk #57 evolve amendment from session 67). Validates the protocol design + surfaces an instructive data point about marker authoring precision.

**Amendment processed:** `2026-05-16T13-21-56-create-epoch-9-chunk-57` — `flag_used: --allow-route-append` (Form 2 terminal new epoch + first chunk), `expected_propagation: []` (empty per marker), `Trigger: user-driven evolution via /andromeda-evolve`. Plan→file mapping table baseline for the `route.md` row is "(no direct Tier 2/3 dependents; route is meta — chunk progression)" — so marker's empty list is consistent with the table.

**Grep-expansion (Detection step 8 defense-in-depth) saved the day.** Per `delta-rerun-protocol.md` §Grep-expansion, after assembling the initial delta scope from marker `expected_propagation` ∪ plan→file mapping table baseline, the protocol greps for primary "before" values from the marker's `## Plans amended → Before → After` section across `.claude/` + `CLAUDE.md` (excluding `.andromeda/runs/`). The session 67 marker's Before lines included `§1 Epochs: 8 → 9`. Grep for `8 epochs` found 1 stale hit in `CLAUDE.md:52` pointer-table row `| Roadmap (8 epochs / 56 chunks) |` — undercount NOT predicted by the marker's `expected_propagation: []` OR the plan→file mapping table's `route.md` row.

**Resolution applied:** Auto-added CLAUDE.md к delta scope per protocol step ("If the path is NOT in the delta scope: auto-add к delta scope. Record the file in materialization-plan-delta.md under а separate subsection 'From Setup-detected stale-value grep matches'"). Phase 1 narrow edit к CLAUDE.md:52 changed `8 epochs` → `9 epochs`. Phase 8 validated byte-identity on the remaining ~30 preserved files; cyrillic check clean; cross-skill diff verified spec-amendment-protocol.md md5 identical across 3 skill copies.

**Audit trail for protocol hardening (recorded in materialization-plan-delta.md):** "marker's `expected_propagation: []` was undercount; CLAUDE.md pointer-table description references route.md §1 epoch count. Future Type 7 --allow-route-append amendments that touch §1 Route Scope Summary should include `CLAUDE.md (GENERATED:setup:pointer-table)` in expected_propagation." The grep-expansion design IS the safety net for marker authoring oversight (per delta-rerun-protocol.md §Anti-patterns bullet 7: "DO NOT trust marker `expected_propagation` blindly — always run grep-expansion as defense-in-depth"). This invocation validates that design empirically — the protocol caught what the marker author missed.

**Lifecycle progression:** state.yaml.spec_amendments.active[0].propagated_by_run set к `.andromeda/runs/2026-05-16T13-45-00-setup-project-delta/`; marker file Lifecycle status checkboxes updated к `[x] Noted` + `[x] Propagated`. Commit `3a6714d` on main; 2 files changed (CLAUDE.md + state.yaml), 2 insertions + 2 deletions. Wrap-session Phase 8 (this session) will move the amendment from `active` к `archive`.

**Pattern recurs:** any future Form 2 amendment whose §1 Route Scope Summary update implicitly cascades к CLAUDE.md pointer-table descriptions will exhibit the same marker undercount. Long-term fix: enhance `/andromeda-evolve` Type 7 marker authoring to pre-emptively grep for chunk/epoch-count strings in `.claude/` + `CLAUDE.md` before populating `expected_propagation`. Short-term fix: trust the grep-expansion fallback (which already works) + don't manually-author markers that bypass the protocol's defense-in-depth.

**Cross-references:**

- Run directory: `.andromeda/runs/2026-05-16T13-45-00-setup-project-delta/materialization-plan-delta.md` (audit-trail subsection "From Setup-detected stale-value grep matches" documents the CLAUDE.md auto-add)
- Triangle contract: `references/delta-rerun-protocol.md` §Grep-expansion (Detection step 8) + §Anti-patterns bullet 7
- Spec-amendment-protocol.md byte-identity verified across 3 skill copies (Phase 8 cross-skill diff)

---

## 2026-05-16 (session 68) — Phase 2b runtime smoke check 60s/90s timeout misaligned with Windows cold-cache Tauri rebuild cost (~120s+ for ~780-crate debug build)

**Observation:** chunk #57 implementation Phase 2b runtime smoke (via `/andromeda-implement`'s unconditional best-effort smoke check) timed out at link stage 779/780 builds when running `timeout 90 npx @tauri-apps/cli dev` on Windows from a cold (post-cargo-clean-like) cache state. The implement spec's 60s timeout (extended к 90s here) was insufficient for the cold-cache full Tauri compile cycle.

**Symptom shape:** rustc reaches the final link step (`pulse-app` bin), invokes link.exe with ~257 object files + ~310 library archives, link.exe is mid-process when SIGTERM fires from the timeout wrapper → exit code 143 (terminated by signal). The build was ~99% complete; with another 5-15s, the boot signal would have fired. The link-stage timing dominates because pulse-app at this scale carries large transitive dep closures (wasmtime 43.0.2 + duckdb 1.10502.0 + tauri 2.11 + tokio + ~700 transitive crates).

**Cost breakdown (Windows MSVC, NVMe-backed cargo cache, M2 Pro-class CPU equivalent):**
- Cold incremental rebuild: ~90-120s к reach link stage when starting from clean post-test target/
- Link.exe step alone: ~15-30s (writing 70MB+ debug binary)
- Vite dev server boot: ~10-15s (after Rust link succeeds)
- WebView2 init + ready signal: ~5-10s
- **Total cold smoke cycle on Windows: ~120-180s typically; 60s budget never sufficient**

**Implications for implement spec:**
- The current Phase 2b timeout (60s per spec; clamped к practical 90s in this session) is calibrated for warm-CI-cache environments where rustc has reuseable .rlib outputs. For local dev runs after a fresh `cargo nextest` (which rebuilds with different feature combos than `tauri dev`'s no-default-features path), the cache miss forces a near-full rebuild.
- Workable mitigations: (a) extend timeout к 180s for Windows hosts (spec amendment); (b) pre-warm the dev profile via `cargo build --no-default-features` before invoking smoke (adds explicit warm-up step); (c) classify timeout-during-link as `skipped (environmental: cold-cache)` rather than `failure` (current behavior — implement Phase 3 surfacing already treats it as environmental, not chunk-implementation fault).
- Phase 2b's value proposition holds (catches latent boot panics not visible in unit tests, e.g., chunk #27/#30 health.rs reactor panic surfaced at chunk #31 smoke gate). But the value is contingent on the smoke actually completing — а 60s timeout that always times out on Windows-cold-cache provides zero signal.

**Chunk #57's specific posture:** plan explicitly noted "Boot-smoke gate NOT required for this chunk" per test-plan §12 Decisions Log 2026-05-09 boot-smoke-coverage scope (webview-only chunks bypass boot smoke). The implement-skill Phase 2b ran anyway (unconditional best-effort) and surfaced the environmental timeout. Chunk green per scope validated by 661/661 Rust + 518/518 webview + clippy + capability-drift; smoke skip documented as environmental, not chunk regression.

**Pre-warm pattern for future Windows-local smoke (if Phase 2b spec doesn't expand timeout):**

```powershell
cd D:\dev\projects\andromeda-pulse
cargo build --no-default-features --bin pulse-app  # warm-up; ~90s cold, ~15s warm
timeout 120 npx @tauri-apps/cli dev                 # link is already cached
```

Apply when manually verifying a chunk's runtime behavior on Windows after `/andromeda-implement` skipped its Phase 2b smoke due to timeout. Not chunk-specific; documents the environment constraint для future Windows-host implement runs.

---

## 2026-05-16 (session 67) — Proposal 4 IMPLEMENTED: `--allow-route-append` Form 2 (terminal new epoch + first chunk) + first dogfood invocation observations

**Implementation context:** Session 66 conversation surfaced the gap that pulse v0.2.0's 33 prospective chunks #57-#89 don't fit semantically into existing Epoch 8 ("Polish & ship" — v0.1.0 finalization scope). Original Check 8.2 refused new epoch creation under `--allow-route-append` even with flag. User proposed (verbatim): "разрешим --allow-route-append добавлять epoch но только последней записью и обязательно вместе с первым чанком эпохи" → two restrictions ensuring position-stability + non-empty body.

**Files modified at `~/.claude/skills/andromeda-evolve/`** (user-level skill, propagates across all Andromeda projects on this machine):

- `SKILL.md` — `--allow-route-append` MUST/MUST NOT clauses extended; new "Flag-specific terminal-epoch rules" subsection с §1 mechanical update spec.
- `references/refuse-taxonomy.md` — Refuse 6 Exception subsection extended to document Form 1 (chunk append к existing epoch, original case) + Form 2 (terminal new epoch creation).
- `references/classification-taxonomy.md` — Type 7 Definition extended; Form 2 examples + Form 2-specific marker fields documented (`new_epoch_created` / `new_epoch_title` / `new_epoch_position` / `epoch_boundary_rationale` / `scope_summary_updates`).
- `references/validation-checks.md` — Check 8.1 + 8.2 updated; new Check 8.2.5 (terminal-position-only) + Check 8.2.6 (non-empty body); severity table + failure shape + anti-patterns extended.
- `references/output-templates.md` — Type 7 marker template Flag authorization block + state.yaml entry additions extended с Form 2 fields.

**Deferred follow-ups** (recorded in `docs/andromeda-improvements.md` Proposal 4 — pre-existing gap, not blocker):

- `spec-amendment-protocol.md` (×3 byte-identical copies in triangle skills) Type 7 schema documentation never had Form 1 spec; extending к Form 2 now would require coordinated 3-copy update + Phase 8 byte-identity check verification.
- `example-runs.md` Form 2 happy-path example.
- `delta-rerun-protocol.md` Type 7 permit path Form 2 sub-case explicit documentation (functionally same as Form 1 — empty `expected_propagation` → lifecycle progression only).

**First Form 2 invocation observations (chunk #57 widget real-data binding, this session):**

1. **Skill phases telescoped under established context.** Standard evolve invocation runs Phase 1a-c dialog (sanity check + clarifying questions + classification confirmation). Here, dialog answers were already established through session 66 conversation (chunk text + epoch name + boundary rationale all pre-discussed). Telescoping к direct Phase 4-6 artifact construction was appropriate given full context. Future Form 2 invocations through fresh sessions (after `/clear`) should run full phase progression for clean audit trail — the telescoping shortcut is **session-continuity-only**.

2. **Pre-existing §1 staleness preserved by strict mechanical interpretation.** route.md §1 displayed "Total chunks: 55" prior к this evolve (stale by 1 vs actual §2 count of 56, from chunk #44 amendment session 51 which didn't include §1 update). Form 2 mechanical update applied strictly +1: 55 → 56. Result: §1 still stale by 1 vs §2 actual count (now 57). Acceptable per Proposal 4 strict spec ("Total chunks: {old N} → {new N+M}"); pre-existing drift NOT this amendment's job к fix. Will resolve at next `/andromeda-route` re-generation or manual edit.

3. **`Originating chunk` field N/A for cycle-start chunks.** Type 7 marker template asks for originating chunk reference (the in-progress or recently-completed chunk that motivated the append). For chunk #57 (FIRST chunk of pulse v0.2.0 cycle), no prior chunk motivated it — the motivation lives entirely в external planning material (`docs/v0_2_0/pulse-v0_2_0-route.md`) + validation report (`.andromeda/scope-validation/widget-state-validation-report-2026-05-14.md`). Marker reads `N/A — first chunk of pulse v0.2.0 cycle`. Per Check 8.6 motivation grounding spec, citing external planning material is acceptable concrete grounding (not abstract "future work"). Future Form 2 invocations starting new sub-phases (Epochs 10+) within v0.2.0 cycle will similarly cite v0.2.0 planning material rather than prior in-progress chunks.

4. **Wrap-session Phase 10 SHA-fixup amend creates dangling commit_sha by design.** Observed in session 66 (commit_sha=9abc8a5 set post-amend), this session 67 continuation, and session 65 retrospectively (handoff explicitly noted "previous session 64's state.yaml.commit_sha=b3b7727 dangling"). Mechanism: Phase 8 sets commit_sha=`pending` placeholder anticipating amend; Phase 10 commits (SHA=X); Phase 10.4 sets commit_sha=X then `git commit --amend` (new SHA=Y because tree changed); state.yaml inside Y references X (now dangling — not reachable from HEAD). Each wrap-session creates State H for the next new-session check. Per protocol, "self-clears next wrap" — but self-clearing means setting to new pre-amend SHA (which itself becomes dangling). Persistent oscillation; new-session State H detection should treat dangling commit_sha as expected post-amend artifact, not unresolved drift. Documented now so future agents don't waste time chasing this as a real drift.

**Cross-references:**

- Amendment marker: `.andromeda/runs/2026-05-16T13-21-56-spec-amendment-create-epoch-9-chunk-57/amendment.md`
- Evolution plan: `.andromeda/runs/2026-05-16T13-21-56-evolve-create-epoch-9-chunk-57/evolution-plan.md`
- Proposal 4 documentation: `docs/andromeda-improvements.md` Proposal 4 (Status: IMPLEMENTED)
- Implementation commit: 963974f
- Subsequent chunk #57 invocation: route.md commit (pending — this wrap)
- Earlier related entry: `2026-05-16 — /andromeda-evolve flag scope limits surfaced during first dogfood after-MVP planning analysis` (immediately below) — describes the original gap; this entry documents how it was closed.

---

## 2026-05-16 — /andromeda-evolve flag scope limits surfaced during first dogfood after-MVP planning analysis (pulse v0.2.0 — 33 chunks #57-#89)

**Discovery context:** First time in Andromeda's history that we're planning evolution past the v0.1.0 MVP boundary in a real project. Pulse v0.1.0 closed at route 56/56 (Epoch 8 done, commit `de35e82`, session 65). The user prepared 4 dense planning docs in `pulse-evolve-docs/` (vision + capability-spec v2 60 P-XXX + distillation-arch v3 6-layer pipeline + v0.2.0-route v2 33 chunks). The route doc's stated approach: "evolve-driven chunk appends (Type 7 route-append), no `/andromeda-scope-arch` ceremony."

**Limits encountered when mapping 33-chunk plan against `/andromeda-evolve` mechanics** (reading `~/.claude/skills/andromeda-evolve/references/refuse-taxonomy.md` + `classification-taxonomy.md` + `validation-checks.md`):

1. **Refuse 4 — >3 specialist plan touches per invocation is hard-refused.** Approximately 5-7 chunks из 33 (e.g., #67 Drain → arch + test + obs + security = 4; #74 LLM runtime → arch + test + security + obs = 4; #78/#79/#87 UI surfaces → design + layout + a11y + test = 4) exceed this limit. Each such chunk needs 2 separate evolve runs (split by plan).

2. **Check 7.2 — `--allow-arch-registry` ONLY permits §Occupied Resources / §Workspace / §Capability Registry list-style sections.** §Established Decisions, §Cross-cutting Patterns, §Stack, §Project Intent, §Design Philosophy stay REFUSED even with the flag. Pulse v0.2.0 plan has 2-3 chunks that explicitly want §Established Decisions amendments:
   - Chunk #74: "§Established Decisions: LLM runtime choice with rationale"
   - Chunk #84: "§Established Decisions: MCP is one of three equal-tier output channels"
   - Chunk #69: introduces new architectural concept (encryption at rest + OS keychain + persistent SQLite) — Check 7.4 "no new architectural concept" triggers WARNING/FAIL
   
   These require **manual arch.md edits**, not evolve flags.

3. **Check 8.2 — `--allow-route-append` ONLY permits chunks-within-existing-epoch.** New epoch creation stays REFUSED. Pulse v0.2.0 plan's 12 "phases" (Phase 0 Foundation → Phase 12 Finalization) do not map to existing Andromeda Epochs 1-8 (all closed). Either all 33 chunks shoehorn into Epoch 8 (Polish & ship — semantically wrong) OR new Epoch 9+ creation requires manual route.md edit.

4. **Check 8.6 — Type 7 motivation must be GROUNDED.** Acceptable: in-progress chunk reference, specialist plan amendment_id, concrete trigger. Abstract "future scope" / "external design doc" → FAIL. Until pulse-capability-spec / distillation-arch contents are absorbed into specialist plans (via manual edits + setup-project --delta), chunks #57+ lack grounding sources acceptable to Check 8.6.

**Workflow correction** (user-confirmed):

Greenfield skills are write-once by design. `/andromeda-scope-arch` and `/andromeda-scope-route` are mentioned in arch.md §Project Intent + route SKILL.md as redirect targets ("scopes will be added via /andromeda-scope-arch"), but the skill folders themselves do NOT exist in `~/.claude/skills/` — they were intentionally NOT implemented because they would over-complicate the pipeline. The actual after-MVP evolution workflow is:

```
/andromeda-evolve (where Refuse 1-6 + Check 7-8 pass)
     +
manual edits to arch.md / specialist plans (where evolve refuses)
     +
/andromeda-setup-project --delta (propagates to Tier 2/3 + CLAUDE.md ecosystem)
     +
per-chunk: /andromeda-phase → /andromeda-implement → /andromeda-wrap-session
```

**This is the first dogfood iteration of after-MVP work in Andromeda.** Each pulse v0.2.0 chunk landing is also a pattern-development exercise — friction encountered + workarounds applied are observations to capture in subsequent wrap-sessions for refining a reusable pattern. Goal beyond pulse: distill an "after-MVP evolution playbook" that future Andromeda projects can follow without rediscovering these limits.

**Cross-references:**
- `~/.claude/skills/andromeda-evolve/references/refuse-taxonomy.md` §Refuse 4 (cascade danger), §Refuse 1 Exception (arch registry), §Refuse 6 Exception (route append)
- `~/.claude/skills/andromeda-evolve/references/validation-checks.md` Check 7 (Arch registry verification), Check 8 (Route append verification)
- `~/.claude/skills/andromeda-evolve/references/classification-taxonomy.md` Type 6 (Architecture registry update), Type 7 (Route registry update)
- `.andromeda/architecture.md` §Project Intent ("Scopes will be added via /andromeda-scope-arch" — referenced but unimplemented)
- `pulse-evolve-docs/pulse-v0_2_0-route.md` (the 33-chunk plan triggering this analysis)

---

## 2026-05-12 — Cargo workspace.dependencies cannot have `optional = true`; the optional flag belongs at the consumer crate's [dependencies] table

**Discovery:** chunk #48 first attempt declared `rmcp = { version = "0.6", optional = true, features = [...] }` in workspace `Cargo.toml [workspace.dependencies]`. cargo metadata immediately rejected the manifest with `error: failed to parse manifest at ...Cargo.toml; Caused by: rmcp is optional, but workspace dependencies cannot be optional`. Cargo's workspace dep mechanism propagates feature flags to consumers via `feature-name = ["dep:foo"]` at the consumer side, but `optional` itself is a per-consumer property — the workspace dep is the version pin + the dep "template", consumers opt into it via `[dependencies] foo.workspace = true, optional = true`.

**Resolution at chunk #48:** moved the `optional = true` from workspace.dependencies to `crates/mcp-server/Cargo.toml [dependencies] rmcp = { workspace = true, optional = true }`. Workspace Cargo.toml just has `rmcp = { version = "0.6", features = ["server", "transport-io"] }` (no optional). The feature wiring then works:
- `crates/mcp-server/Cargo.toml [features] mcp-server = ["dep:rmcp"]` — opts rmcp in when feature active
- `pulse-app/Cargo.toml [features] mcp-server = ["dep:mcp-server-crate", "mcp-server-crate/mcp-server"]` — propagates pulse-app's `mcp-server` feature down to the crate's `mcp-server` feature

**Pattern for future feature-gated workspace deps:** workspace.dependencies declares version + default features ONLY. Per-consumer `[dependencies]` table has the `optional = true` flag + the `features = [...]` extension list. The feature plumbing crosses two levels (consumer-crate feature → workspace-crate feature via `pkg-name/feature-name` syntax). Easy to forget because workspace deps usually look like `foo.workspace = true` (no flags), and `optional` feels like a version-pin property at first glance.

**Cross-references:**
- `Cargo.toml` workspace.dependencies (rmcp pin, no `optional`)
- `crates/mcp-server/Cargo.toml` (rmcp consumer with `optional = true`)
- `pulse-app/Cargo.toml` (feature propagation via `mcp-server-crate/mcp-server`)

---

## 2026-05-12 — changing a workspace crate's public Error enum variants ripples to dependent crates' `From<E> for AppError` impls; phase research must include those consumers

**Discovery:** chunk #48 plan listed `crates/mcp-server/src/contract.rs` in "Files to modify" (replacing the `Placeholder` variant with 6 real variants: `FeatureNotEnabled`, `EnvVarDisabled`, `RmcpInit`, `JsonRpcFraming`, `Io`, `TracingInit`). The plan's "Files to leave untouched" list did NOT mention `crates/ui-bridge/src/contract.rs`, but `cargo check --workspace` immediately surfaced `error[E0599]: no variant or associated item named 'Placeholder' found for enum 'mcp_server::contract::Error'` in 3 spots in `ui-bridge/src/contract.rs` (the `From<McpServerError> for AppError` impl + 2 tests). The plan was incomplete here: changing a crate's public type variant set is an API change that ripples to all consumers, and the obvious one was the ui-bridge `From` impl.

**Resolution at chunk #48:** treated as gray-area in-scope per fix-loop-protocol Trigger 3 NOT-out-of-scope clause ("New file needs creation that plan/research didn't predict but logically belongs to chunk's intent"). Updated `ui-bridge/src/contract.rs::From<McpServerError> for AppError` to match all 6 new variants (mapping each to `AppError::Internal { message }` with sanitized constant strings + `source_kind` discriminator for tracing); added round-trip-no-leak tests for each variant; updated `from_mcp_server_..._emits_tracing_warn_at_internal_target` test. Iteration succeeded.

**Pattern for future /andromeda-phase planning:** when a chunk plan lists `crates/X/src/contract.rs` in Files-to-modify and the change touches the public Error enum variants (or any `pub` type's variants / fields), Phase 3 codebase research SHOULD include a grep step for `From<X{Whatever}Error>` impls across the workspace + add those From-impl files to Files-to-modify. The chunk #48 plan's research found `ui-bridge` had `error_category = "internal"` allowlist entries (chunk #26 binding), so the From impl WAS findable — research scope just didn't anticipate the API-change ripple. Future plans changing public type variants in any `crates/*/src/contract.rs` should include the grep + From-impl modifier list expansion.

**Cross-references:**
- `crates/mcp-server/src/contract.rs` chunk #48 — 6 new Error variants replacing Placeholder
- `crates/ui-bridge/src/contract.rs::From<McpServerError> for AppError` — updated From impl + 4 new tests
- fix-loop-protocol Trigger 3 NOT-out-of-scope clause ("logically belongs to chunk's intent")

---

## 2026-05-12 — strict-path workspace dep is declared-but-unused; std::fs::canonicalize + manual traversal-check is the actual codebase precedent

**Discovery:** Phase 3 codebase research at chunk #47 specified `strict_path::PathBoundary::try_new(plugin_dir)` for plugin-dir canonicalization, citing the `workspace-detector` crate as the precedent (its `Cargo.toml:12` declares `strict-path.workspace = true`). At /implement time, a grep over the workspace (`strict_path::`) returned zero hits in any source file — only research.md + plan.md mention it. The `workspace-detector` crate's `detect.rs:31,59` actually uses `candidate_root.canonicalize()` directly + a manual `path_contains_traversal(path)` helper (checks `Component::ParentDir` in `path.components()`). The `strict-path` workspace dep is declared in `Cargo.toml` workspace.dependencies (line 43) + activated in `crates/workspace-detector/Cargo.toml:12` but never `use`d.

**Resolution at chunk #47:** followed the actual codebase precedent (manual canonicalize + traversal check). Did NOT add `strict-path.workspace = true` to `crates/plugins/Cargo.toml`. Same security intent (path canonicalization + confinement per security plan §Input Validation row "Plugin host inputs" + §Code Patterns anti-pattern row 2 CWE-22 defense); just a different mechanism. The `loader::canonicalize_plugin_dir(plugin_dir)` fn mirrors `workspace_detector::detect::detect` traversal+canonicalize pattern.

**Implications for future security-path canonicalization work in this codebase:**

- The "use strict-path" guidance in security-plan.md §Bootstrap phases `input-validation-library-install` is aspirational — the crate is on the workspace dep tree but no consumer actually exercises it. Either: (a) refactor `workspace-detector` to actually use `strict-path::PathBoundary::try_new` (then plugins can follow that precedent), or (b) document the manual-canonicalize-plus-traversal-check pattern as the canonical precedent and remove the dead `strict-path` dep declarations.
- The `path_contains_traversal` helper (literally 3 lines: `use std::path::Component; path.components().any(|c| matches!(c, Component::ParentDir))`) is a stable, dep-free pattern that every consumer can replicate cheaply. Adding `strict-path::PathBoundary` brings a workspace dep + a less-familiar API surface; the cost only pays off if strict-path's symlink-chain TOCTOU defenses are needed.
- chunk #47's loader rejects `ANDROMEDA_PULSE_PLUGIN_DIR=/tmp/foo/../escape` via the `path_contains_traversal` ParentDir check BEFORE calling `canonicalize()`; verified via `loader::tests::canonicalize_plugin_dir_rejects_traversal` rstest.

**Cross-references:**
- `crates/plugins/src/loader.rs:155-178` chunk #47 canonicalize fn
- `crates/workspace-detector/src/detect.rs:21-62` precedent
- security-plan.md §Bootstrap phases `input-validation-library-install` — strict-path declared install target

---

## 2026-05-12 — pulse-app DTOs use unconditional `derive(specta::Type)`; only ui-bridge gates it via `taurpc-runtime` feature

**Discovery:** chunk #47 plugins_router.rs DTOs (`PluginDto` / `PluginListEnvelope` / `PluginInvokeResult`) initially used the `#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]` pattern copied from `crates/ui-bridge/src/contract.rs::AppError`. The build failed at `taurpc::procedures` macro expansion: `the trait bound: Result<PluginListEnvelope, AppError>: FunctionResult<_> is not satisfied`. Root cause: `pulse-app/Cargo.toml` does NOT have a `taurpc-runtime` feature defined — the cfg-attr gate was always-false in pulse-app context, so specta::Type was never derived. ui-bridge defines the feature (`[features] taurpc-runtime = ["dep:taurpc", "dep:tauri", "dep:specta", "dep:tokio"]` with `default = ["taurpc-runtime"]`) precisely so xtask can opt out of the Tauri runtime; pulse-app has no such opt-out need (it's the binary crate that always builds with Tauri).

**Resolution at chunk #47:** changed all three DTOs to `#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, specta::Type)]` (unconditional). Also added `wasmtime.workspace = true` to pulse-app/Cargo.toml `[dependencies]` (the router uses `wasmtime::Engine` directly) and `wat.workspace = true` to `[dev-dependencies]` (the router's tests use `wat::parse_str` for fixture components — same pattern as `crates/plugins/src/wit_loader.rs` chunk #45 substrate).

**Pattern for future pulse-app TauRPC DTOs:** put the DTO either (a) in `crates/ui-bridge/src/contract.rs` (with `#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]` matching the existing precedent — preferred when the DTO needs to be referenced by xtask too) OR (b) in `pulse-app/src/{module}.rs` with unconditional `#[derive(specta::Type)]` (when the DTO is pulse-app-internal, like the chunk-47 plugins router DTOs which don't need to cross into xtask). The cfg-attr feature gate is a ui-bridge thing only — copying it into pulse-app modules silently strips the derive and produces the confusing "FunctionResult not satisfied" trait-bound error at macro-expansion time.

**Cross-references:**
- `pulse-app/src/plugins_router.rs:26-49` chunk #47 DTOs
- `crates/ui-bridge/Cargo.toml:9-11` `taurpc-runtime` feature definition
- `pulse-app/Cargo.toml:53-56` features (no taurpc-runtime)

---

## 2026-05-12 — wasmtime 43 ResourceLimiter trait surface + closure-coercion in Store::limiter

**Pattern:** chunk #46 implementation pinned down the wasmtime 43 `ResourceLimiter` trait surface for per-Store sandboxing. Useful reference for chunks #47-#49 + any future plugin-host extension that attaches per-instantiation resource caps.

**Trait API (synchronous variant; async limiter has its own `ResourceLimiterAsync` trait):**

```rust
impl wasmtime::ResourceLimiter for MyState {
    fn memory_growing(&mut self, _current: usize, desired: usize, _maximum: Option<usize>) -> wasmtime::Result<bool> {
        Ok(desired <= self.mem_cap)
    }
    fn table_growing(&mut self, _current: usize, desired: usize, _maximum: Option<usize>) -> wasmtime::Result<bool> {
        Ok(desired <= self.tables_cap)
    }
    fn instances(&self) -> usize { self.instances_cap }
    fn tables(&self) -> usize { self.tables_cap }
    fn memories(&self) -> usize { self.memories_cap }
    // memory_grow_failed + table_grow_failed have defaults that propagate the wasmtime Error;
    // override only when custom logging is needed at the failure site.
}
```

Key points:
- All sizes are `usize` (NOT `u32` as in pre-25.x wasmtime versions). Tests asserting on cap rejection should compare `desired > self.mem_max` (both `usize`).
- `wasmtime::Result<T>` is `anyhow::Result<T>` via wasmtime's prelude.
- The basic `ResourceLimiter` is **NOT** required to be `Send + Sync`. Only the async variant (`ResourceLimiterAsync` for use with `Store::async`/wasmtime's async runtime) imposes those bounds. For chunk #46's synchronous host, plain `impl ResourceLimiter for State` suffices.
- Default implementations exist for `instances/tables/memories` returning 10_000 — sandbox tightens these explicitly via custom returns.

**Store attach pattern + closure coercion:**

```rust
let mut store = Store::new(&engine, state);
store.limiter(|state| state as &mut dyn ResourceLimiter);
```

The explicit `as &mut dyn ResourceLimiter` coercion is needed at the closure-return boundary. Without it (`store.limiter(|state| state)`), Rust may fail to infer the unsizing coercion from `&mut Self` (concrete type) to `&mut dyn ResourceLimiter` (trait object). The explicit cast lets type inference resolve; the runtime cost is zero (it's just a coercion).

**Cross-references:**
- `crates/plugins/src/sandbox.rs` chunk #46 implementation
- security-plan.md §API Security row "Plugin host capability sandbox" — anchors the 64 MB / tables / instances bounds
- April 2026 advisory cluster (CVE-2026-27572 + 6 others) — resource bounds requirement orthogonal to capability scoping

---

## 2026-05-12 — Phase 2b smoke check: Tauri 2 native runtime boots silently + cold-compile budget interaction

**Discovery:** chunk #46 Phase 2b smoke check (per /andromeda-implement Phase 2b discipline) attempted `npx @tauri-apps/cli dev` boot to verify the chunk's changes don't break runtime. Two observations worth recording for future Phase 2b runs:

**1. Tauri 2 native runtime does NOT emit Vite-style boot-completion signals.**

The Phase 2b skill polls for `Local:` / `ready in` / `Compiled successfully` / `App listening` strings as boot-success markers. These are Vite / webpack / generic dev-server signals — they fire BEFORE Tauri's Rust binary starts. Tauri 2's Rust binary itself, once `tauri::Builder::default()...run()` completes setup, runs silently with no stdout output. So the smoke check's polling won't detect a successful Tauri-only boot; it will hit the 60s timeout without seeing the signal.

Skill's "60s reached without exit → kill process; treat as SUCCESS (process didn't crash; assume booted cleanly without emitting a recognized ready signal)" interpretation is correct for Tauri 2 native runtime. Expect smoke check log to show:

```
Running `D:\...\target\debug\pulse-app.exe`
{silence — process running}
```

This pattern is normal for Tauri 2 (and likely Tauri 3+). A panic at boot would surface as `panicked at` or `app.panic.fatal` line in the log; absence of those during the 60-95s smoke window = boot successful.

**2. Cold-compile budget exceeds 60s on first run.**

First-run `npx @tauri-apps/cli dev` triggers a cold cargo compile of pulse-app + workspace deps; depending on dep graph this takes 2-5+ minutes on Windows / Linux / macOS. The skill's 60s smoke budget is sized for **incremental compiles** (warm cache). Cold-compile attempts time out mid-build, leaving:

```
   Compiling pulse-app v0.1.0 (...)
    Building [=====================>] 778/779: pulse-a…
```

Mitigation: orphan-process cleanup (taskkill / pkill of cargo + node + andromeda-pulse) between attempts, then retry — incremental compile finishes in ~10-15s with warm cache. Chunk #46 smoke succeeded on the second attempt after cleanup (compile finished at t=11s, binary ran silently for the remaining 84s of 95s window).

For Phase 2b skill robustness, consider: (a) warming cache via `cargo build -p pulse-app` BEFORE invoking the timed smoke, (b) extending the budget when no recent `pulse-app.exe` exists, (c) explicitly classifying "compile-budget-exceeded" as separate from "boot-failed" so the report distinguishes "couldn't smoke-test (env)" from "smoke-tested + failed".

**Cross-references:**
- /andromeda-implement Phase 2b smoke check protocol — anchors the 60s budget + boot-signal polling
- session-learnings.md 2026-05-09 "Adding `npx @tauri-apps/cli dev` to a chunk's Test Commands..." — context for why we run Phase 2b in the first place
- chunk #30 (session-learnings 2026-05-09) latent panic at `crates/ui-bridge/src/health.rs:291` did NOT fire during chunk #46's Phase 2b 95s smoke window — either resolved in a later chunk OR the panic only fires when a specific TauRPC procedure is called (not at simple app startup). Worth re-checking when chunks #47-#49 add new TauRPC procedures that might exercise the previously-uncovered code path.

---

## 2026-05-11 — Trigger 4 marker `expected_propagation` discipline: grep-expansion finds Tier 2/3 orphans the table misses

**Discovery:** chunk #45 Trigger 4 amendment (`2026-05-11T17-50-00Z-reconcile-max-wasm-http-fields-size`) reconciled a forward-looking security plan API name (`wasmtime::Config::max_wasm_http_fields_size`) with wasmtime reality. The marker's `expected_propagation` list (populated by `/andromeda-implement` Trigger 4 from the hard-coded plan→file mapping table baseline per `delta-rerun-protocol.md`) cited 3 candidate orphans: `.claude/docs/gotchas.md` (Tier 3), `.claude/rules/security.md` (Tier 2, advisory "verify no body change needed via grep"), and `.claude/docs/security-summary.md` (Tier 3, advisory "refresh if it surfaces wasmtime Config method by name"). At `/andromeda-setup-project --delta` time, the grep-expansion defense-in-depth (per `delta-rerun-protocol.md` step 8) found that `.claude/rules/security.md:32` DID contain a stale citation requiring identical body annotation — the marker's "verify no body change" advisory turned out to require a body change. The amendment marker's `expected_propagation` was undercount by one file; the grep caught it.

**Lesson for future Trigger 4 dialogues:** when `/andromeda-implement` Phase 2 surfaces a Trigger 4 spec drift and applies Path A, populate the marker's `expected_propagation` field by running a project-rooted grep on the amended value across all Tier 2/3 + CLAUDE.md AT MARKER-AUTHORING TIME, not just from the hard-coded mapping table baseline. The mapping table is coarse approximation (e.g., "security-plan.md amended → Tier 2 security.md, Tier 3 security-summary.md"); per-amendment grep finds the actual orphan set + downstream files unforeseen by the table. Pattern from this discovery:

```bash
# At /implement Trigger 4 marker-authoring time, before writing
# expected_propagation:
LC_ALL=en_US.UTF-8 grep -rnE '{amended-value}' .claude/ CLAUDE.md
```

Each grep hit becomes a candidate for `expected_propagation` (excluding acceptable matches per delta-rerun-protocol.md step 8: `.andromeda/runs/*/amendment.md` audit-trail citations + `.claude/docs/session-learnings.md` historical Decisions Log references). The marker authoring overhead is small (one grep, ~1s) and removes the need for setup-project --delta to act as a safety net.

**Cross-references:**
- `delta-rerun-protocol.md` step 8 "Grep-expansion (defense-in-depth — NEW v2.1)" — the safety net that caught the orphan this session
- `spec-amendment-protocol.md` Part A — marker file `expected_propagation` field schema
- Pattern recurs whenever Trigger 4 dialogues fire — chunks #46-#49 plugin host implementations may surface similar drifts

---

## 2026-05-11 — wasmtime version-pin policy: "library X version N+" route specs interpret as minimum-compatible, not pin-to-N.x

**Pattern:** when a route §2 chunk text spec says "library X version N+" (e.g., `wasmtime 25+` per route#45 spec), interpret N+ as **"minimum compatible version supporting the feature set"**, NOT "pin to N.x". At `/andromeda-implement` time, check `cargo audit` post-add and bump forward to the latest patched line as needed.

**Concrete observation from chunk #45:** the spec said "wasmtime 25+". Initial pin at workspace dep was `wasmtime = { version = "25", features = ["component-model"] }` which resolved to wasmtime 25.0.3. Post-add `cargo audit` flagged 15 RUSTSEC advisories (RUSTSEC-2025-0046, -0118; RUSTSEC-2026-0020/-0021/-0085 through -0096) all unpatched on the 25.0.x line. wasmtime maintainers patch backward to 24.x and forward to 36/42/43 but skip 25.x entirely. Solutions universally specify `>=24.0.7 OR >=36.0.7 OR >=42.0.2 OR >=43.0.1`. Resolution: bumped to `wasmtime = { version = "43", features = ["component-model"] }` to satisfy the "25+" minimum with full patch coverage.

**Generalization:** any major dep where the named-version minor line is abandoned. Verify post-add via:

```bash
cargo audit  # flags RUSTSEC advisories
# If 1+ advisories cite the resolved version with solution >=N.x for N > current:
#   bump workspace Cargo.toml to >=N.x (latest patched stable)
#   cargo audit must return 0 vulnerabilities before proceeding
```

The route §2 chunk text uses "+" intentionally: it documents the feature-set baseline (Component Model in wasmtime 25+, async-component-model in wasmtime ~32+, etc.) without committing to the specific line. Pinning to N.x without auditing risks shipping known-vulnerable transitive deps.

**Cross-references:**
- chunk #45 implementation: workspace Cargo.toml comment block documents the version-pin rationale + April 2026 advisory cluster
- security-plan.md §Dependency Security Pinning — anchors the cargo-audit gate
- This pattern complements the existing "cargo deny check bans multi-versions = deny" canary discipline (deny.toml [bans] skip list with provenance comments per dup) — both gates fire on supply-chain regressions but for different reasons (audit = CVE; deny = duplicate-version)

---

## 2026-05-11 — Deferred AppHandle injection via Arc<OnceLock<AppHandle<Wry>>> for TauRPC resolvers needing Tauri runtime APIs

**Problem:** TauRPC routers are built BEFORE Tauri's setup closure runs. In `pulse-app/src/main.rs`, the chain `tauri::Builder::default()...invoke_handler(invoke_router.into_handler())...setup(move |app| { ... })` constructs and merges resolver impls into the router at builder-build time; the `app: &App` (and thus `app.handle()`) is only available inside the setup closure, which fires later during `.run()`. This means a resolver's `Impl::new(...)` cannot capture `AppHandle` at construction.

**Pattern:** For resolvers needing AppHandle access (clipboard write via `app.clipboard().write_text(...)`, OS notification dispatch via `app.notification().builder()...show()`, real-time push event emit via `app.emit("pulse://stream/X", payload)`), use a deferred-injection pattern via `Arc<OnceLock<AppHandle<Wry>>>`:

1. The `Impl` struct holds `app_handle: Arc<OnceLock<AppHandle<Wry>>>` (std::sync::OnceLock; std is sufficient for set-once-read-many semantics).
2. `Impl::new(...)` creates a fresh empty OnceLock wrapped in Arc; struct derives `Clone`.
3. Before merging into the router, clone the impl for the setup closure: `let snapshot_impl = SnapshotApiImpl::new(...); let snapshot_impl_for_setup = snapshot_impl.clone();`. The clone shares the SAME Arc (cheap reference bump; no OnceLock duplication).
4. Inside the setup closure (after `move |app|`): `snapshot_impl_for_setup.set_app_handle(app.handle().clone());`. The `set_app_handle` method does `let _ = self.app_handle.set(handle);` (ignore the `Result<(), AppHandle>` from `OnceLock::set` — second-call is no-op).
5. Resolver methods check `self.app_handle.get()` at every call — `Some(handle)` after setup completes, `None` only during the (small) window between router-merge and setup-closure-fire.

**Reference implementation:** `pulse-app/src/snapshot_runtime.rs::SnapshotApiImpl` (chunk #44). The AppHandle-dependent operations are best-effort: on `None` they're skipped + `success=false` is emitted in the canonical tracing target (e.g., `snapshot.clipboard.write` with `success=false`). Partial completion is visible via the per-target success flag rather than silent ignore.

**Concrete runtime parameter:** use `AppHandle<Wry>` (NOT generic `AppHandle<R: Runtime>`) since `pulse-app`'s `tauri::Builder::default()` produces `Wry`. Generic-over-Runtime would require all resolvers + main.rs setup to thread `R: Runtime` as a type parameter, adding cognitive weight for no real benefit (pulse-app has no multi-runtime support).

**Distinguishes from `pulse-app/src/viz_routers.rs` precedent:** viz_routers resolvers (`TracesApiImpl` / `MetricsApiImpl` / `LogsApiImpl`) hold `conn: Arc<Mutex<Connection>>` + `state: Arc<VizState>` — both AVAILABLE at router-build time (buffer connection initialized synchronously before router build via `init_buffer()` in `main.rs`). They don't need deferred injection. The OnceLock pattern is specifically for runtime-only handles produced by the Tauri builder lifecycle.

**Anti-pattern:** late-merging the resolver into the router from within setup. That fights Tauri's builder API — at setup time the builder's `invoke_handler` slot is already locked + the router is already merged into the handler. The OnceLock pattern keeps router construction at builder-build time + populates the runtime dependency at setup time — clean separation matching the actual lifecycle order.

**Generalizes to:** chunks #46-#49 MCP-server resolver (`pulse-app/src/mcp_runtime.rs`-equivalent will need AppHandle for `mcp.status`/`mcp.start`/`mcp.stop` lifecycle); any future plugin runtime resolver needing `app.emit()` for real-time push events.

---

## 2026-05-11 — Auto-mode classifier blocks ~/.claude/skills/ self-modification without explicit Bash permission rule (Claude Code harness safety)

When Claude attempts to Edit/Write any file under `~/.claude/skills/`, the Claude Code auto-mode classifier flags the action as "Self-modification: editing the agent's own skill files... without explicit user authorization to modify skill internals." Even if the conversation explicitly authorizes the change at user-decision level (e.g., the user said "yes — modify the skill files"), the classifier doesn't have visibility into conversation context; it sees raw file edits to ~/.claude/skills/ and applies the safety boundary.

The classifier is structurally correct here: skill files control Claude's behavior, and modifying them affects ALL future sessions across all projects. This is a system-level safety boundary (not conversation-level), and the safe default is to require explicit per-file or per-skill-glob permission rules.

**Workaround for legitimate skill modifications:** add a permission rule via `/permissions` command in Claude Code, OR directly in `~/.claude/settings.json`:

```json
{
  "permissions": {
    "allow": [
      "Edit(~/.claude/skills/andromeda-evolve/**)"
    ]
  }
}
```

Or more narrowly, just the specific files needed:

```json
{
  "permissions": {
    "allow": [
      "Edit(~/.claude/skills/andromeda-evolve/SKILL.md)",
      "Edit(~/.claude/skills/andromeda-evolve/references/refuse-taxonomy.md)",
      ...
    ]
  }
}
```

Verified at session 52 wrap when extending /andromeda-evolve to add `--allow-route-append` flag. First 4 SKILL.md edits hit the classifier denial (2 succeeded for non-rule-changing edits to Invocation + Setup; 2 denied for narrow exception clause + flag MUST/MUST NOT additions which materially weakened a refuse rule). After user added permission rule via /permissions interactive command, all 4 denied edits succeeded on retry + the 4 reference file edits + the dogfood pass artifacts all wrote without further denial.

**Lesson for future skill-modification work:** before attempting any Edit on `~/.claude/skills/`, surface to user that explicit authorization is required AND rendering the suggested permission rule text. Don't attempt the Edit first and treat denial as a surprise — the denial is the classifier doing its job, not an error.

Anchors: `~/.claude/skills/andromeda-evolve/SKILL.md` Refuse 6 narrow exception clause (added session 52 after permission rule landed); `~/.claude/skills/andromeda-evolve/references/{refuse-taxonomy,classification-taxonomy,validation-checks,output-templates}.md` (4 reference files updated for Type 7 + Check 8 + Refuse 6 Exception subsection).

---

## 2026-05-11 — Andromeda skill suite supports surgical extension via flag-pattern mirroring (--allow-route-append added to /evolve as Type 7 mirror of --allow-arch-registry Type 6)

When a felt friction surfaces in an existing Andromeda skill mid-session (today: /andromeda-evolve refuses ALL route.md modifications via Refuse 6, but the user wants to add a chunk to capture deferred work — a legitimate additive operation that doesn't restructure), the project's dual-purpose nature (building andromeda-pulse + debugging Andromeda skill suite) means the friction can be resolved via skill extension within the same session before continuing project work, rather than deferred to a separate skill-versioning workflow.

**Pattern: flag-pattern mirroring.** When extending a skill to permit a narrow exception to an existing refuse category, mirror the design of an existing flag exception. /andromeda-evolve already had `--allow-arch-registry` (narrow Refuse 1 exception for arch.md registry-section additions). Adding `--allow-route-append` (narrow Refuse 6 exception for route.md additive chunk insertion) followed the EXACT same template:

- Flag in Invocation section + Setup parsing
- MUST NOT clause for the broader refuse + narrow exception clause
- Flag-specific MUST clauses (parsing + validation activation + marker requirement)
- Flag-specific MUST NOT clauses (don't extend semantics / don't downgrade verification rigor / don't permit modifying existing content / etc.)
- New refuse template variant (additive-variant) + Exception subsection (verification rules) in refuse-taxonomy.md
- New Type N section in classification-taxonomy.md (Type 7 mirrors Type 6)
- New Check N in validation-checks.md (Check 8 mirrors Check 7)
- New marker template variant + Decisions Log entry template + state.yaml entry additions in output-templates.md

The symmetry is the safety: design-by-mirror means future readers can trust that the new exception has equivalent narrowness to the proven one. Documented sub-checks of Check 8 (8.1-8.7) all mirror Check 7's sub-checks (7.1-7.4) extended for route's additional concerns (in-progress chunk shift confirmation at 8.4; chunk text format at 8.5; motivation grounding at 8.6; Decisions Log entry well-formedness at 8.7 vs Check 7's simpler 4-sub-check structure).

**Pattern: dogfood validation immediately after skill change.** After extending /andromeda-evolve with `--allow-route-append`, the immediate next step was to dogfood the new flag for today's actual problem (chunk #43 follow-up addition). This validated the skill end-to-end — flag parsing through marker generation through state.yaml entry through arch/route edit — in the same session that introduced the flag, surfacing any design issues immediately rather than at next-session re-use time. End result: chunk #44 added cleanly to route.md Epoch 6; state.yaml.spec_amendments.active gained a Type 7 entry; second invocation later in the same session (`/andromeda-evolve --allow-arch-registry` for pulse:clipboard) used the same skill suite to land a Type 6 amendment, validating that the two flags are genuinely independent + combinable.

**Lesson for future Andromeda skill work:** when a skill needs a narrow exception, look for an existing flag-exception pattern in the same skill (or sibling skills) that you can mirror. The design symmetry is both a safety mechanism + a documentation aid (future reader sees Type N+1 and immediately knows it follows Type N's verification discipline).

Anchors: `~/.claude/skills/andromeda-evolve/SKILL.md` (Invocation/Setup/MUST/MUST NOT clauses for both flags); refuse-taxonomy.md §Refuse 1 Exception + §Refuse 6 Exception (mirrored design); classification-taxonomy.md §Type 6 + §Type 7 (mirrored structure); validation-checks.md Check 7 + Check 8 (mirrored sub-check pattern); output-templates.md Type 6 marker variant + Type 7 marker variant (mirrored field additions).

---

## 2026-05-11 — Epoch-closer chunks combining substrate activation + IPC promotion + plugin runtime + UI need pre-route splitting (chunk #43 over-scope observation)

Chunk #43 — "Workspace path detection + clipboard + notification — workspace.detect (.andromeda/ marker) + dual .json/.md + 4 preset prompts + 'Snapshot ready' toast" — was authored by /andromeda-route as a single epoch-closing chunk and validated through /andromeda-phase as "single-substantial" with 38 acceptance criteria across 7 domains. /andromeda-implement Phase 2 surfaced that the 14-step plan covered FOUR distinct concerns simultaneously: (a) substrate activation (workspace-detector crate from `pub mod contract;` stub to fully populated 5-file crate with detect/marker/vcs); (b) IPC contract promotion (snapshot.generate refined return type from `Result<(), AppError>` to `Result<SnapshotResultDto, AppError>` + new workspace.detect TauRPC procedure + 3 new IPC DTO types in ui-bridge::contract); (c) plugin runtime integration (tauri-plugin-clipboard-manager + tauri-plugin-notification deps + AppHandle injection through TauRPC resolver for clipboard.write / notification.dispatch / pulse://stream/snapshot-progress event emit); (d) webview UI surface (PresetPromptList component + InvestigationModalForm result-state UI overhaul + provider-context wrap audit + bindings consumer updates). Each concern is a substantial multi-file change; combining all four exceeds reasonable single-/implement budget.

The pragmatic resolution at /implement was to complete (a) + (b) + capability JSON + xtask EXPECTED_PROCEDURES + AllowList scrubber extension + InvestigationModalForm IPC signature update (~60% of plan steps) and DEFER (c) full Tauri runtime integration + (d) UI overhaul to a follow-up. The deferred work is well-bounded: it depends on the IPC contract that (a)+(b) established, so a follow-up chunk can pick it up cleanly.

**Pattern for future route §2 chunk decomposition:** when a chunk title combines a substrate-activation verb (workspace-detect / plugin-load / mcp-start) with multiple integration verbs (clipboard / notification / dual-file / preset-prompts) AND the chunk is the LAST in its epoch (epoch-closer), prefer splitting at /andromeda-route time into 2-3 atomic chunks rather than authoring a single composite chunk. Concretely: chunk #43 could have been three chunks — 43a "Workspace path detection substrate + workspace.detect IPC", 43b "snapshot.generate body — clipboard write + notification dispatch + dual-file persistence (Tauri runtime integration)", 43c "InvestigationModalForm result-state UI + 4 preset prompts + bindings consumer updates". Each ~5-7 acceptance criteria, ~1 day of focused work, ~one /implement invocation each.

Trigger detection at /andromeda-route Phase 2 for this pattern: chunk text containing 4+ noun phrases joined by "+" AND landing as the FINAL chunk of an epoch AND touching ≥3 distinct workspace crates AND introducing ≥2 new external runtime dependencies. Chunks meeting all four signals are epoch-closer composite chunks; split them.

Anchors: `.andromeda/route.md` Epoch 6 chunk #43 text (300+ chars composite title); `.andromeda/phases/phase-40/plan.md` 14 implementation steps + 38 acceptance criteria; `crates/ui-bridge/src/snapshot_ipc.rs` chunk #43 partial implementation note in code comment block; chunk #42 (Investigate trigger only — single concern, fit cleanly in one /implement) as the contrast precedent.

---

## 2026-05-11 — Tauri runtime integration through TauRPC resolver requires AppHandle injection — non-trivial for /implement-time scope (chunk #43 deferred-work observation)

Chunk #43's snapshot.generate IPC body needed to: (a) call `tauri-plugin-clipboard-manager::writeText` to write curated markdown to OS clipboard; (b) call `tauri-plugin-notification::sendNotification` for "Snapshot ready" toast; (c) emit `pulse://stream/snapshot-progress` Tauri event via `app_handle.emit(...)` to surface non-suppressible "X bytes copied" UI signal per security plan §Logging clipboard hygiene. All three require Tauri's `AppHandle` to access plugin extension traits + emit events. The TauRPC resolver pattern in chunks #27 (IntrospectionApiImpl) / #29 (TelemetryApiImpl) / #34 (StreamsApiImpl) / #42 (SnapshotApiImpl placeholder) all use injected state via constructor (`Arc<Mutex<Connection>>`, `Arc<BroadcastSenders>`, `PathBuf`, etc.) but NONE inject `AppHandle` — the AppHandle isn't available at router-construction time in main.rs (only inside the .setup() closure, post-router-build).

Two approaches available, both with cost:

**Approach A — store AppHandle in resolver via `Arc<RwLock<Option<AppHandle>>>`:** SnapshotApiImpl holds the cell; .setup() closure fills it via setter (`impl.init_app_handle(handle.clone())`); resolver checks `if let Some(handle) = state.app_handle.read().await.as_ref()` before each plugin call. Cost: introduces a runtime nullable check per resolver call + subtle race window between setup-fill and first IPC call.

**Approach B — taurpc resolver injection via method signature:** taurpc 0.7 supports method parameters typed as Tauri-managed types (`tauri::State<T>`, `tauri::Window`, `tauri::ipc::Channel<T>`) which Tauri injects automatically. Verifying that `tauri::AppHandle<R>` is in the supported list requires reading taurpc 0.7 internals + writing a probe — none of the existing 4 resolvers in this codebase use this pattern, so the project would be the first user.

Chunk #43 deferred this work to a follow-up chunk that can focus narrowly on it without competing scope. Recommendation when picking it up: try Approach B first (lighter-weight if it works); fall back to Approach A if taurpc 0.7 doesn't support AppHandle injection (verify by reading `D:\dev\rust\cargo\registry\src\index.crates.io-*/taurpc-0.7.1/src/proc_macros/`). Either way, the resolver body should: (1) detect/validate workspace via the new `workspace_detector::detect()` from chunk #43; (2) load spans from buffer (need to determine query API — likely `Arc<Mutex<Connection>>` injected like TracesApiImpl + a SELECT against the spans table); (3) call `snapshot::curate(spans)` then `snapshot::format_markdown(curated, budget)` (chunk #39-#41 entry points); (4) write `.json` + `.md` files under `~/.andromeda-pulse/snapshots/` via `strict-path` confined writes; (5) clipboard write + notification dispatch + event emit via the chosen AppHandle injection path; (6) emit the success-path tracing events at `snapshot.generate.request` (success kind) + `snapshot.clipboard.write` + `snapshot.notification.dispatch` (allowlist registered by chunk #43 wrap session 51); (7) return SnapshotResultDto with real values (currently chunk #43 returns deterministic stub).

Anchor: `crates/ui-bridge/src/snapshot_ipc.rs` runtime mod docstring (chunk #43 explicit deferral note); `pulse-app/src/main.rs:280-282` (where AppHandle is first available — `app: &mut tauri::App` in setup closure).

---

## 2026-05-10 — TauRPC routers live in `crates/ui-bridge/`, not in substrate crates (chunk #42 architectural convention)

The plan for chunk #42 prescribed adding the placeholder `SnapshotApi` TauRPC procedure to `crates/snapshot/src/ipc.rs`, mirroring how the procedure path `snapshot.generate` is reserved at arch §Occupied Resources to the snapshot crate. At `/implement` Phase 1 review the substrate-pollution cost showed clearly: snapshot crate would need `taurpc` + `specta` features behind a `taurpc-runtime` flag (mirroring ui-bridge's pattern), would either need its own `From<SnapshotError> for AppError` impl mirroring ui-bridge's existing one OR a new internal `SnapshotIpcError` enum, and would need new `#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]` on every contract type that crosses the bridge (`MarkdownReport` etc.). Chunks #39/#40/#41 deliberately kept the snapshot crate as pure-Rust substrate (no IPC deps, no Tauri runtime); breaking that for a placeholder swap-in was a high cost.

Resolution: the router lives in `crates/ui-bridge/src/snapshot_ipc.rs` instead, following the existing convention (`telemetry.rs` chunk #29 + `health.rs` chunks #27/introspection). ui-bridge already depends on snapshot, already owns `AppError` + `SnapshotPreset` (specta-derived since chunk #38), and already centralizes all TauRPC routers mounted by `pulse-app/src/main.rs`. The placeholder uses `Result<(), AppError>` (no MarkdownReport crossing the bridge yet; chunk #43 will refine to return real data when it wires workspace.detect + clipboard).

**Pattern for future chunks introducing TauRPC procedures whose path is reserved to a substrate crate:**
- Substrate crate exposes pure-Rust contract types (`SpanRecord` / `MarkdownReport` / `CurationOutput` etc.) without specta derives.
- ui-bridge crate hosts the TauRPC procedure trait + impl in a sibling module (e.g., `snapshot_ipc.rs`, `plugins_ipc.rs`, `mcp_ipc.rs`).
- ui-bridge re-exports `pub use snapshot_ipc::{SnapshotApi, SnapshotApiImpl}` cfg-gated by `taurpc-runtime`.
- pulse-app mounts via `use ui_bridge::snapshot_ipc::{SnapshotApi, SnapshotApiImpl};` then `.merge(SnapshotApiImpl::new().into_handler())` in all 3 router branches.
- xtask `EXPECTED_PROCEDURES` is extended with `"<router>.<method>"` per security.md Session Additions 2026-05-10 (couple D3 cleanup with consuming code).
- Capability JSON unchanged (router-level granularity via `core:default` per security.md Session Additions 2026-05-03).

Arch's §Occupied Resources Tauri IPC routes list reserves PATHS (`snapshot.generate`, etc.); it does NOT constrain WHERE the resolver code lives in Rust source. The router-location convention is project-style, established by chunk precedent, and now documented here. Chunks #43 (snapshot.{list_recent,copy_to_clipboard}), #44+ (plugins.*), #46 (mcp.*), and #47 (workspace.*) should follow this pattern unless a specific reason (e.g., the procedure requires substrate-internal state that ui-bridge can't access) forces the router into the substrate crate.

Anchors: `crates/ui-bridge/src/lib.rs:3-19` (module list + cfg-gated re-exports), `crates/ui-bridge/src/snapshot_ipc.rs` (chunk #42 placeholder), `crates/ui-bridge/src/telemetry.rs:93-134` (canonical pattern reference), `pulse-app/src/main.rs:19-20, 263, 268, 658` (3-branch mount).

---

## 2026-05-10 — Placeholder TauRPC IPC return type: prefer `Result<(), AppError>` over the eventual data type when bindings aren't specta-ready (chunk #42 substrate-vs-bindings reality)

When wiring a placeholder TauRPC procedure that will swap in a real return value in a later chunk, the return type signature must satisfy specta::Type at chunk-introduction time — even though the placeholder body never produces the value. If the eventual return type lives in a substrate crate without specta derives (e.g., `snapshot::contract::MarkdownReport` chunk #41), adding specta to the substrate crate just to satisfy the placeholder is substrate pollution. Using `Result<(), AppError>` for the placeholder sidesteps the bindings constraint cleanly: `()` is always specta-friendly (serializes as `null`), the placeholder always returns `Err(AppError::Internal { ... })`, and the webview can call `await proxy.snapshot.generate(preset)` then `.catch(rawErr => ...)` to render the placeholder error.

Chunk #43 will refine the signature to `Result<MarkdownReport, AppError>` once it needs to surface real curation output. At that point, two options exist: (a) add `#[cfg_attr(feature = "ipc-bindings", derive(specta::Type))]` to MarkdownReport in the snapshot crate (modest substrate concession, gated behind a feature so xtask-style consumers can still skip the dep), OR (b) define an `IpcMarkdownReport` in ui-bridge that mirrors the substrate type with specta derives + a `From<MarkdownReport> for IpcMarkdownReport` conversion. Option (a) is simpler if you accept the feature-gate; option (b) keeps substrate purest.

Corollary to chunk #41's "substrate-vs-IPC enum naming alignment" learning: there, TokenBudget (snapshot crate) and SnapshotPreset (ui-bridge crate) were aligned by-name (Conservative/Balanced/Detailed). Chunk #42 leverages that alignment: the placeholder accepts `SnapshotPreset` (already specta-derived in ui-bridge) directly, no `From<SnapshotPreset> for TokenBudget` conversion needed yet because the placeholder body doesn't reach the snapshot crate. Chunk #43 will need the conversion. The naming alignment from chunk #41 makes that future conversion trivial (by-name match), which is why the naming-alignment discipline pays off chunks later.

Anchor: `crates/ui-bridge/src/snapshot_ipc.rs:11` (`async fn generate(preset: SnapshotPreset) -> Result<(), AppError>;`).

---

## 2026-05-10 — Wrap-pattern consumer visibility: `pub fn` in `pub(crate) mod` for re-export through contract module (chunk #41 corollary to chunks #39/#40)

The chunk #40 session-learnings entry below distinguished primitives that EXTEND `curate()` (operating on raw `&[SpanRecord]`, kept `pub(crate) fn`) from consumers that WRAP `curate()`'s output (operating on `CurationOutput`). Chunk #41 (markdown formatter) is the FIRST chunk to land a wrapping consumer, and the visibility shape needs adjusting from the chunk #40 pattern: a wrapping consumer that will be called from outside the snapshot crate (e.g., chunk #43 `snapshot.generate` IPC; chunk #46 MCP `generate_snapshot` `#[tool]`) MUST declare its public function as `pub fn` (not `pub(crate) fn`) inside the `pub(crate) mod` sibling module. Re-exporting `pub(crate) fn` via `pub use crate::markdown::format_markdown;` in the public `contract.rs` fails to compile with `error[E0364]: pub(crate) item ... cannot be re-exported outside`.

**Pattern for wrap-vs-extend:**
- EXTEND-curate primitives (call-site internal, e.g., `aggregation::aggregate_metrics` / `attribute_filter::filter_attributes`): stay `pub(crate) fn` in `pub(crate) mod`. Called only by `curate()`, never re-exported.
- WRAP-CurationOutput consumers (call-site external, e.g., `markdown::format_markdown`): declare as `pub fn` in `pub(crate) mod`. The mod itself stays `pub(crate)` so the only reachable path is via `pub use` re-export from `pub mod contract`. Effective external surface: `snapshot::contract::format_markdown`.

This preserves the arch §Conventions "only the contract module exposes pub types" rule while allowing wrap-pattern consumers to be reachable across the crate boundary. The mod's `pub(crate)` visibility prevents direct `snapshot::markdown::format_markdown` access; only the contract-mediated path works. Apply to chunk #46 MCP tool wrapper and any future substrate-consumer chunks.

The companion types (`MarkdownReport` / `TruncationState` / `FormatError` / `TokenBudget`) live in `contract.rs` directly (not re-exported) per the existing "pub types in contract.rs" convention; only the orchestrator function (`format_markdown`) needs the re-export-from-sibling-mod pattern. Naming the sibling mod `markdown` (not `format` or `formatter`) keeps the dotted-name `snapshot::contract::format_markdown` parallel to `snapshot::contract::curate` — both verbs, both action-oriented, both top-level entry points to the crate's logical pipelines.

---

## 2026-05-10 — Substrate enum variant naming alignment with already-shipped IPC enum (chunk #41 TokenBudget mirrors chunk #38 SnapshotPreset)

Chunk #41 introduced `TokenBudget { Conservative / Balanced / Detailed }` in `crates/snapshot/src/token_budget.rs`. Three sub-agents in /andromeda-phase Phase 1 suggested DIFFERENT vocabularies: design proposed `Compact / Balanced / Detailed`; security proposed `TenK / TwentyFiveK / FiftyK`; arch proposed `Compact10k / Balanced25k / Generous50k`. The pre-existing `ui-bridge::contract::SnapshotPreset` shipped at chunk #38 already uses `Conservative / Balanced / Detailed`. Chunk #41 aligned with ui-bridge to enable a trivial future `From<SnapshotPreset> for TokenBudget` impl by-name match (`Conservative ↔ Conservative`, `Balanced ↔ Balanced`, `Detailed ↔ Detailed`) when chunk #43 IPC wiring lands.

**Decision rule for substrate-vs-IPC enum naming:** when a substrate type (algorithmic primitive in a `crates/{substrate}` workspace crate) will eventually map to an already-shipped IPC type (TauRPC procedure arg in `crates/ui-bridge`), align the variant names verbatim. Avoid parallel vocabularies (`TenK/TwentyFiveK` vs `Conservative/Balanced/Detailed`) even when the parallel form is more "self-documenting" — the cost of cross-readability + future-impl simplicity outweighs the loss of explicit-numeric naming. Documented values (10_000 / 25_000 / 50_000) live in `as_token_count()` const fn so the IPC variant name doesn't need to encode the numeric.

This reverses the natural intuition that substrate (close to numeric implementation) "should" use numeric naming (`TenK`), and IPC (close to user) "should" use semantic naming (`Conservative`). The opposite preserves naming alignment, which is the more valuable cross-cutting invariant. Apply to any future substrate type whose IPC counterpart already exists (e.g., a future `TraceWindow` substrate enum should match `TraceQueryWindow` IPC enum verbatim if/when introduced).

**Phase 1 sub-agent guidance:** when arch + design + security extracts disagree on naming, the orchestrator's Phase 3 codebase research is the tie-breaker — read `crates/ui-bridge/src/contract.rs` (or equivalent IPC-side public surface) for already-shipped enum names BEFORE Phase 4 plan synthesis picks one. The plan should EXPLICITLY name the choice + cite the ui-bridge precedent in `## Implementation Steps` step 1, so /implement doesn't silently pick from a sub-agent suggestion that diverges.

---

## 2026-05-10 — Algorithmic-substrate chunks: primitives EXTEND `curate()` rather than wrap its output (chunk #40 refinement of chunk #39 entry)

The chunk #39 session-learnings entry below anticipated chunk #40 + #41 would "wrap `curate(...)` outputs without modifying the snapshot crate" — both as downstream CONSUMERS. In practice, chunk #40 (aggregate_metrics + filter_attributes) had to EXTEND `curate()` itself: the new primitives operate on raw `&[SpanRecord]` (not `CurationOutput`), so they belong inside the orchestrator as new pipeline stages, not as wrappers around its return value. CurationOutput grew with `aggregation: AggregationResult` + `kept_attribute_count: usize` + `dropped_attribute_count: usize` (each `#[serde(default)]` to preserve chunk #39 round-trip serde compatibility), and `curate()`'s body was reordered to: `filter_attributes(spans) → dedupe_spans(filtered.spans) → aggregate_metrics(deduped) → detect_anomalies(deduped) → extract_critical_path(deduped)`.

**Refined boundary:** primitives that operate on RAW SpanRecord (or any pre-curation input) extend `curate()` as new pipeline stages. Primitives that operate on CurationOutput (e.g., chunk #41 markdown formatter, chunk #46 MCP tool) wrap `curate(...)` at the call site. The dividing line is whether the primitive needs raw input access vs curated output access.

**Constraint that forces wiring (not allow_dead_code):** `cargo clippy --workspace --all-targets --all-features -- -D warnings` rejects any `pub(crate) fn` not called by non-test code. Marking new primitives `#[allow(dead_code)]` is a smell signaling "this substrate has no caller yet" — accept only when downstream chunk genuinely defers consumption to a separate crate (e.g., mcp-server `#[tool]` wrapper landing several chunks later). For same-crate primitives that the next chunk in the same epoch will wrap, default to wiring through `curate()` so the workspace stays clippy-clean from chunk landing.

**Pattern in `curate()` instrument fields:** when extending the orchestrator with new pipeline stages, append the stage's count fields to `curate()`'s `#[tracing::instrument(skip_all, fields(...))]` field list (e.g., `kept_attribute_count`, `dropped_attribute_count` from filter_attributes) AND record them on the early-return path so empty input still emits the full allowlisted field set. Aggregate's percentile fields stay in `aggregate_metrics`'s own #[instrument] (which emits at `snapshot::aggregation` and resolves via `split('::').next()` fall-through to the `snapshot` allowlist entry).

**Pattern for SpanRecord field extension:** adding `attributes: Vec<(String, String)>` (or any new field) to a chunk-#39-public type requires (a) `#[serde(default)]` on the new field for serde backward-compat; (b) updating ALL test fixture struct literals in same-crate sibling files (`dedupe.rs` / `anomaly.rs` / `critical_path.rs` / `contract.rs`) — Rust struct literal syntax doesn't honor serde defaults. Mechanical chore, but high-touch (4 files modified for a 1-field extension).

---

## 2026-05-10 — Algorithmic-substrate chunks: keep primitives call-site-agnostic for shared TauRPC + MCP consumption (chunk #39)

When a chunk introduces pure-function primitives that multiple downstream surfaces will consume (e.g., the `crates/snapshot` curation primitives at chunk #39 — `dedupe_spans` / `detect_anomalies` / `extract_critical_path` — which feed BOTH chunk #41 TauRPC `snapshot.generate` AND chunk #46 MCP `generate_snapshot` `#[tool]` method per route.md §3 Decisions Log "Snapshot pipeline shared with MCP"), design the public API to be call-site-agnostic. Specifically:

1. **No framework imports in the substrate layer** — primitives in `crates/snapshot/src/{dedupe,anomaly,critical_path}.rs` have zero `taurpc::*`, `tauri::*`, `rmcp::*`, `specta::Type` imports. Public API accepts plain Rust types: `&[SpanRecord]`, returns `Result<CurationOutput, Error>`. Framework wrapping happens at the boundary chunks (#41 TauRPC resolver, #46 MCP tool wrapper).

2. **Cross-bridge data shape is `serde::Serialize` + `serde::Deserialize` only at chunk #39** — `specta::Type` derive deferred to whichever boundary chunk crosses TauRPC first. Snapshot crate stays pre-bridge (no `taurpc-runtime` feature; no `specta` dep). Verified at chunk #39: zero new TauRPC procedures introduced; `cargo xtask capability-drift` clean by-construction (security ↔ tests/CI ↔ arch capability-drift triple binding NOT triggered per `.claude/rules/security.md` Session Additions 2026-05-09 first entry).

3. **Internal modules are `pub(crate)`; public surface is the contract module only** — `crates/snapshot/src/lib.rs` re-exports only `pub mod contract;`; new sibling modules declared as `pub(crate) mod {dedupe,anomaly,critical_path};`. Only types in `contract.rs` (e.g., `SpanRecord` / `CurationOutput` / `AnomalyKind` / `AnomalyMarker` / `CriticalPathStep`) participate in the public API. This keeps the substrate's internal evolution loose while pinning the cross-crate contract.

4. **Deterministic outputs via explicit `sort_by_key` — no `HashMap` iteration without sort** — `dedupe_spans` collects into `HashMap<(service_name, name, duration_bucket), SpanRecord>` for the dedup pass but materializes via `into_values().collect::<Vec<_>>()` followed by `.sort_by(|a, b| ...)` on a stable composite key (service, name, span_id) before return. `detect_anomalies` orchestrator concats sub-detector outputs and `sort_by_key(|m| (Reverse(m.severity), kind_ordinal, first_id))` for severity-descending byte-identical output across repeated invocations. `extract_critical_path` uses `prefer_longer_or_lex` tie-breaker (longer total wins; ties broken by lexicographic span_id first-step) to ensure the same DAG yields the same path on every run.

This pattern decouples shared substrate from any single caller. When the bridge chunks (#41 / #46) land, they wrap `crate::contract::curate(...)` independently — each one converts its own input format (TauRPC arg shape via specta, MCP arg shape via rmcp `#[tool]` macro) to `Vec<SpanRecord>` at the boundary and serializes `CurationOutput` back through its own framework. The substrate is invariant to the choice.

Apply to future algorithmic-substrate chunks (e.g., chunk #40 aggregation + low-signal drop, which extends the same pattern with metric percentile computation; chunk #41 markdown formatter, which is a SECOND consumer alongside future MCP tool — both wrap `curate(...)` outputs without modifying the snapshot crate). The discipline floor is: if chunk introduces functions called by multiple downstream IPC surfaces, audit the imports and reject any framework type leaking into the substrate.

---

## 2026-05-10 — Plan-vs-IPC reality check at /andromeda-implement Phase 1 (chunk #38)

When a chunk plan asserts that a TauRPC procedure exists (e.g., the chunk #38 plan invoked `taurpc.plugins.list()` for the plugin-manager UI section), `/andromeda-implement` Phase 1 should verify the procedure's existence via `pulse-app/ui/src/bindings/index.ts` (the Specta-generated TauRPC bindings — single source of truth for what's actually wireable from webview) BEFORE writing form code that depends on it. The plan is authored upstream of the bindings; if a chunk would need an unrendered procedure, that's an out-of-scope problem (the procedure belongs to a future epoch / chunk) and should degrade to a placeholder rather than expand scope.

Verified at chunk #38: plan section called for `plugins.list` + `plugins.reload` invocation; bindings revealed neither exists yet (`plugins.*` namespace is epoch 7 chunk #43+ territory). Adding the procedures in chunk #38 would have triggered the security ↔ tests/CI ↔ arch capability-drift triple binding the plan was specifically structured to avoid (per `.claude/rules/security.md` Session Additions 2026-05-09). Resolution: render plugin-manager section as a static placeholder (`<section><h3>Plugin manager</h3><p>Plugin discovery + reload UI lands in epoch 7 alongside the plugins.list IPC surface.</p></section>`) and adjust the chunk's tab-order spec to drop the plan's plugin-manager-reload entry. Acceptance criteria still pass; section heading + placeholder text preserved for downstream-chunk visibility.

Pattern: at Phase 1 step 1, before writing TS code for a planned IPC invocation, grep `pulse-app/ui/src/bindings/index.ts` for the procedure name. If absent, surface as scope deviation in Phase 1 banner ("Note re plan vs reality: …") and degrade to placeholder. Capability-drift gate (`cargo xtask capability-drift`) in Phase 2 confirms no new TauRPC namespaces were introduced. The placeholder is deliberately verbose ("lands in epoch 7 alongside plugins.list") so downstream chunks discover it via grep and can replace it with the real UI.

---

## 2026-05-10 — Tauri 2 tray-icon implementation discipline (chunk #36)

Three gotchas surfaced at chunk #36 introducing the OS-native tray surface (`pulse-app/src/tray.rs` + tauri::tray::TrayIconBuilder + tauri::menu builders). Verified on Tauri 2.11.0 / tauri-cli 2.11.1.

**(a) `tray-icon` Cargo feature is NOT in Tauri 2.11 default features.** The default feature set per `cargo metadata` is `["wry", "compression", "common-controls-v6", "dynamic-acl", "x11", "dbus"]`; `tray-icon` is opt-in. Without the feature, `tauri::tray::TrayIconBuilder` and `tauri::menu::*` are not in scope and compile fails with "use of undeclared module". Must explicitly add `tauri = { workspace = true, features = ["tray-icon"] }` to consumer crate's `Cargo.toml` (override at the crate level, NOT in the workspace's `[workspace.dependencies]` — the latter would force every consumer to pull tray-icon even if they don't need it). Verify available features via `cargo metadata --format-version 1 | python -c 'import json,sys; m=json.load(sys.stdin); ts=[p for p in m["packages"] if p["name"]=="tauri" and p["version"].startswith("2.")]; print(ts[0]["features"] if ts else "none")'`.

**(b) Programmatic monochrome icon construction sidesteps PNG-decoder build deps.** `tauri::image::Image::new(rgba: &'static [u8], width: u32, height: u32)` accepts raw RGBA bytes — no `image-png` or `image-ico` features needed (those features pull the `image` crate transitively, ~30 deps). For a static line-based glyph, build 32×32 RGBA in code (one byte per channel; lit pixels = 255-255-255-255, transparent = 0-0-0-0; distance-from-center / arc-coordinate logic produces aperture/circular-pulse motifs in ~30 lines), then `Vec::leak()` for 'static lifetime (~4KB negligible alloc for app lifetime). Pattern: `let pixels = build_glyph_pixels(); let leaked: &'static [u8] = pixels.leak(); tauri::image::Image::new(leaked, 32, 32)`. Avoids external rasterization tooling AND reduces the build-time feature surface. Useful when the design intent is a simple line-based glyph that can be expressed as basic geometry (circle outline, concentric arcs, center dot — all computable from `(x-cx)² + (y-cy)²` distance + threshold checks).

**(c) `TrayIcon` is RAII: caller MUST `app.manage(tray_icon)` to keep it alive.** Dropping the `TrayIcon<R>` handle returned by `TrayIconBuilder::build(app)?` causes the OS-native tray icon to immediately disappear. The setup-closure pattern is `let tray = tray::setup_tray(...)?; app.manage(tray);` — `app.manage()` requires `use tauri::Manager;` in scope (easy to miss; cargo error is "no method named manage found for mutable reference `&mut tauri::App`" with hint to import `tauri::Manager` trait). Same lifetime-ownership shape as `TrayIconBuilder::menu(&menu)` — menu and tray handles both managed via Tauri State for app-lifetime persistence. Stored handles are not retrieved by user code afterward (one-time setup); the `app.manage()` call's only purpose is to extend lifetime past the setup closure return.

---

## 2026-05-10 — Buffer schema extension cross-crate ripple pattern

When extending a viz query response struct (e.g., `TraceRow`, `MetricRow`, `LogRow`) with new fields backed by DuckDB columns, the change ripples across **5 distinct edit sites in 4 files** — anything less leaves the workspace incoherent. Verified at chunk #34 when `TraceRow` extended from 3 fields to 6 (added `service`, `duration_ms`, `error_count`):

1. **`crates/buffer/src/schema.rs`** — both `CREATE_SPANS` const AND the duplicated DDL inside the `SCHEMA_DDL` `concat!()` block. The two strings are intentionally synchronized; the `ddl_constants_match_concatenated_schema` test catches drift between them. Add new columns to both.
2. **`crates/buffer/src/appender.rs`** — `build_spans_record_batch()` Arrow Schema (the `Field::new(...)` list) AND the per-row population loop AND the helper that extracts the new field from OTLP proto (e.g., `extract_service_name(resource: Option<&Resource>)` for service.name attribute lookup). Column count assertion in `build_spans_record_batch_returns_some_for_valid_input` test must update from old N to new N.
3. **`crates/buffer/src/retention.rs`** — the test-only `seed_span()` helper's `INSERT INTO spans (...) VALUES (...)` SQL must include the new columns OR the test inserts will fail with `NOT NULL constraint failed: spans.{new_col}`. Same for `crates/buffer/src/schema.rs::ts_unix_nano_round_trips_full_u64_precision` test which has its own inline INSERT.
4. **`crates/viz/src/query.rs`** — `SELECT_TRACES` SQL constant (add new columns to projection) + `query_traces` row-decode (`row.get(N)` for each new column) + `TraceRow` struct definition + per-row construction site + the test helper `seed_span()` and `seed_span_full()` AND the inline test schema in `open_in_memory_with_schema()` (which mirrors a subset of the production buffer schema).

The TauRPC bindings file `pulse-app/ui/src/bindings/index.ts` auto-regenerates from `cargo build` via specta derive — no manual edit. Verify by `grep TraceRow pulse-app/ui/src/bindings/index.ts` after build.

Failure mode if any site is missed: production builds fine but tests fail at runtime with one of: (a) `NOT NULL constraint failed: spans.{col}` from any test that inserts spans without populating the new columns; (b) row decode panic if SELECT projects N+K columns but the row-decode reads N; (c) Arrow `RecordBatch::try_new` shape mismatch if Schema has K fields but value-arrays Vec has N. Discovery typically surfaces via `cargo nextest run -p buffer` failing first (touches the schema directly), then `cargo nextest run -p viz` (touches the row decode).

This applies to chunks #35 (`MetricRow` extension if metrics-charts surface needs additional columns from `metrics_points` table) and downstream — the same ripple pattern recurs across `MetricRow` / `LogRow` shape changes. Schema-extending chunks should expect ~250 LoC across these 4 files plus 6 new tests for the new column population paths.

---

## 2026-05-09 — taurpc 0.7 `Router::into_handler()` requires tokio runtime in scope; sync `fn main()` panics at boot

`taurpc::procedures`-decorated traits expand into a handler that, when materialized via `Router::into_handler()`, spawns a background handler-manager task during binding emission (taurpc 0.7's mechanism for emitting the merged TS `bindings/index.ts` in dev mode). The spawn requires a tokio runtime to be the **current** runtime in scope (thread-local). In sync `fn main()`, no runtime is current — Tauri's `Builder::run()` only establishes one inside `.run()`, after the router has already been constructed and passed via `.invoke_handler(invoke_router.into_handler())`. The result is a panic at boot reported at the `#[taurpc::procedures]` macro line of the FIRST handler whose `.into_handler()` is called (e.g., `crates/ui-bridge/src/health.rs:291` — the IntrospectionApi macro — for the chunk #27 wiring).

Panic message: `there is no reactor running, must be called from the context of a Tokio 1.x runtime`. The boot panic hook captures it as a JSON line at `~/.andromeda-pulse/logs/agent-latest.jsonl.{date}` with target `app.panic.fatal` and field `location: "crates\\ui-bridge\\src\\health.rs:291"`.

The chunk #25 `emit_taurpc_bindings` test masks this in the test fixture because `#[tokio::test]` runs the test inside a tokio runtime — that's why the test passes despite production main() panicking.

**Canonical fix** (per Tauri 2.11 `tauri::async_runtime::set` rustdoc example at `D:/dev/rust/cargo/registry/src/.../tauri-2.11.0/src/async_runtime.rs:240`): build a multi-thread tokio runtime, enter it via `runtime.enter()`, then call `tauri::async_runtime::set(tokio::runtime::Handle::current())` so Tauri's setup-closure spawns and the pre-`run()` taurpc binding-emission spawns share a single runtime. Must run BEFORE the Tauri Builder is constructed.

```rust
fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime");
    let _enter = runtime.enter();
    tauri::async_runtime::set(tokio::runtime::Handle::current());
    // ... rest of main: observability::init, router construction, Builder::run() ...
}
```

Drop order matters: `_enter` (the entered guard) must drop before `runtime` (the owned Runtime). Local variable declaration order achieves this — Rust drops in reverse declaration order, so `_enter` (declared after) drops first.

`tauri::async_runtime::set` panics if called twice — boot must call it exactly once, before any Tauri/taurpc API. Subsequent `tauri::async_runtime::spawn` and the lazy global `RUNTIME` static both consume the handle we provided.

See: `pulse-app/src/main.rs::main()` runtime entry block (the canonical implementation), this protocol's complement entry below from 2026-05-08 ("taurpc 0.7 emits no-path procedures...") which covers the SHAPE of bindings emission while this entry covers the RUNTIME prerequisite for emission to happen at all.

---

## 2026-05-08 — taurpc 0.7 emits no-path procedures under empty-string router key in bindings.ts

When `#[taurpc::procedures]` is declared WITHOUT a `path = "..."` attribute (top-level procedures per arch §Conventions "Endpoint naming" cross-cutting envelope), taurpc 0.7 emits the procedures into bindings.ts with an empty-string router key. Concretely, for the chunk #27 `IntrospectionApi { app_info, health, ready, get_settings, update_settings }` (no path attribute), the emitted ARGS_MAP line is:

```
const ARGS_MAP = { '':'{"app_info":[],"get_settings":[],"health":[],"ready":[],"update_settings":["settings"]}', ... }
```

And the Router type:

```typescript
export type Router = { "": {app_info: () => Promise<AppInfo>, ... }, "logs": { ... }, ... }
```

This contrasts with `#[taurpc::procedures(path = "X")]` which emits `'X':'{"method":[...]}` (router key is the path string). Both forms coexist in the same merged ARGS_MAP — the bindings.ts shows the union of all routers' methods including any top-level (`''` key) methods.

Consequences for downstream consumers:

1. **xtask capability-drift parser** (chunk #27 `xtask::parse_bindings`): the parser must handle empty-string router keys. When the outer key is `''`, methods are stored as bare `method_name` (top-level); when non-empty, as `router.method` (dotted). The parser at `xtask/src/main.rs::parse_bindings` walks the JS-style object literal byte-by-byte (single-quoted outer delimiters, double-quoted inner JSON) and treats empty router-key strings as the top-level case via `if router.is_empty() { discovered.insert(method_name.clone()) } else { discovered.insert(format!("{router}.{method_name}")) }`.

2. **TS consumers** (e.g., `pulse-app/ui/src/bindings/bindings.test.ts`): top-level procedures are accessed as `Router[""]["health"]()`, NOT `Router["health"]()`. The Router type has 5 keys for chunk #27's wired routers: `'' | 'logs' | 'metrics' | 'streams' | 'traces'`. Type assertions like `keyof Router = ""|"logs"|...` need to include the empty string as a valid key.

3. **Drift-check expected list**: the `EXPECTED_PROCEDURES` constant in xtask treats top-level procedures as bare names (e.g., `"app_info"`) and namespaced procedures as dotted (e.g., `"traces.query"`). This matches the parser's flattened output.

Discovered chunk #27: the IntrospectionApi was implemented with no `path` attribute (rejecting the chunk #25 precedent of `path = "health"` with `check()` method) to conform to arch §Standard Contracts which lists `app_info`/`health`/`ready`/`get_settings`/`update_settings` as top-level cross-cutting envelope procedures. Verified via `emit_taurpc_bindings` test regenerating bindings.ts. The empty-string router key is a stable taurpc 0.7 emission contract — future top-level procedure additions can rely on this shape; future drift-check parser changes should keep the empty-router-as-top-level handling.

See: `crates/ui-bridge/src/health.rs::runtime` mod (chunk #27 `IntrospectionApi` declaration without `path`), `xtask/src/main.rs::parse_bindings + capability_drift_tests::parse_bindings_handles_top_level_procedures_via_empty_router`, `pulse-app/ui/src/bindings/index.ts` ARGS_MAP line (canonical artifact), `pulse-app/ui/src/bindings/bindings.test.ts` "Router top-level (empty key)" test. Complements the 2026-05-07 entry below ("taurpc 0.7 binding emission is RUNTIME in dev mode") which covers the WHEN of emission; this entry covers the SHAPE of emission for the no-path case.

---

## 2026-05-07 — taurpc 0.7 binding emission is RUNTIME in dev mode, requires tokio runtime + proper cwd

The `#[taurpc::procedures(export_to = "...")]` macro arg does NOT cause emission at build time. Emission triggers when `Router::into_handler()` is called per `taurpc-0.7.1/src/lib.rs`:

```rust
pub fn into_handler(self) -> impl Fn(Invoke<R>) -> bool {
    if tauri::is_dev() {
        if let Some(export_path) = self.export_path { export_types(...); }
    }
    ...
}
```

Two compile-time gates that must both align:
1. `tauri::is_dev()` is `pub const fn = !cfg!(feature = "custom-protocol")`. Debug builds (`cargo run`, `cargo nextest`, `cargo build`) → custom-protocol OFF → is_dev()=true → emit. Release/bundled builds (`cargo tauri build`) set custom-protocol → is_dev()=false → no emit. So local dev + tests both emit; production bundles do not.
2. `Router::merge(handler)` calls `handler.spawn()` which the `#[taurpc::resolvers]` macro generates as `tokio::spawn(async move { ... })` (per `taurpc-macros-0.7.1/src/generator.rs:329`). This panics with `there is no reactor running, must be called from the context of a Tokio 1.x runtime` if invoked outside a tokio context. Implication: `fn main()` (non-async, no `#[tokio::main]`) cannot call `Router::new().merge(...)` at top-level — the spawn fires before `tauri::Builder::default().run()` initializes its runtime. This is why `cargo run --bin pulse-app` panics on Windows pre-emission: bare `fn main()` + Tauri 2's `tauri::async_runtime` not yet active. Workaround for emission: drive Router construction from `#[tokio::test]` (test runtime active); for production main(), `cargo tauri dev` sets up runtime before invoking the binary entry. agent-run.sh `boot` is gated `if: runner.os == 'Linux'` partly because of this.

Single-file emission semantics: `Router::merge` collects EVERY merged handler's args/types/fns into one `args_map_json` + `fns_map` + `types` collection. `Router::into_handler()` calls `export_types()` ONCE with the accumulated state, writing one merged TS file covering all routers. EXPORT_PATH last-set-wins across merges (per `Router::merge`: `if H::EXPORT_PATH.is_some() { self.export_path = H::EXPORT_PATH; }`). So putting `export_to` on a single procedures macro (e.g., the root `HealthApi`) is sufficient and idiomatic — subsequent `merge()` calls don't need their own `export_to`.

Path resolution: relative to runtime cwd. `cargo nextest -p pulse-app` runs with cwd=`pulse-app/`; `cargo tauri dev` from the app dir same. Picked path `ui/src/bindings/index.ts` for chunk #25 (relative to pulse-app/), works for both. `cargo run --bin pulse-app` from workspace root would expect a different path — incompatible without changing convention.

See: `pulse-app/src/main.rs::emit_taurpc_bindings` test (drives runtime emission); `crates/ui-bridge/src/health.rs::runtime` mod (root procedures with `export_to = "ui/src/bindings/index.ts"`); chunk #25 fix-loop iteration #2 root cause; taurpc-0.7.1/src/lib.rs:295-310 (Router::into_handler emission gate); taurpc-0.7.1/src/lib.rs:308 (`tauri::is_dev()` definition).

---

## 2026-05-07 — Specta TypeScript export requires explicit BigInt config or fails-by-default for u64/i64

`specta-typescript = "0.0.9"` (transitively pulled by taurpc 0.7) ships a default `BigIntExportBehavior::Fail` config that REJECTS any `i64`/`u64`/`i128`/`u128` BigInt fields with the diagnostic `"Specta configuration forbids exporting BigInt types (i64, u64, i128, u128) because we don't know if your se/deserializer supports it"`. Default-build TauRPC binding emission therefore panics on the first BigInt-typed field (e.g., `HealthEnvelope.uptime_ms: u64`, `TraceRow.ts_unix_nano: i64`).

Resolution requires explicit `Router::export_config()`:

```rust
use specta_typescript::{BigIntExportBehavior, Typescript};

let router = taurpc::Router::<tauri::Wry>::new()
    .export_config(Typescript::default().bigint(BigIntExportBehavior::Number))
    .merge(...)
```

Three behavior choices, each with trade-offs:

- `BigIntExportBehavior::Number` — emit as TS `number`. Acceptable up to 2^53 (`Number.MAX_SAFE_INTEGER`). Loses precision for nanosecond timestamps (current ~2^61), millisecond × very-long-running counters, and any future cardinality-large counter. Picked for chunk #25's binding scaffold; suitable when consumers don't need precision past 2^53.
- `BigIntExportBehavior::BigInt` — emit as TS `bigint`. Preserves precision but JSON.stringify/parse won't round-trip natively (BigInt isn't standard-JSON-serializable). Webview consumers must handle the bigint↔string conversion at I/O boundaries.
- `BigIntExportBehavior::String` — emit as TS `string`. Safest; consumers convert via `BigInt(str)`. Annoying for fields that are obviously numeric (e.g., uptime_ms in milliseconds).

`specta-typescript` is a TRANSITIVE dep of taurpc 0.7 (see taurpc-0.7.1/Cargo.toml deps), but the public API for `BigIntExportBehavior` lives ONLY in `specta-typescript` proper — taurpc's `pub use specta_typescript::Typescript` doesn't re-export the enum. So the consumer crate (pulse-app) must add `specta-typescript = "0.0.9"` directly to `[dependencies]` to access the enum at the call site. Workspace dep already has `taurpc = "0.7"` + `specta = "=2.0.0-rc.22"` (with `chrono` feature); chunk #25 added `specta-typescript = "0.0.9"` as a peer.

Naming gotcha for ts_unix_nano fields: TraceRow/MetricRow/LogRow all carry `ts_unix_nano: i64` (nanoseconds since epoch). With `Number` mapping, current Unix nanoseconds (~2^61) lose ~10 bits of precision in JSON parse — webview "trace at 12:34:56.789..." displays drift by ~1 ms per second elapsed. Documented inline at the helper site for chunk #25; later chunks should switch to BigInt or String for nanosecond-sensitive consumers.

See: `pulse-app/src/main.rs::taurpc_export_config` helper (chunk #25); chunk #25 fix-loop iteration #2 (initial test panicked with "BigInt types forbidden"); specta-typescript-0.0.9/src/typescript.rs:26 (`BigIntExportBehavior` enum); specta-typescript-0.0.9/src/lib.rs:249-256 (per-variant rendering).

---

## 2026-05-07 — npm `taurpc` package versioning is INDEPENDENT of the Rust crate `taurpc` versioning

The Rust crate `taurpc = "0.7"` (current 0.7.1) and the npm package `taurpc` (current 1.8.1) ship from the same upstream repo (MatsDK/TauRPC) but use DIFFERENT semver streams. The README's frontend-install instruction `pnpm install taurpc` is version-agnostic on purpose; users must look up the latest npm version separately.

Pitfall: if you blindly mirror the Rust crate version to the npm package (`taurpc@^0.7.1` in package.json), npm rejects with `ETARGET / No matching version found for taurpc@^0.7.1`. The npm package never published 0.x — the lowest npm version is 1.0.0. Use `^1.x.y` on the npm side; treat the crate version and npm version as separate dimensions.

Compatibility envelope: each crate version corresponds to some npm version that emits compatible `BOILERPLATE_TS_IMPORT` (`import { createTauRPCProxy as createProxy, type InferCommandOutput } from 'taurpc'`). That import must resolve to a `taurpc` npm package that exports those names. Until taurpc 0.x has a major release, npm 1.x.y likely tracks the same boilerplate shape — but no formal compatibility matrix exists. Verify by reading the npm package's exports and matching against the BOILERPLATE_TS_IMPORT in `taurpc-0.{x}.y/src/export.rs`.

For chunk #25: `taurpc = "0.7"` (workspace Cargo.toml) + `taurpc": "^1.8.1"` (pulse-app/ui/package.json devDependencies) — both compatible at session 25.

See: `pulse-app/ui/package.json` chunk #25 (devDependencies entry); chunk #25 fix-loop iteration #1 (`npm install ^0.7.1` rejected); npm `taurpc` view command (`npm view taurpc versions --json`) confirms 1.x stream only.

---

## 2026-05-06 — RecordBatch reuse refactor: extract `build_*_record_batch` from `append_*_batch` to enable fan-out

When a producer crate needs to emit the same Arrow `RecordBatch` data to multiple sinks (e.g., chunk #23: DuckDB persist via `Connection::appender(...).append_record_batch(...)` AND tokio broadcast emit via Arrow IPC StreamWriter byte stream), refactor any existing single-sink `append_*_batch(conn, proto_input) -> Result<u64, Error>` function into two pieces:

1. **Builder:** `build_*_record_batch(proto_input) -> Result<Option<RecordBatch>, Error>` — does the proto → Vec<column-wise> → RecordBatch::try_new construction; returns `None` for zero-row inputs (matches existing semantic of "0 rows = no-op").
2. **Persister:** `append_record_batch_to_table(conn, table_name: &'static str, batch: RecordBatch) -> Result<u64, Error>` — does the DuckDB `appender(table_name).append_record_batch(batch).flush()` work; returns row count from `batch.num_rows()`.

The original `append_*_batch` becomes a thin compose layer (`build → append → tracing log`). Caller for chunk #23-style fan-out flows (`crates/buffer/src/consumer.rs::dispatch_batch`) bypasses the wrapper entirely: build once, encode for broadcast (via `crate::broadcast::encode_*(&record_batch)`), append the (cloned) RecordBatch to DuckDB, emit broadcast bytes if encode-Ok and append-Ok.

Coordination invariant: encode happens BEFORE append (so encode failure aborts the whole flow), but emit happens AFTER append (so subscribers only see durably-stored data). RecordBatch::clone is cheap (Arc bump on the underlying buffers), so the build → clone → append + clone → encode pattern is roughly O(1) extra overhead.

Side effect: with the production path going through builders + writer directly, the old wrappers `append_*_batch` may become unused in production code (only the co-located tests still call them). See the cfg(test) gating learning below for the workflow follow-up.

See: `crates/buffer/src/appender.rs::build_spans_record_batch / build_metrics_record_batch / build_logs_record_batch / append_record_batch_to_table`; `crates/buffer/src/consumer.rs::dispatch_batch` chunk #23 fan-out path.

---

## 2026-05-06 — `#[cfg(test)]` gating of test-only API wrappers after refactor (dead-code under -D warnings)

When extracting a public-API function into helpers + a thin wrapper, the wrapper may end up unused by production code (only co-located tests call it). Rust's `dead_code` lint will fire, and clippy's `-D warnings` gate will reject the build. Solution: gate the wrapper with `#[cfg(test)]`. The wrapper preserves existing test ergonomics + signature; production path bypasses it via the helpers.

Same gating applies to imports newly needed only in test paths. The chunk #23 buffer/appender refactor moved `Instant::now()` calls from the production wrappers into cfg(test)-only territory; the `use std::time::Instant` import then needed `#[cfg(test)]` too:

```rust
use std::sync::Arc;
#[cfg(test)]
use std::time::Instant;
```

Diagnostic shape: `warning: function 'append_spans_batch' is never used` + `warning: unused import: 'std::time::Instant'`. Without gating, both fire as warnings under default rustc, which clippy promotes to errors via `-D warnings`.

Pattern generalizes to refactor-time discipline: when extracting helpers from existing API, audit whether the OLD entry-point (and its imports) is still called from production. If only tests call it, gate with `#[cfg(test)]`. If genuinely unused (no callers anywhere), delete it outright (per CLAUDE.md "no half-finished implementations / TODO panics" guidance) — keeping it cfg(test)-gated is the right move only if tests legitimately need the compose layer.

See: `crates/buffer/src/appender.rs` chunk #23 — `append_{spans,metrics,logs}_batch` wrappers cfg(test)-gated after extraction; `Instant` import gated; chunk #23 fix-loop iteration #2.

---

## 2026-05-06 — TauRPC + tokio broadcast + Tauri Channel API binary-payload forwarding pattern

The chunk #23 push-stream surface (`pulse://stream/{spans,metrics,logs}`) wires three components:

1. **`tokio::sync::broadcast::Sender<bytes::Bytes>`** in the producer crate (buffer): one Sender per stream, capacity 128. After successful DuckDB append, encode the RecordBatch via `arrow::ipc::writer::StreamWriter` to a `bytes::Bytes` payload (with 8 MB cap check), then call `senders.{spans|metrics|logs}.send(bytes)`. SendError when no subscribers — silently drop via `let _ = sender.send(...)`.
2. **TauRPC `#[taurpc::procedures(path = "streams")]`** in the binary crate (`pulse-app/src/streams.rs`) with 3 procedures `subscribe_{spans,metrics,logs}(channel: tauri::ipc::Channel<Vec<u8>>) -> Result<(), AppError>`. Tauri 2.11 + taurpc 0.7 accepts `Channel<Vec<u8>>` as a procedure parameter without special handling; webview creates a Channel via `new Channel<Uint8Array>()`, passes it as the procedure arg, and the procedure stores the handle.
3. **Forwarding task** spawned at procedure entry: clone the relevant `broadcast::Sender`, call `.subscribe()` to get a `Receiver`, then `tokio::spawn(forward_loop(stream_name, receiver, channel))`. The loop: `match receiver.recv().await { Ok(bytes) => { /* size cap re-check, payload = bytes.to_vec(), channel.send(payload), tracing::info! tauri.channel.emit */ }, Err(Lagged(n)) => tracing::warn! tauri.channel.lag, Err(Closed) => break }`. Channel send error (webview disconnect) → break loop, exit task, drop Receiver, decrement subscriber count via `Sender::receiver_count()` natural decay.

Two notable trip-ups during impl:

- **`bytes::Bytes` does NOT implement Serialize**, so `Channel<bytes::Bytes>` doesn't compile. Use `Channel<Vec<u8>>` and convert via `bytes.to_vec()` at the send site. Trade-off: one Vec allocation per emission per subscriber. For 3 subscribers × 10k events/sec ≈ 30k allocs/sec — acceptable within tokio scheduling overhead headroom; revisit only if profiling shows hot-path cost.
- **Subscriber count tracking** lives in `IngestState.broadcast_subscribers` (chunk #18 precedent — single AtomicU32 representing total across streams). Per-tick heartbeat polls `broadcast_senders.{spans|metrics|logs}.receiver_count()` and sums into `IngestState.set_broadcast_subscribers(total_subs as u32)` before emitting `ingest.tick`. Per-stream visibility achieved via separate `metric.ingest.channel.broadcast_subscribers` events with enumerated `channel_name` field — does NOT use unbounded labels per obs cardinality discipline.

Pattern is reusable for any future scope-arch chunk that needs binary push from backend to webview without JSON-stringify tax. Avoid `tauri::Manager::emit(event_name, payload)` for bulk binary data — emit serializes to JSON regardless of T (Vec<u8> becomes a JSON array of u8s).

See: `pulse-app/src/streams.rs` (TauRPC trait + StreamsApiImpl + forward_loop); `crates/buffer/src/broadcast.rs` (Sender trio + encoders + cap); `pulse-app/src/heartbeat.rs::emit_ingest_tick` (subscriber-count poll); chunk #23 plan.md §Implementation Steps 8 + research.md "Open questions".

---

## 2026-05-06 — TauRPC trait+impl pairs belong in the binary crate, not in producer library crates (cargo feature-unification cycle)

When a TauRPC API is exposed by a library crate's content (viz query types + Error, future scope-crate types, etc.), the natural Rust instinct is to colocate the `#[taurpc::procedures] pub trait Api` + `#[taurpc::resolvers] impl Api for ApiImpl` with the data types in the same library crate. This works for `ui-bridge` because ui-bridge OWNS the `AppError` type that procedures return — the trait can be feature-gated and reference AppError directly (see `crates/ui-bridge/src/health.rs::runtime` mod under `#[cfg(feature = "taurpc-runtime")]`).

For peer library crates (viz, future scope crates), procedures still must return `Result<T, AppError>` per arch §Conventions. AppError lives in ui-bridge. So the producer's runtime module needs ui-bridge as a dep. Meanwhile ui-bridge already depends on the producer for `From<ProducerError> for AppError` (per the chunk #18 sibling-dep precedent already documented in this file). This APPEARS solvable via cargo features:

- viz declares feature `taurpc-runtime` → activates optional dep on `ui-bridge`
- ui-bridge → declares dep on `viz` with `default-features = false` (no `taurpc-runtime` active)

Cargo's feature unification breaks this: when pulse-app activates viz's `taurpc-runtime` feature, the unification rule requires EVERY copy of viz across the workspace to share the same feature set. ui-bridge's viz copy thus also gets `taurpc-runtime` active → that viz copy depends on ui-bridge → ui-bridge depends on viz-with-`taurpc-runtime` → CYCLE. Cargo rejects.

Resolution (chunk #22 implement-time deviation from plan): place the TauRPC trait+impl pairs in the binary crate at `pulse-app/src/{name}_routers.rs`. The binary crate already depends on every library crate; the routers module imports types from the producer crate (`use viz::{TracesQueryArgs, ...}`) and constructs the impl with the orchestration handles (`Arc<Mutex<Connection>>` + `Arc<ProducerState>`) the binary already holds. Producer crate stays cycle-free with no `taurpc-runtime` feature. ui-bridge keeps the unconditional `From<ProducerError> for AppError` impl. Mirrors how `pulse-app/src/main.rs` already orchestrates the existing HealthApi (which colocates with its data types in ui-bridge — colocation works for ui-bridge specifically because ui-bridge owns AppError).

Future scope-arch additions of new TauRPC routers in producer crates should default to placing trait+impl in pulse-app from the start, NOT in the producer crate behind a `taurpc-runtime` feature. Plan templates that propose feature-gated cycles need an implement-time verification step (cargo check the workspace under both feature configurations) before assuming cargo will resolve.

See: `pulse-app/src/viz_routers.rs` (TracesApi/MetricsApi/LogsApi triplet); chunk #22 plan.md step 5 (planned `crates/viz/src/runtime.rs`) deviated to actual `pulse-app/src/viz_routers.rs`; sibling pattern at `crates/ui-bridge/src/health.rs::runtime` works ONLY because ui-bridge owns AppError.

---

## 2026-05-06 — taurpc::procedures macro needs serde + specta crates at the call-site crate, plus specta::Type on every touched type

The `#[taurpc::procedures(path = "...")]` attribute macro (taurpc 0.7) emits code that references `taurpc::serde::Serialize`, `specta::Type`, and `specta::function::specta_fn::SpectaFn` directly by path. At macro expansion, these paths resolve via the call-site crate's `[dependencies]` — having `taurpc` in `[dependencies]` is NOT enough. The compiler errors are misleading because they point at the attribute macro line, not the missing dep:

- `error[E0463]: can't find crate for `serde`` (note: `this error originates in the derive macro `taurpc::serde::Serialize``) → add `serde.workspace = true` to the call-site crate
- `error[E0433]: cannot find module or crate `specta``  → add `specta.workspace = true` to the call-site crate
- `error[E0277]: the trait bound `MyType: specta::Type` is not satisfied` (note: `required for `MyType` to implement `FunctionArg``) → derive `specta::Type` on every argument and result type of every procedure

For pulse-app (the binary crate that hosts TauRPC trait+impl pairs per the cycle-break pattern), this meant adding `serde.workspace = true` + `specta.workspace = true` to `[dependencies]` even though pulse-app doesn't directly use the `serde` or `specta` types — they're invoked entirely via taurpc's emitted macro paths.

For producer library crates (viz, future scope crates), every public type crossing a TauRPC procedure surface must derive `specta::Type` — typically alongside `serde::{Serialize, Deserialize}`. Generic wrapper types like `PaginatedResponse<T>` need `T: specta::Type` bound on the type parameter (Rust derives this automatically via `#[derive(specta::Type)]` on the generic struct, but the bound becomes part of the public API — every concrete instantiation must satisfy it).

Two derive strategies, both seen in this workspace:

- **Conditional (ui-bridge precedent):** `#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]` — gates the derive to feature-active builds. Used by `HealthEnvelope`/`HealthStatus`/`SubsystemStatus`. Useful when the type is occasionally used outside taurpc contexts and the specta dep cost is unwanted in those builds.
- **Unconditional (chunk #22 viz precedent):** `#[derive(serde::Serialize, serde::Deserialize, specta::Type)]` always. Simpler when the producer crate has no `taurpc-runtime` feature (because trait+impl lives in pulse-app per the cycle-break pattern). Cost: specta becomes a hard dep of viz and any crate that depends on viz.

Choose conditional when the producer crate may be reused in non-taurpc contexts (mcp-server hypothetically, or stdlib-only consumers). Choose unconditional when the producer is in this workspace's pure TauRPC-IPC pipeline only.

See: `crates/viz/src/query.rs` derives (TracesQueryArgs/MetricsQueryArgs/LogsQueryArgs/PaginatedResponse/TraceRow/MetricRow/LogRow); `pulse-app/Cargo.toml` `[dependencies] serde.workspace = true; specta.workspace = true`; chunk #22 fix-loop iterations 2 + 3.

---

## 2026-05-06 — Extract async helper from periodic-task loop body for unit-testability

When an async task wraps a periodic loop with `tokio::time::interval(...).tick().await` + `tokio::task::spawn_blocking(...)` calls inside, unit tests using `tokio::time::pause()` + `tokio::time::advance()` reliably race with the spawn_blocking thread + the test's `handle.abort()`. The chunk #21 retention task hit this: `run_retention(conn, state, retention_seconds)` spawned blocking DuckDB DELETE work whose completion didn't reliably reach the `state.record_eviction(rows)` call before the abort fired, leaving `state.eviction_count = 0` in tests despite rows being physically evicted.

Resolution: extract the loop body (one tick worth of work) into a separately-callable async helper. For chunk #21:

```rust
pub async fn run_retention(conn, state, retention_seconds) {
    let mut interval = tokio::time::interval(...);
    interval.tick().await;  // skip immediate first tick
    loop {
        interval.tick().await;
        run_one_sweep(&conn, &state, retention_seconds).await;
    }
}

pub(crate) async fn run_one_sweep(conn, state, retention_seconds) {
    // spawn_blocking + state updates + tracing — full sweep deterministic on `.await`
}
```

Tests then call `run_one_sweep(&conn, &state, 60).await` directly — no paused clock, no interval orchestration, no abort race. The behavior is exactly one sweep + state record + tracing event, which is what the test wants to verify. The smoke test `run_retention_can_be_spawned_and_aborted_cleanly` covers the wrapper-loop's spawn/abort lifecycle as a separate concern.

Pattern generalizes to any async task that wraps a periodic body. The `pub(crate)` visibility on `run_one_sweep` keeps the abstraction from leaking into the public surface while still being testable from co-located `mod tests`.

See: `crates/buffer/src/retention.rs::run_one_sweep`; chunk #21 fix-loop iteration #1.

---

## 2026-05-06 — `memory_bytes` heuristic: `rows_active * 256` over `pragma_database_size()` parsing

DuckDB's `pragma_database_size()` returns multiple columns (`database_name`, `database_size`, `block_size`, `total_blocks`, `used_blocks`, `free_blocks`, `wal_size`, `memory_usage`, `memory_limit`) where the size-shaped columns (`database_size`, `wal_size`, `memory_usage`, `memory_limit`) are STRINGS like `"0 bytes"`, `"1.2 KiB"`, `"1.0 GiB"`. Parsing them requires unit-string matching (KiB / MiB / GiB / TiB) and float-to-bytes conversion. For a `memory_bytes` heartbeat gauge tracked at 15s cadence, the parse cost + the inherent imprecision of the human-readable formatting argues for a simpler heuristic.

Chunk #21 chose: `memory_bytes = (rows_ingested - eviction_count) * 256` where 256 is an empirical bytes-per-row estimate (composite BLOB PK + 2 timestamp columns + a few attribute columns averages around this range across the 7 reserved tables). This is monotonic with row count, requires no DuckDB pragma parse, and tracks well-enough with actual buffer memory for SLO purposes (the `metric.buffer.memory_bytes` ≤ 512 MB SLO is a coarse upper bound, not a precise accounting).

If a future need surfaces precise byte accounting (e.g., chunk-level memory profiling for performance regression CI), revisit by parsing `pragma_database_size().memory_usage` — but expect to invest in a unit-string parser that handles all DuckDB-emitted size formats.

See: `crates/buffer/src/retention.rs::run_one_sweep` (`set_memory_bytes(rows_active.saturating_mul(BYTES_PER_ROW_ESTIMATE))`); chunk #21 plan §Implementation notes "memory_bytes measurement source".

---

## 2026-05-06 — DuckDB 1.10502 hangs `INSERT` on duplicate composite-BLOB primary key

The `duckdb` crate 1.10502.0 (DuckDB 1.5 bundled C++) on Windows MSVC enters an unbounded loop when an `INSERT` would violate a `PRIMARY KEY (col1 BLOB, col2 BLOB)` composite. The first insert succeeds; the duplicate insert never returns from `Connection::execute()` / `execute_batch()` — observed via cargo nextest's `SLOW [>2400.000s]` reports on the chunk #20 buffer crate `spans` table (composite PK over `trace_id BLOB(16)` + `span_id BLOB(8)`). Single-INSERT into the same table works fine, both via SQL `INSERT … VALUES (X'…')` and via the Arrow appender (`Connection::appender("spans")?.append_record_batch(...)`). Only the PK-violation path on multi-column BLOB PK hangs. Workaround: assert composite-PK structure via schema introspection (`information_schema.key_column_usage` filtered to `table_schema = 'main' AND table_name = '<table>'`, asserting `column_name` set + `ordinal_position` count) instead of behavioral runtime PK-violation tests.

The chunk #20 schema test `spans_primary_key_is_composite_trace_id_span_id` originally inserted-then-duplicated; rewritten to query `key_column_usage` and assert (a) exactly 2 columns in PK, (b) names contain `trace_id` AND `span_id`. Pattern generalizes to any future schema test that needs to assert composite PK on BLOB-typed columns: prefer information_schema introspection over behavioral PK-violation paths until DuckDB upstream confirms / fixes the issue. Rust `cargo nextest` reports SLOW indefinitely without timeout; use cargo nextest's per-test slow-timeout config or kill the test binary manually. Direct `target/debug/deps/buffer-{hash}.exe` invocation reproduces the hang outside nextest, ruling out test-runner parallelism as cause.

See: `crates/buffer/src/schema.rs::tests::spans_primary_key_is_composite_trace_id_span_id`; `information_schema.key_column_usage` filter discipline; chunk #20 fix-loop iteration #6.

---

## 2026-05-06 — libduckdb-sys 1.10502 needs `rstrtmgr.lib` link hint on Windows MSVC

The `libduckdb-sys` crate 1.10502.0 (DuckDB C++ build) on the `x86_64-pc-windows-msvc` target references Restart Manager APIs (`RmStartSession` / `RmEndSession` / `RmRegisterResources` / `RmGetList` from `Rstrtmgr.dll`) inside `duckdb::AdditionalLockInfo` but does NOT emit the corresponding `rstrtmgr.lib` link directive from its own `build.rs` for downstream test-binary linkage. Linking the consumer crate's lib succeeds (the symbols stay unresolved-but-tolerated until binary link), but the test binary link step fails with `LNK2019 unresolved external symbol` for all four symbols. Workaround: add a `build.rs` to the consuming crate that emits `cargo:rustc-link-lib=dylib=rstrtmgr` when `CARGO_CFG_TARGET_OS == "windows"`. The chunk #20 buffer crate ships `crates/buffer/build.rs` with exactly this guard.

Pattern generalizes to any future workspace crate that takes `duckdb` (or any libduckdb-sys-bundled dep) as a direct or transitive dep with bundled C++ on Windows MSVC. The Linux + macOS targets do not need this — Restart Manager is Windows-specific. Diagnostic shape: `error: linking with link.exe failed: exit code: 1120` followed by `LNK2019 unresolved external symbol Rm{Start|End|RegisterResources|GetList}Session`. If a future libduckdb-sys version fixes its own `build.rs` to emit the link directive (would manifest as `print-cargo:rustc-link-lib=dylib=rstrtmgr` in `cargo build -vv` for libduckdb-sys), the workaround can be removed.

See: `crates/buffer/build.rs`; chunk #20 fix-loop iteration #5.

---

## 2026-05-06 — DuckDB Arrow-appended BLOB does not match `WHERE col = X'…'` hex literal

When the `duckdb` crate Arrow appender (`Connection::appender("table")?.append_record_batch(record_batch)?`) inserts a `BLOB` column from an `arrow::array::BinaryArray`, the resulting stored bytes do NOT match a `WHERE col = X'…'` hex BLOB literal in subsequent SELECT queries — the SELECT returns `QueryReturnedNoRows` even though `SELECT COUNT(*) FROM table` reports the row IS present. SQL-INSERT'd BLOB literals (`INSERT … VALUES (X'…', …)`) and SELECT WHERE hex-literal pairings DO match each other; Arrow-appended BLOB and hex-literal SELECT do NOT. Root cause unverified but consistent with the Arrow → DuckDB BLOB conversion using a different internal storage encoding (e.g., length-prefixed inline vs out-of-line variable-length representation) that the hex-literal-based equality check doesn't normalize across.

Workaround for round-trip tests: don't use `WHERE col = X'…'` on Arrow-appended BLOBs. Read back via `SELECT col, … FROM table ORDER BY ts_unix_nano LIMIT 1` (or LIMIT N + collect rows) and assert on the OTHER columns (timestamps, integer IDs, varchar names). Equality via parameter binding (`stmt.query_row(params![&[u8]_slice], …)`) was NOT tested as workaround — separately known to hang per the chunk #20 PK-on-BLOB issue, so it can't isolate the encoding question. The chunk #20 `append_spans_batch_round_trips_nanosecond_precision` test uses LIMIT 1 + `row.get::<_, i64>(0)` for `ts_unix_nano` exactly because of this constraint.

Implication: any future query router (chunk #22+) that needs to filter spans by `trace_id` BLOB (e.g., `traces.query_by_trace_id`) must validate Arrow-appended BLOBs match the parameter-binding path before assuming `WHERE col = ?` works. Likely the proper path is `WHERE col = CAST(? AS BLOB)` or DuckDB's specific BLOB binding in the duckdb crate's prepared-statement API. Plan chunk #22 acceptance criteria should explicitly probe this before relying on parameterized BLOB queries.

See: `crates/buffer/src/appender.rs::tests::append_spans_batch_round_trips_nanosecond_precision`; chunk #20 fix-loop iteration #7.

---

## 2026-05-05 — `governor` crate uses real-time clock; not mockable via `tokio::time::pause()`

The `governor` crate (used transitively by `tower_governor` 0.8 for OTLP receiver rate limiting per route#19) uses a `quanta`-backed monotonic clock (`governor::clock::DefaultClock` → `QuantaInstant`) for token-bucket replenishment. This clock is independent of tokio's runtime clock; calling `tokio::time::pause()` + `tokio::time::advance(Duration)` does NOT freeze or fast-forward governor's view of time. Rate-limit window assertions therefore cannot use the testing.md "use `tokio::time::pause()` for time-sensitive tests" pattern — saturation/recovery tests must use real-time short sleep with bounded windows.

The chunk #19 `crates/ingest/tests/rate_limit.rs` integration tests use `TIGHT_PERIOD = Duration::from_millis(100)` + `TIGHT_BURST_SIZE = 2` + `RECOVERY_WAIT = Duration::from_millis(250)` — total real-time wall cost ~250-400ms per test, comfortably bounded. The testing.md "NEVER `sleep(N)` for sync" rule applies to event-waiting synchronization (poll for state change); time-elapsed-behavior testing on a real-clock-backed library is a distinct use case where real time IS the canonical signal. Document the deviation in the test file's module docstring; do NOT add the testing.md rule's `tokio = { features = ["test-util"] }` dev-dep just for governor tests — the feature flag wouldn't help.

If a future external middleware library exposes a `Clock` trait or `governor::clock::FakeRelativeClock` becomes accessible through `tower_governor`'s public API, prefer that path; until then, real-time bounded windows are the working pattern. Pattern generalizes to any future timing test against a non-tokio-clock library.

---

## 2026-05-05 — Sibling-isolation grep gates over-specified when permitted DAG edge exists

Plan acceptance criteria of the form `cargo tree -p {sibling_crate} | grep {dep} returns empty` are too coarse when the workspace has a permitted sibling-DAG edge. Concrete case (chunk #19): the criterion `cargo tree -p ui-bridge | grep tower_governor returns empty` was unachievable given the existing `ui-bridge → ingest` sibling dep edge (chunk #18's `From<IngestError> for AppError` impl in `crates/ui-bridge/src/contract.rs` per arch §Conventions Error response schema (Tauri IPC) From-impl-as-contract). Since `ingest` carries `tower_governor` as a direct dep, the whole-tree grep MUST match transitively through ui-bridge → ingest → tower_governor.

The intent (no DIRECT tower_governor dep on ui-bridge) is captured better by:
- `cargo tree -p ui-bridge --depth 1 | grep tower_governor` returns empty (only direct deps), OR
- `grep tower_governor crates/ui-bridge/Cargo.toml` returns empty (declaration check).

Both succeed for chunk #19's actual implementation (tower_governor declared only in `crates/ingest/Cargo.toml`). When future plans assert sibling-isolation, prefer one of these forms. The whole-tree grep is appropriate ONLY when the sibling pair has NO permitted dep edge between them. Document the edge in plan.md "Files к leave untouched" or research.md "Conventions to follow" section to make the constraint visible at planning time.

---

## 2026-05-05 — `tokio::sync::mpsc::Sender::capacity()` returns FREE slots, not used

The Tokio mpsc bounded-channel `Sender::capacity()` method returns the number of currently-available slots (free count), NOT the number of queued messages (used count). This is opposite of what most "capacity" mental models suggest — a bounded channel built with `mpsc::channel(1024)` reports `capacity() == 1024` when empty and `capacity() == 0` when full. To compute "% used" for instrumentation (the obs-plan §3 `buffer_capacity_pct` field on the `ingest.tick` heartbeat carries this), the formula is `(total - sender.capacity()) / total * 100.0`, where `total` is the original constructor argument (NOT exposed by the Sender directly — must be tracked by the caller). The chunk #18 `IngestSender` wrapper at `crates/ingest/src/channel.rs` stores the constructor capacity alongside the inner sender exactly because the Tokio API doesn't surface it; without that snapshot, capacity_pct calculation is impossible.

Implication for future channel-introspection code: any wrapper around `tokio::sync::mpsc::Sender` that wants to report "fullness" must capture the constructor capacity at build time. `Sender::max_capacity()` does NOT exist on stable as of tokio 1.x; only `capacity()` (free) and `len()`-style methods on the receiver side exist. The wrapper-with-snapshot pattern from `ingest::channel::IngestSender` generalizes to any future bounded mpsc that needs introspection.

See: `crates/ingest/src/channel.rs::IngestSender::capacity_pct`; tokio docs `tokio::sync::mpsc::Sender::capacity` (returns free, not used).

---

## 2026-05-05 — ui-bridge → ingest sibling crate dep is permitted because the From impl IS the declared contract

Arch §Cross-cutting Patterns "Module dependency direction" states the workspace dep graph is a DAG with `pulse-app` as the only root, AND "no library crate depends on a sibling unless its declared contract requires it". Chunk #18 introduced `ingest = { path = "../ingest" }` to `crates/ui-bridge/Cargo.toml` — the only sibling-crate edge in the workspace as of session 18. The justification: `From<ingest::contract::Error> for AppError` impl lives in `crates/ui-bridge/src/contract.rs` because arch §Conventions "Error response schema (Tauri IPC)" mandates that `From` impls collapsing module-internal `thiserror` enums to `serde`-friendly `AppError` variants live in the bridge crate (where `AppError` is owned). The From impl IS the declared contract that the dependency edge serves; without it, ui-bridge cannot perform the boundary conversion `pulse-app/src/main.rs` (and future TauRPC procedure call-sites) need.

Future-self gotcha when reading `crates/ui-bridge/Cargo.toml` and wondering "wait, why does ui-bridge depend on ingest?" — the answer is the From impl. The same pattern would apply if/when `From<buffer::Error> for AppError` or `From<viz::Error> for AppError` becomes necessary (Epoch 3 buffer chunk lands a similar impl). Each new module-error-to-AppError conversion adds a sibling-dep edge from ui-bridge to that module's crate; the DAG-discipline language permits this as "declared contract" exception.

See: `crates/ui-bridge/Cargo.toml` `[dependencies] ingest = { path = "../ingest" }`; `crates/ui-bridge/src/contract.rs::From<IngestError> for AppError`; arch.md §Cross-cutting Patterns + §Conventions "Error response schema (Tauri IPC)".

---

## 2026-05-05 — axum 0.8 + tonic 0.14 share tower 0.5 + hyper 1 cleanly (no transitive deny duplicate)

When chunk #17 introduced `axum = "0.8"` + `tower = "0.5"` + `tower-http = "0.6"` alongside the existing `tonic = "0.14"` + `tokio-stream` + `tonic-prost` ingest stack, the expected risk was that `cargo deny check bans` (`multiple-versions = "deny"`) would fire on a transitive `tower 0.4 vs 0.5` or `hyper 0.14 vs 1` duplicate. It did not — the resolved dep graph contains exactly one `tower 0.5` + one `hyper 1` + one `http 1` shared across both receivers. axum 0.8 and tonic 0.14 are version-aligned by design (both target hyper 1 + tower 0.5 + http 1 simultaneously). The pre-existing `deny.toml [bans] skip` list (with the chunk #16 `foldhash` provenance entry) did not need extension for chunk #17.

Implication for future Epoch 2-4 chunks: when adding HTTP/web infrastructure crates that need to coexist with the OTLP/gRPC stack, prefer versions that target hyper 1 + tower 0.5 + http 1 to maintain this clean unification. The `tonic <0.14` deny canary at `deny.toml [bans] deny` continues to enforce the original OTLP-receiver invariant — that line is the canonical anchor for "we use the tonic 0.14 + hyper 1 + tower 0.5 stack only".

Note: `cargo check` output during chunk #17 showed `Checking reqwest v0.13.3` AND `Checking reqwest v0.12.28` (12.x added directly as dev-dep for HTTP integration tests; 13.x pulled transitively by tauri-plugin-updater 2.10's HTTP client). `cargo deny check bans` did NOT fire — the resolver appears to have a tolerance carve-out for dev-dep duplicates that don't enter the production binary's link graph (or the duplicate is benign for this skip-list configuration). No action required.

See: `Cargo.toml` `[workspace.dependencies]` Ingest pipeline + OTLP HTTP receiver sections (route#16 + route#17 dep blocks); `deny.toml` `[bans] deny tonic <0.14` canary (security plan §Dependency Security Pinning).

---

## 2026-05-05 — `axum::Router::layer` chains apply outermost-LAST (each .layer() call wraps the previous)

`Router::new().route(...).layer(L1).layer(L2).layer(L3)` produces a service stack where on the request side, L3 runs first (outermost), then L2, then L1, then the handler; on the response side, the reverse. Each `.layer()` call WRAPS the previous layer, so the LAST `.layer()` chained becomes the OUTERMOST middleware. Without understanding this, middleware ordering goes wrong — e.g., placing `DefaultBodyLimit` BEFORE the Host-header allowlist in code-order means the body-limit check runs INSIDE (closer to handler) and the host check runs OUTSIDE (rejects first). The intuitive reading is reversed.

For the OTLP HTTP receiver at `crates/ingest/src/http.rs::build_router`, the desired security ordering is: tracing instrumentation outermost (so all rejected requests still emit boundary spans for observability), then Host-header allowlist (reject DNS-rebinding attempts before body read), then DefaultBodyLimit (reject oversize bodies before parsing — the JFrog axum-core advisory anchor), then CORS default-deny innermost. The matching code-order in build_router is:

```
.layer(CorsLayer::new())                  // innermost — applied first when entering
.layer(DefaultBodyLimit::max(8 * 1024 * 1024))  // wraps CORS
.layer(middleware::from_fn(host_header_check))  // wraps body-limit
.layer(TraceLayer::new_for_http())        // outermost — wraps everything
```

This is the inverse of how readers naturally scan the code, so worth documenting as a future-self gotcha. The pattern matches `tower::ServiceBuilder` (which chains layers in semantic outer-to-inner order via `.layer()` calls; same trap, different syntax).

Implication for future axum middleware additions: when adding a new layer, check the ordering by tracing one request through: which layer should run first → put it LAST in the `.layer()` chain. Add a brief code comment if ordering matters semantically (e.g., "// security: host check before body parse to short-circuit DNS rebinding").

See: `crates/ingest/src/http.rs::build_router` (chunk #17 layer stack); axum 0.8 docs `Router::layer` semantics; `tower::ServiceBuilder` ordering (same convention).

---

## 2026-05-04 — tonic 0.14 split `prost` integration into separate `tonic-prost` crate

The `tonic = "0.13"` legacy pattern bundled prost message support into the main `tonic` crate via the `prost` feature. `tonic = "0.14"` removed that feature — the available features are `_tls-any, channel, codegen, default, deflate, gzip, router, server, tls-aws-lc, tls-native-roots, tls-ring, tls-webpki-roots, transport, zstd` (no `prost`). Adding `tonic = { version = "0.14", features = ["prost"] }` errors with `package 'ingest' depends on 'tonic' with feature 'prost' but 'tonic' does not have that feature.` The migration: depend on `tonic-prost = "0.14"` separately for the `ProstCodec` runtime + change feature set to `["transport", "router", "server", "codegen"]` (or whatever subset needed). Same story for build dependencies: `tonic-build = "0.14"` is the general gRPC service codegen crate; `tonic-prost-build = "0.14"` is the prost-message codegen crate — both are required when invoking `tonic_prost_build::configure().compile_protos(...)` from `build.rs`. The `tonic-prost-build` crate also pulls in `prost-build` 0.14 transitively, which requires `protoc` on PATH (or a vendored binary via `protoc-bin-vendored = "3"`).

This split is part of the broader tonic 0.14 modularization (see also `tonic-types`, `tonic-reflection`, `tonic-health` as separate crates). Future Rust crates in this project that consume tonic should reference the workspace dep set committed at chunk #16: `tonic.workspace = true` + `tonic-prost.workspace = true` for runtime; `tonic-build.workspace = true` + `tonic-prost-build.workspace = true` + `protoc-bin-vendored.workspace = true` for build-deps.

See: `crates/ingest/Cargo.toml` `[dependencies]` + `[build-dependencies]`; `crates/ingest/build.rs` (codegen invocation + vendored-protoc setup); workspace `Cargo.toml` `[workspace.dependencies]` Ingest pipeline section.

---

## 2026-05-04 — ESLint 9 flat config layered structure for pulse-app/ui

`pulse-app/ui/eslint.config.mjs` (created chunk #13) layers in this order: `ignores` block → `@eslint/js` `js.configs.recommended` → `typescript-eslint` `tseslint.configs.recommended` SPREAD with `...` (it's an ARRAY of configs, not a single object — common footgun) → files-scoped block extending `eslint-plugin-react` `flat.recommended.rules` + `eslint-plugin-react-hooks` (rules-of-hooks: error, exhaustive-deps: warn) + `eslint-plugin-jsx-a11y` `flatConfigs.recommended.rules` → final files-scoped block adding Node globals for `scripts/` + config files. `react/react-in-jsx-scope` is OFF (React 19 + JSX runtime `react-jsx` makes the rule obsolete).

Custom `<Icon glyph="..."/>` components in `pulse-app/ui/src/components/icons/` (chunk #11 deliverable, design-system §Iconography) MUST be scoped out of `jsx-a11y/alt-text` via `{ elements: ['img'], img: ['NextImage'] }` — the rule defaults check Image-named components and false-positive on the project's token-registered Icon registry; without scoping, `npm run lint` errors on every Icon usage. The Icon registry is a design-system convention (icons clarify, not decorate), not raster images.

Companion stack installed at chunk #13: `eslint@^9.x` + `typescript-eslint@^8.x` (metapackage with parser+plugin+configs) + `eslint-plugin-react@^7.37.0` + `eslint-plugin-react-hooks@^5.0.0` + `eslint-plugin-jsx-a11y@^6.10.0` + `globals@^15.0.0`. The chunk title's "7 a11y packages" abbreviation hides this 5-package ESLint companion expansion required because installing `eslint-plugin-jsx-a11y` without ESLint base + recommended-config extension is functionally inert (a11y-plan §11 anti-pattern). Pattern: when chunk titles abbreviate by ecosystem name, expect implicit-peer expansion in the implement scope; surface in plan.md scope-expansion disclosure at Phase 6 user review rather than discovering during /implement.

See: `pulse-app/ui/eslint.config.mjs` (canonical structure); `pulse-app/ui/package.json` devDependencies (companion stack); a11y-plan.md §11 anti-pattern banning lint-only-without-runtime; phase-10/plan.md "Implementation notes" §Scope-expansion disclosure.

---

## 2026-05-04 — Honest provenance principle for Andromeda state schemas

When adding a new field to a shared contract that tracks "which skill performed action X and when", the field's TYPE should match what the writing skill actually produces, not what the schema author imagined. The Iteration 1 spec-amendment-protocol designed `state.yaml.spec_amendments.active[].noted_by_run` and `archived_by_run` as path-strings on the assumption that every lifecycle stage maps to a run-dir. Iteration 2 first-cycle live test exposed the lie: `/andromeda-wrap-session` does NOT create run-dirs (unlike `/andromeda-phase` and `/andromeda-setup-project --delta` which DO). The synthetic path `/andromeda-runs/2026-05-03T23-22-08-wrap-session-11/` was fabricated to fit the schema; no such directory existed on disk. Renamed to `noted_at` and `archived_at` (ISO timestamps) in v2.1; `propagated_by_run` STAYS as path because setup-project --delta creates a real run-dir with materialization-plan-delta.md as audit trail.

Generalizable principle: before locking a schema field type, identify which skill writes it and ask "does that skill actually produce this artifact?" Path = real run-dir audit trail; timestamp = action happened but no separate forensic dir exists. Mismatch = schema dishonesty that papers over with synthetic identifiers — eventually forces ugly migration when the lie surfaces. Applies to drift_warnings (timestamps not paths because wrap-session writes them), curation summaries (counts not paths because curation runs in-place), commit metadata (sha not path because git creates the commit). The honest-provenance test: would the field value resolve to a real disk artifact? If no → use timestamp / count / enum string instead of path.

See: `~/.claude/skills/andromeda-{setup-project,wrap-session,new-session}/references/spec-amendment-protocol.md` Part B Validation §"Field types (NEW v2.1)" for the explicit enumeration.

---

## 2026-05-04 — PYTHONIOENCODING=utf-8 for Python stdout with Unicode on Windows

When running Python one-liners via `python -c '...print("✓ ok")...'` on Windows (Git Bash, cmd.exe, PowerShell), default stdout codec is cp1252 which cannot encode `✓` (U+2713), `✗` (U+2717), `→` (U+2192), `⚠` (U+26A0), `ℹ` (U+2139), or any non-Latin-1 character. The script silently runs the logic but throws `UnicodeEncodeError: 'charmap' codec can't encode character '✓'` at the print statement, masking the actual computation result. Verification scripts that print pass/fail badges with checkmarks die mid-output.

Discipline: prefix verification commands with `PYTHONIOENCODING=utf-8 python -c '...'` (Git Bash) or `$env:PYTHONIOENCODING="utf-8"; python -c '...'` (PowerShell). Alternative: use `sys.stdout.reconfigure(encoding='utf-8')` inside the script (Python 3.7+) but env-var prefix is less invasive for one-liners. The Bash tool inherits the env var per command. The same issue does NOT appear with module-imports or file-output (those default to UTF-8); only stdout to a Windows console.

Discovered while running final verification of Iteration 2 spec-amendment protocol (`yaml.safe_load` + `print` of state.yaml v2.1 fields with `✓`/`✗` badges); first run silently failed at print, masked the YAML-parse-success result behind a UnicodeEncodeError trace. Re-run with `PYTHONIOENCODING=utf-8` rendered cleanly.

See: any verification one-liner emitting Unicode badges (e.g., the 6-contract md5 + state.yaml YAML parse + cyrillic-grep verification triplet from session 12).

---

## 2026-05-03 — Spec-drift workflow formalized as 4-skill cross-cutting protocol

The Variant 3 ad-hoc workflow (manual upstream edit + setup-project rerun pragmatic delta) used in session 10 has been formalized as the **spec-amendment-protocol** spanning all 4 Andromeda skills. When a chunk's harness/test correctly detects a gap between a specialist plan declaration and implementation reality (NOT a code bug, NOT environmental, NOT pure out-of-scope), `/andromeda-implement` Phase 2 fix loop fires Trigger 4 — a soft-exit-to-propose dialogue presenting Path A (amend specialist plan), Path A' (fix implementation to match existing spec), or Path B (defer to handoff Deferred decisions). Path A' MUST be presented prominently to prevent default-amendment bias; sometimes the impl is wrong, not the spec. Path A discipline: orphan-grep verification (`git grep -- "<old-value>"` should return matches only in the Decisions Log entry); coupled-ref updates via Edit `replace_all`; Decisions Log entry with 6 required fields (Trigger / Change / Brand-or-domain impact / Usage scope refinement / Cross-references / Authority statement); marker file at `.andromeda/runs/{ISO}-spec-amendment-{slug}/amendment.md`; state.yaml.spec_amendments.active append. Architecture.md amendment is forbidden as a delta — force re-plan via `/andromeda-arch` (greenfield path). The lifecycle implement (applies) → wrap-session (notes + acks via Phase 6 self-heal + D5 amendment-aware classification) → setup-project --delta (propagates to Tier 2/3 distillations only; bypasses full re-derive) → wrap-session (auto-archives propagated entries) closes the loop. Schema bumped to state.yaml schema_version=2 with `spec_amendments: {active, archive}` field; v1 files migrate automatically on first wrap-session run. The 6th shared contract `spec-amendment-protocol.md` is byte-identical-distributed across the triangle (setup-project / wrap-session / new-session) and cross-referenced from `andromeda-implement/references/spec-drift-protocol.md`.

**Backfill caveat:** Chunk #12's amendment was applied ad-hoc during session 11 BEFORE the protocol existed. The retroactive backfill (`.andromeda/runs/2026-05-03T21-30-00-spec-amendment-lift-accent/amendment.md`) was an exceptional recovery path to make chunk #12 the first use case of the new protocol AND to ensure state.yaml accurately reflects history. **Future amendments authored via `/andromeda-implement` Trigger 4 (Path A) write the marker file + state.yaml entry automatically as part of `spec-drift-protocol.md` §A1-A8 discipline — no backfill needed.** Backfill remains a recognized recovery pattern for amendments applied via tools / processes outside Andromeda's Trigger 4 flow (e.g., direct user edits to specialist plans without invoking `/andromeda-implement`); when needed, replicate the chunk #12 backfill procedure: write the marker file at `.andromeda/runs/{ISO}-spec-amendment-{slug}/amendment.md` per Part A schema, append the entry to `state.yaml.spec_amendments.active` per Part B schema, then proceed through normal lifecycle (wrap-session notes → setup-project --delta propagates → wrap-session archives).

**v2.1 schema refinement (2026-05-04):** Field renames in state.yaml.spec_amendments.active reflect what each skill actually produces — wrap-session does NOT create run-dirs, so `noted_by_run` was renamed to `noted_at` (ISO timestamp); same for `archived_by_run` → `archived_at`. `propagated_by_run` STAYS as a path because setup-project --delta DOES create a real audit-trail run-dir with materialization-plan-delta.md. Honest provenance: each field's type now matches its source skill's actual output. Existing v2 entries migrate via wrap-session Phase 8 best-effort step (extract timestamp from `noted_by_run` path basename if present; else current timestamp).

**v2.1 grep-expansion (2026-05-04):** setup-project --delta no longer trusts marker `expected_propagation` blindly — Detection step 8 runs `LC_ALL=en_US.UTF-8 grep -rn -E '<old-value>' .claude/ CLAUDE.md` against each amendment's primary value(s) extracted from marker `Before → After`. Hits NOT in the marker's `expected_propagation` list are auto-added to delta scope as defense-in-depth. Chunk #12's first-cycle delta exposed this gap: marker listed 1 file, Setup grep found 2 additional files (design-tokens.md + a11y.md). Future amendments authored via Trigger 4 SHOULD grep all Tier 2/3 + CLAUDE.md when populating `expected_propagation`, but the grep-expansion safety net catches authoring oversight.

**v2.1 stale-drift escalation (2026-05-04):** drift_warnings entries gain `first_observed_session_count` + `last_observed_session_count` int fields tracking persistence across wraps. new-session Phase 7 escalates entries with `(current_session_count - first_observed) > 3` to ⚠⚠ rendering with imperative remediation language. Generic D5 carryovers (e.g., test-plan.md from session 10's pragmatic delta) no longer silently re-fire as identical noise; user gets a forced choice after 3 wraps: resolve or accept.

**v2.1 cyrillic check (2026-05-04):** setup-project Phase 8 Check 16 + wrap-session Phase 8 step 6 grep staged files for cyrillic homoglyphs OUTSIDE allowed sections (USER:* / Decisions Log / `## Key Decisions This Session` / code fences). Warning-not-fatal posture; surfaces in commit message body for user review. Built-in complement to the manual sed-based cleanup discipline that emerged ad-hoc in this same session.

**v2.1 SHA-fixup amend (2026-05-04):** wrap-session Phase 10 step 4 captures the new commit SHA post-`git commit` and amends state.yaml.last_completed_chunk.commit_sha from `"pending"` to the real short SHA. One-commit-per-wrap invariant preserved; closes the cosmetic chicken-and-egg lie that surfaced in chunk #12's first-cycle wrap (state.yaml read `commit_sha: pending` for a full session cycle until self-heal next wrap).

---

## 2026-05-03 — Cyrillic-mixing discipline when editing Andromeda skill files

The original Andromeda skill author writes English text with Russian-cyrillic prepositions interleaved (e.g., " к " replacing "to", " с " replacing "with", " в " replacing "in", " не " replacing "not", " без " replacing "without", " против " replacing "against", "Не " at sentence start replacing "Not"). When Claude edits or creates files in `~/.claude/skills/andromeda-*/`, it tends to propagate this style — agents reading the existing files mirror the pattern, leading to ever-more-mixed output. This makes the contracts harder to read for non-Russian speakers and creates orthographic noise. Discipline: post-edit, run `LC_ALL=en_US.UTF-8 grep -E '[а-яА-ЯёЁ]' <files>` to detect remaining cyrillic, then batch-replace via sed with a script handling both word-boundary cases (` к ` → ` to `) and edge cases (`-к-`, ` к$`, ` к.`, `(к `, etc.). The `LC_ALL=en_US.UTF-8` prefix is necessary on Git Bash on Windows where default locale doesn't handle UTF-8 properly (grep counts wrong otherwise). Single-letter Russian prepositions (к, с, в, а, и) are the most common offenders. The shared-contract distribution (`cp` to triangle dirs + `md5sum` verify) must happen AFTER the cleanup, not before, to ensure all 3 byte-identical copies share the cleaned content.

---

## 2026-05-03 — Manual upstream edit + /andromeda-setup-project rerun for minor specialist-plan additions (vs greenfield /andromeda-{specialist} rerun)

`/andromeda-tests`, `/andromeda-security`, etc. are greenfield-only — they regenerate the entire specialist plan from scratch via 7 parallel sub-agents. Using them for а one-line addition (e.g., "Vitest landed at chunk #11" к `test-plan.md`) is overkill: rewrites the plan content, risks losing manual Decisions Log entries, и may diverge from cross-plan binding contracts (obs-plan §3 ↔ tests-plan §3 5-command discipline; a11y-plan §3.5 ↔ tests-plan §9 CI gate; a11y-plan structured violation JSON byte-identical к obs-plan §6 schema).

The pragmatic alternative: **manually edit the specialist plan** + **run `/andromeda-setup-project`** to propagate downstream. Preserves всё manual content и keeps cross-plan bindings intact. The setup-project re-run then:

1. Backs up `CLAUDE.md` к `.claude/backup/CLAUDE.md.pre-setup-{ISO}.md`
2. Regenerates only the `GENERATED:setup:*` sections of CLAUDE.md (`USER:*` preserved; almost always byte-identical если только anti-patterns / pointer table sources changed, which а minor framework addition typically doesn't trigger)
3. Regenerates rule + doc files (preserves `## Session Additions`; updates content above where the changed upstream propagates)
4. Refreshes `state.yaml.plan_freshness.{name}_mtime` к match actual upstream mtime
5. Closes drift D5 (plan-to-CLAUDE.md mtime) + State J (specialist plan freshness mismatch) for the affected upstream

The skill mandates regenerating всё materialized artifacts in Phases 1-6, но for re-runs where most upstream content is unchanged, the regenerated content will be byte-identical к existing files (atomic writes are idempotent on content; git sees no diff). **Pragmatic delta-rerun discipline:** write only the files whose content semantically changed; capture full synthesis intent в `materialization-plan.md` (run dir audit trail) so the rerun is fully auditable even when its file-write delta is minimal.

**Applies к:** minor framework addition (Vitest landing at chunk #11 was the worked example — modified `test-plan.md` §1 surface table + §4 framework section + downstream `tests-summary.md` Test pyramid + `testing.md` Framework + `state.yaml.plan_freshness.tests_mtime`); single Decisions Log append; minor Stack version bump; single anti-pattern revision; decision-rationale clarification.

**Does NOT apply к:** fundamental tier change (Standard → Comprehensive); test framework swap (cargo test → criterion); major architectural decision (Tauri → Electron); auth library swap (no auth → OAuth); logging library swap. Those warrant the greenfield specialist rerun (`/andromeda-tests`, `/andromeda-arch`, etc.) с full sub-agent regeneration so cross-plan bindings re-derive correctly.

See: `.andromeda/runs/2026-05-03T20-26-49-setup-project/materialization-plan.md` (worked example for Vitest propagation, including rejected universal-warning candidates as audit trail); `.claude/rules/testing.md` Framework section (final propagated state); `.andromeda/test-plan.md` §1 surface table + §4 Framework (the upstream edits that triggered the rerun); chunk #11 wrap's drift D3 reading (initial flag + how it closed across two consecutive wrap-session passes).

---

## 2026-05-03 — Vite "asset doesn't exist at build time, will remain unchanged" warning is benign for chained-pipeline outputs

When `index.html` references а static asset by absolute URL (e.g., `<link rel="stylesheet" href="/tokens.css">`) AND the asset is generated by а separate build step that runs BEFORE Vite (in andromeda-pulse: `scripts/build.mjs` orchestrating Tailwind → Vite), Vite's HTML transform during `vite build` emits the warning:

> `/tokens.css doesn't exist at build time, it will remain unchanged to be resolved at runtime`

This is benign и expected: Vite scans the HTML for assets it needs к bundle (modules referenced by `<script type="module">` и `<link rel="modulepreload">`); for everything else (absolute-URL CSS / font / image links), Vite preserves the literal href string in the transformed `dist/index.html` и trusts that the asset will exist at runtime. In the chunk #11 build pipeline, Tailwind has already written `dist/tokens.css` BEFORE Vite reads `index.html` — but Vite's project-root scan (looking at `pulse-app/ui/tokens.css` и `pulse-app/ui/public/tokens.css`) doesn't find it там. The dist-side file IS the intended target; the warning fires because Vite checks the wrong locations.

Triage cost: easy к misinterpret as а build error during CI log inspection. Add к runbook / commit message context when the warning first appears so future maintainers don't chase phantom failures.

NOT applicable to: assets imported by ES modules (`import "./tokens.css"` in main.tsx — Vite would bundle them); assets in `public/` (Vite's publicDir copy pattern; warning doesn't fire because Vite knows к copy them); relative-path links (e.g., `<link href="./tokens.css">` — Vite tries к resolve relative paths through the module graph).

See: `pulse-app/ui/scripts/build.mjs` chunk #11 Vite invocation; `pulse-app/ui/index.html` `<link rel="stylesheet" href="/tokens.css">`; Vite's HTML asset handling docs.

---

## 2026-05-03 — Acceptance-criterion grep patterns over а directory tree match documentation as well as source

Plan acceptance criteria of the form `grep -rE '<animate' src/components/icons/` (intended к enforce "no SVG animation tags in component sources") will match BOTH `.tsx` source files AND `.md` documentation that mentions the banned pattern as а quoted reference (e.g., README explaining the ban). Encountered in chunk #11 implementation: `README.md` documenting "icon components MUST NOT include `<animate>`" caused the criterion grep к return matches even though no actual SVG animation tag was emitted by the components.

Two fixes:
1. **Scope the grep к source files only** — append filename glob filtering: `grep -rE '<animate' --include='*.tsx' --include='*.ts' src/components/icons/`. Cleanest; the criterion's intent is "no animation tags in component output". Use this when authoring future criterion grep patterns over directories that mix source + docs.
2. **Rephrase documentation к avoid the literal substring** — change README from "icon components MUST NOT include `<animate>`" к "icon components MUST NOT include the SVG animation elements `animate`, `animateTransform`, `animateMotion`, or `set`". Same meaning к а human reader; doesn't trigger the literal grep. Use when (a) the criterion is already executed in CI / wrap-session AND (b) documentation lives in the same directory tree as the source it's documenting.

General principle for future plan acceptance criteria authors: when writing `grep -rE PATTERN DIR/` over а directory that may contain README / API docs that quote the pattern itself, either scope the grep к source extensions OR document explicitly that the directory has both source + docs и the pattern must avoid the literal substring in docs. Otherwise the criterion has а silent false-positive surface.

See: `.andromeda/phases/phase-8/plan.md` Test Commands section grep array; `pulse-app/ui/src/components/icons/README.md` ("Motion deferral" section, post-rephrase form).

---

## 2026-05-03 — Node 24 `execFileSync` rejects npm `.cmd` shims on Windows (CVE-2024-27980 hardening)

Node.js since the CVE-2024-27980 batch (Node 18.18.1, 20.5.1, 21.0.0+, all 22.x / 23.x / 24.x) refuses to spawn `.cmd` / `.bat` files via `child_process.spawnSync` / `execFileSync` without `shell: true` — this prevents argument-injection via crafted .cmd path arguments. The error surface is opaque: `EINVAL` with `status: null`, `signal: null`, `stdout: undefined`, `stderr: undefined` — NOT a "file not found" or "permission denied" message that would point at the .cmd file directly. Easy to misdiagnose as a Tailwind / build-tool config error instead of a Node platform behavior.

Symptom in this project: `pulse-app/ui/scripts/build.mjs` initially used `execFileSync(node_modules/.bin/tailwindcss.cmd, [args], { stdio: 'inherit' })` — silent EINVAL crash. The `.cmd` wrapper is just `node ../../@tailwindcss/cli/dist/index.mjs %*` so the workaround is: bypass the wrapper and invoke node directly with the `.mjs` entry path. Snippet from build.mjs:

```js
const tailwindEntry = join(ROOT, "node_modules", "@tailwindcss", "cli", "dist", "index.mjs");
execFileSync(process.execPath, [tailwindEntry, "-i", SRC, "-o", OUT, "--minify"], {
  stdio: "inherit",
  cwd: ROOT,
});
```

Three viable fixes for any future build/test/util script that invokes npm-installed CLI tools from Node on Windows:
1. **Direct .mjs invocation** (used here) — read the `.cmd` wrapper to find the actual entry point under `node_modules/<pkg>/dist/<entry>.mjs`, call `node` on it. Cleanest; no shell semantics; deterministic argument quoting.
2. **`shell: true`** — `execFileSync(cmd, args, { shell: true })` lets cmd.exe interpret the argument list. Works but reintroduces shell-quoting concerns the CVE hardening was meant to prevent.
3. **`process.platform === 'win32'` switch** — branch to `.cmd` on Windows, dotless name on POSIX. Conceptually correct but requires `shell: true` on Windows anyway.

Does NOT apply to: `npm run <script>` from a terminal (that path goes through cmd.exe directly, not Node spawn), Bash invocation of `.cmd` from Git Bash (uses MSYS exec), or POSIX hosts (no `.cmd` wrapper exists; `node_modules/.bin/<name>` is a symlink to the `.mjs`/`.js` entry). Applies specifically to: Node-script-spawning-npm-CLI-tool on Windows.

See: `pulse-app/ui/scripts/build.mjs` (Tailwind v4 invocation pattern); CVE-2024-27980 advisory; `node_modules/.bin/tailwindcss.cmd` (the wrapper that reveals the actual entry path).

---

## 2026-05-03 — Workspace feature unification reactivates Tauri across all test binaries; Windows requires MSVC toolchain for `cargo nextest run --workspace`

The chunk #4 ui-bridge feature-gating fix (`default = ["taurpc-runtime"]` + xtask consumes with `default-features = false`) **only resolves single-package builds** (`cargo run -p xtask`, `cargo build -p xtask`). For workspace-wide test discovery (`cargo nextest run --workspace` — which is what the test-plan §3 5-command harness mandates), Cargo's **feature unification** reactivates `taurpc-runtime` across the entire build:

1. `pulse-app/Cargo.toml` declares `ui-bridge = { path = "../crates/ui-bridge" }` — without `default-features = false`, so pulse-app activates ui-bridge's `taurpc-runtime` feature.
2. Cargo unifies features across all workspace members during a workspace build → ui-bridge is built **once** with `taurpc-runtime` active.
3. That single ui-bridge rlib (linked against tauri / wry / webview2-com / tao) is consumed by every workspace member that depends on ui-bridge — including xtask, despite xtask's `default-features = false` declaration.
4. Result: xtask's **test binary** (built by `cargo nextest run --workspace`) links Tauri DLLs.

On Windows GNU rustup-toolchain hosts, the resulting test binaries fail at startup with `STATUS_ENTRYPOINT_NOT_FOUND` (0xC0000139). The root cause is **NOT a WebView2 DLL search path issue** — it's a GNU vs MSVC ABI mismatch in WinRT API-set linkage. Tauri's wry / tao / webview2-com crates expect MSVC calling conventions for some Windows API-set imports (`api-ms-win-core-winrt-error-l1-1-0.dll`, etc.). MingW GCC linker resolves these symbols, but the resulting binary's import table doesn't match the actual procs available in the system DLLs at runtime.

**Fix — local dev parity with CI**: switch rustup `default-host` to MSVC. Per-user setting in `~/.rustup/settings.toml`, **NOT in the repo** (`rust-toolchain.toml` continues to pin `channel = "1.95.0"` which now resolves to the MSVC variant on this host).

Prerequisites:

1. **Visual Studio 2022 Build Tools** with the **Desktop development with C++** workload (~5-7GB). Download installer: `https://aka.ms/vs/17/release/vs_BuildTools.exe`. Run as admin; check the workload checkbox; install. (`winget install Microsoft.VisualStudio.2022.BuildTools` works on hosts with winget; not all Windows installs ship it.)
2. **WebView2 Runtime** — typically pre-installed on Windows 10/11 via Edge browser. Verify presence via registry `HKLM:\SOFTWARE\WOW6432Node\Microsoft\EdgeUpdate\ClientState\{F3017226-FE2A-4295-8BDF-00C3A9A7E4C5}` (the Evergreen Runtime GUID).
3. `rustup toolchain install stable-x86_64-pc-windows-msvc` (~100MB).
4. `rustup set default-host x86_64-pc-windows-msvc`.
5. `cargo clean` to drop GNU build artifacts (~10GB freed after switch in this project's case).

After the switch, `cargo xtask test` (workspace nextest) succeeds locally — 10 binaries / 0 tests in Foundation epoch state. CI matrix runners (`windows-latest` = MSVC + WebView2 Runtime preinstalled, `macos-latest`, `ubuntu-22.04`) already have this configuration; the toolchain switch is purely about local dev parity.

Adjacent learnings (still valid for their original scopes — these don't replace, they complement):

- chunk #4 entry "Tauri-dependent crates fail xtask runtime on Windows GNU; feature-gate the runtime to allow type-only consumers" — feature-gating fixes `cargo run -p xtask` (single-package, no workspace unification). Does NOT fix workspace test discovery; that requires the MSVC switch above.
- "Windows GNU rustup toolchain doesn't bundle profiler_builtins for cargo-llvm-cov" — the same MSVC switch resolves both blocks (profiler_builtins available in MSVC std + workspace-wide nextest succeeds).

See: `.andromeda/phases/phase-4/plan.md` Implementation notes (re: NEXTEST_EXPERIMENTAL_LIBTEST_JSON env requirement); `xtask/src/main.rs` `run_cargo_nextest()`; chunk #4 wrap entries on ui-bridge feature-gating + cargo-llvm-cov profiler_builtins.

---

## 2026-05-03 — Tauri-dependent crates fail xtask runtime on Windows GNU; feature-gate the runtime to allow type-only consumers

Any binary that transitively depends on the `tauri` crate links against WebView2 / DirectX / etc. Windows DLLs at link time. On Windows GNU rustup-toolchain hosts without WebView2 installed (or any DLL load-path issue), the resulting binary fails at startup with `STATUS_ENTRYPOINT_NOT_FOUND` (exit code `0xc0000139`) — **even if the binary never actually invokes any Tauri runtime code**. This blocks shared-crate designs where the data types live alongside the procedure implementation: an `xtask` binary that imports `ui-bridge` for `HealthEnvelope` (a pure data type) inherits the tauri DLL deps and crashes.

**Solution**: split runtime vs. types via Cargo features.

```toml
# crates/ui-bridge/Cargo.toml
[features]
default = ["taurpc-runtime"]
taurpc-runtime = ["dep:taurpc", "dep:tauri", "dep:specta", "dep:tokio"]

[dependencies]
thiserror.workspace = true
serde.workspace = true
chrono.workspace = true
taurpc = { workspace = true, optional = true }
tauri = { workspace = true, optional = true }
specta = { workspace = true, optional = true }
tokio = { workspace = true, optional = true }
```

Source code uses `#[cfg(feature = "taurpc-runtime")]` to gate the `#[taurpc::procedures]` trait and resolver impl, leaving the data types (`HealthEnvelope`, `AppError`, `SubsystemStatus`, etc.) compiled unconditionally.

```toml
# xtask/Cargo.toml — non-Tauri binary consumes types only
[dependencies]
ui-bridge = { path = "../crates/ui-bridge", default-features = false }
```

The pulse-app binary keeps default features (taurpc-runtime enabled) so the procedure trait + resolver are available for `taurpc::create_ipc_handler(...)` registration in the Tauri Builder chain.

The `#[cfg_attr(feature = "taurpc-runtime", derive(specta::Type))]` pattern lets data types acquire the `specta::Type` derive only when the runtime feature is active — required for taurpc procedure parameter/return types but useless for type-only consumers.

This is also the cleanest architectural split — types belong in the contract module, runtime belongs in the runtime module.

See: `.andromeda/phases/phase-3/plan.md` Implementation note 1; `crates/ui-bridge/Cargo.toml`; `crates/ui-bridge/src/health.rs` `#[cfg(feature = "taurpc-runtime")] mod runtime`.

---

## 2026-05-03 — Cargo alias for `cargo xtask <subcommand>` shortcut

Without an alias, `cargo xtask harness:status` fails with "no such command: xtask" because cargo doesn't know `xtask` is a workspace member shortcut. The fix is `.cargo/config.toml` (project-root):

```toml
[alias]
xtask = "run --quiet --package xtask --"
```

After this, `cargo xtask <subcommand>` works equivalently to `cargo run --package xtask -- <subcommand>` from any directory inside the project. The `--quiet` flag suppresses Cargo's "Compiling … / Finished …" output so the subcommand's stdout (e.g. JSON envelope from `harness:status`) is the only thing on the pipe — important for `jq` / shell-script chains.

The agent-run scripts (`scripts/agent-run.{sh,ps1}`) invoke `cargo xtask harness:status` and depend on this alias being present.

See: `.cargo/config.toml`; `xtask/src/main.rs` clap dispatcher; `scripts/agent-run.sh` `status` case body.

---

## 2026-05-03 — Andromeda chunk scope-split for paid-prereq operator steps

When a route chunk's full scope requires paid external accounts (e.g., chunk #3 code-signing wants Azure Key Vault Premium ~$5/month + Windows EV cert from DigiCert/GlobalSign $300-500/year + Apple Developer ID $99/year + 1-2 weeks of legal-entity verification) but the project is in dogfooding/iteration phase, **split chunk scope** rather than skip the chunk or pay prematurely.

The pattern: `plan.md` divides Implementation Steps + Acceptance Criteria into **ACTIVE** (free + local + reversible work that `/andromeda-implement` runs now — e.g., generate Minisign keypair locally, add deps, edit `tauri.conf.json`, write rotation runbook) and **DEFERRED** (paid + external + bureaucracy items that become pre-v0.1.0 release blockers — Azure Key Vault provisioning, EV cert enrollment, Apple Developer ID, GitHub Environment with secrets). DEFERRED items are tracked in `plan.md §Acceptance Criteria → Deferred` + the runbook describing operator procedure + a `route.md` Decisions Log entry recording the scope-split rationale.

Pipeline integrity is preserved: `state.yaml.last_completed_chunk.route_index` advances when ACTIVE scope lands; DEFERRED items are explicit pre-release blockers tracked across artifacts (not lost). This is better than (a) skipping the chunk entirely (breaks route progression heuristics + state.yaml continuity) or (b) running paid prereqs before the project demonstrates value (premature commitment).

Apply when: chunk has clear paid-vs-free dependency split AND project is in pre-public-release dogfooding phase AND user explicitly states preference to defer paid commitments. Don't apply when: chunk's value depends entirely on paid prereqs (rare for solo OSS projects).

See: `.andromeda/route.md` Decisions Log 2026-05-03 entry "Chunk #3 scope split"; `.andromeda/phases/phase-2/plan.md` §Acceptance Criteria → Active vs Deferred; `docs/runbooks/updater-key-rotation.md` as DEFERRED procedure document.

---

## 2026-05-03 — Standalone minisign 0.12 as Tauri-cli fallback when Windows GNU mingw blocks compile

The rustup `x86_64-pc-windows-gnu` toolchain bundles a minimal mingw-w64 set in `<sysroot>\lib\rustlib\x86_64-pc-windows-gnu\lib\self-contained\` that does NOT include `libktmw32.a` (Windows Kernel Transaction Manager API import library). Modern Tauri 2.x ecosystem crates link transitively against `ktmw32` so `cargo install tauri-cli --version "^2.0" --locked` fails with `ld: cannot find -lktmw32`.

**Refreshing rust-mingw component does NOT fix it** — `rustup component remove rust-mingw && rustup component add rust-mingw` re-downloads the same minimal libset; `libktmw32.a` is not bundled by design.

Two viable paths:
- **MSVC switch (permanent fix)**: install Visual Studio 2022 Build Tools (~5 GB) + `rustup toolchain install stable-x86_64-pc-windows-msvc` + update `rust-toolchain.toml` channel to MSVC variant. Also resolves the `profiler_builtins` issue for `cargo llvm-cov` (separate Tier-3 entry from previous session). ~30 minutes including download.
- **Standalone minisign 0.12** (jedisct1, Frank Denis): download `minisign-0.12-win64.zip` from `https://github.com/jedisct1/minisign/releases` (~500 KB), unpack `x86_64/minisign.exe` into `~/.cargo/bin/` (already in PATH), use `minisign -G -W -f -s ~/.tauri/{name}.key -p ~/.tauri/{name}.key.pub` for no-password keypair (the `-W` flag = "do not encrypt secret key with a password" — acceptable for local dogfooding scope where private key stays in `~/.tauri/` gitignored; production HSM custody re-generates with password before public release).

Both `tauri signer generate` and standalone `minisign -G` produce **interoperable Minisign Ed25519 keypairs** — the tools both follow the public Minisign spec (`https://jedisct1.github.io/minisign/`). The verbatim base64 line from the `.pub` file (line 2, after `untrusted comment:` header) goes into `tauri.conf.json plugins.updater.pubkey` regardless of which tool generated it; `tauri-plugin-updater 2.x` accepts and verifies signatures from either.

Implication: when blocked on `cargo install tauri-cli` due to Windows GNU mingw limitations, the standalone-minisign fallback unblocks keypair generation without committing to the heavyweight MSVC switch. Document in the chunk's runbook that production-ready key custody re-generates the keypair WITH a password and uploads private + password to the production secrets manager (Azure Key Vault Premium SKU per security plan §Code-signing key custody).

See: `.andromeda/security-plan.md` §Code-signing key custody; `docs/runbooks/updater-key-rotation.md` Phase 1 (operator-side keypair generation); `pulse-app/tauri.conf.json` `plugins.updater.pubkey` field; previous Tier-3 entry "Windows GNU rustup toolchain doesn't bundle profiler_builtins for cargo-llvm-cov" (related rustup-toolchain limitation pattern).

---

## 2026-05-03 — Tauri 2.x transitively requires rustc ≥ 1.88

The architecture's security plan pins minimum rustc to 1.85 for Edition 2024 security-positive defaults (`unsafe_op_in_unsafe_fn`, tightened `if let` temporary scopes, `static mut` reference denial). Tauri 2.11.0's transitive dependency tree (`darling 0.23` requires 1.88, `plist 1.9` requires 1.88, `serde_with 3.19` requires 1.88, `time 0.3.47` requires 1.88, `icu_* 2.2` requires 1.86, `icu_normalizer_data 2.2` requires 1.86) pushes the effective floor to rustc 1.88+ for any project that compiles Tauri 2.

Phase 1 implementation chose `channel = "1.95.0"` in `rust-toolchain.toml` to match the host installation while satisfying the security plan's `1.85+` minimum (the AC's grep regex `^channel = "1\.(8[5-9]|9[0-9])'` matches 1.95). Future Tauri version bumps may push the floor higher — bumping `rust-toolchain.toml` is not a security-plan violation as long as the channel stays ≥1.85.

Implication for future Tauri-related chunks: when adding/upgrading Tauri 2.x or its plugins, check if transitive deps require a rustc bump. Coordinate the bump with the security-plan minimum (≥1.85) and the CI matrix runners.

See: `.andromeda/security-plan.md` §Anti-Patterns Universal + §Decisions Log open question on 1.84 → 1.85 bump; `rust-toolchain.toml`.

---

## 2026-05-03 — Tauri 2 capability JSON `identifier` field uses simple kebab-case names

Architecture §Occupied Resources references Tauri capability identifiers as `pulse:default`, `pulse:tray`, `pulse:notification`, `pulse:updater`, `pulse:plugin-fs` — these are **conceptual fully-qualified namespaced** names. The actual Tauri 2 capability JSON `identifier` field uses **simple kebab-case local** names (`default`, `tray`, `notification`, `updater`, `plugin-fs`); Tauri 2 does not accept colons in identifiers, and the bundle id `com.andromeda.pulse` provides implicit namespacing at runtime.

Filenames in `pulse-app/capabilities/` map 1:1 to the local identifiers (`default.json` → identifier `default`). The `pulse:` prefix is preserved in the architecture and security plans as the conceptual reference (e.g., when discussing "do not expose `pulse:updater` to webview JavaScript"), but the JSON file's `identifier` field uses just `updater`.

Implication: any future capability JSON edit (new TauRPC procedure → matching capability entry per security plan §API Security) should use the simple form. The `xtask capability-drift` check (route#22) will diff TauRPC routers against the file inventory by filename / local identifier — not against the namespaced form.

See: `.andromeda/architecture.md` §Occupied Resources Tauri capability identifiers; `.andromeda/security-plan.md` §API Security TauRPC capability authorization; `pulse-app/capabilities/*.json`.

---

## 2026-05-03 — Windows GNU rustup toolchain doesn't bundle profiler_builtins for cargo-llvm-cov

`cargo-llvm-cov` requires the `profiler_builtins` crate (provided by the Rust standard library precompiled with profiler runtime support). The Rust standard library precompiled binaries for `x86_64-pc-windows-gnu` do NOT include `profiler_builtins`, even with the `llvm-tools-preview` rustup component installed. Running `cargo llvm-cov nextest --workspace ...` fails with `error[E0463]: can't find crate for 'profiler_builtins'` during build-script compilation of common deps (e.g., `serde`, `typeid`, `zmij`).

Phase 1 acceptance criterion T8 (`cargo llvm-cov nextest --workspace --lcov --output-path lcov.info --summary-only`) fails on the local Windows GNU host for this reason. The other 11 acceptance test commands pass. Workarounds: (a) install MSVC toolchain — `rustup toolchain install stable-x86_64-pc-windows-msvc` (requires Visual Studio 2022 Build Tools install) and update `rust-toolchain.toml` channel to `1.95.0-x86_64-pc-windows-msvc`; (b) defer coverage to CI Linux/macOS runners where profiler runtime is bundled — route#5 base CI workflow will primarily exercise the coverage gate on those targets; (c) switch to `cargo-tarpaulin` as alternative (different coverage tool, not in plan AC).

Implication: the route#5 CI matrix workflow should run the coverage gate primarily on Linux + macOS. A Windows MSVC runner can also pass; a Windows GNU runner cannot without rebuilding std with profiler support (nightly-only via `-Z build-std`).

See: `.andromeda/test-plan.md` §3 Bootstrap phase 7 "coverage-tooling-install" + §10 Coverage thresholds; route#5 Base CI workflow chunk.

---

## Entry format

Each entry follows this structure:

```
## {ISO-date} — {short title}
{1-3 paragraphs describing what was learned, why it matters, and where it applies. Reference specific files or documented decisions when relevant.}

See: `.claude/docs/services/{service}.md` (or similar cross-reference)
```

## Tier classification

This file is **Tier 3 — on-demand**. Claude reads it when explicitly needed (debugging, planning, reviewing patterns), not at session start.

Other tiers:
- **Tier 1** (always loaded) — universal safety rules in `CLAUDE.md` `USER:session-learnings` section (critical, short)
- **Tier 2** (path-triggered) — directives in `.claude/rules/*.md` `## Session Additions` sections (loaded when matching files touched)
- **Tier 3** (on-demand) — this file (detailed reference, lazy-read)

See the classification gallery in the refactor plan `§3.5` for which tier a given learning belongs to. wrap-session applies this classification automatically during curation.

## Promotion

When this file grows beyond ~200 lines, `/wrap-session` suggests promoting some entries to topic-specific files (e.g., `.claude/docs/services/{service}.md` if the learning is about a specific service). Promotion is a user action, not automatic — wrap-session never moves entries without approval.

## Demotion from CLAUDE.md

If `CLAUDE.md` `USER:session-learnings` section gets too large (≥ 180 lines total CLAUDE.md), wrap-session suggests promoting old Tier 1 entries down to this file (Tier 3) to keep CLAUDE.md within size budget. This is also a user action.
