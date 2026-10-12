# Report — 2026-10-12-stdio-agent-reaches-the-door

**Chunk:** a stdio agent reaches the door's tools with no sidecar
**Date:** 2026-10-12T09:35:51Z
**Commits:** since `last_wrap` 2026-10-11T01:37:45Z (`git log --since=2026-10-11T01:37:45Z`): `d3e694d2` feat(2026-10-10-door-inside-the-engine-s-process) — the previous wrap's own commit, not this chunk's; `2595b5cc` chore(2026-10-12-stdio-agent-reaches-the-door): operator pre-CI commit (this chunk's whole delta, pushed; HEAD = origin).

## Changes (structured — detectors read this)
- **Files:** basis `gate.py scope` (changed 9 · listed 9 · recorded 0) and `cites.py added` (base `d3e694d2`).
  - New: `pulse-app/src/door_bridge.rs` (145 lines) · `pulse-app/tests/unit_door_bridge.rs` (371 lines).
  - Modified: `pulse-app/src/console.rs` · `pulse-app/src/lib.rs` · `pulse-app/tests/unit_console_commands.rs` ·
    `pulse-app/tests/integration_console_engine.rs` · `pulse-app/tests/integration_door.rs` · `.github/workflows/ci.yml` ·
    `pulse-app/tests/quality_gate_workflow.rs`.
  - Not changed (the scope guard entry, exit 0 no output, against `d3e694d2`): `pulse-app/src/door.rs`,
    `pulse-app/src/engine_boot.rs`, `pulse-app/src/main.rs`, `crates/mcp-server/`, every manifest, `Cargo.lock`, every
    xtask verb, every script, `pulse-app/ui/src/bindings/index.ts`.
  - Chunk records riding the commit: `scope.md`, `research.md`, `plan.md`, `inputs/` (I1–I6), `evidence/operator-pass.md`
    (uncommitted at this report; rides the wrap commit), the phase, implement and wrap run dirs,
    `andromeda-pulse-0.4.0/verification-matrix.json` (P-092's phase-P5 dated note), `working-route.md`, `master-route.md`.
- **Symbols / APIs:**
  - **A third word of the console program.** `pulse_app::console::Command::Door` joins `Run` · `Version`;
    `console::parse` takes `door` with the same rules (exactly one word, case-exact, a non-Unicode word refused);
    `console::USAGE` = `usage: andromeda-pulse-engine <run|version|door>` (was `<run|version>`); `console::main` sends
    `Door` to `door_bridge::main()` and returns its exit code. `door` builds no tokio runtime and calls neither
    `engine_boot::init_process` nor `engine_boot::start`. `console::parse` keeps one production caller (`console::main`);
    `console::main` keeps its one (`pulse-app/src/bin/andromeda-pulse-engine.rs:2`, unchanged).
  - **The relay, `pulse_app::door_bridge`** (new `pub mod`, `pulse-app/src/lib.rs:15`):
    - `pub const UNREACHABLE: &str = "andromeda-pulse-engine door: the door cannot be reached"` (`door_bridge.rs:16`);
      `pub const EXIT_UNREACHABLE: u8 = 1` (`:17`); `#[doc(hidden)] pub struct Unreachable` (unit error).
    - `pub fn main() -> ExitCode` (`:27-48`): resolves the socket path, relays stdin to stdout; standard input ending is
      exit 0; anything else that ends it writes the one fixed line to stderr and exits 1. Off Unix it prints the fixed
      line and exits 1 at once (no door is seated there).
    - `#[doc(hidden)] pub fn socket_path(data_dir: &Path) -> Option<PathBuf>` (Unix, `:64-73`): canonicalizes the data
      dir and `{canonical data dir}/run/door/sock` (both sides); `None` when either does not resolve, when the socket
      resolves outside the data dir, or when its path is 108 bytes or more (private `SOCKET_ADDRESS_BYTES = 108`,
      `:62`, a second copy of `door.rs`'s private constant of the same name and value).
    - `#[doc(hidden)] pub fn relay<R: BufRead, W: Write>(input, output, socket) -> Result<(), Unreachable>` (Unix,
      `:75-112`): reads stdin by line through `take(door::MAX_REQUEST_BYTES + 1)`. A line of whitespace alone
      (`u8::is_ascii_whitespace`, the door's own rule — it does NOT count `\x0b`) opens nothing. Every other line gets a
      connection of its own (`exchange`, `:114-129`): the line and its `\n` written (an unterminated last line gets a
      `\n`), the write side shut, the whole answer read until the door closes, THEN written to stdout and flushed — so
      a connection that fails writes nothing to stdout. A line over the bound is sent as its first
      `MAX_REQUEST_BYTES + 1` bytes with no terminator, then the rest of that line is read and discarded up to its `\n`
      (`discard_line`, `:131-144`), never sent. A failed read of STANDARD INPUT ends the relay as its end does (exit 0;
      a choice made at implement, the plan names no such case). A connect, write, shutdown or read failure on the
      socket, or a failed write or flush of stdout → `Unreachable` → exit 1 with the fixed line.
    - It parses no request, writes no answer of its own, retries nothing, has no time bound of its own (the door
      bounds a call at 5 s and an idle connection at 30 s; the relay half-closes after its one line).
  - **What it reads:** the data dir through `engine_boot::resolve_data_dir()` — the function `run` uses, so the same
    variables (`ANDROMEDA_PULSE_DATA_DIR`, else the per-OS default: `XDG_CONFIG_HOME` then `HOME` on Linux) and no
    other; `door::MAX_REQUEST_BYTES`. `resolve_data_dir` now has THREE production callers (`console.rs:80` the `run`
    path, `main.rs:75` the window app, `door_bridge.rs:33`; basis `git grep -n 'resolve_data_dir()' -- pulse-app/src`);
    research had counted two.
  - **What it adds:** no env var, no flag, no value on the word, no listener, no port, no socket of its own (a CLIENT
    of `run/door/sock`), no pid file, no log record, no file, no process started, no variable set, no new binary (the
    word lives in the existing `andromeda-pulse-engine`). Basis: the grep entry over `door_bridge.rs` for
    `init_process|write_pid_file|observability|tracing::|Command::new|set_var|tokio` reads count 0 (exit 1); the
    spawned arm reads the data dir's file set unchanged (log family aside) and the pid file still the engine's.
  - **How the door treats the relay:** as any caller — kernel-read account against the engine's own; one `door.call`
    per answered request; a notification (no `id`) answered with nothing and recorded with nothing; one connection per
    request means no `door.refused {reason: idle}`.
  - **Other-account behaviour, measured:** a connection the door refuses (another account) is closed unread. On Linux
    the client then reads `EPIPE` on its write (door dropped first) or `ECONNRESET` on its read (door dropped after the
    write) — measured on the dev host by a Python AF_UNIX probe (`[('write','EPIPE'),('shutdown','ok'),('read',b'')]`
    and `[('write','ok'),('shutdown','ok'),('read','ECONNRESET')]`). So the relay, run by another account, ends
    UNREACHABLE: exit 1, the fixed line, nothing on stdout — whether it was refused by the directory mode (EACCES at
    canonicalize/connect) or by the door's account check. Read on the runner (`ci#38227488889`, below).
- **Crates / modules:** `pulse-app` gains the module `door_bridge`. No crate added or removed; no `crate_edges` change
  (`std` Unix sockets; `door` and `engine_boot` are siblings inside `pulse-app`).
- **Dependencies:** none added or bumped (`Cargo.toml`, `Cargo.lock` byte-identical to `d3e694d2` — the scope guard).
- **Schema / config:** none. No config key, no log target, no allowlist leaf, no scrub shape.
- **Spec-master edits:** none (no `.andromeda/` master or registry file changed; the sweep below ran `--pre-pass`
  against HEAD and every master/registry equalled it).
- **Counts / qualifiers moved:**
  - The console program's command grammar: two words → three (`run` · `version` · `door`). Masters stating the
    grammar or the program's words — sweep pattern `grammar`
    (`andromeda-pulse-engine (run|version)\b|<run\|version>|\`run\` (\||/|or|and) \`version\`|run \| version|\`run\` \| \`version\``):
    28 standing rows (architecture 9 · security-plan 6 · test-plan 7 · obs-plan 6, registry key files counted to their
    master); the rows that STATE the closed set are `architecture.md:300` ("two-word command line (`run` |
    `version`)"), `security-plan.md:88` ("(`run` or `version`)"), `security-plan.md:104` ("driven by `run` or
    `version`"), `security-plan.md:145` (the console command-line row), `test-plan.md:62` ("(`andromeda-pulse-engine
    run` / `version`)"), `obs-plan.md:67` ("grammar closed at `run` or `version`"); the other 22 name `run` alone as the
    word that boots the engine — no change (`run` is unchanged). Leaves stating it: `CLAUDE.md:100`,
    `.claude/docs/security-summary.md:27`, `.claude/docs/stack.md:72`, `.claude/docs/commands.md:77-78` (the cascade's).
  - `pulse-app/tests/*.rs` files: 112 → 113 (basis `git ls-tree --name-only d3e694d2 pulse-app/tests/` · `ls
    pulse-app/tests/*.rs`); no master states the count (`git grep -n -E '\b11[23] (files|test files)' -- .andromeda`
    reads only a run-dir file); the leaf `.claude/rules/testing.md:25` states "112 files as of chunk
    2026-10-10-door-inside-the-engine-s-process" (the cascade's).
  - The CI step `Door refuses another account`: one arm → two, one required sentence → two.
  - Tests added: 12 — `unit_door_bridge` 8, `integration_door` 2, `unit_console_commands` 1,
    `quality_gate_workflow` 1 (plus 3 renamed); the workspace run read 3186 of 3186 (implement's whole block; the same
    count on `ci#38227488889`'s `lint / test` job).
  - `engine_boot::resolve_data_dir` production callers 2 → 3 (above); no master states the count (research did).
- **Dev-tool versions:** none — `python3` was used as a probe only (host, unchanged); `socat` (research measured
  1.8.1.3 on the dev host) is not used by anything this chunk built.
- **Harness / gate surface:**
  - `.github/workflows/ci.yml`, `lint-test` job, step `Door refuses another account` (lines 114-122 comment, 128 the
    cargo line, 142-145 the new check): its one cargo command's filter becomes `binary(integration_door) &
    (test(=another_account_is_refused_at_each_of_three_layers) |
    test(=another_account_through_the_bridge_receives_nothing))`, same flags (`--profile ci --no-tests=fail
    --success-output immediate`); its checks stand (exit, a non-empty output, the three-layer sentence) and a fourth
    is added: unless the output holds `door bridge other-account arm: no byte reached another account through the
    bridge, the account check refused`, it prints `::error::the bridge other-account arm did not read the account check
    refusing` and exits 1. The step comment names both arms. Still a plain `run` step, no `if:`, no
    `continue-on-error`, no `sudo` in the step itself. Nothing else in the workflow changed.
  - `pulse-app/tests/quality_gate_workflow.rs`: the pins read the two-arm selection and both sentences, the arm file
    must hold both arm names and both skip sentences (`door other-account arm: skipped, no second account` and `door
    bridge other-account arm: skipped, no second account`) and print both sentences; the step simulation now feeds two
    arms' output; new pin `door_other_account_step_fails_a_bridge_arm_that_skipped`; renamed
    `the_door_other_account_step_runs_its_two_arms_and_reads_each_arms_own_sentence` and
    `door_other_account_step_passes_arms_that_read_all_three_layers_and_the_bridge_refused`.
  - The other-account relay arm, `pulse-app/tests/integration_door.rs:1023-1141`: skips with its sentence where `sudo
    -n true` fails (this dev host, every run); else spawns an engine, opens the data dir and `run/` to 0755, copies the
    built `andromeda-pulse-engine` into its own TempDir at 0755, runs `sudo -n [-u nobody] env
    ANDROMEDA_PULSE_DATA_DIR={data dir} {copy} door` with one `ping` from that TempDir as working dir (sudo's
    env_reset drops `LLVM_PROFILE_FILE`), at the door's modes as made and then with `run/door/` 0755 and the socket 0666;
    asserts every one of the four other-account readings wrote 0 bytes to stdout, exited 1 and holds the fixed line as
    ONE line of its stderr; the owner's relay with no sudo answered; exactly 3 `door.refused {other_account, other}`
    records (root as made, nobody opened, root opened — root passes the modes, `nobody` only once both outer layers are
    open); exactly one `door.call` (the owner's `ping`); then prints its sentence. It is the second thing in the suite
    run as another account (the first: the three-layer arm's Python client).
  - No xtask verb, no `agent-run` script, no harness verdict shape changed.
- **Cross-project / external claims:**
  - CI: `ci#38227488889` (pull_request) and `secret-scan#38227488989` (pull_request), both completed/success on
    `2595b5cc` (the pre-CI commit, HEAD = origin), `ci.py conclusion --sha HEAD --wait 2700` → `verdict: green · checks
    7/7 · wall 1628 s`, attempt 1. Its `lint / test (ubuntu-22.04)` job `114738471070`, step `Door refuses another
    account`, printed (escape codes stripped, recorded in `evidence/operator-pass.md`): `nobody as made exit Some(1), 0
    byte(s) on stdout` · `root as made exit Some(1), 0 byte(s)` · `nobody opened exit Some(1), 0 byte(s)` · `root
    opened exit Some(1), 0 byte(s)` · the relay arm's sentence · the three-layer arm's readings `directory 0700
    [Kernel(13), Closed], socket 0600 [Kernel(13), Closed], both opened [Closed, Closed]` and its sentence. The 3
    `door.refused` records the relay arm asserted are in no kept artifact. The run's 7 artifacts were not downloaded.
  - `inputs.py verify` (this wrap): `inputs: 6 entries — unchanged 2 · drifted 0 · vanished 0 · broken 0 · altered 0 ·
    unreachable 0 · n/a 4 · uncited 3 · unparsed 0`. The three uncited before this report are cited here:
    - I1 · message · n/a — the operator's take-up message (`/andromeda-phase`, 2026-10-12); cited by scope, research, plan.
    - I2 · `../additional/pc-overseer/relays/pulse-phase-stdio-agent-reaches-the-door-2026-10-12.md` · copy no-repo ·
      unchanged — the phase directive; cited by scope, research, plan.
    - I3 · message · n/a — the operator's P4 answer, PROVISIONAL (the pc overseer, 2026-10-12): form B, the word `door`,
      one connection per request, no file/listener/pid/env var/binary; proof (b), the relay's own other-account arm in
      the existing step; door unreachable is exit 1 with one fixed stderr line; P-092 a dated note, no claim.
    - inputs#I4 · message · n/a — the operator's P5 word "yes" (the pc overseer, 2026-10-12) on
      `.andromeda/runs/2026-10-12T08-15-38Z-phase/p5-card-2.md` (the governing card; `p5-card-1.md` superseded by it —
      the plan re-worded at plan.md:448-450 on the operator's review of card 1). The card's `## Beyond the plan` rows,
      verbatim, with the word's authority (the operator's delegate word on owners — PROVISIONAL where the fork is the
      operator's):

      | row | proposal |
      |---|---|
      | 1 | Owner of the `door` word's off-Unix branch (fixed line, exit 1): the existing entry `Other operating systems retired from the code`. |
      | 2 | Owner of the never-written "Using MCP with andromeda-pulse" guidance that security-plan §Logging & Monitoring names: the existing entry `Records say what the product is`. A stdio agent through the relay now receives the same unsanitized tool answers it warns of. Research found no such document in the tree (`git grep -l -i 'Using MCP with andromeda-pulse'`: hits only in 2026-05-02 security run files). |

    - inputs#I5 · message · n/a — the operator pass on the operator's word (the pc overseer, the founder's delegate,
      2026-10-12, in the implement session): entries 20–23 and the pre-CI commit; the step-6 departure (a refused
      account reads exit 1 with the fixed line; the plan said exit 0) ACCEPTED AS BUILT, the wrap reconciles the plan;
      stop on a red relay arm with no re-push.
    - inputs#I6 · `../additional/pc-overseer/relays/pulse-wrap-stdio-agent-reaches-the-door-2026-10-12.md` · copy
      no-repo · unchanged (snapped at this wrap) — the wrap directive: a split wrap (P1 alone in the first window, the
      founder's ruling F252 D2: each project's first wrap after the 2026-10-12 deploy runs the cascade's WHOLE mode,
      `cascade.py leaves --all` in the second window); the fourteen Boundary-widening amendments halt in the ORDINARY
      FORM, in full, the operator's answer there PROVISIONAL (no answer card: the founder did not answer this widening
      himself); the playbook rule the handoff held deferred ("a new variable that is the switch of a write crossing a
      newly widened boundary" → escalate) accepted HERE by the operator as delegate, 2026-10-12, in the playbook's
      propose → approve → append form — it only makes a class escalate, never routine, and rides the PROVISIONAL batch;
      the plan reconciled as this report's deviations, saying where each lands in the masters, if anywhere; the two
      Beyond-the-plan owners become CARRYs at the route card; P-092 keeps its P5 note, unclaimed; what the last wrap's
      handoff owes this cascade discharged by name, or said to be nothing; found-and-not-owned to the card with a lean;
      stops after P1, at the widening halt, at the route-resolve card, and before the flip and the commit.
  - The handoff written at `d3e694d2` (`git show d3e694d2:.claude/session-handoff.md`): names NOTHING as owed to the
    next wrap's cascade (`grep -n -iE 'cascade|next wrap|owed to|at the next'` → one hit, line 33, a count of the
    previous cascade's own sweep). Its carried items (stale line citations in preserve-verbatim homes, sidecars past
    the whole-read bound, the carried-not-re-read list) are carried, not owed to a cascade.
- **Reverted / negative API facts:** none. (A test byte, `\x0b` in the blank-line arm, was replaced by `\x0c` at
  implement — a test-input fix, below under Decisions; no surface was written and removed.)
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - **The plan's step 6** (`plan.md`, Implementation Steps 6, "With the outer layers opened, both readings exit 0 with
    empty stdout" and "the `door.refused` records equal the number of other-account readings that exited 0"): FALSE on
    Linux. A connection the door closes unread reads `EPIPE` or `ECONNRESET` at the client (the dev-host probe above),
    and step 2 makes any socket write/read failure exit 1. Measured on the runner: all four other-account readings
    exit 1 (`ci#38227488889`). The arm was written to step 2 and the measurement (exit 1 + the fixed line + 0 stdout
    bytes for all four; refusals counted as 3 by construction). Accepted as built by the operator (inputs#I5);
    reconciled by this report (directive inputs#I6 item 5). A chunk artifact, not a master: no master states the exit
    code of another account's relay (the relay is new). Where it lands in the masters: the security-plan
    §Authentication & Authorization / second-account-step sentence and test-plan §9's `lint-test` row, when the wrap
    amends them for the second arm (Expected amendments 8 and 10), state the relay arm's reading as built — no byte to
    another account, exit 1 with the fixed line, one `door.refused` per reading that reached the door.
  - **The plan's step 5** (`plan.md`, Implementation Steps 5: "exactly four `door.call` records" beside "The
    `tools/list` line equals, byte for byte, the line `ask` reads for the same request straight from the socket"): the
    two are jointly unsatisfiable — the direct read is itself an answered request. Built: five `door.call` records, the
    relay's four (`initialize` · `tools/list` · `tools/call query_logs ok` · `tools/call mark_incident_resolved
    write_off`) then the direct `tools/list`. Lands in no master: the masters state "one `door.call` per answered
    request" (obs-plan §4 Scenario P3), which five answered requests honour.
  - **Research's Graph impact** ("engine_boot::resolve_data_dir — 2 production callers"): now three, by this chunk's
    own addition — not a disproof of research as written, a count this chunk moved (above). No master states it.
- **Expected amendments (from plan):** fourteen, all pattern **Boundary widening**, PROVISIONAL with the door's one item
  (answered by the pc overseer at P4, inputs#I3; the founder did not answer it himself — no answer card; the wrap halts
  on them in the ordinary form, inputs#I6 item 3). Search: this wrap's site sweep (`cascade.py sweep --pre-pass`, run dir
  `report-patterns.toml`, 11 patterns, controls all fired; trail `cascade-2026-10-12-stdio-agent-reaches-the-door-pre-pass.json`;
  `total (11 patterns) · 156 rows over 30 files · standing 101/15 · leaf 52/14 · curation 3/2`). Counts are standing
  ROWS per master, a registry key file counted to its master.
  1. architecture §Occupied Resources → Process / service identity — the console grammar reads `run | version | door`;
     `door` is the stdio relay (a client of the door; no listener, file, pid or variable) — carried: Symbols / APIs
     (the third word), Counts / qualifiers moved (two → three). Search `grammar`: architecture 9 (`architecture.md:300`
     states the two-word line; the rest name `run`).
  2. architecture §Infrastructure Patterns → Deployment model — the runtime topology names the relay process an agent
     starts — carried: Symbols / APIs (the relay a stdio agent starts, short-lived, one per agent session). Search
     `topology` (`Deployment model|[Rr]untime topology`): architecture 3 — the body is the registry key file
     `registries/contracts/architecture/deployment-model.md` (`:1`, `:4` the topology) keyed at
     `registries/architecture-contracts.toml:12`.
  3. architecture §Standard Contracts → Engine door — a stdio caller reaches the door through `andromeda-pulse-engine
     door`, one connection per request — carried: Symbols / APIs. Search `door-contract` (`Engine door`): architecture 6
     (`architecture.md:157` the contract; `:60`, `:63`, `:85`, `:181`, `:211` point to it).
  4. architecture §Occupied Resources → MCP stdio surface, §Established Decisions [MCP Server Surface] and [API Style —
     MCP Server Surface 3] — stdio is no longer the sidecar's alone — carried: Symbols / APIs (a stdio way to the same
     nine tools through the door). Search `mcp-stdio` (`MCP stdio surface|MCP Server Surface|stdio sidecar`):
     architecture 8 (`architecture.md:60`, `:63` the decisions; `:210` the MCP stdio surface; `:211` "The MCP stdio
     surface above stays the sidecar's"; `:25`, `:217`, `:319`; `registries/contracts/architecture/project-directory-structure.md:36`);
     security-plan 7 · test-plan 6 · obs-plan 7 (sites the same fact may reach — the detectors' to judge).
  5. architecture §Occupied Resources → Network ports — the door's PROVISIONAL item now covers the relay word —
     carried: Symbols / APIs; inputs#I3. Search `door-contract` hit `architecture.md:181` (the door's PROVISIONAL
     sentence) and `second-acct` architecture 3 (`:181`, `:34` ×2).
  6. security-plan §Input Validation — the console command-line row's closed grammar holds three words — carried:
     Counts / qualifiers moved. Search `grammar`: security-plan 6 (`security-plan.md:145` the row, ×2; `:88`, `:104`
     state the two words; `:72`, `:148`, `:412` name `run`).
  7. security-plan §Threat Model Summary → Attack surface (the door's vector) — the stdio way through the relay —
     carried: Symbols / APIs. Search `door-vector` (`the door's vector`): security-plan 10 (each a pointer to the vector;
     the vector itself is `security-plan.md:71-73`, its entry point `:72` matched by `grammar` and `mcp-stdio`; the
     MCP stdio surface's own vector is `:67-69`).
  8. security-plan §Authentication & Authorization — the second-account step reads two arms — carried: Harness / gate
     surface; Spec claims disproved (step 6). Search `second-acct` (`Door refuses another account|second account`):
     security-plan 3 (`:73`, `:123`, `:412`); `door-item` (`the door's (one )?(PROVISIONAL )?item|door's one item`):
     security-plan 1 (`:412`, the second-account step's sentence).
  9. test-plan §1 — the door surface row gains the relay's arms — carried: Symbols / APIs, Harness / gate surface,
     Outcome. Search `second-acct`: test-plan 4 (`test-plan.md:63` the door row ×2; `:494`, `:515`); `grammar`
     `test-plan.md:62`, `:63`.
  10. test-plan §9 — the `lint-test` step `Door refuses another account` selects two arms and reads two sentences —
      carried: Harness / gate surface. Search `second-acct`: `test-plan.md:494` (×3), `:515` (×2);
      `registries/contracts/test-plan/per-chunk-gate-discipline.md:52`; `registries/contracts/architecture/ci-cd-approach.md:3`
      (architecture's CI key file).
  11. test-plan §3 Spawned-program witness form — the relay is spawned beside the engine — carried: Harness / gate
      surface (the unit file spawns the word; `integration_door` spawns the relay beside a spawned engine). Search
      `spawned` (`Spawned-program witness`): test-plan 2 (`test-plan.md:62`;
      `registries/contracts/test-plan/per-chunk-gate-discipline.md:52`, the form's home).
  12. obs-plan §4 CLI / main binary row — `door` writes no record — carried: Symbols / APIs (no record, no sink).
      Search `cli-row` (`CLI / main binary|CLI \(command-line entrypoint`): obs-plan 2 (`obs-plan.md:67` "grammar
      closed at `run` or `version`", also a `grammar` hit; `obs-plan.md:192`).
  13. obs-plan §4 Scenario P3 — the stdio way through the door is the door's carrier — carried: Symbols / APIs (one
      `door.call` per answered request; a notification none; no idle refusal). Search `scenario-p3` (`Scenario P3`):
      obs-plan 6 (`:109`, `:195`, `:331`, `:451`, `:662`, `registries/contracts/obs-plan/snapshot-paste-to-ai-integration.md:9`;
      the scenario text itself is `obs-plan.md:223`, matched by `grammar`, `mcp-stdio`, `second-acct`).
  14. obs-plan §11 — the stdout ban reaches the relay (its stdout holds only the door's bytes) — carried: Symbols /
      APIs (nothing on stdout but the door's answer bytes; nothing on stderr but the fixed line). Search `stdout-ban`
      (`stdout reserved|stdout is reserved|reserved for JSON-RPC`): obs-plan 4 (`:76`, `:194`, `:658` §11,
      `registries/contracts/obs-plan/logging-stack.md:5`); `mcp-stdio` `obs-plan.md:662` ("The ban is the stdio
      sidecar's").
  A listing reaches its patterns' wording and no further: the detectors' own sweeps of their masters stay the control
  for reach.
- **Coverage of new surfaces:**
  - `andromeda-pulse-engine door` (the stdio relay, a new command word and a new client of the door's socket) →
    validation ✓ (closed grammar; socket path canonical both sides, confined under the data dir, < 108 bytes; request
    line bounded at `MAX_REQUEST_BYTES + 1`; the door still applies every bound and the account check) · instrumentation
    n/a by design (the relay writes no record; the door writes `door.call` / `door.refused` per its existing leaves) ·
    PII ✓ (no log, no file; stdout carries only the door's answers to the process that started it; the answers are the
    same tool answers the door gives any owner, unsanitized as the sidecar's — the guidance security-plan names is the
    P5 card's Beyond-the-plan row 2) · tests unit (`unit_door_bridge`, 8) / integ (`integration_door`, 2 spawned arms) /
    e2e (the smoke below; the CI second-account step) · a11y n/a · tokens n/a.
  - The CI step `Door refuses another account`, second arm → validation n/a · instrumentation n/a · PII n/a · tests ✓
    (`quality_gate_workflow` pins and a simulation of the step) · a11y n/a · tokens n/a.

## Deviations from intent
- **Step 6 — exit code of a refused other-account reading** (plan expected exit 0 for a reading that reaches the door;
  built: exit 1 with the fixed line for all four readings, 0 stdout bytes, the refusal count asserted as 3 by
  construction). Justification: Linux closes a refused connection with EPIPE/ECONNRESET at the client (measured), and
  step 2 makes that exit 1; the arm's security property (no byte to another account; one `door.refused` per reading
  that reached the door) is the acceptance's and holds. Accepted as built (inputs#I5); reconciled here (inputs#I6).
- **Step 5 — five `door.call` records, not four**: the byte comparison's direct socket read is itself an answered
  request. The arm asserts the relay's four in order, then the direct one.
- **Step 3 — more than the six arms listed**: the unit file also spawns the built word on each of the four unreachable
  cases (exit 1, the fixed line as stderr, empty stdout, no connection reaching a stand-in, no data dir created); adds
  a socket path of exactly 107 bytes reached (the bound's other side) and a stdout that refuses writes ending the
  relay unreachable. The 108-byte case reaches a real socket under a second (hard-link) name, so the path canonicalizes
  inside the data dir.
- **Choices the plan did not settle**: a failed read of stdin ends the relay as its end does (exit 0); the answer is
  read whole before it is written; an unterminated last line is sent with a `\n`; the over-bound line is sent before
  its rest is discarded.
- scope record: none — `gate.py scope` clean (changed 9 · listed 9 · recorded 0 · excluded 77), 0 recorded.

## Decisions & corrections
- **Operator decisions:** the operator pass on the operator's word, the step-6 departure accepted as built (inputs#I5,
  the pc overseer, the founder's delegate, 2026-10-12); the wrap runs split, P1 alone in this window (F252 D2, inputs#I6);
  the deferred playbook rule (a variable that switches a surface whose own registration is a widening → escalate) is
  accepted by the operator as delegate at this wrap, 2026-10-12 (inputs#I6 item 4) — to be appended in the playbook's
  propose → approve → append form in the second window.
- **Measured, AF_UNIX on Linux:** a listener that accepts and drops a connection without reading it leaves the client
  with `EPIPE` on a later write (dropped before the write) or `ECONNRESET` on its read (dropped after the write, the
  request unread in the queue). A client that reads a refused connection as "closed, exit 0" is wrong on this kernel;
  the existing three-layer arm's Python client reads both as `closed` because it catches `OSError` and treats it as no
  bytes.
- **`u8::is_ascii_whitespace` does not count `\x0b`** (vertical tab): space, `\t`, `\n`, `\x0c`, `\r` only. A test that
  fed `\x0b` as a "blank" byte made the relay open a connection to a stand-in serving nothing; `read_to_end` then
  blocked and nextest killed the test at 180 s (the unit entry's first firing: `TIMEOUT [ 180.014s]
  whitespace_only_lines_open_no_connection`). Fixed by `\x0c`. A test design lesson beside it: a stand-in listener
  that serves a fixed count must leave an unexpected connection unanswered in a way the test reads (a nonblocking
  `accept` after the relay returns), and a relay test has no bound of its own — the hang came from the relay waiting
  on a peer the test never served.
- **A test can give a Unix socket a path of 108 bytes or more** by binding it at a short path and hard-linking it into
  the deep directory (`std::fs::hard_link` works on a socket inode; a symlink would canonicalize back to the short
  path). Used for the 108-byte unreachable case.
- **A workflow pin that reads a sentence's SOURCE spelling** (`arm.split_whitespace().join(" ")` containing
  `"{sentence}"`) breaks on a Rust string written with a `\` line continuation: the backslash stays in the source
  text. The relay arm's sentence constant was rewritten on one source line for that reason (the pins file itself keeps
  a continuation, since it compares the string VALUE).
- **Sweep hazard:** a Python edit script anchored on a multi-line Rust string continuation failed its `count == 1`
  assertion on indentation (13 spaces typed, 9 in the file) — the validated replace failed loudly and wrote nothing;
  re-done from a scratchpad file with the file's bytes read first (`cat -A`).
- **Guard recurrences this session** (recurrence-despite-learning candidates): the Bash guard refused a `cat` heredoc
  with a file target (a probe script; re-issued as a Write + run by path — `rules/host-linux.md`, Transports); refused
  `sleep 240; tail …` (re-issued as a backgrounded `until` loop); refused a leading `cd` into the skill's references
  dir in this wrap window (re-issued with absolute paths — host leaf 2026-10-05).
- **The `tools/list` byte comparison**: the relay's line equals the direct read byte for byte, so the relay passes the
  door's line unchanged.
- **For the main overseer:** no `REFUSED id:` printed in any trail of the implement run or this window so far; the
  citation sweep has not run yet (it is P2's, the second window).

## Outcome
**Acceptance criteria** (plan's 15), re-asserted against the diff:
1. (arch) The door's form unchanged against the chunk base — MET: the scope guard entry exit 0, no output;
   `door.rs`, `engine_boot.rs`, `crates/mcp-server/`, manifests and lockfile byte-identical to `d3e694d2`.
2. (arch) Exactly three words; anything else prints the usage line, exits 2, creates nothing — MET:
   `unit_console_commands` (`door_is_the_door_command`, the unknown-word list with `door ` and `Door`, `["door",
   "extra"]` refused, the three-word usage pin) and `integration_console_engine.rs:905`.
3. (arch) The `tools/list` line through the relay equals the direct line byte for byte, names nine tools, `write` on
   `mark_incident_resolved` alone — MET (`a_stdio_agent_reaches_the_door_through_the_bridge_and_leaves_one_call_per_request`).
4. (arch) The relay runs with `--features mcp-server` off; its file holds none of the banned tokens; writes no file
   into the data dir; the pid file still the engine's — MET (the grep entry count 0; the arm's file-set and pid
   assertions under the default-features workspace run).
5. (security) Through the relay another account receives no byte; each reading that reached the door leaves one
   `door.refused {other_account, other}`; the CI step fails without the arm's sentence — MET on `ci#38227488889`
   (four readings, 0 bytes each; the arm's 3-refusal assertion passed; its sentence printed). The exit code differs
   from plan step 6 (exit 1, not 0) — a deviation accepted as built (inputs#I5), not a criterion of the acceptance text.
6. (security) With `ANDROMEDA_PULSE_DOOR_RESOLVE=off`, `mark_incident_resolved` through the relay answers `-32001` —
   MET (the spawned arm).
7. (security) Unreachable door: exit 1, the fixed line, nothing on stdout, for no data dir, no socket, a door directory
   leading outside, a socket path of 108 bytes or more — MET (`a_door_that_cannot_be_reached_is_unreachable_and_nothing_is_written`,
   in process and on the spawned word).
8. (obs) One `door.call` per answered request, every field rendered; a notification none; no `door.refused` — MET
   (five answered requests, five records; see Deviations, step 5).
9. (obs) The planted marker 0 times in the log family; the relay's stderr empty on success; stdout JSON-RPC lines only
   — MET (the spawned arm).
10. (obs) No log target added; exactly one `app.boot.engine` — MET.
11. (tests) The in-process arms pass — MET (8 of 8 in the unit entry).
12. (tests) The workspace entry returns 0 and the seven-times entry reads seven green runs — MET (3186 of 3186; 7 of
    7 runs at 50 tests each).
13. (tests) The bindings close exits 0; the merge-base probe exits 0 directly before the push — MET (implement's
    block; operator entry 21 green, history unmoved).
14. (tests) The CI run of the pushed tip reads `verdict: green`, its id in the evidence — MET (`ci#38227488889`,
    `evidence/operator-pass.md`).
15. No capability claimed; P-092 takes a dated note at P5 and stays pooled — MET (`matrix.py show --id P-092`: the
    2026-10-12 phase-P5 note present, `chunk: None`, P-092 in the unclaimed pool).
No criterion is contradicted by the diff: it adds no file, listener, pid file, environment variable or binary.

**Gates** (implement's whole block, `.andromeda/runs/2026-10-12T08-39-32Z-implement`, second firing; summary `entries
23 · green 19 · red 0 · recorded 0 · timeout 0 · not-run 4`):
- `cargo fmt --check` — green (exit 0).
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — green.
- `git diff --name-only d3e694d2… -- crates pulse-app … ':!…'` (the scope guard) — green (exit 0 · no output).
- `cargo nextest run … -E 'binary(unit_door_bridge) | binary(unit_console_commands) | binary(integration_door) |
  binary(integration_console_engine)'` — red at the first firing (`TIMEOUT` of `whitespace_only_lines_open_no_connection`
  at 180 s, the `\x0b` test byte), green after the fix (the targeted re-run and the whole block).
- `grep -cE 'init_process|write_pid_file|observability|tracing::|Command::new|set_var|tokio' pulse-app/src/door_bridge.rs`
  — green (exit 1 · last line 0).
- `for i in 1 2 3 4 5 6 7; do cargo nextest run … integration_door | integration_engine_boot | integration_console_engine
  | agent_run_shell | engine_cycle_refusals …; done` — green: 7 green runs, each `50 tests run: 50 passed, 0 skipped`
  (recorded as a count, not an exclusion).
- `cargo xtask check:english-sources` — green (`"verdict": "clean"`). `cargo xtask quarantine-tracking` — green.
  `cargo xtask capability-widening-check` — green. `cargo xtask check:ingest-progress` — green. `cargo xtask
  check:staged-artifacts` — green. `cargo xtask capability-drift` — green. `cargo xtask verify:capability-matrix` — green.
- `cargo nextest run --workspace --profile ci --no-tests=fail` — green (3186 of 3186).
- `cargo nextest run -p pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'` — green.
- `git diff --quiet d3e694d2… -- pulse-app/ui/src/bindings/index.ts` — green (exit 0).
- `d="target/engine-cycle/…-door" && cargo xtask harness:engine-cycle --data-dir … && grep … app.boot.door … && ! grep …
  door.call … door.refused …` — green (`"verdict": "pass"`, `"witness_file": "absent"`, `"error_records": 0`,
  `engine-log: PASS`, `cleanup: clean`, `"state":"listening"`).
- `d="$(mise where node@24)" && PATH="$d/bin:$PATH" cargo xtask pre-push:linux` — green (`"verdict": "green"`,
  `"reason": "all-stages-ok"`, `engine-log: PASS`, `cleanup: clean`).
- `test ! -e target/pre-push/run/data/run/andromeda-pulse.pid && ! (exec 3<>/dev/tcp/127.0.0.1/24317) … 24318 …` — green.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` (leg operator) — `hygiene: clean`, exit 0,
  fired as written before the pre-CI commit (`evidence/operator-pass.md`).
- `m="$(git ls-remote origin refs/heads/main | cut -f1)" && test -n "$m" && git merge-base --is-ancestor "$m" HEAD`
  (leg operator) — green, `history: unmoved`.
- `git diff --quiet && git diff --cached --quiet && git push origin build/andromeda-pulse-0.4.0` (leg operator) —
  green, `history moved: refs/remotes/origin/build/andromeda-pulse-0.4.0 d3e694d2→2595b5cc` (the intended one).
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2700` (leg operator) —
  `verdict: green · checks 7/7`, `ci#38227488889` and `secret-scan#38227488989` both completed/success on `2595b5cc`.
- No `defer`, no deferral; no window boot smoke (no boot-smoke trigger path changed, plan prose).
- **Smoke** (implement P3, `console.rs` is the console program's entry point): `agent-run.sh boot engine` on
  24317/24318, ready; the release `andromeda-pulse-engine door` with a cleared environment holding the data dir alone,
  fed initialize · a notification · a blank line · tools/list · ping · tools/call query_traces: exit 0, 4 answer lines
  (ids 1–4), stderr empty; the engine's log then held 4 `door.call` (`initialize ok`, `tools/list ok`, `ping ok`,
  `tools/call ok`), 0 `door.refused`, 0 ERROR, 0 `app.panic.fatal`, 1 `app.boot.engine`; `status engine`
  `running-healthy`, program `console`; `cleanup: clean`. With a data dir that does not exist the word printed the
  fixed line and exited 1.
- **Watches:** none folded.
- **Outcome basis:** the operator pass ran (Setup 4: the pre-CI commit `2595b5cc`, the only commit since the parent
  `d3e694d2`); the gate verdicts rest on implement's whole block (this session's conversation) and the final HEAD's CI
  run `ci#38227488889`, recorded in `chunks/2026-10-12-stdio-agent-reaches-the-door/evidence/operator-pass.md` (the
  record P7 re-verifies). Implement's P4 report is in this window's conversation; its evolve records are
  `2026-10-12T08:48:31Z-a…c`, `2026-10-12T09:01:28Z-a…b`, `2026-10-12T09:02:18Z-a`.
- **Process hygiene** (implement's census, re-measured at the operator pass): every gate entry's processes,
  including the engine-cycle and pre-push engines — started by this run, terminated (entry 19 read the pid file gone
  and both harness ports refusing); the smoke's engine (pid 4104235) — started by this run, terminated (`cleanup:
  clean`; `ps -eo pid,comm,args` held no engine; 24317/24318 refusing; its tempdir removed); the relay processes and
  the Python socket probe — exited. Nothing left running.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: d3e694d2 (the parent of the oldest pre-CI commit 2595b5cc) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/workflows/ci.yml — added 14 line(s) in 3 range(s)
added: 114-122 · 128 · 142-145
### pulse-app/src/console.rs — added 8 line(s) in 6 range(s)
added: 3-5 · 14 · 19 · 26 · 40 · 65
### pulse-app/src/door_bridge.rs — new file · 145 line(s)
- 27-48 @30 «pub fn main() -> ExitCode {»
  - 31-45 @32 «{»
- 50-145 @51 «mod unix {»
  - 64-73 @68 «pub fn socket_path(data_dir: &Path) -> Option<PathBuf> {»
  - 75-112 @78 «pub fn relay<R: BufRead, W: Write>(»
  - 114-129 @116 «fn exchange<W: Write>(»
  - 131-144 «fn discard_line<R: BufRead>(input: &mut R) -> io::Result<()> {»
### pulse-app/src/lib.rs — added 1 line(s) in 1 range(s)
added: 15
### pulse-app/tests/integration_console_engine.rs — added 1 line(s) in 1 range(s)
added: 905
### pulse-app/tests/integration_door.rs — added 341 line(s) in 5 range(s)
added: 16 · 21 · 267-273 · 288 · 811-1141
  - 267-268 «serde_json::from_str(&ask_line(socket, request).await)»
- 815-816 «const SKIP_BRIDGE_NO_SECOND_ACCOUNT: &str =»
- 819-823 «struct Relayed {»
- 825-846 @826 «fn relay_through(mut command: Command, input: &str) -> Relayed {»
  - 830-835 «let mut child = command»
  - 841-845 «Relayed {»
- 848-863 @851 «fn owner_relay(data_dir: &Path, input: &str) -> Relayed {»
  - 854-858 «command»
  - 859-861 «if let Some(profile) = std::env::var_os("LLVM_PROFILE_FILE") {»
- 865-889 @866 «fn file_set(dir: &Path) -> BTreeSet<PathBuf> {»
  - 867-885 «fn walk(root: &Path, dir: &Path, out: &mut BTreeSet<PathBuf>) {»
- 891-1021 @892 «async fn a_stdio_agent_reaches_the_door_through_the_bridge_and_leaves_one_call_per_request() {»
  - 899-903 «engine»
  - 906-921 «let requests = [»
  - 922-925 «let input: String = requests»
  - 935-939 «assert_eq!(»
  - 940-943 «let answers: Vec<Value> = answer_lines»
  - 945-948 «assert_eq!(»
  - 950-952 «let tools = answers[1]["result"]["tools"]»
  - 954-958 «let writes: Vec<&str> = tools»
  - 960-966 «assert!(»
  - 969-970 «let pid = std::fs::read_to_string(engine.data_dir.path().join("run/andromeda-pulse.pid"))»
  - 971-975 «assert_eq!(»
  - 984-999 «let shapes: Vec<(&str, &str, &str)> = calls»
  - 1000-1010 «assert_eq!(»
  - 1011-1014 «assert!(»
  - 1016-1020 «assert_eq!(»
- 1023-1141 @1024 «async fn another_account_through_the_bridge_receives_nothing() {»
  - 1026-1031 «let has_second_account = sudo(None)»
  - 1032-1035 «if !has_second_account {»
  - 1039-1042 «let socket = engine»
  - 1056-1062 «if !(reachable_by_another_account(&data_dir) && reachable_by_another_account(&copy)) {»
  - 1064-1067 «let ping = format!(»
  - 1068-1077 «let as_account = |account: Option<&str>| {»
  - 1088-1093 «let readings = [»
  - 1094-1115 «for (case, reading) in readings {»
  - 1125-1129 «assert_eq!(»
  - 1130-1135 «for refusal in &refusals {»
### pulse-app/tests/quality_gate_workflow.rs — added 70 line(s) in 17 range(s)
added: 942-945 · 975 · 986-988 · 991 · 1001-1002 · 1004-1010 · 1014-1019 · 1026-1032 · 1062 · 1064 · 1067-1070
       1075-1077 · 1079-1084 · 1090-1093 · 1099 · 1118-1134 · 1147-1148
- 943-944 «const DOOR_BRIDGE_REFUSED: &str = "door bridge other-account arm: no byte reached another account \»
  - 986-988 «let selection = format!(»
  - 1004-1009 «for sentence in [DOOR_THREE_LAYERS, DOOR_BRIDGE_REFUSED] {»
  - 1026-1032 «for sentence in [DOOR_THREE_LAYERS, DOOR_BRIDGE_REFUSED] {»
  - 1076-1077 «let (code, printed) =»
  - 1079-1084 «for sentence in [DOOR_THREE_LAYERS, DOOR_BRIDGE_REFUSED] {»
- 1118-1133 @1119 «fn door_other_account_step_fails_a_bridge_arm_that_skipped() {»
  - 1120-1121 «let (code, printed) =»
  - 1122-1126 «assert_eq!(»
  - 1127-1132 «assert!(»
  - 1147-1148 «let (code, printed) =»
### pulse-app/tests/unit_console_commands.rs — added 13 line(s) in 6 range(s)
added: 3-5 · 28-32 · 51-52 · 64 · 97 · 99
- 28-31 @29 «fn door_is_the_door_command() {»
### pulse-app/tests/unit_door_bridge.rs — new file · 371 line(s)
- 24-30 @26 «fn data_dir() -> tempfile::TempDir {»
- 32-34 «fn door_socket(data: &Path) -> PathBuf {»
- 36-43 @40 «struct StandIn {»
- 45-88 «impl StandIn {»
  - 46-79 «fn bind(socket: &Path, answers: Vec<Vec<u8>>) -> Self {»
  - 81-87 @83 «fn taken(self) -> Vec<Vec<u8>> {»
- 90-97 «fn assert_no_connection(listener: &UnixListener) {»
  - 92-96 «match listener.accept() {»
- 99-101 «fn line(text: &str) -> Vec<u8> {»
- 103-107 «fn relay_input(input: &[u8], socket: &Path) -> (Result<(), Unreachable>, Vec<u8>) {»
- 109-132 @112 «fn door_word(data: &Path) -> Output {»
  - 115-122 «command»
  - 123-125 «if let Some(profile) = std::env::var_os("LLVM_PROFILE_FILE") {»
- 134-146 «fn assert_unreachable(output: &Output, case: &str) {»
  - 135-139 «assert_eq!(»
  - 140-144 «assert_eq!(»
- 148-156 @149 «fn data_dir_with_socket_path_of(root: &Path, length: usize) -> PathBuf {»
- 158-189 @159 «fn each_request_line_takes_a_connection_of_its_own_in_order_and_each_answer_passes_unchanged() {»
  - 161-165 «let requests = [»
  - 166-171 @167 «let answers = vec![»
  - 179-183 «assert_eq!(»
  - 184-188 «assert_eq!(»
- 191-203 @192 «fn a_line_the_door_does_not_answer_writes_nothing_and_still_took_one_connection() {»
- 205-216 @206 «fn whitespace_only_lines_open_no_connection() {»
- 218-256 @219 «fn a_line_over_the_bound_is_sent_one_byte_past_it_and_the_next_line_whole_on_its_own_connection() {»
  - 224-227 «let door = StandIn::bind(»
  - 235-239 «assert_eq!(»
  - 240-244 «assert_eq!(»
  - 245-249 «assert_eq!(»
  - 250-254 «assert_eq!(»
- 258-316 @259 «fn a_door_that_cannot_be_reached_is_unreachable_and_nothing_is_written() {»
  - 262-266 «assert_eq!(»
  - 275-276 «std::os::unix::fs::symlink(outside.path(), linked.path().join("run").join("door"))»
  - 277-281 «assert_eq!(»
  - 298-309 @299 «for (data, case) in [»
  - 310-313 «assert!(»
- 318-330 @319 «fn a_socket_path_of_107_bytes_is_reached() {»
- 334-342 «impl Write for RefusingOutput {»
  - 335-337 «fn write(&mut self, _: &[u8]) -> io::Result<usize> {»
  - 339-341 «fn flush(&mut self) -> io::Result<()> {»
- 344-362 @345 «fn an_answer_that_cannot_be_written_out_ends_the_relay_unreachable() {»
  - 350-354 «let relayed = relay(»
  - 357-361 «assert_eq!(»
- 364-371 @365 «fn the_fixed_line_and_its_exit_code() {»
  - 366-369 «assert_eq!(»
