# layouts extract — phase-36

## Chunk relevance

- **route#39 "Curation primitives — dedupe identical spans, anomaly highlight, critical-path extraction"** — out-of-domain. Backend-only curation logic in the `snapshot` crate; no surface element, no component placement, no focus order, no modal pattern, no responsive behavior. The chunk produces in-memory data structures consumed by the markdown formatter (chunk #41) and surfaced via the Investigate trigger (chunk #42) — neither is in scope for chunk #39.

## No domain coverage

Chunk #39 introduces curation primitives entirely within the `snapshot` crate Rust module surface. It does not render, modify, or wire any user-visible surface element. Per `layout-templates.md`:

- **Surface: desktop-webview** — chunk #39 does not touch any of the six primary screens (Compact widget / Traces / Metrics / Logs / Snapshots / Settings panel). The Snapshots view exists as a "list of generated snapshots" (§Primary screens) but its visual layout is owned by chunk #43 / a later viewer chunk per the §Notable surface-specific deferrals "Snapshot detail viewer ... deferred to downstream."
- **Surface: desktop-native** — chunk #39 does not touch the tray icon, tray menu, or notifications. The "Generate Snapshot" tray menu action (§Component — Tray menu) is an existing action item that will eventually invoke the TauRPC `snapshot.generate` command, but chunk #39 is upstream of that command's curation pipeline (chunk #41 builds the markdown formatter; chunk #43 wires workspace detection + clipboard + notification).
- **Investigation modal** (§Component — Investigation modal supporting moment) — its layout is already specified for the trace-detail-view use case; the snapshot-generation Investigation Capture Collapse motion + modal usage lands at chunk #42, not chunk #39.
- **No focus order, no tab sequence, no responsive breakpoint, no wireframe region** is introduced by chunk #39. All user-facing surfacing of the curation primitives' outputs is deferred to chunks #41 (markdown formatter), #42 (Investigate trigger + capture collapse modal), and #43 (workspace + clipboard + notification).

Layouts domain has nothing to extract for chunk #39. The orchestrator should rely on the `snapshot`-crate-internal contract bindings from arch / tests / obs / security extractors for this chunk, and surface layout concerns will re-engage at chunk #42 when the Investigate trigger button placement (widget / main / context-menu / trace-row) and the capture-collapse modal motion become in-scope.
