# design extract

## Relevance
partial — the chunk is a Linux boot/env-lever change (no new UI, tokens or motion), but the lever it may apply (a WebKitGTK renderer posture such as disabling the DMA-BUF renderer) changes the compositing path under every desktop-webview surface, so the design system's "what must still render" mandates bind as regression guards on Linux.

## Constraints
- design-system §Surface: desktop-webview → Component Patterns → Canvas Container requires that any adapter outcome that yields no usable WebGPU device (`navigator.gpu` undefined, null adapter, rejected adapter request, failed device request) render the shared `<canvas>` fallback with its message, never a blank surface. If the chosen lever changes WebGPU availability on Linux, the fallback must engage. Whether the lever changes adapter availability under WebKitGTK at all is research's question.
- design-system §Self-Validation Protocol → 3. Signature Test requires the 0.3.0 severity signature, the constellation DOT hue (`severityToHueFraction`, LCH Earth Blue ↔ Alert Burgundy), to render on both the full dashboard constellation map and the compact widget constellation canvas. A Linux launch posture must not remove it from either surface.
- design-system §Brand Identity (Signature element) and §Motion item 1 put the Halo State Pulse glow layer in DEFERRED status for 0.3.0. The chunk must not build it. Any doc or log text written for the remedy (e.g. the "document" shape, or a posture boot record) must not claim that the glow layer renders. The chunk's scope already lists the Halo glow layer as out of scope.
- design-system §Surface: desktop-webview → Platform-Specific Notes (Linux) requires custom right-aligned window controls on the frameless window, and support for both X11 and Wayland. A Wayland-specific remedy must keep the X11 path working and must not fall back to OS-drawn decorations. Whether a fallback lever touches decorations or the GDK backend choice is research's question.
- design-system §Surface: desktop-webview → Component Patterns → Navigation / App Shell requires the custom 32px titlebar with its drag region on both the compact widget and the full dashboard. A renderer or backend lever must not regress that chrome on Linux.
- design-system §Anti-Patterns → Universal Bans: if a backend posture degrades rendering, the remedy must not compensate with gradient or glassmorphic chrome. Flat, matte surfaces stay the discipline.

## Patterns to follow
- The shared WebGPU-unavailable fallback is the single degrade path (per design-system §Surface: desktop-webview → Canvas Container). Reuse it rather than adding a Linux-specific "renderer disabled" UI state.
- Graceful degradation that keeps semantics intact: the dot-hue severity signature is the live signature in 0.3.0 (per design-system §Self-Validation Protocol → 3), so a degraded renderer posture is acceptable only if that signature survives.
- If the remedy ships as "document" with a user-facing override, keep it out of the chrome. The design direction is "invisible-until-summoned UI, minimal chrome" (per design-system §Brand Identity → Design direction and §Depth Strategy rationale), so the workaround belongs in docs/README, not in a persistent in-app banner. Whether any in-app surface is wanted at all is a layouts/P4 question, not a design mandate.

## Anti-patterns to avoid
- Claiming in any durable text that the Halo State Pulse glow renders on any surface while it is deferred (per design-system §Self-Validation Protocol → 3 and §Brand Identity Signature element).
- Answering a rendering-path problem with decorative compensation: gradient overlays, glass or blur chrome (per design-system §Anti-Patterns → Universal Bans).
- A remedy that leaves the canvas blank (no fallback message) when WebGPU becomes unavailable under the new posture (per design-system §Surface: desktop-webview → Canvas Container).

## Contract bindings
- design ↔ obs: the posture boot record (scope §Surfaces likely touched) carries a closed label only. It is not a design surface, but a label that names the "glow layer" would conflict with the deferred status in design-system §Brand Identity.
- design ↔ a11y: the WebGPU fallback message and the dot-hue signature carry the not-colour-alone and contrast bindings (a11y SC 1.4.1 / 1.4.3). They are unchanged by this chunk, but a Linux renderer lever that alters colour output (e.g. a software path) re-opens that binding only if the hue rendering changes. Whether it does is research's question.
- design ↔ tests/harness: whether a WebGPU adapter is obtained under the chosen posture is observable through the existing `ui.webgpu.adapter` record (per design-system §Surface: desktop-webview → Canvas Container, the four `unavailable` causes). The real-display measurement launches can read it to confirm the canvas path taken.

## Acceptance criteria contributions
- (design) Under the shipped Linux posture on the NVIDIA + Wayland host, both the compact widget and the full dashboard render the constellation with the severity dot hue present (per design-system §Self-Validation Protocol → 3. Signature Test).
- (design) Under the shipped posture, the canvas either obtains a WebGPU adapter or shows the shared "WebGPU not supported" fallback. It is never blank (per design-system §Surface: desktop-webview → Component Patterns → Canvas Container).
- (design) The frameless custom titlebar (32px, right-aligned window controls, drag region) still renders on Linux under the shipped posture, with no OS decorations substituted (per design-system §Surface: desktop-webview → Platform-Specific Notes and Navigation / App Shell).
- (design) No doc, README or log text added by the chunk claims that the Halo State Pulse glow layer renders (per design-system §Brand Identity Signature element).
