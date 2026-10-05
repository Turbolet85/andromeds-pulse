# Fan-out results — 2026-10-05-l4-runs-on-a-small-current-model-chosen-by-measurement

Resumed at P2 (operator, 2026-10-05: "Resume at P2"). 7 Explore doc-agents, one batch; 19 detectors
(arch 2 · security 4 · design 2 · layout 2 · tests 3 · obs 4 · a11y 2 = 19, the drift-base `doc:` total).
`registry.py contracts` exit 3 NOT MIGRATED for arch · tests · obs · a11y → `{contracts_line}` dropped.
Returns carried no HTML entities (no decode needed); stripping removed only `#` commentary below each list.

## Verdicts
- architecture — 6 proposals (stripped: D-arch-resources notes — no IPC/port/env/crate added; `CUDA_VISIBLE_DEVICES` not proposed, operator-command-only setter, serves the retired CPU route)
- security-plan — `proposals: []` (stripped: argv rows :139 / :461 state only the `-p` operand; no site enumerates the argv constant set; CPU-route claim 3 belongs to arch / entry B)
- design-system — `proposals: []` (stripped: no UI, no status claim touched)
- layout-templates — `proposals: []` (stripped: no surface; Halo DEFERRED claims untouched)
- test-plan — 1 proposal (no commentary)
- obs-plan — `proposals: []` (stripped: no hot path, logger, PII or §10 defect touched)
- a11y-plan — `proposals: []` (stripped: no interactive element, no schema change)

## architecture — parsed
1. D-arch-decisions · warning · §Established Decisions → [LLM Inference Runtime] · argv `-c 8192` / `-rea off`; shipped Llama-3.2-3B vs confirmed Qwen3.5-2B; b9305 json-schema grammar-prefill trap · basis arch:71
2. D-arch-decisions · warning · §Established Decisions → [LLM Inference Runtime] · CPU tier founder-retired status note, pending entry B · basis arch:71
3. D-arch-decisions · warning · §Stack → AI/ML serving row · same CPU status note · dependent-of D-arch-decisions (2) · basis arch:30
4. D-arch-decisions · warning · §Occupied Resources → env vars → `_LLAMA_CPU_BIN_PATH` · same note · dependent-of (2) · basis arch:230
5. D-arch-decisions · warning · §Inherited Defaults → LLM inference runtime · same note · dependent-of (2) · basis arch:367
6. D-arch-decisions · warning · §Established Decisions → [Fault Identity] · remainder owner re-homed from "the L4 model-replacement route entry" with the 37/40 confirmation · basis arch:72

## test-plan — parsed
1. D-tests-coverage · warning · §1 Pending coverage triggers → `l4-decision-probe-arg-parse-unit-coverage` · 16 → 29 pins; `--footprint` / `--gbnf` / `--sampling`, arms `nr` / `gb`, readings pinned; STILL OWED narrowed · basis test-plan:144

## Dispositions (Validate)
- arch 1 — APPLY. Check 1: playbook "Accurate this-chunk addition" (routine; constants + model facts are the report's Changes / Outcome) and expected amendment 1 (P5-approved). Text RE-DERIVED: the proposal's "the swap is entry A to Qwen3.5-2B" is superseded by the founder's 2026-10-05 ruling in this wrap's invocation (A ships the founder's pick from the pattern-discrimination entry P, not automatically Qwen3.5-2B) — the body names P as the chooser.
- arch 2 — APPLY-AS-MEASURED. Check 1: playbook "Drift proposal ACCURATELY correcting a doc claim that a MEASUREMENT disproved, where an impl half DOES exist but is too large" (routine; the amendment records the gap and names its owner, entry B). Check 6: disposes report disproved-claim 3. Authority: founder ruling 2026-10-05, relayed by the overseer (inputs#I6), restated in the operator's own words in this wrap's invocation ("B is programmatic L4 without a GPU").
- arch 3, 4, 5 — APPLY with their primary (dependent-of group, atomic).
- arch 6 — APPLY. Expected amendment 2 (P5-approved); playbook "Accurate this-chunk addition". Owner RE-DERIVED to the founder's three-entry ruling: P chooses, A ships.
- test-plan 1 — APPLY. Expected amendment 5a; playbook "Accurate this-chunk addition". Re-derived from the report's Counts / Symbols / Coverage bullets.
- Check 2 (cross-contradiction): none — arch 1/2 touch the same entry in complementary directions.
- Check 3 (intent-consistency): the report's deviations (probe extension on the operator's word; addenda; entry-18 red on the overseer's word) are justified and recorded; scope record none. No escalation.
- Check 4 (absence): arch 3–5 sites read by line (arch:30 661c, :230 226c, :367 627c); arch:71 2 339c read whole; arch:72 7 965c read by offset window @7756 (`model-replacement`).
- Check 5 (expected amendments):
  - 1 → arch 1; 2 → arch 6;
  - 3 `CUDA_VISIBLE_DEVICES` → NO AMENDMENT: set only inside operator-leg commands in `evidence/`, never by the product or a committed harness/xtask/script; registry over-reach (playbook "Registry over-reach") — the detector's decline accepted;
  - 4 security-plan argv row → NO AMENDMENT OWED: :139 and :461 state only the `-p` operand and its bound (unchanged per the report); neither enumerates the argv constant set, so no claim is stale;
  - 5a → test-plan 1; 5b test-plan §4 `unit_llamacli_inference` +2 → NO AMENDMENT OWED: §4 (:342–396) carries no `unit_llamacli_inference` row; the one file hit (:92, §1 security-vector row) enumerates the guard branches (9 path-guard + 6 prompt-bound + 4 allow-root), which the +2 argv pins leave true.
- Check 6 (disproved claims): 1 (scope.md) and 2 (plan.md G1) — chunk-artifact claims, recorded in the report, no amendment owed; 3 (arch CPU route) → arch 2–5.
- Boundary-widening screen: the two argv constants are first-party, fixed values; no new input class crosses the subprocess boundary (the `-p` bound is unchanged) → not the never-routine class.
- Escalations: 0.
