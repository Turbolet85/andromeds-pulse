# Report — 2026-10-10-boot-smoke-s-self-end-named-from-a-run

**Chunk:** Boot smoke's self-end named from a run: the job keeps what names who ended the app; cause closed; equal source reads the same
**Date:** 2026-10-10
**Commits:** `925be35f chore(2026-10-10-boot-smoke-s-self-end-named-from-a-run): operator pre-CI commit` (the one commit since `8936f976`; basis `git log --format='%h %s' 8936f976..HEAD`)

This plan names and does not close (plan.md Goal; inputs#I3). It built the instrument, read its runs and stopped at
the first witnessed self-end. P-129 is not claimed.

## Changes (structured — detectors read this)
- **Files:** 8, all in research's two lists (`gate.py scope`: `clean — changed 8 · listed 8 · recorded 0`).
  New: `scripts/exit-witness.c` (162 lines) · `xtask/src/harness_witness.rs` (599) · `xtask/src/harness_series.rs`
  (756). Modified: `scripts/agent-run.sh` (22 lines added: 44-56, 90, 92-99) · `xtask/src/harness_ready.rs` (11
  added) · `xtask/src/main.rs` (12 added) · `.github/workflows/ci.yml` (16 added: 375-382, 400-407) ·
  `pulse-app/tests/quality_gate_workflow.rs` (95 added: 564-658). No file under `pulse-app/src/`, `pulse-app/ui/`,
  `crates/`, no manifest, no lockfile, no capability file (the plan's scope-guard entry, green).
- **Symbols / APIs:**
  - **A new xtask verb, `cargo xtask harness:boot-series --count N`** (`Cmd::HarnessBootSeries`,
    `xtask/src/main.rs:72-75`, dispatched at `:285`; `harness_series::run`, `xtask/src/harness_series.rs:70-94`).
    It boots the release app N more times, ordinals 2 to N+1, the CI smoke being ordinal 1. Each boot runs one
    cycle (`boot`, `harness:settled`, `status`, `cleanup`; after a failed `boot` the two middle verbs are skipped,
    `cleanup` always runs) under its own `xvfb-run`, on its own data dir `series/boot-{ordinal}/` under the
    resolved data dir, with `XDG_DATA_HOME` and `XDG_CACHE_HOME` set to empty directories inside it. One
    pretty-JSON verdict on stdout, also written to `logs/boot-series.json`: `{verdict, boots, settled, ended,
    other, per_boot}`, each `per_boot` entry `{ordinal, cycle, verdict, ended, app_exit_record, windows_settled,
    exit_witness}`. Verdicts and exits: `all-settled` 0 · `self-ended` 1 (a boot's settle verdict read `ended`) ·
    `not-all-settled` 1 · `cannot-evaluate` 2 (a count outside 1 to 16, not Linux, no data dir or one that already
    holds `series/`, no `xvfb-run` on `PATH`, no boot run, or a cycle labelled `cleanup-not-clean`, `timed-out` or
    `no-record`, after which the series stops). `cycle` is a closed label: `complete` · `boot-failed` ·
    `status-not-healthy` · `cleanup-not-clean` · `timed-out` · `no-record`, and `smoke` for ordinal 1, which is
    listed from the smoke's own `logs/harness-settled.json` when the data dir holds one and stays out of the
    counts. A cycle is bounded at 1200 s. The verb binds no port itself; each boot binds the two OTLP ports the
    environment resolves, one boot at a time. It adds no TauRPC procedure, no IPC route and no capability.
  - **The settle verdict gained an eighth member, `exit_witness`** (`settled_payload`, seventh parameter,
    `xtask/src/harness_ready.rs:359, 369`; read in `run_settled` at `:112-114`). Its two callers are unchanged in
    number: `run_settled` and the pin `settled_payload_carries_the_closed_field_set` (research's graph read). The
    member is one closed label from `harness_witness::label` (`xtask/src/harness_witness.rs:40-55`): `unset` (no
    file) · `unreadable` · `loaded` · `exit-call` · `runtime-exit` · `no-record`. The verdict set, the exit codes
    and the other seven members are unchanged. `logs/harness-settled.json` holds the same eight.
  - **The boot verb's witness arm, sh only** (`scripts/agent-run.sh:44-56, 90, 92-99`). It reads
    `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`, trimmed. Unset or blank: the app is spawned as before. A regular file: the
    one spawn command runs with `LD_PRELOAD` set to it and `ANDROMEDA_PULSE_EXIT_WITNESS_FILE` set to
    `{data dir}/logs/exit-witness.jsonl`; neither is exported, so the pre-build and every other verb run without
    them. Set and not a regular file: `boot: exit witness library not found` on stderr, exit 1, before the
    pre-build. The verb also removes a stale `logs/exit-witness.jsonl` of the same data dir before the spawn.
    `scripts/agent-run.ps1` is unchanged (plan step 3). The 5-command set is unchanged: no sixth verb.
  - **The library, `scripts/exit-witness.c`** (C, no dependency beyond the C library; built by
    `cc -shared -fPIC -O2`). It interposes `exit`, `_exit`, `_Exit`, `quick_exit`, `abort` (`:124-162`). At load
    (`witness_init`, `:63-92`) it opens the file named by `ANDROMEDA_PULSE_EXIT_WITNESS_FILE` for append,
    close-on-exec, removes `LD_PRELOAD` and that variable from the process environment, writes a `loaded` line and
    registers an exit handler. With the variable unset it does nothing; with the file not openable it removes the
    two variables and stays inert.
  - **Environment variables.** New, harness-only, never read by `pulse-app`'s own code
    (`grep -rn 'EXIT_WITNESS\|XDG_DATA_HOME\|XDG_CACHE_HOME\|LD_PRELOAD' pulse-app/src crates --include=*.rs`: 0
    lines): `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` (read by `agent-run.sh boot`; in CI set through `$GITHUB_ENV`) ·
    `ANDROMEDA_PULSE_EXIT_WITNESS_FILE` (set by the boot verb on the spawn line, read by the library). System
    variables: `LD_PRELOAD` set on the app's spawn line by the boot verb · `XDG_DATA_HOME` and `XDG_CACHE_HOME` set
    by the series on each of its boots (the app's webview state then lands there) · `PATH` read by the series by
    value, only to find `xvfb-run`, never printed · `ANDROMEDA_PULSE_PIDFILE` and `ANDROMEDA_PULSE_LOGFILE` removed
    from each series boot's environment, so each resolves under its own data dir.
  - **Filesystem locations**, all harness-written, none read by the product: `logs/exit-witness.jsonl` ·
    `logs/boot-series.json` · `logs/series/boot-{ordinal}/` (copies of that boot's log family, `boot.log`,
    `harness-settled.json`, `xvfb.log`, `exit-witness.jsonl`; `is_kept`, `xtask/src/harness_series.rs:308-310`) ·
    the series' data dirs `series/boot-{ordinal}/` with `logs`, `run`, `xdg-data`, `xdg-cache` and the product's
    own `corpus` (not under `logs/`, not uploaded). Source: `scripts/exit-witness.c`. Local build output:
    `target/exit-witness/exit-witness.so` (ignored); in CI `$RUNNER_TEMP/exit-witness.so`.
- **Crates / modules:** no crate added or removed. `xtask` gains two modules, `harness_witness` and
  `harness_series` (`xtask/src/main.rs:14, 16`).
- **Dependencies:** none added, none bumped (`Cargo.toml` and `Cargo.lock` are outside the diff).
- **Schema / config:** no migration, no config key, no scrub or redaction shape. Three harness-written shapes:
  - `logs/exit-witness.jsonl`, one JSON object per line, each written by one `write` of at most 4096 bytes:
    `{"kind":"loaded","pid","comm"}` · `{"kind":"end","pid","tid","comm","call","code","errno","frames":[{"m","s","o"}]}`
    (`m` a module basename, `s` an exported symbol or empty, `o` a hex offset; frame 0 is the interposed call; at
    most 23 frames; `abort` is recorded with code −1) · `{"kind":"runtime-exit","pid"}`. Names are reduced to
    printable ASCII without `"` and `\`. The reader (`xtask/src/harness_witness.rs:88-135`) takes at most 64 lines
    of at most 4096 printable-ASCII bytes with one of those three kinds and a `u32` pid; anything else reads
    `unreadable`, never a partial record.
  - `logs/harness-settled.json`: eight members (above).
  - `logs/boot-series.json`: the series verdict (above); its per-boot values are reduced to labels of at most 48
    printable-ASCII bytes and a count (`bounded_label`, `boot_entry`, `xtask/src/harness_series.rs:334-359`).
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - `harness:settled`'s object: seven members → eight (stated as a member list at architecture §Occupied
    Resources → xtask CLI surfaces, `architecture.md:252`; `.claude/rules/verification-harness.md` restates it).
  - `ci.yml` steps: 98 → 100 `- name:` lines, 51 → 53 `run:` lines (`grep -c` on `git show 8936f976:…` and on the
    tree). The handoff's carried line "`ci_workflow_test_gates_no_continue_on_error` reads 16 of `ci.yml`'s 51 run
    steps" now reads against 53; the 16 was not re-derived.
  - xtask verbs named `harness:*`: three → four (`grep -c '"harness:' xtask/src/main.rs`: 4).
  - The workspace suite: `2876 tests run` at the last gate call; 40 of them are this chunk's (37 in the two new
    modules, 3 workflow pins). The count before the chunk was not measured.
  - The boot-smoke readings: thirteen pull-request runs of the build branch since `b3ac58a`, five red (the twelve
    and four of research.md, plus `ci#38019133294`).
- **Dev-tool versions:** none installed or changed. `cc` was read on the dev host at `GCC 16.2.1 20260810`
  (`cc --version`, 2026-10-10); the runner's compiler built the library and the controls in `ci#38019133294` and
  its version was not read from that run. `xvfb-run`, `mise`, Node 24 re-read present, unchanged.
- **Harness / gate surface:**
  - **CI, the `boot` job, two added steps and nothing else** (the plan's parsed-workflow probe: `['boot'] ['Boot
    series (equal source)', 'Build the exit witness'] True True True True`; six jobs; `6 of 6` on `ubuntu-22.04`;
    the added-line grep for action references, permissions, secrets, soft gates, job edges, triggers: `0`).
    `Build the exit witness` (`ci.yml:375-382`), after the release build and before the smoke: `cc` builds
    `scripts/exit-witness.c` into `$RUNNER_TEMP` and names it through `$GITHUB_ENV` as
    `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`. `Boot series (equal source)` (`ci.yml:400-407`), after the smoke and
    before `ci-gates`, `if: always()`, a plain `run: cargo xtask harness:boot-series --count 7` with no soft-fail
    key. So one run of the job reads eight boots, and a boot whose settle verdict reads `ended` fails the job. The
    smoke step's lines are unchanged. `ci-gates` is skipped when the smoke or the series fails; the upload still
    runs.
  - **What the job's artifact (`logs-boot-Linux`, the whole `logs/` dir) now holds beyond before:**
    `exit-witness.jsonl`, `boot-series.json`, `series/boot-{2..8}/` with five files each (four where a boot took
    no settle verdict). It held `agent-latest.jsonl*`, `boot.log`, `build.log`, `harness-settled.json`, `xvfb.log`
    before and still does.
  - **Three workflow pins** (`pulse-app/tests/quality_gate_workflow.rs:587-657`): the library is built from
    `scripts/exit-witness.c` before the smoke and named through `$GITHUB_ENV`, with no `LD_PRELOAD` in the step ·
    the series sits after the smoke and before `ci-gates`, under `if: always()`, as the plain step · the series
    step holds no `continue-on-error`, `retry` or `|| true`. The four smoke pins are untouched.
  - **xtask tests:** `harness_witness::tests` (14 reader and decision pins, `:169-381`, and seven controls on the
    built library, `mod built_library`, `:383-598`, Linux only, a missing `cc` fails them) ·
    `harness_series::tests` (16 pins, `:432-755`) · the member pin in `harness_ready` moved to eight.
  - **The measured defect of the series verdict** (see Insufficient fixes): a boot that ends before the boot verb
    reads ready takes no settle verdict, so it is counted `other` with a null `exit_witness`.
- **Cross-project / external claims:**
  - **`ci#38019133294`, attempt 1, on the pushed tip `925be35f` (the merge commit it built: `ab6a1ae6ef0d…`, read
    from every boot's log): red.** Its one failed job is `boot smoke (ubuntu-22.04)` (job `114115991438`); `a11y`,
    `lint / test`, `mcp-server tests`, `supply-chain` and `coverage gate` succeeded. It is the chunk's CI verdict
    and was not re-run; no re-run of the `boot` job was fired (the pass stopped at a self-end). Artifact
    `logs-boot-Linux` `11657059728`, 34178 B, expires 2026-10-24, downloaded to
    `target/boot-smoke/ci-38019133294-attempt-1/`. Record: `evidence/operator-pass.md`.
  - **What that run's eight boots read** (the table in `evidence/operator-pass.md`): seven ended by themselves,
    one settled (ordinal 6, four windows, `exit_witness` `loaded`). Ordinals 1, 2 and 8 ended after `boot: ready`:
    settle verdict `ended`, `exit 1`, `app_exit_record` `absent`, `windows_settled` 0, `display` and `session_bus`
    `reachable`, `exit_witness` `exit-call`. Ordinals 3, 4, 5 and 7 ended before ready (`boot: failed to reach
    ready state within 10s`, `app ended: exit 1`). All eight `xvfb.log` are 0 B; every `boot.log` holds the one
    accessibility-bus warning; no log holds `app.exit` on an ended boot or `app.panic.fatal` on any.
  - **The witness lines.** Eight files, 15 lines (eight `loaded`, seven `end`), each read whole against its shape:
    0 lines hold anything beyond it. The seven `end` lines are equal in every member but `pid` and `tid`, and in
    each `tid` equals `pid` (the main thread): `"call":"_exit","code":1,"errno":11`, 23 frames — frame 0
    `exit-witness.so` `_exit`; then `libgdk-3.so.0` +0x76ccc (a static function); `libX11.so.6` `_XIOError`,
    `_XReply`, `XGetWindowProperty`; `libgdk-3.so.0` +0x781f9, +0x7830e, `gdk_x11_screen_supports_net_wm_hint`,
    +0x7fb4f; `libgobject-2.0.so.0` `g_signal_emit_valist`, `g_signal_emit`; `libgdk-3.so.0` +0x46966, +0x332ad;
    `libglib-2.0.so.0` +0x56318, `g_main_context_dispatch`, +0xab5f8, `g_main_context_iteration`;
    `libgtk-3.so.0` `gtk_main_iteration_do`; five frames of `pulse-app` with equal offsets in all seven lines. The
    ground truth of those libraries is the runner image's, not this repository's; research read the GTK branch
    head's `gdk_x_io_error`, which calls `_exit(1)`, and did not read the runner's build.
  - **Limits of that reading, each a limit** (inputs#I4 item 2): the lines do not say why the X connection's read
    failed · the instrument's reading on green boots on the runner is one boot · seven of eight boots ended here
    against four of twelve first boots in the runs before it, and whether the library, seven boots in a row on
    one runner, or this runner accounts for that is not measured · boot 2 took 3 min 27 s from the series step's
    start to its readiness line against 3 to 12 s for the six after it, and its build log is not kept, so whether
    boots 2 to 8 ran the binary boot 1 ran is not shown beyond the equal offsets.
  - **The dev-host control** (the plan's `-witness-display-lost` leg, both gate calls): the same call, code and
    handler (`_exit`, code 1, `errno` 11, `libgdk-3.so.0` under `_XIOError`), written by a second thread through
    `XNextEvent`; this host's library set (`evidence/local-legs.md`).
  - **Inputs** (`inputs.py verify`: `4 entries — unchanged 2 · drifted 0 · vanished 0 · broken 0 · n/a 2`):
    I1 · the pc overseer's phase relay · copy · unchanged, cited. I2 · the phase invocation message · copy · n/a,
    cited. I3 · the operator's three P4 rulings · copy · n/a, cited. **I4 · the pc overseer's wrap relay
    (`../additional/pc-overseer/relays/pulse-wrap-boot-smoke-self-end-named-2026-10-10.md`) · copy · unchanged**,
    snapped at this wrap and cited here as inputs#I4.
  - **A read outside the repository that is not an input:** /implement read the phase session's scratch probe
    (the measured source of the line format) and `inputs.py snap` then refused it (`a scratch file is no input`).
    The in-tree `p3-measurements.md` carries the same line format.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** the series verdict. It was written so that one run
  records each boot with its settle verdict and its witness label and reds on a self-end. It does that for a boot
  that reaches ready (ordinals 1, 2, 6, 8 of `ci#38019133294`). A boot that ends before ready runs no
  `harness:settled`, so four of that run's seven self-ends carry `verdict` null and `exit_witness` null, and
  `boot-series.json` reads `ended` 2, `other` 4. The step still failed and each witness file is kept with its
  line. Owner of the remainder: the closing entry minted at this wrap, as the first thing it repairs (inputs#I4
  item 4).
- **Spec claims disproved by measurement:**
  - plan.md, Implementation notes: "The series adds about a minute to the `boot` job". Measured on
    `ci#38019133294`: the series step ran 4 min 27 s (03:12:56Z to 03:17:23Z, the job log's step boundaries). A
    chunk-artifact claim: recorded here, no edit.
  - plan.md, Test Commands, the operator pass's step 6 and the acceptance "(obs) On the runner: attempt 1's eight
    boots each carry an `exit_witness` label": four of the eight carry none (the defect above). Routed in Outcome.
  - No spec-master assertion was measured false. obs-plan §7 Error classes captured → Process-end cause lists
    `_exit` among the ends that cannot be logged (`obs-plan.md:396`); the measured end is that one.
- **Expected amendments (from plan):** each site search is `grep -n` over the seven masters and every file under
  `.andromeda/registries/`.
  - architecture §Occupied Resources → xtask CLI surfaces (the `harness:boot-series` row; `harness:settled`'s
    object gains `exit_witness`; the boot verb's witness arm, sh only; the smoke row's two neighbouring steps) —
    carried: Symbols / APIs, Harness / gate surface. Search `harness:settled`: architecture 5 lines (217, 242,
    243, 245, 252), security-plan 3, test-plan 3, obs-plan 3, `registries/contracts/test-plan/pid-file.md` 1;
    `boot-series`: 0 in all.
  - architecture §Occupied Resources → Environment variables (the two `ANDROMEDA_PULSE_EXIT_WITNESS_*` rows;
    `XDG_DATA_HOME`, `XDG_CACHE_HOME` on the series' boots) — carried: Symbols / APIs, Environment variables.
    Search `EXIT_WITNESS\|LD_PRELOAD\|XDG_DATA_HOME\|XDG_CACHE_HOME`: 0 in all masters and key files; the
    neighbouring harness-only rows are `architecture.md:242-246`.
  - architecture §Occupied Resources → Filesystem locations (`scripts/exit-witness.c`; the three kept paths and
    the series' data dirs) — carried: Symbols / APIs, Filesystem locations. Search `harness-settled`: architecture
    2 (217, 252), security-plan 2, test-plan 1, obs-plan 1; `exit-witness`: 0 in all.
  - architecture §Infrastructure Patterns → CI/CD approach (the `boot` job's two added steps and this chunk's
    readings) — carried: Harness / gate surface, Cross-project / external claims. The contract is keyed: its text
    is `registries/contracts/architecture/ci-cd-approach.md` (line 3 describes the six jobs; `grep -n -i boot`: 1
    line).
  - test-plan §9 Pipeline structure → Boot smoke (harness) and Build failure conditions (the witness, the series,
    a non-zero series failing the job, the kept files) — carried: Harness / gate surface. Search `Boot smoke`:
    test-plan 1 (`:496`); the failure-conditions bullet is `test-plan.md:509` (`harness:settled`, 3 lines: 134,
    496, 509).
  - test-plan §3 `boot` (the witness arm and its fail-closed read) and test-plan §1 Pending coverage triggers,
    `harness-cleanup-verdict-and-boot-spawn-shell-coverage` (the arm ships with no committed shell-level test) —
    carried: Symbols / APIs, Coverage. §3 `boot` is keyed: `registries/contracts/test-plan/5-command-implementation.md`
    (its `boot` Command body, line 4, and Exit code, line 6); the trigger: test-plan 1 line (`:134`) and that key
    file 3 lines.
  - obs-plan §9 Telemetry artifact handling → Log file row (what the artifact now holds) and obs-plan §7 Error
    classes captured → Process-end cause (what a run's witness showed of the end with no `app.exit`) — carried:
    Harness / gate surface, Cross-project / external claims. Search `logs-boot`: obs-plan 1 (`:499`), architecture
    1, security-plan 1, test-plan 1; `Process-end cause`: obs-plan 1 (`:396`).
  - security-plan §Security Anti-Patterns → Input, with its two restating sites (§Input Validation, the CLI / env
    var inputs row; §Threat Model Summary) — the harness-only class gains the library loaded on the spawn line,
    the two variables and the kept files, classified routine harness evidence, the app's exit record, by the
    operator, the pc overseer, 2026-10-10, the operator's reading and not the founder's word (inputs#I3) —
    carried: Symbols / APIs, Schema / config, Coverage. Search `XDG_RUNTIME_DIR` (the class's marker): security-plan
    6 lines (84, 85, 138, 167, 343, 395); `harness-settled`: security-plan 2 (85, 395).
- **Coverage of new surfaces:**
  - `cargo xtask harness:boot-series` → validation {a count outside 1 to 16, a non-Linux host, no data dir, an
    existing `series/`, no `xvfb-run`: `cannot-evaluate`✓} · instrumentation {n/a — a harness verb; its verdict
    JSON is its record} · PII {no environment value and no path in the verdict; per-boot values reduced to bounded
    labels✓} · tests {16 unit pins; one local leg (two boots, `all-settled`); one CI run (`self-ended`); the
    timed-out path never ran live} · a11y {n/a} · tokens {n/a}
  - `scripts/exit-witness.c` (the preloaded library) → validation {n/a — it takes one file path, set by the boot
    verb} · instrumentation {n/a} · PII {a closed line shape, names reduced to printable ASCII; 15 runner lines and
    the local legs' lines read whole, 0 beyond the shape✓} · tests {seven controls on the built library: `exit`
    through the linker, a direct `_exit`, `main` returning, a direct system call, the child boundary with its
    known positive, the inert arm; green on the dev host and in the runner's `lint / test` job} · a11y {n/a} ·
    tokens {n/a}
  - `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` at `agent-run.sh boot` → validation {trim, regular-file check, fail-closed
    exit 1✓} · instrumentation {n/a} · PII {the value is never printed✓} · tests {✗ no committed shell-level test;
    the set arm is driven by the three local legs and the CI job, the refusal arm was driven once by hand} · a11y
    {n/a} · tokens {n/a}
  - the witness reader and the `exit_witness` member (`harness:settled`) → validation {64 lines, 4096 bytes,
    printable ASCII, three kinds, a `u32` pid; else `unreadable`✓} · instrumentation {n/a} · PII {only the label
    leaves the module✓} · tests {14 unit pins, the member pin, the three legs} · a11y {n/a} · tokens {n/a}
  - the two `boot` job steps → validation {n/a} · instrumentation {n/a} · PII {n/a} · tests {three workflow pins
    and the plan's four workflow probes} · a11y {the `a11y` job is untouched; its verdict on attempt 1: success} ·
    tokens {n/a}

## Deviations from intent
The implement report's deviations, accepted by the operator as recorded ("Deviations accepted as recorded", the
operator, 2026-10-10, in this session):
- **The witness decision, an arm the plan does not name.** A file with no `loaded` line for the app's pid, or no
  pid at all, reads `unreadable`.
- **The series, values the plan does not fix.** The `cycle` label set; a boot counts settled only on a `complete`
  cycle; after a failed boot `harness:settled` and `status` are skipped and `cleanup` still runs; a data dir that
  already holds `series/` reads `cannot-evaluate`; a cycle is bounded at 1200 s; per-boot values are reduced to
  bounded labels and a count; each cycle's own output goes to stderr so stdout is the one JSON object. The second
  of these is what the runner then showed as the undercount.
- **The library, details beyond the plan.** `abort` is recorded with code −1; names are reduced to printable
  ASCII; the two variables are removed even when the file cannot be opened; `_exit` and `_Exit` call the real
  function through the linker where the research probe used the system call.
- **The boot verb.** The missing-library check sits before the pre-build; a stale witness file of the same data
  dir is removed before the spawn.
- **One control beyond the plan's five:** the library is inert without its file and the code passes through.
- **Step 1's red reading needed stub bodies**, and the member pin's call of `settled_payload` gained its seventh
  argument between the red and the green reading (`evidence/red-before-green.md`).
- **The first gate call read two reds with one cause outside the chunk's edits**: step 1's suite run had left the
  generated bindings in the default-features shape before the block (`evidence/local-legs.md`).
- **The operator pass:** the three reads (the jobs, the job log, the artifact) were made after the run closed, 7
  minutes after the CI read returned at its first failure; the job log was read a second time for its step
  boundaries only. Entries after the artifact read were not fired (the stop).

scope record: none — `gate.py scope` clean, 0 recorded.

## Decisions & corrections
- **The operator's word after the implement report** (2026-10-10, here): deviations accepted as recorded; run the
  operator pass in the plan order; stop at the first witnessed self-end, report its ordinal, label and witness
  lines whole; read every kept witness file whole and stop on anything beyond its shape; fix nothing on top;
  start no skill.
- **The wrap relay (inputs#I4), the operator, the pc overseer, 2026-10-10:** the pass stands as it stopped; the
  red is the chunk's subject and the closing entry owns it; P-129 is not claimed and gets a dated note saying
  which clause this run showed and for which boots; the closing entry is minted first in the tail with its
  measured freight, three research questions and the series undercount as its first repair; a carry on `Window's
  gates retired` names what leaves with the boot job; until the cause is closed the boot job reads red on most
  runs, and that red is read by its per-boot verdicts and is the closing entry's, never a later chunk's to fix;
  a proposal a detector grades `escalate` halts at the card for the operator's word; two stops (the
  route-resolve card; before the flip and the commit).
- **Sweep hazards and traps found this chunk:**
  - A plan step that runs the default-features workspace suite before the gate block leaves the tracked bindings
    rewritten, and the block's scope guard and `capability-drift` then read red on the first pass.
  - A job-log line copied into evidence carries the runner's own temp path, which the hygiene read takes for a
    host path; it was written as `{runner temp}`.
  - A series built on "the smoke's cycle" inherits the smoke's `boot || exit`: a self-end before ready leaves no
    settle verdict.
  - `inputs.py snap` refuses a file under the temp dir: a research instrument kept only in a session scratchpad
    cannot be recorded as an input by the next skill.

## Outcome
**Acceptance criteria, each against the diff and the runs.**
- (obs) A self-ended boot keeps a machine-readable record naming the ending call and its caller — **met**: on the
  dev host by the control leg, and on the runner by seven `end` lines.
- (operator, I3 condition 1) Each way of ending with a code has a known-positive control in the workspace suite —
  **met**, on the dev host and in the runner's `lint / test` job.
- (operator, I3 condition 2) The library stays out of the processes the app starts — **met**: the child control
  and its known positive; the green leg (one `loaded` line with the webview's children running); each of the
  runner's eight files holds lines under the app's pid alone.
- (tests) The settle verdict's exit codes and verdict set unchanged, eight members pinned; `harness:boot-series`
  under the 0 / 1 / 2 contract; every decision a pure function with a pin per arm — **met**.
- (operator, I3) The series records each boot with its ordinal, settle verdict, exit record and witness label; a
  boot that ends by itself fails the step — **UNMET in part.** The step failed. For the four boots that ended
  before ready the verdict records the ordinal and `cycle` `boot-failed` and no settle verdict, exit record or
  label. An unlinked criterion; its owner is named by the operator: the closing entry, first repair (inputs#I4
  item 4).
- (arch) `ci.yml`: two triggers, six jobs on `ubuntu-22.04`, no matrix, no job edge, `permissions: contents:
  read`, a `boot` job differing by two added steps; the four smoke pins pass unchanged — **met**.
- (security) No action reference, permission, secret reference, trigger or soft-fail key added — **met**.
- (security) Every newly kept file is harness-written under `logs/`, never opened by `pulse-app`; each kept
  witness file of each attempt read whole with 0 lines beyond its shape — **met** for the one attempt (eight
  files, 15 lines).
- (arch) No variable this chunk adds is read by `pulse-app`'s own code — **met** (the grep above, 0 lines).
- (a11y) The `a11y` job unchanged, no artifact name added or renamed, its verdict on attempt 1 recorded — **met**
  (`success`).
- (tests) No retry, soft-fail key or wait in the boot job; no CI run re-run for a green; re-runs recorded by
  attempt — **met**; no re-run was fired.
- (obs) On the runner, attempt 1's eight boots each carry an `exit_witness` label and every settled boot reads
  `loaded` — **UNMET in part**: four boots carry no label (the same defect, the same owner); the one boot that
  settled reads `loaded`. The instrument's reading on green boots on the runner is therefore one boot. Each
  boot's instants are in `evidence/operator-pass.md`.
- (arch) Each attempt names the merge commit its log carries — **met**: `ab6a1ae6ef0dbf72574c0931885e42fcc7c94885`.
- (tests) Each local leg ends `cleanup: clean` on 14317 and 14318 — **met**.
- (tests) The standard gate set in order, the bindings close at exit 0, the pre-push check green before the push
  — **met**.
- **What the report states.** A self-end was witnessed in attempt 1, first at ordinal 1: label `exit-call`; the
  lines name `_exit(1)`, `errno` 11, called on the main thread from a static function of `libgdk-3.so.0` entered
  from Xlib's `_XIOError`, reached from `_XReply` under `XGetWindowProperty` under
  `gdk_x11_screen_supports_net_wm_hint`. Against obs-plan §7's partition this is an end that cannot be logged
  (`_exit`), which is why no `app.exit` record exists for it. The lines do not name why the connection's read
  failed. The first boot ended (1 of 1); of the seven later boots six ended and one settled.

**Gates** (the last full gate call of /implement, then the operator pass; `evidence/local-legs.md`,
`evidence/operator-pass.md`).
- `cargo fmt --check` — green · exit 0
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — green · exit 0
- `git diff --name-only 8936f976… -- crates pulse-app xtask scripts docs …` (the scope guard) — green · exit 0,
  no output
- the three `python -X utf8 -c "import yaml…"` workflow probes — green (`a11y boot coverage lint-test mcp-test
  supply-chain` · `6 of 6` · `['boot'] ['Boot series (equal source)', 'Build the exit witness'] True True True
  True`)
- `git diff 8936f976… -- .github/workflows/ci.yml | grep -c -E …` — green · exit 1, last line `0`
- `mkdir -p target/exit-witness && cc -shared -fPIC -O2 -o target/exit-witness/exit-witness.so
  scripts/exit-witness.c` — green · artifact fresh
- `cargo nextest run -p xtask --profile ci -E 'test(/harness_witness::|harness_series::/)'` — green · 37 passed
- the `-witness-green` leg — green · settled, `exit_witness` `loaded`, `loaded-lines 1`, `cleanup: clean`
- the `-witness-display-lost` leg — green by its atoms (no exit atom) · `ended`, `exit 1`, `exit-call`, `_exit`,
  code 1, `libgdk-3`, `_XIOError`
- the `-series` leg (`cargo xtask harness:boot-series --count 2 && ls …`) — green · `all-settled`, ordinals 2
  and 3
- `cargo xtask check:english-sources` — green · `clean`
- `cargo xtask capability-widening-check` · `check:ingest-progress` · `check:staged-artifacts` ·
  `capability-drift` · `verify:capability-matrix` — green
- `cargo nextest run --workspace --profile ci` — green · `2876 tests run: 2876 passed, 0 skipped`
- `cargo nextest run -p pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'` — green
- `git diff --quiet 8936f976… -- pulse-app/ui/src/bindings/index.ts` — green · exit 0
- `d="$(mise where node@24)" && PATH="$d/bin:$PATH" cargo xtask pre-push:linux` — green · `all-stages-ok`; and
  again on the committed tree `925be35f` in the operator pass, green
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` (operator) — green · `hygiene: clean`
- `git diff --quiet && git diff --cached --quiet && git push origin build/andromeda-pulse-0.4.0` (operator) —
  green · exit 0, `8936f976..925be35f`
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2700` (operator,
  attempt 1) — recorded. **Its outcome: `ci#38019133294` red, the one failed job `boot smoke (ubuntu-22.04)`.**
  Disposition: `red — not this chunk's: the same job read red, by itself, with the same exit record (exit 1, no
  app.exit) on the chunk base 8936f976 (ci#38014971549) and on three runs before it, all without this chunk's
  edits → the closing entry minted at this wrap's P5 (inputs#I4 item 1)`. What this chunk changed is that the red
  is now read by its own instrument: who made the ending call is in what the job keeps. One limit travels with
  that basis: the rate here (seven of eight boots) is higher than before, and whether the instrument or the
  series contributes is not measured.
- the jobs read, the boot job's log read, the artifact read (operator, attempt 1) — recorded; their readings are
  under Cross-project / external claims.
- the nine entries after them (three re-runs of the `boot` job, each with its CI read and artifact read) — not
  fired: attempt 1 held a self-end (the plan's step 5; the operator's word).

**Watches:** none folded.

**Outcome basis:** the operator pass ran, so the verdicts rest on its final state: the one commit `925be35f` and
its CI run `ci#38019133294`, recorded in `evidence/operator-pass.md`. /implement's P4 report, as given in this
session and accepted by the operator, is the basis for what only it holds (the deviations, the gate calls, the
census). The operator's directive between implement and this report changed nothing in the tree; the wrap relay
(inputs#I4) sets the route's shape, not the measurements.

**Process hygiene:** /implement's census: eight `pulse-app` processes and eight display servers started by the
legs across two gate calls, all terminated (no such process in the list; 14317 and 14318 refusing); the
hand-driven refusal started nothing. The operator pass started no app (the pre-push check binds no port).
Re-measured at this wrap, 03:30Z: `ps -eo pid,comm,args` holds no `pulse-app`, `Xvfb` or `xvfb-run`.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 8936f976 (the parent of the oldest pre-CI commit 925be35f) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .github/workflows/ci.yml — added 16 line(s) in 2 range(s)
added: 375-382 · 400-407
### pulse-app/tests/quality_gate_workflow.rs — added 95 line(s) in 1 range(s)
added: 564-658
- 568-585 @570 «fn boot_job_step(content: &str, name: &str) -> (usize, Vec<String>) {»
  - 574-577 «let start = lines»
  - 578-583 «let step = lines[start + 1..]»
- 587-619 @588 «fn ci_workflow_boot_job_builds_the_exit_witness_before_the_smoke() {»
  - 592-596 «assert!(»
  - 597-604 «assert!(»
  - 605-612 «assert!(»
  - 613-618 «assert!(»
- 621-644 @622 «fn ci_workflow_boot_series_runs_after_the_smoke_whatever_it_returned_and_before_ci_gates() {»
  - 627-631 «assert!(»
  - 632-636 «assert!(»
  - 637-643 «assert!(»
- 646-657 @647 «fn ci_workflow_boot_series_carries_no_soft_fail() {»
  - 649-656 «for banned in ["continue-on-error", "retry", "|| true"] {»
### scripts/agent-run.sh — added 22 line(s) in 3 range(s)
added: 44-56 · 90 · 92-99
### scripts/exit-witness.c — new file · 162 line(s)
- 42-50 @43 «static void copy_safe(char *dst, size_t cap, const char *src) {»
  - 45-48 «for (; src && src[i] && i + 1 < cap; i++) {»
- 52-55 «static void write_line(const char *line, int len) {»
- 57-61 «static void record_runtime_exit(void) {»
- 63-92 «__attribute__((constructor)) static void witness_init(void) {»
  - 74-82 «if (c >= 0) {»
  - 88-89 «int len = snprintf(line, sizeof(line), "{\"kind\":\"loaded\",\"pid\":%d,\"comm\":\"%s\"}\n",»
- 94-122 «__attribute__((noinline)) static void record_end(const char *call, int code) {»
  - 100-103 «int len = snprintf(line, sizeof(line),»
  - 104-118 @105 «for (int i = 1; i < n && len < LINE_MAX_BYTES - FRAME_MAX_BYTES - 4; i++) {»
- 124-130 «void exit(int code) {»
- 132-138 «void _exit(int code) {»
- 140-146 «void _Exit(int code) {»
- 148-154 «void quick_exit(int code) {»
- 156-162 «void abort(void) {»
### xtask/src/harness_ready.rs — added 11 line(s) in 8 range(s)
added: 9-10 · 112-114 · 122 · 359 · 369 · 642 · 660 · 668
  - 112-113 «let exit_witness =»
### xtask/src/harness_series.rs — new file · 756 line(s)
- 40-43 «if [ "$a" = 0 ]; then»
- 49-54 @50 «struct CycleExits {»
- 56-61 @57 «enum BootClass {»
- 63-68 @64 «struct BootRecord {»
- 70-94 «pub(crate) async fn run(count: u32) -> Result<ExitCode> {»
  - 74-85 «if let Some(data_dir) = &data_dir {»
  - 90-92 «if kept.is_some_and(|path| std::fs::write(path, format!("{text}\n")).is_err()) {»
- 96-106 @98 «fn evaluable_data_dir(count: u32) -> Option<PathBuf> {»
  - 99-101 «if !cfg!(target_os = "linux") || !count_in_range(count) || !on_path("xvfb-run") {»
- 108-111 «fn on_path(name: &str) -> bool {»
  - 109-110 «std::env::var_os("PATH")»
- 113-118 «fn workspace_root() -> PathBuf {»
  - 114-117 «Path::new(env!("CARGO_MANIFEST_DIR"))»
- 120-150 «async fn run_boot(root: &Path, data_dir: &Path, ordinal: u32) -> BootRecord {»
  - 124-128 «let made = [»
  - 131-137 «if !made {»
  - 141-145 «let record = BootRecord {»
  - 146-148 «if copy_kept(&logs, &data_dir.join("logs").join("series").join(&name)).is_err() {»
- 152-191 «async fn run_cycle(root: &Path, boot_dir: &Path) -> (Option<CycleExits>, bool) {»
  - 154-168 «cmd.current_dir(root)»
  - 169-173 «for (key, _) in std::env::vars_os() {»
  - 174-176 «let Ok(child) = cmd.spawn() else {»
  - 178-190 «match tokio::time::timeout(CYCLE_TIMEOUT, child.wait_with_output()).await {»
- 193-211 @196 «async fn stop_timed_out_cycle(root: &Path, boot_dir: &Path, wrapper: Option<u32>) {»
  - 197-202 «if let Some(pid) = wrapper {»
  - 203-210 «let _ = tokio::process::Command::new("bash")»
- 213-215 «fn count_in_range(count: u32) -> bool {»
- 217-241 @219 «fn parse_cycle_marker(stdout: &str) -> Option<CycleExits> {»
  - 220-223 «let line = stdout»
  - 225-231 «let mut field = |name: &str| {»
  - 236-240 «Some(CycleExits {»
- 243-254 @245 «fn cycle_label(exits: Option<CycleExits>, timed_out: bool) -> &'static str {»
  - 246-253 «match exits {»
- 256-259 @257 «fn cycle_stops_series(cycle: &str) -> bool {»
- 261-267 «fn classify(cycle: &str, settle_verdict: Option<&str>) -> BootClass {»
  - 262-266 «match settle_verdict {»
- 269-275 «fn settle_verdict(record: &BootRecord) -> Option<&str> {»
  - 270-274 «record»
- 277-279 «fn class_of(record: &BootRecord) -> BootClass {»
- 281-298 @283 «fn decide_series(records: &[BootRecord]) -> &'static str {»
  - 285-297 «if records.is_empty()»
- 300-306 «fn exit_status(verdict: &str) -> u8 {»
  - 301-305 «match verdict {»
- 308-310 «fn is_kept(name: &str) -> bool {»
- 312-325 @313 «fn copy_kept(from_logs: &Path, to: &Path) -> std::io::Result<usize> {»
  - 316-323 «for entry in std::fs::read_dir(from_logs)? {»
- 327-332 «fn read_settle(path: &Path) -> Option<Value> {»
  - 329-331 «serde_json::from_str::<Value>(&text)»
- 334-344 «fn bounded_label(value: Option<&Value>) -> Value {»
  - 335-343 «match value.and_then(Value::as_str) {»
- 346-359 @348 «fn boot_entry(ordinal: u32, cycle: &str, settle: Option<&Value>) -> Value {»
  - 350-358 «json!({»
- 361-387 @363 «fn series_payload(verdict: &str, records: &[BootRecord], smoke: Option<&Value>) -> Value {»
  - 364-369 «let count = |wanted| {»
  - 370-378 «let per_boot: Vec<Value> = smoke»
  - 379-386 «json!({»
- 389-756 @390 «mod tests {»
  - 394-400 «fn exits(boot: i32, status: Option<i32>, cleanup: i32) -> Option<CycleExits> {»
  - 402-413 «fn settle(verdict: &str) -> Value {»
  - 415-421 «fn boot(ordinal: u32, cycle: &'static str, verdict: Option<&str>) -> BootRecord {»
  - 423-430 «fn keys(value: &Value) -> BTreeSet<&str> {»
  - 432-439 @433 «fn the_count_is_taken_from_1_to_16() {»
  - 441-459 @442 «fn the_cycle_marker_is_read_from_its_own_line() {»
  - 461-465 @462 «fn a_cycle_whose_verbs_all_returned_zero_is_complete() {»
  - 467-477 @468 «fn a_failed_boot_or_an_unhealthy_status_is_named_and_the_series_goes_on() {»
  - 479-492 @480 «fn a_cycle_that_left_its_ports_in_doubt_stops_the_series() {»
  - 494-512 @495 «fn a_boot_is_settled_only_on_a_complete_cycle_and_ended_whatever_the_cycle() {»
  - 514-522 @515 «fn every_boot_settled_is_all_settled() {»
  - 524-533 @525 «fn one_boot_that_ended_by_itself_is_self_ended() {»
  - 535-546 @536 «fn a_boot_that_neither_settled_nor_ended_is_not_all_settled() {»
  - 548-560 @549 «fn no_boot_or_a_cycle_that_stopped_the_series_cannot_be_evaluated() {»
  - 562-604 @563 «fn the_verdict_carries_the_closed_member_sets() {»
  - 606-621 @607 «fn the_smoke_is_listed_as_ordinal_1_and_stays_out_of_the_counts() {»
  - 623-643 @624 «fn an_entry_takes_bounded_labels_and_a_count_and_nothing_else() {»
  - 645-678 @646 «fn the_kept_file_list_is_the_log_family_and_four_named_files() {»
  - 680-724 @681 «fn a_boot_keeps_its_named_files_and_nothing_of_its_run_dir() {»
  - 726-755 @727 «fn the_cycle_script_runs_the_smoke_sequence_and_always_cleans_up() {»
### xtask/src/harness_witness.rs — new file · 599 line(s)
- 20-25 @21 «enum LineKind {»
- 27-31 @28 «struct WitnessLine {»
- 33-38 @34 «enum WitnessRead {»
- 40-55 @42 «pub(crate) fn label(»
  - 47-54 «match data_dir {»
- 57-86 @60 «fn decide(pid: Option<u32>, ended: Option<&str>, read: &WitnessRead) -> &'static str {»
  - 61-65 «let lines = match read {»
  - 66-68 «let Some(pid) = pid else {»
  - 69-73 «let wrote = |kind| {»
  - 74-85 «if wrote(LineKind::End) {»
- 88-102 «fn parse_line(line: &[u8]) -> Option<WitnessLine> {»
  - 89-91 «if line.len() > MAX_LINE_BYTES || !line.iter().all(|byte| (0x20..=0x7e).contains(byte)) {»
  - 94-99 «let kind = match record.get("kind")?.as_str()? {»
- 104-116 @106 «fn parse_lines(bytes: &[u8]) -> Option<Vec<WitnessLine>> {»
  - 108-110 «if body.is_empty() {»
  - 112-114 «if lines.len() > MAX_LINES {»
- 118-135 «fn read_witness(path: &Path) -> WitnessRead {»
  - 119-124 «let size = match std::fs::metadata(path) {»
  - 125-127 «if size > (MAX_LINES * (MAX_LINE_BYTES + 1)) as u64 {»
  - 128-134 «match std::fs::read(path)»
- 137-599 @138 «mod tests {»
  - 147-149 «fn loaded(pid: u32) -> String {»
  - 151-153 «fn runtime_exit(pid: u32) -> String {»
  - 155-159 «fn end(pid: u32) -> String {»
  - 161-167 «fn read_of(lines: &[String]) -> WitnessRead {»
  - 169-176 @170 «fn no_witness_file_is_unset() {»
  - 178-188 @179 «fn an_unreadable_file_is_unreadable_whatever_the_exit_record_says() {»
  - 190-195 @191 «fn a_loaded_line_alone_under_a_live_or_signalled_app_is_loaded() {»
  - 197-206 @198 «fn an_end_line_is_exit_call_with_or_without_the_runtime_line() {»
  - 208-225 @209 «fn the_dev_host_control_line_reads_exit_call() {»
  - 227-231 @228 «fn a_runtime_exit_line_without_an_end_line_is_runtime_exit() {»
  - 233-238 @234 «fn a_loaded_line_alone_under_an_exit_code_is_no_record() {»
  - 240-246 @241 «fn only_the_lines_of_the_app_pid_count() {»
  - 248-257 @249 «fn a_file_with_no_loaded_line_for_the_pid_is_unreadable() {»
  - 259-295 @260 «fn the_reader_takes_the_three_kinds_and_nothing_else() {»
  - 297-306 @298 «fn the_reader_refuses_a_byte_outside_printable_ascii() {»
  - 308-334 @309 «fn the_reader_is_bounded_at_64_lines_of_4096_bytes() {»
  - 336-350 @337 «fn the_reader_takes_a_last_line_without_its_newline_and_refuses_a_blank_one() {»
  - 352-381 @353 «fn the_file_read_tells_absent_from_present_and_bounds_its_size() {»
  - 383-598 @386 «mod built_library {»
### xtask/src/main.rs — added 12 line(s) in 5 range(s)
added: 14 · 16 · 62 · 68-75 · 285
  - 72-75 «HarnessBootSeries {»
