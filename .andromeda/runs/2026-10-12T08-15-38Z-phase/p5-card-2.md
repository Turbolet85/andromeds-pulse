# P5 review — 2026-10-12-stdio-agent-reaches-the-door

Plan: `andromeda-pulse-0.4.0/chunks/2026-10-12-stdio-agent-reaches-the-door/plan.md` (325 prose lines, 23 gate entries,
15 acceptance criteria). Form: the operator's P4 answer, PROVISIONAL (the pc overseer, 2026-10-12, inputs#I3).

## Goal
`andromeda-pulse-engine door` is a short-lived stdio relay to the console engine's door.
- It reads the data dir as `run` does and resolves `run/door/sock` under it, canonical and confined, under 108 bytes.
- Each stdin line gets its own connection: line written, write side closed, the door's bytes copied to stdout unchanged.
- stdin ending is exit 0. A door it cannot reach is exit 1 with the one stderr line
  `andromeda-pulse-engine door: the door cannot be reached`.
- It never writes an answer of its own, and adds no record, file, pid file, variable, listener or binary.
- The door, the tools crate and the sidecar are byte-identical to `d3e694d2` (the scope guard).

## Steps (8)
1. `console.rs`: the third word `door`; usage `<run|version|door>`; no runtime, no `init_process`.
2. `door_bridge.rs` (new, `pub mod` in `lib.rs`): the relay.
3. `unit_door_bridge.rs` (new): in-process arms over a stand-in listener (one connection per line, notification, blank
   line, over-bound line, four unreachable cases, the fixed line).
4. Grammar pins: `unit_console_commands.rs`, `integration_console_engine.rs:905`.
5. `integration_door.rs`: the owner through the relay on a spawned engine — initialize, a notification, tools/list
   (byte-equal to the direct read), query_logs with a planted marker, the write switched off (`-32001`). It asserts
   four `door.call`, no `door.refused`, one `app.boot.engine`, the marker 0 times, the pid file still the engine's,
   and no new file in the data dir.
6. `integration_door.rs`: another account through the relay. `nobody` and root, at the modes as made and opened, run
   a copy of the engine from the arm's own opened TempDir. No byte reaches another account, and one `door.refused`
   is left per reading that reached the door. The arm prints its sentence.
7. `ci.yml`: the `lint-test` step `Door refuses another account` selects both other-account arms in its one cargo
   command and also requires the relay arm's sentence.
8. `quality_gate_workflow.rs`: the step's pins read the two-arm selection and both sentences.

## Touchpoints
- New: `pulse-app/src/door_bridge.rs`, `pulse-app/tests/unit_door_bridge.rs`.
- Modify: `pulse-app/src/console.rs`, `pulse-app/src/lib.rs`, `pulse-app/tests/unit_console_commands.rs`,
  `pulse-app/tests/integration_console_engine.rs`, `pulse-app/tests/integration_door.rs`, `.github/workflows/ci.yml`,
  `pulse-app/tests/quality_gate_workflow.rs`.

## Test commands (23 entries)
- **Static checks:** fmt · clippy · the scope guard (new) · the relay grep (new).
- **Test runs:** the unit vehicle (new) · the seven-times run of the five engine/port binaries · the workspace run.
- **xtask checks:** english-sources, quarantine-tracking, capability-widening-check, ingest-progress,
  staged-artifacts, capability-drift, verify:capability-matrix.
- **Bindings:** the regen · the bindings close (new).
- **Engine and pre-push:** the engine cycle on 24317/24318 · `pre-push:linux` · its teardown probe.
- **Operator entries:** hygiene · the merge-base probe · push · `ci.py conclusion --wait 2700`.
- **Not listed, each with its reason in the plan's prose:** no window boot smoke, since no boot-smoke trigger path
  changes. No webview gates. No deny/audit, since no manifest or lockfile changes.

## Baselines and controls (check 4 (9))
| entry | reading |
|---|---|
| 3 scope guard | green, no output |
| 4 unit vehicle | red, exit 94: no binary `unit_door_bridge` yet |
| 5 relay grep | red, exit 2: no file yet |
| 16 bindings close | green |

The other 15 entries were seen green in the previous chunk's wrap. Entries 3, 5 and 16 each had a must-fail and a
must-pass control in a scratch clone at `d3e694d2`, and all discriminate (`p5-controls.md`).

## What `planlint` check 4 listed (directive item 9)
| check | listed |
|---|---|
| 4 (5) | 0 `leg = 'live'` entries · 2 leg entries with a content atom · 0 without `atom from` |
| 4 (13) | 1 operator CI read · 0 not `ci.py conclusion` |
| 4 (14) | 0 base-less git change reads outside the operator legs |
| 4 (15) | 0 grep→wc counts asserting `exit 0` + `last line 0` |
| 4 (16) | 0 of 1 guard with exclusions leaves a new file in |
| 4 (17) | 0 of 19 asserting entries read a path the wrap writes |

Also check 2: 0 ledger-note entries · check 5: 0 placeholders · check 9: 1 CI read, operator-exempt. Hits: 0.

## P-092 against its acceptance — no claim (directive item 9)
- **Acceptance (ledger, as it stands):** "From the first step in which a console engine exists, every reading from
  outside its process goes through a door inside that process: the engine's own end-to-end gate and the external
  harness at each theme read through it, no stdio sidecar exists, and no later removal has a sidecar to change; an
  agent on the developer's machine reads through it the telemetry the engine holds, filtered by service, time range,
  trace and text, with aggregates, one incident whole with its evidence and the incidents like it from the past, and
  the curated snapshot of recent telemetry; the test harness and the status command use the same door."
- **What this chunk makes true:** an agent on the developer's machine speaking stdio reads through the door, from the
  telemetry the engine holds, with no sidecar in its path.
- **What stays open, with its owners on the working route:**
  - "no stdio sidecar exists": `Stdio sidecar retired`, line 41;
  - the engine's end-to-end gate and the external harness through the door: lines 43 and 49, and each theme's check;
  - the filters and aggregates: `Door telemetry queries`, 132;
  - one incident whole: `One incident whole through the door`, 134;
  - the snapshot: `Curated snapshot through the door`, 136;
  - the status command: 171.
- **Proposed dated note**, written by `matrix.py note` after the word:
  "2026-10-12 (2026-10-12-stdio-agent-reaches-the-door, phase P5): advanced, not claimed. This chunk's plan gives an
  agent on the developer's machine that speaks stdio a way to the door: `andromeda-pulse-engine door`, a relay with
  one connection per request, no sidecar in its path (the operator's P4 answer, PROVISIONAL, the pc overseer,
  2026-10-12). It cannot prove the acceptance: the stdio sidecar still exists until `Stdio sidecar retired`; no
  harness verb or status command reads through the door; the filters and aggregates, the whole incident and the
  snapshot belong to other route entries."

## What the stdio agent and the sidecar each reach (directive item 4)
| | through `andromeda-pulse-engine door` | the sidecar `andromeda-pulse-mcp` |
|---|---|---|
| Runs when | a console engine holds the door on that data dir (Unix) | `--features mcp-server` + `ANDROMEDA_PULSE_MCP_ENABLED=true`; no engine needed |
| Telemetry tools | the engine's live buffer | its own empty in-memory DuckDB |
| Incident tools | the engine's corpus writer and key, in process | the corpus via the OS credential store, the key from `run/workspace-key` |
| `mark_incident_resolved` | `-32001` unless `ANDROMEDA_PULSE_DOOR_RESOLVE` is on | applied under the double gate alone |
| Undeclared argument member | `-32602` | read past |
| Who is answered | the engine's own account | whatever started it |
| Bounds | the door's | none of the door's |

Both carry the same nine tools over the same envelope. Nothing is reachable through one and not the other except the
sidecar's empty telemetry store, its unswitched write and its looser arguments, and those leave with the sidecar.

## Acts that may meet the auto-mode classifier (directive item 6)
- **At implement:** an edit of `.github/workflows/ci.yml`. The `lint-test` step `Door refuses another account` gets
  a two-arm selection and one more required-sentence check, nothing else in the workflow. It is a CI-job edit, not a
  cut.
- **At implement, by the tools:**
  - `cargo xtask pre-push:linux` removes and recreates `target/pre-push/run/`;
  - `harness:engine-cycle` starts an engine on 24317/24318.

  No entry binds 4317/4318.
- **Not on this host:** `sudo` runs nothing here. `sudo -n true` fails, so both other-account arms print their skip
  sentences; they run as `nobody` and root on the runner only.
- **Operator pass:** `git add -A`, the pre-CI commit, the push of `build/andromeda-pulse-0.4.0` after the merge-base
  probe.
- **No directory delete is planned in the tree.** The P5 scratch clone sits in the session scratchpad, outside the
  project.

## Expected amendments (wrap)
Fourteen sites across architecture (5), security-plan (3), test-plan (3) and obs-plan (3), all under the pattern Boundary widening,
answered by the pc overseer on 2026-10-12 (inputs#I3). They are PROVISIONAL under the door's one item; the list is in
the plan's Implementation notes. The founder did not answer this widening himself at P4, so there is no answer card.
The wrap halts on these amendments in the ordinary form, in full: each proposal, why it escalated, and the
recommendation. The operator's answer there is PROVISIONAL too (re-worded at plan.md:448-450 on the operator's review
of card 1; the mechanical set re-ran green: fence 23 entries, planlint 0 hits, lists parsed, baseline 15 seen green +
4 new, 326 prose lines).

## Beyond the plan
| row | proposal |
|---|---|
| 1 | Owner of the `door` word's off-Unix branch (fixed line, exit 1): the existing entry `Other operating systems retired from the code`. |
| 2 | Owner of the never-written "Using MCP with andromeda-pulse" guidance that security-plan §Logging & Monitoring names: the existing entry `Records say what the product is`. A stdio agent through the relay now receives the same unsanitized tool answers it warns of. Research found no such document in the tree (`git grep -l -i 'Using MCP with andromeda-pulse'`: hits only in 2026-05-02 security run files). |

Apply? (yes / review / cancel)
