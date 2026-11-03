# Fan-out results — 2026-11-03-advisories-since-2026-10-12-closed

Batch stage1 · `fanout.py check`: 7 of 7 pass · `proposals: architecture 10 · security-plan 14 · design-system 0 · layout-templates 0 · test-plan 4 · obs-plan 0 · a11y-plan 0`. Every master whole (`detail.py check`: 0 category files), so no cascade and no sampler (`sampler: off`). Citation sweep: `citation-dispositions.md` (1 `claim false — raised at Validate`, 2 `curation home — P3`).

- architecture: 10 proposals · fanout-architecture.md
  note: D-arch-resources clean (no IPC method, port, socket, env var or workspace crate); Retired none; Stack rows naming Tauri 2.x, tauri-plugin-notification 2.x and tauri-plugin-updater 2.x stay true.
- security-plan: 14 proposals · fanout-security-plan.md
  note: the zip and npm proposals carry D-security-deps' escalate severity with no banned dependency added; the door proposals read inputs#I5 as covering the door's whole item (the sixth-stage, exit-witness and test-port PROVISIONAL items left alone); fct-hhbgn5ow and fct-wnflmema left as history / true.
- design-system: 0 proposals · fanout-design-system.md
  note: no UI element; the Tauri bumps leave «Tauri 2.x» true.
- layout-templates: 0 proposals · fanout-layout-templates.md
  note: dependency/config-only chunk; no surface status touched.
- test-plan: 4 proposals · fanout-test-plan.md
  note: D-tests-coverage and D-tests-obs-harness hold; the libduckdb-sys / tao readings at fct-szwcsgq3, fct-gdjnsjph, fct-paxx23uc left as dated history.
- obs-plan: 0 proposals · fanout-obs-plan.md
  note: no new symbol, hot-path op or telemetry library; fct-5ajyzr6y and fct-uk5hczxe history.
- a11y-plan: 0 proposals · fanout-a11y-plan.md
  note: no UI element, no a11y tool moved; fct-rujmfwpg true.

## Dispositions

fanout-architecture.md
1. apply — check 1: rul-zkjjanin (this chunk's wasmtime / cranelift / pulley bump, report Dependencies; the plan's expected amendment, architecture §Stack); check 5: the citation sweep's `claim false` row on fct-wn2ucoyt (`Cargo.lock:1394`, «version = "0.135.5"») is settled here · as: the row's Cranelift qualifier reads «Cranelift 0.135.8, as measured at `Cargo.lock:1403`» (the report's New text, Cargo.lock added 1403 — cranelift-codegen 0.135.8); requirement `"48.0.8"`, lockfile-resolved 48.0.8; the bump from 48.0.5 closed this chunk's wasmtime ids with no advisory ignore; the earlier bumps (48.0.3 closing RUSTSEC-2026-0325 / -0326 / -0327, 46.0.3 closing RUSTSEC-2026-0316) and the 49.x-at-1.95 clause stand
2. apply — check 1: rul-zkjjanin (report Dependencies and Outcome: toolchain 1.95.0, `declared_floor_equals_the_pinned_channel` passes on 48.0.8); the 28-package count kept as its 48.0.5 reading, not re-measured
3. apply — check 1: rul-zkjjanin (report Dependencies; plan's expected amendment [Plugin Runtime])
4. apply — check 1: rul-zkjjanin (report Dependencies; plan's expected amendment §Inherited Defaults)
5. apply — check 1: rul-zkjjanin (report Dependencies: duckdb 1.10505.0 → 1.10506.1, requirement unchanged, inputs#I2 item 1; plan's expected amendment)
6. apply — check 1: rul-zkjjanin (as 5, [Database])
7. apply — check 1: rul-zkjjanin (as 5, §Inherited Defaults)
8. apply — check 1: rul-zkjjanin (report Dependencies tao 0.35.0 → 0.37.2; Counts `tauri-ver`: the mechanism measured on 0.35.x and 0.37.1, the boot series and CI boot smoke green on 0.37.2 — the measured boundary named, never generalized)
9. apply — check 1: rul-zkjjanin (report Counts `tauri-ver`: tauri-utils 2.9.0 → 2.10.2, the default not re-read — no bundle built)
10. apply — check 1: rul-zkjjanin (report Counts `tauri-ver`: Tauri 2.11 → 2.12.5, the rejection message not re-probed at 2.12.5)
11. apply — raised by main, check 5 (Q1 resolved: inputs#I6, the founder's 2026-11-02 confirmations of shaping-0.4.0 items 27 and 28, read against the source by the overseer) · §Occupied Resources (Network ports, the door; fct-twrv2qiu @2325) · the door's item sentence: «**PROVISIONAL — a boundary widening, one item:** … are the operator's answers (…); the founder's own word is owed on the surface itself, and nothing of it is ratified» and «…; the founder's own word is still owed and nothing of it is ratified» · as: the sentence keeps the surface it lists, its status ratified — opened «**A boundary widening, one item, ratified:**», then the endpoint, the owner check, the reach of the nine tools and of the one write, the write-switch variable and the workflow step that runs a client as a second account, and (since chunk 2026-10-12-stdio-agent-reaches-the-door) the console program's third word `door` with its parenthesis and the step's second arm; the operator-answer provenance and both «owed / nothing of it is ratified» clauses leave the body (the ratification and the operator's answers go to the sidecar entry); «Every other statement of the door in this document rests on this one.» stays; the pre-push sixth stage's PROVISIONAL at @4291 on the same fact stays
12. apply — raised by main, check 5 (Q1, inputs#I6) · the door's PROVISIONAL pointer marks on architecture fct-rjyoe4ax (§Design Philosophy), fct-bka6rc62 ×2, fct-wdpv3pq2, fct-rkbubyxp (§Stack and Technologies), fct-rsvfsams, fct-syp7tq5s, fct-wzk2x6ka, fct-3k7t2g4l (§Established Decisions), fct-ob4ts4xm (§Conventions), fct-qsvopkhj (§Standard Contracts), fct-4nu3wh7b, fct-hgj34est, fct-wuyupllp ×2, fct-ulzkjofc, fct-g7bqq44n (§Occupied Resources), fct-r6qmu7wo (§Project Intent), fct-x56az6wm (§Inherited Defaults), and the key files architecture/contracts/ci-cd-approach fct-z32zdd7c and architecture/contracts/deployment-model fct-35cgfrz4 · as: drop each «PROVISIONAL — …(the door | Network ports)» mark with its own separator (a `; ` inside a parenthesis, a standalone closing sentence, or the dashes around it), leaving the rest of each fact byte-identical; a parenthesis left holding only a pointer keeps the pointer; architecture fct-n5rdzzzk (@15512, @38857, @39257 — the pre-push sixth stage's item), fct-twrv2qiu @4291 and fct-finu2rme (the test-support edge) stay PROVISIONAL

fanout-security-plan.md
1. apply — check 1: rul-gwkpiogw (D-security-deps' escalate severity outside its class: no banned or unvetted dependency added — `cargo deny check bans licenses sources` green, report Outcome; the actual class is rul-afnbmtvl, a doc claim a pre-existing reality falsifies with nothing else to fix); check 6: the report's first `Spec claims disproved` entry disposed; the plan's expected amendment
2. apply — check 1: rul-gwkpiogw (no dependency added, the three exceptions on the founder's word, inputs#I2 item 4; actual class rul-zkjjanin, this chunk's policy-file change); the plan's expected amendment
3. escalate → Q1 — check 1: rul-cei62g55 (a boundary widening's ratification: dropping the door's one PROVISIONAL item across the plan reads inputs#I5's «the door's own surface (item 27)» as the whole item; whose word and its reach are the operator's) · resolved: apply (Q1) · as: the item sentence keeps the surface it lists, its status ratified — opened «**A boundary widening, one item, ratified:**» with its components (the console program's word `door` and the step's second arm included); the operator-answer provenance and every «owed / nothing of it is ratified» clause leave the body (the ratification — the founder, 2026-11-02, shaping-0.4.0 items 27 and 28, inputs#I6 — and the operator's answers go to the sidecar entry)
4. apply — check 1: no rule; the operator's recorded direction settles it (inputs#I5, the report's Addendum 1: «For fct-6geshnvr … its PROVISIONAL mark drops too»; the founder's word of 2026-11-02 relayed verbatim, «Подтвердить как есть»); applies on that direction whatever Q1 rules for the group it is dependent of
5. escalate → Q1 — as 3 (fct-gn744k3z) · resolved: apply (Q1)
6. escalate → Q1 — as 3 (fct-lwwyrpa6) · resolved: apply (Q1)
7. escalate → Q1 — as 3 (fct-ty7icrgc, the door-switch mark only) · resolved: apply (Q1)
8. escalate → Q1 — as 3 (fct-j4oj6ftt, the door's three marks only) · resolved: apply (Q1)
9. escalate → Q1 — as 3 (fct-inw2zo7r) · resolved: apply (Q1)
10. escalate → Q1 — as 3 (fct-ashdoovw) · resolved: apply (Q1)
11. escalate → Q1 — as 3 (fct-xsadt7mu) · resolved: apply (Q1)
12. escalate → Q1 — as 3 (fct-ixrmtmoe, the second-account step only) · resolved: apply (Q1)
13. escalate → Q1 — as 3 (fct-cvhwrox6) · resolved: apply (Q1)
14. escalate → Q1 — as 3 (fct-y6svlsyp) · resolved: apply (Q1)

fanout-test-plan.md
1. apply — check 1: rul-zkjjanin (report Dependencies: vitest / @vitest/* 4.1.11 → 4.1.12, package.json unchanged; Counts `npm-pkgs`)
2. apply — check 1: rul-zkjjanin (as 1, §1 row)
3. apply — check 1: rul-zkjjanin (report Dependencies wasmtime / pulley / cranelift; plan's expected amendment test-plan §4 Framework); the 28-package count kept as its 48.0.5 reading
4. apply — check 1: rul-zkjjanin (as 3, §5 plugins row; plan's expected amendment)
5. apply — raised by main, check 5 (Q1, inputs#I6) · §9 CI Integration (fct-ep45valw @3031) · the door's item sentence «**The door is PROVISIONAL — a boundary widening, one item:** …» · as: the sentence keeps the surface it lists, its status ratified — «**The door is a boundary widening, one item, ratified:**» with its components (the relay word and the step's second arm included); the operator-answer provenance and every «owed / nothing of it is ratified» clause leave the body (the ratification and the operator's answers go to the sidecar entry)
6. apply — raised by main, check 5 (Q1, inputs#I6) · §1 Test Scope Summary (fct-mqmrzd2o @3246, @3313) and the key file test-plan/contracts/per-chunk-gate-discipline (fct-b7qfcqmm @5384, @6335) · as: «(§9, which holds the door's PROVISIONAL sentence; the relay word and the step's second arm are PROVISIONAL with it)» → «(§9, which holds the door's item sentence)»; «(test-plan §9, which holds the door's PROVISIONAL sentence)» → «(test-plan §9, which holds the door's item sentence)»; «(PROVISIONAL with the door's item)» dropped with its parenthesis; the rest of each fact byte-identical; the pre-push sixth stage's PROVISIONAL (fct-sxi5nmit, fct-5zpq3w5y) and the branch threshold's (fct-oob3e6vm, fct-o2tgnued, fct-w46dlsao, fct-tzbooaje, bootstrap-phases fct-q32cp4j7) stay

fanout-obs-plan.md
1. apply — raised by main, check 5 (Q1, inputs#I6) · §4 Scenario P3 (fct-4vrfwv27 @1947) · the door's item sentence «**The door is PROVISIONAL — a boundary widening, one item:** …» · as: the sentence keeps the surface it lists, its status ratified — «**The door is a boundary widening, one item, ratified:**» with its components (the relay word and the step's second arm included); the operator-answer provenance and every «owed / nothing of it is ratified» clause leave the body (the ratification and the operator's answers go to the sidecar entry)
2. apply — raised by main, check 5 (Q1, inputs#I6) · the door's PROVISIONAL pointer marks on obs-plan fct-c6we2ewi, fct-sjvcbexl (§1 Obs Scope Summary), fct-rq75gb6d (§4 Span / Trace Coverage), fct-fzliht4o (§6 Log Coverage), fct-liu5ghst (§PII Scrubbing), fct-anuozml7 (§Universal), fct-udg32czu (§Project-specific) and the key file obs-plan/contracts/logging-stack fct-65zmgzbk · as: drop each «PROVISIONAL — §4 Scenario P3» / «PROVISIONAL with the door's item» mark with its own separator, the rest of each fact byte-identical; «§4 Scenario P3, which holds the door's PROVISIONAL sentence» → «§4 Scenario P3, which holds the door's item sentence»; the pre-push sixth stage's PROVISIONAL on fct-bsv3q3lr (@1693, @1960) stays

## Check 5 — the plan's expected amendments, each against the set
- architecture §Stack / §Inherited Defaults / [Plugin Runtime] (wasmtime, cranelift) — architecture 1–4.
- architecture DuckDB 1.10506.1 — architecture 5–7.
- architecture Tauri 2.12.5 where a version is named — architecture 8–10.
- security-plan §Dependency Security pinned-version sentences (rustls 0.23.46, quinn-proto 0.11.19, wasmtime 48.0.8) — no change: the one site (fct-wnflmema, security-plan:222) is the 2026-09-29 discharge's history, true by its own words («At that chunk … exits 0 after the rustls 0.23.45 … bumps»); its standing rules (a plain pass/fail gate, distinct ids, visible dispositions) hold at this chunk (gate `cargo audit` exit 0); this chunk's bumps are provenance, carried by the sidecar entry of security-plan 1 / 2, never the body (amendment-flow §Apply, rule 1).
- security-plan the three zbus-family ignores and the three npm exceptions — npm: security-plan 2; zbus: no master site (pattern `zbus-ign` 0, control unfired) — the body states the ignore rule generically (fct-wnflmema's last sentence: only no-safe-upgrade findings take an ID-scoped ignore), which the three entries meet (report Schema / config: holders, owner, closing condition); no change.
- security-plan the pruned GTK3 / unic-* ignores and the pruned skips — no change: `gtk3-unic-ign` 0 master sites; `deny-skips` 2 sites both true — fct-hhbgn5ow's two carve-outs (core-foundation + security-framework, ureq) still stand and no skip was added; fct-zijlizho generic.
- security-plan Windows-target duplicate skips — superseded (report: no skip added, the notify-rust move); nothing to amend.
- security-plan §Security Anti-Patterns → Input, the zip sentence — security-plan 1.
- test-plan §4 / §5 wasmtime / cranelift / pulley — test-plan 3, 4.
- Wrap-only freight, carry `cry-rdhtyshr` rev 3 — P2: security-plan 4 (fct-6geshnvr) and Q1; the leaf items at the cascade; P3 / P6 / evolve at their phases.

## Check 6 — disproved claims
- security-plan zip sentence (fct-tg5z4vbe) — security-plan 1.
- `cargo deny check advisories` designed-red — no master states it (report); routed: the Tier 2 line `.claude/rules/security.md:180` → P3 curation; the base rule `rul-ax3cufzy` (status superseded, history) → P3's judgment; the leaf `.claude/docs/commands.md:103` → the cascade's leaf comparer (step 3, the carry adds commands.md to the set).
- research's «19 ids» and the plan's step-6 forecast — chunk-artifact claims, report entries, no edit.

## Checks 2, 3, 4, 7
- check 2: no two proposals edit one section in opposing directions.
- check 3: the report's one deviation (notify-rust in place of eight skips) carries the overseer's word (inputs#I3); scope record: none (`gate.py scope` clean, 0 recorded).
- check 4: no proposal claims an absence or a caught-ALL beyond the report's sweep rows.
- check 7: no extracted doc — not applicable.

Rejected for a source the report does not carry: 0.
