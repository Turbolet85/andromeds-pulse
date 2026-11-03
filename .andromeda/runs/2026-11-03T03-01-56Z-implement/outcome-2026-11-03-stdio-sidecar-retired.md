# /andromeda-implement — 2026-11-03-stdio-sidecar-retired

**Outcome: green + smoke** — one entry, 3 (the scope guard), stays red on a recorded companion and is left for the
operator's word at the wrap's light gate (below). Next → the OPERATOR PASS (entries 30 hygiene, 31 merge-base probe,
32 the build-branch push, 33 the CI read, report-only on `supply-chain` per `inputs#I3`), then `/andromeda-wrap-session`.

Run dir `.andromeda/runs/2026-11-03T03-01-56Z-implement/` · gate trail `gate-2026-11-03-stdio-sidecar-retired.json` ·
base `f2bb258f`, nothing committed, nothing pushed.

## What was built
- The stdio sidecar is gone: `andromeda-pulse-mcp`, its `[[bin]]`, `feature_gate.rs`, `tracing_setup.rs`, the crate's
  two subprocess tests, the `mcp-server` cargo feature (crate and `pulse-app`), the workspace `rmcp` entry (`rmcp` and
  `rmcp-macros` left `Cargo.lock`: 108 lines out), `ANDROMEDA_PULSE_MCP_ENABLED`, the five sidecar-only `Error`
  variants and their `AppError` arms, `dispatch_tool` and its three `mcp.*` records.
- The `mcp-server` crate stays as the tools library; `call_tool` is unchanged. `mark_incident_resolved`'s
  description now reads "…The running engine drops it from its active set within one persist cycle."; `serverInfo.name`
  stays `andromeda-pulse-mcp` (`inputs#I2`).
- The engine no longer publishes `run/workspace-key` (no `app.boot.workspace_key` record, its leaf and the `mcp-server`
  leaf removed). Both the window smoke's two boots and the engine cycle read neither the file nor the record.
- Window app: `mcp_router.rs` and its test, the five cfg-gated wiring sites and the bindings test merge, the tray
  toggle, `Settings.mcp_server_enabled`, the ready check of that name, the `config-watcher` arm, the three
  `EXPECTED_PROCEDURES` pins and their pin test. The bindings were regenerated and staged with `xtask/src/main.rs`
  (gate entry 16); they hold no `mcp` namespace and no `mcp_server_enabled`.
- UI: Send to agent button and `use-mcp-delivery` deleted; the Settings MCP switch removed; fixtures and a11y helpers
  cleaned. The a11y baseline is untouched.
- Tests through the door: `e2e_p3_mcp_subprocess_tools_call.rs` and `security_mcp_response_body_redaction.rs` spawn
  `andromeda-pulse-engine run` and ask its socket; `e2e_p3_mcp_resolve_content.rs` and
  `e2e_p3_mcp_incident_events_content.rs` boot the console engine in process (`FakeKeychainBackend`) and ask its socket
  — no credential-store skip any more; `e2e_p3_mcp_incident_tools.rs` runs under default features via `call_tool`.
  `unit_door.rs` gained `an_undeclared_argument_member_is_refused_for_each_of_the_nine_tools` (`inputs#I2`).
- CI: the `mcp-test` job removed (6 → 5 jobs); the boot job's release build is `cargo build --workspace --release`;
  `quality_gate_workflow.rs` pins it and gained `ci_workflow_holds_no_mcp_test_job_and_no_mcp_server_feature`.
- `deny.toml`: the `schemars` / `schemars_derive` skips removed (cargo deny reported them unnecessary); the `darling`
  group stays (serde_with 0.23 and derive_builder 0.20 still split it), its comment re-pointed to those two holders.
- `docs/capability-record.json`: P-039's two `ui-test` scenarios re-pointed to two `nextest-file` scenarios over
  `pulse-app/tests/integration_door.rs`; nothing else changed.

## Gates (one full block, 33 entries; trail `gate-2026-11-03-stdio-sidecar-retired.json`)
`entries 33 · green 26 · red 2 (3,4) · recorded 1 · timeout 0 · not-run 4` on the block's one firing; entry 4 then
re-run once (`--entry 4`) and read green. The four not-run are `leg = 'operator'` (30–33), the operator pass's.
- 6 (targeted door/rewrite set): `38 tests run: 38 passed`. 12 (workspace): `3166 tests run: 3166 passed, 0 skipped`.
- 13–18 the bindings close: regen green, both content probes `last line 0`, staged with the pin,
  `check:staged-artifacts` and `capability-drift` green.
- 19 `cargo deny check bans licenses sources`: green. 20 `cargo audit` (report-only, `inputs#I3`):
  `error: 42 vulnerabilities found!` · `warning: 29 allowed warnings found` · **62 distinct RUSTSEC ids, identical to
  the P5 set** (`p5-audit-ids.md`): none added, none dropped. Owner: «Advisories since 2026-10-12 closed», minted by
  the wrap.
- 21–25 webview: lint, typecheck green; vitest `Tests 848 passed (848)` in 80 files; build green; the a11y suite lists
  `Total: 41 tests in 18 files`.
- 26 window smoke: `"verdict": "all-settled"`, ordinals 2 and 3, two `app.boot.engine` records with
  `"program":"window"`, no `run/workspace-key`, no `app.boot.workspace_key`; artifact fresh.
- 27 engine cycle: `"verdict": "pass"`, `"witness_file": "absent"`, `"error_records": 0`, `engine-log: PASS`,
  `cleanup: clean`, `app.boot.door` `"state":"listening"`, no `run/workspace-key`; artifact fresh.
- 28 `pre-push:linux`: `"verdict": "green"`, `"reason": "all-stages-ok"`. 29: the stage left no pid file and both
  harness ports refuse.
- Smoke (P3): ran as P2 entries 26 and 27, not re-run. The headful `cargo xtask self-verify` is Windows-only
  (`msedgedriver`) and was not run on this Linux host; `cargo xtask test:a11y` is the CI `a11y` job's (plan prose).

## Deviations, with justifications
1. **Entry 3 (scope guard) red — the plan's wording.** Its log lists one file, `xtask/src/staged_gate.rs`. That file
   is a companion research's Files to modify missed: four of its tests build their "missing" set by filtering `mcp.*`
   out of `EXPECTED_PROCEDURES`, which is empty once the three pins leave, so the tests could no longer red. They were
   re-pointed to the `plugins.*` namespace (same arms, same assertions). Scope record: ``- `xtask/src/staged_gate.rs` ·
   companion · serves step 7 · self``. `gate.py scope` reads `clean`. The guard's exclusion list was generated from
   research's list, so no form of this companion passes it; the light gate's ASSERT needs
   `red — the plan's wording: entry 3 against the scope record's companion line · word: "…" — {who}, {date}`. The word,
   given after this report (`inputs#I5`): `red — the plan's wording: entry 3 against the scope record's companion line
   for xtask/src/staged_gate.rs · word: "Entry 3: I accept xtask/src/staged_gate.rs as a companion (read the diff: four
   tests re-pointed from mcp.* to plugins.*, nothing else); record my word for the wrap." — delegate the overseer
   (founder-delegated), 2026-11-03` — PROVISIONAL until the founder's own word (§Scope, *Whose word*).
2. **Entry 4 red on its first firing — block order.** Its grep covers `pulse-app/ui/src`, whose committed
   `bindings/index.ts` still held `mcp_server_enabled` (3 lines) until entry 13 regenerated it. Re-run after entry 13:
   green, exit 1, no output. No edit; one extra firing of a 0.04 s probe.
3. **The redaction marker rides `service.name`.** Step 12 plants a marker "in an attribute value" and reads it back
   through `query_traces`, but a trace row answers only service, ids, duration and error count. The marker is the
   `service.name` resource attribute's value; the test asserts it is in the answer, 0 times in the log family, and one
   `door.call` with `result_type` `paginated_traces` and `result_count` equal to the items.
4. **Step 10's `TauRPC__mcp.*` mocks did not exist** in `tests-a11y/helpers/mock-tauri.ts`; what stood were
   `mcp_server_enabled` in the ready mock and `mcp_enabled` in the settings mock, both removed. The spec's only MCP
   text was its `describe` title, now "P3 Settings modal".
5. **Tests kept rather than deleted** where only one assertion named the setting: `partition.rs`'s restart-bucket test
   (renamed `drain_retention_changes_land_in_restart_bucket`), `health.rs`'s app_info test (passes `example-feature`),
   `contract.rs`'s default-settings test (renamed `settings_default_notifications_enabled_true`).
6. **The double-gate test** (`p3_mcp_subprocess_double_gate_negative_canary_unset_env_exits_clean`) left with the gate;
   the SQL-injection and response-sentinel canaries became one door arm reading the engine's log family.
7. **The declined resolve arm** asserts no `result`, an error answer and the row still `active` and still listed; the
   door answers a fixed message, so the sidecar's "not applied" text is no longer read.
8. `deny.toml:40-60` keeps its history sentences naming the rmcp 3 bump; none names rmcp as a current holder.
9. The `mcp-server` crate keeps its `tempfile` / `rstest` dev-dependencies (no remaining user; nothing reads unused
   dev-dependencies; the plan named only the five runtime ones).

## Surfaced / stuck
None stuck. For the wrap: entry 3's plan-wording red (deviation 1) needs the operator's word; the research list's
missing companion is a report entry, not an edit.

## Matrix
No `coverage` row carries this chunk (`select * from coverage where chunk = '…'`: 0 rows); no ref written. P-092 stays
in the unclaimed pool.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `pulse-app` (two window boots, entry 26) | a leg (entry 26's `harness:boot-series`) | terminated |
| `andromeda-pulse-engine` (entry 27; pre-push sixth stage, entry 28; four spawned test engines, entries 6/12) | entries 27, 28, 6, 12 | terminated |
| `Xvfb` / `xvfb-run`, `inject_demo` | entries 26, 27, 28 | terminated |

Measured by `gate.py census` (comm `pulse-app`, `andromeda-pulse-engine`, `andromeda-pulse-`, `Xvfb`, `xvfb-run`,
`inject_demo`, `WebKitWebProcess`; args `target/boot-smoke`, `target/engine-cycle`, `target/pre-push`):
`census: none running`; loopback 14317 and 24317 refuse. `started by` is hand-attributed.

## Decisions & corrections
- The overseer's directive (founder-delegated, `inputs#I4`'s invocation): lean checks — each of the 33 entries once.
  Kept: one full block; the only re-firing was entry 4 after the regen made its subject current. Outside the block,
  while writing: two `cargo check` passes, one `cargo deny check bans` pair (step 5 decides the skips by it) and one
  single-test `cargo nextest` run of the new nine-tool door test.
- Diff-shaped probes anchored to `f2bb258f`, never HEAD (the same directive): entry 3's base is the chunk base.
- Bash guard, cwd arm, refused a leading `cd` twice (into the skill's references dir; into `pulse-app/ui`), each
  re-issued once with absolute paths — recurrence-despite-learning: `rules/host-linux.md` Session Additions 2026-10-05.
- No product question or hand step arose; no dialog was raised.
</content>
</invoke>
