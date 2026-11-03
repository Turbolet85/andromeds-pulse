# P5 review — 2026-11-03-stdio-sidecar-retired · «Stdio sidecar retired»

**Plan:** `doc:2026-11-03-stdio-sidecar-retired/plan` rev 2 — its render is at
`.andromeda/cache/docs/2026-11-03-stdio-sidecar-retired/plan.md` to open in an editor. No plan stands in a chunk's
folder; writing in that file changes no record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-03-stdio-sidecar-retired`
is a diff of the chunk's records.

## Goal
The stdio sidecar `andromeda-pulse-mcp` leaves whole: the binary, the `mcp-server` feature and `rmcp`,
`ANDROMEDA_PULSE_MCP_ENABLED` (the double gate), its sink, `dispatch_tool` and its `mcp.*` records, the
`run/workspace-key` publish, the window app's MCP side (`mcp.*` procedures, tray toggle, `Settings.mcp_server_enabled`
and its ready check, the Settings switch, the Send to agent button) and the `mcp-test` CI job. The engine is left with
one way in besides telemetry: the door. The `mcp-server` crate stays as the tools library the door serves. The four
tests that spawned the sidecar read through the door; the door itself does not change.

## Your answers this phase (the overseer as the founder's delegate, PROVISIONAL)
- `inputs#I2` — an undeclared argument member is refused at the door alone (`declared_only`, `-32602`), proved by a
  committed test over all nine tools; `serverInfo.name` stays `andromeda-pulse-mcp`.
- `inputs#I3` — `cargo audit` reads red on the untouched tree (42 vulnerabilities, 62 distinct ids with the allowed
  warnings, all external decay since 2026-10-12; the set is in `.andromeda/runs/2026-11-03T02-29-07Z-phase/p5-audit-ids.md`).
  This chunk goes on as a removal; the local audit and the CI read are report-only; the CI verdict reads green on every
  check but `supply-chain`.

## Top acceptance criteria
- No `andromeda-pulse-mcp` `[[bin]]`, no `mcp-server` feature, no `rmcp` in any manifest or `Cargo.lock`;
  `ANDROMEDA_PULSE_MCP_ENABLED` read nowhere; the door's admission, bounds, socket, codes and records unchanged.
- The door refuses an undeclared member `-32602` for each of the nine tools; nothing of it reaches an answer.
- `EXPECTED_PROCEDURES` and the staged bindings hold no `mcp`; `check:staged-artifacts` and `capability-drift` exit 0.
- The four rewritten tests run under default features through the door (telemetry shapes; resolve applied and
  declined; the event vocabulary; a planted marker 0 times in the engine's log beside one `door.call`).
- Neither program writes `run/workspace-key` or `app.boot.workspace_key` (window smoke, engine cycle).
- `ci.yml` has no `mcp-test` job and no `--features mcp-server`; the boot job keeps `cargo build --workspace --release`.
- `verify:capability-matrix` exits 0 with P-039 re-pointed to the door's two tests.
- The pushed tip's CI: every check succeeds but `supply-chain`, red on `cargo audit` alone (the owned decay).

## Touchpoints
57 files (research rev 2's list): the `mcp-server` crate (5 files deleted, 4 edited), the workspace and `pulse-app`
manifests, `Cargo.lock`, `deny.toml`, the window app (`main.rs`, `lib.rs`, `tray.rs`, `engine_boot.rs`,
`observability.rs`, `mcp_router.rs` deleted, the capability description), `ui-bridge` and `config-watcher`, 10
`pulse-app` tests, `xtask/src/main.rs`, `ci.yml`, the capability record, 16 UI and a11y files, two comments in
`corpus` and `triage`. Not touched: `pulse-app/src/door.rs`, `door_bridge.rs`, `workspace-detector`,
`xtask/src/external_resolve.rs`.

## Test commands (33 entries, each run once)
fmt · clippy · a scope guard on base `f2bb258f` · a removed-identifier probe · an `rmcp` lock probe · a targeted nextest
run · english-sources · quarantine · widening · ingest-progress · capability matrix · the workspace nextest · the
bindings regen, two content probes, the staged close, staged-artifacts, capability-drift · `cargo deny` · `cargo audit`
(report-only) · webview lint, typecheck, test, build · the a11y `--list` · the window boot-series smoke · one engine
cycle · the local pre-push check and its port probe · operator: hygiene, merge-base probe, push, the CI read
(report-only).

Baselines on the untouched tree: 17 entries, green 10, red 7. Six reds are the subject this chunk removes (the
identifiers, the lockfile entries, the bindings' `mcp` and `mcp_server_enabled`, `run/workspace-key` in both
programs); the seventh is `cargo audit`'s owned decay. The guard's two controls are in
`.andromeda/runs/2026-11-03T02-29-07Z-phase/p5-controls.md`.

**Changed from what you might expect:**
- `cargo xtask test:a11y` is not a local entry. It read red on this host only because the Playwright headless shell
  is not installed. Lean checks: no install; CI's `a11y` job runs the same command and is read by name.
- No headful self-verify: it is a Windows-host harness; the window smoke and the a11y suite stand in.
- No repeat loop over the engine-spawning binaries.

## Concretization preview
No capability is claimed. P-092, the entry's tag, cannot be proved by this chunk: its acceptance also needs the
filters, the whole incident, the snapshot's consumer and the harness reading through the door, which other route
entries carry. After the word it gets one dated note (advanced, not claimed: the sidecar is gone and the door is the
only stdio way in). `sampler: off` (all seven masters whole; no cascade ran).

## Expected amendments (wrap)
architecture, security-plan, test-plan, obs-plan, design-system, layout-templates, a11y-plan — the sidecar sentences,
"two ways in", the double gate, the stdio carrier, three product binaries → two, the `mcp-test` row, test-plan §1's
three sidecar pending rows and §6 P3's residual settled through the door, the never-built "MCP server status change"
notification corrected as wording (the plan's Implementation notes list each section).

## Beyond the plan
| proposal | kind | place |
|---|---|---|
| «Advisories since 2026-10-12 closed» — the owner of the 42 `cargo audit` vulnerabilities recorded at P5 (`inputs#I3`); the wrap mints it | route insertion | `andromeda-pulse-0.4.0/working-route.md`, directly after «Stdio sidecar retired», Epoch 1 |

Apply? (yes / review / cancel)
