
## 2026-10-02-incident-events-readable-through-mcp — the ninth MCP tool on the threat model and input-validation rows
**Section:** §Threat Model Summary → MCP stdio surface (Entry point) · §Input Validation → MCP stdio inputs row
**Change:** was "Tools: 8" / "all 8 tools dispatched by name"; now 9 — `retrieve_incident_events` (corpus-backed, read-only) joins the roster. Its input `{incident_id: integer}` deserializes through the serde `IncidentIdArgs` with `additionalProperties: false`; its corpus read is a prepared `?1`/`?2` statement bounded at `INCIDENT_EVENTS_READ_LIMIT` = 256 that never selects `payload`; it returns only a coerced closed event-kind label + a timestamp per event.
**Why:** a new crossing on the MCP stdio boundary — a Boundary widening, ratified by the founder at P4 (2026-10-02) for the read surface and by the founder's option A (2026-10-03) for the shared vocabulary it coerces against; recorded here as that ratification.
**Ref:** .andromeda/runs/2026-10-03T23-46-09Z-wrap/
