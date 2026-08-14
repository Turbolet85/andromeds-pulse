# layouts extract

## Relevance
Partial — the chunk docks a NEW disclosure window to the compact-widget surface (layout concerns: anchor surface, frameless/always-on-top posture, focus order, dismiss lifecycle), but the Findings/incidents disclosure component itself is absent from the layout plan body (its layout truth was set by P-080 + chunk #87, not this plan), and the window-creation / `core:window` capabilities / positioning-geometry / re-poll substance is arch/backend/data — out of layouts.

## Constraints
- The Findings panel docks below the **compact widget**, whose established posture is fixed quarter-screen, frameless custom titlebar, always-on-top, snap-to-edge (per layout-templates.md §Wireframe — Compact widget + §IA notes → Responsive behavior). The floating panel inherits the frameless + always-on-top posture; do not introduce OS window chrome.
- The compact widget is a fixed-size glance surface that **does not reflow** and has no CSS media-query breakpoints (per §IA notes → Responsive behavior). Disclosure content therefore belongs in a separate window positioned by absolute geometry, not by growing/reflowing the widget — this is the chunk's documented origin (operator chose "separate floating panel" over "grow-the-window").
- **Esc must be scoped:** the widget's own Esc minimizes it to the tray (per §IA notes → Navigation model + Keyboard shortcuts "`Esc`: minimize widget or close modal"); when the Findings panel is open/focused, Esc must instead dismiss the panel and restore focus to the triggering badge — the panel's Esc-dismiss overrides the widget's Esc→tray only while it owns focus.
- The panel is a **third window-label render branch** alongside widget + dashboard, following the existing multi-surface split; it is independent of the widget↔dashboard toggle (per §Primary screens + §IA notes → Multi-surface coordination).
- Desktop-only surface, no touch/mobile, no responsive breakpoints (per §scope note + §IA notes → Responsive behavior) — placement is absolute window geometry (below-widget with work-area clamp), never a media-query reflow.

## Patterns to follow
- **Compact widget wireframe + posture** (§Wireframe — Compact widget; §IA notes → Navigation model) — frameless custom titlebar, always-on-top, snap-to-edge, fixed quarter-screen; the panel docks against this anchor and matches its chrome-less posture.
- **Modal dismiss + focus semantics** (§Component — Investigation modal; §Component — Settings modal) — the overlay surfaces establish Esc-dismiss + focus behavior; the panel's Esc/blur/row-select/mark-all-read dismiss lifecycle patterns on this, though it is a separate WINDOW (no backdrop / z-index overlay — dismiss+restore semantics match, container differs).
- **Multi-surface / window split** (§Primary screens; §IA notes → Multi-surface coordination) — widget / dashboard / tray already coordinate as distinct windows sharing data feeds; the Findings window extends this as a new label branch.

## Anti-patterns to avoid
- Do NOT grow/reflow or render an in-widget overlay/popover inside the fixed compact widget — the widget is fixed quarter-screen and does not reflow (§IA notes → Responsive behavior; §Wireframe — Compact widget); this is exactly the P-080 interim approach this chunk replaces.
- Do NOT add OS/native window chrome to the panel — the compact-widget family is frameless/custom-titlebar (§Wireframe — Compact widget; §Component — Custom titlebar); the panel is borderless per the same posture.

## Contract bindings
- **Focus order ↔ a11y:** the panel's focusable content + cross-window Esc-restore-to-badge bind to a11y §Focus Order (SC 2.4.3) and a11y §Modal focus trap (Esc dismiss + restore focus) — per focus-guide cross-domain bindings; the chunk explicitly consumes the chunk #87 disclosure a11y contract across windows (consumed, not modified).
- **Disclosure component ↔ prior chunks (not this plan):** the `FindingsDropdown` bounded/scrollable/opaque content and the unread badge are NOT in layout-templates.md — their layout truth lives in the P-080 (`2026-07-06-incidents-panel-dropdown-layout-bug`) + chunk #87 reports; treat those as source of truth for the reused content, not this plan.
- **Out of layouts:** window creation (Tauri `WebviewWindowBuilder`/`WebviewWindow`), `core:window`/`core:webview` capability grants, positioning geometry/clamp APIs, and the `use-findings.ts` re-poll CARRY → arch / backend / data domains.

## Acceptance criteria contributions
- (layouts) The Findings window opens as a borderless, always-on-top panel docked directly BELOW the compact widget — not by reflowing/growing the widget (layout-templates §Wireframe — Compact widget + §IA notes → Responsive behavior "layout does not reflow").
- (layouts) Focus order: opening the panel moves focus into the disclosure content; Esc returns focus to the triggering unread badge in the widget window (cross-window focus restoration), scoped so it does not collapse into the widget's Esc→tray (layout-templates §IA notes → Navigation model / Keyboard shortcuts + focus-guide focus-order binding).
- (layouts) The panel renders as a third window-label branch alongside widget + dashboard, independent of the widget↔dashboard toggle (layout-templates §Primary screens + §IA notes → Multi-surface coordination).
- (layouts) Below-widget placement clamps into the work area near a screen edge / on a second monitor (flip-above or clamp) — no off-screen panel (desktop-only absolute geometry per §scope note + §IA notes → Responsive behavior).

## Relevant amendment history
- **2026-05-02 — Initial layout templates generated (§whole document).** Established the compact-widget posture this chunk docks against: fixed quarter-screen glance surface, frameless titlebar, always-on-top, snap-to-edge, Esc→tray, plus three-surface coherence sharing data feeds. This is the foundational anchor + multi-surface-split precedent the Findings window extends; no later amendment overrides it for the widget.
- **No amendment covers the incidents/Findings disclosure surface, the P-080 popover fix, or chunk #87** — the layout plan has never been amended to document the disclosure component, confirming its layout truth lives in those chunk reports (active-memory note for the implementer: reuse P-080's `FindingsDropdown` as source of truth, not this plan).
- (Tangential, not this chunk's area) 2026-06-29 close-to-tray signpost (hide-to-tray predictability — a distant sibling to the panel's "hide not destroy" lifecycle), 2026-07-08 shared empty-state pattern, and 2026-07-09 bounded internal-scroll table region (a sibling of the bounded/scrollable disclosure content) are only reference precedents — the chunk reuses `FindingsDropdown` verbatim and does not redesign the list.