# security extract

## Relevance
relevant — the chunk widens the MCP stdio trust boundary (a new or extended tool reading the encrypted corpus cross-process); the stderr-sink CARRY is obs-domain and out of this extract.

## Constraints
- The MCP stdio surface must stay double-gated: compile-time `--features mcp-server` AND runtime `ANDROMEDA_PULSE_MCP_ENABLED=true`, with the graceful-degrade `warn` on an env/feature mismatch preserved; a new or extended tool inherits the gate and adds no ungated path (per security-plan §API Security "MCP feature double-gate" row; §Security Anti-Patterns → Code Patterns double-gate ban).
- The new read's tool argument (the incident id, plus any count/limit) must arrive through a `serde` argument struct at the `dispatch_tool` name-dispatch boundary, bounded before use; the plan's MCP stdio row names serde argument structs as the validation mechanism — whether the existing incident tools already bound the id (length / charset) is research's question, and the new read should match or exceed that (per security-plan §Input Validation, "MCP stdio inputs" row).
- MCP tool response bodies must never be logged; the read's self-observation may carry only bounded, non-content fields (per security-plan §Logging & Monitoring "What NEVER to log"; §Security Anti-Patterns → Logging).
- Errors from the new read (unknown incident, corpus open/decrypt failure) must surface as a standard JSON-RPC 2.0 error object — numeric `code`, sanitized `message`, optional `data` — with no file paths, hostnames, IPs, library versions or Rust struct names, and no new envelope invented (per security-plan §Error Handling "MCP server" + "Error format"; §Security Anti-Patterns → Logging "NEVER expose internal hostnames or IPs … MCP error objects").
- `incident_events` cells are AES-256-GCM encrypted with the persisted corpus key; the read must go through the corpus crate's decrypting path, never return a raw ciphertext BLOB, and treat a decrypt/integrity failure as an error, not as data (per security-plan §Data Protection "Persistent incident corpus"; §Threat Model Summary corpus data type — cell-level encryption + GCM integrity tag).
- The plan's `incident_events` NO-SCRUB justification rests on `event_kind` being a bounded label from the closed `IncidentStatus` set and the payload being empty; the read surface must not widen what crosses the boundary (no client text added to the response), or that justification no longer covers the egress (per security-plan §Security Anti-Patterns → Logging, "`incident_events` lifecycle writes are a deliberate NO-SCRUB boundary").
- The corpus read path must preserve role separation: the sidecar's events read is a read; it must not route through a writer surface or open a write transaction (per security-plan §Threat Model Summary MCP stdio vector — the only corpus-mutating MCP tool is `mark_incident_resolved`).

## Patterns to follow
- The chunk #94 corpus-backed MCP tools (`query_incident_list` / `retrieve_report` / `retrieve_telemetry_slice` / `mark_incident_resolved`) are the precedent for a corpus read inside the sidecar: serde argument struct, name-dispatch, standard JSON-RPC error on a missing incident (per security-plan §Input Validation "MCP stdio inputs" row; §Threat Model Summary MCP stdio vector).
- Bounded-closed-label discipline as in the NO-SCRUB boundaries (`ui.ipc.rejection`, `ui.webgpu.adapter`, `app.exit`): a value outside the closed set is surfaced/coerced as a known sentinel rather than passed through verbatim (per security-plan §Security Anti-Patterns → Logging, the NO-SCRUB boundary entries).
- Cross-process corpus reads use the persisted OS-credential-store key (or the opt-in passphrase fallback) exactly as the existing sidecar corpus reads do — no new key source (per security-plan §Secret Management → Runtime, corpus encryption key).
- Error sanitization collapses internal error chains to a one-line message at the boundary while the full chain stays in the internal log, with only stack-local context such as the MCP request id (per security-plan §Error Handling "Internal logging").

## Anti-patterns to avoid
- NEVER log MCP tool response bodies — including the events list returned by the new read (per security-plan §Security Anti-Patterns → Logging).
- NEVER expose internal hostnames, IPs, file paths (e.g. the corpus.db path) or library versions in MCP error objects (per security-plan §Security Anti-Patterns → Logging; §Error Handling).
- NEVER spawn or expose the sidecar surface on a single gate (per security-plan §Security Anti-Patterns → Code Patterns).

## Contract bindings
- security ↔ arch: security-plan §Threat Model Summary (MCP stdio vector, "Tools: 8") and §Input Validation ("all 8 tools dispatched by name") enumerate the tool set; a ninth tool (or a widened `retrieve_report` response) makes both counts/lists drift and owes a wrap amendment alongside arch §Occupied Resources' MCP tool list.
- security ↔ obs: the read's tracing record is bounded to result type label + count (the `result_type_label` / `result_count_for` shape); obs-plan owns the record's schema, security owns the never-log-response-body ban.
- security ↔ tests: an unknown-id error and a decrypt-failure error need test carriers asserting the sanitized JSON-RPC error shape; the event-content fidelity is proven by the Conductor round (P-075 CARRY), which is tests/matrix territory.

## Acceptance criteria contributions
- (security) With the binary built without `--features mcp-server`, or with `ANDROMEDA_PULSE_MCP_ENABLED` unset, the events read is unreachable; the mismatch `warn` still fires (per security-plan §API Security "MCP feature double-gate").
- (security) An unknown or malformed incident id returns a JSON-RPC 2.0 error object whose `message` contains no file path, hostname, library version or Rust type name — same shape as `retrieve_report`'s missing-incident error (per security-plan §Error Handling "MCP server").
- (security) A tool-call log capture for the events read contains no response-body content (no event payload / list echoed), only bounded label + count fields (per security-plan §Logging & Monitoring "What NEVER to log").
- (security) The events response carries only `event_kind` from the closed `IncidentStatus` label set (out-of-set surfaced as a sentinel, never raw), `occurred_unix_nano`, and no raw ciphertext BLOB (per security-plan §Security Anti-Patterns → Logging "`incident_events` … NO-SCRUB boundary"; §Data Protection corpus encryption).
