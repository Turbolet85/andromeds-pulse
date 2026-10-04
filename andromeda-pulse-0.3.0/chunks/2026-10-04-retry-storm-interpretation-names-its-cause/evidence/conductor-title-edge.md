# Cross-project edge — the cause-label title prefix vs Conductor

Recorded at /andromeda-phase P5, 2026-10-04, after the plan's approval; for the wrap to record as a measurement.

- **Source:** the overseer's note on the P5 `yes` (approved under the founder's grant, his live ruling of
  2026-10-04), relayed into this session. The measurement was made on the Conductor side by the overseer, not by
  this session.
- **Measured (overseer):** no live Conductor code compares the incident title for equality.
  - `conductor/crates/conductor-run/tests/lifecycle_harvest.rs:77` is a static JSON fixture carrying
    `"title":"Deterministic verification incident"`. It is never compared against a live harvest.
  - `extract.rs` (Conductor) only READS the title field.
- **Consequence:** the `{Cause label}: {model title}` prefix this chunk writes at the producer breaks nothing on the
  Conductor side. A live deterministic harvest against this build reads, for example,
  `Retry storm: Deterministic verification incident`.
- **Owed at the wrap:** record this measurement, citing the overseer as its source. This replaces the plan's
  "relayed to the overseer at the wrap" cross-repo note, because the overseer has already verified it. Pulse edits
  nothing in Conductor.
