# Scope — Legible labeled constellation

**Marker:** `2026-07-05-legible-labeled-constellation` · **Capability:** P-069 · **Intent:** F9 · **Epoch 3 — State honesty & legibility**

## Capability statement
Each constellation dot is identifiable — its service name shows on/near the dot or on hover — and its health/severity is clearly encoded (design-system color + Halo), so the user can tell at a glance what they are looking at.

## Observed gap (intent F9)
The constellation dots are fuzzy, unlabeled blue blobs: no names, no glanceable health. The user cannot tell which service is which, nor which is healthy vs degraded, without leaving the constellation. This is the direct successor to P-067 (live-only service truth): P-067 made the dot set HONEST (only currently-live services render) and explicitly deferred labels + health-encoding to THIS chunk — P-067 chose "hide non-live" over "mark historical" precisely because "the canvas has no per-dot labels until P-069, so marking would be color-only → SC 1.4.1 violation." P-069 is the chunk that adds those labels and a non-color-only health cue.

## What this chunk builds
- **Per-dot identity.** Each rendered constellation dot exposes its service name — a persistent label on/near the dot and/or a hover/focus affordance — so every dot is identifiable.
- **Glanceable health/severity encoding.** Each dot's color encodes its health/severity via the design-system palette (+ Halo treatment), so the user reads service health at a glance.
- **Non-color-only health cue (a11y).** Because health is now surfaced per dot, the severity signal is carried by more than hue alone (the label text / a shape / an icon / a severity token) to satisfy WCAG SC 1.4.1 (use of color) — closing the gap P-067 flagged.

## Surfaces / contracts it touches (candidates — confirmed at /implement)
- Webview constellation renderer: `pulse-app/ui/src/dashboard/routes/traces/ConstellationCanvas.tsx` (the dot render surface) + the widget constellation pipeline (`constellation-pipeline.ts` / `constellation-types.ts` — dot-model construction) + `use-service-constellation.ts` (the resolver-polling hook feeding dots).
- Data source: `services.list_with_states` → `ServiceListPayload` / `ServiceListItem` — already carries service `name` + `state: ServiceLifecycleState` + `priority_tier` (chunk #91). Confirm at /implement whether the health/severity field the design wants is already present (likely no new TauRPC field → the low-cost no-new-namespace path).
- Design tokens: `.andromeda/design-system.md` health/severity color scale + Halo State Pulse mapping; a11y contrast (SC 1.4.11) + use-of-color (SC 1.4.1) for the dot + its label.

## Boundaries (explicitly NOT in this chunk)
- NOT the plain-language connection-status line (P-070 · F10) — words for sources-connected / spans-per-second / buffer-state is the next chunk.
- NOT re-touching liveness filtering (P-067 owns which dots are live) — P-069 builds on the already-honest live-only dot set.
- NOT anomaly surfacing in the Traces table (P-068 · shipped).
- NOT a new lifecycle state, corpus schema, or TauRPC namespace unless /implement research proves the needed health field is absent from `ServiceListItem`.
- Respect `prefers-reduced-motion: reduce` for any Halo motion (CLAUDE.md universal invariant — Halo degrades to static glow; hue still updates per severity).

## Open questions (resolved at /phase P4 via AskUserQuestion)
1. **Label placement** — always-visible text labels on/near every dot, vs hover/focus-only, vs hybrid (always-on when sparse, hover when dense)? Legibility vs canvas clutter + render cost.
2. **Label rendering surface** — draw text on the `<canvas>` itself vs a positioned DOM/HTML overlay layer above the canvas (affects a11y — DOM labels are screen-reader-reachable + selectable; canvas text is not).
3. **Health/severity source + palette** — which field drives dot color (lifecycle `state` vs `priority_tier` vs an incident-derived severity), and the exact design-system color mapping (+ Halo).
4. **Non-color cue form** — how the SC 1.4.1 non-color signal is carried (the text label alone suffices? a shape/icon/badge? a text severity token like the P-068 error tokens?).

## Acceptance (verification-matrix P-069 · method: webview)
Each rendered dot exposes its service name (persistent label or hover) AND a health/severity-encoded color; the health signal is not conveyed by color alone (SC 1.4.1).
