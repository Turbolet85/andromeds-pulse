# Report — 2026-10-10-door-inside-the-engine-s-process

**Chunk:** a door inside the engine's process: tools read the engine's stores, owner only, own host, call records
allowlisted; the stdio sidecar and its double gate leave (the last clause moved to a removal entry by the cut,
inputs#I3)
**Date:** 2026-10-11T00:48:41Z (read from `date -u`)
**Commits:** `2e4ee5ec chore(2026-10-10-door-inside-the-engine-s-process): operator pre-CI commit` (the one commit
since the chunk base `303fa9a8`; basis `git log --format='%h %s' 303fa9a8..HEAD`)

This report was authored in the first of two wrap windows, on the operator's directive (inputs#I5 item 2,
inputs#I6). The implement conversation and the operator pass were in the same window, so every statement below that
is not a diff reading comes from that conversation, not from the evolve records.

## Changes (structured — detectors read this)

- **Files** (16; basis `gate.py scope`: changed 16 · listed 16 · recorded 0, base `303fa9a8`):
  new `pulse-app/src/door.rs` · `pulse-app/tests/integration_door.rs` · `pulse-app/tests/unit_door.rs` ·
  `pulse-app/tests/unit_observability_allowlist_door.rs`; modified `pulse-app/src/engine_boot.rs` ·
  `pulse-app/src/lib.rs` · `pulse-app/src/observability.rs` · `pulse-app/Cargo.toml` ·
  `crates/mcp-server/src/tools.rs` · `crates/mcp-server/src/jsonrpc.rs` ·
  `pulse-app/tests/integration_console_engine.rs` · `pulse-app/tests/unit_engine_boot_seam.rs` ·
  `pulse-app/tests/integration_engine_boot.rs` · `docs/capability-record.json` · `.github/workflows/ci.yml` ·
  `pulse-app/tests/quality_gate_workflow.rs`. `Cargo.lock`, `deny.toml`, the bindings file, every script, every
  `xtask` file, every capability JSON and every `pulse-app/ui/**` file are unchanged (the scope-guard gate entry
  prints nothing).

- **Symbols / APIs** (coordinates copied from `## New text, by line`):
  - **A listening socket, the console program's alone.** `andromeda-pulse-engine run` now opens a Unix-domain stream
    socket at `{data dir}/run/door/sock`, mode 0600, inside the directory `{data dir}/run/door/`, mode 0700, created
    before anything is bound (`door::bind_endpoint`, `pulse-app/src/door.rs:366-407`). The path is canonicalized and
    refused when it leaves the data dir; the socket is bound at its canonical path. The window program `pulse-app`
    opens none and writes no door record. On a system that is not Unix the socket code is compiled out
    (`#[cfg(unix)] mod socket`, `door.rs:319-749`) and no door is seated.
  - **Endpoint outcomes, closed** (`EndpointState`, `door.rs:76-84`): `listening` · `path_too_long` (the canonical
    socket path is 108 bytes or more; 107 binds, measured by `unit_door.rs`) · `in_use` (a live listener holds the
    path) · `occupied` (the path exists and is not a socket) · `bind_failed` (anything else, a door directory that
    resolves outside the data dir among them). A socket file an earlier run left is replaced only when it is a socket
    and a connect to it is refused. In every outcome but `listening` the engine goes on with no door.
  - **The owner check** (`door::admits`, `door.rs:163-167`; `door::own_account`, `door.rs:359-364`, `libc::geteuid`):
    on every accepted connection the peer's account is read from the kernel (`tokio::net::UnixStream::peer_cred`) and
    compared with the engine's own effective account. Any other account, root included, and a peer whose account
    cannot be read, is closed with no byte written.
  - **The call surface.** JSON-RPC 2.0, one request per line, one answer line per request that carries an `id`; a
    request with no `id` is not answered and runs nothing. Methods `initialize` · `tools/list` · `tools/call` ·
    `ping`, through the envelope functions of `crates/mcp-server/src/jsonrpc.rs`. `tools/call` runs one of the nine
    tools through `mcp_server::tools::call_tool` over `door::EngineTools` (`door.rs:255-317`): a read connection of
    the door's own taken with `viz::query::read_connection` from the engine's buffer connection, the engine's
    `Arc<dyn CorpusWriter>`, and the engine's `incident_workspace_key`. The door opens no database, opens no corpus,
    reads no `run/workspace-key` and prepares no SQL statement (the probe gate entry reads 0).
  - **Bounds** (`door.rs`, public constants): `MAX_REQUEST_BYTES` 65 536 (a longer line: answer `-32600`, connection
    closed) · `MAX_ANSWER_BYTES` 1 048 576 (a longer answer is not cut: error `-32003`) · `CALL_TIMEOUT` 5 s (error
    `-32002`; the door then interrupts its own read connection through a DuckDB interrupt handle) · `IDLE_TIMEOUT`
    30 s (a connection with no complete line, or a caller that does not read its answer, is closed) ·
    `MAX_CONNECTIONS` 8 (a ninth is closed at accept). One tool call runs at a time inside the door.
  - **Error answers, fixed messages** (`refusal_of`, `door.rs:567-585`): `-32700` parse error · `-32600` request too
    large · `-32601` unknown tool / method not found · `-32602` invalid arguments · `-32603` tool failed · `-32001`
    the write is switched off · `-32002` timed out · `-32003` answer too large: the bound is 1048576 bytes. No
    answer carries the tool's own error text.
  - **Undeclared argument members are refused by the door itself** (`declared_only`, `door.rs:633-646`): `arguments`
    must be an object whose every member is declared in the tool's `inputSchema.properties` of
    `tools_list_manifest()`, else `-32602` before any tool runs.
  - **The write switch.** `ResolveWrite` (`door.rs:43-74`): `On` · `Off` · `Rejected`. `mark_incident_resolved` is
    refused with `-32001` before the dispatcher is called unless the switch is `On`; `tools/list` still names it.
  - **A new environment variable, product-read:** `ANDROMEDA_PULSE_DOOR_RESOLVE` (`ENV_DOOR_RESOLVE`,
    `engine_boot.rs:86-87`; `resolve_door_write`, `engine_boot.rs:134-139`). Unset or `on` is on; `off` is off; any
    other value, a value that is not UTF-8 included, is off and reads `rejected`. The value is never logged. It is
    read by `EngineConfig::from_env`, which BOTH programs call, so the window app reads it too; only the console
    program's door uses it.
  - **`EngineConfig` gained a field**, `door_resolve: ResolveWrite` (`engine_boot.rs:374`). Its two constructions:
    `EngineConfig::from_env` (both entry points, unchanged) and the hand-built one in
    `pulse-app/tests/integration_engine_boot.rs`.
  - **The door is seated in `engine_boot::start` and nowhere else** (`engine_boot.rs:1037-1053`): for
    `Program::Console` only, after the buffer and the corpus exist, `door::open(…)` then `tokio::spawn(door.serve())`.
    `door::open` (`door.rs:409-432`) binds and writes the boot record. Neither entry point changed.
  - **Three log targets, each an exact allowlist leaf** (`observability.rs:1022-1045`); no bare `door` key and no
    bare `app` key exists:
    - `app.boot.door` — `state` (`listening` | `path_too_long` | `in_use` | `occupied` | `bind_failed`) ·
      `resolve_write` (`on` | `off` | `rejected`). Once per boot of a program that seats the door. INFO when
      `listening` with the switch not `rejected`, WARN otherwise. Emit site `door::emit_boot`, `door.rs:182-201`.
    - `door.call` — `method` (`initialize` | `tools/list` | `tools/call` | `ping` | `other` | `none`) · `tool` (one
      of the nine names | `none`) · `outcome` (`ok` | `tool_error` | `invalid_arguments` | `unknown_tool` |
      `unknown_method` | `parse_error` | `write_off` | `timed_out` | `answer_too_large`) · `write` (bool) ·
      `result_type` · `result_count` · `answer_bytes` · `duration_ms`. Once per request that is answered, written
      before the answer line is written. INFO for `ok`, WARN otherwise. Emit site `door::emit_call`,
      `door.rs:203-234`.
    - `door.refused` — `reason` (`other_account` | `too_many_connections` | `request_too_large` | `idle`) · `peer`
      (`owner` | `other`). WARN. Emit site `door::emit_refused`, `door.rs:236-244`.
    No record carries an argument, an answer, a path, an account id or an error's text; none is written at ERROR.
  - **Library records now written by a console engine when a tool is called through the door** (the tools' own
    callees, unchanged code): `viz.query.traces` · `viz.query.metrics` · `viz.query.logs` (basis `grep -o 'target:
    "…"' crates/viz/src/query.rs`: those three and `viz.query`, the read-connection fallback warning) and, for
    `generate_snapshot`, `metric.snapshot.token_count_ms` · `snapshot.token.count.validate` (basis the same grep
    over `crates/snapshot/src/*.rs`). Which of them a real call wrote was not enumerated from a log; the spawned arm
    asserts only that the marker and the full data-dir path are absent and that no record is at ERROR.
  - **`crates/mcp-server`:** new `pub struct ToolAnswer { value, result_type, result_count }` (`tools.rs:159-166`)
    and `pub fn call_tool(…) -> Result<ToolAnswer, Error>` (`tools.rs:168-185`), the dispatch with no log record.
    `dispatch_tool` keeps its signature and its three records by calling it; its callers are untouched
    (`andromeda-pulse-mcp.rs`, 9 tests in `tools.rs`, 7 in `pulse-app/tests/e2e_p3_mcp_incident_tools.rs`; research's
    graph reading, not re-queried). `call_tool` has two callers: `dispatch_tool` and `door::EngineTools`.
    `tools_list_manifest()` gives every tool entry a boolean member `write`, true for `mark_incident_resolved`
    alone (`jsonrpc.rs`, the nine added lines 130-234 and the pin `396-412`). This is the one change the stdio
    sidecar's own output takes: its `tools/list` answer carries the member too.
  - **The `initialize` answer through the door** is `jsonrpc::initialize_result()` unchanged: `serverInfo.name`
    reads `andromeda-pulse-mcp`, protocol `2024-11-05`.

- **Crates / modules:** added module `pulse_app::door` (`pulse-app/src/lib.rs:14`). No crate added or removed.
  `pulse-app` now links `mcp-server` in every build.

- **Dependencies:** none added, none bumped. `pulse-app/Cargo.toml`: the dependency `mcp-server-crate` (package
  `mcp-server`) is no longer `optional`; the feature reads `mcp-server = ["mcp-server-crate/mcp-server"]`
  (`Cargo.toml:87`; it had named `dep:mcp-server-crate` as well). `Cargo.lock` is byte-identical to the base (not in
  `git diff --name-only 303fa9a8`). `rmcp` still enters only under the feature. `cargo deny check bans licenses
  sources` and `cargo audit` exit 0 with `deny.toml` unedited.

- **Schema / config:** no table, no migration, no settings key. One environment variable (above). One new place on
  disk, inside the data dir: `run/door/` (0700) holding `sock` (0600). The data dir's top entries stay `corpus`,
  `logs`, `run`. The socket file is not removed when the engine ends; the next boot replaces it. No scrub or
  redaction shape changed.

- **Spec-master edits:** none.

- **Counts / qualifiers moved** (each with its basis; the masters and leaves stating the old value are for the
  cascade to find):
  - `pulse-app/tests/*.rs`: 109 → 112 (`ls pulse-app/tests/*.rs | wc -l` = 112). The plan predicted 112.
  - Workspace nextest: 3132 → 3174 tests (`Summary` line of the workspace gate entry; 3132 from the previous chunk's
    `evidence/operator-pass.md`). The 42 are this chunk's: `unit_door` 19 · `unit_observability_allowlist_door` 6 ·
    `integration_door` 5 · `integration_engine_boot` +5 (6 → 11) · `quality_gate_workflow` +6 (35 → 41) ·
    `mcp-server` `jsonrpc` +1.
  - `ENGINE_PARTS` in `unit_engine_boot_seam.rs`: 17 → 18 (`door::open(` joined).
  - A console engine's listeners: two loopback TCP receivers → those two and one `unix` listener at the canonical
    `run/door/sock` (set equality in `integration_console_engine.rs`, both census arms).
  - Test binaries that claim ports through `test_support::ports`: `integration_door` joins them.
  - A console engine's log per boot: one more record, `app.boot.door` (the engine-cycle gate's kept log held 336
    records, exactly one of that target; the whole count varies from cycle to cycle and is not compared here).
  - `lint / test` job steps: one added, `Door refuses another account`, directly after `cargo xtask test`.
  - Product environment variables: one added. Allowlist exact leaves: three added.
  - Product-written locations OUTSIDE the data dir: unchanged at three (the door's place is inside it).
  - Two test functions renamed in `integration_console_engine.rs` (see Deviations); no tracked file named the old
    names (`git grep` outside the chunk folders and run dirs: 0 hits).

- **Dev-tool versions:** none — no host tool was installed or upgraded; `sudo -n true` was read failing on the dev
  host (no passwordless second account), which is a host fact, not a tool version.

- **Harness / gate surface:**
  - `.github/workflows/ci.yml:114-139`: the step `Door refuses another account` in `lint-test`, a plain `run` step
    after `cargo xtask test`, no `if:`, no `continue-on-error`, no upload. It runs `cargo nextest run --workspace
    --profile ci --no-tests=fail --success-output immediate -E 'binary(integration_door) &
    test(=another_account_is_refused_at_each_of_three_layers)'` with its output kept in
    `$RUNNER_TEMP/door-other-account.txt`, and exits non-zero unless nextest exited 0, the file is not empty and the
    file holds `door other-account arm: directory refused, socket mode refused, account check refused`. The step's
    text holds no `sudo`; the arm's one client (`python3 -c …` under `sudo -n -u nobody` and under `sudo -n`) is
    the only thing run as another account.
  - `pulse-app/tests/quality_gate_workflow.rs:937-1111`: two pins (the step's place and plain form; its one cargo
    line, its selection, the sentence it reads and the same sentence in the arm's source) and four witnesses that
    run the step's own script with `bash -e`, `cargo` standing as a shell function that prints a captured file.
  - No `xtask` verb, no `scripts/agent-run.*` word, no status or verdict shape changed. `harness:ready`,
    `harness:status`, `check:engine-log` and `harness:engine-cycle` read what they read before; the engine they
    start now also opens the door, and `check:engine-log` passes over a log holding `app.boot.door`.
  - The pre-push check's sixth stage starts a console engine with the door built in (stage data dir, socket path 82
    bytes by research's computation, not re-measured).

- **Cross-project / external claims:**
  - `ci#38097928056` (`pull_request`, attempt 1, pull request #40) on the pushed tip `2e4ee5ec`, built on the merge
    commit `6b553ea4de20`: `verdict: green · checks 7/7`; all six jobs `success`; `secret-scan#38097928018`
    success. The sha is the record: this wrap's commit adds to that tree. Read at the operator pass
    (`evidence/operator-pass.md`); the operator read the verdict too (inputs#I5 item 1).
  - On that run the step `Door refuses another account` concluded success and printed: readings as
    `[nobody, root]`, directory 0700 `[Kernel(13), Closed]`, socket 0600 `[Kernel(13), Closed]`, both opened
    `[Closed, Closed]`. On the runner's kernel a connect by another account needs search permission on the socket's
    directory and write permission on the socket file; root is held by neither and was closed by the door's account
    check at all three layers.
  - Conductor reads the engine through this door from its first check (inputs#I1 item 3, the relay's own reading);
    Conductor's intent was not read here.
  - Inputs, from `inputs.py verify` at this wrap (6 entries · drifted 0 · vanished 0 · broken 0 · unparsed 0):
    I1 · the phase relay file · copy · unchanged, cited · I2 · the phase invocation message · copy · n/a, cited ·
    I3 · the P4 answers · copy · n/a, cited · inputs#I4 · the operator's word on the implement report and for the
    operator pass · copy · n/a, cited here and in `evidence/operator-pass.md` · inputs#I5 · the operator's wrap
    directive (a file outside this repository) · copy · unchanged, snapped and cited at this wrap · inputs#I6 · the
    wrap invocation message · copy · n/a, snapped and cited at this wrap. `verify` printed I4, I5 and I6 `UNCITED`
    because it ran before this report existed.

- **Reverted / negative API facts:** none shipped and removed. Two things were written during implement and changed
  before the first gate firing: the test stand-in in `unit_door.rs` (a lock held across a wait) and the owner's call
  in the other-account arm (a `ping`, then a `query_logs` over OTLP content, to match P-126's acceptance). Neither
  is product code. The four step-12 mutations were applied and put back (`evidence/mutations.md`).

- **Insufficient fixes (written, kept, not the remedy):** none.

- **Spec claims disproved by measurement:**
  1. **Plan step 9 and the plan's security acceptance: "an argument field no tool declares answers `-32602`" by
     "pass[ing] the tools' serde structs".** The argument structs of `crates/mcp-server/src/tools.rs`
     (`QueryTracesArgs`, `QueryMetricsArgs`, `QueryLogsArgs`, `GenerateSnapshotArgs`, `IncidentIdArgs`) carry no
     `deny_unknown_fields` and read past an unknown member; `dispatch_query_incident_list` parses no arguments at
     all (read in the source at implement). The manifest's `additionalProperties: false` is not enforced by the
     dispatcher. Disposition taken: the door refuses an undeclared member itself (`declared_only`); the sidecar is
     unchanged and still accepts one. Accepted by the operator as "the stricter and right form" (inputs#I4). It is a
     plan-artifact claim (no master states it); whether any master restates "arguments validated by serde structs"
     for the MCP surface is the detectors' to read.
  2. **Plan step 9: with the door directory still 0700 the client run as root "is refused by the kernel".** Measured
     false on `ci#38097928056`: root's connect went through at the 0700 directory and at the 0600 socket and was
     closed by the door (`Closed`); only `nobody` was refused by the kernel (`Kernel(13)`) at those two layers. The
     arm had been written to record each reading as measured. P-126's ledger acceptance says "one `door.refused`
     record … per client": the run holds four such records (root three times, `nobody` once), one per client RUN
     that reached the door. The P-126 ref states the four.
  3. **Research, Patterns detected: "How the kernel treats the socket file's mode is not read from a local source."**
     Now measured twice: on the dev host (Linux 7.2.5) the owner's connect to a socket of mode 000 failed with errno
     13; on the hosted runner `nobody`'s connect to the 0600 socket failed with errno 13.
  4. **Research: "A second account on this host: unmeasured."** Measured: `sudo -n true` fails on the dev host, so
     the arm prints its skip sentence there in every run.

- **Expected amendments (from plan)** — the sites were located at this wrap with one pass of case-insensitive greps
  over the seven masters and `.andromeda/registries/**` (hit counts as `architecture / security-plan / test-plan /
  obs-plan / design-system / layout-templates / a11y-plan / registries`): `sidecar` 9/11/16/7/1/1/2/5 ·
  `andromeda-pulse-mcp` 10/6/4/2/0/0/0/3 · `ANDROMEDA_PULSE_MCP_ENABLED` 1/5/5/3/0/0/0/1 · `workspace-key`
  1/3/4/2/0/0/0/0 · `app\.boot\.engine` 4/2/2/3/0/0/0/5 · `ANDROMEDA_PULSE_RETENTION_SECONDS` 3/4/0/0/0/0/0/0 ·
  `\bdoor\b` 1/0/0/0/0/0/0/0 (the one hit is `architecture.md:290`, a line on test-time telemetry injection) · `Unix-domain|unix
  socket|peer_cred` 1/0/0/0/0/0/0/0 · `Corpus Write Arbitration|seventh writer|seven production writers`
  1/0/0/0/0/0/0/0 · `mcp-test|mcp-server tests` 1/0/2/0/0/0/0/1 · `MCP server status` 0/0/0/0/1/1/0/0. The hits
  were counted, not read one by one; each line below names the section the plan names and the detectors read the
  sites.
  1. architecture §Occupied Resources → Network ports (the console engine's door socket) — **carried**: Symbols /
     APIs, the listening socket. `architecture.md:173` is the section head; the engine programs' listener sentence
     is among the `app\.boot\.engine` and `sidecar` hits there. Boundary widening: HALTS in the ordinary form
     (inputs#I5 item 3).
  2. architecture §Occupied Resources → Filesystem locations and → Environment variables (`run/door/` with its
     modes; `ANDROMEDA_PULSE_DOOR_RESOLVE`) — **carried**: Symbols / APIs and Schema / config. The sibling variable
     `ANDROMEDA_PULSE_RETENTION_SECONDS` has 3 hits in architecture and 4 in security-plan, which locates the
     variable registries. Same halt.
  3. architecture §Standard Contracts (a new entry, the door's form; `architecture.md:100`) and §Established
     Decisions [MCP Server Surface] (`:60`) and [Corpus Write Arbitration] (`:78`) — **carried**: the call surface,
     the error answers and the bounds above; the door's resolve reaches `update_incident_status` through
     `dispatch_mark_incident_resolved` as the sidecar's does (the arm
     `the_door_resolves_an_incident_and_says_it_was_a_write`). The contract's form is in the plan's Implementation
     notes; the undeclared-member refusal is an addition to it found at implement, and the `write` member of
     `tools/list` (plan step 2) is not in the notes' form.
  4. security-plan §Threat Model Summary (`:18`), §Authentication & Authorization (`:115`), §Input Validation
     (`:121`) and §Security Anti-Patterns → Input (`:391`) — **carried**: the owner check as the first
     caller-identity mechanism, the door's inputs and bounds, the product-read variable, and the workflow step that
     runs a client as a second account (Harness / gate surface). Boundary widening, PROVISIONAL, one item awaiting
     the founder's word; same halt.
  5. security-plan §Security Anti-Patterns → Logging (`:431`) — **carried**: the three targets and that no record
     holds the socket's path, an account id or an error's text. The siblings are the `app\.boot\.engine` hits (2 in
     security-plan).
  6. obs-plan §6 Log Coverage (`:297`) and §8 PII Scrubbing (`:404`), both sites, and §4 Scenario P3 (`:215`) —
     **carried**: the three targets with their fields and levels; the tool path's second carrier. `app\.boot\.engine`
     has 3 hits in obs-plan and 5 in the registries (3 files), which locates the dual-site rows.
  7. test-plan §1 Test Scope Summary (`:20`), §3 Per-chunk gate discipline (keyed:
     `.andromeda/registries/contracts/test-plan/per-chunk-gate-discipline.md`) and §9 CI Integration (`:484`) —
     **carried**: the door as a surface under test, the second-account arm and its step, the count 109 → 112. What
     the three sidecar pending rows of §1 (`:112` on) still hold once their subject is also read through the door
     was not decided at implement: nothing of the sidecar was removed.
  No ledger-note entry is in the plan's list. The dated notes for P-092 and P-093 (advanced, not claimed) are the
  operator's directive (inputs#I5 item 6): **ledger-note — owner P7.3**.

- **Coverage of new surfaces:**
  - `run/door/sock` (the listening socket, accept path) → validation mechanism✓ (kernel peer account, compared in
    one function; 0700 directory, 0600 socket) · instrumentation log✓ (`app.boot.door`, `door.refused`) · PII n/a
    (closed labels only) · tests unit + integ + spawned, and a real second account on the runner · a11y n/a ·
    tokens n/a
  - `tools/call` through the door (the nine tools) → validation mechanism✓ (closed tool set, declared members only,
    then the tools' serde structs; request and answer size, time) · instrumentation log✓ (`door.call`, eight
    fields, none redacted on a real call) · PII redacted✓ (no argument, answer or error text in the record; marker
    0 times in the log family) · tests unit + integ + spawned · a11y n/a · tokens n/a
  - `mark_incident_resolved` through the door (the one write) → validation mechanism✓ (the switch, before the
    dispatcher; then the guarded status update) · instrumentation log✓ (`write` true on its record) · PII n/a ·
    tests unit + integ (applied in process) + spawned (refused with the switch off or rejected; with it on the
    call reaches the tool and answers `-32603`, no such incident) · a11y n/a · tokens n/a
  - `ANDROMEDA_PULSE_DOOR_RESOLVE` → validation mechanism✓ (closed set, fail closed, never logged) ·
    instrumentation log✓ (`resolve_write` on the boot record) · PII n/a · tests unit + spawned · a11y n/a · tokens
    n/a
  - CI step `Door refuses another account` → validation n/a · instrumentation n/a · PII n/a · tests pins +
    script witnesses · a11y n/a · tokens n/a
  - A timed-out call's interrupt of a real running query → tests ✗ (the stand-in's interrupt is counted in
    process; no query of this chunk runs long enough; the plan names `Door admission lifecycle and bounds` as
    owner)

## Deviations from intent

Each was in the implement report and accepted by the operator "as recorded" (inputs#I4).

1. **The door refuses an undeclared argument member itself** (Spec claims disproved 1). In `door.rs`, a listed file;
   the sidecar and the tools crate's argument structs are untouched.
2. **The other-account arm records each of its six readings as measured** and asserts: the kernel's refusal for
   `nobody` at the closed directory; that no client of another account got a byte; that with both layers opened
   both clients are closed by the door; one `door.refused` record per client that reached the door; one `door.call`,
   the owner's. It prints the three-layer sentence only when `nobody` was refused at the socket's mode too, else a
   sentence saying the mode did not refuse, which fails the CI step. The plan had root refused by the kernel.
3. **The owner's call in that arm** is a `query_logs` returning a log record sent over OTLP (P-126's acceptance),
   where plan step 9 says only that the owner's call "is answered".
4. **The five door arms of `integration_engine_boot.rs` run on runtimes whose threads carry the arm's name**
   (`run_door_arm`, `:448-465`), the capture keeps the emitting thread's name, and the four older boot arms count
   without those threads. Door records carry no arm identity by design, so a bare process-wide capture cannot tell
   arms apart under in-process libtest.
5. **Two test functions renamed** in `integration_console_engine.rs`:
   `the_engine_listens_on_its_two_loopback_receivers_and_on_nothing_else` →
   `…_receivers_and_its_door_and_on_nothing_else`;
   `a_rejected_grpc_port_leaves_that_receiver_unstarted_and_the_http_receiver_alone_listening` →
   `…_and_the_http_receiver_and_the_door_listening`. The old names asserted what is no longer true.
6. **One tool call at a time inside the door** (a `tokio::sync::Mutex` held for the call), so the interrupt after a
   timeout reaches only the call that passed its bound. The plan names the interrupt, not the turn.
7. **`#[cfg(unix)]` on the socket code**, after `engine_boot.rs`'s own `#[cfg(unix)]` precedent. The plan is silent.
8. **A write to a caller that does not read is bounded by the idle bound** and then leaves an `idle` refusal record
   beside the call record. The plan bounds reads only.
9. **With no buffer connection every tool call answers `-32603`**, the corpus tools included: `call_tool` takes the
   connection for every tool, and the door may open none.
10. **The write switch's positive arm on a spawned engine** is "the write reaches the tool" (`-32603`, no such
    incident, record `tool_error` with `write` true). An applied write on a spawned engine would need an incident
    formed there; the applied write is shown in process.
11. **P3 did not drive a third boot.** The plan's smoke entry and engine-cycle entry are whole boot-to-cleanup runs
    and fired at P2; P3 read their kept logs.
12. **P-126's ref was written after the operator pass**, not at /implement: its acceptance names the CI run id.

Scope record: none — `gate.py scope` clean at /implement P4 and again at this wrap (`changed 16 · listed 16 ·
recorded 0`), no `scope-record.md`.

## Decisions & corrections

Raw material for curation. The implement conversation ends with this window; what is not written here is gone.

**The operator's words in the conversation.**
- On the implement report (inputs#I4): "Deviations accepted as recorded - the door refusing an undeclared field
  itself is the stricter and right form." A ruling on this chunk's door, not a standing rule.
- For the operator pass (inputs#I4): its order; "then write the P-126 ref with the run id"; on a red or skipped
  runner reading "stop and report what the job kept whole - the arm own lines, the door.refused records; fix
  nothing on top. Start no skill - the wrap is mine to call." The pass read green, so the stop arm did not fire.
- For this wrap (inputs#I5, inputs#I6): two windows; the widening halts in full; the removal entry minted now; the
  stdio gap takes an owner at the card; P-126's flip is the coverage step's; found-and-not-owned goes to the card.
- No "from now on" convention was given in this conversation.

**Decisions made at implement, with their reasons.**
- Fixed error messages for every refused or failed call (the plan's acceptance). Consequence: "incident not found"
  and a real fault both read `tool failed` to the caller.
- The `door.call` record is written before the answer line, so a write that was applied always has its record even
  when the caller is gone.
- A peer whose account cannot be read is refused as `other_account` / `other` (fail closed).
- The interrupt handle is kept only for a read connection of the door's own: where `read_connection` fell back to
  the engine's shared connection, an interrupt would end the engine's own statement.
- The CI step's witnesses shadow `cargo` with a shell function prepended to the step's script, so the step's text
  runs unmodified with no executable stand-in on disk.
- The arm's body was run once on the dev host under a stand-in named `sudo` first on `PATH` that runs the client as
  the caller: every client was answered and the arm read red, a known positive of its "no byte" assertion
  (`evidence/other-account-control.md`). The first reading on a runner then passed on attempt 1.

**Mistakes of this session and what caught them.**
- A test stand-in took a value out of a mutex inside an `if let` scrutinee
  (`if tool == X && let Some(r) = self.block.lock().unwrap().take()`). Under edition 2024 the guard lives through
  the then-block, where the stand-in waits; the next call blocked on the lock. One red run (18 of 19). Fixed by
  taking the value in a statement of its own.
- The other-account arm was written to the plan step, not to the claimed cap's acceptance in the ledger. Seen only
  when the ledger was read after the first green block; cost one edit and a second firing of the whole block. The
  ledger entry of a claimed cap is read BEFORE its proving test is written.
- The plan's "refused by the tools' serde structs" was believed until the spawned arm was being written; a grep for
  `deny_unknown_fields` over the argument structs answers it in one call.
- A recursive `grep -rn` over the whole work tree walked `target/` and passed the 120 s bound; `git grep` over
  tracked files answered in under a second.
- In this wrap window a command opening with `cd` into `.andromeda/` was refused by the Bash guard and re-issued
  once as a `( cd … && … )` subshell. This is the recurrence the handoff already tracks (`host-linux.md` Session
  Additions 2026-10-05), once more.

**Techniques that worked, each measured here.**
- Telling arms apart in a process-wide `tracing` capture when records carry no identity: build the arm's runtime
  with `thread_name(arm)`, run the whole arm inside `tokio::spawn` (so a record written synchronously during boot
  lands on a named worker too), capture `std::thread::current().name()`, filter by it. Green under nextest and
  under in-process `cargo test`.
- A blocking tool stand-in must be releasable: the runtime's drop waits for `spawn_blocking` tasks, so the arm holds
  a channel sender and drops it at its end.
- A client that the door has already closed fails its next write: a test caller sends a line and its terminator in
  ONE write and treats a failed write or a reset read as "closed".
- A door answer written just before close reaches the caller even with unread bytes in the door's receive queue
  (the over-long-line arm reads the `-32600` line, then the close).
- `cargo nextest … --success-output immediate` is what makes a PASSING test's printed lines readable by a step
  that greps them.
- Reading a hosted job's log: `gh api --allow-escape-sequences …/jobs/{id}/logs` to a scratch file, strip the
  colour escapes and the leading timestamp, count a known-present line as the control, then grep.

**Sweep hazards found.**
- **A Unix socket path must be shorter than 108 bytes, and the session scratchpad's path is longer than that.** A
  scratch socket under the scratchpad fails with `OSError: AF_UNIX path too long`, whose `errno` is `None` in
  Python (a client that prints `e.errno` prints `None`); a short directory directly under the temp root works.
- **Root is not held by a directory's or a socket's mode.** A test of "another account is refused by the modes"
  that runs its client as root reads the account check, not the modes; only an unprivileged account reads the
  modes. Measured on the runner.
- **`for_target` and the door's targets:** `app.boot.door` and `door.call` resolve only through their exact
  leaves; narrowing the `door.call` leaf by one field left the resolve pin green and only the set-equality pin red
  (mutation 2), the shape `rules/observability.md` already records.
- **A grep for the ignore attribute or for `sudo` over `ci.yml`:** the step's comment names sudo in prose; the pin
  reads the step's script, not its comment, so the comment may say it.
- **`evidence/mutations.md` and `evidence/other-account-control.md` cite test lines as read at that run**
  (`integration_engine_boot.rs:567`, `:665`; `integration_door.rs:757`). They are readings of a run, not live
  coordinates.

**For the main overseer (inputs#I5 item 9).** No `REFUSED id:` and no `REFUSED ts:` occurred in any evolve append
of the implement session (three appends, nine records, each landed first time) or, so far, of this window. The citation sweep is the
second window's.

**The last wrap's handoff (inputs#I5 item 7).** `git show 303fa9a8:.claude/session-handoff.md` holds 0 lines
matching `cascade` or `next wrap` (case-insensitive grep, exit 1). It names nothing as owed to this wrap's cascade.
What it carries that this chunk's facts touch, for the second window to read against the cascade: the test-file
count stated in `rules/testing.md` (109), and the either-or on the capability record's P-040, which this chunk
settled by stating the sentence.

**Found and not owned — for the route-resolve card (inputs#I5 item 8), each with a lean.**
1. The door's `initialize` answer names the server `andromeda-pulse-mcp`. Lean: the removal entry.
2. The tool list's descriptions speak the sidecar's language through the door ("the in-memory buffer"; for the
   resolve, "the main app's in-memory registry reflects the change on next launch", where the engine's reconcile
   step does it within one persist cycle). Lean: the removal entry.
3. The tools' argument structs read past an unknown member; the door's own check is the only refusal. Lean: the
   removal entry (once the sidecar is gone, one of the two forms is chosen).
4. A failed tool call reads `tool failed` whatever failed. An agent cannot tell "no such incident" from a fault.
   Lean: `Door one incident whole` (`working-route.md:130`), which widens what the door says of an incident.
5. With no buffer connection the corpus tools are unanswered too. Lean: `Door admission lifecycle and bounds`
   (`:98`).
6. The socket file outlives the engine and is replaced at the next boot. Lean: `One place on a node`, beside the
   lock file it already carries.
7. That an interrupt ends a running query is unshown. The plan already names `:98` as owner.
8. The window app reads `ANDROMEDA_PULSE_DOOR_RESOLVE` through `EngineConfig::from_env` and uses nothing of it.
   Lean: leaves with `Window retired`; the variable's registry line says which program uses it.
9. `rules/security.md` (Session Additions, 2026-05-12) says `mcp-server` is "the only optional crate" of
   `pulse-app`; it is no longer optional. A preserve-verbatim home: carried, not corrected, unless the operator
   says otherwise.
10. On a system that is not Unix no door is seated. Lean: `Other operating systems retired from the code`.
11. The stdio gap the P5 card named (inputs#I5 item 5): after the removal a same-host stdio agent has no
    first-class way to the socket. Dispositions to present: a carry on `Door reachable from another host` (`:100`),
    a carry on the removal entry, or an entry of its own. Lean: a carry on `:100`, which already owns where an
    admitted party's credential lives on the developer's machine; a new command word of the engine is a widening
    and is not chosen at the card.

**Recorded at the wrap's widening halt, second window (inputs#I7; added on the operator's word there).** The
registration of `ANDROMEDA_PULSE_DOOR_RESOLVE` matched two playbook rules with opposite verdicts: `New env var
registration` (routine: the input is code-validated and unit-tested) and `Boundary widening` (escalate). The
discriminating fact: the variable is the switch of the write that crosses the newly widened boundary. It went to the
halt with both rules cited and was applied PROVISIONAL with the door. The operator declined a discriminator rule for
the playbook ("no playbook rule"), so a later variable of this kind is judged again; the collision rides the handoff
as a deferred learning.

## Outcome

**Acceptance criteria, each against the diff.**
- (arch) The door is started from `engine_boot::start` and nowhere else — **met**: `door::open(` is in the seam's
  wired set of 18 and refused in the three entry-point files.
- (arch) A spawned console engine's listeners are exactly the two receivers and the one `unix` listener; the
  window seats no door — **met**: set equality in both census arms; `the_window_program_seats_no_door`; the window
  smoke's kept logs hold 0 door records and no `run/door`.
- (arch) The door's resolve goes through the guarded status update — **met**: the arm reads the row `resolved`
  through the engine's own `CorpusWriter`.
- (security) Every door argument passes the tools' serde structs, an unknown tool and an undeclared field are
  refused, and the store probe reads 0 — **met, by a different mechanism for the undeclared field** (the door's own
  check; Spec claims disproved 1). The probe entry reads `exit 1`, last line `0`.
- (security) Marker 0 times, no full path, each target's field set equals its leaf — **met**.
- (security) A refused or failed call answers one of the stated codes with a fixed message — **met**; `-32600` for
  an over-long line is among the stated codes of the plan's contract form.
- (security) `cargo deny check bans licenses sources` and `cargo audit` exit 0, `deny.toml` unedited — **met**.
- (obs) No field of `door.call` renders redacted on a real call; the guards pin exact resolve, set equality and
  `for_target("door").is_none()` — **met**.
- (obs) A refusal of another account, a request outside the tool set and an answer cut at its bound each leave one
  WARN record with a closed label and no caller-supplied text — **met**; the first of the three on the runner only.
- (obs, tests) The engine-cycle entry reads `pass`, 0 error records, `engine-log: PASS`, `cleanup: clean`,
  `"state":"listening"`, no call or refusal record — **met**, in both firings and on the runner's cycle step.
- (tests) Every entry reads as its `expect` says, the workspace run returns 0, the repeated run is seven of seven —
  **met** (below).
- (tests) Size bound and write switch beside their positive arm on a spawned engine, the time bound in process,
  four mutations red — **met**, the switch's positive arm in the form of Deviation 10.
- (tests) `cargo xtask verify:capability-matrix` exits 0 on the restated P-040 — **met** (82 ids, 36 claimed, 46
  retired, 0 violations).
- (tests) Coverage at or above 75 % line and 85 % function on the pass's run, nothing lowered, no retry, no ignore
  attribute — **met**: `Line: 35521/39153 = 90.7%`, `Function: 3750/4186 = 89.6%` on `ci#38097928056`.
- (tests, operator leg) **P-126** — **met on the run, with one count to read at the coverage step**: the step
  passed, the arm connected as `nobody` and as root, both were closed by the door with both layers opened, the
  owner's `query_logs` returned OTLP content. "One `door.refused` record per client" holds per client run that
  reached the door: four records, not two (Spec claims disproved 2). The ref says so. P-126 is `implemented`.
- (design, layouts, a11y) No screen, window, tray item, notification or control; no webview file — **met**: the
  scope guard prints nothing.

**Gates** (the plan's block; verdicts are the gate tool's words from the second firing at /implement on the final
tree, `entries 26 · green 22 · red 0 · not-run 4`; the first firing read the same).
- `cargo fmt --check` — green · exit 0
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — green · exit 0
- `git diff --name-only 303fa9a8… -- crates pulse-app xtask scripts … ':!…'` (the scope guard) — green · exit 0 ·
  no output
- `cargo nextest run … -E 'binary(unit_door) | binary(unit_observability_allowlist_door) |
  binary(integration_door)'` — green · exit 0 (30 tests; the other-account arm skipped on this host)
- `grep -cE 'open_in_memory|Corpus::open|read_published_workspace_key|SELECT |\.prepare\(' pulse-app/src/door.rs` —
  green · exit 1 · last line 0
- `for i in 1 2 3 4 5 6 7; do cargo nextest run … five binaries … || exit 1; done` — green · exit 0, seven of
  seven, 48 tests each (`evidence/repeated-run.md`)
- `cargo deny check bans licenses sources` — green · `cargo audit` — green
- `cargo xtask check:english-sources` — green · contains `"verdict": "clean"`
- `cargo xtask quarantine-tracking` · `capability-widening-check` · `check:ingest-progress` ·
  `check:staged-artifacts` · `capability-drift` · `verify:capability-matrix` — each green · exit 0
- `cargo nextest run --workspace --profile ci --no-tests=fail` — green · 3174 passed
- `cargo nextest run -p pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'` — green
- `git diff --quiet 303fa9a8… -- pulse-app/ui/src/bindings/index.ts` — green · exit 0
- `d="target/boot-smoke/…-series" && … cargo xtask harness:boot-series --count 2 && grep …` (the window smoke) —
  green · `all-settled`, ordinals 2 and 3, `"program":"window"`, artifact fresh
- `d="target/engine-cycle/…-door" && cargo xtask harness:engine-cycle … && grep … && ! grep …` — green · `pass`,
  `"state":"listening"`, artifact fresh
- `d="$(mise where node@24)" && PATH="$d/bin:$PATH" cargo xtask pre-push:linux` — green · `all-stages-ok`; fired a
  third time at the operator pass on the committed tree, green (`head 2e4ee5ec`, `tree beda4dbf`)
- `test ! -e target/pre-push/run/data/run/andromeda-pulse.pid && ! (…24317) && ! (…24318)` — green
- `leg = 'operator'` entries, from `evidence/operator-pass.md`: `gate.py hygiene` — fired as written, `hygiene:
  clean` · the merge-base probe — green, history unmoved (also once at /implement, `evidence/operator-entry-24.txt`)
  · `git diff --quiet && git diff --cached --quiet && git push origin build/andromeda-pulse-0.4.0` — green, history
  moved `303fa9a8 → 2e4ee5ec` on the build branch's remote-tracking ref, the intended move · `ci.py conclusion
  --sha HEAD --wait 2700` — fired as written, exit 0, `verdict: green · checks 7/7`.
- No entry carries `defer`. No deferral was taken.
- Smoke: Deviation 11. Window: two boots settled, 0 door records, 0 ERROR. Console engine: 1 `app.boot.door`
  `listening`, 0 `door.call`, 0 `door.refused`, directory 700, socket 600, socket path 99 bytes.

**Watches:** none — the entry folded no `watch:`.

**Outcome basis.** The operator pass ran: the verdicts rest on its final state, `2e4ee5ec` and `ci#38097928056`
(`evidence/operator-pass.md`), which P7 re-verifies. What only /implement holds — the deviations, the mutation
readings, the control run, the census — is from the implement conversation in this same window, as given in its P4
report. The operator's directive between implement and this report (inputs#I4) changed nothing in the tree; it
accepted the deviations and ordered the pass. After the push the tree took three changes that ride this wrap's
commit: `verification-matrix.json` (P-126 `planned → implemented`), the ledger tool's trail in the implement run
dir, and `evidence/operator-pass.md`.

**Process hygiene.** From /implement's census and re-measured after the operator pass against the host's process
list (`ps -eo pid,comm,args`, no `andromeda-pulse-engine`, `target/release/pulse-app`, `inject_demo` or `Xvfb`
row): window app boots and Xvfb of the two boot series — terminated; the console engines and injectors of the two
engine cycles and of the three pre-push sixth stages — terminated, `cleanup: clean`; test-spawned engines of the
nextest entries, the mutation runs and the control run — terminated. Ports 14317, 14318, 24317 and 24318 refuse.
Started at this wrap: `python scripts/code-graph.py refresh`, in the background, by Setup step 7; its
`.refresh-done` marker was present when this report was written. Left on the host, none in the tree: two small
scratch directories in the session's temp dir; the kept data dirs under `target/boot-smoke/` and
`target/engine-cycle/`; the job logs fetched to the session scratchpad.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 303fa9a8 (the parent of the oldest pre-CI commit 2e4ee5ec) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/workflows/ci.yml — added 26 line(s) in 1 range(s)
added: 114-139
### crates/mcp-server/src/jsonrpc.rs — added 28 line(s) in 11 range(s)
added: 124 · 130 · 144 · 158 · 172 · 185 · 195 · 208 · 221 · 234 · 396-413
  - 396-412 @397 «fn tools_list_manifest_marks_exactly_one_tool_as_a_write() {»
### crates/mcp-server/src/tools.rs — added 58 line(s) in 6 range(s)
added: 159-186 · 199 · 203 · 207-208 · 216 · 230-254
- 159-166 @162 «pub struct ToolAnswer {»
- 168-185 @172 «pub fn call_tool(»
  - 180-184 «Ok(ToolAnswer {»
  - 240-254 «match tool_name {»
### docs/capability-record.json — added 7 line(s) in 4 range(s)
added: 810-811 · 814 · 818-820 · 823
### pulse-app/Cargo.toml — added 1 line(s) in 1 range(s)
added: 87
### pulse-app/src/door.rs — new file · 749 line(s)
- 43-50 @46 «pub enum ResolveWrite {»
- 52-74 «impl ResolveWrite {»
  - 53-61 @55 «pub fn from_value(raw: Option<&OsStr>) -> Self {»
  - 63-69 «pub fn label(self) -> &'static str {»
  - 71-73 «pub fn allows_write(self) -> bool {»
- 76-84 @78 «pub enum EndpointState {»
- 86-96 «impl EndpointState {»
  - 87-95 «pub fn label(self) -> &'static str {»
- 98-110 @100 «pub enum Outcome {»
- 112-126 «impl Outcome {»
  - 113-125 «pub fn label(self) -> &'static str {»
- 128-135 @130 «pub enum Refusal {»
- 137-146 «impl Refusal {»
  - 138-145 «pub fn label(self) -> &'static str {»
- 148-152 @149 «pub enum Peer {»
- 154-161 «impl Peer {»
  - 155-160 «pub fn label(self) -> &'static str {»
- 163-167 @165 «pub fn admits(peer_account: u32, own_account: u32) -> bool {»
- 169-180 @171 «pub struct CallRecord {»
- 182-201 @184 «pub fn emit_boot(state: EndpointState, resolve: ResolveWrite) {»
  - 186-200 «if state == EndpointState::Listening && resolve != ResolveWrite::Rejected {»
- 203-234 @205 «pub fn emit_call(record: CallRecord) {»
  - 207-233 «if record.outcome == Outcome::Ok {»
- 236-244 @237 «pub fn emit_refused(reason: Refusal, peer: Peer) {»
  - 238-243 «tracing::warn!(»
- 246-253 @247 «pub trait Tools: Send + Sync + 'static {»
- 255-266 @258 «pub struct EngineTools {»
- 268-291 «impl EngineTools {»
  - 269-290 «pub fn new(»
- 293-317 «impl Tools for EngineTools {»
  - 294-310 «fn call(&self, tool: &'static str, arguments: &Value) -> Result<ToolAnswer, ToolError> {»
  - 312-316 «fn interrupt(&self) {»
- 319-749 @320 «mod socket {»
  - 329-332 «use mcp_server_crate::jsonrpc::{»
  - 340-344 «use super::{»
  - 359-364 @360 «pub fn own_account() -> u32 {»
  - 366-407 @369 «pub fn bind_endpoint(data_dir: &Path) -> Result<UnixListener, EndpointState> {»
  - 409-432 @412 «pub fn open(data_dir: &Path, tools: Arc<dyn Tools>, resolve: ResolveWrite) -> Option<Door> {»
  - 434-439 «pub struct Door {»
  - 441-469 «impl Door {»
  - 471-475 «enum Line {»
  - 477-494 «async fn read_line(»
  - 496-500 «async fn write_line(writer: &mut OwnedWriteHalf, bytes: &[u8]) -> std::io::Result<()> {»
  - 502-550 «async fn serve_connection(stream: UnixStream, calls: &Calls) {»
  - 552-561 @553 «struct Answered {»
  - 563-565 «fn error_line(id: Value, code: i64, message: &str) -> Vec<u8> {»
  - 567-585 @568 «fn refusal_of(outcome: Outcome) -> (i64, String) {»
  - 587-604 «fn refused(»
  - 606-631 @607 «fn answered(»
  - 633-646 @636 «fn declared_only(tool: &str, arguments: &Value) -> bool {»
  - 648-654 «struct Calls {»
  - 656-748 «impl Calls {»
### pulse-app/src/engine_boot.rs — added 34 line(s) in 9 range(s)
added: 68 · 86-87 · 134-140 · 368 · 374 · 378-380 · 387 · 484 · 1037-1053
- 134-139 @137 «pub fn resolve_door_write() -> ResolveWrite {»
  - 1037-1051 @1042 «if program == Program::Console {»
### pulse-app/src/lib.rs — added 1 line(s) in 1 range(s)
added: 14
### pulse-app/src/observability.rs — added 24 line(s) in 1 range(s)
added: 1022-1045
### pulse-app/tests/integration_console_engine.rs — added 26 line(s) in 8 range(s)
added: 434-447 · 511-512 · 525 · 531-535 · 576 · 582 · 591 · 593
- 434-446 @438 «fn door_listener(data_dir: &Path) -> (&'static str, String) {»
  - 439-444 «let socket = data_dir»
### pulse-app/tests/integration_door.rs — new file · 802 line(s)
- 34-43 «const CALL_FIELDS: [&str; 8] = [»
- 46-48 «fn loopback(port: u16) -> SocketAddr {»
- 50-52 «fn accepts(port: u16) -> bool {»
- 54-62 «struct Running {»
- 64-71 «impl Drop for Running {»
  - 65-70 «fn drop(&mut self) {»
- 73-182 «impl Running {»
  - 74-108 «fn spawn(extra_env: &[(&str, &str)]) -> Self {»
  - 110-112 «fn socket(&self) -> PathBuf {»
  - 114-116 «fn still_running(&mut self) -> bool {»
  - 118-132 @119 «async fn wait_for_log(&mut self, what: &str, condition: impl Fn(&[Value]) -> bool) {»
  - 134-153 @136 «async fn wait_until_ready(&mut self) {»
  - 155-177 «async fn end_by_sigterm(&mut self) -> Vec<Value> {»
  - 179-181 «fn stream(&self, name: &str) -> String {»
- 184-198 «fn family_paths(dir: &Path) -> Vec<PathBuf> {»
  - 185-187 «let Ok(entries) = std::fs::read_dir(dir.join("logs")) else {»
  - 188-195 «let mut paths: Vec<PathBuf> = entries»
- 200-214 «fn family_lines(dir: &Path) -> Vec<Value> {»
  - 202-212 «for path in family_paths(dir) {»
- 216-222 @217 «fn family_text(dir: &Path) -> String {»
  - 218-221 «family_paths(dir)»
- 224-229 «fn records<'a>(lines: &'a [Value], target: &str) -> Vec<&'a Value> {»
  - 225-228 «lines»
- 231-237 «fn appended(lines: &[Value], table: &str) -> u64 {»
  - 232-236 «records(lines, "duckdb.append")»
- 239-246 «fn assert_no_log_record_in(stream: &str, name: &str) {»
  - 240-245 «for line in stream.lines() {»
- 248-253 «fn now_ns() -> u64 {»
  - 249-252 «SystemTime::now()»
- 255-262 «fn tool_call(id: u64, name: &str, arguments: Value) -> Value {»
  - 256-261 «json!({»
- 264-281 @265 «async fn ask(socket: &Path, request: Value) -> Value {»
  - 266-268 «let stream = tokio::net::UnixStream::connect(socket)»
  - 270-273 «writer»
  - 275-278 «let read = tokio::time::timeout(ANSWER_WAIT, BufReader::new(read_half).read_line(&mut line))»
- 283-300 @284 «async fn send_logs(engine: &Running, service: &str, body: &str, count: usize) {»
  - 285-287 «let mut client = LogsServiceClient::connect(format!("http://{}", loopback(engine.grpc)))»
  - 290-299 «for batch in stamps.chunks(200) {»
- 302-312 @304 «fn own_fields(record: &Value) -> BTreeSet<String> {»
  - 305-311 «record["fields"]»
- 314-322 «fn assert_nothing_redacted(record: &Value) {»
  - 315-321 «for (name, value) in record["fields"].as_object().expect("fields object") {»
- 324-333 «fn the_one_door_boot_record(lines: &[Value]) -> &Value {»
  - 328-331 «assert_eq!(»
- 335-408 @336 «async fn the_owner_is_answered_with_what_the_engine_holds_and_the_call_record_holds_none_of_it() {»
  - 343-347 «engine»
  - 350-352 «let items = answer["result"]["items"]»
  - 353-356 «assert!(»
  - 369-373 «assert_eq!(»
  - 386-390 «assert_eq!(»
  - 392-398 «let resolved = engine»
  - 399-405 «for path in [&full, &resolved] {»
- 410-482 @411 «async fn a_request_outside_the_tool_set_is_refused_beside_an_answered_call() {»
  - 422-425 «assert!(»
  - 429-432 «let undeclared = ask(»
  - 435-438 «let unknown_method = ask(»
  - 441-450 «for refused in [&unknown_tool, &undeclared, &unknown_method] {»
  - 454-463 «let outcomes: Vec<(&str, &str)> = records(&lines, "door.call")»
  - 464-473 «assert_eq!(»
  - 475-481 «for canary in [UNKNOWN_TOOL, UNDECLARED, UNKNOWN_METHOD] {»
- 484-537 @485 «async fn an_answer_over_the_size_bound_is_refused_and_the_same_query_at_a_small_limit_is_answered()»
  - 497-501 «engine»
  - 504-507 «let cut = ask(»
  - 510-513 «assert_eq!(»
  - 517-519 «let items = small["result"]["items"]»
  - 530-533 «assert!(»
- 539-601 @540 «async fn the_write_switch_is_read_from_the_environment_and_a_value_outside_the_set_leaves_it_off() {»
  - 552-555 «assert_eq!(»
  - 561-600 «for (value, label, level) in [("off", "off", "INFO"), (OUTSIDE, "rejected", "WARN")] {»
- 606-607 «const THREE_LAYERS: &str =»
- 616-617 «try:»
- 618-620 «except OSError as e:»
- 621-623 «try:»
- 624-625 «except OSError:»
- 629-636 @630 «enum Reading {»
- 638-646 «fn sudo(account: Option<&str>) -> Command {»
  - 641-643 «if let Some(account) = account {»
- 648-668 @650 «fn client_as(account: Option<&str>, socket: &Path) -> Reading {»
  - 651-655 «let output = sudo(account)»
  - 657-661 «assert!(»
  - 662-667 «match printed.split_whitespace().collect::<Vec<_>>().as_slice() {»
- 670-673 «fn set_mode(path: &Path, mode: u32) {»
  - 671-672 «std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode))»
- 675-681 @677 «fn reachable_by_another_account(path: &Path) -> bool {»
  - 678-680 «path.ancestors()»
- 683-802 @684 «async fn another_account_is_refused_at_each_of_three_layers() {»
  - 688-693 «let has_second_account = sudo(None)»
  - 694-697 «if !has_second_account {»
  - 701-704 «let socket = engine»
  - 713-717 «if !reachable_by_another_account(&data_dir) {»
  - 718-725 «assert_eq!(»
  - 736-739 «println!(»
  - 743-747 «engine»
  - 749-754 «assert!(»
  - 757-760 «assert!(»
  - 761-764 «assert!(»
  - 765-769 «assert_eq!(»
  - 773-776 «let reached_the_door = every»
  - 778-782 «assert_eq!(»
  - 783-788 «for refusal in &refusals {»
  - 795-801 «if socket_mode_refused {»
### pulse-app/tests/integration_engine_boot.rs — added 387 line(s) in 15 range(s)
added: 10-14 · 35 · 42 · 49-55 · 60-62 · 90-99 · 117 · 135 · 137 · 142-157 · 170-176 · 191-198 · 208 · 222 · 446-769
  - 174-176 «fn door_socket(&self) -> PathBuf {»
- 448-465 @451 «fn run_door_arm<F>(name: &'static str, arm: F)»
  - 456-461 «let runtime = tokio::runtime::Builder::new_multi_thread()»
  - 462-464 «if let Err(ended) = runtime.block_on(async move { tokio::spawn(arm).await }) {»
- 467-474 «fn tool_call(id: u64, name: &str, arguments: Value) -> Value {»
  - 468-473 «serde_json::json!({»
- 476-493 @477 «async fn door_ask(socket: &Path, request: Value) -> Value {»
  - 478-480 «let stream = tokio::net::UnixStream::connect(socket)»
  - 482-485 «writer»
  - 487-490 «let read = tokio::time::timeout(WAIT, BufReader::new(read_half).read_line(&mut line))»
- 495-518 @497 «async fn form_one_incident(engine: &EngineHandles) -> i64 {»
  - 499-501 «wait_until("both digest subscribers are listening", || {»
  - 503-507 «engine»
  - 510-512 «wait_until("one incident forms", || {»
  - 514-517 «engine»
- 520-528 «fn corpus_row(engine: &EngineHandles, id: i64) -> corpus::contract::IncidentRowRaw {»
  - 521-527 «engine»
- 530-535 «fn field<'a>(fields: &'a BTreeMap<String, String>, name: &str) -> &'a str {»
  - 531-534 «fields»
- 537-590 @538 «fn the_door_answers_from_the_stores_the_boot_built() {»
  - 541-589 «run_door_arm(ARM, async {»
- 592-641 @593 «fn the_door_resolves_an_incident_and_says_it_was_a_write() {»
  - 595-640 «run_door_arm(ARM, async {»
- 643-701 @644 «fn the_door_refuses_the_write_with_the_switch_off() {»
  - 646-700 «run_door_arm(ARM, async {»
- 703-746 @704 «fn an_incident_forms_with_the_door_listening_and_no_reader() {»
  - 706-745 «run_door_arm(ARM, async {»
- 748-768 @749 «fn the_window_program_seats_no_door() {»
  - 751-767 «run_door_arm(ARM, async {»
### pulse-app/tests/quality_gate_workflow.rs — added 182 line(s) in 2 range(s)
added: 8-14 · 937-1111
- 939-940 «const DOOR_THREE_LAYERS: &str =»
- 943-968 @944 «fn ci_workflow_runs_the_door_other_account_arm_after_the_test_step_with_no_soft_fail() {»
  - 947-953 «let position = |name: &str| {»
  - 955-959 «assert!(»
  - 961-967 «for key in ["continue-on-error", "if:", "uses:"] {»
- 970-1017 @971 «fn the_door_other_account_step_runs_its_one_arm_and_reads_the_arms_own_sentence() {»
  - 973-976 «let cargo_lines: Vec<&str> = script»
  - 977-981 «assert_eq!(»
  - 983-987 «assert!(»
  - 988-992 «assert!(»
  - 993-997 «assert!(»
  - 998-1001 «assert!(»
  - 1004-1005 «let arm = std::fs::read_to_string(&arm_path)»
  - 1006-1011 «for text in [DOOR_OTHER_ACCOUNT_ARM, DOOR_SKIPPED] {»
  - 1013-1016 «assert!(»
- 1019-1044 @1022 «fn run_door_other_account_step(captured: &str, arm_status: i32) -> (Option<i32>, String) {»
  - 1030-1037 «let output = std::process::Command::new("bash")»
  - 1038-1042 «let printed = format!(»
- 1046-1053 «fn arm_output(sentence: &str) -> String {»
  - 1047-1052 «format!(»
- 1055-1063 @1056 «fn door_other_account_step_passes_an_arm_that_read_all_three_layers() {»
  - 1059-1062 «assert!(»
- 1065-1090 @1066 «fn door_other_account_step_fails_an_arm_that_skipped_or_read_a_layer_open() {»
  - 1067-1076 «let cases = [»
  - 1077-1089 «for (case, captured) in cases {»
- 1092-1100 @1093 «fn door_other_account_step_fails_an_arm_that_printed_nothing() {»
  - 1096-1099 «assert!(»
- 1102-1110 @1103 «fn door_other_account_step_fails_a_failed_arm_whatever_it_printed() {»
  - 1106-1109 «assert!(»
### pulse-app/tests/unit_door.rs — new file · 873 line(s)
- 22-25 «use pulse_app::door::{»
- 35-37 «fn socket_path(data_dir: &Path) -> PathBuf {»
- 39-45 «fn mode(path: &Path) -> u32 {»
  - 40-44 «std::fs::symlink_metadata(path)»
- 49-60 @50 «fn the_owner_predicate_admits_the_engines_own_account_and_no_other() {»
  - 53-56 «assert!(»
- 64-79 @65 «async fn the_directory_reads_0700_and_the_socket_0600() {»
  - 72-78 «assert!(»
- 81-92 @82 «async fn a_door_directory_left_wider_is_closed_before_the_bind() {»
  - 86-87 «std::fs::set_permissions(&door_dir, std::fs::Permissions::from_mode(0o755))»
- 94-127 @95 «async fn a_data_dir_too_deep_reads_path_too_long() {»
  - 104-107 «assert!(»
  - 112-115 «let Some(fill) = 107usize.checked_sub(fixed).filter(|fill| *fill >= 1) else {»
  - 123-126 «assert_eq!(»
- 129-144 @130 «async fn a_regular_file_at_the_sockets_path_reads_occupied() {»
  - 139-143 «assert_eq!(»
- 146-158 @147 «async fn a_live_listener_reads_in_use_and_keeps_listening() {»
  - 154-157 «assert!(»
- 160-181 @161 «async fn a_stale_socket_is_replaced() {»
  - 165-168 «assert!(»
  - 169-172 «assert!(»
  - 177-180 «assert!(»
- 183-201 @184 «async fn a_door_directory_that_leaves_the_data_dir_reads_bind_failed() {»
  - 188-189 «std::os::unix::fs::symlink(elsewhere.path(), dir.path().join("run").join("door"))»
  - 194-200 «assert!(»
- 205-236 @206 «fn the_switch_reads_on_when_unset_or_on_off_when_off_and_rejected_otherwise() {»
  - 211-215 «for outside in [»
  - 216-220 «assert_eq!(»
  - 224-227 «assert!(»
  - 228-235 «assert_eq!(»
- 238-249 @239 «fn the_bounds_and_the_three_codes_read_as_stated() {»
  - 245-248 «assert_eq!(»
- 256-266 «fn global_capture() -> Captured {»
  - 258-265 «Arc::clone(CAPTURE.get_or_init(|| {»
- 268-270 «struct CapturingSubscriber {»
- 272-305 «impl tracing::Subscriber for CapturingSubscriber {»
  - 273-275 «fn enabled(&self, _: &tracing::Metadata<'_>) -> bool {»
  - 276-278 «fn new_span(&self, _: &tracing::span::Attributes<'_>) -> tracing::span::Id {»
  - 281-302 «fn event(&self, event: &tracing::Event<'_>) {»
- 307-321 @310 «fn captured(events: &Captured, target: &str, signature: &[(&str, &str)]) -> bool {»
  - 311-320 «events»
- 323-332 «async fn wait_until(what: &str, mut condition: impl FnMut() -> bool) {»
  - 325-331 «while !condition() {»
- 336-343 @339 «struct StandIn {»
- 345-386 «impl Tools for StandIn {»
  - 346-381 «fn call(&self, tool: &'static str, arguments: &Value) -> Result<ToolAnswer, ToolError> {»
  - 383-385 «fn interrupt(&self) {»
- 388-394 «struct Seated {»
- 396-413 «fn seat(resolve: ResolveWrite) -> Seated {»
  - 399-403 «let tools = Arc::new(StandIn {»
  - 404-405 «let seated = door::open(dir.path(), Arc::clone(&tools) as Arc<dyn Tools>, resolve)»
  - 407-412 «Seated {»
- 415-418 «struct Caller {»
- 420-457 «impl Caller {»
  - 421-430 «async fn connect(socket: &Path) -> Self {»
  - 432-439 @434 «async fn send(&mut self, line: &[u8]) {»
  - 441-451 @442 «async fn next(&mut self) -> Option<Value> {»
  - 453-456 «async fn ask(&mut self, request: Value) -> Value {»
- 459-466 «fn tool_call(id: u64, name: &str, arguments: Value) -> Value {»
  - 460-465 «json!({»
- 468-470 «fn ping(id: u64) -> Value {»
- 472-516 @473 «async fn the_four_methods_are_answered_and_a_tool_call_carries_the_tools_result() {»
  - 478-480 «let initialized = caller»
  - 484-486 «let listed = caller»
  - 487-490 «assert_eq!(»
  - 494-496 «let called = caller»
  - 501-515 «assert!(»
- 518-608 @519 «async fn a_request_outside_the_tool_set_is_refused_with_a_fixed_message_and_no_tool_is_called() {»
  - 530-532 «let unknown_method = caller»
  - 536-538 «let no_name = caller»
  - 546-550 «assert_eq!(»
  - 552-560 @554 «let undeclared = caller»
  - 563-565 «let no_object = caller»
  - 567-571 «assert_eq!(»
  - 573-581 @575 «let rejected = caller»
  - 584-586 «let failed = caller»
  - 590-607 «for answer in [»
- 610-630 @611 «async fn a_request_with_no_id_is_not_answered() {»
  - 615-621 «caller»
  - 625-629 «assert_eq!(»
- 632-690 @633 «async fn the_write_is_refused_before_any_tool_runs_when_the_switch_is_not_on() {»
  - 634-674 «for resolve in [ResolveWrite::Off, ResolveWrite::Rejected] {»
  - 678-684 «let applied = caller»
  - 685-688 «assert!(»
- 692-713 @693 «async fn an_answer_over_its_bound_is_an_error_naming_the_bound() {»
  - 701-704 «assert_eq!(»
  - 708-712 «assert!(captured(»
- 717-756 @718 «async fn an_over_long_line_is_answered_invalid_request_and_the_connection_closed() {»
  - 722-724 @723 «let frame =»
  - 725-730 «let padded = json!({»
  - 735-738 «let answered = at_bound»
  - 746-749 «assert!(»
  - 751-755 «assert!(captured(»
- 758-800 @759 «async fn a_ninth_connection_is_closed_at_accept() {»
  - 764-772 «for id in 0..MAX_CONNECTIONS as u64 {»
  - 775-778 «assert!(»
  - 779-783 «assert!(captured(»
  - 788-799 «loop {»
- 802-828 @803 «async fn an_idle_connection_is_closed_at_the_idle_bound() {»
  - 810-813 «assert!(»
  - 816-819 «assert!(»
  - 820-826 «wait_until("the idle refusal is recorded", || {»
- 832-873 @833 «async fn a_call_over_its_time_bound_answers_timed_out_and_the_next_call_is_answered() {»
  - 845-848 «assert!(»
  - 849-853 «assert_eq!(»
  - 859-862 «assert!(»
  - 863-867 «assert_eq!(»
  - 868-872 «assert!(captured(»
### pulse-app/tests/unit_engine_boot_seam.rs — added 2 line(s) in 2 range(s)
added: 21 · 39
### pulse-app/tests/unit_observability_allowlist_door.rs — new file · 265 line(s)
- 17-20 «use pulse_app::door::{»
- 27-33 «const STATES: [EndpointState; 5] = [»
- 35-45 «const OUTCOMES: [Outcome; 9] = [»
- 46-51 «const REFUSALS: [Refusal; 4] = [»
- 53-62 «fn leaf(target: &str) -> BTreeSet<String> {»
  - 54-61 «AllowList::production()»
- 64-69 @65 «fn the_emit_site_targets_are_the_registered_ones() {»
- 71-85 @72 «fn each_door_target_resolves_and_no_fallback_key_could_resolve_it() {»
  - 74-79 «for target in [BOOT, CALL, REFUSED] {»
- 90-102 @92 «fn global_capture() -> Captured {»
  - 94-101 «Arc::clone(CAPTURE.get_or_init(|| {»
- 104-106 «struct CapturingSubscriber {»
- 108-142 «impl tracing::Subscriber for CapturingSubscriber {»
  - 109-111 «fn enabled(&self, _: &tracing::Metadata<'_>) -> bool {»
  - 112-114 «fn new_span(&self, _: &tracing::span::Attributes<'_>) -> tracing::span::Id {»
  - 117-139 «fn event(&self, event: &tracing::Event<'_>) {»
- 144-160 @145 «fn emitted(target: &str, emit: impl FnOnce()) -> (BTreeMap<String, String>, tracing::Level) {»
  - 150-157 «let mine: Vec<_> = events»
- 162-164 «fn names(fields: &BTreeMap<String, String>) -> BTreeSet<String> {»
- 166-191 @167 «fn the_boot_record_emits_exactly_its_leaf_and_is_info_only_when_listening_with_the_switch_read() {»
  - 169-190 «for state in STATES {»
- 193-226 @194 «fn the_call_record_emits_exactly_its_leaf_and_is_info_only_for_ok() {»
  - 196-225 «for outcome in OUTCOMES {»
- 228-244 @229 «fn the_refusal_record_emits_exactly_its_leaf_at_warn() {»
  - 231-243 «for reason in REFUSALS {»
- 246-265 @247 «fn no_door_record_is_written_at_error_and_every_label_is_closed() {»
  - 248-250 «let closed = |label: &str| {»
  - 251-260 «for label in STATES»
  - 261-263 «let (_, worst_boot) = emitted(BOOT, || {»
