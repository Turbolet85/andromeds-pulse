# design extract

## Relevance
Relevant — this chunk renders explanatory text + a domain glyph in an empty-state surface, squarely in tokens + iconography + component-pattern reuse. (Placement/focus-order is layouts'; formal contrast conformance is a11y's — bindings flagged below.)

## Constraints
- **Empty-state message uses Tertiary text color `--color-text-tertiary` (#7D8697), centered, with an optional custom icon** — this chunk generalizes the existing pattern (currently "No traces yet" / "Snapshot not generated"), per design-system §Surface: desktop-webview → Component Patterns (Loading / Empty States).
- **Prose is `--font-body` (IBM Plex Sans 400 / 14px), NOT monospace** — the message + hint are "human-readable summary / operator communication"; the sans-vs-mono split is domain-semantic (per §Typography, Body row + Rationale). Nuance for /implement: the port literals `:4318` / `:4317` are "technical values" that per the Code role (JetBrains Mono) may be rendered inline-mono inside the sans sentence.
- **Icon = existing Observatory custom glyph via `<Icon glyph="…" />`** (prefer reuse: `telescope` reads as "looking for signal"), monochrome, tinted to the message register (Tertiary, not the default #E8EEF7 primary — "adaptable per state"), at 24px hero size (per §Iconography — custom set, Size grid 24px; "use library icons only when a domain-specific glyph does not exist").
- **Icon must carry meaning, not decorate** — "If removing an icon loses no meaning, remove it" (per §Iconography Rule); never color-for-decoration (per §Anti-Patterns Universal Bans).
- **Contrast caveat on the message token** — Tertiary #7D8697 is ~4.2:1 = **"large text only"**; Muted #56606E is ~2.1:1 = **"decorative only"** (per §Color Palette → Text Hierarchy). The message is meaningful body text, so Muted MUST NOT carry it; Tertiary is only safe if rendered large-text, else escalate to `--color-text-secondary` (#B4BCCB, 6.8:1). Binds a11y (below).
- **Empty branch renders instantly — no entrance/flash animation** — "Entrance animations: none; components render instantly; motion confirms state changes only, not arrivals" (per §Motion). Reinforces the scope's "must NOT flash during the initial fetch."
- **All layout values from the 4px spacing scale** (e.g. `space-md` 16px container padding, `space-micro` 2px icon-to-text gap) (per §Spacing).

## Patterns to follow
- **The existing "Empty state" component pattern** (§Surface: desktop-webview → Component Patterns) — centered Tertiary text + optional 24px telescope icon; this chunk lifts it into a shared reusable component so Metrics/Logs "read as one system."
- **Three-state discipline already encoded** (§Component Patterns) — skeleton loading (discrete opacity pulse, out of scope/unchanged) is visually distinct from the empty state; keep them separate.
- **Icon-as-registered-React-component** — `<Icon glyph="telescope" />` from `src/components/icons/` (§Iconography); reuse an existing glyph before adding one.
- **Token-via-`var()` consumption** — colors/typography/spacing pulled as CSS custom properties (`var(--color-text-tertiary)`, `var(--font-body)`) (§Surface: desktop-webview → Tokens).

## Anti-patterns to avoid
- **NEVER use color purely for decoration** — glyph/message color must communicate the empty state, not ornament (§Anti-Patterns → Universal Bans).
- **NEVER use a generic/stock (Lucide/Heroicons) icon where a domain glyph exists** — the Observatory custom set is the primary visual language; library icons are fallback only (§Anti-Patterns → Rejected Defaults "Generic icons"; §Iconography Secondary library).
- **NEVER set the human-readable copy in JetBrains Mono** — monospace signals "immutable telemetry fact" (data values), not operator prose (§Typography Rationale; §Anti-Patterns → Rejected Defaults).

## Contract bindings
- **Token contrast ↔ a11y §Contrast (SC 1.4.3):** the message color pair on Base #1A1D24 — Tertiary #7D8697 (~4.2:1, large-text-only) / Muted #56606E (~2.1:1, decorative-only). Design flags that a 14px-regular meaningful message on Tertiary sits below the 4.5:1 normal-text threshold; a11y owns the formal derivation (per §Self-Validation → Contrast Test + Downstream Readiness "For a11y specialist").
- **Iconography ↔ a11y §Use of Color (SC 1.4.1):** state is conveyed by the text (message + hint), never glyph color alone — the icon+text pairing satisfies not-color-alone by construction. Icon at 24px is non-text (SC 1.4.11 3:1); Tertiary clears 3:1.
- **Motion ↔ a11y §Animation (SC 2.3.3):** empty branch has no entrance animation, so no reduce-motion override is required for it; the loading skeleton pulse (out of scope, unchanged) already carries that contract.

## Acceptance criteria contributions
- (design) Empty-state message + hint render in `var(--font-body)` (IBM Plex Sans), sans register — no JetBrains Mono for the sentence (§Typography).
- (design) Empty-state glyph is an Observatory custom SVG via `<Icon glyph="…" />` (reuse telescope/aperture/constellation-grid), monochrome, 24px — not a Lucide/stock icon (§Iconography; §Anti-Patterns).
- (design) Uses only design tokens — message color `var(--color-text-*)`, `var(--font-body)`, spacing from the 4px scale, icon 24px — no hardcoded hex/px (§Surface: desktop-webview Tokens; Self-Validation Token Test).
- (design) Message color meets contrast for its rendered size: `--color-text-muted` (2.1:1) MUST NOT carry the message; `--color-text-tertiary` (4.2:1) only at large-text sizing, else `--color-text-secondary` (6.8:1) (§Color Palette → Text Hierarchy; binds a11y §Contrast).

## Relevant amendment history
- **2026-05-03 — Accent lift #8B2E3B → #C7556A (Color Palette; Error component pattern).** Relevant as precedent, not as a direct edit to empty states: it established that a sub-4.5:1 token is reclassified non-text and that **body-size meaningful TEXT (≤14px regular) must move to `--color-text-primary` + border/icon rather than rely on a low-contrast color** (SC 1.4.1 / 1.4.3), under the ruling "a11y > design on conflict." The empty-state message is analogous body-size meaningful text, so the same precedent constrains the Tertiary/Muted choice flagged above — Muted (2.1:1) cannot carry the copy, Tertiary (4.2:1) is large-text-only. (No amendment has previously touched the Loading/Empty-States pattern itself.)