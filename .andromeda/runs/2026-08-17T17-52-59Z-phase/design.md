# design extract

## Relevance
Partial — the chunk's substance is backend Rust (producer, contract, cue threading, retrieval) with no UI to build; design binds only at one downstream seam, the diagnostic report's "Previously Seen" section (`pulse-app/ui/src/report/ReportRenderer.tsx`), whose population this repair changes.

## Constraints
- Any UI touched must resolve every style value through Tailwind v4 `@theme` `var(--*)` custom properties — no inline hex or magic pixel values (per design-system.md §Anti-Patterns → Universal Bans; §Self-Validation Protocol #4 Token Test).
- If a fingerprint/hash value is ever surfaced to the operator, it takes the Data/Code role — JetBrains Mono, 12px, tabular numerals — never IBM Plex Sans; monospace encodes "immutable telemetry fact" and the split is mandated, not stylistic (per design-system.md §Typography).
- A section that newly becomes non-empty must render instantly: entrance animations are specified as **none**, and staggered reveals beyond component load are a hard limit at expression 0.3–0.35 (per design-system.md §Motion → Hard limits).
- Any "previously seen"/match state indicator must pair color with a text label or icon; Accent `#C7556A` is a **non-text** token (≈3.8:1 on Base) and body-size state text must use `--color-text-primary` with an accent border/icon (per design-system.md §Color Palette → Accent usage (non-text token)).
- If an empty or error presentation is added for the retrieval result, reuse the shared `EmptyState` — message text on `#B4BCCB` (Secondary), never `#7D8697` (Tertiary, large-text-only) — with the honest-error variant checked BEFORE the empty branch so a query failure never reads as "no matches" (per design-system.md §Surface: desktop-webview → Component Patterns → Loading / Empty States).
- Containers added to the report surface follow borders-only depth: Raised-1 `#262A33` + 1px `rgba(74,144,226,0.3)`, `radius-md` 6px, `space-md` 16px padding — no shadows, no gradients, no glassmorphism (per design-system.md §Depth Strategy; §Component Patterns → Cards / Panels).

## Patterns to follow
- The existing report-renderer token discipline (every `style` value via `var(--*)`, no hex literals) — extend it, do not re-style around it (per design-system.md §Surface: desktop-webview → Tokens (platform-specific)).
- Shared `EmptyState` (decorative glyph `aria-hidden` + message + optional actionable hint) and its distinct honest-error variant, already reused across Metrics / Logs / Snapshots (per design-system.md §Component Patterns → Loading / Empty States).
- Telemetry-data presentation pattern: JetBrains Mono 12px, tabular numerals, `space-sm` (8px) cell padding, 1px `rgba(74,144,226,0.1)` row borders (per design-system.md §Component Patterns → Tables (telemetry data)).
- Custom Observatory glyph set via `<Icon glyph="…" />` (aperture / telescope / constellation-grid / star / circular-pulse) before reaching for Lucide/Heroicons fallbacks (per design-system.md §Iconography).

## Anti-patterns to avoid
- Hardcoded hex / pixel literals or Tailwind default palette colors anywhere in touched UI (per design-system.md §Anti-Patterns → Universal Bans).
- Animating the newly-populating section in — no staggered reveal, no scroll-triggered reveal, no opacity fade beyond 200ms (per design-system.md §Motion → Hard limits).
- Color-alone conveyance of a match / anomaly state (per design-system.md §Anti-Patterns → Universal Bans, "NEVER use color purely for decoration"; §Color Palette Accent note).

## Contract bindings
- **Token contrast ↔ a11y §Contrast (SC 1.4.3):** any body-size text added on the report surface needs ≥4.5:1 — Secondary `#B4BCCB` (≈6.8:1) is the floor; Tertiary `#7D8697` (≈4.2:1) is large-text-only (design-system.md §Text Hierarchy).
- **State color + label ↔ a11y §Use of Color (SC 1.4.1):** the existing degraded-mode banner precedent (icon + text label) governs any new state signal.
- **Motion ↔ a11y §Animation (SC 2.3.3):** only binds if motion is introduced; `prefers-reduced-motion: reduce` override is mandatory (design-system.md §Motion → Accessibility).
- **tests specialist:** design supplies the lint targets — banned fonts, banned effects, the 7 motion hard limits, and Component-Pattern hex/spacing/radius verification (per design-system-amendments.md §Downstream Readiness → "For tests specialist").
- **Research question (not a design assertion):** whether `Incident.fingerprint` reaches any rendered surface at all — the TauRPC `IncidentRecord` view is documented as dropping it as an internal grouping identifier — and whether repairing the producer makes the "Previously Seen" section non-empty for the first time in production, is P3's to answer. If the answer is "no rendered value changes," design collapses to out-of-scope for this chunk.

## Acceptance criteria contributions
- (design) If any UI is touched, every style value resolves through `var(--*)` `@theme` custom properties; no inline hex or magic pixel literals (per design-system.md §Anti-Patterns → Universal Bans).
- (design) Any fingerprint/hash value rendered uses the Data/Code role — JetBrains Mono 12px, tabular numerals — not IBM Plex Sans (per design-system.md §Typography).
- (design) A "Previously Seen" section that newly becomes non-empty renders instantly — no entrance animation, no staggered reveal, no fade >200ms (per design-system.md §Motion).
- (design) Any empty/error presentation added reuses the shared `EmptyState` with message text on `--color-text-secondary` `#B4BCCB` and the error variant checked before the empty branch (per design-system.md §Surface: desktop-webview → Component Patterns → Loading / Empty States).

## Relevant amendment history
- **2026-07-08-self-explaining-empty-states** — empty-state MESSAGE token corrected Tertiary `#7D8697` → Secondary `#B4BCCB` (body-size text needs ≥4.5:1, SC 1.4.3), and the honest-error `EmptyState` variant was documented (static "Couldn't load …", no actionable hint, checked before the empty branch). Directly governs any "no previously-seen matches" or retrieval-failure presentation this chunk's repair might expose — do not re-derive a bespoke empty state, and do not let a failed retrieval read as "no matches".
- **2026-05-03 — Accent lift `#8B2E3B` → `#C7556A` + non-text reclassification** — Accent is a non-text token (≈3.8:1 on Base): clears SC 1.4.11/large-text, fails SC 1.4.3 normal-text. Relevant because a "previously seen"/anomaly indicator is the kind of signal that reaches for Accent; body-size text must use `--color-text-primary` with an accent border/icon instead.
- **2026-05-29 — Halo State Pulse re-driven by cumulative incident severity + activity + connection state** — noted as a guard, not an action: the Halo is the one design surface driven by `Incident` data, but its hue axis reads cumulative incident **severity** (max active-incident priority tier), not `fingerprint`. Since the scope keeps coalesce-per-cue-identity DECIDED and defers per-fingerprint dedupe, no Halo-visible change is expected from this chunk; if P3 finds incident counts do shift, that becomes a design-visible consequence and should be raised rather than carried silently.
