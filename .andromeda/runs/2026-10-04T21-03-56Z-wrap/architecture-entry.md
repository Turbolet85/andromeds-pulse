
## 2026-10-04-l4-interpretation-names-its-triggering-cue — Fault Identity: the model layer is framed at prompt composition
**Section:** §Established Decisions → [Fault Identity — what makes two faults ONE fault]
**Change:** The clause "the model-authored symptom, timeline and ranked hypotheses are untouched (a prompt-side remedy for the model layer is its own route entry)" was retired, as was that remedy's deferral in the `Kept` of "2026-10-04-retry-storm-interpretation-names-its-cause — Fault Identity: the cue kind reaches the incident text, never its identity". The body now says:
- The producer still leaves those fields as the model wrote them.
- The model layer is framed at prompt composition. `triage::digest::render_payload` renders a `TRIGGER: {cue_cause_label(kind)}` line, keyed on `cues.first()` (the cue the identity is taken from), directly after `OVERALL:`, and none without a cue.
- When corpus matches exist, a static note under the unchanged `CORPUS MATCHES:` header frames them as other or past incidents, context only.
- `interpretation::prompt::TRIGGER_FRAMING_INSTRUCTION` (ASCII) sits after `CITING_INSTRUCTION` in all three tiers' Output Instructions. It says the title, symptom and first hypothesis describe the TRIGGER signal, never a corpus match.
- Prompt lineage is v2.4 / v1.3-fallback / v1.3-reflection.
- This is framing, not identity: the tuple, the coalesce predicate and the title grounding are unchanged.
- Its effect on the rank-1 hypothesis is UNMEASURED. The pre-registered series (`l4_decision_probe --min-rank1 36`: rank-1 names the retry in ≥ 36/40, `nf` recorded) is gated on a Linux llama.cpp CUDA binary.
**Why:** This chunk is the route entry the clause deferred to. The framing lives in shared composition, never in a runner impl. Applied on the plan's recorded expected amendment, with the scope ruled by the overseer (founder-delegated, 2026-10-04): the TRIGGER line carries the kind label only, never `scope_id`.
**Kept:** No `triggering_cue` builder parameter (~50 call sites). The dead `# Corpus Retrieval` prompt sites are untouched, because production passes `""` there.
**Ref:** .andromeda/runs/2026-10-04T21-03-56Z-wrap/
