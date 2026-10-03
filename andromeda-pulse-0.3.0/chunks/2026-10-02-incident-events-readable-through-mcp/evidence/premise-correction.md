# Premise correction — "creation records no event" (2026-10-03)

The phase premise that an incident's CREATION records no `incident_events` row was false at the product level, and
assertion 7 of the first round request failed on S = `4a26ad8` because of it (relayed by the overseer, 2026-10-03).
This file is the record the wrap reads: `plan.md`, `research.md` and `scope.md` are immutable to /implement, so their
lines stay as written and are corrected here, never in place.

## What is true at HEAD (measured 2026-10-03)
- `incident_events` has TWO production writers. (1) `CorpusWriter::update_incident_status`
  (`crates/corpus/src/contract.rs`) — one row per status VALUE change, inside the guarded transaction, `event_kind` = the
  new status label. (2) The L4 incident producer — `create_incident_from_l4_output`
  (`pulse-app/src/inference_runtime.rs`) calls `IncidentPersistence::save_incident_event(id, "created", now)` right after
  `save_new_incident`, through `CorpusIncidentPersistence` (`pulse-app/src/incident_persistence.rs`) into
  `CorpusWriter::save_incident_event` — since chunk #92 (`001a768`, 2026-05-31), pinned by
  `pulse-app/tests/unit_incident_producer.rs` (`vec!["created"]`).
- The producer is the ONLY production path that opens an incident (`grep -rn 'save_new_incident(\|create_incident_from_l4_output('`
  over `crates pulse-app/src`: one caller each). So every production incident's ledger begins with `created`.
- What research measured was true of the CORPUS layer only: `Corpus::save_incident` inserts the incident row alone, and
  the corpus pin `status_value_change_records_one_lifecycle_event_per_transition` asserts no event after it. The writer
  one layer up (the `IncidentPersistence` trait) was never mapped — a narrow-basis claim.

## The vocabulary, now (founder's word 2026-10-03, relayed by the overseer — option A)
One closed set in `triage::contract`: `INCIDENT_EVENT_CREATED = "created"` plus the three `IncidentStatus` labels,
returned by `incident_event_kinds()`. The producer writes `INCIDENT_EVENT_CREATED`; the sidecar's `coerce_event_kind`
passes exactly that set and coerces anything else to `unknown`.

## Plan lines superseded (cited by line at S; not edited)
| plan.md line | as written | corrected reading |
|---|---|---|
| 15-16 (Goal) | "status transitions only, no event at creation" | the ledger as it is: the producer's `created` event, then one row per status value change |
| 64-65 (Step 4) | coercion passes a value equal to `incident_status_label` of the three statuses | coercion passes `triage::contract::incident_event_kinds()` — `created` plus the three status labels |
| 76 (Step 6) | manifest description "creation records none" | the description names `created` as the first event, written when the interpretation path opens the incident |
| 303 (acceptance, corpus) | "one incident's status transitions" | holds for the corpus read as written: it returns every row of both writers, oldest first |
| 363 (Forks) | "creation event = none, ledger as decided" | the ledger was never "none at creation" at the product level; no opening event was ADDED by this chunk — the existing `created` row is now read under its own kind |

## For the wrap's amendments (plan §Implementation notes → Expected amendments)
Any amendment text describing the `incident_events` read must state the four-kind vocabulary and that `created` is the
first event of every incident the interpretation path opens — never "creation records no event". The arch §Occupied
Resources entry gains the producer as the table's SECOND production writer as well as the sidecar as its first reader.
The `IncidentEventRow` claim that the payload column is empty by construction still holds: both writers pass an empty
payload.
