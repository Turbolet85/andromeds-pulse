# Adapter-record wire witness — dev host, operator slot granted 2026-09-30

The overseer granted the window slot ("4317/4318 free, no nvda/pulse-app/conductor running; conductor-builder is held").

## Binary freshness

The first release build (entry 25) had embedded a `ui/dist` built BEFORE the bindings regen (entry 17 ran ahead of
entry 31), so its bundled `ARGS_MAP` lacked `record_webgpu_adapter`. The count over `ui/dist/assets/*.js` was 1
occurrence, the caller's own property access only, so the fire-and-forget reporter would have returned silently. Entries 17 and 25
were re-run after the regen: `ui/dist` and `target/release/pulse-app.exe` then carry 2 occurrences each (map entry +
caller). Both legs below ran that binary (`perf:frame-sample` printed `built 2m ago`).

## `cargo xtask self-verify` (entry 27) — PASS

```
self-verify: binary D:\dev\projects\andromeda-pulse\target\release\pulse-app.exe
self-verify: launched PID=38788
self-verify: receivers ready (:4317 + :4318)
self-verify: shell health OK — 264 log lines; boot trio + window-shown + heartbeat present; zero panics
self-verify: clean quit — zero orphan
self-verify: a11y/contrast PASS
self-verify: PASS
```

## `cargo xtask perf:frame-sample` (entry 29, driven by hand) — exit 0

```
perf:frame-sample: first frame sample seen — feeding inject_demo --sustained for 30s
perf:frame-sample: app stopped, :4317/:4318 released
perf:frame-sample: log family preserved at D:\dev\projects\andromeda-pulse\target\perf-frame\2026-09-30T19-12-33Z
perf:frame-sample: perf-budget: frame p99 2.7 ms <= 33 ms (n=8045) PASS
perf:frame-sample: PASS
```

Frame PASS unchanged on the GPU host (predecessor: p99 2.6 ms, n = 7971).

## `ui.webgpu.adapter` records in the preserved log (entry 30: count `2`, exit 0)

```
{"fields":{…,"outcome":"obtained",…,"window_label":"compact-widget"},"level":"INFO","message":"webgpu adapter request recorded","target":"ui.webgpu.adapter","timestamp":"2026-09-30T19:12:34.698Z"}
{"fields":{…,"outcome":"obtained",…,"window_label":"main"},"level":"INFO","message":"webgpu adapter request recorded","target":"ui.webgpu.adapter","timestamp":"2026-09-30T19:12:34.806Z"}
```

One record per mounted canvas window, INFO on `obtained`, both fields unredacted (the exact leaf resolves), and no
field beyond `outcome` + `window_label` (the three identity fields are the formatter's defaults on every line).

## Teardown

After the leg: `:4317` / `:4318` both refuse (`connect_ex` → 10061, 10061); repo-owned `pulse-app` / `inject_demo` /
`msedgewebview2` processes: 0.
