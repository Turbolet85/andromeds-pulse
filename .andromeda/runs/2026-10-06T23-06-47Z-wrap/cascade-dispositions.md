# Cascade dispositions — 2026-10-06-npm-supply-chain-gate-is-green-again

Written from the `cascade.py sweep` listing of this run (trail `cascade-2026-10-06-npm-supply-chain-gate-is-green-again.json`), after both bodies were applied and before any sidecar entry.

## The pass's amendments
1. security-plan §Dependency Security → npm channel: the dated current-state run (line 222).
2. test-plan §9 CI Integration → the Supply chain row (line 494).

## The search
Patterns (`cascade-patterns.toml`, 10; each control fired on the pre-pass text at the baseline `b1fcba57`):
- amendment 1, retired wording: `scrubber-path-false-positive`: still` · `same four exceptions` · `brace-expansion` · `ip-address` · `advisories reported after the prior green run` (case-insensitive)
- amendment 1, the claim's state words: `green-with-dispositions` · `extract-zip`
- amendment 2, retired wording and mechanism: `never installed in this job` · `node_modules is never` · `` no `npm ci` `` (case-insensitive)

Swept by the tool: the seven masters, `.andromeda/registries/**`, the three curation homes, the two judgment bases, the leaf bodies. NOT looked for: the gate's exit contract wording (`0 green · 1 findings/policy red · 2 cannot-evaluate`), which neither amendment touches; the package names `seroval` / `source-map-js`, measured at P1 at 0 hits in all seven masters and the registries before the pass (after it: security-plan `:222` only, this pass's own text).

Line profiles (`splice.py summary`): security-plan longest line 6 905 chars, 9 lines over 2 000; test-plan longest 8 006, 9 over 2 000. Line 222 of security-plan (4 054 chars after the edit) and architecture `:249` (15 413 chars) were resolved by offset, not from a clipped view.

## Every row
| row | disposition |
|---|---|
| `reread-0929` · 0 rows | the retired sentence is gone from every swept file |
| `five-adv` · 0 rows | gone |
| `never-inst` · 0 rows | gone |
| security-plan.md:222 `four-exc` standing, edited @c2446 | amended — this pass's own sentence ("with the same four exceptions and `unexcepted []`"); the count is still true (2 advisory + 2 license) |
| security-plan.md:222 `brace-exp` new @c3094 · `ip-address` new @c3114 | amended — this pass's pointer to the earlier close ("The same close was taken at chunk `2026-09-29-scrubber-path-false-positive` (brace-expansion and ip-address)"); the retired version numbers are not restated |
| security-plan.md:222 `nm-never` standing, edited @c719 | no change — "so node_modules is never needed" speaks of the GATE (the scanning step), which is true; window read: "…`npm audit` reads lockfile+registry, so node_modules is never needed. Exit contract…" |
| security-plan.md:222 `no-npm-ci` standing, edited @c521 | no change — "running in `ci.yml`'s `supply-chain` job as a plain `run:` step … deliberately NO `npm ci`: the license walk is lockfile-only" — the gate step's property, true |
| security-plan.md:259 `no-npm-ci` standing | no change — "`cargo xtask check:npm-supply-chain` in the `supply-chain` job (SHA-pinned setup-node, no `npm ci`)": the gate's wiring prerequisites, true (the gate step runs before the job's `npm ci`) |
| architecture.md:249 `no-npm-ci` standing @c845 | no change — "Reads `package-lock.json` alone for license + dependency-class data (no `npm ci`, no node_modules; `npm audit` reads lockfile+registry)": the verb's contract, true |
| test-plan.md:494 `no-npm-ci` standing, edited | amended — this pass's own sentence ("the gate step itself needs NO `npm ci` … and runs BEFORE the job's own `npm ci` + `npm run build` steps") |
| .claude/rules/security.md:183 `no-npm-ci` curation | no change — a Session Additions entry: "license and dependency-class checks read the COMMITTED lockfile alone … no `npm ci` needed": about the gate's read, true; preserve-verbatim home, nothing to route to curation |
| .claude/rules/security.md:78 `no-npm-ci` leaf @c806 | no change — "`cargo xtask check:npm-supply-chain` (ci.yml `supply-chain` job; policy …; lockfile-only, no `npm ci`; exit 0 green · …)": the gate's contract, true |
| .claude/rules/verification-harness.md:87 `no-npm-ci` leaf @c161 | no change — "`xtask check:npm-supply-chain` — … lockfile-only license+class source, no `npm ci`": the verb, true |
| architecture.md:249 `state-arm` standing @c379 | no change — the six-arm contract list, not a current-state claim |
| security-plan.md:222 `state-arm` standing, edited ×2 @c1484, @c2412 | @c1484 no change — "Current state (chunk `2026-09-29-p-025-…`): exit 0 `green-with-dispositions` — 2 advisory + 2 license exceptions": still true; @c2412 amended — this pass's re-read |
| security-plan.md:222 `extract-zip` standing, edited @c1575 | no change — "both advisory exceptions are extract-zip, and neither has a fixed release anywhere": the gate still reports both as excepted |
| a11y-plan.md:39 `extract-zip` standing | no change — "pa11y-ci 4.1.1 … covered by the npm-policy extract-zip exception": still true, pa11y-ci is not in the lockfile delta |

## Lateral binds
- test-plan §3 ↔ obs-plan §3 (harness commands): neither amendment touches §3. No change.
- a11y-plan schema ↔ obs-plan schema: untouched. No change.

## Leaves (step 3 — recomputed from the amended sections, not scanned for wording)
Leaves naming the amended masters by provenance (`grep -l`): security-plan → `.claude/docs/security-summary.md`, `.claude/rules/security.md`, `.claude/rules/observability.md`, `.claude/docs/gotchas.md`; test-plan → `.claude/docs/tests-summary.md`, `.claude/rules/testing.md`, `.claude/rules/verification-harness.md`; plus CLAUDE.md `GENERATED:setup:warnings`.
- `security-summary.md`: carries no npm-channel state (`grep -n 'npm'`: 0 lines; its sections are threat model, classifications, vectors, bootstrap phases, anti-patterns, residual risks, key custody). Nothing derives from the amended run. No change.
- `rules/security.md` body (§Supply chain + CI, line 78): states the gate's contract and wiring, never its dated current state or exception count. No change.
- `tests-summary.md` §Quality gates (CI) and §CI integration (lines 71–87, read whole): no supply-chain job row, no install-order claim. No change.
- `rules/testing.md`, `rules/verification-harness.md`, `rules/observability.md`, `docs/gotchas.md`, `docs/commands.md:99`, CLAUDE.md lines 15 / 63 / 91: name the verb or the job; none states the job's install order or the gate's dated state. No change.

Result: 2 body amendments · 0 further master sites · 0 leaves re-derived · 0 curation-home or judgment-base hits needing a route.
