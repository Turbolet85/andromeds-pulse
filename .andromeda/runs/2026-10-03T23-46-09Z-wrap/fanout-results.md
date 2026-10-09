# Fan-out results — 2026-10-02-incident-events-readable-through-mcp (resumed wrap)

Fan-out fired 2026-10-04 in the resumed run (P1 report reused as is; scope read re-fired `clean`). Keyed-contract
renders: `registry.py contracts` → `NOT MIGRATED` for architecture · test-plan · obs-plan · a11y-plan (exit 3), so the
contracts line was dropped from every prompt. No return needed stripping beyond its trailing `#` commentary (the
detector's own no-drift notes, kept below as the verdict basis); no HTML entities in any return (decode probe:
entities=0); no raw twin warranted.

## Verdicts
- architecture — 5 proposals (D-arch-resources ×5: 1 primary roster + 3 dependents, 1 census/reader); D-arch-decisions: no drift (Dependencies none, Cargo.lock identical).
- security-plan — 3 proposals (D-security-input ×2 escalate-severity, D-security-logging ×1); D-security-auth / D-security-deps: no drift.
- design-system — 0 (`proposals: []`; no UI, tokens n/a; no status claim touched).
- layout-templates — 0 (`proposals: []`; no surface; MCP mentions count-free).
- test-plan — 4 proposals (D-tests-coverage ×2, D-tests-framework ×2); D-tests-obs-harness: no drift.
- obs-plan — 13 proposals (D-obs-instrumentation ×5: roster/method set; D-obs-stack ×8: app sink file-only); D-obs-pii / D-obs-defect-narrative: no drift.
- a11y-plan — 0 (`proposals: []`; no interactive element, no schema change).

## Parsed proposals + dispositions

### architecture
| # | detector | section | basis | dep | disposition |
|---|---|---|---|---|---|
| A1 | D-arch-resources | §Standard Contracts → MCP server — spec-conformant (roster 8→9 + the tool's contract) | architecture.md:150 | — | APPLY — check 1 `Accurate this-chunk addition` (routine); check 5 floor entry |
| A2 | D-arch-resources | §Conventions → External wire — MCP server (roster) | :80 | A1 | APPLY (group) |
| A3 | D-arch-resources | §Stack and Technologies → MCP server row ("8 named tools") | :25 | A1 | APPLY (group) |
| A4 | D-arch-resources | §Established Decisions → [MCP Server Surface] ("8-tool wire surface") | :55 | A1 | APPLY (group); the decision itself unchanged |
| A5 | D-arch-resources | §Occupied Resources → Corpus SQLite → Reserved tables (`incident_events` writer census 1→2, four-kind vocabulary, first reader) | :210 | — | APPLY — check 1 `Drift proposal ACCURATELY correcting a doc claim … PRE-EXISTING reality falsifies` (no impl half → plain apply) + `Accurate this-chunk addition` (reader); check 6 disposes report disproved #2 |

### security-plan
| # | detector | section | basis | dep | disposition |
|---|---|---|---|---|---|
| S1 | D-security-input (escalate) | §Input Validation → MCP stdio inputs row ("all 8 tools" → 9; the tool's validation) | security-plan.md:140 | — | ESCALATED-AND-RESOLVED — check 1 `Boundary widening` (escalate): the ninth read surface was ratified by founder ruling 2026-10-02 at P4 (plan Provenance, Expected amendments) and the shared-vocabulary widening by founder ruling 2026-10-03 (option A, scope-record) → APPLY, ratification named in the sidecar |
| S2 | D-security-input (escalate) | §Threat Model Summary → MCP stdio Entry point ("Tools: 8") | :68 | S1 | same resolution → APPLY (group) |
| S3 | D-security-logging | §Security Anti-Patterns → Logging, `incident_events` NO-SCRUB paragraph (vocabulary, two writers, one reader) | :440 | — | APPLY — check 1 measurement-disproved doc claim, no impl half; check 6 disposes report disproved #1; restating sites (:46-48, :166) read by the detector, table name only |

### test-plan
| # | detector | section | basis | dep | disposition |
|---|---|---|---|---|---|
| T1 | D-tests-coverage | §1 Pending coverage triggers → `mcp-incident-read-back-cross-process-coverage` (narrowed: the ninth tool shipped with two cross-process content legs; mcp-gated count no longer 2; row stays OPEN) | test-plan.md:125 | — | APPLY — `Accurate this-chunk addition`; check 5 floor entry |
| T2 | D-tests-coverage | §6 Scenario P3 → Current residual | :507 | T1 | APPLY (group) |
| T3 | D-tests-framework | §3 Process-end witness form ("unrunnable on the Windows dev host, run by pre-push:linux") | :337 | — | APPLY — report Changes carries the host move (Dev-tool versions) + the Linux count 2575 = 2573 + the two Unix arms; the operator's recorded direction (founder, host move 2026-10-03) settles it |
| T4 | D-tests-framework | §3 `cargo xtask pre-push:linux` (dev-host only on Windows+WSL; unrunnable on the Linux dev host; native stages per founder ruling; port owed) | :327 | T3 | APPLY (group) — applied text re-derived: the port's placement is the founder/overseer ruling of 2026-10-04 (after "Retry-storm…", before the version close), not the report's "after P1" |

### obs-plan
| # | detector | section | basis | dep | disposition |
|---|---|---|---|---|---|
| O1 | D-obs-instrumentation | §4 Scenario P3 → Required span attributes (`method` set 4 → the nine tools; `result_type` incident_events) | obs-plan.md:308 | — | APPLY — `Accurate this-chunk addition` + report disproved #4 (pre-existing staleness since #94, now 9); check 5 floor entry |
| O2 | D-obs-instrumentation | §3 Snapshot / paste-to-AI → MCP tools bullet (four tools = the external-OTLP subset) | :236 | O1 | APPLY (group) |
| O3 | D-obs-instrumentation | §1 Critical paths → P3 row (`method` set) | :109 | O1 | APPLY (group) |
| O4 | D-obs-instrumentation | §4 Scenario P3 → Must-trace spans (DuckDB child scoped to the telemetry tools) | :307 | O1 | APPLY (group) |
| O5 | D-obs-instrumentation | §1 Snapshot / paste-to-AI → External-OTLP snapshot bullet (four tools = subset of nine) | :96 | O1 | APPLY (group) |
| O6 | D-obs-stack | §3 Logging stack → Sink (app file-only) | :198 | — | APPLY — report disproved #6 (measured `observability.rs::init`, file layer only; re-read at HEAD this wrap, `pulse-app/src/observability.rs:2648-2658`); check 5 floor entry |
| O7 | D-obs-stack | §1 Logging stack | :76 | O6 | APPLY (group) |
| O8 | D-obs-stack | §1 Tracing init step (2) | :72 | O6 | APPLY (group) |
| O9 | D-obs-stack | §3 Init order step (2) | :175 | O6 | APPLY (group) |
| O10 | D-obs-stack | §1 Telemetry surfaces → CLI row Sink | :67 | O6 | APPLY (group) |
| O11 | D-obs-stack | §4 Instrumentation per surface → CLI / main binary row | :281 | O6 | APPLY (group) |
| O12 | D-obs-stack | §6 Sink configuration → App | :437 | O6 | APPLY (group); :438 (sidecar stderr) stays |
| O13 | D-obs-stack | §7 Panic hook (record file-only; stderr carries the chained prior hook's output) | :471 | O6 | APPLY (group) — the plan's Expected amendments states the panic-hook form; re-read at HEAD (`install_panic_hook`, `take_hook` → prior hook chained) |

## Validate — the six checks
1. Playbook — 24 routine applies; S1/S2 match `Boundary widening` (escalate) and are resolved by the founder's recorded ratifications (2026-10-02 P4; 2026-10-03 option A), named in the sidecar. No two-rule collision.
2. Cross-contradiction — none: no two proposals edit one section in opposing directions.
3. Intent-consistency — the scope record's three `widening` lines carry the founder's word (option A, 2026-10-03) → justified; the plan's "creation records none" premise is superseded (premise-correction.md) and every applied text states the four-kind vocabulary with `created` first. Plan gate 6 (no pulse-app source change) is red by the same widening — superseded by that founder ruling (operator direction 2026-10-04: record it so at P7).
4. Absence needs evidence — the arch and security detectors' "no other site restates" claims are not taken on trust: the cascade sweep (step 2) over all seven masters decides them.
5. Expected-amendments floor — every plan entry is matched: arch roster ×4 + `incident_events` reader (A1–A5); security Threat Model / Input Validation / Logging (S1–S3); obs P3 method set + app-sink sites :67 :72 :76 :175 :198 :281 :437 :471 (O1–O13; the §4 MCP stdio row :283 names no roster and its sidecar stderr claim is true); test-plan :125 / :507 (T1–T2); the leaves `.claude/rules/observability.md` / `.claude/docs/obs-summary.md` → cascade step 3, and the `rules/observability.md` `[correction]` Session Additions entry → P3 curation (a preserve-verbatim home); `matrix#P-075 notes` → P7.3.
6. Disproved claims — #1 → S3 · #2 → A5 · #3 (CLAUDE.md §Modules corpus) → cascade leaf re-derive of arch · #4 → O1–O5 · #5 chunk artifacts → recorded in the report + `evidence/premise-correction.md`, no amendment owed · #6 → O6–O13.

Escalations open after validate: 0.
