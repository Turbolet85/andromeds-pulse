# Cascade dispositions — 2026-10-02-incident-events-readable-through-mcp (resumed wrap)

Sweep: `cascade.py sweep --patterns-file cascade-patterns.toml` (run after the LAST body amendment of the pass), baseline
`a69030a9` (the parent of the oldest pre-CI commit). Patterns — the retired claims and their verbs/phrasings:
`eight-tools` (8/eight tools, Tools: 8, all 8 tools, 8-tool) · `last4-94` · `roster4-slash` (the four-tool slash roster) ·
`ie-first-writer` · `ie-status-set` (closed IncidentStatus set) · `ie-chokepoint` · `ie-lifecycle-row` (one row per
status VALUE-change) · `dual-sink` · `stderr-layer` (stderr layer / with_writer(stderr) / stderr pretty / both stderr /
stderr + file) · `tty-pretty` · `win-devhost`. Two candidate patterns were REFUSED by the tool (control never fired on
the pre-pass masters) and hand-controlled instead: `tools_list_with_8_tools` and "creation records no event" —
`grep -rnE -i` over CLAUDE.md, `.claude/`, `.andromeda/*.md`: 0 hits (the rename and the falsified premise live only in
source and chunk artifacts). Zero-row patterns `last4-94` / `ie-status-set` are statements about the pattern (their
controls fired: architecture.md:80, security-plan.md:440 pre-pass) — both retired sites were amended.

Leaf enumeration beyond the sweep (the table is a floor): `grep -rnE 'mark_incident_resolved|incident_events|
retrieve_telemetry_slice'` and `grep -rnE -i 'pre-push:linux|unrunnable|stderr|tool methods|\(8\)|mcp-incident-read-back|
tools/list'` over CLAUDE.md, `.claude/docs`, `.claude/rules` — that second grep found `services/mcp-server.md:4`
"Tool methods (8)", which the `eight-tools` regex did not reach.

## Rows
| row | class | disposition |
|---|---|---|
| CLAUDE.md:30 eight-tools | leaf | RE-DERIVED — `mcp-server` module: 9 tools + `retrieve_incident_events` |
| test-plan.md:125 roster4-slash @c617 | standing | NO CHANGE — "the committed subprocess tests `tools/call` only the 4 telemetry tools (…)" is the historical 2026-08-17 finding the row's later NARROWED clauses update; true as a statement of that measurement |
| architecture.md:210 ie-first-writer @c796 | standing (edited) | this pass's own text quoting the retired census as retired — intended |
| .claude/docs/services/corpus.md:26 ie-first-writer | leaf | RE-DERIVED — two writers, four-kind vocabulary, one reader |
| architecture.md:73 ie-chokepoint ×2 @c134,1162 | standing | NO CHANGE — [Corpus Write Arbitration]: the `incidents` status-write choke point (seven writers of the `incidents` row), a different, true claim (windows read) |
| security-plan.md:434 ie-chokepoint ×2 | standing | NO CHANGE — `extract_service_name` choke point (three consumers), unrelated |
| security-plan.md:440 ie-chokepoint | new | this pass's text — `update_incident_status` is the choke point every STATUS writer reaches (true; the creation writer is named separately) |
| .claude/docs/session-learnings.md:2605 ie-chokepoint @c2639 | curation | NO CHANGE — the `incidents` write-race learning (the corpus statement as the seven writers' choke point), true; preserve-verbatim home |
| CLAUDE.md:32 · .claude/rules/security.md:62 ie-chokepoint | leaf | NO CHANGE — `service_name` scrub choke point, unrelated |
| architecture.md:210 ie-lifecycle-row @c1319 | standing (edited) | this pass's text — the status writer, now one of two |
| security-plan.md:440 ie-lifecycle-row | new | this pass's text |
| CLAUDE.md:31 ie-lifecycle-row | leaf | RE-DERIVED — `corpus` module: two writers + vocabulary + reader |
| .claude/docs/services/corpus.md:26 ie-lifecycle-row | leaf | RE-DERIVED (same edit as above) |
| .claude/rules/observability.md:151 dual-sink / tty-pretty | curation | ROUTED TO P3 — the Session Additions `[correction]` entry whose subject (the false body claim) this pass fixed at the source; retired per the plan's expected amendment (route CARRY, overseer 2026-10-02) — curation's channel, never a cascade edit |
| .claude/rules/observability.md:38 dual-sink / tty-pretty | leaf | RE-DERIVED — "Single sink (app)" |
| .claude/rules/observability.md:20 (enumeration) | leaf | RE-DERIVED — init order without the stderr layer |
| .claude/docs/obs-summary.md:56 dual-sink / tty-pretty | leaf | RE-DERIVED — "App single sink" |
| .claude/docs/obs-summary.md:14 (enumeration) | leaf | RE-DERIVED — init step 2 without stderr |
| security-plan.md:440 · test-plan.md:125 stderr-layer | new | this pass's text — "stderr and file-sink canary absence" is the SIDECAR leg's measurement, true |
| obs-plan.md:72 :175 :198 :281 stderr-layer | new | this pass's text ("no stderr layer") — intended |
| .claude/docs/services/mcp-server.md:30 stderr-layer | leaf | NO CHANGE — the SIDECAR's forced-stderr JSON writer, true (obs-plan :438 / :740 stand) |
| test-plan.md:78 · :533 win-devhost | standing | NO CHANGE — dated past measurements ("live on the Windows dev host since 2026-08-23"; a headful leg measured there) |
| CLAUDE.md:149 win-devhost @c2707 | curation | NO CHANGE — quotes Conductor's wdio config; unrelated |
| .claude/rules/verification-harness.md:90 · .claude/docs/commands.md:106 win-devhost | leaf | NO CHANGE — `perf:frame-sample` is a Windows-only verb by design; states the verb's requirement, not the dev host |
| .claude/docs/services/mcp-server.md:4 · :43 (enumeration) | leaf | RE-DERIVED — roster 9 + the tool's contract |
| .claude/docs/conventions.md:23 (enumeration) | leaf | RE-DERIVED — tool methods (9) (it listed only the four telemetry tools, stale since chunk #94) |
| .claude/docs/tests-summary.md:88 · :101 (enumeration) | leaf | RE-DERIVED — pre-push:linux unrunnable on the Linux dev host; Unix arms native |
| .claude/rules/verification-harness.md:95 (enumeration) | leaf | RE-DERIVED — same |
| .claude/docs/commands.md:101 · CLAUDE.md:14 · :36 (enumeration) | leaf | NO CHANGE — describe the verb ("Windows host: … WSL"), true of the verb |

Leaves checked and left unchanged (no amended fact): `.claude/docs/security-summary.md`, `.claude/docs/stack.md`,
`.claude/docs/gotchas.md` (`:49` the sidecar's stderr, true), `.claude/rules/security.md` body (no `incident_events`
distillation), `.claude/rules/testing.md` body. CLAUDE.md `GENERATED:setup:warnings` / pointer table: no amended fact.
Judgment bases (`playbook.md`, `drift-base.md`): 0 rows.
