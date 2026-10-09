
## 2026-09-30-p-027-discovery-bound — smoke:discovery, the fourth scenario leg; discovery-observer wiring trigger
**Section:** §3 → Per-chunk gate discipline (scenario legs) · §1 → Pending coverage triggers
**Change:**
- §3: new `cargo xtask smoke:discovery` as the **fourth SCENARIO leg** — not a gate-set member, dev-host only, not CI-wired, `xtask/src/discovery.rs`. It waits for the webview's `services.list_with_states.request`, starts `inject_demo --sustained --error-pct=0`, grades the first `metric.constellation.discovery_ms` at or after the first `duckdb.append {table_name: spans}` within 30 s: PASS iff interval ≤ 5000 ms AND anchor within 1000 ms of that append AND 0 `app.panic.fatal` AND 0 ERROR. INCONCLUSIVE on no spans append, no poll before it, or a data dir already holding a log family; a window below 21 s refused. Exit 0/1/2; artifact `target/discovery/`. Measured RED 15 219 ms / GREEN 177 ms.
- §1: new open trigger `discovery-observer-wiring-coverage` — `main.rs` composes `DiscoveryObserverAdapter` after the baseline adapter (order load-bearing), but the only order pin builds its own composite; owed: an assertion over the production composition (a seam out of `main.rs`).
**Why:** the chunk shipped a new scenario leg and a production wiring site proven only by that leg, the same one-time-proof class as the viz-read-connection and damper wiring triggers.
**Ref:** .andromeda/runs/2026-09-30T11-29-23Z-wrap/
