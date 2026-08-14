# Report — 2026-08-14-fingerprint-feed-capture-repair

**Chunk:** Fingerprint-feed capture and repair — the storm detector's feed observed live, then the dead region between OTLP ingest receipt and the per-span-event fingerprint hook diagnosed and repaired (absorbs the Rust gate deferral open since 2026-07-06)
**Date:** 2026-08-14
**Commits:** (this chunk is uncommitted at authoring time; prior since last_wrap: `c956a20` chore(route): operator-requested adaptation — 0-pending wrap)

## Changes (structured — detectors read this)

- **Files:** `crates/buffer/src/state.rs` · `crates/buffer/src/appender.rs` · `crates/buffer/src/consumer.rs` · `crates/buffer/src/contract.rs` · `pulse-app/src/heartbeat.rs` · `pulse-app/src/main.rs` · `pulse-app/src/observability.rs` · `pulse-app/tests/unit_observability_allowlist_feed.rs` (new) · `scripts/agent-run.sh` · `scripts/agent-run.ps1` · `.claude/settings.json` · `.gitignore`

- **Symbols / APIs:**
  - `buffer::state::BufferState::record_feed_counts(u64, u64, u64)` — NEW pub method; folds one batch's feed tallies into three `AtomicU64`.
  - `buffer::state::BufferState` — 3 new private fields (`span_events_seen`, `fingerprints_computed`, `observer_invocations`).
  - `buffer::state::BufferStateSnapshot` — 3 new pub fields (same names).
  - `buffer::contract::BufferHeartbeat` — 3 new pub fields (same names); populated in `heartbeat_payload` from the snapshot (no signature change).
  - `buffer::appender::build_span_events_record_batch` — `pub(crate)` signature gains a third param `state: &BufferState`; 7 call sites threaded (1 production `consumer.rs:119`, 6 test).
  - `pulse_app::observability::AllowList` + `::production()` + `::for_target()` — visibility elevated private → `pub` + `#[doc(hidden)]` (testability only; `baseline_persistence.rs` precedent).
  - **No new TauRPC procedure, no IPC method, no endpoint, no port, no socket, no env var.**

- **Crates / modules:** none added, removed, or renamed. Changes confined to the existing `buffer` library crate and the `pulse-app` binary crate.

- **Dependencies:** none added, none bumped. `Cargo.toml` / `Cargo.lock` untouched.

- **Schema / config:**
  - `.claude/settings.json` — new top-level `env` block (`PYTHONUTF8=1`, `PYTHONIOENCODING=utf-8`), merged; `hooks` untouched.
  - `.gitignore` — the `.andromeda/runs/` exclusion REMOVED (operator decision, this session): the audit-trail run directories become tracked (23 MB / 397 dirs / 2080 files land in this commit).
  - No DuckDB or corpus migration; no new table, column, or config key. No violation-schema change.

- **Harness + log-format changes (D-tests-obs-harness reads this):**
  - `scripts/agent-run.sh` — added `export PYTHONUTF8=1` + `export PYTHONIOENCODING=utf-8` after `set -euo pipefail`.
  - `scripts/agent-run.ps1` — added the same two as `$env:` assignments plus `[Console]::OutputEncoding` / `[Console]::InputEncoding` = UTF8.
  - **Neither touches the 5-command discipline, the status-endpoint shape, the PID-file path, or the log path/format contract** — the 5 verbs, their exit semantics, and the `agent-latest.jsonl` sink are unchanged. test-plan §3 ↔ obs-plan §3 remain in agreement; this is an encoding-relay addition, not a contract change.
  - **Log format, additive:** `buffer.tick` gains 3 fields (`span_events_seen`, `fingerprints_computed`, `observer_invocations`); one NEW tracing target `app.boot.buffer.degraded` (WARN, fields `reason` + `consequence`, fires once per boot when the buffer connection is absent). Both registered in the `AllowList` (the `buffer` entry + a new explicit leaf entry).

- **Counts / qualifiers moved:** workspace test count 1730 → 1734 (+4 allowlist probes). No documented count in any spec master states the test total, so no doc restatement is implicated.

- **Dev-tool versions:** none.

- **Reverted / negative API facts:** none. (Nothing was written then removed; the planned repair was never written — see below.)

- **Spec claims disproved by measurement:**
  **The dead-region-at-HEAD premise measured FALSE.** The claim — that a dead region exists between OTLP ingest receipt and the per-span-event fingerprint hook, and that this chunk repairs it — is stated in `chunks/2026-08-14-fingerprint-feed-capture-repair/scope.md` (§Outcome, §What this chunk delivers item 2), in `plan.md` (§Goal, Implementation Step 8), and in the working-route entry's "then the dead region … repaired" clause. Measuring evidence, from the live capture (`capture-datadir/logs/agent-latest.jsonl.2026-08-14`, this run): `buffer.tick` reported `span_events_seen=936 · fingerprints_computed=936 · observer_invocations=936` — three counts taken independently at three stages of the path, all equal and all non-zero; `triage.pattern.storm.tick` moved `storms_detected_total` 0 → 2; `triage.pattern.storm.detected` fired at occurrence 5 (suggested) and 10 (autonomous) on one shared fingerprint `c33df842`; 9 cues and 10 incidents followed. Branch 1 of the five ranked candidates was eliminated directly — `buffer.schema.init` succeeded and the new `app.boot.buffer.degraded` warn fired 0 times, so `buffer_conn` was `Some`. Branches 2-5 are excluded by the equal non-zero counters themselves. **No branch held; the planned repair had no target.** The hand-off's own measurement (31 zeroed tick lines across three arms) is NOT invalidated — it stands as measured; what is falsified is that the defect is present at HEAD in a normal boot. Needs disposition (Validate check 6).

- **Coverage of new surfaces:**
  - `BufferState::record_feed_counts` + the 3 counters (internal hot-path op) → validation n/a (no external input) · instrumentation ✓ (surfaced on the existing 15s `buffer.tick`; no per-event emission — obs-plan §11 hot-path rule) · PII ✓ (counts only; no service/span/fingerprint identity — the aggregate-only rule) · tests ✓ (unit: appender ×2 counter assertions, consumer end-to-end ×1; live capture) · a11y n/a · tokens n/a
  - `app.boot.buffer.degraded` (new tracing target) → validation n/a · instrumentation ✓ (WARN, fires once at boot) · PII ✓ (two bounded static strings) · tests ✓ (2 of the 4 allowlist probes target it, incl. a prefix-fallback guard) · a11y n/a · tokens n/a
  - `AllowList` visibility elevation → validation n/a · instrumentation n/a · PII ✓ (read-only accessor; no data surface) · tests ✓ (the 4 probes exist because of it) · a11y n/a · tokens n/a
  - UTF-8 relay in `agent-run.{sh,ps1}` → validation n/a · instrumentation n/a · PII n/a · tests `unrunnable-here` (no harness leg asserts child-process encoding; the change is verified by parse-check + the exports being present, not by a behavioural test) · a11y n/a · tokens n/a

## Deviations from intent

1. **`crates/buffer/src/contract.rs` added to the touchpoint set** (not in plan's Files to modify). *Justification:* `buffer.tick` fields transit `BufferHeartbeat`, so plan step 4 is unimplementable without it. Same crate, and it is the declared transport between two files the plan DOES list (`state.rs` → `heartbeat.rs`). Judged the discipline's in-scope gray area rather than an out-of-scope soft-exit.
2. **Per-batch counting instead of the previewed per-span-event atomics.** *Justification:* the approved option was "permanent, tick-aggregated"; the preview illustrated cost as three `fetch_add` per span-event. Implementation derives the tallies from the already-built row vectors and folds once per batch — identical observable totals, strictly fewer atomic ops, and a better fit to the plan's own "the loop is a named hot path" constraint.
3. **Allowlist probes placed in `pulse-app/tests/`, costing three visibility elevations.** *Justification:* plan step 6 mandated probes but not a location; the conventional home (`observability.rs`'s `mod tests`) is dead code under `[lib] test = false`. The baseline run proved it live — zero pulse-app src-level tests executed while ~15 existing `allowlist_for_target_resolves_*` probes sit there. A probe that cannot fail is not a guard.
4. **Step 9's guard went into the existing consumer end-to-end test, not the planned new file.** *Justification:* with no repair to guard, a file premised on one would be theatre; the genuinely uncovered seam was the counters surviving the consumer path into `BufferState`, which that test already drives.
5. **Step 8 (the repair) produced no code.** *Justification:* the premise falsification above — no branch held. Recorded as a surfaced outcome, not silently absorbed.
6. **Two out-of-chunk instance items ride this commit** (operator-directed at wrap): the UTF-8 relay and the `.andromeda/runs/` tracking decision. Neither is chunk work; both were standing items explicitly assigned to this commit.

## Decisions & corrections

- **Instrumentation ships permanently** (operator, at /phase P4): tick-aggregated counters + the degraded-boot warn, over diagnostic-only scaffolding or boot-warn-only. Reason given: the defect survived because "nothing reached the observer" could only be inferred from silence.
- **Entry order 3 → 1 → 2** (operator, at the route adaptation): causal chain, and diagnose-first de-risks the schedule.
- **Route entry provenance belongs in the trailing parenthetical**, not a free-standing sentence — a free sentence is neither title-hint nor named annotation class, so promotion's fold list cannot see it. (Operator correction; curated Tier 3 at the prior wrap.)
- **Matrix: link nothing** (operator, ratified at /phase P5): this repairs behavior the spec already assumes rather than adding a capability; P-083 minting stays available at a later route pass.
- **`.andromeda/runs/` tracked forward** (operator, this wrap): 23 MB / 397 dirs / 2080 files, measured and presented before the decision; the `.gitignore` line removed so the restore lands in this commit rather than as a silent partial track.
- **The gauge is not the acceptance signal** — `tracked_fingerprints_count` is windowed, post-eviction, and counts DISTINCT fingerprints; the capture reproduced this deliberately (`gauge=1` on a healthy 936-event storm). The latched `storms_detected_total` / `fingerprints_evicted_total` are the discriminators.

## Outcome

**Acceptance criteria:** met, except the one whose premise was falsified. Feed counters non-zero and mutually equal ✓ · no raw `exception.*` value in `agent-latest.jsonl` (0 occurrences for all three canary literals) ✓ · no per-span-event `info` emission ✓ · zero `app.panic.fatal`, zero ERROR, no >45s tick gap ✓ · scrubber invocation and fingerprint-before-scrub ordering preserved ✓ · ingest invariant checks unweakened ✓ · latched `storms_detected_total` moved 0 → 2 ✓ · guard test at the defect's tier ✓ (adapted: guards the instrumentation, there being no repair). **The "dead region is named and repaired" criterion is not met and cannot be — no dead region existed at HEAD.**

**Gates green** (all raw, exit codes trustworthy): `cargo build --workspace --tests --jobs 4` (CARGO_INCREMENTAL=0) · `cargo nextest run --workspace --profile ci` **1734 passed / 0 failed / 1 skipped** · `cargo fmt --check` · `cargo clippy --workspace --all-targets --all-features -- -D warnings` · bindings regen (`--features mcp-server --bin pulse-app`) · `cargo xtask capability-drift` clean (0 missing, 0 extra) · `cargo xtask capability-widening-check` clean (0 violations / 3 inspected) · post-guard re-run `buffer` 153/153 + probes 4/4. Fix-loop: 1 iteration, formatting only.

**Rust gate deferral CLOSED** — open since `2026-07-06-incidents-panel-dropdown-layout-bug` across five chunks; the 1734-green run also covers the two out-of-loop hygiene commits (`6fbbd30`, `de00fbc`) that touched Rust source with no wrap gate over them.

**Smoke (boot-path changed — `pulse-app/src/main.rs`):** ✓ full boot on a fresh data dir with deterministic L4; 4482 rows ingested, 936 span-events fed; 10 incidents created; 0 panics; 0 ERROR; clean shutdown by specific PID with `:4317`/`:4318` confirmed released, zero orphans. The smoke and the chunk's central diagnostic were the same run by design.
