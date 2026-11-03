proposals:
  - detector: D-tests-framework
    severity: warning
    section: '§3 → Per-chunk gate discipline'
    change: 'The standard gate set bindings-regen line reads `cargo nextest run -p pulse-app --bin pulse-app -E ''test(emit_taurpc_bindings)''` (no feature since chunk 2026-11-03-stdio-sidecar-retired: the default build is the whole procedure set), still the last cargo-adjacent step.'
    sidecar: 'Per-chunk gate discipline: the bindings regen runs with no `--features mcp-server` (feature removed, 2026-11-03-stdio-sidecar-retired).'
    rationale: 'Report Harness bullet: the bindings regen now runs with no feature; Schema bullet: cargo feature `mcp-server` removed from `crates/mcp-server` and `pulse-app`.'
    basis: 'test-plan fct-7jzqlhyk; doc:2026-11-03-stdio-sidecar-retired/report fct-vyk3olls; doc:2026-11-03-stdio-sidecar-retired/report fct-xsrenprx'
  - detector: D-tests-framework
    severity: warning
    section: '§3 → Per-chunk gate discipline'
    change: 'The ordering note says the `emit_taurpc_bindings` regen runs with no feature, and that since chunk 2026-11-03-stdio-sidecar-retired a default-features workspace run no longer rewrites the bindings to a smaller shape (the bindings hold no `mcp` namespace; the clobbered-worktree reading is history up to that chunk); the procedure-set close drops the `grep -c ''"mcp":''` ≥ 1 probe and keeps the new procedure name probe, the `git add` with `EXPECTED_PROCEDURES`, `check:staged-artifacts` then `capability-drift` last.'
    sidecar: 'Per-chunk gate discipline ordering note: no-feature regen; the workspace run no longer shrinks the bindings; the `"mcp":` probe dropped.'
    rationale: 'Report Harness bullet: the default build is the whole procedure set, so a default-features test run no longer rewrites the bindings to a smaller shape; Symbols bullet: bindings regenerated with no `mcp` namespace; Gates: `grep -c ''"mcp"''` exit 1, 0.'
    basis: 'test-plan fct-2ilc4zyz; doc:2026-11-03-stdio-sidecar-retired/report fct-vyk3olls; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc; doc:2026-11-03-stdio-sidecar-retired/report fct-6fhefylz'
    dependent-of: D-tests-framework
  - detector: D-retired
    severity: warning
    section: '§1 Test Scope Summary → Coverage scope'
    change: 'The mcp-server row reads: **mcp-server (the tools library the door serves)** — since chunk 2026-11-03-stdio-sidecar-retired no binary, no cargo feature, no env gate and no `rmcp`; the crate holds `contract`, `jsonrpc`, `tools` and `pulse-app` names it as a plain dependency; `mcp_server::tools::call_tool` (no log record) has one product caller, `door::EngineTools::call`, testable in process and on a spawned engine (Surfaces under test, the door row); `dispatch_tool` is removed; `tools_list_manifest()` gives every tool a boolean `write`, true for `mark_incident_resolved` alone (pin `tools_list_manifest_marks_exactly_one_tool_as_a_write`).'
    sidecar: 'Coverage scope: mcp-server row restated as the tools library the door serves; the stdio sidecar, its double gate, `rmcp` and `dispatch_tool` retired.'
    rationale: 'Report Retired bullet names the sidecar and its double gate; Symbols bullet removes the binary, `feature_gate`, `tracing_setup` and `dispatch_tool`, `call_tool` keeping one caller in `door.rs`; Dependencies bullet removes `rmcp`; Crates bullet keeps `mcp-server` as the door tools library.'
    basis: 'test-plan fct-rjbmbvd5; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc; doc:2026-11-03-stdio-sidecar-retired/report fct-yabevdww; doc:2026-11-03-stdio-sidecar-retired/report fct-yn7uwxjo'
  - detector: D-retired
    severity: warning
    section: '§1 Test Scope Summary → Justification'
    change: 'The entity list names the mcp-server tools served through the console engine door in place of the MCP sidecar.'
    sidecar: 'Justification: MCP sidecar entity replaced by the door-served mcp-server tools.'
    rationale: 'Report Retired bullet: the stdio sidecar `andromeda-pulse-mcp` retired; Crates bullet: `mcp-server` stays as the tools library the door serves.'
    basis: 'test-plan fct-6c37jotm; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-yn7uwxjo'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§1 Test Scope Summary → Coverage scope'
    change: 'The console engine row Notes read: one of the two product binaries beside `pulse-app` since chunk 2026-11-03-stdio-sidecar-retired (the third beside `pulse-app` and `andromeda-pulse-mcp` from chunk 2026-10-10-console-engine-entry-point until it); the rest of the row unchanged.'
    sidecar: 'Console engine row: product binaries 3 → 2.'
    rationale: 'Report Symbols bullet: the product binaries are now two, `pulse-app` and `andromeda-pulse-engine`; Counts bullet: product binaries 3 → 2.'
    basis: 'test-plan fct-qlyoxiil; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc; doc:2026-11-03-stdio-sidecar-retired/report fct-wnpvzb4g'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§1 Test Scope Summary → Surfaces under test'
    change: 'The MCP stdio sidecar (rmcp on stdin/stdout) row is retired since chunk 2026-11-03-stdio-sidecar-retired (binary, compile and runtime gates and subprocess tests gone); an agent on stdio reaches the nine tools through the relay `andromeda-pulse-engine door` (the door row).'
    sidecar: 'Surfaces under test: MCP stdio sidecar row retired; stdio agents use the door relay.'
    rationale: 'Report Retired bullet: the stdio sidecar, its double gate and its subprocess tests; Files bullet deletes `sidecar_subprocess.rs` and `incident_events_subprocess.rs`.'
    basis: 'test-plan fct-dxqwgkbv; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-zjdbc44m'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§1 Test Scope Summary → Critical paths'
    change: 'P3 reads: surfaces the console engine door (`run/door/sock`, or the stdio relay `andromeda-pulse-engine door`) + buffer access + JSON-RPC 2.0 contract; agent spawns `andromeda-pulse-engine run`, sends telemetry over OTLP, writes a JSON-RPC 2.0 `tools/call` `query_traces` line to the door and asserts the result shape, one `door.call` record per answered request (`pulse-app/tests/e2e_p3_mcp_subprocess_tools_call.rs`); no feature flag and no `ANDROMEDA_PULSE_MCP_ENABLED`.'
    sidecar: 'Critical paths P3: driven through the console engine door, not the retired feature-gated sidecar.'
    rationale: 'Report Retired bullet: sidecar and double gate; Coverage of new surfaces: the four rewritten door tests carry `door.call` and run at integration tier; New text lists `e2e_p3_mcp_subprocess_tools_call.rs` with a spawned `Engine` and `ask(socket, …)`.'
    basis: 'test-plan fct-ot5xbdcp; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-vdoco5gy; doc:2026-11-03-stdio-sidecar-retired/report fct-sq3jdxab'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§1 Test Scope Summary → Coverage triggers'
    change: 'The security-vector-coverage MCP feature + runtime gating trigger is retired with the gate at chunk 2026-11-03-stdio-sidecar-retired (no `mcp-server` feature, no `ANDROMEDA_PULSE_MCP_ENABLED`, no sidecar); the nine tools answer only through the door, whose negative tests stand: another account refused at three layers and through the relay, the write refused `-32001` with `ANDROMEDA_PULSE_DOOR_RESOLVE` off or rejected, and an undeclared argument member refused `-32602` for each of the nine tools (`declared_only`).'
    sidecar: 'Coverage triggers: MCP double-gate trigger retired; door negative tests carry Vector 4.'
    rationale: 'Report Retired bullet: the double gate (`--features mcp-server` + `ANDROMEDA_PULSE_MCP_ENABLED`); Symbols bullet: `declared_only` is now the one refusal of an undeclared argument member; Outcome: `-32602` for all nine tools met.'
    basis: 'test-plan fct-ft7bwy6m; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc; doc:2026-11-03-stdio-sidecar-retired/report fct-lmgi5jdu'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§4 Unit Test Strategy → What unit tests cover'
    change: 'mcp-server crate: JSON-RPC 2.0 frame parsing; tool call routing (`call_tool`); schema validation — the feature-gate build verification is dropped (the `feature_gate` module and the feature removed at chunk 2026-11-03-stdio-sidecar-retired).'
    sidecar: '§4 mcp-server crate unit scope: feature-gate verification dropped.'
    rationale: 'Report Symbols bullet: `mcp_server::feature_gate` removed; Schema bullet: cargo feature `mcp-server` removed.'
    basis: 'test-plan fct-7n6noty7; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc; doc:2026-11-03-stdio-sidecar-retired/report fct-xsrenprx'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§5 Integration Test Strategy → Driver(s)'
    change: 'Module ↔ engine door: a `tokio::net::UnixStream` client writing JSON-RPC 2.0 lines to `run/door/sock`, on an engine booted in process by `engine_boot::start` or a spawned `andromeda-pulse-engine run`; the Module ↔ MCP sidecar subprocess driver left with the sidecar.'
    sidecar: '§5 drivers: MCP sidecar subprocess driver replaced by the door client.'
    rationale: 'Report Retired bullet: the sidecar and its subprocess tests; New text shows the rewritten tests booting `engine_boot::start` in process and asking through `tokio::net::UnixStream::connect(socket)`.'
    basis: 'test-plan fct-o6sm757u; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-u6kaem4n; doc:2026-11-03-stdio-sidecar-retired/report fct-tys4u62d'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§6 E2E Test Strategy → Drivers per surface'
    change: 'The MCP stdio sidecar driver row reads as the console engine door: a `tokio::net::UnixStream` client with JSON-RPC 2.0 lines (and the spawned relay `andromeda-pulse-engine door`), on an engine in process or spawned; no feature flag, no env var.'
    sidecar: '§6 drivers: MCP stdio sidecar row replaced by the door.'
    rationale: 'Report Retired bullet: sidecar and double gate; Coverage of new surfaces: the rewritten door tests at integration tier.'
    basis: 'test-plan fct-6l7zuqld; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-vdoco5gy'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§6 E2E Test Strategy → Scenario: P3'
    change: 'Surfaces involved: the console engine door (socket or stdio relay), buffer access, JSON-RPC 2.0 contract.'
    sidecar: 'Scenario P3 surfaces: the door in place of the feature-gated sidecar.'
    rationale: 'Report Retired bullet: the stdio sidecar and its double gate.'
    basis: 'test-plan fct-y4dwpurc; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§6 E2E Test Strategy → Scenario: P3'
    change: 'Steps: spawn the built `andromeda-pulse-engine run` on its own data dir and claimed ports; send synthetic spans and logs over OTLP gRPC; write JSON-RPC 2.0 `tools/call` lines (`query_traces`, `query_metrics`, `query_logs`, `generate_snapshot`) to `run/door/sock`; parse each answer line and assert `result` and the tool shape; after the end read one `door.call` per answered request and the SQL-injection fragment and sentinel cursor 0 times in the log family (`pulse-app/tests/e2e_p3_mcp_subprocess_tools_call.rs`).'
    sidecar: 'Scenario P3 steps: a spawned engine asked through its door; no feature build, no sidecar spawn.'
    rationale: 'Report Retired bullet; Deviation 6: the SQL-injection and response-sentinel canaries became one door arm `p3_mcp_tools_call_sql_injection_and_sentinel_cursor_stay_out_of_the_log`; New text lists the spawned `Engine`, `engine_with_telemetry`, the metrics / logs / snapshot asks and the `door.call` records.'
    basis: 'test-plan fct-c5f6426b; doc:2026-11-03-stdio-sidecar-retired/report fct-s734nksv; doc:2026-11-03-stdio-sidecar-retired/report fct-ray6bj2y'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§6 E2E Test Strategy → Scenario: P3'
    change: 'Cleanup: end the spawned engine by SIGTERM and read its log family after the end (spawned-program witness form), in place of the sidecar PID.'
    sidecar: 'Scenario P3 cleanup: the engine, not the sidecar.'
    rationale: 'Report Retired bullet: the sidecar; New text lists `end_by_sigterm` on the spawned `Engine` of the rewritten P3 test.'
    basis: 'test-plan fct-erazkrzq; doc:2026-11-03-stdio-sidecar-retired/report fct-sq3jdxab'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§6 E2E Test Strategy → Scenario: P3'
    change: 'Current residual: the sidecar subprocess files are deleted with the sidecar; `e2e_p3_mcp_incident_tools.rs` asserts through `call_tool` (no `dispatch_tool`) under default features; `e2e_p3_mcp_resolve_content.rs` and `e2e_p3_mcp_incident_events_content.rs` boot the console engine in process with `FakeKeychainBackend` and ask through the door socket inside the default workspace run, on CI too (no feature, no skip); the declined resolve arm asserts no `result`, an error answer, the row still `active` and listed, the door fixed message in place of the not-applied text; the still-owed set as the §1 row `mcp-incident-read-back-cross-process-coverage` states it.'
    sidecar: 'Scenario P3 residual: sidecar legs gone; the incident content tests read through the door under default features.'
    rationale: 'Report Files bullet deletes both subprocess tests; Symbols bullet removes `dispatch_tool`; Harness bullet: the two corpus tests boot the console engine in process with `FakeKeychainBackend` and run on CI; Outcome: `e2e_p3_mcp_incident_tools.rs` under default features, 3166 passed, 0 skipped; Deviation 7.'
    basis: 'test-plan fct-m4xvcm7g; doc:2026-11-03-stdio-sidecar-retired/report fct-zjdbc44m; doc:2026-11-03-stdio-sidecar-retired/report fct-vyk3olls; doc:2026-11-03-stdio-sidecar-retired/report fct-ip2ajk36; doc:2026-11-03-stdio-sidecar-retired/report fct-s3pzoqqs'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§8 Mocking & Stubbing Discipline → Mocking libraries'
    change: 'The Process mocking (MCP sidecar) row reads Process (spawned engine and door relay): spawn the built `andromeda-pulse-engine` (`run`, or the `door` relay) and write JSON-RPC 2.0 lines to the door; the sidecar row retired.'
    sidecar: '§8 process row: the spawned engine and relay replace the sidecar.'
    rationale: 'Report Retired bullet: the stdio sidecar and its subprocess tests.'
    basis: 'test-plan fct-nsjvxbz7; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§3 → Per-chunk gate discipline'
    change: '`cargo xtask smoke:external-resolve` is unchanged in code and now refuses at its precondition (no `andromeda-pulse-mcp` binary) until the route entry «Scenario legs driven against the console engine» (CARRY 2); its resolve through a real `andromeda-pulse-mcp` subprocess and the measured GREEN / mutation readings stand as history up to chunk 2026-11-03-stdio-sidecar-retired.'
    sidecar: 'Per-chunk gate discipline: smoke:external-resolve refuses at its precondition until CARRY 2.'
    rationale: 'Report Harness bullet: the `smoke:external-resolve` verb is untouched and now refuses at its precondition (no `andromeda-pulse-mcp` binary) until «Scenario legs driven against the console engine» (CARRY 2).'
    basis: 'test-plan fct-m53lur54; doc:2026-11-03-stdio-sidecar-retired/report fct-vyk3olls'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§1 Test Scope Summary → Pending coverage triggers'
    change: '`mcp-incident-read-back-cross-process-coverage`: its sidecar statements are history up to chunk 2026-11-03-stdio-sidecar-retired; since it `crates/mcp-server/tests/` is gone, `e2e_p3_mcp_resolve_content.rs` and `e2e_p3_mcp_incident_events_content.rs` boot the console engine in process with `FakeKeychainBackend` and read through the door socket inside the default workspace run, on CI (no feature gate, no credential-store skip), and `e2e_p3_mcp_incident_tools.rs` calls `call_tool` in place of the removed `dispatch_tool`; the declined resolve arm asserts no `result`, an error answer, the row still `active` and listed; the still-owed clause unchanged.'
    sidecar: 'Pending trigger mcp-incident-read-back: sidecar legs retired; the content legs read through the door under default features.'
    rationale: 'Report Files bullet deletes both `crates/mcp-server/tests` files (no `tests/` dir); Harness bullet: the two corpus tests boot the console engine with `FakeKeychainBackend` and run on CI; Symbols bullet removes `dispatch_tool`; Deviation 7; Outcome 3166 passed, 0 skipped.'
    basis: 'test-plan fct-js2thrtn; doc:2026-11-03-stdio-sidecar-retired/report fct-zjdbc44m; doc:2026-11-03-stdio-sidecar-retired/report fct-vyk3olls; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc; doc:2026-11-03-stdio-sidecar-retired/report fct-s3pzoqqs'
  - detector: D-retired
    severity: warning
    section: '§1 Test Scope Summary → Pending coverage triggers'
    change: '`security-vector-coverage: MCP response body redaction` is LANDED 2026-11-03-stdio-sidecar-retired, no longer pending: `pulse-app/tests/security_mcp_response_body_redaction.rs` asks a spawned console engine through its door with a marker riding the `service.name` resource attribute (a trace row carries no other attribute) and asserts the marker 0 times in the engine log family; the `result_type` / `result_count` leaves left with the mcp-server allowlist; row retained for the audit trail.'
    sidecar: 'Pending trigger MCP response body redaction: landed through the door.'
    rationale: 'Report Coverage of new surfaces: the four rewritten door tests, PII the marker 0× in the log; Deviation 4 and Spec claims disproved: the marker rides `service.name`; Schema bullet removes the mcp-server allowlist leaves `result_type`, `result_count`; New text lists the spawned `Engine` and `engine_with_marked_spans` in that file.'
    basis: 'test-plan fct-i5odf4oq; doc:2026-11-03-stdio-sidecar-retired/report fct-vdoco5gy; doc:2026-11-03-stdio-sidecar-retired/report fct-tp7pstg2; doc:2026-11-03-stdio-sidecar-retired/report fct-xsrenprx; doc:2026-11-03-stdio-sidecar-retired/report fct-osknwxpk'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§1 Test Scope Summary → Coverage scope'
    change: 'The workspace-detector row drops the `<data_dir>/run/workspace-key` publish/read round-trip clause (app writes at boot, sidecar reads): since chunk 2026-11-03-stdio-sidecar-retired neither program writes the file; the key derivation stays a library API and the engine stamps incidents with `incident_workspace_key`.'
    sidecar: 'Coverage scope workspace-detector: the run/workspace-key publish clause retired.'
    rationale: 'Report Retired bullet: the `run/workspace-key` publish; Symbols bullet: `publish_workspace_key_for_sidecar` removed, neither program writes the file, `incident_workspace_key` stays, the contract functions stay in the library with no product caller.'
    basis: 'test-plan fct-p2dhprbq; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc'
  - detector: D-retired
    severity: warning
    section: '§1 Test Scope Summary → Pending coverage triggers'
    change: '`workspace-key-cross-process-coverage` is RETIRED 2026-11-03-stdio-sidecar-retired, its subject no longer existing: neither program writes `run/workspace-key` and the sidecar that read it is gone; `publish_workspace_key` / `read_published_workspace_key` stay in `workspace-detector` with no product caller until «Workspace detection retired»; row retained for the audit trail.'
    sidecar: 'Pending trigger workspace-key-cross-process-coverage retired with the publish and the sidecar.'
    rationale: 'Report Retired bullet: the `run/workspace-key` publish and the sidecar; Symbols bullet: the crate leaves at «Workspace detection retired».'
    basis: 'test-plan fct-5c3xpzsh; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§5 Integration Test Strategy → Boundary types covered'
    change: 'The pulse-app boot → `run/workspace-key` → mcp-server `init_incident_context` boundary row is retired since chunk 2026-11-03-stdio-sidecar-retired: no program publishes the key and no sidecar reads it.'
    sidecar: '§5 boundary: cross-process workspace-key handoff row retired.'
    rationale: 'Report Retired bullet: the `run/workspace-key` publish and the sidecar; Outcome (obs): neither program writes `run/workspace-key`.'
    basis: 'test-plan fct-rnb7vczj; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-dv3watly'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§9 CI Integration → Pipeline structure'
    change: 'The MCP-feature tests (`mcp-test` job) row is retired: the job left `ci.yml` at chunk 2026-11-03-stdio-sidecar-retired with the feature it tested, pinned absent by `ci_workflow_holds_no_mcp_test_job_and_no_mcp_server_feature`.'
    sidecar: '§9: mcp-test job row retired.'
    rationale: 'Report Retired bullet: the `mcp-test` CI job; Harness bullet: the job removed and the new pin `ci_workflow_holds_no_mcp_test_job_and_no_mcp_server_feature`.'
    basis: 'test-plan fct-mbichyuo; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-vyk3olls'
  - detector: D-retired
    severity: warning
    section: '§9 CI Integration → Pipeline structure'
    change: 'The Lint + tests row says the same tests run in the `coverage` job (the `mcp-test` job left at chunk 2026-11-03-stdio-sidecar-retired), and its cache cell names the key the `a11y` job restores read-only.'
    sidecar: '§9 lint-test row: mcp-test job references dropped.'
    rationale: 'Report Counts bullet: ci.yml jobs 6 → 5, lint-test · a11y · boot · supply-chain · coverage; Retired bullet: the `mcp-test` CI job.'
    basis: 'test-plan fct-ep45valw; doc:2026-11-03-stdio-sidecar-retired/report fct-wnpvzb4g'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§9 CI Integration → Matrix builds'
    change: 'Each of the five jobs of `ci.yml` (lint-test · a11y · boot · supply-chain · coverage; six until chunk 2026-11-03-stdio-sidecar-retired) declares `runs-on: ubuntu-22.04`; the rest unchanged.'
    sidecar: '§9 matrix builds: job count 6 → 5.'
    rationale: 'Report Counts bullet: ci.yml jobs 6 → 5; `data_dir_export_precedes_every_consumer` pins `("ci.yml", 5)`.'
    basis: 'test-plan fct-p7fl7gmb; doc:2026-11-03-stdio-sidecar-retired/report fct-wnpvzb4g'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§9 CI Integration → Pipeline structure'
    change: 'The Boot smoke (harness) row opens with `cargo build --workspace --release` (no `--features mcp-server` since chunk 2026-11-03-stdio-sidecar-retired) → `Build the exit witness` → …'
    sidecar: '§9 boot job: release build without the mcp-server feature.'
    rationale: 'Report Harness bullet: the boot job release build step renamed and run as `cargo build --workspace --release` (was `… --features mcp-server`).'
    basis: 'test-plan fct-b5tpgm6t; doc:2026-11-03-stdio-sidecar-retired/report fct-vyk3olls'
  - detector: D-retired
    severity: warning
    section: '§9 CI Integration → Pipeline structure'
    change: 'The Release build row names the `boot` job release build as `cargo build --workspace --release` beside the `supply-chain` job auditable build, both pinned by `ci_workflow_keeps_the_linux_release_build_witnesses`.'
    sidecar: '§9 release build row: boot build without the feature.'
    rationale: 'Report Harness bullet: `ci_workflow_keeps_the_linux_release_build_witnesses` reads `cargo build --workspace --release`.'
    basis: 'test-plan fct-gnnl6mzk; doc:2026-11-03-stdio-sidecar-retired/report fct-vyk3olls'
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: '§4 Unit Test Strategy → Conventions'
    change: 'Test file location counts 112 top-level `pulse-app/tests/*.rs` files as of chunk 2026-11-03-stdio-sidecar-retired (`unit_mcp_router.rs` deleted with the `mcp.*` procedures), 113 at its base; the earlier dated readings unchanged.'
    sidecar: '§4 conventions: pulse-app/tests count 113 → 112.'
    rationale: 'Report Counts bullet: `pulse-app/tests/*.rs` 113 → 112; Files bullet deletes `pulse-app/tests/unit_mcp_router.rs`; Retired bullet: the `mcp.*` TauRPC procedures.'
    basis: 'test-plan fct-n45lk2to; doc:2026-11-03-stdio-sidecar-retired/report fct-wnpvzb4g; doc:2026-11-03-stdio-sidecar-retired/report fct-zjdbc44m'
note: 'D-tests-coverage holds (each new path carries its tier: `an_undeclared_argument_member_is_refused_for_each_of_the_nine_tools` unit, the four door tests integ, the new ci.yml pin), though the door row in Surfaces under test (fct-mqmrzd2o) still counts `unit_door.rs` at 19 tests and the report states no new count; D-tests-obs-harness holds (no change to the 5-command harness, status shape or log format); fct-3pi7zfqv names `unit_mcp_router` and `--features mcp-server` as a dated discharge reading and was left as history.'
