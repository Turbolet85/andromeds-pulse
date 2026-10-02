# Security validation — route draft

## No suggestions

Security domain covered. v0.3.0 route inherits complete v0.1/v0.2 bootstrap scaffolding; no new requirements introduced.
- **Bootstrap items present:** input-validation (env-var + strict-path canonicalization), dep-audit (cargo-audit/deny/auditable CI), secret-management (Azure Key Vault OIDC + Tauri Minisign + corpus AES-256-GCM per chunk #68/#73), secret-scanning-ci, error-sanitization (AppError boundary), logging-redaction (uniform scrubber per chunk #72), dep-security-ci (xtask capability-drift + harden-runner) → all v0.1/v0.2. Skipped per plan: auth-scaffolding (Minimal tier = no user auth), supply-chain-signing (Minisign in secret-management; sigstore deferred to Standard).
- **Sequencing deps satisfied:** loopback-only OTLP bind, MCP double-gate (--features mcp-server AND ANDROMEDA_PULSE_MCP_ENABLED per §API Security), corpus encryption + PII scrubber all pre-exist the new v0.3.0 features.
- **Coverage:** telemetry scrubbed at all persistence boundaries (chunk #72); corpus AES-256-GCM + scrubbed; no new data classifications or threat-model boundaries in L4-mode / Investigate / window-hardening chunks.
