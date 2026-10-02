# Security validation — route draft

## No suggestions

Draft adequately covers security domain.

- **Bootstrap items present:**
  - **input-validation-library-install**: present as "Code-signing setup — Azure Key Vault HSM + GitHub OIDC + Apple Developer ID + Tauri updater Minisign Ed25519 keypair" (Foundation) + implicit serde/strict-path wiring across Ingest, TauRPC bridge, plugin, and MCP boundaries per §Input Validation
  - **dep-audit-tooling-install**: present as "Supply-chain CI gates + secret-scanning — cargo-audit/deny/auditable + Dependabot + gitleaks + GitHub Environment production-release" (Foundation) per §Dependency Security
  - **secret-management-init**: present as "Code-signing setup — Azure Key Vault HSM + GitHub OIDC + Apple Developer ID + Tauri updater Minisign Ed25519 keypair" (Foundation) per §Secret Management
  - **secret-scanning-ci-gate**: present as "Supply-chain CI gates + secret-scanning" (Foundation) per §Secret Management
  - **error-sanitization-wire**: present as "AppError serde enum + From impls — Validation/NotFound/Internal/Plugin/Storage/Ingest with sanitization (no stack traces / paths)" (Epoch 4) per §Error Handling
  - **logging-redaction-wire**: present as "Tracing self-observation harness — tracing + tracing-subscriber JSON + tracing-appender daily + tracing-error + service.name" (Foundation) per §Logging & Monitoring
  - **dep-security-ci-gate**: present as "Base CI workflow — matrix Linux/macOS/Windows + harden-runner SHA-pinned + cargo-nextest + cargo-llvm-cov coverage gate" (Foundation) + "IPC introspection + capability-drift check" (Epoch 4) per §API Security and §Bootstrap phases

- **Sequencing dependencies satisfied:**
  - **Secret management before credential-using chunks**: confirmed — Code-signing setup (Foundation) precedes all downstream ingest/storage/webview/plugin epochs
  - **AppError sanitization before user-facing chunks**: confirmed — AppError serde enum + From impls (Epoch 4 Webview shell) precedes Visualization surfaces (Epoch 5)
  - **CI security gate in Foundation**: confirmed — Base CI workflow + supply-chain gates land in Foundation before any feature epoch
  - **OTLP input validation at loopback boundaries**: confirmed — Ingest pipeline (Epoch 2) specifies "post-decode invariants" and "CORS deny" per §Input Validation OTLP/gRPC and OTLP/HTTP rows
  - **WASM sandboxing before plugin invocation**: confirmed — Plugin host capability sandbox + ResourceLimiter wired in Epoch 7 per §API Security plugin-host capability sandbox
  - **MCP feature double-gate**: confirmed — "rmcp stdio sidecar + double-gate" in Epoch 7 per §API Security MCP feature double-gate

- **Coverage:**
  - **Every threat-model boundary acknowledged**: confirmed — OTLP loopback receivers (Ingest), TauRPC capability-drift check (Epoch 4), WASM Component Model sandboxing (Epoch 7), plugin WIT capability scope (Epoch 7), MCP stdio isolation (Epoch 7)
  - **Data classification implementations present**: confirmed — OTLP ingest boundaries (Epoch 2), DuckDB prepared statements (Epoch 3), snapshot clipboard hygiene via "Workspace path detection + clipboard + notification" (Epoch 6), plugin path logging via "basename-only path logging" (Epoch 7 summary)
  - **Minimal-tier security tier requirements satisfied**: confirmed — no auth scaffolding chunks (tier = none); loopback-only OTLP binding (Ingest bounds to 127.0.0.1); no persistent user DB (DuckDB :memory:); code-signing + Minisign updater verification (Foundation + Epoch 8 release pipeline)
