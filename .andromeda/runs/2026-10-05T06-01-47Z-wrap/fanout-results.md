# Fan-out results — 2026-10-04-l4-rank-1-hypothesis-names-the-retry-on-every-storm-shape

Seven Explore doc-agents, one parallel batch, prompts `prompt-{doc}.md` sent verbatim (19 detectors; sum of `doc:`
names = 19). No keyed-contracts line: `registry.py contracts` exit 3 `NOT MIGRATED` for architecture, test-plan,
obs-plan and a11y-plan. Every return was YAML with a trailing `#` commentary block. Stripping removed only that
commentary, so each verdict line carries its substance. No `&lt;`/`&gt;`/`&amp;` entities were present (entities=0),
so no raw twin is warranted.

## Verdicts

- **architecture.md — 2 proposals.** Stripped commentary: D-arch-resources no drift (no new resource; the probe flag
  extends an existing dev-only example, outside the formalized-CLI registry). Sweep: retired-claim hits only at :72;
  :237 (canned deterministic rank-1) is unchanged canned-runner behaviour.
- **security-plan.md — 2 proposals.** Stripped commentary: D-security-auth, -deps, -logging no drift; new surfaces
  validated (static text under `validate_prompt_bounded` + ASCII pin; probe `--shapes` refuses unknown ids). Sweep
  `7,185|7185|2.28|9,199|headroom|synthetic S4`: exactly :139 and :461.
- **design-system.md — `proposals: []`.** No UI; no status claim touched (Halo deferral sites unchanged).
- **layout-templates.md — `proposals: []`.** No surface; no L4 status claim in the doc.
- **test-plan.md — 2 proposals.** Stripped commentary: D-tests-framework and D-tests-obs-harness no drift. Sweep:
  retired-claim hits only :144 and :385.
- **obs-plan.md — `proposals: []`.** No emit site; no `prompt_version` literal in the doc; §10 lists no L4 defect.
- **a11y-plan.md — `proposals: []`.** No interactive element; no schema change.

## Proposals (parsed) and dispositions

1. **architecture · D-arch-decisions · warning** · §Established Decisions → [Fault Identity] — the framing clause:
   the reworded `TRIGGER_FRAMING_INSTRUCTION` (adds the first-hypothesis-names-the-signal and
   other-metric-is-cause-or-effect obligations) and the lineage parenthetical v2.5 / v1.4-fallback / v1.4-reflection.
   basis architecture.md:72.
   → **APPLY** (check 1: playbook "Accurate this-chunk addition" routine — the named symbols are in the report's
   Changes; check 5: the plan's expected amendment names "replace the prompt lineage … and name the shipped lever").
2. **architecture · D-arch-decisions · warning** · same section — replace the measured-effect run (30/40, nf 18/40,
   "shortfall lies in S2 and S3", owner "its own 0.3.0 route entry") with this chunk's two-slot result and the new
   owner. basis architecture.md:72.
   → **APPLY** (check 1 routine, as above; check 5: the plan's expected amendment names "replace the measured clause
   (FAIL 30/40) with this chunk's result in the same boundary form"). The applied text is re-derived from the report
   and keeps the predecessor's v2.4 30/40 as history in its boundary form.
3. **security-plan · D-security-input · escalate** · §Input Validation → L4 inference argv prompt row — observed
   maximum 7,185 B (v2.4) → 7,405 B (v2.5 synthetic S4); ratio ~2.28× → ~2.21×; headroom 9,199 → 8,979 B; primary
   builder only, fallback and reflection unmeasured. basis security-plan.md:139.
   → **APPLY** (check 1: playbook "ESCALATE-severity detector … OUTSIDE the class" — routine-apply-by-actual-class.
   (a) the report affirms no unvalidated boundary: the bound and `validate_prompt_bounded` are unchanged and the ASCII
   pin is green. (b) the actual class is a measured-figure re-base of this chunk's own measurement, governed by
   "Accurate this-chunk addition". Check 5: the plan's expected amendment names it, and its condition (> 7,185 B) is
   met per the report's Counts bullet.)
4. **security-plan · D-security-input · escalate · dependent-of D-security-input** · §Security Anti-Patterns → Code
   Patterns L4 `-p` bullet — the lockstep mirror of #3. basis security-plan.md:461.
   → **APPLY** with #3 (the dependent-of group applies atomically).
5. **test-plan · D-tests-coverage · warning** · §1 `l4-decision-probe-arg-parse-unit-coverage` — 8 → 16 collected
   pins, the 8 new pins named, the existing flags' parse still owed. basis test-plan.md:144.
   → **APPLY** (check 1 routine "Accurate this-chunk addition"; check 5 expected amendment). The proposal's
   still-owed list names "INCONCLUSIVE exit 2 on an unset / guard-rejected L4 path, A2/A4 and `first_keys`" — not
   carried by the report; applied only as far as the body already states them (the re-derivation tell guards the rest).
6. **test-plan · D-tests-coverage · warning** · §4 interpretation crate — lineage v2.5 / v1.4-*, the reworded
   instruction, the new obligation pin, `-p interpretation` 127 → 128. basis test-plan.md:385.
   → **APPLY** (check 1 routine; check 5 expected amendment).

## Validate — the six checks

- **1 Playbook:** all six routine (above). No two rules collide; no boundary widening (no channel gains a crossing:
  static first-party template text under an unchanged bound).
- **2 Cross-contradiction:** none — #1/#2 edit different spans of one line; #3/#4 are one lockstep group; #5/#6
  different rows.
- **3 Intent-consistency:** the report's FAIL 34/40 diverges from the entry's "names the retry on every storm shape" —
  JUSTIFIED: the entry says "measured against the pre-registered bar", and the P4 ruling (carried in scope item 3)
  makes a FAIL a completed record, never a pass. Scope record: none (`gate.py scope` clean, 0 recorded).
- **4 Absence needs evidence:** the agents' "only these sites" claims are re-derived by
  `sites.py` (this wrap, P1): `30/40` arch 1 (:72); `v2\.4\b` arch 1 · security 2 · test-plan 1;
  `v1\.3-(fallback|reflection)` arch 1 · test-plan 1; `7,185|7185` security 2; the coverage-row id test-plan 1. The
  long lines (arch :72 6,848 chars; test-plan :385 3,405; security :139 / :461) are read by offset window at apply
  time, never from a grep view.
- **5 Expected amendments:** all four plan entries proposed (#1+#2 arch; #6 test-plan §4; #5 test-plan §1; #3+#4
  security pair). None raised by the orchestrator.
- **6 Disproved claims:** the report's two chunk-level readings (stem gap = 1/40; selection carry-over 37 → 34) are
  DISPOSED as recorded in the report, no amendment owed (chunk-artifact readings; the stem finding also enters the
  arch clause via #2's stem readings).

Escalations: 0.
