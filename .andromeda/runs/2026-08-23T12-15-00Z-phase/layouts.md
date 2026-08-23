# layouts extract

## Relevance
Partial — the harness itself is out of my domain, but the chunk's canonical leg drives a layout-owned control (the custom titlebar's window-controls ✕) on the desktop-webview surface.

## Constraints
- The driven control sits in the **window-controls** flex segment of the custom titlebar; layout-templates.md §Component — Custom titlebar requires the segment order `[app-icon + title | grow | settings-button | window-controls]`, so the leg must target the right-end window controls and not the adjacent settings/aperture button (which opens the Settings modal per the same §).
- Window controls are **platform-conditional**: layout-templates.md §Component — Custom titlebar requires three DOM buttons (minimize / maximize / close) on the right for Windows / Linux, while macOS uses OS traffic-light chrome on the left. The leg is therefore Windows-host-bound by layout mandate, not merely by driver availability — consistent with the scope's note that `usePlatform()` returns `"windows"` here.
- The titlebar container is a **full-width drag region except the buttons** (layout-templates.md §Component — Custom titlebar, `data-tauri-drag-region`). A coordinate-based press that misses the button box drags the window instead of activating it, so the leg's press must be element-targeted or box-verified.
- The **same custom titlebar is required on two webview surfaces** — layout-templates.md §Wireframe — Compact widget and §Wireframe — Full dashboard (Traces primary screen) both specify it, and §Primary screens names compact widget and full dashboard as distinct windows. The harness must therefore state which window it attached to; whether the two surfaces' rendered titlebars are actually identical at HEAD is research's question.
- Close on the compact widget is **hide-to-tray, not exit**: layout-templates.md §IA notes → Navigation model requires the compact widget to be the always-on-top glance surface that minimizes to the tray, with the full dashboard reached by expanding it. The observable the leg asserts is a hidden window / collapse back to widget, never a process exit.
- layout-templates.md §Primary screens also lists the **findings** and **report** windows as *borderless* always-on-top webview windows. They carry no custom titlebar and therefore no ✕ target; a driver that attaches to "the real Tauri window" must not land on one of them and conclude the control is missing.
- layout-templates.md §Component — Custom titlebar fixes the segment inventory (app icon, title, settings button, window controls). No test-only affordance may be added to it to make the app drivable.

## Patterns to follow
- **One shared frameless titlebar across both webview surfaces** (layout-templates.md §Component — Custom titlebar; §Wireframe — Compact widget / §Wireframe — Full dashboard) — a single selector shape should work against either window, which is what makes the ✕ a *canonical* leg rather than a surface-specific one.
- **Platform-divergent chrome is already modelled** in layout-templates.md §Component — Custom titlebar (Windows/Linux DOM buttons vs macOS OS chrome). The driver's host-specificity mirrors an existing layout rule; it does not introduce a new one.
- **Measured-at-HEAD recording with an orphan check** — layout-templates.md §Component — Halo State Pulse canvas carries a MEASURED note that probed for a production render site before declaring a surface built/unbuilt. The scope applies the same check to `WindowControls`; this is the plan's established shape for writing a probe result back.
- **Keyboard activation is specified only for nav/modals** — layout-templates.md §Component — Primary navigation gives Tab / Enter-Space / Escape for tabs and sidebar, and §IA notes → Keyboard shortcuts gives `Esc` to minimize the widget or close a modal. The plan states no tab-order position for titlebar buttons, so a keyboard-driven variant of the leg has no layout-side ordering guarantee to lean on.

## Anti-patterns to avoid
- Adding a synthetic/test-only button, data attribute affordance, or extra segment to the titlebar to make it drivable — layout-templates.md §Component — Custom titlebar defines the segment inventory, and a fourth user-visible affordance would be a layout change the chunk explicitly does not want.
- Targeting macOS traffic-light buttons as DOM elements, or asserting three window-control buttons unconditionally — layout-templates.md §Component — Custom titlebar puts macOS controls in OS chrome.
- Asserting "the app quit" or driving via the titlebar drag surface — layout-templates.md §IA notes → Navigation model requires close to mean minimize-to-tray, and §Component — Custom titlebar makes the non-button titlebar area a drag region.

## Contract bindings
- **layouts ↔ a11y** — layout-templates.md §Component — Custom titlebar fixes the *visual* segment order; focus order for those buttons is unspecified in my plan and is owned by a11y (§Focus Order SC 2.4.3). If the leg presses via Tab+Enter rather than a targeted click, DOM/focus order becomes load-bearing and a11y owns that mandate.
- **layouts ↔ obs / desktop-native** — the close-to-tray signpost is a desktop-native notification trigger (layout-templates.md §Surface: desktop-native → §Component — Notifications, trigger #4, once per session, gated on `notifications_enabled`). Layouts owns only "close means hide"; the signpost/event observable is obs's.
- **layouts ↔ tests harness** — the E2E driver stack choice for desktop-webview is test-plan's; layouts contributes only the target's surface, region, and segment position.
- **layouts ↔ route implementation** — layout-templates-amendments.md 2026-05-02 handoff note assigns the drag region and window-control buttons to the route specialist, with the exact Tauri 2 minimize/maximize/close IPC owned by implementation; layouts supplies structure only.

## Acceptance criteria contributions
- (layouts) The pressed control is the close button in the window-controls segment at the right end of the custom titlebar — not the settings button and not the drag region (per layout-templates.md §Component — Custom titlebar).
- (layouts) The leg's assertion is that the webview window becomes hidden (widget → tray, dashboard → collapse to widget), never that the process exited (per layout-templates.md §IA notes → Navigation model).
- (layouts) The harness records which desktop-webview surface it attached to — compact widget or full dashboard — since both are specified to render the same titlebar, and must not attach to the borderless findings/report windows, which have no titlebar control (per layout-templates.md §Primary screens, §Wireframe — Compact widget, §Wireframe — Full dashboard).
- (layouts) No new focusable element or test-only affordance is added to the titlebar segment inventory to enable driving (per layout-templates.md §Component — Custom titlebar).

## Relevant amendment history
- **2026-06-29-predictable-close-self-verify** — added the first-window-close-to-tray signpost as notification trigger #4 (§Component — Notifications). This is the exact close path the leg exercises; it is why "close" is a documented tray-minimize affordance rather than an exit, and it is the same marker under which the scope says `core:window:allow-close` was granted.
- **2026-08-21-delegated-timing-observables** — recorded the Halo State Pulse canvas as SPECIFIED-BUT-UNBUILT after a three-probe orphan check for a production render site, and named the working-route entry that owns build-or-retire. Relevant twice over: it is the precedent for the scope's "has a production render site" check on `WindowControls`, and it is the plan's pattern for writing a HEAD measurement back into a §Component note.
- **2026-07-10-incidents-floating-window-disclosure** — added the findings and report borderless always-on-top windows to §Primary screens. Relevant because "attach to the real Tauri window" is now ambiguous: more than two webview windows can exist, and the new ones carry no titlebar ✕.
- **2026-05-02 initial generation (handoff notes)** — recorded the custom titlebar ownership split: design owns layout structure + token references, implementation owns the Tauri 2 minimize/maximize/close IPC. Sets the boundary for what this chunk may change if a probe demands drivability tweaks.
