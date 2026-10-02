# Cascade dispositions — 2026-10-01-real-model-incident-surfacing

Pass amendments (8): arch §Occupied Resources xtask CLI surfaces ×2 (`check:english-sources` registered; `pre-push:linux`
six stages) · test-plan §3 `pre-push:linux` paragraph · §4 interpretation crate (lineage v2.3 / v1.2-* + schema order) ·
§4 triage crate (OVERALL render pins) · §1 trigger `l4-decision-probe-arg-parse-unit-coverage` · §9 lint-test row ·
obs-plan §8 `interpretation.incident.skipped`.

## The search
`cascade.py sweep --patterns-file cascade-patterns.toml` (baseline `09d08091`, the oldest pre-CI commit's parent), four
patterns, each control fired on the pre-pass masters:
- `stages5` — regex ``script-modes`[^·\n]{0,80}· `npm` `` (the retired five-stage list's adjacency, both phrasings:
  bare and the parenthesised test-plan form) — control `architecture.md:244`.
- `stagecount` — `5/5 stages|five stages|5 stages`, case-insensitive (the retired count) — control `test-plan.md:325`.
- `lineage` — `v2\.2\b|v1\.1-fallback|v1\.1-reflection` (the retired prompt-version lineage) — control `security-plan.md:139`.
- `prepush-list` — `` `ci-gates` in order `` (the list's terminator, to reach a restatement whose head differs) —
  control `architecture.md:244`.
The NEW wording (`check:english-sources` / `english-sources` / `Cyrillic` / `interpretation.incident.skipped` /
`OVERALL` / `active-bypass` / `PROMPT_VERSION`) has no pre-pass master hit, so it cannot carry a control; it was grepped
by hand over `CLAUDE.md` + `.claude/rules` + `.claude/docs` (`grep -rnE "Cyrillic|english-sources|interpretation\.incident\.(created|skipped)|OVERALL|active-bypass|pre-push:linux|PROMPT_VERSION"`) — 11 rows, dispositioned below.

## Sweep rows (11 printed)
- `.claude/rules/verification-harness.md:94` stages5 leaf — STALE five-stage list → **re-derived** (six stages, `source-lint` second).
- `.claude/docs/tests-summary.md:88` stages5 + prepush-list leaf — STALE → **re-derived** (six stages).
- `.andromeda/test-plan.md:326` stagecount standing, edited — "Measured green 2026-09-29 (5/5 stages …)" is a dated
  measurement, kept as history beside the new 2026-10-01 6/6 reading → **no change** (true dated claim).
- `.andromeda/test-plan.md:530` stagecount standing — "a revoke mutation reddens FIVE stages" is the webview-drive leg's
  revoke arm, a different subject → **no change** (true claim sharing the token).
- `.claude/rules/verification-harness.md:129` stagecount curation — the webview-drive "guarded by five stages" (Session
  Additions) → **no change** (same unrelated subject; curation home, preserve-verbatim).
- `.claude/rules/testing.md:51` stagecount leaf — "one revoke cycle reddens five stages" (webview-drive P5) → **no change**.
- `.andromeda/security-plan.md:139` lineage standing @c1497 — "sparse-digest v2.2 legs ran 6,386–6,387 B" is a dated
  measurement note inside the L4 argv row → **no change** (history; the bound and the content set are unchanged).
- `.andromeda/security-plan.md:458` lineage standing — "the 2026-08-26 sustained v2.2 range was 6715..=6830 B" is a dated
  measurement note → **no change**.
- `.claude/rules/security.md:170` lineage curation — "rode every v2.2 prompt through 3,678 clean generations" (Session
  Additions, dated) → **no change** (curation home; a true historical claim).
- `.andromeda/architecture.md:244` prepush-list standing, edited @c12034 — this pass's own six-stage text → **amended** (re-read: one occurrence, no duplicate on the line).
- `.claude/docs/tests-summary.md:88` prepush-list leaf — same row as above → **re-derived**.

## Hand-grep rows (new wording)
- `CLAUDE.md:14`, `:36` — the xtask surface listings → **re-derived** (`check:english-sources` added; pre-push "six stages").
- `.claude/docs/commands.md:101` — `pre-push:linux` command line → **re-derived** (six stages named) + the
  `check:english-sources` command line added.
- `.claude/rules/observability.md:66` and `.claude/docs/obs-summary.md:119` — the `interpretation.incident.created` leaf
  entries → **re-derived** (a sibling entry for `interpretation.incident.skipped` added beside each).
- `.claude/rules/verification-harness.md:93`, `:94` — :94 re-derived above; :93 (`smoke:hue-shift` naming `.created`) → no change.
- `.claude/docs/services/interpretation.md:29` — the `.created` exact-leaf note in the crate's service doc → **no change**
  (the skip target is emitted in pulse-app, not this crate; the no-bare-key invariant it states still holds).
- `.claude/docs/session-learnings.md:275`, `:2154` — curation home (Tier 3), unrelated (`priority_tier`; a 2026-05-03
  skill-file Cyrillic note) → **no change**.

## Leaf set (re-derived, by the cascade table + provenance)
- arch changed → `CLAUDE.md` GENERATED blocks (key-dirs + modules lines recomputed for the xtask surface) ·
  `.claude/docs/commands.md` (the xtask command list).
- test-plan changed → `.claude/docs/tests-summary.md` · `.claude/rules/verification-harness.md` (xtask-scoped) ·
  `.claude/rules/testing.md` (read: carries no moved fact — no change).
- obs-plan changed → `.claude/docs/obs-summary.md` · `.claude/rules/observability.md`.
- `CLAUDE.md` `GENERATED:setup:warnings` — read: no line restates a moved fact → no change.
- Judgment bases (`playbook.md`, `drift-base.md`) — 0 rows.
