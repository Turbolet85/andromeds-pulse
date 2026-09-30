# RED leg at the chunk base — `cargo xtask smoke:discovery`

- **Run:** 2026-09-30T10:06:42Z → 10:07:29Z (UTC), port slot granted by the operator (overseer) for this run only;
  :4317/:4318 measured refusing before and after.
- **Binary:** `target/release/pulse-app.exe`, sha256 prefix `8e382bb2b8c16b68`, built 2026-09-30 12:05:39 +0200
  by `cargo build -p pulse-app --release` with the product source untouched (the tree differed from `71f3369` only
  in `xtask/src/main.rs` + the new `xtask/src/discovery.rs`; neither is linked into `pulse-app`).
- **Exit:** 1 (FAIL).

## Printed lines (verbatim)

```
smoke:discovery: app=D:\dev\projects\andromeda-pulse\target\release\pulse-app.exe (built 1m ago)
smoke:external-resolve: building inject_demo (outside the timed section)
smoke:discovery: data dir D:\dev\projects\andromeda-pulse\target\external-resolve\run-2956
smoke:discovery: webview polling — starting a healthy feed over real OTLP
smoke:discovery: first-sighting→dot interval_ms=15219 anchor_error_ms=14784 discovered_count=5
smoke:discovery: within the 5000 ms bound: no
smoke:discovery: log family preserved at D:\dev\projects\andromeda-pulse\target\discovery\2026-09-30T10-07-06Z
smoke:discovery: FAIL — the first-sighting→dot interval is 15219 ms (bound 5000 ms); the sample's anchor is 14784 ms from the first spans append (tolerance 1000 ms) — it measures a different interval
```

## Timeline read from the preserved log family (first record of each kind)

| UTC | record | fields |
|---|---|---|
| 10:07:08.269 | `services.list_with_states.request` | `item_count: 0` — the webview was polling first |
| 10:07:09.009 | `duckdb.append` | `table_name: spans`, `rows_appended: 27` — first sighting (A) |
| 10:07:23.792 | `triage.lifecycle.tick` | `services_bootstrapping: 5` — the registry's first tick |
| 10:07:23.792 | `triage.lifecycle.transition` | `unknown → bootstrapping`, `count: 5` |
| 10:07:24.228 | `metric.constellation.discovery_ms` | `duration_ms: 434.9`, `discovered_count: 5` — the paint (D) |

4 909 records, 0 `level: ERROR`, 0 `app.panic.fatal`, 1 discovery record.

## Reading

The dot appeared 436 ms after the registry's first lifecycle tick, and the tick came 14.8 s after the first spans
append. The discovery sample reports 435 ms because its anchor (`last_seen_unix_nano`) is the tick stamp, not the
first sighting. Measured 15 219 ms against the forecast of 10–16 s. The P-025 chunk measured 9 986 ms; the
difference is where the first sighting fell inside the 15 s tick period. Here it fell ~0.2 s after the heartbeat started (first tick 10:07:23.792 minus the 15 s period = 10:07:08.79), so nearly the whole period elapsed before the tick.
