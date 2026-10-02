# Codebase Research — 2026-08-30-npm-advisory-coverage

## Scope
- **Depth:** moderate · **Reads:** 6 file reads (ci.yml full, xtask/main.rs ×3 sections, xtask/Cargo.toml, run_npm_script impl) · **Globs/Greps:** 9 (+ 3 measurement probes: `npm audit --json`, license enumeration over node_modules, lockfile multi-version scan)

## Files inspected
- `.github/workflows/ci.yml` (full, 335 lines) — THREE jobs: `lint-test-build` (3-OS matrix), `supply-chain` (ubuntu-only), `coverage` (ubuntu-only). The `supply-chain` job (:221) EXISTS and holds cargo audit (rustsec/audit-check action, SHA-pinned), cargo-deny-action (SHA-pinned), the Cranelift plain-`run:` assertion (:249-250 — the shipped precedent for a supply-chain `run:` step), and cargo-auditable. `harden-runner` (SHA-pinned, egress audit) is the FIRST step of all three jobs; workflow-level `permissions: contents: read` (:8). `npm ci` runs in `lint-test-build` only (:90-92, working-directory pulse-app/ui, node 22, NO npm cache configured); the a11y gate IS wired (`cargo xtask test:a11y` :124 — answers the a11y extract's question). Every third-party action already 40-char SHA-pinned.
- `xtask/src/main.rs` (:80-130, :230-262, :780-800, :1093-1118) — clap-derive `Cmd` enum with kebab `#[command(name = …)]`; **`Cmd::Audit` and `Cmd::DenyBans` already exist as the Rust-side supply-chain verbs** (run_cargo wrappers, :237-238); `run_npm_script(script, extra)` (:1093) spawns `npm.cmd`/`npm run <script>` with cwd `pulse-app/ui` — the npm-spawn precedent (an `npm audit` sibling helper spawns `npm audit --json` directly, not `npm run`). `XTASK_GATES` (:786-795) is the capability-matrix mode-validation list — joined only if a matrix cap cites the new verb (none does).
- `xtask/Cargo.toml` — deps available to the gate: **serde_json** (parse audit JSON + policy file + lockfile), **tokio** (process — spawn npm), anyhow, clap, chrono, reqwest, tempfile. NO `toml` crate → the policy file is JSON, with provenance/reason as FIELDS (better for machine-parse than comments anyway).
- `xtask/ci/` — 6 paired `.sh`/`.ps1` post-step check scripts (heartbeat-gap-check et al.) — the parse→assert→exit-1 shape.
- `pulse-app/ui/package.json` + `package-lock.json` (P1 + P3) — 29 devDeps + 9 runtime deps; lockfile 910 package entries; **lockfile `packages` entries carry `"dev": true` flags** → dependency CLASS (prod vs dev) is derivable from the lockfile alone, and the lockfile package list is the canonical enumeration for the license walk (a blind node_modules fs-walk hits nested test-fixture stubs — measured: `test-fixtures`/`baz`/`browser_field` name-less entries).
- `.npmrc` — ABSENT (repo root + pulse-app/ui): no registry override, no audit config.

## Graph impact (rust plane; trace at `.andromeda/runs/2026-08-30T00-20-00Z-phase/tree-query-2026-08-30-npm-advisory-coverage.json`)
- **npm-symbol collision check** — only `xtask::run_npm_script()` exists; `npm_gate`/`NpmGate` names are FREE.
- **`run_npm_script` callers** — exactly 3, all `main()` dispatch arms (main.rs:239-241: Lint/Typecheck/TestA11y). Caller threading for the new verb: Cmd variant + one dispatch arm + new module — nothing else threads.

## Measurements (the first-scan reality the plan dispositions)
- **`npm audit --json` (2026-08-30, exit 1)** — **39 vulnerabilities: 1 critical / 20 high / 16 moderate / 2 low** (raw JSON archived at `.andromeda/runs/2026-08-30T00-20-00Z-phase/npm-audit-first-scan.json`). Direct-dep findings: **vitest** (CRITICAL, UI-server arbitrary file read, fix < ^3 boundary — non-major), **vite** (high, fix within ^7), **lighthouse** (high, vulnerable range tops at 13.4.0 — ALL of ^12 vulnerable; `fixAvailable: true` is ambiguous against the ^12 pin — re-measure with `npm audit fix --dry-run` at implement), **pa11y** (high, only fix = 10.0.0 `isSemVerMajor: true` — crosses the a11y-plan §3 pinned major), **pa11y-ci** (high, `fixAvailable: true`), **webdriverio** (high — the whole wdio cluster's suggested "fix" is 8.14.6, a major DOWNGRADE `isSemVerMajor: true` → no forward fix at HEAD → exception-class). Transitive fix-true findings: brace-expansion (10 ids), js-yaml, lodash, nanoid, postcss, ws, esbuild, ip-address, @babel/core, an @opentelemetry/* + @sentry/node cluster (rides lighthouse's tail), @puppeteer/browsers, puppeteer/-core, extract-zip, deepmerge-ts.
- **Licenses (823 installed packages)** — MIT 612 · Apache-2.0 96 · ISC 45 · BSD-2/3 40 · BlueOak-1.0.0 6 · MPL-2.0 4 · CC-BY-4.0 2 (`@promptbook/utils`, `caniuse-lite` — data files) · **LGPL-3.0-only 2 = `pa11y` 9.1.1 + `pa11y-ci` 4.1.0 THEMSELVES** (dev-only tools, not distributed — a per-CLASS license policy handles this: runtime class strict, dev class broader) · Python-2.0 1 (`argparse`) · misc dual/single permissive · **2 genuinely license-less: `css-value` 0.0.1, `union` 0.5.0** (old wdio-chain transitives — exception-class with closing condition).
- **Multi-version packages: 43 of 830** (brace-expansion ×4, minimatch ×4, lru-cache ×3, …) — npm nests by design; **cargo-deny's `multiple-versions = "deny"` does NOT port**. The npm "bans" half = package-NAME denylist (first real entries: the alternate ARIA component libraries per a11y-plan §11) — NOT duplicate-version deny, which would be permanently red.
- **web-vitals: ABSENT** from package.json, lockfile, and ui/src — obs-plan §1/§3 name `web-vitals` 5.x as the frontend bridge mechanism; at HEAD the bridge is the hand-rolled TauRPC `telemetry.frontend.*` surface without it. TARGET-state mandate not yet implemented; NOT this chunk's to fix and no disposition here can break it (nothing to break). Noted for the wrap report's spec-observation channel.
- **Second ARIA library: NOT present** — `node_modules/react-aria` is `react-aria-components`' own dependency (same family); radix/headlessui/ariakit/reach all 0 hits. The a11y §11 ban is satisfied at HEAD; the bans denylist keeps it that way.
- **CI note (context, not this chunk's scope):** the `supply-chain` job's rustsec/audit-check step is subject to the same upstream RustSec-DB decay the pin-#22 standing deferral records (`cargo audit` exit 1 locally, byte-identical basis) — the branch is 44 commits unpushed, so current CI state is unmeasured here.

## Patterns detected
- **xtask supply-chain verbs** (main.rs:237-238): `Cmd::Audit` → `run_cargo("audit")`, `Cmd::DenyBans` → `run_cargo("deny", check bans licenses sources)` — the new npm verb is their npm-side sibling.
- **npm spawn** (main.rs:1093): `npm.cmd` on Windows, cwd `pulse-app/ui`, `tokio::process::Command`, `status_to_code` exit mapping.
- **Plain `run:` supply-chain CI step** (ci.yml:249-250): the Cranelift assertion — grep + `exit 1`, no third-party action, no SHA-pin obligation.
- **Co-located `#[cfg(test)] mod tests`** in all 7 xtask modules (bundle_format:95 … webview_drive:855) — xtask is a bin crate (no `[lib] test = false` trap); new-module pins follow suit, with the wrap-time "each test collected by name" check per the 2026-08-29 precedent.
- **Pinned-expectation gate** (EXPECTED_PROCEDURES + capability-drift): the reviewable-expectation-list shape the policy file mirrors.

## Conventions to follow
- **Verb naming**: kebab + namespace-colon (`check:ingest-progress`, `smoke:gap-resume`, `test:a11y`, `verify:capability-matrix`) — a gate verb belongs in the `check:` namespace.
- **Exception entries as structured fields** (id, package, reason, owner, closing_condition, added-date) per security-plan carve-out discipline — JSON fields, since serde_json is the available parser and JSON carries no comments.
- **Two-invocation lesson applied**: the npm gate is its OWN CI step whose red is legible — never folded into an existing invocation.

## New files to create
- `xtask/src/npm_gate.rs` — the gate module: spawn `npm audit --json`, parse; read `package-lock.json` for the package enumeration + dev-class flags; read each lockfile-listed package's `package.json` license field; evaluate all three checks against the policy file; machine-parseable verdict + exit code; co-located `mod tests` over fixture JSON.
- `pulse-app/ui/npm-policy.json` (name final at plan) — project-owned policy: license allowlists per class (runtime strict / dev broader), package-name denylist, ID-scoped advisory exceptions with provenance fields.
- CI: one step block in the existing `supply-chain` job (setup-node + `npm ci` + `cargo xtask <verb>`) — a modify, listed below.

## Files to modify
- `xtask/src/main.rs` — Cmd variant + dispatch arm (the graph-enumerated threading; nothing else).
- `.github/workflows/ci.yml` — `supply-chain` job gains setup-node (SHA-pinned action already used in lint-test-build) + `npm ci` + the gate step.
- `pulse-app/ui/package.json` + `package-lock.json` — the disposition upgrades (vitest critical + the fix-true set; majors per the P4 decision).
- (conditional on the majors decision) `pulse-app/ui/tests-a11y/baselines/*` — a11y re-baseline if pa11y/lighthouse cross majors; plus the full a11y chain re-run evidence.

## Open questions
- **How to disposition advisory fixes that CROSS a11y-plan §3 pinned majors** (pa11y → 10.0.0 measured; lighthouse → 13.x candidate pending dry-run) → blocks: **plan-decision** (P4 resolves via operator question — the a11y extract's "joint decision" clause; options: upgrade + full-chain re-baseline in-chunk / ID-scoped exception with the pin as reason + re-pin closing condition / leave red).
- **Exact remediation set `npm audit fix --dry-run` will apply** (which transitives move, whether lighthouse's `fixAvailable: true` is real within ^12) → blocks: **implementation-scope** (the modify-list's package.json/lockfile delta is provisional until measured at implement; the plan prescribes disposition RULES, not exact versions).
