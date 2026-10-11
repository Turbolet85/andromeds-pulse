# Cascade dispositions — 2026-10-10-door-inside-the-engine-s-process

Written from the third run of `cascade.py sweep` (cascade v1.2), made after every body amendment and every leaf
re-derivation of this pass. The listing's own lines:

    total (21 patterns) · 145 rows over 29 files
    per class · new 5/3 · standing 90/13 · leaf 43/15 · curation 7/3 · base 0/0

## What was searched

Patterns (`cascade-patterns.toml`, 21; every control fired on the pre-pass masters and key files at `303fa9a8`):
`listeners` (two receivers and nothing else, no third socket, no socket beyond them) · `alone` (the other receiver
alone) · `tworecv` (the boot list "the two receivers, the buffer") · `dispatchsite` (the single `dispatch_tool`
site) · `stdio` (JSON-RPC 2.0 over stdio / stdin/stdout, MCP over stdio, "(no socket)") · `featgate` (gated by the
feature, only present when, feature-gated) · `doublegate` · `portbins` (six test binaries, four port-using, four
binaries) · `count109` · `stageports` (binds loopback 24317, two loopback ports) · `authnone` (no per-user identity,
auth approach none, bans omitted) · `threebin` · `surfaces8` · `eventsreader` (read by the sidecar, one production
READER) · `addprops` (`additionalProperties: false`) · `seventh` (SEVEN production writers) · `lintseq` (the
`lint-test` step sequence after `cargo xtask test`) · `envsibling` (the sibling variable, to find every variable
list) · `bootrecord` (`app.boot.engine`, to find every site a boot record is enumerated) · `wskey`
(`run/workspace-key`) · `testinject` (the console program's read-back paths).

Three patterns print 0 rows with their control fired — `threebin`, `surfaces8`, `lintseq`: each retired wording
stood at the base and no longer stands anywhere (obs-plan §9 reads four; obs-plan §1 and a11y-plan §1 read 9; the
two `lint-test` sequences were rewritten). Each zero is a statement about its pattern.

One pattern was dropped from the tool's set and controlled by hand: `optionalcrate` (`only optional crate` ·
`optional crate` · `optional = true`) fired on no pre-pass master, so the tool refused it. Run by hand as `git grep
-n -E` over CLAUDE.md, `.claude/`, the four amended masters and the registries; its known positive is
`.claude/rules/security.md:111` (the 2026-05-12 Session Addition, "an OPTIONAL feature-gated workspace crate").
Hits: that line (a curation home — the entry says `mcp-server` is the only optional crate, which this chunk ended;
carried to the route card as found and not owned, item 9, never edited here) and ten lines of
`docs/session-learnings.md` on `rmcp` and workspace-dependency optionality (another subject, still true). No master
and no body of a leaf states the retired claim.

Found by the sweep and amended in this pass, outside every proposal: `a11y-plan.md` §1 Justification quoted
obs-plan's "8 major surfaces" (the first run's one `surfaces8` row outside obs-plan) — now 9, with its own sidecar
entry; and the second mention of the sixth stage on the pre-push paragraph of the test-plan key file, which took a
pointer to the sentence added earlier on the same line.

Sections read whole for restatements however worded: architecture §Design Philosophy, §Established Decisions
[MCP Server Surface] and [Corpus Write Arbitration], §Standard Contracts, §Occupied Resources (Network ports,
Process identity, Filesystem locations, Environment variables); security-plan §Threat Model Summary (Attack
surface, Auth model, Infrastructure), §Authentication & Authorization, the §Input Validation table, §API Security,
§Logging & Monitoring, §Security Anti-Patterns (Authentication, Input, Logging); test-plan §1 (Surfaces, Coverage
scope, Coverage triggers, Pending), §9; obs-plan §1, §4 (per-surface table, Scenario P3), §6 warn row, §8
allowlist, §9, §10, §11. Not searched: the sidecars, the archives, chunk artifacts and run dirs (never opened by
this step), and design-system and layout-templates beyond the rows below (their detectors returned no drift; their
MCP-toggle and "MCP server status change" lines are the removal entry's).

## Every row, by pattern

### `listeners`

- standing: `.andromeda/architecture.md:180` · `.andromeda/test-plan.md:62` · `.andromeda/test-plan.md:155` · `.andromeda/registries/contracts/test-plan/per-chunk-gate-discipline.md:52` → amended — the retired wording stands on the line only as the dated reading of the earlier chunk ("until that chunk …" / the pending row's LANDED text, which this pass qualified by a re-read sentence on the same line)

### `alone`

- new: `.andromeda/test-plan.md:62` → this pass's own text — the amended wording itself

- standing: `.andromeda/test-plan.md:155` · `.andromeda/registries/contracts/test-plan/per-chunk-gate-discipline.md:52` → amended — as `listeners`: the dated reading of the landing chunk, beside the current one

### `tworecv`

- standing: `.andromeda/architecture.md:217` · `.andromeda/security-plan.md:104` → amended — the boot list on the line now names the door

### `dispatchsite`

- standing: `.andromeda/security-plan.md:147` → amended — the row states `call_tool` under `dispatch_tool`; "name-dispatch via `tools.rs::dispatch_tool`" is the sidecar's path and true

### `stdio`

- standing: `.andromeda/architecture.md:5` · `.andromeda/architecture.md:210` → no change — §Design Philosophy names MCP over stdio as one edge, still true of the sidecar; and the `MCP stdio surface` bullet is the sidecar's, the door has its own bullet beside it

- standing: `.andromeda/architecture.md:25` · `.andromeda/architecture.md:60` · `.andromeda/architecture.md:63` · `.andromeda/architecture.md:85` · `.andromeda/architecture.md:319` · `.andromeda/security-plan.md:68` · `.andromeda/test-plan.md:38` · `.andromeda/registries/contracts/obs-plan/snapshot-paste-to-ai-integration.md:9` → amended — "over stdio" is scoped to the sidecar and the door named as the second carrier on the same line

- standing: `.andromeda/registries/contracts/obs-plan/logging-stack.md:5` → no change — the sidecar's stderr sink, true of the sidecar

- standing: `.andromeda/registries/obs-plan-contracts.toml:34` → no change — the index repeats the key file's label text, which this pass did not rename

- leaf: `CLAUDE.md:100` · `.claude/docs/security-summary.md:22` · `.claude/docs/services/mcp-server.md:4` · `.claude/docs/stack.md:21` → re-derived — the leaf states the sidecar's stdio and, where its master section changed, the door beside it (CLAUDE.md's architecture block is recomputed from §Design Philosophy, unchanged, so its text is the same)

### `featgate`

- standing: `.andromeda/architecture.md:25` · `.andromeda/security-plan.md:207` → amended — the gate is scoped to the sidecar and the `rmcp` anchor on the same line

- standing: `.andromeda/architecture.md:194` · `.andromeda/architecture.md:218` · `.andromeda/architecture.md:292` → no change — the `mcp.*` TauRPC procedures, the sidecar binary and the tray toggle are the sidecar's and the window's, each still behind the feature

- standing: `.andromeda/security-plan.md:60` · `.andromeda/security-plan.md:67` → no change — the `mcp.*` procedures and the title of the sidecar's own vector; both true of the sidecar

- new: `.andromeda/security-plan.md:68` → this pass's own text — the amended wording itself

- standing: `.andromeda/design-system.md:325` → no change — the tray toggle exists only under the feature, true; it leaves with the removal entry

- standing: `.andromeda/test-plan.md:344` · `.andromeda/test-plan.md:356` → no change — scenario P3's sidecar surfaces and its guard-declined arms, true of the sidecar

- standing: `.andromeda/registries/contracts/architecture/deployment-model.md:4` → no change — the optional sidecar process of the runtime topology, true; the door adds no process

- curation: `.claude/docs/session-learnings.md:1328` → curation home — no edit: read, another subject sharing the token (a session number, a line range, a serde note, a run ordinal, a test's env gate)

- leaf: `.claude/docs/services/mcp-server.md:44` · `.claude/docs/services/ui-bridge.md:17` · `.claude/docs/stack.md:21` → re-derived — scoped to the sidecar and `rmcp` where the master section changed; `mcp.*` procedures and the sidecar binary stay feature-gated, true

### `doublegate`

- new: `.andromeda/security-plan.md:68` → this pass's own text — the amended wording itself

- standing: `.andromeda/security-plan.md:207` · `.andromeda/obs-plan.md:109` · `.andromeda/obs-plan.md:662` → amended — the double gate is scoped to the sidecar on the same line

- standing: `.andromeda/security-plan.md:485` → no change — the Code Patterns ban on spawning the sidecar without both gates, the sidecar's

- standing: `.andromeda/obs-plan.md:194` → no change — the `MCP stdio` row of the per-surface table, the sidecar's; the door has its own row directly below

- leaf: `CLAUDE.md:47` · `.claude/rules/security.md:39` · `.claude/rules/security.md:40` · `.claude/rules/security.md:42` · `.claude/docs/obs-summary.md:198` · `.claude/docs/security-summary.md:22` · `.claude/docs/services/mcp-server.md:4` · `.claude/docs/services/mcp-server.md:47` · `.claude/docs/services/mcp-server.md:63` → re-derived — the leaf now says the double gate is the sidecar's alone

### `portbins`

- standing: `.andromeda/architecture.md:183` → amended — the count reads seven and five; "seven of seven hand runs of the four binaries" on the same line is the dated reading of chunk 2026-10-10-shared-telemetry-test-data and stands

- standing: `.andromeda/test-plan.md:502` · `.andromeda/registries/contracts/test-plan/per-chunk-gate-discipline.md:52` → amended — the four-binary count is dated to chunk 2026-10-10-shared-telemetry-test-data and the five stated beside it

### `count109`

- standing: `.andromeda/architecture.md:275` · `.andromeda/test-plan.md:206` · `.andromeda/registries/contracts/test-plan/per-chunk-gate-discipline.md:32` → amended — 112 leads; each 109 left on the line is a dated reading

- curation: `.claude/rules/observability.md:144` · `.claude/docs/session-learnings.md:776` · `.claude/docs/session-learnings.md:802` · `.claude/docs/session-learnings.md:1392` → curation home — no edit: read, another subject sharing the token (a session number, a line range, a serde note, a run ordinal, a test's env gate)

- leaf: `.claude/rules/testing.md:25` → re-derived — 112 leads, 109 a dated reading

### `stageports`

- standing: `.andromeda/architecture.md:275` · `.andromeda/security-plan.md:89` · `.andromeda/security-plan.md:412` · `.andromeda/obs-plan.md:562` · `.andromeda/registries/contracts/test-plan/per-chunk-gate-discipline.md:32` → amended — the line states that the stage's engine also opens the door's socket, under the same PROVISIONAL item; a second mention on the same line was read for an intra-line duplicate and takes a pointer or stands under the sentence added on that line

- leaf: `CLAUDE.md:45` · `.claude/rules/security.md:17` · `.claude/rules/verification-harness.md:100` · `.claude/docs/security-summary.md:44` · `.claude/docs/tests-summary.md:96` → re-derived — the leaf carries the socket sentence and the same PROVISIONAL item

### `authnone`

- standing: `.andromeda/security-plan.md:121` · `.andromeda/security-plan.md:260` → no change — the section's opening sentence and the bootstrap phase's SKIP speak of user auth, of which there is still none; the owner check is stated in the paragraph this pass added directly below the first

- standing: `.andromeda/security-plan.md:326` · `.andromeda/security-plan.md:398` → amended — the sentence now says user accounts, and names the door peer's account id or the owner check

### `eventsreader`

- standing: `.andromeda/architecture.md:225` · `.andromeda/security-plan.md:461` → amended — two readers named

- standing: `.andromeda/security-plan.md:76` → no change — `run/workspace-key` is read by the sidecar, true; the door reads none

### `addprops`

- standing: `.andromeda/architecture.md:155` · `.andromeda/security-plan.md:147` → amended — declared and not enforced on the sidecar, as measured

- leaf: `.claude/rules/security.md:53` · `.claude/docs/security-summary.md:22` · `.claude/docs/services/mcp-server.md:6` → re-derived — this pass's own text: declared, unenforced on the sidecar

### `seventh`

- standing: `.andromeda/architecture.md:78` → amended — the seventh writer's second carrier stated

- standing: `.andromeda/test-plan.md:518` → no change — the seventh UPLOAD of `ci.yml`, another subject sharing the word

- curation: `.claude/docs/session-learnings.md:2842` → curation home — no edit: read, another subject sharing the token (a session number, a line range, a serde note, a run ordinal, a test's env gate)

- leaf: `.claude/docs/services/corpus.md:14` · `.claude/docs/tests-summary.md:87` → re-derived (`services/corpus.md`) or another subject (the seventh upload, `tests-summary.md`)

### `envsibling`

- standing: `.andromeda/architecture.md:255` → no change — the sibling variable's own row; the new variable's row stands in the same list

- standing: `.andromeda/security-plan.md:144` → amended — the variable list gained `ANDROMEDA_PULSE_DOOR_RESOLVE`

- standing: `.andromeda/test-plan.md:231` → no change — a unit-strategy mention of that variable's resolver, another subject

- standing: `.andromeda/registries/contracts/test-plan/per-chunk-gate-discipline.md:22` · `.andromeda/registries/contracts/test-plan/per-chunk-gate-discipline.md:24` → no change — the scenario legs' own environment, another subject

- curation: `.claude/rules/testing.md:278` → curation home — no edit: read, another subject sharing the token (a session number, a line range, a serde note, a run ordinal, a test's env gate)

### `bootrecord`

- standing: `.andromeda/architecture.md:231` · `.andromeda/architecture.md:275` · `.andromeda/security-plan.md:144` · `.andromeda/security-plan.md:412` · `.andromeda/test-plan.md:62` · `.andromeda/obs-plan.md:192` · `.andromeda/obs-plan.md:331` → amended on another count — the line gained a door sentence; its `app.boot.engine` mention is a true claim sharing the token

- standing: `.andromeda/architecture.md:256` · `.andromeda/architecture.md:261` · `.andromeda/test-plan.md:68` · `.andromeda/obs-plan.md:450` · `.andromeda/registries/contracts/obs-plan/heartbeat-ticks.md:3` · `.andromeda/registries/contracts/obs-plan/heartbeat-ticks.md:5` · `.andromeda/registries/contracts/test-plan/5-command-implementation.md:5` · `.andromeda/registries/contracts/test-plan/5-command-implementation.md:30` · `.andromeda/registries/contracts/test-plan/pid-file.md:8` → no change — a site that reads or describes `app.boot.engine`, a true claim sharing the token; the door's own record `app.boot.door` is registered at its own sites

- leaf: `CLAUDE.md:35` · `.claude/rules/observability.md:68` · `.claude/rules/security.md:17` · `.claude/rules/verification-harness.md:22` · `.claude/rules/verification-harness.md:24` · `.claude/rules/verification-harness.md:59` · `.claude/docs/commands.md:104` · `.claude/docs/commands.md:105` · `.claude/docs/obs-summary.md:63` · `.claude/docs/services/interpretation.md:44` · `.claude/docs/stack.md:38` · `.claude/docs/tests-summary.md:12` · `.claude/docs/tests-summary.md:14` → no change — a true claim sharing the token; `rules/observability.md` gained the door's bullet directly below this one

### `wskey`

- new: `.andromeda/architecture.md:157` · `.andromeda/security-plan.md:148` → this pass's own text — the amended wording itself

- standing: `.andromeda/architecture.md:231` · `.andromeda/security-plan.md:68` → amended on another count — the line gained a door sentence; `run/workspace-key` stays the sidecar's

- standing: `.andromeda/security-plan.md:76` · `.andromeda/security-plan.md:143` · `.andromeda/test-plan.md:36` · `.andromeda/test-plan.md:126` · `.andromeda/test-plan.md:264` · `.andromeda/obs-plan.md:252` · `.andromeda/obs-plan.md:438` → no change — `run/workspace-key` is published for the sidecar and read by it, true; the door reads none

- leaf: `.claude/rules/security.md:52` · `.claude/docs/services/mcp-server.md:18` · `.claude/docs/services/workspace-detector.md:4` → no change — the key is the sidecar's, true; `rules/security.md`'s hit is this pass's "reads no `run/workspace-key`"

### `testinject`

- standing: `.andromeda/architecture.md:295` → amended — the third read-back path stated
