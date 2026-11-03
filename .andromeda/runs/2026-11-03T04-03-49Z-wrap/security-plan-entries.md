## 2026-11-03-stdio-sidecar-retired — the engine's one way in besides telemetry, the door
**Section:** §Threat Model Summary (Attack surface: the MCP stdio surface vector, the door's vector; Infrastructure → Networking); §Input Validation (the dominant-boundaries sentence)
**Change:**
- The vector `MCP stdio surface (feature-gated)` is removed whole: the `andromeda-pulse-mcp` sidecar over stdin/stdout, its double gate, its `rmcp` anchor import and its `run/workspace-key` read.
- The door's vector: `tools/call` serves the nine tools (was "the sidecar's nine tools"). Was: the relay is "not a third way in", and until the route entry `Stdio sidecar retired` lands the engine has two ways in besides telemetry, this door and the stdio sidecar; now: the relay is not a second way in, and the engine has one way in besides telemetry, this door, reached over its socket or through the stdio relay (2 → 1).
- Networking: the clause "the stdio sidecar opens no socket" is dropped.
- The dominant boundaries of §Input Validation no longer name the MCP stdio surface.
**Why:** the chunk retires the stdio sidecar `andromeda-pulse-mcp` and its double gate. That takes the engine's ways in besides telemetry from 2 to 1: the door, over its socket and through the stdio relay.
**Kept:** the door's vector otherwise as it stood: the socket and its modes, `run` alone opening it, the four methods, the `ANDROMEDA_PULSE_DOOR_RESOLVE` write switch, and the relay `andromeda-pulse-engine door` with no listener, port, file or record of its own. The door's code is untouched by the chunk.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — the tool inputs row: one carrier, `call_tool`, no `rmcp`, `declared_only`
**Section:** §Input Validation (the MCP tool inputs row)
**Change:**
- The row was `MCP stdio inputs (mcp-server crate — …)`; it now reads `MCP tool inputs (mcp-server crate, served through the console engine's door alone — …)`.
- Name dispatch was `tools.rs::dispatch_tool`, with `rmcp` as a feature-gated anchor import; it is now `mcp_server::tools::call_tool`, whose one product caller is `door::EngineTools`, and there is no `rmcp`.
- The mitigation was `rmcp` pinned `"3"` (lockfile 3.1.4), with `setup-project` installing `rmcp = "3"`. Now `rmcp` is in no manifest and not in `Cargo.lock`.
- The undeclared argument member was an INTENDED vs MEASURED gap: not enforced on the sidecar (serde dropped the member), only the door refusing it, and the route entry `Stdio sidecar retired` owning the gap. Now the door's `declared_only` is the one refusal: `-32602` for all nine tools before any tool runs, and nothing of the member is answered (PROVISIONAL, the door's vector). The argument structs still carry no `deny_unknown_fields`. `call_tool` returns `ToolAnswer` and writes no log record. `tools/list` gives every tool a boolean `write`, true for `mark_incident_resolved` alone (was "on both carriers").
**Why:** the chunk removes `dispatch_tool` and the three records only it wrote, which leaves `call_tool` one product caller, the door. `rmcp` and `rmcp-macros` left `Cargo.lock`. The new `unit_door` test measured the refusal for all nine tools. Making `declared_only` the one refusal was the chunk's P4 answer, PROVISIONAL with the door's item. This chunk is the gap's owner entry, so the owner pointer is discharged.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — tool responses through the door alone, and the records the sidecar wrote
**Section:** §Logging & Monitoring (the prompt-injection hygiene note; What NEVER to log, the door's bullet); §Security Anti-Patterns → Logging (the `incident_events` NO-SCRUB paragraph; the lock-dir clause of the full-path ban)
**Change:**
- The hygiene note had a second carrier (the door) and a third (the relay), the note applying "as to the sidecar's client". Now tool responses reach an agent through the console engine's door alone: over its socket, or written byte for byte to standard output by `andromeda-pulse-engine door`. The note applies to an agent reading through either (PROVISIONAL, the door's vector).
- The door's NEVER-log bullet was "a tool response body on a second carrier"; it now reads "a tool response body".
- `incident_events`: the vocabulary was read by the sidecar and is now read by the `retrieve_incident_events` tool. `mark_incident_resolved` reaches `update_incident_status` through the door alone (was the cross-process sidecar and the door). The one reader is served by the door alone through `call_tool` (was the sidecar and the door). The canary absence was measured on stderr and the file sink; it now reads the file sink.
- Lock-failure records were `corpus.open.error` / `mcp.boot.corpus.init`; they are now `corpus.open.error` alone.
**Why:** the chunk retires the sidecar, its stderr-JSON sink and the `mcp.*` record chain. No `mcp.*` record has an emitter in product code any more, and `call_tool`'s one product caller is the door.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — product binaries three → two
**Section:** §Threat Model Summary (Infrastructure → Hosting; Attack surface, the CLI / env input vector's entry point); §Data Protection → At rest (the corpus; the corpus-key lock file); §Security Anti-Patterns → Input (the lock-dir carve-out; the NARROWED EXCEPTION)
**Change:**
- Hosting was "one of three product binaries" plus "so the product binaries stay three" and "the stdio sidecar `andromeda-pulse-mcp`". It now reads "one of two product binaries": the window app `pulse-app` and the console program `andromeda-pulse-engine`, with the `door` word living in that binary.
- The CLI vector and the lock-dir carve-out: `XDG_RUNTIME_DIR` is read through `Corpus::open` by both product binaries, `pulse-app` and `andromeda-pulse-engine` (was all three, `andromeda-pulse-mcp` included).
- The NARROWED EXCEPTION: `pulse-app` alone "of the two product binaries" (was "among the three").
- At rest: the processes that start together on the corpus key, and the lock file's writers, are the window app and the console program (was also the MCP sidecar).
**Why:** the chunk's count of product binaries goes from 3 to 2, because the binary `andromeda-pulse-mcp` is removed.
**Kept:** "one of THREE product-written locations outside the data dir" (the lock file, the `~/Downloads` sink, the L4 grammar temp file). It counts locations, not binaries, and is unchanged.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — the double gate and `ANDROMEDA_PULSE_MCP_ENABLED` retired
**Section:** §API Security (the MCP feature double-gate row); §Security Anti-Patterns → Code Patterns (the sidecar double-gate ban); §Logging & Monitoring → What to log (the feature-flag mismatch warning); §Input Validation (the CLI / env var row); §Threat Model Summary → Attack surface (the CLI vector's trust boundary)
**Change:**
- Removed: the API Security row `MCP feature double-gate`; the ban "NEVER spawn the `andromeda-pulse-mcp` sidecar without verifying both" gates; the What-to-log bullet on the MCP feature-flag mismatch `warn`.
- The CLI / env var row: `ANDROMEDA_PULSE_MCP_ENABLED` is dropped from its inputs. Its rule "mcp_enabled: bool with the existing graceful-degrade warning" and the sentence "mismatched MCP feature flag must continue to log at `warn` and proceed" are removed.
- The CLI vector's trust boundary: the clause "an MCP feature-flag env-var mismatch logs a warning rather than failing" is dropped.
**Why:** the chunk retires the double gate (`--features mcp-server` + `ANDROMEDA_PULSE_MCP_ENABLED`). It removes the variable from every reader and removes the cargo feature `mcp-server`. The `mcp.feature.gate.check` / `mcp.gate.check` records left with the binary.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — the `mcp.*` procedures and the Settings MCP flag
**Section:** §Threat Model Summary (Data classification, config; Attack surface, the IPC vector's entry point); §Input Validation (the TauRPC bridge row)
**Change:**
- The IPC vector's procedures drop `mcp.{status,start,stop}` (feature-gated).
- The TauRPC bridge row's routers drop `mcp.*`. `EXPECTED_PROCEDURES` was (44) and is now (41).
- The config classification's surface drops "MCP enabled flag".
**Why:** the chunk removes `mcp.status`, `mcp.start` and `mcp.stop`, their router and their three `EXPECTED_PROCEDURES` pins. It retires the Settings MCP switch and removes `Settings.mcp_server_enabled`; a `config.toml` that still holds the key still loads.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — the published workspace key retired
**Section:** §Threat Model Summary → Attack surface (the config / env / filesystem vector: entry point, trust boundary); §Input Validation (the Published workspace key row)
**Change:**
- Entry point: the `<data_dir>/run/workspace-key` clause, written by either engine program and read by the sidecar, is dropped.
- Trust boundary: the sentence on the published key's declared discipline is dropped. It now reads only `serde` + smart enum types + `TryFrom<u16>`.
- The row `Published workspace key` is removed: its bounded UTF-8 read, its write-side canonicalize-and-confine, and the value used as an opaque filter string.
**Why:** the chunk removes `engine_boot::publish_workspace_key_for_sidecar` and its call. Neither program writes `run/workspace-key` or `app.boot.workspace_key` (met at the window smoke and the engine cycle), and the key's reader left with the sidecar.
**Kept:** the `workspace_detector::contract::publish_workspace_key` precedent for the both-sides-canonicalize primitive, in the CLI / env var row and the NARROWED EXCEPTION. The library functions stay, with no product caller, until «Workspace detection retired».
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/

## 2026-11-03-stdio-sidecar-retired — ci.yml jobs six → five
**Section:** §Threat Model Summary → Infrastructure → CI/CD
**Change:**
- `ci.yml` was "six jobs on `ubuntu-22.04`". It now reads five jobs (lint-test · a11y · boot · supply-chain · coverage).
**Why:** the chunk removes the `mcp-test` job, so the job count goes from 6 to 5, pinned by `quality_gate_workflow.rs`.
**Ref:** .andromeda/runs/2026-11-03T04-03-49Z-wrap/
