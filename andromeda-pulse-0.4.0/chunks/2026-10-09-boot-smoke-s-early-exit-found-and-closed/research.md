# Codebase Research — 2026-10-09-boot-smoke-s-early-exit-found-and-closed

## Scope
- **Depth:** deep on the harness, the exit record and the boot job; nothing read in the webview source · **Reads:** 11 · **Globs/Greps:** 9 · **Local boots:** 56 (51 measured, 5 controls)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, Session Additions included; 4 additions applied: one exported data dir per leg (2026-08-23), the binary prebuilt and its age known before a leg reads it (2026-07-05 extension in `rules/testing.md`, 2026-08-28), the log family read by glob `agent-latest.jsonl*` (2026-06-29), a harness's own absence never reported as a product fact without a second source (2026-08-29). `.claude/rules/testing.md` and `.claude/rules/observability.md` were loaded with the files read.
- **Platform issues consulted:**
  - Query `tauri app exits with code 1 silently under xvfb-run GitHub Actions ubuntu-22.04 webkit2gtk-4.1` and query `Xvfb crashes OR "lost connection" webkit2gtk-4.1 Tauri app exits ubuntu-22.04 GitHub Actions xvfb-run`: the results held no report of this signature (an app that ends with exit 1 and prints nothing a fraction of a second after its webview's first calls). Nothing was fetched from them as a source.
  - Fetched `https://dbus.freedesktop.org/doc/api/html/group__DBusBus.html`: `dbus_bus_get` "will call dbus_connection_set_exit_on_disconnect(), so the application will exit if the connection closes"; `dbus_bus_get_private` the same. Fetched `…/group__DBusConnection.html`: `dbus_connection_set_exit_on_disconnect` — "Set whether _exit() should be called when the connection receives a disconnect signal." This is one documented library end that writes no record. The bus-stopped control below did not produce it on this host.
- **External inputs:** `inputs#I1` — the pc overseer's phase directive (cause from run evidence, a measured local rate first, the readiness gap in scope, `ci#37971913620` the first reading, the window not removed); `inputs#I2` — the invocation word (read the relay whole, reproduce before theorizing, stop at P5).

## What the runs show

### The recorded CI runs (re-read at P3)
Five `logs-boot-Linux` artifacts were downloaded and read record by record: red `ci#37924991598` (`b3ac58a`), red `ci#37961031489` (`f36a2ac`), green `ci#37945548047` (`569604b`), green `ci#37954318153` (`b3e5859`), green `ci#37971913620` (`277d65d`).
- The entry's first `CARRY:` holds on re-read. Both red logs end on a webview-originated record (`ui.webgpu.adapter`, `outcome: no_navigator_gpu`) and hold no `app.exit` and no `app.panic.fatal`. Every green log ends on `app.exit` with `signal: sigterm`, the smoke's own `cleanup`.
- All five `boot.log` files hold one line, the accessibility-bus warning. No run's `boot.log` holds anything else.
- Up to the webview's calls, a red log and a green log that got that far hold the same record sequence. In the two greens the app was stopped 0.13 s and 0.20 s after its last webview-originated record. In red `ci#37924991598` the last record is at 11:47:59.228Z and `status` printed `not-running` about 0.4 s later.
- **This chunk's first reading:** `boot smoke (ubuntu-22.04)` ended `success` on `ci#37971913620` (`277d65d`, job `113960407205`, 18:14:43Z to 18:23:33Z). Ready printed at 18:23:27.03Z, `status` read `running-healthy` 0.32 s later, `cleanup: clean` followed. Its log holds 42 records and no webview-originated record, so the app was stopped before the webview's first call. The whole run read `verdict: green · checks 7/7 · wall 1190 s` (`ci.py conclusion`, re-read at P3). That makes two reds in eight readings.
- **No recorded run shows the app alive more than 0.27 s after the webview's first call.** Four of the eight logs never reach that call. So the eight runs cannot say whether a green run would have stayed up.
- The CI log lines carry `ci.run.id` and `git.commit.sha` (both keys present in all five logs).
- **What the job keeps:** the artifact is `${ANDROMEDA_PULSE_DATA_DIR}/logs/` and nothing else (`ci.yml:391-398`): `agent-latest.jsonl.{date}`, `boot.log`, `build.log`. The wrapper's exit record sits under `run/` and is not uploaded; it reaches the job log only through the `ended` field of the `status` verdict. The display server's own error output is not kept anywhere: `xvfb-run` sends it to `/dev/null` unless `-e FILE` is given (`xvfb-run --help`, line 9, read on this host; the job passes no `-e`, `ci.yml:380`).

### The local reproduction (2026-10-09, 18:30Z to 18:50Z)
The job's three verbs were run on this host under `xvfb-run -a --server-args="-screen 0 1280x1024x24"`, with the release binary rebuilt at `277d65d` by the boot verb itself (2 m 31 s, the first boot). Three stated differences from the runner: the OTLP ports moved to 14317 and 14318 through `ANDROMEDA_PULSE_OTLP_{GRPC,HTTP}_PORT`, so 4317 and 4318 were never bound; `WAYLAND_DISPLAY` unset, so the window opened on the virtual display; a fresh data dir per boot. Host load was 22 to 33 throughout (the `load1` column of the summaries; other builders' cargo was running). The summaries are in the phase run dir, `repro-*.tsv`.

| Arm | Boots | What was varied | Self-ends | Reading |
|---|---|---|---|---|
| The CI sequence unchanged (boot, status, cleanup) | 21 | nothing | 0 | 21 of 21 green; every boot was stopped before the webview's first call (0 of 21 logs hold `ui.webgpu.adapter` or `ui-bridge.ready`; 41 records each) |
| Hold: boot, then 15 s watching the exit record, then status, cleanup | 20 | the app is left alive | 0 | 20 of 20 lived 15 s and ended on the smoke's SIGTERM with an `app.exit` record; 297 to 305 records |
| Hold under an empty private session bus | 10 | a `dbus-daemon` with no service directory, so the keyring and the accessibility bus answer `ServiceUnknown` | 0 | 10 of 10 lived 15 s; the log takes the runner's shape (`corpus.open.error KeyringUnavailable`, the three persist warnings, the same accessibility-bus line in `boot.log`) |

**The early exit did not reproduce on this host: 0 self-ends in 51 boots.** Thirty of those boots lived well past the point where both red runs ended.

Two controls, each stopping something the app depends on 5 s after ready:

| Control | Boots | How the app ended | `app.exit` record | New line in `boot.log` |
|---|---|---|---|---|
| The private session bus stopped | 3 | `signal 15 (TERM)`, within 1 ms of the stop, 3 of 3 | present (`signal`, `sigterm`), preceded by a WARN "D-Bus message processing error: … disconnected from D-Bus?" | none |
| The virtual display stopped (`Xvfb` by pid) | 2 | `exit 1`, within 0.1 s of the stop, 2 of 2 | **none** | **none** |

- **The display-stopped control is the first local run with the reds' signature.** `status` printed `"ended": "exit 1"`, `"verdict": "not-running"`; the log ends on an ordinary record (`triage.cue.tick`) with no `app.exit`; `boot.log` gained nothing. Those are the three facts both red runs show.
- It is a control, not a reproduction. It shows what a lost display connection looks like in what the job keeps. It does not show that the runner's display goes away, and no recorded run can show it either way: the display server's errors are discarded and nothing reads whether it is alive after the app ends.
- The bus-stopped control ends the app too, but with a record and by signal, so on this host a lost session bus does not look like the reds. The runner's library versions differ (Ubuntu 22.04 against glib2 2.88.3, dbus 1.16.2, webkit2gtk-4.1 2.52.6 here, `pacman -Q`), so this excludes nothing about the runner.
- An end by `exit()` would be recorded: the `atexit` hook hands off to the reporter thread (`pulse-app/src/observability.rs:2838-2882`, witnessed by `pulse-app/tests/integration_exit_cause_record.rs`). An end by `_exit` is on the record's stated unloggable list (`observability.rs:2715-2717`). The display-stopped control therefore ends through a path of that unloggable class; which function calls it was not measured (no tracer is installed on this host: `strace` absent, checked with `command -v`).

### The readiness gap
- `scripts/agent-run.sh:105-116` prints `boot: ready` when `cargo xtask harness:status` exits 0, with no port probe. The same script already probes both resolved ports through `/dev/tcp` in `cleanup` (`:211-216`).
- `scripts/agent-run.ps1:186-199` has the same gap: ready is printed when `Invoke-Status` returns, and its `cleanup` already probes with `System.Net.Sockets.TcpClient` (`:83`).
- `harness:status` derives its verdict from the pid file, that pid's liveness and the log family's mtime (`xtask/src/harness_status.rs`, `classify` and `run`). Receiver bind is not an input.
- The app logs `app.boot.otlp.grpc.bind` and `app.boot.otlp.http.bind` 0.17 s to 0.35 s after its first record (three CI logs and two local boots read). In three local hold boots read record by record, the webview's first call came 0.8 s to 1.4 s after the first record.
- **A signal the app already emits says its windows settled.** `pulse-app/src/window.rs:260-296` emits `app.boot.window.navigation` once per window 5 s after boot (`NAVIGATION_SETTLE`), with `window_label`, `navigated`, `reason`. In every local hold boot all four labels (`compact-widget`, `main`, `findings`, `report`) appeared at 5.17 s after the first record. No CI log has ever held this record, because the smoke stops the app within 1.40 s.
- The `a11y` job does not run the `boot` verb: `agent-run` appears in `ci.yml` on lines 382 to 384 only, all in the boot job (`grep -n agent-run .github/workflows/ci.yml`).

## Files inspected
- `scripts/agent-run.sh` (full) — the five verbs; the waiting wrapper sends the app's stdout and stderr to `logs/boot.log` (`:79`), and the subshell's own streams to `/dev/null` (`:89`); the readiness poll (`:105-116`); the failure path (`:117-134`); the cleanup port probe (`:211-216`).
- `scripts/agent-run.ps1` (140-214) — the Windows twin of the boot verb; same readiness poll.
- `.github/workflows/ci.yml` (325-398) — the boot job: the smoke step (`:375-385`), `ci-gates` (`:388-389`), the upload (`:391-398`).
- `xtask/src/harness_status.rs` (outline and the function list) — `run`, `classify`, `pid_alive`, `ps_state_is_live`, `read_ended`, `end_file`, `newest_family_member`, with their in-file pins.
- `xtask/src/main.rs` (the verb list) — `harness:status` is declared at `:50`.
- `xtask/src/pre_push.rs` (27, 207-234) — the `script-modes` stage reads the git mode of `scripts/agent-run.sh` alone.
- `pulse-app/src/observability.rs` (2700-2915) — the `app.exit` record: `record_exit`, `exit_after_event_loop`, the `atexit` hand-off, the signal listener.
- `pulse-app/src/main.rs` (278-300, 1556-1579) — `main()`'s order and the `run_return` tail.
- `pulse-app/src/render_posture.rs` (full) — the one prior Linux launch end closed in the product; a native-toolkit `_exit(1)` on another surface.
- `pulse-app/src/window.rs` (the settle lines, by grep) — the post-settle navigation record.
- `.andromeda/registries/contracts/test-plan/5-command-implementation.md` (1-12) — the `boot` clauses: Readiness signal with the open gap, Exit code, Timeout.
- `pulse-app/tests/quality_gate_workflow.rs` (428, by grep) — the one test line that names the boot job's upload.

## Graph impact (rust plane, `tree-query-2026-10-09-boot-smoke-s-early-exit-found-and-closed.json`, one query, 51 rows)
- **`classify`, `pid_alive`, `newest_family_member`, `read_ended`** (`xtask/src/harness_status.rs`) — each has one production caller, `run @ xtask/src/harness_status.rs:41-45`; every other caller is an in-file pin (`harness_status.rs:253-371`). `classify` in `xtask/src/staged_gate.rs` is another function of the same name. Reusing these probes from a sibling module touches no caller.
- **`record_exit`** — three production callers: `exit_after_event_loop @ pulse-app/src/observability.rs:2819`, `run_exit_reporter @ :2865`, `install_signal_listener @ :2907`; two test callers in `pulse-app/tests/integration_exit_cause_record.rs`. **`install_exit_hook`, `install_signal_listener`, `hold_log_guard`, `exit_after_event_loop`** — `main @ pulse-app/src/main.rs:295-298, 1578` plus that one test file. This chunk's plan changes none of them.
- **`apply_linux_default`** — `main @ pulse-app/src/main.rs:279` and `pulse-app/tests/unit_render_posture.rs`. Unchanged.

## Patterns detected
- **A pure verdict in xtask, pinned per arm, with the scripts as thin callers** (`xtask/src/harness_status.rs`: `classify` plus probe parsers, pins at `:248-371`): the shape a readiness decision and a settle decision follow.
- **An independent TCP probe on the resolved ports** (`scripts/agent-run.sh:211-216`, `scripts/agent-run.ps1:83`): the cleanup verdict's probe, which readiness reads in the other direction.
- **The exit recorder beside the pid file** (`scripts/agent-run.sh:75-89`, read by `read_ended`): one bounded line, harness-written, never read by the product.
- **Workflow self-lint in `pulse-app/tests/quality_gate_workflow.rs`**: the workflow is read as text and pinned by substring; the boot job's upload name is pinned at `:428`.

## Conventions to follow
- **xtask verdict verbs print one JSON object and exit 0, 1 or 2** (`xtask/src/harness_status.rs:56-76`; arch §Occupied Resources, xtask CLI surfaces).
- **A harness tool reads system variables by presence and prints closed labels only** (the basename and env-name rules in `.claude/rules/security.md`, Logging & redaction).
- **Every xtask child cargo goes through `cargo_command()`** (`.claude/rules/verification-harness.md`, 2026-09-29).
- **`scripts/agent-run.sh` stays git mode 100755** (`xtask/src/pre_push.rs:228-234`).

## New files to create
- `xtask/src/harness_ready.rs` — the readiness verdict and the settle verdict with their probes and pins

## Files to modify
- `xtask/src/main.rs` — two verbs declared and dispatched
- `xtask/src/harness_status.rs` — its path, liveness and exit-record helpers made reachable from the sibling module
- `scripts/agent-run.sh` — the boot verb polls the readiness verdict and names a receivers-never-accepted failure
- `scripts/agent-run.ps1` — the same change, mirrored
- `.github/workflows/ci.yml` — the boot job's smoke step
- `pulse-app/tests/quality_gate_workflow.rs` — pins for the smoke step's shape

## Open questions
- How many consecutive green runs of equal source the capability asks for, and which pushes make them → blocks: plan-decision. Answered at P4 (inputs#I3): three, from the pass's pushes, only once a cause is named and closed; otherwise no claim and the count runs to eleven.
- Whether keeping the display server's own error output and the settle verdict in the uploaded artifact is a boundary widening → blocks: plan-decision. Answered at P4 (inputs#I3): routine harness evidence, with a stop if the display output carries telemetry.
- Whether the runner's display goes away when the app ends → blocks: implementation-scope. No recorded run can answer it; the operator pass's first run under this plan reads it.
