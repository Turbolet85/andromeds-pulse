
## 2026-10-04-retry-storm-interpretation-names-its-cause — Fault Identity: the cue kind reaches the incident text, never its identity
**Section:** §Established Decisions → [Fault Identity — what makes two faults ONE fault]
**Change:**
- Added: the cue KIND, not only its fingerprint, now reaches the incident TEXT. The producer writes `Incident.title`, and the `title` inside the persisted L4 JSON, as `{cue_cause_label(kind)}: {model title}`, a closed ASCII label grounded before the scrub (a reflection incident's synthetic `ReflectionTrend` included).
- Identity is unchanged: the tuple `(kind, scope, scope_id)`, the coalesce predicate, severity, priority tier and the evidence union all stand.
- A deduped incident keeps its creation title, which is coherent because `kind` is part of the identity key.
- The model-authored symptom, timeline and ranked hypotheses are untouched.
**Why:** An incident must name its trigger on every surface whatever the model wrote (founder ruling 2026-10-04, relayed by the overseer). Text and identity are kept separate so that the title never becomes a dedupe key.
**Kept:** Fingerprint stays out of the identity key (possible-but-declined). The model layer's remedy (a prompt that names the triggering cue and frames corpus matches as past/other incidents) is its own route entry, not this decision.
**Ref:** .andromeda/runs/2026-10-04T14-54-42Z-wrap/
