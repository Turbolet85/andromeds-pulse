# layouts extract

## Relevance
partial. The chunk creates or modifies no surface, region, focus order, modal or breakpoint. Its only layout contact is the frame-sample target (`metric.webgpu.frame_duration_ms`): the WebGPU canvas that layout-templates names as the frame-driving surface is recorded there as having no production render site.

## Constraints
- The WebGPU render loop at 60 FPS via `requestAnimationFrame` is specified on the Halo State Pulse canvas. Per layout-templates §Component — Halo State Pulse canvas (hero / signature), that canvas is MEASURED as not rendering on desktop-webview and DEFERRED to the next version by the operator ruling of 2026-08-29. A frame-sample producer must therefore not be assumed to live on that canvas. Whether any production render site emits `metric.webgpu.frame_duration_ms` today, and from which surface, is research's question.
- The live desktop-webview signature is the constellation dot hue on the compact widget and on the full dashboard Traces hero (per layout-templates §Surface: desktop-webview → Signature placement and §Primary screens). Any CI run that has to drive frames needs a window that mounts one of these surfaces. Which window the frame emitter actually runs in is research's question.
- Per layout-templates §Surface: desktop-webview → Signature placement, the halo-layer glow is exempt from the chrome motion budget while deferred. A frame budget sample must measure a surface that actually renders, never the deferred spec record.
- The snapshot-sample target sits behind `snapshot.generate`. Layout-templates reaches it from the Full dashboard (Snapshots view) and from the tray "Generate Snapshot" action (per §Primary screens and §desktop-native → Wireframe — Tray menu). Driving a snapshot sample through a UI path would open a window, and the scope's Operating constraints make that an operator slot. Whether the sample can be produced without a surface is research's question.
- No layout change is in scope. Per the scope's Boundaries (no new product feature), any emitter edit that P3 finds necessary must leave every wireframe region and component placement in layout-templates §Wireframe — Compact widget and §Wireframe — Full dashboard unchanged.

## Patterns to follow
- Before instrumenting a surface, probe its render site. Layout-templates §Component — Halo State Pulse canvas records the precedent: three probes (`<HaloCanvas` non-test hits, `Halo|halo` over `Dashboard.tsx` / `CompactWidget.tsx`) showed zero production render sites. Apply the same grep to whatever component the frame emitter is attached to.
- P-025's `metric.constellation.hue_update_ms` is the example of a timing observable placed on the surface that really renders (the constellation dot), not on the deferred canvas (per layout-templates §Component — Halo State Pulse canvas status note). A frame-duration producer, if P3 finds one is needed, follows that placement.

## Anti-patterns to avoid
- Do not attach a frame emitter to, or read frame samples from, the Halo State Pulse canvas. Per layout-templates §Component — Halo State Pulse canvas (hero / signature), it has no production render site, so such an emitter never fires and the gate reads NEUTRAL again: the vacuous class this chunk exists to close.
- Do not treat the ASCII wireframe halo labels as evidence of a live surface. Per layout-templates §Wireframe — Full dashboard notes and the Halo canvas status note, the sketches lag current truth and the status note governs.

## Contract bindings
- layouts ↔ obs: the frame-sample target `metric.webgpu.frame_duration_ms` (obs-plan §10 frame p99 ≤ 33 ms) must be emitted from a surface that layout-templates §Surface: desktop-webview records as rendering (the constellation dot on the compact widget or the dashboard Traces hero), not from the deferred Halo canvas.
- layouts ↔ tests/harness: a frame-producing CI or live leg must open a window that mounts that surface. The scope classes this as an operator slot, and the Windows host cannot run the webview headless.

## Acceptance criteria contributions
- (layouts) The frame-sample producer that the gate reads resolves to a component with at least one non-test production render site (a JSX mount under `pulse-app/ui/src` that is not a `vi.mock` factory or a comment). The Halo State Pulse canvas is excluded (per layout-templates §Component — Halo State Pulse canvas (hero / signature)).
- (layouts) No wireframe region, component placement or primary-screen composition changes in the chunk diff against `fb93fca` under `pulse-app/ui/src`, beyond an emitter hook on an already-rendered component (per layout-templates §Wireframe — Compact widget and §Wireframe — Full dashboard (Traces primary screen)).
