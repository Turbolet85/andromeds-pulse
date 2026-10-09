# design extract

## Relevance
Relevant — chunk controls which services render visible constellation dots and animated Halo State Pulse; affects empty-state rendering (zero-telemetry honesty) and recency-label semantics, all design-domain surfaces.

## Constraints
1. Halo State Pulse breathing (4–5 s quiet → ~2 s active, ≈0.2–0.5 Hz, opacity + blur ONLY, never scale) applies ONLY to currently-live services per activity state; historical/stale/dormant show static glow, dimmed color, or hidden per design-system §Brand Identity §Signature element & P-026.
2. If mark-historical chosen: state distinction must pair visual color with label or icon badge, never color alone (design-system §Color Palette "Accent usage (non-text token)" & WCAG SC 1.4.1).
3. Recency/last-span labels render using Typography tokens (Label 12px or Data 12px per §Typography) paired with semantic colors (Secondary #B4BCCB for neutral recency; Alert Burgundy #C7556A only with icon/label; never color alone).
4. Zero-telemetry empty state renders per Component Patterns §Loading / Empty States: centered text/icon, #7D8697 Tertiary, no spinning animation per 0.35 expression level.
5. Connection state (live vs offline/disconnected) visually distinct via orthogonal grayout/desaturation axis per §Brand Identity & P-004, independent of the liveness-color axis.

## Patterns to follow
1. Halo animates only for currently-live services; on transition to Silent/Dormant/Archived, breathing halts, glow static/dim (breath period = activity-driven, not throughput; §Brand Identity).
2. Empty state (zero live services) follows §Loading / Empty States: centered text (#7D8697), optional telescope icon (24px), background #1A1D24, no animation.
3. If services marked historical: Raised-1 (#262A33) bg + Subtle border + secondary/tertiary text + optional "historical" icon/badge — signal reduced priority without animation.

## Anti-patterns to avoid
1. NEVER animate Halo breathing on historical/silent/dormant services — breathing is a live-activity signal.
2. NEVER use color alone to distinguish live vs historical (SC 1.4.1) — pair state color with label/badge/icon.
3. NEVER imply current liveness in recency labels ("just now") for restored-but-unseen services — timestamp must reflect actual last-observed time.

## Contract bindings
- **Halo animation** ↔ a11y §Animation (SC 2.3.3: prefers-reduced-motion override mandatory; Halo → static glow, no rhythm).
- **State color + label pairing** ↔ a11y §Use of Color (SC 1.4.1).
- **Recency label text** ↔ a11y §Contrast (Secondary ≈6.8:1, Tertiary ≈4.2:1 large-text only).

## Acceptance criteria contributions
1. (design) Currently-live services render animated Halo State Pulse (breathing 4–5 s / 2 s, opacity+blur per §Brand Identity P-026).
2. (design) Historical/stale/dormant render static glow or hidden halo; no pulsing rhythm (§Brand Identity activity-driven breathing).
3. (design) State distinction pairs color with label/badge icon, never color alone (SC 1.4.1; §Color Palette non-text token).
4. (design) Recency labels use correct Typography tokens + Text-Hierarchy colors and reflect ACTUAL last-observed time, not boot-reset.
5. (design) Zero-telemetry empty state follows §Loading / Empty States (no animation, centered text + optional icon, #7D8697, instant render).
6. (design) All motion respects `prefers-reduced-motion: reduce`; Halo → static glow; recency updates instant (§Motion Accessibility).

## Relevant amendment history
- **2026-05-29** — Halo re-driven by activity state (not throughput), breathing 4–5 s quiet → ~2 s active; connection state = orthogonal grayout/desaturation axis (P-004). Direct application: this chunk's liveness classification drives whether each service's Halo breathes; only activity-state-marked services animate.
- **2026-05-03** — Accent #C7556A reclassified as non-text token. If mark-historical: use #C7556A for badges/border, paired with secondary text label/icon for SC 1.4.1 (never color alone).
