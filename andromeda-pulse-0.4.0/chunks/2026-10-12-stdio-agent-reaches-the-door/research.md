# Codebase Research — 2026-10-12-stdio-agent-reaches-the-door

## Scope
- **Depth:** moderate · **Reads:** 11 · **Globs/Greps:** 9
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read structurally (78 253 B, past the read cap): its body sections `## 5-command discipline` to `## Anti-patterns` in full (lines 1–116), its `## Session Additions` indexed by `grep -n '^- 20'` and the four entries bearing on a spawned-engine leg read whole (2026-06-29 log-family glob; 2026-10-10 ×3: library swap, `engine-log:` arm-line shape, summary-not-grep). Applied: read the log family `agent-latest.jsonl*`, never the base name; read a nextest count from its `Summary` line. `.claude/rules/testing.md` and `.claude/rules/observability.md` were loaded whole by the harness for this window.
- **Platform issues consulted:** none — no runner-only bullet (Setup 5a read `d3e694d2` green 7/7) and no CI-reading entry outside the operator leg.
- **External inputs:** `inputs#I1` — the operator's take-up message: this entry, stop at P4 with the forms and at P5 for the yes · `inputs#I2` — the phase directive: forms laid out at P4 with what each adds and a lean; the door's surface does not move; the sidecar stays; Conductor is untouched; P5 lists the classifier acts, planlint check 4 and P-092's concretization.

## Files inspected
- `pulse-app/src/door.rs` (full, 749 lines) — the door: `socket::bind_endpoint` (canonical data dir, `run/door/` 0700, `sock` 0600, under 108 bytes), `Door::serve` (kernel-read peer account against `geteuid`, closed with no byte on a mismatch), `serve_connection` (one line read under `IDLE_TIMEOUT` 30 s; `read_line` returning 0 bytes is `Line::End`, which returns with NO record; an idle timeout writes `door.refused {reason: idle}`), `Calls::answer` (a request with no `id` is not answered), `declared_only`, the write switch. The door holds no client code.
- `pulse-app/src/console.rs` (full) — the closed grammar `run` | `version` (`parse`, `main`, `USAGE`), `run()` builds the runtime, calls `engine_boot::init_process` then `engine_boot::start`.
- `pulse-app/src/engine_boot.rs` (186–350) — `resolve_data_dir` (`ANDROMEDA_PULSE_DATA_DIR` taken as written, no canonicalize, then the per-OS default); `write_pid_file` writes `run/andromeda-pulse.pid` under the data dir; `init_process` = log sink + guard, exit hook, signal listener, start instant, pid file.
- `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs` (full) — the sidecar's stdio loop: one JSON-RPC line in, one line out, `\n`-terminated, a notification (no `id`) unanswered — the same framing as the door's; it opens its own EMPTY in-memory DuckDB (`init_buffer_connection`), the corpus through the OS credential store and reads `run/workspace-key`; `:72` names Conductor as a process that spawns it; its `mark_incident_resolved` has no switch beyond the double gate.
- `crates/mcp-server/src/jsonrpc.rs` (102–113) — `initialize_result()` names `andromeda-pulse-mcp`, protocol `2024-11-05`; the door answers it unchanged.
- `crates/mcp-server/src/tools.rs` (172–197) — `call_tool` (no record) and `dispatch_tool` (the sidecar's) signatures.
- `pulse-app/tests/integration_door.rs` (1–340, arm list to 802) — the spawned-engine harness this chunk's arm sits beside: `Running::spawn` (cleared env, own TempDir, two claimed ports, passphrase), `wait_until_ready`, `ask` (one request per connection, read one line), `send_logs` through `test_support::otlp`, `end_by_sigterm`, `family_lines` over `agent-latest.jsonl*`, `assert_no_log_record_in`.
- `pulse-app/tests/unit_console_commands.rs` (full) — pins `USAGE` verbatim (`:90`) and the unknown-word list (`:34-46`).
- `pulse-app/tests/integration_console_engine.rs` (895–912) — pins the usage line on stderr verbatim (`:905`) for a refused command line.
- `pulse-app/src/lib.rs` (1–40) — module registration; `door` and `console` are `pub mod`.
- `.andromeda/architecture.md` (`:60`, `:63`, `:85`, `:157`, `:181`, `:211`) — the door's contract, its PROVISIONAL sentence, the MCP stdio surface kept the sidecar's.

## Graph impact (from the code-graph query)
Query trace `.andromeda/runs/2026-10-12T08-15-38Z-phase/tree-query-2026-10-12-stdio-agent-reaches-the-door.json` (rust plane, 41 rows). Lines are editor lines (SCIP 0-indexed + 1).
- **console::parse** — 1 production caller, `console::main` at `pulse-app/src/console.rs:54`; 9 test sites in `pulse-app/tests/unit_console_commands.rs`. A new word changes `parse`'s result set; its test pins move with it.
- **console::main** — called by the binary `pulse-app/src/bin/andromeda-pulse-engine.rs:2` and by `unit_console_commands.rs` (`:73-75`, `:84`). The binary file needs no change.
- **engine_boot::init_process** — 2 production callers (`console::run` at `console.rs:75`, the window's `main` at `pulse-app/src/main.rs:76`) and 2 test sites in `integration_engine_boot.rs`. A bridge word that called it would write the engine's pid file and log family (below).
- **engine_boot::resolve_data_dir** — 2 production callers (`console.rs:74`, `main.rs:75`). Reusable by a bridge as is; it does not canonicalize.
- **door::socket::bind_endpoint / open / own_account** — callers are `door::socket::open` (`door.rs:413`), `engine_boot::start` (`engine_boot.rs:1048`) and `unit_door.rs`. No client exists to reuse, so a bridge adds its own connect path.
- No `crate_edges` change is needed by the leaned form (code in `pulse-app` alone, `std` Unix sockets).

## Patterns detected
- **The door's stdio twin is framing-identical** (`crates/mcp-server/src/bin/andromeda-pulse-mcp.rs:160-179`, `pulse-app/src/door.rs:477-500`): both read one `\n`-terminated JSON-RPC line and write one line back; a notification gets nothing. A byte relay between stdio and the socket therefore needs no parsing.
- **A half-closed connection ends silently** (`door.rs:482-484`, `:513`): after the answer line the door reads again; end of stream is `Line::End`, and the connection returns with no record. Equality the leaned form rests on, verified by reading the frame: for ONE request line followed by end of stream, the door writes one `door.call` (`:530`), then the answer line (`:540`), then returns with no `door.refused`; for a line with no `id`, it writes nothing and returns.
- **A long-lived connection is cut at 30 s idle with a WARN** (`door.rs:508-512`): `door.refused {reason: idle}` and the stream dropped. A relay that holds one connection for an agent's whole session (a generic `socat STDIO UNIX-CONNECT:…`) loses the session at the agent's first 30 s pause and leaves a WARN for each.
- **The process start is shared and writes shared files** (`engine_boot.rs:324-331`, `:210-251`): `init_process` writes `run/andromeda-pulse.pid` and opens the data dir's log family — both read by the harness (`harness:status`, `check:engine-log`). A short-lived client program must not call it.
- **Owner-only admission covers any client** (`door.rs:452-457`): the door reads the account of whatever process connects. A bridge run by the engine's owner is admitted; run by another account, root included, it is closed with no byte, exactly as a direct client is.

## Conventions to follow
- **Closed command grammar**: a `Command` enum, exactly one word, a non-Unicode word refused, refused lines exit 2 with the usage line on stderr and nothing created (`console.rs:19-42`, `:60-63`; pinned `unit_console_commands.rs`, `integration_console_engine.rs:895-912`).
- **Path confinement for a product `*_DIR` read**: both-sides-canonicalize and assert under the data dir, the socket path under 108 bytes (`door.rs:379-389`; CLAUDE.md Critical Warnings).
- **Spawned-program arm**: built binary by `CARGO_BIN_EXE_andromeda-pulse-engine`, cleared environment, own TempDir, claimed ports, records awaited under a bound, SIGTERM end, log family read after the end (`integration_door.rs:74-182`).
- **No stdout but protocol** for a process that speaks JSON-RPC on stdout (obs-plan §11; the sidecar forces its records to stderr).

## New files to create
- `pulse-app/src/door_bridge.rs` — the stdio-to-door relay a stdio agent starts (form B, the P4 lean): socket path from the canonical data dir, one connection per request line, bytes passed unchanged, a fixed one-line failure on stderr.
- `pulse-app/tests/unit_door_bridge.rs` — in-process arms of the relay over a stand-in listener: path refusal, passthrough, notification, no-door failure.

## Files to modify
- `pulse-app/src/console.rs` — a third word in the closed grammar and its usage line.
- `pulse-app/src/lib.rs` — register the relay module.
- `pulse-app/tests/unit_console_commands.rs` — the new word's parse and the usage pin.
- `pulse-app/tests/integration_console_engine.rs` — the refused-line usage pin.
- `pulse-app/tests/integration_door.rs` — two arms: a stdio client through the relay on a spawned engine, and another account through the relay.
- `.github/workflows/ci.yml` — the `lint-test` step `Door refuses another account` selects both other-account arms and reads both sentences.
- `pulse-app/tests/quality_gate_workflow.rs` — the step's pins read the two-arm selection and both sentences.

## Sweeps
- `run|version` (fixed string) over the work tree by `git grep -n -F 'run|version' -- ':!*.md' ':!.andromeda'`: 3 hits · 3 changed (`console.rs:16`, `unit_console_commands.rs:90`, `integration_console_engine.rs:905`) · 0 no-change; the `*.md` hits are two historical records (a wrap run's fan-out results, the console chunk's report) — no change, history.
- `andromeda-pulse-mcp|mcpServers|ANDROMEDA_PULSE_MCP_ENABLED` over `README.md` and `docs/` (excluding `docs/v0_2_0`) by `git grep -n -i`: 0 hits — no user document tells an agent which command to start; none is changed by the lean (the masters' wrap amendments name the word).
- `Using MCP with andromeda-pulse` over the tree by `git grep -l -i`: hits only in 2026-05-02 security run files — the guidance security-plan names was never written as a document; no change (outside this chunk).

## Measured on the dev host (live state, each with its control)
- **A general-purpose stream relay IS installed:** `command -v socat` → `/usr/bin/socat`, `socat -V` → `socat version 1.8.1.3`; `nc` / `ncat` / `netcat` / `busybox` / `websocat` absent; `python3` present. Control: the same `command -v` loop found `python3` present in the same call. Whether the `ubuntu-22.04` runner has `socat` is unmeasured.
- **`origin/main` is `178ebac56e262ee2cea8ed8a2f31254a5a1f6d86`** (`git ls-remote origin refs/heads/main`), as the directive read it.

## Open questions
- none — both plan-decision questions were answered at P4 (inputs#I3, the pc overseer, 2026-10-12, PROVISIONAL): form B, the word `door`; proof (b), the relay's own other-account arm in the existing `Door refuses another account` step. The lists above are written to that answer; the CI step and its pins (`.github/workflows/ci.yml:118-138`, `pulse-app/tests/quality_gate_workflow.rs:937-1010`) were read at P4 and joined the modify list then.
