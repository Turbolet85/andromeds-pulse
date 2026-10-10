# Fan-out results — 2026-10-10-no-gate-stands-while-reading-nothing

Seven doc-agents, one batch, each with its doc, the report and its detectors (19 detector ids over the seven
prompts: arch 2 · security-plan 4 · design-system 2 · layout-templates 2 · test-plan 3 · obs-plan 4 · a11y-plan 2).
The return transport delivered each list as text; no HTML entity stood in any value (`&lt;` / `&gt;` / `&amp;`: 0
occurrences in the three carrying returns).

**A limit of this record:** for the three carrying docs each proposal is kept with its `detector`, `severity`,
`section`, `basis`, `dependent-of` and its `change` as returned; `sidecar` is not kept (the entries are written from
the verified bodies) and `rationale` is kept as the report facts it cites. The four `proposals: []` returns carried
comment lines that stripping removed, so each has its raw twin beside this file.

## Verdict lines

- architecture — 4 proposals (preamble: none).
- security-plan — `proposals: []` · stripped: per-detector reasons and three adjacent claims read true (`.raw-fanout-security-plan.md`).
- design-system — `proposals: []` · stripped: per-detector reasons (`.raw-fanout-design-system.md`).
- layout-templates — `proposals: []` · stripped: per-detector reasons (`.raw-fanout-layout-templates.md`).
- test-plan — 15 proposals (preamble: none).
- obs-plan — 7 proposals · stripped preamble: three detectors read no drift; four passages read and left alone (§9 Pipeline integration `xtask test` row `obs-plan.md:508`, already true; §3 → Heartbeat ticks key; the §1 frame trigger and §8 `ui.webgpu.adapter` leaf; §5's post-test analysis rationale).
- a11y-plan — `proposals: []` · stripped: per-detector reasons and a 0-occurrence sweep of its doc and ten key files (`.raw-fanout-a11y-plan.md`).

Total: 26 proposals · 0 rejected for a source the report does not carry (every coordinate is a line the report
states or a row of its `## New text, by line`).

## Validate — what decided each

- **Check 1 (playbook).** *Accurate this-chunk addition* (apply) governs the proposals that bring a body to what the
  chunk shipped. *Registry over-reach* does not match A2 and A3: its clause "a registry that tracks its subject at
  CATEGORY grain" fails — `architecture.md:260` enumerates each xtask verb that carries a formal exit contract, one
  entry per verb. The 2026-08-14 rule (a doc claim a pre-existing reality falsifies, the impl correct, one artifact)
  governs O5. The 2026-08-28 rule (recording a measured OPEN gap with a named owner) governs O4 and O6; its owner is
  a route annotation, pinned at P5 at the route-resolve card, where the operator's directive sends the question
  (inputs#I3 item 5). The branch-threshold retirement (T1 to T6) is a reversal of a stated threshold, the class
  *Accurate this-chunk addition* excludes: it was put to the operator at the plan's P4 dialog and answered
  (inputs#I2 item 1), and its wording is directed (inputs#I3 item 2) — PROVISIONAL, awaiting the founder's own word.
  No boundary widening: no proposal records a new crossing of a hardened boundary.
- **Check 2 (cross-contradiction).** None. T9 and T10 edit one row, both additive; A1 to A3 edit one registry line,
  additive.
- **Check 3 (intent-consistency).** The report matches the entry and the plan's criteria; its deviations were
  accepted by the operator as recorded (inputs#I4). The scope record holds no line.
- **Check 4 (absence needs evidence).** The report's sweeps carry pattern, hit count and a disposition per hit. Each
  site on a line over 2 000 chars was read by offset before its edit (`architecture.md:260` at char 26646;
  `obs-plan.md:545`, `:559`, `:499`; `test-plan.md:490`, `:496`, `:509`, `:135`; the key file's line 32).
- **Check 5 (expected amendments).** All five plan entries are covered: 1 → T1 to T6 · 2 → T8, T9 · 3 → T11, T12,
  T14, T15 · 4 → O1 to O7 (`obs-plan.md:508` read true by the detector, no edit) · 5 → A1 to A4. Entry 5's third
  part, the coverage job's name "where the key file states it": read — `registries/contracts/architecture/
  ci-cd-approach.md` names the job by its key `coverage` three times and never by its display name, so nothing there
  is retired. `citation-dispositions.md` holds no `claim false` row.
- **Check 6 (disproved claims).** 1 → T1 to T6 · 2 → T8, T9 · 3 → O4 · 4 → O6 · 5 → O1 · 6 → O2 · 7 → O3 · 8 → O7 ·
  9 → T11, T12, T13, T14 · 10 → A1, A2 · 11 → T15 · 12 → a chunk-artifact precision: the report entry is its record,
  no edit.
- **A report correction made at Validate.** Disproved-claim 3 said "no step of `lint-test` reads a log". The job's
  `perf:budget` step reads the perf-samples producer's log for its budget arms. The bullet now says no step of
  `lint-test` makes that check; O4's applied text is re-derived from the corrected fact.

## architecture — 4 proposals

- **A1** · D-arch-resources · warning · §Occupied Resources → xtask CLI surfaces (dev/CI gates) · basis `.andromeda/architecture.md:260`
  change: in the `cargo xtask perf:budget` entry, the sentence "`cargo xtask ci-gates` and `perf:load-profiles` run the same grader in-process with no arm required (`ci-gates: perf-budget PASS|FAIL|NEUTRAL`; no log → `perf-budget NEUTRAL (no log file to grade)`)" becomes: `cargo xtask perf:load-profiles` alone runs the same grader in-process with no arm required; `cargo xtask ci-gates` runs no grader and prints no perf-budget, frame or heartbeat line since this chunk.
  rationale: report disproved-claim 10; Symbols; Counts (callers two → one).
  → **apply** (check 1, accurate this-chunk change; check 6).
- **A2** · D-arch-resources · warning · the same section · basis `xtask/src/ci_gates.rs:20-37`
  change: add an entry for `cargo xtask ci-gates` — the obs-log reading gate at `xtask/src/ci_gates.rs`: two arms, zero-spans and zero-panic, over the `agent-latest.jsonl*` family under the resolved log dir; exit 0 PASS with exactly the two lines · exit 1 FAIL (zero records, or a panic record named by member file name and line) · exit 2 cannot-evaluate (log dir absent or holding no member); closed labels, counts, a file name and a line number only; no heartbeat, perf-budget or frame line; callers the `boot` job's step and `pre-push:linux`'s sixth stage.
  rationale: report Symbols (the new module and the changed contract); Counts (arms four → two; exits `0 · 1` → `0 · 1 · 2`).
  → **apply** (checks 1 and 5; the proposal's "the last step of the `boot` job" is not applied — the upload follows it).
- **A3** · D-arch-resources · warning · the same section · basis `xtask/ci/quarantine-tracking-check.sh:27-32`
  change: add an entry for `cargo xtask quarantine-tracking` — the script pair `xtask/ci/quarantine-tracking-check.{sh,ps1}`: a missing search dir → exit 1; a scan of zero `.rs` files → exit 1; an untracked `#[ignore]` fails; zero `#[ignore]` → `PASS (0 quarantine(s) across {N} file(s))`; tracked ones → PASS with the file count; no NEUTRAL arm; the `.sh` held by five pins, the `.ps1` parsed and never run.
  rationale: report Symbols (the verb's contract); Coverage (`.ps1` tests ✗).
  → **apply** (checks 1 and 5).
- **A4** · D-arch-decisions · warning · §Stack and Technologies · basis `pulse-app/tests/quality_gate_workflow.rs:757-829`
  change: add a test-time row for `bash` and `awk` on PATH beside the Python 3, `cc` and `node` rows: the thresholds witness (four tests) and the five quarantine pins run the gates' own scripts; a missing tool fails them, never skips them; no manifest or lockfile change.
  rationale: report Harness / gate surface, New pins.
  → **apply** (checks 1 and 5).

## obs-plan — 7 proposals

- **O1** · D-obs-defect-narrative · warning · §10 → CI gates, the perf-budget bullet · basis `xtask/src/ci_gates.rs:93-128`
  change: "`ci-gates` grades the boot-smoke log with no arm required and prints `perf-budget NEUTRAL` over it" and every present-tense boot-smoke `ci-gates` line reading give way to: since this chunk the verb grades no perf arm and prints no perf-budget or frame line; the frame `cannot-evaluate` line on CI is the lint-test `perf:budget` step's alone, read on `ci#38042949735`; the earlier boot-smoke-line readings stay as dated history; "the lint-test `perf:budget` line was not re-read" is dropped.
  rationale: report disproved-claim 5; Symbols; Counts.
  → **apply** (checks 1, 5, 6).
- **O2** · warning · §10 → Performance budgets, the frame row · basis `xtask/src/main.rs:488-489` · dependent-of D-obs-defect-narrative
  change: the row's `ci-gates` readings are the verb's behaviour before this chunk; "the step's order makes that the expected line …" gives way to: since this chunk the boot job's `ci-gates` step prints no frame line on any run; the log still holds the adapter records, not re-read on `ci#38042949735`.
  rationale: report disproved-claim 6.
  → **apply** (with O1).
- **O3** · warning · §10 → Load-profile constraints · basis `xtask/src/main.rs:715` · dependent-of D-obs-defect-narrative
  change: "`ci-gates` and `perf:load-profiles` grade with no arm required" becomes `perf:load-profiles` alone; the NEUTRAL-tolerance rule is scoped to the perf grader's unrequired arms, and `ci-gates` is not such a check: an absent log family is exit 2.
  rationale: report disproved-claim 7; Counts.
  → **apply** (with O1).
- **O4** · warning · §10 → CI gates, the zero-span bullet · basis `xtask/src/ci_gates.rs:97-99`
  change: "enforced by `xtask test` step exit code validation" gives way to what was measured: `cargo xtask test` returns nextest's own status and reads no log; the one check of the kind is the `zero-spans` arm of `ci-gates` in the `boot` job, counting log records of any target; zero records → exit 1, no family member → exit 2 (110 records on `ci#38042949735`); the bullet's original reading stands unmet, its owner named at the route-resolve card.
  rationale: report disproved-claim 3 (as corrected above); inputs#I3 item 5.
  → **apply as measured** (check 1, the 2026-08-28 rule; the owner is P5's pin).
- **O5** · warning · §10 → CI gates, the zero-panic bullet · basis `xtask/src/ci_gates.rs:85-91` · dependent-of D-obs-defect-narrative
  change: "enforced by post-test `jq … | grep -q .` check" gives way to: enforced by the `zero-panic` arm of `ci-gates` in the `boot` job, read in-process over the boot log's family; an `app.panic.fatal` record at ERROR → exit 1 with the file-name-and-line line; otherwise `ci-gates: zero-panic PASS`.
  rationale: not among the report's disproved claims; rests on its Symbols bullet and its Outcome (obs). Re-derived by the orchestrator: the `jq` step exists in no workflow (`grep -c 'jq' .github/workflows/ci.yml`: 0), and the arm read in-process before this chunk as after it.
  → **apply** (check 1, the 2026-08-14 rule: the impl is correct, the doc alone misstates it, one artifact).
- **O6** · warning · §10 → CI gates, the heartbeat bullet · basis `xtask/src/main.rs:741`
  change: "Build fails if heartbeat tick stalls detected … enforced by post-test gap analysis" gives way to: no CI step makes the gap check since this chunk; before it the only one was `ci-gates` in the `boot` job, over one tick per target; the scripts stay, called by `perf:load-profiles` alone, which no workflow runs; the > 45 s rule stays the stall signal and its CI enforcement stands unmet, its owner named at the route-resolve card.
  rationale: report disproved-claim 4; Counts; inputs#I3 item 5.
  → **apply as measured** (check 1, the 2026-08-28 rule; the owner is P5's pin).
- **O7** · warning · §9 → Telemetry artifact handling, the Log file row · basis `xtask/src/ci_gates.rs:93-128` · dependent-of D-obs-defect-narrative
  change: after "the one `ci-gates` reads in that job", what the verb reads of it since this chunk: the record count and the panic read.
  rationale: report disproved-claim 8; the plan's expected amendment 4 names the line.
  → **apply** (check 5).

## test-plan — 15 proposals

- **T1** · D-tests-coverage · warning · §4 → Coverage target · basis `.andromeda/test-plan.md:194`
  change: ≥ 75 % line, ≥ 85 % function; the ≥ 70 % branch threshold retired as never measured, PROVISIONAL, awaiting the founder's own word (the operator's reading, the pc overseer, 2026-10-10), with its cause (a nightly-only compiler feature, the pin `1.95.0`, no branch count asked) and what a real count would need; the rest of the sentence stays.
  → **apply** (the operator's recorded direction, inputs#I2 item 1 and inputs#I3 item 2).
- **T2** · warning · §9 → Pipeline structure, the Quality gates row · basis `:501` · dependent-of D-tests-coverage
  change: assert coverage ≥ 75 % line / 85 % function, no branch threshold; a report tracking 0 lines or 0 functions fails, and an absent `lcov.info` fails; assert zero flaky tests.
  → **apply** (with T1).
- **T3** · warning · §9 → Build failure conditions · basis `:510` · dependent-of D-tests-coverage
  change: line < 75 % OR function < 85 %, OR an `lcov.info` absent or tracking 0 lines or 0 functions.
  → **apply** (with T1).
- **T4** · warning · §10 → Coverage thresholds table · basis `:542` · dependent-of D-tests-coverage
  change: the Standard row states two enforced thresholds; the branch cell reads not enforced, retired, PROVISIONAL.
  → **apply** (with T1).
- **T5** · warning · §10 → Build failure conditions · basis `:585` · dependent-of D-tests-coverage
  change: coverage below threshold (75 % line, 85 % function), or a report that tracks nothing.
  → **apply** (with T1).
- **T6** · warning · §3 → Bootstrap phases, item 8 · basis `.andromeda/registries/contracts/test-plan/bootstrap-phases.md:10` · dependent-of D-tests-coverage
  change: the workflow enforces line ≥ 75 % and function ≥ 85 %; the branch arm retired, PROVISIONAL.
  → **apply** (with T1; a key file).
- **T7** · D-tests-coverage · warning · §1 → Pending coverage triggers · basis `xtask/ci/quarantine-tracking-check.ps1:18-23`
  change: a new row `quarantine-tracking-ps1-mirror-coverage`: the `.ps1` mirror gained the absent-input arms with no committed test; parsed by `pwsh` 7.6.6 (0 parse errors), never run; owed a pin per arm, or the mirror leaves with the other-systems entry.
  → **apply** (check 1, accurate this-chunk addition; the section's own precedent row is `l4-latency-p99-ps1-run-coverage`).
- **T8** · D-tests-framework · warning · §9 → Pipeline structure, the MCP-feature tests row · basis `:492`
  change: the command spells `--no-tests=fail`; a selection matching no test fails the job.
  → **apply** (checks 1, 5, 6).
- **T9** · warning · §9 → Pipeline structure, the `lint-test` row · basis `xtask/src/main.rs:430-439` · dependent-of D-tests-framework
  change: the perf-samples command spells `--no-tests=fail`; `cargo xtask test` and `cargo xtask perf:slo-load` each pass nextest the flag from a pinned argument list; the step is named `cargo xtask test`.
  → **apply** (with T8).
- **T10** · D-tests-framework · warning · the same row · basis `pulse-app/tests/quality_gate_workflow.rs:757-829`
  change: the row's list of what the workspace tests need on the runner gains `bash` and `awk` on PATH, with the fail-never-skip clause and the run that read them green.
  → **apply** (check 1; the row enumerates each such need).
- **T11** · D-tests-obs-harness · warning · §9 → Pipeline structure, the Boot smoke row · basis `:496`
  change: where the row names `cargo xtask ci-gates` over the log the smoke wrote, it states the two arms, the exits 0 · 1 · 2 and that no heartbeat, perf-budget or frame line is printed.
  → **apply** (checks 1, 5, 6).
- **T12** · warning · §1 → Coverage triggers, the WebGPU canvas throughput row · basis `:100` · dependent-of D-tests-obs-harness
  change: on CI the unrequired frame arm's `cannot-evaluate` line is the `lint-test` budget step's alone; the boot job prints no frame line on any run; the earlier boot-job readings stand as dated.
  → **apply** (with T11).
- **T13** · warning · §1 → Pending coverage triggers, the `perf-slo-check-arm-coverage` row · no basis line · dependent-of D-tests-obs-harness
  change: the clause "the boot job's line reads `… no WebGPU adapter (no_navigator_gpu)`" is bounded to before this chunk.
  → **apply** (with T11; the site read at `test-plan.md:135`).
- **T14** · warning · §9 → Build failure conditions · basis `:509` · dependent-of D-tests-obs-harness
  change: where it runs, the `boot` job also fails when `cargo xtask ci-gates` is non-zero: exit 1, or exit 2 on an absent log family.
  → **apply** (with T11).
- **T15** · warning · §3 → Per-chunk gate discipline · basis `.andromeda/registries/contracts/test-plan/per-chunk-gate-discipline.md:32` · dependent-of D-tests-obs-harness
  change: in the `pre-push:linux` paragraph the sixth stage reads two lines over its seed and makes no heartbeat or perf-budget read; the `test` stage's `cargo xtask test` exits non-zero on a selection that matches no test.
  → **apply** (with T11; a key file).

## Escalations

None halted this phase. Two amendments (O4, O6) state a CI reading as unmet and leave its owner to the route: the
question goes to the route-resolve card, where the operator's directive places it (inputs#I3 item 5).
