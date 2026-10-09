# security extract

## Relevance
partial — security does not drive this data-flow correctness fix, but the workspace key it reconciles is workspace-detector-derived and flows into a corpus SQL filter (and likely diagnostic logs), so several guardrails constrain the implementation.

## Constraints
- Where the reconciled workspace key is applied as a filter in `list_active()` / the per-service join, it MUST be bound as a prepared-statement `?` parameter, NEVER string-interpolated into SQL — the key is a filesystem path (can contain quotes/separators) and incidents live in the encrypted corpus.db SQLite (per security-plan §Input Validation "DuckDB query parameters" row + §Security Anti-Patterns §Input SQL-interpolation ban).
- Workspace-detector output — now the canonical key on both sides — MUST NOT reach `tokio::process::Command::new(...).arg(...)`; workspace-detector output is a named command-injection taint source, and this chunk elevates it to a load-bearing value (per §Security Anti-Patterns §Code Patterns, CVE-2025-49596 cluster).
- Canonicalization for cross-platform string-equality (`\\?\` prefix, trailing-sep/case) SHOULD use the plan's approved primitive (`strict-path` / `Path::canonicalize()`), not a hand-rolled prefix strip (per §Input Validation env-var canonicalization discipline + §Bootstrap phases `input-validation-library-install`). Note: the assert-under-data-dir confinement ban (§Anti-Patterns §Input) does NOT apply — the key intentionally points at the host project tree, not under the data dir.
- Any NEW crate pulled in for `\\?\` de-UNC (e.g. `dunce`) MUST pass `cargo deny check bans licenses sources` and be version-pinned in the workspace `Cargo.toml` (per §Dependency Security + §Bootstrap phases `dep-security-ci-gate`); reuse `strict-path` to avoid a new dep.
- Diagnostic/tracing output proving the key match MUST NOT emit the full workspace path — redact to basename or stable hash (per §Logging & Monitoring "NEVER log full plugin file paths — basename only"; a full workspace path leaks the user home/username).
- Errors surfaced if workspace resolution fails MUST collapse to a sanitized `AppError` variant across the TauRPC bridge with no file path in the message (per §Error Handling + §Anti-Patterns §Code Patterns "NEVER serialize `anyhow::Error` across the bridge").

## Patterns to follow
- Prepared-statement + `?` placeholder binding for the workspace filter — the existing `viz`/corpus query pattern (per §Input Validation "DuckDB query parameters" row).
- `strict-path` canonicalize primitive already wired for plugin/config/env paths — reuse it for the key-parity canonicalization (per §Bootstrap phases `input-validation-library-install`).
- `From<thiserror::Error> for AppError` sanitizing conversion at the bridge (per §Error Handling; §Bootstrap `error-sanitization-wire`).
- Bounded truthy-parse env-gating for the deterministic-L4 storm harness (`ANDROMEDA_PULSE_L4_DETERMINISTIC`, `1|true|yes`) used by the acceptance test (per §Input Validation "CLI / env var inputs" row).

## Anti-patterns to avoid
- String-interpolating the workspace key into the `list_active()` SQL (§Anti-Patterns §Input DuckDB ban).
- Piping workspace-detector output into a subprocess arg (§Anti-Patterns §Code Patterns command-injection ban).
- Logging the full workspace path or leaking it in an `AppError::Internal { message }` (§Anti-Patterns §Logging path-redaction + error-disclosure bans).

## Contract bindings
- Logging path-redaction binds to obs (self-observation `tracing` sink; field redaction at the subscriber layer per §Logging "Log format") — a new workspace-key diagnostic field must be redacted there; binds to tests (no real user path in fixtures).
- Deterministic-L4 storm gating binds to the tests harness via `ANDROMEDA_PULSE_L4_DETERMINISTIC` (the validated env boundary) — one env-gated integration test.
- CI security gate: NOT triggered — chunk reuses existing `services.list_with_states` + `incidents.list_active` (no new TauRPC procedure → no `pulse-app/capabilities/` diff → xtask capability-drift gate unaffected, per §API Security TauRPC capability authorization). Planner should not touch capabilities.

## Acceptance criteria contributions
- (security) The workspace-key filter uses prepared-statement parameter binding — read/grep confirms no `format!`-interpolated workspace value in the `list_active()` / join SQL (§Anti-Patterns §Input).
- (security) No workspace-detector-derived value reaches a `Command::new(...).arg(...)` call site (§Anti-Patterns §Code Patterns).
- (security) Chunk-added diagnostic/tracing output emits no unredacted full workspace path (§Logging NEVER-log path discipline).
- (security) If a dep is added for `\\?\` normalization, `cargo deny check` passes and the crate is pinned (§Dependency Security); else N/A.

## Relevant amendment history
- `2026-06-28-deterministic-env-gated-l4-mode` — registered `ANDROMEDA_PULSE_L4_DETERMINISTIC` (bounded truthy-parse) in §Input Validation. Directly relevant: this chunk's acceptance drives a deterministic-L4 storm through that env var — it is the validated boundary the test harness uses; codified as routine (bounded-config-input playbook), so no re-escalation.
- `2026-06-29-window-geometry-movable-shell` — registered the `window-geometry.json` boundary; precedent for (a) the "no coordinate/path values logged" redaction discipline this chunk inherits, and (b) the playbook rule that a value/canonical-form change to an EXISTING surface (no new deserialized file/env var) is routine registry-completeness, NOT an unvalidated-boundary HALT — so this chunk adds no new §Input Validation boundary registration.
- `2026-05-22` (chunk #77) — established that active incidents persist in the encrypted corpus.db (`incidents` table, AES-256-GCM); background for why `list_active()` filters against the corpus SQLite and must use prepared statements.
