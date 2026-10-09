### Bootstrap phases

1. **test-runner-install:** `cargo-nextest` 0.9.x (per Phase 2 research; provides per-process isolation for port-binding tests)
2. **5-command-discipline-wire:** Materialize `scripts/agent-run.sh` with 5-command bodies (boot / run / status / cleanup / logs) per Section 3 above
3. **status-endpoint-implement:** Wire TauRPC `health` command to return required fields (status / subsystems / pid / uptime_ms)
4. **log-format-bind-with-obs:** Configure `tracing-subscriber::fmt().json()` to emit JSON per-line format (binds with obs-plan §3 JSON schema)
5. **pid-file-commitment-wire:** Write `<data_dir>/run/andromeda-pulse.pid` at boot (data-dir-relative, per §3 PID file → Location); read + verify at cleanup
6. **test-data-bootstrap-wire:** Implement `MockTraceSpan` builder factories in `ingest` crate; `MockArrowBatch` in `buffer` crate; `MockMetricPoint` in `viz` crate; implement `rstest` fixtures for composition. **UNIMPLEMENTED — measured 2026-08-23 (chunk `2026-08-23-metrics-points-identity`): all THREE factories return 0 workspace hits**, so this item is an owed mandate, not shipped bootstrap. The `rstest` fixture half DID land and is in use. Self-bootstrapping (§7) is satisfied in practice by OTLP ingest through the live receiver plus per-test in-memory DuckDB seeding; tracked as §1 `test-data-bootstrap-factories-unimplemented`
7. **coverage-tooling-install:** `cargo-llvm-cov` 0.8.5 for LCOV/Cobertura output
8. **quality-gate-config-emit:** GitHub Actions workflow enforces: line coverage ≥ 75%, branch ≥ 70%, zero-flakiness (no retries), perf-budget < 100ms p99 for RPC latency
