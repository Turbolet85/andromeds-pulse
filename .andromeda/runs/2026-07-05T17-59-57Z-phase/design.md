# design extract

## Relevance
Relevant — tokens (color / typography / motion / iconography) + the Halo signature apply directly; per-dot label *placement* and canvas-vs-DOM surface are layout/a11y calls (out of my scope per focus guide).

## Constraints
- **Health/severity = the Halo LCH hue axis.** Dot severity is encoded by LCH interpolation Earth Blue `#4A90E2` (`--color-primary`) ↔ Alert Burgundy `#C7556A` (`--color-accent`) driven by *cumulative incident severity*; **connection state is an ORTHOGONAL grayout/desaturation axis, independent of the severity-hue axis** (P-004). This is the answer scaffold for scope open-question 3 (per design-system §Brand Identity "Signature element" + §Motion High-impact moment #1).
- **Palette is the sole color source — no new hex.** Dot fill + label color must derive from the Color World tokens (`#1A1D24` / `#4A90E2` / `#C7556A` / `#E8EEF7` / `#2C3E7F` / `#17B3A3`); relevant semantic states: Success `#17B3A3`, Warning/Error `#C7556A`, Info `#4A90E2` (per §Color Palette Core + Semantic Colors; §Anti-Patterns Universal Bans).
- **Non-color-only cue is mandatory.** Accent `#C7556A` is a **non-text token** (≈3.8:1 on base, below the 4.5:1 normal-text bar); severity must be paired with a second carrier — the label text, a shape, an icon, or a severity token — never hue alone (per §Color Palette "Accent usage (non-text token)" + §Anti-Patterns "NEVER use color purely for decoration"). Directly satisfies the chunk's SC 1.4.1 goal.
- **Label typography must honor the mono/sans split.** Service name renders on the type scale — either Label = IBM Plex Sans 500/12px (UI control identifier) or, since a service name is an "immutable telemetry fact" identifier akin to a trace ID, Data/Code = JetBrains Mono 12px; the split itself is the constraint, the specific role is an /implement call. Default label text color `#E8EEF7` (`--color-text-primary`) (per §Typography).
- **Reduce-motion override.** Any Halo motion respects `prefers-reduced-motion: reduce` — degrades to **static glow (no breathing rhythm) but hue still updates per severity**; breathing is opacity + blur only, never scale (P-026) (per §Motion Accessibility + High-impact moment #1). Echoes the scope's CLAUDE.md invariant.
- **Depth = borders-only; Halo is the ONLY gradient exception.** A DOM/overlay label is flat/matte (no shadow, no glassmorphism); the Halo (radial gradient + blur on the WebGPU canvas) is the sole place visual weight is permitted (per §Depth Strategy + §Anti-Patterns Universal Bans).

## Patterns to follow
- **Halo State Pulse canvas** — extend the existing signature layer (WebGPU, canvas background `#0F1117` Inset; blur 4–16px per cycle mapped from severity); the dot's health color is this same hue axis, not a new mechanism (§Brand Identity; §Surface: desktop-webview "Canvas Container").
- **Iconography custom set** — if the non-color cue is an icon, use the registered domain glyphs `<Icon glyph="…" />`, monochrome `#E8EEF7`, sized 16/20/24px; Lucide/Heroicons only as generic fallback (§Iconography).
- **Loading / Empty / Error states** — for absent/unnamed dots, reuse Empty pattern (centered Tertiary `#7D8697` text, optional telescope glyph 24px); Error text `#C7556A` (§Surface: desktop-webview "Loading / Empty States").
- **Focus / Keyboard navigation** — if a label gains a hover/focus affordance, it must be Tab-reachable with the standard focus ring (3–4px outset `#4A90E2`, `box-shadow 0 0 0 3px rgba(74,144,226,0.2)`, `:focus-visible`) (§Surface: desktop-webview "Focus / Keyboard Navigation").

## Anti-patterns to avoid
- **No uniform monochrome status icons (green check / red X) or a generic colored dot** in place of the two-dimensional Halo encoding (breathing = alive, hue = severity) (§Anti-Patterns "Rejected Defaults").
- **No color-alone health signal** — hue without a paired label/shape/icon/token is a color-for-decoration ban + SC 1.4.1 fail (§Anti-Patterns Universal Bans).
- **No hardcoded hex / magic numbers / banned fonts** (Inter, Roboto, Arial, Helvetica, etc.) for the label or dot — tokens + JetBrains Mono/IBM Plex Sans only (§Anti-Patterns Universal Bans; §Self-Validation Token Test).

## Contract bindings
- **Token contrast ↔ a11y §Contrast:** label text on canvas Inset `#0F1117` must clear SC 1.4.3 4.5:1 (`#E8EEF7` primary satisfies); accent `#C7556A` dot fill is non-text SC 1.4.11 (3:1). a11y owns the formal derivation.
- **State color + label ↔ a11y SC 1.4.1 (use of color):** the chunk's central not-color-alone requirement — design supplies the paired-carrier rule, a11y verifies.
- **Motion ↔ a11y SC 2.3.3 / 2.3.1:** reduce-motion override mandatory; Halo cadence 0.2–0.5 Hz ≪ 3 Hz three-flashes threshold.
- **Severity source ↔ data contract:** the hue-driving field ("cumulative incident severity") maps to `services.list_with_states` → `ServiceListItem` (`state` / `priority_tier`) or an incident-derived severity — resolve field at /implement (scope open-question 3); no new TauRPC namespace unless the field is proven absent.

## Acceptance criteria contributions
- (design) Dot health/severity color derives only from design-system tokens along the Earth Blue ↔ Alert Burgundy LCH severity axis, with connection state as an orthogonal grayout — no hardcoded hex.
- (design) The health signal is paired with a non-color carrier (label text / shape / icon / severity token), never hue alone (SC 1.4.1).
- (design) Service-name label uses the type scale (IBM Plex Sans Label or JetBrains Mono Data, honoring the mono/sans split) with `--color-text-primary`; no banned fonts.
- (design) Any Halo motion respects `prefers-reduced-motion: reduce` — static glow, hue still updates per severity.

## Relevant amendment history
- **2026-05-29 — Halo re-driven by incident severity + activity + connection state** (supersedes the throughput+error-rate model). Directly resolves scope open-question 3: dot hue = *cumulative incident severity* (max active-incident priority tier), **not** error rate; **connection state = orthogonal grayout axis (P-004)**; breathing = activity state. This is the authoritative mapping the health-encoding must follow.
- **2026-05-03 — Accent `#8B2E3B` → `#C7556A`, reclassified as a non-text token** (≈3.8:1 on base). Constrains the severity/dot color: at body-text size accent fails 4.5:1, so a per-dot severity color demands the label/icon pairing to satisfy SC 1.4.1 — reinforcing this chunk's non-color-only requirement.
