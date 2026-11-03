proposals:
  - detector: D-retired
    severity: warning
    section: §Threat Model Summary → Attack surface
    change: "Remove the vector `MCP stdio surface (feature-gated)` — the `andromeda-pulse-mcp` sidecar, its double gate (`--features mcp-server` + `ANDROMEDA_PULSE_MCP_ENABLED`), its `rmcp` anchor import and its `run/workspace-key` read are retired; a stdio agent reaches the nine tools only through `andromeda-pulse-engine door`, the relay to the console engine's door (the door's vector, PROVISIONAL), and `mcp-server` stays the tools library the door serves, a plain dependency of `pulse-app`."
    sidecar: "Attack surface — the MCP stdio sidecar vector removed: the sidecar, its double gate and its workspace-key read retired at chunk 2026-11-03-stdio-sidecar-retired; stdio reaches the nine tools only through the door relay."
    rationale: "The report's Retired bullet names the stdio sidecar `andromeda-pulse-mcp`, its double gate and the `run/workspace-key` publish; Symbols removes the binary; Dependencies removes `rmcp`; Crates makes `mcp-server` a plain dependency of `pulse-app`."
    basis: "security-plan fct-gs5dp5l3; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc; doc:2026-11-03-stdio-sidecar-retired/report fct-yabevdww; doc:2026-11-03-stdio-sidecar-retired/report fct-yn7uwxjo"
  - detector: D-retired
    severity: warning
    section: §Threat Model Summary → Attack surface
    change: "In the door's vector, `tools/call` serves the nine tools (not the sidecar's nine tools), and the closing sentence reads — the engine has one way in besides telemetry, this door, reached over its socket or through the stdio relay `andromeda-pulse-engine door`; the stdio sidecar was retired at chunk `2026-11-03-stdio-sidecar-retired`."
    sidecar: "Attack surface, the door's vector — the engine's ways in besides telemetry 2 → 1 (the door: socket and stdio relay); the sidecar retired."
    rationale: "The report's Counts bullet moves engine ways in besides telemetry 2 → 1, the door (socket and stdio relay) (`two-ways` pattern); the Retired bullet names the sidecar."
    basis: "security-plan fct-kmsyqgta; doc:2026-11-03-stdio-sidecar-retired/report fct-wnpvzb4g; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Threat Model Summary → Infrastructure
    change: "Networking reads — local only: OTLP receivers bound `127.0.0.1` only; the console engine serves the nine tools over a Unix-domain socket inside the data dir (`run/door/sock`), own host only, no TCP or network listener; updater outbound HTTPS to GitHub Releases; no inbound internet exposure — dropping `the stdio sidecar opens no socket`."
    sidecar: "Infrastructure → Networking — the stdio sidecar clause dropped (sidecar retired)."
    rationale: "The report's Retired bullet names the stdio sidecar; Symbols removes the `andromeda-pulse-mcp` binary."
    basis: "security-plan fct-3433tvtn; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Input Validation
    change: "The Boundary sentence's dominant boundaries drop `the MCP stdio surface` — the console engine's door socket (reached directly or through the stdio relay `andromeda-pulse-engine door`) is the one tool-serving boundary."
    sidecar: "Input Validation boundary list — the MCP stdio surface dropped (sidecar retired)."
    rationale: "The report's Retired bullet names the stdio sidecar; Symbols states `call_tool` has one product caller left, `pulse-app/src/door.rs`."
    basis: "security-plan fct-w6v5n2bs; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Input Validation
    change: "Rewrite the MCP stdio inputs row as the nine tools' inputs served through the console engine's door alone — name-dispatch via `mcp_server::tools::call_tool`, whose one product caller is `door::EngineTools` (`dispatch_tool` and its three records removed); no `rmcp` (it left the workspace and `Cargo.lock`); the undeclared-member gap closed with the sidecar — the door's `declared_only` is the one refusal of an undeclared argument member, `-32602` for all nine tools (`inputs#I2`, PROVISIONAL); `tools/list`'s boolean `write` on the one carrier; drop the sidecar-only INTENDED vs MEASURED paragraph and the `rmcp` pin / setup-project install clause."
    sidecar: "Input Validation, MCP inputs row — sidecar carrier, `dispatch_tool`, `rmcp` pin and the sidecar's undeclared-member gap retired; `declared_only` the one refusal (PROVISIONAL)."
    rationale: "Report Symbols removes `dispatch_tool` and its three records and names `declared_only` the one refusal (`inputs#I2`); Dependencies removes `rmcp`; Outcome meets the undeclared-member refusal for all nine tools; Retired names the sidecar."
    basis: "security-plan fct-6geshnvr; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc; doc:2026-11-03-stdio-sidecar-retired/report fct-yabevdww; doc:2026-11-03-stdio-sidecar-retired/report fct-lmgi5jdu; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Logging & Monitoring
    change: "The hygiene note's carriers read — tool responses reach an agent through the console engine's door, over its socket or written byte for byte to standard output by `andromeda-pulse-engine door`, unsanitized OTLP-derived content included; drop the sidecar's client as a carrier (the stdio sidecar retired)."
    sidecar: "Logging & Monitoring hygiene note — the sidecar's client dropped as a tool-response carrier; the door and its stdio relay remain."
    rationale: "The report's Retired bullet names the stdio sidecar; Symbols states `call_tool` has one product caller left, the door."
    basis: "security-plan fct-cvhwrox6; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Logging & Monitoring
    change: "The door's NEVER-log bullet reads `a call's arguments and its answer (a tool response body)` — dropping `on a second carrier`, the door now being the tools' one carrier."
    sidecar: "Logging & Monitoring → What NEVER to log — `second carrier` dropped (the sidecar carrier retired)."
    rationale: "The report's Retired bullet names the stdio sidecar, the first carrier this ordinal counted from."
    basis: "security-plan fct-hjatqh7q; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §API Security
    change: "Remove the `MCP feature double-gate` row — the sidecar and its gate (cargo feature `mcp-server`, `ANDROMEDA_PULSE_MCP_ENABLED`) are retired and no reader of the variable remains; the nine tools' controls are the door's (owner-only socket and peer-account check, request bounds, the `ANDROMEDA_PULSE_DOOR_RESOLVE` switch; §Input Validation, §Authentication & Authorization, PROVISIONAL)."
    sidecar: "API Security — the MCP feature double-gate row removed with the sidecar and its gate."
    rationale: "The report's Retired bullet names the double gate (`--features mcp-server` + `ANDROMEDA_PULSE_MCP_ENABLED`); Symbols removes the env var from every reader; Schema removes the cargo feature `mcp-server`."
    basis: "security-plan fct-qvdy4zh6; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc; doc:2026-11-03-stdio-sidecar-retired/report fct-xsrenprx"
  - detector: D-retired
    severity: warning
    section: §Security Anti-Patterns → Code Patterns
    change: "Remove the ban `NEVER spawn the andromeda-pulse-mcp sidecar without verifying both` gates — the sidecar, its feature and its env var are retired."
    sidecar: "Anti-Patterns → Code Patterns — the sidecar double-gate ban removed (sidecar and gate retired)."
    rationale: "The report's Retired bullet names the sidecar and its double gate."
    basis: "security-plan fct-wea2m7cj; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Logging & Monitoring
    change: "Remove the What-to-log bullet `MCP feature flag mismatch warning` — the feature and `ANDROMEDA_PULSE_MCP_ENABLED` are gone, so no mismatch and no graceful-degrade warn exist."
    sidecar: "Logging & Monitoring → What to log — the MCP feature-flag mismatch warning removed (gate retired)."
    rationale: "The report's Retired bullet names the double gate; Symbols removes `ANDROMEDA_PULSE_MCP_ENABLED` from every reader and the `mcp.feature.gate.check` / `mcp.gate.check` records left with `tracing_setup.rs`."
    basis: "security-plan fct-zvxze7ay; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Input Validation
    change: "In the CLI / env var row, drop `ANDROMEDA_PULSE_MCP_ENABLED` from the boundary list, the `mcp_enabled: bool with the existing graceful-degrade warning` clause, and the sentence `Per Occupied Resources, mismatched MCP feature flag must continue to log at warn and proceed` — the variable has no reader."
    sidecar: "Input Validation, CLI / env var row — `ANDROMEDA_PULSE_MCP_ENABLED` and its graceful-degrade rule removed (no reader remains)."
    rationale: "Report Symbols — env var removed from every reader: `ANDROMEDA_PULSE_MCP_ENABLED`; Retired names the double gate."
    basis: "security-plan fct-ty7icrgc; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Threat Model Summary → Attack surface
    change: "In the CLI input vector, `XDG_RUNTIME_DIR` is read by both product binaries — `pulse-app` and `andromeda-pulse-engine` — through `Corpus::open`, and the trust-boundary clause `an MCP feature-flag env-var mismatch logs a warning rather than failing` is dropped."
    sidecar: "Attack surface → CLI input — product binaries three → two; the MCP feature-flag mismatch clause dropped."
    rationale: "The report's Counts bullet moves product binaries 3 → 2 (`bin-count`); Symbols removes `ANDROMEDA_PULSE_MCP_ENABLED` from every reader; Retired names the sidecar and its gate."
    basis: "security-plan fct-j4oj6ftt; doc:2026-11-03-stdio-sidecar-retired/report fct-wnpvzb4g; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Threat Model Summary → Infrastructure
    change: "Hosting reads — runs as one of two product binaries, the window app `pulse-app` and the console program `andromeda-pulse-engine` (`run` · `version` · `door`; the `door` word lives in this binary); drop `so the product binaries stay three` and `and the stdio sidecar andromeda-pulse-mcp`."
    sidecar: "Infrastructure → Hosting — product binaries 3 → 2; the stdio sidecar retired."
    rationale: "Report Symbols — the product binaries are now two, `pulse-app` and `andromeda-pulse-engine` (console: run · version · door); Counts moves product binaries 3 → 2."
    basis: "security-plan fct-zjnqq4og; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc; doc:2026-11-03-stdio-sidecar-retired/report fct-wnpvzb4g"
  - detector: D-retired
    severity: warning
    section: §Security Anti-Patterns → Input
    change: "The corpus-key lock-dir carve-out reads — both product binaries (`pulse-app`, `andromeda-pulse-engine`) read the system `XDG_RUNTIME_DIR` (Linux) through `Corpus::open` — dropping `andromeda-pulse-mcp`."
    sidecar: "Anti-Patterns → Input, lock-dir carve-out — product binaries three → two."
    rationale: "The report's Counts bullet moves product binaries 3 → 2 (`bin-count`, a security-plan row in the anti-patterns); Symbols removes the binary."
    basis: "security-plan fct-ixrmtmoe; doc:2026-11-03-stdio-sidecar-retired/report fct-wnpvzb4g; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Security Anti-Patterns → Input
    change: "The NARROWED EXCEPTION reads the window app `pulse-app` alone of the two product binaries (not among the three)."
    sidecar: "Anti-Patterns → Input, narrowed exception — product binaries three → two."
    rationale: "The report's Counts bullet moves product binaries 3 → 2 (`bin-count`)."
    basis: "security-plan fct-tg5z4vbe; doc:2026-11-03-stdio-sidecar-retired/report fct-wnpvzb4g"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Data Protection → At rest
    change: "The corpus-key lock file is written by the window app and the console program `andromeda-pulse-engine` through `Corpus::open` — dropping the MCP sidecar."
    sidecar: "Data Protection → At rest, lock file — the MCP sidecar dropped from its writers (sidecar retired)."
    rationale: "The report's Retired bullet names the stdio sidecar; Symbols removes the `andromeda-pulse-mcp` binary."
    basis: "security-plan fct-bglfrqeg; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Data Protection → At rest
    change: "The corpus bullet's concurrently starting processes read (the window app and the console program `andromeda-pulse-engine`, or several instances) — dropping the MCP sidecar."
    sidecar: "Data Protection → At rest, corpus — the MCP sidecar dropped from the processes sharing the key (sidecar retired)."
    rationale: "The report's Retired bullet names the stdio sidecar; Symbols removes the binary."
    basis: "security-plan fct-rofiv5jh; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Input Validation
    change: "Remove the `Published workspace key` row — neither program writes `{data dir}/run/workspace-key` (`engine_boot::publish_workspace_key_for_sidecar` and its call removed) and no reader remains; `workspace_detector::contract::{publish_workspace_key, read_published_workspace_key}` stay in the library with no product caller."
    sidecar: "Input Validation — the published workspace key row removed: the `run/workspace-key` publish and its sidecar reader retired."
    rationale: "The report's Retired bullet names the `run/workspace-key` publish; Symbols removes `publish_workspace_key_for_sidecar` and states the library functions stay with no product caller; Outcome (obs) meets that neither program writes it."
    basis: "security-plan fct-6e5kbvwq; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc; doc:2026-11-03-stdio-sidecar-retired/report fct-dv3watly"
  - detector: D-retired
    severity: warning
    section: §Threat Model Summary → Attack surface
    change: "In the filesystem-reads vector, drop the `<data_dir>/run/workspace-key` entry point (no program writes or reads it) and the trust-boundary sentence on the published workspace key."
    sidecar: "Attack surface → filesystem reads — the published workspace key dropped (publish and reader retired)."
    rationale: "The report's Retired bullet names the `run/workspace-key` publish; Symbols — neither program writes `{data dir}/run/workspace-key`."
    basis: "security-plan fct-ausvrcpe; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Threat Model Summary → Attack surface
    change: "The IPC vector's procedure list drops `mcp.{status,start,stop}` (feature-gated)."
    sidecar: "Attack surface → IPC — the `mcp.*` TauRPC procedures removed."
    rationale: "The report's Retired bullet names the `mcp.*` TauRPC procedures; Symbols removes `mcp.status`, `mcp.start`, `mcp.stop` and their `EXPECTED_PROCEDURES` pins."
    basis: "security-plan fct-f2fvtxyp; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc"
  - detector: D-retired
    severity: warning
    section: §Input Validation
    change: "The TauRPC bridge row's router list drops `mcp.*`."
    sidecar: "Input Validation, TauRPC row — `mcp.*` dropped from the routers."
    rationale: "The report's Retired bullet names the `mcp.*` TauRPC procedures; Symbols removes the router `pulse-app/src/mcp_router.rs`."
    basis: "security-plan fct-3pov4uwe; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Threat Model Summary → Data classification
    change: "The config classification's surface drops `MCP enabled flag` — enum settings, port ranges, file paths, retention seconds, log level, plugin dir."
    sidecar: "Data classification → config — the MCP enabled flag dropped (Settings MCP switch retired)."
    rationale: "The report's Retired bullet names the Settings MCP switch; Schema removes `Settings.mcp_server_enabled` (an existing `config.toml` holding the key still loads)."
    basis: "security-plan fct-chk2itue; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-xsrenprx"
  - detector: D-retired
    severity: warning
    section: §Threat Model Summary → Infrastructure
    change: "CI/CD reads `ci.yml` with five jobs on `ubuntu-22.04` (lint-test · a11y · boot · supply-chain · coverage), not six."
    sidecar: "Infrastructure → CI/CD — ci.yml jobs 6 → 5 (the `mcp-test` job retired)."
    rationale: "The report's Retired bullet names the `mcp-test` CI job; Counts moves ci.yml jobs 6 → 5 (`mcp-test-job`, a security-plan row); Harness removes the job."
    basis: "security-plan fct-dspgzb7t; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-wnpvzb4g; doc:2026-11-03-stdio-sidecar-retired/report fct-vyk3olls"
  - detector: D-security-logging
    severity: warning
    section: §Security Anti-Patterns → Logging
    change: "The lock-dir clause of the full-path ban reads — a lock-dir, open or lock failure surfaces only as the unit `KeychainError::Unavailable` / `Failed` → `KeyringUnavailable` on the existing `corpus.open.error` record — dropping `mcp.boot.corpus.init`, which left with the sidecar."
    sidecar: "Anti-Patterns → Logging — `mcp.boot.corpus.init` dropped from the lock-failure records (no `mcp.*` record has a product emitter)."
    rationale: "Report Symbols — no `mcp.*` record has an emitter in product code any more; the sidecar's records left with the binary and `tracing_setup.rs`."
    basis: "security-plan fct-odqor6ow; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc"
  - detector: D-security-logging
    severity: warning
    section: §Security Anti-Patterns → Logging
    change: "The `incident_events` NO-SCRUB paragraph reads — the vocabulary is written by the producer and read by the `retrieve_incident_events` tool; the status writers reaching `update_incident_status` include `mark_incident_resolved` through the console engine's door only (the cross-process sidecar caller retired); the table's one production reader is served by the door alone through `call_tool`, whose `door.call` record carries no answer; drop the sidecar's stderr sink from the canary-absence claim."
    sidecar: "Anti-Patterns → Logging, incident_events boundary — the sidecar dropped as writer, reader and stderr-sink carrier; the door the one caller."
    rationale: "The report's Retired bullet names the stdio sidecar and its stderr-JSON sink; Symbols — `call_tool` has one product caller left, `pulse-app/src/door.rs`."
    basis: "security-plan fct-y6svlsyp; doc:2026-11-03-stdio-sidecar-retired/report fct-wvc3ru2y; doc:2026-11-03-stdio-sidecar-retired/report fct-z7lqgxlc"
note: "Not proposed — the `workspace_detector::contract::publish_workspace_key` precedent named in fct-mlwuy3m4, fct-ty7icrgc, fct-4wywx7zd, fct-tg5z4vbe stays as library code with no product caller (report fct-z7lqgxlc), so the precedent wording was left; fct-vlpnzd6u's rmcp pin item (b) is history per the report's exception (b); fct-3pov4uwe's `EXPECTED_PROCEDURES (44)` figure was not touched because the report gives only three fewer, not a total. D-security-input, D-security-auth, D-security-deps — no new input surface, auth change or added dependency."
