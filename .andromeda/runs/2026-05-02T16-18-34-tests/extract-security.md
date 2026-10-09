## 2. Security Plan Excerpt

### Security Tier

- **Tier:** Minimal (0)
- **Justification:** This is a local-first, zero-infrastructure single-user desktop app with no user accounts, no persistent user data store, no internet-exposed network surface (OTLP receivers bound to 127.0.0.1 only), and no compliance-regulated data classifications.

### Attack Vectors

- **Vector 1:** public API (network-bound on loopback only) — scope: OTLP/gRPC on `:4317` and OTLP/HTTP on `:4318`, both bound `127.0.0.1` only. Mitigation: Loopback-only binding is the de facto authorization boundary; plaintext loopback OTLP servers run without TLS per Minimal-tier risk model.

- **Vector 2:** IPC (Tauri TauRPC bridge — webview ↔ Rust) — scope: procedures `app_info`, `health`, `ready`, `get_settings`, `update_settings`, routers `traces.*`, `metrics.*`, `logs.*`, `snapshot.*`, `plugins.*`, `mcp.*` (feature-gated), `workspace.*`. Mitigation: Tauri 2 capability gating `pulse:default` permits exactly the enumerated procedures; errors are forced to `Serialize`-able `AppError` enum variants.

- **Vector 3:** plugin host (WASM Component Model) — scope: `~/.andromeda-pulse/plugins/` filesystem load by `wasmtime` 25+ and `plugins.invoke` IPC dispatch. Mitigation: Capability-scoped sandboxing — guests receive only host imports declared in their WIT; no syscalls, file, or socket access unless explicitly granted.

- **Vector 4:** MCP stdio surface (feature-gated) — scope: `andromeda-pulse-mcp` rmcp sidecar, JSON-RPC 2.0 over stdin/stdout with tools `query_traces`, `query_metrics`, `query_logs`, `generate_snapshot`. Mitigation: Stdio caller is an LLM client with no network exposure; gated by compile-time `--features mcp-server` AND runtime `ANDROMEDA_PULSE_MCP_ENABLED=true`.

- **Vector 5:** filesystem reads (config + workspace detection) — scope: `config.toml` under per-platform data dir; env-var-overridable paths (`ANDROMEDA_PULSE_CONFIG_PATH`, `ANDROMEDA_PULSE_DATA_DIR`, `ANDROMEDA_PULSE_PLUGIN_DIR`); workspace-detector crate. Mitigation: Canonicalize and confine all `ANDROMEDA_PULSE_*_PATH` / `*_DIR` env vars using `std::path::Path::canonicalize()` + assertion that canonical path starts under resolved data dir.

- **Vector 6:** in-app updater — scope: `tauri-plugin-updater` 2.x consuming `latest.json` from GitHub Releases. Mitigation: Updater public key baked into Tauri config; Minisign Ed25519 signature verification is mandatory and cannot be disabled.

- **Vector 7:** CLI input (env vars + binary launch) — scope: Reserved env vars `ANDROMEDA_PULSE_*`, `RUST_LOG`. Mitigation: `serde` + `TryFrom<u16>` for ports; path env vars must undergo canonicalization and confinement per Vector 5 mitigation.

- **Vector 8:** webview content (host-side) — scope: WebView2 (Windows) / WKWebView (macOS/Linux) rendering WebGPU canvas from Arrow data under `pulse-app/ui/`. Mitigation: Tauri capability model `pulse:default` gates IPC procedures; no `fs`, `shell`, `dialog`, `http` core APIs granted to webview.

### Anti-Patterns Rejected

- **NEVER trust `Content-Length` or trailers as substitute for `DefaultBodyLimit` on OTLP HTTP `:4318`** — rejected because: JFrog axum-core advisory documents unbounded request DoS root cause.

- **NEVER format user input directly into SQL (e.g. `format!("SELECT … WHERE service_name = '{}'", user_input)`)** — rejected because: Dagster GHSA-mjw2-v2hm-wj34, Vanna CVE-2024-5827, dagster-duckdb CVE-2026-41490 cluster proves SQL injection is the dominant 2026 DuckDB vulnerability class; ALWAYS use `Connection::prepare` with `?` placeholders and parameter binding.

- **NEVER skip post-`prost`-decode invariant checks on OTLP payloads** — rejected because: `prost` only validates wire format; `span_id` (8 bytes), `trace_id` (16 bytes), attribute key sizes, etc. must be checked before passing to `buffer::Appender`.

- **NEVER read path env vars without `Path::canonicalize()` + confinement assertion** — rejected because: CWE-22 (Path Traversal); zip crate CVE-2025-29787, RustFS CVE-2025-68705 document symlink-chain / TOCTOU traversal class affecting user-overridable path env vars.

- **NEVER call `tonic::transport::Server::builder()` without `.max_decoding_message_size(8 * 1024 * 1024)`** — rejected because: `tonic` historically accepted 1 GB+ unary requests by default.

- **NEVER trust plugin-returned Arrow IPC bytes without a size cap** — rejected because: Risk of unbounded memory consumption on re-emit to Tauri `Channel` API.

- **NEVER use non-Cranelift `wasmtime` feature flag on x86_64** — rejected because: April 2026 advisory cluster documents two Critical sandbox escapes (CVE-2026-34941, CVE-2026-35195) in non-Cranelift backends; Cranelift is the unaffected configuration.

- **NEVER override `tauri-plugin-updater` Minisign signature verification** — rejected because: Minisign Ed25519 verification cannot be disabled; any wrapper must not attempt bypass.

- **NEVER ship Tauri updater Minisign private key in repo or artifact** — rejected because: Only public key belongs in `tauri.conf.json`.

- **NEVER store Windows EV signing cert outside Azure Key Vault HSM** — rejected because: Private key must never leave HSM per DigiCert/GlobalSign supported HSM-RSA CA policy.

- **NEVER enable DuckDB encryption at rest on in-memory ring buffer** — rejected because: CVE-2025-64429 is documented against encryption feature; in-memory `:memory:` connection avoids surface.

- **NEVER use `CorsLayer::permissive()` or `allow_origin(Any)` on OTLP HTTP `:4318`** — rejected because: Default-deny is correct posture.

- **NEVER skip Host-header allowlist middleware on `:4318`** — rejected because: DNS rebinding is documented attack against localhost OTLP/HTTP per Coder Agent API CVE-2025-09-19, CVE-2025-66414; "localhost is not a security boundary."

- **NEVER add TauRPC procedure without matching capability entry in `pulse-app/capabilities/`** — rejected because: Silent runtime rejection becomes hard-to-diagnose UX bug; xtask drift check enforces.

- **NEVER grant Tauri core APIs (`fs`, `shell`, `dialog`, `http`) to `pulse:default` without explicit per-feature capability** — rejected because: Must have stated rationale and only when necessary.

- **NEVER expose `pulse:updater` to webview JavaScript** — rejected because: Updater capability is bound to `tauri-plugin-updater` flow only; webview exposure bypasses gate.

- **NEVER widen `pulse:notification` or `pulse:tray` beyond outbound emit** — rejected because: Granting webview-side input handlers converts outbound surfaces into phishing / IPC-abuse vectors.

- **NEVER widen `pulse:plugin-fs` beyond host's read of resolved plugin dir** — rejected because: Webview exposure converts internal loader into unconstrained filesystem-read; write/execute widening converts to plugin-supply-chain compromise vector.

- **NEVER bind OTLP receivers to `0.0.0.0` or non-`127.0.0.1` interface** — rejected because: Loopback binding is de facto authorization boundary; `0.0.0.0` invalidates entire Minimal-tier risk model.

- **NEVER instrument product's own telemetry with OTLP exporter to product's own `:4317`/`:4318`** — rejected because: Creates infinite loop and bypasses loopback authorization model.

- **NEVER commit `.env`, `*.p12`, `*.pem`, `*.cer`, `*.key` files to git** — rejected because: `.gitignore` enforcement + gitleaks CI verification required.

- **NEVER store Azure Key Vault service principal as long-lived GitHub Actions secret** — rejected because: Use GitHub OIDC federation instead.

- **NEVER reference third-party GitHub Actions by floating tag** — rejected because: Pin by 40-char commit SHA; tj-actions/changed-files CVE-2025-30066 (23k repos) demonstrated retroactive-tag-rewrite attack.

- **NEVER grant `permissions: { contents: write }` at workflow level** — rejected because: Set to `contents: read`, elevate to `write` per-job only on publish per minimum-permission principle.

- **NEVER place Azure Key Vault, Apple Developer ID, or Tauri Minisign private key outside `production-release` GitHub Environment with manual approval** — rejected because: Credential containment critical for code-signing supply chain.

- **NEVER log raw OTLP attribute values, payload contents, snapshots, clipboard, or MCP responses** — rejected because: May contain incidentally captured secrets from instrumented application.

- **NEVER log full plugin file paths** — rejected because: Log basename of canonicalized path only.

- **NEVER log DuckDB query parameter values** — rejected because: Log query identifier + parameter count instead.

- **NEVER expose stack traces, struct names, file paths, or library versions in `AppError::Internal` to webview** — rejected because: Must sanitize at `From<thiserror::Error> for AppError` impl.

- **NEVER expose internal hostnames or IPs in OTLP or MCP error responses** — rejected because: Risk of information disclosure.

- **NEVER use `tokio::process::Command::new()` with user input from OTLP, MCP, or workspace-detector** — rejected because: MCP STDIO command-injection cluster (CVE-2025-49596, CVE-2025-54994, CVE-2025-54136, CVE-2026-22252) demonstrates live 2025–2026 class.

- **NEVER load plugin from uncanicalized path or outside resolved plugin dir** — rejected because: Defends against symlink-traversal via `ANDROMEDA_PULSE_PLUGIN_DIR`; pair with Vector 5 mitigation.

- **NEVER spawn rmcp sidecar without verifying compile-time `--features mcp-server` AND runtime `ANDROMEDA_PULSE_MCP_ENABLED=true`** — rejected because: Single-gate is regression of double-gate architecture per Established Decisions.

- **NEVER serialize `anyhow::Error` directly across TauRPC bridge** — rejected because: Must convert to `serde`-friendly `AppError` enum; direct serialization leaks error chain to webview.

- **NEVER use `serde_json::from_slice::<T>()` on buffer without `DefaultBodyLimit` size bound** — rejected because: Covers OTLP HTTP/JSON path on `:4318`.

- **NEVER use `wasmtime` `Linker` to expose host functions outside WIT contract** — rejected because: Bypasses capability-scoped sandbox per architecture.

- **NEVER let rust-toolchain drift below 1.85.0** — rejected because: Edition 2024 parsing + security-positive defaults (`unsafe_op_in_unsafe_fn`, `unsafe extern`, `static mut` denial, tightened `if let` scopes) require 1.85+.

- **NEVER ship release without resolving `tonic` 0.14 vs `opentelemetry-otlp` 0.31 / `tonic` 0.13 duplicate** — rejected because: `cargo deny check bans` enforces; duplicate versions create linker conflicts.

### Data Classifications

- **user-content (telemetry payloads — traces, metrics, logs)** (Sensitivity: medium) — stored in OTLP receivers (`ingest` crate on `:4317`/`:4318`), DuckDB in-memory ring buffer (`buffer` crate), broadcast to viz/MCP, and curated snapshots at `~/.andromeda-pulse/snapshots/`; testability hint: partially-testable with stub (loopback binding, in-memory buffer are deterministic; snapshot file I/O requires filesystem stub).

- **config (user settings)** (Sensitivity: low) — stored in `~/.andromeda-pulse/config.toml` and env vars `ANDROMEDA_PULSE_*`; testability hint: testable (enum settings, port ranges, file paths, retention seconds, log level are directly exercisable).

- **config (signing/release credentials — out of app runtime)** (Sensitivity: high) — stored in GitHub Actions encrypted secrets, Azure Key Vault HSM, and Apple Developer ID; testability hint: untestable in current harness (build/release pipeline only; runtime artifact contains only public verification key).

- **user-content (third-party WASM plugin binaries)** (Sensitivity: medium) — stored in `~/.andromeda-pulse/plugins/` loaded by `wasmtime` Component Model host; testability hint: partially-testable with stub (plugin lifecycle testable; arbitrary guest behavior requires fixture mocking or sandbox instrumentation).
