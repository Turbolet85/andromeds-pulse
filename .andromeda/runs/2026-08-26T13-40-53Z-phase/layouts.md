# layouts extract

## No domain coverage

This chunk's in-scope surface is `pulse-app/src/llamacli_inference.rs` plus five security-doc artifacts (and possibly workspace `Cargo.toml`) — a Rust runtime guard, an argv bound, and doc consistency, with no desktop-webview or desktop-native surface, region, component placement, focus order, modal, or wireframe touched; the degraded-boot path (`ModelStatus::Error` → `ModelNotConfigured`) is explicitly bounded to backend behaviour by the scope's Boundaries, and layout-templates documents no model-status readout in any surface.
