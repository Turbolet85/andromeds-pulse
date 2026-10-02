
## 2026-09-29-p-025-hue-shift-observable-made-gradable — P-025 hue interval stated at the IPC route
**Section:** §Occupied Resources → Tauri IPC routes → `telemetry.frontend.record_*` delegated-timing entry
**Change:** The `metric.constellation.hue_update_ms` clause now states its `duration_ms`: the paint instant minus the service's `ServiceListItem.tier_effective_at_unix_nano` (replayed by `triage::contract::tier_effective_at` — rise = the opening incident's `opened_at_unix_nano`, fall = the last max-tier holder's `resolved_at_unix_nano`), one record per service whose tier changed, emitted by `hueShiftSamples` only for changes witnessed after mount, no service id. Leaf name and fields unchanged.
**Why:** the chunk re-anchored the observable to the interval the P-025 budget bounds, per Conductor's measurement contract.
**Kept:** the `services.list_with_states` entry — the new `tier_effective_at_unix_nano` payload field is a field inside an already-registered procedure, which this registry does not enumerate (chunk #91's `priority_tier` join was never registered either).
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
