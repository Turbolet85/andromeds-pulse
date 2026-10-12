proposals:
  - detector: D-arch-resources
    severity: escalate
    section: '§Occupied Resources → Process / service identity (Console engine binary)'
    change: 'The console engine binary''s command line is a closed THREE-word grammar (`run` | `version` | `door`; usage `usage: andromeda-pulse-engine <run|version|door>`): since chunk 2026-10-12-stdio-agent-reaches-the-door `door` is the stdio relay (`pulse_app::door_bridge::main`, `pulse-app/src/door_bridge.rs:27-48`) — a client of the running console engine''s `run/door/sock`, started by a stdio agent, which builds no tokio runtime, calls neither `engine_boot::init_process` nor `engine_boot::start`, and adds no listener, port, file, pid file, log record, env var or binary; it ends 0 when standard input ends and 1 with the one fixed stderr line `andromeda-pulse-engine door: the door cannot be reached` when the door cannot be reached (off Unix at once); any other command line still prints usage on stderr and exits 2, creating nothing; the product binaries stay three. PROVISIONAL — §Network ports, the door.'
    sidecar: '2026-10-12 · §Occupied Resources → Process / service identity · the console command grammar reads `run | version | door`; `door` is the stdio relay to the door (chunk 2026-10-12-stdio-agent-reaches-the-door; PROVISIONAL, the operator''s P4 answer inputs#I3).'
    rationale: 'Report Changes → Symbols / APIs (a third word of the console program: `Command::Door`, `console::USAGE` = `<run|version|door>`, `door` builds no runtime and calls neither boot function; What it adds: none of listener/port/file/pid/record/env var/binary) and Counts / qualifiers moved (two words → three); Expected amendment 1. The body states "a closed two-word command line". Boundary widening, PROVISIONAL with the door''s one item (inputs#I3), halting in the ordinary form (inputs#I6 item 3).'
    basis: '.andromeda/architecture.md:217'
  - detector: D-arch-resources
    severity: escalate
    section: '§Project Intent → Product type'
    change: 'The console program `andromeda-pulse-engine` is driven by a closed three-word command line (`run` | `version` | `door`) with no window and no display — `door` (since chunk 2026-10-12-stdio-agent-reaches-the-door) a stdio relay to the running engine''s door, hosting no engine part.'
    sidecar: '2026-10-12 · §Project Intent · the console program''s closed command line reads three words, `door` added (chunk 2026-10-12-stdio-agent-reaches-the-door).'
    rationale: 'Report Counts / qualifiers moved names `architecture.md:300` ("two-word command line (`run` | `version`)") as a row that STATES the closed set; the grammar is now three words (Symbols / APIs).'
    basis: '.andromeda/architecture.md:300'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: escalate
    section: '§Standard Contracts → Engine door'
    change: 'Add a **Stdio** clause: a caller with no socket client reaches the door by starting `andromeda-pulse-engine door` as the engine''s own account (since chunk 2026-10-12-stdio-agent-reaches-the-door; the data dir resolved by `engine_boot::resolve_data_dir()` as `run` resolves it; the socket path `{data dir}/run/door/sock` canonicalized on both sides, refused outside the data dir or at 108 bytes or more). Each stdin line that is not whitespace alone takes a connection of its own, in order: the line is sent with its `\n`, the write side shut, the whole answer read and only then written to stdout unchanged (the `tools/list` line equals the direct socket read byte for byte); a line over 65 536 bytes is sent as its first 65 537 bytes with no terminator and the rest of it discarded. The relay parses nothing, writes no answer of its own, retries nothing and has no time bound of its own. Standard input ending is exit 0; a socket connect/write/shutdown/read failure or a failed stdout write is exit 1 with the one fixed stderr line `andromeda-pulse-engine door: the door cannot be reached` and nothing on stdout from the failed connection. The door treats it as any caller: the account check, one `door.call` per answered request, a notification answered and recorded with nothing, no idle refusal (one connection per request); another account through the relay receives no byte and reads exit 1 with the fixed line. The door''s own form is unchanged.'
    sidecar: '2026-10-12 · §Standard Contracts → Engine door · a stdio caller reaches the door through `andromeda-pulse-engine door`, one connection per request, the door''s lines passed unchanged; the door''s form unchanged (chunk 2026-10-12-stdio-agent-reaches-the-door; PROVISIONAL).'
    rationale: 'Report Changes → Symbols / APIs (the relay `pulse_app::door_bridge`: `main`, `socket_path`, `relay`, `exchange`, `discard_line`; How the door treats the relay; Other-account behaviour, measured) and Expected amendment 3; Outcome criteria 1, 3, 7 and 8; Deviations step 6 (exit 1 for a refused account, accepted as built, inputs#I5). The Engine door is the form Conductor reads, so the added stdio way is a contract addition the operator should see, though the socket form itself did not change (criterion 1).'
    basis: '.andromeda/architecture.md:157; pulse-app/src/door_bridge.rs:64-73; pulse-app/src/door_bridge.rs:75-112'
  - detector: D-arch-resources
    severity: escalate
    section: '§Occupied Resources → MCP stdio surface'
    change: 'MCP stdio surface: standard MCP `initialize`, `tools/list`, `tools/call`, `notifications/*` over stdin/stdout when the `andromeda-pulse-mcp` sidecar is started; and, since chunk 2026-10-12-stdio-agent-reaches-the-door, the door''s methods (`initialize` · `tools/list` · `tools/call` · `ping`, a request with no `id` answered with nothing) over the stdin/stdout of `andromeda-pulse-engine door`, the relay to `run/door/sock` — Unix only, no feature gate. PROVISIONAL — §Network ports, the door.'
    sidecar: '2026-10-12 · §Occupied Resources → MCP stdio surface · stdio is no longer the sidecar''s alone: the relay word `andromeda-pulse-engine door` carries the door''s methods over stdio (chunk 2026-10-12-stdio-agent-reaches-the-door).'
    rationale: 'Report Expected amendment 4 ("stdio is no longer the sidecar''s alone") and Symbols / APIs (the relay relays stdin to stdout through the door); the surface entry names the sidecar as the only stdio carrier.'
    basis: '.andromeda/architecture.md:210'
  - detector: D-arch-resources
    severity: escalate
    section: '§Occupied Resources → Engine door surface'
    change: 'Replace "The MCP stdio surface above stays the sidecar''s." with: a stdio caller reaches this surface through `andromeda-pulse-engine door` (since chunk 2026-10-12-stdio-agent-reaches-the-door), which relays stdin lines to the socket one connection per request and passes the answer lines unchanged to stdout; the sidecar remains the other stdio carrier until the route entry `Stdio sidecar retired` lands.'
    sidecar: '2026-10-12 · §Occupied Resources → Engine door surface · retires "the MCP stdio surface stays the sidecar''s": the door is reached over stdio through the relay word (chunk 2026-10-12-stdio-agent-reaches-the-door).'
    rationale: 'Report Expected amendment 4 cites `architecture.md:211` ("The MCP stdio surface above stays the sidecar''s") among the sites; the relay (Symbols / APIs) falsifies it.'
    basis: '.andromeda/architecture.md:211'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: escalate
    section: '§Established Decisions → [MCP Server Surface]'
    change: 'After "the engine has two ways in besides telemetry — the door and the stdio sidecar", add: since chunk 2026-10-12-stdio-agent-reaches-the-door a stdio agent reaches the nine through the door with no sidecar and no feature, by starting `andromeda-pulse-engine door` (`pulse_app::door_bridge`, `std` Unix sockets, no crate edge and no dependency added), which relays its stdin lines to `run/door/sock` and the door''s answer lines to its stdout and parses nothing — so stdio is no longer the sidecar''s alone, while the ways in stay two (the relay enters through the door).'
    sidecar: '2026-10-12 · §Established Decisions [MCP Server Surface] · a stdio agent reaches the nine tools through the door via `andromeda-pulse-engine door`, no sidecar, no feature (chunk 2026-10-12-stdio-agent-reaches-the-door).'
    rationale: 'Report Expected amendment 4 names §Established Decisions [MCP Server Surface] (`architecture.md:60`); Crates / modules (`door_bridge`, no `crate_edges` change); Dependencies none; Outcome criterion 4 (the relay runs with `--features mcp-server` off).'
    basis: '.andromeda/architecture.md:60'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: escalate
    section: '§Established Decisions → [API Style — MCP Server Surface 3]'
    change: 'Where it says the envelope has two transports, "stdio (the sidecar, per MCP spec) and the console engine''s door", add that since chunk 2026-10-12-stdio-agent-reaches-the-door the door is also reached over stdio through `andromeda-pulse-engine door`, which passes the door''s lines unchanged, so a stdio caller there gets the door''s envelope, its fixed-message codes `-32001` · `-32002` · `-32003` and its bounds, never a custom envelope of the relay''s own.'
    sidecar: '2026-10-12 · §Established Decisions [API Style — MCP Server Surface 3] · stdio is no longer the sidecar''s transport alone: the relay word carries the door''s envelope unchanged (chunk 2026-10-12-stdio-agent-reaches-the-door).'
    rationale: 'Report Expected amendment 4 names [API Style — MCP Server Surface 3] (`architecture.md:63`); Decisions & corrections (the `tools/list` byte comparison: the relay passes the door''s line unchanged); the relay writes no answer of its own (Symbols / APIs).'
    basis: '.andromeda/architecture.md:63'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: escalate
    section: '§Conventions → External wire — MCP server'
    change: '"Over stdio per MCP spec" stays the sidecar''s wire; the door''s line-delimited JSON-RPC 2.0 is reached over the Unix-domain socket `run/door/sock` and, since chunk 2026-10-12-stdio-agent-reaches-the-door, over stdio through the relay `andromeda-pulse-engine door`, one connection per request.'
    sidecar: '2026-10-12 · §Conventions → External wire — MCP server · the door''s wire is also reached over stdio through the relay word (chunk 2026-10-12-stdio-agent-reaches-the-door).'
    rationale: 'Report Expected amendment 3 lists `architecture.md:85` as a site pointing to the door contract; the sentence restates stdio as the sidecar''s wire alone, which the relay (Symbols / APIs) retires.'
    basis: '.andromeda/architecture.md:85'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: escalate
    section: '§Stack and Technologies → MCP server row'
    change: 'After "the console engine serves the nine through the door with no feature", add: and, since chunk 2026-10-12-stdio-agent-reaches-the-door, to a stdio agent through the console program''s word `andromeda-pulse-engine door` (a relay to the door), also with no feature — so `--features mcp-server` gates the stdio sidecar, not every stdio way to the nine.'
    sidecar: '2026-10-12 · §Stack and Technologies, MCP server row · the nine reach a stdio agent through the relay word with no feature (chunk 2026-10-12-stdio-agent-reaches-the-door).'
    rationale: 'Report Expected amendment 4 lists `architecture.md:25` among the `mcp-stdio` sites; Outcome criterion 4 (the relay runs with `--features mcp-server` off).'
    basis: '.andromeda/architecture.md:25'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: escalate
    section: '§Inherited Defaults → API style — external MCP'
    change: 'After "`--features mcp-server` gates the stdio sidecar and the `rmcp` anchor, no longer the layer", add: a stdio caller also reaches the layer, through the door, by starting `andromeda-pulse-engine door` with no feature (since chunk 2026-10-12-stdio-agent-reaches-the-door).'
    sidecar: '2026-10-12 · §Inherited Defaults → API style — external MCP · JSON-RPC over stdio no longer needs the sidecar: the relay word reaches the door (chunk 2026-10-12-stdio-agent-reaches-the-door).'
    rationale: 'Report Expected amendment 4 lists `architecture.md:319` among the `mcp-stdio` sites; the relay (Symbols / APIs) gives a featureless stdio way to the layer.'
    basis: '.andromeda/architecture.md:319'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: escalate
    section: '§Occupied Resources → Network ports (the door)'
    change: 'The door''s PROVISIONAL item also covers, since chunk 2026-10-12-stdio-agent-reaches-the-door, the console program''s third word `door` (the stdio relay: one connection per request, no file, listener, pid, variable or binary; an unreachable door exit 1 with one fixed stderr line) and the second arm of the workflow step that runs a client as a second account — the operator''s answers at that chunk''s P4 (the pc overseer, 2026-10-12, inputs#I3) and its pass (inputs#I5); the founder''s own word is still owed and nothing of it is ratified.'
    sidecar: '2026-10-12 · §Occupied Resources → Network ports · the door''s one PROVISIONAL item now covers the relay word `door` and the step''s second arm (chunk 2026-10-12-stdio-agent-reaches-the-door; inputs#I3, inputs#I5).'
    rationale: 'Report Expected amendment 5 ("the door''s PROVISIONAL item now covers the relay word", `architecture.md:181`); Cross-project claims inputs#I3 (form B, the word `door`, proof (b) in the existing step, PROVISIONAL) and inputs#I6 item 3 (no answer card; the founder did not answer this widening himself).'
    basis: '.andromeda/architecture.md:181'
  - detector: D-arch-resources
    severity: escalate
    section: '§Infrastructure Patterns → Deployment model'
    change: 'The runtime topology adds, since chunk 2026-10-12-stdio-agent-reaches-the-door, short-lived `andromeda-pulse-engine door` processes — the stdio relay a stdio agent starts, one per agent session, a client of the running console engine''s door that hosts no engine part (no tokio runtime, neither boot function, no file, pid file or log record) — beside the optional `andromeda-pulse-mcp` sidecar, which stays as written.'
    sidecar: '2026-10-12 · §Infrastructure Patterns → Deployment model · the runtime topology names the relay process a stdio agent starts (chunk 2026-10-12-stdio-agent-reaches-the-door; PROVISIONAL).'
    rationale: 'Report Expected amendment 2 (the topology names the relay process; the body is the key file `deployment-model.md`, `:4` the topology); Symbols / APIs (`door` builds no runtime, calls neither `init_process` nor `start`; What it adds: nothing on disk, no record).'
    basis: '.andromeda/registries/contracts/architecture/deployment-model.md:4'
  - detector: D-arch-resources
    severity: escalate
    section: '§Infrastructure Patterns → CI/CD approach'
    change: 'The `lint-test` step `Door refuses another account` runs two arms of `integration_door` in its one cargo command — the filter `binary(integration_door) & (test(=another_account_is_refused_at_each_of_three_layers) | test(=another_account_through_the_bridge_receives_nothing))`, flags unchanged — and fails unless nextest exited 0, its kept output is not empty and holds both the three-layer sentence and `door bridge other-account arm: no byte reached another account through the bridge, the account check refused` (else `::error::the bridge other-account arm did not read the account check refusing`, exit 1); two things now run as another account, the three-layer arm''s Python client and the relay arm''s copy of the built `andromeda-pulse-engine` run as `door` under `sudo -n` and `sudo -n -u nobody`; still a plain `run` step with no `if:`, no `continue-on-error`, no `sudo` in its own text; as measured on `ci#38227488889` (pull_request, the pushed tip `2595b5cc`): green 7/7, all four other-account relay readings exit 1 with 0 bytes on stdout (the step''s pins are in `pulse-app/tests/quality_gate_workflow.rs`, among them `door_other_account_step_fails_a_bridge_arm_that_skipped`). PROVISIONAL — §Occupied Resources → Network ports, the door.'
    sidecar: '2026-10-12 · §Infrastructure Patterns → CI/CD approach · the second-account step selects two arms and reads two sentences; the relay run as another account is the second thing run as another account (chunk 2026-10-12-stdio-agent-reaches-the-door; PROVISIONAL).'
    rationale: 'Report Harness / gate surface (`.github/workflows/ci.yml` lines 114-122, 128, 142-145: the two-arm filter and the fourth check; the relay arm at `pulse-app/tests/integration_door.rs:1023-1141`, "the second thing in the suite run as another account"); Cross-project claims (`ci#38227488889` readings); Expected amendment 10 lists `registries/contracts/architecture/ci-cd-approach.md:3`. The key file states a one-arm selection, one sentence, and "the arm''s one client ... is the only thing run as another account".'
    basis: '.andromeda/registries/contracts/architecture/ci-cd-approach.md:3; .github/workflows/ci.yml:114-122; .github/workflows/ci.yml:142-145'
  - detector: D-arch-resources
    severity: warning
    section: '§Stack and Technologies → Gate-script pins (test-only) row'
    change: 'Since chunk 2026-10-12-stdio-agent-reaches-the-door one more test binary, `unit_door_bridge`, needs Unix-domain sockets (it reaches a socket of 107 bytes and a 108-byte name made by hard-linking a short-path socket), and a second arm of `integration_door`, `another_account_through_the_bridge_receives_nothing`, runs a copy of the built `andromeda-pulse-engine` as `door` under `sudo -n -u nobody` and `sudo -n`, so it needs a passwordless `sudo` (not `python3`); it too prints its own skip sentence and passes where `sudo -n true` fails (the dev host), and the `lint-test` step reads both arms'' sentences and fails on either skip. No Cargo or npm dependency; `Cargo.lock` byte-identical.'
    sidecar: '2026-10-12 · §Stack and Technologies, Gate-script pins row · `unit_door_bridge` needs Unix sockets; the relay''s other-account arm needs passwordless sudo and skips like the first, the step failing on either skip (chunk 2026-10-12-stdio-agent-reaches-the-door).'
    rationale: 'Report Harness / gate surface (the relay arm skips with its sentence where `sudo -n true` fails, runs `sudo -n [-u nobody] env ANDROMEDA_PULSE_DATA_DIR=… {copy} door`; the pins require both skip sentences); Files (new `pulse-app/tests/unit_door_bridge.rs`); Deviations step 3 (the 107-byte and hard-linked 108-byte cases); Dependencies none. The row (`architecture.md:34`) states one arm runs as another account and three door test binaries need sockets.'
    basis: '.andromeda/architecture.md:34; pulse-app/tests/integration_door.rs:1023-1141'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: '§Infrastructure Patterns → Project directory structure'
    change: 'Under `pulse-app/src/`, beside `door.rs`, list `door_bridge.rs` — the console program''s `door` word: a stdio relay to the door, one connection per request (a `pub mod` of `lib.rs`).'
    sidecar: '2026-10-12 · §Infrastructure Patterns → Project directory structure · `pulse-app/src/door_bridge.rs` listed (chunk 2026-10-12-stdio-agent-reaches-the-door).'
    rationale: 'Report Files (new `pulse-app/src/door_bridge.rs`, 145 lines) and Crates / modules (`pulse-app` gains the module `door_bridge`, `pulse-app/src/lib.rs:15`); the key file lists each `pulse-app/src` module of the two programs, `door.rs` included, and not this one.'
    basis: '.andromeda/registries/contracts/architecture/project-directory-structure.md; pulse-app/src/lib.rs:15'
note: 'D-arch-decisions found no drift: no dependency added or bumped, no crate or crate edge, `std` Unix sockets only (report Dependencies, Crates / modules). Every escalate row is one of the report''s fourteen Boundary-widening amendments (PROVISIONAL, inputs#I3; halting in the ordinary form, inputs#I6 item 3). Left for the orchestrator, not proposed: §Design Philosophy says "the OS sees one process per program" and the Stack Message-broker row and [Mobile / Message Broker / Push Notifications] say "each product program is a single process" — the engine still is, but the console binary may now also run as relay processes that host no engine part; judge whether that wording needs a clause. §Filesystem locations'' "`run/door/sock` written by `andromeda-pulse-engine run` alone" still holds (the relay is a client and writes nothing).'
