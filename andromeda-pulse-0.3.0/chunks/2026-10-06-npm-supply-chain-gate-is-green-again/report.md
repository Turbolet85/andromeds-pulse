# Report — 2026-10-06-npm-supply-chain-gate-is-green-again

**Chunk:** the npm supply-chain gate reads green again: seroval and source-map-js advisories fixed at source or excepted with a closing condition
**Date:** 2026-10-06 (UTC; report authored 23:07Z)
**Commits:** `fc4ebdf` chore(2026-10-06-npm-supply-chain-gate-is-green-again): operator pre-CI commit (the one commit since `b1fcba5`, the chunk base; `git log --format='%h %s' b1fcba5..HEAD`)

## Changes (structured — detectors read this)
- **Files:** `pulse-app/ui/package-lock.json` — the one codebase file changed (`gate.py scope`: `changed 1 · listed 1`; base `b1fcba5`). Nothing else under `pulse-app`, `xtask`, `crates`, `scripts`, `.github`, or the Rust manifests differs from the base (the plan's scope-guard entry: exit 0, no output). `package.json` and `pulse-app/ui/npm-policy.json` are byte-identical to the base.
- **Symbols / APIs:** none — no source file changed. No IPC method, endpoint, port, env var, xtask verb or export added, removed or changed. The TauRPC bindings equal the chunk base (`git diff --quiet b1fcba5 -- pulse-app/ui/src/bindings/index.ts`: exit 0).
- **Crates / modules:** none.
- **Dependencies:** none added, none removed (lockfile entries 884 before and after; the added-entry guard printed `0`). Three TRANSITIVE npm packages bumped inside their dependents' existing semver ranges, lockfile only:
  - `seroval` 1.5.4 → 1.6.8 (runtime class, MIT; dependent `@tanstack/router-core` 1.169.2, `^1.5.4`)
  - `seroval-plugins` 1.5.4 → 1.6.8 (runtime class, MIT; dependent `@tanstack/router-core` 1.169.2, `^1.5.4`)
  - `source-map-js` 1.2.1 → 1.2.2 (dev class, BSD-3-Clause; dependents `@tailwindcss/node` 4.2.4 and `postcss` 8.5.26, `^1.2.1`)
  - Basis: `git diff -U0 b1fcba5 -- pulse-app/ui/package-lock.json` — 9 lines out, 9 in, the `version` / `resolved` / `integrity` lines of three entries (`evidence/lockfile-delta.md`). No direct dependency, no pinned major and no license field moved. No telemetry SDK, no ARIA-component family and no `web-vitals` entry entered the tree (no entry was added).
- **Schema / config:** none. `npm-policy.json` unchanged: its exception set stays 2 advisory (`extract-zip` GHSA-7pqw-9j4j-h8q3, GHSA-jmr9-qjv8-65gv) + 2 license; no `overrides` entry added to `package.json`.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:** the npm gate's CURRENT STATE as security-plan §Dependency Security → npm channel records it (`.andromeda/security-plan.md:222`, the dated current-state run ending "Re-read at chunk `2026-09-29-scrubber-path-false-positive`: still `green-with-dispositions` with the same four exceptions …"):
  - Between the last green CI run (`5f77859`, supply-chain job 2026-10-05T16:22Z–16:28Z) and this chunk the gate read exit 1 `findings-red` on CI (`1124148`, `373b576`, `ce5857b`, `b1fcba5`) and on this host, with `advisory.distinct` 5: the 2 excepted `extract-zip` ids and 3 unexcepted — `seroval` GHSA-p6vx-979v-rg4c (critical), `seroval` GHSA-jp82-f5mq-hwhp (high), `source-map-js` GHSA-68fv-2mgg-jv7q (high). No in-repo file the gate reads moved between green and red (`git diff 5f77859 ce5857b` over the lockfile, `package.json`, the policy, `xtask/`, `.github/`: 0 lines); the two `seroval` advisories were published 2026-10-05T23:40Z, the `source-map-js` one was published 2026-09-18 and last updated 2026-10-05T23:31Z (`gh api /advisories/{id}`, research.md §Scope).
  - After this chunk (measured 2026-10-06 ~22:34Z on this host, `evidence/advisory-record.md`): exit 0, arm `green-with-dispositions`, `advisory.distinct` 2, both excepted, `unexcepted []`; license arm `checked 883 · excepted 2 · runtime_packages 37 · violations []`; ban arm `hits []`. Each of the three had a fixed release inside its dependents' range, so NO exception was taken and `package.json` is untouched: in-range lockfile bumps closed them (the versions above). The exception set is unchanged at 2 advisory + 2 license; neither `extract-zip` closing condition fired.
  - Sites stating the gate's current state (`grep -c 'green-with-dispositions\|extract-zip'` per master, 2026-10-06): security-plan 1 line (`:222`, the record above — the owner) · architecture 1 line (`:249`, the six-arm CONTRACT list, a contract statement, not a current-state claim — no change) · a11y-plan 1 line (`:39`, "pa11y-ci 4.1.1 … covered by the npm-policy extract-zip exception" — still true, no change) · the other four masters 0 · `.andromeda/registries/` 0 files · `CLAUDE.md`, `.claude/rules/security.md`, `.claude/rules/verification-harness.md`, `.claude/docs/commands.md`, `.claude/docs/security-summary.md` 0 lines each.
  - Package names (`grep -c 'seroval\|source-map-js'`): 0 in all seven masters and 0 files under `.andromeda/registries/`, so no version cell names them.
- **Dev-tool versions:** none — npm re-read at 11.19.1 and Node at v26.8.2 on the dev host; CI's runners read npm 11.19.0 / Node v24.21.0 (the red job's log, research.md). `seroval`, `seroval-plugins` and `source-map-js` are lockfile-resolved packages, NOT this line's subject.
- **Harness / gate surface:** none — `xtask/src/npm_gate.rs`, the workflows and the gate's exit contract are untouched. One behaviour reading, not a change: on the red commits step 14 of the CI `supply-chain` job (`cargo xtask check:npm-supply-chain`) ended the job, so steps 15–18 (`npm ci`, `npm run build`, install cargo-auditable, the auditable release build) read `skipped`; on `fc4ebdf` all of them ran and read `success`.
- **Cross-project / external claims:**
  - CI run **ci#37541745674** on `fc4ebdfe6b32964a82529f8cd8071c7041d084e6`: `completed/success`, attempt 1; with secret-scan#37541745786, checks 13/13, `verdict: green` (`ci.py conclusion --sha HEAD --wait 2400`, fired 22:37:22Z, returned 23:05:47Z; `evidence/operator-pass.md`). Twelve jobs each `success`; the `supply-chain` job (112536140445) with steps 9, 10, 11, 14, 15, 16, 17, 18 each `success`, none skipped (`gh api …/runs/37541745674/jobs`). The verdict was taken on `fc4ebdf`; this wrap's own commit adds spec, route and record files on top of it and no codebase file. The overseer states an independent read of the same run on GitHub (the wrap directive, below).
  - GitHub advisory records (`gh api /advisories/…`, read at research): GHSA-p6vx-979v-rg4c first patched `seroval` 1.6.2; GHSA-jp82-f5mq-hwhp first patched 1.6.3; GHSA-68fv-2mgg-jv7q first patched `source-map-js` 1.2.2. None withdrawn.
  - npm registry (`npm view`, read at research; re-resolved by `npm update` at implement to the same versions): `seroval` and `seroval-plugins` latest 1.6.8, `source-map-js` latest 1.2.2.
  - `I1 · message: the overseer (founder-delegated), in the /andromeda-phase invocation, 2026-10-06 · copy message · n/a — a message has no live source` — cited at scope.md:12, :15, :37 · research.md:7 · plan.md:21, :54, :99 (in fence), :254, :269, :272. `inputs: 1 entries — unchanged 0 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 1 · uncited 0 · unparsed 0`. No drift.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none. Scope note, not an insufficiency: the fix is on `chore/migrate-pulse-to-v3` only (see Outcome, open item).
- **Spec claims disproved by measurement:**
  - test-plan §9 CI Integration, the **Supply chain** row (`.andromeda/test-plan.md:494`) states "deliberately NO `npm ci` (the license walk and `npm audit` are lockfile-only, so node_modules is never installed in this job)". Measured false as a statement about the JOB: on `fc4ebdf` the `supply-chain` job ran step 15 `npm ci (pulse-app/ui)` and step 16 `npm run build` after the gate step, both `success` (ci#37541745674, job 112536140445), and on the red commits those steps read `skipped` because the gate step ended the job; `.github/workflows/ci.yml` carries the two steps between the gate and `Install cargo-auditable`. What holds is the GATE STEP's property: it needs no `npm ci` and runs before the install (the workflow's own comment says "No npm ci needed"). Pre-existing, not introduced by this chunk (the workflows are untouched here); added to this bullet at the wrap's Validate, after the test-plan detector reported it as an out-of-detector observation. Sibling sites read and still true, because each speaks of the gate, not the job: security-plan `:222` ("deliberately NO `npm ci` … so node_modules is never needed") and `:259` ("`cargo xtask check:npm-supply-chain` in the `supply-chain` job (SHA-pinned setup-node, no `npm ci`)"); architecture `:249` ("Reads `package-lock.json` alone … (no `npm ci`, no node_modules …)"). Search: `grep -rn 'never installed\|node_modules is never'` over the seven masters and `.andromeda/registries/` → test-plan 1 (`:494`, false), security-plan 1 (`:222`, true of the gate); `.claude/`, `CLAUDE.md`, the playbook and the drift-base 0.
  - One chunk-artifact claim was already corrected inside the chunk's own window and needs no amendment: research.md first read "npm 11.19 does not run install scripts unasked"; P5 measured the plain `npm ci` running puppeteer's postinstall and exiting 1 on this host's partial shared browser cache, and corrected the bullet in place before the plan was final (research.md §Measured, the bracketed correction). Recorded here, no amendment owed.
- **Expected amendments (from plan):**
  - security-plan §Dependency Security → npm channel, the dated current-state record — **carried**: the fact is the **Counts / qualifiers moved** bullet above (the re-read date 2026-10-06; three advisories that entered the feed after the last green run closed by in-range lockfile bumps with no exception — `seroval` and `seroval-plugins` 1.5.4 → 1.6.8, `source-map-js` 1.2.1 → 1.2.2, by GHSA id; the exception set unchanged at 2 advisory + 2 license; the gate at `green-with-dispositions`). The previous "last re-read" (the `2026-09-29-scrubber-path-false-positive` sentence) must not stay standing as the latest beside the new one. Search that located the site: `grep -n 'Re-read at chunk'` over the seven masters → security-plan 1 hit (`:222`), the other six 0; `grep -c 'green-with-dispositions\|extract-zip'` → security-plan 1 · architecture 1 (contract list, no change) · a11y-plan 1 (still true, no change); `.andromeda/registries/` 0 files. Owner: security-plan.
  - The plan's two notes "no other master names either package" and "the 8 dev-class `@opentelemetry/*` lockfile entries" are stated as NOT expected amendments; their facts: the package-name grep reads 0 across masters and registries (above), and this chunk adds no lockfile entry, so that set is the base's, unchanged.
- **Coverage of new surfaces:** none — the chunk adds no external surface, no hot-path operation and no UI element. The changed artifact is a lockfile; the built webview bundle is byte-identical to the chunk base (three `sha256sum` readings equal to the base's, `evidence/installed-tree.md`).

## Deviations from intent
none — the registry resolved the versions research measured, the lockfile delta is byte-identical to research's recorded diff (`cmp` over the changed lines: exit 0), and no plan STOP condition fired.

scope record: none — gate.py scope clean, 0 recorded (`scope: clean — changed 1 · listed 1 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 46`, base `b1fcba5`).

## Decisions & corrections
- **Phase directive (inputs#I1; the overseer, founder-delegated, 2026-10-06):** prefer the fix at its source; an exception only where no forward fix is measured; measure the installed tree after the change. Followed as given; no exception was needed.
- **Operator-pass go (the overseer, founder-delegated, 2026-10-07, in the session):** "Go: the operator pass as planned: hygiene (26), the pre-CI commit and clean-tree push (27), the ci.py CI read (28). If a CI job goes red on a runner or network fault with no link to this diff, re-run the failed job yourself once and record both readings. Report the verdict and stop before the wrap." The implementing agent drove the three entries by hand on that word; the re-run allowance was not used.
- **Wrap directive (the overseer, founder-delegated, 2026-10-07, in the /andromeda-wrap-session invocation):** run the wrap whole; route resolve mints nothing and the L4 first-hypothesis entry becomes the head again; the main-branch exposure and the 72 h SLA are the founder's to rule and are recorded here as open and owned by him, "not as a residual of this chunk"; technical forks go to the overseer.
- No correction of the agent's work was given in this chunk's implement or operator pass.
- Sweep hazard found: `grep 'green-with-dispositions\|extract-zip\|npm-supply-chain'` over the leaves hits `CLAUDE.md`, `.claude/rules/security.md`, `.claude/rules/verification-harness.md` and `.claude/docs/commands.md` on the VERB name alone; none of them states the gate's current state (0 lines each without the verb name). A leaf sweep for the state must leave the verb name out of the pattern.

## Outcome
Acceptance criteria, each re-asserted against the diff (one lockfile, three entries):
- (security) the gate exits 0, arm `green-with-dispositions`, `unexcepted []`, none of the three GHSAs in its output, `violations []`, `hits []` — **met** (`cargo xtask check:npm-supply-chain`; exit 0, not 2).
- (security) each GHSA closed at its source, none excepted — **met** (the lockfile probe `0 2`; `npm-policy.json` identical to the base).
- (security · inputs#I1) the installed tree carries the fix — **met** on this host (the installed-tree probe `0 2` after a fresh install of the changed lockfile; the only copies on disk are `seroval` 1.6.8, `seroval-plugins` 1.6.8, `source-map-js` 1.2.2; `evidence/installed-tree.md`).
- (security · a11y) the two `extract-zip` exceptions stand unchanged — **met** (the gate names both as excepted; the policy file untouched).
- (arch · tests · design) the delta is the three entries alone — **met** (18 changed lines; 0 entries added or removed).
- (arch) the gate's contract and every registered resource untouched; the dual license still carried — **met** (the scope guard; `license_check::license_npm_manifest_and_lock_root_agree` PASS in the workspace nextest).
- (tests) the standard gate set green, whole and in order — **met** (below).
- (tests · design) install and build succeed on the changed lockfile; the accent token still emitted — **met** on the dev host in the `PUPPETEER_SKIP_DOWNLOAD=1` form; the plain `npm ci` is witnessed on the runners (step 15 of the `supply-chain` job, `success`).
- (a11y) the config-named Playwright listing reads `Total: 41 tests in 18 files` — **met**. No a11y tool major moved (pa11y 10.0.0, pa11y-ci 4.1.1, lighthouse 13.4.1, axe-core 4.11.4, @playwright/test 1.59.1 are not in the delta), and the `extract-zip` exceptions did not change.
- (obs) no telemetry emit site added or removed, no telemetry SDK brought in — **met** (no source file differs; no lockfile entry added).
- (security · tests · a11y · obs) the CI run on this chunk's own pushed commit reads `verdict: green` — **met**: ci#37541745674 on `fc4ebdf`, checks 13/13.

Gates (the plan's entries by `run`, in order; /implement run `2026-10-06T22-32-37Z-implement`: `entries 28 · green 24 · red 0 · recorded 1 · timeout 0 · not-run 3`):
- `PUPPETEER_SKIP_DOWNLOAD=1 npm ci --prefix pulse-app/ui` — green (exit 0)
- the lockfile probe (`python -X utf8 -c "import json; P=json.load(open('pulse-app/ui/package-lock.json'…`) — green (exit 0, last line `0 2`)
- the installed-tree probe (`python -X utf8 -c "import json,glob; …`) — green (exit 0, last line `0 2`)
- `git diff -U0 b1fcba5… -- pulse-app/ui/package-lock.json | grep -cE '^[-+] '` — green (exit 0, last line `18`)
- `git diff -U0 b1fcba5… -- pulse-app/ui/package-lock.json | grep -cE '^[-+] +"node_modules/'` — green (exit 1, last line `0`, as expected)
- `git diff --name-only b1fcba5… -- pulse-app xtask crates scripts .github Cargo.toml Cargo.lock deny.toml rust-toolchain.toml ':!pulse-app/ui/package-lock.json'` — green (exit 0, no output)
- `cargo xtask check:npm-supply-chain` — green (exit 0, all ten atoms held)
- `npm run build --prefix pulse-app/ui` — green
- `grep -c -e '--color-accent:#c7556a' pulse-app/ui/dist/tokens.css` — green (last line `1`)
- `sha256sum pulse-app/ui/dist/assets/*.js pulse-app/ui/dist/tokens.css pulse-app/ui/dist/index.html` — recorded (exit 0): the three hashes equal the chunk-base hashes in the plan, as forecast
- `npm run lint --prefix pulse-app/ui` — green · `npm run typecheck --prefix pulse-app/ui` — green
- `npm run test --prefix pulse-app/ui` — green (`863 passed (863)`, 82 files)
- `(cd pulse-app/ui && npx playwright test --config=playwright-a11y.config.ts --list)` — green (`Total: 41 tests in 18 files`)
- `cargo fmt --check` — green · `cargo clippy --workspace --all-targets --all-features -- -D warnings` — green
- `cargo xtask check:english-sources` · `capability-widening-check` · `check:ingest-progress` · `check:staged-artifacts` · `capability-drift` · `verify:capability-matrix` — green, each
- `cargo nextest run --workspace --profile ci` — green (2697 run, 2697 passed, 0 skipped)
- `cargo nextest run -p pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'` — green
- `git diff --quiet b1fcba5… -- pulse-app/ui/src/bindings/index.ts` — green
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — `leg = 'operator'`, driven by hand: exit 0, `hygiene: clean` (`evidence/operator-pass.md`)
- `git diff --quiet && git diff --cached --quiet && git push origin chore/migrate-pulse-to-v3` — `leg = 'operator'`, driven by hand on the overseer's go: exit 0, `b1fcba5..fc4ebdf` (`evidence/operator-pass.md`)
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2400` — `leg = 'operator'`, driven by hand: exit 0, `verdict: green`, ci#37541745674 (`evidence/operator-pass.md`)
- No entry was deferred; no `defer` key is in the block. Smoke: skipped — no boot-path file touched, the plan lists no smoke or self-verify entry, the bundle is byte-identical to the base.

Watches: none folded.

**Open, not this chunk's residual — owned by the founder (the wrap directive):** the fix is on `chore/migrate-pulse-to-v3`; `main` keeps the lockfile with `seroval` 1.5.4 until PR #39 (a draft, the founder's) merges. security-plan `Critical CVE response SLA` (`.andromeda/security-plan.md:226`) reads "72h from public advisory disclosure to Dependabot PR merge + tagged patch release"; the critical advisory GHSA-p6vx-979v-rg4c was published 2026-10-05T23:40:43Z, so the window runs to 2026-10-08T23:40:43Z. A merge or a release is the founder's to rule; the overseer holds it on the FOR DISCUSSION list with that deadline. Unmeasured beside it: whether the shipped SPA bundle includes any `seroval`-importing module (research.md §Measured, indirect only; the lockfile class is runtime).

Not measured on this host, witnessed by CI on `fc4ebdf`: the plain `npm ci`, `cargo auditable build --workspace --release`, and the full a11y chain (the three `a11y` legs). Not read: the step-14 log on the runner (its arm there is read from the step's `success`).

Outcome basis: the operator pass ran — the gate verdicts rest on its final state: the commit list `b1fcba5..HEAD` = `fc4ebdf` alone, and the final HEAD's CI run ci#37541745674 recorded in `evidence/operator-pass.md`. Implement's P4 report (in this session's conversation) is the basis for what only it holds. Two overseer directives sit between implement and this report: the operator-pass go (it changed who fired entries 26–28: the agent, by hand) and the wrap directive (route-resolve mints nothing; the main-branch exposure is the founder's, recorded as open).

Process hygiene (implement's census, re-measured here 2026-10-06T23:07Z against `ps -eo pid,comm`):

| process | started by | final state |
|---|---|---|
| npm / node (install, update, build, lint, typecheck, vitest, playwright `--list`, `npm ls`) | implement's run | terminated — none in the process list |
| cargo / nextest / xtask | implement's run | terminated — none in the process list |
| `ci.py conclusion` (the 1705 s poll) | the operator pass | terminated — returned at 23:05:47Z |
| `scripts/code-graph.py refresh` | this wrap, Setup | terminated — exited 0; 0 matching rows at the 23:08Z re-read |

No app was booted; nothing listens on 4317/4318.
