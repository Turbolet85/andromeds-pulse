# P-075 round request — 2026-10-01-conductor-return

Written by /implement (plan Step 10) for the relay to conductor-builder. Pulse names the binary and the assertion set;
the round, its scenarios and its evidence are Conductor's (read-only from here; cited, never copied — CLAUDE.md
2026-08-21).

## The binary under test
- Built from **S** = this chunk's operator pre-CI commit (`chore(2026-10-01-conductor-return): operator pre-CI commit`),
  recorded with its build command in `evidence/round-binary.md` once the operator pass prints it (`git rev-parse HEAD`).
  S does not exist at the time of writing.
- Build: `cargo build --workspace --release --features mcp-server` — the MCP sidecar exists only under that feature
  (arch §Cross-cutting Patterns → Feature-gate hygiene).
- Artifacts: `target/release/pulse-app[.exe]` and the sidecar `target/release/andromeda-pulse-mcp[.exe]` (arch §Occupied
  Resources → Process / service identity).
- Since S, every `pulse-app` process end writes one `app.exit` record (`exit_class` / `exit_code` / `exit_code_known` /
  `signal`) except the classes unloggable by construction (SIGKILL, Windows `Stop-Process -Force`, a Rust
  `process::exit` on Windows). A Windows teardown by `Stop-Process -Force` therefore leaves no `app.exit` — expected, not a
  finding.

## Mode
- ONE round, deterministic: `ANDROMEDA_PULSE_L4_DETERMINISTIC` set on the app child (the arch-registered gate), a fresh
  data dir. Ports 4317/4318 are shared with this tree's work: the round runs in the operator's slot.

## The six graded assertions (the concretized P-075 acceptance, `verification-matrix.json#P-075`)
1. **Read-back content fidelity** — the incident read back through MCP `retrieve_telemetry_slice` carries, in
   `fingerprint_refs`, the full 32-hex fingerprint Conductor derives for the storm it emitted; the incident opened after
   the storm's emission instant; `retrieve_report` renders it with `degraded_mode: false`.
2. **Runtime-state fidelity** — `mark_incident_resolved` applies, and the incident leaves `query_incident_list`'s active
   set.
3. **P-025** — the worst `metric.constellation.hue_update_ms.duration_ms` in the leg window <= 2000, graded per Conductor's
   `contracts/pulse-p025-measurement-contract.md` (the constellation dot hue; never the deferred Halo glow).
4. **P-027** — `metric.constellation.discovery_ms.duration_ms` (first-sighting anchored since Pulse
   `2026-09-30-p-027-discovery-bound`) <= 5000.
5. **P-037** — `metric.report.render_ms.value` <= 2000 (the Report window).
6. **P-045** — the worst `metric.findings.counter_refresh_ms.duration_ms` <= 1000 (the Findings counter).

Read-back tools by name: `query_incident_list`, `retrieve_telemetry_slice`, `retrieve_report`, `mark_incident_resolved`.

## Grading posture
- Hard Pass / Fail at the MEASURED value. An absent sample is UNGRADED and never counts as met.
- A FAIL is a Pulse finding: no re-drive for a pass (Conductor v3-08 posture). Pulse surfaces it as a wrap escalation.
- Pulse's `ref` (plan Step 11) cites Conductor's graded test ids and the evidence path at Conductor's commit, written only
  when all six read PASS there.

## Named gap (for the founder, not for the round)
`incident_events` (the lifecycle table, the one corpus table whose content is not L4-authored) reaches no MCP tool, so
content fidelity in this round is scoped to the payload-varying fingerprint plus runtime state. Open question for the
founder: build a read surface in 0.3.0, route it to 0.4.0, or accept the narrowed claim as final.
