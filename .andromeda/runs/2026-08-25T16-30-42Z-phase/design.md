# design extract

## Relevance
partial — the chunk is overwhelmingly backend/tooling/spec work, but the headline ManualCheck leg ends in an operator judging a *visible* incident state (service dot) and the sustained-mode rationale turns on the traces view's no-data surface.

## Constraints
- The shipped carrier of incident severity on desktop-webview is the constellation **dot** hue (Earth Blue #4A90E2 → Alert Burgundy #C7556A, LCH, driven by cumulative incident severity); design-system §Brand Identity → Signature element records the Halo canvas layer as SPECIFIED-but-unbuilt on this surface with build-or-retire owned by a *separate* working-route entry. This chunk's proof must neither depend on the halo layer nor opportunistically build it (per design-system §Brand Identity).
- Alert Burgundy is a **non-text token** (borders, icons, badges, dot fills) at ≈3.8:1 on Base — it clears SC 1.4.11 non-text but not SC 1.4.3 normal text; body-size text about the incident must use `--color-text-primary` with the accent as border/icon (per design-system §Color Palette → Accent usage (non-text token)).
- State must never be conveyed by color alone — a red dot the operator reads as "incident" needs an accompanying label, icon, or text in the brief (per design-system §Color Palette → Accent usage / §Anti-Patterns "NEVER use color purely for decoration"). Whether the dashboard's dot already carries a paired label/icon is research's question.
- If any no-data / failed-query path in the traces view is touched or asserted (the scope's "table honestly reads *No traces yet*"), design-system §Surface: desktop-webview → Component Patterns → Loading / Empty States requires the shared `EmptyState` with message in `#B4BCCB` (Secondary, not Tertiary), decorative glyph `aria-hidden`, ports in `font-code`, and the **error variant checked BEFORE the empty branch** so a failure never masquerades as "no data".
- Any UI value introduced must trace to a token — no raw hex, no magic px (per design-system §Self-Validation Protocol #4 Token Test; §Surface: desktop-webview → Tokens: `--color-*`, `--spacing-*`, `--radius-*`, `--font-*`, `--duration-*`, `--easing-*`).
- No new chrome motion is warranted by this chunk; should any appear (e.g. a live-feed indicator during the sustained run), the 0.35-webview budget caps it at 150 ms hover/focus, 200 ms fade, no entrance animations, and it must honor `prefers-reduced-motion` (per design-system §Motion → This project's values, Hard limits, Accessibility).

## Patterns to follow
- Severity→hue mapping already expressed on the dot (`severityToHueFraction` is the mechanism named in design-system §Brand Identity) — the real-L4 leg should observe *that* path, not a new visualization.
- Shared `EmptyState` primitive (glyph + message + optional actionable hint) reused across Metrics / Logs / Snapshots, with its distinct honest-error variant (per design-system §Surface: desktop-webview → Loading / Empty States).
- Connection state is an **orthogonal** grayout/desaturation axis, independent of the severity-hue axis (per design-system §Brand Identity, §Motion high-impact moment 1) — a stalled injector must not read as an incident, which is the visual analogue of the chunk's own `rows_ingested > 0` feed-precondition idea.
- Data/telemetry values render in JetBrains Mono 12px with tabular numerals; operator-facing prose in IBM Plex Sans (per design-system §Typography) — relevant if the interpretation brief or any smoke output is surfaced in-app.

## Anti-patterns to avoid
- NEVER introduce a new canvas/WebGL layer as part of this proof — the only exemption to the canvas ban is the dedicated Halo State Pulse layer, whose disposition is another entry's (per design-system §Motion → Hard limits; §Anti-Patterns → Universal Bans).
- NEVER substitute a generic status glyph (green check / red X) for the two-dimension encoding (activity rhythm + severity hue) when demonstrating the incident (per design-system §Anti-Patterns → Rejected Defaults).
- NEVER let a demo/injector-driven path hardcode palette hex or Tailwind default colors into any surfaced output (per design-system §Anti-Patterns → Universal Bans; §Self-Validation Protocol #4).

## Contract bindings
- **design ↔ tests (lint targets):** scope item 4c may relocate inline `--rule` args into `pulse-app/ui/eslint.config.mjs`. design-system-amendments §Downstream Readiness → "For tests specialist" assigns design-derived lint targets (banned fonts, motion durations/easings, the 7 motion hard-limit bans) to that same harness — the relocation must not drop or shadow them.
- **design ↔ a11y:** accent/base non-text classification (SC 1.4.11) and not-color-alone (SC 1.4.1) for the red service dot; contrast targets in §Color Palette → Text Hierarchy are design's numbers, a11y owns the formal conformance derivation.
- **design ↔ obs:** if the real-L4 leg instruments the severity→hue path, design-system-amendments §Downstream Readiness → "For obs specialist" specifies logging the halo/pulse color state and frequency **as-is, without re-clamping**.

## Acceptance criteria contributions
- (design) The ManualCheck proof's visible incident signal is the constellation dot's severity hue shift toward Alert Burgundy; passing does NOT require the Halo canvas layer (per design-system §Brand Identity → Signature element).
- (design) Any incident state the operator reads is paired with a text label or icon, never color alone, and body-size incident text uses `--color-text-primary` with accent as border/icon (per design-system §Color Palette → Accent usage (non-text token)).
- (design) If the traces no-data path is exercised or asserted, the error variant is evaluated before the empty branch and message text uses `#B4BCCB` Secondary (per design-system §Surface: desktop-webview → Component Patterns → Loading / Empty States).
- (design) Any UI value this chunk adds or edits resolves through a token — no hardcoded hex or px literal (per design-system §Self-Validation Protocol → #4 Token Test).

## Relevant amendment history
- **2026-08-21-delegated-timing-observables** — recorded the Halo State Pulse canvas as specified-but-unbuilt on desktop-webview (three HEAD probes found no production render site) and named the constellation dot + `severityToHueFraction` as what actually carries severity hue; build-or-retire routed to the "Halo State Pulse canvas disposition" entry. *Why it matters here:* it fixes what the operator can legitimately be asked to observe in this chunk's live proof, and keeps the halo out of scope.
- **2026-07-08-self-explaining-empty-states** — corrected the empty-state message token Tertiary→Secondary and documented the shared `EmptyState` plus the honest-error variant checked before the empty branch. *Why it matters here:* the scope's sustained-mode rationale hinges on "No traces yet" being an honest read of an aged-out window rather than a masked failure — the same distinction the amendment encodes.
- **2026-05-29 (Halo re-driven) / 2026-08-23-a11y-verification** — established that hue is driven by cumulative incident severity (not error rate) with connection state as an orthogonal axis, and that accent is non-text-only in fact as well as policy. *Why it matters here:* the injector's 100%-error firehose vs. sustained-MODERATE choice changes what severity tier the visual actually reaches, and the accent classification bounds how the incident may be rendered in text.
