## 2026-11-03-stdio-sidecar-retired — the stdio sidecar retired; mcp-server the tools library the door serves
**Section:** §1 Test Scope Summary (the Justification; Coverage scope: the mcp-server row, the console engine program row; Surfaces under test: the MCP stdio sidecar row); §4 Unit Test Strategy (the mcp-server crate); §5 Integration Test Strategy (the module driver); §6 E2E Test Strategy (the driver row); §8 Mocking & Stubbing Discipline (the process row)
**Change:**
- The Justification's entity list names the mcp-server tools served through the console engine door (was the MCP sidecar).
- The mcp-server row reads "the tools library the door serves": no binary, no cargo feature and no env gate; the crate holds `contract`, `jsonrpc` and `tools`, and `pulse-app` names it as a plain dependency; `mcp_server::tools::call_tool` has one product caller, `door::EngineTools::call` (was a feature-gated, runtime-gated stdio sidecar with `rmcp` a feature-gated anchor dependency and `dispatch_tool` calling `call_tool`); `tools_list_manifest()` still marks `mark_incident_resolved` alone as a write.
- The console engine program is one of the two product binaries, beside `pulse-app` (was the third, beside `pulse-app` and `andromeda-pulse-mcp`).
- The MCP stdio sidecar surface row reads N/A — retired: no sidecar binary, no compile-time or runtime gate, no subprocess test; a stdio agent reaches the nine tools through the relay `andromeda-pulse-engine door`.
- §4: the mcp-server crate's unit scope names tool call routing (`call_tool`) and drops feature-gate build verification.
- §5: Module ↔ engine door, a `tokio::net::UnixStream` client writing JSON-RPC 2.0 lines to `run/door/sock` on an engine booted by `engine_boot::start` or a spawned `andromeda-pulse-engine run` (was Module ↔ MCP sidecar over `tokio::process::Command`).
- §6: the console engine door row replaces the MCP stdio sidecar row: no feature flag, no env var, one request per line, the answer line parsed.
- §8: the process row spawns the built `andromeda-pulse-engine` (`run`, or the `door` relay), no mocking library (was `tokio::process::Command` on the sidecar).
**Why:** the chunk removed the `andromeda-pulse-mcp` binary, its double gate (`--features mcp-server` + `ANDROMEDA_PULSE_MCP_ENABLED`), `rmcp`, `feature_gate`, `tracing_setup`, `dispatch_tool` and the sidecar's subprocess tests; `mcp-server` stays a workspace member as the tools library the door serves.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — the run/workspace-key publish retired
**Section:** §1 Test Scope Summary (Coverage scope: the workspace-detector row; Pending coverage triggers: `workspace-key-cross-process-coverage`); §5 Integration Test Strategy (the pulse-app boot → `run/workspace-key` → `init_incident_context` boundary row)
**Change:**
- workspace-detector: the engine stamps incidents with `incident_workspace_key`; neither program writes `<data_dir>/run/workspace-key`; `publish_workspace_key` / `read_published_workspace_key` stay in the library with no product caller (was a publish/read round-trip, the app writing at boot and the sidecar reading, exercised at the filesystem boundary).
- `workspace-key-cross-process-coverage` reads RETIRED — its subject no longer exists; the two functions' removal is owned by the route entry «Workspace detection retired»; the row retained for audit trail (was an unblocked, smoke-measured, uncommitted cross-process leg).
- The §5 key-handoff boundary row reads N/A — retired: no program publishes the key and no sidecar reads it.
**Why:** the chunk removed `engine_boot::publish_workspace_key_for_sidecar` and its call in `start`, and with the sidecar gone nothing reads the key; `incident_workspace_key` stays.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — the door's unit tests and the pulse-app test file count
**Section:** §1 Test Scope Summary (Surfaces under test: the console engine door row); §4 Unit Test Strategy (the Rust convention)
**Change:**
- `pulse-app/tests/unit_door.rs` holds 20 tests (was 19).
- `pulse-app/tests/*.rs` counts 112 top-level files, as measured by `ls pulse-app/tests/*.rs | wc -l` at this chunk's work tree (was 113 as of chunk 2026-10-12-stdio-agent-reaches-the-door); the earlier history of the count stands.
**Why:** the chunk added `an_undeclared_argument_member_is_refused_for_each_of_the_nine_tools` to `unit_door.rs` and deleted `pulse-app/tests/unit_mcp_router.rs` with the `mcp.*` TauRPC procedures.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — P3 driven through the console engine door
**Section:** §1 Test Scope Summary (Critical paths: P3); Scenario: P3 — MCP server query (Surfaces involved, Steps, Cleanup)
**Change:**
- P3: the agent spawns `andromeda-pulse-engine run`, sends telemetry over OTLP, writes a JSON-RPC 2.0 `tools/call` line to the door (`run/door/sock`, or the stdio relay `andromeda-pulse-engine door`), asserts `result` and the tool's shape and one `door.call` record per answered request (`pulse-app/tests/e2e_p3_mcp_subprocess_tools_call.rs`); no feature flag, no env gate (was the feature-gated sidecar fed on stdin).
- Surfaces involved: the console engine door, socket or stdio relay (was the MCP stdio sidecar, feature-gated).
- Steps: spawn the engine on its own data dir and two claimed loopback ports; send spans and logs over OTLP gRPC; ask through `run/door/sock`, one connection per request line; `tools/call` lines for `query_traces`, `query_metrics`, `query_logs`, `generate_snapshot` and two carrying an SQL-injection fragment and a sentinel cursor; the four calls answer `result`; each tool's shape, one `door.call` per answered request, the fragment and the sentinel 0 times in the log family (was a feature build, a sidecar spawn and `result.traces` read on stdout).
- Cleanup: end the spawned engine by SIGTERM and read its log family after the end (was SIGTERM to the sidecar PID, exit 0, no zombie).
**Why:** the sidecar left; the P3 test reads through the door of a spawned engine, and the SQL-injection and response-sentinel canaries became one door arm, `p3_mcp_tools_call_sql_injection_and_sentinel_cursor_stay_out_of_the_log`.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — Vector 4's coverage carried by the door
**Section:** §1 Test Scope Summary (Coverage scope: `security-vector-coverage: MCP feature + runtime gating`; Pending coverage triggers: `security-vector-coverage: MCP response body redaction`)
**Change:**
- The gating row reads N/A — retired: no `mcp-server` feature, no runtime gate, no sidecar; the nine tools answer only through the door (its owner check; the one write behind `ANDROMEDA_PULSE_DOOR_RESOLVE`), and its negative tests carry Vector 4: another account refused at three layers and through the stdio relay (runner only), the write refused with `-32001` when the switch is `off` or rejected, and an undeclared argument member refused with `-32602` for each of the nine tools (`declared_only`) (was the sidecar's compile-time and runtime negative and positive tests, the door a second path under neither gate).
- The redaction trigger reads LANDED, no longer pending: `pulse-app/tests/security_mcp_response_body_redaction.rs` calls `query_traces` through the door of a spawned engine over spans whose `service.name` resource attribute carries a marker, and asserts the marker 0 times in the engine's log family; the row retained for audit trail (was: assert `result_content` absent, only `result_type` + `result_count` metadata).
**Why:** the double gate left with the sidecar; `declared_only` is now the one refusal of an undeclared argument member; a trace row answers no attribute but `service.name`, so the marker rides it.
**Kept:** the door's PROVISIONAL sentence in §9 and every door statement resting on it stand as written; this pass narrows a surface and ratifies nothing.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — the incident tools' content tests read through the door
**Section:** §1 Test Scope Summary (Pending coverage triggers: `mcp-incident-read-back-cross-process-coverage`); Scenario: P3 — MCP server query (Current residual)
**Change:**
- The trigger row: in-process `call_tool` assertions in `pulse-app/tests/e2e_p3_mcp_incident_tools.rs` under default features (was `dispatch_tool`); `e2e_p3_mcp_resolve_content.rs` asserts `mark_incident_resolved` in both arms — the declined arm no `result`, an error answer carrying the door's fixed message, the row still `active` and listed — and `e2e_p3_mcp_incident_events_content.rs` asserts `retrieve_incident_events`; both boot the console engine in process with `FakeKeychainBackend` and read through the door socket in the default workspace run, on CI too, with no feature gate and no credential-store skip; `crates/mcp-server` holds no `tests/` dir (was feature-gated sidecar subprocess legs that skipped without an OS credential store).
- Still owed, the row OPEN: cross-process content for `retrieve_telemetry_slice.fingerprint_refs`, `retrieve_report`'s Evidence section and `query_incident_list`.
- The P3 residual restates the same and points the declined arm and the three owed tools at the §1 row (was the dated history of the sidecar legs).
**Why:** the chunk deleted `sidecar_subprocess.rs`, `incident_events_subprocess.rs` and `dispatch_tool`; the two corpus tests boot the engine with `FakeKeychainBackend`, so they need no credential store; the door's fixed message replaces the sidecar's "not applied" text.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — ci.yml's five jobs, the mcp-test job retired
**Section:** §9 CI Integration (the lint-test row; Release build; MCP-feature tests; Boot smoke; Matrix builds)
**Change:**
- The `mcp-test` row reads N/A — retired: `ci.yml` holds no `mcp-test` job and no `mcp-server` feature, pinned absent by `ci_workflow_holds_no_mcp_test_job_and_no_mcp_server_feature` (was `cargo nextest run --workspace --features mcp-server --profile ci --no-tests=fail`, restoring `lint-test-Linux`).
- lint-test: the engine-harness tests run in the `coverage` job too (was `mcp-test` and `coverage`); the `a11y` job alone restores the cache key read-only (was `mcp-test` and `a11y`).
- Matrix builds: each of the five jobs, lint-test · a11y · boot · supply-chain · coverage (was six).
- The boot job's release build reads `cargo build --workspace --release` in the Boot smoke and Release build rows, pinned by `ci_workflow_keeps_the_linux_release_build_witnesses` (was `… --features mcp-server`).
**Why:** the chunk removed the `mcp-test` job and the `mcp-server` feature; `data_dir_export_precedes_every_consumer` pins `("ci.yml", 5)`.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — the bindings regen with no feature
**Section:** §3 Per-chunk gate discipline (contracts/per-chunk-gate-discipline: the regen line; the drift and regen ordering)
**Change:**
- The regen line reads `cargo nextest run -p pulse-app --bin pulse-app -E 'test(emit_taurpc_bindings)'` (was with `--features mcp-server`).
- The default-features workspace run rewrites `pulse-app/ui/src/bindings/index.ts` in the default build's shape, the whole procedure set, so it no longer shrinks the bindings (was: to the no-mcp shape, repaired by the feature regen).
- A chunk that changes the procedure set closes with the regen → one content probe (the new procedure's name) → `git add` with the `EXPECTED_PROCEDURES` pin → `check:staged-artifacts` → `capability-drift` LAST (was two probes, the second `grep -c '"mcp":'` ≥ 1).
**Why:** with the feature gone the default build carries the whole procedure set, and the `mcp` namespace left the bindings.
**Kept:** `capability-drift` still runs before the workspace run, though the clobber it was placed to avoid no longer occurs.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — smoke:external-resolve refuses at its precondition
**Section:** §3 Per-chunk gate discipline (contracts/per-chunk-gate-discipline: the `smoke:external-resolve` scenario leg)
**Change:**
- The leg refuses at its precondition: the `andromeda-pulse-mcp` binary it resolves through is built by no target; its leg against the console engine is owned by the route entry «Scenario legs driven against the console engine» (CARRY 2). Its measured description stands.
**Why:** `xtask/src/external_resolve.rs` is untouched while the binary it spawns left with the sidecar.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/
