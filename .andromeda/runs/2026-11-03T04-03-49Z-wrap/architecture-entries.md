## 2026-11-03-stdio-sidecar-retired — the MCP surface: the door its one carrier, no rmcp
**Section:** §Stack and Technologies (the MCP server row); §Established Decisions ([MCP Server Surface], [API Style — MCP Server Surface 3]); §Conventions (External wire — MCP server); §Inherited Defaults (API style — external MCP)
**Change:**
- MCP server row: the `jsonrpc.rs` layer is served inside the console engine's process over `run/door/sock` and reached over stdio only through the relay `andromeda-pulse-engine door`; no `rmcp`; `pulse-app` names `mcp-server` as a plain dependency linked in every build, no feature gate and no stdio sidecar (was stdio and the door, `rmcp` a feature-gated anchor at `"3"`, 3.1.4, `--features mcp-server` gating the sidecar).
- [MCP Server Surface]: was "over stdio; `rmcp` linked as a feature-gated anchor"; now "served by the console engine's door; no `rmcp`" — no manifest and no `Cargo.lock` entry names `rmcp`; `call_tool` has one product caller, `door::EngineTools`, which writes `door.call` (was two, with `dispatch_tool` and its three `mcp.*` records); no `mcp.*` record has an emitter in product code; one way in besides telemetry, the door (socket and stdio relay) (was two, the door and the sidecar); `mcp-server` is the plain `[dependencies.mcp-server-crate]` of `pulse-app`, no feature (was the feature forwarding `mcp-server-crate/mcp-server`).
- [API Style — MCP Server Surface 3]: the envelope has one carrier, the door (was two transports, the sidecar's stdio and the door); `-32001` · `-32002` · `-32003` stand.
- External wire: JSON-RPC 2.0 per MCP spec, the wire the door, reached over stdio through the relay, one connection per request (was "over stdio per MCP spec", the sidecar's wire).
- Inherited Defaults: no feature gate, no stdio sidecar and no `rmcp` (was `--features mcp-server` gating the sidecar and the `rmcp` anchor).
**Why:** the chunk retired the stdio sidecar `andromeda-pulse-mcp` and its double gate, removed `dispatch_tool` and the records only it wrote, and took `rmcp` and `rmcp-macros` out of the workspace and `Cargo.lock`; the engine's ways in besides telemetry moved 2 → 1.
**Kept:** the closed 1.5.0-vs-0.3.x reconciliation stands as history, now ending "the published line reached 3.x" (was "… and the workspace pins it").
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — the mcp-test CI job retired: ci.yml jobs 6 → 5
**Section:** §Stack and Technologies (the Gate-script pins row); §Infrastructure Patterns → CI/CD approach (ci-cd-approach.md)
**Change:**
- Gate-script pins row: the tests run in ci.yml's `lint / test` and `coverage gate` jobs, green on `ci#41572801661` (was `lint / test`, `mcp-server tests` and `coverage gate`, green on `ci#38079305603`).
- `ci.yml` runs five independent jobs — `lint-test` · `a11y` · `boot` · `supply-chain` · `coverage` — the count pinned by `data_dir_export_precedes_every_consumer`, `("ci.yml", 5)` (was six, with `mcp-test`, nextest with `--features mcp-server`).
- The `boot` job's release build, and its release-build witness pinned by `ci_workflow_keeps_the_linux_release_build_witnesses`, is `cargo build --workspace --release` (was the mcp-feature build, `… --release --features mcp-server`); no step names `--features mcp-server` and no job is `mcp-test`, pinned by `ci_workflow_holds_no_mcp_test_job_and_no_mcp_server_feature`.
- Caches: `a11y` restores `lint-test-{os}` read-only (was `mcp-test` and `a11y`).
**Why:** the chunk removed the `mcp-test` job with the sidecar's feature and runs the boot release build with no feature; on `ci#41572801661` every check succeeded but `supply-chain`, which failed on `cargo audit` alone.
**Kept:** the dated readings of earlier runs (`checks 7/7`, `all six jobs`) stand as history.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — the corpus's seventh writer and its event reader: the door alone
**Section:** §Established Decisions ([Corpus Write Arbitration]); §Occupied Resources (Corpus SQLite reserved tables, `incident_events`)
**Change:**
- [Corpus Write Arbitration]: the seventh of the seven production writers is the `crates/mcp-server/src/tools.rs` incident-resolve write (`dispatch_mark_incident_resolved` → `update_incident_status`), which bypasses `IncidentPersistence`; its one carrier is the console engine's door, over the engine's own `Arc<dyn CorpusWriter>`, refused with `-32001` unless `ANDROMEDA_PULSE_DOOR_RESOLVE` reads on (was the cross-process `andromeda-pulse-mcp` sidecar, the door a second carrier, "no longer cross-process only"); the guard, its existence probe and `Applied` / `DeclinedStale` unchanged.
- `incident_events` readers: one, the `retrieve_incident_events` tool through the door, via `CorpusWriter::load_incident_events` (was two, the sidecar's and the door's).
**Why:** the stdio sidecar retired; `call_tool` has one product caller left, the door.
**Kept:** the dated "Measured live: a real `andromeda-pulse-mcp` subprocess resolved a row …" reading stands as history.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — the window side: no mcp procedures, tray toggle or ready check
**Section:** §Conventions (Workspace API style — internal Rust↔webview; Endpoint naming); §Standard Contracts (the `ready` example); §Occupied Resources (Tauri IPC routes); §Cross-cutting Patterns (Tray icon policy)
**Change:**
- IPC routes: no `mcp.*` procedures — the `mcp-server` crate mounts no TauRPC router, its nine tools served by the console engine's door (was `mcp.status`, `mcp.start`, `mcp.stop`, only when `--features mcp-server`).
- Workspace API style: the per-crate router list drops `mcp-server`.
- Endpoint naming: the dotted-namespace examples drop `mcp.start`.
- Tray icon policy: the menu is open/focus, the ingest summary, generate snapshot and quit — the same menu in every build (was also a toggle MCP server item, only when `--features mcp-server` was built).
- ready: the example's mcp_server_enabled check line is removed.
**Why:** the chunk removed the `mcp.*` TauRPC procedures with their router and pins, the tray MCP toggle, and `ReadyChecks.mcp_server_enabled` with the Settings MCP switch.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — the mcp-server feature and its switch retired
**Section:** §Conventions (Feature flags); §Standard Contracts (the `app_info` example); §Occupied Resources (Environment variables); §Cross-cutting Patterns (Feature-gate hygiene); §Infrastructure Patterns → Project directory structure (project-directory-structure.md)
**Change:**
- Feature flags: Cargo features `otap-ingest`; default features minimal, no `otap-ingest` (was `mcp-server`, `otap-ingest`).
- `app_info` example: `"features": []` (was `["mcp-server"]`).
- `ANDROMEDA_PULSE_MCP_ENABLED`: N/A — read by no program; the MCP surface has no feature gate and no switch of its own, the door's one write switch being `ANDROMEDA_PULSE_DOOR_RESOLVE` (was `true|false`, honoured only under `--features mcp-server`, a warning against a build without it).
- `ANDROMEDA_PULSE_L4_DETERMINISTIC`: a bounded truthy-parse (was "per the `crates/mcp-server/src/feature_gate.rs` pattern").
- Feature-gate hygiene: `--features otap-ingest` (and any future flag) off by default, its flag-specific dependencies the OTAP-specific crates; the workspace crate `mcp-server` a plain dependency linked in every build (was `--features mcp-server` too, with `rmcp`, and the feature forwarding `mcp-server-crate/mcp-server`).
- Project directory structure: the `mcp-server/` comment reads linked by pulse-app in every build and served by the door; no binary, no feature (was the stdio sidecar binary and the rmcp anchor feature-gated).
**Why:** the chunk removed the cargo feature `mcp-server` from `crates/mcp-server` and `pulse-app` (`pulse-app`'s features `default`, `otap-ingest`), `ANDROMEDA_PULSE_MCP_ENABLED` from every reader, the `feature_gate` module and `rmcp`; `app_info.features` is always empty.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — the door's contract with no sidecar beside it
**Section:** §Standard Contracts (MCP server — spec-conformant; Engine door); §Occupied Resources (MCP stdio surface; Engine door surface)
**Change:**
- MCP server — spec-conformant: `tools/call` invokes the nine through the console engine's door, over `run/door/sock` or over stdio through `andromeda-pulse-engine door` (was stdio, the sidecar's carrier, the door beside it); every `tools/list` entry carries `write`, true for `mark_incident_resolved` alone; the manifest's `additionalProperties: false` is enforced by the door's `declared_only`, the one refusal of an undeclared argument member (`-32602`) (was declared and not enforced by the sidecar).
- Engine door, Call: `initialize` is `jsonrpc::initialize_result()` unchanged (was "the sidecar's").
- MCP stdio surface: the door's methods over the stdin/stdout of `andromeda-pulse-engine door`, the one stdio surface (was also the sidecar's `initialize`, `tools/list`, `tools/call`, `notifications/*`).
- Engine door surface: the relay is the one stdio carrier (was the sidecar the other until the route entry `Stdio sidecar retired` lands).
**Why:** the stdio sidecar retired and the door is untouched; `declared_only` is the one undeclared-member refusal, the P4 answers (`inputs#I2`), PROVISIONAL.
**Kept:** `serverInfo.name` reads `andromeda-pulse-mcp`, a true claim.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — product binaries 3 → 2
**Section:** §Occupied Resources (Process / service identity); §Infrastructure Patterns → Deployment model (deployment-model.md)
**Change:**
- The declared-`[[bin]]` `target/{profile}/andromeda-pulse*` path is one, the console program `andromeda-pulse-engine` (was two, with the sidecar).
- The product binaries are two: `pulse-app` (the window app) and `andromeda-pulse-engine` (the console program: `run` · `version` · `door`) (was three, with `andromeda-pulse-mcp`).
- MCP sidecar binary: N/A — no stdio sidecar; a stdio MCP client starts `andromeda-pulse-engine door` (was `andromeda-pulse-mcp`, when the feature was enabled).
- Deployment model, runtime topology: no optional `andromeda-pulse-mcp` sidecar process (was one, under `--features mcp-server` and `ANDROMEDA_PULSE_MCP_ENABLED=true`).
**Why:** the chunk removed the binary `andromeda-pulse-mcp` with its `[[bin]]` and `required-features`.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — the run/workspace-key publish retired; the lock file's writers two
**Section:** §Occupied Resources (Filesystem locations: subpaths under the resolved root; the out-of-data-dir corpus-key lock file)
**Change:**
- The `run/workspace-key` subpath entry is removed (was published at boot by either engine program through `engine_boot::publish_workspace_key_for_sidecar` and read cross-process by the `andromeda-pulse-mcp` sidecar).
- The corpus-key lock file is written by the window app and the console program `andromeda-pulse-engine`, both through `Corpus::open` from the shared `engine_boot::start` (was also the `andromeda-pulse-mcp` sidecar).
**Why:** the chunk removed `publish_workspace_key_for_sidecar` and its call in `start`, so neither program writes `{data dir}/run/workspace-key` or `app.boot.workspace_key`; the sidecar retired.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — smoke:external-resolve refuses at its precondition
**Section:** §Occupied Resources (xtask CLI surfaces: `cargo xtask smoke:external-resolve`)
**Change:**
- `smoke:external-resolve` drives its resolve through an `andromeda-pulse-mcp` subprocess, a binary no workspace target builds, so it refuses at its precondition (no `andromeda-pulse-mcp` binary) and is not a runnable leg, owner the route entry «Scenario legs driven against the console engine», CARRY 2 (was: resolves an incident through a real `andromeda-pulse-mcp` subprocess); its grade, `reconciled_count` ≥ 1 with `declined_count` 0, unchanged.
**Why:** `xtask/src/external_resolve.rs` is untouched while the sidecar binary left; the report holds it refusing at its precondition until «Scenario legs driven against the console engine» (CARRY 2).
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/
