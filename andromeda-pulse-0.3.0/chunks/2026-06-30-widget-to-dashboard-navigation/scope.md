# Scope — Widget-to-dashboard navigation

**Marker:** `2026-06-30-widget-to-dashboard-navigation`
**Capability:** P-066 (`andromeda-pulse-0.3.0/verification-matrix.json#P-066`)
**Intent:** F6 — No widget→dashboard navigation (Epoch 2 — Window & shell hygiene)
**Version:** andromeda-pulse-0.3.0

## The gap (OBSERVED → EXPECT)

- **OBSERVED:** the full dashboard (`main` window — Traces/Metrics/Logs/Snapshots/Settings) is reachable ONLY
  via the tray "Open …" menu item. The small glance widget (`compact-widget` window) has **no in-app affordance**
  to open it.
- **EXPECT:** an explicit in-app affordance (button/click) in the glance widget that expands/opens the widget
  into the full dashboard. Activating it **opens or focuses** the `main` dashboard window.

## What this chunk builds

1. A single, visible, accessible affordance inside the **compact-widget** webview (`CompactWidget.tsx`) — an
   "expand / open dashboard" control — that the user can click (and reach by keyboard).
2. The wiring that, on activation, **shows + focuses the `main` dashboard window** (which may be hidden after a
   close-to-tray per P-063, minimized, or already visible — the action is idempotent: "open if hidden, focus if
   shown"). The exact mechanism (direct `@tauri-apps/api` window call from the widget vs. a pulse-app IPC
   procedure) + the capability grant it requires are a **P4 decision** (see Open questions) — this is a
   negative-default Tauri surface (cf. the P-061 lesson: `core:window:*` ops are silently rejected under
   `core:default` unless explicitly granted).
3. Whatever capability / permission entry the chosen mechanism requires, granted explicitly with a stated
   rationale, so the affordance is not a dead control (silently-dropped IPC).

## Boundaries / non-goals

- **NOT** the reverse direction (dashboard → collapse-to-widget) — F6 is widget→dashboard only.
- **NOT** removing or changing the existing **tray "Open …"** path — it stays as the honest tray surface
  (arch §Tray icon policy); this chunk ADDS an in-widget affordance beside it.
- **NOT** a layout/visual redesign of either window — only the new affordance is added (it must respect
  design-system tokens + layout-templates for the widget, and a11y focus/label/keyboard rules).
- **NOT** changing the two-window topology, window geometry/size (P-061/P-062), or close behavior (P-063).
- **NOT** building the Epoch-4 headful e2e for this affordance — the matrix `e2e` acceptance is owned/verified
  by this chunk per affordance-honesty (a real activation must be exercised), but the broad integration-UX
  suite is P-076.

## Surfaces & contracts likely touched (confirm in research/plan)

- `pulse-app/ui/src/widget/CompactWidget.tsx` — host of the new affordance.
- A window-management call path to show/focus `main` — either `@tauri-apps/api/webviewWindow`
  (`getByLabel("main")` → `.show()` + `.setFocus()`/`.unminimize()`) gated by `core:window:*` permissions on the
  **compact-widget** capability, OR a new pulse-app TauRPC procedure (heavier: router + capability JSON +
  arch §Occupied Resources + `xtask` EXPECTED_PROCEDURES + the `emit_taurpc_bindings` test).
- `pulse-app/capabilities/*.json` — the explicit grant for whichever mechanism is chosen (negative-default).
- `pulse-app/src/tray.rs` / `pulse-app/src/window.rs` — reference for how `main` is currently shown/focused
  from the tray, to reuse the same show/focus semantics.

## Acceptance (from verification-matrix P-066)

> Activating the in-widget affordance opens or focuses the full dashboard (`main`) window.

Affordance-level (P-066 `method: e2e`): a **real activation** of the in-widget control (click / keyboard) must
drive the dashboard window to shown+focused — not a process-proxy. Exact in-chunk proof vehicle (webview vitest
with a mocked window API + a real event dispatch, plus the P-078 self-verify boot smoke; full headful
tauri-driver folded into P-076) is a P4 decision.

## Open questions for P4

- **Q1 (mechanism + capability):** direct `@tauri-apps/api` window call from the widget (lighter — just a
  `core:window:*` grant on the compact-widget capability) vs. a new pulse-app TauRPC procedure (heavier — full
  namespace binding chain). Lean: the lighter `core:window` path mirrors P-061/P-063 and avoids a new TauRPC
  namespace, but research must confirm the compact-widget capability can reach the `main` window's ops.
- **Q2 (affordance shape + placement):** what the control looks like and where it sits in the widget
  (design-system + layout-templates owned).
- **Q3 (show/focus semantics):** behavior when `main` is hidden (post-close-to-tray) vs. minimized vs. already
  visible — confirm the reused tray/window show+focus+unminimize sequence.

## Resolution + premise correction (recorded P4/P5 — 2026-06-30)

- **Premise correction (RESEARCH-CORRECTS-INTENT, P-061/P-063 family):** research confirmed the missing button
  AND surfaced a deeper unstated mechanism — the cross-window show/focus path already exists
  (`use-keyboard-shortcuts.ts::toggleCompactDashboard`, the Cmd+Shift+P handler) but is **silently broken**:
  `default.json` never granted `core:window:allow-show`/`allow-set-focus`/`allow-hide` (and `core:default` is
  getters-only). The OUTCOME (an in-widget affordance opens the dashboard) is unchanged; this chunk must also
  grant the missing `core:window` permissions (which repairs the dead toggle for free). See `research.md`.
- **Q1 (mechanism) — RESOLVED:** direct `@tauri-apps/api/webviewWindow` call + `core:window:*` grant (codebase
  precedent; the titlebar window controls + the existing toggle all work this way). NOT a new TauRPC procedure.
- **Q2 (placement) — RESOLVED:** a conditional titlebar icon-button on the compact-widget (mirrors the existing
  Investigate/Settings buttons), `<button aria-label="Expand to dashboard">` (the label a11y-plan §P5 names).
- **Q3 (expand semantics) — RESOLVED (user P4 choice "Expand & hide widget"):** click → `main.show()` +
  `main.setFocus()` + `compact.hide()`; grant `allow-show` + `allow-set-focus` + `allow-hide`. The always-on-top
  widget no longer floats over the dashboard; symmetric with Cmd+Shift+P.
- **Obs — DECIDED:** no new Rust `ui.layout.transition` span (the webview→core op has no Rust seam, like the
  existing toggle/minimize); forcing a TauRPC procedure purely for a span is the rejected over-reach (handoff
  playbook rule).
