# Report — 2026-10-09-supply-chain-job-same-on-push-and-pull-request

**Chunk:** Supply-chain job same on push and pull request: the audit step fails on a finding, never on its own reporting; repair witnessed on a push
**Date:** 2026-10-09T17:51:01Z
**Commits:** on `build/andromeda-pulse-0.4.0` since the last wrap, `f36a2ac3 chore(2026-10-09-supply-chain-job-same-on-push-and-pull-request): operator pre-CI commit` (`git log --format='%h %s' b3e5859..HEAD`: 1). On `main`, not on this branch: `5b307e4c fix(ci): cargo audit runs as a plain step` (the second branch's one commit) and `178ebac5 Merge pull request #41 from Turbolet85/hotfix/p-120-audit-step` (the founder's merge).

## Changes (structured — detectors read this)
- **Files:** `.github/workflows/ci.yml` (1 insertion, 3 deletions) · `pulse-app/tests/quality_gate_workflow.rs` (67 insertions) — `git diff --stat b3e5859 -- . ':!.andromeda' ':!andromeda-pulse-0.4.0' ':!.claude'`: 2 files, 68 insertions, 3 deletions. Evidence, in the chunk folder: `evidence/{red-before.md, mutation-checks.md, audit-control.lock, audit-control.md, main-hotfix.patch, main-hotfix-commit.txt, main-hotfix-pr.md, operator-pass.md}`. No other source file, manifest, lockfile or script changed.
- **Symbols / APIs:** none public. Two test functions in `pulse-app/tests/quality_gate_workflow.rs`: `ci_workflow_triggers_are_pull_request_and_push_on_main` (`:148-169`, head at `:150`) and `ci_workflow_audit_step_is_a_plain_run_step` (`:301-342`, head at `:302`); one array element, `"cargo audit"`, in `ci_workflow_test_gates_no_continue_on_error`'s `gate_substrings` (`:264`). `workflow_job_block` (`:79`) gains a third caller and is unchanged. No IPC method, endpoint, port, env var or export.
- **Crates / modules:** none added, removed or changed.
- **Dependencies:** none added or bumped. One third-party GitHub Action LEFT the workflow: `rustsec/audit-check@69366f33c96575abad1ee0dba8212993eecbe998` (v2.0.0). No action was added. `Cargo.lock`, `Cargo.toml` and `deny.toml` are byte-identical to `b3e5859` (the plan's scope-guard entry: exit 0, no output).
- **Schema / config:** none.
- **Spec-master edits:** none by this chunk before the wrap.
- **Counts / qualifiers moved:**
  - Third-party actions referenced by `ci.yml`'s `supply-chain` job: one fewer (the audit action). `secrets.` references in `ci.yml`: 1 → 0 (`grep -c -E 'rustsec/audit-check|secrets\.' .github/workflows/ci.yml`: 2 at `b3e5859`, 0 now). Docs stating the action: `security-plan.md:216` and `:259` (`grep -n -E 'audit-check|actions-rust-lang/audit'` over the seven masters and `.andromeda/registries/`: 2 hits, both security-plan; 0 elsewhere).
  - `pulse-app/tests/quality_gate_workflow.rs`: 19 → 21 tests on the build branch (20 on `main`, which has the audit pin and not the trigger pin). `gate_substrings`: 15 → 16. Workspace nextest: 2800 → 2802 passed. No master states any of the three numbers (`grep -E '2800 tests|15 gate|gate substrings'` over the masters and registries: 0 hits) — none to amend, verified.
  - Jobs of `ci.yml`: 6, unchanged; trigger block unchanged (the plan's base-comparison probe: `['supply-chain'] ['cargo audit (RustSec advisory DB)'] True True True True`).
- **Dev-tool versions:** none changed. `cargo-audit` re-read at `0.22.2` on the dev host (`cargo audit --version`: `cargo-audit-audit 0.22.2`); the CI image `ubuntu-22.04` version `20261004.315.1` was read from three job logs and its software list names "Cargo audit 0.22.2" (research.md, read at P3). `gh` on the dev host reads `gh version 2.102.0 (2026-09-30)` (first recorded reading; it is the version on which `gh api …/logs` refuses to print without `--allow-escape-sequences`).
- **Harness / gate surface:**
  - `ci.yml`, job `supply-chain`, step `cargo audit (RustSec advisory DB)` (`ci.yml:426-427`): was `uses: rustsec/audit-check@69366f33…` with `with: token: ${{ secrets.GITHUB_TOKEN }}`; is the plain step `run: cargo audit` (`:427`). It runs the runner image's own `cargo-audit`; nothing installs it. Its three exits, measured on the dev host with the same version: 0 clean (informational warnings allowed) · 1 a vulnerability · 2 could not evaluate. A `run:` step fails on 1 and on 2. No `--deny warnings`: the 10 informational advisories do not fail the job, as before.
  - The step no longer receives a token and publishes nothing. The workflow-level `permissions: contents: read` and the trigger block (`pull_request`, and `push` on `main`) are unchanged; no job carries a `permissions:` key.
  - No xtask verb, agent-run script or status shape changed. `main`'s workflow received the same step change through pull request #41 and nothing else: it keeps seven jobs and its three-system matrix.
- **Cross-project / external claims:**
  - **Three CI runs, each a first attempt, none re-run** (the repository `Turbolet85/andromeds-pulse`; read through `ci.py conclusion`, `gh api …/jobs` and the job logs; records in `evidence/operator-pass.md`):
    - `ci#37961031489`, event `pull_request`, on `f36a2ac` (the build branch, draft #40; the run's `GIT_COMMIT_SHA` is the pull request's merge commit `70042b8b`): **failure**. `supply-chain` `success`, its audit step `success` (6 s), the job log holding `Scanning Cargo.lock for vulnerabilities (916 crate dependencies)` once and `Resource not accessible by integration` 0 times. `boot smoke (ubuntu-22.04)` **`failure`**; the other four jobs `success`.
    - `ci#37961092401`, event `pull_request`, on `5b307e4` (pull request #41 into `main`): **success**, 13 of 13 checks, twelve jobs `success`, audit step `success`.
    - `ci#37964887106`, event **`push`**, on `178ebac` (`main`, the founder's merge of #41 at 2026-10-09T17:14:43Z): **success**, twelve of twelve jobs; `supply-chain` check green; audit step `success` (5 s); job `113936654152`'s log holds the scan line once and the check-run denial 0 times; its token reads `Contents: read`, `Metadata: read`, as the red run's did. **This is P-120's witness.** The run it repairs: `ci#37907730264`, event `push`, on `60ef43c`, `failure` in that step.
  - `main` moved `60ef43c` → `178ebac` by the founder's hand; this pass merged nothing and pushed nothing to `main`. `178ebac`'s parents are `60ef43c` and `5b307e4` (`git log -1 --format=%P origin/main`).
  - Pull request #41: `MERGED` at 2026-10-09T17:14:43Z by `Turbolet85` (`gh pr view 41`). Draft pull request #40: open, a draft, head `f36a2ac`, `MERGEABLE · UNSTABLE` at 17:44:17Z (its run is the red one).
  - **The merged branch was deleted on the operator's directive** (inputs#I5 item 5), after `git merge-base --is-ancestor 5b307e4 origin/main` exited 0: `git branch -d hotfix/p-120-audit-step` and `git push origin --delete hotfix/p-120-audit-step`, 2026-10-09T17:50:00Z; `git ls-remote origin refs/heads/hotfix/p-120-audit-step` returns no line. Nothing else outside the tree was deleted.
  - **The Actions cache, re-read at 2026-10-09T17:49:51Z** (`gh api …/actions/cache/usage` and `…/actions/caches`): 10 entries, 12 208 662 121 B, the same count and bytes as at 15:13Z and 15:58Z; no entry evicted, none created (every `created_at` is unchanged). The four orphaned entries on `refs/heads/main` (`lint-test-Windows`, `lint-test-macOS`, `release-macOS`, `release-Windows`) read `last_accessed` 17:15Z, where they read 13:05Z before: the push run on `main`'s old workflow restored them. The two on `refs/pull/39/merge` still read 08:24Z. So "least recently used" now names the two `pull/39` entries alone, and the listing no longer tests whether GitHub evicts the six first.
  - The runner image, from all three `supply-chain` logs: `Image: ubuntu-22.04`, `Version: 20261004.315.1`.
  - The boot smoke's application logs of seven runs (`logs-boot-Linux` artifacts `11613618010`, `11632850540`, `11628170519`, `11632610741`, `11633501597` and those of `ci#37934330231` and `ci#37945548047`) were downloaded and read at this wrap; the reading is under Outcome → watches.
  - Inputs (`inputs.py verify`, 2026-10-09T17:49Z: `6 entries — unchanged 3 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 3`):
    - I1 · `../additional/pc-overseer/relays/pulse-phase-supply-chain-push-and-pr-2026-10-09.md` · copy · unchanged
    - I2 · message: the operator (pc overseer), the /andromeda-phase invocation arguments · copy · n/a — a message has no live source
    - I3 · `../additional/pc-overseer/relays/pulse-phase-p5-review-main-now-2026-10-09.md` · copy · unchanged
    - I4 · message: the operator (pc overseer), the P5 review answer · copy · n/a
    - I5 · `../additional/pc-overseer/relays/pulse-wrap-supply-chain-push-and-pr-2026-10-09.md` · copy · unchanged — snapped at this wrap, cited here as inputs#I5: the wrap directive (the witness stands; the operator-ratified reading of P-120's build-branch clause; entries 40–42 as the plan's defect; the watch recurred and needs an owner; delete the merged branch; the two carries; the stops).
    - I6 · message: the operator (pc overseer), the /andromeda-wrap-session invocation arguments · copy · n/a — snapped at this wrap, cited here as inputs#I6.
    - I7 · message: the operator (pc overseer, founder-delegated), the route-resolve card answer · copy · n/a — snapped at this wrap's P5, after this report was first written, cited here as inputs#I7: mint the boot-smoke entry first in the tail with its `CARRY:` freight; P-129 approved as written, provisional; the readiness clause kept as the contract with the measured gap beside it and the new entry as owner; both carries consumed; Epoch 1 at 15, no split (`inputs.py verify` at 18:13Z: 7 entries — unchanged 3 · n/a 4 · drifted 0).
- **Reverted / negative API facts:**
  - A `push-witness/**` trigger and a witness ref were planned at the first P5 and withdrawn before approval on the founder's ruling (inputs#I3); no trigger was ever written. The trigger block is now pinned by a test.
  - A job-level `checks: write` was never written (the plan's rejected widening); mutation 4 shows the pin refuses it.
  - Five one-shot mutations of `ci.yml` were written and reverted at /implement (`evidence/mutation-checks.md`); after the last revert the workflow diff equals the patch's first 15 lines byte for byte.
- **Insufficient fixes (written, kept, not the remedy):** none. (The repair is the remedy for the defect it was written for: the push run on `main` is green. The build branch's own run is red in another job; see Outcome.)
- **Spec claims disproved by measurement:**
  1. **"The CI boot job stops the app before the webview issues any IPC"**, so "no CI job witnesses" `ui.webgpu.adapter` and the boot job's log "holds no webview record". Stated at `obs-plan.md:132`, `obs-plan.md:545` ("23 backend records over 14 ms, 0 webview-originated", measured on `ci#36765040464`), `test-plan.md:100`, `test-plan.md:142`, `security-plan.md:447`, `architecture.md:180` (`grep -n -i -E 'stops the app|before the webview issues|before any webview'` over the seven masters: 6 lines in 4 masters; registries: 0). **Measured false as a rule** on seven boot-job logs of 2026-10-09: four hold webview-originated records (`ui-bridge.ready`, `viz.query.traces`, `services.list_with_states.request`, `connection.current_state.request`, and `ui.webgpu.adapter` with `outcome: no_navigator_gpu`, WARN) — `ci#37924991598`, `ci#37945548047`, `ci#37954318153`, `ci#37961031489`; three hold none — `ci#37934330231`, `ci#37961092401`, `ci#37964887106`. Whether the webview gets that far depends on how long the app lives before `cleanup` stops it, which varied from 0.57 s to 1.40 s. The claim was true of the one run it was measured on. The same claim stands at two more obs-plan sites the grep above does not match ("its only live witness", the §8 `ui.webgpu.adapter` leaf; the §10 CI-gates bullet's `ci#36765040464` reading), found by the obs-plan detector. Read after the fan-out, from the boot job logs: the `ci-gates` frame line follows the log — `frame: cannot-evaluate: 0 samples, no WebGPU adapter (no_navigator_gpu)` on `ci#37945548047` and `ci#37954318153`, `… no adapter record in this log` on `ci#37934330231`, `ci#37961092401` and `ci#37964887106` (the step was skipped on the two red runs).
  2. **`cargo audit` "using `actions-rust-lang/audit` action"** (`security-plan.md:216`; the action named again at `:259`). The workflow never used that action: it used `rustsec/audit-check` until this chunk and uses no action now. (The entry's second `CARRY:`; corrected at this wrap to what ships.)
  3. **"until then a fix lands on the version's build branch and reaches `main` with the version"** (`security-plan.md:226`, the founder's ruling of 2026-10-07). This repair reached `main` ahead of the version, by a separate pull request, on the founder's ruling of 2026-10-09 (inputs#I3). The sentence's direction otherwise stands; the departure is one dated instance.
  4. **P-120's acceptance, one clause** (`verification-matrix.json#P-120`, a ledger, not a master): "the pull-request run of the build branch is green on the same repair". `ci#37961031489` is red, in `boot smoke` alone; its `supply-chain` check and audit step are `success`. The operator ratified the reading that the clause is about the supply-chain check of that run, as the requirement is (inputs#I5 item 2: operator, the pc overseer, founder-delegated, 2026-10-09 — relayed by the pc overseer). Owner: P7.3's `refine`.
  5. **Three forecasts of the plan** (plan.md, not a master): the `supply-chain` job "reaches the audit step in under a minute on every run" — measured 89 s, 90 s and 64 s; "the build branch's run registers 7 checks" — held; the second branch's run 13 — held. And the plan's entries 40–42 (below, Deviations).
- **Expected amendments (from plan):**
  - architecture §Infrastructure Patterns → CI/CD approach (the audit step named as a plain `cargo audit`; the fourth dated cache reading) — **carried**: Harness / gate surface, and Cross-project (the cache re-read). Site: the key file `.andromeda/registries/contracts/architecture/ci-cd-approach.md` (`grep -c -F 'cargo audit'`: `architecture.md` 0; that key file 1 line, its `supply-chain` clause, which names the auditable build and no audit step).
  - security-plan §Dependency Security → CI integration (a plain `run:` step, no action, no token; its three exits) — **carried**: Harness / gate surface; disproved claim 2. Site: `security-plan.md:216` (`grep -n -E 'audit-check|actions-rust-lang/audit' .andromeda/security-plan.md`: 2 hits, `:216` and `:259`).
  - security-plan §Bootstrap phases `dep-security-ci-gate` (the same action name, the second site) — **carried**: disproved claim 2. Site: `security-plan.md:259`.
  - security-plan §Dependency Security → the Critical CVE response paragraph (a dated departure) — **carried**: disproved claim 3. Site: `security-plan.md:226` (`grep -c -E 'reaches .main. with the version'`: security-plan 1, the other six masters 0).
  - test-plan §9 Pipeline structure, the Supply chain row (gains the step and its exits) — **carried**: Harness / gate surface. Site: `test-plan.md:494` (`grep -n 'Supply chain' .andromeda/test-plan.md`: the one table row; it names setup-node, the Cranelift assertion, the npm gate and the auditable build, and no `cargo audit` step).
- **Coverage of new surfaces:**
  - `ci.yml` step `cargo audit (RustSec advisory DB)` as a plain run step → validation n/a · instrumentation n/a (a CI step; its record is the job log and the run id) · PII n/a · tests unit (`ci_workflow_audit_step_is_a_plain_run_step`, `ci_workflow_test_gates_no_continue_on_error`; five mutations) + the three CI runs + the local red control (`cargo audit --file evidence/audit-control.lock`: exit 1, `RUSTSEC-2020-0071`) · a11y n/a · tokens n/a
  - `ci.yml` trigger block, pinned as it stands → tests unit (`ci_workflow_triggers_are_pull_request_and_push_on_main`; one mutation) · everything else n/a
  - No new external-input surface, hot-path operation, log target or UI element.

## Deviations from intent
- **Entries 40, 41 and 42 of the plan's gate block were defective as written.** Each pipes `gh api "repos/…/actions/jobs/{id}/logs"` into `grep`. On this host that call prints nothing without `--allow-escape-sequences` (it says so on its error stream), so `grep` read an empty stream: entry 40 (`expect exit 0`) read **red** on its own defect; entry 41 (`expect exit 1`, `last line 0`) read **green by its atoms, vacuously** — it would read the same over a log full of denials — and is not evidence; entry 42 (report-only) printed nothing. The log was then read once in a corrected form, the same call with the flag, saved to a file and counted from the file: the scan line 1, the denial 0, the image lines as recorded. P-120's two log clauses rest on that read. The handoff's host note already named the flag; the plan's entries did not carry it.
  - **What a later plan must write so that an empty stream cannot read green:** a log read is a two-step entry — fetch to a file with `--allow-escape-sequences` and assert the file is not empty (`test -s`, or a `contains` atom on a line every job log holds, such as `Image: `), then count from the file. An absence count (`expect exit 1`, `last line 0`) is never asserted over a pipe from a producer whose failure also prints nothing; it carries a known-present control read by the same form in the same entry.
- **The build branch's pull-request run is red** (`boot smoke (ubuntu-22.04)`), where the plan's acceptance for P-120 reads it green. The repair's own job and step are green on that run. Not re-run. See Outcome.
- **Three mutations beyond the plan's two** (`|| true` on the run line; a job-level `permissions:`; the action re-added as a second step), one per remaining assertion of the audit pin. Each went red and was reverted. Justification: the red-before reading stops at the pin's first assertion.
- **One local reading the plan did not list:** `main`'s test file with the patch applied, compiled alone with `rustc --test` in the scratchpad against `main`'s patched workflow: 19 of 19. Justification: no workspace build, and it was the only reading of `main`'s side before the pull request's own run. Not a gate.
- **Two comment lines head `evidence/audit-control.lock`**, marking it as evidence; the plan gave its content as two packages. The control entry reads it green.
- **The operator pass crossed a host restart** between entries 31 and 32 (16:54Z). The wait it killed was a read; entry 31 was not driven again. Recorded in `evidence/operator-pass.md`.
- **Entry 45's reading** was `UNKNOWN · UNKNOWN` (GitHub had not recomputed); a second read 22 s later is recorded beside it as a new read.
- scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 2 · listed 2 · recorded 0 … · excluded 64`, base `b3e5859`, the parent of the pre-CI commit).

## Decisions & corrections
- **The founder's ruling, 2026-10-09** (inputs#I3; founder, by dialog — relayed verbatim by the pc overseer): `main` is repaired now by a separate pull request he merges; the push run his merge makes is the witness. He merged #41 himself.
- **The operator's ratified reading of P-120's build-branch clause** (inputs#I5 item 2; operator: the pc overseer, founder-delegated, 2026-10-09): the clause reads on the supply-chain check of the build branch's run, as the requirement's subject is the supply-chain job; "an individually ratified refinement … not a weakening"; the run's red is not hidden by it.
- **The operator's direction on the watch** (inputs#I5 item 4): it recurred and needs an owner, not another watch; the lean, for the card to test, is an entry of its own, first in the markerless tail, that finds the cause and closes it.
- **The operator's words during the pass**, quoted whole in `evidence/operator-pass.md`: run entries 24–36 and stop; resume after the restart, re-run nothing; run entries 37–45 after the merge, write P-120's ref, stop. "Re-run nothing" held throughout.
- **The second branch was made from the build branch's own diff**, never typed twice; the trial merges before and after the founder's merge both write the build branch's own tree (`c44633d3…`).
- **Sweep hazards found:**
  - `cargo audit` is a prefix of `cargo auditable`: a fixed-string grep for the step, and the new `"cargo audit"` gate substring, both match the auditable build step and one comment on `main`. Read before adding it: no block that matches carries `continue-on-error`, on either branch. A count of `cargo audit` in a master counts the auditable build too (`test-plan.md:491`, `:494`).
  - `gh api …/actions/jobs/{id}/logs` piped into `grep -c`: prints `0` and exits 1 whether the log holds no match or `gh` refused to print. The refusal goes to the error stream, so a `2>&1` capture shows it and a pipe does not.
  - `ci.py conclusion` returns at the first failed check, with the run still open: a step or jobs read taken right after it reads an unfinished run.
  - `ci.py conclusion`'s `contains pull_request completed/success` atom is satisfied by the `secret-scan` run's words when the `ci` run is still `in_progress` (entry 31: it held while the verdict was red).
  - A `pull_request` run's `GIT_COMMIT_SHA` is the pull request's merge commit, not the branch tip the read names.
- **Hook behaviour met:** the Bash guard refused one heredoc-with-file-target call during the pass (a stray `cat >> /dev/null <<'EOF'` line); the records went through the Edit tool.

## Outcome
**Acceptance criteria, each against the diff and the recorded runs:**
- (capability) P-120 — **met on the requirement, one clause of the concrete acceptance refined.** The push run on `main` (`ci#37964887106` on `178ebac`) is green in `supply-chain`, its audit step `success`, its job log holding the scan line and no check-run denial (the corrected read); the step is `run: cargo audit` with no action, no token and no soft-fail, the job without a `permissions:` key (the parsed-workflow probe: `1 ['name', 'run'] cargo audit False`; the pin); the control ends 1 naming `RUSTSEC-2020-0071` on the dev host with cargo-audit 0.22.2. "The pull-request run of the build branch is green": **the run is red** (`ci#37961031489`, boot smoke); its `supply-chain` check and audit step are `success` → the operator-ratified reading (inputs#I5 item 2) → P7.3's `refine` with both run ids, before the flip. Both run ids are named.
- (security) workflow-level `permissions:` is `contents: read` alone, no job-level key — met (base-comparison probe; audit-step probe; mutation 4).
- (security) no `uses:` line added or changed, no `secrets.` reference — met (the diff-add probe: 0; `grep -c`: 0). One `uses:` line was removed.
- (security) the audit step turns red on an advisory above warning — met on the dev host (the control: exit 1, one distinct id); **not exercised on a runner** (stated in `evidence/audit-control.md`). No `deny.toml`, ignore flag or lockfile change — met (scope guard).
- (security) `harden-runner` remains the first step of `supply-chain` — met (the audit step is the job's only step that differs).
- (arch) six jobs on `ubuntu-22.04`, no matrix, no `needs:`, trigger block unchanged — met (`6 of 6`; the base-comparison probe).
- (arch) `supply-chain` writes no cache key; its auditable build step unchanged — met.
- (arch) no Rust outside one test file, no workspace member, no env var — met (scope guard).
- (tests) the standard gate set passes in order, closing with the bindings base-identity probe exit 0 — met at /implement (23 of 23); re-run at this wrap's light gate.
- (tests) every test that reads `ci.yml` passes in the workspace nextest — met (2802 passed, 0 skipped; `quality_gate_workflow` 21, `a11y_perf_workflow` among them); on CI, `lint / test (ubuntu-22.04)` `success` on all three runs.
- (tests) what `main` received is what the build branch ships and nothing more — met: the patch names two files, applied to `60ef43c`, is still held byte for byte by the tree (the three patch entries); the second branch differed from `main` in those two files (entry 27); its run green, audit step `success` (entries 34, 35).
- (tests) the red control is recorded with evidence that the audit ran and reported a finding; the push reading with the step's own log line; neither from an exit code alone — met, the push reading through the corrected log read (entries 40–41 as written do not carry it).
- (tests) each of the three readings is a first run's conclusion; no run re-run — met (attempt 1 on all three).
- (obs) the log-artifact upload steps of `lint-test` and `boot` unchanged — met (base-comparison probe).
- (obs) each witness names a run id and the job-log text an agent read — met.
- (obs) no artifact upload added to `supply-chain`; the distinct-name check not applicable — met.
- (a11y) no line inside the `a11y` job changes; the `on:` block untouched — met.
- (a11y) every run read records its `a11y` conclusions as read; none re-run — met: `success` on all seven `a11y` legs of the three runs (278 s on the build run).
- (a11y) `a11y_perf_workflow.rs` passes on the chunk's tree — met.

**Gates, the plan's entries by `run`, at /implement (one `gate.py run`, first run, 0 fix iterations) and in the operator pass:**
- `cargo fmt --check` — green · `cargo clippy --workspace --all-targets --all-features -- -D warnings` — green
- the scope guard (`git diff --name-only b3e5859… -- crates pulse-app xtask …`) — green (exit 0, no output)
- the four parsed-workflow probes — green (`a11y boot coverage lint-test mcp-test supply-chain` · `6 of 6` · `['supply-chain'] ['cargo audit (RustSec advisory DB)'] True True True True` · `1 ['name', 'run'] cargo audit False`)
- `grep -c -E 'rustsec/audit-check|secrets\.' .github/workflows/ci.yml` — green (exit 1, `0`) · the diff-add probe — green (exit 1, `0`)
- `cargo audit` — green (exit 0, scan line, 10 allowed warnings) · `cargo audit --file …/evidence/audit-control.lock` — green (exit 1, `RUSTSEC-2020-0071`, `error: 1 vulnerability found!`)
- the three patch entries (`grep -c '^diff --git '` · `git apply --cached --check` on a throwaway index from `60ef43c` · `git apply --check --reverse`) — green
- `cargo xtask check:english-sources` · `capability-widening-check` · `check:ingest-progress` · `check:staged-artifacts` · `capability-drift` · `verify:capability-matrix` — green
- `cargo nextest run --workspace --profile ci` — green (2802 passed, 0 skipped) · the `--features mcp-server` `emit_taurpc_bindings` regen — green · `git diff --quiet b3e5859… -- pulse-app/ui/src/bindings/index.ts` — green
- **`leg = 'operator'`, driven once by hand, records in `evidence/operator-pass.md`:**
  - `gate.py hygiene` — green · the build-branch push — green (`b3e5859..f36a2ac`)
  - the second-branch entry (`git fetch origin main && test … && git worktree add -b hotfix/p-120-audit-step …`) — green (`5b307e4`) · `git diff --name-only origin/main hotfix/p-120-audit-step | paste -sd' '` — green · `git merge-tree --write-tree --name-only hotfix/p-120-audit-step HEAD` — green · the second branch's push — green · `gh pr create … --base main --head hotfix/p-120-audit-step` — green (#41)
  - `ci.py conclusion --sha HEAD --wait 2700` — **red · `contains verdict: green` failed**: `ci#37961031489` on `f36a2ac`, `failure`, in `boot smoke (ubuntu-22.04)` alone. **`red — not this chunk's`: basis** — the same job, same step, same signature (`boot` prints ready, `status` reads `"ended": "exit 1"`) red on `b3ac58a` (`ci#37924991598`), a tree without this chunk's edits, and green on `5b307e4` and `178ebac`, two trees WITH this chunk's repair; this chunk's two files are not built or read by the boot job's failing step (a workflow step of another job, and a test file the release build does not compile). **→ owner** — this wrap's P5 Recurrence-watch disposition (the card). It is a runner-only red with no two-sided probe: the basis is the earlier red on this branch without the chunk, not a control run minted for it, and the wrap says so on the card.
  - the audit-step read of that run (`gh api "…/actions/runs/<id>/jobs?per_page=100" --jq '… select(.name == "cargo audit (RustSec advisory DB)") …'`, `<id>` 37961031489) — green · its jobs listing — recorded: five `success`, `boot smoke` **`failure`** (the red above, its owner the same), `a11y` `success`
  - `ci.py conclusion --sha hotfix/p-120-audit-step --wait 2700` — green (`ci#37961092401`, 13/13) · its audit-step read — green · its jobs listing — recorded: twelve `success`
  - `git fetch origin main && git show origin/main:.github/workflows/ci.yml | grep -c -E '^        run: cargo audit$'` — green (`1`)
  - **`ci.py conclusion --sha origin/main --wait 2700 --name supply-chain` — green: the witness** (`ci#37964887106`, `push completed/success`, no `pull_request` in the output)
  - its audit-step read — green
  - `gh api "…/actions/jobs/$(…)/logs" | grep -c 'Cargo.lock for vulnerabilities'` — **red · `exit 0` failed (exit 1, `0`)**: the entry's own defect, an empty stream; the corrected read: 1. Not a finding about the run. Owner: this report's Deviations; no route owner needed — the clause it served is proven by the corrected read.
  - `gh api "…/logs" | grep -c 'Resource not accessible by integration'` — green by its atoms (exit 1, `0`), **vacuous, not counted**; the corrected read: 0.
  - `gh api "…/logs" | grep -E 'Image: |Included Software: '` — recorded: nothing printed (exit 1); the corrected read: `Image: ubuntu-22.04`, `Version: 20261004.315.1`.
  - the push run's jobs listing — recorded: twelve `success`; **`main` as a whole reads green**.
  - `git merge-tree --write-tree --name-only origin/main HEAD` — green (the merged tree is the build branch's)
  - `gh pr view 40 … --json mergeable,mergeStateStatus` — recorded: `UNKNOWN · UNKNOWN`, then `MERGEABLE · UNSTABLE`.
- No deferral: the chunk's Rust delta is one test file and every gate ran. Smoke: skipped — no boot-path or UI-surface change.

**Watches:**
- boot smoke on ubuntu-22.04 (folded `watch:`, 2/3 at take-up) · **RECURRED `ci#37961031489` on `f36a2ac`** · first diagnostic: `"ended": "exit 1"` / `"verdict": "not-running"` 0.67 s after `boot: ready (PID=7072, …)`, then `##[error]Process completed with exit code 1.` Other readings this chunk: green `ci#37954318153` on `b3e5859` (read at P3), green `ci#37961092401` on `5b307e4`, green `ci#37964887106` on `178ebac`. All readings since the first red: `b3ac58a` red · `0b61bfb` green · `569604b` green · `b3e5859` green · `f36a2ac` RED · `5b307e4` green · `178ebac` green — two reds in seven, both pull-request runs of the build branch.
- **The application logs, read at this wrap** (seven `logs-boot-Linux` artifacts; `agent-latest.jsonl.2026-10-09`, `boot.log`, the boot job logs; times are seconds after the app's first log record):

| Run · commit | Verdict | ready printed | first webview-originated record | last record | `status` printed | `app.exit` |
|---|---|---|---|---|---|---|
| `ci#37924991598` · `b3ac58a` | RED | +0.61 | +0.76 | +0.85 | +1.25 `not-running`, `exit 1` | none |
| `ci#37934330231` · `0b61bfb` | green | +0.33 | none | +0.71 | +1.14 `running-healthy` | +1.16 `sigterm` |
| `ci#37945548047` · `569604b` | green | +0.51 | +1.14 | +1.27 | +1.40 `running-healthy` | +1.40 `sigterm` |
| `ci#37954318153` · `b3e5859` | green | +0.48 | +1.17 | +1.20 | +1.39 `running-healthy` | +1.40 `sigterm` |
| `ci#37961031489` · `f36a2ac` | RED | +0.91 | +0.91 | +0.93 | +1.58 `not-running`, `exit 1` | none |
| `ci#37961092401` · `5b307e4` | green | +0.24 | none | +0.66 | +0.93 `running-healthy` | +0.94 `sigterm` |
| `ci#37964887106` · `178ebac` | green | +0.01 | none | +0.02 | +0.56 `running-healthy` | +0.57 `sigterm` |

  - **What the two red logs hold that no green log holds: nothing.** Each red log has the same one ERROR every log has (`corpus.open.error`, `error_kind: KeyringUnavailable`) and the same warning targets; no `app.panic.fatal`; `boot.log` holds the one AT-SPI warning all seven hold. Red `b3ac58a`'s record set equals green `b3e5859`'s target for target, less the exit record.
  - **What the two red logs lack: an `app.exit` record.** Every green log ends with `app.exit` `exit_class: signal`, `signal: sigterm` — the smoke's own `cleanup`. The red logs end on a webview-originated record and then stop; the wrapper read `exit 1`. The product's exit record covers every end except the ones its contract names as unloggable; so the logs say the app ended by itself with code 1 in a way that wrote no record, and they do not say how.
  - **What separates red from green in these seven: when `status` looked, relative to the webview's first call.** In no run did the app live longer than 1.40 s before the smoke stopped it. In the four runs where the webview's first calls were logged, `status` read the app 0.23 s and 0.27 s after them (alive, green) or 0.49 s and 0.67 s after them (ended, red). In the three runs with no webview record the app was stopped before that point, at +0.57 s to +1.16 s of life. The app's age alone does not separate them: alive at +1.39 s twice, ended by +1.25 s once.
  - **What this does not show.** It does not show a cause. It does not show that the end belongs to the window: by the time of the webview's first calls the receivers are bound and the first ticks have fired, so every part of the engine is running too. It is four runs. No log shows the app alive more than 0.27 s after the webview's first call on a runner, so the logs cannot say whether a green run would have stayed up.
  - A second fact these logs show: `boot` prints ready as soon as `harness:status` reads a live pid and a written log (12 ms after the first record on `178ebac`), before the OTLP receivers bind; on `178ebac` the app was stopped having logged neither `app.boot.otlp.grpc.bind` nor any tick.

**Outcome basis:** the operator pass ran, so the verdicts rest on its final state: the commit list above (one commit on the build branch, `f36a2ac`), the three CI runs recorded in `evidence/operator-pass.md`, and the founder's merge `178ebac`. /implement's P4 report as given in this session stays the basis for the 23 local gate entries, the red-before reading, the mutations and the control. Operator directives between implement and this report: the three pass words (quoted in `operator-pass.md`) and the wrap directive (inputs#I5, inputs#I6). P-120's ref was written at the end of the pass on the operator's word; its text states both discrepancies.

**Process hygiene** (implement's census, re-measured at 2026-10-09T17:51Z against `ps -eo pid,comm,etimes,args`):

| Process | Started by | Final state |
|---|---|---|
| `gate.py run` and its 23 entries; four targeted `cargo nextest` runs, five mutation runs, `cargo audit` ×3 | /implement | terminated |
| `ci.py conclusion` ×3 (entries 31, 34, 38) | the operator pass | terminated (returned 16:53:30Z, 17:10:12Z, 17:42:58Z) |
| `gh run watch 37961031489`, first | the operator pass | ended with the host restart at 16:54Z, wrote nothing |
| `gh run watch 37961031489`, second | the operator pass | terminated (returned 17:09:28Z) |
| the scratch worktree `{tmp}/pulse-p-120-main-hotfix` | entry 26 | removed by the entry |
| `scripts/code-graph.py refresh` | this wrap | terminated (exit 0) |
| `cargo`, `rustc`, `cargo-mutants` under another project's tree (`escher`) | not this session | left running — not this session's |
## New text, by line
Generated by `cites.py added` (cites v1.3); pasted by `splice.py`. No line of this section is typed or edited.
The diff: b3e58597 (the parent of the oldest pre-CI commit f36a2ac3) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/workflows/ci.yml — added 1 line(s) in 1 range(s)
added: 427
### pulse-app/tests/quality_gate_workflow.rs — added 67 line(s) in 3 range(s)
added: 148-170 · 264 · 301-343
- 148-169 @150 «fn ci_workflow_triggers_are_pull_request_and_push_on_main() {»
  - 153-156 «let start = lines»
  - 157-162 «let triggers: Vec<&str> = lines[start + 1..]»
  - 163-168 «assert_eq!(»
- 301-342 @302 «fn ci_workflow_audit_step_is_a_plain_run_step() {»
  - 307-310 «let start = job_lines»
  - 311-316 «let step = job_lines[start + 1..]»
  - 317-322 «assert!(»
  - 323-329 «for banned in ["uses:", "with:", "token", "continue-on-error", "||"] {»
  - 330-336 «assert!(»
  - 337-341 «assert!(»
