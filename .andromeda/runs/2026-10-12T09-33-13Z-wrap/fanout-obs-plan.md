proposals:
  - detector: D-obs-instrumentation
    severity: warning
    section: §1 Obs Scope Summary → surface table, CLI (command-line entrypoint) row
    change: 'The console program grammar is closed at `run`, `version` or `door` (was `run` or `version`); `door` (the stdio relay, `pulse_app::door_bridge`, chunk 2026-10-12-stdio-agent-reaches-the-door) builds no tokio runtime, calls neither `engine_boot::init_process` nor `engine_boot::start`, installs no sink and writes no record, no file and no pid file, like `version`; its calls are recorded only by the engine it reaches (`door.call` / `door.refused`, §4 Scenario P3); its stdout carries only the door''s answer bytes and its stderr only the fixed line `andromeda-pulse-engine door: the door cannot be reached` on exit 1. PROVISIONAL with the door''s one item.'
    sidecar: 'obs-plan §1 CLI row: console grammar three words (`run` | `version` | `door`); `door` is the stdio relay, writes no record and creates nothing; its calls are recorded by the engine''s door (chunk 2026-10-12-stdio-agent-reaches-the-door; PROVISIONAL with the door''s item).'
    rationale: 'Report Changes → Symbols / APIs: `console::Command::Door` joins `Run` · `Version`, `console::USAGE` = `<run|version|door>`, `door` builds no runtime and calls neither boot function; What it adds: no log record, no file, no pid file (grep over door_bridge.rs for init_process|observability|tracing:: reads 0); Counts / qualifiers moved names obs-plan.md:67 ("grammar closed at `run` or `version`") as a row stating the closed set; Expected amendment 12. Instrumentation n/a by design (Coverage of new surfaces) — the plan must state the new word''s no-record posture rather than imply only `version` and a refusal write none.'
    basis: .andromeda/obs-plan.md:67
  - detector: D-obs-instrumentation
    severity: warning
    section: §4 Span / Trace Coverage → surface table, CLI / main binary row
    change: 'After "`version` and a refused command line emit nothing" add `door` (the stdio relay, chunk 2026-10-12-stdio-agent-reaches-the-door): it emits nothing of its own and opens no sink; each request it relays is recorded by the running console engine as `door.call` (one per answered request), a refused account reaching the door as `door.refused`.'
    sidecar: 'obs-plan §4 CLI / main binary row: the console program''s `door` word emits nothing of its own; relayed calls are the engine''s `door.call` / `door.refused` (chunk 2026-10-12-stdio-agent-reaches-the-door).'
    rationale: 'Same retired claim as the §1 CLI row (the console program''s word set and which words write no record); report Symbols / APIs and What it adds (no log record, no runtime, no boot call); Expected amendment 12 names obs-plan.md:192 as the second `cli-row` site.'
    basis: .andromeda/obs-plan.md:192
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §4 Span / Trace Coverage → Scenario P3 (Two carriers)
    change: 'The door carrier is reached two ways: a client of `{data dir}/run/door/sock` directly, or a stdio agent through `andromeda-pulse-engine door` (the relay; one connection per request line, the write side half-closed after it, the whole answer read then written to stdout unchanged). Through the relay each answered request leaves one `door.call` in the engine''s log, a notification none, and no `door.refused {reason: idle}`; another account''s relay reading that reaches the door leaves one `door.refused {other_account, other}` and the relay exits 1 with its fixed stderr line and 0 stdout bytes; the relay writes no record itself. PROVISIONAL with the door''s one item.'
    sidecar: 'obs-plan §4 Scenario P3: a stdio agent reaches the door carrier through `andromeda-pulse-engine door`; one `door.call` per answered request, none for a notification, no idle refusal; a refused other account leaves `door.refused` and the relay exits 1 (chunk 2026-10-12-stdio-agent-reaches-the-door; PROVISIONAL).'
    rationale: 'Report Symbols / APIs → How the door treats the relay ("one `door.call` per answered request; a notification ... recorded with nothing; one connection per request means no `door.refused {reason: idle}`") and Other-account behaviour, measured (exit 1, fixed line, nothing on stdout); Outcome criterion 8 (five answered requests, five records) and criterion 5 (one `door.refused` per reading that reached the door, 3 asserted on ci#38227488889); Expected amendment 13. Scenario P3 states the stdio way as the sidecar''s alone ("Everything above is the first, the stdio sidecar"), the door reached only over the socket.'
    basis: .andromeda/obs-plan.md:223
  - detector: D-obs-instrumentation
    severity: warning
    section: §1 Obs Scope Summary → must-trace table, P3 MCP server query row
    change: 'Where the row names the door as the second carrier ("a Unix-domain socket, console program only"), add that a stdio agent reaches it through `andromeda-pulse-engine door` (the relay, no `mcp.*` span, no double gate, no record of its own), its must-log record still `door.call` — §4 Scenario P3.'
    sidecar: 'obs-plan §1 P3 row: the door carrier is also reached over stdio through the `door` relay; must-log record unchanged, `door.call` (chunk 2026-10-12-stdio-agent-reaches-the-door).'
    rationale: 'Restates the Scenario P3 carrier claim (stdio is the sidecar''s way, the door a socket); report Symbols / APIs (the relay) and Expected amendment 4 (stdio no longer the sidecar''s alone; `mcp-stdio` obs-plan 7 sites, the detectors'' to judge) and 13 (`scenario-p3` hit obs-plan.md:109).'
    basis: .andromeda/obs-plan.md:109
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §4 Span / Trace Coverage → Scenario P3, Surfaces involved
    change: 'Surfaces involved: after "also the console engine''s door (the second carrier, below)" add "reached over its socket directly or by a stdio agent through `andromeda-pulse-engine door` (chunk 2026-10-12-stdio-agent-reaches-the-door)".'
    sidecar: 'obs-plan §4 Scenario P3 Surfaces involved: the door carrier is reached through the stdio relay too (chunk 2026-10-12-stdio-agent-reaches-the-door).'
    rationale: 'Same carrier claim as the Scenario P3 Two-carriers paragraph: the bullet ties "hand-rolled JSON-RPC stdio" to the CLI/sidecar and the door to its socket alone; report Symbols / APIs (the relay a stdio agent starts).'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §3 Observability Harness Contract → Snapshot / paste-to-AI integration
    change: 'MCP tools label: replace "\"Over stdio\" in this label is the sidecar''s transport; the door''s is a Unix-domain socket" with: over stdio is the sidecar''s transport and, since chunk 2026-10-12-stdio-agent-reaches-the-door, also the `andromeda-pulse-engine door` relay''s, which carries each line to the door''s Unix-domain socket; calls through it are recorded as `door.call` only (obs-plan §4 Scenario P3).'
    sidecar: 'obs-plan §3 Snapshot / paste-to-AI integration: "over stdio" is no longer the sidecar''s alone; the `door` relay carries stdio to the door socket, recorded as `door.call` (chunk 2026-10-12-stdio-agent-reaches-the-door).'
    rationale: 'The key file states stdio as the sidecar''s transport alone and the door''s as the socket; report Symbols / APIs (the relay) and Expected amendment 13 (`scenario-p3` hit registries/contracts/obs-plan/snapshot-paste-to-ai-integration.md:9) and 4 (`mcp-stdio`).'
    basis: .andromeda/registries/contracts/obs-plan/snapshot-paste-to-ai-integration.md:9
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §11 Obs Anti-Patterns → MCP double-gate ban
    change: 'After "the console engine''s door serves the same nine tools behind neither gate" add "and since chunk 2026-10-12-stdio-agent-reaches-the-door a stdio agent reaches them through `andromeda-pulse-engine door` behind neither gate, the door''s owner check still the admission".'
    sidecar: 'obs-plan §11 double-gate ban: a stdio agent reaches the door''s nine tools through the `door` relay behind neither gate (chunk 2026-10-12-stdio-agent-reaches-the-door; PROVISIONAL).'
    rationale: 'Report Expected amendment 14 names `mcp-stdio` obs-plan.md:662 ("The ban is the stdio sidecar''s"); Outcome criterion 4 (the relay runs with `--features mcp-server` off) and Symbols / APIs (How the door treats the relay: kernel-read account check as any caller). The ban''s statement that stdio reach goes through the double gate holds only for the sidecar.'
    basis: .andromeda/obs-plan.md:662
    dependent-of: D-obs-instrumentation
  - detector: D-obs-stack
    severity: warning
    section: §11 Obs Anti-Patterns → stdout ban
    change: 'Extend the ban to `andromeda-pulse-engine door` (the stdio relay, chunk 2026-10-12-stdio-agent-reaches-the-door): NEVER write to its stdout anything but the door''s answer bytes, unchanged, and to its stderr anything but the one fixed line on exit 1; it installs no `tracing` subscriber and writes no record.'
    sidecar: 'obs-plan §11 stdout ban reaches the `door` relay: stdout holds only the door''s answer bytes, stderr only the fixed unreachable line (chunk 2026-10-12-stdio-agent-reaches-the-door).'
    rationale: 'The relay matches the stack (no logger, no OTel: grep over door_bridge.rs for observability|tracing:: reads 0; no Dependencies change), but it is a second stdio JSON-RPC program and the stack''s stdout reservation names the `andromeda-pulse-mcp` sidecar alone. Report Coverage of new surfaces ("stdout carries only the door''s answers"), Outcome criterion 9 (stderr empty on success; stdout JSON-RPC lines only), Expected amendment 14 (`stdout-ban` obs-plan.md:658).'
    basis: .andromeda/obs-plan.md:658
  - detector: D-obs-stack
    severity: warning
    section: §3 Observability Harness Contract → Logging stack
    change: 'Sink: after the mcp-server sidecar clause add that the console program''s `door` word (the stdio relay) has no sink at all — no subscriber, no record — its stdout reserved for the door''s JSON-RPC answer lines and its stderr for one fixed line on exit 1 (chunk 2026-10-12-stdio-agent-reaches-the-door).'
    sidecar: 'obs-plan §3 Logging stack Sink: the `door` relay installs no sink; stdout reserved for the door''s answer lines (chunk 2026-10-12-stdio-agent-reaches-the-door).'
    rationale: 'The §3 key file states the stdout reservation for the sidecar alone; report Expected amendment 14 (`stdout-ban` hit registries/contracts/obs-plan/logging-stack.md:5) and What it adds (no log record).'
    basis: .andromeda/registries/contracts/obs-plan/logging-stack.md:5
    dependent-of: D-obs-stack
  - detector: D-obs-stack
    severity: warning
    section: §1 Obs Scope Summary → Logging stack
    change: 'After the mcp-server sidecar stdout clause add that the `andromeda-pulse-engine door` relay writes no `tracing` output, its stdout reserved for the door''s answer lines (chunk 2026-10-12-stdio-agent-reaches-the-door).'
    sidecar: 'obs-plan §1 Logging stack: the `door` relay has no tracing output; stdout reserved for the door''s answers (chunk 2026-10-12-stdio-agent-reaches-the-door).'
    rationale: 'Restates the §3 Logging stack Sink clause (stdout reserved, sidecar alone); report Expected amendment 14 (`stdout-ban` hit obs-plan.md:76).'
    basis: .andromeda/obs-plan.md:76
    dependent-of: D-obs-stack
note: 'D-obs-pii: no drift — the relay adds no logging (stdout carries the door''s tool answers to the agent that started it, unsanitized as the sidecar''s; the guidance owner is the P5 card''s Beyond-the-plan row 2, a security-plan matter). D-obs-defect-narrative: no drift — §10 names no door or relay defect; the disproved claims are plan steps 5 and 6 (chunk artifacts). Two proposals above carry dependent-of D-obs-instrumentation from two different primaries: the §1 CLI row (primary) has the §4 CLI / main binary row as its dependent; the Scenario P3 Two-carriers paragraph (primary) has the §1 P3 row, the P3 Surfaces-involved bullet, the snapshot key file and the §11 double-gate ban as its dependents. Seen and left: the §9 `xtask test` row says four test binaries start `andromeda-pulse-engine run` — still true, but `unit_door_bridge` now also spawns the built console program with the word `door` (no engine, no log family); the §6 sink line for MCP server stderr JSON-only is the sidecar''s and unchanged.'
