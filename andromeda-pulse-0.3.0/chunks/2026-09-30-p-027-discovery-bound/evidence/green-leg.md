# GREEN leg after the fix — `cargo xtask smoke:discovery`

- **Run:** 2026-09-30T10:34:38Z → 10:34:46Z (UTC), in the operator-granted GREEN port slot; :4317/:4318 measured
  refusing before (gate entry 6) and after (entry 8).
- **Binary:** `target/release/pulse-app.exe`, sha256 prefix `9e51d1d92e80fdc0`, built 12:26:44 +0200 by gate entry
  4, newer than every fix source (`registry.rs` 12:08:35, `discovery_observer.rs` 12:09:06, `main.rs` 12:09:28).
- **Exit:** 0 (PASS).

## Printed lines (verbatim)

```
smoke:discovery: app=D:\dev\projects\andromeda-pulse\target\release\pulse-app.exe (built 7m ago)
smoke:external-resolve: building inject_demo (outside the timed section)
smoke:discovery: data dir D:\dev\projects\andromeda-pulse\target\external-resolve\run-20172
smoke:discovery: webview polling — starting a healthy feed over real OTLP
smoke:discovery: first-sighting→dot interval_ms=177 anchor_error_ms=6 discovered_count=5
smoke:discovery: within the 5000 ms bound: yes
smoke:discovery: log family preserved at D:\dev\projects\andromeda-pulse\target\discovery\2026-09-30T10-34-38Z
smoke:discovery: PASS
```

## Timeline read from the preserved log family

| UTC | record | fields |
|---|---|---|
| 10:34:39.838 | `services.list_with_states.request` | `item_count: 0` — the webview was polling first |
| 10:34:40.630 | `duckdb.append` | `table_name: spans`, `rows_appended: 27` — first sighting (A) |
| 10:34:40.807 | `metric.constellation.discovery_ms` | `duration_ms: 182.7`, `discovered_count: 5` — the paint (D) |

673 records, 0 `level: ERROR`, 0 `app.panic.fatal`. The leg ended before the registry's first 15 s tick, so no
`triage.lifecycle.tick` is in this family: the dot existed with no tick at all.

## RED → GREEN

| reading | RED (base `71f3369`) | GREEN (fix) |
|---|---|---|
| first-sighting→dot `interval_ms` | 15 219 | 177 |
| `anchor_error_ms` | 14 784 | 6 |
| exit | 1 FAIL | 0 PASS |

Forecast was ≤ ~1.5 s (one poll + paint); measured 177 ms, because the webview's 1 s poll happened to land shortly
after the first append.

## Second GREEN sample, and the tick fold live

The `smoke:hue-shift` run in the same slot (`target/hue-shift/2026-09-30T10-35-01Z/`, fresh data dir, same binary)
is an independent fresh boot and carries the same records:

| UTC | record | fields |
|---|---|---|
| 10:35:03.142 | `services.list_with_states.request` | `item_count: 0` |
| 10:35:03.810 | `duckdb.append` | `table_name: spans` — first sighting |
| 10:35:04.104 | `metric.constellation.discovery_ms` | `duration_ms: 300.0`, `discovered_count: 5` — 294 ms after A |
| 10:35:18.629 | `triage.lifecycle.tick` | `services_bootstrapping: 5` |
| 10:35:18.629 | `triage.lifecycle.transition` | `unknown → bootstrapping`, `count: 5` |

The five services were registered at first sighting, 14.8 s before the first tick. The tick found them already at
`Bootstrapping` and emitted no transition event for them, yet the aggregate still reads `count: 5`. That count comes
from the first-sighting fold (`take_first_sightings`), which the wire contract keeps identical to the base run's
`unknown → bootstrapping count 5` in `red-leg-at-base.md`.
