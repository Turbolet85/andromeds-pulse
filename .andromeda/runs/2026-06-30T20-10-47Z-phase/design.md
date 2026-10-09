# design extract

## Relevance
relevant — chunk adds an in-app affordance (button/control) to the `desktop-webview` compact-widget surface, which is directly covered by design-system constraints.

## Constraints

1. per design-system §Color Palette + §Component Patterns Buttons: button colors must use design tokens only (`--color-primary` #4A90E2 for primary action button, or `--color-text-primary` #E8EEF7 on `--color-secondary` #262A33 background for secondary).
2. per design-system §Typography: button text must use **Label** (12px IBM Plex Sans, 500 weight) or **Body** (14px IBM Plex Sans, 400) — no other sizes.
3. per design-system §Spacing: button internal padding must be `space-xs` (4px vertical) + `space-sm` (8px horizontal) per Component Patterns Buttons section.
4. per design-system §Border Radius: button corner radius must be `radius-sm` (4px), never radius-full or squared.
5. per design-system §Motion: if affordance has hover/focus transition, duration ≤150ms with `ease-out` cubic-bezier(0.4, 0, 0.2, 1) easing; all transitions respect `prefers-reduced-motion: reduce` (degrades to instant, not fade).
6. per design-system §Component Patterns Focus / Keyboard Navigation: affordance MUST be Tab-navigable with `:focus-visible` focus ring (3–4px outset, #4A90E2, rendered via `box-shadow: 0 0 0 3px rgba(74, 144, 226, 0.2)` or `outline: 3px solid #4A90E2`).
7. per design-system §Iconography: if affordance icon used, prefer domain-specific glyphs (aperture, telescope, constellation-grid, star, circular-pulse) registered as React `<Icon glyph="..." />` components; fallback to Lucide/Heroicons only if no domain glyph exists.

## Patterns to follow

1. Primary Button component pattern from §Component Patterns Buttons: `background #4A90E2` (Primary), text `#1A1D24` (dark), hover background `#5BA5F0` (+10% lightness), transition 150ms ease-out.
2. Focus ring pattern from §Component Patterns Focus / Keyboard Navigation: `:focus-visible` always shown, 3–4px outset #4A90E2.
3. Spacing pattern: container padding `space-md` (16px) per compact-widget shell; button padding `space-xs` (4px) vertical + `space-sm` (8px) horizontal.
4. If affordance has any motion (hover state change, entrance), adhere to Motion §Duration scale (150ms ease-out for micro-interactions); no staggered reveals, no spring physics, no 3D transforms.

## Anti-patterns to avoid

1. Never hardcode hex values or pixel values outside the design-system palette and spacing scale; never use Inter, Roboto, or system-ui fonts.
2. Never use hover-only interactions without keyboard alternatives — affordance must be fully navigable and operable via keyboard (§Anti-Patterns desktop-webview).
3. Never use animations >200ms for button chrome transitions, spring easing, or 3D transforms; never add gradient overlays or glassmorphism to button chrome (visual weight reserved for Halo State Pulse canvas only).

## Contract bindings

- Motion tokens (if used) bind to a11y §Animation (all transitions must degrade per `prefers-reduced-motion: reduce`).
- Focus ring color (#4A90E2) binds to a11y §Contrast (focus indicator must remain perceivable on Raised-1 #262A33 or Base #1A1D24 backgrounds; verify against SC 1.4.3 / 1.4.11).
- Button text + background bind to a11y §Use of Color (text must meet minimum contrast ratio, state never conveyed by color alone).

## Acceptance criteria contributions

1. (design) Affordance button uses only design tokens for color, spacing, typography size/weight, radius, and motion duration/easing — no hardcoded hex or pixel values.
2. (design) Button styling matches Component Patterns §Buttons (primary or secondary per context): correct background/text colors, hover state, focus ring per §Focus / Keyboard Navigation.
3. (design) Affordance is Tab-navigable and `:focus-visible` focus ring appears (3–4px, #4A90E2) — keyboard-only users can activate it without mouse.
4. (design) If affordance has transition (hover, focus), respects `prefers-reduced-motion: reduce` (degrades to instant, not fade) and uses ≤150ms ease-out timing per §Motion hard limits.

## Relevant amendment history

(none) — The design system has not been amended for compact-widget navigation affordances or button design since initial system generation. General constraints on button colors, spacing, typography, motion, and accessibility were established in the initial design system (2026-05-02, current truth folded into body §Component Patterns Buttons, §Motion, §Focus / Keyboard Navigation). Subsequent amendments (2026-05-03 accent, 2026-05-29 Halo motion) do not directly affect button chrome design in this chunk's scope.
