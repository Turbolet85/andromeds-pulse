# Frame gate — the phase fallback (operator decision, 2026-09-30)

Decided by the overseer (founder-delegated) after round 1's frame reading came back 0; this record carries it into
the plan evidence. `plan.md` is not edited.

## The measurement that decided it — `ci#36723465727` (sha `d708ad7`)

- Job `109914215821`, release build (windows-latest), step `./target/release/xtask.exe perf:frame-sample` under
  `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--enable-unsafe-webgpu --enable-features=Vulkan --use-vulkan=swiftshader
  --use-webgpu-adapter=swiftshader` (child env only).
- **0 frame samples.** The app was healthy: it accepted on :4317, all four windows navigated, and the webview polled
  `services.list_with_states` 185 times. The leg waited its full 60 s first-frame window. 1261 records, 0 ERROR.
- **Indistinguishable here:** "WebView2 applied the flag set and WebGPU still returned no adapter" versus "WebView2
  did not apply the flag set". The product records no adapter-state event (that gap goes to the wrap's route-resolve
  as a candidate, not this chunk).
- Full reading: `evidence/operator-pass.md` §#29.

## What ships

- **The frame budget gate is `cargo xtask perf:frame-sample` on the GPU dev host** — plan entry #14, green at this
  chunk: `frame p99 2.6 ms <= 33 ms (n=7971) PASS`, exit 0 (`evidence/slot-legs.md` §#14). Not CI-wired.
- **On CI, the frame arm is a named cannot-evaluate line**, never a PASS and never silent: wherever the frame arm
  reads empty and is not required (lint-test Linux `perf:budget`, `ci-gates`), it prints
  `perf-budget: frame: cannot-evaluate: 0 samples, no WebGPU adapter in this run`. Pinned in
  `xtask/src/perf_budget.rs::tests::frame_absent_is_neutral`.
- **Memory and snapshot stay hard-required** on CI: `cargo xtask perf:budget --data-dir target/tmp/perf-budget-samples
  --require memory,snapshot` in lint-test Linux, unchanged.
- **The Windows release job carries no frame boot step** — no `inject_demo` build, no `perf:frame-sample`, no
  `logs-perf-frame-*` upload, so no 60 s wait. It keeps its own `release-${{ runner.os }}` cache key.

## Consequences for the plan's letter

- The acceptance line "(obs, CI) … the release Windows job's `perf:frame-sample` prints a frame line with n ≥ 1" is
  replaced by this decision: the frame arm on CI is the named cannot-evaluate line, and the frame acceptance rides
  the dev-host leg.
- Plan entries #14 (dev-host frame leg) and #11 (release `inject_demo` build it needs) stand as dev-host entries.
