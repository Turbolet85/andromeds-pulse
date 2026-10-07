# Fan-out results — wrap of 2026-10-07-l4-probe-reproduces-the-canary-history-miss

Seven doc-agents, one batch, 19 detectors (arch 2 · security-plan 4 · design-system 2 · layout-templates 2 ·
test-plan 3 · obs-plan 4 · a11y-plan 2; the drift-base carries 19 `doc:` names). Each return was stripped of
commentary and probed for HTML entities (0 in every return). Validated 2026-10-07 by the orchestrator.

## Verdict lines

- architecture: 3 proposals (1 primary, 2 dependents), all D-arch-decisions, all on architecture.md:73. Stripped: a
  preamble reading D-arch-resources as no drift (no resource added; the `L4_REPLAY_*` names are gate-entry shell
  variables; the four keyed contracts read and untouched).
- security-plan: 4 proposals (2 primaries, 2 dependents), all D-security-input at severity warning. Stripped: a
  preamble reading D-security-auth, D-security-deps and D-security-logging as no drift, and stating that
  D-security-input's escalate condition (an unvalidated boundary) did not fire.
- design-system: 0 proposals. Stripped commentary kept in `.raw-fanout-design-system.md`.
- layout-templates: 0 proposals. Stripped commentary kept in `.raw-fanout-layout-templates.md`.
- test-plan: 3 proposals (1 primary, 1 dependent, 1 primary), all D-tests-coverage. Stripped: a trailer reading
  D-tests-framework and D-tests-obs-harness as no drift.
- obs-plan: 0 proposals. Stripped commentary kept in `.raw-fanout-obs-plan.md`; it holds two observations that are
  not proposals: (1) the attempt-1 boot-smoke death left no `app.exit` record and the report does not classify that
  end (routed by the report to the route entry "The Linux boot smoke is deterministic"); (2) `row_count_returned` on
  `digest.corpus.retrieve` still counts candidates, so with the narrowing shipped it can exceed the rendered lines
  and no record carries the kept count.
- a11y-plan: 0 proposals. Stripped commentary kept in `.raw-fanout-a11y-plan.md`.

## architecture — parsed list and dispositions

1. D-arch-decisions · warning · §Established Decisions [Fault Identity] · add a dated clause for this chunk: the
   corpus-selection remedy (`select_corpus_matches` gains `triggering_scope`), the three readings and the founder's
   choice, what `CX` does not show, and that the probe keeps no durable known-positive.
   basis: architecture.md:73 · crates/triage/src/digest/retrieval.rs:96-108 · crates/triage/src/digest/assembler.rs:301
   - **Disposition: REJECT as a proposal (pre-check, the re-derivation tell): the basis cites source lines the
     report does not carry. The amendment itself is RAISED by the orchestrator under check 5** (the plan's Expected
     amendment for this site names the change; the report substantiates it) → apply, playbook "Accurate this-chunk
     addition". The applied text is re-derived from the report and the orchestrator's own reads at HEAD.
2. dependent-of D-arch-decisions · same section · the scope obligation's reading ends "whether it fixes that case is
   unmeasured — Conductor's fifth series is the only end-to-end reading of it"; date it to its chunk and state what
   this chunk measured. basis: architecture.md:73 · report.md:14-16, :147-148
   - **Disposition: falls with its primary (the group is atomic); raised by the orchestrator** as the intra-line
     duplicate the cascade's step 2 requires re-reading the amended line for → apply.
3. dependent-of D-arch-decisions · same section · qualify "Incidents with no triggering cue … match on scope only":
   under a cue-bearing digest the scope arm keeps the triggering scope's candidates only.
   basis: architecture.md:73 · crates/triage/src/digest/retrieval.rs:106-108
   - **Disposition: falls with its primary; raised by the orchestrator** for the same reason (a restatement of the
     scope arm's mechanism without the amended tokens) → apply.

## security-plan — parsed list and dispositions

1. D-security-input · warning · §Input Validation, row "L4 inference argv prompt" (:139) · re-base the observed
   maximum: 8,059 B on a live product-composed prompt, 8,067 B on the unremedied synthetic S14 / S15, 8,293 B the
   largest harness cell; ~2.03×, 8,317 B headroom; the constant and the whitelist unchanged.
   basis: report.md:71-77, :128-130, :154-157 · security-plan.md:139
   - **Disposition: APPLY** — check 1: the plan's Expected amendment names this change itself (re-base at both sites
     if a composition reads above 7,824 B), and the playbook's "Accurate this-chunk addition" and the 2026-08-23
     by-actual-class rule both govern (escalate condition affirmatively absent: the new input is validated; the
     finding's class is a measured count). Check 6: disposes the report's disproved claim.
2. dependent-of D-security-input · §Security Anti-Patterns → Code Patterns (:463) · the mirror of 1.
   - **Disposition: APPLY** with its primary.
3. D-security-input · warning · §Input Validation, the same row · record the dev probe's replay input: a captured
   product prompt read from a run-time path outside the tree and passed as the probe's `-p` operand; its validation;
   nothing printed, stored or committed; the product's argv unchanged.
   basis: report.md:164-169, :154-157, :49-51, :271
   - **Disposition: ESCALATE (playbook "Boundary widening", never routine) — RESOLVED on the founder's recorded
     ratification, no new halt.** The founder's own word, 2026-10-07 13:13 local, by dialog, relayed verbatim by the
     pc overseer (inputs#I14 §1): the builder reads the digest itself. The vehicle and that the captures never enter
     a repository: his pick, 2026-10-07 13:49 local, by dialog, relayed by the pc overseer with the picked option
     quoted (inputs#I15). The wrap relay (§2) names this amendment as carrying the pattern. → APPLY, the ratification
     recorded in the sidecar entry.
4. dependent-of D-security-input · §Security Anti-Patterns → Code Patterns (:463) · the mirror of 3; the product
   count stays ONE.
   - **Disposition: APPLY** with its primary (same escalation, same resolution).

Not proposed, read by the orchestrator: the detector flagged the harness-only carve-out enumerations
(§Threat Model Summary → CLI input, §Input Validation env-var row, §Security Anti-Patterns → Input) as listing env
vars and state files only; the replay input is a flag of an example binary and falsifies none of them. No change.

## test-plan — parsed list and dispositions

1. D-tests-coverage · warning · §1 Pending coverage triggers, row `l4-decision-probe-arg-parse-unit-coverage` ·
   append a dated clause: 157 collected pins, `ARMS` 18 → 23, shapes S1–S17, the `replay.rs` module, the replay
   input, the section reader, the candidate arms and `nb`, `--product-path`, no durable known-positive.
   basis: test-plan.md:144 · pulse-app/examples/l4_decision_probe.rs:275, :3901-3903, :3920-3926 · a `grep -c` over
   three source files
   - **Disposition: REJECT as a proposal (the re-derivation tell: source lines and a source grep). RAISED by the
     orchestrator under check 5** (the plan's Expected amendment names the clause) → apply. The orchestrator's own
     reads at HEAD: `const ARMS: [&str; 23]`; `#[test]` 101 + 27 + 29 = 157 over the probe file and its two modules.
2. dependent-of D-tests-coverage · the same row's STILL OWED sentence · the nine flags this chunk added join the
   "every flag but" list.
   - **Disposition: falls with its primary; raised by the orchestrator** → apply. Read at HEAD: each of the nine
     flags has its parse site and stands in the test module (3 to 11 occurrences each).
3. D-tests-coverage · warning · §4 Unit Test Strategy, triage crate · the selection under a triggering scope and its
   five in-crate pins. basis: test-plan.md:228 · retrieval.rs:96, :261, :270, :278 · assembler.rs:1019, :1025
   - **Disposition: REJECT as a proposal (the re-derivation tell). RAISED by the orchestrator under check 5** (the
     plan's Expected amendment, "only if a remedy ships"; one ships) → apply. The five pin names read at HEAD.

## Checks 2-6

- Check 2 (cross-contradiction): none. Proposals on one line (architecture.md:73, security-plan.md:139 and :463,
  test-plan.md:144) edit different sentences in the same direction.
- Check 3 (intent-consistency): two acceptance criteria read UNMET as written. Both were escalated at P1 and are
  resolved on the operator's word, the pc overseer, 2026-10-07, given at this wrap's resume:
  - "(arch) Remedy SELECTED: exactly that candidate is in the product" — answered by the founder's ruling for `CX`,
    2026-10-07 20:08 local, by dialog, relayed by the pc overseer (inputs#I29 §1), recorded as his. The order's
    selection of `CO` stands as measured beside it.
  - "(tests) `S1`-`S16` compose byte for byte as the first reading measured them under `shipped`" — read against
    `nb` for the shapes whose block the shipped remedy changes: any shipped remedy changes them, and the measured
    bytes are held under `nb`. Recorded as the criterion MET IN ITS INTENT AND UNMET IN ITS LETTER.
  The scope record: none (`gate.py scope` clean, 0 recorded).
- Check 4 (absence needs evidence): the security detector's "no third site" rests on its grep of `7,824`, `2.09`,
  `8,560`, `8.4 KiB`, `observed maximum`, `headroom`; the cascade sweep re-runs those patterns over all seven
  masters and the registries (`cascade-dispositions.md`).
- Check 5 (expected amendments): four entries, four covered — architecture [Fault Identity] (raised), test-plan §1
  row (raised), test-plan §4 triage (raised), security-plan row and mirror (proposed, applied).
- Check 6 (disproved claims): security-plan 7,824 B → amended at both sites · the chunk's own premise (a corpus-block
  shape alone reproduces the miss) → a chunk-artifact claim, recorded in the report, no edit owed; architecture.md:73
  takes the reading · the plan's `S1`-`S16` acceptance → check 3 above · the source comment in
  `crates/triage/src/digest/retrieval.rs` ("currently STARVED") → not a master and not this wrap's to edit (a wrap
  touches no source); it goes to P5 as a carry-forward with no owner entry, an armed question for the operator.

## Playbook

- One rule appended on the founder's approval ("add the rule", 2026-10-07 12:06 local, by dialog, relayed by the pc
  overseer, the wrap relay §5): registering a PRE-EXISTING, code-validated, tested env var is registry completeness,
  routine. No other rule proposed: every applied amendment matched an existing rule or a recorded direction.
- No drift-base detector proposed.

## Totals

10 proposals parsed · 4 applied as proposed (security-plan) · 6 rejected as proposals on the re-derivation tell and
raised by the orchestrator at the same sites (architecture 3, test-plan 3) · escalations: 3 resolved (two acceptance
criteria on the operator's word; one boundary widening on the founder's recorded ratification) · 0 open.
