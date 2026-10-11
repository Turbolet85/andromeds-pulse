# Fan-out results - 2026-10-10-door-inside-the-engine-s-process

Seven doc-agents, one batch, the second window of this wrap. Each return was taken from the agent's final
hand-back by script and entity-probed; the probe read `entities=0` on all seven, so no decode was applied.
Detectors sent: 19, one prompt per doc (architecture 2, security-plan 4, design-system 2, layout-templates 2, test-plan 3, obs-plan 4, a11y-plan 2), equal to the 19 `doc:` names of the drift-base.

Proposals: 67 (architecture 22, security-plan 22, design-system 0, layout-templates 0, test-plan 13, obs-plan 10, a11y-plan 0).
Rejected for a source the report does not carry: 0 - every coordinate a proposal cites was checked by script
against the report: 117 citations, each a line the report's bullets state or a row, an `@` line, a row span or an
added range of its last section; the checker's known-positive control (three planted coordinates) fired.
Dispositions: escalate 44 (seven HALT groups, G1-G7) - apply 23 - reject 0.

## Raised by the orchestrator (no detector proposed them)

- obs-plan §9 Pipeline integration, the `xtask test` row: "three test binaries of the run start the console
  program" - `integration_door` is a fourth (report, Files and Counts). Seen by the obs-plan agent, owned by none
  of its detectors. disposition: apply (after the halt) - check 1: Accurate this-chunk addition (a count) -
  check 5: raised.
- obs-plan §10 CI gates, the heartbeat bullet: the pre-push sixth stage "now starts a program and binds two
  loopback ports" - the engine it starts also opens the door (report, Harness / gate surface). disposition:
  escalate - HALT group G6, with architecture 8, security-plan 12 and test-plan 6 - check 5: raised.
- test-plan §1, the three sidecar pending rows (plan entry 7, its third part): read by the test-plan agent and
  left; every sentence in them is still true of the sidecar, which this chunk did not remove. disposition: no
  change - check 5: entry 7, its third part; what they hold after the removal is the removal entry's (route card).
- test-plan §11 Universal, "NEVER use real time ... without injection": the test-plan agent could not read it
  from the report. Read here: `pulse-app/tests/unit_door.rs` holds no `start_paused`, `time::pause` or
  `time::advance` (grep, 0 hits) and its idle and time-bound arms wait on the real clock (about 30 s and 5 s).
  The master is not wrong; the tests depart from it. disposition: no amendment - a source matter, to the route
  card as found and not owned.

## Check 6 - Spec claims disproved, each DISPOSED

1. the tools' serde structs refuse an undeclared member: security-plan proposal 5 (G7) and architecture proposal
   5; the plan-artifact claim keeps its report entry and takes no edit; the choice of form is route-card item 3.
2. root refused by the kernel, and P-126's "one `door.refused` record per client": a premise of a claimed
   acceptance disproved after the claim - the ledger's channel, `matrix.py refine` with a premise-correction
   note at P7.3, put to the operator at this halt (G8); the measured layers enter security-plan 13 and 15.
3. research, the kernel's treatment of the socket's mode unread: a chunk-artifact claim - its report entry
   stands, no edit; the measurement enters security-plan 13.
4. research, a second account on this host unmeasured: a chunk-artifact claim - its report entry stands, no edit;
   the measurement enters architecture 21 and test-plan 2 and 5.

## architecture

verdict: proposals 22 - nothing stripped

### architecture proposal 1

```yaml
  - detector: D-arch-resources
    severity: escalate
    section: "§Occupied Resources → Network ports (the sentence \"Both engine programs bind these two receivers and nothing else … no third socket listens … holds no socket beyond them … the HTTP receiver alone listens\")"
    change: >-
      Since chunk 2026-10-10-door-inside-the-engine-s-process the console program `andromeda-pulse-engine run` holds three listeners, not two: the two loopback TCP receivers and one Unix-domain stream listener, the door, at the canonical `{data dir}/run/door/sock` (mode 0600, inside `run/door/` mode 0700; no TCP port added, bind address still not configurable); the window app `pulse-app` opens no door, and on a system that is not Unix the socket code is compiled out and no door is seated. `pulse-app/tests/integration_console_engine.rs` now asserts by set equality, in both census arms, that the listeners are exactly those two receivers and the one `unix` listener, and that under a rejected gRPC port variable the HTTP receiver and the door listen. A door that cannot bind (`path_too_long` · `in_use` · `occupied` · `bind_failed`) leaves the engine running with no door.
    sidecar: >-
      2026-10-11 · chunk 2026-10-10-door-inside-the-engine-s-process · §Occupied Resources → Network ports: "two receivers and nothing else / no third socket" retired for the console program — third listener is the door's Unix-domain socket at `run/door/sock`; window unchanged. Boundary widening, halts for the operator's word.
    rationale: >-
      Report Changes → Symbols / APIs ("A listening socket, the console program's alone") and Counts ("A console engine's listeners: two loopback TCP receivers → those two and one `unix` listener at the canonical `run/door/sock` (set equality in `integration_console_engine.rs`, both census arms)"); Deviation 5 renames the two tests because "the old names asserted what is no longer true". Expected amendments item 1 names this section (`architecture.md:173` the section head) and marks it "Boundary widening: HALTS in the ordinary form (inputs#I5 item 3)" — hence escalate.
    basis: "pulse-app/src/door.rs:366-407"
```

disposition: escalate - check 1: Boundary widening (the socket: a third listener of the console engine and its place on disk). HALT group G1 - check 5: entry 1

### architecture proposal 2

```yaml
  - detector: D-arch-resources
    severity: escalate
    section: "§Occupied Resources → Filesystem locations → Subpaths under the resolved root"
    change: >-
      Register `run/door/` (directory, mode 0700, created before anything is bound) holding `sock` (the door's Unix-domain socket, mode 0600): written by the console program `andromeda-pulse-engine run` only (the window app writes neither), the path canonicalized and refused when it leaves the data dir, the socket bound at its canonical path, which must be shorter than 108 bytes (107 binds; longer reads `path_too_long` and no door is seated). The socket file is not removed when the engine ends; the next boot replaces it only when it is a socket and a connect to it is refused (a live listener reads `in_use`, a non-socket `occupied`). Inside the data dir, so the THREE product-written locations outside the data dir are unchanged and the data dir's top entries stay `corpus`, `logs`, `run`.
    sidecar: >-
      2026-10-11 · chunk 2026-10-10-door-inside-the-engine-s-process · §Occupied Resources → Filesystem locations: registered `run/door/` (0700) + `run/door/sock` (0600), console program only, under 108 bytes, left on disk at exit and replaced at next boot. Boundary widening, halts.
    rationale: >-
      Report Changes → Schema / config ("One new place on disk, inside the data dir: `run/door/` (0700) holding `sock` (0600) … The socket file is not removed when the engine ends; the next boot replaces it") and Symbols / APIs (Endpoint outcomes, closed). The Subpaths registry lists `run/andromeda-pulse.pid`, `run/andromeda-pulse.spawn`, `run/workspace-key` and has no `run/door` entry. Expected amendments item 2 ("Same halt").
    basis: "pulse-app/src/door.rs:366-407"
```

disposition: escalate - check 1: Boundary widening (the socket: a third listener of the console engine and its place on disk). HALT group G1 - check 5: entry 2

### architecture proposal 3

```yaml
  - detector: D-arch-resources
    severity: escalate
    section: "§Occupied Resources → Environment variables (reserved at arch level)"
    change: >-
      Add `ANDROMEDA_PULSE_DOOR_RESOLVE` — product-read write switch of the console engine's door (`ENV_DOOR_RESOLVE`, `resolve_door_write`, `pulse-app/src/engine_boot.rs`): unset or `on` is on; `off` is off; any other value, one that is not UTF-8 included, is off and reads `rejected` (fail closed); the value is never logged, only the closed label `resolve_write` (`on` | `off` | `rejected`) on `app.boot.door`. Read by `EngineConfig::from_env` (field `door_resolve: ResolveWrite`), which both programs call, so the window app reads it too and uses nothing of it; only the console program's door uses it — `mark_incident_resolved` through the door answers `-32001` unless the switch is on.
    sidecar: >-
      2026-10-11 · chunk 2026-10-10-door-inside-the-engine-s-process · §Occupied Resources → Environment variables: registered `ANDROMEDA_PULSE_DOOR_RESOLVE` (product-read, closed set on/off, anything else off + `rejected`, never logged; read by both programs, used by the console door alone). Boundary widening, halts.
    rationale: >-
      Report Changes → Symbols / APIs ("A new environment variable, product-read: `ANDROMEDA_PULSE_DOOR_RESOLVE`") and Counts ("Product environment variables: one added"). The variable has no entry in the registry (the sibling `ANDROMEDA_PULSE_RETENTION_SECONDS` locates it). Expected amendments item 2 ("Same halt").
    basis: "pulse-app/src/engine_boot.rs:134-139"
```

disposition: escalate - check 1: two rules, opposite verdicts - `New env var registration` (routine: the input is code-validated and unit-tested) and `Boundary widening` (escalate); the discriminating fact: the variable is the switch of the write that crosses the newly widened boundary. HALT group G5 - check 5: entry 2

### architecture proposal 4

```yaml
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts (new entry: the console engine's door)"
    change: >-
      Add an entry "Engine door — the tools inside the engine's process": JSON-RPC 2.0 over the Unix-domain socket `run/door/sock`, one request per line, one answer line per request that carries an `id` (a request with no `id` is not answered and runs nothing); methods `initialize` (the unchanged `jsonrpc::initialize_result()`: `serverInfo.name` `andromeda-pulse-mcp`, protocol `2024-11-05`) · `tools/list` · `tools/call` · `ping`; `tools/call` runs one of the nine tools through `mcp_server::tools::call_tool` over the engine's own buffer read connection, `Arc<dyn CorpusWriter>` and `incident_workspace_key` (the door opens no database or corpus, reads no `run/workspace-key`, prepares no SQL). Admission: the peer's kernel-read account must equal the engine's effective account — any other, root included, or an unreadable one, is closed with no byte written. `arguments` must be an object whose every member is declared in the tool's `inputSchema.properties`, else `-32602` before any tool runs. Write switch: `mark_incident_resolved` answers `-32001` unless `ANDROMEDA_PULSE_DOOR_RESOLVE` reads on; `tools/list` still names it. Bounds: request line 65 536 B (`-32600`, connection closed) · answer 1 048 576 B (never cut: `-32003`) · call 5 s (`-32002`, the door interrupts its own read connection) · idle 30 s · 8 connections (a ninth closed at accept) · one tool call at a time. Fixed-message errors, never the tool's own text: `-32700` · `-32600` · `-32601` · `-32602` · `-32603` · `-32001` · `-32002` · `-32003`. Records: `app.boot.door`, `door.call`, `door.refused`, closed labels only (obs-plan owns their fields).
    sidecar: >-
      2026-10-11 · chunk 2026-10-10-door-inside-the-engine-s-process · §Standard Contracts: new entry for the console engine's door (line-delimited JSON-RPC 2.0 over `run/door/sock`; four methods; owner-only admission; declared-members-only arguments; write switch; five bounds; eight fixed error codes).
    rationale: >-
      Report Changes → Symbols / APIs (the call surface, the owner check, Bounds, Error answers, undeclared members, the write switch, the `initialize` answer). Expected amendments item 3 names "architecture §Standard Contracts (a new entry, the door's form; `architecture.md:100`)" and notes the undeclared-member refusal and the `write` member are additions not in the plan's form. No Standard Contracts entry describes this surface.
    basis: "pulse-app/src/door.rs:255-317"
```

disposition: escalate - check 1: Boundary widening (the reach: the nine tools served inside the engine outside the sidecar double gate, and the resolve write). HALT group G3 - check 5: entry 3 (the contract Conductor reads)

### architecture proposal 5

```yaml
  - detector: D-arch-resources
    severity: warning
    section: "§Standard Contracts → MCP server — spec-conformant"
    change: >-
      State that since chunk 2026-10-10-door-inside-the-engine-s-process every `tools/list` entry carries a boolean member `write`, true for `mark_incident_resolved` alone (`tools_list_manifest()`; the stdio sidecar's own answer carries it too), and that "`tools/call` invokes them with JSON-RPC 2.0 framing over stdio" describes the sidecar: the same nine tools are also invoked through the console engine's door (§Standard Contracts → Engine door), where an undeclared argument member is refused by the door itself while the sidecar is unchanged and still reads past one.
    sidecar: >-
      2026-10-11 · chunk 2026-10-10-door-inside-the-engine-s-process · §Standard Contracts → MCP server: `tools/list` entries gained boolean `write` (true for `mark_incident_resolved` only); "over stdio" scoped to the sidecar, door named as the second carrier.
    rationale: >-
      Report Changes → `crates/mcp-server` ("`tools_list_manifest()` gives every tool entry a boolean member `write`, true for `mark_incident_resolved` alone … This is the one change the stdio sidecar's own output takes: its `tools/list` answer carries the member too") and Spec claims disproved 1 ("the sidecar is unchanged and still accepts one"). The entry describes `tools/list` output and names stdio as the framing of `tools/call` with no `write` member.
    basis: "crates/mcp-server/src/jsonrpc.rs:396-412"
```

disposition: escalate - check 1: Boundary widening (the reach: the nine tools served inside the engine outside the sidecar double gate, and the resolve write). HALT group G3. Its `write`-member half is routine (Accurate this-chunk addition) and stands whatever the answer

### architecture proposal 6

```yaml
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources (new bullet beside \"MCP stdio surface\": the engine door surface)"
    change: >-
      Add "Engine door surface": `initialize`, `tools/list`, `tools/call`, `ping` as line-delimited JSON-RPC 2.0 over the Unix-domain socket `{data dir}/run/door/sock` of a running `andromeda-pulse-engine run` (console program only, Unix only, in every build — no feature gate); the "MCP stdio surface" bullet stays the sidecar's.
    sidecar: >-
      2026-10-11 · chunk 2026-10-10-door-inside-the-engine-s-process · §Occupied Resources: registered the door's four methods as a surface of its own beside the MCP stdio surface.
    rationale: >-
      Report Changes → Symbols / APIs ("Methods `initialize` · `tools/list` · `tools/call` · `ping`, through the envelope functions of `crates/mcp-server/src/jsonrpc.rs`") and Crates / modules ("`pulse-app` now links `mcp-server` in every build"). §Occupied Resources registers each wire surface's methods (TauRPC procedures, OTLP HTTP routes, MCP stdio surface) and holds none for the door.
    basis: "pulse-app/src/door.rs:319-749"
```

disposition: escalate - check 1: Boundary widening (the socket: a third listener of the console engine and its place on disk). HALT group G1. Not `Registry over-reach`: a wire surface with its own transport and method set (`ping`, no `notifications/*`), of a kind the registry enumerates

### architecture proposal 7

```yaml
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Process / service identity → Console engine binary"
    change: >-
      Where it says `run` "boots the two receivers, the buffer, the detectors and the corpus through the shared `pulse_app::engine_boot`", add that since chunk 2026-10-10-door-inside-the-engine-s-process `engine_boot::start` also seats the door for `Program::Console` only, after the buffer and the corpus exist (`door::open(…)` then `tokio::spawn(door.serve())`); neither entry point changed and the door is started nowhere else.
    sidecar: >-
      2026-10-11 · chunk 2026-10-10-door-inside-the-engine-s-process · §Occupied Resources → Console engine binary: `run`'s boot list gains the door (console only, seated in `engine_boot::start`).
    rationale: >-
      Second occurrence of the retired "two receivers and nothing else" claim: the console binary's entry enumerates what `run` boots without the door. Report Changes → Symbols / APIs ("The door is seated in `engine_boot::start` and nowhere else … for `Program::Console` only, after the buffer and the corpus exist") and Counts (`ENGINE_PARTS` 17 → 18).
    basis: "pulse-app/src/engine_boot.rs:1037-1053"
    dependent-of: D-arch-resources
```

disposition: escalate - check 1: Boundary widening (the socket: a third listener of the console engine and its place on disk). HALT group G1 - a dependent: it applies or falls with its primary

### architecture proposal 8

```yaml
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → xtask CLI surfaces → `cargo xtask pre-push:linux` (the sentence \"the sixth stage, and no other, starts the console engine `andromeda-pulse-engine` and binds loopback 24317 and 24318\")"
    change: >-
      Add that since chunk 2026-10-10-door-inside-the-engine-s-process the console engine the sixth stage starts also opens the door's Unix-domain listener at `run/door/sock` inside the stage data dir (socket path 82 bytes by that chunk's research computation, not re-measured); the same holds for every engine `harness:engine-cycle` starts. No verb, script word, status or verdict shape changed, and `check:engine-log` passes over a log holding one `app.boot.door` record.
    sidecar: >-
      2026-10-11 · chunk 2026-10-10-door-inside-the-engine-s-process · §Occupied Resources → xtask CLI surfaces → pre-push:linux: the stage's engine binds the two cycle ports AND opens the door socket in its data dir; verbs and verdicts unchanged.
    rationale: >-
      Third occurrence of the retired claim (what a started console engine binds). Report Changes → Harness / gate surface: "`harness:ready`, `harness:status`, `check:engine-log` and `harness:engine-cycle` read what they read before; the engine they start now also opens the door, and `check:engine-log` passes over a log holding `app.boot.door`" and "The pre-push check's sixth stage starts a console engine with the door built in (stage data dir, socket path 82 bytes by research's computation, not re-measured)".
    dependent-of: D-arch-resources
```

disposition: escalate - check 1: Boundary widening (it grows a standing PROVISIONAL item: what a harness-started console engine opens). HALT group G6 - a dependent: it applies or falls with its primary

### architecture proposal 9

```yaml
  - detector: D-arch-resources
    severity: warning
    section: "§Cross-cutting Patterns → Test-time telemetry injection"
    change: >-
      Where it says a test of the console program "reads state back from the program's own log records on a spawned run, or from the `EngineHandles` that `engine_boot::start` returns in an in-process arm", add a third read-back path since chunk 2026-10-10-door-inside-the-engine-s-process: the door (`pulse-app/tests/integration_door.rs` sends a log record over real OTLP and reads it back with a `query_logs` call on `run/door/sock`). The door is a product surface, not a test-mode bypass: injection is still OTLP only.
    sidecar: >-
      2026-10-11 · chunk 2026-10-10-door-inside-the-engine-s-process · §Cross-cutting Patterns → Test-time telemetry injection: console read-back paths gain the door (`integration_door.rs`); injection rule unchanged.
    rationale: >-
      The line (`architecture.md:290`, the one `\bdoor\b` hit the report's sweep names) enumerates the console program's read-back paths as two; the report's Deviation 3 and Outcome (P-126: "the owner's `query_logs` returned OTLP content") show a third now in the test tree. Same retired mechanism claim — the console engine exposes nothing but its receivers and its log.
    dependent-of: D-arch-resources
```

disposition: escalate - check 1: Boundary widening (the socket: a third listener of the console engine and its place on disk). HALT group G1 - a dependent: it applies or falls with its primary; routine in itself (a test read-back path)

### architecture proposal 10

```yaml
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Network ports → test-only loopback ports `:20000`–`:31999` (\"taken by six test binaries\")"
    change: >-
      The one port claim `test_support::ports::claim_port` / `PortClaim` is taken by seven test binaries since chunk 2026-10-10-door-inside-the-engine-s-process: `integration_door` joins the port-using binaries (`agent_run_shell`, `engine_cycle_refusals`, `integration_console_engine`, `integration_engine_boot`), beside the `xtask` unit-test binary and the member's own tests.
    sidecar: >-
      2026-10-11 · chunk 2026-10-10-door-inside-the-engine-s-process · §Occupied Resources → Network ports (test-only range): port-claiming test binaries six → seven (`integration_door`).
    rationale: >-
      Report Changes → Counts: "Test binaries that claim ports through `test_support::ports`: `integration_door` joins them." The entry states the count as six and names the binaries without it.
```

disposition: apply (after the halt) - check 1: Accurate this-chunk addition (a count the report moved). The count is the doc's own (six named) plus the report's `integration_door joins them`

### architecture proposal 11

```yaml
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Filesystem locations → Test-made port-claim files outside every data dir (\"the six test binaries of §Network ports\")"
    change: >-
      Read "the seven test binaries of §Network ports" — `integration_door` writes claim files under `target/test-ports/` too since chunk 2026-10-10-door-inside-the-engine-s-process; still never product-written, the THREE product-written exceptions unchanged.
    sidecar: >-
      2026-10-11 · chunk 2026-10-10-door-inside-the-engine-s-process · §Occupied Resources → Filesystem locations (test port-claim files): "six test binaries" → seven.
    rationale: >-
      Duplicate of the count the previous proposal retires; report Changes → Counts ("`integration_door` joins them") and "Product-written locations OUTSIDE the data dir: unchanged at three".
    dependent-of: D-arch-resources
```

disposition: apply (after the halt) - check 1: Accurate this-chunk addition (a count the report moved) - a dependent: it applies or falls with its primary

### architecture proposal 12

```yaml
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [MCP Server Surface]"
    change: >-
      Amend three statements. (1) "every one dispatched through the single `dispatch_tool` emission site in `tools.rs`": since chunk 2026-10-10-door-inside-the-engine-s-process the dispatch is `pub fn call_tool(…) -> Result<ToolAnswer, Error>`, which writes no log record and has two callers — `dispatch_tool` (signature and its three records kept, the sidecar's path) and `door::EngineTools` (the console engine's door, which writes `door.call` instead). (2) The surface is no longer stdio alone: the same nine tools and the same `jsonrpc.rs` envelope functions are served inside the console engine's process over the Unix-domain socket `run/door/sock` (§Standard Contracts → Engine door), where `initialize` still answers `andromeda-pulse-mcp` / `2024-11-05`. (3) "Gated by `--features mcp-server` (default off) so rmcp only enters the build when explicitly enabled": `rmcp` still enters only under the feature, but the `mcp-server` crate itself is no longer an optional dependency of `pulse-app` — it is linked in every build and the feature reads `mcp-server = ["mcp-server-crate/mcp-server"]`. `tools_list_manifest()` marks each tool with a boolean `write`.
    sidecar: >-
      2026-10-11 · chunk 2026-10-10-door-inside-the-engine-s-process · §Established Decisions [MCP Server Surface]: single `dispatch_tool` site → `call_tool` with two callers; stdio-only → stdio sidecar plus the in-process door; `mcp-server` crate linked in every build, `rmcp` still feature-only; `write` member on `tools/list`.
    rationale: >-
      Report Changes → `crates/mcp-server` ("new … `pub fn call_tool` … the dispatch with no log record. `dispatch_tool` keeps its signature and its three records by calling it … `call_tool` has two callers: `dispatch_tool` and `door::EngineTools`"), Crates / modules ("`pulse-app` now links `mcp-server` in every build") and Dependencies ("the dependency `mcp-server-crate` … is no longer `optional` … `rmcp` still enters only under the feature"). Expected amendments item 3 names this decision (`:60`). No dependency was added or bumped (`Cargo.lock` byte-identical), so the drift is the contradicted wording of a locked decision, not a new library.
    basis: "crates/mcp-server/src/tools.rs:168-185"
```

disposition: escalate - check 1: Boundary widening (the reach: the nine tools served inside the engine outside the sidecar double gate, and the resolve write). HALT group G3 - check 5: entry 3

### architecture proposal 13

```yaml
  - detector: D-arch-decisions
    severity: warning
    section: "§Stack and Technologies → MCP server row"
    change: >-
      Technology cell: keep "Hand-rolled serde JSON-RPC 2.0 over stdio" for the sidecar and add the second carrier — the same layer over a Unix-domain socket inside the console engine's process (the door, `pulse-app/src/door.rs`, Unix only). Role cell: the nine tools are "gated by `--features mcp-server`" only as the sidecar and the `rmcp` anchor; since chunk 2026-10-10-door-inside-the-engine-s-process `pulse-app` links the `mcp-server` crate in every build and the console engine serves the nine through the door with no feature.
    sidecar: >-
      2026-10-11 · chunk 2026-10-10-door-inside-the-engine-s-process · §Stack → MCP server row: stdio sidecar plus the in-process door; feature gate scoped to the sidecar and `rmcp`.
    rationale: >-
      Restates the [MCP Server Surface] claims the primary retires ("over stdio", "gated by `--features mcp-server`"). Report Dependencies ("`mcp-server-crate` … is no longer `optional`") and Symbols / APIs (the call surface).
    basis: "pulse-app/Cargo.toml:87"
    dependent-of: D-arch-decisions
```

disposition: escalate - check 1: Boundary widening (the reach: the nine tools served inside the engine outside the sidecar double gate, and the resolve write). HALT group G3 - a dependent: it applies or falls with its primary

### architecture proposal 14

```yaml
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [API Style — MCP Server Surface 3]"
    change: >-
      Add that the hand-rolled `jsonrpc.rs` envelope is carried by two transports since chunk 2026-10-10-door-inside-the-engine-s-process — stdio (the sidecar, MCP spec) and the console engine's door (one JSON-RPC 2.0 request per line over `run/door/sock`); the door uses the same envelope functions, never a custom envelope, and adds its own fixed-message error codes `-32001` · `-32002` · `-32003`.
    sidecar: >-
      2026-10-11 · chunk 2026-10-10-door-inside-the-engine-s-process · §Established Decisions [API Style — MCP Server Surface 3]: the JSON-RPC envelope now has two transports (stdio, the door's socket).
    rationale: >-
      Restates "JSON-RPC 2.0 over stdio … the canonical surface" as the one transport. Report Changes → Symbols / APIs ("JSON-RPC 2.0, one request per line … through the envelope functions of `crates/mcp-server/src/jsonrpc.rs`"; Error answers, fixed messages).
    basis: "pulse-app/src/door.rs:567-585"
    dependent-of: D-arch-decisions
```

disposition: escalate - check 1: Boundary widening (the reach: the nine tools served inside the engine outside the sidecar double gate, and the resolve write). HALT group G3 - a dependent: it applies or falls with its primary

### architecture proposal 15

```yaml
  - detector: D-arch-decisions
    severity: warning
    section: "§Conventions → External wire — MCP server"
    change: >-
      Add the second wire for the same nine tool methods: the console engine's door, line-delimited JSON-RPC 2.0 over the Unix-domain socket `run/door/sock` (methods `initialize` · `tools/list` · `tools/call` · `ping`), since chunk 2026-10-10-door-inside-the-engine-s-process; "over stdio per MCP spec" stays the sidecar's wire.
    sidecar: >-
      2026-10-11 · chunk 2026-10-10-door-inside-the-engine-s-process · §Conventions → External wire — MCP server: door named as the second wire of the nine tools.
    rationale: >-
      Restates the tools' wire as stdio alone. Report Changes → Symbols / APIs (the call surface: "`tools/call` runs one of the nine tools through `mcp_server::tools::call_tool` over `door::EngineTools`").
    basis: "pulse-app/src/door.rs:255-317"
    dependent-of: D-arch-decisions
```

disposition: escalate - check 1: Boundary widening (the reach: the nine tools served inside the engine outside the sidecar double gate, and the resolve write). HALT group G3 - a dependent: it applies or falls with its primary

### architecture proposal 16

```yaml
  - detector: D-arch-decisions
    severity: warning
    section: "§Inherited Defaults → API style — external MCP"
    change: >-
      Replace "(feature-gated behind `--features mcp-server`)" as a property of the `jsonrpc.rs` layer: since chunk 2026-10-10-door-inside-the-engine-s-process the layer is linked by `pulse-app` in every build and also serves the console engine's door over `run/door/sock`; the feature gates the `rmcp` anchor dependency (still requirement `"3"`, unchanged) and the stdio sidecar.
    sidecar: >-
      2026-10-11 · chunk 2026-10-10-door-inside-the-engine-s-process · §Inherited Defaults → API style — external MCP: `jsonrpc.rs` layer in every build and serving the door; feature gate scoped to `rmcp` and the sidecar.
    rationale: >-
      Restates the feature-gate and stdio-only claims of [MCP Server Surface]. Report Dependencies ("no longer `optional`; the feature reads `mcp-server = [\"mcp-server-crate/mcp-server\"]` … `rmcp` still enters only under the feature").
    basis: "pulse-app/Cargo.toml:87"
    dependent-of: D-arch-decisions
```

disposition: escalate - check 1: Boundary widening (the reach: the nine tools served inside the engine outside the sidecar double gate, and the resolve write). HALT group G3 - a dependent: it applies or falls with its primary

### architecture proposal 17

```yaml
  - detector: D-arch-decisions
    severity: warning
    section: "§Cross-cutting Patterns → Feature-gate hygiene"
    change: >-
      Narrow "flag-specific dependencies (`rmcp`, OTAP-specific crates) only enter the build graph when the feature is enabled": that still holds for `rmcp`, but since chunk 2026-10-10-door-inside-the-engine-s-process the workspace crate `mcp-server` is not flag-specific — `pulse-app` names it as a plain dependency and links it in every build (the console engine's door calls its dispatch), and the `mcp-server` feature only forwards `mcp-server-crate/mcp-server`. `Cargo.lock` and `deny.toml` unchanged.
    sidecar: >-
      2026-10-11 · chunk 2026-10-10-door-inside-the-engine-s-process · §Cross-cutting Patterns → Feature-gate hygiene: `mcp-server` crate left the flag-specific set (linked always); `rmcp` remains feature-only.
    rationale: >-
      The pattern as applied until this chunk had the `mcp-server` crate enter only under the flag (the feature "had named `dep:mcp-server-crate` as well"). Report Dependencies and Crates / modules ("`pulse-app` now links `mcp-server` in every build"). Found-and-not-owned 9 records the same retired statement in a preserve-verbatim home, which is not this detector's to edit.
    basis: "pulse-app/Cargo.toml:87"
    dependent-of: D-arch-decisions
```

disposition: escalate - check 1: Boundary widening (the reach: the nine tools served inside the engine outside the sidecar double gate, and the resolve write). HALT group G3 - a dependent: it applies or falls with its primary; routine in itself (a dependency stopped being optional)

### architecture proposal 18

```yaml
  - detector: D-arch-decisions
    severity: warning
    section: "§Infrastructure Patterns → Project directory structure"
    change: >-
      In the tree, the `crates/mcp-server/` comment "MCP stdio sidecar (feature-gated; hand-rolled JSON-RPC 2.0)" becomes: the tools, their dispatch and the hand-rolled JSON-RPC 2.0 layer, linked by `pulse-app` in every build; the stdio sidecar binary and the `rmcp` anchor stay feature-gated. Under `pulse-app/src/` add `door.rs  # the console engine's door: owner-only Unix socket serving the nine tools` (module `pulse_app::door`).
    sidecar: >-
      2026-10-11 · chunk 2026-10-10-door-inside-the-engine-s-process · §Infrastructure Patterns → Project directory structure: `mcp-server/` comment no longer "feature-gated" as a crate; `pulse-app/src/door.rs` listed.
    rationale: >-
      Keyed contract `project-directory-structure` restates the retired "feature-gated" claim for the crate and lists the engine's seam files without the new module. Report Changes → Files (new `pulse-app/src/door.rs`), Crates / modules ("added module `pulse_app::door` (`pulse-app/src/lib.rs:14`)") and Dependencies.
    basis: "pulse-app/src/lib.rs:14"
    dependent-of: D-arch-decisions
```

disposition: escalate - check 1: Boundary widening (the reach: the nine tools served inside the engine outside the sidecar double gate, and the resolve write). HALT group G3 - a dependent: it applies or falls with its primary; routine in itself (a module listed, a comment corrected)

### architecture proposal 19

```yaml
  - detector: D-arch-decisions
    severity: warning
    section: "§Established Decisions → [Corpus Write Arbitration]"
    change: >-
      Where it names "the seventh — the cross-process `andromeda-pulse-mcp` sidecar at `crates/mcp-server/src/tools.rs` — calls `CorpusWriter` directly", add that since chunk 2026-10-10-door-inside-the-engine-s-process that same `tools.rs` write (`dispatch_mark_incident_resolved` → `update_incident_status`) has a second carrier: the console engine's door, in the engine's own process, over the engine's own `Arc<dyn CorpusWriter>`, refused with `-32001` before the dispatcher unless the write switch `ANDROMEDA_PULSE_DOOR_RESOLVE` reads on. It still traverses the corpus-layer choke point, so the monotonic guard, its existence probe and `Applied` / `DeclinedStale` are unchanged and the placement rationale holds; the writer is no longer cross-process only.
    sidecar: >-
      2026-10-11 · chunk 2026-10-10-door-inside-the-engine-s-process · §Established Decisions [Corpus Write Arbitration]: the `tools.rs` resolve write is reached by the sidecar (cross-process) and now by the console engine's door (in-process, behind the write switch); guard unchanged.
    rationale: >-
      Expected amendments item 3 names this decision (`:78`): "the door's resolve reaches `update_incident_status` through `dispatch_mark_incident_resolved` as the sidecar's does (the arm `the_door_resolves_an_incident_and_says_it_was_a_write`)"; Outcome: "(arch) The door's resolve goes through the guarded status update — met: the arm reads the row `resolved` through the engine's own `CorpusWriter`." The decision describes this writer as the sidecar, cross-process, only.
    basis: "pulse-app/src/door.rs:43-74"
```

disposition: escalate - check 1: Boundary widening (the reach: the nine tools served inside the engine outside the sidecar double gate, and the resolve write). HALT group G3 (the resolve write) - check 5: entry 3

### architecture proposal 20

```yaml
  - detector: D-arch-decisions
    severity: warning
    section: "§Occupied Resources → Corpus SQLite database / schema names → Reserved tables (`incident_events`: \"read by the sidecar. Reader: the `andromeda-pulse-mcp` sidecar's `retrieve_incident_events`\")"
    change: >-
      Name two readers: the `andromeda-pulse-mcp` sidecar's `retrieve_incident_events` and, since chunk 2026-10-10-door-inside-the-engine-s-process, the same tool called through the console engine's door (`call_tool` over the engine's own `Arc<dyn CorpusWriter>`); the read path `CorpusWriter::load_incident_events` and "No DDL change" are unchanged.
    sidecar: >-
      2026-10-11 · chunk 2026-10-10-door-inside-the-engine-s-process · §Occupied Resources → Corpus SQLite reserved tables: `incident_events` reader is the sidecar AND the door's `retrieve_incident_events`.
    rationale: >-
      Another site that names the sidecar as the only carrier of a corpus-backed tool. Report Changes → Symbols / APIs: "`tools/call` runs one of the nine tools through `mcp_server::tools::call_tool` over `door::EngineTools` … the engine's `Arc<dyn CorpusWriter>`"; Schema / config: "no table, no migration".
    basis: "pulse-app/src/door.rs:255-317"
    dependent-of: D-arch-decisions
```

disposition: escalate - check 1: Boundary widening (the reach: the nine tools served inside the engine outside the sidecar double gate, and the resolve write). HALT group G3 - a dependent: it applies or falls with its primary

### architecture proposal 21

```yaml
  - detector: D-arch-decisions
    severity: warning
    section: "§Stack and Technologies → Gate-script pins (test-only) row (or a sibling test-only row for the door's other-account arm)"
    change: >-
      Record the test-only host needs chunk 2026-10-10-door-inside-the-engine-s-process added, with no Cargo or npm dependency and `Cargo.lock` byte-identical: the arm `another_account_is_refused_at_each_of_three_layers` of `pulse-app/tests/integration_door.rs` runs its one client as `python3 -c …` under `sudo -n -u nobody` and under `sudo -n`, so it needs passwordless `sudo` and `python3`; unlike the row's other pins it does NOT fail on a host without them — it prints its skip sentence (the dev host, where `sudo -n true` fails, in every run) — and the ci.yml `lint-test` step `Door refuses another account` is what turns a skip, an empty output or a failed arm into a red job. Three more test binaries in `pulse-app/tests/` (`unit_door`, `unit_observability_allowlist_door`, `integration_door`; 109 → 112 files) and Unix-domain sockets under a path shorter than 108 bytes on every host that runs the workspace tests.
    sidecar: >-
      2026-10-11 · chunk 2026-10-10-door-inside-the-engine-s-process · §Stack → test-only rows: the door's other-account arm needs `sudo -n` + `python3`, skips on a host without a second account, and is enforced by the CI step `Door refuses another account`; no dependency change.
    rationale: >-
      Report Changes → Harness / gate surface ("the arm's one client (`python3 -c …` under `sudo -n -u nobody` and under `sudo -n`) is the only thing run as another account"), Dev-tool versions ("`sudo -n true` was read failing on the dev host"), Spec claims disproved 4 ("the arm prints its skip sentence there in every run") and Counts (`pulse-app/tests/*.rs` 109 → 112). The Stack table registers each test-only host tool (Python 3, `node`, `bash`/`awk`, `cc`) with a "FAILS, never skips" rule; `sudo` is new there and this arm's skip-on-dev-host form departs from that rule. Dependencies: none added, none bumped — no library drift.
    basis: ".github/workflows/ci.yml:114-139"
```

disposition: escalate - check 1: Boundary widening (the workflow step that runs a client as a second account). HALT group G4. It also states a departure from the row's own rule (a test-only host tool FAILS, never skips): named in the halt

### architecture proposal 22

```yaml
  - detector: D-arch-decisions
    severity: warning
    section: "§Infrastructure Patterns → CI/CD approach"
    change: >-
      In the `lint-test` step sequence ("… the xtask gates → `cargo xtask test`, then `perf:slo-load` …") insert, directly after `cargo xtask test`, the step `Door refuses another account` (chunk 2026-10-10-door-inside-the-engine-s-process): a plain `run` step with no `if:`, no `continue-on-error` and no upload, running `cargo nextest run --workspace --profile ci --no-tests=fail --success-output immediate -E 'binary(integration_door) & test(=another_account_is_refused_at_each_of_three_layers)'` with its output kept in `$RUNNER_TEMP/door-other-account.txt`, and exiting non-zero unless nextest exited 0, the file is not empty and it holds `door other-account arm: directory refused, socket mode refused, account check refused`; pinned by two pins and four `bash -e` witnesses in `pulse-app/tests/quality_gate_workflow.rs`. As measured on `ci#38097928056` (pull_request, the pushed tip `2e4ee5ec`, the merge commit `6b553ea4de20`): `verdict: green · checks 7/7`, the step success — `nobody` refused by the kernel at the 0700 directory and at the 0600 socket, root held by neither mode and closed by the door's account check at all three layers.
    sidecar: >-
      2026-10-11 · chunk 2026-10-10-door-inside-the-engine-s-process · §Infrastructure Patterns → CI/CD approach: `lint-test` gains the plain step `Door refuses another account` directly after `cargo xtask test`; green on `ci#38097928056`.
    rationale: >-
      Keyed contract `ci-cd-approach` enumerates `lint-test`'s steps in order with `perf:slo-load` directly after `cargo xtask test`. Report Changes → Counts ("`lint / test` job steps: one added, `Door refuses another account`, directly after `cargo xtask test`"), Harness / gate surface and Cross-project / external claims (`ci#38097928056`; the step's printed readings).
    basis: "pulse-app/tests/quality_gate_workflow.rs:937-1111"
    dependent-of: D-arch-decisions
```

disposition: escalate - check 1: Boundary widening (the workflow step that runs a client as a second account). HALT group G4 - a dependent: it applies or falls with its primary

## security-plan

verdict: proposals 22 - stripped: the agent's own comment lines, kept below as returned

```
# D-security-deps: no drift. Report Dependencies: "none added, none bumped"; Cargo.lock byte-identical to the base;
# `cargo deny check bans licenses sources` and `cargo audit` exit 0 with deny.toml unedited. The `mcp-server-crate`
# optional -> non-optional change (pulse-app/Cargo.toml:87) adds no dependency; security-plan carries no "optional crate" claim.
# All D-security-input / D-security-auth proposals below belong to ONE boundary widening the report classes PROVISIONAL,
# awaiting the founder's word, and says HALTS in the ordinary form (Expected amendments 4, inputs#I5 item 3).
# No security-plan line number is cited beyond the section heads the report states (:18, :115, :121, :391, :431).
```

### security-plan proposal 1

```yaml
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → boundary table (new row: console engine door)"
    change: >-
      Add a row for the console engine's door — the Unix-domain stream socket `{data dir}/run/door/sock`, seated by `andromeda-pulse-engine run` alone (`engine_boot::start`, `Program::Console` only; the window app seats none and writes no door record; compiled out off Unix). What to validate: directory `run/door/` 0700 created before the bind and socket 0600; the path canonicalized, refused when it leaves the data dir, bound at its canonical path, which must be under 108 bytes; closed endpoint outcomes `listening` · `path_too_long` · `in_use` · `occupied` · `bind_failed` (any but `listening` leaves the engine running with no door; a left-over socket is replaced only when it is a socket and a connect to it is refused); the peer-account check on every accepted connection (§Authentication & Authorization); JSON-RPC 2.0, one request per line, closed methods `initialize` · `tools/list` · `tools/call` · `ping`, the closed nine-tool set, `arguments` an object whose every member is declared in the tool's `inputSchema.properties` (`declared_only`, `-32602` before any tool runs), then the tools' serde structs and the shared prepared-statement path; bounds `MAX_REQUEST_BYTES` 65 536 (longer line: `-32600`, connection closed) · `MAX_ANSWER_BYTES` 1 048 576 (never cut: `-32003`) · `CALL_TIMEOUT` 5 s (`-32002`, then an interrupt of the door's own read connection) · `IDLE_TIMEOUT` 30 s · `MAX_CONNECTIONS` 8 · one tool call at a time; `mark_incident_resolved` refused `-32001` before the dispatcher unless the write switch is `On`; every refusal a fixed message carrying no tool error text. How: `std` + `tokio` Unix sockets + `libc::geteuid`, the envelope functions of `crates/mcp-server/src/jsonrpc.rs`, `mcp_server::tools::call_tool` over `door::EngineTools`; the door opens no database and no corpus, reads no `run/workspace-key`, prepares no SQL. Classification: a boundary widening, PROVISIONAL (the founder's word owed). Not shown: a timed-out call's interrupt of a real running query; the socket file outlives the engine and is replaced at the next boot.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: §Input Validation gains the console engine door row (Unix socket `run/door/sock`, 0700/0600, closed methods and tools, declared-members-only, five bounds, fixed error answers); PROVISIONAL widening."
    rationale: >-
      Report Changes → Symbols / APIs adds a new listening socket and JSON-RPC call surface (the listening socket, endpoint outcomes, call surface, bounds, error answers, `declared_only`, write switch); Coverage of new surfaces marks its validation mechanism present. §Input Validation (report-stated head :121) carries no row for it, so the boundary's validation cannot be confirmed against the section. Expected amendments 4 names §Input Validation as carried and classes the widening PROVISIONAL with a halt.
    basis: "pulse-app/src/door.rs:366-407 (bind_endpoint) · door.rs:76-84 (EndpointState) · door.rs:633-646 (declared_only) · door.rs:567-585 (refusal_of) · door.rs:255-317 (EngineTools) · pulse-app/src/engine_boot.rs:1037-1053"
```

disposition: escalate - check 1: Boundary widening (the socket: a third listener of the console engine and its place on disk). HALT group G1 - check 5: entry 4

### security-plan proposal 2

```yaml
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → **Boundary** sentence (the list of dominant boundaries)"
    change: >-
      Name the console engine's door socket among the dominant boundaries, beside the loopback OTLP receivers, the TauRPC bridge, the WASM plugin host, the MCP stdio surface, the console program's command line and env-var/config-file inputs.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: §Input Validation boundary list names the console engine door socket."
    rationale: >-
      The sentence enumerates the system's input boundaries; the report adds one (`run/door/sock`), so the enumeration is stale once the table row lands.
    basis: "pulse-app/src/door.rs:366-407"
    dependent-of: D-security-input
```

disposition: escalate - check 1: Boundary widening (the socket: a third listener of the console engine and its place on disk). HALT group G1 - a dependent: it applies or falls with its primary

### security-plan proposal 3

```yaml
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → CLI / env var inputs row"
    change: >-
      Add `ANDROMEDA_PULSE_DOOR_RESOLVE` to the row's variable list and state its rule: a closed set parsed by `ResolveWrite::from_value` — unset or `on` ⇒ on; `off` ⇒ off; any other value, one that is not UTF-8 included ⇒ off and read `rejected` (fail closed; the boot record `app.boot.door` is then WARN); not a path, never canonicalized, the value never logged (only the closed label `resolve_write` = `on` | `off` | `rejected`). It is read by `EngineConfig::from_env` (field `door_resolve`), which BOTH programs call, so it joins the list of variables the console program reads and the window app reads it too and uses nothing of it; only the console program's door consumes it.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: env-var row registers `ANDROMEDA_PULSE_DOOR_RESOLVE` (closed on/off set, anything else off + `rejected`, never logged; read by both programs, used by the console door alone)."
    rationale: >-
      Report Changes: "A new environment variable, product-read: `ANDROMEDA_PULSE_DOOR_RESOLVE`" and "Product environment variables: one added". The row is security-plan's registry of product-read variables and their validation and does not list it; its statement of what the console program reads is also incomplete without it.
    basis: "pulse-app/src/engine_boot.rs:86-87 (ENV_DOOR_RESOLVE) · engine_boot.rs:134-139 (resolve_door_write) · engine_boot.rs:374 (door_resolve) · pulse-app/src/door.rs:53-61 (from_value)"
```

disposition: escalate - check 1: two rules, opposite verdicts - `New env var registration` (routine: the input is code-validated and unit-tested) and `Boundary widening` (escalate); the discriminating fact: the variable is the switch of the write that crosses the newly widened boundary. HALT group G5 - check 5: entry 4

### security-plan proposal 4

```yaml
  - detector: D-security-input
    severity: escalate
    section: "§Threat Model Summary → Attack surface → Vector: CLI input (Trust boundary)"
    change: >-
      Add that `ANDROMEDA_PULSE_DOOR_RESOLVE` is a closed on/off switch read by both programs through `EngineConfig::from_env` and used by the console program's door alone; a value outside the set fails closed to off (`rejected`) and is never logged (§Input Validation).
    sidecar: "2026-10-10-door-inside-the-engine-s-process: CLI-input vector's trust boundary names the door write switch variable."
    rationale: >-
      The trust-boundary sentence restates, per variable class, the validation the env-var row holds; the report adds a product-read variable (Expected amendments 4: "the product-read variable" under §Threat Model Summary, report-stated head :18).
    basis: "pulse-app/src/engine_boot.rs:134-139"
    dependent-of: D-security-input
```

disposition: escalate - check 1: two rules, opposite verdicts - `New env var registration` (routine: the input is code-validated and unit-tested) and `Boundary widening` (escalate); the discriminating fact: the variable is the switch of the write that crosses the newly widened boundary. HALT group G5 - a dependent: it applies or falls with its primary

### security-plan proposal 5

```yaml
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → MCP stdio inputs row"
    change: >-
      Correct the argument-validation claim: the manifest's `additionalProperties: false` is NOT enforced by the dispatcher — the serde argument structs (`QueryTracesArgs`, `QueryMetricsArgs`, `QueryLogsArgs`, `GenerateSnapshotArgs`, `IncidentIdArgs`) carry no `deny_unknown_fields` and read past an unknown member, and `dispatch_query_incident_list` parses no arguments — so the stdio sidecar still accepts an undeclared member; only the console engine's door refuses one, by its own `declared_only` check (`-32602` before any tool runs). Also state the dispatch as built: `call_tool` (returns `ToolAnswer`, writes no log record) is the name dispatch, `dispatch_tool` keeps its signature and its three records by calling it, and `call_tool` has two callers, `dispatch_tool` and `door::EngineTools`; `tools/list` gives every tool a boolean `write`, true for `mark_incident_resolved` alone, on both carriers.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: MCP stdio row corrected — `additionalProperties: false` is declared but unenforced on the sidecar (no `deny_unknown_fields`); the door's `declared_only` is the only refusal; `call_tool` under `dispatch_tool`, two callers."
    rationale: >-
      Report Changes → Spec claims disproved by measurement 1 measures that the argument structs read past an unknown member and that the manifest's `additionalProperties: false` is unenforced, and hands "whether any master restates 'arguments validated by serde structs' for the MCP surface" to the detectors. The row says `retrieve_incident_events`' input "deserializes through the serde `IncidentIdArgs` with `additionalProperties: false`" and "tool argument structs via `serde`" as the validation — read as a refusal the sidecar does not perform. An input boundary whose stated validation is absent is this detector's drift.
    basis: "pulse-app/src/door.rs:633-646 (declared_only) · crates/mcp-server/src/tools.rs:168-185 (call_tool) · crates/mcp-server/src/tools.rs:159-166 (ToolAnswer) · crates/mcp-server/src/jsonrpc.rs:396-412"
```

disposition: escalate - check 1: escalate-severity detector whose condition is not affirmatively absent (the sidecar reads past an undeclared member), governed by `routine-APPLY-AS-MEASURED` only once an owner is named; the owner is decided at the route card. check 6: Spec claims disproved 1. HALT group G7

### security-plan proposal 6

```yaml
  - detector: D-security-input
    severity: escalate
    section: "§Threat Model Summary → Attack surface (new Vector: console engine door)"
    change: >-
      Add a vector — Entry point: the Unix-domain stream socket `{data dir}/run/door/sock` (0600, in `run/door/` 0700), opened by `andromeda-pulse-engine run` alone; JSON-RPC 2.0 one request per line (`initialize` · `tools/list` · `tools/call` · `ping`) serving the nine tools in the engine's own process over the engine's buffer read connection, its `CorpusWriter` and its `incident_workspace_key`; one write, `mark_incident_resolved`, behind the `ANDROMEDA_PULSE_DOOR_RESOLVE` switch. Trust boundary: own host only (no network listener); the caller must be the engine's own account by the kernel's peer credential, any other account, root included, closed with no byte written; directory and socket modes are outer layers that hold an unprivileged account and do not hold root; requests bounded and refused with fixed messages (§Input Validation). Conductor is the named reader. PROVISIONAL widening.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: Threat Model attack surface gains the console engine door vector (Unix socket, owner only, own host)."
    rationale: >-
      The attack-surface list is the doc's census of entry points; the report adds a listener ("A console engine's listeners: two loopback TCP receivers → those two and one `unix` listener") and measures the trust boundary on ci#38097928056 (root passed the 0700 directory and the 0600 socket and was closed by the door's account check; `nobody` refused by the kernel). Expected amendments 4 names §Threat Model Summary (:18).
    basis: "pulse-app/src/door.rs:366-407 · door.rs:163-167 (admits) · pulse-app/tests/integration_door.rs:683-802"
    dependent-of: D-security-input
```

disposition: escalate - check 1: Boundary widening (the socket: a third listener of the console engine and its place on disk). HALT group G1 - a dependent: it applies or falls with its primary - check 5: entry 4 (the entry point)

### security-plan proposal 7

```yaml
  - detector: D-security-input
    severity: escalate
    section: "§Threat Model Summary → Attack surface → Vector: MCP stdio surface (feature-gated)"
    change: >-
      Scope the "Only present when `--features mcp-server` AND `ANDROMEDA_PULSE_MCP_ENABLED=true`" clause to the stdio sidecar and add that the same nine tools are also reachable through the console engine's door, which is seated for `Program::Console` in `engine_boot::start` and is not under that double gate: `pulse-app` links `mcp-server` in every build (the dependency is no longer `optional`; the feature now reads `mcp-server = ["mcp-server-crate/mcp-server"]` and `rmcp` still enters only under it). The sidecar itself is unchanged except that its `tools/list` answer carries the `write` member.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: MCP stdio vector scoped — the double gate governs the sidecar only; the nine tools are also served by the console engine door; `mcp-server` linked in every `pulse-app` build."
    rationale: >-
      The vector asserts the tool set's presence is conditional on the double gate. Report Changes: `tools/call` through the door runs the nine tools via `call_tool`; "`pulse-app` now links `mcp-server` in every build"; nothing of the sidecar was removed. The report names no feature or `ANDROMEDA_PULSE_MCP_ENABLED` condition on the door (orchestrator: the report is silent rather than explicit on that negative).
    basis: "pulse-app/Cargo.toml:87 · pulse-app/src/engine_boot.rs:1037-1053 · pulse-app/src/door.rs:255-317"
    dependent-of: D-security-input
```

disposition: escalate - check 1: Boundary widening (the reach: the nine tools served inside the engine outside the sidecar double gate, and the resolve write). HALT group G3 - a dependent: it applies or falls with its primary

### security-plan proposal 8

```yaml
  - detector: D-security-input
    severity: escalate
    section: "§Threat Model Summary → Infrastructure → Hosting"
    change: >-
      In the console program's description, the engine boot's listeners are the two loopback receivers AND, on Unix, one Unix-domain listener at the canonical `{data dir}/run/door/sock` (the door); the window app seats no door.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: Hosting — the console program's listeners are the two loopback receivers plus the door's Unix socket."
    rationale: >-
      Hosting describes the console program as "the same engine boot — the two loopback receivers, the buffer, the detectors, the corpus". Report Counts / qualifiers moved: "A console engine's listeners: two loopback TCP receivers → those two and one `unix` listener at the canonical `run/door/sock`" (set equality in both census arms).
    basis: "pulse-app/tests/integration_console_engine.rs:434-446 (door_listener) · pulse-app/src/engine_boot.rs:1037-1053"
    dependent-of: D-security-input
```

disposition: escalate - check 1: Boundary widening (the socket: a third listener of the console engine and its place on disk). HALT group G1 - a dependent: it applies or falls with its primary

### security-plan proposal 9

```yaml
  - detector: D-security-input
    severity: escalate
    section: "§Threat Model Summary → Infrastructure → Networking"
    change: >-
      Replace "MCP over stdio (no socket)" with: the stdio sidecar opens no socket; the console engine also serves the nine tools over a Unix-domain socket inside the data dir (`run/door/sock`) — own host only, no TCP or network listener added; OTLP receivers stay bound `127.0.0.1` only.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: Networking — the tool surface is no longer socket-free: the console engine door is a Unix-domain socket in the data dir."
    rationale: >-
      "MCP over stdio (no socket)" asserts the tool surface has no socket; the report's listening socket retires that for the console engine.
    basis: "pulse-app/src/door.rs:366-407"
    dependent-of: D-security-input
```

disposition: escalate - check 1: Boundary widening (the socket: a third listener of the console engine and its place on disk). HALT group G1 - a dependent: it applies or falls with its primary

### security-plan proposal 10

```yaml
  - detector: D-security-input
    severity: escalate
    section: "§API Security → table row: MCP feature double-gate"
    change: >-
      State that the double gate (`--features mcp-server` AND `ANDROMEDA_PULSE_MCP_ENABLED=true`) governs the stdio sidecar only; the console engine's door serves the same nine tools outside it, its controls being the owner-only socket (0700 / 0600 + peer-account check), the request bounds and the `ANDROMEDA_PULSE_DOOR_RESOLVE` write switch (§Input Validation, §Authentication & Authorization). The graceful-degrade `warn` clause is unchanged.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: double-gate row scoped to the stdio sidecar; the door's own controls named beside it."
    rationale: >-
      The row presents the double gate as the mitigation against accidental enablement of the tool surface. After this chunk the tools are reachable through the door without it (report: the call surface; `pulse-app` links `mcp-server` in every build), so the row's mechanism claim is stale as the tool surface's only gate.
    basis: "pulse-app/Cargo.toml:87 · pulse-app/src/door.rs:43-74 (ResolveWrite)"
    dependent-of: D-security-input
```

disposition: escalate - check 1: Boundary widening (the reach: the nine tools served inside the engine outside the sidecar double gate, and the resolve write). HALT group G3 - a dependent: it applies or falls with its primary

### security-plan proposal 11

```yaml
  - detector: D-security-input
    severity: escalate
    section: "§Security Anti-Patterns → Input (harness classes: a new entry for the second-account CI step)"
    change: >-
      Record the workflow step `Door refuses another account` in `lint-test` (a plain `run` step directly after `cargo xtask test`; no `if:`, no `continue-on-error`, no upload): it runs the one arm `another_account_is_refused_at_each_of_three_layers` of `integration_door` and passes only when nextest exits 0, its kept output is not empty and holds the arm's three-layer sentence. The step's text holds no `sudo`; the arm's one client (`python3 -c …`, run under `sudo -n -u nobody` and under `sudo -n`) is the only thing run as another account, against a test-spawned engine's socket. On a host with no passwordless second account the arm prints its skip sentence (the dev host: `sudo -n true` fails), which fails the CI step. Test-only, not product-consumed. Classification PROVISIONAL — part of the door's boundary widening, the founder's word owed.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: harness record — CI step `Door refuses another account` runs a client as `nobody` and as root (`sudo -n`) against a test engine's door; PROVISIONAL."
    rationale: >-
      Report Changes → Harness / gate surface adds a CI step that runs code as a second account, root included; Expected amendments 4 names §Security Anti-Patterns → Input (:391) for "the workflow step that runs a client as a second account". The section is the doc's register of harness-only classes and their classification and has no entry for it.
    basis: ".github/workflows/ci.yml:114-139 · pulse-app/tests/quality_gate_workflow.rs:937-1111 · pulse-app/tests/integration_door.rs:683-802"
    dependent-of: D-security-input
```

disposition: escalate - check 1: Boundary widening (the workflow step that runs a client as a second account). HALT group G4 - a dependent: it applies or falls with its primary - check 5: entry 4

### security-plan proposal 12

```yaml
  - detector: D-security-input
    severity: escalate
    section: "§Security Anti-Patterns → Input (the console-engine harness and pre-push sixth-stage paragraphs)"
    change: >-
      Add that, since this chunk, every console engine the harness starts (`agent-run.sh boot engine`, `harness:engine-cycle`, the `Console engine cycle` CI step, the sixth stage of `pre-push:linux`) also opens the door: a Unix-domain listener at `run/door/sock` (0700 directory, 0600 socket) inside that cycle's or stage's own data dir, beside the two loopback ports; the harness verbs read what they read before, and `check:engine-log` passes over a log holding one `app.boot.door` record per boot. The socket file is not removed when the engine ends.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: harness paragraphs note that harness-started console engines now also listen on the door socket inside their own data dir."
    rationale: >-
      Report Harness / gate surface: "the engine they start now also opens the door, and `check:engine-log` passes over a log holding `app.boot.door`"; "The pre-push check's sixth stage starts a console engine with the door built in". The section describes those engines as binding the two loopback ports only. Same harness fact is restated in §Threat Model Summary → CLI input (trust boundary) and in the §Input Validation env-var row's pre-push / engine-cycle text — orchestrator may fold those two into this apply; "writes only its per-cycle data dir" stays true (the socket is inside it).
    basis: "pulse-app/src/engine_boot.rs:1037-1053"
    dependent-of: D-security-input
```

disposition: escalate - check 1: Boundary widening (it grows a standing PROVISIONAL item: what a harness-started console engine opens). HALT group G6 - a dependent: it applies or falls with its primary

### security-plan proposal 13

```yaml
  - detector: D-security-auth
    severity: warning
    section: "§Authentication & Authorization"
    change: >-
      Keep "no user accounts, no token issuance, no session, no per-user scope" and add the one caller-identity mechanism that now exists: on every connection the console engine's door accepts, the peer's account is read from the kernel (`tokio::net::UnixStream::peer_cred`) and compared in one function (`door::admits`) with the engine's own effective account (`door::own_account`, `libc::geteuid`); any other account, root included, and a peer whose account cannot be read, is closed with no byte written (fail closed; `door.refused`, reason `other_account`). The 0700 directory and 0600 socket are outer layers: they refuse an unprivileged account at the kernel and do not hold root, which the account check alone refuses (measured on ci#38097928056). No secret, credential, key or auth library is involved; authorization of the one write is the `ANDROMEDA_PULSE_DOOR_RESOLVE` switch. PROVISIONAL widening, the founder's word owed.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: §Authentication records the door's owner check (kernel peer account == engine's effective account; root refused; fail closed) as the first caller-identity mechanism; no credential, no library."
    rationale: >-
      The report touches identity: Symbols / APIs → "The owner check" and Expected amendments 4 ("the owner check as the first caller-identity mechanism", §Authentication & Authorization :115). The section says auth is N/A and that Tauri capability gating substitutes for runtime authorization — no flow there matches a kernel peer-credential check, so the flow in use is undescribed. No off-spec library or secret source: `tokio` and `libc` only, Dependencies none added.
    basis: "pulse-app/src/door.rs:163-167 (admits) · door.rs:359-364 (own_account) · pulse-app/tests/integration_door.rs:683-802"
```

disposition: escalate - check 1: Boundary widening (owner-only: the first caller-identity mechanism, three layers). HALT group G2 - check 5: entry 4

### security-plan proposal 14

```yaml
  - detector: D-security-auth
    severity: warning
    section: "§Threat Model Summary → Auth model"
    change: >-
      Approach stays `none` for user accounts and Provider N/A; Scope adds: the console engine's door admits only the engine's own OS account, by the kernel's peer credential on its Unix socket (§Authentication & Authorization) — beside Tauri capability gating for the webview and the loopback-only OTLP binding.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: Auth model scope names the door's owner check beside capability gating and loopback binding."
    rationale: >-
      Scope states the trust model as Tauri capability gating plus loopback binding only ("N/A"); the report's owner check is a third authorization boundary. The tier justification's "no user authentication surface" and the header's "no user auth" stay true as worded (no user accounts) and are left.
    basis: "pulse-app/src/door.rs:163-167"
    dependent-of: D-security-auth
```

disposition: escalate - check 1: Boundary widening (owner-only: the first caller-identity mechanism, three layers). HALT group G2 - a dependent: it applies or falls with its primary

### security-plan proposal 15

```yaml
  - detector: D-security-auth
    severity: warning
    section: "§Security Anti-Patterns → Authentication"
    change: >-
      Replace "Auth-specific bans omitted — auth approach=none" with: no user-auth bans (no accounts), plus the door's bans — NEVER write a byte to a door peer before its account is read from the kernel and found equal to the engine's own effective account; NEVER admit root or a peer whose account cannot be read; NEVER widen `run/door/` beyond 0700 or `sock` beyond 0600; NEVER treat those modes as the control against root (they do not hold it — the account check does).
    sidecar: "2026-10-10-door-inside-the-engine-s-process: Authentication anti-patterns gain the door's owner-check bans."
    rationale: >-
      The subsection omits all bans on the ground that no auth mechanism exists; the report's owner check and its measured layers (Spec claims disproved 2: root passed both modes and was closed by the door) retire that ground. Each ban restates a behaviour the report gives as built and tested.
    basis: "pulse-app/src/door.rs:163-167 · pulse-app/tests/integration_door.rs:683-802"
    dependent-of: D-security-auth
```

disposition: escalate - check 1: Boundary widening (owner-only: the first caller-identity mechanism, three layers). HALT group G2 - a dependent: it applies or falls with its primary. It mints four NEVER bans from built behaviour: named in the halt

### security-plan proposal 16

```yaml
  - detector: D-security-auth
    severity: warning
    section: "§Error Handling → Internal logging"
    change: >-
      Amend "there is no per-user identity surface to redact": the door reads its peer's OS account id from the kernel; that id is never logged — `door.refused` carries only the closed label `peer` = `owner` | `other`.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: Error Handling notes the door's peer account id as an identity value that is read and never logged."
    rationale: >-
      The sentence asserts no identity surface exists, on "auth approach=none". Report: the peer's account is read on every accepted connection, and "No record carries an argument, an answer, a path, an account id or an error's text".
    basis: "pulse-app/src/door.rs:236-244 (emit_refused) · door.rs:148-152 (Peer)"
    dependent-of: D-security-auth
```

disposition: escalate - check 1: Boundary widening (owner-only: the first caller-identity mechanism, three layers). HALT group G2 - a dependent: it applies or falls with its primary

### security-plan proposal 17

```yaml
  - detector: D-security-logging
    severity: warning
    section: "§Security Anti-Patterns → Logging (new paragraph beside the `app.exit` NO-SCRUB paragraph)"
    change: >-
      Add: `app.boot.door`, `door.call` and `door.refused` are deliberate NO-SCRUB log boundaries, each behind its OWN exact allowlist leaf; no bare `door` key and no bare `app` key exists. `app.boot.door` — `{state, resolve_write}` (`listening` | `path_too_long` | `in_use` | `occupied` | `bind_failed` · `on` | `off` | `rejected`), once per boot of a program that seats the door, INFO when `listening` with the switch not `rejected`, WARN otherwise. `door.call` — `{method, tool, outcome, write, result_type, result_count, answer_bytes, duration_ms}` (`method` and `tool` closed, anything else `other` / `none`; `outcome` one of nine closed labels), once per answered request, written BEFORE the answer line so an applied write always has its record, INFO for `ok`, WARN otherwise. `door.refused` — `{reason, peer}` (`other_account` | `too_many_connections` | `request_too_large` | `idle` · `owner` | `other`), WARN. Nothing client-controlled crosses them: no argument, answer, path (the socket's included), account id or error text; none is written at ERROR. Pinned in `pulse-app/tests/unit_observability_allowlist_door.rs` (exact resolve, field-set equality, `for_target("door")` none) and by the spawned arms (marker 0 times in the log family, no full data-dir path, the three refused-request canaries absent). Stated limits: a tool called through the door also writes the tools' own library records in the console engine's log (`viz.query.traces` · `viz.query.metrics` · `viz.query.logs` · `viz.query`, and for `generate_snapshot` `metric.snapshot.token_count_ms` · `snapshot.token.count.validate`) — which of them a real call wrote was not enumerated from a log; a caller that does not read its answer leaves an `idle` refusal beside the call record; "no such incident" and a real fault both record `tool_error`. No scrub or redaction shape changed.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: Logging anti-patterns record three new NO-SCRUB exact-leaf log boundaries (`app.boot.door`, `door.call`, `door.refused`), their closed fields and levels, and the unenumerated library records a door tool call adds to a console engine's log."
    rationale: >-
      Report Changes → "Three log targets, each an exact allowlist leaf" adds three log boundaries with a stated exclusion set and stated limits (the library-records bullet: "Which of them a real call wrote was not enumerated from a log"). §Security Anti-Patterns → Logging (:431) enumerates each deliberate NO-SCRUB log boundary with its leaf and failure mode (`ui.ipc.rejection`, `ui.webgpu.adapter`, `app.exit`) and does not carry these; Expected amendments 5 marks it carried. The scrub coverage set itself is unchanged ("No scrub or redaction shape changed"), so the restating sites (Data classification sensitivity note; §Data Protection → At rest) need no edit.
    basis: "pulse-app/src/observability.rs:1022-1045 · pulse-app/src/door.rs:182-201 (emit_boot) · door.rs:203-234 (emit_call) · door.rs:236-244 (emit_refused) · pulse-app/tests/integration_door.rs:475-481"
```

disposition: apply (after the halt) - check 1: Accurate this-chunk addition; the plan's Expected amendment 5 names the change itself, not marked a widening - check 5: entry 5

### security-plan proposal 18

```yaml
  - detector: D-security-logging
    severity: warning
    section: "§Security Anti-Patterns → Logging → the `incident_events` NO-SCRUB paragraph"
    change: >-
      Update the actor census: `CorpusWriter::update_incident_status` is reached by the cross-process sidecar's `mark_incident_resolved` AND, since this chunk, by the console engine's door in the engine's own process (through `dispatch_mark_incident_resolved` over the engine's `CorpusWriter`, only when the write switch is `On`); the table's reader is the `retrieve_incident_events` tool, served by the sidecar and by the door through `call_tool` — the door's `door.call` record carries no answer. The two writer PATHS and the closed vocabulary are unchanged.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: `incident_events` paragraph — the status choke point and the one reader tool are now also reached in process through the console engine door."
    rationale: >-
      The paragraph names the sidecar as the status writer's cross-process caller and "the table's one production READER is the sidecar's `retrieve_incident_events`". Report: `tools/call` runs the nine tools through `call_tool` over `door::EngineTools`, and "the door's resolve reaches `update_incident_status` through `dispatch_mark_incident_resolved` as the sidecar's does" (arm `the_door_resolves_an_incident_and_says_it_was_a_write`).
    basis: "pulse-app/src/door.rs:255-317 · pulse-app/tests/integration_engine_boot.rs:592-641"
    dependent-of: D-security-logging
```

disposition: escalate - check 1: Boundary widening (the reach: the nine tools served inside the engine outside the sidecar double gate, and the resolve write). HALT group G3 (the resolve write and the reader reached in process); written with G3's answer though its primary is routine

### security-plan proposal 19

```yaml
  - detector: D-security-logging
    severity: warning
    section: "§Logging & Monitoring → What NEVER to log"
    change: >-
      Add to the list: the door socket's path and the data dir's full path on any door record; a door call's arguments and its answer (a tool response body on a second carrier); the tool's own error text; the peer's account id.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: never-log list covers the door's socket path, call arguments, answers, tool error text and peer account id."
    rationale: >-
      The list restates the never-log posture per surface; the report's door records exclude "an argument, an answer, a path, an account id or an error's text" and the list has no entry for a socket path or an account id.
    basis: "pulse-app/src/observability.rs:1022-1045"
    dependent-of: D-security-logging
```

disposition: apply (after the halt) - check 1: Accurate this-chunk addition; the plan's Expected amendment 5 names the change itself, not marked a widening - check 5: entry 5 - a dependent: it applies or falls with its primary

### security-plan proposal 20

```yaml
  - detector: D-security-logging
    severity: warning
    section: "§Security Anti-Patterns → Logging → bullet \"NEVER log a full product-consumed filesystem path\""
    change: >-
      Add the door's socket path (`{data dir}/run/door/sock`) to the covered paths: no door record carries it or any path — `app.boot.door` reports the endpoint as a closed `state` label only; the spawned arms read no full data-dir path in the log family.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: full-path ban's covered list names the door socket path (closed `state` label only)."
    rationale: >-
      The bullet enumerates each product path the ban covers and how its records comply; the report adds a product-created path and states no record holds it (Expected amendments 5: "no record holds the socket's path").
    basis: "pulse-app/src/door.rs:182-201 · pulse-app/tests/integration_door.rs:399-405"
    dependent-of: D-security-logging
```

disposition: apply (after the halt) - check 1: Accurate this-chunk addition; the plan's Expected amendment 5 names the change itself, not marked a widening - check 5: entry 5 - a dependent: it applies or falls with its primary

### security-plan proposal 21

```yaml
  - detector: D-security-logging
    severity: warning
    section: "§Logging & Monitoring → What to log"
    change: >-
      Add the door's security events: a refused door connection or request (`door.refused` — another account, too many connections, an over-long request, an idle connection), a refused or failed door call (`door.call` at WARN — unknown tool or method, invalid arguments, write switched off, timed out, answer too large), and a door that did not come up or whose write switch value was rejected (`app.boot.door` at WARN).
    sidecar: "2026-10-10-door-inside-the-engine-s-process: What-to-log gains the door's refusal, failed-call and non-listening boot records."
    rationale: >-
      The list is the doc's census of operationally relevant security events that are logged; the report adds three WARN-level record kinds at a new boundary (Outcome: "A refusal of another account, a request outside the tool set and an answer cut at its bound each leave one WARN record").
    basis: "pulse-app/src/door.rs:203-234 · door.rs:236-244"
    dependent-of: D-security-logging
```

disposition: apply (after the halt) - check 1: Accurate this-chunk addition; the plan's Expected amendment 5 names the change itself, not marked a widening - check 5: entry 5 - a dependent: it applies or falls with its primary

### security-plan proposal 22

```yaml
  - detector: D-security-logging
    severity: warning
    section: "§Logging & Monitoring → Snapshot / clipboard / MCP-tool-response hygiene"
    change: >-
      State that tool responses now have a second carrier: the console engine's door returns the same `query_*` / `generate_snapshot` answers, unsanitized OTLP-derived content included, to a caller of the engine's own account over the Unix socket (bounded at 1 048 576 bytes per answer, never cut); the indirect-prompt-injection note applies to an agent reading through the door as it does to a stdio client.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: tool-response hygiene note names the door as the second carrier of unsanitized tool answers."
    rationale: >-
      The paragraph names the external LLM client over MCP stdio as the path by which tool responses leave the app. Report: the owner's `query_logs` through the door "returned OTLP content" (Deviation 3; the operator-leg acceptance on ci#38097928056), and "Conductor reads the engine through this door".
    basis: "pulse-app/tests/integration_door.rs:335-408"
    dependent-of: D-security-logging
```

disposition: escalate - check 1: Boundary widening (the reach: the nine tools served inside the engine outside the sidecar double gate, and the resolve write). HALT group G3 (tool answers have a second carrier); written with G3's answer though its primary is routine

## design-system

verdict: proposals 0 - stripped: the agent's own comment lines, kept below as returned

```
# D-design-tokens: no drift. The report's Coverage bullet lists six new surfaces (the door socket, tools/call through the door, mark_incident_resolved through the door, ANDROMEDA_PULSE_DOOR_RESOLVE, the CI step, the timed-out-call interrupt); the first five read `tokens n/a`, the sixth carries no tokens flag, and none reads `hardcoded✗`. Files bullet: every `pulse-app/ui/**` file unchanged. Outcome: "(design, layouts, a11y) No screen, window, tray item, notification or control; no webview file — met".
# D-design-status-narrative: no drift. `Spec-master edits: none`. The four `Spec claims disproved by measurement` items (undeclared-argument refusal by serde structs; root refused by the kernel; the socket-mode reading; the second account on the dev host) touch no surface whose status design-system states. The one operator ruling recorded (inputs#I4, the door refusing an undeclared field) is "a ruling on this chunk's door, not a standing rule". None reaches §Brand Identity Signature element, §Motion high-impact moments, §Surface: desktop-native component patterns or §Self-Validation.
# Near-miss read and left alone: design-system §Surface: desktop-native → Tray Menu (.andromeda/design-system.md:325) says "Toggle MCP Server" exists "only when `--features mcp-server` is built", and Notifications (:333) names "MCP server status change". The report's Dependencies bullet makes the `mcp-server-crate` dependency non-optional but keeps the `mcp-server` feature (`pulse-app/Cargo.toml:87`), says `rmcp` still enters only under it, and Expected amendments item 7 says "nothing of the sidecar was removed"; the sidecar's removal moved to a removal entry (inputs#I3). The Changes bullets state no change to the tray item, its gate or the notification, so those lines stand this chunk. They are the design-system sites that removal entry will reach.
# The report's Expected-amendments grep counts for design-system (`sidecar` 1, `MCP server status` 1) resolve to :426 (the amendments-sidecar pointer, not the stdio sidecar) and :333; the plan's seven expected amendments name no design-system section.
```

raw twin: `.raw-fanout-design-system.md` (a `proposals: []` return that stripping changed)

## layout-templates

verdict: proposals 0 - stripped: the agent's own comment lines, kept below as returned

```
# D-layout-surface: no drift. The report's Changes add no user-facing UI surface or region. Files bullet: "every `pulse-app/ui/**` file [is] unchanged"; Outcome: "(design, layouts, a11y) No screen, window, tray item, notification or control; no webview file — met"; Coverage of new surfaces marks every new surface (run/door/sock, tools/call through the door, the write switch, ANDROMEDA_PULSE_DOOR_RESOLVE, the CI step) "a11y n/a · tokens n/a". The door is a Unix-domain socket of the console program, not a webview or native-UI region, so there is nothing to map to a §Wireframe entry.
# D-layout-status-narrative: no drift. "Spec-master edits: none." The four "Spec claims disproved by measurement" items concern the door's undeclared-argument refusal, root vs. the socket/directory modes, the kernel's reading of the socket mode, and the dev host's second account — none touches a surface, layer or signature placement whose status layout-templates states. The one operator ruling recorded (inputs#I4, "the door refusing an undeclared field itself is the stricter and right form") is about the door's argument check, not a layout surface.
# Checked and left alone: the doc's MCP-toggle claims (§Primary screens Settings panel, §Component — Settings modal "visible only when the `mcp-server` Cargo feature is built", §Component — Tray menu, §Component — Notifications "MCP server status change ... when feature is built") are not moved by this chunk — the Dependencies bullet says the `mcp-server` feature still exists (`mcp-server = ["mcp-server-crate/mcp-server"]`, pulse-app/Cargo.toml:87) and only the crate's `optional` flag left; the report states nothing of the sidecar was removed and no UI file changed. The doc's single `sidecar` grep hit (the report's Expected-amendments count 1 for layout-templates) is the phrase "amendment sidecar handoff note" in §Wireframe — Full dashboard, unrelated to the stdio sidecar. Its single `MCP server status` hit is the Notifications trigger list, likewise unmoved.
```

raw twin: `.raw-fanout-layout-templates.md` (a `proposals: []` return that stripping changed)

## test-plan

verdict: proposals 13 - stripped: the agent's own comment lines, kept below as returned

```
# D-tests-framework: no drift. Every test command in the report is libtest under `cargo nextest run --workspace --profile ci` (narrowed with `-E` under `--workspace`, never `-p`, in the gate block and in the new CI step); the one `-p pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'` entry is the standard gate set's own bindings regen. Dependencies bullet: none added, none bumped. The new tests are `#[test]` / `#[tokio::test]` fns in `pulse-app/tests/*.rs`, the stand-in is a hand-written `Tools` impl handed in as `Arc<dyn Tools>` (§8 constructor injection).
# D-tests-obs-harness: no drift. Report, Harness / gate surface: "No `xtask` verb, no `scripts/agent-run.*` word, no status or verdict shape changed"; the log format is unchanged (three new targets are allowlist leaves, obs-plan's detectors' to read). Nothing one-sided to reconcile between test-plan §3 and obs-plan §3; the keyed contracts 5-command implementation, Status endpoint shape, Log format and PID file hold no claim the door retires (cleanup still probes the pid and the two resolved OTLP ports only).
# Read and left, not proposed: the three sidecar pending rows of §1 (`:112` on — `workspace-key-cross-process-coverage`, `mcp-incident-read-back-cross-process-coverage`, `security-vector-coverage: MCP response body redaction`) and the §6 P3 scenario's residual. The report states that what they hold once their subject is also read through the door "was not decided at implement: nothing of the sidecar was removed"; every sentence in them is still true of the sidecar.
# Not verifiable from the report (flag for the orchestrator, no proposal): whether the door's idle (30 s) and call-timeout (5 s) arms in `unit_door.rs` run on paused tokio time, against §11 Universal "NEVER use real time … without injection" — the report does not say.
```

### test-plan proposal 1

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Pending coverage triggers (new row `door-timeout-interrupt-real-query-coverage`)"
    change: >-
      Add a row: **NEW 2026-10-10-door-inside-the-engine-s-process.** A `tools/call` through the console engine's door that passes `CALL_TIMEOUT` (5 s) answers `-32002` and the door then interrupts its own read connection through a DuckDB interrupt handle (`EngineTools::interrupt`). The time bound, the answer and the next call being answered are pinned in process over a stand-in whose interrupt is only counted (`pulse-app/tests/unit_door.rs`, `a_call_over_its_time_bound_answers_timed_out_and_the_next_call_is_answered`); that the interrupt ends a REAL running query is unshown at any tier — no query of that chunk runs long enough. Owed: an integration-tier arm over a real long-running query on the engine's own stores. Owner: route entry `Door admission lifecycle and bounds`.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: minted pending trigger `door-timeout-interrupt-real-query-coverage` (the door's interrupt of a real running query has no test; stand-in count only)."
    rationale: >-
      Report, Changes → Coverage of new surfaces, last bullet: "A timed-out call's interrupt of a real running query → tests ✗ (the stand-in's interrupt is counted in process; no query of this chunk runs long enough; the plan names `Door admission lifecycle and bounds` as owner)"; restated in Found and not owned item 7. §2 requires integration-tier coverage of cross-module boundaries; this new path has none, so the gap is recorded where the plan keeps documented gaps.
    basis: "pulse-app/tests/unit_door.rs:832-873 (the stand-in arm); pulse-app/src/door.rs:312-316 (`fn interrupt`)"
```

disposition: apply (after the halt) - check 1: recording a measured OPEN gap with a named owner (the 2026-08-28 rule; Sequencing deferral): the owner `Door admission lifecycle and bounds` takes a `CARRY:` at route-resolve - check 5: entry 7

### test-plan proposal 2

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Surfaces under test (new row, directly after the console engine program row)"
    change: >-
      Add a row **console engine door (`{data dir}/run/door/sock`, a Unix-domain stream socket; JSON-RPC 2.0, one request per line — `initialize` · `tools/list` · `tools/call` over the nine tools · `ping`)**, since chunk 2026-10-10-door-inside-the-engine-s-process. Driver: a `tokio::net::UnixStream` client from the tests at three depths — a seated door over a stand-in `Tools` in process (`pulse-app/tests/unit_door.rs`, 19 tests; the three log leaves in `unit_observability_allowlist_door.rs`, 6), the door seated by `engine_boot::start` in process (five door arms of `integration_engine_boot.rs`, each on a runtime whose threads carry the arm's name), and a spawned `andromeda-pulse-engine run` (`pulse-app/tests/integration_door.rs`, 5 arms; ports claimed through `test_support::ports`); the other-account arm's client is `python3 -c` run under `sudo -n -u nobody` and under `sudo -n`. Signal: the answer line (a result, or one fixed code of `-32700` / `-32600` / `-32601` / `-32602` / `-32603` / `-32001` / `-32002` / `-32003` with a fixed message), a close with no byte written for another account, directory mode 0700 and socket mode 0600, and the engine's own log family (`app.boot.door` once per boot, `door.call` once per answered request, `door.refused`; the planted marker 0 times, no full path, no record at ERROR). Boundary: single-surface, Unix only (`#[cfg(unix)]`); the window program seats no door (`the_window_program_seats_no_door`). Notes: the other-account arm prints its skip sentence and passes on a host where `sudo -n true` fails (the dev host), so a real second account is read on the runner alone, by the `lint-test` step `Door refuses another account` (§9); the applied write is shown in process, the spawned engine showing the write reaching the tool (`-32603`, no such incident) and its refusal with the switch off or rejected; open: §1 pending `door-timeout-interrupt-real-query-coverage`.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: §1 Surfaces under test gains the console engine's door (unit + in-process boot + spawned + a real second account on the runner)."
    rationale: >-
      Report, Symbols / APIs (the listening socket, the call surface, the error answers) and Coverage of new surfaces (`run/door/sock`, `tools/call` through the door, `mark_incident_resolved` through the door: "tests unit + integ + spawned, and a real second account on the runner"); Expected amendments item 7 names test-plan §1 "the door as a surface under test" as carried. The inventory lists every other surface and holds no row for this one, so the tier the new surface is held to is unstated.
    basis: "pulse-app/src/door.rs:366-407 (`bind_endpoint`); pulse-app/tests/integration_door.rs:683-802 (the other-account arm); pulse-app/tests/integration_engine_boot.rs:537-768 (the five door arms)"
```

disposition: apply (after the halt) - check 1: Accurate this-chunk addition - check 5: entry 7 (the door as a surface under test). Where it names the other-account arm it takes G4's marker

### test-plan proposal 3

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Surfaces under test → console engine program (`andromeda-pulse-engine run` / `version`) row, Signal cell"
    change: >-
      Replace "the process's own socket list under `/proc` (exactly the two loopback receivers and no socket beyond them), a rejected port variable's failed-bind record with the other receiver alone listening" with: the process's own socket list under `/proc` — since chunk 2026-10-10-door-inside-the-engine-s-process exactly the two loopback receivers and one `unix` listener at the canonical `run/door/sock`, by set equality in both census arms (until that chunk the two receivers and no socket beyond them) — and a rejected port variable's failed-bind record with the other receiver and the door listening.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: console engine row — the socket census is two loopback receivers + one unix door listener; the rejected-port arm leaves the HTTP receiver and the door."
    rationale: >-
      Report, Counts / qualifiers moved: "A console engine's listeners: two loopback TCP receivers → those two and one `unix` listener at the canonical `run/door/sock` (set equality in `integration_console_engine.rs`, both census arms)"; Deviation 5 renames the two tests because "the old names asserted what is no longer true". The row states the old set as current.
    basis: "pulse-app/tests/integration_console_engine.rs:434-446 (`door_listener`)"
```

disposition: apply (after the halt) - check 1: Accurate this-chunk addition - check 5: entry 7

### test-plan proposal 4

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Pending coverage triggers → `engine-boot-rejected-port-and-socket-census-coverage`"
    change: >-
      Append to the LANDED text: **Re-read at chunk 2026-10-10-door-inside-the-engine-s-process:** the census set is now the two receivers on `127.0.0.1` and one `unix` listener at the canonical `run/door/sock` (set equality, both census arms), and under a rejected gRPC port the HTTP receiver and the door stand; the two arms are renamed `the_engine_listens_on_its_two_loopback_receivers_and_its_door_and_on_nothing_else` and `a_rejected_grpc_port_leaves_that_receiver_unstarted_and_the_http_receiver_and_the_door_listening`. "Exactly the two receivers … and no socket beyond them" and "the HTTP receiver alone stands in the census" are the readings of the landing chunk. The file still holds 7 arms; nothing further is owed on this row.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: socket-census row re-read — two receivers + the door's unix listener; two arms renamed."
    rationale: >-
      Same retired claim as the console engine Surfaces row, restated here in the present tense ("finds exactly the two receivers on `127.0.0.1` and no socket beyond them", "the HTTP receiver alone stands in the census"). Report, Counts / qualifiers moved (listeners) and Deviation 5 (the two renames; no arm added to the file).
    basis: "pulse-app/tests/integration_console_engine.rs:434-446"
    dependent-of: D-tests-coverage
```

disposition: apply (after the halt) - check 1: Accurate this-chunk addition - check 5: entry 7 - a dependent: it applies or falls with its primary

### test-plan proposal 5

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Per-chunk gate discipline (the **Spawned-program witness form** paragraph)"
    change: >-
      In the three readings joined at chunk 2026-10-10-engine-harness-proofs-completed, date the two that moved: since chunk 2026-10-10-door-inside-the-engine-s-process the `/proc` census reads exactly the two loopback receivers and one `unix` listener at the canonical `run/door/sock` (set equality), and the rejected-port arm leaves the other receiver and the door listening (not "the other receiver alone"). And add: the form has a second spawned binary since that chunk, `pulse-app/tests/integration_door.rs` (5 arms on `andromeda-pulse-engine run`: the owner answered from the engine's stores with a call record holding none of it; a request outside the tool set refused beside an answered call; an answer over the size bound refused beside the same query at a small limit; the write switch read from the environment; another account refused at three layers), which joins the port-using binaries taking `test_support::ports` — five, was four. Its other-account arm runs one client (`python3 -c`) under `sudo -n -u nobody` and under `sudo -n`, records each of its six readings as measured, and prints the sentence `door other-account arm: directory refused, socket mode refused, account check refused` only when `nobody` was refused at the socket's mode too; where `sudo -n true` fails it prints its skip sentence and passes, so the reading is taken on the runner by the `lint-test` step `Door refuses another account` (§9), never from nextest's pass count.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: Spawned-program witness form — census and rejected-port readings include the door; `integration_door.rs` is the form's second binary and a fifth port claimer; the other-account arm's skip is read by a CI step."
    rationale: >-
      The keyed contract restates the retired census claim ("exactly the two loopback receivers, and no socket beyond them"; "the other receiver alone listening") and the count "taken by four port-using binaries". Report, Counts / qualifiers moved: the listener set, and "Test binaries that claim ports through `test_support::ports`: `integration_door` joins them"; Harness / gate surface (the sentence the step reads; the arm's one client); Deviation 2; Spec claims disproved 4 (the arm prints its skip sentence on the dev host in every run). Expected amendments item 7 names this keyed contract for "the second-account arm and its step".
    basis: "pulse-app/tests/integration_door.rs:683-802; pulse-app/tests/integration_console_engine.rs:434-446"
    dependent-of: D-tests-coverage
```

disposition: apply (after the halt) - check 1: Accurate this-chunk addition - check 5: entry 7 - a dependent: it applies or falls with its primary. Where it names the other-account arm it takes G4's marker

### test-plan proposal 6

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Per-chunk gate discipline (the `cargo xtask pre-push:linux` paragraph)"
    change: >-
      Where the paragraph says the sixth stage, and no other, starts the console engine and binds loopback 24317 and 24318, add: since chunk 2026-10-10-door-inside-the-engine-s-process that engine also opens its door, a Unix-domain socket at `run/door/sock` under the stage data dir (directory 0700, socket 0600; path 82 bytes by that chunk's research computation, not re-measured), left on disk when the engine ends and replaced at the next boot; the stage's teardown still reads the two ports and the pid alone. Green on the committed tree `2e4ee5ec` (`all-stages-ok`). The PROVISIONAL marker on what the stage starts and binds is unchanged and now covers the socket too.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: pre-push sixth stage — the engine it starts also opens the door socket inside the stage data dir."
    rationale: >-
      Report, Harness / gate surface: "The pre-push check's sixth stage starts a console engine with the door built in (stage data dir, socket path 82 bytes by research's computation, not re-measured)"; Schema / config: the socket file is not removed when the engine ends. The contract states what the stage's engine listens on as the two ports only — the same listener claim the census retires.
    dependent-of: D-tests-coverage
```

disposition: escalate - check 1: Boundary widening (it grows a standing PROVISIONAL item: what a harness-started console engine opens). HALT group G6 - a dependent: it applies or falls with its primary

### test-plan proposal 7

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Per-chunk gate discipline (the **Engine cycle gate form** paragraph)"
    change: >-
      Add: since chunk 2026-10-10-door-inside-the-engine-s-process the cycle's engine also opens its door under the cycle's data dir and its log holds exactly one `app.boot.door` record, over which `cargo xtask check:engine-log` reads `PASS`; the verb, the script and the verdict shape are unchanged. The canonical socket path must be shorter than 108 bytes (107 binds): the default cycle data dir measured 99 bytes on the dev host, and a deeper `--data-dir` boots with no door (`state` `path_too_long`, the record at WARN), the engine going on without one — a cycle that must read the door asserts `"state":"listening"` itself.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: Engine cycle gate form — one `app.boot.door` record per cycle; the 108-byte socket-path bound against the cycle data dir (99 bytes measured)."
    rationale: >-
      Report, Harness / gate surface: `harness:ready`, `harness:status`, `check:engine-log` and `harness:engine-cycle` "read what they read before; the engine they start now also opens the door, and `check:engine-log` passes over a log holding `app.boot.door`"; Counts: one more record per boot (336 records kept, exactly one of that target); Symbols: `path_too_long` at 108 bytes or more, 107 binds; Outcome → Gates, Smoke: "socket path 99 bytes"; Sweep hazards (the 108-byte bound).
    dependent-of: D-tests-coverage
```

disposition: apply (after the halt) - check 1: Accurate this-chunk addition - check 5: entry 7 - a dependent: it applies or falls with its primary

### test-plan proposal 8

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → Conventions → Test file location (Rust)"
    change: >-
      Lead the count with the current reading: 112 top-level `pulse-app/tests/*.rs` files as of chunk 2026-10-10-door-inside-the-engine-s-process — the three new ones `unit_door.rs`, `unit_observability_allowlist_door.rs`, `integration_door.rs` — 109 at its base (and at chunks 2026-10-10-console-engine-entry-point and 2026-10-10-engine-harness-proofs-completed, kept as dated readings); the closing sentence "`pulse-app/tests/*.rs` still counts 109 at chunk 2026-10-10-engine-harness-proofs-completed" stays a reading of that chunk and is followed by the 112.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: `pulse-app/tests/*.rs` 109 → 112 (three door files)."
    rationale: >-
      Report, Counts / qualifiers moved: "`pulse-app/tests/*.rs`: 109 → 112 (`ls pulse-app/tests/*.rs | wc -l` = 112)"; Files: the three new test files; Expected amendments item 7 ("the count 109 → 112"). The line's latest reading is 109, stated twice.
    basis: "report Changes → Counts / qualifiers moved, first bullet"
```

disposition: apply (after the halt) - check 1: Accurate this-chunk addition (a count the report moved) - check 5: entry 7

### test-plan proposal 9

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration (`:484`) → Pipeline structure → Lint + tests (`lint-test` job) row"
    change: >-
      After `cargo xtask test` and before `cargo xtask perf:slo-load`, state the step added at chunk 2026-10-10-door-inside-the-engine-s-process: `Door refuses another account`, a plain `run` step directly after `cargo xtask test` (no `if:`, no `continue-on-error`, no upload) that runs `cargo nextest run --workspace --profile ci --no-tests=fail --success-output immediate -E 'binary(integration_door) & test(=another_account_is_refused_at_each_of_three_layers)'`, keeps the output in `$RUNNER_TEMP/door-other-account.txt`, and exits non-zero unless nextest exited 0, the file is not empty and it holds `door other-account arm: directory refused, socket mode refused, account check refused` — so an arm that skipped, or that read a layer open, fails the step although it passes in the workspace run. It needs a second account on the runner reachable by `sudo -n` (root, and `-u nobody`) and `python3`; the step's own text holds no `sudo`. Pinned by two pins and four `bash -e` witnesses in `pulse-app/tests/quality_gate_workflow.rs` (35 → 41 tests), `cargo` standing as a shell function that prints a captured file. Green on `ci#38097928056`: `nobody` refused by the kernel at the 0700 directory and at the 0600 socket (errno 13), root passed both and was closed by the door's account check at all three layers. The workspace run counted 3174 tests on the dev host at that chunk (3132 at the previous one).
    sidecar: "2026-10-10-door-inside-the-engine-s-process: `lint-test` gains the step `Door refuses another account` after `cargo xtask test`; pins + witnesses in `quality_gate_workflow.rs`; green on `ci#38097928056`; workspace 3132 → 3174."
    rationale: >-
      Report, Harness / gate surface (`.github/workflows/ci.yml:114-139`; `quality_gate_workflow.rs:937-1111`), Counts ("`lint / test` job steps: one added, `Door refuses another account`, directly after `cargo xtask test`"; nextest 3132 → 3174; `quality_gate_workflow` +6, 35 → 41), Cross-project / external claims (the run and its readings). The row enumerates the job's steps in order with `perf:slo-load` next after `cargo xtask test`, and lists what the runner must provide; both are now short by this step.
    basis: ".github/workflows/ci.yml:114-139; pulse-app/tests/quality_gate_workflow.rs:937-1111"
```

disposition: escalate - check 1: Boundary widening (the workflow step that runs a client as a second account). HALT group G4 - check 5: entry 7

### test-plan proposal 10

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Pipeline structure → Integration tests row"
    change: >-
      Add after the fourth-binary sentence: since chunk 2026-10-10-door-inside-the-engine-s-process a fifth binary takes the claim, `pulse-app/tests/integration_door.rs` (its spawned engines' two ports); `integration_engine_boot.rs` holds 11 tests at that chunk (6 before; five door arms added), and the "four in-process arms" and "seven of seven hand runs of the four binaries" are readings of chunk 2026-10-10-shared-telemetry-test-data. At the door chunk seven of seven repeated runs of five binaries were green, 48 tests each (a count, not an exclusion).
    sidecar: "2026-10-10-door-inside-the-engine-s-process: `integration_door` is the fifth binary claiming ports through `test_support::ports`; 7 of 7 repeated runs green."
    rationale: >-
      Report, Counts / qualifiers moved: "Test binaries that claim ports through `test_support::ports`: `integration_door` joins them" and "`integration_engine_boot` +5 (6 → 11)"; Outcome → Gates: the repeated run "seven of seven, 48 tests each" over five binaries. The row names four binaries as the claimers. Whether each new door arm of `integration_engine_boot.rs` claims ports is not stated by the report, so the change dates the old wording rather than restating a per-arm count.
    dependent-of: D-tests-coverage
```

disposition: apply (after the halt) - check 1: Accurate this-chunk addition - check 5: entry 7

### test-plan proposal 11

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration → Build failure conditions"
    change: >-
      Add a bullet: the `lint-test` step `Door refuses another account` fails (since chunk 2026-10-10-door-inside-the-engine-s-process) — its one nextest selection exits non-zero or matches no test, its captured output is empty, or the output lacks the arm's three-layer sentence, which is the case when the arm skipped for want of a second account or read the socket's mode as not refusing; no soft-fail key, pinned by `ci_workflow_runs_the_door_other_account_arm_after_the_test_step_with_no_soft_fail` and the four step witnesses.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: build fails when the door other-account step does not read its three-layer sentence (a skipped arm included)."
    rationale: >-
      Report, Harness / gate surface: the step "exits non-zero unless nextest exited 0, the file is not empty and the file holds" the sentence; Deviation 2: the other sentence "fails the CI step". "Any … test fails" does not cover a passing-but-skipped arm, which this step turns red.
    basis: "pulse-app/tests/quality_gate_workflow.rs:943-968 (the place-and-plain-form pin); :1065-1090 (the skipped / layer-open witness)"
```

disposition: escalate - check 1: Boundary widening (the workflow step that runs a client as a second account). HALT group G4

### test-plan proposal 12

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage scope → mcp-server (stdio sidecar) row"
    change: >-
      Scope the gating sentence to the sidecar and state the second carrier: the stdio sidecar stays feature-gated (`--features mcp-server`) and runtime-gated (`ANDROMEDA_PULSE_MCP_ENABLED=true`), and `rmcp` still enters only under the feature; since chunk 2026-10-10-door-inside-the-engine-s-process the crate itself is linked by `pulse-app` in every build (the dependency is no longer optional) and its dispatch is also reached, under neither gate, through the console engine's door — `mcp_server::tools::call_tool` (the dispatch with no log record; `dispatch_tool` keeps its signature and calls it) over `door::EngineTools` — testable in process and on a spawned engine (§1 Surfaces under test, door row). `tools_list_manifest()` gives every tool a boolean `write`, true for `mark_incident_resolved` alone, in the sidecar's `tools/list` answer too (pin `tools_list_manifest_marks_exactly_one_tool_as_a_write`).
    sidecar: "2026-10-10-door-inside-the-engine-s-process: mcp-server entity — crate linked in every `pulse-app` build; `call_tool` reached through the door under neither sidecar gate; `tools/list` carries `write`."
    rationale: >-
      Report, Dependencies: "`mcp-server-crate` (package `mcp-server`) is no longer `optional`"; Crates / modules: "`pulse-app` now links `mcp-server` in every build"; Symbols → `crates/mcp-server` (`ToolAnswer`, `call_tool`, its two callers, the `write` member as "the one change the stdio sidecar's own output takes"). The row presents the whole entity as feature- and runtime-gated; that now holds for the sidecar and `rmcp` only.
    basis: "pulse-app/Cargo.toml:87; crates/mcp-server/src/tools.rs:168-185 (`call_tool`); crates/mcp-server/src/jsonrpc.rs:396-412 (the `write` pin)"
```

disposition: escalate - check 1: Boundary widening (the reach: the nine tools served inside the engine outside the sidecar double gate, and the resolve write). HALT group G3

### test-plan proposal 13

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Coverage triggers → security-vector-coverage: MCP feature + runtime gating"
    change: >-
      Append to the Required Test Type cell: both gates are the stdio sidecar's. Since chunk 2026-10-10-door-inside-the-engine-s-process the same nine tools also answer through the console engine's door under neither gate, admitted by the door's owner check (the peer's kernel-read account equal to the engine's own; any other account, root included, closed with no byte) with the one write behind `ANDROMEDA_PULSE_DOOR_RESOLVE`; that path's negative tests are the door's — another account refused at three layers (runner only), the write refused with `-32001` when the switch is `off` or rejected — per §1 Surfaces under test, door row.
    sidecar: "2026-10-10-door-inside-the-engine-s-process: MCP gating trigger scoped to the sidecar; the door's tool path is admitted by the owner check and the write switch, with its own negative tests."
    rationale: >-
      The row restates, as the gate on agent tool access, the mechanism the entity row above names (compile-time feature AND runtime variable). Report, Symbols: the owner check (`door::admits`), the write switch (`ResolveWrite`, `-32001` before the dispatcher), the product-read variable; Dependencies / Crates: the dispatch is in every build. Nothing of the sidecar was removed (report, Expected amendments item 7), so the sidecar's two negative tests stand as written.
    basis: "pulse-app/src/door.rs:163-167 (`admits`); pulse-app/src/door.rs:43-74 (`ResolveWrite`); pulse-app/src/engine_boot.rs:134-139 (`resolve_door_write`)"
    dependent-of: D-tests-coverage
```

disposition: escalate - check 1: Boundary widening (the reach: the nine tools served inside the engine outside the sidecar double gate, and the resolve write). HALT group G3 - a dependent: it applies or falls with its primary

## obs-plan

verdict: proposals 10 - stripped: the agent's own comment lines, kept below as returned

```
# obs-plan drift pass — chunk 2026-10-10-door-inside-the-engine-s-process
# D-obs-stack: no drift (Dependencies: none added, none bumped; the door's three emit sites are `tracing` events on the engine's existing file sink).
# D-obs-pii: no drift (no record carries an argument, an answer, a path, an account id or an error text; marker 0 times in the log family; no door.call field redacted on a real call; the variable's value never logged). The §8 leaf registration below rides D-obs-instrumentation at warning, not escalate.
# D-obs-defect-narrative: no drift (none of the four disproved claims, the Outcome, or a Coverage row touches a §10 defect; the door takes its own read connection through viz::query::read_connection and interrupts only that one, which is the reference pattern; the three-fixed / one-closed tally stands).
# D-obs-instrumentation: drift — the door carries the P3 tool path with one `door.call` log record and none of the `mcp.*` spans §4 P3 requires; the primary and nine other occurrences follow.
# Seen, owned by none of my four detectors (not proposed): §9 Pipeline integration `xtask test` row and the §9 triage bullet count "three test binaries" that start the console program — the report's integration_door arms run on a spawned engine and lint-test gained the step `Door refuses another account` (.github/workflows/ci.yml:114-139); §10 CI gates heartbeat bullet says the pre-push sixth stage "binds two loopback ports" — that engine now also opens the door's unix listener.
```

### obs-plan proposal 1

```yaml
  - detector: D-obs-instrumentation
    severity: warning
    section: "§4 Span / Trace Coverage → Scenario: P3 — MCP server query (JSON-RPC 2.0 tool call)"
    change: >-
      State two carriers. (a) The stdio sidecar, unchanged: the `mcp.tools.call.request` → `mcp.feature.gate.check` → per-tool read → `mcp.tools.call.response` chain through `dispatch_tool`. (b) The door inside the console engine's process (`andromeda-pulse-engine run`, JSON-RPC 2.0 one request per line over the Unix-domain socket `{data dir}/run/door/sock`; Unix only, the window program seats none): `tools/call` runs `mcp_server::tools::call_tool` over `door::EngineTools`, which writes no `mcp.*` record and passes no feature or runtime gate — the carrier's one record is the log event `door.call` {`method`, `tool`, `outcome`, `write`, `result_type`, `result_count`, `answer_bytes`, `duration_ms`}, once per answered request (a request with no `id` is not answered and leaves none), written before the answer line, INFO for `ok` and WARN otherwise, never ERROR; a refused connection or line leaves `door.refused` {`reason`, `peer`} at WARN; the tools' own callee records (`viz.query.traces` / `.metrics` / `.logs`, and for `generate_snapshot` `metric.snapshot.token_count_ms` + `snapshot.token.count.validate`) are written by the console engine on such a call, not enumerated from a log at this chunk. Surfaces involved gains the door; Cleanup gains: the door closes an idle connection at 30 s and answers a call over 5 s with `-32002`.
    sidecar: "§4 Scenario P3 — second carrier stated: the console engine's door records a tool call as one `door.call` event through `call_tool`, with no `mcp.*` span and no gate check (chunk 2026-10-10-door-inside-the-engine-s-process)."
    rationale: >-
      Report Changes → Symbols / APIs: `call_tool` is "the dispatch with no log record" with two callers, `dispatch_tool` (which keeps its records by calling it) and `door::EngineTools`; the door's records are the three targets `app.boot.door`, `door.call`, `door.refused` and nothing else. §4 P3 (obs-plan :215) requires the `mcp.tools.call.request` / `mcp.feature.gate.check` / `mcp.tools.call.response` spans with `feature_flag_enabled` / `env_var_mcp_enabled` on every MCP tool call and names stdio as the only carrier; the door path carries none of them. Expected amendments item 6 names this site ("§4 Scenario P3 (:215) … the tool path's second carrier"), and the Outcome's obs criteria read met on `door.call`.
    basis: "pulse-app/src/door.rs:203-234 (emit_call) · crates/mcp-server/src/tools.rs:168-185 (call_tool) · pulse-app/src/door.rs:255-317 (EngineTools)"
```

disposition: apply (after the halt) - check 1: Accurate this-chunk addition; the plan's Expected amendment 6 names the change - check 5: entry 6. Its sentence on the gates follows G3's answer

### obs-plan proposal 2

```yaml
  - detector: D-obs-instrumentation
    severity: warning
    section: "§1 Obs Scope Summary → Critical paths (must-trace) → P3 row"
    change: >-
      Scope the row's span chain and its `feature_flag_enabled` / `env_var_mcp_enabled` fields to the stdio sidecar, and add the second carrier: a tool call through the console engine's door (Unix-domain socket, console program only) emits no `mcp.*` span and passes no double gate; its must-log record is `door.call` (`method`, `tool`, `outcome`, `write`, `result_type`, `result_count`, `answer_bytes`, `duration_ms`); Surfaces involved gains the door (§4 Scenario P3).
    sidecar: "§1 Critical paths P3 row — spans and gate fields scoped to the stdio sidecar; the door carrier and its `door.call` record added."
    rationale: >-
      Same claim as the primary, restated in the critical-path table: the row states the `mcp.tools.call.request` / `mcp.feature.gate.check` / `mcp.response.emit` spans and the two gate fields as THE P3 path. Report Changes: the door runs the nine tools through `call_tool`, "the dispatch with no log record", and writes `door.call`.
    basis: "pulse-app/src/door.rs:203-234"
    dependent-of: D-obs-instrumentation
```

disposition: apply (after the halt) - check 1: Accurate this-chunk addition; the plan's Expected amendment 6 names the change - check 5: entry 6 - a dependent: it applies or falls with its primary

### obs-plan proposal 3

```yaml
  - detector: D-obs-instrumentation
    severity: warning
    section: "§3 → Snapshot / paste-to-AI integration"
    change: >-
      In the MCP tools label, replace "Every tool emits `mcp.tools.call.request` → `mcp.tools.call.response` … through the single `dispatch_tool` emission site" with: through the stdio sidecar every tool emits that pair at `dispatch_tool`, which now calls `mcp_server::tools::call_tool` (returns `ToolAnswer { value, result_type, result_count }`, writes no record); `call_tool` has a second caller, `door::EngineTools` in the console engine, whose calls are recorded as `door.call` carrying `result_type` + `result_count` and never the answer (vector 4 holds on both carriers). The label's "over stdio" names the sidecar's transport; the door's is a Unix-domain socket.
    sidecar: "§3 Snapshot / paste-to-AI — MCP tools: the `mcp.*` pair scoped to `dispatch_tool` (stdio); `call_tool` and its second caller, the door, recorded as `door.call`."
    rationale: >-
      The keyed contract restates the primary's claim as "Every tool emits … through the single `dispatch_tool` emission site". Report Changes → `crates/mcp-server`: new `ToolAnswer` and `call_tool`, "the dispatch with no log record"; "`call_tool` has two callers: `dispatch_tool` and `door::EngineTools`".
    basis: "crates/mcp-server/src/tools.rs:159-166 · crates/mcp-server/src/tools.rs:168-185"
    dependent-of: D-obs-instrumentation
```

disposition: apply (after the halt) - check 1: Accurate this-chunk addition; the plan's Expected amendment 6 names the change - check 5: entry 6 - a dependent: it applies or falls with its primary

### obs-plan proposal 4

```yaml
  - detector: D-obs-instrumentation
    severity: warning
    section: "§1 Obs Scope Summary → Telemetry triggers → logging-sensitive row, Security plan Vector 4"
    change: >-
      Add the second enforcement site: beside the `mcp-server` boundary's `mcp.tools.call.response` (`result_type` + `result_count`, no `result_content`), the console engine's door records a tool call as `door.call` with `result_type`, `result_count` and `answer_bytes` — no argument, no answer and no error text, every refusal a fixed message; negative reading: a marker planted in OTLP content and returned by the owner's `query_logs` appears 0 times in the engine's log family.
    sidecar: "§1 Telemetry triggers, Vector 4 — the door named as a second MCP boundary; its `door.call` record carries metadata only."
    rationale: >-
      The trigger row states the record of a tool call's result as the `mcp.tools.call.response` span at the `mcp-server` boundary alone. Report Changes: "No record carries an argument, an answer, a path, an account id or an error's text"; Coverage row `tools/call` through the door: "PII redacted✓ (no argument, answer or error text in the record; marker 0 times in the log family)".
    basis: "pulse-app/src/door.rs:203-234 · pulse-app/src/door.rs:567-585 (refusal_of)"
    dependent-of: D-obs-instrumentation
```

disposition: apply (after the halt) - check 1: Accurate this-chunk addition; the plan's Expected amendment 6 names the change - check 5: entry 6 - a dependent: it applies or falls with its primary

### obs-plan proposal 5

```yaml
  - detector: D-obs-instrumentation
    severity: warning
    section: "§4 Span / Trace Coverage → Instrumentation per surface (table)"
    change: >-
      Add a row beside "MCP stdio": the door (console engine, JSON-RPC 2.0 over the Unix-domain socket `{data dir}/run/door/sock`, Unix only; `#[cfg(unix)] mod socket`) — hook: three `tracing` log events and no span, on the engine's one file sink: `app.boot.door` once per boot of a program that seats the door (`door::emit_boot`), `door.call` once per answered request (`door::emit_call`), `door.refused` per refused connection or line (`door::emit_refused`); no feature gate and no runtime gate — admission is the kernel-read peer account compared with the engine's own (`door::admits`), the one write behind `ANDROMEDA_PULSE_DOOR_RESOLVE`; seated in `engine_boot::start` for `Program::Console` only, the window program writes no door record.
    sidecar: "§4 Instrumentation per surface — new row: the console engine's door, three log targets, no span."
    rationale: >-
      The per-surface table states the MCP tool boundary as the "MCP stdio" row alone. Report Changes → Symbols / APIs: a listening socket "the console program's alone", seated "in `engine_boot::start` and nowhere else", with three log targets; Coverage of new surfaces lists `run/door/sock` and `tools/call` through the door as new surfaces with "instrumentation log✓".
    basis: "pulse-app/src/engine_boot.rs:1037-1053 · pulse-app/src/door.rs:182-201 · pulse-app/src/door.rs:236-244 · pulse-app/src/door.rs:319-749"
    dependent-of: D-obs-instrumentation
```

disposition: apply (after the halt) - check 1: Accurate this-chunk addition; the plan's Expected amendment 6 names the change - check 5: entry 6 - a dependent: it applies or falls with its primary

### obs-plan proposal 6

```yaml
  - detector: D-obs-instrumentation
    severity: warning
    section: "§1 Obs Scope Summary → Justification (Cross-platform desktop scope bullet)"
    change: >-
      "8 major surfaces (… MCP stdio)" → 9, the ninth the console engine's door (JSON-RPC 2.0 over a Unix-domain socket, console program only) — the count the §4 per-surface table's new row moves.
    sidecar: "§1 Justification — major surfaces 8 → 9: the console engine's door joined."
    rationale: >-
      The count restates the surface set the §4 per-surface table enumerates; the report's Changes add one listening surface ("A console engine's listeners: two loopback TCP receivers → those two and one `unix` listener") that carries the MCP tool calls. Apply only together with the §4 table row.
    dependent-of: D-obs-instrumentation
```

disposition: apply (after the halt) - check 1: Accurate this-chunk addition (a count the report moved) - a dependent: it applies or falls with its primary

### obs-plan proposal 7

```yaml
  - detector: D-obs-instrumentation
    severity: warning
    section: "§4 Span / Trace Coverage → Instrumentation per surface → CLI / main binary row"
    change: >-
      Extend the console `run` record set: besides `app.boot.tracing.init`, `app.boot.pid`, the panic and process-end records, `app.boot.engine` and the shared engine wiring's records, a console `run` writes `app.boot.door` once per boot (the engine-cycle kept log: exactly one, `"state":"listening"`), and, when a caller uses the door, `door.call` / `door.refused` and the called tools' own records (`viz.query.traces` / `.metrics` / `.logs`, the `viz.query` fallback warning, and for `generate_snapshot` `metric.snapshot.token_count_ms` + `snapshot.token.count.validate` — not enumerated from a log at this chunk); the window app writes no door record.
    sidecar: "§4 CLI / main binary row — a console run's records gain `app.boot.door` and, on a door call, `door.call` / `door.refused` and the tools' callee records."
    rationale: >-
      The row enumerates what a console `run` carries and stops at `app.boot.engine` plus the shared wiring. Report Changes → Counts / qualifiers moved: "A console engine's log per boot: one more record, `app.boot.door`"; Symbols / APIs: "Library records now written by a console engine when a tool is called through the door"; Outcome smoke: window 0 door records, console engine 1 `app.boot.door`.
    basis: "pulse-app/src/engine_boot.rs:1037-1053 · pulse-app/src/door.rs:182-201"
    dependent-of: D-obs-instrumentation
```

disposition: apply (after the halt) - check 1: Accurate this-chunk addition; the plan's Expected amendment 6 names the change - check 5: entry 6 - a dependent: it applies or falls with its primary

### obs-plan proposal 8

```yaml
  - detector: D-obs-instrumentation
    severity: warning
    section: "§6 Log Coverage → Log levels mapping → `warn` row"
    change: >-
      Add the three door records: `warn!(target: "app.boot.door", state, resolve_write)` — once per boot of a program that seats the door (the console program on Unix), off any hot path per §11: INFO when `state` is `listening` and the switch is not `rejected`, WARN otherwise (`path_too_long` | `in_use` | `occupied` | `bind_failed`, in each of which the engine goes on with no door; or `resolve_write` `rejected`); `warn!(target: "door.call", method, tool, outcome, write, result_type, result_count, answer_bytes, duration_ms)` — once per answered request, written before the answer line, INFO for `ok` and WARN for the other eight outcomes; `warn!(target: "door.refused", reason, peer)` — WARN, one per refused connection or line (`other_account` | `too_many_connections` | `request_too_large` | `idle`); none of the three is ever written at ERROR (§8; chunk 2026-10-10-door-inside-the-engine-s-process).
    sidecar: "§6 warn row — `app.boot.door`, `door.call`, `door.refused` added with their levels and emission bounds."
    rationale: >-
      §6 (obs-plan :297) enumerates each once-per-boot / once-per-event record with its INFO and WARN arms (`app.boot.engine` is the sibling) and holds none of the door's. Report Changes → "Three log targets, each an exact allowlist leaf" with fields, levels and emit sites; "none is written at ERROR". Expected amendments item 6 names §6 Log Coverage as carried.
    basis: "pulse-app/src/door.rs:182-201 · pulse-app/src/door.rs:203-234 · pulse-app/src/door.rs:236-244"
    dependent-of: D-obs-instrumentation
```

disposition: apply (after the halt) - check 1: the 2026-08-16 D-obs-pii leaf-registration rule, both conditions hold (every emitted field in the leaf; the guard in `pulse-app/tests/`), dual site §6 + §8 - check 5: entry 6

### obs-plan proposal 9

```yaml
  - detector: D-obs-instrumentation
    severity: warning
    section: "§8 PII Scrubbing & Compliance → PII Scrubbing → Default-deny posture (whitelist per module)"
    change: >-
      Register three EXACT leaves, each carrying every field its emit site emits and each a closed label, a bool or a number: `app.boot.door`: `state`, `resolve_write`; `door.call`: `method`, `tool`, `outcome`, `write`, `result_type`, `result_count`, `answer_bytes`, `duration_ms`; `door.refused`: `reason`, `peer` (`owner` | `other`, never an account id). Never an argument, an answer, the socket's path, an account id, an error's text or the value of `ANDROMEDA_PULSE_DOOR_RESOLVE` (a value outside `on` / `off` reads `rejected`). No bare `door` key and no bare `app` key exists (`for_target("door").is_none()` pinned), so without its leaf each target redacts whole. Asserted by `pulse-app/tests/unit_observability_allowlist_door.rs` (6 tests: exact resolve, set equality at each emit site, levels, closed labels, none at ERROR; mutation-checked — the `door.call` leaf narrowed by one field left the resolve pin green and only the set-equality pin red). Wire-read: one `app.boot.door` `listening` in the engine-cycle kept log; on `ci#38097928056` one `door.call` (the owner's, no field redacted) and four `door.refused`.
    sidecar: "§8 allowlist — exact leaves `app.boot.door`, `door.call`, `door.refused` registered with their guard file; no bare `door` key."
    rationale: >-
      §8 (obs-plan :404) enumerates every exact allowlist leaf with its field set, its guard and its wire reading (`app.boot.engine` is the sibling) and holds none of the door's. Report Changes: three leaves at `observability.rs:1022-1045`, "no bare `door` key and no bare `app` key exists"; Counts: "Allowlist exact leaves: three added"; Outcome (obs): the guards pin exact resolve, set equality and `for_target("door").is_none()` — met; Sweep hazards records mutation 2. Raised here at warning because the invariant of D-obs-pii holds; this is the registry's completeness, the dual site of the §6 row.
    basis: "pulse-app/src/observability.rs:1022-1045"
    dependent-of: D-obs-instrumentation
```

disposition: apply (after the halt) - check 1: the 2026-08-16 D-obs-pii leaf-registration rule, both conditions hold (every emitted field in the leaf; the guard in `pulse-app/tests/`), dual site §6 + §8 - check 5: entry 6

### obs-plan proposal 10

```yaml
  - detector: D-obs-instrumentation
    severity: warning
    section: "§11 Obs Anti-Patterns → Project-specific (andromeda-pulse) → MCP double-gate ban"
    change: >-
      Scope the ban to the stdio sidecar (`andromeda-pulse-mcp`): its compile-time `--features mcp-server` + runtime `ANDROMEDA_PULSE_MCP_ENABLED=true` double gate and the `mcp.feature.gate.check` span stand unchanged. The console engine's door serves the same nine tools behind neither gate (`pulse-app` links `mcp-server` in every build; `rmcp` still enters only under the feature) and emits no gate-check span: its admission is the owner check on every accepted connection, and its state is read from `app.boot.door` (`state`, `resolve_write`) and `door.refused` (`reason`, `peer`).
    sidecar: "§11 Project-specific — the double-gate ban scoped to the stdio sidecar; the door's admission records named."
    rationale: >-
      The ban restates the primary's retired claim without its span tokens: that the MCP tool surface is reached only past the double gate whose state rides `mcp.feature.gate.check`. Report Changes → Crates / modules: "`pulse-app` now links `mcp-server` in every build"; Dependencies: `mcp-server-crate` no longer `optional`; Symbols / APIs: the owner check and the write switch are the door's admission. The sidecar itself is unchanged this chunk (its removal moved to a removal entry), so the ban is narrowed, not retired.
    basis: "pulse-app/Cargo.toml:87 · pulse-app/src/door.rs:163-167 (admits) · pulse-app/src/engine_boot.rs:86-87"
    dependent-of: D-obs-instrumentation
```

disposition: escalate - check 1: Boundary widening (the reach: the nine tools served inside the engine outside the sidecar double gate, and the resolve write). HALT group G3 (it narrows a NEVER ban to the sidecar); written with G3's answer though its primary is routine

## a11y-plan

verdict: proposals 0 - stripped: the agent's own comment lines, kept below as returned

```
# a11y-plan drift-detector — chunk 2026-10-10-door-inside-the-engine-s-process
# D-a11y-surface: no drift. The report adds no interactive UI element: Files bullet states every
#   `pulse-app/ui/**` file is unchanged (scope-guard entry prints nothing); Outcome states "(design, layouts,
#   a11y) No screen, window, tray item, notification or control; no webview file — met"; every row of
#   Coverage of new surfaces reads `a11y n/a`. The new surface is a Unix-domain socket (JSON-RPC), an
#   environment variable and a CI step in `lint-test` — none is a §5 / §6 / §7 subject.
# D-a11y-obs-schema: no drift. The report changes neither the a11y violation schema nor the obs record
#   envelope: it adds three allowlist exact leaves (`app.boot.door`, `door.call`, `door.refused`,
#   `pulse-app/src/observability.rs:1022-1045`) and states "No scrub or redaction shape changed". The keyed
#   contract `§3 → Structured violation JSON schema` (timestamp / level / target `a11y::assertion` / message /
#   fields) and the §1 body copy are untouched by any Changes bullet.
# Sweep for retired claims: none retired for this doc. The sidecar was not removed this chunk ("nothing of
#   the sidecar was removed"), so a11y-plan.md's P3 row ("mcp-server sidecar identity", "MCP server toggle")
#   and the settings-modal MCP toggle mentions stay true. The new CI step sits in `lint-test`; the keyed
#   contract `§3 → CI integration` and §9 speak only of the `a11y` job, which the report does not touch.
# Key files read: structured-violation-json-schema.md, ci-integration.md, harness-wiring-conventions.md.
```

raw twin: `.raw-fanout-a11y-plan.md` (a `proposals: []` return that stripping changed)

## Escalation resolved — the operator's answers at the widening halt (inputs#I7)

word: "halt read whole. 1 - G1 to G6: apply as recommended, written PROVISIONAL, my reading as operator of 2026-10-11, the founder own word owed, one item. 2 - S15: the four bans in, PROVISIONAL with the rest. 3 - A21: yes, the skip as a stated exception with its reason. 4 - G5: no playbook rule; record the collision in the report and the handoff as a deferred learning. 5 - G7: the removal entry owns it - the sidecar leaves there, so no fix is built; the sentence says so. 6 - G8: refine as worded with the premise-correction note, then the flip at coverage. 7 - yes: in full once per master at its primary site, a pointer at the restating sites." — the operator (the pc overseer), 2026-10-11

- G1 to G6 (43 proposals and the one raised for obs-plan §10): applied, each body re-derived from the report, never pasted from a `change` line. The door's full PROVISIONAL sentence stands once per master: architecture §Occupied Resources → Network ports; security-plan §Threat Model Summary → Attack surface, the door's vector; test-plan §9, the `lint-test` row; obs-plan §4 Scenario P3. Every restating site carries a pointer to it.
- security-plan 15: the four bans are in §Security Anti-Patterns → Authentication.
- architecture 21: the skip is written as a stated exception to the row's FAILS-never-skips rule, with its reason.
- G5: no playbook rule minted. The collision is recorded in the report (Decisions & corrections) and rides the handoff as a deferred learning.
- G7 (security-plan 5): applied as measured; the row names the removal entry as owner and says no fix is built.
- G8 (P-126): written at P7.3 — `matrix.py refine` with the premise-correction note, then the flip.
- The 23 routine proposals and the one raised for obs-plan §9: applied after the halt.
- Added by the cascade's sweep, outside every proposal: a11y-plan §1 Justification, which quoted obs-plan's surface count (8, now 9); routine — a cross-master citation of an amended count (amendment-flow, Cascade step 2).

Totals of the pass: 70 amendments (architecture 22, security-plan 22, test-plan 13, obs-plan 12, a11y-plan 1) in five masters and five key files; 14 sidecar entries; 0 proposals rejected; 0 escalations open.
