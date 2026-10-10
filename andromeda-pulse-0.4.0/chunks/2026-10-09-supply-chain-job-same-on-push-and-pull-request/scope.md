# Scope — Supply-chain job same on push and pull request

**Marker:** `2026-10-09-supply-chain-job-same-on-push-and-pull-request` · version `andromeda-pulse-0.4.0` · Epoch 1 — Foundation: base CI, the capability record, a console engine with its door, its gates
**Working entry (`working-route.md:13`):** "Supply-chain job same on push and pull request — fails on a finding,
never on its own reporting; repair witnessed on a push (P-120)", with three freight blocks folded below.
**Chunk base:** `b3e5859` (HEAD at take-up). Every diff-shaped probe of this chunk names this sha, never HEAD: the
operator pre-CI commit moves HEAD before the wrap re-runs them (inputs#I1 item 6).

## Intent
Requirement P-120 reads: "the supply-chain job ends the same on a push as on a pull request; it fails on a finding
and never on its own reporting; the repair is witnessed where the defect shows" (`matrix.py show --id P-120`, read
at take-up). `main` has been red since the push that merged PR #39 (`60ef43c`). This chunk repairs the `cargo
audit` step of the `supply-chain` job on the build branch, carries the same repair to `main` through a separate
pull request the founder merges, and reads the push run his merge makes.

**The founder's ruling, 2026-10-09** (inputs#I3; founder, 2026-10-09, by dialog — relayed verbatim by the pc
overseer; the review word that brought it is inputs#I4). Asked whether the red `main` is repaired now by a separate
small pull request or stays red until the whole version merges, he picked «Чиним main сейчас (Рекомендую)». The
option as he read it: the builder prepares a separate pull request into `main` with this workflow repair alone and
the test that holds it; he merges it by his own hand; the real push to `main` reads green, and that push is the
witness of the repair where the defect shows. The cost he accepted: his hand on the merge, the chunk waits for it,
one more CI round on `main` with its three-system matrix, the plan rewritten, and a departure from
`security-plan.md:226` ("reaches `main` with the version") for this repair.

## What the chunk builds
- **For the same tree, the `supply-chain` job ends the same on a push as on a pull request** (the entry). The step
  in question, read at take-up at `b3e5859`: `cargo audit (RustSec advisory DB)`, `rustsec/audit-check` pinned at
  `69366f33c96575abad1ee0dba8212993eecbe998` (v2.0.0), passed `token: ${{ secrets.GITHUB_TOKEN }}`
  (`ci.yml:426-429`). The workflow's top-level `permissions:` is `contents: read` alone (`ci.yml:8-9`), and the job
  sets none of its own (`ci.yml:400-474`).
- **The step fails on a finding and never on its own reporting** (the entry). Both halves need a witness: a run
  that stays green when the report cannot be published, and a control that the step still turns red on an advisory.
  - Verified at P3: a green run alone cannot witness the second half, since a step that never fails also "ends the
    same" on both events. The control exists and discriminates. Measured on cargo-audit 0.22.2, the version the
    runner image lists (`Ubuntu2204-Readme.md`, image `20261004.315.1`, "Cargo audit 0.22.2"): a scratch lockfile
    naming `time 0.1.43` exits 1 with `RUSTSEC-2020-0071` and `error: 1 vulnerability found!`; the project's own
    lockfile exits 0 with `warning: 10 allowed warnings found`; an unreadable lockfile exits 2. Three exits, and
    "cannot evaluate" is not a pass. The control is local, on this host; the operator accepted that (inputs#I3,
    point 3), and the report says so.
- **The repair is witnessed on a push, where the defect showed** (the entry; P-120's third clause).
  - Verified at P3: the build branch gets no push-event run. `ci.yml:3-6` triggers on `pull_request` and on a push
    to `main` only, and the branch's 12 runs are all `pull_request` events (`gh api
    …/actions/runs?branch=build/andromeda-pulse-0.4.0`, grouped by event, read 2026-10-09T16:03Z). A green
    pull-request run does not witness the repair, and the plan says so (inputs#I1 item 2).
  - [premise-corrected: the founder's ruling of 2026-10-09, inputs#I3] **The witness is the push run on `main`
    that the founder's merge makes.** The plan first read at P5 obtained the witness on a dedicated ref through a
    new `push-witness/**` trigger and left `main` red; both are withdrawn. The trigger block of `ci.yml` stays as
    it is.
  - Added at P5: **a second branch carries the same repair to `main`.** It is cut from `origin/main` (`60ef43c`,
    equal to the remote's `refs/heads/main`, re-read at P5) and holds the audit-step change and the test that holds
    it, nothing else of 0.4.0. Its pull request into `main` is opened ready for the founder's merge; the builder
    never merges it and never pushes to `main`. `main`'s workflow keeps its three-system matrix and its seven jobs;
    that is not this chunk's to change there.
  - Added at P5: **the operator pass has a founder-attended stop.** After the second branch's pull-request run is
    green the pass stops and says so; the operator brings it to the founder; after his merge the pass reads the
    push run on `main` and its `supply-chain` job. That reading, named by run id, is P-120's witness.
  - Added at P5: **the two branches meet again without a conflict, measured.** Trial merges in a scratch clone
    (git 2.55.0, nothing pushed, the project's repository not written): with the repair byte-identical on both
    sides, the version's merge into a `main` that already holds it is clean and the merged tree equals the build
    branch's tree; a `main` that holds it merged with the build branch as it stands today is clean too, so the
    draft pull request #40 keeps a computable merge and its runs keep firing whichever lands first. The control: a
    pin that differs by one token between the two branches conflicts in the test file. So the second branch is
    made from the build branch's own diff, never typed twice.
- **The cause is now a measured fact of this chunk.** Two sources stated it; both are closed against the two runs
  and the action's source at its pinned commit:
  - Verified at P3, with one correction of precision. The relay's `hypothesis:` read "on a push the step tries to
    create a check run the token may not write; on a pull request the same denial is swallowed" (inputs#I1 item 1).
    Push run `37907730264` on `60ef43c`, job `113745162405`: the token reads `Contents: read`, `Metadata: read`
    (log `:23-24`); the step prints `No vulnerabilities were found`, `10 warnings found!`, `Found 8 unmaintained, 2
    unsound`, then `##[error]Resource not accessible by integration - …/checks/runs#create-a-check-run` (`:657-660`)
    and fails; the nine steps after it are `skipped`. Pull-request run `37904682919` on `0e45d58`, job
    `113735187440`: the same token and the same three lines, then `##[error]Unable to publish audit check! Reason:
    HttpError: Resource not accessible by integration`, "Posting audit report here instead." and `No critical
    vulnerabilities were found, not marking check as failed` (`:698-701`, `:889`); the step succeeds. The source
    (`src/reporter.ts:176-205` at the pin) catches the failed check-run call and falls back to printing only when
    `GITHUB_HEAD_REF` is set, which is every pull-request event; otherwise it rethrows. [premise-corrected: the
    denial bites only when there is something to report. `src/main.ts:87-89` returns before any API call when the
    audit finds no vulnerability and no warning; the tree carries 10 informational warnings, so the call is made on
    every run today.]
  - Verified at P3: the matrix's `observed_gap` states the same mechanism, and it holds. Its second coordinate
    names the step where `main` has it (`ci.yml:493-496` at `60ef43c`, re-read at P5); on the build branch the step
    stands at `ci.yml:426-429`.
- **The repair widens nothing.** Verified at P3: the step becomes a plain `run: cargo audit`, with no third-party
  action and no token. The binary is on the runner as the job stands (the push log calls
  `/home/runner/.cargo/bin/cargo audit` 0.15 s after the action starts, with no install); the operator accepted the
  step without an install (inputs#I3, point 4), and the runner image's version is recorded in the evidence. The
  workflow-level `permissions:` block and every job's token stay as they are, and the step no longer receives the
  token at all. The widening that would also turn the push run green, a job-level `checks: write`, is not needed
  and would not meet the entry: with it the push path still fails whenever the check-run call fails
  (`src/reporter.ts:204`), so the job would still end on its own reporting. No boundary widening is planned
  (inputs#I1 item 3, inputs#I2; the playbook's pattern at `.andromeda/playbook.md:118`).
- **What reads `ci.yml` in the tree is kept true.** Verified at P3 (`grep -rln 'ci\.yml'` over the Rust, script,
  workflow, manifest and JSON files outside `target/`, `.andromeda/` and the chunk folders): two test files and one
  xtask module read the workflow's content.
  - `pulse-app/tests/a11y_perf_workflow.rs:151-160` pins `permissions:` and `contents: read` inside the first 40
    lines; `:119-149` pins every `uses:` line to a 40-character sha.
  - `pulse-app/tests/quality_gate_workflow.rs:131-146` pins the `supply-chain` job's auditable build; `:233-276`
    bans `continue-on-error: true` on a listed set of gate commands, and `cargo audit` is not in the list. No test
    pins the audit step's action, its token or the trigger block.
  - `xtask/src/pre_push.rs` reads the Node major and the first `apt-get install` list; the repair touches neither.
  - Read at P5 on `main` (`60ef43c`): the same holds there. `a11y_perf_workflow.rs`, `xtask/` and `Cargo.lock` are
    byte-identical on the two branches; `main`'s `quality_gate_workflow.rs` has the helper and the soft-fail ban
    the pin needs, and no test of `main` pins the action or the token.
- **No other workflow carries the step.** Verified at P3 (`grep -n -E 'audit-check|rustsec'` over the four workflow
  files: one line, `ci.yml:427`). `secret-scan.yml:29` hands the token to gitleaks as an environment variable; its
  push run on `60ef43c` succeeded (`secret-scan#37907730180`), so it does not share the defect. `release.yml` and
  `update-channels.yml` hold no audit step.
- Added at P3, re-cut at P5: **the pins, in the test file that already pins the workflow.**
  - On both branches, identical: the audit step is a plain `run:` step with no action, no token input and no
    soft-fail, and the job carries no `permissions:` block; `cargo audit` joins the no-`continue-on-error` list. The
    pin is written before the edit and read red against the workflow as it stands.
  - On the build branch only: the trigger block is pinned as it stands, `pull_request` and a push to `main`, so no
    other event can be added unseen.
- **The records that describe this step are corrected at the wrap, not here.** Phase amends no spec source. The
  known sites: security-plan §Dependency Security, CI integration (`security-plan.md:216`) and §Bootstrap phases
  `dep-security-ci-gate` (`:259`), both naming `actions-rust-lang/audit`; `security-plan.md:226`, whose direction
  this repair departs from on the founder's ruling; architecture's CI/CD approach key file, for the step and the
  cache reading; test-plan §9's Supply chain row, which names no `cargo audit` step at all.

## Capability
- **P-120 is this entry's one capability**, and no other markerless entry names it (`grep -n 'P-120'` over
  `working-route.md` reads line 13 alone). Its concrete form reads on `main` itself: the push run on `main` green
  in `supply-chain`, the audit step `success`, the job log holding cargo audit's scan line and no check-run denial;
  the pull-request run of the build branch green on the same repair; the step's shape held by a probe and a test;
  the local red control on a lockfile with a known advisory (inputs#I3). The claim is previewed at P5.

## Boundaries
- **`No CI step reads nothing` (P-128) is two entries later** (`working-route.md:17`): the baseline downloads and
  the uploads that find no file are its work, not this chunk's, even with the same file open (inputs#I1 item 5).
- **`boot`, `a11y`, `lint-test`, `mcp-test` and `coverage` are not touched**, on either branch. Inside
  `supply-chain`, the `cargo deny` step, the npm gate and the auditable build are not this chunk's subject.
- **`main` receives the repair and its test, nothing else.** Its three-system matrix, its `release` job and its
  other tests stay as they are; they leave `main` when the version reaches it.
- **The builder never merges the pull request into `main` and never pushes to `main`.** The merge is the
  founder's hand (inputs#I3).
- **Nothing outside the tree is deleted.** The six orphaned cache entries are re-read at the wrap, not removed.
- **A boundary widening halts for the founder's own word**, at P4 (inputs#I1 item 3, inputs#I2). None is planned.
- **The plan card is printed at P5 and the chunk stops there for the operator's `yes`** (inputs#I1 item 6,
  inputs#I2, inputs#I4).
- No run binds 4317 or 4318 without the operator's word (the handoff's host note); the chunk's own edits need none.

## Folded freight (the entry's three blocks)
- watch: boot smoke on ubuntu-22.04 ended exit 1 within 0.4 s of its last record, after ready, with no process-end
  record; no cause in the application log, whose warnings match the green run on 7f99c38 — run 37924991598 on
  b3ac58a; hypothesis: the window's boot; the job leaves at Window's gates retired (2/3; since
  2026-10-09-0-pending-adaptation-wrap). An observation: no acceptance criterion, no plan task, no gate entry.
  - Readings so far: red on `b3ac58a` (`ci#37924991598`), green on `0b61bfb` (`ci#37934330231`), green on `569604b`
    (`ci#37945548047`) — from the handoff, not re-read at take-up.
  - Read at P3: `boot smoke (ubuntu-22.04)` ended `success` on `b3e5859` (`ci#37954318153`, job `113900821704`,
    15:47:19Z to 15:57:54Z). The wrap counts it. Its verdict is recorded for every CI run this chunk reads
    (inputs#I1 item 4); a recurrence halts at route-resolve. The runs on `main`'s tree are readings too, on equal
    source: no source file differs between `60ef43c` and `b3e5859` outside `ci.yml` and one test file.
- carry: six Actions cache entries sit on keys no job writes since 2026-10-09-ci-on-linux-alone (release-Windows,
  lint-test-Windows, lint-test-macOS and release-macOS on main; lint-test-Windows and release-Windows on pull/39),
  8 531 028 085 B of 12 208 662 121 B, the cache 1 471 243 881 B over its cap as read 2026-10-09T15:13:15Z; nothing
  in the tree reads or writes them and the wrap deleted none. **This entry's wrap re-reads the listing and records
  the reading in architecture §Infrastructure Patterns, CI/CD approach** (its key file, read at take-up from the
  index: `.andromeda/registries/contracts/architecture/ci-cd-approach.md`). The plan carries it as an expected wrap
  amendment; it is not implement's work.
  - Read once at P3 (2026-10-09T15:58:06Z, `gh api …/actions/caches` and `…/actions/cache/usage`): the listing is
    unchanged, 10 entries and 12 208 662 121 B, the six orphaned entries last accessed 13:05Z (the four on `main`)
    and 08:24Z (the two on `pull/39`). The block's claim, marker kept verbatim, "hypothesis: GitHub evicts them
    first, as the least recently used", is neither shown nor excluded by that: nothing has been evicted yet.
  - [premise-corrected: `main`'s own workflow still holds the jobs that use those keys (`git show
    60ef43c:.github/workflows/ci.yml`: seven jobs, `release` and the two matrices among them)] "Keys no job
    writes" is true of the build branch's tree only. The two rounds this chunk now runs on `main`'s workflow, the
    second branch's pull-request run and the push run after the merge, restore those keys and may save them, so the
    wrap's re-read no longer tests the eviction hypothesis cleanly: it records what the listing shows and says
    which entries those two rounds touched.
- carry: security-plan §Dependency Security, CI integration, names the action `actions-rust-lang/audit` for the
  cargo audit step while `ci.yml` uses `rustsec/audit-check` (read 2026-10-09 at the 2026-10-09-ci-on-linux-alone
  wrap, outside its report); **the master line is corrected at this entry's wrap**. Re-read at take-up:
  `security-plan.md:216` names `actions-rust-lang/audit`, and `ci.yml:427` uses `rustsec/audit-check`.
  - Verified at P3: the repair replaces the action with a plain step, so the wrap's correction names what ships
    (`cargo audit` as a `run:` step, no action), at both sites that name the action (`:216` and `:259`).

## Second fold source — the CI verdict read at Setup 5a
The last master flip is HEAD itself (`b3e5859`), so one sha was read through `ci.py conclusion`:
- `b3e58597dbe9`: **verdict not yet available** at Setup — in progress at 2026-10-09T15:49Z, 7 checks registered
  (`ci#37954318153`; `secret-scan#37954318186` success). It settled during this phase: read again at P5, `verdict:
  green · checks 7/7 · wall 1717 s`. Nothing is folded from it.

The red this entry exists for is not among the shas Setup 5a covers: `ci#37907730264` on `60ef43c` predates the
last flip. It is the entry's own subject, carried in "What the chunk builds" above.

## Gate
- none — the entry carries no `BLOCKED-ON` (`route.py blocked`: 0 blocks on a pending, gated or markerless line).
  The founder's merge is a stop inside the operator pass, not a block on the entry.
