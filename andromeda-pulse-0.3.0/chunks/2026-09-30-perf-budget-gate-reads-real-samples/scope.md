# Scope — 2026-09-30-perf-budget-gate-reads-real-samples

## Source

Working-route entry (`andromeda-pulse-0.3.0/working-route.md:140`, taken up 2026-09-30), verbatim:

> Perf-budget gate reads real samples — the CI perf-budget gate can fail: it reads a log carrying frame, memory and
> snapshot samples · CONTEXT: measured at 2026-09-29-p-025-hue-shift-observable-made-gradable — CI's `ci-gates` feeds
> `perf-slo-check` only the boot-smoke log (52 records, 0 frame / memory / snapshot samples), so it reads NEUTRAL and
> cannot fail; obs-plan §1 and §10 record the gates VACUOUS with this entry as their owner; test-plan §1
> `perf-slo-check-arm-coverage` owes the script's per-arm tests · CARRY: the CI `release` job (macOS/Windows) restores
> the `lint-test-{os}` rust cache read-only instead of owning a key — the 10 GB repository cap cannot hold two more
> target caches (8.58 GB in use); that key still carries release dependencies from
> 2026-09-29-ci-wall-time-and-round-trips' cold round, but when `Cargo.lock` next changes `lint-test` re-saves without
> them and the release job builds them cold every round (~13+ min on Windows) until the cache allocation is revisited
> (owner assigned by the overseer at that chunk's wrap) · CARRY: mirror `agent-run.sh`'s boot end-status recorder in
> `scripts/agent-run.ps1` — the waiting wrapper writing `run/andromeda-pulse.{spawn,exit}` — so `harness:status`'s
> `ended` stops being always null on Windows (owner assigned by the overseer at the same wrap)

Operator directive at take-up (2026-09-30, `/andromeda-phase` arguments), verbatim:

> Perf-budget gate reads real samples (:140), with its two CARRYs. Founder rulings: CI speed is a priority; nothing
> deferred. Operator slots: conductor-builder is running NVDA screen-reader legs that need a quiet desktop, so ANY
> run that launches pulse-app or opens a window (self-verify, a perf sample run) is a slot even when the ports are
> free: STOP and ask, in research and in implement. Disk: 68 GB free, target/ 104 GB; plan a cargo clean --profile
> dev before heavy builds if under 60 GB. Diff-shaped probes name the chunk base fb93fca (W182).

Freight folded (`route.py pins`, row 140): one `CONTEXT:` block (377 chars) and two `CARRY:` blocks (534 + 264
chars), all quoted above in full. No `PREREQ` / `BLOCKED-ON` / `WATCH` on this entry.

CI verdict read at Setup 5a (every commit from the last wrap's flip through HEAD): `fb93fca` **green**, 13/13
checks, wall-clock 1297 s (ci#36710506171 + secret-scan#36710506165). Nothing red to disposition.

## What this chunk builds

1. **A perf-budget gate that can fail.** CI's perf-budget gate reads a log that actually carries frame, memory and
   snapshot samples, so a regression past a budget turns the gate RED instead of reading NEUTRAL. The budgets are
   the obs-plan §10 rows the script already encodes: frame p99 ≤ 33 ms, `buffer.memory_bytes` max ≤ 512 000 000,
   snapshot p99 ≤ 500 ms. They are not relaxed.
   - Coordinates re-verified at HEAD during the fold (VERIFIED at P3): `cargo xtask ci-gates` is the step at
     `.github/workflows/ci.yml:451-452` (the `boot` job, `shared-key: boot-Linux`). `run_ci_gates`
     (`xtask/src/main.rs:454`) hands `invoke_perf_slo_check` (`:705`) only `log_files.last()`, a single log. The
     script arms (`xtask/ci/perf-slo-check.{sh,ps1}`) select targets `metric.webgpu.frame_duration_ms`,
     `metric.buffer.memory_bytes` and `metric.snapshot.token_count_ms`, and map an empty stream to NEUTRAL (exit 0).
     The no-log branch also prints `perf-budget DEFERRED (no criterion bench yet)` (`:465`).
   - `[premise-corrected: ci#36710506171 at fb93fca carries 23 records, not 52; the 0-sample NEUTRAL→PASS
     mechanism holds]` The mechanism claim, with its marker text verbatim, is *"measured at
     2026-09-29-p-025-hue-shift-observable-made-gradable — … only the boot-smoke log (52 records, 0 frame / memory /
     snapshot samples), so it reads NEUTRAL and cannot fail"*. P3 re-derived it at the chunk base from the
     `logs-boot-Linux` artifact of `ci#36710506171`: 23 records spanning 14 ms, 0 samples of each of the three
     targets and 0 `buffer.tick`. The job log prints three NEUTRAL arms, then `ci-gates: perf-budget PASS`.
   - All three targets HAVE production emitters, and every one has an exact allowlist leaf carrying its graded
     field (research §Files inspected). So no producer is missing. What is missing is a CI run that exercises
     each one:
     - **memory** needs the app alive past its first buffer tick (+1.30 s locally), and the CI app lived about
       1.5 s;
     - **frame** needs a WebGPU adapter in the webview. `[premise-corrected at P4: app.boot.gpu.check is a
       hardcoded false (pulse-app/src/window.rs:91-96), so it proves nothing]` On the dev host, WebView2 exposed
       an adapter only under the Vulkan-SwiftShader flag set (research §P4 frame-adapter probes). A hosted
       runner's adapter is unmeasured;
     - **snapshot** runs only when the webview starts `snapshot.generate` / `investigate.run_action`. The
       sidecar's `generate_snapshot` runs over its own empty in-memory DB and has no snapshot leaf.
2. **Per-arm tests for the perf-slo-check script.** This closes test-plan §1's `perf-slo-check-arm-coverage`
   trigger: each arm (frame / memory / snapshot) is proven to PASS under budget, FAIL over budget and read NEUTRAL
   when empty, so a gate that can fail is also shown to fail.
3. **A guard against a vacuous gate** (VERIFIED at P3: the live PASS over three NEUTRAL arms is quoted above).
   Once the gate has a sample-bearing log, a CI run where every arm reads NEUTRAL must no longer pass silently.
   P4 decides the exact form: a sample-presence floor, or a hard fail when the sample-bearing log is absent.
   Its reach is bounded: the local `pre-push:linux` stage runs `ci-gates` over ONE seeded boot record
   (`xtask/src/pre_push.rs:45,249`), and `perf:load-profiles` reuses `invoke_perf_slo_check`. Both must stay
   green, so the guard is opt-in where samples are EXPECTED and never a flip of the script's NEUTRAL arm.
4. **CARRY — the release job's cache allocation.** The `release` job (macOS/Windows) builds its release
   dependencies warm on every round, including the first round after a `Cargo.lock` change. Today it restores
   `lint-test-{os}` with `save-if: false` (`ci.yml:217-222`), and that key loses the release dependencies at
   `lint-test`'s next re-save. The founder ruled that CI speed is a priority and nothing is deferred, so this
   CARRY is fixed in this chunk, not re-carried.
   - VERIFIED at P3 (`gh api …/actions/cache/usage`): 8 576 022 877 B across 6 entries, which matches the CARRY.
     The release Windows job restored `lint-test-Windows` with a full match and recompiled only the 16 workspace
     members (build 9 m 35 s). The fix must fit the 10 GB cap.
   - Measured context: at `fb93fca` the round's critical path is coverage at 21 m 37 s, and release Windows ran
     12 m 53 s. A cold release job (the CARRY's ~13 + min build plus about 3.3 min of setup) would still end
     before coverage. So today the CARRY costs runner minutes and headroom, not round wall-clock. It is fixed
     anyway, per the founder's "nothing deferred" ruling.
5. **CARRY — the Windows boot end-status recorder.** `scripts/agent-run.ps1`'s boot mirrors `agent-run.sh`'s
   waiting wrapper (`agent-run.sh:71-76`, which writes `andromeda-pulse.{spawn,exit}` beside the pidfile), so
   `harness:status` reports a real `ended` on Windows instead of an always-null one.
   - VERIFIED at P3: `agent-run.ps1` spawns the app with `Start-Process` (`:150`) and records no spawn/exit
     files. `harness:status` reads `pidfile.with_file_name("andromeda-pulse.exit")` through `read_ended`
     (`xtask/src/harness_status.rs:200,205`): one trimmed line of ≤ 48 ASCII graphic-or-space chars. A Windows
     `exit N`, even an NTSTATUS such as `exit 3221225477`, fits that grammar. `signal N` has no Windows analogue.
6. **The spec leaves follow the gate** (VERIFIED at P3: obs-plan §10's memory row and both CI-gates bullets carry
   the VACUOUS status with this entry as owner). obs-plan §1 and §10 record the perf gates as VACUOUS with this
   entry as their owner. The VACUOUS status goes away through wrap's amendment of the masters, never as a
   phase-time edit. The plan names these as expected amendments.

## Boundaries (non-goals)

- The budgets themselves (33 ms / 512 MB / 500 ms) are not changed.
- No new product feature, and no perf-emitter edit. P3's producer-existence check found all three targets emitted in
  production with exact allowlist leaves, so this chunk changes what CI RUNS, not what the product EMITS.
- `[amended at P5 val-1, intent-incomplete; operator-ruled at P4]` Two product-code touches ARE in scope:
  - `app.boot.gpu.check` stops asserting `gpu_available = false`. It is a hardcoded constant
    (`pulse-app/src/window.rs:91-96`), found by the P4 adapter probes. Overseer: "Fix in this chunk", under the
    nothing-deferred ruling.
  - `load_curated_markdown` is widened to `pub`, so the in-process producer drives the production snapshot path.
  - Every WebView2 flag stays out of product config. It lives only in the frame leg's child environment.

## Resolved at P4 (operator rulings, 2026-09-30)
- Frame arm: SwiftShader on the Windows release job (after three slot probes: 0 / 0 / 496 frames). A 0-sample or
  over-33 ms CI reading comes back as a decision, never a retune.
- Memory + snapshot producer: an in-process test in lint-test Linux. The memory budget is the rows × 256 B
  gauge, not RSS.
- Release cache: its own `release-${{ runner.os }}` key. Acceptance: ≤ 10 GB and a full-match restore on round 2.
- Arm logic: ported to xtask Rust, scripts deleted, and the no-`jq` class becomes a pinned FAIL.
- Span-level redaction, real-model incident surfacing and the Conductor return (P-075) are separate route entries,
  not folded here.
- The heartbeat-gap and zero-panic arms of `ci-gates` are untouched, except where sharing the new log changes what
  they read. P3 checks that they stay green on it.

## Operating constraints (directive, binding on P3 and /implement)

- **Operator slots.** Any run that launches `pulse-app` or opens a window (self-verify, a perf-sample run, a boot
  smoke) is an operator slot even when ports 4317/4318 are free: conductor-builder is running NVDA screen-reader
  legs that need a quiet desktop. STOP and ask before each one, in research and in implement.
- **Disk.** Measured 68 GB free, `target/` 104 GB at take-up. Before any heavy build, re-measure: under 60 GB free,
  run `cargo clean --profile dev` first.
- **Diff-shaped probes** name the chunk base `fb93fca` (W182).
- **CI speed is a priority.** A new sample-producing run must not lengthen the critical path of the CI round
  (measured wall-clock 1297 s at `fb93fca`) without the operator's word. P4 costs it.

## Open questions (for P3 / P4)

- Which run produces the samples in CI: the existing `boot` job's smoke (extended to drive frames and a
  snapshot), the `perf-slo-load` suite (`Cmd::PerfSloLoad`, `xtask/src/main.rs:288`), or a new dedicated leg? This
  depends on where each target's emitter lives and whether the frame sample needs a live webview (it cannot run
  headless on the Windows host).
- The release-cache allocation: give `release` its own key and evict or shrink another, have `lint-test` build the
  release dependency set so its re-save keeps them, or a split registry/target cache. It has to fit the 10 GB cap
  measured through `gh`, not assumed.
