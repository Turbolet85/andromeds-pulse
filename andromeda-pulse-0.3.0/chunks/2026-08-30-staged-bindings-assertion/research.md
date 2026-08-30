# Codebase Research — 2026-08-30-staged-bindings-assertion

## Scope
- **Depth:** moderate · **Reads:** 9 files/sections · **Globs/Greps:** ~14 · **Graph queries:** 3 (rust plane; trace at `.andromeda/runs/2026-08-30T11-26-13Z-phase/tree-query-2026-08-30-staged-bindings-assertion.json`)

## Files inspected
- `xtask/src/main.rs` (100–265, 1100–1400, grep of 1476+) — the whole gate substrate. `Cmd` enum (clap derive, `name = "..."`), dispatch match at :251–252, `EXPECTED_PROCEDURES` const at :1116 (44 active procedures incl. `mcp.status/start/stop` UNCOMMENTED — the pin already demands the mcp namespace), `capability_drift()` at :1164, `capability_widening_check()` at :1272, `parse_bindings` (free fn, takes `&str` — content-source-agnostic), `capability_drift_tests` mod at :1476 (collected by the workspace run — xtask is a `[[bin]]` target with default `test = true`, unlike pulse-app's lib).
- `xtask/src/main.rs::capability_drift()` — **reads the WORKTREE**: `fs::read_to_string(workspace_root/pulse-app/ui/src/bindings/index.ts)` at :1173–1176. The root-cause hypothesis is VERIFIED: nothing in the shipped gate reads the git index. Exit surface is binary 0/1 (`ExitCode::SUCCESS/FAILURE`); writes `target/capability-drift/report.json` + one structured JSON event line on stdout.
- `xtask/src/main.rs::capability_widening_check()` — also worktree reads; parses `pulse-app/capabilities/{notification,tray,plugin-fs}.json` via `serde_json::Value`, checks `permissions` substrings + `windows` emptiness, 3 caps only, widening direction only. The JSON-shape precedent for the grants check (`json.get("permissions").and_then(as_array)` etc.).
- `pulse-app/capabilities/*.json` (all 6, full) — the grant inventory is SMALL and pinnable: `default.json` (windows: compact-widget/main/findings/report; 11 permissions: `core:default`, 9 × `core:window:allow-*` — start-dragging, minimize, toggle-maximize, close, show, set-focus, hide, set-position, set-size — plus `updater:default`), `clipboard.json` (3 windows / 1 permission), `notification.json` (2 windows / 4 permissions), `tray.json` (2 windows / 0 permissions), `plugin-fs.json` (0 / 0), `updater.json` (0 windows / `updater:default`). The `description` fields are large prose (default.json ≈ 9.5 KB) — pinning them would red the gate on documentation edits; the semantic surface is `{identifier, windows, permissions}`.
- `.andromeda/playbook.md` :74–76 — the interim ordering rule VERIFIED verbatim: capability-drift runs LAST in the P7 light gate AND "the assertion reads the **STAGED** copy (`git show :pulse-app/ui/src/bindings/index.ts | grep -c '"mcp":'` ≥ 1)"; "The mechanical half … is owned by its own markerless route entry; until it lands, this rule is the interim." → re-point at this chunk's wrap.
- `.andromeda/test-plan.md` :131 (trigger `webview-drive-mutation-arm-not-gated` — "Owed: a staged-copy assertion over `pulse-app/capabilities/*.json` (CARRY'd onto the Staged-bindings assertion route entry)"), :298/:310 (§3 standard gate set, capability-drift LAST + the staged-copy note citing playbook 2026-08-22), :671 (§9 matrix step runs after capability-drift).
- `.claude/rules/testing.md` :169 (2026-05-13), :179 (2026-05-17), :258 (2026-08-15) + `.claude/rules/security.md` 2026-06-12 (in-context) — all FOUR prose entries VERIFIED present as the route entry cites them.
- `.github/workflows/ci.yml` :103–110 (capability-drift step, widening step, matrix step in the main test job), :191–196 (capability-drift report artifact upload), :221 (`supply-chain` job). Wiring precedent: a plain named `run:` step beside :103.
- `xtask/src/npm_gate.rs` (verdict grep) — the tri-state contract to mirror: `print_verdict(&json!({gate, arm, verdict, …}))` + `ExitCode::from(0|1|2)` with `cannot-evaluate` as its own arm (:352–396, :406–450).

## Graph impact (rust plane)
- **`capability_drift` / `capability_widening_check`** — callers: `main()` dispatch ONLY (main.rs:250–251). Extending `capability_drift()`'s body or adding a sibling verb has zero caller blast radius beyond the dispatch match.
- **`parse_bindings`** — callers: `capability_drift` + 3 `capability_drift_tests` fns. It takes `&str`, so the staged-content read reuses it unchanged (no signature work, no test churn).
- **`staged*` / `EXPECTED_GRANTS`** — zero symbols; names free (collision check clean).
- `run_ci_gates` (main.rs:335) is log-derived (zero-spans/zero-panic/heartbeat/perf over `agent-latest.jsonl`) — unrelated to this gate; do not wire there.

## Patterns detected
- **Gate-module-per-verb** (`xtask/src/npm_gate.rs`, `harness_status.rs`, `gap_resume.rs`): own file, `run()` → `Result<ExitCode>`, pure classifier + thin IO, co-located `#[cfg(test)] mod tests`. The new gate follows this shape (`xtask/src/staged_gate.rs`).
- **Tri-state verdict contract** (npm_gate.rs:352+, arch §Occupied Resources xtask CLI surfaces): named arms, exit 0 green / 1 findings-red / 2 cannot-evaluate, one JSON verdict object on stdout. A staged read has a real cannot-evaluate class: git unspawnable, not a repo, subject path absent from the index.
- **In-code pin as baseline** (`EXPECTED_PROCEDURES`, main.rs:1116; guarded by ~20 pin tests in `capability_drift_tests`): the grants baseline lands as a sibling `EXPECTED_GRANTS` const — per file `{identifier, windows, permissions}` — updated in the same commit as a legitimate grant change, exactly the EXPECTED_PROCEDURES discipline.
- **Subprocess spawn precedent** (Command::new throughout xtask: cargo/pwsh/bash/injector) — but **zero `"git"` invocations exist anywhere in xtask today**; `git -C <workspace_root> show :<repo-relative-path>` is a new (first) git subprocess. Index paths use forward slashes literally; `git show :<path>` yields the INDEX blob (on a fresh CI checkout, index == HEAD, so the same read validates the commit itself).
- **Structured event line + report.json twin** (capability_drift :1191–1240, capability_widening_check :1355+): `target/<gate>/report.json` + one obs-§3-shaped JSON line to stdout. xtask inits no tracing subscriber — stdout is the surface (so no obs allowlist leaf is owed; the obs extract's "research question" answers NO product tracing).

## Conventions to follow
- clap `#[command(name = "check:...")]` verb naming (`check:npm-supply-chain`, `check:ingest-progress` precedent) → `check:staged-artifacts` (or similar) fits the namespace.
- Pure-classifier + thin-IO split (test-plan §3 `harness_status::classify()` precedent): the verdict fn takes (staged bindings content or its parse result, staged capability JSON values, the two pins) → typed verdict; git IO stays outside; per-branch unit pins on the pure fn.
- Fixture repos built at test runtime in TempDir (`git init` + write + `git add`), never pre-staged, never mutating the real repo (test-plan §7). Consequence: the real `pulse-app/capabilities/*.json` is never modified by this chunk, so the §3 boot-smoke conditional trigger does NOT attach.
- Both current copies verified healthy at HEAD (worktree `"mcp":` count 1, staged count 1) — the gate lands GREEN against the real repo, and the RED proof runs in fixture repos (code-driven, repeatable) rather than by mutating the real tree.

## New files to create
- `xtask/src/staged_gate.rs` — the staged-artifacts gate: `EXPECTED_GRANTS` pin (per capability file: identifier + windows + permissions; descriptions deliberately excluded), git-index reads of `pulse-app/ui/src/bindings/index.ts` + `pulse-app/capabilities/*.json`, procedure diff via `crate::parse_bindings` against `crate::EXPECTED_PROCEDURES`, grant-set equality BOTH directions, tri-state verdict (JSON on stdout + exit 0/1/2), co-located per-branch unit tests + fixture-repo tests.

## Files to modify
- `xtask/src/main.rs` — `mod staged_gate;` declaration · new `Cmd` variant (`check:staged-artifacts`) + dispatch arm · `capability_drift()` extended to ALSO run the staged assertion after its worktree diff (any non-clean staged outcome → capability-drift's existing FAILURE exit, inner verdict printed), so every existing capability-drift invocation — every gate list where it runs LAST, the wrap's P7 slot, ci.yml :103 — transitively runs the staged check and it cannot be skipped. `parse_bindings` + `EXPECTED_PROCEDURES` are crate-root items visible to the child module (privacy: root-module items are visible to descendants); no signature changes.
- `.github/workflows/ci.yml` — one plain named `run:` step for the new verb beside the capability-drift step (:103 region), matching the npm-gate/widening-check precedent; optional report-artifact upload mirroring :191–196.
<!-- Caller threading: graph-proven none beyond the dispatch match (capability_drift's only caller is main()).
No manifest changes (git via subprocess; serde_json already a dep). The EXPECTED_GRANTS pin is an in-code
registry (xtask-side, per arch: a grant inventory is NOT an arch §Occupied Resources item); the arch
xtask-CLI-surface registry entry + test-plan §3/§9 + playbook re-point are WRAP amendments, not touchpoints. -->

## Open questions
- none — the wiring lean (fold into capability-drift + own verb + own CI step) and the pin surface ({identifier, windows, permissions}, descriptions excluded) are decisive material leans; both are stated in plan Provenance and ride the P5 review card.
