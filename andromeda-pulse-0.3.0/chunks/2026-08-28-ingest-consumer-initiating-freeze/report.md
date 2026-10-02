# Report — 2026-08-28-ingest-consumer-initiating-freeze

**Chunk:** Ingest consumer initiating freeze — the trigger behind the wedge gets a named cause, not only a survivable aftermath
**Date:** 2026-08-28
**Commits:** none since `last_wrap` (2026-08-28T16:37Z) — this wrap creates the chunk commit

## Changes (structured — detectors read this)

- **Files:**
  - NEW `xtask/src/gap_resume.rs` (~410 lines incl. 7 colocated tests)
  - MOD `xtask/src/main.rs` (module decl · `Cmd::SmokeGapResume` variant · dispatch arm)
  - MOD `crates/triage/src/cue/emitter.rs` (ONE test added inside the existing `#[cfg(test)] mod tests`; no production line changed)
- **Symbols / APIs:**
  - New xtask subcommand `smoke:gap-resume` with four bounded clap args (`--sustained`, `--bootstrap-seconds`, `--gap-seconds`, `--observe-minutes`). Harness-only; not a product surface.
  - New `pub` items in `xtask::gap_resume` (crate-internal to the xtask binary, no external consumer): `GapResumeOptions`, `StormEvidence`, `LegOutcome`, `storm_evidence()`, `judge()`, `run_gap_resume()`.
  - **No product API change.** No TauRPC procedure, no IPC method, no endpoint, no port, no export. `EXPECTED_PROCEDURES` untouched; `capability-drift` clean.
  - **Env vars:** the leg CONSUMES none. It SETS three already-registered vars into the spawned app's environment (`ANDROMEDA_PULSE_DATA_DIR`, `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS`, `ANDROMEDA_PULSE_L4_DETERMINISTIC`) — no new registry entry owed.
  - **Remaining-caller facts:** `ingest_progress::{evaluate, parse_lines, TARGET_BUFFER_TICK}` gain a SECOND caller (`gap_resume`) alongside the existing `run_check_ingest_progress` — not a sole caller, and neither signature changed. `self_verify::{workspace_root, locate_pulse_binary}` likewise gain a second caller, unchanged.
- **Crates / modules:** none added or removed. `xtask` gains one module (`gap_resume`).
- **Dependencies:** none added, none bumped. No manifest or lockfile delta.
- **Schema / config:** none. No DDL, no corpus table, no config key, no violation schema.
- **Spec-master edits:** none made by implement (correctly — implement is read-only on specs). Two owed, listed under *Expected amendments* below.
- **Counts / qualifiers moved:**
  - workspace nextest **2034 → 2042** (+1 skip unchanged) — stated in `session-handoff.md` and reproduced in chunk reports.
  - `xtask` module count 5 → 6 (`bundle_format` · `gap_resume` · `ingest_progress` · `self_verify` · `smoke` · `webview_drive`).
  - test-plan §3's per-chunk gate list gains a **scenario** leg (not a gate) — see *Expected amendments*.
- **Dev-tool versions:** none — no external CLI installed or upgraded.
- **Reverted / negative API facts:**
  - `build_injector` was NOT promoted to `pub(crate)` in `webview_drive.rs`; a ~25-line local copy lives in `gap_resume.rs` instead, because `webview_drive.rs` is outside this chunk's touchpoints. Deliberate duplication, de-duplication deferred.
  - A hypothesis was pursued and **discarded**: "the consumer loop exited because `recv()` returned `None`". Falsified by reading `pulse-app/src/main.rs:293-294` — the sender is an `Arc<IngestSender>` held for the app's lifetime, so senders never all drop. No code was written against it.
- **Spec claims disproved by measurement:**
  1. **The working-route entry's causal clauses** — "the freeze happened under LIGHT load", "the cue storm FOLLOWED it by 32 seconds", "the runaway is an amplifier and recovery-blocker, **never the trigger**". All three measured false at P3 against the preserved 2026-08-25 corpus: `17:20:50` is the end of injector run 1 (`ingest.tick.span_count` flat at 2187, `buffer_capacity_pct` **0.0** for 13 consecutive ticks — nothing to consume); the producer resumed at `17:24:16`; tier1 `service_went_silent` triggers began `17:23:21`, **~55 s BEFORE** onset. *Disposition: RECORDED HERE, no amendment owed* — the working-route entry is the chunk's historical intent source (RESEARCH-CORRECTS-INTENT), and `scope.md` already carries the `[premise-corrected]` bullets. This bullet IS the disposition.
  2. **The amplifier-bound survivability assumption** — that the successor chunks' bound (`CueLatch` + the idle-observer damper) "makes any initiator a survivable hiccup". **Measured FALSE for the gap→resume shape.** Arm A ran at HEAD with the runaway fully bounded — `cadence.trigger` **65** (wedge: 2,683) · L1a queries **650** (26,821) · `triage.cue.emit` **62** (4,599) — and the consumer died anyway: `rows_ingested` frozen at 3,240, channel 0 → 85.7 %, `buffer.consumer.stalled` fired, `ingest.channel.full` fired. *Disposition: RECORDED HERE, no spec amendment owed* — grep confirms the phrasing lives only in the master-route RECORD for `2026-08-26-cadence-runaway-blocking-pool` (immutable history), not in any of the seven spec masters. The repair entry's first arm exists to attribute what remains.
  3. **This chunk's own selected shape** — plan/scope outcome 4 ("attributed, and ALREADY FIXED upstream"), the operator-selected P4 lean. Dead on the same measurement as (2). *Disposition: recorded here; `scope.md` §2 already flags outcome 4 as the leading candidate with a premise-correction, and this is the measurement that closes it.*
  4. **A relayed coordinate** — "both `spawn_blocking` users died on DIFFERENT mutexes, so no single lock explains both". Derived from HEAD, but the corpus is 2026-08-25 and `viz::query::read_connection` landed at `27c5ac7` on 2026-08-26 — so at wedge time viz held the SAME shared appender connection. Did not change the conclusion (retention swept successfully at 17:21:25 and 17:23:05, excluding that mutex independently). *Disposition: recorded here.*
  5. **test-plan §3's RED-leg rule** — "a MEASURE-FIRST chunk's RED leg is by construction an unhealthy run that MUST produce the ERROR it is measuring; asserting 0 ERROR over it would contradict the measurement". **Measured false for a BLOCK-shaped defect.** Arm A is a RED leg that produced **0 ERROR and 0 `app.panic.fatal`** — nothing returns, so nothing reports — which means §3's clean-log assertion would pass VACUOUSLY over it. The evidence of record for this shape is absence-of-progress (`rows_ingested` frozen, consecutive zero-delta `buffer.tick`s, `buffer.consumer.stalled`, `ingest.channel.full`), not a recorded ERROR + `reject_reason`. *(Found by the test-plan drift detector, not by this report's first draft — added here at P2 so Validate check 6 sees the full set.)* *Disposition: AMENDED — test-plan §3 Direct-binary smoke variant, this wrap.*
  6. **obs-plan §10's closed-isolation framing** — the DuckDB Connection Isolation subsection enumerates three defects as "identified and fixed" plus a fourth consumer joining the isolated set, reading as a solved problem. **The shared appender connection still carries a reproducible consumer death**: under gap→resume both users of that connection (the consumer's `dispatch_batch` and the retention sweep) stop, while `viz` survives on its cloned connection. *Disposition: AMENDED — obs-plan §10 gains a fourth, explicitly OPEN entry with a named owner (operator-resolved escalation, this wrap; the directive had framed this amendment as "no semantic change" — the measurement required one).*
- **Coverage of new surfaces:**
  - `cargo xtask smoke:gap-resume` (harness scenario leg) → validation **n/a** (clap-parsed bounded args, harness-only, never product input) · instrumentation **n/a** (console-only verdict; consumes the app's existing obs record, emits no telemetry of its own — **no new obs target, no new allowlist leaf**) · PII **n/a** (reads the app's already-scrubbed log family; prints only aggregate counts and bounded static tokens, never record content) · tests **unit ✓** (7 colocated pins incl. both discriminating arms) · a11y **n/a** (no UI) · tokens **n/a** (no UI)
  - `CueLatch` multi-window pin (`emitter.rs`) → validation n/a · instrumentation n/a · PII n/a · tests **unit ✓** (deterministic, injected `now_nanos`, no sleep) · a11y n/a · tokens n/a

## Deviations from intent

1. **Plan step 4 was PRE-SATISFIED, not written as specified.** The step described a scenario-shaped `CueLatch` pin; `cue_latch_bounds_the_measured_wedge_shape` already existed at `emitter.rs:1429` (landed by the predecessor) covering it verbatim. *Justification:* writing it as specified would have produced a duplicate. Reused the existing pin and added only the genuinely-new part — the existing pin spans exactly ONE 60 s refractory window while the scenario's gap spans three, and nothing pinned the admission RATE across multiple windows. This is the "plan says ADD but it already EXISTS" reconciliation class.
2. **`build_injector` duplicated rather than shared.** *Justification:* the existing one is private in `webview_drive.rs`, which is not in Files-to-modify; duplication kept the scope boundary intact. Recorded as a de-duplication candidate.
3. **Added a binary-freshness line to the leg** (`binary=… (built Nm ago)`), not in the plan's Implementation Steps. *Justification:* the plan's own Implementation notes anticipated exactly this trap — `locate_pulse_binary` prefers `target/release/` and the release build on disk was a day old, which is the documented silent-stale-binary failure (a leg that runs, logs clean, and reports OLD behaviour as current). Built `--release` and made the selection visible rather than silent. The leg's first run then printed `built 1m ago`, proving it measured HEAD.
4. **Arm A did not reach the plan's expected PASS.** *Justification:* it reproduced a real defect — see *Outcome*. Plan step 6 prescribed stop-and-report on exactly this branch; executed as written, no repair improvised.

## Decisions & corrections

- **Operator fork at /phase P4** — three options offered after research falsified the entry's premise; operator selected **verify-and-pin at HEAD**. That selection is what produced the reproduction.
- **Operator fork at /implement P4** — after arm A reproduced the defect, three scoping options offered; operator selected **new route entry** (repair scoped out of this chunk), against the recorded standing preference for in-chunk fixes, because the cause is narrowed but not identified and the repair needs its own research phase.
- **Wrap directive, item 1** — arm A's red is a TRUE POSITIVE with an owner: this chunk CREATED a scenario that reproduces a PRE-EXISTING open defect. It is neither a regression of this chunk nor external decay. Never weaken, remove, or `--expect-absent` it. It registers in test-plan §3 as a **SCENARIO**, not a standing gate, until the repair entry turns it green. **Arm B alone is gate-grade today.**
- **Wrap directive, item 2** — the repair entry is placed **FIRST markerless**, ahead of App-registry reconciliation (overseer lean, operator-placed at the P5 trajectory dialogue): a reproducible consumer death under gap→resume is data loss on a shape real deployments hit daily (any producer restart), and the ~13-minute scenario makes the repair cheap to verify.
- **Wrap directive, item 3** — the amplifier-bound survivability assumption is stated plainly as measured-false above.
- **Measurement discipline that paid** — the plan's "prove the scenario capable before believing its verdict" criterion is encoded in the leg itself: `judge()` returns `Inconclusive`, not `Pass`, when the gap arm produces no storm. Arm B inverts it (a storm in the control arm is also `Inconclusive`), which is what makes the two arms different tests rather than one test run twice.

## Expected amendments (wrap)

- `test-plan.md` §3 — register `cargo xtask smoke:gap-resume` as a **scenario leg**, explicitly NOT a member of the unconditional per-chunk gate set: arm A currently reproduces an open defect and exits 1 by design; arm B (`--sustained`) is gate-grade today. Ownership of turning arm A green belongs to the repair entry.
- `obs-plan.md` §10 — record that the drain-progress detector (`buffer.consumer.stalled` / `check:ingest-progress`) was consumed by a scenario leg and **fired correctly on a fresh 2026-08-28 reproduction**, with its bounded tokens observed at the wire (`reason=rows_static_while_channel_queued`, `consequence=accepted_spans_not_persisted`). No semantic change — this strengthens the existing claim rather than contradicting it.

## Outcome

**Acceptance criteria: met**, with the smoke's finding replacing the expected PASS on arm A.

**Gates green** — `cargo fmt --check` · `cargo clippy --workspace --all-targets --all-features -D warnings` (0) · `cargo build --workspace --tests --jobs 4` · `cargo nextest run --workspace --profile ci` **2042/2042 + 1 skip** (+8 = exactly the 7 `gap_resume` pins + 1 `emitter` pin) · `cargo xtask capability-widening-check` clean (0/3) · `cargo xtask check:ingest-progress` PASS · `cargo deny check bans licenses sources` exit 0 · `cargo xtask capability-drift` clean LAST after bindings regen (`bindings/index.ts` byte-identical to HEAD). **1 fix-loop iteration** (the documented rustfmt-hook/edition mismatch). No gate deferrals — there is Rust delta, so every workspace gate ran.

**Smoke — the chunk's acceptance evidence. Two arms, one binary, one host:**

| | Arm A (gap → resume) | Arm B (sustained control) |
|---|---|---|
| verdict | **FAIL** — stalled 450 s | **PASS** |
| `rows_ingested` | frozen at **3,240** | **29,160** |
| `duckdb.append` | 230, all pre-freeze | **1,849** |
| zero-delta ticks | ~40 consecutive | **1 of 38** |
| `ingest.channel.full` | fired | 0 |
| `cadence.trigger` | 65 | 12 |
| ERROR / panics | 0 / 0 | 0 / 0 |

Clean teardown both arms: 0 orphan processes, `:4317`/`:4318` released.

**The wedge now reproduces on demand at HEAD — the first time ever.** Arm B reproduces the predecessor's "healthy" result exactly, which is *why* the wedge was believed irreproducible: sustained load never disconnects, so the shape was never exercised.

**What is established:**
- The `CueLatch` bound is not the fix (see *Spec claims disproved* 2).
- **Research Q2 answered: it wedges on RESUME, not during the quiet window.** The consumer drained seed 1 completely, and retention swept successfully at 17:59:54 **and** 18:01:34 — so it held no lock through the gap. The producer resumed 18:02:29; `rows_ingested` never moved again.
- **The block is on the SHARED appender connection.** `dispatch_batch` takes `conn.lock()` first and holds it across every append. Retention's cadence is `retention_seconds.max(60)/6` = 100 s, matching its two observed sweeps exactly; its 3rd was due ~18:03:14 and never came. Both users of the shared connection died; `viz.query.traces` ran to the end (771 records) on its own cloned connection, isolated 2026-08-26. Zero ERROR and zero panics because nothing returns — it blocks.
- **Excluded by measurement:** the in-spec non-defect obs-plan §10 warns about (row-group maintenance blocking >60 s on a ~12.7 M-row table). This table held 3,240 rows and blocked 9+ minutes.

**Stated, not implied:**
- WHY the first post-resume batch blocks is NOT established.
- Arm A changes **two** variables (180 s idle AND a new gRPC connection — the injector connects once per process). Arm B controls the combined shape and does not separate them. The discriminating experiment is a reconnect-WITHOUT-gap arm, which is the repair entry's first move.
- The reproduction logs (81 MB + 59 MB) live under `target/gap-resume/`, gitignored and volatile; the scenario regenerates them in ~13 minutes, so nothing needed preserving.
