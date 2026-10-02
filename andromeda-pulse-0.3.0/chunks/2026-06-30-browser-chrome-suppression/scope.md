# Scope — Browser-chrome suppression

**Marker:** `2026-06-30-browser-chrome-suppression`
**Capability:** P-064 + P-065 (intent F4/F5 · Epoch 2 — Window & shell hygiene)
**Version:** andromeda-pulse-0.3.0
**Carried annotations:** none (the working entry had no `PREREQ:` / `CARRY:` suffix)

## Intent anchor (verbatim)

- **Working entry:** "Browser-chrome suppression — default WebView2 context menu and canvas image-save suppressed app-wide in production (P-064/P-065 · intent F4/F5)"
- **Intent F4 · Browser context menu leaks.** OBSERVED: right-click anywhere shows the default WebView2 menu (Back / Refresh / Save as / Print / Inspect) — zero app meaning. EXPECT: suppress the browser context menu app-wide (or replace with an app menu); no Inspect/Print/Save-as in a production build.
- **Intent F5 · Canvas image-save leaks.** OBSERVED: clicking the halo/canvas triggers the browser's save-image interaction. EXPECT: the canvas is not treated as a saveable/draggable browser image.
- **P-064 acceptance (matrix):** "A contextmenu event in a production build shows no default WebView2 menu (suppressed or app-replaced)." (method: webview)
- **P-064 observed_gap (matrix):** "Right-click anywhere shows the default WebView2 menu (Back/Refresh/Save-as/Print/Inspect) with zero app meaning."
- **P-065 acceptance (matrix):** "Canvas elements expose no image-save/drag affordance (no draggable image, no save-image on interaction)." (method: webview)
- **P-065 observed_gap (matrix):** "Clicking the halo/canvas triggers the browser's save-image interaction."

## What this chunk builds

Two paired webview-chrome suppressions so the app stops leaking browser-grade affordances that carry zero meaning in a desktop telemetry tool:

1. **Context-menu suppression (P-064)** — a right-click anywhere in a production build no longer raises the default WebView2 menu (Back / Refresh / Save-as / Print / Inspect). Suppressed app-wide across both windows (the `main` dashboard + the `compact-widget`).
2. **Canvas-not-an-image (P-065)** — the halo / constellation `<canvas>` surfaces are not treated as saveable/draggable browser images (no drag-to-save, no "Save image as").

The two are paired because the SAME WebView2 default-chrome family produces both symptoms: the global context-menu suppression removes the canvas "Save image as" entry, and a drag-affordance suppression on the canvas elements removes drag-to-save. Both are quick-win frontend hygiene (intent §5 "Quick wins: F1–F6 — mostly standard Tauri/webview config; high visible payoff per effort").

## Surfaces / contracts touched (expected)

- **Webview frontend root** (`pulse-app/ui/src/` — the app bootstrap / a top-level effect or a small dedicated hook) — a global `contextmenu` suppression, **gated to production** so the dev inner loop keeps right-click → Inspect/devtools. The exact host (a root `App` effect vs the `main.tsx` bootstrap vs a `use-*` hook) is a P3 read; both the dashboard and the widget entry points must be covered (app-wide).
- **Canvas component(s)** (the constellation / halo canvas — e.g. `ConstellationCanvas` + any sibling `<canvas>`) — drag/save-affordance suppression (`draggable={false}` + CSS `user-select`/`-webkit-user-drag: none`), and/or coverage by the global `contextmenu` handler.
- **Global stylesheet** (the webview's Tailwind/global CSS) — IF the canvas drag suppression is expressed in CSS rather than per-element props.
- **Production gate** — `import.meta.env.PROD` (Vite) or the equivalent, so the suppression is production-only per the acceptance "in a production build".

## Boundaries (explicitly OUT of scope)

- **Window geometry / movability / size constraints / close** → P-061 / P-062 / P-063 (all DONE).
- **Widget→dashboard navigation affordance** → P-066 (next chunk).
- **Replacing the suppressed menu with a real custom app context menu** — the acceptance permits "suppressed OR app-replaced"; this chunk SUPPRESSES (the lower-cost, intent-matching path). A bespoke app menu is NOT built here.
- **Dev-build right-click / devtools / Inspect** — deliberately PRESERVED in dev (the suppression is production-gated); this chunk does not degrade the dev inner loop.
- No new TauRPC namespace, no `pulse-app/capabilities/` JSON entry, no broadcast topic, no env var, no workspace dep expected — this is webview-JS + CSS hygiene; a JS `contextmenu` `preventDefault` listener and CSS need no Tauri capability grant (capabilities gate IPC procedures, not DOM event handlers). Confirmed-default unless P3 finds a Tauri/WebView2 config-level mechanism is preferable.

## Acceptance (the bar this chunk must clear)

- **P-064:** a real `contextmenu` event in a production build raises no default WebView2 menu (the installed handler calls `preventDefault` / suppresses it); dev builds are unaffected.
- **P-065:** canvas elements expose no image-save/drag affordance — no draggable image, no save-image on interaction.
- **a11y / UX:** the suppression must not break legitimate text interaction — native copy/paste/selection in editable fields (the Settings form inputs) must still work (via keyboard at minimum); no focus trap, no keyboard regression. (Whether to EXEMPT editable elements from the `contextmenu` suppression is a P4 scope-ambiguity.)
- All standard gates green (fmt/clippy on any Rust touched — expected none; webview typecheck/lint + vitest; capability-drift clean — expected no-op, no new procedure).
- Verification method is **webview** (matrix) for both caps — a vitest/DOM test that dispatches a real `contextmenu` event and asserts suppression (in PROD mode) + asserts the canvas carries no drag/save affordance. A live headful tauri-driver right-click may CARRY to the Epoch-4 e2e suite (P-076) if the in-chunk webview test cannot fully prove the packaged production-build behavior (mirrors the P-061/P-062 headful-delta CARRY precedent).

## P4 resolutions (val-1 reconciliation — appended at /phase P5)

Research (research.md) + the P4 user dialogue resolved the scope's open question + grounded the mechanism. Recorded here so this anchor matches `plan.md`:

- **Mechanism (research):** both windows render through the single `App.tsx` root → ONE `document`-level suppression hook (`use-suppress-browser-chrome.ts`) mounted in `App` is app-wide. **P-064** = a PROD-gated `contextmenu` handler; **P-065** = a global `canvas { -webkit-user-drag: none; user-select: none }` rule in `tokens.css` (covers all 5 canvas hosts) + a PROD-gated `dragstart`-on-canvas/img handler in the same hook (the contextmenu handler also removes the canvas "Save image as").
- **Net-new finding (research):** NO existing `contextmenu` / `import.meta.env` / `draggable` / `user-select` in the webview — the P2 extracts' "mirror the existing PROD-gate pattern" is INACCURATE; this is the first `import.meta.env.PROD` use (standard Vite, just unused until now). Build it fresh (no "already exists" surprise).
- **Q1 — contextmenu scope → "Exempt text fields" (user-confirmed at /phase P4):** suppress everywhere EXCEPT editable elements (`input` / `textarea` / `[contenteditable="true"]`) so native copy/paste survives in the Settings form (a11y SC 3.3.2 / 4.1.2 + design "text selection on non-text"). Blanket-suppress was the alternative (rejected — removes mouse copy/paste in form fields).
- **PROD-gate (settled by intent, not ambiguous):** the contextmenu/dragstart handlers are gated on `import.meta.env.PROD` per intent/matrix "in a production build"; dev keeps right-click → Inspect. The canvas CSS rule is always-on (harmless in dev).
- **Verification:** in-chunk **webview** vitest (real `contextmenu`/`dragstart` dispatch + `vi.stubEnv('PROD')`) is the OWNING affordance proof for both caps; the live headful tauri-driver right-click CARRIES to **P-076** (Epoch-4 e2e), mirroring the P-061/P-062 headful-delta CARRYs.
- **Boundaries unchanged:** P-061/062/063 (window — DONE), P-066 (widget→dashboard nav), bespoke app context menu (deferred — acceptance permits "suppressed OR app-replaced"; this chunk suppresses) all remain out of scope. No new TauRPC namespace / capability JSON / broadcast topic / env var / workspace dep (DOM handlers need no capability; window/IPC untouched).
