# Codebase Research — 2026-10-10-door-inside-the-engine-s-process

## Scope
- **Depth:** deep · **Reads:** 24 · **Globs/Greps:** 27 · graph queries 8 (rust plane; trail `tree-query-2026-10-10-door-inside-the-engine-s-process.json` in the phase run dir)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read whole by structural extraction (an index of its 12 sections and 25 dated entries, then lines 1-118 and 119-167). Applied: the engine cycle's gate reading (its line 86); the atom rule for `cargo xtask check:engine-log` (line 165: `engine-log: PASS` is the verdict line alone); a count or verdict is read from a captured log's own summary (line 167); `agent-run.sh run` sends no telemetry (line 133); every log reader globs the `agent-latest.jsonl*` family (line 127).
- **Platform issues consulted:** none — no runner-only bullet stands in the scope and no CI-reading entry is planned outside the operator leg. The one CI run Setup read (`ci#38092921384` on `303fa9a8`) was in progress at take-up, not red.
- **External inputs:** `inputs#I1` — the operator's phase directive for this chunk (the relay file, eight items). `inputs#I2` — the invocation directive (fold the relay; at P5 print the plan card and stop for the operator's yes). `inputs#I3` — the operator's four answers at the P4 dialog, each PROVISIONAL.

## Files inspected
- `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs` (full) — the sidecar's entry point. It opens its own in-memory database (lines 51-56), opens the corpus through the OS credential store (63-67), reads the published workspace key (77-79), checks the double gate (108) and serves JSON-RPC 2.0 lines over stdio (160-243): `initialize`, `tools/list`, `tools/call`, `ping`. Its comment at line 72 names Conductor as a spawner of this binary.
- `crates/mcp-server/src/tools.rs` (lines 1-748) — nine tools behind one `dispatch_tool(conn, viz_state, incident_ctx, tool_name, arguments)` (line 163). It takes a buffer connection, a `VizState`, and an optional `IncidentToolContext { corpus: Arc<dyn CorpusWriter>, workspace_root }`. Nothing in it is tied to stdio or to a second process. It emits three records per call itself (`mcp.tools.call.response`, `metric.mcp.tool_call_duration_ms`, `mcp.tools.call.error`, lines 192-216); the error record carries `error_detail = %e`, whose text can repeat a caller's field name.
- `crates/mcp-server/src/{jsonrpc,feature_gate,contract,tracing_setup}.rs` (outlines) — `jsonrpc.rs` holds the envelope (`parse_request`, `success`, `error`, `tools_list_manifest`, protocol `2024-11-05`); `feature_gate.rs` and `tracing_setup.rs` serve the sidecar process alone.
- `crates/mcp-server/Cargo.toml` (full) and `pulse-app/Cargo.toml` (lines 30-90) — the crate is an OPTIONAL dependency of `pulse-app` under the alias `mcp-server-crate`, behind the feature `mcp-server`; `rmcp` enters only under that feature. `crates/ui-bridge/Cargo.toml:25` names the crate unconditionally (`crates/ui-bridge/src/contract.rs:7` maps its error type), so the crate already compiles in every default build.
- `pulse-app/src/engine_boot.rs` (lines 160-330, 356-612, outline of 1253) — `start(config, inputs) -> EngineHandles` (line 464) builds every store the tools need: `buffer_conn` (489), one `Arc<Corpus>` with reader and writer views (502-519), `incident_workspace_key` (a field of `EngineHandles`, line 403). It spawns every engine task with `tokio::spawn`. `EngineConfig` (358-376) has four fields and one hand-built construction outside the module (`pulse-app/tests/integration_engine_boot.rs:149`). `publish_workspace_key_for_sidecar` (250) is called from `start` (755).
- `pulse-app/src/console.rs` (full) — the closed command line `run` | `version`; `run` calls `engine_boot::init_process` then `engine_boot::start` and waits for a signal.
- `pulse-app/src/mcp_router.rs` (lines 1-120), `pulse-app/src/main.rs`, `pulse-app/src/tray.rs`, `pulse-app/src/lib.rs` (line hits) — the window's side of the sidecar: the `mcp.{status,start,stop}` TauRPC procedures, five feature-gated wiring sites in `main.rs`, the tray's "Toggle MCP Server" item (`tray.rs:198-204`, built under the feature; its handler logs "not yet wired", `:245-255`).
- `pulse-app/tests/integration_console_engine.rs` (lines 296-470, 556-676, 800-822) — the socket census reads the kernel's TCP, UDP and unix tables by inode; `the_engine_listens_on_its_two_loopback_receivers_and_on_nothing_else` (line 558) asserts the listener set equals the two receivers. A unix listener is reported as `("unix", path)` with `listening` read from the flags column (`00010000`, lines 399-411).
- `pulse-app/tests/unit_engine_boot_seam.rs` (outline) — source-text pins: each entry point calls the two boot functions once, no entry point wires an engine part, the shared boot wires every part of a listed set.
- `crates/viz/src/query.rs` (outline) — `read_connection` (line 113) returns a `try_clone()`d connection; `query_traces` / `query_metrics` / `query_logs` take a connection, a `VizState` and a window plus a limit (`LIMIT_MAX` 1000). The file holds no query interrupt.
- `crates/triage/src/incident/persistence.rs` (lines 100-135, 505-535) — the persist cycle's reconcile step reads the durable active set and resolves registry rows a writer outside the registry already resolved.
- `xtask/src/external_resolve.rs` (lines 1-40, 355-420) — `cargo xtask smoke:external-resolve` boots the window app on 4317 / 4318 and resolves an incident through a spawned `andromeda-pulse-mcp`.
- `xtask/src/engine_log.rs` (constants) — the engine check reads three tick targets, the exit target and the memory metric; no arm reads targets against a closed set.
- `docs/capability-record.json` (the P-040 entry and every entry naming the sidecar) and `xtask/src/capability_record.rs` (scenario kinds).
- `.github/workflows/ci.yml`, `pulse-app/tests/quality_gate_workflow.rs`, `xtask/src/main.rs`, `xtask/src/staged_gate.rs` (line hits) — the `mcp-test` job, the feature on the `boot` job's release build, the pin that names it, the three `mcp.*` procedure pins.
- `pulse-app/ui/src/**` and `pulse-app/ui/tests-a11y/**` (line hits) — the Settings modal's MCP switch, the report's "Send to agent" button and its hook, the bindings, the a11y helpers.

## Graph impact (rust plane)
- **`dispatch_tool`** — 2 non-test callers' worth of call sites, all in one place: `dispatch_tools_call` at `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs:274`. Test callers: 9 in `crates/mcp-server/src/tools.rs` and 7 in `pulse-app/tests/e2e_p3_mcp_incident_tools.rs`. A change of its signature reaches those three files and no other.
- **`validate_double_gate`** — called from the sidecar's `main` (`andromeda-pulse-mcp.rs:108`) and three times in `pulse-app/src/mcp_router.rs` (lines 110, 147, 221). Removing the gate removes the router's ground.
- **`engine_boot::start`** — three callers: `pulse-app/src/console.rs:84`, `pulse-app/src/main.rs:143`, `pulse-app/tests/integration_engine_boot.rs:148`. A change of `EngineConfig`'s fields reaches the third.
- **Crate edges of `mcp-server`** — inbound from `pulse-app` and `ui-bridge`; outbound to `buffer`, `viz`, `snapshot`, `curation`, `corpus`, `triage`, `interpretation`, `workspace-detector`.
- **`publish_workspace_key_for_sidecar`** — one production caller, `start` (`engine_boot.rs:755`). Its reader `read_published_workspace_key` has one production caller, the sidecar (`andromeda-pulse-mcp.rs:78`).
- **`reconcile_externally_resolved`** — one production caller, `run_incident_persist_cycle` (`crates/triage/src/incident/persistence.rs:216`).
- **`read_connection`** — three production callers, the three constructors in `pulse-app/src/viz_routers.rs` (lines 25, 59, 93). The sidecar does not call it (it owns its whole database).
- **`update_incident_status`** (the guarded write) — production callers: `dispatch_mark_incident_resolved` (`tools.rs:469`), the persist cycle, the incident observer, the persistence adapter, the report resolver and two producer sites. The door's resolve reaches it through the first.
- **A name beginning `door`** — no symbol on the rust plane (`rows: 0` on `symbol`, an existence query; the name is free).
- **ts plane** — not queried. The window's side is listed below from a name sweep; it is queried if the removal stays in this chunk.

## Patterns detected
- **The tools are already separable from the sidecar process** (`crates/mcp-server/src/tools.rs:163`): the dispatcher's three inputs are exactly what `EngineHandles` holds. A door that hands it the engine's own handles answers all nine tools from the stores the running engine holds, with no second database and no second corpus open.
- **Receivers are bound by the boot from validated values, and a refused value leaves that receiver unstarted while the engine goes on** (`engine_boot.rs:142-173`, the `resolve_port` path; pinned by `a_rejected_grpc_port_leaves_that_receiver_unstarted…`, `integration_console_engine.rs:497`).
- **Readiness reads this run's own bound record** (`cargo xtask harness:ready`): a listener is `accepting` only when the log family holds its bind record after the last `app.boot.engine` record. A door endpoint has no such record today.
- **A reader takes its own database connection** (`crates/viz/src/query.rs:113`); the one production precedent for interrupting a timed-out query is `crates/triage/src/baseline/sql.rs:462-482` (`interrupt_handle()` then `interrupt()`). The corpus has one connection behind one mutex (`crates/corpus/src/contract.rs:114`) and no interrupt: a door read of the corpus holds that mutex for the read, and its reads are point reads with a row bound, except `load_active_incidents`, which returns every active row of a workspace.
- **The resolve write through the tools bypasses the in-memory registry** (`tools.rs:451-481` writes the corpus directly). In the engine's process that is still a writer the registry does not see, so the persist cycle's reconcile step keeps its object: it brings the registry to the corpus within one 60 s cycle.
- **The engine's log allowlist has no key the tools' records resolve to**: `pulse-app/src/observability.rs` registers `mcp-server` (a hyphenated key, lines 586-592) and no `mcp` key, no `door` key, and a bare `metric` key (`grep -A1 'by_target.insert(' pulse-app/src/observability.rs | grep -i '"mcp\|"door\|"metric",'` : two hits, `"mcp-server",` and `"metric",`; control: `"metric",` is known to exist). Emitted inside the engine, `mcp.tools.call.*` would resolve to nothing and every field would be redacted; `metric.mcp.tool_call_duration_ms` would keep `value` and lose its labels.
- **A socket address is 108 bytes on Linux** (`libc-0.2.186/src/unix/linux_like/mod.rs:204`, `sun_path: [c_char; 108]`). Candidate lengths for `{data dir}/run/door.sock`, computed: 104 bytes for a default cycle dir on the hosted runner's workspace path, 98 for the dev host's default cycle dir, 82 for the pre-push stage, 65 for the CI cycle step's `$RUNNER_TEMP` dir, 45 for the default data dir. The margin is 3 bytes in the longest case; a longer name or a deeper data dir does not fit.
- **`run/` is mode 755 today** (`stat` over the eight `target/engine-cycle/*/run` directories on this host; umask 0022). Anything placed there is reachable by path from every account unless its own mode or its own directory closes it.
- **Peer credentials need no new dependency**: `tokio` is built with its full feature set (`Cargo.toml:32`; resolved 1.52.1), whose `UnixStream::peer_cred()` (`tokio-1.52.1/src/net/unix/stream.rs:939`) returns a `UCred` with `uid()`; `libc` is already a dependency of `pulse-app`.
- **How the kernel treats the socket file's mode is not read from a local source** (the `unix(7)` page is not installed on this host). A leg that connects as another account measures it; nothing in this chunk may assert it from memory.
- **A second account on this host: unmeasured.** The probe (`sudo -n true`) was declined by the session's permission layer and was not retried. The hosted runner uses `sudo` in every job's apt step (`ci.yml:57`, `:180`, …), so a second account is drivable there.

## What stands on the sidecar today (the directive's item 4; sweep `git grep -il mcp` over tracked code: 75 files · `crates/mcp-server` 9 · other crates 14 · `pulse-app/src` 9 · `pulse-app/tests` 15 · `pulse-app/ui` 20 · `xtask` 3 · `.github` 1 · manifests 3 · the capability record 1)
  The sidecar process itself:
  - the binary `andromeda-pulse-mcp` and its `[[bin]]`, `src/bin/andromeda-pulse-mcp.rs`, `feature_gate.rs`, `tracing_setup.rs`, the crate feature `mcp-server`, the `rmcp` dependency (`Cargo.toml:117`, with its `deny.toml` skip entries at lines 217-220), the runtime variable `ANDROMEDA_PULSE_MCP_ENABLED`;
  - its two crate tests (`crates/mcp-server/tests/{sidecar_subprocess,incident_events_subprocess}.rs`) and four `pulse-app` tests that spawn the binary (`e2e_p3_mcp_incident_events_content`, `e2e_p3_mcp_resolve_content`, `e2e_p3_mcp_subprocess_tools_call`, `security_mcp_response_body_redaction`).
  What the engine does for it:
  - `publish_workspace_key_for_sidecar` and the file `run/workspace-key` (written by both programs, read by the sidecar alone); the reconcile step's wording ("a writer outside this process").
  The window's side:
  - `pulse-app/src/mcp_router.rs` and its test `unit_mcp_router.rs`; five wiring sites in `main.rs`; `lib.rs:25-26`; the tray item (`tray.rs`, test `unit_tray_glyph_menu.rs`); the `mcp.*` pins in `xtask/src/main.rs:1114-1116` and the fixtures in `xtask/src/staged_gate.rs` that use `mcp.` as their missing namespace; the bindings (`pulse-app/ui/src/bindings/index.ts`);
  - `Settings.mcp_server_enabled` and `ReadyChecks.mcp_server_enabled` (`crates/ui-bridge/src/{contract,health}.rs`), the restart-required key in `crates/config-watcher/src/partition.rs:70`, the allowlist field at `observability.rs:539`, the `mcp-server` allowlist key at `:587`;
  - the webview: the Settings modal's MCP switch (`SettingsModalForm.tsx:419-438`), the report's "Send to agent" button and `use-mcp-delivery.ts`, their tests, the a11y spec `p3-settings-modal.spec.ts` and two helpers.
  No code emits an "MCP server status" notification (`grep -rn -i notification pulse-app/src/mcp_router.rs pulse-app/src/tray.rs`: 0 hits; control: `tauri-plugin-notification` is a dependency of `pulse-app`). Design-system and layout-templates name that trigger; it was never built.
  CI and harness:
  - the `mcp-test` job (`ci.yml:144-199`) and the feature on the `boot` job's release build (`ci.yml:327`), pinned at `pulse-app/tests/quality_gate_workflow.rs:147`;
  - `cargo xtask smoke:external-resolve` (`xtask/src/external_resolve.rs`), dev host only, in no workflow; route entry `Scenario legs driven against the console engine` (`working-route.md:39`) owns moving its write to the door.
  The capability record:
  - P-036 and P-039 name `pulse-app/tests/e2e_p3_mcp_incident_tools.rs` as proof (an in-process test of the dispatcher, not of the sidecar process); P-039 also names two webview tests of the "Send to agent" button; P-040 names `SettingsModalForm.test.tsx` with the anchor `mcp`. None of the record's closed `surfaces` words names the sidecar.
  Outside this repository:
  - Conductor: the directive says it reads through the door from its first check (`inputs#I1` item 3, the relay's own reading, not read here). In this repository the sidecar's own comment names Conductor as a caller that spawns it (`andromeda-pulse-mcp.rs:72`).

## Size, measured (the directive's item 6)
  The door alone: 4 new files and 12 modified (the two lists below, as decided at P4): ten Rust files under `pulse-app/` and `crates/mcp-server/`, one manifest, the capability record, the workflow file and its pin test.
  The sidecar's removal: 9 files of the crate's process side and tests deleted or emptied, 5 `pulse-app` tests deleted or rewritten onto the door, and about 45 more files changed across `pulse-app/src`, three other crates, `xtask`, the workflow and its pins, the manifests, 20 webview files and the a11y suite. It brings the webview gates, the a11y job and the procedure-set close into the gate block, none of which the door needs.
  The removal's rewritten tests read through the door, so the door has to answer before they can be written.

## Conventions to follow
- **Engine parts are wired in `engine_boot::start` and nowhere else**; the pins are `pulse-app/tests/unit_engine_boot_seam.rs:93` and `:108`. A new part joins the pinned list.
- **`pulse-app` probes live in `pulse-app/tests/*.rs`** and reach internals through `pub` + `#[doc(hidden)]` (`[lib] test = false`; a test attribute in a lib source reddens the ratchet in `unit_observability_allowlist_sweep.rs`).
- **A new log target gets its own exact allowlist leaf and a guard under `pulse-app/tests/`** asserting exact-resolve, field-set equality in both directions and a no-fallback discriminator (`pulse-app/tests/unit_observability_allowlist_boot_engine.rs` is the nearest form).
- **A spawned-engine arm** follows `integration_console_engine.rs`: the built `andromeda-pulse-engine run` under a cleared environment, its own TempDir data dir, two claimed ports from `test_support::ports::claim_port`, readiness from accepting endpoints, records awaited by target under a bound, the log family read after the process has ended.
- **Telemetry in tests comes from `test_support::otlp`** and enters over the live receiver.
- **A refusal is a record at WARN with a closed category**, never an ERROR: the engine cycle's `error_records` member must read 0 with the door present.

## New files to create
- `pulse-app/src/door.rs` — the door: where its endpoint sits under the data dir, the listener, the owner check on every connection, the bounds on a call and on an answer, the write switch, and its boot, call and refusal records
- `pulse-app/tests/integration_door.rs` — spawned-engine arms: the owner answered with content the test sent over OTLP, the three layers read as another account, a request outside the tool set refused, an answer cut at the size bound, the write refused with the switch off
- `pulse-app/tests/unit_door.rs` — in-process arms on the door's `#[doc(hidden)]` surfaces: the owner predicate, the endpoint's place and its refusals, the request bounds, the time bound
- `pulse-app/tests/unit_observability_allowlist_door.rs` — the door targets' leaf guards

## Files to modify
- `pulse-app/src/engine_boot.rs` — `EngineConfig` carries the write switch and `start` seats the door for the console program from the stores it built
- `pulse-app/src/lib.rs` — the door module is declared
- `pulse-app/src/observability.rs` — exact allowlist leaves for the door's three targets
- `pulse-app/Cargo.toml` — the tools crate becomes an unconditional dependency
- `crates/mcp-server/src/tools.rs` — a form of the dispatcher that writes no record and hands its caller the result's type and count
- `crates/mcp-server/src/jsonrpc.rs` — the tool list marks the one write
- `pulse-app/tests/integration_console_engine.rs` — the socket census names three listeners
- `pulse-app/tests/unit_engine_boot_seam.rs` — the door joins the pinned set of wired parts
- `pulse-app/tests/integration_engine_boot.rs` — the door over the booted engine: its stores, the applied write, no reader, no door in the window program
- `docs/capability-record.json` — P-040's entry states the sentence and names its check
- `.github/workflows/ci.yml` — one step that runs the other-account arm and fails when it skipped
- `pulse-app/tests/quality_gate_workflow.rs` — the pins on that step

## Open questions
- none — the four plan-decision questions research carried were answered at the P4 dialog (`inputs#I3`): a Unix-domain socket under the data dir's `run/`; the directory, the socket's mode and the peer's account, shown by a real second-account leg on the hosted runner; the nine tools with the resolve write on and switchable; the cut, the sidecar's removal in a second chunk. The lists above are the decided write set: the sidecar's entry point and `pulse-app/tests/e2e_p3_mcp_incident_tools.rs` left them, since the dispatcher's existing form keeps its signature and its records.
