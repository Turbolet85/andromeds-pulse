# Apply — architecture · part 1 of 2

## Verdicts
- fanout-architecture.md 1 · applied · E1
- fanout-architecture.md 2 · applied · E2
- fanout-architecture.md 3 · applied · E3
- fanout-architecture.md 4 · applied · E4
- fanout-architecture.md 5 · applied · E5
- fanout-architecture.md 6 · applied · E6
- fanout-architecture.md 7 · applied · E7
- fanout-architecture.md 8 · applied · E8
- fanout-architecture.md 9 · applied · E9
- fanout-architecture.md 10 · applied · E10
- fanout-architecture.md 11 · applied · E11
- fanout-architecture.md 12 · folded into fanout-architecture.md 11
- fanout-architecture.md 13 · applied · E12
- fanout-architecture.md 14 · applied · E13
- fanout-architecture.md 15 · applied · E14
- fanout-architecture.md 18 · refused · a hand removal · fct-vqhoi5hb
- fanout-architecture.md 19 · applied · E15
- fanout-architecture.md 20 · applied · E16
- fanout-architecture.md 21 · applied · E17
- fanout-architecture.md 22 · applied · E18
- fanout-architecture.md 23 · applied · E19
- fanout-architecture.md 24 · applied · E20
- fanout-architecture.md 25 · folded into fanout-architecture.md 24
- fanout-architecture.md 26 · applied · E21
- fanout-architecture.md 27 · applied · E22
- fanout-architecture.md 28 · applied · E23

## Edits
@fact id=fct-wuyupllp expect_rev=1
old: The CORRECT `target/{profile}/andromeda-pulse*` paths are two, each a declared `[[bin]]` name:
new: The CORRECT `target/{profile}/andromeda-pulse*` path is one, a declared `[[bin]]` name:
old: one per program) and the sidecar below. `target/{profile}/andromeda-pulse` itself
new: one per program). `target/{profile}/andromeda-pulse` itself
old: The product binaries are three: `pulse-app` (the window app), `andromeda-pulse-engine` (the console program), `andromeda-pulse-mcp` (the stdio sidecar).
new: The product binaries are two: `pulse-app` (the window app) and `andromeda-pulse-engine` (the console program: `run` · `version` · `door`).
old: MCP sidecar binary (when feature enabled): `andromeda-pulse-mcp`
new: MCP sidecar binary: N/A — no stdio sidecar; a stdio MCP client starts `andromeda-pulse-engine door` (above)

@fact id=fct-bka6rc62 expect_rev=1
old: Hand-rolled serde JSON-RPC 2.0 over stdio (`crates/mcp-server/src/jsonrpc.rs`, MCP protocol `2024-11-05`) and, since chunk 2026-10-10-door-inside-the-engine-s-process, the same layer over a Unix-domain socket inside the console engine's process (the door, `pulse-app/src/door.rs`, Unix only; PROVISIONAL — §Occupied Resources → Network ports); `rmcp` linked as a feature-gated anchor dependency — requirement `"3"`, lockfile-resolved 3.1.4 as of 2026-08-29 |
new: Hand-rolled serde JSON-RPC 2.0 (`crates/mcp-server/src/jsonrpc.rs`, MCP protocol `2024-11-05`) served inside the console engine's process over the Unix-domain socket `run/door/sock` (the door, `pulse-app/src/door.rs`, Unix only; PROVISIONAL — §Occupied Resources → Network ports) and reached over stdio only through the relay `andromeda-pulse-engine door`; no `rmcp` |
old: — gated by `--features mcp-server` as the stdio sidecar and the `rmcp` anchor only: since chunk 2026-10-10-door-inside-the-engine-s-process `pulse-app` links the `mcp-server` crate in every build and the console engine serves the nine through the door with no feature; since chunk 2026-10-12-stdio-agent-reaches-the-door a stdio agent reaches them through the console program's word `andromeda-pulse-engine door` (a relay to the door), also with no feature — so `--features mcp-server` gates the stdio sidecar, not every stdio way to the nine (PROVISIONAL
new: — `pulse-app` names `mcp-server` as a plain dependency linked in every build, with no feature gate and no stdio sidecar; the console engine serves the nine through the door, and a stdio agent reaches them through the console program's word `andromeda-pulse-engine door` (a relay to the door) (PROVISIONAL

@fact id=fct-rkbubyxp expect_rev=1
old: They run in ci.yml's `lint / test`, `mcp-server tests` and `coverage gate` jobs, green on `ci#38079305603`.
new: They run in ci.yml's `lint / test` and `coverage gate` jobs, green on `ci#41572801661`.

@fact id=fct-rsvfsams expect_rev=1
old: Hand-rolled serde JSON-RPC 2.0 over stdio; `rmcp` linked as a feature-gated anchor dependency (requirement `"3"`, lockfile-resolved 3.1.4 as of 2026-08-29)**: the stdio protocol is the measured hand-rolled layer
new: Hand-rolled serde JSON-RPC 2.0, served by the console engine's door; no `rmcp`**: the protocol is the hand-rolled layer
old: with name-dispatch in `tools.rs` — the sidecar's ONLY rmcp source contact repo-wide is the feature-gated `use rmcp as _;` anchor at `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs:20` (measured at chunk `2026-08-29-advisory-backlog`; the rmcp-macro mechanism this entry previously described was never implemented, while the wire contract is MCP-spec-conformant and proven by real-subprocess legs).
new: with name dispatch in `tools.rs`; the wire contract is MCP-spec-conformant, and no manifest and no `Cargo.lock` entry names `rmcp`.
old: Since chunk 2026-10-10-door-inside-the-engine-s-process the name dispatch is `pub fn call_tool(…) -> Result<ToolAnswer, Error>` in `tools.rs`, which writes no log record and has two callers: `dispatch_tool` — the sidecar's path, its signature and its three `mcp.*` records kept by calling `call_tool` — and `door::EngineTools`, the console engine's door, which writes `door.call` instead.
new: The name dispatch is `pub fn call_tool(…) -> Result<ToolAnswer, Error>` in `tools.rs`, which writes no log record and has one product caller, `door::EngineTools` (`pulse-app/src/door.rs`), the console engine's door, which writes `door.call`; no `mcp.*` record has an emitter in product code.
old: **The surface is no longer stdio alone:** the same nine tools and the same `jsonrpc.rs` envelope functions are served inside the console engine's process over the Unix-domain socket `run/door/sock` (§Standard Contracts → Engine door), behind neither of the sidecar's two gates; until the route entry `Stdio sidecar retired` lands, the engine has two ways in besides telemetry — the door and the stdio sidecar.
new: The nine tools and the `jsonrpc.rs` envelope functions are served inside the console engine's process over the Unix-domain socket `run/door/sock` (§Standard Contracts → Engine door), behind no feature gate; the engine has one way in besides telemetry, the door (socket and stdio relay).
old: Since chunk 2026-10-12-stdio-agent-reaches-the-door a stdio agent reaches the nine through the door with no sidecar and no feature, by starting `andromeda-pulse-engine door`
new: A stdio agent reaches the nine through the door by starting `andromeda-pulse-engine door`
old: and parses nothing — so stdio is no longer the sidecar's alone, while the ways in stay two: the relay enters through the door (§Standard Contracts → Engine door, Stdio).
new: and parses nothing — the relay enters through the door (§Standard Contracts → Engine door, Stdio).
old: `rmcp` still enters the build only under `--features mcp-server` (default off), and the feature still gates the sidecar binary; the `mcp-server` crate itself is no longer an optional dependency of `pulse-app` — it is linked in every build and the feature reads `mcp-server = ["mcp-server-crate/mcp-server"]`, with `Cargo.lock` and `deny.toml` unchanged.
new: `mcp-server` is the plain `[dependencies.mcp-server-crate]` of `pulse-app`, linked in every build, with no feature.
old: the published line reached 3.x and the workspace pins it.
new: the published line reached 3.x.

@fact id=fct-syp7tq5s expect_rev=1
old: Since chunk 2026-10-10-door-inside-the-engine-s-process that envelope has two transports: stdio (the sidecar, per MCP spec) and the console engine's door (one JSON-RPC 2.0 request per line over `run/door/sock`), which uses
new: That envelope has one carrier, the console engine's door (one JSON-RPC 2.0 request per line over `run/door/sock`), which uses
old: Since chunk 2026-10-12-stdio-agent-reaches-the-door the door is also reached over stdio through `andromeda-pulse-engine door`
new: The door is reached over stdio through `andromeda-pulse-engine door`

@fact id=fct-3k7t2g4l expect_rev=1
old: the seventh — the cross-process `andromeda-pulse-mcp` sidecar at `crates/mcp-server/src/tools.rs` — calls
new: the seventh — the `crates/mcp-server/src/tools.rs` incident-resolve write (`dispatch_mark_incident_resolved` → `update_incident_status`) — calls
old: Since chunk 2026-10-10-door-inside-the-engine-s-process that same `tools.rs` write (`dispatch_mark_incident_resolved` → `update_incident_status`) has a second carrier: the console engine's door,
new: Its one carrier is the console engine's door,
old: so the seventh writer is no longer cross-process only, it still traverses this choke point,
new: so the seventh writer traverses this choke point,

@fact id=fct-qiuibtut expect_rev=1
old: `mcp.status`, `mcp.start`, `mcp.stop` — mcp-server crate (only when `--features mcp-server`)
new: no `mcp.*` procedures — the `mcp-server` crate mounts no TauRPC router (its nine tools are served by the console engine's door, §Engine door surface)

@fact id=fct-22txvdxt expect_rev=1
old: `plugins.list`, `mcp.start`, `workspace.detect`
new: `plugins.list`, `workspace.detect`

@fact id=fct-rfsw56g3 expect_rev=1
old: `plugins`, `mcp-server`, `corpus`
new: `plugins`, `corpus`

@fact id=fct-ob4ts4xm expect_rev=1
old: JSON-RPC 2.0 over stdio per MCP spec; tool methods are
new: JSON-RPC 2.0 per MCP spec; tool methods are
old: "Over stdio per MCP spec" is the sidecar's wire; since chunk 2026-10-10-door-inside-the-engine-s-process the same nine tool methods have a second wire, the console engine's door —
new: The wire is the console engine's door —
old: Since chunk 2026-10-12-stdio-agent-reaches-the-door that wire is also reached over stdio through the relay `andromeda-pulse-engine door`, one connection per request; "over stdio per MCP spec" stays the sidecar's wire.
new: That wire is reached over stdio through the relay `andromeda-pulse-engine door`, one connection per request.

@fact id=fct-g7bqq44n expect_rev=1
old: `ANDROMEDA_PULSE_MCP_ENABLED` — `true|false` (only honored when binary built with `--features mcp-server`). When set to `true` against a binary built without the feature, startup logs a warning at `warn` level naming the missing feature flag and proceeds with MCP disabled (rather than failing to start), so a misconfigured environment variable does not block the rest of the app.
new: `ANDROMEDA_PULSE_MCP_ENABLED` — N/A: read by no program; the MCP surface has no feature gate and no switch of its own (the door's one write switch is `ANDROMEDA_PULSE_DOOR_RESOLVE`, below)
old: bounded truthy-parse per the `crates/mcp-server/src/feature_gate.rs` pattern;
new: bounded truthy-parse;

@fact id=fct-hvcuzwfp expect_rev=1
old: (`mcp-server`, `otap-ingest`); default features are minimal (no `mcp-server`, no `otap-ingest`).
new: (`otap-ingest`); default features are minimal (no `otap-ingest`).

@fact id=fct-dm325geu expect_rev=1
old: `--features mcp-server` and `--features otap-ingest` (and
new: `--features otap-ingest` (and
old: dependencies (`rmcp`, OTAP-specific crates)
new: dependencies (OTAP-specific crates)
old: That holds for `rmcp`; the workspace crate `mcp-server` is not flag-specific since chunk 2026-10-10-door-inside-the-engine-s-process —
new: The workspace crate `mcp-server` is not flag-specific —
old: calls its dispatch), and the `mcp-server` feature only forwards `mcp-server-crate/mcp-server`; `Cargo.lock` and `deny.toml` did not change.
new: calls its dispatch).

@fact id=fct-vg6rrokz expect_rev=1
old: "features": ["mcp-server"],
new: "features": [],

@fact id=fct-exoqgaup expect_rev=1
old: framing over stdio — the sidecar's carrier; the same nine are also invoked through the console engine's door (the next entry). Since chunk 2026-10-10-door-inside-the-engine-s-process every `tools/list` entry
new: framing through the console engine's door (the next entry), over `run/door/sock` or over stdio through `andromeda-pulse-engine door`. Every `tools/list` entry
old: `crates/mcp-server/src/jsonrpc.rs`), in the sidecar's own answer too — the one change the sidecar's output took at that chunk. The manifest's `additionalProperties: false` is declared and not enforced by the sidecar: the tools' argument structs read past an undeclared member (as measured at that chunk, read in the source); the door refuses one itself.
new: `crates/mcp-server/src/jsonrpc.rs`). The manifest's `additionalProperties: false` is enforced by the door's `declared_only`, the one refusal of an undeclared argument member (`-32602`); the tools' argument structs read past one (read in the source).

@fact id=fct-qsvopkhj expect_rev=1
old: `initialize` (the sidecar's `jsonrpc::initialize_result()` unchanged
new: `initialize` (`jsonrpc::initialize_result()` unchanged

@fact id=fct-4nu3wh7b expect_rev=1
old: standard MCP `initialize`, `tools/list`, `tools/call`, `notifications/*` over stdin/stdout when the `andromeda-pulse-mcp` sidecar is started; and, since chunk 2026-10-12-stdio-agent-reaches-the-door, the door's methods
new: the door's methods
old: — Unix only, no feature gate.
new: — Unix only, no feature gate, the one stdio surface.

@fact id=fct-hgj34est expect_rev=1
old: ; the sidecar stays the other stdio carrier until the route entry `Stdio sidecar retired` lands.
new: ; the relay is the one stdio carrier.

@fact id=fct-cgszvbwl expect_rev=1
old: Readers, two: the `andromeda-pulse-mcp` sidecar's `retrieve_incident_events` and, since chunk 2026-10-10-door-inside-the-engine-s-process, the same tool called through the console engine's door (`call_tool` over the engine's own `Arc<dyn CorpusWriter>`), both via
new: Readers, one: the `retrieve_incident_events` tool called through the console engine's door (`call_tool` over the engine's own `Arc<dyn CorpusWriter>`), via

@fact id=fct-ulzkjofc expect_rev=1
old: `run/workspace-key` (the resolved incident workspace key, published at boot by either engine program — the window app or the console program `andromeda-pulse-engine` — through `engine_boot::publish_workspace_key_for_sidecar` inside the shared `engine_boot::start`, at the same path, via atomic `.tmp`+rename with a canonicalize-and-confine guard and read cross-process by the `andromeda-pulse-mcp` sidecar so both filter incidents by the identity the app stamps; single line, bounded ≤ `MAX_WORKSPACE_KEY_BYTES` (4096) on read, consumed as an opaque filter string and never joined as a path; `workspace_detector::contract::{workspace_key, publish_workspace_key, read_published_workspace_key}` — chunk 2026-08-14-workspace-key-alignment), `run/door/` holding
new: `run/door/` holding
old: Written by the window app, the console program `andromeda-pulse-engine` AND the `andromeda-pulse-mcp` sidecar, all through `Corpus::open` (the two engine programs from the shared `engine_boot::start`; for the console program
new: Written by the window app and the console program `andromeda-pulse-engine`, both through `Corpus::open` from the shared `engine_boot::start` (for the console program

@fact id=fct-n5rdzzzk expect_rev=1
old: external-resolve resolves an incident through a real `andromeda-pulse-mcp` subprocess and grades on `reconciled_count` ≥ 1 with `declined_count` 0 (INCONCLUSIVE when no incident formed or the resolve did not apply).
new: external-resolve drives its resolve through an `andromeda-pulse-mcp` subprocess, a binary no workspace target builds, so it refuses at its precondition (no `andromeda-pulse-mcp` binary) and is not a runnable leg (owner: the route entry «Scenario legs driven against the console engine», CARRY 2); its grade is `reconciled_count` ≥ 1 with `declined_count` 0 (INCONCLUSIVE when no incident formed or the resolve did not apply).

@fact id=fct-ddrna4w3 expect_rev=1
old: retention used), toggle MCP server (only when `--features mcp-server` is built and the `mcp-server` crate is loaded), generate snapshot
new: retention used), generate snapshot
old: (invokes `snapshot.generate`), quit.
new: (invokes `snapshot.generate`), quit — the same menu in every build.

@fact id=fct-x56az6wm expect_rev=1
old: JSON-RPC 2.0 over stdio, implemented by the hand-rolled `crates/mcp-server/src/jsonrpc.rs` layer — since chunk 2026-10-10-door-inside-the-engine-s-process linked by `pulse-app` in every build and also serving the console engine's door over `run/door/sock`; `--features mcp-server` gates the stdio sidecar and the `rmcp` anchor, no longer the layer; since chunk 2026-10-12-stdio-agent-reaches-the-door a stdio caller also reaches the layer, through the door, by starting `andromeda-pulse-engine door` with no feature (PROVISIONAL
new: JSON-RPC 2.0 by the hand-rolled `crates/mcp-server/src/jsonrpc.rs` layer, linked by `pulse-app` in every build and serving the console engine's door over `run/door/sock`, reached over stdio through `andromeda-pulse-engine door` (PROVISIONAL
old: the door); `rmcp` rides as the feature-gated anchor dependency at requirement `"3"`, lockfile-resolved 3.1.4 as of 2026-08-29 (the old
new: the door); no feature gate, no stdio sidecar and no `rmcp` (the old

## Patterns
[[pattern]]
id = "arch1-01"
fixed = "The product binaries are three"

[[pattern]]
id = "arch1-02"
fixed = "and the sidecar below"

[[pattern]]
id = "arch1-03"
fixed = "MCP sidecar binary (when feature enabled)"

[[pattern]]
id = "arch1-04"
fixed = "gated by `--features mcp-server` as the stdio sidecar"

[[pattern]]
id = "arch1-05"
fixed = "`rmcp` linked as a feature-gated anchor dependency"

[[pattern]]
id = "arch1-06"
fixed = "`mcp-server tests` and `coverage gate` jobs"

[[pattern]]
id = "arch1-07"
fixed = "the sidecar's ONLY rmcp source contact"

[[pattern]]
id = "arch1-08"
fixed = "has two callers: `dispatch_tool`"

[[pattern]]
id = "arch1-09"
fixed = "the engine has two ways in besides telemetry"

[[pattern]]
id = "arch1-10"
fixed = "while the ways in stay two"

[[pattern]]
id = "arch1-11"
fixed = "`rmcp` still enters the build only under `--features mcp-server`"

[[pattern]]
id = "arch1-12"
fixed = "the published line reached 3.x and the workspace pins it"

[[pattern]]
id = "arch1-13"
fixed = "that envelope has two transports"

[[pattern]]
id = "arch1-14"
fixed = "the cross-process `andromeda-pulse-mcp` sidecar at `crates/mcp-server/src/tools.rs`"

[[pattern]]
id = "arch1-15"
fixed = "the seventh writer is no longer cross-process only"

[[pattern]]
id = "arch1-16"
fixed = "mcp-server crate (only when `--features mcp-server`)"

[[pattern]]
id = "arch1-17"
fixed = "`plugins.list`, `mcp.start`"

[[pattern]]
id = "arch1-18"
fixed = "`plugins`, `mcp-server`, `corpus`"

[[pattern]]
id = "arch1-19"
fixed = "over stdio per MCP spec\" is the sidecar's wire"
ignore_case = true

[[pattern]]
id = "arch1-20"
fixed = "stays the sidecar's wire"

[[pattern]]
id = "arch1-21"
fixed = "(only honored when binary built with `--features mcp-server`)"

[[pattern]]
id = "arch1-22"
fixed = "per the `crates/mcp-server/src/feature_gate.rs` pattern"

[[pattern]]
id = "arch1-23"
fixed = "(`mcp-server`, `otap-ingest`)"

[[pattern]]
id = "arch1-24"
fixed = "flag-specific dependencies (`rmcp`, OTAP-specific crates)"

[[pattern]]
id = "arch1-25"
fixed = "the `mcp-server` feature only forwards `mcp-server-crate/mcp-server`"

[[pattern]]
id = "arch1-26"
fixed = '"features": ["mcp-server"]'

[[pattern]]
id = "arch1-27"
fixed = "over stdio — the sidecar's carrier"

[[pattern]]
id = "arch1-28"
fixed = "is declared and not enforced by the sidecar"

[[pattern]]
id = "arch1-29"
fixed = "the sidecar's `jsonrpc::initialize_result()`"

[[pattern]]
id = "arch1-30"
fixed = "when the `andromeda-pulse-mcp` sidecar is started"

[[pattern]]
id = "arch1-31"
fixed = "the sidecar stays the other stdio carrier"

[[pattern]]
id = "arch1-32"
fixed = "Readers, two: the `andromeda-pulse-mcp` sidecar's"

[[pattern]]
id = "arch1-33"
fixed = "read cross-process by the `andromeda-pulse-mcp` sidecar"

[[pattern]]
id = "arch1-34"
fixed = "AND the `andromeda-pulse-mcp` sidecar, all through `Corpus::open`"

[[pattern]]
id = "arch1-35"
fixed = "resolves an incident through a real `andromeda-pulse-mcp` subprocess and grades"

[[pattern]]
id = "arch1-36"
fixed = "toggle MCP server (only when `--features mcp-server` is built"

[[pattern]]
id = "arch1-37"
fixed = "`--features mcp-server` gates the stdio sidecar and the `rmcp` anchor"

[[pattern]]
id = "arch1-38"
fixed = "`rmcp` rides as the feature-gated anchor dependency"
