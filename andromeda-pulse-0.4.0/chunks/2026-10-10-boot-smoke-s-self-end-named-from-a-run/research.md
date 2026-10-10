# Codebase Research — 2026-10-10-boot-smoke-s-self-end-named-from-a-run

## Scope
- **Depth:** deep on the boot job, the harness driver and the settle verdict; nothing read in the webview source · **Reads:** 17 · **Globs/Greps:** 15 · **Local boots:** 16 (13 of the unchanged sequence, 3 controls) · **CI artifacts read:** 8
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, Session Additions included; 6 additions applied: one exported data dir per leg (2026-08-23), the log family read by glob `agent-latest.jsonl*` (2026-06-29), a harness's own absence never reported as a product fact without a second source (2026-08-29), every xtask child cargo through `cargo_command()` (2026-09-29), a leg proves its own precondition and reads INCONCLUSIVE when it cannot (2026-08-28), the leg's binary rebuilt before it is read (2026-07-05 extension; the first local boot rebuilt the release binary at `8936f976`, 87 s). `.claude/rules/testing.md` and `.claude/rules/observability.md` were loaded with the files read.
- **Platform issues consulted:**
  - Fetched `https://raw.githubusercontent.com/actions/runner-images/main/images/ubuntu/Ubuntu2204-Readme.md`: image version `20261004.315.1`, kernel `6.8.0-1064-azure`. The document does not list `strace`, `ltrace`, `gdb`, `lldb`, `bpftrace`, `perf` or `valgrind`. It lists `gcc 4:11.2.0-1ubuntu1`, `binutils 2.38-4ubuntu2.12`, `libunwind8 1.3.2-2build2.1`, `systemd-coredump 249.11-0ubuntu3.22`, `xvfb 2:21.1.4-2ubuntu1.7~22.04.16`, `dbus 1.12.20-2ubuntu4.1`.
  - Fetched `https://gitlab.gnome.org/GNOME/gtk/-/raw/gtk-3-24/gdk/x11/gdkmain-x11.c`: `gdk_x_io_error` logs with `g_debug` ("We g_debug() instead of g_warning(), because g_warning() could possibly be redirected to the log") and then calls `_exit (1)`; the file installs it with `XSetIOErrorHandler (gdk_x_io_error)` and does not call `XInitThreads`. So one documented library end is an exit with code 1 that prints nothing by default and that the product's exit record cannot see. This is the branch head; the runner's GTK build (Ubuntu 22.04's 3.24 package) was not read.
  - Query `tauri webkit2gtk app exits with code 1 silently no output xvfb-run GitHub Actions ubuntu-22.04 intermittent after webview load` and query `"Fatal IO error 11 (Resource temporarily unavailable) on X server" tauri OR tao OR wry OR webkit2gtk xvfb`: the results held no report of this signature for this stack. Nothing was fetched from them as a source.
- **External inputs:** `inputs#I1` — the pc overseer's phase directive (the cause named from what a CI run keeps, two research questions, no retry, soft-fail key or timing pad, the boot job's steps and uploads as the boundary, a founder halt on an artifact wider than the app's log and exit record); `inputs#I2` — the invocation word (fold the relay, print the plan card at P5 and stop).

## What the runs show

The readings, tables and the probe's records are in the phase run dir, `p3-measurements.md`.

### The recorded CI runs (re-read at P3)
Eight `logs-boot-Linux` artifacts were read record by record: red `ci#37924991598` (`b3ac58a9`), red `ci#37961031489` (`f36a2ac3`), green `ci#37979648967` (`e2931127`), green `ci#37986543962` (`8a89e368`), green `ci#37990081874` (`8f655cae`), red `ci#38010977166` (`cb8cc4dc`), green `ci#38014639286` (`b7101914`), red `ci#38014971549` (`8936f976`, the chunk base).
- **The fourth red arrived during this phase.** `ci#38014971549`, job `114103109878`, artifact `11656415762` (4510 B, expires 2026-10-24): `harness-settled.json` reads `verdict ended`, `ended "exit 1"`, `app_exit_record absent`, `windows_settled 0`, `display reachable`, `session_bus reachable`; `xvfb.log` is 0 B; `boot.log` holds the one accessibility-bus line; the log is 57 records long, `ui-bridge.ready` at 0.928 s and the last record (`triage.cue.tick`) at 1.273 s after the first. No `app.exit`, no `app.panic.fatal`, no `app.boot.window.navigation`. The commit under it changed `scripts/code-graph.py` and four files of a run dir (`git show --stat 8936f976`), nothing a job builds.
- **The four reds share one shape.** Each log holds both `ui.webgpu.adapter` records (`main` and `compact-widget`, both `no_navigator_gpu`) and ends 0 to 0.34 s after the second. Each run's one failed job is `boot smoke (ubuntu-22.04)` (`gh api …/actions/runs/{id}/jobs`, the four reds).
- **A lead that did not hold.** On the two reds read first the webview's first record came later (0.829 s, 0.898 s) than on two greens (0.440 s, 0.565 s). Two other greens read 1.150 s and 1.390 s. It separates nothing.
- **The count, re-derived.** `gh api repos/{owner}/{repo}/actions/workflows/ci.yml/runs?per_page=40`, rows created from `2026-10-09T11:38:31Z` on: fourteen runs, each attempt 1. Twelve are pull-request runs on the build branch, four of them red (`b3ac58a9`, `f36a2ac3`, `cb8cc4dc`, `8936f976`); one is the hotfix branch's (green), one a push on `main` (green). Under the present smoke step (the settle verdict, from `e2931127` on) the `boot` job has six readings, two red.
- **What "display reachable" does and does not say.** The settle verdict's `display` label is a fresh connection attempt to the display's socket made after the app has ended (`xtask/src/harness_ready.rs`, `unix_socket_accepts`). It shows the server was accepting. It does not show the app's own connection was sound.
- **The freight's claims hold on re-read** (scope.md, "The causal and measured claims"): 53 records with `ui-bridge.ready` at 0.835 s and the last record at 0.907 s on `ci#38010977166`; the navigation records 4.129 s to 4.696 s after `ui-bridge.ready` on four greens.

### The instrument (the relay's question 3a)
- **The runner image carries no tracer and carries a C compiler** (the image's own list, above). A tracer would be a package the job installs; a library built from a short C source needs nothing the image lacks.
- **A library loaded into the app process alone records the ending call with its caller.** A research probe of about 130 lines (kept in the session scratchpad, not in the tree) interposes `_exit`, `_Exit`, `exit`, `quick_exit` and `abort`, and at the call appends one JSON line: pid, thread id, process name, the call, its code, `errno`, and the caller frames as module basename, exported symbol and offset. It was loaded through `LD_PRELOAD` on the app's spawn line of a scratch copy of `scripts/agent-run.sh` (one line changed, line 79).
- **Read on a known member of the class (the display stopped 2 s after ready, 2 boots):** settle verdict `ended`, `exit 1`, no `app.exit`, nothing new in `boot.log`, and the probe's record: `_exit`, code 1, `errno` 11, called from `libgdk-3.so.0` +0x8ca2d (a static function, so no symbol), entered from `libX11.so.6` `_XIOError` under `_XEventsQueued` under `XPending`, under GLib's main loop and `gtk_main_iteration_do`, on the main thread, 2 of 2. One boot also recorded a second thread reaching the same handler through `XNextEvent` called from the app binary. Without the probe the same control reads only "exit 1, no record" (1 boot).
- **Its cost on green boots is not visible at this n.** Six boots with it against six without, alternating, load 2.2 to 5.9: the boot verb returned in 1.02 s to 1.09 s in both arms; the webview's first record came at 0.869 s (0.854 to 0.913) with it and 0.911 s (0.852 to 1.017) without; the four windows settled at 5.173 s in both. It wrote 0 lines on every green boot (a SIGTERM end is no exit call) and added no line to `boot.log`. **Not measured: its cost on the runner.**
- **What it cannot see, by construction.** An end by a signal. An end that returns from `main` (measured: `/usr/bin/false` under the probe leaves 0 lines; the C runtime's own exit is not an interposable call), which the product's `atexit` hook does record as `app.exit`. A direct system call that bypasses the C library's wrappers. A static caller has no symbol, so the record names it as module and offset; the exported frames around it carry names.
- **A direct-call child is a deterministic witness for a committed test.** `python3 -c "import os; os._exit(7)"` under the probe wrote one line: `"call":"_exit","code":7`, the frames under `libpython3.14.so.1.0`.
- **The webview's child processes were not instrumented and not measured.** Seven probe boots with the variable inherited wrote 0 child lines and no loader refusal line; whether those processes load an inherited library was not established either way. The record needed is of the app process's own end.
- **The corpus key does not reach such a record.** The record holds what the library formats and nothing of the process's memory, environment or arguments. On the runner the app holds no corpus key at all: every boot log read carries `corpus.open.error {error_kind: KeyringUnavailable}`, and `ci.yml` names no passphrase variable (`grep -n PASSPHRASE .github/workflows/ci.yml`, 0 lines).

### Several boots in one run (the relay's question 3b)
- **The workflow's recorded shape admits it as a step, not as a job or a matrix value.** `ci_workflow_runs_on_linux_only` refuses a `matrix.os` token and any other runner; the trigger block is pinned; the four smoke pins read only the step named `Boot pulse-app smoke` up to the next `- name:` line (`pulse-app/tests/quality_gate_workflow.rs:435-449`), so a following step is not read by them. A second boot inside the smoke step itself would trip `only_line_with`, which requires exactly one line per verb.
- **Each boot needs its own data dir.** `harness:settled`, `harness:status` and `ci-gates` resolve one log family from `ANDROMEDA_PULSE_DATA_DIR`; both readers list the log directory's own files only (`xtask/src/main.rs:623-642`, `xtask/src/smoke.rs:519-536`), so a file kept one directory below `logs/` is not swept into the family `ci-gates` grades.
- **`run/` must not be uploaded with it.** The data dir's `run/` holds the product-written `workspace-key`, a path-valued file; the job uploads `logs/` alone, so what a further boot keeps is copied into `logs/` file by file.
- **Booting again changes what is measured in two named ways.** The webview keeps state under the user's data directory: one probe boot with `XDG_DATA_HOME` and `XDG_CACHE_HOME` pointed at fresh directories created `com.andromeda.pulse/` there with `WebKitCache`, `CacheStorage`, `storage`, `mediakeys` and `hsts-storage.sqlite`, and settled with four windows. Without the two variables a second boot in one job finds the first boot's state. The file cache is warm for every boot after the first. Every recorded red was the first and only boot of a fresh runner.
- **Not measured: whether a later boot ends by itself at the first boot's rate.** No run has held more than one boot. On the dev host thirteen sequential boots of the unchanged sequence all settled, which says nothing about the runner.
- **What one run would read.** At two reds in six, one boot per run reads about one self-end in three runs. If later boots end at that rate, eight boots in one run hold at least one self-end with probability about 0.96; if they do not, the first boot still carries the instrument.

### The harness as it stands
- `scripts/agent-run.sh:71-99` runs the app under a waiting subshell that is its direct parent, records the app's pid in `run/andromeda-pulse.spawn` and its end in `run/andromeda-pulse.exit`. A library preloaded on that spawn line leaves the pid, the parent and the reaper as they are (measured: every probe boot's exit record and `cleanup: clean` read as in the plain arm).
- `scripts/agent-run.ps1` mirrors the recorder (`:170-182`). A preloaded library has no form there; the script is parsed and never run on this host and leaves with "Other operating systems retired from the code".
- `xtask/src/harness_ready.rs:87-122` (`run_settled`) reads the exit record, the log evidence and the two socket labels after the verdict, prints one JSON object and writes it to `logs/harness-settled.json`. Its member set is pinned by `settled_payload_carries_the_closed_field_set` (`:624`).
- The `a11y` job does not run the `boot` verb: `agent-run` appears in `ci.yml` on lines 385, 387 and 388 only, all in the `boot` job (`grep -n agent-run .github/workflows/ci.yml`).
- The `boot` job's steps (`ci.yml:325-403`): harden-runner first, the data dir exported to `$GITHUB_ENV`, the build, the smoke step, `ci-gates` (skipped when the smoke fails), the upload under `if: always()` of `${{ env.ANDROMEDA_PULSE_DATA_DIR }}/logs/`.
- A sweep hazard for any new workflow comment: `ci_workflow_runs_on_linux_only` refuses the token `windows` on every line of `ci.yml`, comments included (`pulse-app/tests/quality_gate_workflow.rs:114-121`), so a step comment cannot name the app's windows by that word.

## Files inspected
- `.github/workflows/ci.yml` (1-22, 325-403) — the triggers and permissions, the `boot` job whole.
- `scripts/agent-run.sh` (full) — the five verbs; the spawn line (`:79`), the waiting wrapper (`:78-89`), the readiness poll (`:106-118`), `cleanup`.
- `scripts/agent-run.ps1` (by grep) — the mirrored recorder and `Invoke-Ready`.
- `xtask/src/harness_ready.rs` (1-125, 160-380, the test list) — `run_settled`, `decide_settled`, `read_exit_record`, `settled_payload`, `keep_verdict`, the socket labels.
- `xtask/src/harness_status.rs` (the function list) — `resolve_paths`, `resolve_data_dir`, `end_file`, `read_ended`, `read_pid`, `pid_alive`.
- `xtask/src/main.rs` (the verb list, 373-381, 623-642) — the three `harness:*` verbs, `cargo_command`, `collect_log_files`.
- `xtask/src/smoke.rs` (519-545) — `read_jsonl_lines`, the family reader.
- `xtask/src/pre_push.rs` (by grep) — `script-modes` reads the mode of `scripts/agent-run.sh` alone.
- `pulse-app/src/observability.rs` (2700-2914) — the `app.exit` record, the `atexit` hand-off, the signal listener, the stated unloggable list.
- `pulse-app/tests/quality_gate_workflow.rs` (101-131, 418-562) — the Linux-only pin, the upload pin, the four smoke pins.
- `Cargo.toml` (252-261) — the release profile strips symbols (`strip = true`), so the app binary's own frames carry no name.
- `.andromeda/registries/contracts/architecture/project-directory-structure.md` (full) — the directory contract; `scripts/` is the driver's registered home in §Occupied Resources.
- The previous two chunks' records: `2026-10-09-boot-smoke-s-early-exit-found-and-closed/{scope,research}.md`, `2026-10-09-pre-push-check-native-on-linux/evidence/operator-pass.md` (91-190).

## Graph impact (rust plane, `tree-query-2026-10-10-boot-smoke-s-self-end-named-from-a-run.json`, one query, 41 rows, `db_state regenerated`)
- **`settled_payload`** — 2 callers: `run_settled @ xtask/src/harness_ready.rs:111` and the pin `settled_payload_carries_the_closed_field_set @ xtask/src/harness_ready.rs:625`. A member added to the verdict changes both and nothing else.
- **`run_settled`** — 1 caller, `main @ xtask/src/main.rs:274`. **`keep_verdict`**, **`read_exit_record`** — 1 caller each, `run_settled @ xtask/src/harness_ready.rs:120` and `:99`.
- **`read_ended`**, **`end_file`**, **`resolve_data_dir`** (`xtask/src/harness_status.rs`) — production callers `run @ harness_status.rs:45`, `run_ready @ harness_ready.rs:64`, `run_settled @ harness_ready.rs:99`, `read_exit_record @ harness_ready.rs:210`, `keep_verdict @ harness_ready.rs:369`, `resolve_paths @ harness_status.rs:124`; the rest are in-file pins. Reusing them from a sibling module touches no caller.
- **`cargo_command`** — 9 call sites in `xtask/src/{main,external_resolve,gap_resume,webview_drive}.rs`; a new child cargo joins them, none changes.
- Name sweep for the settle verdict's readers outside its module (`grep -rn 'harness-settled\|app_exit_record\|harness:settled' crates pulse-app/src pulse-app/tests xtask scripts .github`, `xtask/src/harness_ready.rs` left out): 9 hits · 0 changed · 9 no-change (the workflow step line and two comment lines, the four pin lines that match the verb's name, the verb's declaration and its `about` text). No test outside `xtask/src/harness_ready.rs` reads the verdict's members.

## Patterns detected
- **A pure verdict in xtask, pinned per arm, with the scripts as thin callers** (`xtask/src/harness_ready.rs:133-140, 169-182`, pins `:393-507`): the shape a series verdict and a witness reading follow.
- **A bounded-grammar read of a harness-written file** (`xtask/src/harness_status.rs:210-215`, `read_ended`: one line, at most 48 printable ASCII characters, else no record): the shape for reading a witness record.
- **A harness-only tool locator, guarded by trim, `is_file()` and a clean skip** (`ANDROMEDA_PULSE_MSEDGEDRIVER_PATH`, `.claude/rules/security.md`, Input validation): the shape for a variable that names a built library.
- **The verdict kept beside the log** (`xtask/src/harness_ready.rs:366-378`, `keep_verdict`): written only when `logs/` exists, a failure to write reported on stderr and never changing the exit code.
- **Workflow self-lint by substring over one named step** (`pulse-app/tests/quality_gate_workflow.rs:435-466`).

## Conventions to follow
- **xtask verdict verbs print one JSON object and exit 0, 1 or 2; `cannot-evaluate` never passes** (`xtask/src/harness_ready.rs:124-131`).
- **A harness tool reads system variables by presence and prints closed labels only; no value and no path in a kept file** (`xtask/src/harness_ready.rs:12-14`).
- **Every xtask child cargo goes through `cargo_command()`** (`xtask/src/main.rs:373-381`).
- **`scripts/agent-run.sh` stays git mode 100755** (`xtask/src/pre_push.rs:26`, the `script-modes` stage).
- **A data dir exported once per leg; the receivers moved off 4317 and 4318 on this host through the two port variables** (the handoff's host note).
- **xtask pins sit beside their code in `xtask/src/`; a workflow pin sits in `pulse-app/tests/quality_gate_workflow.rs`.**

## New files to create
- `scripts/exit-witness.c` — the library's source: the interposed exit calls and the one-line record
- `xtask/src/harness_witness.rs` — the bounded reader of the witness record and what the settle verdict says of it, with pins
- `xtask/src/harness_series.rs` — the verb that boots again, each boot on its own data dir and display, and its verdict, with pins

## Files to modify
- `scripts/agent-run.sh` — the boot verb preloads the library on the app's spawn line when the harness names a built one
- `xtask/src/harness_ready.rs` — the settle verdict says what the witness recorded; its member pin moves
- `xtask/src/main.rs` — the new modules and the series verb declared and dispatched
- `.github/workflows/ci.yml` — the boot job builds the library, names it to the harness and runs the series
- `pulse-app/tests/quality_gate_workflow.rs` — pins for the boot job's new steps

## Open questions
- Whether the kept witness record sits inside "the app's own log and exit record" (inputs#I1 item 5) or is wider and the founder's to admit, and who classifies it → blocks: plan-decision.
- Whether the series stands as a gating step of the `boot` job, or the job keeps one witnessed boot and waits for a natural red → blocks: plan-decision.
- Whether this plan names the cause only, with its closing planned from the run that names it, or holds the closing too → blocks: plan-decision.
