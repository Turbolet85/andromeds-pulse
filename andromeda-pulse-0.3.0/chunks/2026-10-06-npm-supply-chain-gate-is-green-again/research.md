# Codebase Research — 2026-10-06-npm-supply-chain-gate-is-green-again

## Scope
- **Depth:** moderate · **Reads:** 9 · **Globs/Greps:** 11 (plus 14 measurements run on a scratch worktree of HEAD `b1fcba5`, removed afterwards; the working tree was never edited)
- **Harness rules consulted:** none — no live leg in this chunk (no app boots; the UI gates and the gate verb are plain commands)
- **Platform issues consulted:** `gh api /advisories/GHSA-p6vx-979v-rg4c` → critical, `seroval`, vulnerable `>= 0.12.0, <= 1.6.0`, first patched 1.6.2, published 2026-10-05T23:40:43Z · `gh api /advisories/GHSA-jp82-f5mq-hwhp` → high, `seroval`, vulnerable `<= 1.6.2`, first patched 1.6.3, published 2026-10-05T23:40:40Z · `gh api /advisories/GHSA-68fv-2mgg-jv7q` → high, `source-map-js`, vulnerable `>= 1.0.0, < 1.2.2`, first patched 1.2.2, published 2026-09-18T18:31:44Z, updated 2026-10-05T23:31:22Z. None is withdrawn. The runner failure is the gate's own findings, not a platform fault, so no runner issue tracker was searched.
- **External inputs:** `inputs#I1` — the phase directive (prefer the fix at its source; an exception only where no forward fix is measured, with its re-check trigger; measure the installed tree after the change; every step may run at night)

## Files inspected
- `xtask/src/npm_gate.rs` (`:10-21`, `:29-30`, `:314-350`, `:405-413`, outline of the rest) — the six arms and their exits: `clean` (0), `green-with-dispositions` (0), `findings-red` (1), `policy-red` (1), `registry-unreachable` (2), `missing-input` (2). `ui_dir()` resolves from `env!("CARGO_MANIFEST_DIR")`, so a built xtask always reads the checkout it was compiled in. The advisory arm spawns `npm audit --json` in `pulse-app/ui`; the JSON verdict goes to stdout and the one-line summary to stderr. Whether the policy parser demands `added` was not read: this chunk adds no exception.
- `pulse-app/ui/npm-policy.json` (head, the two advisory exceptions and the license block) — both advisory exceptions are `extract-zip`, each with `reason` / `owner` / `closing_condition` / `added`. Untouched by this chunk.
- `pulse-app/ui/package.json` — `overrides` holds one entry (`basic-ftp ^6.2.1`); `license` is `MIT OR Apache-2.0`; the scripts are `build` (`node scripts/build.mjs`), `test` (`vitest run`), `typecheck`, `lint`. Untouched by this chunk.
- `pulse-app/ui/package-lock.json` (parsed, not read whole) — lockfileVersion 3, 884 entries, 0 `inBundle` entries, root license `MIT OR Apache-2.0`.
- `.github/workflows/ci.yml` (`:467-543` step names; the `npm ci` and `node-version` lines) — the `supply-chain` job order is `cargo audit` → `cargo deny check bans licenses sources` → the Cranelift assertion → Setup Node → Linux libraries → `cargo xtask check:npm-supply-chain` → `npm ci` → `npm run build` → Install cargo-auditable → `cargo auditable build --workspace --release`. `node-version: '24'` at 7 steps and `run: npm ci` at 7 steps, one per job (`grep -n` on each pattern).
- `xtask/src/license_check.rs` (`:99-105`) — asserts `package.json` `license` and the lockfile root entry's license equal the dual expression; it reads both files this chunk's toolchain touches.
- CI job 112517747768 (run 37536230886, sha `1124148`), step list and log — steps 9–11 `success`, step 14 `failure`, steps 15–18 `skipped`; the log prints `node: v24.21.0` / `npm: 11.19.0`.
- `andromeda-pulse-0.3.0/chunks/2026-10-04-supply-chain-advisories-on-wasmtime-resolved/plan.md` (`:1-150`) — the closest precedent's plan shape and its gate block.
- `.andromeda/playbook.md` (`:110-116`) — the External decay rule (routine) and the Boundary widening rule (escalate).

## Graph impact (from the code-graph query; "cold-start — empty DB" if early)
- **derived-without-graph** — the chunk changes no symbol on either plane: its write set is three version entries in a lockfile. No query was composed.

## Measured
All on 2026-10-06, 22:15Z–22:21Z. "Scratch tree" is a detached `git worktree` of HEAD `b1fcba5` under the session scratchpad.

- **The red reproduces on this host.** `cargo xtask check:npm-supply-chain` at HEAD: exit 1, `arm findings-red`, `advisory.distinct 5`, 2 excepted (`extract-zip` GHSA-7pqw-9j4j-h8q3, GHSA-jmr9-qjv8-65gv), 3 unexcepted (`source-map-js` GHSA-68fv-2mgg-jv7q high; `seroval` GHSA-jp82-f5mq-hwhp high; `seroval` GHSA-p6vx-979v-rg4c critical); `license.checked 883`, `excepted 2`, `runtime_packages 37`, `violations []`; `bans.hits []`.
- **Holder census at HEAD** (a walk of every lockfile entry's `dependencies` / `peerDependencies` / `optionalDependencies` / `devDependencies`): `node_modules/seroval` 1.5.4 (prod, MIT), `node_modules/seroval-plugins` 1.5.4 (prod, MIT), `node_modules/source-map-js` 1.2.1 (dev, BSD-3-Clause) — one path each. Dependents: `@tanstack/router-core` 1.169.2 → `seroval ^1.5.4`, `seroval-plugins ^1.5.4`; `seroval-plugins` → peer `seroval ^1.0`; `@tailwindcss/node` 4.2.4 → `source-map-js ^1.2.1`; `postcss` 8.5.26 → `source-map-js ^1.2.1`.
- **Registry** (`npm view`): `seroval` latest 1.6.8 (1.6.2 … 1.6.8 published), `seroval-plugins` latest 1.6.8, `source-map-js` latest 1.2.2. Licenses unchanged: MIT, MIT, BSD-3-Clause. `@tanstack/router-core` 1.171.34 (its latest) itself requires `seroval ^1.6.7` and `seroval-plugins ^1.6.7`.
- **Every patched release is inside every dependent's range**, so the close needs no `overrides` entry and no `package.json` edit.
- **The update form matters.** Two forms were measured:
  - `npm update seroval seroval-plugins source-map-js --package-lock-only`: moves the three entries AND adds six `inBundle` entries under `node_modules/@tailwindcss/oxide-wasm32-wasi/node_modules/` (82 changed lines). Control: a plain `npm install --package-lock-only` on HEAD's two manifest files, with no update at all, adds the same six. They come from this host's npm re-deriving the lock without an installed tree, not from the update.
  - `npm ci`, then `npm update seroval seroval-plugins source-map-js` against that installed tree: 9 lines in, 9 out, two hunks — the `version` / `resolved` / `integrity` lines of the three entries and nothing else (`.andromeda/runs/2026-10-06T22-06-57Z-phase/research-lock-delta.diff`). `package.json` unchanged. This is the form the plan uses.
- **Installed tree after the change** (scratch tree): the only copies on disk are `node_modules/seroval` 1.6.8, `node_modules/seroval-plugins` 1.6.8, `node_modules/source-map-js` 1.2.2 (`find` over `node_modules` for each package's `package.json`); `npm ls seroval seroval-plugins source-map-js` exits 0 and shows the same versions on the same chains. The lockfile reads the same three versions, 884 entries, 0 `inBundle`, root license unchanged.
- **`npm audit --json` after the change:** only GHSA-7pqw-9j4j-h8q3 and GHSA-jmr9-qjv8-65gv remain (both `extract-zip`); `critical 0`.
- **The gate after the change** (xtask built inside the scratch tree, because `ui_dir()` is compile-time): exit 0, `arm green-with-dispositions`, `advisory.distinct 2`, both excepted, `unexcepted []`; `license.checked 883`, `violations []`; `bans.hits []`. So neither `extract-zip` closing condition fired, and no class flag or license field moved.
- **UI gates on the changed lockfile** (scratch tree): `npm run typecheck` exit 0 · `npm run lint` exit 0 · `npm run build` exit 0 · `npm run test` 82 files / 863 tests passed, exit 0.
  - On a tree with no `dist/`, `npm run test` run BEFORE `npm run build` fails one test: `src/contrast/parse-tokens.test.ts` reads `dist/tokens.css` (`ENOENT`). It is an ordering property of the suite, present with or without this chunk's change; the main tree has a `dist/` so it does not show there. The plan orders build before test.
- **A11y suite listing** (`npx playwright test --config=playwright-a11y.config.ts --list`): `Total: 41 tests in 18 files`, exit 0, on the main tree at HEAD and on the scratch tree after the change. The full chain (`cargo xtask test:a11y`) was not run in research.
- **Pinned majors unchanged on the changed lockfile:** vitest 4.1.11, @vitest/mocker 4.1.11, jsdom 26.1.0, @testing-library/react 16.3.2, vite 7.3.6, postcss 8.5.26, tailwindcss 4.2.4, @tailwindcss/cli 4.2.4, @tanstack/react-router 1.169.2, @tanstack/router-core 1.169.2, react 19.2.5, react-aria-components 1.17.0, webdriverio 9.32.0, @crabnebula/tauri-driver 2.0.9 (its `tauri-driver-win32-x64-msvc` optional entry still present), pa11y 10.0.0, pa11y-ci 4.1.1, lighthouse 13.4.1, axe-core 4.11.4, @playwright/test 1.59.1, http-server 14.1.1. None of these entries is in the delta.
- **Dependency-absence probes on the changed lockfile:** `web-vitals` 0 in `package.json` and 0 in the lockfile; `@radix-ui` / `radix-ui` 0 and 0. `@opentelemetry/*`: 0 in `package.json`, 8 lockfile entries, all `dev`-flagged, pulled by `@sentry/node` — the SAME 8 at HEAD (51 matching lines in both lockfiles), 0 files under the built `dist/` naming `opentelemetry`. The obs extract's criterion "any browser OTel SDK package reads 0 entries in the lockfile" is therefore not true at HEAD and never was this chunk's to make true; what holds is: no runtime-class OTel package, none in the bundle, and the set unchanged by this chunk.
- **Built stylesheet:** `--color-accent:#c7556a` in the scratch tree's `dist/tokens.css`; `dist/index.html` holds 0 `http(s)://` references.
- **Does `seroval` reach the shipped bundle?** Indirect only: the one built JS file contains the string `seroval` 0 times and none of five marker tokens, and inside `@tanstack/router-core` the only modules importing `seroval` sit under `dist/esm/ssr/`. A minified bundle need not carry the name, so this does not prove absence. The lockfile class (runtime) is what the gate and the policy read, and the fix is the same either way.
- **`npm ls --all` is not usable as a gate:** it exits 1 with `invalid: proxy-agent@6.5.0` on the main tree AND on a fresh `npm ci` of the same lockfile in the scratch tree. It is a standing reading of the lockfile with its `basic-ftp` override (`get-uri` declares `basic-ftp ^5.0.2`, the override resolves 6.2.1), unrelated to this chunk. The targeted `npm ls {three packages}` exits 0.
- **Main tree's installed modules:** `node_modules/.package-lock.json` holds 800 entries, 0 absent from the lockfile and 0 at a different version. `npm ci` first is still the plan's form, since the minimal delta was measured from a fresh install.
- **Host vs CI toolchain:** host Node v26.8.2 / npm 11.19.1; CI Node v24.21.0 / npm 11.19.0 (the red job's log). Same npm major and minor.
- **Install scripts and the browser cache** [corrected at P5, 2026-10-06: this bullet first read "npm 11.19 does not run install scripts unasked … and run none of them" — inferred from the `npm warn install-scripts` wording while every scratch run set `PUPPETEER_SKIP_DOWNLOAD=1`, which masked the script]. Measured at P5 on the main tree: the plain `npm ci --prefix pulse-app/ui` runs puppeteer 24.43.1's postinstall and exits 1, because `~/.cache/puppeteer/chrome/linux-148.0.7778.97` exists without its executable (the standing PC22 defect, CARRY (a) on the route entry "pre-push:linux runs natively on Linux"); a failed `npm ci` leaves `node_modules` empty. `PUPPETEER_SKIP_DOWNLOAD=1 npm ci --prefix pulse-app/ui` exits 0 (588 top-level `node_modules` entries). Every scratch measurement above was taken with the variable set, so the measured forms are the ones carrying it.
- **Timing:** the last green `supply-chain` job (111865383180 on `5f77859`) ran 2026-10-05T16:22:17Z–16:28:52Z. The critical advisory was published 2026-10-05T23:40:43Z, so security-plan's 72 h Critical window runs to 2026-10-08T23:40:43Z.
- **CI on HEAD `b1fcba5`** (read 22:21Z): `supply-chain` failure; 11 other checks success; `coverage gate` still in progress.
- **No Dependabot PR carries this move:** the 30 open PRs (`gh pr list`) name neither package nor `@tanstack/react-router`.
- **No spec or source names the packages:** `grep -rn 'seroval\|source-map-js'` over the seven masters, `.claude/rules`, `.claude/docs`, `CLAUDE.md`, `pulse-app/ui/src`, `tests-a11y`, `scripts` and `xtask/src`: 0 hits outside the lockfile (the one hit is this chunk's own master-route record).

## Patterns detected
- **In-range lockfile bump, `package.json` untouched** (security-plan §Dependency Security → npm channel, the brace-expansion / ip-address precedent; `.claude/rules/security.md` Session Additions 2026-08-30, extended 2026-09-30): confirm the patched release exists, then a targeted `npm update {pkg}`; prove it with `npm audit` plus the gate.
- **Apply and re-audit; the residual is the truth** (same rule): the gate is re-enumerated whole after the change, not checked only for the three ids' absence.
- **Requirement-plus-resolved reporting** (arch §Stack, the plugin-runtime row): the dependents' ranges stay as they are; the resolved versions are stated from the lockfile with their date.

## Conventions to follow
- **The gate's contract is registered and is not edited** (arch §Occupied Resources → xtask CLI surfaces): green comes from the gate's inputs.
- **Exit 2 is never green** (`xtask/src/npm_gate.rs:18-20`): a `registry-unreachable` run proves nothing.
- **Standard gate set, unconditional and ordered** (test-plan §3 `Per-chunk gate discipline`): `capability-drift` before the default-features workspace nextest; the `--features mcp-server` bindings regen last; `git diff --quiet {chunk-base} -- pulse-app/ui/src/bindings/index.ts` as the close; the three webview gates because the write set is under `pulse-app/ui/`.

## New files to create
- none

## Files to modify
- `pulse-app/ui/package-lock.json` — three entries move: seroval and seroval-plugins to 1.6.8, source-map-js to 1.2.2

## Open questions
- none
