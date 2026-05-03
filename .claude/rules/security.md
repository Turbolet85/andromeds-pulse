# Security Rules

Universal security requirements. Apply to all files in this project. This rule file has no `paths:` frontmatter — it loads unconditionally.

**Authoritative source:** `.andromeda/security-plan.md` (tier=Minimal — local-first Tauri 2 desktop app, loopback-only OTLP, in-memory DuckDB). For full threat model, vector list, and bootstrap phasing, read the plan.

## Loopback-only network surfaces
- OTLP receivers MUST bind to `127.0.0.1` only — `:4317` (tonic 0.14 gRPC) and `:4318` (axum 0.8 HTTP). Loopback is the de-facto authorization boundary; no auth model.
- NEVER bind to `0.0.0.0` or any non-loopback interface — invalidates the entire Minimal-tier risk model.
- OTLP HTTP `:4318` MUST have Host-header allowlist middleware (reject non-`{127.0.0.1, localhost, [::1]}:configured-port`); DNS rebinding lesson from Coder Agent CVE-2025-09-19 + MCP TypeScript SDK CVE-2025-66414.
- OTLP HTTP `:4318` MUST have CORS deny-by-default — no `CorsLayer::permissive()` / `allow_origin(Any)`; no browser context should ever post OTLP.
- OTLP `:4318` MUST set `axum::DefaultBodyLimit::max(8 * 1024 * 1024)`; OTLP `:4317` MUST set `.max_decoding_message_size(8 * 1024 * 1024)` per `*ServiceServer` builder. JFrog axum-core advisory + tonic 1 GB-default issue are the anchors.

## Input validation at every boundary
- Post-`prost`-decode invariant checks REQUIRED on every OTLP payload — `span_id` is 8 bytes, `trace_id` is 16 bytes, attribute keys/values bounded. `prost` only validates wire format; semantic invariants are the `ingest` crate's job.
- DuckDB queries MUST use `Connection::prepare` + `?` placeholders + `Statement::execute([&param])` — NEVER `format!("SELECT … WHERE service_name = '{}'", user_input)`. 2026 DuckDB CVE cluster (Dagster GHSA-mjw2-v2hm-wj34, Vanna CVE-2024-5827, dagster-duckdb CVE-2026-41490).
- Path env vars (`ANDROMEDA_PULSE_*_PATH` / `*_DIR`) MUST canonicalize via `strict-path` and assert the resolved path lives under the resolved data dir before use. CWE-22 path traversal class defense.
- TauRPC procedure args MUST go through `serde` deserialization on every `#[taurpc::procedure]` argument struct; `TryFrom<u16>` on every port; `serde`-friendly enums on every settings field.
- Plugin file paths MUST canonicalize + confine under the resolved plugin dir before `wasmtime` load. Pair with the path-env-var canonicalization above.

## Tauri capability gating (negative-default)
- Every TauRPC procedure MUST have a matching entry in `pulse-app/capabilities/` JSON — Tauri capabilities are negative-default; a missing entry is a silent runtime rejection.
- Two enforcement pieces required: (1) router registration in the owning crate, AND (2) capability JSON entry. The `xtask capability-drift` check enforces in CI.
- NEVER grant Tauri core APIs (`fs`, `shell`, `dialog`, `http`) to `pulse:default` without an explicit per-feature capability addition with stated rationale.
- NEVER expose `pulse:updater` to webview JavaScript — bound to `tauri-plugin-updater` flow only.
- `pulse:notification` and `pulse:tray` MUST be outbound-emit-only — input-event handlers (notification action callbacks, tray menu IPC re-entry) require a separately-named capability.
- `pulse:plugin-fs` MUST stay scoped to the host's read of the canonicalized plugin dir — never widened to webview / write / execute.

## Plugin host + WASM sandbox
- WASM Component Model guests receive ONLY host imports declared in their WIT — no syscalls, file, or socket access unless explicitly granted.
- `wasmtime::Config` MUST set `epoch_interruption(true)` for plugin call timeouts (2-3× faster than fuel) + `max_wasm_http_fields_size` per April 2026 advisory cluster CVE-2026-27572.
- Per-Store `wasmtime::ResourceLimiter` MUST cap memory (e.g., 64 MB), tables, instances.
- Cranelift backend on x86_64 MUST stay enabled — was the unaffected configuration for April 2026 Critical sandbox escapes (CVE-2026-34941, CVE-2026-35195). NEVER add a non-Cranelift wasmtime feature flag on x86_64 builds.
- NEVER use the `wasmtime::Linker` to expose host functions outside the WIT contract — bypasses capability scoping.
- NEVER trust plugin-returned Arrow IPC bytes without an 8 MB size cap before re-emit on the Tauri Channel API.

## MCP feature double-gate
- MCP stdio surface MUST require BOTH compile-time `--features mcp-server` AND runtime `ANDROMEDA_PULSE_MCP_ENABLED=true`. Single-gate is a regression of the architecture-declared double-gate.
- The graceful-degrade `warn` (env var set against a binary built without the feature) MUST be preserved — do not regress to fail-startup.

## Error sanitization at boundaries
- Every `#[taurpc::procedure]` returns `Result<T, AppError>` — `AppError` is the `serde`-friendly enum (`Validation { field, reason }` / `NotFound { resource }` / `Internal { message }` / `Plugin { plugin_id, message }` / `Storage { message }` / `Ingest { message }`).
- Module-internal `thiserror` 2.x enums convert at the bridge via `From` impls. NEVER serialize `anyhow::Error` directly across the bridge.
- `AppError::Internal { message }` carries a sanitized one-liner — strip stack traces, file paths, library versions, Rust struct names. Full chain stays in internal log via `tracing-error` SpanTrace.
- OTLP receiver errors collapse to standard `tonic::Status` codes / `Status` proto in body per OTLP HTTP spec.
- MCP server errors use standard JSON-RPC 2.0 error object (numeric `code`, `message`, optional `data`).

## Logging & redaction (snapshot/clipboard/MCP hygiene)
- NEVER log raw OTLP attribute values, span/log/metric content payloads, snapshot file contents, clipboard contents, or MCP tool response bodies — incidentally captured secrets from instrumented host apps.
- NEVER log full plugin file paths — basename of canonicalized path only.
- NEVER log DuckDB query parameter values — use `query_id` + `param_count` + `param_types` instead.
- NEVER expose internal hostnames / IPs / library versions in OTLP error responses or MCP error objects.
- `snapshot.copy_to_clipboard` MUST emit a `pulse://stream/snapshot-progress` event at clipboard-write time so the UI can show a non-suppressible "X bytes copied" toast.

## Self-observation discipline
- Self-observation runtime is `tracing` + `tracing-subscriber` JSON + `tracing-appender` daily file sink ONLY. NO OTel SDK linked into the product binary for self-observation.
- NEVER instrument the product's own telemetry with an OTLP network exporter pointed at the product's own `:4317`/`:4318` — infinite loop + bypasses loopback authorization model.

## Code-signing key custody
- Tauri updater Minisign Ed25519 verification cannot be disabled — never override.
- NEVER ship the Minisign **private** key in the repo or in any build artifact — only the public key belongs in `tauri.conf.json`.
- Windows EV signing cert lives in Azure Key Vault HSM (DigiCert / GlobalSign HSM-RSA only); key never leaves HSM.
- GitHub Actions reaches Azure Key Vault via OIDC federation — NEVER store a long-lived Azure service principal secret in GitHub Actions secrets.

## Supply chain + CI
- Pin every third-party GitHub Action by 40-char commit SHA: `<owner>/<repo>@<40-char-SHA> # vX.Y.Z`. NEVER `@v2` or floating tag (`tj-actions/changed-files` CVE-2025-30066, 23k repos).
- `permissions:` at workflow level MUST be `contents: read`; elevate per-job to `write` only on the publish step.
- All signing secrets, Apple Developer ID credentials, and Tauri updater private key MUST be in GitHub Environment `production-release` with manual approval gate.
- Run `cargo audit` + `cargo deny check bans licenses sources` in CI on every PR. The `bans` check catches the documented `tonic 0.14 ↔ opentelemetry-otlp 0.31 (pinned tonic 0.13)` duplicate.
- Run `step-security/harden-runner` (SHA-pinned) as the first step of every job; egress-policy starts at `audit`, promotes to `block` after a clean window.
- Secret-scanning step (gitleaks or trufflehog, SHA-pinned) runs pre-commit + per-PR. `.gitignore` covers `*.p12`, `*.pem`, `*.cer`, `.env*`, `*.key`, `~/.tauri/*.key`.
- `cargo-auditable` wraps the `tauri-action` build — embeds dependency tree as JSON section in the binary for post-release CVE scanning.

## Rust toolchain
- `rust-toolchain.toml` MUST pin to `1.85.0` minimum — Edition 2024 cannot parse below 1.85; Edition 2024 also requires the security-positive defaults (`unsafe_op_in_unsafe_fn`, `unsafe extern`, `static mut` reference denial, tightened `if let` temporary scopes).
- NEVER ship a release without resolving the `tonic 0.14 ↔ tonic 0.13 (via opentelemetry-otlp 0.31)` duplicate — `cargo deny check bans` is the enforcement.

## Process commands
- NEVER `tokio::process::Command::new(...).arg(user_input)` against any string sourced from an OTLP attribute, MCP tool argument, or workspace-detector output — MCP STDIO command-injection cluster (CVE-2025-49596 MCP Inspector, CVE-2025-54994, CVE-2025-54136 Cursor, CVE-2026-22252 LibreChat).

## Session Additions
_This section is owned by `/wrap-session`. setup-project preserves content added here on re-run. See `section-markers.md` for the convention._

- 2026-05-03: Each Tauri major + minor-version bump introduces new transitive duplicate crates that fail `cargo deny check bans` with `multiple-versions = "deny"`. Extend `deny.toml [bans] skip` list with one-line provenance comments per duplicate; do NOT relax the `multiple-versions` setting. Tauri 2.10 → tauri-plugin-updater 2.10 added 4 new known-benign duplicates: `jni`, `jni-sys`, `redox_syscall`, `windows_i686_gnullvm`. The `tonic 0.14 ↔ 0.13` check (security plan §Dependency Security anchor) MUST stay outside the skip list — it's the canary for the OTLP-receiver / opentelemetry-otlp duplicate.

- 2026-05-03: TauRPC integrates at the Tauri IPC channel level — a single invoke handler (`taurpc::create_ipc_handler(...)`) dispatches ALL router methods. Capability JSON `permissions` array contains `core:default` + plugin-level perms (e.g. `updater:default`), NOT per-procedure entries (no `"health"` / `"app_info"` strings). This is a tension with arch §Cross-cutting Patterns "Webview IPC capability policy" which assumes per-procedure capability JSON pairing. The `xtask capability-drift` check (route#22) must therefore introspect the TauRPC router as the unit of granularity (router-level coverage), not per-procedure. Per-procedure granularity is only available with stock `#[tauri::command]` (auto-generated `core:command:allow-{name}` permissions) — not with TauRPC. Document this difference when implementing `xtask capability-drift` so the gate doesn't false-positive expecting per-procedure entries.
