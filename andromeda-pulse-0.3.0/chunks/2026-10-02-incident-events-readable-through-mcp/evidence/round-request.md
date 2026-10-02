# P-075 round request — 2026-10-02-incident-events-readable-through-mcp

Written by /implement (plan Step 10) for the relay to conductor-builder. Pulse names the binary and the assertion set;
the round, its scenarios and its evidence are Conductor's (read-only from here; cited, never copied — CLAUDE.md
2026-08-21). This is a FRESH round: P-075 is re-verified on this chunk's binary (founder ruling 2026-10-02, relayed by
the pc overseer), with content fidelity extended to the incident events read back through MCP.

## The binary under test
- Built from **S** = this chunk's operator pre-CI commit
  (`chore(2026-10-02-incident-events-readable-through-mcp): operator pre-CI commit, for the run this chunk's verdict reads`),
  recorded with its build command in `evidence/round-binary.md` once the operator pass prints it (`git rev-parse HEAD`).
  S does not exist at the time of writing.
- Build: `cargo build --workspace --release --features mcp-server` — the MCP sidecar exists only under that feature
  (arch §Cross-cutting Patterns → Feature-gate hygiene).
- Artifacts: `target/release/pulse-app[.exe]` and the sidecar `target/release/andromeda-pulse-mcp[.exe]` (arch §Occupied
  Resources → Process / service identity).
- As at the prior round, a Windows teardown by `Stop-Process -Force` leaves no `app.exit` record — expected, not a
  finding.

## What is new at S
- The sidecar's `tools/list` carries **nine** tools: the eight of the prior round plus `retrieve_incident_events`.
  Conductor's readiness gate pins required tools against its contract manifest (Conductor `.andromeda/architecture.md:93`),
  so the new name joins that manifest for this round.
- `retrieve_incident_events` — input `{incident_id: integer}` (required; no other property accepted). Response:
  `{"incident_id", "events": [{"event_kind", "occurred_unix_nano"}], "total", "truncated"}`. `events` are the incident's
  status transitions, oldest first, at most 256 (`truncated` true when more exist). `event_kind` is one of `active` /
  `acknowledged` / `resolved`; any other stored value reads `unknown`. Creation records NO event, so an incident read
  before its first status change returns `events: []`. An unknown id returns a JSON-RPC error, code -32603, message
  containing `incident not found`.

## Mode
- ONE round, deterministic: `ANDROMEDA_PULSE_L4_DETERMINISTIC` set on the app child (the arch-registered gate), a fresh
  data dir. Ports 4317/4318 are shared with this tree's work: the round runs in the operator's slot.

## The seven graded assertions
The first six are the concretized P-075 acceptance (`verification-matrix.json#P-075`), unchanged from the prior round.

1. **Read-back content fidelity** — the incident read back through MCP `retrieve_telemetry_slice` carries, in
   `fingerprint_refs`, the full 32-hex fingerprint Conductor derives for the storm it emitted; the incident opened after
   the storm's emission instant; `retrieve_report` renders it with `degraded_mode: false`.
2. **Runtime-state fidelity** — `mark_incident_resolved` applies, and the incident leaves `query_incident_list`'s active
   set.
3. **P-025** — the worst `metric.constellation.hue_update_ms.duration_ms` in the leg window <= 2000, graded per Conductor's
   `contracts/pulse-p025-measurement-contract.md` (the constellation dot hue; never the deferred Halo glow).
4. **P-027** — `metric.constellation.discovery_ms.duration_ms` (first-sighting anchored) <= 5000.
5. **P-037** — `metric.report.render_ms.value` <= 2000 (the Report window).
6. **P-045** — the worst `metric.findings.counter_refresh_ms.duration_ms` <= 1000 (the Findings counter).
7. **Incident events read-back** (new) — for the round's incident:
   - read through `retrieve_incident_events` BEFORE `mark_incident_resolved`: the response carries NO `resolved` event;
   - read again AFTER it: the LAST event is `resolved`, and its `occurred_unix_nano` lies inside the resolve call's
     wall-clock window (request sent → response received);
   - every `event_kind` in both reads is one of `active` / `acknowledged` / `resolved`.
   The assertion is over the LAST event and the absence of `resolved` before the call, never an exact sequence: whether
   the app adds earlier transitions (an acknowledgement, an auto-resolve) is the round's to observe, not Pulse's to fix.

Read-back tools by name: `query_incident_list`, `retrieve_telemetry_slice`, `retrieve_report`, `mark_incident_resolved`,
`retrieve_incident_events`.

## Grading posture
- Hard Pass / Fail at the MEASURED value. An absent sample is UNGRADED and never counts as met.
- A FAIL is a Pulse finding: no re-drive for a pass (Conductor v3-08 posture). Pulse surfaces it as a wrap escalation.
- Pulse records the round in `evidence/round-result.md` (plan Step 11): Conductor's commit, CI run, the graded test ids and
  the evidence path for all seven, each verdict read from Conductor's files.
