# Security Summary

_Distilled from `.andromeda/security-plan.md` by `/setup-project`. Read on demand. **Tier: Minimal (0)** — local-first Tauri 2 desktop app, loopback-only OTLP, in-memory DuckDB, no user accounts, no compliance triggers._

## Threat model snapshot
- **Tier:** Minimal (0). Justification: zero-infrastructure single-user desktop app; in-memory ring buffer with 5–10 min retention; OTLP receivers bound `127.0.0.1` only; no compliance-regulated data classifications.
- **Auth model:** none — Tauri 2 capability gating (`pulse:default`, `pulse:tray`, `pulse:notification`, `pulse:updater`, `pulse:plugin-fs`) substitutes for runtime authorization. The OS user owns the app.
- **Compliance triggers:** none — no PCI DSS (no payment), no GDPR-as-controller (telemetry stays on user machine), no HIPAA (no health framing), no COPPA.

## Data classifications
- **user-content (telemetry payloads):** OTLP traces / metrics / logs in-memory in DuckDB ring buffer + curated snapshots persisted to `~/.andromeda-pulse/snapshots/`. Telemetry can incidentally contain secrets, IDs, URLs, error messages, SQL fragments from instrumented host applications — OTLP attributes are user-controlled content.
- **config:** `~/.andromeda-pulse/config.toml` + env vars. Surface is small (enum settings, port ranges, file paths) — with ONE secret-bearing exception: `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` (opt-in corpus-key fallback; bounded parse ≤1024 bytes, secret-class, never logged).
- **secret (corpus encryption key):** 32-byte AES-256-GCM cell key for `~/.andromeda-pulse/corpus/corpus.db`, sourced from the OS credential store via `keyring` 3 declared with its explicit platform feature set (keyring 3.x has no `default` feature — a bare `keyring = "3"` links no backend and silently yields a per-process key). Opt-in `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` BLAKE3 fallback, warning once per boot; nothing written to disk. Never logged, never exported (chunk 2026-08-15-corpus-key-persistence).
- **config (signing/release):** GitHub Actions encrypted secrets + Azure Key Vault Premium SKU. Apple Developer ID. Updater public key baked into `tauri.conf.json`.
- **user-content (third-party WASM plugins):** `~/.andromeda-pulse/plugins/` loaded by `wasmtime` Component Model host. Capability-scoped per WIT imports. Signed-plugin verification deferred post-v1.
- **NONE:** credentials / PII / payment / health.

## Attack surface (vectors)
1. **OTLP/gRPC `:4317` + OTLP/HTTP `:4318`** — bound `127.0.0.1` ONLY. `tonic` 0.14 + `axum` 0.8 servers; `prost`/`tonic` decode validates wire format.
2. **TauRPC IPC bridge** — `pulse:default` admits the webview to the IPC layer as a whole (no per-procedure enumeration; one TauRPC invoke handler, measured 2026-08-21); per-procedure coverage is the validated argument struct + the `EXPECTED_PROCEDURES` drift gate. Errors collapse to `serde`-friendly `AppError` enum.
3. **Plugin host (WASM Component Model)** — `~/.andromeda-pulse/plugins/` filesystem load by `wasmtime` 25+. Capability-scoped sandboxing; guests receive only host imports declared in WIT.
4. **MCP stdio surface** — `andromeda-pulse-mcp` rmcp sidecar. JSON-RPC 2.0 over stdin/stdout. Feature double-gated (`--features mcp-server` + `ANDROMEDA_PULSE_MCP_ENABLED=true`).
5. **Filesystem reads (config + workspace detection)** — env-overridable paths require `strict-path` canonicalization + confinement.
6. **In-app updater** — `tauri-plugin-updater` 2.x consuming `latest.json` from GitHub Releases. Minisign Ed25519 verification mandatory and cannot be disabled.
7. **OS notification / tray** — outbound user-facing only (capability-gated).
8. **CLI / env vars** — `serde` + `TryFrom<u16>` validation.
9. **Webview content** — Tauri capability model gates IPC; no `fs`/`shell`/`dialog`/`http` core APIs granted to `pulse:default`.

## Bootstrap phases (route ordering)
Per security plan §Bootstrap phases:
1. `input-validation-library-install` — `serde` + `TryFrom<u16>` + add `strict-path` for path canonicalization. Defer `garde` 0.20+ until plugin manifest cross-field validation needed.
2. `dep-audit-tooling-install` — `cargo-audit` 0.22.1 + `cargo-deny` 0.19.4 (with `[bans] multiple-versions = "deny"` + explicit `tonic` ban entry until reconciliation lands; `[licenses]` SPDX allowlist; `[sources]` restricted to `crates-io`) + `cargo-auditable` 0.7.4 + Dependabot.
3. `secret-management-init` — Azure Key Vault Premium SKU (HSM-RSA Windows EV) + GitHub OIDC federation; Tauri updater Minisign Ed25519 keypair generation; private keys never leave Vault.
4. `secret-scanning-ci-gate` — pre-commit + per-PR (gitleaks or trufflehog SHA-pinned). `.gitignore` covers `*.p12`, `*.pem`, `*.cer`, `.env*`, `*.key`, `~/.tauri/*.key`.
5. `error-sanitization-wire` — `AppError` boundary collapse; `tonic::Status` for OTLP; JSON-RPC 2.0 error object for MCP. No stack traces / paths / library versions / Rust struct names leak.
6. `logging-redaction-wire` — `tracing-subscriber::fmt::Layer::json()` writing to `~/.andromeda-pulse/logs/agent-latest.jsonl` (canonical self-observation surface per obs-plan §3 `2026-05-02 — Phase 3.5 pivot to tracing-only self-observation`; legacy `opentelemetry-stdout` references in security-plan.md §Data Protection / §Bootstrap phases / §Logging & Monitoring bodies are obsolete-but-equivalent — both produce JSON-per-line at the same path; functionally identical; no security-posture change. Body sites annotated with `> **DEPRECATED (2026-05-08)**` blockquotes preserving content verbatim for audit trail). Snapshot/clipboard/MCP-tool-response paths apply attribute-value redaction. Per amendments `2026-05-08T17-28-25Z-reconcile-otel-stdout-references` (Decisions Log only) + `2026-05-08T21-00-00-obs-pivot-security-bodies` (body annotations at security-plan.md lines 154/239/330).
7. `dep-security-ci-gate` — `cargo audit` + `cargo deny check` + `Cargo.lock` integrity + `xtask capability-drift` + `step-security/harden-runner` (SHA-pinned, egress-policy: audit then promote to block).

## Top anti-patterns (universal — see `.claude/rules/security.md` for full list)
- NEVER bind OTLP receivers to `0.0.0.0` or any non-loopback interface.
- NEVER `format!("SELECT … WHERE service_name = '{}'")` against the `duckdb` crate — ALWAYS prepared statements with `?` placeholders.
- NEVER skip post-`prost`-decode invariant checks (`span_id` is 8 bytes, `trace_id` is 16 bytes).
- NEVER read path env vars without `strict-path` canonicalize + confinement.
- NEVER add a TauRPC procedure without its `EXPECTED_PROCEDURES` pin in `xtask/src/main.rs` + a validated argument struct (per-procedure capability JSON entries do NOT exist — one TauRPC invoke handler). NEVER grant a core API (`fs`/`shell`/`dialog`/`http`) or a `core:window:*` permission without an explicit capability addition — THOSE are silently rejected at runtime.
- NEVER override `tauri-plugin-updater` Minisign verification; NEVER ship the Minisign **private** key in repo.
- NEVER reference 3rd-party Actions by `@v2` / floating tag — pin by 40-char SHA (tj-actions/changed-files CVE-2025-30066, 23k repos).
- NEVER widen `pulse:default` with Tauri core APIs (`fs`, `shell`, `dialog`, `http`).
- NEVER `tokio::process::Command::new(...).arg(user_input)` against OTLP attribute / MCP tool argument / workspace-detector output.
- NEVER ship release without resolving `tonic 0.14 ↔ tonic 0.13 (via opentelemetry-otlp 0.31)` duplicate.
- NEVER let rust-toolchain drift below `1.85.0` — Edition 2024 cannot parse without it.

## Open residual risks (Decisions Log)
- **WASM plugin signature verification deferred post-v1** — third-party plugins run unverified in v1; capability-scoped WIT + `ResourceLimiter` mitigate impact, not provenance. Document in user-facing plugin install README.
- **Snapshot / clipboard / MCP tool response surfaces** — documented OTLP-attribute leakage paths; addressed via user-facing warnings + visible clipboard-write event + README documentation rather than attempted sanitization.
- **No hot key rotation path for Tauri updater Minisign keypair** — runbook required before v0.1.0 ships.

## Code-signing key custody
- Tauri updater Minisign Ed25519 private key + Windows EV cert HSM-RSA + Apple Developer ID — all in Azure Key Vault Premium SKU. GitHub OIDC federation (no long-lived service principal secret in GitHub Actions).
- All signing secrets in GitHub Environment `production-release` with manual approval gate.
- `permissions: { contents: read }` at workflow level; elevate per-job to `write` only on publish step.

Full plan: `.andromeda/security-plan.md`.
