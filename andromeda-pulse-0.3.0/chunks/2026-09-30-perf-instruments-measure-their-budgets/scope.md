# Scope — Perf instruments measure what their budgets name

**Marker:** `2026-09-30-perf-instruments-measure-their-budgets` · **Version:** andromeda-pulse-0.3.0 · **Epoch:** Epoch 4 — Polish & ship: verification
**Working entry (intent anchor):** `working-route.md:142` — "Perf instruments measure what their budgets name — the snapshot timer spans what its 500 ms budget bounds and the gate reads it; a frame-less run names its cause"
**Chunk base (diff-shaped probes):** `ea50ca2` (W182, operator directive)
**Premise closure:** P3, 2026-09-30 — every `[inferred]` bullet below closed against `research.md` (verified → tag dropped; corrected → `[premise-corrected: …]`).

## Goal
Every perf instrument the `perf:budget` gate grades measures the quantity its budget names: the snapshot sample spans the
whole generation the 500 ms budget bounds (not the markdown formatting alone), and a CI run that yields no frame samples
names WHY from the app's own log instead of printing an undifferentiated `frame: cannot-evaluate`. The CI `release` job's
own rust-cache key saves on a red round like every other owning key, and the Actions cache stays inside its 10 GB cap.

## In scope

### A. Snapshot timer spans what its budget bounds
- The snapshot sample `perf:budget` grades (`metric.snapshot.token_count_ms` · `duration_ms`, `xtask/src/perf_budget.rs:49`)
  is started at `crates/snapshot/src/markdown.rs:43` (`let started = Instant::now();` inside `format_markdown`) and read as
  whole ms at `:158` — verified at HEAD — so it times formatting only.
- MEASURED at `2026-09-30-perf-budget-gate-reads-real-samples`: "50 × 0 ms against 61–76 ms per full load + curate + format
  (debug build)" — spot-checked still true at HEAD: the emit sites (`markdown.rs:178`, `:230`) and the producer path
  (`perf_budget_samples.rs:209` → `snapshot_runtime.rs:140-143`) are unchanged since that measurement (obs-plan §10 records it).
- obs-plan §10 (snapshot row, "Snapshot generation (25k token budget)") and §5 (row "Snapshot generation") bound the
  generation, and §4's P2 must-trace chain defines it as load (`duckdb.query.aggregation`) → render → token validate — verified
  (`obs-plan.md:299`, `:370`, `:628`). "Generation" = load + curate + format; the resolver's file writes, clipboard and
  notification are outside the P2 chain and outside the timed span.
- The timer, or a second one, spans what the budget bounds, and `perf:budget` reads THAT value (entry: "the timer, or a
  second one, must span what the budget bounds, and `perf:budget` must read that value").
- The generation is orchestrated in THREE places, all timed the same way: `pulse-app/src/snapshot_runtime.rs`
  `load_curated_markdown` (:137-146) and the `snapshot.generate` resolver (:198-214), and `crates/mcp-server/src/tools.rs`
  `dispatch_generate_snapshot` (:289-295) — research's graph query.
- [premise-corrected: the producer calls `load_curated_markdown` (`perf_budget_samples.rs:209`), so timing inside that function reaches the CI gate with NO producer edit] The in-process producer `pulse-app/tests/perf_budget_samples.rs`
  (`--profile perf-samples`, CI lint-test Linux) emits the whole-generation sample once `load_curated_markdown` is timed; the
  memory + snapshot arms stay required.
- HYPOTHESIS (entry, verbatim marker "hypothesis: … — measure it"): "the p99 moves by orders of magnitude and stays well under
  500 ms". Supported by the predecessor's 61–76 ms debug reading; this chunk MEASURES it (base reading at `ea50ca2`, after
  reading at the change) and records the number — never assumed.

### B. A frame-less run names its cause
- `app.boot.gpu.check` (`pulse-app/src/window.rs:93`) lost its hardcoded `gpu_available` at the predecessor chunk, so no app
  record states whether a WebGPU adapter was obtained. `ci#36723465727` read 0 frames with the app healthy and could not
  tell "flags not applied" from "no adapter".
- The frontend's adapter request (`pulse-app/ui/src/canvas/webgpu-adapter.ts:50`, `navigator.gpu.requestAdapter()`; the
  `unavailable` arm's reason at `:52`) — verified at HEAD — has its RESULT reach the app's own log (entry e.g.: "a record of
  the frontend's adapter-request result").
- The CI `frame: cannot-evaluate` line names its cause from that record.
- [premise-corrected: no existing procedure can carry it — the five `telemetry.frontend.*` methods carry durations, a count and an IPC-rejection triple (`crates/ui-bridge/src/telemetry.rs:229-241`)] The record crosses the webview→backend bridge through a NEW TauRPC procedure in the `telemetry.frontend.*` family, which carries
  its `EXPECTED_PROCEDURES` pin + a validated argument struct (closed outcome enum + coerced window label) + an exact tracing
  allowlist leaf; no capability JSON changes.
- [premise-corrected: `await navigator.gpu.requestAdapter()` at `webgpu-adapter.ts:50` is unguarded — a rejection escapes as an unhandled promise, no fallback renders and nothing is recorded] The cause vocabulary is closed and distinguishes: no `navigator.gpu` · the adapter request returned null · the adapter
  request REJECTED (a new, caught arm) · the device request failed · adapter obtained · and, on the grader side, no adapter
  record at all (the webview never reached the request, e.g. the in-process producer log).
- Where the frame line prints (research): `perf:budget` over the lint-test producer log (no webview in that run — today's
  "no WebGPU adapter" cause is one that run cannot contain), `ci-gates` over the CI boot job's log (a real WebKitGTK webview
  under xvfb, `ci.yml:458-471`), `perf:load-profiles`, and `perf:frame-sample`'s own fixed 0-sample line
  (`xtask/src/perf_frame.rs:244`). All four name the cause from the same derivation.

### C. CARRY — release cache saves on failure; cache cap re-read
- `.github/workflows/ci.yml:241` (`shared-key: release-${{ runner.os }}`) — verified at HEAD — has no
  `cache-on-failure: true`, which the `lint-test` (:50), `boot` (:434) and `coverage` (:588) owning keys set.
- MEASURED at the predecessor (CARRY text): "the red Windows release round (`ci#36723465727`) saved no key, so the next round
  rebuilt Windows cold (35 m 48 s)" — the evidence pointer (`chunks/2026-09-30-perf-budget-gate-reads-real-samples/evidence/ci-cache.md`) stands at HEAD.
- After the change, re-read `gh api …/actions/cache/usage` against the 10 737 418 240 B cap. Read at P3 (before the change):
  10 605 172 169 B in 8 entries (7 rust keys on lock hash `769a9503` + gitleaks), 132 246 071 B (1.23 %) headroom.
- Founder ruling "nothing deferred": `cache-on-failure` re-saves the SAME key (no new entry), and this chunk adds no
  dependency (no `Cargo.lock` change → no new key set), so it creates no growth; if the post-change re-read nonetheless shows
  the cap reached or an owning key evicted, this chunk resolves it (not a follow-on entry).

## Founder / operator rulings folded (2026-09-30, this take-up)
- CI speed is a priority. No change in this chunk adds a CI job, a CI step that builds, or a cold rebuild; the cache fix is
  itself a CI-speed measure, and the frame-cause witness rides the EXISTING boot job's `ci-gates` step.
- Nothing deferred: every item above lands in this chunk.
- Operator slots: any run that launches `pulse-app` or opens a window (self-verify, `perf:frame-sample`, the ps1 boot leg,
  any headful probe, the boot-smoke gate) is the operator's slot even with ports 4317/4318 free (conductor-builder runs NVDA
  legs) — STOP and ask, stating the run's length. The in-process `--profile perf-samples` producer launches no binary and
  opens no window (ephemeral loopback port, `perf_budget_samples.rs:142`) — confirmed not a slot run (below).
- **Boundary widening RATIFIED (P4, 2026-09-30, playbook `verdict: escalate` pattern "Boundary widening").** Asked with
  this exact shape: a new `telemetry.frontend.record_webgpu_adapter` procedure, a closed 5-value outcome enum
  (`obtained` / `no_navigator_gpu` / `adapter_null` / `adapter_request_rejected` / `device_request_failed`), a coerced window
  label, an exact allowlist leaf, an `EXPECTED_PROCEDURES` pin, on the `record_ipc_rejection` precedent, no capability
  change. Founder ratification, live, verbatim: «Да, делай».
- The in-process `--profile perf-samples` producer is NOT an operator-slot run (P4 answer, overseer concurring verbatim:
  "agreed; no executable, no window, ephemeral loopback only, so it runs freely").
- Disk: 66 GB free at take-up; `cargo clean --profile dev` first if free space drops under 60 GB.
- Diff-shaped probes name the chunk base `ea50ca2`.

## CI verdict read at Setup (5a)
- `ea50ca2` (the predecessor's wrap commit, the only sha since the last flip): **verdict not yet available** — `ci#36741143328`
  in progress at Setup and still at P3 (13 checks; 5 running at the P3 re-read, oldest lint/test windows-latest 973 s);
  `secret-scan#36741143347` completed/success. Re-read at P5: **verdict: green** — `ci#36741143328` completed/success,
  checks 13/13, wall 1606 s. Nothing to fold.

## Boundaries (out of scope)
- Span-level redaction, real-model incident surfacing, Conductor return (P-075) — the next three route entries.
- No RSS memory gate (the memory arm stays rows × 256 B, per the predecessor's obs-plan amendment).
- The frame p99 gate stays the dev-host `cargo xtask perf:frame-sample`; this chunk does not make CI produce frames — it
  makes a frame-less CI run name its cause.
- No change to the snapshot budget values (500 ms p99) or to the grader's nearest-rank p99 rule.
- No new visible UI: the adapter record is log-only; the shared `<Fallback />` rendering is unchanged.
