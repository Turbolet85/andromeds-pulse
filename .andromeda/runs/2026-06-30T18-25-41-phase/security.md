# security extract

## Relevance
partial — webview content surface is a declared threat vector; suppression is UI hygiene, not a security control, but affects the attack surface inherited from browser defaults.

## Constraints
1. Webview security posture is defined by CSP per §API Security; any inline event handler (contextmenu suppression) must comply with `script-src 'self'` and NOT use `eval()` or inline event attributes per §API Security CSP row.
2. All webview source code is first-party per Threat Model Summary Vector webview content; no external content, no CDN scripts, no inline `<script>` tags from untrusted sources.
3. Production vs. dev distinction must be enforced via `import.meta.env.PROD` (Vite) or equivalent so dev-build right-click → Inspect affordance is preserved per the acceptance bar; the suppression MUST be production-only per §Input Validation CLI / env var inputs row `ANDROMEDA_PULSE_L4_DETERMINISTIC` example (bounded truthy-parse, not unbounded string).
4. No new TauRPC procedures, capabilities, or env vars introduced — webview JS/CSS hygiene is client-side only; capability-drift check remains a no-op per §API Security TauRPC capability authorization xtask enforcement.
5. Canvas suppression (drag/save affordance) must not interfere with native copy/paste in form inputs (editable elements); neither contextmenu suppression nor canvas CSS overrides should trap focus or block keyboard selection per the a11y / UX acceptance bar.
6. Snapshot/clipboard surfaces are documented OTLP-attribute leakage paths per §Logging & Monitoring snapshot/clipboard/MCP-tool-response hygiene; this chunk does not sanitize canvas rendering (Arrow data is already scrubbed at buffer/corpus persistence boundaries per chunk #72 uniform scrubber). Canvas image-save suppression removes a user-facing leak vector but does not introduce a new scrubbing obligation.

## Patterns to follow
1. Production-gated suppression via `import.meta.env.PROD` with dev-build right-click preserved (mirrors P-061 window-geometry dev/prod pattern from the 2026-06-29 amendment).
2. CSS-based drag suppression (`user-select: none` / `-webkit-user-drag: none` on canvas elements) paired with JS contextmenu `preventDefault()` handler on document root; both are first-party mechanisms requiring no external dependency or runtime privilege escalation.
3. Global app-level contextmenu handler (root App effect or main.tsx bootstrap) covers both dashboard + compact-widget entry points per scope requirement ("app-wide").

## Anti-patterns to avoid
1. NEVER suppress context menu via Tauri capability manipulation or `pulse:default` IPC — the suppression is a webview-side DOM event handler, not an IPC procedure; adding a capability would be a scope drift.
2. NEVER log canvas contents, snapshot file contents, or clipboard payloads (per §Logging Anti-Patterns "NEVER log raw OTLP attribute values, ... snapshot file contents, clipboard contents").
3. NEVER expose canvas rendering state (buffer references, shader names, coordinate values) in error messages or console logs per §Error Handling sanitization + §Logging redaction rules.

## Contract bindings
(none) — pure webview/UI work; verification is webview (vitest + production-build right-click test), not cross-domain.

## Acceptance criteria contributions
- (ui) A `contextmenu` event in a production build raises no default WebView2 menu (vitest dispatches real event, asserts `preventDefault()` called).
- (ui) Canvas elements expose no image-save/drag affordance (vitest asserts no `draggable` attribute, checks CSS `user-select` + `-webkit-user-drag`).
- (ui) Dev builds preserve right-click → Inspect/devtools affordance (vitest runs in `import.meta.env.DEV` mode, asserts contextmenu handler is NOT installed).
- (a11y) Native copy/paste in form inputs (Settings form) still functional; no keyboard regression or focus trap introduced by global suppression.

## Relevant amendment history
(none) — 2026-06-29-window-geometry amendment touched adjacent webview UI surface but did not introduce new security boundaries. No prior amendments to the suppressed-context-menu or canvas-drag-affordance areas exist in the plan's history.
