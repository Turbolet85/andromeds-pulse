# security extract

## Relevance
Partial — the chunk is primarily webview presentation (row ordering, semantic error token, filter control), which is design/a11y-domain; security applies only on the conditional "surface the signal" branch that touches the `viz` `traces.*` query, plus the cross-cutting logging-redaction / error-sanitization / no-real-PII-fixture discipline any code on the `traces.*` path inherits.

## Constraints
- **DuckDB parameterization (conditional).** IF the "row lacks an error/status signal" branch adds a field or an errors-only predicate to the `viz` crate `traces.*` handlers, the query MUST use `duckdb` prepared statements with `?` placeholders + parameter binding; NEVER `format!("… WHERE … = '{}'")` (per security-plan §Input Validation DuckDB row + §Security Anti-Patterns §Input — Dagster/Vanna/dagster-duckdb CVE cluster). Filter/limit/offset stay bound; table/column names are spec-fixed and user input never names a table.
- **No attribute-value logging.** Any diagnostic/log emission added around sort/filter/anomaly-signal code MUST NOT log OTLP attribute values (error message, service/span name) or DuckDB query parameter values — incidental-secret vectors (per §Logging & Monitoring "What NEVER to log" + §Security Anti-Patterns §Logging). Log query identifier + parameter count, not values.
- **Bridge error sanitization.** IF the viz-query path is touched and can error, errors crossing the TauRPC bridge MUST collapse to the `serde`-friendly `AppError` enum — no stack traces, file paths, Rust struct names, or library versions to the webview; NEVER serialize `anyhow::Error` directly (per §Error Handling + §Security Anti-Patterns §Logging/§Code Patterns).
- **No new procedure / capability discipline.** Scope confirms no new TauRPC procedure; reusing existing enumerated `traces.*` needs no capability change. IF a procedure signature is nonetheless touched, a matching `pulse-app/capabilities/` entry is required or the call is silently runtime-rejected (per §API Security TauRPC capability authorization + §Security Anti-Patterns §API).
- **Presentation path introduces no new input boundary.** Client-side filter/sort operates on already-fetched first-party Arrow IPC rows from `viz`/`buffer` (outbound-only) — the preferred path adds zero untrusted-input surface; only plugin-sourced Arrow re-emit would need a size cap, which is out of this chunk's scope (per §Input Validation Tauri `Channel` streaming-payloads row).

## Patterns to follow
- **Prepared-statement query path** — reuse the existing `Connection::prepare` + `?` + `Statement::execute([&param])` shape already used by `traces.*`/`metrics.*`/`logs.*` handlers for any error-count/status column addition (per §Input Validation DuckDB row).
- **`AppError` `From` conversion** — module-internal `thiserror` 2.x enums convert to `AppError` via `From` impls at the bridge; consume the existing envelope, do not invent a new one (per §Error Handling TauRPC bridge).
- **Subscriber-layer redaction** — field redaction is applied at the `tracing-subscriber` layer (sink `~/.andromeda-pulse/logs/agent-latest.jsonl`, tracing-only, no OTel SDK), and `security::scrubber::scrub_attribute` is the shared primitive at persistence/log boundaries; presentation code should not hand-roll redaction (per §Logging & Monitoring Log format + §Security Anti-Patterns §Logging uniform-scrubber).
- **Capability negative-default** — `pulse:default` enumerates exactly the existing `traces.*` procedures; the scope's preferred "consume the existing procedure" path requires no capability edit (per §Input Validation TauRPC row + §API Security).

## Anti-patterns to avoid
- NEVER `format!`-interpolate a filter/service-name/limit into DuckDB SQL — prepared statements with `?` only (per §Security Anti-Patterns §Input). Applies only if a filter is pushed server-side rather than done client-side.
- NEVER log raw OTLP attribute values (error messages, service/span names) or DuckDB query parameter values when instrumenting the sort/filter/anomaly code (per §Security Anti-Patterns §Logging).
- NEVER add a `traces.*` procedure without a matching `pulse-app/capabilities/` entry — silent runtime rejection (per §Security Anti-Patterns §API); scope expects no new procedure at all.

## Contract bindings
- **security ↔ tests (no real PII in fixtures):** the P-068 "mixed dataset" fixture (healthy + `payment-service` erroring) MUST use synthetic error content — no real secrets/PII — per the focus-guide PII-scrubbing binding + §Security Anti-Patterns §Logging uniform-scrubber discipline.
- **security ↔ obs:** `security::scrubber::scrub_attribute` is the shared enforcement primitive for any attribute value crossing into a log/persistence sink; this chunk should stay clear of that path (surface in webview only, do not persist/log the anomaly attribute content) (per §Security Anti-Patterns §Logging + §Data Protection At-rest).
- Semantic-error-token color/contrast/non-color-only encoding binds to design + a11y (SC 1.4.1), **not** security — flagged only to disclaim (design tokens/layout are out of the security domain per the focus guide).

## Acceptance criteria contributions
- (security) IF a viz-query field/filter is added: grep confirms no `format!`-built SQL in the touched `viz` handler and `?`-placeholder binding is present.
- (security) grep confirms no new log/`tracing` call in the touched sort/filter/anomaly code emits OTLP attribute values or query parameter values.
- (security) git diff shows no new `#[taurpc::procedure]`; if any procedure signature changed, the xtask capability-drift check (TauRPC procedures vs `pulse-app/capabilities/`) passes.
- (security) the P-068 webview-test fixture uses synthetic error content (no real secrets/PII), consistent with the no-real-PII-in-fixtures / uniform-scrubber discipline.

## Relevant amendment history
- **2026-05-22 (chunk #77) — §Anti-Patterns §Logging uniform-scrubber-coverage framing:** established that `security::scrubber::scrub_attribute` covers ALL persistence + self-observation log boundaries (not single-site). Relevant because this chunk's "no attribute-value logging" constraint is enforced by that scrubber; why — reconciled the plan body with post-#72 uniform-coverage reality.
- **2026-06-28 — `opentelemetry-stdout` → tracing-only self-observation:** self-observation is now `tracing-subscriber` JSON → `~/.andromeda-pulse/logs/agent-latest.jsonl` (no OTel SDK), redaction applied at the subscriber layer. Relevant because the logging-redaction discipline this chunk inherits points at that sink; why — obs-plan §12 Phase 3.5 pivot (no security-posture change).
- Other amendments (v3 Decisions-Log externalization, `max_wasm_http_fields` correction, §Auth trim, `ANDROMEDA_PULSE_L4_DETERMINISTIC`, `window-geometry.json` boundary) do not touch this chunk's area (traces webview presentation / viz query / DuckDB / bridge errors).
