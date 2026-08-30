# design extract

## Relevance
Partial — the ACL-rejection record mechanism (backend/IPC/allowlist/bindings) is outside design; in-domain are CARRY item 6 (`pulse-app/ui/src/report/Report.tsx` `ErrorState` token fix) and any user-visible error text on the retro-fitted webview catch path (scope items 4, 6).

## Constraints
- Accent `#C7556A` is a **non-text token** (≈3.8:1 on Base `#1A1D24`: clears SC 1.4.11 non-text and SC 1.4.3 large-text, fails SC 1.4.3 normal-text). Body-size error message text (≤14px regular) must be `--color-text-primary`, with accent carried as border/icon only — per design-system §Color Palette → Core Colors → "Accent usage (non-text token)". Whether `Report.tsx` `ErrorState` already violates this is research's question; the plan states the target, not the shipped state.
- The Semantic Colors **Error** row requires background Base, border `#C7556A`, text `#E8EEF7` — per design-system §Color Palette → Semantic Colors. A body-size error surface rendering the accent as text contradicts this row.
- Two distinct error patterns exist and the chunk must pick the one matching the context: a **form/field** error message uses `var(--color-text-primary)` (§Surface: desktop-webview → Component Patterns → Input Fields / Form Controls → Error state), while a **data/report LOAD** failure uses the honest-error `EmptyState` variant — static message, no actionable hint, `#B4BCCB` (Secondary), checked BEFORE the empty branch (§Component Patterns → Loading / Empty States → Error state). Both clear 4.5:1; the scope's proposed `--color-text-primary` satisfies the non-text ban either way, but the plan's load-error pattern names Secondary — the plan owns which applies to the report window.
- Never render the raw `AppError`; the load-error message is static ("Couldn't load …") so a failure cannot masquerade as "no data" — per §Component Patterns → Loading / Empty States → Error state.
- All values on any touched surface must resolve through Tailwind v4 `@theme` CSS custom properties (`--color-*`, `--spacing-*`, `--radius-*`, `--font-*`) — no raw hex, no magic px — per §Surface: desktop-webview → Tokens (platform-specific) and §Self-Validation Protocol #4 (Token Test).
- Implementation contrast must match the Text Hierarchy table (Primary ≈8.5:1, Secondary ≈6.8:1, Tertiary ≈4.2:1 large-text-only); any divergence must be recorded in `design-system-amendments.md` — per §Self-Validation Protocol #6 (Contrast Test).
- Depth is borders-only: an error boundary is a 1px `rgba(199, 85, 106, 0.5)` border, never a shadow or glow — per §Depth Strategy → Specific values (Error/alert border).

## Patterns to follow
- The Input Fields **Error state** treatment (accent border + `var(--color-text-primary)` message below) — the exact shape the `2026-08-23-a11y-verification` pass applied at its three sites, per §Surface: desktop-webview → Component Patterns → Input Fields / Form Controls.
- The shared honest-error `EmptyState` variant (glyph + static message, no hint, checked before the empty branch), reused across data views, per §Component Patterns → Loading / Empty States.
- Typography roles: message body = IBM Plex Sans 400 / 14px (`font-body`); any technical value (window label, port, path) = JetBrains Mono 12px (`font-code`) — per §Typography.
- Iconography: an optional error icon is monochrome, 16px or 20px per the size grid, drawn from the custom Observatory set first with Lucide/Heroicons only as fallback for generic validation glyphs, and `aria-hidden` when decorative — per §Iconography and the `EmptyState` precedent in §Component Patterns.
- The shipped primitive layer is `react-aria-components` (first-party `Modal` for dialogs) — per §Surface: desktop-webview → Toolkit / Framework, as corrected 2026-08-30.

## Anti-patterns to avoid
- NEVER use `var(--color-accent)` as the color of body-size text — accent is non-text-only (§Color Palette → Accent usage; §Anti-Patterns → Universal Bans "color purely for decoration").
- NEVER convey the error state by color alone; the accent border/icon must be paired with a text label (§Color Palette → Accent usage, SC 1.4.1 binding).
- NEVER hardcode hex or px literals or reach for Tailwind default palette values on the touched surface; and NEVER surface a rejection via `alert()`/`confirm()`/`prompt()` (§Anti-Patterns → Universal Bans; §Anti-Patterns → Per-Surface Bans, desktop-webview).

## Contract bindings
- **design ↔ a11y:** design supplies target ratios (Text Hierarchy table) and the non-text accent classification; a11y owns the formal SC 1.4.3 (4.5:1 normal text) and SC 1.4.1 (use of color) conformance derivation — per design-system-amendments §Downstream Readiness → "For a11y specialist".
- **design ↔ tests:** the vitest/a11y coverage named in the scope's surfaces list verifies Component Patterns token values against the Color Palette table — per design-system-amendments §Downstream Readiness → "For tests specialist".
- **design ↔ obs:** the rejection record's bounded fields (`error_category`, sanitized window label, byte count) have no design surface; if any of it reaches the UI it must render through the error-state pattern above, never as raw error text.
- **Amendment obligation:** §Color Palette currently asserts the accent-as-error-text migration is COMPLETE as of `2026-08-23-a11y-verification`. If research confirms `Report.tsx` `ErrorState` as a fourth un-migrated site, that claim needs correcting in this chunk's wrap amendment.

## Acceptance criteria contributions
- (design) The report `ErrorState` body-size message renders a ≥4.5:1 text token (`--color-text-primary`, or `--color-text-secondary` if the plan rules it a load-error `EmptyState`), never `var(--color-accent)` (per design-system §Color Palette → Core Colors → Accent usage (non-text token)).
- (design) The error state is conveyed by a text label plus at most an accent border/icon — never color alone — and the message is static, never the raw `AppError` (per design-system §Component Patterns → Loading / Empty States → Error state).
- (design) Every value on the touched surface resolves to a design token; no hardcoded hex or px literal is introduced (per design-system §Self-Validation Protocol #4 Token Test).
- (design) If a fourth accent-as-error-text site is confirmed, the "migration is COMPLETE" wording under §Color Palette is amended at wrap rather than left standing (per design-system §Self-Validation Protocol #6 Contrast Test, divergence-recording clause).

## Relevant amendment history
- **2026-05-03 — Lift `--color-accent` `#8B2E3B` → `#C7556A`:** created the non-text reclassification and the original deferral of error *message text* to `--color-text-primary`. This is the root of the CARRY class.
- **2026-08-23-a11y-verification — accent-as-error-text deferral discharged (3 sites):** retired that deferral as COMPLETE based on measurement at `InvestigationModalForm.tsx:280`/`:381`, moving the Semantic Colors Error row's Text cell to `#E8EEF7` and restating the Input Fields error pattern as a token. This is precisely the sweep the scope alleges missed `Report.tsx` — so the chunk's CARRY both applies the same fix and tests that amendment's completeness claim.
- **2026-07-08-self-explaining-empty-states — Tertiary→Secondary + honest-error variant:** established the load-error `EmptyState` variant (static message, no hint, checked before the empty branch) on `--color-text-secondary`, and the ≥4.5:1 body-size rule that rules Tertiary out. Governs which token a report LOAD-error message takes.
- **2026-08-30-diagnostics-un-muting-harness-truth-sweep — shipped component stack:** the a11y-primitive layer is `react-aria-components` (radix denylisted by `pulse-app/ui/npm-policy.json`); relevant if the fix touches or adds any primitive on the report surface.