# security extract

## Relevance
Partial — reuses the existing read-only `viz.query.traces` IPC under `pulse:default`; adds no new boundary/dep/secret/auth. Security bites only on the conditional Rust CARRY (DuckDB cursor SQL) plus standing error-sanitization + log-redaction disciplines on the reused viz-query path.

## Constraints
- **DuckDB prepared-statement discipline (CONDITIONAL on CARRY):** if the `next_cursor` keying fix activates and touches the `viz` traces query SQL, cursor/limit/offset values MUST bind via `Connection::prepare` + `?` placeholders; identifier interpolation is rejected entirely and table/column names stay spec-fixed (per security-plan §Input Validation, DuckDB query parameters row).
- **Capability reuse, no drift:** the re-poll reuses `viz.query.traces` — do NOT add a router procedure, and add no `pulse-app/capabilities/` entry; both-piece rule means an unlisted procedure is silently runtime-rejected (per security-plan §API Security, TauRPC capability authorization).
- **Bridge error sanitization:** errors from the re-invoked query surfaced to the UI must be the `serde`-friendly `AppError` enum (`Validation`/`NotFound`/`Internal`/`Storage`); never serialize `anyhow::Error` across the bridge; `Internal { message }` carries only a sanitized one-liner (per security-plan §Error Handling, TauRPC bridge).
- **Log redaction on the query path:** any tracing added on the refresh/query path MUST NOT emit DuckDB query-parameter values (log query id + param count) or raw OTLP attribute/span-content values (per security-plan §Logging & Monitoring, NEVER-log list).
- **No-new-dependency guard:** the ~1s cadence should use existing webview timer / tokio primitives; any added crate pulls in `cargo-audit` + `cargo deny check bans` gates and 40-char-SHA action pinning (per security-plan §Dependency Security) — expected N/A.

## Patterns to follow
- Existing `viz` traces/metrics/logs handlers already use `duckdb` prepared statements with `?` binding — the CARRY cursor fix extends this existing pattern, it does not invent a new query shape (security-plan §Input Validation).
- TauRPC procedures return `Result<T, AppError>` with `From<thiserror::Error>` conversion at the bridge — the re-poll reuses the existing `viz.query.traces` result contract unchanged (security-plan §Error Handling).
- `viz.query.traces` is a read-only query over the in-memory DuckDB ring buffer behind `pulse:default`; the re-poll inherits this IPC trust boundary unchanged (security-plan §Threat Model Summary, IPC vector).
- Field redaction lives at the `tracing-subscriber` subscriber layer, not at call sites — honor this if instrumentation is added (security-plan §Logging & Monitoring, Log format).

## Anti-patterns to avoid
- NEVER `format!("SELECT … WHERE … = '{}'", user_input)` against the `duckdb` Connection — always prepared statements + `?` (fires only if the CARRY touches cursor SQL) (security-plan §Security Anti-Patterns §Input).
- NEVER add a TauRPC procedure to a router without a matching `pulse-app/capabilities/` entry (silent runtime rejection) — guard against a re-poll design that reaches for a new procedure (security-plan §Security Anti-Patterns §API).
- NEVER expose stack traces / Rust struct names / file paths / library versions in `AppError::Internal` surfaced to the webview; sanitize at the `From<…> for AppError` impl (security-plan §Security Anti-Patterns §Logging + §Code Patterns).

## Contract bindings
- **Security CI gate ↔ tests harness:** if any Rust is touched (CARRY active), the deferred `capability-drift` xtask check + `clippy` + `nextest --workspace` re-runs (per scope Gate note) come due — the capability-drift check is the security-owned job inside the tests CI workflow (security-plan §API Security / §Bootstrap `dep-security-ci-gate`).
- **Log redaction ↔ obs PII scrubbing:** this chunk renders to the first-party webview only — it does NOT cross a persistence/log sink, so the uniform `security::scrubber::scrub_attribute` persistence-boundary coverage is NOT triggered here; binding is latent, not active.
- Auth / design / a11y: (none).

## Acceptance criteria contributions
- (security) IF CARRY active: touched `viz` traces cursor/limit binds via `?` placeholders — grep of the changed query shows no `format!(`-into-SQL interpolation.
- (security) No new TauRPC procedure and no new `pulse-app/capabilities/` entry added (re-poll reuses `viz.query.traces`); if any Rust is touched, `xtask` capability-drift check passes.
- (security) Any refresh-path query error surfaced to the UI is a sanitized `AppError` variant — no stack trace / file path / struct name reaches the webview.
- (security) No DuckDB query-parameter values or raw OTLP attribute values added to log/tracing emission on the refresh path; if Rust touched, `cargo audit` + `cargo deny check` stay green.

## Relevant amendment history
(none directly) — no amendment touches the traces query / `viz` DuckDB path / pagination-cursor / webview data-fetch area. Nearest are the two §Input Validation registry-completeness registrations (2026-06-28 `ANDROMEDA_PULSE_L4_DETERMINISTIC`; 2026-06-29 `window-geometry.json`), which establish that NEW input boundaries get registered in §Input Validation — but this chunk adds no new boundary (reuses `viz.query.traces`; the CARRY modifies an existing DuckDB query boundary, not a new one), so that pattern does not fire here.
