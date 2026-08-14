# design extract

## Relevance
Partial — chunk is mostly window mechanism / capabilities / focus-restore / CARRY (outside design), but it adds a floating chrome surface and reuses design-token surfaces, so surface/depth/anti-pattern tokens apply.

## Constraints
- The Findings window is dashboard **chrome**: flat, matte, opaque — never glassmorphic or gradient-overlaid (the scope's "opaque" requirement is a hard design ban, per design-system.md §Anti-Patterns → Universal Bans; the Halo is the sole gradient exception, not this panel).
- Elevation is **borders-only, no shadow** — the always-on-top panel defines its boundary with a 1px border + z-index/always-on-top, never a drop shadow or tint (per design-system.md §Depth Strategy).
- The reused FindingsDropdown content background is `--color-inset` (#0F1117), opaque — do not introduce a new surface hue for the window; keep the P-080 inset surface token (per design-system.md §Color Palette → Surface Scale).
- The panel must keep the dark-first, disciplined mission-control voice; it is chrome, so visual weight stays reserved for the constellation/Halo, not this disclosure surface (per design-system.md §Brand Identity, "chrome is flat, matte, disciplined").
- The scrollable Findings list must use the **custom styled scrollbar** (dark background + Earth Blue thumb), never the default web scrollbar (per design-system.md §Anti-Patterns → Per-Surface Bans: desktop-webview).
- The new window's webview must hide Chromium artifacts — suppress right-click context menu + disable text selection on non-text elements, same discipline as widget/dashboard (per design-system.md §Anti-Patterns → Per-Surface Bans: desktop-webview).
- When Esc restores focus to the unread badge, the badge must render the #4A90E2 3–4px outset `:focus-visible` ring (per design-system.md §Color Palette → Border Progression [Focus] and §Surface: desktop-webview → Focus / Keyboard Navigation).

## Patterns to follow
- **Reuse the P-080 FindingsDropdown surface verbatim** — bounded / scrollable / opaque `--color-inset` — rather than re-tokenizing the list (per design-system.md §Color Palette → Surface Scale [Inset]).
- **Cards / Panels boundary pattern** for the floating container edge: 1px subtle border rgba(74,144,226,0.3), radius-md (6px), padding space-md (16px) — apply if not already inherited from the interim popover (per design-system.md §Surface: desktop-webview → Component Patterns: Cards / Panels).
- **Focus / Keyboard Navigation pattern** — `:focus-visible` always-shown outset ring, all interactive elements Tab-navigable (per design-system.md §Surface: desktop-webview → Focus / Keyboard Navigation).
- **Motion default = none** — components render instantly; motion confirms state changes only, not arrivals. Prefer instant window show/hide (aligns with the scope's "hide-not-destroy") over an entrance animation (per design-system.md §Motion, "Entrance animations: none").

## Anti-patterns to avoid
- No glassmorphism / gradient / tint on the panel chrome — flat matte opaque only (per design-system.md §Anti-Patterns → Universal Bans).
- No shadow-based elevation for the floating panel — borders + always-on-top/z-index only (per design-system.md §Depth Strategy).
- No unstyled web scrollbar and no visible Chromium context-menu / text-selection in the new webview (per design-system.md §Anti-Patterns → Per-Surface Bans: desktop-webview).

## Contract bindings
- **Focus ring ↔ a11y:** design owns the #4A90E2 `:focus-visible` ring token; a11y owns the cross-window focus order / Esc-restore / disclosure contract (chunk #87, SC 2.4.3) — the ring must render on the badge when focus lands (design-system.md §Border Progression ↔ a11y §Focus).
- **Motion ↔ a11y:** any window show/hide or dismiss transition binds a11y §Animation (`prefers-reduced-motion: reduce`, SC 2.3.3) and the §Motion ≤200ms fade hard-limit (design-system.md §Motion ↔ a11y §Animation).
- **Surface/text tokens ↔ a11y contrast:** the reused inset surface + text tokens carry P-080's established contrast; introduce no new color pair that would need re-derivation (design-system.md §Color Palette ↔ a11y §Contrast).

## Acceptance criteria contributions
- (design) The Findings window renders on `--color-inset` opaque background using only design tokens — no hardcoded hex / pixel values (design-system.md §Color Palette → Surface Scale).
- (design) The floating panel defines its boundary via a 1px border (borders-only) — no shadow, gradient, or glassmorphic effect (design-system.md §Depth Strategy + §Anti-Patterns).
- (design) On Esc-restore, the unread badge shows the #4A90E2 `:focus-visible` outset ring (design-system.md §Border Progression [Focus]); pairs with the a11y focus-order check.
- (design) Any window show/hide transition respects `prefers-reduced-motion: reduce`, and default is instant / no entrance animation (design-system.md §Motion).

## Relevant amendment history
(none) — no prior entry in design-system-amendments.md touches the floating-window / depth-strategy / Findings-surface area; the closest predecessor decision (P-080's bounded, opaque `--color-inset` FindingsDropdown) lives in this chunk's scope, not the amendment sidecar, and the 2026-07-08 empty-state token amendment governs the Metrics/Logs/Snapshots `EmptyState`, not this disclosure surface.