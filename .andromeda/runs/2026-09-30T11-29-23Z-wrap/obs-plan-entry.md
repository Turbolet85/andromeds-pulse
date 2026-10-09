
## 2026-09-30-p-027-discovery-bound — the P-027 discovery_ms anchor stated in §8
**Section:** §8 → the delegated timing leaves → `metric.constellation.discovery_ms`
**Change:** was fields only (`duration_ms`, `discovered_count`, the service-label ban); now also the anchor — `duration_ms` = paint instant − `ServiceListItem.last_seen_unix_nano`, stamped at a brand-new service's FIRST SIGHTING by the first-sighting span observer (`DiscoveryObserverAdapter` → `ServiceRegistry::register_first_sighting`, before the heartbeat's first 15 s tick), so the first-appearance sample measures first-seen-to-dot, graded by `cargo xtask smoke:discovery`; before, the tick's `last_seen` stamp hid the wait (435 ms sample over a 15 219 ms true interval). A service re-entering liveness is anchored on a tick-refreshed `last_seen`. The registration emits no record (§11): first sightings fold into the tick's `triage.lifecycle.transition {unknown → bootstrapping, count}` aggregate.
**Why:** the §8 leaf named the observable without its anchor while its P-025 sibling states both, so it could not tell the tick-anchored metric from the fixed one.
**Kept:** the field set, the 10_000 clamp and the service-label ban.
**Ref:** .andromeda/runs/2026-09-30T11-29-23Z-wrap/
