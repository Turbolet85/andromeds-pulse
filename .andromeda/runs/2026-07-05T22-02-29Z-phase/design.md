# design extract

## Relevance
partial — backend data-flow / workspace-key reconciliation; it changes NO design token, type, motion, icon, or component pattern. My domain contributes only the guardrails on the severity encoding this chunk finally lights up, plus the not-color-alone acceptance check the scope already names ("hue + non-color token").

## Constraints
- Severity hue resolves on the existing Earth Blue `#4A90E2` → Alert Burgundy `#C7556A` LCH axis, driven by **cumulative incident severity** (max active-incident priority tier); this chunk supplies that input and MUST NOT introduce an alternate/hardcoded color model (per design-system §Brand Identity signature element + §Motion High-impact moments).
- Health-vs-severity orthogonality (P-004): connection state is a *separate* grayout/desaturation axis, independent of the severity-hue axis; the reconciled query drives SEVERITY (incident tier) only — feeding it must not desaturate/grayout, and vice versa (per design-system §Motion, amendment 2026-05-29).
- State is never conveyed by color alone: the non-healthy dot's hue must be paired with a non-color token (severity label/icon/badge) (per design-system §Anti-Patterns → Universal Bans "never use color purely for decoration" + §Color Palette "Accent usage (non-text token)").
- Accent `#C7556A` is a NON-TEXT token (halo/border/icon/badge, ≈3.8:1 on Base `#1A1D24`) — valid for the severity glow/token, but not for body-size severity text; body-size labels use `--color-text-primary` + accent border/icon (per design-system §Color Palette "Accent usage (non-text token)").
- Burgundy strictly means anomaly: a genuinely zero-active-incident result after reconciliation must render the calm Earth-Blue baseline, never false-anomaly burgundy (per design-system §Color Palette Semantic Colors + §Anti-Patterns "every color communicates meaning").

## Patterns to follow
- Halo State Pulse severity encoding (design-system §Brand Identity) — the already-built signature the reconciled data lights up. Feed the "cumulative incident severity" input; do not reshape the LCH hue mapping (P-069 / chunk #90-#91 own it).
- Semantic Colors state model (design-system §Color Palette Semantic Colors) — reuse existing tokens: Earth Blue baseline for healthy, Alert Burgundy border/icon for anomaly. No new hues.
- Non-text-token usage of accent (design-system §Color Palette "Accent usage") — severity surfaced via border/icon/badge alongside a text/icon label, matching the P-069 render this chunk feeds.

## Anti-patterns to avoid
- NEVER convey severity by hue alone — the non-healthy dot must carry a non-color token (label/icon) (design-system §Anti-Patterns → Universal Bans; binds a11y SC 1.4.1).
- NEVER introduce a new/hardcoded or Tailwind-default severity color — the NASA palette (`#4A90E2` / `#C7556A`) is the source of truth (design-system §Anti-Patterns → Universal Bans).

## Contract bindings
- Severity hue + non-color token → a11y §Use of Color SC 1.4.1 (not-color-alone); the scope's "hue + non-color token" acceptance IS this binding.
- Accent `#C7556A` as non-text token (≈3.8:1) → a11y §Contrast SC 1.4.11 (non-text 3:1); any severity TEXT at body size in burgundy fails SC 1.4.3 (4.5:1) — use text-primary + accent border/icon.
- "Cumulative incident severity" the Halo consumes → obs/data-viz Halo-color-state hook (design-system §Downstream Readiness "For obs specialist"): this chunk is the upstream reconciliation that finally makes that logged severity non-zero.

## Acceptance criteria contributions
- (design) The non-healthy constellation dot conveys severity via hue PLUS a non-color token (severity label/icon/badge), not color alone (design-system §Anti-Patterns / §Color Palette; binds a11y SC 1.4.1).
- (design) Severity hue resolves on the existing Earth Blue `#4A90E2` → Alert Burgundy `#C7556A` LCH axis — the wiring introduces no new or hardcoded severity color (design-system §Brand Identity / §Color Palette).
- (design) The genuinely-healthy (zero-active-incident) state still renders the calm Earth-Blue baseline — reconciliation must not paint false burgundy anomaly (design-system §Color Palette Semantic Colors).
- (design) Connection-state grayout stays orthogonal to the severity-hue axis — feeding severity data must not trigger desaturation (design-system §Motion, P-004).

## Relevant amendment history
- **2026-05-29 — Halo re-driven by incident severity + activity + connection state.** Directly on-point: this amendment made "cumulative incident severity (max active-incident priority tier)" the hue driver (superseding the pre-#90 throughput/error-rate model) and added connection state as an orthogonal grayout axis (P-004). THIS chunk is the data-flow reconciliation that finally delivers non-zero cumulative severity to that axis — the amendment defines the target the wiring must light up, and confirms severity ≠ connection state.
- **2026-05-03 — Accent `#8B2E3B` → `#C7556A`, reclassified NON-TEXT.** Relevant because the non-healthy severity endpoint is exactly `#C7556A`; its non-text classification governs how the lit-up severity is expressed (border/icon/badge + text-primary label, never burgundy body text) when the acceptance proof drives a real storm.
