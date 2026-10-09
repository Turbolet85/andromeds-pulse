# Codebase Research — 2026-10-09-supply-chain-job-same-on-push-and-pull-request

## Scope
- **Depth:** moderate · **Reads:** 15 · **Globs/Greps:** 14
- **Harness rules consulted:** none — no live leg in this chunk (it boots no app and drives no external process;
  its CI readings are operator entries)
- **Platform issues consulted:**
  - `repo:rustsec/audit-check "Resource not accessible by integration"` (GitHub issue search) → issue #28, "Unable
    to publish audit check from forked repos", open since 2024-10-08, 5 comments. Its body quotes both readings this
    chunk measured: the fallback text ("Unable to publish audit check! … Posting audit report here instead. … No
    critical vulnerabilities were found, not marking check as failed") and the bare `Error: Resource not accessible
    by integration - …/checks/runs#create-a-check-run`, from a repository that is not a fork.
  - `rustsec/audit-check` at the pinned commit `69366f33c96575abad1ee0dba8212993eecbe998`, fetched whole:
    `action.yml` (`token` is a required input), `src/main.ts`, `src/reporter.ts`, `README.md`. The README's
    "Granular Permissions" block names `issues: write` and `checks: write` as "the typically used permissions"
    (`:46-60`) and its "Limitations" paragraph states the fallback to stdout for pull requests from forks
    (`:67-73`).
  - `actions/runner-images`, `images/ubuntu/Ubuntu2204-Readme.md` as fetched 2026-10-09: image version
    `20261004.315.1` (the version the push log names at its "Runner Image" group) lists "Cargo audit 0.22.2" among
    the Rust packages (`:161`).
- **External inputs:**
  - `inputs#I1` — the pc overseer's phase directive: the defect as read in the two runs, the push-event witness,
    the permission-widening halt, the freight, the P-128 scope edge.
  - `inputs#I2` — the invocation directive: a plan that widens a token permission halts at P4 for the founder's own
    word; stop at P5 for the operator's `yes`.
  - `inputs#I3` — the operator's answer at P5 on the founder's ruling of 2026-10-09: `main` is repaired now by a
    separate pull request he merges, the witness is the push run on `main`, no `push-witness` trigger; the meeting
    of the two branches is measured by a trial merge.
  - `inputs#I4` — the P5 review word: revise the plan to that ruling and stop at P5 again.

## Files inspected
- `.github/workflows/ci.yml` (`:1-12`, `:400-474`, and a key grep over the whole file) — the trigger block
  (`:3-6`), the workflow-level `permissions:` (`:8-9`), the `supply-chain` job (`:400-474`) and its audit step
  (`:426-429`). The job has no `permissions:` key. `secrets.` occurs once in the file, at `:429` (`grep -n -o -E
  'secrets\.[A-Z_a-z]+' .github/workflows/ci.yml`).
- `.github/workflows/secret-scan.yml`, `release.yml`, `update-channels.yml` (key grep) — no audit step in any
  (`grep -n -E 'audit-check|rustsec' .github/workflows/*.yml`: 1 line, `ci.yml:427`). `secret-scan.yml:3-6`
  triggers on `pull_request` and on a push to `main`; it passes the token to gitleaks as an environment variable
  (`:29`).
- `pulse-app/tests/quality_gate_workflow.rs` (`:79-99`, `:131-146`, `:233-276`, and its function index; 490 lines)
  — `workflow_job_block(content, job)` returns one job's lines (`:79-99`);
  `ci_workflow_keeps_the_linux_release_build_witnesses` reads the `supply-chain` block through it (`:131-146`);
  `ci_workflow_test_gates_no_continue_on_error` walks every `      - name: ` step and bans `continue-on-error:
  true` on a step whose block holds one of 15 listed gate substrings (`:233-276`). `cargo audit` is not among the
  15. No function reads a single step's block by name; the no-`continue-on-error` test cuts step blocks inline.
- `pulse-app/tests/a11y_perf_workflow.rs` (`:119-160`, and its function index) —
  `ci_workflow_uses_sha_pin_discipline_unchanged` (`:119-149`) checks every `uses:` line;
  `ci_workflow_preserves_workflow_level_contents_read_permission` (`:151-160`) reads the first 40 lines for
  `permissions:` and `contents: read`.
- `xtask/src/main.rs` (`:109-110`, `:297`) — `Cmd::Audit` exists and runs `cargo audit` through `run_cargo("audit",
  &[])`. No workflow calls it. `xtask/src/pre_push.rs` holds no line naming the audit (`grep -rn -i -E
  '\baudit\b|cargo.audit' xtask/src/*.rs`, the `auditable` lines dropped: hits in `main.rs` and `npm_gate.rs` only).
- `.andromeda/registries/contracts/architecture/ci-cd-approach.md` (full) — "`ci.yml` runs on every PR and push to
  main as six independent jobs"; the three dated cache readings.
- `.andromeda/security-plan.md` (`:216`, `:226`, `:259`) — the two sites naming `actions-rust-lang/audit`; the
  sentence "until then a fix lands on the version's build branch and reaches `main` with the version" (`:226`,
  inside the Critical CVE response paragraph, founder ruling 2026-10-07).
- `.andromeda/playbook.md` (`:118-120`) — the one `verdict: escalate` pattern, Boundary widening (`awk` over every
  `- pattern:` whose verdict is `escalate`: 1).
- `andromeda-pulse-0.4.0/chunks/2026-10-09-ci-on-linux-alone/plan.md` (`:90-253`, `:305-387`) — the standard gate
  set in its current order and the operator entries' forms.
- The two job logs, fetched whole through `gh api …/actions/jobs/{id}/logs`:
  - push run `37907730264` on `60ef43c`, job `113745162405` (841 lines): token `Contents: read`, `Metadata: read`
    (`:23-24`); the step starts at 08:53:50.403Z and calls `/home/runner/.cargo/bin/cargo audit --json --file
    ./Cargo.lock` at 08:53:50.559Z (`:637`, `:653`); `No vulnerabilities were found` (`:657`), `10 warnings found!`
    (`:658`), `Found 8 unmaintained, 2 unsound` (`:659`), `##[error]Resource not accessible by integration -
    https://docs.github.com/rest/checks/runs#create-a-check-run` (`:660`). Step 9 `failure`, steps 10 to 18
    `skipped` (the run's jobs listing).
  - pull-request run `37904682919` on `0e45d58`, job `113735187440` (3220 lines): the same token (`:23-24`) and
    the same three lines (`:695-697`); then `##[error]Unable to publish audit check! Reason: HttpError: Resource
    not accessible by integration …` (`:698`), "It seems that this Action is executed from the forked repository."
    (`:699`), "Posting audit report here instead." (`:701`), `No critical vulnerabilities were found, not marking
    check as failed` (`:889`). All 18 steps `success`.
- `ci#37954318153` on `b3e5859` (the jobs listing, read 2026-10-09T16:03:51Z) — five jobs `success`, `coverage`
  still running; `boot smoke (ubuntu-22.04)` `success`, job `113900821704`, 15:47:19Z to 15:57:54Z. Settled by the
  P5 revision: `verdict: green · checks 7/7 · wall 1717 s`.
- `main`, read at the P5 revision after the founder's ruling (inputs#I3):
  - `origin/main` is `60ef43c6859cb35a9f9ee5664ab892339cffdf4f`, equal to the remote's `refs/heads/main` (`git
    ls-remote origin refs/heads/main`) and to the merge base with HEAD (`git merge-base origin/main HEAD`): the
    build branch is strictly ahead of `main`. It is a merge commit, "Merge pull request #39"; `main` is not
    protected (`gh api …/branches/main/protection`: 404, "Branch not protected").
  - Outside the planning folders two files differ between `60ef43c` and `b3e5859`: `.github/workflows/ci.yml` and
    `pulse-app/tests/quality_gate_workflow.rs` (`git diff --name-only 60ef43c b3e5859` with `.andromeda`, the
    version folders, `.claude`, `CLAUDE.md` and `docs` excluded). `Cargo.lock`, `xtask/` and
    `pulse-app/tests/a11y_perf_workflow.rs` are byte-identical (`git diff --quiet`, exit 0).
  - `main`'s `ci.yml` (`git show 60ef43c:.github/workflows/ci.yml`): the same trigger block (`:3-6`) and
    `permissions:` (`:8-9`); seven jobs, `release` among them; `supply-chain` on `ubuntu-22.04` (`:467`); the audit
    step at `:493-496`, the same four lines as on the build branch, with the same three lines of context above and
    below.
  - `main`'s `quality_gate_workflow.rs` (456 lines): `workflow_job_block` at `:79`,
    `ci_workflow_test_gates_no_continue_on_error` at `:200` with its `gate_substrings` at `:202-218`,
    `ci_workflow_env_includes_ci_run_id` at `:244`. The build branch differs from it in two hunks only (`git diff
    60ef43c b3e5859 -- pulse-app/tests/quality_gate_workflow.rs`: `@@ -98,16 +98,50` and `@@ -410,7 +444,7`). The
    lines around `"cargo deny check",` and around the `#[test]` of `ci_workflow_env_includes_ci_run_id` are the
    same on both branches (`:204-208`, `:240-246` on `main`; `:238-242`, `:274-280` on the build branch).
  - A pull-request run on `main`'s workflow registers 13 checks and took 1293 s (`ci.py conclusion` on `0e45d58`);
    its push run took 1923 s (`60ef43c`).
  - Draft pull request #40 (`build/andromeda-pulse-0.4.0` into `main`) reads `mergeable MERGEABLE`, state `CLEAN`
    (`gh pr view 40`, read 2026-10-09T16:21Z).
- The repository cache (`gh api …/actions/caches`, `…/actions/cache/usage`, read 2026-10-09T15:58:06Z) — 10
  entries, 12 208 662 121 B. The Linux keys the pull-request runs restore sit on `refs/heads/main`
  (`v0-rust-boot-Linux-Linux-x64-0e159c75-69f35c02`, `v0-rust-lint-test-Linux-Linux-x64-0e159c75-69f35c02`,
  `v0-rust-coverage-Linux-x64-0e159c75-69f35c02`), last accessed 15:47Z by the run on `b3e5859`.

## Graph impact (from the code-graph query; "cold-start — empty DB" if early)
- **workflow_job_block** — 2 callers, both in `ci_workflow_keeps_the_linux_release_build_witnesses`, at
  `pulse-app/tests/quality_gate_workflow.rs:134` and `:140` (the canonical impact query on the `rust` plane, 2
  rows; the trace's 0-indexed lines 133 and 139 are editor lines 134 and 140; `grep -n 'workflow_job_block('`
  agrees). The chunk calls the helper a third time and does not change it.
- The chunk changes no existing symbol's shape: it adds two test functions and one element of a local array
  inside `ci_workflow_test_gates_no_continue_on_error`. The workflow file is on no indexed plane.

## Patterns detected
- **A gate as a plain named `run:` step** (`ci.yml:436-437`, the Cranelift assertion; `:456-457`, the npm gate):
  no action, no `with:`, the step's exit is the command's.
- **A workflow claim pinned by a test that reads `ci.yml`** (`quality_gate_workflow.rs:131-146`): the test cuts the
  job block with `workflow_job_block` and asserts on substrings, with the plan section it serves in the message.
- **A step block cut by the step-name prefix** (`quality_gate_workflow.rs:253-264`): `      - name: ` positions
  split the file into step blocks.
- **Three exits for a gate** (cargo-audit 0.22.2, measured below): 0 clean, 1 finding, 2 could not evaluate — the
  shape the npm gate and the staged-artifacts gate state for themselves.

## Conventions to follow
- **Tests of `pulse-app` live in `pulse-app/tests/*.rs`**, never in lib sources (test-plan amendment
  `2026-08-14-fingerprint-feed-capture-repair`); a new pin goes beside the ones it resembles, in
  `quality_gate_workflow.rs`.
- **Assertion messages name the rule they serve** (`quality_gate_workflow.rs:137-138`, `a11y_perf_workflow.rs:157-159`).
- **Source annotations are ASCII English** (`cargo xtask check:english-sources`, a gate of the standard set); the
  workflow's comments follow the same habit (`ci.yml:419`, `:444-445`).
- **A diff-shaped probe names the chunk base by sha**, `b3e58597dbe9f10e541c0bc1d122b8647ba8c62a` (inputs#I1 item
  6; plan-template §Test Commands, `run`).

## Mechanisms the plan rests on, each stated as the equality it needs and how it was read
- **The defect.** On a push, `rustsec/audit-check` v2.0.0 fails the step when the check-run create call is denied
  and there is something to report; on a pull request it does not. Frame: `src/reporter.ts:176-205` at the pin —
  `reporter.startCheck('queued')` inside a `try`; the `catch` prints and returns (or throws only on `stats.critical
  > 0`) when `process.env.GITHUB_HEAD_REF` is set, else `throw error`. `src/main.ts:87-89` returns before the
  reporter when nothing was found; `:98-102` routes every non-`schedule` event to `reportCheck`. Both arms are in
  the two logs above. `GITHUB_HEAD_REF` is set on a pull-request event and unset on a push: this is how GitHub
  describes its default variables, recalled and not fetched at P3; the two logs are consistent with it.
- **"Critical" in that action** is a vulnerability whose `advisory.informational` is null
  (`src/reporter.ts:95-107`); warnings never count (`:115-130`). So the step as it stands fails on a vulnerability
  and not on the 10 informational warnings.
- **The replacement's exits.** `cargo audit` on cargo-audit 0.22.2, measured on this host 2026-10-09 (the local
  `cargo audit --version` prints 0.22.2, the version the runner image lists):
  - a scratch lockfile of two packages naming `time 0.1.43`: exit 1, `ID: RUSTSEC-2020-0071`, `error: 1
    vulnerability found!` (read under the name `Cargo.lock` and again under another file name through `--file`:
    the same);
  - the project's `Cargo.lock`: exit 0, `Scanning Cargo.lock for vulnerabilities (916 crate dependencies)`,
    `warning: 10 allowed warnings found`; the 10 distinct ids are RUSTSEC-2024-0370, -2024-0429, -2024-0436,
    -2025-0075, -2025-0080, -2025-0081, -2025-0098, -2025-0100, -2025-0141, -2026-0221 (`grep -o` over the output,
    sorted unique), the same ten the pull-request log prints;
  - `--file` naming a file that does not exist: exit 2.
  The advisory database it read is the one under `~/.cargo/advisory-db` (`CARGO_HOME` is unset on this host, so the
  two spellings name one directory), 1296 advisories, the same count both CI logs print.
- **The same posture as today.** The action fails on a vulnerability only; plain `cargo audit` exits 1 on a
  vulnerability and 0 on informational warnings unless `--deny warnings` is passed. The plan passes no flag.
- **A `run:` step fails on a non-zero exit.** The Cranelift step of the same job relies on it (`ci.yml:436-437`,
  `… || exit 1`). No `continue-on-error` stands on the audit step, and the plan's pin keeps it so.
- **The founder's merge makes the push run that witnesses.** A merge into `main` is a push to `main`, and `ci.yml`
  runs on it (`:3-6`, the same block on `main`). The base's red run is the instance: `ci#37907730264`, event
  `push`, made by the merge of PR #39.
- **Three commits, three runs, and how the reader names each.** `ci.py conclusion` reads a sha's runs and prints
  each run's event in its `runs:` line (read on `60ef43c`: `runs: secret-scan#37907730180 push completed/success ·
  ci#37907730264 push completed/failure`; on `b3e5859`: `ci#37954318153 pull_request in_progress/-`; on `569604b`
  with `--wait 30 --name supply-chain`, a settled green: `569604be1f07 verdict: green · checks 1/7` and `runs:
  secret-scan#37945547982 pull_request completed/success · ci#37945548047 pull_request completed/success`). The
  build branch's tip, the second branch's tip and `main`'s merge commit are three different shas, so no read can
  meet another's run; each read still asserts its event word.
- **The second branch is the build branch's own diff, applied to `main`.** Measured in a scratch clone (git
  2.55.0; the project's repository not written): the diff of the build branch against `b3e5859` for the two
  files, taken when it holds only the audit-step edit, the pin and the gate-list line (3 hunks, 51 lines),
  passes `git apply --cached --check` on an index read from `60ef43c` (exit 0), and the tree it writes there
  equals the tree of the same edits made directly on `main` (`3809cc759901398ee34644e387734d5603bdee2a` both).
  `git apply` places the hunks by context, and the context is the same on both branches (above). Two facts about
  the patch file itself, measured there: this host's git writes its headers with the prefixes `c/` and `w/`
  (`git config --get diff.mnemonicPrefix`: true), so a count of files reads the `diff --git` lines (2), and the
  patch is taken with `--src-prefix=a/ --dst-prefix=b/` to read the usual way; and `git apply --check --reverse`
  of the patch exits 1 on the build base without the edits, 0 on the edits plus the build-only pin, and 1 on a
  pin differing by one token.
- **How the two branches meet again, measured by trial merges** (`git merge-tree --write-tree`, the same scratch
  clone; `main-after` is `60ef43c` plus the second branch merged with a merge commit, as PR #39 was merged;
  `build-after` is `b3e5859` plus this chunk's edits):
  - `main-after` with `build-after`: clean, exit 0; the merged tree equals `build-after`'s tree, whole. The
    version's eventual merge takes the repair once and changes nothing else for it.
  - `main-after` with the build branch as it stands at `b3e5859`: clean, exit 0; the merged workflow holds one
    `run: cargo audit` line and no action line, the merged test file one pin. So pull request #40 keeps a
    computable merge even if `main` receives the repair before the build branch is pushed.
  - `60ef43c` with `b3e5859`, today: clean, exit 0.
  - The control, `main-after` with a build branch whose pin differs from `main`'s by one token (`token:` for
    `token`): exit 1, `CONFLICT (content): Merge conflict in pulse-app/tests/quality_gate_workflow.rs`. The clean
    readings hold only while the shared hunks are byte-identical.
- **Where the build-only pin goes.** The trigger pin is inserted before
  `nextest_load_profiles_profile_preserves_zero_flake_posture` (`quality_gate_workflow.rs:148-149`), inside the
  region the previous chunk already changed on the build side alone (`@@ -98,16 +98,50`); `build-after` in the
  trial carried it there and merged clean.
- **The cache and `main`'s workflow.** `main`'s workflow still holds the `release` job and the two matrices, so its
  runs restore, and may save, the keys the build branch no longer uses. The two rounds this chunk runs on that
  workflow are therefore expected to touch the four orphaned entries on `refs/heads/main`. A forecast: the wrap's
  cache listing measures it.

## Sweeps
- `audit-check|actions-rust-lang/audit` over `*.md`, `*.yml`, `*.rs`, `*.toml`, `*.json` outside `target/`,
  `node_modules`, the chunk folders, the run dirs and the amendment sidecars: 12 hits · 1 changed (`ci.yml:427`) ·
  11 no-change — 2 spec-master lines corrected at the wrap (`security-plan.md:216`, `:259`); 4 lines of earlier
  phase artifacts under `.andromeda/phases/` (history); 2 handoff lines (the wrap's); `intent.md:403` and
  `verification-matrix.json:493` (their coordinate `:493-496` is the step's place on `main`, where it still
  stands); `working-route.md:13`, the frozen entry's own freight.
- `ci\.yml` readers (`grep -rln` over `*.rs`, `*.sh`, `*.ps1`, `*.yml`, `*.py`, `*.toml`, `*.json` outside
  `target/`, `.andromeda/` and the chunk folders): 6 files · 1 changed (`quality_gate_workflow.rs`) · 5 no-change
  — `a11y_perf_workflow.rs` and `pre_push.rs` read nothing the edit moves; `release.yml:47` and
  `update-channels.yml:69` name the file in a comment each; `verification-matrix.json` names it in P-120's
  `observed_gap` text.

## New files to create
- none

## Files to modify
- `.github/workflows/ci.yml` — the audit step becomes a plain run step
- `pulse-app/tests/quality_gate_workflow.rs` — the audit-step pin, one more gate substring in the soft-fail ban, the trigger pin

## Open questions
- none
