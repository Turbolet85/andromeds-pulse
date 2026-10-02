# design extract

## Relevance
partial — the poll mechanism / timer lifecycle / filter-state preservation is out-of-domain (route/frontend logic), but the rendered Traces table states (empty / populated / error / hover) and the empty→populated transition motion fall under design Component Patterns + Motion.

## Constraints
- Re-poll must preserve the **Tables (telemetry data)** pattern unchanged: data cells JetBrains Mono 12px tabular-nums, padding space-sm (8px), header #E8EEF7 600/12px on Base, row borders 1px rgba(74,144,226,0.1), hovered row rgba(74,144,226,0.1) with **NO transition (instant change per 0.35 expression)** — per design-system §Surface: desktop-webview → Component Patterns → Tables.
- "No traces yet" empty state shows only while genuinely empty: centered Tertiary #7D8697 text + optional telescope icon (24px, #7D8697) — per §Surface: desktop-webview → Component Patterns → Loading/Empty States and §Iconography (telescope glyph).
- If a refresh error surfaces: error text #C7556A + optional error icon, 1–2 lines; accent is a **non-text token**, so state must be conveyed by icon/label, never color alone — per §Color Palette (Accent usage — non-text token) + §Loading/Empty States.
- Empty→populated transition and per-poll updates are **state-confirming, not arriving**: NO entrance animation, NO staggered/sequential row reveal, NO opacity fade >200ms — per §Motion (Entrance: none; Hard limits). This is the design lever against the scope's "row-flicker that defeats the anomaly-first read."
- Any skeleton/loading pulse during refresh uses the skeleton spec (Raised-1 #262A33, opacity 0.5–1.0 pulse) and must degrade to static under `prefers-reduced-motion` — per §Motion (Accessibility) + §Loading/Empty States.
- Anomaly-first (erroring) rows carry semantic Alert Burgundy meaning, not decoration — per §Anti-Patterns (never use color purely for decoration) + §Color Palette (Semantic → Error).

## Patterns to follow
- **Tables (telemetry data)** pattern — reuse the existing P-068 table styling verbatim across refreshes; a re-poll updates data in place, it does not restyle (§Surface: desktop-webview → Component Patterns → Tables).
- **Loading / Empty States** pattern — the honest empty/error visuals ("No traces yet" Tertiary + telescope icon; error #C7556A + icon) already exist; drive transitions between them, don't invent new state visuals (§Component Patterns → Loading/Empty States).
- **Motion "confirms state changes only, not arrivals"** — data landing is a state change rendered instantly, not an entrance to animate (§Motion — Entrance: none; This project's values).
- **Design Direction — Data & Analysis** (fast interaction feedback, data-density priority) endorses the live-cadence refresh so the table promptly reflects buffer state (§Brand Identity — Design direction from library-shortlist).

## Anti-patterns to avoid
- NO staggered / sequential entrance of list items when rows land, and NO re-triggering the skeleton pulse on every poll (either reads as flicker) — per §Motion Hard limits.
- NO opacity fade longer than 200ms on the empty→populated swap — per §Motion Hard limits.
- Never use color purely for decoration — anomaly-first rows keep semantic Alert Burgundy, paired with a non-color cue — per §Anti-Patterns Universal Bans.

## Contract bindings
- **Motion tokens ↔ a11y §Animation (SC 2.3.3):** skeleton/loading pulse reduce-motion override is mandatory.
- **State color + label ↔ a11y §Use of Color (SC 1.4.1):** error/anomaly rows must not signal by color alone.
- **Token contrast ↔ a11y §Contrast (SC 1.4.3):** empty-state Tertiary #7D8697 is ~4.2:1 "large text only" — flag if "No traces yet" renders at body size (<large); a11y owns the formal check.
- **obs ↔ design (Downstream Readiness → For obs specialist):** obs may instrument transitions into/out of the loading/empty/error states without modifying visual tokens.

## Acceptance criteria contributions
- (design) Rows land empty→populated with no staggered/sequential entrance animation and no >200ms fade; the table renders instantly on each re-poll (§Motion Hard limits).
- (design) Refresh preserves the Tables pattern tokens — data cells stay JetBrains Mono 12px tabular-nums, hovered row instant (no transition); no new hardcoded hex/px introduced (§Component Patterns → Tables; Self-Validation Token Test).
- (design) "No traces yet" uses Tertiary #7D8697 + telescope icon and appears only while genuinely empty; any error state pairs #C7556A with icon/label, not color alone (§Loading/Empty States; §Color Palette Accent note).
- (design) Any skeleton/loading pulse shown during refresh respects `prefers-reduced-motion` (degrades to static) (§Motion Accessibility).

## Relevant amendment history
- **2026-05-03 — Accent lifted to #C7556A / reclassified as non-text token.** Directly governs the Traces error/empty rendering: accent clears non-text (3:1) but is below normal-text (4.5:1), so body-size error message text should use `--color-text-primary` with an accent border+icon (never color alone); the "Error state" pattern still retains #C7556A for the text role, with the text-role migration deferred to the error-UI chunk. Bears on how this chunk conveys a failed refresh / anomaly-first rows.
- (The 2026-05-29 Halo re-driven amendment is out of this chunk's area — it governs the constellation/Halo motion drivers, and the constellation is explicitly out of scope; noted only to confirm non-applicability.)
