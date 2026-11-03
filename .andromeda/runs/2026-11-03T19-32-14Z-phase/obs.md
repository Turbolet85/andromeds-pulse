# obs extract

## Relevance
partial. The chunk adds no instrumentation. Obs is involved only because the bumped crates (tokio, wasmtime, the tonic/axum TLS family via rustls/quinn-proto, zstd, zip) sit under the self-observation runtime and the CI gates that read its log.

## Constraints
- The self-observation crate set must stay at `tracing` 0.1, `tracing-subscriber` 0.3, `tracing-appender` 0.2 and `tracing-error` 0.2, with the same init order (per obs-plan §3 Tracing init). Research must check whether any upgrade, a tokio bump included, moves one of these crates or changes the `WorkerGuard` drain.
- No upgrade may link an OTel SDK into the product binary. The OTLP wire crates (`tonic`/`axum`/`prost`/`opentelemetry-proto`) parse wire format only (per obs-plan §1 Obs Scope Summary, its architectural choice). Research must check whether any bumped crate's new dependency tree pulls one in.
- wasmtime 48.0.5 → 48.0.8 must still resolve with the Cranelift-only feature. The plan treats this as a build-time gate, not runtime telemetry (per obs-plan §1 Obs Scope Summary, the Cranelift-only WASM backend row).
- A dependency bump does not count as behaviour-neutral just because `Cargo.toml` is unchanged. The plan records a lockfile-only bump (`libduckdb-sys` 1.10502 → 1.10505) that changed runtime behaviour, so it was verified on the wire (per obs-plan §10 DuckDB Connection Isolation & Load-Profile Operational Constraints, defect 4).
- The tokio 1.52.1 bump touches three mechanisms: the 15 s heartbeat (`tokio::time::interval`), the >45 s stall signal, and the drain-progress fields. All must hold in both programs (per obs-plan §10 Standard+ invariants). The SIGTERM/SIGINT listener must still write exactly one `app.exit` per process end (per obs-plan §7 Error Capture & Reporting).
- Code changed because a bumped crate's API changed must leave every emitted record's field set unchanged. Exact-leaf allowlists redact any field they do not name (per obs-plan §8 PII Scrubbing). The required fields of the log JSON schema must stay unchanged on every line (per obs-plan §6 Log Coverage).

## Patterns to follow
- To verify a bump, read the wire: record counts, 0 panics, and the ERROR set read from the log family. A compile is not enough. Defect 4 was closed this way (per obs-plan §10 DuckDB Connection Isolation & Load-Profile Operational Constraints).
- Use the gates that already read a built app's log: `cargo xtask ci-gates` (zero-spans, zero-panic) and `cargo xtask check:engine-log` (family, heartbeat-gap, progress, budget) in the `boot` job, plus `cargo xtask perf:budget --require memory,snapshot` in `lint-test` (per obs-plan §10 CI gates).
- If forced code touches an emit site, rely on the field-set equality guards under `pulse-app/tests/unit_observability_allowlist_*.rs`. They run there because of `[lib] test = false` (per obs-plan §8 PII Scrubbing).

## Anti-patterns to avoid
- NEVER link an OTel SDK into the product's self-observation runtime (per obs-plan §11 Universal (agent-driven specific)).
- NEVER define soft SLO budgets or relax a gate to get the upgraded build through. A formal budget must fail the build (per obs-plan §11 SLO).

## Contract bindings
- obs ↔ security: the advisory rule (no ignore-list entry where a safe upgrade exists, ID-scoped entries only) belongs to security-plan §Dependency Security, not obs. Obs adds only the crate-set and no-OTel-SDK limits above (per obs-plan §3 Tracing init).
- obs ↔ tests: the post-bump reading of the app is the `boot` job's `logs-boot-Linux` / `logs-engine-Linux` and `lint-test`'s `logs-perf-samples-Linux`, each uploaded with `if-no-files-found: error`. The `supply-chain` job's steps 10–18 are not obs gates (per obs-plan §9 CI Integration).

## Acceptance criteria contributions
- On the upgraded build, the `boot` job's `cargo xtask ci-gates` reads `zero-spans PASS` and `zero-panic PASS` (per obs-plan §10 CI gates).
- `cargo xtask check:engine-log` over the console engine cycle's log reads `engine-log: PASS`, including its heartbeat-gap arm (≤ 45 000 ms per engine tick target) and progress arm (per obs-plan §10 Standard+ invariants).
- `cargo xtask perf:budget --require memory,snapshot` reads PASS in `lint-test` (per obs-plan §10 Performance budgets).
- `cargo tree` for the product binaries shows no `opentelemetry_sdk`, and shows `tracing` 0.1, `tracing-subscriber` 0.3, `tracing-appender` 0.2 and `tracing-error` 0.2 (per obs-plan §3 Tracing init).
