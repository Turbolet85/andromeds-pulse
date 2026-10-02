# Codebase Research — 2026-08-16-baseline-family-reachability

## Scope
- **Depth:** moderate · **Reads:** 4 (2 files, 2 sections each) · **Globs/Greps:** 7 · **Graph queries:** 1

## Files inspected
- `crates/triage/src/baseline/activity_floor.rs` (1–140, plus grep of the rest) — owns the EFFECTIVE gate.
  `BOOTSTRAP_WINDOW_SECONDS: u64 = 3_600` at `:33`; `BootstrapState { Learning, Ready }` at `:69`; the
  bootstrap comparison at `:178` (`bootstrap_nanos = BOOTSTRAP_WINDOW_SECONDS * 1e9`). `observe()` at `:112`
  sets `first_observed_unix_nanos` on the first call and pushes the inter-observation gap into the
  quiet-duration t-digest from the second call onward.
- `crates/triage/src/cue/evaluate.rs` (1–199) — the three family gates side by side. `ErrorRateSpike`:
  `snapshot.samples < thresholds.min_ewma_samples` (`:38`). `LatencyRegression`:
  `snapshot.samples < thresholds.min_latency_samples` (`:88`). `ServiceWentSilent`:
  `snapshot.bootstrap_state != BootstrapState::Ready` (`:164`), then the
  `> max(p95, MIN_QUIET_SECONDS)` floor (`:174`).
- `crates/triage/src/cue/thresholds.rs` (grep) — `DEFAULT_BOOTSTRAP_WINDOW_SECONDS = 3_600` at `:94`, the
  `Thresholds.bootstrap_window_seconds` field at `:141`, its default at `:163`, its validation at `:243`
  (rejects 0). No read site anywhere.
- `crates/triage/src/cue/emitter.rs` (grep, `:58`–`:86`) — the production caller of
  `evaluate_service_went_silent`, and the site that already counts + emits the warm-state aggregate.
- `crates/ui-bridge/src/contract.rs` (grep of `Settings` fields) — no bootstrap/warm-up field exists.
- `pulse-app/src/observability.rs` (grep) — the allowlist leaf + its guarding test for the warm-state signal.

## Graph impact (`.andromeda/runs/2026-08-16T16-30-00Z-phase/tree-query-2026-08-16-baseline-family-reachability.json`)
Query: `SELECT callee, file, line FROM refs WHERE callee LIKE '%BOOTSTRAP_WINDOW_SECONDS%'` — **`rows: 11`,
`db_state: fresh`** (the trace's `rows` field is the authority here; an earlier `tail`-clipped read of this
same result showed only 6 and was corrected against the trace).

- **`baseline/activity_floor/BOOTSTRAP_WINDOW_SECONDS`** — **7 references**, all inside `crates/triage`:
  `activity_floor.rs:54` + `:55` (the module-level `const _` assert block), `activity_floor.rs:177`
  (**the gate comparison** — the one load-bearing site), `activity_floor.rs:315` + `:324` (existing tests
  straddling the boundary), plus the two re-export sites `baseline/mod.rs:37` and `contract.rs:17`.
- **`cue/thresholds/DEFAULT_BOOTSTRAP_WINDOW_SECONDS`** — **4 references**: `contract.rs:41` and
  `cue/mod.rs:33` (re-exports), `thresholds.rs:162` (the `Default` impl), `thresholds.rs:275` (a compile-time
  assert). **No evaluation-path reference** — this is the inert half.
- **Blast radius: zero cross-crate.** All 11 references live inside `crates/triage`; the constants surface
  outward only through `contract.rs`. `evaluate_service_went_silent` has exactly one production caller
  (`emitter.rs:58`). The two existing tests at `:315`/`:324` already assert the boundary against the
  hardcoded const and will need to move to the resolved value.

## Patterns detected
- **The gate is a derived enum, not a raw duration comparison at the call site**
  (`activity_floor.rs:178` → `BootstrapState`; consumed at `evaluate.rs:164`). Any mechanism that changes
  reachability must change what `BootstrapState` resolves to — changing a constant that nothing reads
  changes nothing.
- **A decorative mirror** (`thresholds.rs:92-94`): the doc comment claims the constant "Mirrors
  `baseline::BOOTSTRAP_WINDOW_SECONDS` so config-path / hot-reload stay consistent", and the field is fully
  plumbed (default + validation + tests) — but `evaluate_service_went_silent` takes `_thresholds`
  underscore-prefixed, so the field is read by nothing. The config path it promises does not exist.
- **The warm-state signal already exists and is already un-redacted** (`emitter.rs:66-86` →
  `triage.baseline.service_went_silent.evaluate` with `services_in_bootstrap` / `services_ready`;
  allowlisted at `observability.rs:1365-1384`; guarded at `:3522` for both the required field set and the
  chunk #62/#63 PII bans). This is the harness's positive reachability probe, and it needs no new obs work.
- **Aggregate-only triage discipline** holds here (curated 2026-05-17 session 84): the evaluate event carries
  counts, never `service_name` / `scope_id`. Any field this chunk might add must respect that.

## Conventions to follow
- **Compile-time sanity asserts on constants** live in a module-level `const _: () = { assert!(…) }` block
  (`activity_floor.rs:51-61`), not inside `#[test]` fns — `clippy::assertions_on_constants` forbids the
  latter under `-D warnings` (rules/testing.md 2026-05-11). Any new bound must extend that block, and any
  relation it must preserve (e.g. `BOOTSTRAP_WINDOW_SECONDS < WINDOW_DURATION_SECONDS`, `:56`) is asserted
  there.
- **Bounded-field validation returns a named field error** (`thresholds.rs:243-245`,
  `field: "bootstrap_window_seconds"`) — the shape for any new bounded knob.
- **Co-located `#[cfg(test)] mod tests`** in `crates/triage/src/**` (test-plan §4); paused clock via
  `#[tokio::test(start_paused = true)]` + `advance()` for time-gated assertions (test-plan §7/§8).
- **Explicit allowlist leaf per dotted target, never a bare prefix** — if any emission changes, the leaf must
  enumerate every field the emit site actually emits (rules/observability.md 2026-08-15).

## New files to create
- None anticipated. The change is confined to existing `crates/triage` modules plus their co-located tests.

## Files to modify
- `crates/triage/src/baseline/activity_floor.rs` — the gate site (`:33` constant, `:178` comparison). The
  mechanism must land here or make this site read the value from elsewhere.
- `crates/triage/src/cue/evaluate.rs` — `evaluate_service_went_silent` (`:157`), whose `_thresholds`
  parameter must stop being ignored if the cue-side field becomes the source of truth.
- `crates/triage/src/cue/thresholds.rs` — the currently-inert constant/field, if it becomes load-bearing.
- `crates/triage/src/contract.rs` / `baseline/mod.rs` / `cue/mod.rs` — re-export lines only, and only if a
  symbol is renamed or added (graph shows these are the sole reference sites).
- **Caller threading:** the graph found no cross-crate callers, so there is no registration/plumbing set to
  thread. `emitter.rs:58` is the single production call site and its signature is unchanged unless the
  mechanism moves the value into `Thresholds`.
- **NOT to be modified unless the plan deliberately chooses config-surfacing:** `crates/ui-bridge/src/
  contract.rs` — touching it trips the test-plan §3 boot-smoke trigger and regenerates the TauRPC TS
  bindings under `pulse-app/ui/**`, which pulls in the three `npm run` gates. A `crates/triage`-only change
  keeps the gate list to the Rust standard set.

## Open questions
- **Which mechanism delivers the bound** (shorten the default / make the inert field load-bearing and
  config-surfaced / add an env-gated verification affordance in the `ANDROMEDA_PULSE_L4_DETERMINISTIC`
  shape) → blocks: **plan-decision**. The three differ in blast radius: a shortened default changes
  production detection for every user; the other two do not. P4 must resolve before synthesis.
- **Whether the acceptance may assert reachability with a paused clock alone, or must also prove it on a
  real fresh-dir boot** → blocks: **plan-decision**. The corpus-key precedent (curated 2026-08-15) warns
  that an in-process test constructing warm state directly can pass while a fresh-dir boot still cannot
  reach it; the counter-consideration is the `>30s` quiet floor, which makes a live proof cost ≥30s of
  wall-clock even at a zero window.
