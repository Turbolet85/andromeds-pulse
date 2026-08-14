# design extract

## Relevance
Partial — D1 (internal-scroll table region) and D3 (constellation labels) carry token/surface/scrollbar/motion obligations in my domain; the scroll mechanic + region stacking (D1) and the wireframe doc (D2) are layouts-distiller, and D4 (announce-in-updater fix) is a11y/React code — both out of my domain.

## Constraints
- The new internal-scroll region introduces a scrollbar that MUST be custom-styled (dark track + Earth Blue thumb via Tailwind v4 `scrollbar-*` utilities), never the default web scrollbar — per design-system.md §Anti-Patterns → Per-Surface Bans (desktop-webview).
- Region separation (fixed hero/toolbar vs. scrolling body) is expressed via borders + surface-lightness only — NO drop-shadow/gradient/glassmorphic elevation cue when rows scroll under the header — per design-system.md §Depth Strategy (borders-only) + §Anti-Patterns Universal.
- A fixed/sticky table header must keep an OPAQUE `#1A1D24` (Base) background with `#E8EEF7` Primary text weight-600 12px so scrolling rows don't bleed through; data cells stay JetBrains Mono 12px tabular-nums, row borders `rgba(74,144,226,0.1)` (Subtle) — per design-system.md §Surface: desktop-webview → Component Patterns → Tables.
- All values via existing tokens (`var(--color-*)`, `--radius-*`, `--font-*`, `--spacing-*`); no hardcoded hex/magic numbers — scope confirms "reuse existing tokens; no new token unless research shows a gap" — per design-system.md §Surface: desktop-webview → Tokens + §Self-Validation Token Test.
- No scroll-driven motion may be introduced on the new scroll region (no parallax, scroll-triggered reveal, or scroll-driven opacity); hovered-row remains an instant change (no transition, per 0.35 expression) — per design-system.md §Motion → Hard limits + §Tables.
- D3 label collision-avoidance touches the constellation hero where the Halo State Pulse lives; it must not perturb the Halo encoding (opacity+blur ONLY never scale — P-026; severity-hue and connection-grayout axes) — per design-system.md §Brand Identity (Signature element) + §Motion.

## Patterns to follow
- Tables pattern (design-system.md §Component Patterns → Tables): transparent rows, opaque Base header, Subtle inter-row borders, JetBrains Mono data cells — the internal-scroll table reuses this unchanged; only the header/toolbar gain fixed positioning.
- Custom scrollbar pattern (design-system.md §Anti-Patterns → Per-Surface Bans, desktop-webview): dark background + Earth Blue thumb via `scrollbar-*` utilities — apply to the new `overflow-y:auto` wrapper.
- Surface-layering tokens (design-system.md §Component Patterns → Canvas Container + Cards/Panels): constellation hero on `#0F1117` (Inset) canvas bg; toolbar/table on `#262A33` (Raised-1) card — fixed regions retain their existing surface tokens.
- Constellation-label typography (design-system.md §Typography Label 12px / Data): always-on per-dot service labels are UI text — IBM Plex Sans Label 12px (or JetBrains Mono Data if a service ID), colored from the Text Hierarchy, never a raw hex.

## Anti-patterns to avoid
- NEVER add a shadow/gradient elevation cue to the fixed header on scroll — borders-only discipline (design-system.md §Depth Strategy + §Anti-Patterns Universal: no gradient/glassmorphic chrome).
- NEVER ship the new scroll region with an unstyled web-style scrollbar (design-system.md §Anti-Patterns → Per-Surface Bans desktop-webview).
- NEVER attach scroll animations / scroll-driven opacity / parallax to the internal scroll (design-system.md §Motion → Hard limits).

## Contract bindings
- Token contrast → a11y §Contrast (SC 1.4.3): fixed-header text `#E8EEF7`/Base ≈8.5:1 holds; any body-size label text (D3) must use `#B4BCCB` Secondary (≥4.5:1), NOT `#7D8697` Tertiary (4.2:1, large-text-only) — flag to a11y.
- Constellation label state color → a11y §Use of Color (SC 1.4.1 / 4.1.2): D3 must preserve the P-069 always-on accessible name + non-color severity token; color-token choices here bind to the "never color alone" rule.
- D4 announce-in-updater fix is a11y/StatusLiveRegion (React render-safety) — no design/token surface; binding owned by the a11y + tests domains, not design.

## Acceptance criteria contributions
- (design) The internal-scroll region's scrollbar is custom-styled (dark track + Earth Blue thumb via `scrollbar-*`), not the browser default (design-system.md §Anti-Patterns → Per-Surface Bans).
- (design) Fixed header/toolbar use opaque surface tokens and are separated from the scrolling body by border only — no drop-shadow/gradient appears on scroll (design-system.md §Depth Strategy).
- (design) Uses only design tokens (`var(--color-*)`, `--font-*`, `--radius-*`, `--spacing-*`) with no hardcoded hex; table cells keep JetBrains Mono 12px tabular-nums (design-system.md §Tokens + §Self-Validation Token Test).
- (design) No scroll-driven animation/parallax is introduced and hovered-row stays instant; D3 does not alter Halo opacity/blur/hue encoding or introduce dot scaling (design-system.md §Motion Hard limits + §Brand Identity).

## Relevant amendment history
- **2026-07-08-self-explaining-empty-states** — body-size text token corrected Tertiary `#7D8697` (4.2:1, large-text-only) → Secondary `#B4BCCB` (≥4.5:1), and references the prior **chunk-#99 LogTable/LogFilter tertiary→secondary remediation**. Directly relevant: any body-size text added/repositioned in the Traces table (D1) or the constellation labels (D3) must follow the same Secondary-not-Tertiary rule already applied to the sibling LogTable.
- **2026-05-29 Halo re-driven by incident severity + activity + connection state** — locks the Halo invariants (opacity+blur only never scale P-026; cumulative-severity LCH hue; orthogonal connection-grayout axis). Relevant because D3's collision-avoidance operates on the same constellation hero and must leave these encoding axes untouched.
