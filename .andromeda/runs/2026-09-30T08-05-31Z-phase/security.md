# security extract

## Relevance
partial — the chunk adds no new network surface, dependency or secret. It does open a new OTLP-fed write path into the service registry (event-driven registration at first sighting), and it may add log records and change an IPC payload, so the scrub, logging and IPC rules apply to those seams.

## Constraints
- A service name that enters the registry through a new first-sighting path must be the SCRUBBED identity. security-plan §Security Anti-Patterns → Logging (the `extract_service_name` choke-point paragraph) requires `service_name` to be scrubbed at one choke point that feeds the `spans` column, the storm observer and the baseline `SpanObserver` tap. A registration path that reads the raw resource attribute would reopen a raw path into `triage` and desync DuckDB service identity from the registry. Whether the candidate first-sighting hook sits downstream of that choke point is research's question.
- The registry's corpus persist stays inside the scrubbed boundary. security-plan §Security Anti-Patterns → Logging (MEASURED reality) lists "ServiceRegistry corpus persist" among the scrubbed persistence boundaries, and security-plan §Threat Model Summary classifies `service_registry` as a cell-level AES-256-GCM encrypted corpus table. Any new write into the registry (for example a new Discovered/first-seen state or a first-seen timestamp) must go through the same persist path and must not add an unencrypted or unscrubbed side channel.
- No new or changed tracing record may carry an OTLP attribute value verbatim. security-plan §Logging & Monitoring "What NEVER to log" bans OTLP attribute values, and `service.name` is one. A first-sighting or registration record, or a change to `metric.constellation.discovery_ms`, carries only bounded fields (durations, counts, enum labels) or the already-scrubbed name. Whether the existing discovery record carries a service identifier at all is research's question.
- If the P-027 anchor change adds a field to the `services.list_with_states` payload or adds a procedure, the IPC rules hold. security-plan §Threat Model Summary (IPC vector) and §API Security (TauRPC capability authorization row) require per-procedure coverage through argument-level `serde` validation plus the `EXPECTED_PROCEDURES` drift gate, since the capability layer does not enumerate procedures. A NEW procedure needs its pin. A new field on an existing response needs none but must pass the drift gate.
- Resolver errors on any touched TauRPC path stay `AppError` variants with sanitized messages. security-plan §Error Handling and §Security Anti-Patterns → Code Patterns require no stack traces, file paths, struct names or `anyhow::Error` across the bridge.
- A registration path fed by OTLP arrival runs on untrusted, loopback-sourced input. security-plan §API Security (rate-limiting row) records that a runaway local producer can saturate the OTLP path. The new path must not add per-span unbounded work, such as a corpus write per span instead of per first sighting. The plan mandates no registry-size cap, so whether one exists is research's question and not a requirement here.

## Patterns to follow
- The choke-point scrub shape: consume the identity `extract_service_name` already produces rather than re-extracting from the resource (per security-plan §Security Anti-Patterns → Logging).
- `ServiceRegistry` corpus persist through its existing scrubbed and encrypted writer, reusing a dedicated column-specific writer if one exists rather than widening a general updater (per security-plan §Security Anti-Patterns → Logging MEASURED reality, and §Threat Model Summary corpus classification).
- Bounded-field log records, on the `ui.ipc.rejection` precedent: a record behind its own allowlist leaf carrying only a closed set of bounded fields (per security-plan §Security Anti-Patterns → Logging, the NO-SCRUB boundary paragraph).
- `EXPECTED_PROCEDURES` pin plus `serde`-validated argument struct for any procedure-set change (per security-plan §API Security, TauRPC capability authorization row).

## Anti-patterns to avoid
- NEVER log raw OTLP attribute values, including `service.name`, from the new discovery or registration path (per security-plan §Logging & Monitoring "What NEVER to log").
- NEVER build DuckDB SQL by interpolating a service name if the fix adds or changes a first-seen query. Use `Connection::prepare` with `?` placeholders (per security-plan §Security Anti-Patterns → Input).
- NEVER serialize `anyhow::Error` across the TauRPC bridge from a touched resolver (per security-plan §Security Anti-Patterns → Code Patterns).

## Contract bindings
- security ↔ obs: redaction on any new or changed discovery/registration tracing record binds to the obs-plan's field allowlist and schema. Security owns the "no OTLP attribute value" ban, and obs owns the record's shape and allowlist leaf.
- security ↔ tests: the live leg's wire read of `agent-latest.jsonl` doubles as the redaction witness. A synthetic service name in the fresh-boot leg can be asserted absent from, or present only in scrubbed form in, the new records. Fixtures carry no real PII.
- security ↔ arch/tests (CI gate): `cargo xtask capability-drift`, run before the workspace nextest per the operator's gate order, is the enforcement point if the IPC payload or procedure set changes.

## Acceptance criteria contributions
- The registry entry created by the first-sighting path holds the same scrubbed `service_name` that the `spans` column holds for that service. A credential-shaped service name in a test reaches the registry as the redaction placeholder, never verbatim (per security-plan §Security Anti-Patterns → Logging, `extract_service_name` choke point).
- The live leg's log contains no raw OTLP attribute value from the new or changed discovery/registration records. Grep the fresh-boot log for a canary service name and require 0 verbatim hits, or only scrubbed hits (per security-plan §Logging & Monitoring "What NEVER to log").
- `cargo xtask capability-drift` is clean, including the staged-bindings assertion, after any `services.list_with_states` payload or procedure-set change (per security-plan §API Security, TauRPC capability authorization row).
- `cargo deny check bans licenses sources` passes unchanged. The chunk adds no dependency (per security-plan §Dependency Security).
