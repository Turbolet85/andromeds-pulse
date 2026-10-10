# Scope — No gate stands while reading nothing

**Marker:** `2026-10-10-no-gate-stands-while-reading-nothing` · version `andromeda-pulse-0.4.0` · Epoch 1 — Foundation: base CI, the capability record, a console engine with its door, its gates
**Working entry (`working-route.md:25`):** "No gate stands while reading nothing — coverage branch and empty-report
arms, empty test selections, boot budget and heartbeat reads, zero-span check: each reads something or leaves (P-128)"
**Chunk base:** `7419496b` (HEAD at take-up). Every diff-shaped probe of this chunk names this sha, never HEAD.

## Intent
A gate of the `ci` workflow that prints PASS or NEUTRAL over an empty or absent input passes while it reads nothing.
The last chunk settled every baseline comparison and every upload; five gates of another shape were recorded there
and not edited. This chunk leaves each of them in one of two states: it reads something that can make it fail, or
it is gone. There is no third state: a gate that prints NEUTRAL or PASS over nothing is the defect, not an option
(inputs#I1 item 2). With them settled, the closing sentence of P-128's acceptance is true and the capability is
claimed here.

## What the chunk builds
- **Each of the five carried gates reads something that can make it fail, or leaves.** The entry's first `CARRY:`
  names them with how each was read; every reading below enters as a hypothesis and is MEASURED at P3 before the
  plan chooses (inputs#I1 item 1: what the gate reads on a run where its input is there, and what it does when its
  input is empty or absent; "a gate described from its text is not yet a reading"). Coordinates were re-read at
  take-up on `ci.yml` and `xtask/src/main.rs` at `7419496b`.
  - (1) "the coverage thresholds step exits 0 when lcov.info tracks 0 lines and 0 functions (read from
    the step's text; the arm did not fire on either run read)". Job `coverage`, step `Enforce coverage thresholds`,
    `ci.yml:519-522`; its message reads "Foundation epoch — gate trivially passes; activates at chunk #15 first
    tests" (`:520`). Verified at P3: the step body run over an empty report exits 0 with that line, and over no
    report exits 1; on `ci#38038281709` the arm did not fire (38252 lines, 4055 functions). The percentages also
    print `100.0` over a zero total (`:523-525`), a second pass over nothing in the same step (research.md).
  - (2) [premise-corrected: the uploaded report holds 144 `BRF:0` and 144 `BRH:0` records and no `BRDA:` line
    (artifact 11664724073 of `ci#38038281709`): a zero total, not an absent record.] The branch arm read
    `Branch: 0/0 = 100.0% (threshold 70%)` on `ci#38038281709`, as on the two runs the entry names. The same
    step, `ci.yml:515-516` (the `BRF:` / `BRH:` sums), `:524` (a total of 0 prints `100.0`), `:527`, `:531`; the
    job's name states the branch threshold (`:450`). Why the total is zero, as research found: branch
    instrumentation is a nightly-only compiler feature, the project pins `1.95.0`, the runner used the pin, and
    `cargo xtask test:coverage` asks for no branch count. "a threshold that can only be met by a toolchain the
    project does not pin is said so, with its cost" (inputs#I1 item 4): the cost is in research.md, and what the
    workspace's branch coverage would read was not measured.
  - (3) "--no-tests=pass on the mcp-test job's nextest step and inside cargo xtask test and cargo xtask
    test:coverage (read from their text; no run was read with an empty selection)". `ci.yml:199`;
    `xtask/src/main.rs:440` and `:469`, with the two verbs' `about` strings at `:78` and `:86`; the `lint-test`
    step that runs the first verb is named "cargo xtask test (no-tests=pass at Foundation epoch)" (`ci.yml:111`).
    Verified at P3: the three selections held 2928, 2890 and 2890 tests on `ci#38038281709`; on the dev host an
    empty selection exits 0 with the flag and 4 without it (cargo-nextest 0.9.146; the runner installs 0.9.133,
    its default not read).
  - (4) "cargo xtask ci-gates in the boot job read "perf-budget: memory NEUTRAL — populated 0 of 1",
    "snapshot NEUTRAL" and "heartbeat-gap-check: max gap 0ms in (threshold 45000ms) PASS" on ci#38031822696, over
    a boot that lives about five seconds". Job `boot`, step `cargo xtask ci-gates`, `ci.yml:363-364`. Verified at
    P3: the same lines on `ci#38038281709`, with a fourth the entry does not name (`frame: cannot-evaluate: 0
    samples, no WebGPU adapter (no_navigator_gpu)`); a kept runner log of the step holds one tick per target, one
    memory sample and no snapshot record, so none of the three arms can fail there; the verb's `zero-spans` and
    `zero-panic` arms read the log's 112 records and fail on what they are for; over an absent log the verb exits
    0 with four NEUTRAL lines (research.md, the table).
  - (5) "obs-plan §10's "build fails if xtask test produces zero spans" has no step in lint-test, the
    zero-span check running only inside ci-gates in the boot job". Verified at P3; `cargo xtask test` reads no
    log, and the arm in `ci-gates` counts log records, not spans.
  - "(4) and the one home of (5)'s check sit in the boot job, which leaves at Window's gates retired;
    (1), (2) and (3) sit in jobs that stay". `Window's gates retired` is `working-route.md:48`. Verified at P3.
- **The two further `--no-tests=pass` literals are measured and settled with (3).** The entry's second `CARRY:`:
  "seen and not measured at 2026-10-10-no-ci-step-reads-nothing: two further --no-tests=pass literals
  in xtask/src/main.rs, in run_perf_slo_load (the verb the lint-test job's perf:slo-load step runs) and
  run_perf_load_profiles (not run in CI); what either selects on a run was not read". Read at take-up:
  `xtask/src/main.rs:749` and `:780`; the `lint-test` step is `ci.yml:115-116`. Verified at P3: the first
  selected 1 test on `ci#38038281709`; the second runs in no workflow and lists 4 tests on the dev host.
- **The a11y upload's path members.** The entry's third `CARRY:`: "the a11y job's a11y-violations upload
  has three path members and its if-no-files-found: error fails the step only when none matches, so one missing
  member is invisible (all three matched on ci#38031822696, 5 files); the a11y job leaves at Window's gates
  retired". Read at take-up: `ci.yml:258-268`, three members (`:264`, `:265`, `:266`). Verified at P3: 5 files
  again on `ci#38038281709`; each member's producer is a link of the `&&` chain inside `cargo xtask test:a11y`;
  the action's own text states its no-file rule for the path input as a whole; one member missing beside a
  matching one was not read on a runner. Whether a missing member is a gate "reading nothing" in the
  acceptance's sense, and what the job loses before it leaves, is part of the P4 dialog.
- **Found at P3, not stated by the entry, and brought to the same dialog:** `cargo xtask quarantine-tracking`
  (`lint-test`) prints NEUTRAL on every run and exits 0 over a root where none of its search dirs exists; the
  sixth stage of `cargo xtask pre-push:linux` runs `ci-gates` over one record the stage wrote itself (not a step
  of `ci.yml`). Every other gating step of the workflow read something on `ci#38038281709` (research.md, the
  census).
- **Which way each falls is the operator's technical fork, brought at P4 as ONE dialog** (inputs#I1 item 2): each
  with what repairing costs, what the gate is worth to the version as it will stand, and where it sits.
- **Where a gate sits decides half of it** (inputs#I1 item 3). Gate (4), the one home of gate (5)'s check and the
  a11y upload's path members sit in jobs that leave at `Window's gates retired`. For those the plan says what the
  job loses today if the gate leaves now, and whether a later entry already owns the same reading over the engine:
  `Agent harness drives the console engine` (`working-route.md:31`) and `Engine end-to-end gate reachable`
  (`working-route.md:43`) are the two named. "Nothing is repaired for a surface on its way out unless leaving it
  would let a defect through before then."
- **What reads the edited steps in the tree is kept true.** [premise-corrected: no test in the tree pins an arm of
  `ci-gates`, of the two nextest wrappers or of either check script; `grep -n` over `xtask/src/*.rs` finds only
  their definitions and the `main` dispatch.] Two test files read the workflow's steps
  (`pulse-app/tests/quality_gate_workflow.rs`, 25 tests; `pulse-app/tests/a11y_perf_workflow.rs`, 11): they pin
  a step's presence, its neighbours and the soft-fail ban's list of 14 gate commands, never an arm. The grader's
  arms alone are pinned in `xtask` (`xtask/src/perf_budget.rs`, 21 tests). So a gate that stays gains the pin it
  never had, one that reads what makes it fail; one that goes takes its presence pin and its list member with it.
- **A witness that cannot pass vacuously.** The acceptance is read "on a CI run of the tree", so the
  proof is a run on the chunk's pre-CI commit read gate by gate, beside pins in the tree; a gate that is kept is
  shown failing on an empty or absent input (a mutation or a constructed input), not only passing on a full one.
  Verified at P3 as the obligation the tests and obs extracts state (test-plan §3 Per-chunk gate discipline;
  obs-plan §10 CI gates).
- **The masters' sentences this entry's wrap corrects are listed in the plan as expected amendments, as
  measured.** Phase amends no spec source. Verified at P3, the sites the extracts locate: obs-plan §10 CI gates
  (`obs-plan.md:556`, `:558`, `:559`), §10 Performance budgets (`:545`), §10 Load-profile constraints (`:579`),
  §9 (`:499`, `:508`); test-plan §4, §9 Pipeline structure, §9 and §10 Build failure conditions, §10 Coverage
  thresholds, §3 Bootstrap phases and Per-chunk gate discipline; architecture §Infrastructure Patterns → CI/CD
  approach and §Occupied Resources → xtask CLI surfaces; a11y-plan §1, §3, §9 only if the upload changes.
- **The plan lists the merge-base probe directly before its push entry** (inputs#I1 item 6; test-plan §3 →
  Per-chunk gate discipline, new at the last wrap).

## Decided at P4 (the operator's answers, the pc overseer, 2026-10-10 — inputs#I2)
- **Coverage: an empty report fails, and the branch arm is retired** (inputs#I2 item 1). A report tracking 0 lines
  or 0 functions fails the step. The branch arm, its threshold line and the job-name clause leave: "a threshold that
  has never read a number is a false statement, and a second unpinned toolchain is not its repair." The retirement
  is recorded PROVISIONAL for the founder, and the plan says what a real branch count would need.
- **All five empty selections fail** (inputs#I2 item 2). Each `--no-tests=pass` becomes `--no-tests=fail`, spelled
  out; the step's name and the two help strings follow.
- **`ci-gates` is narrowed to the two arms that read the boot log** (inputs#I2 item 3): the record count and the
  panic read stay, an absent log is an exit of its own and no longer four NEUTRAL lines, and the heartbeat and
  perf-budget arms leave the verb. The zero-span sentence is amended to its measured home. The plan names every
  place that reads the two lines that leave (the pre-push stage, any pin, any master sentence) and what each reads
  after.
- **The quarantine check is repaired; the a11y upload and the pre-push stage are not** (inputs#I2 item 4). The
  check fails on an absent input and prints what it scanned. The two that stay "each keep an owner and a stated
  reading; nothing is repaired for a job on its way out": the a11y upload's reading moves to `Window's gates
  retired`, the pre-push stage's to `Agent harness drives the console engine`. "P-128 is about what a CI job does":
  the concretization says the pre-push stage is outside it and who owns it.

- **The plan's own decisions, outside the four answers and shown on the P5 card:** the workflow's sixth nextest
  step (`--profile perf-samples`, which carries no flag and ran 1 test) is spelled `--no-tests=fail` with the
  five, because the runner's nextest is older than the host's and its default was not read; the narrowed verb's
  panic line names a basename and a line number where it prints the log's full path and the record's text today
  (security-plan §Security Anti-Patterns → Logging); the coverage step stays inline and is pinned by a witness
  that runs it.

## Capability
- **P-128 is claimed here, if the plan makes its whole acceptance true** (inputs#I1 item 5). Its acceptance: "On a
  CI run of the tree, every comparison against a baseline reads a baseline that a workflow produced, or the
  workflow no longer makes that comparison; every upload uploads a file, or the workflow no longer makes that
  upload. No gate stands while reading nothing." The last chunk made the first two clauses true (its report); the
  third is this chunk's.
- **The reading for the claim** is the entry's fourth `CARRY:`: "the reading for P-128's claim, the operator's
  word (the pc overseer, 2026-10-10; 2026-10-10-no-ci-step-reads-nothing's inputs#I2 item 3): a baseline is
  "produced by a workflow or committed in the tree"; the acceptance's wording was narrower than the requirement
  line". Re-read at take-up in that chunk's `inputs/I2-relay-1.md.txt`, item 3, and in the ledger's dated note on
  `verification-matrix.json#P-128`: both carry it. The concretization is shown against the acceptance at P5.
- The claim rests on the first two clauses still holding at this chunk's commit: the comparisons and
  uploads the last chunk settled are re-read on this chunk's run, not assumed. Verified at P3 on
  `ci#38038281709`: no download step, six uploads finding 1, 1, 5, 1, 42 and 1 files, the a11y comparison reading
  the tree baseline.
- P-128 is PROVISIONAL until the founder's own word (the ledger's notes; the handoff).

## Boundaries
- **"A gate of the ci workflow"** is a step of `ci.yml` (or an arm of a tool such a step runs) whose exit decides a
  job of a workflow that a push or a pull request of the tree triggers. `release.yml` and `update-channels.yml` are
  outside, as at the last chunk; they leave at `Desktop distribution retired`.
- **The `a11y` job and the `boot` job stay** until `Window's gates retired` (`working-route.md:48`). This chunk
  settles only the named gates inside them. The boot smoke, its series and the exit witness are not this chunk's to
  change; the boot job's `Boot series (equal source)` step stays a gating step, and a self-end in it is a new
  reading, brought to the operator (the handoff).
- **A change that needs a new permission, token, trigger or third-party action halts for the founder's own word**
  (inputs#I1 item 7). The workflow's `permissions: contents: read` (`ci.yml:8-9`) stays.
- **No new requirement is minted by phase**, and no spec source is amended by it.
- No run binds 4317 or 4318 without the operator's word (the handoff's host note).
- **At P5** the card says what `planlint` check 4 listed, and the phase stops for the operator's `yes`
  (inputs#I1 item 8; the invocation's directive, which names inputs#I1's file and the same stop).

## Folded freight (the entry's four blocks)
`route.py pins` lists four blocks on `working-route.md:25`, all `CARRY:` (1232, 289, 268 and 273 characters); each
is folded above, whole.
- First `CARRY:` (the five gates, each with how it was read, and where each sits): "What the chunk builds", first
  bullet and its six sub-bullets.
- Second `CARRY:` (the two further literals): "What the chunk builds", second bullet.
- Third `CARRY:` (the a11y upload's path members): "What the chunk builds", third bullet.
- Fourth `CARRY:` (the reading for the claim): "Capability", second bullet.
- No `PREREQ:`, no `WATCH:`, no `BLOCKED-ON:`.

## Second fold source — the CI verdict read at Setup 5a
The base is the last commit that flipped a master record, `7419496b`, which is HEAD: one sha, read through
`ci.py conclusion`.
- `7419496bdf9f` (HEAD, the last wrap's commit): **green** · 7 of 7 checks · wall 1683 s (`ci#38038281709` and
  `secret-scan#38038281700`, both pull-request runs, completed/success). It is the newest run of the tree this
  chunk edits, and the one the five gates are re-read on at P3.
- No red and no `not green` was read, so nothing is dispositioned here.

## Gate
- none — the entry carries no `BLOCKED-ON` (`route.py blocked`: 0 blocks on a pending, gated or markerless line).
