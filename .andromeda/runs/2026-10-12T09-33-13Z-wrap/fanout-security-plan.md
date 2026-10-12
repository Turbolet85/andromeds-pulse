proposals:
  - detector: D-security-input
    severity: escalate
    section: §Input Validation → the console program command line row
    change: 'The row reads `andromeda-pulse-engine run` / `version` / `door` — a closed THREE-word grammar under the same rules (exactly one word, case-exact, a non-Unicode word refused; anything else prints `usage: andromeda-pulse-engine <run|version|door>` on stderr, exits 2 and creates nothing); `door` is the stdio relay `pulse_app::door_bridge`: it builds no runtime and calls neither `init_process` nor `start`, resolves the data dir through `engine_boot::resolve_data_dir` (the `run` path''s variables and no other), canonicalizes the data dir and `{data dir}/run/door/sock` both sides and treats a socket that does not resolve, resolves outside the data dir or is 108 bytes or more as unreachable, reads stdin by line bounded at `door::MAX_REQUEST_BYTES` + 1 (an over-bound line sent as its first bound + 1 bytes unterminated and its rest discarded, never sent; a whitespace-only line opens nothing), parses no request and sends each line on a connection of its own to the door, which applies every bound and the account check; an unreachable door or a failed socket or stdout operation is exit 1 with one fixed stderr line and nothing on stdout; it adds no env var, flag, value, listener, port, file, pid file or log record; the third word is a boundary widening, PROVISIONAL with the door''s one item (the operator''s P4 answer, the pc overseer, 2026-10-12, inputs#I3) — not covered by the 2026-10-10 bounded-input reading of `run` and `version`; pinned by `unit_console_commands.rs`, `unit_door_bridge.rs` and the spawned arms of `integration_door.rs`.'
    sidecar: '2026-10-12-stdio-agent-reaches-the-door: §Input Validation console command-line row — grammar two words to three; the `door` relay and its validation (canonical confined socket path under 108 bytes, bounded request line, fixed unreachable line, exit 1) recorded; the third word PROVISIONAL with the door''s item.'
    rationale: 'Report Changes → Symbols / APIs (the third word `Command::Door`, `console::USAGE`, `door_bridge::socket_path` / `relay`), Counts / qualifiers moved (two words to three; names `security-plan.md:145` as a row that STATES the closed set) and Coverage of new surfaces (validation ✓ for the new external-input surface); Expected amendment 6. The row still says "a closed two-word grammar" and "these two words are its first closed members", and records no validation of the new stdin-to-socket surface.'
    basis: '.andromeda/security-plan.md:145; pulse-app/src/door_bridge.rs:64-73; pulse-app/src/door_bridge.rs:75-112; pulse-app/src/door_bridge.rs:131-144'
  - detector: D-security-input
    severity: escalate
    section: §Threat Model Summary → Attack surface (CLI input vector, entry point)
    change: 'The entry point reads "the command line of the console program `andromeda-pulse-engine` (`run`, `version` or `door`)".'
    sidecar: '2026-10-12-stdio-agent-reaches-the-door: CLI input vector entry point names the third word `door`.'
    rationale: 'Report Counts / qualifiers moved names `security-plan.md:88` ("(`run` or `version`)") as a row stating the closed set, now three words.'
    basis: '.andromeda/security-plan.md:88'
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §Threat Model Summary → Attack surface (CLI input vector, trust boundary)
    change: 'The trust boundary reads "the console program''s command line is a closed three-word grammar (`door` PROVISIONAL with the door''s item)" — any other command line prints usage on stderr and exits 2, creating nothing.'
    sidecar: '2026-10-12-stdio-agent-reaches-the-door: CLI input vector trust boundary — closed two-word grammar becomes three-word.'
    rationale: 'Same retired claim as the primary (the closed two-word grammar), restated in the vector''s trust boundary; report Counts / qualifiers moved (two words to three).'
    basis: '.andromeda/security-plan.md'
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §Threat Model Summary → Infrastructure → Hosting
    change: 'The console program is "driven by `run` or `version`, ended by SIGTERM or SIGINT", with a third word `door` that boots no engine and relays a stdio agent''s lines to the door of an engine already running in the same data dir.'
    sidecar: '2026-10-12-stdio-agent-reaches-the-door: Hosting names the console program''s third word `door` (the stdio relay; still three product binaries).'
    rationale: 'Report Counts / qualifiers moved names `security-plan.md:104` ("driven by `run` or `version`") as a row stating the closed set; Symbols / APIs: the word lives in the existing binary, no new binary.'
    basis: '.andromeda/security-plan.md:104'
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §Threat Model Summary → Attack surface (the console engine's door vector)
    change: 'The door''s entry point adds the stdio way in: a stdio agent reaches the door through `andromeda-pulse-engine door`, a short-lived relay it starts (one per agent session) — a client of `run/door/sock`, one connection per request line, no listener, port, file, pid file, env var or log record of its own; the door treats it as any caller (the account check; one `door.call` per answered request; a notification answered and recorded with nothing; no idle refusal); it is a way through the door, not a third way in; the PROVISIONAL item now also names the relay word (the operator''s P4 answer, the pc overseer, 2026-10-12, inputs#I3; the founder''s own word owed).'
    sidecar: '2026-10-12-stdio-agent-reaches-the-door: the door''s vector names the stdio relay `andromeda-pulse-engine door` and its PROVISIONAL item covers the relay word.'
    rationale: 'Report Changes → Symbols / APIs ("How the door treats the relay", "What it adds") and Cross-project claims inputs#I3 (form B, the word `door`, one connection per request, PROVISIONAL); Expected amendment 7 names this vector (`security-plan.md:71-73`). The vector names only `andromeda-pulse-engine run` as the opener and the socket as the entry point; a stdio agent''s way to it is a new external-input surface the vector does not list.'
    basis: '.andromeda/security-plan.md:71-73; pulse-app/src/door_bridge.rs:114-129'
  - detector: D-security-input
    severity: escalate
    section: §Threat Model Summary → Attack surface (MCP stdio surface vector)
    change: 'Stdio is no longer the sidecar''s alone: since chunk `2026-10-12-stdio-agent-reaches-the-door` a stdio agent reaches the same nine tools through `andromeda-pulse-engine door`, the relay to the console engine''s door, behind neither gate of the double gate; its caller is checked by the door''s account check, not by this vector''s trust boundary (see the door''s vector).'
    sidecar: '2026-10-12-stdio-agent-reaches-the-door: MCP stdio surface vector — a second stdio carrier of the nine tools (the door relay) recorded.'
    rationale: 'Report Expected amendment 4 ("stdio is no longer the sidecar''s alone"), listing security-plan among the masters the fact reaches; the vector (`security-plan.md:67-69`) frames the stdio caller as the sidecar''s alone.'
    basis: '.andromeda/security-plan.md:67-69'
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: §API Security → MCP feature double-gate row
    change: 'Add: since chunk `2026-10-12-stdio-agent-reaches-the-door` a stdio agent reaches the door''s nine tools over stdin/stdout through `andromeda-pulse-engine door`, behind neither gate; the door''s own controls (owner-only socket, peer-account check, request bounds, `ANDROMEDA_PULSE_DOOR_RESOLVE`) are what hold it (PROVISIONAL — the door''s vector).'
    sidecar: '2026-10-12-stdio-agent-reaches-the-door: MCP double-gate row — a stdio way to the nine tools outside the gate (the door relay) stated.'
    rationale: 'The row opens "MCP stdio surface only present when `--features mcp-server` AND `ANDROMEDA_PULSE_MCP_ENABLED=true`"; report Symbols / APIs: the relay word lives in `andromeda-pulse-engine` with no feature gate (acceptance 4: it runs with `--features mcp-server` off), so a stdio way to the same tools now exists outside both gates.'
    basis: '.andromeda/security-plan.md'
    dependent-of: D-security-input
  - detector: D-security-auth
    severity: warning
    section: §Authentication & Authorization
    change: 'Add: a stdio caller through `andromeda-pulse-engine door` is admitted or refused by the same owner check as any peer — the relay adds no identity mechanism, secret or library (`std` Unix sockets); run by another account it receives no byte: a refused connection reads `EPIPE` or `ECONNRESET` at the client on Linux, so the relay ends exit 1 with its fixed line and nothing on stdout — as measured on `ci#38227488889`, four readings (`nobody` and root, at the modes as made and with `run/door/` 0755 and the socket 0666), each exit 1 with 0 stdout bytes, and 3 `door.refused {other_account, other}` records (root as made, `nobody` opened, root opened; `nobody` as made stopped by the directory mode).'
    sidecar: '2026-10-12-stdio-agent-reaches-the-door: §Authentication & Authorization — the relay is checked as any door peer; its other-account reading on ci#38227488889 recorded.'
    rationale: 'Report Symbols / APIs ("How the door treats the relay", "Other-account behaviour, measured"), Spec claims disproved (plan step 6: exit 1, not 0; "Where it lands in the masters: the security-plan §Authentication & Authorization / second-account-step sentence ... state the relay arm''s reading as built"), Cross-project claims (the step''s printed readings); Expected amendment 8. No off-spec auth library or secret source is involved.'
    basis: 'pulse-app/tests/integration_door.rs:1023-1141; .andromeda/security-plan.md:123'
  - detector: D-security-auth
    severity: warning
    section: §Security Anti-Patterns → Input (the second-account step, a harness record)
    change: 'The `lint-test` step `Door refuses another account` runs TWO arms of `pulse-app/tests/integration_door.rs` — `another_account_is_refused_at_each_of_three_layers` and `another_account_through_the_bridge_receives_nothing` (filter `binary(integration_door) & (test(=…) | test(=…))`, same flags) — and passes only when nextest exits 0, its kept output is not empty and holds BOTH arms'' sentences (the three-layer sentence and `door bridge other-account arm: no byte reached another account through the bridge, the account check refused`), else `::error::` and exit 1; two things run as another account, root included: the three-layer arm''s Python client and the relay arm''s copy of the built `andromeda-pulse-engine`, run as `sudo -n [-u nobody] env ANDROMEDA_PULSE_DATA_DIR={data dir} {copy} door` against an engine the test spawned (the data dir and `run/` opened to 0755, the copy in the arm''s own TempDir); either arm''s skip sentence fails the step; the step''s own text still holds no `sudo`.'
    sidecar: '2026-10-12-stdio-agent-reaches-the-door: second-account step — one arm to two, one required sentence to two; the relay copy is a second thing run as another account.'
    rationale: 'Report Harness / gate surface (ci.yml step: two-arm filter, fourth check on the bridge sentence; the other-account relay arm runs the copied binary under sudo, "the second thing in the suite run as another account") and Counts / qualifiers moved ("one arm → two, one required sentence → two"); Expected amendment 8 names `security-plan.md:412`. That sentence still says the step "runs the one arm" and the Python client "is the only thing run as another account".'
    basis: '.andromeda/security-plan.md:412; .github/workflows/ci.yml:114-122; .github/workflows/ci.yml:128; .github/workflows/ci.yml:142-145'
  - detector: D-security-logging
    severity: warning
    section: §Logging & Monitoring → Snapshot / clipboard / MCP-tool-response hygiene
    change: 'The MCP-tool-response bullet adds the stdio relay as a carrier: `andromeda-pulse-engine door` writes the door''s answers unchanged (byte for byte) to the stdout of the process that started it, unsanitized OTLP-derived content included, and the prompt-injection note applies to a stdio agent through the relay as to the sidecar''s client; and it no longer says the note is "surfaced in the README under ''Using MCP with andromeda-pulse''": no such document is in the tree (research''s `git grep`), the guidance is unwritten and owned by the route entry `Records say what the product is`.'
    sidecar: '2026-10-12-stdio-agent-reaches-the-door: tool-response hygiene — the relay named as a third carrier; the README claim corrected to unwritten, owner `Records say what the product is`.'
    rationale: 'Report Coverage of new surfaces (PII: stdout carries only the door''s answers to the process that started it, "unsanitized as the sidecar''s — the guidance security-plan names is the P5 card''s Beyond-the-plan row 2"), Outcome criterion 3 (relay line equals the direct line byte for byte) and inputs#I4 row 2 ("Research found no such document in the tree ... hits only in 2026-05-02 security run files"; owner `Records say what the product is`). The bullet states the README surfacing as fact.'
    basis: '.andromeda/security-plan.md'
note: 'No D-security-deps proposal: the report''s Dependencies bullet lists none added or bumped (Cargo.toml / Cargo.lock byte-identical). D-security-logging found no stale coverage claim in §Security Anti-Patterns → Logging itself: the relay writes no log record and adds no target, leaf or scrub shape (report Schema / config), and the door''s three record leaves are unchanged. The report''s `security-plan.md:72`, `:148` and the other `run`-only grammar hits need no change (`run` is unchanged).'
