# Slot legs — #12 to #15 (operator slot granted 2026-09-30 by the overseer)

Run back to back after the overseer measured 0 listeners on 4317/4318/14317/14318 and no nvda/pulse-app. Temp
data-dir paths are written `<TEMP>/…` (host paths stay out of committed evidence).

## #12 `cargo xtask self-verify` — gate tool, green (exit 0, 31.55 s)

```
self-verify: binary D:\dev\projects\andromeda-pulse\target\release\pulse-app.exe
self-verify: receivers ready (:4317 + :4318)
self-verify: shell health OK — 254 log lines; boot trio + window-shown + heartbeat present; zero panics
self-verify: clean quit — zero orphan
self-verify: a11y/contrast PASS
self-verify: PASS
```

## #13 frame-leg precondition — gate tool, green (exit 0, 4.16 s)

## #14 `cargo xtask perf:frame-sample` — fired by hand, 13:28:25Z → 13:28:58Z, exit 0

```
perf:frame-sample: app=D:\dev\projects\andromeda-pulse\target\release\pulse-app.exe (built 4m ago) data dir <TEMP>/.tmpL4YUjA
perf:frame-sample: first frame sample seen — feeding inject_demo --sustained for 30s
perf:frame-sample: app stopped, :4317/:4318 released
perf:frame-sample: log family preserved at D:\dev\projects\andromeda-pulse\target\perf-frame\2026-09-30T13-28-25Z
perf:frame-sample: perf-budget: frame p99 2.6 ms <= 33 ms (n=7971) PASS
perf:frame-sample: perf-budget: memory NEUTRAL — populated 0 of 3
perf:frame-sample: perf-budget: snapshot NEUTRAL — no metric.snapshot.token_count_ms record
perf:frame-sample: PASS
```

Preserved family read afterwards: 0 `ERROR`, 0 `app.panic.fatal`; 7971/7971 frame samples labelled
`wgpu_backend: vulkan`. The rebuilt binary's `app.boot.gpu.check` reads
`{"wgpu_backend":"dx12"}` with the message "compile-target default wgpu backend; no adapter probe runs here (the
frame loop's adapter branch is the adapter evidence)" — no `gpu_available` field. The record says `dx12` while the
frames say `vulkan`, which is the record-is-not-adapter-evidence finding seen live.

Limit, unchanged from research: this host has a GPU, so the log cannot prove the adapter was SwiftShader. The
discriminating reading is the hosted Windows runner (operator pass, round 1).

## #15 ps1 `ended` leg — fired by hand (the plan's `run` verbatim), 13:29:19Z → 13:34:01Z, exit 1

```
boot: ready (PID=33276, data_dir=<TEMP>/agent-run-ended-3f589dad-1406-4f0d-94bf-5f613e41a220)
  OTLP gRPC:    127.0.0.1:14317
  OTLP HTTP:    127.0.0.1:14318
{
  "ended": "exit -1",
  "last_write_age_seconds": 0,
  "log_file_basename": "agent-latest.jsonl.2026-09-30",
  "pid": 33276,
  "stale_after_seconds": 60,
  "verdict": "not-running"
}
```

`expect = ['exit 1', 'contains "ended": "exit']` — both hold. `exit -1` is the code `Stop-Process -Force` leaves
(`TerminateProcess` with -1). Most of the 4 m 42 s was `boot`'s warm release pre-build.

## After the slot (host census, 13:34Z)

0 `pulse-app` / `inject_demo` / `xtask` processes · 0 listeners on 4317, 4318, 14317, 14318 · 0 `powershell`
processes carrying `-EncodedCommand` (the boot wrapper exited with its app).
