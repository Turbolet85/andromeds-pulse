# The operator's answer at P4 — the model seat of the console engine (2026-10-10)

Asked through the session's question dialog at P4 of chunk `2026-10-10-console-engine-entry-point`. The question,
its three options and the answer, verbatim.

## Question
At the point where 0.3.0 called the model, what does the console engine do? (Today an incident is created only from
the model's output; the only no-model path is the canned runner behind ANDROMEDA_PULSE_L4_DETERMINISTIC.)

## Options
1. **Follow the variable (Recommended)** — Console reads the same registered variable. Set: canned runner,
   incidents form (what the scenario-leg and baseline entries need, as the harness sets it today). Unset: cues and
   digests only, no incident, and one boot record says so. Keeps one meaning for the variable and never writes
   fixture text as a finding in an ungated run. Cost: an ungated console run reports no incident until `Incident is
   the engine's own record`.
2. **Canned runner always** — Console always uses the canned runner and reads no variable. Simplest, and every run
   forms incidents, so later entries need no gate. Cost: every incident from an ordinary console run carries the
   fixture narrative ("Synthetic incident emitted by the deterministic L4 mode") until Epoch 3 replaces it.
3. **No interpretation at all** — Console never creates an incident in this version until `Incident is the engine's
   own record` (Epoch 3). Cleanest reading of "no model". Cost: `Scenario legs` (line 37) and `Detection baseline`
   (line 45) need an incident through the console engine in Epoch 1, so the route would have to be reordered at a
   wrap.

## Answer
"Follow the variable (Recommended)"

## Notes given with the answer
Operator (pc overseer): one meaning for the variable, and fixture text never stands as a finding of an ordinary run. Name in the plan the entry that gives the console engine its own incident (Incident is the engine own record) as the owner of the unset arm, and pin both arms: set - an incident forms; unset - none forms and the boot record says why..
