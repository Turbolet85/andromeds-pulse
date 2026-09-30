# obs extract

## Relevance
relevant — the chunk's core deliverable is the obs-plan §10 perf-budget CI gates (frame / memory / snapshot), which §1 and §10 record as VACUOUS and name this route entry as their owner; the two CARRYs (release cache, Windows end-status recorder) are harness/CI plumbing with only an indirect obs tie.

## Constraints
- The three budgets are fixed by obs-plan §10 Performance budgets: `metric.webgpu.frame_duration_ms` p99 ≤ 33 ms, `metric.buffer.memory_bytes` max `value` ≤ 512_000_000, `metric.snapshot.token_count_ms` p99 ≤ 500 ms. They are enforced as written, never relaxed or softened (per obs-plan §10 CI gates; §11 SLO "NEVER define soft SLO budgets").
- The sample shapes the gate reads are the ones §1 perf-budget-instruments and §5 Metric Coverage specify: frame and snapshot carry `duration_ms`, memory carries `value` (per tick on the `buffer.tick` heartbeat, 15 s). Percentiles are computed agent-side from raw per-event values; no precomputed quantile is emitted (per obs-plan §5 naming + §11 Metrics "NEVER skip p50/p95/p99 coverage"). Whether production emitters exist today for all three targets, and in which run each can fire in CI, is research's question (the frame emitter is frontend-bridged through `telemetry.frontend.record_frame_ms` per §4 Scenario P1, so it needs a live webview).
- Each emitted field the gate parses must survive the self-observation allowlist: obs-plan §8 PII Scrubbing states that a `metric.*` target without its OWN exact leaf falls back to the bare `metric` key, which keeps only `value` / `unit` / `module` and redacts every other field. A `duration_ms` rendered `"<redacted>"` would be an unreadable sample. Whether `metric.webgpu.frame_duration_ms` and `metric.snapshot.token_count_ms` currently have exact leaves carrying `duration_ms` is research's question.
- The two-state posture holds per obs-plan §10 (frame row) and §10 Load-profile constraints: check scripts over `agent-latest.jsonl` stay NEUTRAL-tolerant (an absent stream is NEUTRAL, not FAIL) so headless verification and full-app sessions share one script set. The chunk's vacuity guard (scope item 3) must therefore sit at the gate/caller layer that KNOWS a sample-bearing log was expected, not by turning the script's empty-stream arm into a FAIL.
- Scripts stay scoped to the run's time window (`write_run_window_log`, per obs-plan §10 frame row) so a longer or shared log does not produce false cross-session heartbeat-gap FAILs; the heartbeat-gap (>45 s per `ingest/buffer/viz/plugins.tick`) and zero-panic (`app.panic.fatal`) arms of §10 CI gates must stay green on whatever log the gate now reads.
- Any new or widened sample-producing emitter stays aggregate-only with bounded labels (per obs-plan §5 cardinality discipline; §11 Metrics) — `wgpu_backend` / `webview_backend` / `timing_method` / `token_budget` enumerated values only, never a service name or trace id.
- CI telemetry artifacts are retained: the log the gate reads is uploaded `if: always()` under a run-unique artifact name (upload-artifact v4 refuses duplicates), per obs-plan §9 Telemetry artifact handling.

## Patterns to follow
- `metric.{module}.{measure}` targets emitted via `tracing::info!(target: "metric.*", ...)` on the JSON file sink; the gate aggregates post-run with `jq`-style selection by `.target` and `.fields.<field>` (obs-plan §5 naming; §10 Performance budgets assertion column).
- One exact allowlist leaf per `metric.*` target, carrying every field its emit site emits, asserted by a `pulse-app/tests/unit_observability_allowlist_*.rs` guard with a fallback-discrimination test (obs-plan §8, delegated-timing leaves precedent).
- Existing scenario-leg shape for a live, gradable sample run — `cargo xtask smoke:hue-shift` / `smoke:discovery` grading a `metric.*` observable from the run-window log (obs-plan §8 delegated-timing entries); the `perf-slo-load` / load-profile suites are the other in-plan producer of sustained `buffer.tick` samples (obs-plan §10 Load-profile constraints).
- Liveness-plus-progress thinking from §3 Heartbeat ticks: presence of a record is not proof of the property — the analogue here is that a PASS with zero samples is not proof the budget held, which is exactly what the vacuity guard encodes.

## Anti-patterns to avoid
- NEVER define soft SLO budgets — a formal SLO MUST fail the build when exhausted; a gate that can only read NEUTRAL is a soft budget (per obs-plan §11 SLO).
- NEVER log in a hot path at `info` or add per-frame work that inflates the log beyond what the §5 per-frame (~60 Hz) emission already specifies; profile before instrumenting tight loops (per obs-plan §11 Telemetry Strategy + Logs).
- NEVER link an OTel SDK or point any exporter at own `:4317`/`:4318` to generate samples — samples come from the `tracing` JSON sink only (per obs-plan §11 Universal).

## Contract bindings
- obs ↔ tests: the gate's per-arm behaviour (PASS under budget / FAIL over / NEUTRAL empty) is the test-plan §1 `perf-slo-check-arm-coverage` trigger; obs-plan §10 supplies the thresholds and field shapes the tests must fixture.
- obs ↔ tests harness: `harness:status`'s `ended` (CARRY 2) consumes the boot end-status records the harness writes beside the pidfile; the record grammar is the test-plan §3 harness contract, not an obs-plan field — obs only requires the log path/location stay per obs-plan §3 Log file location.
- obs ↔ security: any field newly admitted to the allowlist is bounded/aggregate so the §8 scrub posture and security-plan §Logging hold.
- obs ↔ CI (arch): the release-cache CARRY touches `ci.yml` but no telemetry artifact; it must not drop or rename the `logs-*` artifacts obs-plan §9 relies on.
- Expected wrap amendments: obs-plan §1 perf-budget-instruments frame row, §10 Performance budgets (frame + memory rows) and §10 CI gates (perf-budget + snapshot bullets) carry the VACUOUS status with this entry as owner — retired by wrap's master amendment, not a phase-time edit.

## Acceptance criteria contributions
- The CI perf gate reads a log with ≥1 sample for each of `metric.webgpu.frame_duration_ms`, `metric.buffer.memory_bytes` and `metric.snapshot.token_count_ms`, and an injected over-budget sample (frame p99 > 33 ms, memory `value` > 512_000_000, snapshot p99 > 500 ms) turns the gate RED (per obs-plan §10 CI gates + Performance budgets).
- A CI run in which the expected sample-bearing log is absent or every perf arm is empty FAILS the gate instead of passing NEUTRAL, while the check script itself stays NEUTRAL-tolerant for headless runs (per obs-plan §10 Performance budgets two-state posture + §11 SLO).
- The fields the gate parses render as numbers at the wire, not `"<redacted>"` — 0 `<redacted>` on the three perf targets in the gate's log (per obs-plan §8 PII Scrubbing `metric` fallback rule).
- Heartbeat-gap and zero-panic gates stay green on the new log, and the log is uploaded as a CI artifact `if: always()` (per obs-plan §10 CI gates + §9 Telemetry artifact handling).
