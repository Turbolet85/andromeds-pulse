# Route-resolve card — 2026-10-10-agent-harness-drives-the-console-engine (wrap, second window)

Nothing below is written to the route yet. The frozen line of this chunk is not edited here: its five carries are
stripped by the flip-compaction at P7, which archives the line verbatim first.

## A. The second entry (inputs#I4 item 2) — minted directly after this chunk's line, before `Shared telemetry test data`

Proposed line (title and scope: 24 words after the dash; no id, since it claims nothing):

    Engine harness proofs completed — rejected port, socket census, lock-file place read on a spawned engine; pre-push reads a real engine cycle; cleanup, boot, logs verbs pinned in shell (no capability claimed)

Its annotations, each a `CARRY:` block:

1. Minted at this chunk's wrap on the operator's word (the P4 answer that cut the freight in two; the wrap
   directive, item 2). It discharges three test-plan §1 pending rows —
   `engine-boot-rejected-port-and-socket-census-coverage`, `harness-cleanup-verdict-and-boot-spawn-shell-coverage`,
   `harness-log-family-resolution-coverage` — and claims nothing in the ledger: P-086's open half is carried by the
   four entries that name P-086.
2. Moved from the completed line (C5): the three readings of the engine boot — the rejected-port arm of
   `engine_boot::start`, a socket census on a spawned run, the corpus-key lock file's directory.
3. Moved from the completed line (C2, the half that was cut): the sixth stage of `cargo xtask pre-push:linux` still
   runs `ci-gates` over a seed it wrote itself; its doc comment was corrected at this chunk; the stage is read over a
   real engine cycle here.
4. The shell-level pins: `cleanup`'s four tokens, `boot`'s failure arms, the `logs` family branch, and the word arms
   of `boot` / `status` (none, `engine`, any other) that shipped driven by hand.
5. **Proposed, beyond the directive's three rows (my lean: yes):** it also takes the row opened at this wrap,
   `engine-log-arm-mutation-and-engine-cycle-glue-coverage` — one mutation per arm of `check:engine-log`, a pin for
   each of the four refusal labels no test names. (Found and not owned 4.)

## B. The five carries on the completed line (inputs#I4 item 3)

| Carry | Disposition |
|---|---|
| The heartbeat-gap check and "a run that produces zero spans fails the build" (C1) | **struck — built**: the `heartbeat-gap`, `family` and `progress` arms of `check:engine-log`, run by the `boot` job's cycle step; obs-plan §10 amended. What stays true and is stated there: no CI step grades the window's ticks; the test-run form is made by no step |
| The pre-push sixth stage and its stale doc comment (C2) | comment **struck — built**; the stage's re-aim **moved** to the second entry (A.3) |
| `check:ingest-progress` reads NEUTRAL over a log with no tick (C3) | **struck — built** as the `progress` arm; the standard entry's NEUTRAL line is named in obs-plan §3 and §10 |
| The console program's panic and at-exit witness; the harness verbs not aimed at it (C4) | first half **retired, master amended**: test-plan §1 `exit-hook-main-composition-coverage` says where the witness is made and why no trigger is built; second half **struck — built** (`boot engine`, `status engine`) |
| The three readings of the engine boot (C5) | **moved** to the second entry (A.2) |

## C. A home for the engine cycle when the boot job leaves (inputs#I4 item 4) — applied as directed

- `Window's gates retired` gains: the boot job it removes holds two steps that do NOT leave with it, `Console engine
  cycle` and `Upload engine logs artifact` (with four pins in `quality_gate_workflow.rs`); they move to the engine's
  own CI job. Added as a fact of the same carry: the cycle verb's exit-witness parts (its pass-through of the witness
  variable, the `witness_file` member, the gate entry's witness build) leave with the witness, or are kept by that
  entry's card.
- `Engine end-to-end gate reachable` gains: its CI job takes the cycle step and the `logs-engine-Linux` upload.

## D. Found and not owned — each with a lean (inputs#I4 item 7)

1. **The cycle's cleared child environment rebuilt on the runner** (`pulse-app` in release 44.61 s, 14 crates in dev
   11.51 s, the injector 25.20 s; the step 100 s against 19 to 41 s here; one reading; cause not measured). Lean: a
   `CARRY:` on `Engine end-to-end gate reachable`, the cause written as a hypothesis, that entry ruling whether the
   build-affecting part of the job's environment joins the pinned child set.
2. **`logs-engine-Linux` carries `build.log` and an empty `boot.log` beside the log family; `build.log` holds the
   runner's checkout paths.** The operator's reading at P4 named "the engine's own log family". This is the one open
   escalation of this wrap's reconcile (D-obs-pii). **The operator's word is needed.** Lean: the class covers them —
   the boot job's upload has held the same two files under the same class since 2026-10-09, they hold cargo's and
   the boot verb's own text and no telemetry. If not: the upload path is narrowed to the log family, a `CARRY:` on
   the second entry.
3. **A red cycle has not been seen on a runner** (two red steps for one cause when a cycle is refused before its
   boot). Lean: accept; recorded in the new test-plan row.
4. **No mutation check against the written check.** Lean: the second entry (A.5).
5. **`wrong-program` in the other direction was not driven live.** Lean: accept; recorded in the widened shell row.
6. **Cycle data dirs accumulate under `target/engine-cycle/`.** Lean: a handoff line; the operator's to delete.
7. **`agent-run.ps1` takes no `engine` word.** Lean: a `CARRY:` on `Other operating systems retired from the code`.
8. **`check:ingest-progress` still prints NEUTRAL** as a standard gate entry. Lean: none owed (B, C3).

Found at this window:

9. **The exit-witness variable has a second reader**: `harness:engine-cycle` passes it on by value so that a cycle
   can show the engine's spawn carries no preload. Named under the PROVISIONAL item in security-plan with no
   classification of its own. Lean: a handoff line under the founder's PROVISIONAL item; nothing on the route.
10. **Ten places where a master is behind the code or another master** (listed in `cascade-dispositions.md`, last
    section: four LANDED marks missing in test-plan §1, the status-endpoint shape, the boot data-dir name,
    `opentelemetry-stdout` in architecture, "twelve crates", xtask `release` / `sign` / `notarize` verbs that do not
    exist, three routers credited to their crates, the retention SQL, the directory-structure key file, resize
    "deferred"). None is this chunk's; the report carries none, so none was amended. An in-version follow-up with no
    entry named for it — the four dispositions: **pin** one `CARRY:` on `Records say what the product is` (Epoch 3,
    which rewrites the project record and the architecture intent) · mint an entry · residuals.md · drop. Lean: pin.
11. **Leaf statements no master states (about 12) and 28 file-layout lines in the service notes that the tree
    contradicts** were left as they are and listed in `cascade-dispositions.md`. Lean: leave; the layouts are not in
    any master, and most of those crates leave in Epochs 2 and 3. Say if they should be corrected from a source read.

## E. For the operator, not a route edit

- **How the leaf recompute was made** (item 6): eight read-only comparers read master and leaf whole; this session
  verified and wrote every edit; 75 of about 150 finding blocks were applied by a unique-anchor replace script after
  a dry run, the rest with the Edit tool. That departs from the cascade's letter (main derives; Edit tool). The
  returns, the apply record and the dispositions are in the run dir.
- **Item 9:** no `REFUSED id:` in any trail of either window; the citation sweep read `held 0` (4 re-pointed, 0
  changed, 0 stretched; five leaf rows in preserve-verbatim homes, surfaced).
- **Epoch 1** holds 18 entries and 19 with the mint (frozen complete 10, pending 1). No split is proposed.
- **No playbook rule is proposed.** The one no-match (D.2) is a classification, the operator's each time.
- **P-086** is not claimed; its dated note is written at P7.3 and names the second entry by the title approved here.
