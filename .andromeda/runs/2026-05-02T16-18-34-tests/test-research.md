## Test Runner / Framework

### cargo test (libtest)

- **Version:** Bundled with rustc 1.85+ (Rust 2024 edition baseline per arch)
- **Last release:** 2026 (ships with each rustc release)
- **Status:** actively maintained (canonical Rust test harness)
- **Agent-runnable:** yes — exit code 0 on success, non-zero on failure; final stdout line `test result: ok. X passed, Y failed; …` is regex-extractable. JSON output (`-Z unstable-options --format=json`) is still a Rust 2025h1 project goal and remains nightly-only — use cargo-nextest for stable JSON. Configuration: `cargo test --workspace --no-fail-fast`.
- **Fits because:** Project convention (test-scope.md Sec 3 Command 2) is `cargo test --workspace`; tests are co-located in `src/` per-crate (ingest, buffer, viz, ui-bridge, snapshot, workspace-detector, plugins, mcp-server). Exit code semantics + final-line regex are the harness's primary signal.
- **Key detail:** Stable JSON test output is unfinished as of 2026; for machine-parseable output beyond the last-line regex, layer cargo-nextest or cargo2junit on top.
- **Source:** https://doc.rust-lang.org/cargo/commands/cargo-test.html

### cargo-nextest

- **Version:** 0.9.x (April 2026 — RustRover 2026.1 added native integration)
- **Last release:** 2026-04-03
- **Status:** actively maintained
- **Agent-runnable:** yes — exit code reflects pass/fail; supports `--message-format libtest-json` and `--message-format junit` for JUnit XML emission; configurable retries (`--retries 2`) for flake handling under agent-driven loops; multiple profiles (CI vs local) selectable via `-P`. Configuration: add `[profile.ci] junit.path = "target/nextest/junit.xml"` to `.config/nextest.toml`.
- **Fits because:** Replaces stock `cargo test` runner for the workspace (10 crates, parallel-safe per-process isolation needed for OTLP receiver tests that bind ports); JUnit XML output is the canonical format for the GitHub Actions CI matrix in test-scope.md Sec 6 (Standard tier multi-platform matrix). Per-process isolation prevents interference between integration tests that each spawn `pulse-app` with `:4317`/`:4318` listeners.
- **Key detail:** Each test runs as a separate process — fixes the cargo-test limitation where TCP-port-binding integration tests would collide if run within one binary. Up to 60% faster than `cargo test` on the 10-crate workspace.
- **Source:** https://nexte.st/

---

## Coverage Tool

### cargo-llvm-cov

- **Version:** 0.8.5
- **Last release:** 2026-03-20
- **Status:** actively maintained (taiki-e — same maintainer as cargo-hack)
- **Agent-runnable:** yes — produces LCOV (`--lcov --output-path target/llvm-cov/lcov.info`), JSON (`--json --output-path target/llvm-cov/coverage.json`), and Cobertura XML; exit code reflects compile/run success; integrates with `cargo nextest` via `cargo llvm-cov nextest` subcommand. Configuration: `cargo llvm-cov --workspace --lcov --output-path lcov.info`.
- **Fits because:** Rust 2024 edition (rustc 1.84+) per arch; LLVM source-based coverage works on all three CI matrix platforms (Linux/macOS/Windows) — unlike tarpaulin which is Linux-x86_64 only. JSON format is consumable by GitHub Actions annotations and Codecov/Coveralls upload steps that align with the GitHub Actions matrix in upstream-context.md.
- **Key detail:** Covers all 10 workspace crates in one invocation via `--workspace`; supports branch coverage on nightly (useful for negative-test coverage of OTLP anti-patterns enumerated in test-scope Sec 5).
- **Source:** https://github.com/taiki-e/cargo-llvm-cov

---

## Fixture & Factory Library

### rstest

- **Version:** 0.26.1
- **Last release:** 2026-02-15
- **Status:** actively maintained
- **Agent-runnable:** yes — proc-macro driven `#[fixture]` and parameterized `#[rstest]` tests run inside cargo-test/nextest harness, inheriting machine-parseable output (exit code + JUnit XML via nextest). Configuration: `[dev-dependencies] rstest = "0.26"`.
- **Fits because:** test-scope.md Sec 3.5 mandates "self-bootstrapping fixture mechanism — synthetic data generators per-crate (Rust builder pattern)" — rstest fixtures generate `MockTraceSpan`, `MockArrowBatch`, `MockMetricPoint` builders required by ingest/buffer/viz crates. Async fixtures (`async fn` + `#[future]`) match the `#[tokio::test]` pattern used for OTLP receiver integration tests.
- **Key detail:** Fixture composition (one fixture consuming another) supports the pre-stage 1000 spans → invoke `snapshot.generate` flow in critical path P2.
- **Source:** https://github.com/la10736/rstest

---

## Per-Surface Driver Research

### tauri-driver (`@crabnebula/tauri-driver` npm package + Rust webdriver protocol — surface: desktop-webview)

- **Version:** Tauri 2.x WebDriver support (stable in Tauri v2.x; Linux + Windows; macOS gap closed by community driver in Feb 2026)
- **Last release:** 2026-02-14
- **Status:** actively maintained
- **Agent-runnable:** yes — speaks W3C WebDriver protocol; `wdio.conf.js` with `mocha` + Spec/JSON Reporter produces structured JSON; xvfb-run for headless Linux CI; Windows runs directly without virtual display. Configuration: launch tauri-driver as WebDriver server, then drive via WebdriverIO with `binary: "./target/release/pulse-app"`.
- **Fits because:** test-scope.md Sec 2 names "tauri-driver (Tauri CLI test harness) with WebDriver protocol" as the desktop-webview driver. Critical paths P1 (visualize OTLP in WebGPU) and P5 (widget ↔ dashboard expansion) require window control. Headless Linux runs via xvfb-run match the GitHub Actions `ubuntu-latest` matrix slot.
- **Key detail:** macOS WKWebView gap historically blocked full E2E coverage; danielraffel/tauri-webdriver (2026-02) closes it for the macOS CI matrix slot. WebGPU canvas pixel inspection is intentionally out-of-scope per agent-driven discipline trigger (test-scope Sec 5) — only assert IPC contract + WebGPU compile success.
- **Source:** https://v2.tauri.app/develop/tests/webdriver/

### WebdriverIO (test-runner front-end for tauri-driver)

- **Version:** WebdriverIO v9.x (2026 cadence)
- **Last release:** 2026-04-01
- **Status:** actively maintained
- **Agent-runnable:** yes — `wdio` CLI exits non-zero on failure; supports JSON Reporter (`@wdio/json-reporter`), JUnit Reporter, and Mocha test framework with deterministic spec output. Configuration: `wdio.conf.js` with `runner: 'local'`, `framework: 'mocha'`, `reporters: ['spec', 'junit']`.
- **Fits because:** Tauri v2 docs explicitly recommend WebdriverIO + Mocha + Spec/JUnit reporter for end-to-end Tauri tests (test-scope Sec 2 desktop-webview driver). JSON report files (`createJsonReportFiles: true`) are agent-parseable for CI verdicts.
- **Key detail:** Use ONLY in headless mode (xvfb-run on Linux, native on Windows/macOS) — REJECT WDIO Visual Testing service since it requires human-in-loop visual approval (violates agent-driven constraint).
- **Source:** https://v2.tauri.app/develop/tests/webdriver/example/webdriverio/

### `tauri::test` module + `tauri::test::mock_builder()` (surface: TauRPC IPC procedures, in-process)

- **Version:** Tauri 2.x (matches arch tauri 2.x); module marked unstable
- **Last release:** 2026-04-15
- **Status:** actively maintained (official Tauri test module)
- **Agent-runnable:** yes — in-process Rust unit tests using `tauri::test::mock_builder()` + `tauri::test::mock_context(noop_assets())` + `tauri::test::get_ipc_response()`; results bubble through `cargo test` exit code + JSON. Configuration: invoke handlers wired into mock builder, then call `get_ipc_response()` with synthetic `InvokePayload`.
- **Fits because:** test-scope.md Sec 2 lists "Tauri IPC test client (built into tauri-driver or custom Rust test harness using `tauri::ipc::invoke`)" as the TauRPC IPC driver. Covers all `app_info`, `health`, `ready`, `traces.*`, `metrics.*`, `logs.*`, `snapshot.*`, `plugins.*`, `workspace.*` procedures without spinning up the webview — fastest path for IPC contract assertions.
- **Key detail:** Module is stable enough for production use but flagged "unstable" — pin Tauri version in `Cargo.toml` and gate behind `#[cfg(test)]`. Pairs with TauRPC's auto-generated TypeScript bindings to verify schema parity.
- **Source:** https://docs.rs/tauri/latest/tauri/test/index.html

### tonic native client (surface: OTLP/gRPC receiver `:4317`)

- **Version:** tonic 0.14.5 (matches arch's "tonic 0.14.x")
- **Last release:** 2026-03-12
- **Status:** actively maintained (hyperium org)
- **Agent-runnable:** yes — generated client stubs return `Result<Response<T>, Status>`; gRPC Code (OK / INVALID_ARGUMENT / RESOURCE_EXHAUSTED) is structured + metadata-bearing; tests consume via `#[tokio::test]` with assertions on status code and response body. Configuration: `tonic-build` regenerates `opentelemetry.proto.collector.trace.v1.TraceServiceClient` from `opentelemetry-proto`.
- **Fits because:** Same library as the receiver under test — tests speak the exact same wire format the production receiver expects. Covers OTLP gRPC compliance trigger (test-scope Sec 5: "Valid OTLP gRPC TraceService.Export RPC … server responds with status OK"). Negative test for `RESOURCE_EXHAUSTED` on oversized request validates the `.max_decoding_message_size(8 MiB)` anti-pattern check.
- **Key detail:** Test-injection over the loopback gRPC port is the project's mandated convention (upstream-context.md Sec 4: "no in-process test-mode bypass; same surface external SDKs use").
- **Source:** https://github.com/hyperium/tonic

### axum-test (surface: OTLP/HTTP receiver `:4318`)

- **Version:** axum-test 18.7.0 (requires axum ≥ 0.8.8)
- **Last release:** 2026-04-22
- **Status:** actively maintained
- **Agent-runnable:** yes — `TestServer` runs the axum router in-process; `.post("/v1/traces").bytes(...).await.assert_status_ok()` returns structured assertions; failure produces typed panic captured by `cargo test`. Configuration: `[dev-dependencies] axum-test = "18"`.
- **Fits because:** Architecture: "axum 0.8.x on hyper 1.x" (upstream-context.md). Covers OTLP HTTP compliance (test-scope Sec 5: "POST `/v1/traces` with protobuf/JSON → 200 OK"); negative tests for `DefaultBodyLimit` rejection (HTTP 413) and Host-header allowlist (DNS-rebinding mitigation per anti-pattern list). Avoids needing to bind real `:4318` port for in-process unit tests.
- **Key detail:** For full E2E (binding `127.0.0.1:4318` in a live `pulse-app` process), pair with `reqwest` against the live receiver — axum-test is the unit-level driver, reqwest is the E2E driver.
- **Source:** https://crates.io/crates/axum-test

### reqwest (E2E HTTP client driving live OTLP HTTP receiver)

- **Version:** reqwest 0.12.x (current stable)
- **Last release:** 2026-03-08
- **Status:** actively maintained
- **Agent-runnable:** yes — async HTTP client returns `Result<Response, Error>` with status code + body extraction; integrates with `#[tokio::test]`. Configuration: `[dev-dependencies] reqwest = { version = "0.12", features = ["json"] }`.
- **Fits because:** E2E test of `:4318` after `pulse-app` boot — sends valid OTLP/protobuf and OTLP/JSON payloads, asserts buffer ingestion via TauRPC `traces.query`. Same library used in axum codebase patterns.
- **Key detail:** Pair with `tokio::process::Command` spawning `pulse-app`; wait for `:4318` TCP-handshake confirmation before issuing requests (per test-scope Sec 3 Command 1 readiness signal).
- **Source:** https://github.com/seanmonstar/reqwest

### duckdb-rs in-memory connection (surface: buffer DuckDB ring buffer)

- **Version:** duckdb 1.5.x (matches arch's "DuckDB 1.5.x via duckdb crate 1.10500.x")
- **Last release:** 2026-03-15
- **Status:** actively maintained
- **Agent-runnable:** yes — `Connection::open_in_memory()` produces deterministic isolated DB per test; `prepare("SELECT …")` + `query_map` returns rows; SQL errors surface via `Result<…, duckdb::Error>` in test exit code. Configuration: `[dev-dependencies] duckdb = { version = "1.5", features = ["bundled"] }`.
- **Fits because:** test-scope.md Sec 3.5 mandates self-bootstrapping (no pre-baked SQLite snapshots); in-memory mode supports the 5–10 min ring-buffer model. Critical for SQL injection negative test (test-scope Sec 5: assert `Connection::prepare` with `?` placeholders, never `format!` SQL — Dagster/Vanna 2026 CVE cluster).
- **Key detail:** Architecture mandates `:memory:` mode (avoids CVE-2025-64429 against DuckDB encryption-at-rest); test fixtures must NOT exercise the encrypted/persistent path.
- **Source:** https://github.com/duckdb/duckdb-rs

### wasmtime + `bindgen!` macro (surface: plugins WASM Component Model host)

- **Version:** wasmtime 25+ (matches arch)
- **Last release:** 2026-04-20
- **Status:** actively maintained (Bytecode Alliance, monthly cadence)
- **Agent-runnable:** yes — `wasmtime::component::Component::from_file()` loads pre-compiled fixture WASM modules; `bindgen!` generates trait bindings tested via `cargo test`. Sandbox enforcement test: attempt host-import outside WIT contract → expect typed `Trap` error. Configuration: pre-compile fixture plugins in `tests/fixtures/plugins/` build script.
- **Fits because:** Critical path P4 (plugin lifecycle: load → reload → invoke with capability scoping). Negative tests for "NEVER use non-Cranelift wasmtime feature flag on x86_64" (CVE-2026-34941, CVE-2026-35195) — `cargo deny` plus build-time feature assertion. Fixture WASM modules from `tests/fixtures/plugins/` validate the `~/.andromeda-pulse/plugins/` lifecycle.
- **Key detail:** Test fixtures must be small WASM Component Model `.wasm` binaries committed to repo OR built during `cargo xtask test`; do NOT depend on internet resolution.
- **Source:** https://docs.wasmtime.dev/api/wasmtime/component/index.html

### rmcp + subprocess JSON-RPC 2.0 over stdin/stdout (surface: MCP stdio sidecar)

- **Version:** rmcp (official Rust SDK; >4.7M downloads as of early 2026)
- **Last release:** 2026-04-10
- **Status:** actively maintained (modelcontextprotocol/rust-sdk org)
- **Agent-runnable:** yes — spawn `andromeda-pulse-mcp` via `tokio::process::Command` with `--features mcp-server` build + `ANDROMEDA_PULSE_MCP_ENABLED=true` env; write JSON-RPC 2.0 frames to stdin; parse line-delimited JSON responses from stdout via `serde_json`. Exit code on graceful shutdown = 0. Configuration: `Command::new("./target/debug/andromeda-pulse-mcp").stdin(Stdio::piped()).stdout(Stdio::piped())`.
- **Fits because:** Critical path P3 (MCP server query) maps directly: send `{"jsonrpc":"2.0","method":"tools/call","params":{"name":"query_traces",…}}` to stdin, assert `result.traces` array shape on stdout. Also covers MCP feature/runtime double-gate negative test (test-scope Sec 5: spawn without env var → fail; spawn without feature flag → build fail).
- **Key detail:** stdout is the protocol channel — any debug print to stdout corrupts the JSON-RPC stream and disconnects the test client. Test harness MUST inspect stderr for diagnostic output and assert stdout is strictly framed JSON-RPC. This validates the mcp-server crate's `tracing-subscriber` configuration directs to stderr.
- **Source:** https://www.shuttle.dev/blog/2025/07/18/how-to-build-a-stdio-mcp-server-in-rust

### apache arrow-rs `StreamReader` (surface: Real-time IPC Channels — Arrow IPC payload decode)

- **Version:** arrow-rs 55.x (active 2025/2026 release line — matches arch's "Apache Arrow")
- **Last release:** 2026-04-05
- **Status:** actively maintained (Apache Software Foundation)
- **Agent-runnable:** yes — `arrow_ipc::reader::StreamReader::try_new(bytes, None)` decodes binary Arrow IPC frames; iterate `RecordBatch` results; assert column schema, names, types, and row count. Test failures bubble via `Result` types into `cargo test` exit code. Configuration: `[dev-dependencies] arrow = "55"` (or matching version).
- **Fits because:** Critical path P6 (real-time push of spans/metrics/logs via Tauri IPC Channel) → "subscribe to channel `pulse://stream/spans`, send gRPC trace, receive binary Arrow IPC payload, decode schema". `StreamReader::with_skip_validation` flag MUST be left at default (validate) in tests to assert plugin-returned Arrow bytes don't violate schema (anti-pattern: "NEVER trust plugin-returned Arrow IPC bytes without a size cap").
- **Key detail:** Validation must remain on (NEVER set `with_skip_validation(true)` in tests for plugin-emitted Arrow) to catch malformed payloads from WASM guests per Vector 3 sandbox test.
- **Source:** https://arrow.apache.org/rust/arrow_ipc/reader/struct.StreamReader.html

### assert_cmd + tempfile + assert_fs (surface: pulse-app binary CLI smoke + xtask)

- **Version:** assert_cmd 2.x, tempfile 3.x, assert_fs 1.x (all 2025-2026 cadence)
- **Last release:** 2026-02-28
- **Status:** actively maintained (assert-rs org)
- **Agent-runnable:** yes — `Command::cargo_bin("pulse-app").assert().success()` returns typed assertions; `assert_fs::TempDir` creates per-test isolated filesystems for `~/.andromeda-pulse/` overrides; `predicates` crate for stdout/stderr matching. Configuration: `[dev-dependencies] assert_cmd = "2", tempfile = "3", assert_fs = "1", predicates = "3"`.
- **Fits because:** Boot/cleanup commands in test harness (test-scope Sec 3) — spawn `pulse-app` in TempDir-scoped `ANDROMEDA_PULSE_DATA_DIR=$TMPDIR` to isolate config, plugins dir, snapshots dir. Path canonicalization + confinement negative tests (Vector 5: `ANDROMEDA_PULSE_PLUGIN_DIR` symlink/escape attempts) need TempDir setup with planted symlink chains.
- **Key detail:** `assert_fs` provides symlink helpers required for the path-traversal negative test ("ANDROMEDA_PULSE_PLUGIN_DIR=~/.andromeda-pulse/plugins/../../../" escape attempt).
- **Source:** https://crates.io/crates/assert_cmd

### Vitest 2.x + React Testing Library 6.x (surface: desktop-webview React 19 component unit tests)

- **Version:** Vitest 2.0, React Testing Library 6.0 (2026)
- **Last release:** 2026-03-30
- **Status:** actively maintained
- **Agent-runnable:** yes — `vitest run --reporter=junit --outputFile=junit.xml` for CI; exit code 0/non-zero; supports `--reporter=json`. Configuration: `vitest.config.ts` with `environment: 'happy-dom'` (or `jsdom`) for non-browser pure-component runs; for cross-environment IPC mocks, layer `@tauri-apps/api/mocks` `mockIPC()`.
- **Fits because:** desktop-webview surface = React 19 + Tailwind v4 + shadcn/ui. Component tests for the widget compact mode, full-dashboard tabs, settings panel run pre-deployment without spinning up the Tauri shell. Pairs with `mockIPC()` to stub TauRPC procedures during component tests.
- **Key detail:** Use Vitest in `node` / `happy-dom` mode (non-browser) for agent-driven CI; REJECT Vitest Browser Mode + Playwright preview for these tests since cross-process Playwright headed mode adds flakiness. Browser-mode coverage is handled by tauri-driver E2E layer instead.
- **Source:** https://vitest.dev/

---

## CI Integration Pattern

### GitHub Actions matrix (Linux/macOS/Windows) + tauri-action + cargo-nextest JUnit

- **Version:** tauri-action @v0 (2026-active), dtolnay/rust-toolchain@stable, swatinem/rust-cache@v2
- **Last release:** 2026-04-25
- **Status:** actively maintained
- **Agent-runnable:** yes — workflow exit code reflects step success; `actions/upload-artifact` uploads `target/nextest/junit.xml` and `target/llvm-cov/lcov.info`; `dorny/test-reporter` action surfaces JUnit results inline on PR. Configuration: matrix `strategy: { matrix: { platform: [macos-latest, ubuntu-22.04, windows-latest] } }`.
- **Fits because:** upstream-context.md Sec 6 — "matrix over Linux/macOS/Windows; tauri-action builds .msi/.dmg/.AppImage/.deb"; multi-platform-compat trigger (test-scope Sec 5) requires this exact matrix shape. Cross-platform tauri-driver runs (xvfb-run for ubuntu) plug into the same matrix.
- **Key detail:** Anti-pattern enforcement: "NEVER reference third-party GitHub Actions by floating tag" (tj-actions/changed-files CVE-2025-30066) — pin `tauri-action`, `dtolnay/rust-toolchain`, `swatinem/rust-cache` by 40-char commit SHA in `.github/workflows/*.yml`. CI ALSO needs `cargo deny check bans` to enforce duplicate-version constraint (anti-pattern: "NEVER ship release without resolving tonic 0.14 vs opentelemetry-otlp 0.31 / tonic 0.13 duplicate").
- **Source:** https://v2.tauri.app/distribute/pipelines/github/

---

## Structured Log Parsing

### tracing + tracing-subscriber JSON formatter (product-side emission)

- **Version:** tracing 0.1.x, tracing-subscriber 0.3.x (active 2026)
- **Last release:** 2026-03-22
- **Status:** actively maintained (tokio-rs)
- **Agent-runnable:** yes — `tracing_subscriber::fmt().json().with_timer(UtcTime::rfc_3339()).flatten_event(true).init()` emits one JSON object per event line on stderr; `jq 'select(.level == "ERROR") | .message'` extracts errors; `grep -o "rows_ingested: [0-9]*" | tail -1` works on string fields. Configuration: feature-gate `[dependencies] tracing-subscriber = { version = "0.3", features = ["json", "env-filter"] }`.
- **Fits because:** test-scope.md Sec 3 explicitly references "Rust `tracing` crate JSON output" with example line shape `{"timestamp": "…", "level": "INFO", "target": "ingest::grpc", …}`. Anti-patterns demand this for sanitized error envelopes (NEVER expose stack traces / file paths / library versions). MCP sidecar requires tracing → stderr to keep stdout strictly JSON-RPC.
- **Key detail:** Test must assert specific log lines (e.g., "listening on 127.0.0.1:4317") AND assert NO `tracing` events match the data-leakage anti-patterns (no raw OTLP attributes, no plugin paths beyond basename, no DuckDB query parameter values).
- **Source:** https://docs.rs/tracing

### tracing-test + tracing-fluent-assertions (test-side log parsing)

- **Version:** tracing-test 0.2.x; tracing-fluent-assertions 0.4.x (active)
- **Last release:** 2026-01-20
- **Status:** actively maintained
- **Agent-runnable:** yes — `tracing_test::traced_test` macro injects `logs_contain(value: &str)` into annotated tests; tracing-fluent-assertions provides span lifecycle assertions. Configuration: `[dev-dependencies] tracing-test = "0.2"`.
- **Fits because:** Discipline trigger (test-scope Sec 5): "every test must exit with deterministic signal … log line match readable by CI agent without human review". Tests asserting "no DuckDB query parameters logged" use `assert!(!logs_contain("query parameters: ["))` for the data-leakage NEVER list.
- **Key detail:** Pair with structured tracing JSON output for both human and agent debugging.
- **Source:** https://docs.rs/tracing-test/

---

## Mocking & Stubbing

### httpmock

- **Version:** 0.7.x (MSRV raised to 1.88 — verify against project's rustc 1.85+; may need to pin 0.6 if MSRV bumps cause friction)
- **Last release:** 2026-02-10
- **Status:** actively maintained (httpmock org — split from prior httpmock-1 fork)
- **Agent-runnable:** yes — `MockServer::start()` boots local HTTP mock; `mock.assert()` (or `mock.assert_async().await`) panics on mismatch which surfaces via cargo-test exit code. Parallel-test safe (separate mock servers per test). Configuration: `[dev-dependencies] httpmock = "0.7"`.
- **Fits because:** Mocks `tauri-plugin-updater`'s `latest.json` HTTP fetch from GitHub Releases (test-scope Sec 5: "Mock latest.json with valid Minisign signature → updater accepts; invalid → rejects"). Also useful for mocking external HTTP that workspace-detector might consume if it ever adds remote VCS resolution.
- **Key detail:** Verify MSRV at adoption — if httpmock 0.7 requires 1.88 but project pins to 1.85, prefer mockito 0.x (MSRV 1.85.0) instead, OR pin httpmock to last 1.85-compatible version.
- **Source:** https://github.com/httpmock/httpmock

### mockito (alternative HTTP mock)

- **Version:** 1.6.x (MSRV 1.85.0 — matches arch's rustc 1.85 floor)
- **Last release:** 2026-03-05
- **Status:** actively maintained
- **Agent-runnable:** yes — `mockito::Server::new_async().await` + `server.mock("GET", "/path").create_async().await`; assertions panic on mismatch → cargo-test exit code. Configuration: `[dev-dependencies] mockito = "1"`.
- **Fits because:** Backup HTTP mock if httpmock MSRV friction blocks adoption; same use case (latest.json updater path, external HTTP fakes).
- **Key detail:** Lower-feature than httpmock but stable MSRV — keep as Tier-2 fallback.
- **Source:** https://github.com/lipanski/mockito

### tokio time pause (`tokio::time::pause()` + `advance(Duration)`)

- **Version:** built into tokio (matches arch's "tokio current stable")
- **Last release:** 2026-04-18 (with each tokio release)
- **Status:** actively maintained
- **Agent-runnable:** yes — `#[tokio::test(start_paused = true)]` freezes virtual clock; `tokio::time::advance(Duration::from_secs(600))` skips 10 min instantly; deterministic across CI runs. Configuration: feature `tokio = { version = "1", features = ["time", "test-util"] }`.
- **Fits because:** DuckDB ring buffer retention test ("inject 10k spans, advance virtual clock past 10-min retention window, assert oldest spans evicted") — without virtual time, the test would take >10 real minutes, violating CI budgets. Aligns with chaos trigger in test-scope Sec 5.
- **Key detail:** Avoids dependency on third-party time-mocking crates; native to the tokio runtime arch already uses.
- **Source:** https://tokio.rs/tokio/topics/testing

### mockall (trait-based stub generator) — for ui-bridge / snapshot trait boundaries

- **Version:** mockall 0.13.x (active 2026 cadence)
- **Last release:** 2026-03-18
- **Status:** actively maintained
- **Agent-runnable:** yes — `#[automock]` macro generates `MockMyTrait`; expectations `mock.expect_query().returning(…)`; mismatched calls panic at drop with structured failure. Pair with dependency injection (constructor-injected `Arc<dyn MyTrait>`) per the "REJECT monkey-patching" rule. Configuration: `[dev-dependencies] mockall = "0.13"`.
- **Fits because:** ui-bridge crate exposes TauRPC routers that depend on buffer/viz traits — mockall stubs allow IPC procedure tests without full receiver+buffer pipeline. Snapshot crate's curation pipeline (dedupe → critical-path → token-budget) can be tested with mocked buffer-query trait.
- **Key detail:** Architecture must use trait-based abstractions for buffer access (constructor-injected dyn traits) — anti-pattern would be to monkey-patch via runtime swap (REJECTED by research-targets rule).
- **Source:** https://docs.rs/mockall

---

## Integration Test Patterns

### `#[tokio::test]` + spawned `pulse-app` via `assert_cmd` for full-stack E2E

- **Version:** tokio 1.x, assert_cmd 2.x (current stable)
- **Last release:** 2026-04-18
- **Status:** actively maintained
- **Agent-runnable:** yes — `#[tokio::test]` wraps async test; spawn `pulse-app` as child process with `tokio::process::Command`; tests block on TCP-handshake confirmation that `:4317`/`:4318` accept connections (boot signal per test-scope Sec 3 Command 1); cleanup via SIGTERM to PID. Exit code from cargo-test reports pass/fail.
- **Fits because:** test-scope Sec 3 mandates: "Integration tests: `#[tokio::test]` spawning full pulse-app, injecting OTLP, querying via TauRPC". Critical paths P1, P2, P6 require live receiver + buffer + IPC roundtrip.
- **Key detail:** `flavor = "multi_thread"` recommended for tests that exercise concurrent OTLP ingest + query — single-threaded runtime would serialize the test and miss real-world concurrency bugs.
- **Source:** https://tokio.rs/tokio/topics/testing

### Insta snapshot testing (snapshot crate output verification)

- **Version:** insta 1.x, cargo-insta 1.x (active 2026)
- **Last release:** 2026-03-25
- **Status:** actively maintained (mitsuhiko)
- **Agent-runnable:** yes — `insta::assert_yaml_snapshot!(curated_snapshot)` compares to stored `.snap`; in CI, mismatches return exit code 1 and write `.snap.new` (no human review needed for pass/fail). For agent-driven loops, run `INSTA_FORCE_PASS=0 cargo insta test --check` (CI mode strictly fails on diff). Configuration: `[dev-dependencies] insta = { version = "1", features = ["json", "yaml"] }`.
- **Fits because:** Critical path P2 (curated snapshot generation) — assert structural shape of generated markdown / JSON snapshots is consistent: anomaly markers present, dedup ratios reported, p50/p95/p99 fields present. Easier than enumerating every assertion individually.
- **Key detail:** REJECT cargo-insta's interactive `cargo insta review` mode for CI (human-in-loop); use `--check` mode in agent-driven CI which fails the run on diff. Reviewing/accepting new snapshots is a developer-mode-only workflow.
- **Source:** https://insta.rs/

---

## Test Data Strategy

### proptest (property-based generators for OTLP fuzzing — used selectively at Standard tier)

- **Version:** proptest 1.10.0 (MSRV 1.84 — matches arch)
- **Last release:** 2026-02-28
- **Status:** mostly feature-complete, passive maintenance per maintainer notes — still actively bug-fixed
- **Agent-runnable:** yes — `proptest!(|(payload in arb_otlp_request())| { … })` runs N random cases; on failure, automatic shrinking finds minimal repro; exit code reflects failure; failure seeds saved to `proptest-regressions/` for deterministic re-run. Configuration: `[dev-dependencies] proptest = "1.10"`.
- **Fits because:** test-scope Sec 3.5 explicitly names "Property-Based Generators: For stress tests and cardinality exploration (e.g., '10k spans/sec ingest throughput'), use proptest crate to generate random valid OTLP payloads and assert buffer handles them without panic". Also covers post-`prost`-decode invariant tests (test-scope Sec 5: random `span_id` lengths, random `trace_id` lengths) — assert ingest rejects invalid IDs without panicking. [trigger-driven; pulled in by test-scope Sec 3.5 explicit fixture-strategy mandate; not standard for Standard tier but required for trigger coverage].
- **Key detail:** `proptest-regressions/` directory MUST be committed to git so CI can deterministically replay reductions found in prior runs.
- **Source:** https://github.com/proptest-rs/proptest

### duckdb-rs Arrow appender + Rust builder factories (programmatic seeding)

- **Version:** matches duckdb 1.5.x adoption
- **Last release:** 2026-03-15
- **Status:** actively maintained
- **Agent-runnable:** yes — synchronous Rust API with `Result` return types; no human pre-seeding. Configuration: per-test factory `MockArrowBatch::builder().rows(N).columns(["trace_id", "span_name", "duration_ms"]).build()`.
- **Fits because:** test-scope Sec 3.5 mandates "No Developer-Seeded DB: All fixture data is generated at test runtime via OTLP or programmatic Rust builders; no `.sql` scripts or pre-baked SQLite files". Buffer crate tests need the Arrow appender path covered.
- **Key detail:** Pair with `rstest` fixtures for composable test setup.
- **Source:** https://duckdb.org/docs/current/clients/rust

---

## Test Isolation Patterns

### cargo-nextest per-process isolation + `tempfile`/`assert_fs` per-test directories

- **Version:** nextest 0.9.x, tempfile 3.x (current)
- **Last release:** 2026-04-03
- **Status:** actively maintained
- **Agent-runnable:** yes — nextest runs each `#[test]` in its own process (no shared global state); per-test `TempDir::new()` isolates filesystem; randomized port allocation (e.g., `ANDROMEDA_PULSE_OTLP_GRPC_PORT=0` to ask OS for ephemeral port, then read back via `health` IPC) prevents collision when the matrix runs parallel suites. Configuration: in `nextest.toml`, set `[profile.ci.junit] path = "junit.xml"` + `slow-timeout = { period = "60s", terminate-after = 2 }`.
- **Fits because:** OTLP receivers bind to specific TCP ports — running tests in parallel within one cargo-test binary would conflict. Per-process isolation + ephemeral port allocation is the agent-driven discipline pattern (test-scope Sec 5: "deterministic harness invocations").
- **Key detail:** `ANDROMEDA_PULSE_OTLP_GRPC_PORT` and `ANDROMEDA_PULSE_OTLP_HTTP_PORT` env vars (upstream-context.md Sec 4) are the wire-up — tests set these to `0` for ephemeral ports OR pre-allocate from a port pool to avoid the bind-then-test race.
- **Source:** https://nexte.st/docs/running/

---

## Performance & Load Testing

[trigger-driven; pulled in by `performance-budget: WebGPU canvas throughput` and `performance-budget: Snapshot token budget enforcement` triggers from test-scope Sec 5; not standard for Standard tier but required for trigger coverage]

### criterion.rs (microbenchmark + regression detection)

- **Version:** criterion 0.5.x (active 2026)
- **Last release:** 2026-02-22
- **Status:** actively maintained
- **Agent-runnable:** yes — `cargo bench` produces structured stdout + HTML reports; `--save-baseline pre-change` then `--baseline pre-change` programmatically detects regressions; exit code reflects regression (with `--save-baseline`-based gates via wrapper script). Outputs `target/criterion/<bench>/new/estimates.json` for CI parsing. Configuration: `[[bench]] name = "ingest_throughput" harness = false`.
- **Fits because:** Performance-budget trigger (test-scope Sec 5: "10k spans/sec target throughput baseline"). Criterion's statistical confidence intervals avoid the "flaky budget" trap (a single noisy CI run won't fail the budget). Microbenchmark `prost` decode of synthetic OTLP payloads, Arrow batch insertion to DuckDB.
- **Key detail:** For end-to-end throughput (10k spans/sec sustained for 10s), criterion is the wrong tool — use a custom load-driver test that times `tokio::time::interval(Duration::from_micros(100))` driven gRPC sends and asserts buffer `rows_ingested` count. Criterion is for unit-level perf hot-spots (snapshot dedup loop, Arrow append).
- **Source:** https://github.com/bheisler/criterion.rs

### Custom load-driver test (no third-party load tool needed for loopback OTLP)

- **Version:** N/A — synthesized pattern using tonic 0.14.x + tokio 1.x
- **Last release:** 2026-04-18 (tokio cadence)
- **Status:** actively maintained (built on first-party tokio + tonic)
- **Agent-runnable:** yes — in-process Rust `#[tokio::test(flavor = "multi_thread")]` + `tokio::time::interval` + parallel `tonic` clients; `assert!(rows_ingested >= 100_000)` after 10s of injection; metrics surfaced via `health` TauRPC subsystem state. No external load-test runner needed since target is loopback OTLP at modest scale (10k/s). Configuration: per-test concurrent `tokio::spawn` of N workers, each looping `client.export(span)` on `tokio::time::interval`.
- **Fits because:** Avoids pulling in heavyweight external load tools (k6, vegeta) — those target HTTP services, not loopback OTLP. The test fits inside `cargo nextest run --test load_otlp_throughput`. Performance-budget triggers (Sec 5) are validated via `assert!(p99_latency_ms < 100)` from collected per-RPC durations.
- **Key detail:** Synthesized from test-scope Sec 5 perf-budget triggers + tonic + tokio multi-thread runtime patterns.
- **Source:** https://tokio.rs/tokio/topics/testing

---

## Multi-Platform Compat Matrix

[trigger-driven; pulled in by `multi-platform-compat: Windows WebView2 vs macOS WKWebView vs Linux GTK WebKit` trigger from test-scope Sec 5; standard for Standard tier]

### GitHub Actions matrix `os: [macos-latest, ubuntu-22.04, windows-latest]` + tauri-action

- **Version:** as cited in CI Integration Pattern above
- **Last release:** 2026-04-25
- **Status:** actively maintained
- **Agent-runnable:** yes — workflow file is declarative; per-platform job exit code aggregates; `tauri-action` builds platform-specific bundles (`.msi` / `.dmg` / `.AppImage` / `.deb`); xvfb-run for Linux headless tauri-driver runs.
- **Fits because:** Multi-platform compat trigger (Sec 5: "Verify tauri-driver harness can boot app on each platform; verify IPC works consistently; verify WebGPU canvas is available; verify tray icon renders; verify notifications dispatch via native Notification Center / Action Center / freedesktop"). Architecture (upstream-context.md Sec 6) explicitly mandates "matrix over Linux/macOS/Windows".
- **Key detail:** Linux runner needs `libwebkit2gtk-4.1-dev` (Tauri v2) — install in workflow before build. macOS runner needs both `aarch64-apple-darwin` and `x86_64-apple-darwin` targets if shipping universal binaries. Windows runner needs `windows-latest` for native WebView2.
- **Source:** https://v2.tauri.app/distribute/pipelines/github/

---

## Chaos & Fault Injection

[trigger-driven; pulled in by `chaos-test: Buffer overflow / retention window enforcement` trigger from test-scope Sec 5; not standard for Standard tier but required for trigger coverage]

### turmoil (deterministic network simulation for tokio)

- **Version:** turmoil 0.6.x (active early 2026)
- **Last release:** 2026-02-12
- **Status:** actively maintained (tokio-rs)
- **Agent-runnable:** yes — runs multi-host tokio simulation in single thread with seeded RNG for repro; `Sim::run()` returns `Result`; deterministic trace replay. Configuration: `[dev-dependencies] turmoil = "0.6"`.
- **Fits because:** Chaos trigger (Sec 5: "Kill ingest channel mid-stream → assert buffer gracefully pauses; assert next ingest reconnects and resumes"). Turmoil simulates network partitions / packet drops / latency injection without requiring real OS network manipulation, enabling agent-driven repro of ingest-channel-failure scenarios.
- **Key detail:** Turmoil is most valuable for distributed systems with multiple hosts; for THIS project (single-process desktop app with loopback OTLP), use a simpler `tokio::time::pause` + manual mpsc drop. Keep turmoil as a Tier-2 option for the broadcast-subscriber-disconnect scenarios.
- **Source:** https://docs.rs/turmoil/

### tokio time pause + manual broadcast-channel drop (lightweight in-house chaos pattern)

- **Version:** N/A — synthesized pattern using tokio 1.x + tokio::sync primitives
- **Last release:** 2026-04-18 (tokio cadence)
- **Status:** actively maintained (built on first-party tokio)
- **Agent-runnable:** yes — pure Rust, deterministic, no external infra. `tokio::time::pause()` + `tokio::time::advance(Duration::from_secs(700))` to skip past 10-min retention; explicit `drop(broadcast_sender)` to simulate ingest-channel close; `tokio::sync::mpsc::Sender::close()` for graceful pause. Test exit code reflects chaos-recovery success.
- **Fits because:** Chaos trigger (Sec 5: "Inject spans continuously at 10k/sec for 15 min (exceeds 10 min window) → assert oldest spans are evicted"). Tokio's virtual clock makes the 15-min scenario complete in milliseconds.
- **Key detail:** Preferred over turmoil for single-process scenarios; turmoil reserved for broadcast-subscriber chaos.
- **Source:** https://tokio.rs/tokio/topics/testing

---

## Supply-Chain & Anti-Pattern Enforcement

### cargo-deny

- **Version:** cargo-deny 0.16.x (active 2026)
- **Last release:** 2026-03-10
- **Status:** actively maintained (Embark)
- **Agent-runnable:** yes — `cargo deny check bans` returns non-zero exit on duplicate-version violations; `cargo deny check advisories` for RustSec advisory matches; `cargo deny check licenses` for license policy violation. Configuration: `deny.toml` with `[bans] multiple-versions = "deny"`.
- **Fits because:** Anti-pattern: "NEVER ship release without resolving tonic 0.14 vs opentelemetry-otlp 0.31 / tonic 0.13 duplicate — `cargo deny check bans` enforces; duplicate versions create linker conflicts" (upstream-context.md Anti-Patterns). Also covers RustSec CVE matching in CI per the Cranelift-only WASM enforcement and tar-crate CVE-2026-33056 vigilance.
- **Key detail:** Pin `cargo-deny` action by 40-char SHA in CI, NOT by version tag (per "NEVER reference third-party GitHub Actions by floating tag" anti-pattern). Run on every PR.
- **Source:** https://blog.logrocket.com/comparing-rust-supply-chain-safety-tools/

### cargo-audit (RustSec Advisory DB matching)

- **Version:** cargo-audit 0.22.x (active 2026 — RustSec curated)
- **Last release:** 2026-04-08
- **Status:** actively maintained (rustsec org)
- **Agent-runnable:** yes — `cargo audit --json` outputs structured advisory matches; non-zero exit on critical findings; runs offline against pre-fetched RustSec DB. Configuration: `cargo audit --json --deny warnings`.
- **Fits because:** Backstop for cargo-deny advisory matching; lighter-weight gate run on every push to detect new RustSec advisories against pinned dependency versions (DuckDB 1.5.x, wasmtime 25+, tonic 0.14.x, axum 0.8.x). Matches Anti-Pattern enforcement of "no unresolved critical security issues older than 6 months".
- **Source:** https://crates.io/crates/cargo-audit
