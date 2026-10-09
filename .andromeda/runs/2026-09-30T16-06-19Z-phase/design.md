# design extract

## Relevance
partial — only scope §B touches a design-governed surface (the WebGPU adapter-request path in `pulse-app/ui/src/canvas/webgpu-adapter.ts`, which feeds the canvas fallback); scope §A (snapshot timer) and §C (CI cache) are backend/CI and carry no design content. The chunk's stated deliverable in §B is a LOG record, not a visible UI change — any visible change would exceed scope.

## Constraints
- design-system §Surface: desktop-webview → §Component Patterns → Canvas Container requires that WebGPU initialization falls back to `<canvas>` with the message "WebGPU not supported in this browser" when `navigator.gpu` is undefined. Adding an adapter-result record to the `unavailable` arm must leave that user-facing fallback behaviour intact; whether the code already renders that exact message today is research's question (P3), not something this chunk should change.
- design-system §Surface: desktop-webview → Performance notes requires canvas rendering to stay outside React's render tree. The adapter-result record is a side emission of the adapter request; it must not pull canvas/adapter state into React render or add re-render churn on the constellation surface.
- design-system §Anti-Patterns → Per-Surface Bans forbids blocking the UI thread. The webview→backend adapter record (whatever TauRPC path P3 picks) must be fire-and-forget with respect to canvas init: the canvas (or its fallback) must not wait on the bridge call.
- design-system §Motion requires `prefers-reduced-motion: reduce` handling on every transition. This chunk should add no transition or animation; if one appears in the diff it is out of scope and falls under this rule.
- design-system §Surface: desktop-webview → CSP policy (`script-src 'self'`, no remote scripts) applies to any frontend code the adapter record adds: no external telemetry or diagnostic script.

## Patterns to follow
- Canvas fallback contract per design-system §Surface: desktop-webview → §Component Patterns → Canvas Container: the `unavailable` arm keeps its visible fallback; the new record only reports the cause alongside it.
- Error-state discipline per design-system §Surface: desktop-webview → Loading / Empty States (Error state): a static message, never the raw error. If any adapter-cause text ever reaches the UI, it must be a fixed string from the closed cause vocabulary, never the raw `requestAdapter()` exception text. (Scope keeps this log-only; this applies only if the plan surfaces it.)
- Canvas-outside-React per design-system §Surface: desktop-webview → Performance notes: emit the record from the adapter module (`webgpu-adapter.ts`), not from a React component's render path.

## Anti-patterns to avoid
- Replacing or suppressing the "WebGPU not supported in this browser" fallback while wiring the adapter record (design-system §Surface: desktop-webview → Canvas Container).
- Awaiting the backend adapter-record call before canvas init or fallback render, which blocks the UI thread (design-system §Anti-Patterns → Per-Surface Bans).
- Adding a new visible diagnostic surface (banner, toast, badge) for the adapter cause. That falls outside this chunk's log-only scope, and a new surface would need design tokens and a11y review (design-system §Anti-Patterns → Universal Bans: never reuse one layout for different information types).

## Contract bindings
- design ↔ obs: the adapter-request result that the design fallback branches on (`navigator.gpu` absent / adapter null or threw / obtained) is the same closed cause vocabulary scope §B asks to reach the app log. The fallback branch and the logged cause should come from ONE classification, so the visible fallback and the recorded cause cannot disagree (design-system §Surface: desktop-webview → Canvas Container ↔ obs-plan tracing allowlist).
- design ↔ a11y: none new. The existing fallback message falls under a11y §Contrast only if its rendering changes, and scope does not change it.

## Acceptance criteria contributions
- (design) With `navigator.gpu` undefined, the canvas container still shows the `<canvas>` fallback message "WebGPU not supported in this browser" after the change. The adapter record is added without changing the visible fallback (per design-system §Surface: desktop-webview → §Component Patterns → Canvas Container).
- (design) The adapter-result emission does not block canvas init. Canvas or fallback rendering does not await the webview→backend call (per design-system §Anti-Patterns → Per-Surface Bans).
- (design) The chunk's frontend diff adds no new visible UI element, color, animation or hardcoded hex/px value. If one does appear, it uses only design tokens and honors `prefers-reduced-motion: reduce` (per design-system §Motion and §Surface: desktop-webview → Tokens).
