# Apply — security-plan

## Verdicts
- fanout-security-plan.md 1 · refused · a hand removal · fct-gs5dp5l3
- fanout-security-plan.md 2 · applied · E1
- fanout-security-plan.md 3 · applied · E2
- fanout-security-plan.md 4 · applied · E3
- fanout-security-plan.md 5 · applied · E4
- fanout-security-plan.md 6 · applied · E5
- fanout-security-plan.md 7 · applied · E6
- fanout-security-plan.md 8 · refused · a hand removal · fct-qvdy4zh6
- fanout-security-plan.md 9 · refused · a hand removal · fct-wea2m7cj
- fanout-security-plan.md 10 · refused · a hand removal · fct-zvxze7ay
- fanout-security-plan.md 11 · applied · E7
- fanout-security-plan.md 12 · applied · E8
- fanout-security-plan.md 13 · applied · E9
- fanout-security-plan.md 14 · applied · E10
- fanout-security-plan.md 15 · applied · E11
- fanout-security-plan.md 16 · applied · E12
- fanout-security-plan.md 17 · applied · E13
- fanout-security-plan.md 18 · refused · a hand removal · fct-6e5kbvwq
- fanout-security-plan.md 19 · applied · E14
- fanout-security-plan.md 20 · applied · E15
- fanout-security-plan.md 21 · applied · E16
- fanout-security-plan.md 22 · applied · E17
- fanout-security-plan.md 23 · applied · E18
- fanout-security-plan.md 24 · applied · E19
- fanout-security-plan.md 25 · applied · E20
- fanout-security-plan.md 26 · folded into fanout-security-plan.md 21

## Edits
@fact id=fct-kmsyqgta expect_rev=1
old: `tools/call` serves the sidecar's nine tools in the engine's own process
new: `tools/call` serves the nine tools in the engine's own process
old: It is a way through the door, not a third way in. Until the route entry `Stdio sidecar retired` lands, the engine has two ways in besides telemetry: this door and the stdio sidecar.
new: It is a way through the door, not a second way in: the engine has one way in besides telemetry, this door, reached over its socket or through the stdio relay.

@fact id=fct-3433tvtn expect_rev=1
old: bound `127.0.0.1` only; the stdio sidecar opens no socket; since chunk
new: bound `127.0.0.1` only; since chunk

@fact id=fct-w6v5n2bs expect_rev=1
old: the WASM plugin host, the MCP stdio surface, the console engine's door socket
new: the WASM plugin host, the console engine's door socket

@fact id=fct-6geshnvr expect_rev=1
old: | MCP stdio inputs (`mcp-server` crate — all 9 tools dispatched by name:
new: | MCP tool inputs (`mcp-server` crate, served through the console engine's door alone — all 9 tools dispatched by name:
old: name-dispatch via `tools.rs::dispatch_tool`; `rmcp` present only as the feature-gated anchor import `use rmcp as _;` — measured chunk `2026-08-29-advisory-backlog`); tool argument structs via `serde`
new: name-dispatch via `mcp_server::tools::call_tool`, whose one product caller is `door::EngineTools`; no `rmcp`); tool argument structs via `serde`
old: | `rmcp` pinned `"3"` (lockfile-resolved 3.1.4) — the former 0.3.x-pending-reconciliation instruction is RETIRED, the reconciliation CLOSED by measurement at chunk `2026-08-29-advisory-backlog`; `setup-project` installs `rmcp = "3"` and carries no open question. Tool argument structs use `serde`;
new: | No `rmcp`: it is in no manifest and not in `Cargo.lock`. Tool argument structs use `serde`;
old: **INTENDED vs MEASURED — an undeclared argument member (as measured at chunk `2026-10-10-door-inside-the-engine-s-process`, read in the source):** the tool manifest declares `additionalProperties: false` on every tool, and that declaration is NOT enforced on the sidecar — the argument structs (`QueryTracesArgs`, `QueryMetricsArgs`, `QueryLogsArgs`, `GenerateSnapshotArgs`, `IncidentIdArgs`) carry no `deny_unknown_fields` and read past an unknown member, and `dispatch_query_incident_list` parses no arguments at all. Consequence: the sidecar accepts a call carrying an undeclared member; the member is dropped by `serde` and reaches no query, no path and no log. Only the console engine's door refuses one (the next row). No fix is built: the sidecar leaves at the route entry `Stdio sidecar retired`, minted at that chunk's wrap, and that entry owns this gap (the operator's word, the pc overseer, 2026-10-11). Since that chunk the name dispatch is `call_tool` (it returns `ToolAnswer` and writes no log record); `dispatch_tool` keeps its signature and its three records by calling it, and `call_tool` has a second caller, the door's `door::EngineTools`; `tools/list` gives every tool a boolean `write`, true for `mark_incident_resolved` alone, on both carriers.
new: **An undeclared argument member:** the tool manifest declares `additionalProperties: false` on every tool, and the door's `declared_only` is the one refusal of it — `-32602` for all nine tools before any tool runs, nothing of it answered (the next row; PROVISIONAL — §Threat Model Summary → Attack surface, the door's vector); the argument structs (`QueryTracesArgs`, `QueryMetricsArgs`, `QueryLogsArgs`, `GenerateSnapshotArgs`, `IncidentIdArgs`) carry no `deny_unknown_fields`. `call_tool` returns `ToolAnswer` and writes no log record; `tools/list` gives every tool a boolean `write`, true for `mark_incident_resolved` alone.

@fact id=fct-cvhwrox6 expect_rev=1
old: Since chunk `2026-10-10-door-inside-the-engine-s-process` tool responses have a second carrier: the console engine's door returns the same `query_*`
new: Tool responses reach an agent through the console engine's door alone: the door returns the `query_*`
old: never cut), and the note applies to an agent reading through the door as it does to a stdio client (PROVISIONAL — §Threat Model Summary → Attack surface, the door's vector). Since chunk `2026-10-12-stdio-agent-reaches-the-door` a third carrier: `andromeda-pulse-engine door` writes the door's answers unchanged, byte for byte, to the standard output of the process that started it, unsanitized OTLP-derived content included, and the note applies to a stdio agent through the relay as to the sidecar's client.
new: never cut), and `andromeda-pulse-engine door` writes them unchanged, byte for byte, to the standard output of the process that started it; the note applies to an agent reading through either (PROVISIONAL — §Threat Model Summary → Attack surface, the door's vector).

@fact id=fct-hjatqh7q expect_rev=1
old: (a tool response body on a second carrier)
new: (a tool response body)

@fact id=fct-ty7icrgc expect_rev=1
old: `ANDROMEDA_PULSE_LOG_LEVEL`, `ANDROMEDA_PULSE_MCP_ENABLED`, `ANDROMEDA_PULSE_L4_DETERMINISTIC`
new: `ANDROMEDA_PULSE_LOG_LEVEL`, `ANDROMEDA_PULSE_L4_DETERMINISTIC`
old: log level: enum parse; mcp_enabled: bool with the existing graceful-degrade warning when feature not built; l4_deterministic:
new: log level: enum parse; l4_deterministic:
old: prefix). Per Occupied Resources, mismatched MCP feature flag must continue to log at `warn` and proceed (do not regress this graceful-degrade behavior). —
new: prefix). —

@fact id=fct-j4oj6ftt expect_rev=1
old: read by all three product binaries — `pulse-app`, `andromeda-pulse-engine`, `andromeda-pulse-mcp` — through `Corpus::open`
new: read by both product binaries — `pulse-app` and `andromeda-pulse-engine` — through `Corpus::open`
old: creating nothing (§Input Validation); an MCP feature-flag env-var mismatch logs a warning rather than failing.
new: creating nothing (§Input Validation).

@fact id=fct-zjnqq4og expect_rev=1
old: as one of three product binaries: the window app `pulse-app` (the Tauri process); the console program
new: as one of two product binaries: the window app `pulse-app` (the Tauri process) and the console program
old: the word lives in this binary, so the product binaries stay three); and the stdio sidecar `andromeda-pulse-mcp`. How
new: the word lives in this binary). How

@fact id=fct-ixrmtmoe expect_rev=1
old: all three product binaries (`pulse-app`, `andromeda-pulse-engine`, `andromeda-pulse-mcp`) read
new: both product binaries (`pulse-app`, `andromeda-pulse-engine`) read

@fact id=fct-tg5z4vbe expect_rev=1
old: alone among the three product binaries:
new: alone of the two product binaries:

@fact id=fct-bglfrqeg expect_rev=1
old: written by the window app, the console program `andromeda-pulse-engine` and the MCP sidecar through `Corpus::open`
new: written by the window app and the console program `andromeda-pulse-engine` through `Corpus::open`

@fact id=fct-rofiv5jh expect_rev=1
old: (the window app, the console program `andromeda-pulse-engine` and the MCP sidecar, or several instances)
new: (the window app and the console program `andromeda-pulse-engine`, or several instances)

@fact id=fct-ausvrcpe expect_rev=1
old: (`workspace.detect`, `workspace.list`); `<data_dir>/run/workspace-key`, written by whichever engine program boots — the window app or the console program `andromeda-pulse-engine`, through the shared engine boot — and read by the sidecar (chunk 2026-08-14-workspace-key-alignment).
new: (`workspace.detect`, `workspace.list`).
old: `TryFrom<u16>`. For the published workspace key the discipline IS declared (§Input Validation): canonicalize-and-confine on write, bounded + UTF-8 + control-char-rejecting read, and the value used only as an opaque filter string, never as a path.
new: `TryFrom<u16>`.

@fact id=fct-f2fvtxyp expect_rev=1
old: `plugins.{list,reload,invoke}`, `mcp.{status,start,stop}` (feature-gated), `workspace.{detect,list}`
new: `plugins.{list,reload,invoke}`, `workspace.{detect,list}`

@fact id=fct-3pov4uwe expect_rev=1
old: `plugins.*`, `mcp.*`, `workspace.*`
new: `plugins.*`, `workspace.*`
old: pinned in `EXPECTED_PROCEDURES` (44)
new: pinned in `EXPECTED_PROCEDURES` (41)

@fact id=fct-chk2itue expect_rev=1
old: log level, MCP enabled flag, plugin dir
new: log level, plugin dir

@fact id=fct-dspgzb7t expect_rev=1
old: six jobs on `ubuntu-22.04`
new: five jobs (lint-test · a11y · boot · supply-chain · coverage) on `ubuntu-22.04`

@fact id=fct-odqor6ow expect_rev=1
old: `corpus.open.error` / `mcp.boot.corpus.init` records
new: `corpus.open.error` record

@fact id=fct-y6svlsyp expect_rev=1
old: read by the sidecar, never client text
new: read by the `retrieve_incident_events` tool, never client text
old: the cross-process MCP sidecar's `mark_incident_resolved` included and, since chunk `2026-10-10-door-inside-the-engine-s-process`, the same tool called through the console engine's door in the engine's own process
new: `mark_incident_resolved` included, called through the console engine's door alone in the engine's own process
old: served by the sidecar and, since that chunk, by the door through `call_tool`
new: served by the door alone through `call_tool`
old: (stderr and file-sink canary absence measured)
new: (file-sink canary absence measured)

## Patterns
[[pattern]]
id = "sec-01"
fixed = "serves the sidecar's nine tools"

[[pattern]]
id = "sec-02"
fixed = "two ways in besides telemetry"

[[pattern]]
id = "sec-03"
fixed = "the stdio sidecar opens no socket"

[[pattern]]
id = "sec-04"
fixed = "plugin host, the MCP stdio surface"

[[pattern]]
id = "sec-05"
regex = 'name-dispatch via `?tools\.rs::dispatch_tool'

[[pattern]]
id = "sec-06"
fixed = 'installs `rmcp = "3"`'

[[pattern]]
id = "sec-07"
fixed = "NOT enforced on the sidecar"

[[pattern]]
id = "sec-08"
fixed = "as to the sidecar's client"

[[pattern]]
id = "sec-09"
fixed = "on a second carrier"

[[pattern]]
id = "sec-10"
fixed = "mcp_enabled: bool with the existing graceful-degrade warning"

[[pattern]]
id = "sec-11"
fixed = "mismatched MCP feature flag must continue to log"
ignore_case = true

[[pattern]]
id = "sec-12"
fixed = "feature-flag env-var mismatch logs a warning"

[[pattern]]
id = "sec-13"
regex = '(all|the) three product binaries|product binaries stay three|one of three product binaries'

[[pattern]]
id = "sec-14"
regex = 'and the MCP sidecar(,| through `?Corpus::open)'

[[pattern]]
id = "sec-15"
fixed = "and read by the sidecar (chunk 2026-08-14-workspace-key-alignment)"

[[pattern]]
id = "sec-16"
fixed = "For the published workspace key the discipline IS declared"

[[pattern]]
id = "sec-17"
fixed = "`mcp.{status,start,stop}` (feature-gated)"

[[pattern]]
id = "sec-18"
fixed = "`plugins.*`, `mcp.*`"

[[pattern]]
id = "sec-19"
fixed = "MCP enabled flag"
ignore_case = true

[[pattern]]
id = "sec-20"
fixed = "six jobs on `ubuntu-22.04`"

[[pattern]]
id = "sec-21"
fixed = "`mcp.boot.corpus.init` records"

[[pattern]]
id = "sec-22"
regex = 'read by the sidecar, never client text|served by the sidecar and'

[[pattern]]
id = "sec-25"
fixed = "the cross-process MCP sidecar's `mark_incident_resolved` included"

[[pattern]]
id = "sec-23"
fixed = "stderr and file-sink canary absence"

[[pattern]]
id = "sec-24"
fixed = "`EXPECTED_PROCEDURES` (44)"
