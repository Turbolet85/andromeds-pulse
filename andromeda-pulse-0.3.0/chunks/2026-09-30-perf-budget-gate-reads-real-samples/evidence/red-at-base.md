# RED at the chunk base — `fb93fca`

Recorded at /implement Phase 1 step 1, before any edit. No new run: the operator slot was not requested for this
record. Every figure below is quoted from `research.md` (§CI measurements, §P4 frame-adapter probes), which
re-derived it at the chunk base.

## The CI perf gate passes over zero samples

- Run `ci#36710506171`, sha `fb93fca` (13/13 green).
- `logs-boot-Linux` artifact (`gh run download 36710506171 -n logs-boot-Linux`), parsed by target:
  - 23 records spanning 14 ms (`11:55:49.542Z → .556Z`);
  - 0 `metric.webgpu.frame_duration_ms` · 0 `metric.buffer.memory_bytes` · 0 `metric.snapshot.token_count_ms` ·
    0 `buffer.tick`.
- Job log (`gh api …/jobs/109870888182/logs`) `:1591-1594`: `perf-slo-check` prints three NEUTRAL lines (frame,
  memory, snapshot), then `ci-gates: perf-budget PASS`.

The mechanism: `run_ci_gates` (`xtask/src/main.rs:531-532`) prints PASS whenever the script exits 0, and the script
exits 0 when every arm stream is empty. The gate could not fail on that log.

## Frame-adapter probes (dev host, release binary sha256 `9e51d1d9…bf9ab4`, operator slots granted 2026-09-30)

| run | WebView2 flags | frame samples |
|---|---|---|
| control | none | 578 (`vulkan`/`webview2`) |
| probe 1 | `--use-webgpu-adapter=swiftshader --enable-unsafe-swiftshader` | 0 in 40 s |
| probe 2 | `--disable-gpu` | 0 in 40 s |
| probe 3 | `--enable-unsafe-webgpu --enable-features=Vulkan --use-vulkan=swiftshader --use-webgpu-adapter=swiftshader` | 496 in 2 s (p50 0.3 ms · p99 4.1 ms · max 14.6 ms) |

Limit (research): the host has a GPU and the frame label reads `vulkan` in the control and in probe 3 alike, so the
log cannot prove probe 3's adapter was SwiftShader. The discriminating measurement is a GPU-less hosted runner.

## GREEN

The listed gates re-derive GREEN after the change; the grader's ability to FAIL is proven by its per-arm pins
(required-empty → FAIL, unreadable → FAIL, over-budget → FAIL per arm), not by a re-run of this CI log.
