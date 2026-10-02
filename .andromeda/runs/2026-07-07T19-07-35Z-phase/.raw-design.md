# design extract

## Relevance
Partial — this is a rendered text surface (desktop-webview): typography + text-color + state-color-not-alone apply directly; Halo/motion/depth/layout are mostly out of scope.

## Constraints
- **Monospace-vs-sans semantic split is mandatory and load-bearing here.** The worded line mixes human summary + telemetry values; surrounding words render in IBM Plex Sans (Body 14px / Label 12px), while the embedded numerics (service count, `~540 spans/s`, `buffer 2 min / 10 min`) render in JetBrains Mono **Data** role (12px, tabular-nums) — mono = "immutable telemetry fact," sans = "human-readable summary" (per design-system.md §Typography; this split is a locked, domain-encoding decision, not aesthetic).
- **Text color from the hierarchy, not Tertiary at body size.** Line text uses `--color-text-primary` #E8EEF7 (~8.5:1); de-emphasized metadata may use `--color-text-secondary` #B4BCCB (~6.8:1). `--color-text-tertiary` #7D8697 is **large-text-only** (~4.2:1) — do not use it for 12–14px segments (per §Color Palette → Text Hierarchy).
- **Connection state conveyed by words, and any state color derives from the semantic palette** (Info #4A90E2 / Success #17B3A3 / Warning-Error #C7556A) — never a hardcoded or decorative hue (per §Color Palette → Semantic Colors; §Anti-Patterns → Universal Bans "NEVER use color purely for decoration").
- **Tokens only — no hardcoded hex/px.** All color/font/spacing via `--color-*`, `--font-*`, `--spacing-*` (per §Surface: desktop-webview → Tokens).
- **Live values update instantly — no transition/entrance animation on tick.** The spans/s counter and buffer readout confirm state, they don't animate; matches Tables "instant change" and Motion "Entrance: none" (per §Motion; §Surface: desktop-webview → Tables).
- **Inline gaps use the low end of the scale** — `·` separators / icon-to-text via space-micro (2px) / space-xs (4px) (per §Spacing).

## Patterns to follow
- **Read-only footer/summary precedent** — §Surface: desktop-webview → Navigation Pattern already defines a compact-widget footer "Ingest: {spans/sec} · Retention: {min} · Error rate: {%}", read-only, no interactions. The dashboard worded line is the plain-language expansion of exactly this; reuse its read-only, glance-only treatment.
- **Data-cell typography** — mirror §Surface: desktop-webview → Tables "Data cells" (JetBrains Mono 12px, tabular-nums) for the numeric segments so the status line reads consistently with trace latency/count rows.
- **Empty/zero state** — the honest "No telemetry yet — listening on :4317/:4318" phrasing follows §Surface: desktop-webview → Loading/Empty States (contemplative copy, Tertiary tone reserved for genuinely large/centered empty text; keep the primary readout at Primary/Secondary).
- **Contemplative, observational voice** — terse, data-driven state phrasing ("Receiving from 5 services") per §Brand Identity and the desktop-native Tray Menu tone note.
- **ConnectionDot (CARRY)** — the dot is a status badge (radius-full per §Border Radius) whose color derives from Semantic Colors; its tooltip surface is Raised-2 #2D3139 (§Color Palette → Surface Scale). The tooltip text is the dot's not-color-alone pairing.

## Anti-patterns to avoid
- **Color alone for connection state** — the worded line is the non-color conveyance by construction; do not regress the ConnectionDot into color-only (per §Anti-Patterns → Universal Bans; binds a11y SC 1.4.1).
- **Generic/system fonts or hardcoded palette** — must use bundled JetBrains Mono + IBM Plex Sans and NASA-palette tokens (per §Anti-Patterns → Universal Bans; banned: Inter/Roboto/Arial/system-ui/etc.).
- **Animating the live counter** — no opacity fade >200ms, no per-tick entrance/transition (per §Motion → Hard limits).

## Contract bindings
- **Token contrast → a11y §Contrast (SC 1.4.3):** chosen text tokens must meet contrast at the rendered size; the 12px Data-role numerics are normal-text, so they must be Primary/Secondary, not Tertiary. Design supplies target ratios; a11y owns formal conformance. (Scope notes this render may be "contrast-exempt, TBD P4/P5" — the token-choice binding still holds.)
- **State color + label → a11y SC 1.4.1 (not-color-alone):** the worded line satisfies this for connection state; the CARRY's ConnectionDot binds here via its tooltip pairing.
- **Motion → a11y SC 2.3.3:** minimal — no chrome motion is added, so no reduce-motion override is introduced by this chunk (flag only if a live-update transition is added against the constraint above).

## Acceptance criteria contributions
- (design) Status line uses only design tokens — no hardcoded hex/pixel values (§Surface: desktop-webview → Tokens).
- (design) Numeric telemetry segments (source count · spans/s · buffer min) render in JetBrains Mono Data role with tabular-nums; surrounding words in IBM Plex Sans (§Typography).
- (design) Line text uses `--color-text-primary` (or `--color-text-secondary`), never `--color-text-tertiary` at 12–14px (§Color Palette → Text Hierarchy; binds a11y SC 1.4.3).
- (design) Connection state is stated in words with any state color drawn from the semantic palette, never color-alone (§Color Palette; §Anti-Patterns).

## Relevant amendment history
- **2026-05-03 — Accent lifted to #C7556A + reclassified non-text.** Directly constrains this chunk: for any "disconnected"/error worded status at body size (≤14px), do **not** use accent #C7556A as the text color (3.8:1, below SC 1.4.3 4.5:1) — use `--color-text-primary` with an accent border/icon instead. Accent is an input-border/icon/badge token only. (Reason: chunk #12 contrast harness; a11y > design on conflict.)
- **2026-05-29 — Halo re-driven; connection state added as an orthogonal grayout/desaturation axis (P-004).** Establishes how connection is *visually* encoded elsewhere (desaturation = not-live, orthogonal to severity hue). The worded line is the plain-language complement of that model, so keep its connected/stale semantics consistent with the grayout axis and with the P-067 live-only recency gate the scope requires. The Halo itself is out of this chunk's scope.
