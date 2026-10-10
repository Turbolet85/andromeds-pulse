CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "SWEEP HAZARD — `ci_workflow_runs_on_linux_only` refuses the tokens `windows`, `macos`, `matrix.os` and `runner.os ==` on any line of ci.yml, a comment included" (confidence 0.8)
    Proof: a real gate failure at implement — the workflow self-lint target read `24 passed; 1 failed`, the one red
    `ci_workflow_runs_on_linux_only` at `ci.yml:380`, on a step comment using the word for the app's windows; the
    comment was reworded and the target read `25 passed` (the chunk's report, Harness / gate surface, "A wording
    constraint found"). Score: verified by measurement +0.4 · specific technical detail +0.2 · reached no other
    durable home +0.2 (not on the route, not amended into a master, not a matrix note).
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 1 dup · 0 task-specific · 0 conflict · 0 deferred · 2 below threshold · 1 owned by a master · 1 guard-held · 1 recurrence (→ handoff)
  No-other-home: "SWEEP HAZARD — `ci_workflow_runs_on_linux_only` refuses the token `windows` on any line of ci.yml"

Candidates and their dispositions (six besides the one written):
- A pull-request CI run builds the merge ref — owned by a master: amended into architecture §Infrastructure
  Patterns → CI/CD approach and obs-plan §9 at this wrap's P2. Not curated.
- A green-on-arrival pin proves nothing until mutated — duplicate (Filter 1): `verification-harness.md` Session
  Additions 2026-08-23 ("a green-on-arrival regression guard MUST be mutation-checked") and `testing.md`
  2026-08-17. The matched entries are rules, not defect records, and the rule was followed here.
- `git fetch --depth=1 {sha}` into a full clone marks it shallow — below threshold (specific technical detail
  +0.2; it changed no design, falsified no claim and failed no gate). In the report's Decisions & corrections.
- A deterministic alive-but-never-ready app by a privileged port variable — below threshold (+0.2). In the report
  and in `evidence/live-legs.md`, where the owed harness-level test will find it.
- The Bash guard refused a leading `cd` out of the project root — not a recurrence: the guard held the rule at the
  cost of its one re-issue (host leaf Session Additions 2026-10-05 states it). The older open item on the handoff
  stays as it stands.
- A long listing read through two partial views skipped a row (the cascade sweep's first listing) —
  recurrence-despite-learning: `testing.md` Session Additions 2026-06-05 (a result read through a clipped view)
  and the host leaf's Exit codes rule. To the handoff.

Surfaced, not corrected: four citation rows of this wrap's sweep sit in `## Session Additions` homes
(`rules/frontend.md:99`, `rules/observability.md:148` twice, `rules/security.md:160`); their numbers were stale
before this chunk. They join the handoff's standing note on stale citations in preserve-verbatim homes.

CLAUDE.md size: read from `health.py check` at P7.
