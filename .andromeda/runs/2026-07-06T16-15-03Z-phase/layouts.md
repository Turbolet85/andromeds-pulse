# layouts extract

## Relevance
Partial — the surface (desktop-webview **compact widget**) and its no-reflow / bounded-overlay constraints apply directly, but the Findings-dropdown popover itself is a downstream component (~chunk #91) not documented in the plan; layouts governs only the surface-level bounding/placement.

## Constraints
- Compact widget is a **fixed quarter-screen glance surface whose layout does NOT reflow** — user-resizable but no re-layout (per layout-templates.md §IA notes → Responsive behavior). The reported window/content stretch violates this; the fix must keep the widget geometry fixed.
- Compact-widget region hierarchy is a vertical column: frameless titlebar (top) → Halo canvas hero (fills) → read-only status footer band (bottom) (per layout-templates.md §Wireframe — Compact widget / §Component — Footer). The `findings-band` is the bottom region; anything anchored there opens against the widget's bottom edge.
- Compact widget has **no CSS media-query breakpoints** — min size stays readable on a 13-inch display, layout does not reflow (per layout-templates.md §IA notes → Responsive behavior). No breakpoint logic may be introduced to reposition the popover.
- Overlay surfaces are **viewport-bounded** — the modal patterns cap to ~80% of viewport with scrollable overflow rather than expanding the host (per layout-templates.md §Component — Investigation modal / §Settings modal). The popover must stay within the widget viewport and scroll internally, never push the surface bounds.
- Compact-widget motion sits within the 0.35 webview expression budget (panel transitions) and degrades to static under `prefers-reduced-motion` (per layout-templates.md §Surface: desktop-webview Expression level; §Halo canvas reduced-motion). Repositioning must preserve the reduced-motion static path.

## Patterns to follow
- Compact-widget flex-column composition — fixed titlebar + filling canvas + bottom band (per layout-templates.md §Wireframe — Compact widget); anchor/reposition the popover without displacing titlebar or canvas.
- Overlay bounding discipline — bounded card, capped size, internal scroll, never expands the host surface (per layout-templates.md §Component — Investigation modal); apply the same discipline to the popover.
- Footer-anchored dropdown precedent — full-dashboard footer uses button-with-dropdown-caret controls (time-range / service filter) (per layout-templates.md §Component — Footer, Full dashboard). Closest existing "dropdown from a bottom band," though the plan does not specify its bounding/open-direction (gap owned by this chunk's implementation).
- Reduced-motion static fallback for panel transitions (per layout-templates.md §Component — Settings modal Motion / §Halo canvas reduced-motion).

## Anti-patterns to avoid
- Do NOT let the compact widget reflow or stretch to contain content — it is fixed quarter-screen, layout does not reflow (per layout-templates.md §IA notes → Responsive behavior). The window-stretch symptom is exactly this ban.
- Do NOT introduce media-query breakpoints into the compact widget to solve positioning (per layout-templates.md §IA notes → Responsive behavior).
- Do NOT let an overlay/popover overflow the viewport unbounded — cap and scroll internally per the modal bounding rule (per layout-templates.md §Component — Investigation modal).

## Contract bindings
- **layouts ↔ design (tokens):** the popover background-token tension (`--color-inset` vs `--color-raised-2`, scope §Boundaries) is design's authority — layouts asserts only "opaque, bounded surface within the widget"; the token value/reconciliation belongs to design (focus guide: tokens out-of-scope).
- **layouts ↔ a11y (focus/disclosure):** popover placement must preserve the chunk #87 disclosure contract (role / aria-expanded / Escape dismiss / click-outside / focus-return) and the p8 axe spec — focus trap, Escape, and focus-return are a11y's authority (focus guide: Modal patterns → a11y §Modal focus trap; Focus order → a11y §Focus Order SC 2.4.3). Layouts supplies the anchor + visible reading-order position.
- **layouts ↔ design (motion):** reduced-motion timing/tokens are design §Motion; layouts references only the no-reflow static structure.

## Acceptance criteria contributions
- (layouts) Opening the Findings dropdown does NOT stretch or reflow the compact widget — it keeps its fixed quarter-screen geometry (layout-templates §IA notes → Responsive behavior "fixed quarter-screen … layout does not reflow").
- (layouts) The popover renders fully within the widget viewport as a bounded surface anchored to the unread badge — no off-viewport / mis-clipped region; overflow scrolls internally (layout-templates §Component — Investigation modal "capped … with scrollable overflow").
- (layouts) The `findings-band` remains the bottom region of the compact-widget column and the popover placement does not displace the titlebar or Halo canvas hero (layout-templates §Wireframe — Compact widget).
- (layouts) No CSS media-query breakpoint is introduced to position the popover (layout-templates §IA notes → Responsive behavior "no CSS media-query breakpoints").

## Relevant amendment history
- **2026-05-02 initial generation** — during v3 normalization the "Responsive posture (webview)" item was folded into desktop-webview IA notes: compact widget fixed quarter-screen, no CSS media-query breakpoints, user-resizable but layout does not reflow. This is the foundational constraint the current bug violates and that the fix must honor. (The 2026-06-29-predictable-close notification-trigger amendment is unrelated to the findings dropdown.)
