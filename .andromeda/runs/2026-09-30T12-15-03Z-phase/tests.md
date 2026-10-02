# tests extract

## Relevance
relevant: the chunk closes the test-plan §1 `perf-slo-check-arm-coverage` trigger. It also changes the CI harness surfaces §9 owns (the `boot` job's `ci-gates` feed and the `release` job cache) and the §3 `status` `ended` field on the ps1 twin.

## Constraints
- test-plan §1 Pending coverage triggers → `perf-slo-check-arm-coverage` requires a committed per-arm assertion for each arm: an empty stream reads NEUTRAL, an in-budget stream reads PASS, an over-budget stream reads FAIL. For this chunk that means each of the three targets (frame, memory, snapshot). A one-time live observation does not discharge it. The §1 rows `viz-read-connection-router-wiring-coverage` and `harness-cleanup-verdict-and-boot-spawn-shell-coverage` name this class "one-time proof standing in for a gate". Whether the `.sh` and `.ps1` arms both need pins, and in which harness (xtask unit test vs script-level leg), is research's question.
- test-plan §10 Performance budgets → Frame-budget assertion: the asserted frame budget is obs-plan §10's ms-form `metric.webgpu.frame_duration_ms` p99 ≤ 33 ms. No test may assert an fps number. The §10 build-failure conditions list "Performance budget regression (p99 > threshold)" as a build failure. Scope Boundaries fix that no budget is relaxed.
- test-plan §3 Per-chunk gate discipline requires the plan's `## Test Commands` to carry the full standard gate set in its stated order: `capability-drift` BEFORE the workspace nextest, the `--features mcp-server` `emit_taurpc_bindings` regen as the last cargo-adjacent step, and then `git diff --quiet fb93fca -- pulse-app/ui/src/bindings/index.ts`. The webview npm gates apply only if `pulse-app/ui/**` is touched. The boot-smoke gate is conditional: it becomes mandatory if an emitter edit touches `pulse-app/src/main.rs`, `pulse-app/src/observability.rs` or `crates/ui-bridge/src/`, and any such run is an operator slot (scope §Operating constraints).
- test-plan §3 `status` + §3 PID file require the Windows mirror to write the same bytes the sh wrapper writes, beside the pidfile:
  - `run/andromeda-pulse.spawn` holds the app pid.
  - `run/andromeda-pulse.exit` holds `exit N` or `signal N (NAME)`, bounded to one line of ≤ 48 printable ASCII characters. Anything else reads as no record, and the pure `read_ended` parser is unit-pinned to that rule.
  - The harness writes these files, never the product binary.
  - §3 currently records `ended` as "always null under `agent-run.ps1`". §1 `harness-cleanup-verdict-and-boot-spawn-shell-coverage` owes a ps1 boot-cycle leg.
- test-plan §3 Per-chunk gate discipline (the `check:ingest-progress` NEUTRAL paragraph and the §10 Load-profiles "NEUTRAL-tolerant obs check scripts") states that a NEUTRAL arm is what makes a check safe to run unconditionally when nothing booted. The scope's vacuous-gate guard (item 3) must not break that property for the scripts' other callers (`perf:load-profiles`, local `pre-push:linux` `ci-gates` over a seeded non-metric data dir). Whether the guard lives in `run_ci_gates` or in the script is P4's decision.
- test-plan §9 Pipeline structure sets the cache facts the CARRY fix works within:
  - `lint-test` saves `lint-test-${{ runner.os }}`, and `release`, `mcp-test` and `a11y` restore it read-only.
  - `boot` owns `boot-Linux`, and `supply-chain` restores it.
  - `coverage` is registry-only because an instrumented target cache "would not fit the 10 GB repository cap".
  - Any re-allocation must keep those consumers' restores valid and must update the §9 cache column. That update is a wrap-time amendment.
- test-plan §9 E2E row states that the frame sample's natural source, the `webview-drive` leg, is not wired into any workflow, and that it skips clean without an `msedgedriver`. A CI wiring "must assert the leg RAN (its per-stage report), never just its exit code". §1 performance-budget: WebGPU frame-rate validation is "deferred to tauri-driver headful E2E suite". So which CI run can produce frame samples is research's question, not assumed.

## Patterns to follow
- test-plan §3 scenario legs (`smoke:gap-resume` / `smoke:external-resolve` / `smoke:hue-shift` / `smoke:discovery`) have a three-way exit: 0 PASS, 1 FAIL, 2 INCONCLUSIVE. An unmet precondition (no samples, feed never delivered) reports INCONCLUSIVE, never PASS, and each leg proves its own preconditions. This is the shape for the sample-presence floor in scope item 3.
- test-plan §3 Direct-binary smoke variant: MEASURE-FIRST runs RED at the base (`fb93fca`) and GREEN after the fix, each on its own fresh data dir. A gate that reads NEUTRAL over an empty stream is the vacuous-pass shape: the RED leg's evidence is the ABSENCE of samples (0 frame / memory / snapshot records), not a clean log. Decide the leg's shape before choosing its evidence.
- test-plan §3 `status`: a pure classifier fn is unit-pinned per arm (`classify(pid, alive, newest_log)`, `read_ended`). The per-arm perf-slo tests and any new floor or verdict logic follow this pure-fn-plus-per-arm-pin form.
- test-plan §2 Test directory conventions: xtask logic is tested co-located (`#[cfg(test)] mod tests` in `xtask/src/*`). Any `pulse-app` emitter probe goes in `pulse-app/tests/*.rs`; the flat-zero ratchet `pulse_app_src_carries_no_new_dead_test_attributes` reds any `#[test]` in `pulse-app/src/` other than `main.rs`.
- test-plan §3 `run`: narrow a `pulse-app` run with `--workspace -E`, not `-p`. The §9 `lint-test` row: `perf:slo-load` selects `--workspace -E 'binary(perf_slo_10k_spans)'` to reuse the test build. A new sample-producing CI step reuses an existing build rather than adding a compile, in line with the scope's CI-speed ruling.

## Anti-patterns to avoid
- test-plan §11 CI / Quality: NEVER lower a threshold to pass the build, and NEVER add retry-once. A red perf gate is fixed at the cause, and the 33 ms / 512 MB / 500 ms budgets stay as they are.
- test-plan §11 E2E / Universal: NEVER synchronize with `sleep(N)` (wait on an explicit signal such as a `harness:status` verdict or a log record), and NEVER use real time without injection in unit pins. The per-arm tests feed synthetic metric streams, never wall-clock-dependent runs.
- test-plan §11 CI: NEVER reference a third-party action by floating tag. Any action the cache re-allocation adds or changes is pinned by 40-char SHA.

## Contract bindings
- tests ↔ obs: the §10 frame-budget authority is obs-plan §10 (ms-form metric names and budgets; the gates are recorded VACUOUS there, and the owner is this entry). The §3 `logs` rotated `agent-latest.jsonl*` family is the log `ci-gates` reads (obs-plan §3 Log file location). Whether `run_ci_gates` handing only `log_files.last()` misses samples in another family member is research's question.
- tests ↔ harness scripts: `harness:status` `ended` (§3) reads the `run/andromeda-pulse.exit` grammar that `agent-run.sh` writes. `agent-run.ps1` must match it byte-for-byte (§3 PID file lifecycle).
- tests ↔ security: SHA-pinned actions plus harden-runner as the first step for any new or edited CI job (test-plan §11 CI; security-plan §Supply chain + CI).

## Acceptance criteria contributions
- For each of frame, memory and snapshot, a committed test collected by name proves: empty ⇒ NEUTRAL (exit 0), in-budget ⇒ PASS, over-budget ⇒ FAIL. This discharges §1 `perf-slo-check-arm-coverage` (per test-plan §1 Pending coverage triggers).
- A RED/GREEN measurement shows the CI perf-budget gate reading a sample-bearing log. The RED leg at `fb93fca` records 0 samples, or NEUTRAL, as its evidence. The GREEN leg shows at least one sample per arm graded against the unchanged obs-plan §10 budgets. A run whose every arm reads NEUTRAL fails, or reports INCONCLUSIVE, rather than passing (per test-plan §3 Direct-binary smoke variant + scenario-leg precondition rule).
- After an `agent-run.ps1` boot and a later app exit, `cargo xtask harness:status` reports a non-null `ended` matching the §3 grammar (`exit N` or `signal N (NAME)`, ≤ 48 printable ASCII characters). The live leg is an operator slot (per test-plan §3 `status` / PID file).
- The §3 standard gate set passes in its stated order, closing on `git diff --quiet fb93fca -- pulse-app/ui/src/bindings/index.ts` exit 0. The CI round stays green, including the `boot` job's `ci-gates` heartbeat-gap and zero-panic arms on the new log (per test-plan §3 Per-chunk gate discipline; §9 Build failure conditions).
