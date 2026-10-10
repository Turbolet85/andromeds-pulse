# Scope — Engine harness proofs completed

**Marker:** `2026-10-10-engine-harness-proofs-completed` · version `andromeda-pulse-0.4.0` · Epoch 1 — Foundation: base CI, the capability record, a console engine with its door, its gates
**Working entry (`working-route.md:33`):** "Engine harness proofs completed — rejected port, socket census, lock-file
place read on a spawned engine; pre-push reads a real engine cycle; cleanup, boot, logs verbs pinned in shell (no
capability claimed)"
**Chunk base:** `514fbe57` (HEAD at take-up). Every diff-shaped probe of this chunk names this sha, never HEAD.

## Intent
The two chunks before this one made the engine a console program and aimed the agent harness at it. Both closed
with proofs owed: readings of the new boot that no committed test makes, a pre-push stage that grades a record it
wrote itself, harness verbs whose arms were driven by hand once, and checks whose red reading was taken against
stubs. This chunk makes those proofs and commits them, so that each of the four test-plan §1 pending rows the entry
names has a committed test or a recorded reading behind every clause it lists as owed. It builds no new product
surface and claims no capability.

## What the chunk builds (the entry's own words)
- **Rejected port, socket census, lock-file place read on a spawned engine.** Three readings of the engine boot,
  each taken from a spawned engine: a rejected port variable leaves that receiver unstarted; the process listens
  on the two loopback receivers and nothing else; the corpus-key lock file's directory is read.
- **Pre-push reads a real engine cycle.** The sixth stage of `cargo xtask pre-push:linux` grades a real console
  engine cycle in place of a seed record the stage wrote itself.
- **Cleanup, boot, logs verbs pinned in shell.** Committed shell-level tests hold the arms of the three verbs of
  `scripts/agent-run.sh`.

## What the directive adds (inputs#I1, named by inputs#I2 — closed at P3, or at P4 where the bullet says so)
- **Each proof is a reading made on a spawned engine or a real cycle, not on a constructed input alone**
  (inputs#I1 item 1): "the rejected-port arm, the socket census, the lock file's directory, one mutation per arm
  of `check:engine-log`, the four refusal labels, and the shell-level pins of `cleanup`, `boot` and `logs`."
  Verified at P3 as reachable for each (research, "What each owed proof is read on"): the three engine-boot
  readings on a spawned engine with no stand-in; each mutation read on its unit pins and on the kept log of a
  real cycle; the shell pins on a real engine through the committed script, with three named stand-ins (a
  `cargo` that skips the pre-build, the test's own listener, the test's own unreaped child). One class has no
  spawned reading by construction: the four refusal labels are returned before anything starts
  (`xtask/src/engine_cycle.rs:219-258`, all above the boot at `:261`), so they are read on the built verb under a
  constructed environment. The plan states which is which.
- **The pre-push sixth stage is the one design question, and it goes to the operator at P4** (inputs#I1 item 2).
  The directive's reading, kept with its marker: "measured in the record of the chunk that made the check native
  (`2026-10-09-pre-push-check-native-on-linux`): its plan states that no stage starts `pulse-app` or binds a
  port, and P-103 is VERIFIED on six named stages in a named order. A sixth stage that runs a real engine cycle
  starts a program and binds two ports." Both halves verified at P3: that chunk's `plan.md:20` and `:402` and
  the verb's own header (`xtask/src/pre_push.rs:14`) state the sentence; `matrix.py show --id P-103` reads
  `verified`, its acceptance naming `script-modes`, `source-lint`, `npm`, `clippy`, `test`, `ci-gates` in that
  order. Research adds: P-103's acceptance states the stage names, their order and the verdict, and nothing
  about ports or about what the `ci-gates` stage reads. P4 brings the operator: what the stage becomes; what the
  check's contract then states (ports, a second engine already running on this host, the time it adds); the
  change shown against P-103's concretized acceptance. "A verified capability is not re-worded by a later
  chunk's convenience."
- **Other builders run on this host** (inputs#I1 item 3). A stage or leg that binds a port refuses cleanly when
  the port is taken and says so; it never waits on, kills or reuses another session's process. Ports 4317 and
  4318 are not used. Read at P3, by hand on a spawned engine whose gRPC port the reading's own listener held
  (research, Hand readings, n = 1): the engine keeps running and logs an ERROR bind record; `harness:ready
  --program console` reads `ready`, because it accepts any listener on the two ports as the engine's; and
  `harness:engine-cycle` has no refusal for a taken port (`xtask/src/engine_cycle.rs:105-118`), so its injector
  would feed the holder. The cycle is the leg this chunk's pre-push question rests on, so the refusal is this
  chunk's to build; whether the readiness verb's own reading is closed here for both programs is put to the
  operator at P4.
- **The socket census and the lock file's place are readings, not repairs** (inputs#I1 item 4). A reading that
  shows something that should not stand ("a socket beyond the two loopback receivers; a lock file in the shared
  temp dir, which `One place on a node` owns") is recorded and brought at P5. That entry's subject
  (`working-route.md:41`) is not built here. Read at P3 by hand (research, Hand readings, n = 1 each): the census
  shows the two loopback receivers and no other listener; the lock file sits in the directory the rule yields
  for the environment supplied, and under the existing spawned form's environment (no temp-dir variable, no
  runtime dir) that directory is the shared temp dir, where the file stands on this host. That second reading is
  the one the directive names; it is brought at P5.

## Folded freight (five `CARRY:` blocks on `working-route.md:33`; `route.py pins` lists five rows for the line)
Each is carried as a hypothesis. Its named coordinates were checked at take-up; its claims are closed at P3
against the tree at the chunk base.
- **C1 — provenance, the four rows, no claim.** "minted at 2026-10-10-agent-harness-drives-the-console-engine's
  wrap on the operator's word (the pc overseer, 2026-10-10: the answer at that chunk's plan dialog that cut its
  freight in two, the wrap directive item 2, and the word at the route-resolve card); it discharges four test-plan
  §1 pending rows — engine-boot-rejected-port-and-socket-census-coverage,
  harness-cleanup-verdict-and-boot-spawn-shell-coverage, harness-log-family-resolution-coverage and
  engine-log-arm-mutation-and-engine-cycle-glue-coverage — and claims nothing in the ledger: P-086's open half is
  carried by the four entries that name P-086". Checked at take-up: the four rows stand at
  `.andromeda/test-plan.md:154`, `:135`, `:134` and `:155`. Each row ends in its own "Owed" list, and two of them
  owe more than the entry's title names (below, "The rows' owed lists").
- **C2 — three readings the engine boot's tests do not make:** "the rejected-port arm of engine_boot::start, a
  socket census on a spawned run, the corpus-key lock file's directory (moved from that chunk's completed line;
  record: 2026-10-10-console-engine-entry-point; the operator's word at that card, 2026-10-10)". Checked at
  take-up: the `invalid_port` arms stand at `pulse-app/src/engine_boot.rs:852-873`; the in-process and spawned
  suites exist (`pulse-app/tests/integration_engine_boot.rs`, `integration_console_engine.rs`). The row's claims
  verified at P3: no committed test drives `start` or the console program with a rejected port (`grep -rn -E
  'grpc_port: None|http_port: None' pulse-app/tests xtask/src crates`: 0 hits; `grep -n -i -E
  'invalid_port|rejected'` over the two suites: 0 hits); no test reads a socket list (`grep -rn -l -E
  '/proc/|proc/net'` over the same trees: 0 files); the spawned suite holds no line naming the lock file.
- **C3 — the pre-push sixth stage.** "the sixth stage of cargo xtask pre-push:linux runs ci-gates over a seed
  record the stage wrote itself and is not a step of a CI job; its doc comment in xtask/src/pre_push.rs was
  corrected at that chunk (one boot record and two ticks), and the stage is read over a real engine cycle here
  (moved from that chunk's completed line; measured at 2026-10-10-no-gate-stands-while-reading-nothing)". Checked
  at take-up: `Stage::CiGates` is the sixth of six (`xtask/src/pre_push.rs:208-213`) and calls `seed` first
  (`:278-279`, `seed` at `:520`). Verified at P3: the seed is one `app.boot.ready` record and two `ingest.tick`
  records 15 s apart (`SEED_LOG`, `:32-39`), and the stage runs `cargo xtask ci-gates` over it (`:280`), which
  reads two arms, the record count and the panic read. What the stage becomes is the P4 question above.
- **C4 — shell-level pins.** "the shell-level pins the widened shell-coverage row owes — cleanup's four tokens,
  boot's failure arms, the logs verb's family branch, and the word arms of boot and status (none, engine, any
  other word: usage, exit 2), which shipped driven by hand; agent-run.ps1 takes no engine word". Checked at
  take-up: `scripts/agent-run.sh` holds the five verbs as case arms (`boot` `:69`, `status` `:216`, `cleanup`
  `:227`, `logs` `:315`), the four cleanup tokens (`:293-300`) and the usage exit (`:38-48`); `agent-run.ps1`
  holds no line naming `engine`; the tree holds no committed shell-level test of the script (`git ls-files`: no
  `.bats` file and no `tests/*.sh`). The `.ps1` twin is not this chunk's: its missing `engine` word and its unrun
  arms are carried on `Other operating systems retired from the code` (verified at P3: `working-route.md:64`
  names `agent-run.ps1` twice and `Invoke-Ready` once).
- **C5 — mutations and refusal labels.** "the row opened at that wrap owes one mutation per arm of cargo xtask
  check:engine-log (its red reading is against stubs that pass everything: 55 of 66 failed, then 66 of 66 passed)
  and a pin for each of the four refusal labels only engine_cycle::drive returns (home-unset, path-unset,
  data-dir-unusable, injector-build-failed); a red cycle has not been seen on a runner (the operator's word at the
  card, 2026-10-10)". Checked at take-up: the four labels stand in `xtask/src/engine_cycle.rs` (`:219`,
  `:236-239`, `:258`); `xtask/src/engine_log.rs` holds seven functions named `*_arm` (`family`, `program`,
  `panic`, `heartbeat`, `progress`, `process_end`, `budget`). Verified at P3: `evaluate` builds exactly those
  seven arms, in that order (`xtask/src/engine_log.rs:156-164`), so "one per arm" is seven mutations; the two
  modules hold 45 and 21 tests (`grep -c '#\[test\]'`). A runner reading of a red cycle
  "when one occurs" is not something this chunk can make happen; it stays an open clause of the row unless a red
  cycle is read on a runner during this chunk.

## The rows' owed lists (read whole at take-up; what this chunk owes each)
- `engine-boot-rejected-port-and-socket-census-coverage` (`test-plan.md:154`): "an in-process or spawned arm with a
  rejected port variable reading the failed-bind record and a refusing port; a socket census on the spawned run; an
  assertion on the lock file's directory." All three are this chunk's (C2).
- `engine-log-arm-mutation-and-engine-cycle-glue-coverage` (`:155`): "one mutation per arm of `check:engine-log`; a
  pin per remaining refusal label; a runner reading of a red cycle when one occurs." The first two are this
  chunk's (C5); the third is conditional. The row also records that `engine_cycle::drive` and
  `engine_log::run_check` / `run_settled` have no unit pin and are read only by the whole cycle (verified at P3:
  the graph shows `drive` called by `run` alone, and no test calls either glue function); whether the pins of
  the four labels and the pre-push stage's real cycle close that clause is said at P4.
- `harness-log-family-resolution-coverage` (`:134`): "the harness-level assertion for the shell `logs` verb". This
  chunk's (C4).
- `harness-cleanup-verdict-and-boot-spawn-shell-coverage` (`:135`), widened seven times. Its owed clauses, as
  read whole at take-up and checked against the route at P3 (`working-route.md:64` names the `.ps1` script;
  `:50` names the exit witness and `harness:boot-series`): (a) a harness-level assertion per `cleanup` verdict arm; (b) one per `boot` failure-reason
  arm, "the receivers line printed and silent included"; (c) a committed ps1 boot-cycle leg covering the
  recorder's arms and `Invoke-Ready`; (d) one per exit-witness-arm branch (unset, set, refused); (e) a live
  reading of `harness:boot-series`' `timed-out` path; (f) one per word arm of `boot` / `status` (none, `engine`,
  any other). The entry and C4 name (a), (b) and (f). (c) is the `.ps1` twin's, carried on `working-route.md:64`.
  (d) and (e) are the window app's boot arms, which leave at `Window's gates retired` (`working-route.md:50`).
  P4 says for (c), (d) and (e) whether this chunk builds the pin, or the row stays open on that clause with its
  owner named; "discharges" is not written over a clause that was not closed.

## Decided at P4 (the operator's answers, the pc overseer, 2026-10-10, given here — inputs#I3)
- **Stage six runs one real console engine cycle, then `ci-gates` over that cycle's log**, in place of the seed.
  The six stage names, their order and the six verdict members stay as P-103 verified them; its acceptance is
  not re-worded and gets a dated ledger note. The operator's reason: "a stage that reads a record it wrote itself
  reads nothing, and the check exists to predict CI, which now runs the cycle." The check now starts a product
  program and binds two loopback ports, which the playbook's Boundary-widening pattern names. **Recorded
  PROVISIONAL**, in the operator's words: "the playbook calls it a widening, so the founder own word is owed; it
  is my reading meanwhile, dated today." Conditions, verbatim: "loopback only, never 4317 or 4318, a taken port
  is a clean refusal that names it and reads cannot-evaluate, never green; the stage ends what it started
  whichever way it ends, and that is pinned."
- **`harness:ready` is fixed at its source, for both programs.** "ready is read only when the run own log holds
  both receivers bound records, for both programs. This host is shared - a foreign listener must never be taken
  for the program under test." This widens the chunk past the entry's own words, on the operator's word: the
  readiness decision and its pins, a label for a port that accepts but is not this run's, the `boot` verb's
  failure line for it, and the window smoke in the gate block (the window program's `boot` reads readiness
  through the same decision).
- **For the wrap card**, the operator's word: "bring to the wrap card, with your lean, the product side you
  measured: the console engine stays up and ticks with a receiver that failed to bind." The plan carries the
  reading and the lean; no product code changes here.
- **Leaned at P4 with no question, each settled by an artifact:** the three clauses of the shell-coverage row
  this chunk does not build keep their owners on the route — (c) the `.ps1` leg on `working-route.md:64`, (d)
  the exit-witness branches and (e) the boot series' `timed-out` path on `:50`, both the window app's. The row
  stays open on those three clauses; it is not written discharged over them.

## Capability
- none claimed (the entry: "no capability claimed"; C1: "claims nothing in the ledger"). P-086's open half stays
  with the four entries that name it. P-103 (`verification-matrix.json#P-103`) is read, not claimed: the P4
  question shows the stage's change against its concretized acceptance.

## Boundaries (inputs#I1 items 3, 4 and 5)
- "A new listening socket, place on disk, credential, command word of the engine, or anything reachable from
  another host halts for my word."
- Ports 4317 and 4318 are shared on this host and are not used; a leg that binds a port refuses cleanly on a taken
  port and never waits on, kills or reuses another session's process.
- A reading that shows what should not stand is recorded and brought at P5, not repaired. `One place on a node`'s
  subject (operator-set locations, a `stop` command, the lock file's place) is not built here.
- The window app, its boot job and its series stand after this chunk as before it.
- The spec masters are read-only at phase; what this chunk changes in them is the wrap's.

## Process items carried by the directive (inputs#I1 items 5 and 6; inputs#I2)
- The plan lists the merge-base probe directly before its push entry.
- At P5: what `planlint` check 4 listed, the plan card, and a stop for the operator's `yes`.

## Second fold source — the CI verdict read at Setup 5a
- One sha since the last wrap's flip: `514fbe57` (the wrap's own commit; the flip is in it). `ci#38072937534` on it
  read **in progress** at take-up (checks 7/7 listed, 6 running, the oldest the supply-chain job at 206 s);
  `secret-scan#38072937478` completed, success. The verdict was not yet available: it is not folded and not read
  as green. Read again at P3: still in progress (1 running, the coverage gate at 955 s), not folded and not read
  as green.
  **Read again at P5, after the run closed: `verdict: green`, checks 7/7, wall 1462 s** (`ci#38072937534`
  completed, success). No red and no `not green` is carried into this chunk.

## Gate
- none — the entry carries no `BLOCKED-ON` (`route.py cursor`: no annotation-position line; `route.py pins`: five
  rows for the line, all `CARRY:`).
