# Operator pass — 2026-09-30-perf-budget-gate-reads-real-samples

Run by /implement on the overseer's word (2026-09-30): entries #26 → #32 in order, no window opens.

## Hygiene blocker — fixture moved (overseer's instruction)

`gate.py hygiene` refused P3 on the phase run's known-positive control for plan entry #8 (a 49-byte `.rs` fixture
holding the WEBVIEW2 variable name). Moved, not deleted:

- from `.andromeda/runs/2026-09-30T12-15-03Z-phase/ctl/pulse-app/x.rs`
- to `.andromeda/cache/phase-ctl-2026-09-30/ctl/pulse-app/x.rs` (`.andromeda/cache/` is gitignored — `.gitignore:8`)
- sha256 `08c19304faa822051fa2a151fecef0601b4a73e9d123f51570c46a2ec7e8d208`, 49 B; the whole `ctl/` directory moved
  as one (it held only this file)

Entry #8's `baseline` still cites the old path; the fixture's content is unchanged at the new one.

## #26 `gate.py hygiene` — exit 0

`hygiene: clean — read 31 (runs 28 · evidence 3) · trails 12 not read · binary 0 not read by P1`

## #27 `cargo xtask pre-push:linux` — exit 0, 13:37:22Z → 13:40:44Z

Verdict document `"verdict": "green"`, tree `2742ef7a4cc9ca04d035a35ae2b77401b1527c1b`; stages ok through `clippy`,
`test`, `ci-gates`. The `ci-gates` stage over the seeded record printed three arm NEUTRAL lines, then
`ci-gates: perf-budget NEUTRAL`.

## Pre-CI commit + #28 push — exit 0

`d708ad7 chore(2026-09-30-perf-budget-gate-reads-real-samples): operator pre-CI commit`, pushed
`fb93fca..d708ad7` to `origin/chore/migrate-pulse-to-v3`.

## #29 round-1 CI read — verdict RED

```
d708ad74e22b verdict: red · checks 13/13 · first-fail +1793 s release build (windows-latest) · wall 1793 s
runs: secret-scan#36723465704 pull_request completed/success · ci#36723465727 pull_request completed/failure
failed 1: release build (windows-latest) (failure)
```

### The Windows frame reading — 0 samples (the stop-and-return case)

Job `109914215821`, step `./target/release/xtask.exe perf:frame-sample`:

```
14:10:02Z perf:frame-sample: app=…\target\release\pulse-app.exe (built 1m ago) data dir <RUNNER_TEMP>\.tmpzSPPpU
14:11:07Z perf:frame-sample: app stopped, :4317/:4318 released
14:11:07Z perf:frame-sample: perf-budget: frame NEUTRAL — no metric.webgpu.frame_duration_ms record (required) FAIL
14:11:07Z perf:frame-sample: perf-budget: memory NEUTRAL — populated 0 of 5
14:11:07Z perf:frame-sample: perf-budget: snapshot NEUTRAL — no metric.snapshot.token_count_ms record
14:11:07Z perf:frame-sample: frame: 0 samples — no WebGPU adapter in this run
14:11:07Z perf:frame-sample: FAIL
##[error]Process completed with exit code 1.
```

The app reached :4317 and the leg waited the full 60 s first-frame window; no `feeding inject_demo` line, so the
injector never ran. Artifact `logs-perf-frame-Windows` (1261 records), read:

- `app.boot.webview.init {webview_backend: WebView2}` · `app.boot.gpu.check {wgpu_backend: dx12}` (the new shape).
- `app.boot.window.navigation` `navigated: true` for all four windows (compact-widget, main, findings, report).
- 185 `services.list_with_states.request` — the webview's script ran and its IPC reached the backend.
- 0 `metric.webgpu.frame_duration_ms`. No ERROR; WARNs only `interpretation.model.allow_root` (unconfined) and
  `interpretation.inference.error` (`model_not_configured`), both expected with no model configured.

What the log cannot separate: the product records no adapter-state event, so "WebView2 applied the flag set and
WebGPU still returned no adapter" and "WebView2 did not apply the flag set" read identically here.

Not re-run and not retuned (overseer's rule). #30–#32 are held.

### The Linux memory + snapshot gate — PASS

Job `109914215990` (lint / test ubuntu-22.04), `cargo xtask perf:budget --data-dir target/tmp/perf-budget-samples
--require memory,snapshot`:

```
perf-budget: graded 120 record(s) across 1 file(s)
perf-budget: frame NEUTRAL — no metric.webgpu.frame_duration_ms record
perf-budget: memory max 7680000 B <= 512000000 B (n=3, populated 3) PASS
perf-budget: snapshot p99 0.0 ms <= 500 ms (n=50) PASS
perf-budget: PASS
```

### Round shape and the release cache (cold first round of the new key)

| job | start → end | wall |
|---|---|---|
| release build (windows-latest) | 13:41:27 → 14:11:18 | 29 m 51 s (failure) |
| release build (macos-latest) | 13:41:30 → 14:05:24 | 23 m 54 s |
| coverage gate | 13:41:26 → 14:01:44 | 20 m 18 s |
| lint / test (windows-latest) | 13:41:26 → 13:59:27 | 18 m 01 s |
| lint / test (ubuntu-22.04) | 13:41:26 → 13:49:45 | 8 m 19 s |

- Both release jobs read `No cache found.` for the new `release-${{ runner.os }}` key and built cold: `Compiling`
  lines 757 (Windows) and 605 (macOS), against 16 at the base's warm run.
- macOS saved its key (`... Saving cache ...`). The Windows job failed, and its log shows no save, so
  `release-Windows` does not exist yet; a rerun of this sha would build Windows cold again.
- The two cold release jobs are this round's critical path (1793 s vs 1297 s at `fb93fca`).

Held here: #30–#32, pending the operator's decision on the frame arm.

## After the operator's decision — the phase fallback (`evidence/frame-gate-decision.md`)

Edits: `xtask/src/perf_budget.rs` (an empty, non-required frame arm prints the named cannot-evaluate line; its
existing pin asserts it), `.github/workflows/ci.yml` (the Windows release job's `inject_demo` build, frame step and
`logs-perf-frame-*` upload removed), `xtask/src/perf_frame.rs` (module doc: the dev-host frame gate).

Gate block re-run (entries #1–#11, #16–#25; #12/#13 skipped as slot entries, green earlier this run with no
pulse-app source change since): 20 green · 0 red · 1 recorded. Printed lines:

```
perf-budget: frame: cannot-evaluate: 0 samples, no WebGPU adapter in this run
perf-budget: memory max 7680000 B <= 512000000 B (n=3, populated 3) PASS
perf-budget: snapshot p99 0.0 ms <= 500 ms (n=50) PASS
perf-budget: PASS
```

`ci-gates` over the seeded record: `ci-gates: perf-budget: frame: cannot-evaluate: …` then `ci-gates: perf-budget
NEUTRAL`. Selector 17/17; workspace 2466/2466.

- #26 `gate.py hygiene` — `hygiene: clean`, exit 0.
- #27 `cargo xtask pre-push:linux` — exit 0, 14:25:12Z → 14:27:51Z, `"verdict": "green"`, tree
  `6ff31451f6bc7703d2d9a3682f700843e2ee3f30`; its `ci-gates` stage prints the named frame line then
  `perf-budget NEUTRAL`.
