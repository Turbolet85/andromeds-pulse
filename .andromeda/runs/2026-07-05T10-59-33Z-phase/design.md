# design extract

## Relevance
Relevant — surface chunk that renders the Traces table (text, numeric cells, a semantic error flag, and a filter control); tokens/typography/component-patterns apply.

## Constraints
- Anomaly/error semantic hue is **Alert Burgundy `#C7556A` (`--color-accent`)** — the locked "anomaly indicator" color; reuse it, do not invent a new error/red (per design-system.md §Color Palette → Core Colors "Accent"; §Anti-Patterns → Universal Bans "NASA palette is source of truth").
- `#C7556A` is a **NON-TEXT token** (≈3.8:1 on Base #1A1D24): valid for row border / error icon / alert badge, but body-size error/anomaly text (≤14px regular) must use `--color-text-primary` #E8EEF7 (per §Color Palette "Accent usage (non-text token)").
- Anomaly must be conveyed by **more than color** — accent paired with a text label or icon/badge, never color alone (per §Anti-Patterns → Universal Bans "NEVER use color purely for decoration"; binds a11y SC 1.4.1).
- Error/alert row boundary = **1px `rgba(199,85,106,0.5)`** (Alert Burgundy 50%); Error/Warning semantic rows use Base #1A1D24 background (per §Depth Strategy; §Color Palette → Semantic Colors).
- Numeric telemetry cells (error counts, latency) render in **JetBrains Mono 400 / 12px / tabular-nums** — "immutable telemetry fact" (per §Typography → Data; §Surface: desktop-webview → Component Patterns → Tables).
- **No new motion:** row re-order / hover state changes are instant (Tables pattern is "no transition per 0.35 expression"); nothing new to gate under reduce-motion (per §Motion → Hard limits; §Component Patterns → Tables).
- Filter control must be **keyboard-reachable** (Tab + `:focus-visible`, 3–4px `#4A90E2` focus ring) — no hover-only affordance (per §Surface: desktop-webview → Focus/Keyboard Navigation; §Per-Surface Bans → desktop-webview).

## Patterns to follow
- **Tables (telemetry data)** — §Component Patterns → Tables: transparent rows, hovered row `rgba(74,144,226,0.1)` (instant, no transition), header #1A1D24 / 12px / 600, data cells JetBrains Mono tabular-nums, 1px `rgba(74,144,226,0.1)` row separators. Reuse for the reordered + flagged rows.
- **Error state visual** — §Component Patterns → Loading/Empty/Error States (error text/icon, 1–2 lines) combined with §Color Palette → Semantic Colors (Error / Warning rows) for the row flag.
- **Filter toggle** — as a Secondary button (§Component Patterns → Buttons: #262A33 bg, #E8EEF7 text, border `rgba(74,144,226,0.3)`, hover border #4A90E2, radius-sm) or a pill (§Border Radius → radius-full for pills/toggles/badges).
- **Anomaly badge** — §Border Radius radius-full + §Spacing space-micro/space-xs internal padding; icon + text, not a bare color chip.
- **Iconography for the alert glyph** — §Iconography: no custom anomaly glyph exists in the primary set, so a Lucide/Heroicons fallback for validation/alert states is permitted (monochrome, 16px inline / 20px list-item).

## Anti-patterns to avoid
- NEVER convey the anomaly by **color alone** — always add label/icon/badge (§Anti-Patterns → Universal Bans; binds SC 1.4.1).
- NEVER invent a new error/red or use Tailwind default palette — the six-color NASA palette is source of truth (§Anti-Patterns → Universal Bans).
- NEVER add a hover-only sort/filter affordance without a keyboard path (§Per-Surface Bans → desktop-webview).

## Contract bindings
- **design ↔ a11y (Use of Color, SC 1.4.1):** anomaly hue must pair with a non-color cue — scope cites this directly.
- **design ↔ a11y (Contrast, SC 1.4.3):** `#C7556A` fails normal-text 4.5:1 (3.8:1) → any body-size anomaly text uses `--color-text-primary`; accent confined to border/icon/badge (non-text 3:1).
- **design ↔ viz-query / data-spine:** the semantic error token needs an error-count/status field on the trace row; if absent, a viz-query field addition is threaded through the established bindings (per scope §Boundaries) — design renders whatever error/status the row exposes.
- **design ↔ a11y (Motion):** none added — chunk introduces no new motion, so no new `prefers-reduced-motion` gate (per scope §Boundaries).

## Acceptance criteria contributions
- (design) Anomaly rows use only palette semantic error tokens (`#C7556A` for border/icon/badge) — no invented hex, no hardcoded pixel values (§Color Palette; §Anti-Patterns).
- (design) Anomaly is conveyed by color **plus** a text label or icon/badge, never color alone (§Color Palette; SC 1.4.1).
- (design) Any body-size anomaly/error message text (≤14px) uses `--color-text-primary` #E8EEF7, not #C7556A; accent appears only as border/icon/badge (§Color Palette "Accent usage" note).
- (design) Numeric error-count / latency cells keep JetBrains Mono tabular-nums; the filter control keeps existing button/pill tokens and shows a visible `:focus-visible` ring (§Typography → Data; §Component Patterns).

## Relevant amendment history
- **2026-05-03 — Lift `--color-accent` `#8B2E3B` → `#C7556A`** (directly governs this chunk's error token): reclassified accent as a **NON-TEXT token** (base contrast lifted 2.05:1 → 3.8:1; clears SC 1.4.11 non-text + SC 1.4.3 large-text, still below normal-text 4.5:1), and mandated `--color-text-primary` + accent border/icon for body-size error text (SC 1.4.1). It explicitly flags that the "Error state" component pattern still retains `#C7556A` for the text role, with the migration to `--color-text-primary` **pending the error-UI chunk** — anomaly-surfacing renders anomaly text, so treat that migration as live here: reserve `#C7556A` for border/icon/badge, prefer `--color-text-primary` for any message text. Why: chunk #12 contrast harness flagged accent/base at 2.05:1; a11y outranks design aesthetics under a11y-tier=Standard.
- (2026-05-29 Halo re-driven by incident severity is **out of this chunk's area** — it governs the WebGPU Halo canvas, not the Traces table; no action.)
