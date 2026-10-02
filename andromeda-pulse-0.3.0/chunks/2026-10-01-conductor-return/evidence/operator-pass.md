# Operator pass — 2026-10-01-conductor-return

Run by the session on the overseer's word (2026-10-02: "continue with the operator pass up to printing S, and stop for me
to relay S"), after /implement closed green + smoke.

## Boot smoke (plan gates 17-19, operator slot #2b — 4317/4318 measured free by the overseer)
- Gate 17 `cargo build --workspace --release --features mcp-server`: green, exit 0, 645.82 s; `pulse-app.exe` and
  `andromeda-pulse-mcp.exe` written fresh in this run.
- The release `pulse-app.exe` booted BY PATH on a fresh, once-exported `ANDROMEDA_PULSE_DATA_DIR` (the dir did not exist
  before the boot); pidfile + `:4317` accepting after ~1 s; app pid 42076.
- Gate 18 (non-priming `:4317` liveness): green, exit 0. Gate 19 (boot health over the `agent-latest.jsonl*` family while
  the app ran): green, `exit 1` / `last line 0` — 0 `app.panic.fatal`, 0 `"level":"ERROR"`. The family carried every
  boot record, all five tick families, live webview traffic (WebGPU frames, `ui.webgpu.adapter`), and one WARN
  (`interpretation.model.allow_root`, the documented unconfined posture).
- Teardown: `Stop-Process -Id 42076 -Force`; pid gone, no `pulse-app` process left, `:4317` and `:4318` refused.
  0 `app.exit` records in that family — expected: TerminateProcess is unloggable by construction.

## Gate 20 — hygiene
`python -X utf8 …/gate.py hygiene`: exit 0, `hygiene: clean — read 33 (runs 30 · evidence 3)`; re-fired after this file
was written (see the commit's own hygiene read).

## Gate 21 — `cargo xtask pre-push:linux`
Exit 0, verdict document `"verdict": "green"`, six stages. The WSL `test` stage ran the workspace suite at 2562/2562
passed, including the two Unix-only arms that cannot run on the Windows host:
`exit_cause_process_exit_is_recorded_outside_event_loop` (b) and `exit_cause_sigterm_is_recorded_and_still_ends_by_signal`
(d) — the SIGTERM child ended BY signal 15 with exactly one `app.exit {exit_class: signal, signal: sigterm}` record — plus
(a), (c), (e), (e2) and the four leaf pins.

## Gate 22 — S
Printed after the pre-CI commit; recorded in `round-binary.md`.
