# design extract

## Relevance
partial — the chunk builds no rendered surface, token or component (a CI job, the `boot` verb's readiness, the app's exit evidence); design binds only where the cause or its closing change lands in the window, which the scope leaves open ("if the cause sits in the window alone").

## Constraints
- design-system §Surface: desktop-webview → Component Patterns → Canvas Container requires that every adapter request yielding no usable device (`navigator.gpu` undefined, a null adapter, a rejected adapter request, a failed device request) renders the shared fallback message. The plan states this as the target; whether the code already does so under the smoke's virtual display, and whether the process outlives that path, is research's question.
- design-system §Surface: desktop-webview → Platform-Specific Notes requires the Linux window to carry custom controls and both X11 and Wayland tray protocols; the plan holds NO requirement for a host with no tray host or no compositor. A boot on such a host is unspecified by design, so a behaviour chosen there by this chunk is a new decision, not a reading of the plan.
- design-system §Surface: desktop-native → Component Patterns → Tray Menu names "Quit" as the affordance that terminates the process, and design-system §Surface: desktop-webview → Component Patterns → Navigation / App Shell names the titlebar window controls. The plan specifies no surface that ends the process without a user action; a closing change must leave both affordances as specified.
- design-system §Surface: desktop-webview → Tokens requires every colour, spacing, radius, font, duration and easing value in webview source to come from the `@theme` custom properties. It applies only if the closing change edits webview source that renders.
- design-system §Motion requires that components render instantly with no entrance animation and that all motion respects `prefers-reduced-motion`. It applies only if the closing change alters what the window does at mount.
- design-system §Brand Identity records the Halo State Pulse glow layer as DEFERRED on every surface. A window-side change in this chunk neither builds it nor describes it as rendering.

## Patterns to follow
- An unavailable WebGPU device is a rendered state, not a failure of the window: the shared fallback in the canvas container (per design-system §Surface: desktop-webview → Component Patterns → Canvas Container).
- A window-side failure the user can see is the `EmptyState` error variant: a static message, never the raw error, checked before the empty branch (per design-system §Surface: desktop-webview → Component Patterns → Loading / Empty States).
- Charts draw directly to the canvas outside React's render tree (per design-system §Surface: desktop-webview → Performance notes); a fix at the canvas mount stays on that side of the boundary.
- The tray icon is drawn once at startup and updated on state changes, with no animation in the glyph (per design-system §Surface: desktop-native → Performance notes).

## Anti-patterns to avoid
- NEVER `alert()` / `confirm()` / `prompt()` to report a window-side failure; styled modals only (per design-system §Anti-Patterns → Per-Surface Bans → desktop-webview).
- NEVER block the UI thread with a dialog to report an end or an error (per design-system §Anti-Patterns → Per-Surface Bans → desktop-native).
- NEVER introduce a colour or a magic number outside the palette and scales while touching window code (per design-system §Anti-Patterns → Universal Bans).

## Contract bindings
- design ↔ tests harness: the boot smoke runs the window under a virtual display, so the canvas container's no-device path (design-system §Surface: desktop-webview → Component Patterns → Canvas Container) is the path the job most plausibly exercises; which path the runner takes is research's to read from the logs.
- design ↔ obs: the adapter request's outcome and the process end are recorded by obs-plan's records, not by this plan; design owns only what the window renders for each outcome.
- design ↔ a11y: `prefers-reduced-motion` is an app-wide token-bound mandate owned by a11y-plan §6 (cited from design-system §Motion → Accessibility); text contrast of any newly visible state binds to the a11y contrast derivation (design-system §Self-Validation Protocol → 6. Contrast Test).
- design ↔ layouts: no layout change is in scope; the window is neither removed nor rewritten in this chunk.

## Acceptance criteria contributions
- If the closing change edits webview source, the diff from the chunk base adds no hardcoded hex or pixel value; every value is a design token (per design-system §Surface: desktop-webview → Tokens).
- With no usable WebGPU device, the canvas container shows the shared fallback message (per design-system §Surface: desktop-webview → Component Patterns → Canvas Container); research states whether the smoke's host takes this path.
- No artifact of the chunk states that the Halo State Pulse glow layer renders (per design-system §Self-Validation Protocol → 3. Signature Test).
- Any state the window newly shows for a failure is a static-message `EmptyState` error variant, never the raw error text (per design-system §Surface: desktop-webview → Component Patterns → Loading / Empty States).
