
## 2026-09-30-p-027-discovery-bound — the scrubbed tap name now also keys the lifecycle registry
**Section:** §Security Anti-Patterns → Logging (the `extract_service_name` choke-point paragraph)
**Change:** `extract_service_name` still has THREE consumers (the `spans` column, the storm `FingerprintObserver`, the baseline `SpanObserver` tap). New: the tap drives one `CompositeSpanObserver` fanning each span to baseline · restart · the first-sighting `DiscoveryObserverAdapter`, which registers a baseline-admitted service in the lifecycle `ServiceRegistry` at its first span — so the tap's scrubbed name now also keys that registry directly, where before only `tick_all` inserted it from the baseline's own map. The desync argument now covers the lifecycle registry as well as the baseline registry.
**Why:** the tap's downstream fan-out grew by one observer while the choke point's callers did not, so a consumer count read off the function's callers would miss the new write path.
**Kept:** not a boundary widening — the registry receives the same scrubbed names it already received from `tick_all`, only earlier; no new input class and no new crossing.
**Ref:** .andromeda/runs/2026-09-30T11-29-23Z-wrap/
