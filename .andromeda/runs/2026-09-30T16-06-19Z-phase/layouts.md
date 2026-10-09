# layouts extract

## Relevance
partial — no surface, region, wireframe or focus order is created or changed. Only item B touches layout: the frontend WebGPU adapter-request path (`webgpu-adapter.ts`), whose `unavailable` arm feeds a specified fallback rendering on the desktop-webview canvas. Items A (snapshot timer) and C (CI cache) are outside the layouts domain.

## Constraints
- The adapter-result record is a diagnostic side-channel only. It adds no visible element, region or focusable control to any desktop-webview surface. The Compact widget and Full dashboard primary screens keep their current region set (per layout-templates §Surface: desktop-webview → §Primary screens; §Wireframe — Compact widget; §Wireframe — Full dashboard).
- layout-templates §Component — Halo State Pulse canvas → "Fallback (if WebGPU unavailable)" requires the canvas to show a static message when WebGPU is unavailable. Recording the adapter result must not replace, suppress or re-word that user-facing fallback. Whether the current `unavailable` arm renders this fallback today is research's question, because §Component — Halo State Pulse canvas is itself marked DEFERRED / not rendering and the live surface is the constellation dot.
- The cause vocabulary is diagnostic and internal (a closed enum reaching the app log). layout-templates does not specify it as user-visible copy, so no layout text binds it. Surfacing the cause in the webview, for example in the footer or on the canvas, would be a new layout element that no §Component section specifies. That makes it out of scope for this chunk (per layout-templates §Component — Footer (read-only status bar), which fixes the footer's content set).
- The Snapshots view's layout (list of snapshots with a token count, per §Primary screens → Full dashboard (Snapshots view)) does not change when the snapshot timer's span changes. Item A changes a measured duration, not a rendered field.
- The status note under §Component — Halo State Pulse canvas (DEFERRED, render half targeted at 0.4.0) governs. A frame-less-cause record must not be read as reviving or building the halo canvas layer.

## Patterns to follow
- Failure is not "no data": layout-templates §Component — Empty / error state requires a failure to be a distinct branch, checked before the empty branch, so it never masquerades as "no data". The adapter cause vocabulary follows the same logic: "no adapter" / "adapter null or threw" / "adapter obtained, zero frames" / "no record" stay distinct and are never collapsed into one undifferentiated state.
- Read-only, non-focusable diagnostic presentation: where any status is shown, it is read-only and affordance-exempt (per §Component — Empty / error state "No focusable element"; §Component — Footer read-only status bar). The chunk's record is log-only, which is consistent with this.

## Anti-patterns to avoid
- Do not show the raw adapter string or a raw error in any webview surface. §Component — Empty / error state bans raw `AppError` text in user-facing messages ("never the raw `AppError`").
- Do not add a new footer item, badge, or canvas overlay to carry the frame/adapter cause. The footer content set and the canvas fallback are fixed per §Component — Footer and §Component — Halo State Pulse canvas.

## Contract bindings
- layouts ↔ a11y: the WebGPU fallback text is the specified user-facing state when the adapter is absent (§Component — Halo State Pulse canvas → Fallback). Any change to the `unavailable` render path binds to a11y's text-contrast and never-color-alone rules. The fallback names `color-text-tertiary`, which §Component — Empty / error state notes is large-text-only (4.2:1). If the chunk touches that render path, a11y should check the fallback colour.
- layouts ↔ obs: the adapter-result record is an obs-domain log record. layouts contributes only the constraint that it stays out of the rendered layout.

## Acceptance criteria contributions
- (layouts) No desktop-webview region, component or focusable element is added or removed. A diff of `pulse-app/ui/src` against `ea50ca2` shows no new rendered JSX element or tab stop that carries the adapter/frame cause (per layout-templates §Surface: desktop-webview → §Primary screens).
- (layouts) When WebGPU is unavailable, the canvas's user-facing fallback behaviour is unchanged by the adapter-result recording. It is the same rendered state as at `ea50ca2`, and no raw adapter or error string is rendered (per layout-templates §Component — Halo State Pulse canvas → Fallback; §Component — Empty / error state).
