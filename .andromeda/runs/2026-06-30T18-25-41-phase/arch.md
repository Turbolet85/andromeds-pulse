# arch extract

## Relevance — one line
Partial — pure webview frontend work; inherits Tauri 2.x shell and developer-tool surface design principles; adds no Rust crates, IPC, resources, or workspace boundaries.

## Constraints — domain rules that apply

1. **Tauri 2.x webview shell** (per §Established Decisions [Platform] Tauri 2 desktop shell) — the suppression is a webview/WebView2 DOM handler + CSS property, constrained by the host browser environment's native affordances.
2. **Webview WebGPU visualization surface** (per §Stack and Technologies + §Established Decisions [WebGPU Visualization Surface]) — the target canvas elements (`ConstellationCanvas`, halo surfaces) run WebGPU WGSL; suppression must not interfere with GPU rendering or the binary Arrow IPC data path.
3. **Developer-tool surface design philosophy** (per §Design Philosophy item 8) — the suppression reinforces the "dense, chart-first, low-chrome dashboard targeting developers" principle by removing meaningless browser chrome (Back / Refresh / Save-as / Print / Inspect).
4. **Production-gated code path discipline** (implicit from Cross-cutting Patterns [Feature-gate hygiene]) — the suppression is gated to production builds via `import.meta.env.PROD` (Vite); dev inner loop preserves right-click → Inspect for developer ergonomics.
5. **Webview IPC capability boundary** (per §Webview IPC capability policy) — a `contextmenu` event handler or CSS property does not require a new capability grant; the `pulse:default` capability already gates the webview's access to DOM/JavaScript APIs.

## Patterns to follow — existing patterns relevant to implementation

1. **Vite environment gating** — the codebase already gates features on `import.meta.env.PROD` (established Tauri + frontend build stack); apply the same pattern for `contextmenu` suppression and canvas drag suppression.
2. **Webview-scoped event handlers** — global `contextmenu` suppression via `preventDefault()` is established in the webview's root entry point (either top-level `App` effect, `main.tsx` bootstrap, or a shared `use-*` hook per the scope); no bridge to Rust.
3. **Canvas element styling discipline** — the constellation/halo canvas already exists and is styled in the webview's global/component CSS; drag-affordance suppression (`draggable={false}`, CSS `user-select: none`, `-webkit-user-drag: none`) follows standard HTML/CSS patterns for image elements.

## Anti-patterns to avoid — domain bans that apply

1. **No Inspect suppression in dev builds** — the right-click → Inspect flow is deliberately PRESERVED in development (scope: "the dev inner loop keeps right-click → Inspect/devtools"); conditional suppression is required, not blanket.
2. **No custom app context menu without explicit scope expansion** — the scope accepts "suppressed OR app-replaced" but this chunk SUPPRESSES (the lower-cost path); building a bespoke context menu is out of scope and would require a separate decision + design domain involvement.
3. **No new TauRPC / capability / env var** — scope explicitly excludes "No new TauRPC namespace, no `pulse-app/capabilities/` JSON entry, no broadcast topic, no env var, no workspace dep expected"; any handler is pure webview, not Rust-side.

## Contract bindings — where your domain ties into another

- **Tests domain** — verification method per scope is "webview" (matrix): vitest DOM test dispatching a real `contextmenu` event and asserting suppression in PROD mode + canvas drag/save affordance absence (scope §Acceptance).
- **(none other)** — no IPC, no new crate, no env var, no Rust changes implied; this is a webview-only change.

## Acceptance criteria contributions — concrete pass/fail checks your domain adds

1. (arch) Suppression is production-gated via `import.meta.env.PROD` (Vite); dev builds preserve right-click → Inspect behavior.
2. (arch) Canvas elements carry no `draggable` affordance or `user-drag` CSS property (either per-element or via global stylesheet); no browser "Save image as" / drag-to-save interaction exposed.
3. (arch) No new TauRPC procedure, capability, env var, or Cargo crate; webview-only implementation (per scope §Boundaries OUT of scope + Surfaces / contracts touched [No new workspace dep expected]).
4. (arch) Standard gates green: webview typecheck/lint passes (TauRPC-generated `.d.ts` + `tsc --noEmit`); vitest suite green; capability-drift clean (expected no-op, no new procedure).

## Relevant amendment history — prior amendments to your plan touching this chunk's area + why

(none) — the amendment history includes no prior amendments to webview chrome / context-menu / canvas affordance handling. **Related prior chunk:** P-061 (2026-06-29-window-geometry-movable-shell, chunk #61) added `window-geometry.json` filesystem location for remembered window position; that chunk also touched Tauri/webview shell configuration. P-064/P-065 (this chunk) are a separate paired quick-win in the same shell/UI category (Epoch 2 — Window & shell hygiene), but no architectural resource overlap.
