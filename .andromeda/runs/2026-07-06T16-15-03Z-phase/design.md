# design extract

## Relevance
partial — the chunk renders a popover surface (its background/border/radius/motion tokens are my domain), but its core is positioning/anchoring/overflow-bounding, which is the layouts distiller's; I own only the token + depth + motion treatment.

## Constraints
- Dropdown/popover background is `--color-raised-2` (#2D3139) — per design-system.md §Color Palette → Surface Scale, which assigns Raised-2 to "Dropdowns, popovers, tooltips". This confirms the code's existing token is correct and resolves the scope tension: `--color-inset` (#0F1117) is the input-field/canvas surface, NOT the popover surface. Do not switch to Inset.
- Depth is borders-only: elevate the popover via background lightness + a 1px border and z-index stacking, never a shadow — per §Depth Strategy.
- Popover border is 1px solid rgba(74, 144, 226, 0.3) (Subtle/Standard Earth-Blue) — per §Depth Strategy "Default border" and §Color Palette → Border Progression.
- Rectangular popover container uses radius-md (6px, panels) — per §Border Radius.
- Positional offsets are multiples of the 4px base unit; the `--spacing-xs` (4px) anchor offset the code already uses is a valid token — per §Spacing.
- Open/close is a 200ms ease-out fade (panel transition); no entrance animation, and the hard limit forbids opacity fades > 200ms — per §Motion.
- Surface stays flat, matte, opaque — no gradient/glassmorphic fill; the "white background" symptom must resolve to the opaque design token, not a translucent effect — per §Anti-Patterns → Universal Bans.

## Patterns to follow
- Elevation-via-lightness+border (not shadow): Raised-2 background + 1px subtle border + z-index for stacking — per §Depth Strategy + §Color Palette → Surface Scale.
- Cards/Panels treatment as the analogous bounded container (1px subtle border, space-md padding, radius-md), adapted to the Raised-2 surface level for a popover — per §Surface: desktop-webview → Component Patterns (Cards/Panels).
- Panel-emergence motion: 200ms ease-out fade — per §Motion (Transitions: panel open).
- Focus/Keyboard Navigation: preserve the 3–4px #4A90E2 `:focus-visible` ring on the interactive rows and "Mark all as read" retained inside the panel — per §Surface: desktop-webview → Component Patterns (Focus / Keyboard Navigation).

## Anti-patterns to avoid
- No gradient/glassmorphic fill on chrome (Halo State Pulse is the sole exception); the popover stays flat/matte/opaque — per §Anti-Patterns → Universal Bans.
- No drop shadow to "fix" the mis-clip — borders-only depth strategy; elevation is lightness + border + z-index only — per §Depth Strategy.

## Contract bindings
- Motion ↔ a11y: the 200ms popover fade must honor `prefers-reduced-motion: reduce` → 0ms (§Motion → Accessibility); binds to a11y §Animation SC 2.3.3 and to the chunk's preserved reduced-motion handling.
- Tokens ↔ layouts distiller: I own the surface/border/radius/motion tokens the fix consumes; the anchor position and overflow-bounding (where the panel sits relative to the bottom-anchored badge) is the layouts distiller's decision.
- Token contrast ↔ a11y §Contrast: dropdown text on the Raised-2 surface must keep its established pairing (§Color Palette → Text Hierarchy); this layout-only change must not alter any text/surface color pair.

## Acceptance criteria contributions
- (design) Findings popover background is the opaque `--color-raised-2` token — not `--color-inset`, not a white/translucent fill (§Color Palette → Surface Scale).
- (design) Elevation is background-lightness + a 1px subtle Earth-Blue border + radius-md (6px); NO shadow (§Depth Strategy / §Border Radius).
- (design) Uses only design tokens for background/border/radius/offset — no hardcoded hex or pixel values.
- (design) Open/close transition respects `prefers-reduced-motion: reduce` (fade → instant) (§Motion).

## Relevant amendment history
(none) — no amendment has touched the dropdown/popover surface, the Raised-2 token, or the findings band. The 2026-05-03 accent lift (`#C7556A`, error-text role) governs incident/error coloring that can appear in rows, but this chunk is layout-only (no color or text-role change), so it does not apply.
