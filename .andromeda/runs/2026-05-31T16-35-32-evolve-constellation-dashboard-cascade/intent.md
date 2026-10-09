# Intent — constellation-dashboard-cascade

_Captured by /andromeda-evolve Phase 1 at 2026-05-31T16:35:32Z._

## Invocation
`/andromeda-evolve --allow-route-append ConstellationCanvas dashboard cascade`

## Brief intent (Phase 1b)
Register the deferred "ConstellationCanvas dashboard cascade" as a new route chunk.

## Depth (Phase 1c — grounded from source plan + codebase; 0/4 clarifying questions used)
- **Target:** route.md §2 Epoch 9 — Foundation v0.2.0 (append a new terminal chunk #93).
- **Change:** add chunk #93 covering the dashboard-side halo/constellation cascade deferred
  by chunks #90 (Halo formula refactor) + #91 (Service constellation rendering): migrate the
  full-window dashboard `pulse-app/ui/src/dashboard/routes/traces/ConstellationCanvas.tsx`
  + `pulse-app/ui/src/halo/HaloCanvas.tsx` to the new per-service severity/activity API;
  delete legacy `error-rate-to-blur.ts` + `throughput-to-hz.ts`; clean orphaned
  `use-widget-metrics.ts`; retype `HaloInput`.
- **Why now:** the route reached 92/92 (complete); this is the strongest forward candidate
  surfaced in the session-166 /andromeda-new-session dashboard (forward option a). Source plan:
  `docs/v0_2_0/pulse-v0_2_0-route.md` §91. Legacy files confirmed still present on disk
  (error-rate-to-blur.ts, throughput-to-hz.ts, use-widget-metrics.ts, dashboard ConstellationCanvas.tsx).
