# design extract

## Relevance
partial — the chunk is CI / harness / cache plumbing with no planned visual surface. Design binds only where scope item 1's producer-existence check leads to a product edit on the webview canvas path, which is where the `metric.webgpu.frame_duration_ms` sample would come from.

## Constraints
- The 0.3.0 frame-producing canvas is the constellation canvas carrying the dot severity hue. The Halo State Pulse glow layer is DEFERRED and has no render site on any surface (per design-system §Brand Identity → Signature element; §Motion → High-impact moments item 1). If P3 finds the frame target has no production producer, the emitter must go on the canvas that renders at HEAD, never on the deferred Halo layer. Whether a live frame emitter exists today, and where, is research's question.
- design-system §Surface: desktop-webview → Performance notes requires charts to render directly to canvas outside React's render tree, with Arrow data pushed through WebGPU. A frame sample stands for that canvas path, so any emitter or driver added for the frame arm must measure the canvas frame and not a React re-render. Whether the current emitter already measures the canvas frame is research's question.
- design-system §Motion states that canvas motion is a separate dimension from the chrome expression budget. The 33 ms frame budget is a canvas/data-viz budget, and chrome motion tokens (150 ms / 200 ms) are not the frame gate's subject. Do not conflate the two when choosing what a sample run drives.
- design-system §Motion → Accessibility makes `prefers-reduced-motion` an app-wide token-bound mandate. A sample-producing run must not add, alter or bypass motion tokens or the reduce-motion override to generate frames. It drives data into the canvas; it does not animate chrome.
- No new visual surface, token, color, typography or icon is in scope (scope §Boundaries: "No new product feature"). A product-code edit limited to an emitter adds no rendered output (per design-system §Anti-Patterns → Universal Bans: "NEVER use color purely for decoration", which applies by extension to any debug or perf overlay).

## Patterns to follow
- Keep canvas work outside the React tree, with data pushed into the canvas via WebGPU (per design-system §Surface: desktop-webview → Performance notes). A frame-timing mark belongs in the canvas render loop and not in a component render body.
- Snapshot generation stays asynchronous with no blocking modal (per design-system §Anti-Patterns → Per-Surface Bans: "NEVER block the UI thread with dialogs. Snapshots generate asynchronously"). If the sample run triggers a snapshot to feed the snapshot arm, it should use the existing async path.

## Anti-patterns to avoid
- Placing the frame emitter or driver on the deferred Halo State Pulse layer. It has no production render site, so the emitter would never fire and the frame arm would stay NEUTRAL, which is the vacuous-gate class this chunk exists to close (per design-system §Brand Identity → Signature element).
- Adding visible perf/debug chrome (an FPS counter, a sample overlay or a banner) to the webview in order to produce or show samples (per design-system §Anti-Patterns → Universal Bans; §Motion → Hard limits "NO canvas/WebGL" beyond the sanctioned canvas layers).

## Contract bindings
- design ↔ obs: the frame budget (frame p99 ≤ 33 ms, obs-plan §10) is measured on the design system's canvas render path (design-system §Surface: desktop-webview → Performance notes). The emitter site has to be the canvas that renders at HEAD.
- design ↔ a11y: any driven-frame run leaves the reduce-motion override intact (design-system §Motion → Accessibility ↔ a11y-plan §6 / SC 2.3.3).

## Acceptance criteria contributions
- (design) If a frame emitter is added or relocated, its call site is on a canvas with a non-test production render site. It is not on the deferred Halo layer (per design-system §Brand Identity → Signature element).
- (design) The chunk diff (base `fb93fca`) under `pulse-app/ui/**` adds no design token, color literal, typography, icon or visible element. Any UI-side delta is limited to a non-rendering timing emitter (per design-system §Anti-Patterns → Universal Bans).
- (design) No motion token or `prefers-reduced-motion` override is changed to produce samples (per design-system §Motion → Accessibility).
