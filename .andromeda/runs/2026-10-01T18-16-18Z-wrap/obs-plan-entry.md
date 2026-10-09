
## 2026-10-01-real-model-incident-surfacing — the no-incident L4 outcome gets its own exact leaf
**Section:** §8 PII Scrubbing & Compliance → default-deny allowlist — `interpretation.incident.skipped` (new exact leaf)
**Change:**
- Registered `interpretation.incident.skipped` with four bounded labels, ALL the emit site emits:
  - `skip_reason` ∈ `model_resolution_summary` | `decision_dismiss` | `severity_none` | `no_cue` (the first gate in code order);
  - `decision`;
  - `severity`;
  - `digest_kind`.
- INFO, exactly once per cleanly-parsed L4 generation that creates no incident, from three sites:
  - the model-set `is_resolution_summary` routing seam;
  - the creation predicate;
  - its no-cue return.
- Never scope_id, title, symptom, digest, prompt or model text.
- Its own exact leaf under the no-bare-`interpretation` invariant, guarded where the pins run.
**Why:** a parse-`ok` generation that created nothing left no record, so a dismissal, a `severity: none`, a model-set resolution flag and a cue-less digest read the same, and "the model dismissed" was being inferred from silence. Measured at this chunk, 1 of 30 such generations was a dismissal and the rest were `severity: none`. Registered routine — a new target whose fields are bounded non-PII, with a guard that runs.
**Ref:** .andromeda/runs/2026-10-01T18-16-18Z-wrap/
