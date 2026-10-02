# Intent — smoke-check-boot-discipline

## User intent (verbatim)

**Phase 1b (sanity check, single sentence):**

> there is no reactor running, must be called from the context of a Tokio 1.x runtime

(The user pasted the panic message from the chunk #27/#30 latent bug at `crates/ui-bridge/src/health.rs:291`, surfaced by chunk #31's Phase 2b smoke check.)

**Phase 1c (clarifying dialogue, 1 follow-up):**

Skill offered three interpretations (A: fix code → Refuse 5; B: discipline that prevents class of failure → Type 3; C: known-issue tracking → Type 3). User chose **(B)**:

> Document the panic + a discipline that prevents this class of failure going forward — e.g., add a Decisions Log entry to test-plan.md §3 saying "chunks modifying boot/setup paths MUST include `npx @tauri-apps/cli dev` in their Test Commands so latent boot panics surface at the introducing chunk" — this would be a Type 3 documented gap addition I can handle.

## Final slug

`smoke-check-boot-discipline`

## Final framing

Add a Decisions Log entry to **`.andromeda/test-plan.md`** §3 Test Harness Contract documenting:
1. The gap (the 5-command harness + test command list does not currently require runtime smoke for boot-path-touching chunks)
2. The empirical surfacing trigger (chunk #27/#30 latent panic caught only by chunk #31's Phase 2b smoke gate)
3. The discipline going forward (boot-path-touching chunks MUST include `npx @tauri-apps/cli dev` in Test Commands)
4. The recommended cleanup (formalize as coverage trigger in next `/andromeda-tests` re-run; apply in `/andromeda-phase` plan authoring for chunks #32+)

Pure spec amendment — no implementation code touched. The panic itself remains for separate resolution (out of scope per Refuse 5).
