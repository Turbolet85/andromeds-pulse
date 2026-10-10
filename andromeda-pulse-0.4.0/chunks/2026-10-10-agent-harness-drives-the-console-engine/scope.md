# Scope — Agent harness drives the console engine

**Marker:** `2026-10-10-agent-harness-drives-the-console-engine` · version `andromeda-pulse-0.4.0` · Epoch 1 — Foundation: base CI, the capability record, a console engine with its door, its gates
**Working entry (`working-route.md:31`):** "Agent harness drives the console engine — five verbs target it, verdict
arms pinned by tests; panic, heartbeat-gap, process-end, budget checks grade its log (P-086)"
**Chunk base:** `8394da4f` (HEAD at take-up). Every diff-shaped probe of this chunk names this sha, never HEAD.

## Intent
The engine has been a console program, `andromeda-pulse-engine` (`run` | `version`), since the chunk before this
one. The agent harness has not followed it: the five verbs of `scripts/agent-run.sh` build and spawn
`target/release/pulse-app` by path, and the checks that read a boot's log read the window app's. This chunk aims
the harness at the console engine, so that an agent can start it, ask whether it is up, read its log and end it by
the same five verbs, and so that the checks which grade a run read the engine's own log and can fail on it. The
window app and its boot job stand beside it until Epoch 2.

## What the chunk builds (the entry's own words)
- **Five verbs target it.** `boot`, `run`, `status`, `cleanup` and `logs` drive `andromeda-pulse-engine`.
- **Verdict arms pinned by tests.** Each verdict a verb or a check can return has a test that fails if the arm
  stops returning it.
- **Panic, heartbeat-gap, process-end, budget checks grade its log.** Four checks read the console engine's log
  family: no panic record, no heartbeat stall, exactly one record of how the process ended, and the budget arms.

## What the directive adds (inputs#I1 — closed at P3, or at P4 where the bullet says so)
- **One table of every carried reading, before planning** (inputs#I1 item 1): what it is, where it was measured,
  and whether this chunk BUILDS it for the console engine, RETIRES it (the master sentence amended at the wrap,
  with the reason), or hands it to a named later entry because its subject does not exist yet. No fourth column.
  Closed at P4: the table stands in `plan.md`, over the readings under "Folded freight" below.
- **Two programs, one harness, until Epoch 2** (inputs#I1 item 2). The directive's reading, kept with its marker:
  "measured at the last chunk: the window's boot job launches `target/release/pulse-app` by path and stays green
  on eight boots a run." Verified at P3 at the chunk base: `boot` builds and spawns that binary by name
  (`scripts/agent-run.sh:61`, `:77`), and the `boot` job runs one smoke boot and a series of seven
  (`.github/workflows/ci.yml`, `harness:boot-series --count 7`). Its last settled verdict is the prior chunk's
  (`ci#38056942758`, green; that chunk's `evidence/operator-pass.md`). Research adds what the plan rests on: of
  the five verbs only `boot` names a program, and of the xtask harness verbs only `harness:settled` is
  window-shaped; `status`, `cleanup`, `logs` and `harness:ready` act on the pid file, the log family and the two
  ports, which either program writes the same way (research, Measured facts). The plan says how the same five
  verbs serve both programs, which verb stays the window's until `Window's gates retired`
  (`working-route.md:48`), and what forbids a verb from quietly grading the wrong program.
- **The checks grade a log that can fail them** (inputs#I1 item 3). The directive's reading, kept with its marker:
  "The heartbeat-gap and budget arms left `ci-gates` at an earlier chunk because a boot of six seconds could never
  fail them." Verified at P3 against `.andromeda/obs-plan.md:559` and `:560`: the boot log held one tick per
  target and one memory sample, so neither arm had a reading that could fail it. A check built here over the
  console engine's log is shown red on a constructed log before it is trusted green — each arm, by a pin — and
  the run it grades is held long enough to hold what the check reads (research: a consecutive tick pair needs
  about 16 s; a non-zero memory sample needs a retention sweep after spans landed).
- **The console program is ended by signal only today** (inputs#I1 item 4). Verified at P3
  (`pulse-app/src/console.rs:27-42`, `:94-96`): the grammar is `run` | `version`, and `run` ends by SIGTERM or
  SIGINT. Its pid file's place and a `stop` command are `One place on a node`'s (`working-route.md:39`). The plan
  says what the verbs use now to find and end the engine and what that entry replaces. That entry's subject is
  not built here.
- **If the freight does not fit one chunk, the cut is proposed at P4** (inputs#I1 item 5): a first chunk that ends
  with verbs that run and are gated, and what the second holds. The route is the operator's to change at a wrap.

## Folded freight (five `CARRY:` blocks on `working-route.md:31`; `route.py pins` lists five rows for the line)
Each was carried as a hypothesis and closed at P3 against the tree at the chunk base.
- **C1 — two readings obs-plan §10 CI gates states as unmet belong to this entry** ("the operator's word, the pc
  overseer, 2026-10-10, at 2026-10-10-no-gate-stands-while-reading-nothing's route-resolve card"). (a) "the
  heartbeat-gap check is made by no CI step since that chunk (cargo xtask ci-gates left it;
  xtask/ci/heartbeat-gap-check.sh's one caller is perf:load-profiles, which no workflow runs)"; (b) "'a test run
  that produces zero spans fails the build' was never made by any step (cargo xtask test reads no log; measured
  at that chunk)". "This entry's plan rules for each whether the engine version of the check is built or the
  sentence retired, and nothing is retired before then." Verified at P3: `xtask/src/ci_gates.rs` holds two arms;
  the script's one caller is `invoke_heartbeat_check`, reached from `run_perf_load_profiles` alone
  (`xtask/src/main.rs:614`, called at `:742` inside `run_perf_load_profiles`, `:703`); `grep -rn 'load-profiles\|heartbeat' .github/workflows/` finds nothing; the
  body of `run_cargo_nextest` reads no log. Research adds: the script cannot grade a console log as it stands
  (it ignores `connection.tick` and passes on no tick and on one tick).
- **C2 — the pre-push check's sixth stage** "runs ci-gates over one seed record the stage wrote itself and reads
  two lines over it since that chunk (the record count and the panic read; measured there, exit 0); it is not a
  step of a CI job, so P-128's sentence does not reach it; the seed's doc comment in xtask/src/pre_push.rs still
  says it drives the heartbeat arm and the grader's empty-arm reading, which the verb no longer runs".
  `[premise-corrected: SEED_LOG read at xtask/src/pre_push.rs:32-39]` The seed holds three records, not one: one
  `app.boot.ready` record and two `ingest.tick` records 15 s apart. The rest holds: the stage writes the seed and
  runs `cargo xtask ci-gates` over it (`:279-280`), and the doc comment (`:30-31`) describes two readings the verb
  no longer makes.
- **C3 — `cargo xtask check:ingest-progress`**, "a standard gate entry and not a CI step, prints 'NEUTRAL — no
  buffer.tick events in the log family' and exits 0 over a log family with no tick (seen at that chunk's gate
  block on the dev host, not measured further); the progress read over the engine's log is this entry's (the
  operator's word, as above)". Verified at P3 (`xtask/src/ingest_progress.rs:86-90`, `xtask/src/main.rs:502-505`).
- **C4 — the console program's own panic and at-exit records** "are witnessed one level down, in re-exec children
  of engine_boot::init_process (integration_engine_boot), never on the console program itself, which is ended
  only by SIGTERM and SIGINT in its tests; the harness verbs that read run/andromeda-pulse.pid are not aimed at
  it although it writes that file (record: 2026-10-10-console-engine-entry-point; the operator's word at the
  card, the pc overseer, 2026-10-10)". Verified at P3 (`pulse-app/tests/integration_engine_boot.rs:531`, `:574`;
  `pulse-app/tests/integration_console_engine.rs:355`, `:475`; `pulse-app/src/engine_boot.rs:206`). Research
  adds: no input of the console program reaches a panic or a C `exit()` after its log sink exists, so a witness
  on the program itself needs a trigger the product would have to read.
- **C5 — three readings the new boot's tests do not make**, test-plan §1's row
  `engine-boot-rejected-port-and-socket-census-coverage` (`.andromeda/test-plan.md:154`): the rejected-port arm of
  `engine_boot::start`, a socket census on a spawned run, the corpus-key lock file's directory ("record:
  2026-10-10-console-engine-entry-point; the operator's word at the card, 2026-10-10"). Verified at P3: no test
  passes a rejected port to `start` or to the console program (`grep -rn 'grpc_port: None\|http_port: None'
  pulse-app/tests`: 0 hits; the four `invalid_port` tests in `unit_engine_boot_env.rs` pin the resolver alone).

## Decided at P4 (the operator's answers, the pc overseer, 2026-10-10 — inputs#I2)
- **The freight is cut in two.** This chunk ends with verbs that run on the console engine and are gated: `boot`
  and `status` aimed at it with a wrong-program guard, the check over its log (panic, heartbeat gap, process end,
  memory budget, progress) pinned red per arm, one live cycle in the gate block and one CI step. The operator
  mints the second entry at this chunk's wrap, directly after this one.
- **What the second entry holds** (none of it built here): C5's three readings (the rejected-port arm, a socket
  census of a spawned engine, the lock file's directory); the pre-push check's sixth stage run over a real engine
  cycle in place of the seed it writes (the rest of C2); shell-level pins of `cleanup`'s four tokens, `boot`'s
  failure arms and the `logs` verb's family branch.
- **C4's first half is retired with its reason, and no trigger is built.** The amended sentence says where the
  witness is made (children of the process-start function both programs call) and that the console program
  itself has no input that reaches a panic. The harness-side panic and process-end checks over the engine's log
  are built and pinned here.
- **Three new harness readings are routine harness evidence, on the operator's own reading of 2026-10-10:** the
  cycle's data dir under `target/engine-cycle/` (the runner's temp dir in CI), harness-written and ignored by
  git; a CI upload `logs-engine-Linux` holding the engine's own log family, which fails on no file; the dev
  injector `inject_demo` reading the registered gRPC port variable to pick its port. None runs inside the product
  process or adds a product input.

## Capability
- **P-086** (`verification-matrix.json#P-086`, `planned`, unclaimed, advanced by the chunk before this one).
  Acceptance: "The engine starts as a console program on a Linux server with no display, no GPU and no desktop
  session, runs in the background and answers its commands; every kind of finding 0.3.0 detected, one incident
  for a sustained storm and automatic resolution are each still observed on the same telemetry."
  Six working-route lines name it, read at take-up: `:29` (frozen, complete), this one, `:37`, `:43`, `:45` and
  `:81` (inputs#I1 item 7 says six). P5 shows what of the acceptance this chunk shows and claims nothing that was
  not shown.

## Boundaries (inputs#I1 item 6)
- A listening socket other than the two loopback receivers, a new place on disk, a new credential, anything
  reachable from another host, or a new command word of the engine halts for the operator's word.
- Ports 4317 and 4318 are shared on this host: the legs of this chunk use other ports.
- The window app, its boot job, its eight boots a run and its a11y job stand after this chunk as before it;
  `Window's gates retired` and `Window retired` remove them.
- `One place on a node`'s subject (operator-set locations for the stores, the log and the pid, and a `stop`
  command) is not built here.
- The spec masters are read-only at phase; what this chunk changes in them is the wrap's.

## Process items carried by the directive (inputs#I1 items 7 and 8)
- The plan lists the merge-base probe directly before its push entry.
- At P5: what `planlint` check 4 listed, what of P-086's acceptance this chunk shows, the plan card, and a stop
  for the operator's `yes`.

## Second fold source — the CI verdict read at Setup 5a
- One sha since the last wrap's flip: `8394da4f` (the wrap's own commit; the flip is in it). `ci#38061540583` on
  it read **in progress** at take-up (checks 7/7 listed, 6 running, the oldest the coverage gate at 191 s);
  `secret-scan#38061540570` completed, success. Read again at P3: still in progress (1 running, the coverage gate
  at 973 s), and it was not folded or read as green then.
  **Read again at P5, after the run closed: `verdict: green`, checks 7/7, wall 1103 s** (`ci#38061540583`
  completed, success; attempt 1). No red and no `not green` is carried into this chunk.

## Gate
- none — the entry carries no `BLOCKED-ON` (`route.py cursor`: no annotation-position line; `route.py blocked`:
  0 blocks on a pending, gated or markerless line).
