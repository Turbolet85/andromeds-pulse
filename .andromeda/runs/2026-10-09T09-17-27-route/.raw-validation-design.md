# Design validation — route draft

## Rewrite
- `Window retired`: "Tauri shell, webview interface, IPC bridge, tray, updater and desktop bundles leave; design, layout and a11y masters state no interface this version" → "Tauri shell, webview, IPC bridge, tray, updater, bundles leave; design, layout, a11y masters state no interface this version, the deferred signature's fate"
  Reason: design-system §Brand Identity → Signature element (echoed in §Motion, §Self-Validation #3 and layout-templates §Component — Halo State Pulse canvas) pins the glow layer as deferred to the next version and not deleted from the spec, and residual `2026-08-29-halo-state-pulse-signature-deferred` targets 0.4.0, which this chunk leaves with no surface to render it. No tokens, brand, scaffold or primitives chunk is owed, because this chunk retires both design masters whole and no later chunk consumes a token.

- `System notification on a change of state`: "carrying the short report" → "carrying the short report, its terse form stated"
  Reason: design-system §Surface: desktop-native → Notifications and layout-templates §Component — Notifications (OS-native) are the only authority on notification form, and both leave whole at `Window retired` five epochs earlier, so the five-part report arrives against a terse-length rule that has no stated successor.
