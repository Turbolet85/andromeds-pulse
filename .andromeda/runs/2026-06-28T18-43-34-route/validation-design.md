# Design validation — route draft

## Rewrite

- Epoch 3, "Legible labeled constellation" — Old: "per-dot service names with clear health/severity encoding" → New: "per-dot service names with health/severity encoding via Primary/Accent hues and Halo breathing".
  Reason: constellation must reference color-primary/color-accent (design-system §Color Palette) + the Halo State Pulse motion signature (§Brand Identity).

- Epoch 3, "Self-explaining empty states" — Old: "explain themselves with an actionable hint" → New: "show actionable hints with telescope icon and color-text-tertiary".
  Reason: empty states require the telescope icon glyph (design-system §Iconography) + tertiary text color (§Text Hierarchy).

- Epoch 3, "Plain-language connection status" — Old: "a human-readable services-connected, spans-per-second, and buffer-state line" → New: "human-readable connection line using font-label labels and font-data for metrics".
  Reason: must cite font-label + font-data (design-system §Typography) for the correct mono/sans hierarchy for telemetry display.

- Epoch 3, "Anomaly surfacing" — Old: "visually flagged and filterable" → New: "flagged with error-state border and color-accent".
  Reason: visual flagging requires the error-state semantic color (design-system §Semantic Colors).

- Epoch 2, "Window geometry + movable shell" — Old: "a working frameless-titlebar drag region" → New: "custom titlebar scaffold with drag region per layout-templates".
  Reason: the custom titlebar component (layout-templates §Component — Custom titlebar) must be referenced to align desktop-webview chrome.
