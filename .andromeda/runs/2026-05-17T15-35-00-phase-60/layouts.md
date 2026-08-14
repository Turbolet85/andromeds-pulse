# layouts extract — phase-60

## No domain coverage

Chunk #64 is out-of-domain for layouts. Reason: pure backend in-memory state machine inside `crates/triage/baseline/` — per-service 24h rolling histogram + ServiceWentSilent gating logic. Route #64 detail explicitly confirms zero surface impact: TauRPC delta = none, broadcast topics delta = none, workspace deps delta = none, arch registry delta = none. The `ServiceWentSilent` cue (when emitted) flows through the pre-existing `pulse://stream/attention-cues` broadcast wired in chunk #62, so even the cue-to-webview transport is established prior infrastructure — no new wireframe region, no new component placement, no new focus order, no new modal pattern, no responsive breakpoint concern. Persistence sub-scope is deferred per user ordering decision (in-memory-only this phase), so no Settings modal / config surface changes either.

Per layout-templates.md §Surface: desktop-webview and §Surface: desktop-native, neither the compact widget, full dashboard, nor tray icon surfaces require structural revision for this chunk. Any user-visible consequence of activity floor learning (e.g., a ServiceWentSilent attention cue surfacing in the widget badge area or footer band) is governed by the existing cue-rendering layout established when `pulse://stream/attention-cues` was first wired — chunk #64 only changes WHICH cues fire, not WHERE they render.

Continue к other specialists' extracts.
