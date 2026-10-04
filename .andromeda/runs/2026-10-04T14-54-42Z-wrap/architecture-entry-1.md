
## 2026-10-04-retry-storm-interpretation-names-its-cause — the incident title is cue-grounded in every L4 mode
**Section:** §Occupied Resources → `ANDROMEDA_PULSE_L4_DETERMINISTIC`
**Change:** The entry now states that the incident title is cue-grounded in EVERY mode, this one included.
- The producer prefixes the triggering cue kind's closed ASCII cause label (`triage::contract::cue_cause_label` — `Error-rate spike` · `Latency regression` · `Restart event` · `Service went silent` · `Retry storm` · `Reflection trend`) onto the model title BEFORE the scrub.
- So `Incident.title` and the `title` inside `resolution_summary_text` read `{Cause label}: {model title}` at creation, on the dedupe refresh (the JSON only; a deduped incident's own title is not rewritten) and in the resolution final write.
- Under this mode a storm incident reads `Retry storm: Deterministic verification incident` on the report header, Findings rows, MCP `query_incident_list` / `retrieve_report` and the digest's CORPUS MATCHES lines.
- Rows written before the chunk stay unprefixed (no migration).
- The canned rank-1 hypothesis names a retry storm for every incident, so only the title discriminates the cause in this mode.
**Why:** The model can drop the triggering cause (Conductor's d3 interpretation named no retry for a retry-storm-born incident), and the product had no deterministic field naming it. The founder ruled on 2026-10-04 (relayed by the overseer): ship the deterministic cause, in the title, at the producer, with existing rows left unprefixed. External harvests (Conductor) now observe the prefixed string; the overseer measured that no live Conductor code compares the title for equality.
**Ref:** .andromeda/runs/2026-10-04T14-54-42Z-wrap/
