# Fan-out results — 2026-10-12-stdio-agent-reaches-the-door

Batch (a 2a resume's first fan-out; no earlier `fanout-*.md` stood): before `tree 9b81054086c56e1427d9a17120246584eac431f3
index 6cbbc4f9b03d8f80393012350234916ed895374d head 2595b5cc47a9cb51b4e03d904209a12f49fdf125`, after `tree
606268ee349032b9ddd8e49d65bef65c274c0644`, index and head unchanged; `git diff-tree` printed exactly the seven `A
.andromeda/runs/2026-10-12T09-33-13Z-wrap/fanout-{doc}.md` lines. Every receipt's count equals its file's. Entity probe 0
in all seven; no fix, no raw twin. Detector prompts: arch 2 · security 4 · design 2 · layout 2 · test 3 · obs 4 · a11y 2
= 19, the drift-base's `doc:` names.

Validate's opening rule: 0 rejected (every source coordinate cited is the report's or its New-text section's:
`door_bridge.rs:27-48 · :64-73 · :75-112 · :114-129 · :131-144`, `lib.rs:15`, `ci.yml:114-122 · :128 · :142-145`,
`integration_door.rs:1023-1141`; every other basis is a master line).

Playbook: the subject of every proposal below except arch 15 and test 7 is the door's widening this chunk extends — the
relay word, the stdio way through the door, the second-account step's second arm — so each escalates under **Boundary
widening** (check 1, judged by subject whatever its severity), PROVISIONAL with the door's one item. The founder did not
answer this widening himself at P4 (inputs#I3 is the pc overseer's answer): no answer card; the halt is the ordinary
form (inputs#I6 item 3). Group = the plan's expected amendment it serves (E1–E14).

architecture: 15 proposals · fanout-architecture.md
note: D-arch-decisions no drift; the one-process wording (§Design Philosophy, Stack Message-broker row, [Mobile / Message
Broker / Push Notifications]) left to the orchestrator — raised below as arch 16–18.
1. escalate (check 1, E1) — §Occupied Resources → Process / service identity, the three-word grammar and the relay
2. escalate (check 1, E1, dependent of 1) — §Project Intent → Product type, three-word command line
3. escalate (check 1, E3) — §Standard Contracts → Engine door, the Stdio clause
4. escalate (check 1, E4) — §Occupied Resources → MCP stdio surface, the relay carries the door's methods over stdio
5. escalate (check 1, E4, dependent of 4) — §Occupied Resources → Engine door surface, retire "stays the sidecar's"
6. escalate (check 1, E4, dependent) — §Established Decisions [MCP Server Surface]
7. escalate (check 1, E4, dependent) — §Established Decisions [API Style — MCP Server Surface 3]
8. escalate (check 1, E4, dependent) — §Conventions → External wire — MCP server
9. escalate (check 1, E4, dependent) — §Stack and Technologies → MCP server row
10. escalate (check 1, E4, dependent) — §Inherited Defaults → API style — external MCP
11. escalate (check 1, E5) — §Occupied Resources → Network ports, the PROVISIONAL item covers the word and the arm
12. escalate (check 1, E2) — §Infrastructure Patterns → Deployment model (key file), the relay process
13. escalate (check 1, E10 site) — §Infrastructure Patterns → CI/CD approach (key file), two arms, two sentences
14. escalate (check 1, E10/E8 subject: the step's second arm) — §Stack and Technologies → Gate-script pins row
15. apply (check 1: Accurate this-chunk addition, routine) — §Infrastructure Patterns → Project directory structure,
    `door_bridge.rs` listed (the key file lists each `pulse-app/src` module, `door.rs` among them)
16. escalate (raised by main, check 1, E2 dependent) — §Design Philosophy (`architecture.md:4`), "the OS sees one process
    per program" → one engine process per program; the console program's `door` word runs as short-lived relay
    processes that host no engine part
17. escalate (raised by main, E2 dependent) — §Stack and Technologies → Message broker row (`:29`), the same clause
18. escalate (raised by main, E2 dependent) — §Established Decisions [Mobile / Message Broker / Push Notifications]
    (`:70`), the same clause

security-plan: 10 proposals · fanout-security-plan.md
note: no D-security-deps proposal (Dependencies none); no stale coverage claim in §Security Anti-Patterns → Logging.
1. escalate (check 1, E6) — §Input Validation → console command-line row, three words and the relay's validation
2. escalate (check 1, E6 dependent) — §Threat Model Summary → CLI input vector, entry point (`:88`)
3. escalate (check 1, E6 dependent) — §Threat Model Summary → CLI input vector, trust boundary
4. escalate (check 1, E6 dependent) — §Threat Model Summary → Infrastructure → Hosting (`:104`)
5. escalate (check 1, E7) — §Threat Model Summary → the door's vector, the stdio way through the relay
6. escalate (check 1, E7 dependent) — §Threat Model Summary → MCP stdio surface vector
7. escalate (check 1, E7 dependent) — §API Security → MCP feature double-gate row
8. escalate (check 1, E8; check 6: plan step 6 lands here) — §Authentication & Authorization, the relay checked as any
   peer, its other-account reading
9. escalate (check 1, E8) — §Security Anti-Patterns → Input, the second-account step sentence (`:412`), two arms
10. escalate (check 1, E7 subject: the relay as a carrier; the README half is Applied-as-measured with its owner
    `Records say what the product is`, the P5 card's Beyond-the-plan row 2) — §Logging & Monitoring → tool-response
    hygiene

test-plan: 7 proposals · fanout-test-plan.md
note: D-tests-framework and D-tests-obs-harness no drift; the sidecar-scoped P3 rows left (they describe the sidecar
and stay true; P3 at `test-plan.md:78` and §6 Scenario P3 at `:342` read and left — neither says stdio is the
sidecar's alone).
1. escalate (check 1, E9) — §1 door row, the fourth depth (`unit_door_bridge` 8; `integration_door` 7 arms, was 5)
2. escalate (check 1, E9 dependent) — §1 console engine program row, three words
3. escalate (check 1, E9 dependent) — §1 MCP feature + runtime gating row, the relay's negative readings
4. escalate (check 1, E10; check 6: plan step 6 lands here) — §9 `lint-test` row, two arms, two sentences; main adds
   the `quality_gate_workflow.rs` count the report moves (41 → 42)
5. escalate (check 1, E10 dependent) — §9 Build failure conditions, either sentence missing
6. escalate (check 1, E11) — §3 → Per-chunk gate discipline (key file), Spawned-program witness form
7. apply (check 1: Accurate this-chunk addition, routine) — §4 Rust test location, 112 → 113 files

obs-plan: 10 proposals · fanout-obs-plan.md
note: D-obs-pii and D-obs-defect-narrative no drift; the §9 `xtask test` row ("four test binaries start
`andromeda-pulse-engine run`") read and left — still true.
1. escalate (check 1, E12) — §1 CLI row, grammar closed at three words, `door` writes no record
2. escalate (check 1, E12 dependent) — §4 CLI / main binary row (`:192`)
3. escalate (check 1, E13) — §4 Scenario P3, the door carrier reached two ways
4. escalate (check 1, E13 dependent) — §1 must-trace P3 row (`:109`)
5. escalate (check 1, E13 dependent) — §4 Scenario P3, Surfaces involved
6. escalate (check 1, E13 dependent) — §3 → Snapshot / paste-to-AI integration (key file)
7. escalate (check 1, E13 dependent) — §11 MCP double-gate ban (`:662`)
8. escalate (check 1, E14) — §11 stdout ban reaches the relay (`:658`)
9. escalate (check 1, E14 dependent) — §3 → Logging stack (key file)
10. escalate (check 1, E14 dependent) — §1 Logging stack (`:76`)

design-system: 0 proposals · fanout-design-system.md
note: no new UI element (`tokens n/a`); no status claim touched.

layout-templates: 0 proposals · fanout-layout-templates.md
note: no new surface or region; the MCP toggle rows describe the sidecar's toggle, unchanged.

a11y-plan: 0 proposals · fanout-a11y-plan.md
note: no interactive element; schema unmoved; `a11y-plan.md:15` quotes obs-plan §1's surface count — no obs proposal
moves that count (re-checked at the cascade sweep).

## Checks
- check 2 (cross-contradiction): none — no two proposals edit one section in opposing directions.
- check 3 (intent-consistency): the scope record holds 0 lines; the report's deviations (plan steps 3, 5, 6, choices the
  plan did not settle) are justified and step 6 was accepted as built by the operator (inputs#I5). No unjustified
  divergence.
- check 4 (absence): security 10's "no such document" rests on research's `git grep -l -i 'Using MCP with
  andromeda-pulse'` (report, inputs#I4 row 2), its hits only 2026-05-02 security run files.
- check 5 (expected amendments): E1 arch 1 · E2 arch 12 · E3 arch 3 · E4 arch 4–7 · E5 arch 11 · E6 security 1 · E7
  security 5 · E8 security 8 and 9 · E9 test 1 · E10 test 4 (and arch 13, the CI key file the entry names as a site) ·
  E11 test 6 · E12 obs 1 · E13 obs 3 · E14 obs 8. All fourteen proposed; none raised by main. No `claim false` row in
  `citation-dispositions.md`.
- check 6 (disproved claims): plan step 6 → security 8 and test 4 state the relay arm's reading as built (exit 1 with
  the fixed line, no byte to another account, one `door.refused` per reading that reached the door); plan step 5 → no
  master states a count of four (obs-plan states one `door.call` per answered request) — DISPOSED, a chunk artifact,
  the report's deviation; research's two-caller count → no master states it — DISPOSED, a chunk artifact.

## Escalation resolved
The halt (one text stop, three asks) was answered by the operator as the founder's delegate, PROVISIONAL with the
door's one item (inputs#I7, verbatim in `relay-1.md`): (1) the fourteen groups applied as recommended — every
`escalate` line above → apply, re-derived from the report; (2) arch 16–18 applied ("one engine process per program",
the `door` word as short-lived relay processes hosting no engine part); (3) the playbook rule approved as written and
appended (`.andromeda/playbook.md`, the variable-switching-a-widened-surface discriminator). Dialogue rounds: 1.
Totals: 45 applied (42 detector proposals + 3 main-raised) · 0 rejected · 43 escalated and resolved · 2 routine.
