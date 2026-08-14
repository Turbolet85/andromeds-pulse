# layouts extract — phase-58

## No domain coverage

Chunk #62 "Attention cue emitter" is a backend-only chunk implemented entirely within the `triage` crate (background tick task evaluating EWMA / t-digest / RollingWindow thresholds and emitting `AttentionCue` to a tokio broadcast channel + tier-2 cues to a `cadence-triggers` channel for chunk #72). It introduces:

- A background tick task (1-2s interval) in `crates/triage/`
- Threshold evaluation logic (3.0× error rate multiplier, 2.5× latency multiplier)
- `AttentionCue` broadcast emission with `PriorityTier` classification (Hard / Medium / Baseline)
- A new internal broadcast channel `cadence-triggers` for downstream consumption by chunk #72 Cadence Coordinator

**No surface elements are created or modified.** The chunks-being-planned context confirms this explicitly: "NO UI surfaces are touched in this chunk. UI consumption of AttentionCue arrives in later v0.2.0 chunks (likely chunks #80+)."

Layouts plan (`.andromeda/layout-templates.md`) covers two surfaces (desktop-webview compact widget / full dashboard / settings modal / trace data table / investigation modal; desktop-native tray icon / tray menu / notifications / file picker). None of these surfaces consume `AttentionCue` at chunk #62 scope — that wiring is deferred to later v0.2.0 chunks per the route plan.

Cross-domain bindings that layouts would normally flag (focus order → a11y SC 2.4.3, modal patterns → a11y focus trap, responsive breakpoints → design spacing scale) have no applicable surface in this chunk.

When UI consumption of `AttentionCue` arrives (chunks #80+ per v0.2.0 plan — e.g., compact-widget badge state encoding, full-dashboard cue banner, tray-icon notification triggers), layouts extracts will become relevant at that time.
