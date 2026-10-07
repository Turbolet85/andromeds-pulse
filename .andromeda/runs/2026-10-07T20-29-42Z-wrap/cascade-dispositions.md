# Cascade dispositions — wrap of 2026-10-07-l4-probe-reproduces-the-canary-history-miss

Written from the listing of `cascade.py sweep` (cascade v1.1, baseline `48714f09`, the pre-CI parent), run after
every body of this pass was applied. The trail is `cascade-2026-10-07-l4-probe-reproduces-the-canary-history-miss.json`.

## The search

Patterns: `cascade-patterns.toml`, 15 patterns, each with a control that fired on the pre-pass masters. Two changes
to the set between runs, both before any disposition:
- `synthetic-only` (`every string is synthetic | never captured telemetry`) was DROPPED: its control never fired on
  the pre-pass masters (the tool refused the first run, exit 3). Controlled by hand instead: `grep -rn -i` of both
  phrasings over the seven masters, `.andromeda/registries/`, `.claude/rules/`, `.claude/docs/` and `CLAUDE.md`
  returned 0 lines. The phrase lives only in the probe's own module doc (source), which implement corrected.
- `other-incidents` first carried the alternative `sibling` and listed 120 rows (the canary's name in every probe
  record). `sibling` names a fixture, not the claim; it was removed and the sweep re-run. The 84-line listing below
  is the second run.

What was NOT looked for: the numbers of the readings themselves (199 of 200, 11 of 20, …), which are new this pass
and stand in one line each; the model and runner names; Conductor's series ordinals beyond `fifth series`.

## Per pattern

| pattern | rows | disposition |
|---|---|---|
| `max-7824` | security-plan.md:139 @c2802 · :463 @c1349 | amended lines; the two surviving `7,824` are the dated reading of chunk 2026-10-06 ("measured 7,824 B", "before it 7,824 B"), kept as history. No change. |
| `ratio-2.09` · `headroom-8560` · `kib-8.4` | 0 rows each, control fired | the retired ratio, headroom and KiB figures stand nowhere after the pass. |
| `observed-max` | security-plan.md:139 ×3 (@c3025 that chunk's "largest product composition was 7,693 B", a dated reading of its real-model run; @c4032, @c4068 this pass's text) · :463 @c898 (this pass's text) | amended. |
| `observed-max` | test-plan.md:491 · registries/contracts/architecture/ci-cd-approach.md:3 ×3 (window @c1840 read) | a true claim sharing the token: the CI cache's byte headroom. No change. |
| `observed-max` | curation: rules/testing.md:236, :248 · docs/session-learnings.md:1969 | disk and scheduling headroom. Unrelated; nothing routed to P3. |
| `one-argv-path` | architecture.md:72 @c2481 (window read: "`-p {prompt}` is last and stays the only OTLP-derived operand") | a true claim: it describes the PRODUCT's argv, which this chunk did not change. No change. |
| `one-argv-path` | security-plan.md:139 @c5359 · :463 ×2 | amended lines; "ONE" stands for the product, the harness reader is named beside it. |
| `one-argv-path` | leaf: rules/security.md:17 · docs/security-summary.md:48 · docs/services/interpretation.md:31 | re-derived (step 3): the first two gained the harness-only reader; interpretation.md:31 describes the product argv and is unchanged, its "Measurement tool" item below it gained the replay line. |
| `known-positive` | architecture.md:73 ×3 (@c11055 "the probe had no known-positive", dated to chunk 2026-10-06 and still true of the tree; @c12601, @c15601 this pass's text) · test-plan.md:144 ×2 (this pass's text) | amended. |
| `known-positive` | curation: docs/session-learnings.md:14 (read: a probe needing a real failing digest as its control needs a sanctioned capture path decided before the run) | consistent with this chunk (the capture path was the founder's pick). No change, nothing routed to P3. |
| `only-reading` | architecture.md:73 @c11273 | amended: "was unmeasured at that chunk"; the retired "fifth series is the only end-to-end reading" stands nowhere. |
| `scope-arm` | architecture.md:73 ×5 (the amended "match on scope only" sentence and this pass's text) · test-plan.md:228 ×2 (this pass's text) | amended. |
| `scope-arm` | architecture.md:309 ("Active scope drives Epoch 9") · curation docs/session-learnings.md:2408 ("ACTIVE scope lands") | route vocabulary, unrelated. No change. |
| `corpus-block` | architecture.md:73 ×10 | amended line. Standing spans re-read: "when corpus matches exist, a static note under the unchanged `CORPUS MATCHES:` header framing them as other or past incidents" stays true (header and note unchanged; the block now holds fewer lines); "never a corpus match" is the framing instruction, unchanged; "S4 alone carrying a corpus match" and "S8 with two corpus lines naming the canary" are dated readings of earlier chunks. |
| `corpus-block` | architecture.md:239 @c3361 (the title prefix "on … the digest's CORPUS MATCHES lines") | true wherever a line renders. No change. |
| `corpus-block` | security-plan.md:139 ×5 · :463 ×4 | amended lines; the standing spans are dated compositions (S4 with one corpus match, A6 cueless with three, S8 with two). A6 is cueless, so its block is unchanged by the narrowing. |
| `corpus-block` | test-plan.md:144 ×3 · :228 ×6 | amended lines. :144 @c713 "the S4 shape rendering a framed corpus match" is the dated clause of chunk 2026-10-04; the new clause states the measured-block pins moved to `nb`, S4 among the shapes. :228 "dropping the note reds its pin and the probe's S4 pin" is a dated mutation record, not re-run at this wrap. |
| `corpus-block` | test-plan.md:229 ×2 (windows @c340, @c1461 read: the prompt's section order; "CORPUS MATCHES lines are other incidents" inside the framing instruction's wording) | the instruction text is unchanged by this chunk. No change. |
| `corpus-block` | curation: docs/session-learnings.md:688, :690, :698 (the corpus INIT block in `main.rs`, chunk #70) | unrelated. |
| `corpus-block` · `other-incidents` | leaf: docs/services/triage.md:28, :32 | re-derived (step 3): :28 gained the selection under a triggering scope; :32 (the re-exports) is unchanged. |
| `other-incidents` | architecture.md:73 @c6159 | the framing-note sentence above. No change. |
| `pins-91` · `arms-count` · `shape-set` | test-plan.md:144 | amended line; the hits are the dated clause of chunk 2026-10-06 ("to 91 collected pins", "`ARMS` 15 → 18", "the shape set S1–S8"), kept as history before the new clause. |
| `probe` | architecture.md:73 · security-plan.md:139, :463 · test-plan.md:144 | amended lines. |
| `probe` | leaf: docs/services/interpretation.md:38 | re-derived (step 3). |

Judgment bases (`playbook.md`, `drift-base.md`): 0 rows for every pattern.

## Step 2 — lateral binds

`test-plan §3 ↔ obs-plan §3` and `a11y-plan schema ↔ obs-plan schema`: neither side changed (the report's Harness /
gate surface and Schema / config bullets read none). Nothing to keep consistent.

## Step 3 — leaves, recomputed from the amended sources

| source | leaf | result |
|---|---|---|
| architecture.md [Fault Identity] | CLAUDE.md, the Fault Identity warning | one sentence added: the corpus block is prompt context selected per cue, never identity; the scope arm under a cue; the fingerprint arm unchanged. |
| architecture.md | CLAUDE.md overview · modules · pointer table · architecture blocks | recomputed, no change: no stack member, module, crate, command or pointer moved. |
| architecture.md | docs/services/triage.md | :28 re-derived (above). |
| architecture.md | docs/stack.md · docs/conventions.md (the two leaves whose provenance header names architecture.md) | recomputed, no change: §Stack and §Conventions were not amended, and no pattern fired in either. |
| security-plan.md | docs/security-summary.md | :48 re-derived: the harness-only reader and the re-based ratio. |
| security-plan.md | rules/security.md (body, above `## Session Additions`) | :17 re-derived: the harness-only reader. |
| security-plan.md | CLAUDE.md warnings block | recomputed, no change: it carries no L4 argv line. |
| test-plan.md | docs/tests-summary.md · rules/testing.md (body) | recomputed, no change: neither distills the `l4-decision-probe-arg-parse-unit-coverage` row or the triage unit strategy (0 hits for the probe, the row id and the selection). |
| security-plan.md · architecture.md | docs/services/interpretation.md | the "Measurement tool" item gained two lines (the replay input; `nb` and the candidate arms). |

`## Session Additions` and `USER:session-learnings` were not edited by the cascade.
