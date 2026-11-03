proposals:
  - detector: D-retired
    severity: warning
    section: "§Occupied Resources → Process / service identity"
    change: "The declared-[[bin]] `target/{profile}/andromeda-pulse*` paths are one, the console program `target/{profile}/andromeda-pulse-engine` (drop 'and the sidecar below'); 'The product binaries are three …' reads 'The product binaries are two: `pulse-app` (the window app) and `andromeda-pulse-engine` (the console program: `run` · `version` · `door`)'; the sub-bullet 'MCP sidecar binary (when feature enabled): `andromeda-pulse-mcp`' is removed — the binary, its `[[bin]]` and `required-features` left at chunk 2026-11-03-stdio-sidecar-retired."
    sidecar: "Process identity: the stdio sidecar binary `andromeda-pulse-mcp` retired; product binaries 3 → 2."
    rationale: "The report's Retired bullet names the stdio sidecar `andromeda-pulse-mcp`; Symbols / APIs records the binary removed and the product binaries now two; Counts moved records product binaries 3 → 2."
    basis: "architecture fct-wuyupllp; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-z7lqgxlc, fct-wnpvzb4g"
  - detector: D-retired
    severity: warning
    section: "§Stack and Technologies"
    change: "MCP server row: technology reads 'Hand-rolled serde JSON-RPC 2.0 (`crates/mcp-server/src/jsonrpc.rs`, MCP protocol `2024-11-05`) served inside the console engine's process over the Unix-domain socket `run/door/sock` (the door, `pulse-app/src/door.rs`, Unix only) and reached over stdio only through the relay `andromeda-pulse-engine door`; no `rmcp` (left the workspace and `Cargo.lock` at chunk 2026-11-03-stdio-sidecar-retired)'; role keeps the nine named tools and reads that `pulse-app` names `mcp-server` as a plain dependency linked in every build, with no `--features mcp-server` gate and no stdio sidecar (PROVISIONAL — §Occupied Resources → Network ports, the door)."
    sidecar: "Stack MCP server row: sidecar, its feature gate and the `rmcp` anchor dependency removed; the door and its stdio relay the one carrier."
    rationale: "Retired names the sidecar and its double gate; Dependencies records `rmcp` removed from the workspace and `Cargo.lock`; Schema / config records the `mcp-server` feature removed; Crates / modules records `mcp-server` a plain dependency of `pulse-app`."
    basis: "architecture fct-bka6rc62; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-yabevdww, fct-xsrenprx, fct-yn7uwxjo"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Stack and Technologies"
    change: "Gate-script pins row: 'They run in ci.yml's `lint / test`, `mcp-server tests` and `coverage gate` jobs, green on `ci#38079305603`' reads 'They run in ci.yml's `lint / test` and `coverage gate` jobs (green on `ci#38079305603`, when the `mcp-server tests` job, retired at chunk 2026-11-03-stdio-sidecar-retired, also ran them)'."
    sidecar: "Stack gate-script pins row: the retired `mcp-test` job no longer named as a standing runner of the tests."
    rationale: "Retired names the `mcp-test` CI job; Harness / gate surface records the job removed from ci.yml and ci.yml jobs 6 → 5."
    basis: "architecture fct-rkbubyxp; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-vyk3olls, fct-wnpvzb4g"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Established Decisions"
    change: "[MCP Server Surface] heading reads 'Hand-rolled serde JSON-RPC 2.0, served by the console engine's door; no `rmcp`'; body: the layer at `jsonrpc.rs` with name dispatch `pub fn call_tool` in `tools.rs`, which writes no record and has one product caller, `door::EngineTools` (`pulse-app/src/door.rs`), which writes `door.call`; the nine tools served over `run/door/sock` and over stdio through `andromeda-pulse-engine door` (`pulse_app::door_bridge`); since chunk 2026-11-03-stdio-sidecar-retired the stdio sidecar `andromeda-pulse-mcp`, its double gate, `dispatch_tool` with its three `mcp.*` records and `rmcp` (workspace entry, the crate's optional dependency, `Cargo.lock`) are gone, `mcp-server` is the plain `[dependencies.mcp-server-crate]` of `pulse-app` with no feature, and the engine has one way in besides telemetry, the door (socket and stdio relay) — the 'until the route entry `Stdio sidecar retired` lands … two ways' and 'rmcp still enters the build only under `--features mcp-server`' sentences go; the dated rmcp-anchor measurement and the closed 1.5.0-vs-0.3.x reconciliation stay as history."
    sidecar: "[MCP Server Surface]: the sidecar, `dispatch_tool`, its `mcp.*` records and `rmcp` retired; `call_tool` has the door as its one product caller; one way in besides telemetry."
    rationale: "Retired names the sidecar, its double gate and the `mcp.*` record chain; Symbols / APIs records `dispatch_tool` removed and `call_tool` with one product caller; Dependencies records `rmcp` gone; Counts moved records engine ways in 2 → 1 (`two-ways`, owner-title rows discharged)."
    basis: "architecture fct-rsvfsams; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc, fct-yabevdww, fct-yn7uwxjo, fct-wnpvzb4g"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Established Decisions"
    change: "[API Style — MCP Server Surface 3]: 'that envelope has two transports: stdio (the sidecar, per MCP spec) and the console engine's door' reads 'that envelope has one carrier, the console engine's door (one JSON-RPC 2.0 request per line over `run/door/sock`), reached over stdio through `andromeda-pulse-engine door`, which passes the door's lines unchanged' — the sidecar's stdio transport retired at chunk 2026-11-03-stdio-sidecar-retired; the same envelope functions and the door's codes `-32001` · `-32002` · `-32003` unchanged."
    sidecar: "[API Style — MCP Server Surface 3]: the sidecar's stdio transport retired; the door, by socket or relay, the one carrier."
    rationale: "Retired names the stdio sidecar; Symbols / APIs records the door untouched and the binary removed."
    basis: "architecture fct-syp7tq5s; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Established Decisions"
    change: "[Corpus Write Arbitration]: the seventh writer reads 'the `tools.rs` incident-resolve write, carried by the console engine's door alone — in the engine's own process, over its own `Arc<dyn CorpusWriter>`, refused with `-32001` unless `ANDROMEDA_PULSE_DOOR_RESOLVE` reads on — which bypasses `IncidentPersistence` and traverses this choke point'; the cross-process `andromeda-pulse-mcp` sidecar carrier and 'no longer cross-process only' go (sidecar retired at chunk 2026-11-03-stdio-sidecar-retired); the dated 'Measured live: a real `andromeda-pulse-mcp` subprocess …' reading stays as history."
    sidecar: "[Corpus Write Arbitration]: the seventh writer's cross-process sidecar carrier retired; the door its one carrier."
    rationale: "Retired names the stdio sidecar; Symbols / APIs records `call_tool` with one product caller, `pulse-app/src/door.rs`; Counts moved exception (b) keeps the dated subprocess reading as history."
    basis: "architecture fct-3k7t2g4l; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-z7lqgxlc, fct-wnpvzb4g"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Occupied Resources → Tauri IPC routes (TauRPC procedures)"
    change: "The bullet '`mcp.status`, `mcp.start`, `mcp.stop` — mcp-server crate (only when `--features mcp-server`)' is removed: the three procedures, their router and `McpApi` / `McpApiImpl` left at chunk 2026-11-03-stdio-sidecar-retired, with their three `EXPECTED_PROCEDURES` pins."
    sidecar: "IPC routes: the `mcp.*` TauRPC procedures retired."
    rationale: "Retired names the `mcp.*` TauRPC procedures; Symbols / APIs records them, their router and pins removed and the bindings regenerated with no `mcp` namespace."
    basis: "architecture fct-qiuibtut; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-z7lqgxlc"
  - detector: D-retired
    severity: warning
    section: "§Conventions"
    change: "Endpoint naming: the per-crate router examples read '(e.g., `traces.query`, `snapshot.generate`, `plugins.list`, `workspace.detect`)' — `mcp.start` dropped, a procedure retired at chunk 2026-11-03-stdio-sidecar-retired."
    sidecar: "Conventions endpoint naming: retired `mcp.start` dropped from the examples."
    rationale: "Retired names the `mcp.*` TauRPC procedures; Symbols / APIs records `mcp.start` removed."
    basis: "architecture fct-22txvdxt; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Conventions"
    change: "Workspace API style — internal Rust↔webview: the per-module router list drops `mcp-server` (its `mcp` router and `McpApi` left at chunk 2026-11-03-stdio-sidecar-retired; the crate mounts no TauRPC router)."
    sidecar: "Conventions router list: `mcp-server` no longer a router-bearing crate."
    rationale: "Retired names the `mcp.*` TauRPC procedures; Symbols / APIs records the router and `McpApi`/`McpApiImpl` removed."
    basis: "architecture fct-rfsw56g3; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Conventions"
    change: "External wire — MCP server: JSON-RPC 2.0 per MCP spec with the nine tool methods unchanged; the wire is the console engine's door — line-delimited JSON-RPC 2.0 over `run/door/sock`, methods `initialize` · `tools/list` · `tools/call` · `ping` — reached over stdio through the relay `andromeda-pulse-engine door`, one connection per request; both '\"over stdio per MCP spec\" is / stays the sidecar's wire' sentences go (sidecar retired at chunk 2026-11-03-stdio-sidecar-retired)."
    sidecar: "Conventions external MCP wire: the sidecar's stdio wire retired; the door, by socket or relay, the one wire."
    rationale: "Retired names the stdio sidecar; Symbols / APIs records the door untouched."
    basis: "architecture fct-ob4ts4xm; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Occupied Resources → Environment variables"
    change: "The `ANDROMEDA_PULSE_MCP_ENABLED` entry is removed: no program reads it since chunk 2026-11-03-stdio-sidecar-retired, which retired the double gate (`--features mcp-server` + this variable) with the sidecar."
    sidecar: "Env vars: `ANDROMEDA_PULSE_MCP_ENABLED` retired with the sidecar's double gate."
    rationale: "Retired names the double gate including `ANDROMEDA_PULSE_MCP_ENABLED`; Symbols / APIs records the env var removed from every reader."
    basis: "architecture fct-g7bqq44n; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-z7lqgxlc"
  - detector: D-retired
    severity: warning
    section: "§Occupied Resources → Environment variables"
    change: "`ANDROMEDA_PULSE_L4_DETERMINISTIC`: 'bounded truthy-parse per the `crates/mcp-server/src/feature_gate.rs` pattern' reads 'bounded truthy-parse (`1|true|yes`)' — the `feature_gate` module left with the double gate at chunk 2026-11-03-stdio-sidecar-retired."
    sidecar: "Env vars: L4_DETERMINISTIC no longer names the deleted `feature_gate.rs` as its parse pattern."
    rationale: "Symbols / APIs records `mcp_server::feature_gate` removed; Files records `crates/mcp-server/src/feature_gate.rs` deleted; Retired names the double gate."
    basis: "architecture fct-g7bqq44n; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc, fct-zjdbc44m"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Conventions"
    change: "Feature flags: 'Cargo features are `kebab-case` (`otap-ingest`); default features are minimal (no `otap-ingest`)' — the `mcp-server` feature left `crates/mcp-server` and `pulse-app` at chunk 2026-11-03-stdio-sidecar-retired (`pulse-app`'s features `default`, `otap-ingest`)."
    sidecar: "Conventions feature flags: the `mcp-server` feature retired."
    rationale: "Schema / config records cargo feature `mcp-server` removed and `pulse-app`'s features `default`, `otap-ingest`."
    basis: "architecture fct-hvcuzwfp; doc:2026-11-03-stdio-sidecar-retired/report fct-xsrenprx"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Cross-cutting Patterns"
    change: "Feature-gate hygiene: '`--features otap-ingest` (and any future flags) are off by default; flag-specific dependencies (OTAP-specific crates) enter the build graph only when the feature is enabled, preserving the small-binary baseline'; there is no `mcp-server` feature and no `rmcp` since chunk 2026-11-03-stdio-sidecar-retired, and the workspace crate `mcp-server` is the plain dependency `pulse-app` links in every build."
    sidecar: "Feature-gate hygiene: the `mcp-server` feature and its `rmcp` dependency retired."
    rationale: "Schema / config records the `mcp-server` feature removed; Dependencies records `rmcp` gone from manifests and `Cargo.lock`; Crates / modules records the plain dependency."
    basis: "architecture fct-dm325geu; doc:2026-11-03-stdio-sidecar-retired/report fct-xsrenprx, fct-yabevdww, fct-yn7uwxjo"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Standard Contracts"
    change: "`app_info` example: `\"features\": []` (was `[\"mcp-server\"]`) — `app_info.features` is always empty since chunk 2026-11-03-stdio-sidecar-retired."
    sidecar: "Standard Contracts app_info: the example's features list empty; the `mcp-server` feature retired."
    rationale: "Symbols / APIs records `app_info.features` now always empty (`main.rs`: one unconditional `Vec::new()`); Schema / config records the feature removed."
    basis: "architecture fct-vg6rrokz; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc, fct-xsrenprx"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Infrastructure Patterns → CI/CD approach"
    change: "The `boot` job begins with the release build `cargo build --workspace --release` (not 'the mcp-feature release build'), and the workspace release build's two CI witnesses read `supply-chain`'s `cargo auditable build --workspace --release` and `boot`'s `cargo build --workspace --release`, pinned by `ci_workflow_keeps_the_linux_release_build_witnesses`; no ci.yml step names `--features mcp-server` (pinned by `ci_workflow_holds_no_mcp_test_job_and_no_mcp_server_feature`)."
    sidecar: "CI/CD approach: boot's release build and its witness drop `--features mcp-server`."
    rationale: "Harness / gate surface records the boot job's release build run as `cargo build --workspace --release` and the witness test reading it, plus the new pin; Retired names the double gate."
    basis: "architecture/contracts/ci-cd-approach fct-z32zdd7c; doc:2026-11-03-stdio-sidecar-retired/report fct-vyk3olls; pulse-app/tests/quality_gate_workflow.rs:155-169"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Infrastructure Patterns → CI/CD approach"
    change: "`ci.yml` runs as five independent jobs — `lint-test` · `a11y` · `boot` · `supply-chain` · `coverage`; the '`mcp-test` (nextest with `--features mcp-server`)' item is removed and the cache sentence reads '`a11y` restores `lint-test-{os}` read-only' — the job retired at chunk 2026-11-03-stdio-sidecar-retired; `data_dir_export_precedes_every_consumer` pins `(\"ci.yml\", 5)`; dated readings ('checks 7/7', 'all six jobs') stay as history."
    sidecar: "CI/CD approach: the `mcp-test` job retired; ci.yml jobs 6 → 5."
    rationale: "Retired names the `mcp-test` CI job; Counts moved records ci.yml jobs 6 → 5 with the key file `ci-cd-approach.md` among the standing rows; Harness / gate surface records the job removed and the count pin."
    basis: "architecture/contracts/ci-cd-approach fct-z32zdd7c; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-wnpvzb4g, fct-vyk3olls"
  - detector: D-retired
    severity: warning
    section: "§Standard Contracts"
    change: "`ready` example: the `\"mcp_server_enabled\": false` check line is removed — `ReadyChecks.mcp_server_enabled` and `Settings.mcp_server_enabled` left at chunk 2026-11-03-stdio-sidecar-retired."
    sidecar: "Standard Contracts ready: the `mcp_server_enabled` check retired with the Settings MCP switch."
    rationale: "Retired names the Settings MCP switch; Schema / config records `Settings.mcp_server_enabled` and `ReadyChecks.mcp_server_enabled` removed; Counts moved: ready checks lose `mcp_server_enabled`."
    basis: "architecture fct-vqhoi5hb; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-xsrenprx, fct-wnpvzb4g"
  - detector: D-retired
    severity: warning
    section: "§Standard Contracts"
    change: "MCP server — spec-conformant: standard `tools/call` invokes the nine with JSON-RPC 2.0 framing through the console engine's door (the next entry), over `run/door/sock` or over stdio through `andromeda-pulse-engine door`; every `tools/list` entry carries the boolean `write`, true for `mark_incident_resolved` alone; the manifest's `additionalProperties: false` is enforced by the door's `declared_only`, the one refusal of an undeclared argument member (`-32602`); the clauses 'the sidecar's carrier', 'in the sidecar's own answer too …' and 'declared and not enforced by the sidecar …' go (sidecar retired at chunk 2026-11-03-stdio-sidecar-retired)."
    sidecar: "Standard Contracts MCP server: the sidecar carrier retired; `declared_only` the one undeclared-member refusal."
    rationale: "Retired names the stdio sidecar; Symbols / APIs records `declared_only` now the one refusal of an undeclared argument member (`inputs#I2`) and the door untouched."
    basis: "architecture fct-exoqgaup; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Standard Contracts"
    change: "Engine door, Call: '`initialize` (`jsonrpc::initialize_result()` unchanged — `serverInfo.name` reads `andromeda-pulse-mcp`, protocol `2024-11-05`)' — 'the sidecar's' possessive dropped, the sidecar retired at chunk 2026-11-03-stdio-sidecar-retired; the `serverInfo.name` claim stands."
    sidecar: "Standard Contracts engine door: `initialize_result()` no longer attributed to the retired sidecar."
    rationale: "Retired names the stdio sidecar; Symbols / APIs records `initialize`'s `serverInfo.name = \"andromeda-pulse-mcp\"` unchanged and Counts moved exception (a) keeps it a true claim."
    basis: "architecture fct-qsvopkhj; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc, fct-wnpvzb4g"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Occupied Resources → MCP stdio surface"
    change: "MCP stdio surface: the door's methods (`initialize` · `tools/list` · `tools/call` · `ping`, a request with no `id` answered with nothing) over the stdin/stdout of `andromeda-pulse-engine door`, the relay to `run/door/sock` — Unix only, no feature gate, the one stdio surface; the clause on `initialize`, `tools/list`, `tools/call`, `notifications/*` when the `andromeda-pulse-mcp` sidecar is started goes (retired at chunk 2026-11-03-stdio-sidecar-retired)."
    sidecar: "MCP stdio surface: the sidecar's stdio surface retired; the relay the one stdio surface."
    rationale: "Retired names the stdio sidecar `andromeda-pulse-mcp`."
    basis: "architecture fct-4nu3wh7b; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Occupied Resources → Engine door surface"
    change: "Engine door surface: 'the sidecar stays the other stdio carrier until the route entry `Stdio sidecar retired` lands' reads 'the relay is the one stdio carrier since chunk 2026-11-03-stdio-sidecar-retired'."
    sidecar: "Engine door surface: the owner pointer to «Stdio sidecar retired» discharged."
    rationale: "Retired names the stdio sidecar; Counts moved (c) marks the owner-title rows the owner pointer this chunk discharges, stale."
    basis: "architecture fct-hgj34est; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-wnpvzb4g"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Occupied Resources → Corpus SQLite database / schema names"
    change: "`incident_events` readers: 'Readers, one: the `retrieve_incident_events` tool called through the console engine's door (`call_tool` over the engine's own `Arc<dyn CorpusWriter>`), via `CorpusWriter::load_incident_events(incident_id, limit)` …' — the `andromeda-pulse-mcp` sidecar reader left at chunk 2026-11-03-stdio-sidecar-retired."
    sidecar: "Corpus incident_events: the sidecar reader retired; the door the one reader."
    rationale: "Retired names the stdio sidecar; Symbols / APIs records `call_tool`'s one product caller `pulse-app/src/door.rs`."
    basis: "architecture fct-cgszvbwl; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Occupied Resources → Filesystem locations"
    change: "The `run/workspace-key` subpath entry is removed: neither program writes `{data dir}/run/workspace-key` or the `app.boot.workspace_key` record since chunk 2026-11-03-stdio-sidecar-retired (`engine_boot::publish_workspace_key_for_sidecar` and its call in `start` gone, no sidecar reads it); `incident_workspace_key` stays, and `workspace_detector::contract::{publish_workspace_key, read_published_workspace_key}` stay in the library with no product caller."
    sidecar: "Filesystem locations: the `run/workspace-key` publish retired."
    rationale: "Retired names the `run/workspace-key` publish; Symbols / APIs records `publish_workspace_key_for_sidecar` removed, the file and record unwritten, the library functions kept with no product caller."
    basis: "architecture fct-ulzkjofc; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-z7lqgxlc"
  - detector: D-retired
    severity: warning
    section: "§Occupied Resources → Filesystem locations"
    change: "Corpus-key lock file: 'Written by the window app and the console program `andromeda-pulse-engine`, both through `Corpus::open` from the shared `engine_boot::start`' — 'AND the `andromeda-pulse-mcp` sidecar' goes (retired at chunk 2026-11-03-stdio-sidecar-retired)."
    sidecar: "Filesystem locations lock file: the sidecar no longer named as a writer."
    rationale: "Retired names the stdio sidecar `andromeda-pulse-mcp`; Symbols / APIs records the binary removed."
    basis: "architecture fct-ulzkjofc; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Occupied Resources → xtask CLI surfaces (dev/CI gates)"
    change: "`smoke:external-resolve` (`xtask/src/external_resolve.rs`, untouched) still drives its resolve through an `andromeda-pulse-mcp` subprocess and, since chunk 2026-11-03-stdio-sidecar-retired, refuses at its precondition (no `andromeda-pulse-mcp` binary) until the route entry «Scenario legs driven against the console engine» (CARRY 2); it no longer stands as a runnable leg."
    sidecar: "xtask surfaces: `smoke:external-resolve` refuses at its precondition with the sidecar retired (CARRY 2)."
    rationale: "Harness / gate surface records the verb untouched and now refusing at its precondition until «Scenario legs driven against the console engine» (CARRY 2)."
    basis: "architecture fct-n5rdzzzk; doc:2026-11-03-stdio-sidecar-retired/report fct-vyk3olls"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Cross-cutting Patterns"
    change: "Tray icon policy: the menu's 'toggle MCP server (only when `--features mcp-server` is built and the `mcp-server` crate is loaded)' item is removed — the tray MCP toggle left at chunk 2026-11-03-stdio-sidecar-retired; the menu is the same in every build."
    sidecar: "Tray icon policy: the MCP toggle retired."
    rationale: "Retired names the tray MCP toggle; Symbols / APIs records `MENU_ID_MCP_TOGGLE`, the check item and its handler removed and the menu the same in every build."
    basis: "architecture fct-ddrna4w3; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-z7lqgxlc"
  - detector: D-retired
    severity: warning
    section: "§Inherited Defaults"
    change: "API style — external MCP: JSON-RPC 2.0 by the hand-rolled `crates/mcp-server/src/jsonrpc.rs` layer, linked by `pulse-app` in every build and serving the console engine's door over `run/door/sock`, reached over stdio through `andromeda-pulse-engine door` (PROVISIONAL — §Occupied Resources → Network ports, the door); no `--features mcp-server`, no stdio sidecar and no `rmcp` since chunk 2026-11-03-stdio-sidecar-retired; the closed 1.5.0-vs-0.3.x question kept as history."
    sidecar: "Inherited Defaults external MCP: the sidecar, its feature and the `rmcp` anchor retired."
    rationale: "Retired names the sidecar and its double gate; Dependencies records `rmcp` gone; Schema / config records the feature removed."
    basis: "architecture fct-x56az6wm; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y, fct-yabevdww, fct-xsrenprx"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Infrastructure Patterns → Deployment model"
    change: "Runtime topology: the closing sentence 'Plus one optional `andromeda-pulse-mcp` sidecar process (only when `--features mcp-server` … and `ANDROMEDA_PULSE_MCP_ENABLED=true` …)' is removed — the sidecar and its double gate retired at chunk 2026-11-03-stdio-sidecar-retired."
    sidecar: "Deployment model: the optional sidecar process retired."
    rationale: "Retired names the stdio sidecar and its double gate."
    basis: "architecture/contracts/deployment-model fct-35cgfrz4; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§Infrastructure Patterns → Project directory structure"
    change: "The `mcp-server/` comment reads '# the nine tools, their dispatch and the hand-rolled JSON-RPC 2.0 layer, linked by pulse-app in every build and served by the door; no binary, no feature' — 'the stdio sidecar binary and the rmcp anchor stay feature-gated' goes (both retired at chunk 2026-11-03-stdio-sidecar-retired)."
    sidecar: "Project directory structure: the `mcp-server` comment drops the retired sidecar binary and rmcp anchor."
    rationale: "Files records `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs` deleted; Dependencies records `rmcp` removed; Schema / config records the feature removed."
    basis: "architecture/contracts/project-directory-structure fct-ahat5ytg; doc:2026-11-03-stdio-sidecar-retired/report fct-zjdbc44m, fct-yabevdww, fct-xsrenprx"
    dependent-of: D-retired
note: "D-arch-resources and D-arch-decisions hold: the report adds no IPC method, endpoint, event, socket, port, env var, crate or dependency (Dependencies: rmcp removed, nothing added or bumped); every proposal is a D-retired correction. Design Philosophy fct-cumfgtc2 'MCP over stdio' kept: the stdio relay `andromeda-pulse-engine door` still carries it. Two proposals each touch fct-ulzkjofc and fct-g7bqq44n (distinct retired claims); fct-z32zdd7c takes two (the mcp-test job; the --features mcp-server release build)."
