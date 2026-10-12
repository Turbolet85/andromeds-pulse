# Cascade dispositions — 2026-10-12-stdio-agent-reaches-the-door

## Step 2 — the cross-master sweep

The search: `cascade.py sweep` (cascade v1.4) over `cascade-patterns.toml`, written after the last amendment of this
pass was applied (architecture 18 · security-plan 10 · test-plan 7 · obs-plan 10, key files included), baseline
`d3e694d2` (the pre-CI parent). Twelve patterns, each keyed on a retired claim's own wording and its verbs, not its
name: `two-word` (the closed two-word grammar) · `grammar` (every spelling of the `run` / `version` pair) ·
`sidecar-alone` (stdio stated as the sidecar's alone: "stays the sidecar's", "sidecar's transport / wire", "over stdio
per MCP spec") · `only-thing` (one client as the only thing run as another account; "runs the one arm") · `counts`
(5 arms, 41 tests, 112 files, 3174 tests) · `one-process` (one process per program / single process) · `readme`
("Using MCP with") · `second-acct` (the step, the second account) · `no-record` (which words write no record) ·
`ways-in` (two ways in besides telemetry) · `stdout-ban` (stdout reserved) · `three-sentence` (the one required
sentence). Every control fired on the pre-pass masters. Sections read beside the rows: architecture §Occupied Resources
(Process / service identity, MCP stdio surface, Engine door surface, Network ports), §Standard Contracts → Engine door,
§Established Decisions, §Conventions; security-plan §Threat Model Summary, §Input Validation, §API Security,
§Authentication & Authorization, §Logging & Monitoring, §Security Anti-Patterns → Input; test-plan §1, §4, §9; obs-plan
§1, §3, §4 Scenario P3, §11; the five key files this pass touched.

The listing: `total (12 patterns) · 105 rows over 28 files` · `per class · new 1/1 · standing 58/14 · leaf 46/14 ·
curation 0/0 · base 0/0`. Trail `cascade-2026-10-12-stdio-agent-reaches-the-door.json`.

### Master rows (new 1 · standing 58)
- **Amended this pass (`edited` rows, re-read for an intra-line duplicate of the retired claim):** architecture
  `:4`, `:25`, `:29`, `:34`, `:60`, `:63`, `:70`, `:85`, `:157`, `:181`, `:210`, `:211`, `:217`, `:300`, `:319`;
  security-plan `:68`, `:72`, `:73`, `:88`, `:89`, `:104`, `:123`, `:145`, `:207`, `:368`, `:412`; test-plan `:62`,
  `:63`, `:97`, `:206`, `:494`, `:515`; obs-plan `:67`, `:76`, `:109`, `:192`, `:218`, `:223`, `:658`, `:662`; key files
  `deployment-model.md:4`, `ci-cd-approach.md:3`, `project-directory-structure.md`, `per-chunk-gate-discipline.md:52`,
  `logging-stack.md:5`, `snapshot-paste-to-ai-integration.md:9`. Each swept text still standing on them is read and is a
  TRUE claim kept beside the amendment: the `run`-only grammar hits name `run` as the word that boots the engine
  (unchanged); `security-plan.md:145` ×2 `two-word` — "those two words are its first closed members" now speaks of
  `run` and `version` as classified on 2026-10-10, and the row states `door` outside that reading; `architecture.md:60`
  / `security-plan.md:72` `ways-in` — the ways in stay two (the relay enters through the door), as the operator's
  answer reads; `architecture.md:63` / `:85` "over stdio per MCP spec" and `security-plan.md:68` "the double gate is
  the sidecar's alone" — the sidecar's wire and gate, still true, each now beside the relay clause;
  `architecture.md:34` `only-thing` "runs its one client" — the three-layer arm's own client, still one;
  `per-chunk-gate-discipline.md:52` "(5 arms" — the dated 2026-10-10 sentence, followed in the same line by this
  pass's "holds 7 arms (was 5)"; `test-plan.md:494` "3174 tests … at that chunk" — the dated count, followed by 3186;
  obs-plan `:76`, `:658`, `logging-stack.md:5` stdout reservation — the sidecar's ban, now beside the relay's.
- **Standing, not amended — no change:** architecture `:215`, `:231`, `:256`, `:275` ×2 (the `grammar` hit names `run`;
  the `counts` hits are "five arms" of `harness:status` / `harness:ready` verdicts, read by window at `@c1557` and
  `@c4230` — unrelated); security-plan `:148` (`run` seats the door alone — the relay seats nothing, true); test-plan
  `:68`; obs-plan `:72`, `:194` (the MCP stdio row: the sidecar's stdout reservation, true), `:512`; key files
  `heartbeat-ticks.md:3`, `tracing-init.md:4`, `5-command-implementation.md:4`, `per-chunk-gate-discipline.md:48`,
  `pid-file.md:8` (each names `run`); `registries/contracts/a11y-plan/harness-wiring-conventions.md:5` ("41 tests in 18
  files" — the a11y suite, unrelated); `architecture.md:4` heading "Single-process modular monolith" (the case variant
  of `one-process`) — the engine is one process per program, which the amended sentence on the same line now states.
- **The `new` row:** `test-plan.md:515` `three-sentence` — this pass's text, "the three-layer sentence or the bridge
  sentence".
- No hit in another master citing an amended master as saying something it no longer says. `a11y-plan.md:15` quotes
  obs-plan §1's surface count, which no amendment moved — no change.

### Curation homes and judgment bases
- `curation 0/0`, `base 0/0`: no hit in a preserve-verbatim home or in `playbook.md` / `drift-base.md` (the rule this
  wrap appended to `playbook.md` quotes no retired wording).

### Leaf rows (46 over 14 files)
- Each `leaf` row adds its leaf to step 3's set; this wrap runs step 3 in the WHOLE mode (`cascade.py leaves --all`,
  the founder's ruling F252 D2 relayed in inputs#I6 item 2), which covers every leaf whatever the rows.

### What the last wrap left to this cascade (inputs#I6 item 8)
- The handoff written at `d3e694d2` names nothing owed to this wrap's cascade (report, Cross-project claims: one hit
  of `grep -n -iE 'cascade|next wrap|owed to|at the next'`, line 33, a count of that wrap's own sweep). Nothing to
  discharge.

## Leaves

Mode: WHOLE for every group (`cascade.py leaves --all`; the founder's ruling F252 D2, relayed in inputs#I6 item 2: the
first wrap after the 2026-10-12 deploy). Set call: 17 groups · 32 leaves · 763 866 B; `unmapped 2` —
`.claude/rules/host-linux.md` (the host leaf: no master states it, no comparer) and
`.claude/docs/andromeda-after-mvp-playbook.md` (the pipeline's own reference, derived from no master; no comparer).
One comparer per group, one batch inside one snapshot pair (before `tree 7b4c2e76d0202b8a99bb2995e66a7ac763da39b5`,
after `tree b166994bdb888f5ea384ae1a49b18fd95d1511ea`, index and head unchanged; exactly 17 `A leaf-{group}.md`
lines). Every receipt equalled its file's block count. Every finding's source quote was found on the line it cites
(a scripted check over all 78: 76 checked, 0 off their line; the two `tree` findings cite no source line).

Tool line (the apply): `total · blocks 78 · applied 57 · preserved 1 · reshaped 7 · held 13`.

| group | findings | applied (tool) | by hand | declined |
|---|---|---|---|---|
| architecture-1 | 4 | 3 (`conventions.md:23`, `gotchas.md:47`, `services/corpus.md:10`) | 1 (`commands.md:78`, with test-plan-1's — a conflict) | 0 |
| architecture-2 | 3 | 2 (`services/mcp-server.md:6`, `:45`) | 0 | 1 — `services/ui-bridge.md:9`: left on the operator's standing word (the previous handoff: `ui-bridge` leaves with the window) |
| architecture-3 | 6 | 3 (`stack.md:12`, `:21`, `:50`) | 1 (`stack.md:72`, reshaped — a pipe in a code span) | 2 — `workflow.md:31` (`CHANGELOG.md`) and `:89` (`dependency-tree.md` / `api-surface.md`): tree findings on statements no master states (setup's template text) — no comparer's, no step-3 edit; carried to the route card as found-not-owned |
| architecture-4 | 4 | 1 (`CLAUDE.md:100` #2, one engine process per program) | 3 (`CLAUDE.md:35`, `:47`, `:100` #1 — conflicts and a reshape with the three other CLAUDE.md groups; one combined text per line) | 0 |
| security-plan-1 | 5 | 4 (`security-summary.md:22`, `:23` ×2, `:59`) | 1 (`security-summary.md:27`, reshaped) | 0 |
| security-plan-2 | 4 | 4 (`rules/security.md:17`, `:42`, `:45`, `:46`) | 0 | 0 |
| security-plan-3 | 2 | 0 | 2 (`CLAUDE.md:35`, `:100` — folded into the combined text) | 0 |
| design-system | 13 | 11 (`design-summary.md:17`, `:31`, `:94`; `design-tokens.md:20`, `:41`, `:66`, `:79`, `:97`; `frontend.md:17`, `:45`, `:47`) | 2 (`frontend.md:66` #1, #2 — folded with layout-templates' finding on the same line) | 0 |
| layout-templates | 5 | 4 (`design-summary.md:75`, `:77`, `:78`, `:85`) | 1 (`frontend.md:66`, reshaped — `{metrics\|logs}` read as a cell boundary; written without it) | 0 |
| test-plan-1 | 9 | 8 (`tests-summary.md:45`, `:92` ×6, `:93`) | 1 (`commands.md:78` — a new `door` line beside the `version` line) | 0 |
| test-plan-2 | 2 | 2 (`rules/testing.md:22`, `:25`) | 0 | 0 |
| test-plan-3 | 2 | 2 (`rules/verification-harness.md:22`, `:25`) | 0 | 0 |
| test-plan-4 | 2 | 0 | 2 (`CLAUDE.md:35`, `:100` — folded) | 0 |
| obs-plan-1 | 5 | 5 (`obs-summary.md:17`, `:57`, `:73`, `:191`, `:198`) | 0 | 0 |
| obs-plan-2 | 2 | 2 (`rules/observability.md:39`, `:69`) | 0 | 0 |
| obs-plan-3 | 3 | 0 | 3 (`CLAUDE.md:35`, `:47`, `:100` — folded) | 0 |
| a11y-plan | 7 | 6 (`rules/a11y.md:12`, `:74`, `:81`, `:82`; `a11y-summary.md:18`, `:29`) | 1 (`a11y-summary.md:44`, preserved — a heading) | 0 |

78 findings: 57 applied by the tool, 18 written by hand (7 lines), 3 declined. The whole mode caught staleness with no
cause in this chunk (a11y tool majors, design token prose, the Settings modal, the Windows key store, the
`call_tool` dispatch): each was read against its source line and applied as current truth; none touches a master.
