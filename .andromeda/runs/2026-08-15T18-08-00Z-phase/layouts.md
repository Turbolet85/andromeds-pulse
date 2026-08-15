# layouts extract

## No domain coverage

This chunk is a backend regression repair (storm→cue→incident seam in `crates/triage/`, `pulse-app/src/digest_runtime.rs`, workspace-key resolution) plus an obs-allowlist repair, adding no surface, wireframe, component placement, focus order, responsive behavior, modal, or navigation change — the only downstream user surface it feeds, the findings window incident list (layout-templates.md §Surface: desktop-webview → Primary screens), is populated with data by the fix but structurally unchanged by it, and the scope explicitly declares "No new capability surface" and bounds out P-075.
