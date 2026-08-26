# layouts extract

## No domain coverage

This chunk is a backend ingest/consumer-drain + obs-record chunk — its scope names only Rust anchors (`crates/ingest`, `crates/buffer`, `pulse-app/src/heartbeat.rs`, `pulse-app/src/observability.rs`) and its deliverable B observable targets the app's own obs log ("must not depend on a human reading a heartbeat"), so it creates/modifies no surface, region, component placement, focus order, breakpoint, modal, nav, or empty state; if a later chunk ever routes this wedge signal to a user-facing readout, its home would be the full-dashboard `ConnectionStatusLine` buffer-fill readout (per layout-templates §Component — Footer (read-only status bar) → Full dashboard), but that is out of this chunk's stated boundaries.
