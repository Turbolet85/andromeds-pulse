# obs extract — phase-84

_Chunk #87 "Findings counter + dropdown" — terminal Epoch 9 (Foundation v0.2.0) chunk. Widget UI extension layer rendering an incident-derived findings counter + severity-colored dropdown over `pulse://stream/incidents` subscription + new `incidents.mark_all_read()` TauRPC procedure._

## Chunk relevance

**Chunk #87 IS instrumentable** (multiple new boundary points cross the self-observation surface):

1. **New TauRPC procedure `incidents.mark_all_read()`** — backend resolver in `pulse-app/src/incidents_router.rs` (extends existing chunk #78 `incidents.*` namespace; see obs-plan §4 IPC-internal row + §6 boundary-call wrappers TauRPC invocation row). Standard TauRPC handler instrumentation pattern applies: `#[tracing::instrument(skip_all, fields(traceparent = %tp, ...))]` per obs-plan §4 IPC-internal row.

2. **Frontend dropdown component `FindingsDropdown.tsx`** — consumer-side subscriber on `pulse://stream/incidents` (already-instrumented broadcast topic per chunk #78). No new backend span surface for the subscription itself (the emit side spans live in `crates/triage/src/incident/broadcast.rs` from chunk #78), but the dropdown opens / closes / row-click ARE state transitions that fit obs-plan §6 log-levels `info` boundary call summaries IF bridged through TauRPC (otherwise pure frontend state, out of self-observation scope per obs-plan §3 Frontend bridge).

3. **Counter derivation via corpus query on incident events** — per project-doc §86 "Counter derivated via corpus query on incident events and widget focus, no separate state file" — this is a NEW read-path against `corpus::IncidentReader` (chunk #78 substrate). The derivation query happens on widget focus event (frontend → TauRPC roundtrip — likely via an existing `incidents.list_active` call OR a NEW counter-specific query). If a NEW backend query method is added к `IncidentsApiImpl`, it gets the same TauRPC instrumentation as the mark_all_read procedure.

**Chunk #87 affects existing instrumentation** at:
- `pulse-app/src/observability.rs` AllowList — needs +1 entry для the new `incidents.mark_all_read.request` event field set (aggregate-only fields per cardinality discipline)
- `pulse-app/src/incidents_router.rs` — chunk #78 already-instrumented; mark_all_read becomes the 4th procedure in the resolver (`list_active` / `acknowledge` / `mark_resolved` / `mark_all_read`)

## Constraints

Per obs-plan §11 Obs Anti-Patterns:
- **NEVER** include incident message content / acknowledgement note text / resolution prose в span attributes (mirrors Vector 1 raw-OTLP-attribute discipline — incident records contain telemetry-derived prose that may carry user instrumentation strings) — emit `incident_count` + `incident_kind_tag` aggregates only.
- **NEVER** use unbounded label cardinality на `metric.*` event fields (e.g., a per-incident-id label would explode JSON log size; bounded enumerated values only) — see obs-plan §11 Metrics row 1.
- **NEVER** log raw `acknowledgement_note` / `resolution_note` strings (these are free-form text fields per chunk #78 `incidents.acknowledge` / `incidents.mark_resolved` procedures — pattern continues для `mark_all_read` even though that procedure takes no acknowledgement note argument); aggregate-only `note_length_bytes` if needed.
- **NEVER** log в hot path в `info` level (obs-plan §11 Logs row 4) — counter derivation runs on widget focus + on every incident broadcast event; use `debug` for per-event detail, `info` only for boundary call summaries (mark_all_read invocation = single-event, OK at info).
- **NEVER** skip `trace_id` / `span_id` fields in logs (obs-plan §11 Logs row 2) — IPC envelope `traceparent` field extracted per chunk #78 precedent, attached к span via `fields(traceparent = %tp)`.
- **NEVER** include raw incident message content в the JSON log file — counter derivation operates on incident COUNTS, not contents; obs allowlist for `incidents.*` namespace must constrain to id/severity/kind/count/timestamp aggregates (mirrors chunk #78 instrumentation precedent).

## Patterns к follow

Per obs-plan §3 + §4 + §6 + chunk #78 / #80 / #82 / #86 precedent (from Context):

1. **TauRPC handler instrumentation** (obs-plan §4 IPC-internal row): `#[tracing::instrument(skip_all, fields(traceparent = %tp, ...))]` on each `incidents.mark_all_read` resolver method. Boundary log fields per obs-plan §6 TauRPC invocation row: `method_name = "incidents.mark_all_read"`, `argument_digest` (hash if any non-aggregate args; mark_all_read appears к take no args per project-doc §86), `result_type = "mark_all_read_ack"`, `latency_ms`.

2. **`incidents.{verb}.request` span event** convention — chunk #78 precedent already emits `incident.created` / `incident.acknowledged` / `incident.resolved`; chunk #87 extends с `incident.mark_all_read.request` IPC span + `incident.bulk_acknowledged` lifecycle event when the corpus persist completes (aggregate `affected_count` field — NOT а list of acknowledged ids; cardinality discipline).

3. **`incident.broadcast.{event_kind}` precedent** — chunk #78 already emits `incident.broadcast.{event_kind}` events on `pulse://stream/incidents` per Context note; chunk #87 adds а NEW event_kind variant `bulk_acknowledged` (singular event covering all-incidents-acked, NOT per-incident emit х N — cardinality discipline).

4. **Aggregate-only fields**: derive counter values, emit at the API surface as `unread_count` (integer) + `severity_max` (enum: `critical|warning|info|none`). NEVER emit а list of unread incident ids в the counter event; the counter value is the entire aggregate.

5. **Corpus query trace context inheritance** — counter derivation query against `corpus::IncidentReader::count_unread()` (or equivalent) inherits trace_id from the encompassing `incidents.list_active.request` / `incidents.mark_all_read.request` span via `tracing::Span::current()` propagation (obs-plan §3 Trace context propagation > Internal async boundaries: `.in_current_span()` extension trait).

6. **AllowList entry pattern (chunk #82 / chunk #86 precedent)** — `pulse-app/src/observability.rs::AllowList` discipline: add entry для `incidents.mark_all_read.request` with allowlisted fields `[method_name, traceparent, affected_count, latency_ms, result_type]`; mirrors chunk #86's 7 AllowList entries для L4 degraded-mode + `diagnostics.retry_interpretation` precedent.

7. **Heartbeat / health unaffected** — chunk #87 adds NO new long-running task / subscriber loop / async tick (counter derivation is on-demand at widget focus; subscription к `pulse://stream/incidents` is consumer-side from the existing chunk #78 emit path). No `findings.tick` / `findings.heartbeat` event needed.

## Anti-patterns к avoid

Per obs-plan §11:
- **NEVER skip the `incident.broadcast.bulk_acknowledged` event** for the mark-all-read action (and corresponding `incidents.mark_all_read.completed` log line) — without it the agent loses ability to reconstruct "when did the user clear N findings?" from log paste-to-AI. Single aggregate event с `affected_count` is sufficient (cardinality-safe).
- **NEVER emit а per-incident span on the bulk-mark-read path** (would multiply span count by N per click; cardinality discipline violation — emit one parent span + one aggregate completion event с `affected_count`).
- **NEVER include incident.message_content / corpus row payload в span fields** (Vector 1 family — incident records derived от telemetry whose attribute values are user-controlled content; aggregate-only — `incident_id` is bounded ulid-like opaque token, OK; full prose body is NOT OK).
- **NEVER log on per-render / per-focus events at info level** (counter re-derivation on every widget focus would spam the log file) — use `debug` for per-focus/per-render trace, `info` only on the bulk-mark-read action (single-event boundary).
- **NEVER instrument the subscription poll loop inside `pulse://stream/incidents` consumer at `info` level** — emit subscription-lifecycle events (`subscribe` / `unsubscribe`) at `info` once each; per-event re-render at `debug` only.
- **NEVER conflate the corpus-query-on-widget-focus path with the live-stream-broadcast path** в same span — the two are independent triggers (focus event = pull; broadcast event = push); each gets its own boundary instrumentation.

## Contract bindings

- **Tests harness contract binding (obs-plan §3 Log format JSON schema):** the new `incidents.mark_all_read.request` event MUST conform к the verbatim JSON schema (timestamp / level / target / message / fields) so tests can assert the boundary log line via `jq` post-test (tests-plan §3 / §5 binding contract). The bulk-acknowledged broadcast event ALSO must conform.
- **Security PII binding (obs-plan §8 PII Scrubbing + security.md vector 1):** incident records carry telemetry-derived prose (exception messages / log template snippets) which incidentally contain instrumented-app secrets — the obs-plan §8 default-deny posture for `mcp-server` / `viz` modules extends к `incidents.*`: bounded ids + counts + timestamps + enumerated severity/kind labels OK; any free-form prose field NOT OK in log output.
- **A11y violation JSON schema binding (NOT relevant к chunk #87)** — this chunk does not emit a11y violations; binding flagged as not-applicable per obs-plan §3.
- **Heartbeat tick binding (obs-plan §3 Heartbeat ticks)** — chunk #87 does NOT introduce а new heartbeat-required subsystem (no long-running task spawned); existing heartbeats unaffected.
- **Tests harness AllowList enforcement** — chunk #82 / chunk #86 precedent established that `pulse-app/src/observability.rs::AllowList` is the canonical gate for per-module event field admission; chunk #87 extends AllowList с +1 entry for `incidents.mark_all_read.request` (aggregate fields only) mirroring chunk #86 +7 entries pattern.

## Acceptance criteria contributions

1. **(obs) New TauRPC procedure `incidents.mark_all_read` emits boundary log line** in NDJSON format с required fields per obs-plan §6 (level=`info`, target=`incidents::mark_all_read`, message human-readable summary, fields={method_name, traceparent, affected_count, latency_ms, result_type}, plus default subscriber fields service.name / service.version / deployment.environment); verifiable via `jq '. | select(.target == "incidents::mark_all_read")' ~/.andromeda-pulse/logs/agent-latest.jsonl` returning а matching line after test invocation.

2. **(obs) PII scrubbing: no raw incident message content / acknowledgement-note / resolution-prose strings appear в JSON log file** during chunk #87 surfaces (counter derivation + dropdown open + row click + mark-all-read bulk action) — verified via negative test grepping for а seeded canary substring planted в an incident's message field, asserting zero matches в the post-test log artifact (mirrors obs-plan §8 default-deny posture + chunk #78 incident lifecycle scrubbing discipline).

3. **(obs) Span instrumentation: `incidents.mark_all_read.request` span propagates traceparent from IPC envelope** + emits child span `corpus.bulk_mark_unread_persist` (or equivalent) inheriting the same trace_id; verifiable post-test via `jq '. | select(.fields.traceparent != null) | .fields.traceparent' agent-latest.jsonl | sort -u | wc -l` showing trace_id continuity across the boundary call + corpus persist child.

4. **(obs) Metric cardinality discipline: bulk-acknowledged broadcast event emits one aggregate `incident.broadcast.bulk_acknowledged` event с `affected_count` integer field**, NOT per-incident span events (would multiply by N). Verifiable post-test via `jq '. | select(.target == "incident::broadcast" and .fields.event_kind == "bulk_acknowledged")' agent-latest.jsonl | wc -l` returning 1 per mark-all-read invocation regardless of N affected incidents.
