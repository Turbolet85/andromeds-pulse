# layouts extract

## Relevance
partial. The chunk changes the timing observable behind the constellation dot hue (`ConstellationCanvas.tsx` fire site, the `ServiceListItem` payload). It adds no surface, region, focusable element, modal or breakpoint. Layout's job here is to make sure the rendered constellation hero, and where it sits, stay as they are.

## Constraints
- The surface under measurement is the live constellation dot hue keyed to cumulative incident severity (tiers none/curious/suggested/autonomous), not the Halo State Pulse canvas. layout-templates §Component — Halo State Pulse canvas (status note) names `metric.constellation.hue_update_ms` as measuring "that real surface", so the corrected observable must stay anchored to the dot-hue render path (per layout-templates §Component — Halo State Pulse canvas; §Surface: desktop-webview Signature placement).
- The Halo State Pulse glow layer is DEFERRED to the next version on every surface (webview per-service halos, compact-widget aggregated badge, tray). The chunk must not build it, mount it or instrument it. A timing mark placed on halo/`HaloCanvas` code would fire from a site that has no production render (per layout-templates §Component — Halo State Pulse canvas; §IA notes Multi-surface coordination).
- The constellation hero on the Full dashboard Traces view is a fixed (`flex-shrink:0`) region above the `Errors only` toolbar and the flex-filling table, inside a bounded flex column with no outer page scroll. Any render-path change to the canvas (e.g. re-rendering per changed service) must leave that region's placement and fixed sizing as they are (per layout-templates §Wireframe — Full dashboard (Traces primary screen) Wireframe notes).
- The hero is a `<section>` landmark with the STABLE literal accessible name `"Telemetry traces chart"`, described by the visually-hidden `constellation-summary` element, and no service name enters the accessible tree. Per-changed-service emission must add no data-tracking name, no per-service label and no live-region role to the hero (per layout-templates §Wireframe — Full dashboard (Traces primary screen) Wireframe notes).
- Under `prefers-reduced-motion: reduce` the hue still updates per cumulative incident severity; the rule attaches to whichever surface renders, which today is the constellation dot. The hue-shift observable therefore has to exist and be gradable under reduced motion as well. Whether the fire site is gated on a motion path is research's question (per layout-templates §Component — Halo State Pulse canvas, Reduced motion).
- The constellation renders on both the Compact widget and the Full dashboard Traces view. Which of these mount `ConstellationCanvas` (and therefore emit P-025) is research's question; the plan does not settle it (per layout-templates §Primary screens).

## Patterns to follow
- Treat the status note as authoritative wherever an ASCII wireframe disagrees with it. The sketches keep their "Halo pulses" labels only as illustration; the dot hue is the real signature (per layout-templates §Component — Halo State Pulse canvas; §Wireframe — Full dashboard Wireframe notes on sketch lag).
- The per-dot always-on name and severity-token labels (P-069) are part of the hero's current layout truth. Leave them as they are while changing what the fire site measures (per layout-templates §Wireframe — Full dashboard (Traces primary screen) Wireframe notes).
- The compact widget and full dashboard share data feeds rather than per-surface fetches. A new `tier_effective_at` field should travel on the existing shared payload, not on a surface-specific channel (per layout-templates §IA notes Navigation model / Multi-surface coordination).

## Anti-patterns to avoid
- Do not instrument or reintroduce `HaloCanvas` or any halo-glow layer as the P-025 anchor. The deferred spec is a design record, not a live surface (per layout-templates §Component — Halo State Pulse canvas).
- Do not give the hero landmark a dynamic name, and do not add `role="status"` to the summary element, as a side effect of per-service hue tracking (per layout-templates §Wireframe — Full dashboard (Traces primary screen) Wireframe notes).

## Contract bindings
- layouts ↔ a11y: the hero landmark's stable name, the `aria-describedby` summary and the exclusion of service names from the accessible tree bind to the a11y plan's region/landmark rules. Changing the canvas render path must not alter them (per layout-templates §Wireframe — Full dashboard (Traces primary screen)).
- layouts ↔ design: the dot hue interpolation `color-primary` → `color-accent` by cumulative incident severity is design-owned (design-system-amendments 2026-05-29). Layout only fixes where it renders; the tier-to-hue mapping is not this chunk's to change (per layout-templates §Surface: desktop-webview Signature placement).
- layouts ↔ obs: `metric.constellation.hue_update_ms` is the obs leaf measuring the dot-hue surface that layout-templates identifies as the real P-025 surface (per layout-templates §Component — Halo State Pulse canvas status note).

## Acceptance criteria contributions
- (layouts) The P-025 fire site sits on the constellation-dot render path in the Traces hero (and in the compact widget if it mounts the canvas). No `HaloCanvas`/halo-glow code is added or instrumented; a non-test grep for `<HaloCanvas` over `pulse-app/ui/src` still returns zero hits (per layout-templates §Component — Halo State Pulse canvas).
- (layouts) The Traces hero keeps its placement: fixed region above the `Errors only` toolbar, the table still flex-fills and scrolls internally, and there is no outer page scrollbar (per layout-templates §Wireframe — Full dashboard (Traces primary screen)).
- (layouts) The hero `<section>` accessible name stays exactly `"Telemetry traces chart"` and no service name enters the accessible tree after the change (per layout-templates §Wireframe — Full dashboard (Traces primary screen) Wireframe notes).
- (layouts) With `prefers-reduced-motion: reduce` set, a tier change still updates the dot hue and still emits the hue-shift observable (per layout-templates §Component — Halo State Pulse canvas, Reduced motion).
