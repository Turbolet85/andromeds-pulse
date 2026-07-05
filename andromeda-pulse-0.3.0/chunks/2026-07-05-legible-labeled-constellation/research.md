# Codebase Research — 2026-07-05-legible-labeled-constellation

## Scope
- **Depth:** deep (mature webview surface) · **Reads:** 8 · **Globs/Greps:** 2 glob + 1 grep · **Graph queries:** 1

## Files inspected
- `pulse-app/ui/src/widget/constellation-types.ts` (full) — the SHARED pure-function dot model. `ConstellationDot` **already carries** `service` (name), `hueFraction` (severity color), `state`, `priorityTier`, `brightness`, `x/y`. `visibleDots()` builds dots (drop archived + non-live, sort by name, cap 20, `severityToHueFraction(tier)` hue). `constellationSummary()` builds the aggregate off-canvas accessible name (count + per-state + "N with active findings"). `lifecycleLabel(state)` → word. **GAP: no per-dot label rendering and no label-position/severity-token helper — identity exists in the model but surfaces only in the aggregate aria-label.**
- `pulse-app/ui/src/dashboard/routes/traces/ConstellationCanvas.tsx` (full) — the DASHBOARD renderer. Pure WebGPU: each dot → a soft Halo blob (`lchInterpolate(dot.hueFraction, --color-primary, --color-accent)`, position, brightness). Wrapper `<section aria-label={summary} data-testid="constellation-canvas" data-service-count={dots.length}>` with `position: relative`, `height: 240px`. **No per-dot DOM labels.** Reduced-motion already degrades to static glow (hue still encodes).
- `pulse-app/ui/src/widget/ConstellationCanvas.tsx` (full) — NEAR-IDENTICAL sibling; `data-testid="service-constellation"`, `height: 100%`. Consumes the same `visibleDots`/`constellationSummary`. **Must stay aggregate-glance — no per-dot labels on the quarter-screen widget (layout boundary).**
- `pulse-app/ui/src/halo/severity-to-halo.ts` (full) — `severityToHueFraction(tier)`: **null/None=0 (calm Earth Blue, healthy), curious=0.33, suggested=0.67, autonomous=1.0 (Alert Burgundy)** — the 4 severity levels the non-color cue must mirror. `connectionStateToDim` = orthogonal grayout (P-004), independent of severity.
- `pulse-app/ui/src/widget/constellation-pipeline.ts` (full) — WebGPU pipeline factory (Vite `?raw` WGSL + sanitized tagged-union failure). **Not changed** — the label overlay is DOM, not canvas.
- `pulse-app/ui/tests-a11y/axe/p11-constellation-semantics.spec.ts` (full) — currently mounts the COMPACT WIDGET (`installTauriIpcMock(page, v02WidgetOverrides, "compact-widget")`) + asserts every `<canvas>` lives in a labelled `<section>`. **Extension target: per-dot name/severity semantics on the dashboard.**
- `pulse-app/ui/src/dashboard/routes/traces/ConstellationCanvas.test.tsx` (full) — `item(service, state, priorityTier=null, lastSeen)` factory + `ITEMS=[item("svc-a","active","autonomous"), item("svc-b","quiet")]`; the `conveys per-state counts + active findings ... (not color-alone)` aria-label test is the SC 1.4.1 template to extend per-dot. `data-testid="constellation-canvas"`.
- `pulse-app/ui/src/bindings/index.ts` (355–395) — `ServiceListItem = { service; state: ServiceLifecycleState; last_seen_unix_nano; manual_override; priority_tier?: PriorityTier | null }`; `PriorityTier = "autonomous" | "suggested" | "curious"`; the doc-comment confirms `priority_tier` "Drives the constellation dot hue" (resolver joins the incident registry on `scope_id`).

## Graph impact (from the code-graph query)
- **`ServiceListItem`** — Rust producer at `crates/triage/src/lifecycle/registry.rs:53` (fields `service`/`state`/`last_seen_unix_nano`/`manual_override` + `priority_tier` @ def_line 64); consumed by the `services.list_with_states` resolver + the `pulse-app` observability allowlist test. The TS constellation surface this chunk modifies is **NOT in tree.db** (rust-analyzer SCIP indexes Rust only) — expected for a webview-only chunk; the Rust data source is confirmed present and **untouched by scope** (no new field needed — `service` + `priority_tier` already flow). Query trace: `.andromeda/runs/2026-07-05T17-59-57Z-phase/tree-query-2026-07-05-legible-labeled-constellation.json`.

## Patterns detected
- **Shared-model / two-copies fan-out** (`constellation-types.ts` → `widget/ConstellationCanvas.tsx` + `dashboard/.../ConstellationCanvas.tsx`): additive pure helpers do NOT force both edits; only a shared-SIGNATURE change does (frontend.md 2026-07-02). Adding labels to the dashboard only ⇒ widget copy untouched.
- **WebGPU-canvas + off-canvas `<section aria-label>`** is the established SR pattern (a11y-plan §1); per-dot identity MUST be DOM, not the canvas bitmap (canvas text is SR-invisible + agent-untestable).
- **Design tokens read from CSSOM at init** (`readDesignToken('--color-primary', …)`); labels must use tokens (`--color-text-primary` on `--color-inset`), never hardcoded hex (design-tokens.md; §Self-Validation Token Test).
- **4-tier severity model** (`severity-to-halo.ts`) already drives hue — reuse it for the non-color cue rather than inventing a parallel scale.
- **`<section aria-label>` takes the implicit `region` role** — never add explicit `role="region"` (a11y.md 2026-05-09 `no-redundant-roles`).

## Conventions to follow
- **No new TauRPC procedure / capability** — reuse `services.list_with_states` → `ServiceListItem` (arch extract; `service` + `priority_tier` already present). Keeps the security↔tests↔arch capability-drift triple binding untouched.
- **Untrusted service-name string** renders as escaped React text (never `innerHTML`/`dangerouslySetInnerHTML`/style-interpolation); React default escaping is the XSS control (security extract; `crates/.../` untouched).
- **Design tokens only** — IBM Plex Sans Label 12px OR JetBrains Mono Data 12px; `--color-text-primary`; no banned fonts, no magic hex (design-tokens.md).
- **DOM/accessible-name test assertions, never canvas pixels**; fixtures via the existing `item()` factory; `?? null`-normalize `priority_tier` (tests extract; frontend.md 2026-05-30).
- **No new per-frame/service metric**; the DOM label overlay is a canvas SIBLING (the render loop is unchanged) → WebGPU frame budget (`metric.webgpu.frame_duration_ms` p99 ≤33ms) preserved by construction (obs extract).
- **Reduced-motion**: labels are static text (no motion); the canvas already degrades — nothing new to gate (design/a11y extracts).

## New files to create
- (likely none) — per-dot labels can be a DOM overlay rendered inline in the dashboard `ConstellationCanvas.tsx` + pure helpers added to `constellation-types.ts`. A small `ConstellationLabels.tsx` sub-component is OPTIONAL for cleanliness (decide at /implement). If the p11 dashboard audit needs a fixture, extend `tests-a11y/helpers/v02-fixtures.ts` (no new file).

## Files to modify
- `pulse-app/ui/src/widget/constellation-types.ts` — ADD pure helpers (additive; no existing-signature change): a normalized-position→CSS-% converter for label placement (e.g. `dotLabelPosition(dot)`, with the y-flip for CSS top-down) + a non-color severity-token/label mapper over the 4-tier `PriorityTier | null` model.
- `pulse-app/ui/src/dashboard/routes/traces/ConstellationCanvas.tsx` — render a DOM label overlay (absolutely-positioned per-dot name + non-color severity token) inside the existing `position: relative` `<section>`, layered above the `<canvas>`.
- `pulse-app/ui/src/widget/constellation-types.test.ts` — add `it.each` tests for the new pure helpers (position mapping + severity-token table).
- `pulse-app/ui/src/dashboard/routes/traces/ConstellationCanvas.test.tsx` — extend to assert each dot's service NAME + its non-color severity token in the DOM (extending the "not color-alone" template).
- `pulse-app/ui/tests-a11y/axe/p11-constellation-semantics.spec.ts` — extend with dashboard per-dot name/severity semantics (may need a dashboard route mount + a service fixture in `helpers/v02-fixtures.ts`).

## Open questions (resolved at P4)
- **Label density / visibility** (Q1) — always-on labels for all ≤20 dots, vs hover/focus-only, vs hybrid. Real UX + crowding tradeoff on the 240px hero → AskUserQuestion at P4.
- **Non-color severity cue form** (Q4) — a text severity token, an icon/glyph, or both, paired with the name → AskUserQuestion at P4.
- **RESOLVED by research (recorded, not asked):** canvas-text-vs-DOM-overlay (Q2) → **DOM overlay** (canvas text fails SC 4.1.2 + is agent-untestable; a11y+tests+layout converge). Severity source (Q3) → **`priority_tier` via the existing 4-tier `severityToHueFraction` model** (design 2026-05-29 amendment + the shipped code). No new TauRPC field.
