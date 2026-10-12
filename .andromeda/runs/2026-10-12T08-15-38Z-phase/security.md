# security extract

## Relevance
relevant — the chunk opens a new way (stdio) onto the door, a PROVISIONAL boundary whose every statement rests on one threat-model item; the way's form is itself the widening P4 brings to the operator.

## Constraints
- Whatever form is chosen, the stdio way reaches the door as a door peer like any other and must not weaken the owner check. security-plan §Authentication & Authorization requires the peer's account to be read from the kernel (`peer_cred`) and compared in `door::admits` with the engine's own effective account. Any other account, root included, and any peer whose account cannot be read, is closed with no byte written. A relay or bridge that answers or buffers on the door's behalf before that check, or that runs under a different account, would bypass it. Whether a form keeps the check end to end is the question P4 must answer for each alternative (per security-plan §Authentication & Authorization).
- The door's surface does not move. security-plan §Input Validation (the console engine's door row) fixes the endpoint (`run/door/` 0700, `run/door/sock` 0600, canonical path inside the data dir, under 108 bytes), the closed methods `initialize` · `tools/list` · `tools/call` · `ping`, the closed nine-tool set, `declared_only` arguments (`-32602`), and the bounds: a 65 536 B request line, a 1 048 576 B answer that is never cut, a 5 s call, 30 s idle, 8 connections, one call at a time. A form that needs a new listener, a port, a credential, or a changed bound or method is a separate widening (per security-plan §Input Validation).
- The one write keeps its switch. security-plan §Input Validation (door row) requires `mark_incident_resolved` to be refused with `-32001` before the dispatcher unless `ANDROMEDA_PULSE_DOOR_RESOLVE` is on, a value outside the closed set failing closed to off. A stdio agent going through the door inherits that switch and nothing else (per security-plan §Input Validation).
- The double gate is the sidecar's alone, and the new way does not inherit it. security-plan §API Security (MCP feature double-gate row) states that `--features mcp-server` plus `ANDROMEDA_PULSE_MCP_ENABLED=true` governs the stdio sidecar only. The door's own controls are the owner-only socket, the request bounds and the write switch. If a form proposes any extra gate on the stdio way, it is new and is said so at P4. Nothing in the chunk may change the sidecar's gate or its graceful-degrade `warn` (per security-plan §API Security).
- If the form adds a command word to `andromeda-pulse-engine` (for example a stdio-bridge word), it enters a closed two-word grammar. security-plan §Input Validation (console program command line row) classifies `run` / `version` as a bounded input. A word that takes a value, a path or a network address is a new question each time. Any unknown or extra word prints usage on stderr and exits 2, creating nothing (per security-plan §Input Validation).
- If the way resolves the socket from `ANDROMEDA_PULSE_DATA_DIR` inside a shipped binary, that is a product-binary `*_DIR` read. security-plan §Security Anti-Patterns → Input requires `canonicalize()` plus an assertion that the path is under the resolved data dir, on the `publish_workspace_key` both-sides-canonicalize precedent. Whether the door's existing path resolution can be reused unchanged is research's question (per security-plan §Security Anti-Patterns → Input).
- The way is recorded as part of the door's one PROVISIONAL item, or as a widening of its own. security-plan §Threat Model Summary → Attack surface (the door's vector) holds that the endpoint, the owner check, the reach of the nine tools and of the write are the operator's answers, with the founder's word owed. The same section's MCP stdio vector states that "the stdio caller is whatever process the user launched". After this chunk, the engine's ways in besides telemetry are the door and the sidecar, and the plan states what each one reaches (per security-plan §Threat Model Summary).

## Patterns to follow
- Fail-closed owner admission, as `door::admits` does it: `tokio::net::UnixStream::peer_cred` compared with `libc::geteuid`, with no secret, key or auth library involved (per security-plan §Authentication & Authorization).
- Refusals and errors use the JSON-RPC 2.0 error object through the envelope functions of `crates/mcp-server/src/jsonrpc.rs`: a stable numeric `code` and a fixed `message`, never the tool's own error text, with no stack trace, path or library version (per security-plan §Error Handling, MCP server; §Input Validation, door row).
- Closed command-line grammar on the `pulse_app::console::parse` precedent: a closed `Command` enum, a non-Unicode word refused rather than panicking, exit 2 with nothing created (per security-plan §Input Validation, console program command line row).
- Any new log record is a NO-SCRUB boundary behind its own exact allowlist leaf, every field a closed label, bool or number, as with `app.boot.door` / `door.call` / `door.refused`. There is no bare `door` or `app` key and no ERROR level (per security-plan §Security Anti-Patterns → Logging).
- The other-account proof is read on the hosted runner, where a second account exists. The dev host has none (per security-plan §Authentication & Authorization).

## Anti-patterns to avoid
- The door's four bans (per security-plan §Security Anti-Patterns → Authentication):
  - NEVER write a byte to a peer before its kernel-read account equals the engine's own.
  - NEVER admit root, or a peer whose account cannot be read.
  - NEVER widen `run/door/` beyond 0700 or `run/door/sock` beyond 0600.
  - NEVER treat those modes as the control against root.
- NEVER build a process command line from a string sourced from an MCP tool argument (`Command::new(...).arg(user_input)`). If the form spawns or execs anything, as a bridge or relay would, its argv must be first-party (per security-plan §Security Anti-Patterns → Code Patterns).
- NEVER log a full product-consumed path. That includes the door socket path `{data dir}/run/door/sock` and the data dir's full path: basename or a closed `state` label only (per security-plan §Security Anti-Patterns → Logging).

## Contract bindings
- sec ↔ arch: the door's form is the contract Conductor reads, and Conductor is the door's named reader (security-plan §Threat Model Summary → Attack surface, the door's vector). Its PROVISIONAL sentence is mirrored once per master (arch §Occupied Resources → Network ports; §Standard Contracts → Engine door). If the stdio way changes what a door peer sees, the change is visible to that contract.
- sec ↔ obs: a record the new way writes needs its own allowlist leaf and the door NEVER-log list (no socket path, no data-dir path, no arguments, no answer, no tool error text, no peer account id) (security-plan §Logging & Monitoring; §Security Anti-Patterns → Logging).
- sec ↔ tests: the `lint-test` step `Door refuses another account` is the hosted-runner proof of the owner check. A form that adds a hop between the agent and the socket needs its own other-account reading at the same tier (security-plan §Authentication & Authorization). If the form adds a crate, `cargo deny check bans licenses sources` and `cargo audit` apply, with `multiple-versions = "deny"` never relaxed (security-plan §Dependency Security).
- sec ↔ docs: tool answers reaching a stdio agent through the door carry unsanitized OTLP-derived content. The indirect-prompt-injection note ("Using MCP with andromeda-pulse") applies to this way as it does to the sidecar and the door (security-plan §Logging & Monitoring, snapshot / clipboard / MCP-tool-response hygiene).

## Acceptance criteria contributions
- Through the chosen way, a client running as another account (root included) receives no byte from the door, and the way adds no path that admits such a client. This is read on the hosted runner (per security-plan §Authentication & Authorization).
- A diff against `d3e694d2` shows the door unchanged in all of the following (per security-plan §Input Validation, the console engine's door row):
  - no new listening socket, TCP port or network listener
  - `run/door/` still 0700 and `run/door/sock` still 0600
  - the request / answer / time / connection bounds, the four methods and the nine tools unchanged
- `mark_incident_resolved` sent through the stdio way answers `-32001` with `ANDROMEDA_PULSE_DOOR_RESOLVE=off`, and the write applies with it unset or `on` (per security-plan §Input Validation, the console engine's door row).
- A grep of the log family from a stdio-way run finds 0 occurrences of each of the following (per security-plan §Logging & Monitoring, What NEVER to log):
  - the socket path
  - the data dir's full path
  - a planted argument canary
  - answer text
