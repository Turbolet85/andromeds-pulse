## Relevant amendment history
- **2026-05-08** (Document missing PII vector test coverage) — added the `MCP response body redaction` pending trigger (security-plan §Logging vector 4) to §1. It is one of the three sidecar pending rows whose fate this chunk settles at the wrap. The chunk's tests must not go on asserting it as a sidecar behaviour.
- **2026-05-09** — created the §3 Boot-smoke gate (conditional) and the `boot-smoke-coverage` trigger, with `pulse-app/src/main.rs` and `pulse-app/capabilities/*.json` among its paths. This chunk edits five cfg-gated sites in `main.rs` and possibly the capabilities (the tray item, the mcp procedures), so the smoke gate or its accepted form is owed.
- **2026-05-10** (chunk plans MUST include the standard gate baseline) — made the standard gate set unconditional in every chunk plan's Test Commands, and the three webview gates required whenever `pulse-app/ui/**` is touched. This chunk touches it: the Send to agent button, `use-mcp-delivery` and the Settings switch.
- **2026-06-10** — the original capability-matrix rule: a chunk that changes a capability's proofs updates the record in the same chunk, and `verify:capability-matrix` fails on dangling paths or anchors. This is the origin of the rule that P-039's re-point must land in this chunk, since its two webview proofs are deleted. The current gate form is in 2026-10-10-capability-record-re-based.
- **2026-08-14-fingerprint-feed-capture-repair** — set the `pulse-app` exception: `[lib] test = false`, so its tests live in `pulse-app/tests/*.rs` and reach internals through `pub` + `#[doc(hidden)]`. The four rewritten tests stay there.
- **2026-08-14-workspace-key-alignment** — this entry did four things:
  - added the §5 boundary `pulse-app` boot-publish → `<data_dir>/run/workspace-key` → `mcp-server` `init_incident_context`, which this chunk removes along with `publish_workspace_key_for_sidecar`;
  - minted the `workspace-key-cross-process-coverage` pending trigger for the sidecar-subprocess leg;
  - put `capability-widening-check` in the standard gate set.

  No committed test may go on asserting this handoff.
- **2026-08-15-corpus-key-persistence** — `workspace-key-cross-process-coverage` was unblocked but stayed unimplemented. It presumes a credential store or a configured passphrase. Trap: the sidecar-era rows assume a store-backed host, and the rewritten door tests inherit the same clean-skip conditions.
- **2026-08-15-tier-1-incident-path-investigation** — narrow a `pulse-app` run with `-E` under `--workspace`, not `-p`, because `-p` resolves a narrower feature graph and was measured to fail linking after a workspace build. This bears on the rewritten bindings-regen line: today's form is `cargo nextest run -p pulse-app --features mcp-server …`, and it must be restated without the feature. It also bears on the rewritten tests' selection.
- **2026-08-17-conductor-e2e-verification-closure** — two changes:
  - minted `mcp-incident-read-back-cross-process-coverage`: the cross-process subprocess tests `tools/call` only the telemetry tools;
  - added the §6 P3 "Current residual": the incident-tool leg is in-process despite the `e2e_p3_` filename.

  Both are sidecar rows this chunk's wrap settles. `pulse-app/tests/e2e_p3_mcp_incident_tools.rs` is the in-process file that P-036/P-039 keep.
- **2026-08-29-app-registry-reconciliation** — two changes:
  - `pulse-app/tests/e2e_p3_mcp_resolve_content.rs` spawns a real `andromeda-pulse-mcp` and asserts `mark_incident_resolved` content in both the applied and the guard-declined arms. It is feature-gated, and it is one of the sidecar-spawning tests this chunk rewrites to read through the door. Keep both arms.
  - `cargo xtask smoke:external-resolve` was registered as a non-gate scenario leg. Its verdict excludes `item_count` and it is not drivable by `--sustained`. CARRY 2 asks whether it spawns the sidecar.
- **2026-08-29-advisory-backlog** — the §1 mcp-server row's citation moved from arch's "rmcp" Stack wording to the hand-rolled JSON-RPC / anchor-dependency entry. This chunk drops the `rmcp` dependency and its `deny.toml` entries, so that citation and the row are what the wrap re-reads.
- **2026-08-26-ingest-consumer-stall-under-sustained-load** — the first ordering note for `capability-drift` relative to binding regeneration. Its LAST placement was later superseded twice (2026-09-30-dual-license, then 2026-09-30-perf-instruments-measure-their-budgets). The ordering exists because the workspace run rewrites `bindings/index.ts`.
- **2026-08-30-staged-bindings-assertion** — added `check:staged-artifacts` to the gate set, folded into `capability-drift`. It reads the git index for the bindings and every `capabilities/*.json`, and `staged_gate::EXPECTED_GRANTS` checks set equality in both directions. Removing the tray item or the mcp procedures from capabilities moves these pins.
- **2026-08-30-diagnostics-un-muting-harness-truth-sweep** — shipped the ratchet `pulse_app_src_carries_no_new_dead_test_attributes` (`main.rs` exempt). Deleting `pulse-app/src/mcp_router.rs` and editing `main.rs` must keep it green.
- **2026-08-30-dead-lib-src-test-migration** — set the ratchet to flat zero and moved `mcp_router`'s tests to the feature-gated `pulse-app/tests/unit_mcp_router.rs` (6 tests). This chunk deletes it with `mcp_router.rs`. The by-name ledger and the file count (now 113) move.
- **2026-09-29-ci-wall-time-and-round-trips** — added the `mcp-test` CI job row (restore-only cache) among the jobs. This chunk removes that job.
- **2026-09-30-perf-budget-gate-reads-real-samples** — the `lint-test` cache key is restored read-only by `mcp-test` and `a11y` only. Removing `mcp-test` changes that cache statement in §9.
- **2026-09-30-dual-license** — set the gate order: `capability-drift` before the default-features workspace nextest. The workspace run rewrites the bindings to the no-mcp shape. Then comes the `--features mcp-server` `emit_taurpc_bindings` regen as the last cargo step, closed by `git diff --quiet <base> -- pulse-app/ui/src/bindings/index.ts`. Once the feature leaves, the no-mcp and mcp shapes are the same, and the chunk must restate the regen form.
- **2026-09-30-perf-instruments-measure-their-budgets** (procedure-changing bindings close specified) — specified the close this chunk must run, because three `mcp.*` pins leave `EXPECTED_PROCEDURES`: mcp-feature regen after the workspace run → content probes → `git add` of the bindings with the pin → `check:staged-artifacts` → `capability-drift` LAST. The regen also comes before the `ui/dist` and release builds. Its "mcp-feature regen" and the `"mcp":` probe depend on the removed feature, the conflict P2 must settle.
- **2026-10-02-incident-events-readable-through-mcp** — added two sidecar cross-process content legs:
  - `crates/mcp-server/tests/incident_events_subprocess.rs`, one of the `mcp-server` crate's two subprocess tests this chunk deletes;
  - `pulse-app/tests/e2e_p3_mcp_incident_events_content.rs` (`--features mcp-server`, real producer into the real sidecar, four-kind vocabulary with `created` first), one of the four rewritten tests.

  Both clean-skip without a credential store. The real-producer leg caught the `created` vocabulary defect, so the door rewrite must keep the real producer.
- **2026-10-09-ci-on-linux-alone** — named the `boot` job's `cargo build --workspace --release --features mcp-server` as a release-build witness, pinned by `ci_workflow_keeps_the_linux_release_build_witnesses`. It also stated "six jobs" in §9 Matrix builds. This chunk removes the feature from that step and the `mcp-test` job, so the pin and the job count move.
- **2026-10-10-no-ci-step-reads-nothing** — three relevant points:
  - `ci_workflow_test_gates_no_continue_on_error` reads every step block, and its listed-gate arm has 14 commands. The `mcp-test` nextest line is presumably among them, and the arm loses it.
  - Each upload has `if-no-files-found: error`.
  - The a11y regression detector reads the baseline committed in the tree and fails when it is absent. Deleting the Send to agent a11y spec must leave that baseline present and passing.
- **2026-10-10-no-gate-stands-while-reading-nothing** — two sub-entries:
  - The MCP-feature tests row moved to `--no-tests=fail`, pinned by `ci_workflow_nextest_runs_fail_on_an_empty_selection`. That pin and the row leave with `mcp-test`; every remaining nextest line keeps the flag.
  - The coverage gate is line ≥ 75 % and function ≥ 85 % (branch retired), and an empty report fails. Deleting the sidecar moves the measure; the thresholds stay.
- **2026-10-10-capability-record-re-based** — `verify:capability-matrix` reads `docs/capability-record.json` against `andromeda-pulse-0.4.0/working-route.md`. A claimed id's scenario file refs must exist and their `contains` anchors must be found. Standing trap: a record that names a deleted file or a renamed or retired route entry reddens the verb and the pin `the_committed_record_reads_clean_over_the_committed_route`. P-039 must be re-pointed in the same chunk.
- **2026-10-10-console-engine-entry-point** — four relevant points:
  - recorded the §3 Spawned-program witness form;
  - added `pulse-app/src/engine_boot.rs` to the boot-smoke path list;
  - noted that `main.rs` is one of two `[[bin]]` targets;
  - moved wiring rows from `main.rs` to `engine_boot`.

  The witness form governs the rewritten spawned tests. `main.rs`'s cfg-gated sites are what this chunk strips.
- **2026-10-10-agent-harness-drives-the-console-engine** — extended the `boot` job's chain: smoke → series → `ci-gates` → `Console engine cycle` → uploads, with the job failing on a non-zero cycle. This chunk edits the same job's release-build step, and the step sequence and its workflow pins must stay intact.
- **2026-10-10-engine-harness-proofs-completed** (the harness contract) — spawned tests claim two ports from 20000–31999 under an advisory file lock (24317/24318 excluded). They never bind 0 and release it, and they add the rejected-port, socket-census and lock-file readings. The rewritten spawned tests follow this.
- **2026-10-10-shared-telemetry-test-data** — sub-entries on the factories and the one port claim:
  - one telemetry factory set, `test_support::otlp` in `crates/test-support`, with residue counts of hand-built literals recorded;
  - one port claim, `test_support::ports::claim_port`, with no bind-and-release anywhere.

  The rewritten tests seed through the factories with no new hand-built literals, and claim ports through `claim_port`.
- **2026-10-10-door-inside-the-engine-s-process** (the door as a surface under test) — this entry:
  - added the door row: three depths (`unit_door.rs`, the in-process arms of `integration_engine_boot.rs`, a spawned `integration_door.rs`), eight fixed codes, Unix only;
  - restated the mcp-server row and the `MCP feature + runtime gating` trigger: both gates are the sidecar's alone and the dispatch is reached through the door under neither;
  - noted that `tools/list` carries `write`;
  - left the three sidecar pending rows and §6 P3's residual explicitly for "the removal entry", this chunk.
- **2026-10-10-door-inside-the-engine-s-process** (the second-account step in CI) — added the `lint-test` step `Door refuses another account`, pinned by `pulse-app/tests/quality_gate_workflow.rs` pins and `bash -e` witnesses, with a skip reading red. This chunk edits the same workflow file and the same pin file, and must leave this step and its pins intact.
- **2026-10-12-stdio-agent-reaches-the-door** (relay witnesses) — added the fourth depth, the stdio relay `andromeda-pulse-engine door`, witnessed by `unit_door_bridge.rs` and `integration_door.rs` (7 arms). A chunk touching the door's answers is gated by both. The `initialize` name and tool-description edits fall here, as does `quality_gate_workflow.rs` (42 tests).
- **2026-10-12-stdio-agent-reaches-the-door** (the `pulse-app/tests` file count) — 113 top-level `pulse-app/tests/*.rs` files. This chunk deletes `unit_mcp_router.rs` (and any other sidecar-only file), so the count moves at the wrap.
