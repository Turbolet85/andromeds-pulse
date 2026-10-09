# design extract

## Relevance
Partial — adds UI for progress/result/error states to Investigation modal; primarily backend-functional with frontend rendering implications.

## Constraints

1. Motion (§Motion): Progress state animation ≤200ms for chrome transitions (150ms ease-out for busy feedback), unless classified as a supporting moment; Investigation Capture Collapse precedent (350ms scale+opacity) suggests user-initiated analysis display *may* exceed hard limits per expression level 0.35 waiver — clarify at planning.

2. Typography (§Typography): Result/error text uses design system stack (IBM Plex Sans Body 14px regular for multi-line analysis; Label 12px 500 for single-line summaries); monospace (JetBrains Mono 12px) for telemetry snippets embedded in result.

3. Color (§Color Palette): Error states use `--color-accent` (#C7556A) **ONLY for borders/icons, never text alone**; pair with `--color-text-primary` (#E8EEF7) for message text per SC 1.4.1 binding (Accent usage reclassified 2026-05-03 as non-text token due to low contrast 3.8:1 on base).

4. Component Patterns (§Surface: desktop-webview): Loading state → skeleton pulse (opacity animation only, 1.2s cycle, 0.5–1.0 envelope, no scale); error alert → role="alert" with accent border (1px rgba(199, 85, 106, 0.5)), error icon (24px), primary text.

5. Expression level (§Brand Identity): desktop-webview surface 0.35 — state-confirming motion, no decorative flourish; Halo State Pulse breathing (4–5 s / ~2 s) is exempt canvas layer; chrome motion inherits micro/supporting-moment timing per Motion table.

6. Motion accessibility (§Motion): All transitions respect `prefers-reduced-motion` media query (skeleton pulse → static glow, 200ms fade → instant, no animation state).

## Patterns to follow

1. Reuse existing modal `phase` + `aria-busy` + `aria-live` status discipline (InvestigationModalForm pattern, already established).

2. Skeleton pulse for loading: opacity animation only (no scale per P-026), 1.2s ease-in-out infinite cycle, 50–100% envelope — reference existing Component Patterns §Loading / Empty States.

3. Error alert structure: Raised-1 background (#262A33), accent border (1px rgba(199, 85, 106, 0.5)), primary text color (#E8EEF7), optional error icon (20px, accent color #C7556A) — never text-color-alone per SC 1.4.1.

4. Result text hierarchy: Body (IBM Plex Sans 14px) for multi-line analysis; Label (12px 500) for single-line summaries or metadata (timestamp, token count, service span).

## Anti-patterns to avoid

1. NEVER use a generic loading spinner or three-dot pulse — skeleton pulse (opacity-only modulation per P-026) or inherit modal phase rendering per design-system §Component Patterns Loading / Empty States.

2. NEVER use Alert Burgundy (#C7556A) for error message text alone — pair with primary text (#E8EEF7) and optional icon/border per design-system §Color Palette Accent usage (SC 1.4.1 binding; accent reclassified as non-text token 2026-05-03).

3. NEVER exceed 200ms for chrome progress feedback, unless analysis result display is formally classified as a supporting moment (user-initiated investigation action would justify 250–350ms scale+opacity per Motion §Supporting moments precedent, pending planning clarification).

## Contract bindings

- **Motion ↔ a11y §Animation**: All progress/result transitions respect `prefers-reduced-motion` override (fade → instant, pulse → static).
- **Color (accent) ↔ a11y §Use of Color**: Error state requires paired text + icon, never accent color alone (SC 1.4.1).
- **Typography (text hierarchy) ↔ a11y §Contrast**: Result/error text contrast values must match Text Hierarchy table — Primary ≈8.5:1, Secondary ≈6.8:1 (per design-system §Color Palette).

## Acceptance criteria contributions

1. (design) Progress state uses only design tokens: `--duration-*`, `--easing-*` (150ms ease-out or 250–350ms supporting-moment easing); no hardcoded pixel/timing values.

2. (design) Progress animation respects `prefers-reduced-motion: reduce` (skeleton pulse → static glow; status fade/transition → 0ms instant).

3. (design) Error message pairs alert accent (#C7556A) with primary text (#E8EEF7) and icon/border affordance; error text never relies on color alone.

4. (design) Result text follows typography hierarchy — Body (IBM Plex Sans 400 14px) for multi-line analysis; Label (500 12px) for single-line metadata; monospace (JetBrains Mono 400 12px) for embedded spans/values.

## Relevant amendment history

**2026-05-29 — Motion / Supporting moments precedent:** Investigation Capture Collapse (snapshot visual confirmation) established 350ms scale+opacity as an exemption from the 200ms chrome hard limit for user-initiated investigation workflows. If this chunk's analysis result display adopts similar visual emphasis (result card scale/fade into focus), that precedent applies; otherwise, chrome transitions remain ≤200ms per 0.35 expression level. Pending planning clarification of "supporting moment" classification for analysis results (see chunk scope "Open questions" → "Progress model").