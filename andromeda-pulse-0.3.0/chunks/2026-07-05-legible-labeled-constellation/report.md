# Report — 2026-07-05-legible-labeled-constellation

**Chunk:** Legible labeled constellation — per-dot service names + health/severity via design-system color + Halo (P-069)
**Date:** 2026-07-05
**Commits:** none yet (uncommitted; this wrap commits)

## Changes (structured — detectors read this)
- **Files:** (webview-only, all under `pulse-app/ui/`)
  - `src/widget/constellation-types.ts` — +2 pure helpers (`severityToken`, `dotLabelPosition`) + `DotLabelPosition` interface (additive; `visibleDots`/`constellationSummary` signatures unchanged → widget copy untouched)
  - `src/dashboard/routes/traces/ConstellationCanvas.tsx` — always-on DOM label overlay (name + non-color severity token) over the canvas; label chip anchored BELOW the dot (Fix #1)
  - `src/widget/constellation-types.test.ts` — +9 tests (`severityToken` it.each + `dotLabelPosition`)
  - `src/dashboard/routes/traces/ConstellationCanvas.test.tsx` — +1 test (per-dot name + token in DOM)
  - `tests-a11y/axe/p11-constellation-semantics.spec.ts` — +2 tests (dashboard per-dot axe + SC 1.4.1/4.1.2 assertion)
  - `tests-a11y/helpers/v02-fixtures.ts` — +`liveServiceListPayload` (recency-live dashboard fixture)
  - (ledger, not code) `andromeda-pulse-0.3.0/verification-matrix.json` — P-069 → `implemented`
- **Symbols / APIs:** 2 new exported TS fns (`severityToken(tier: PriorityTier | null): string`, `dotLabelPosition(dot: ConstellationDot): DotLabelPosition`) + `DotLabelPosition` interface. **No new TauRPC procedure · no IPC method · no endpoint · no port/socket · no env var · no Rust symbol.** Reuses the existing `services.list_with_states` → `ServiceListItem` surface.
- **Crates / modules:** none added / removed / changed (zero `.rs` delta; no workspace crate touched).
- **Dependencies:** none.
- **Schema / config:** none.
- **Coverage of new surfaces:**
  - `dashboard constellation per-dot label overlay (name + severity token)` → validation n/a (no input) · instrumentation n/a (canvas render loop unchanged → WebGPU frame budget preserved by construction; no new metric/log) · PII redacted✓ (service name renders as escaped React text — no `innerHTML`/`dangerouslySetInnerHTML`/style-interpolation; untrusted OTLP attr per security-plan) · tests unit✓ (vitest 693) + a11y✓ (p11 axe + per-dot) · a11y WCAG✓ (SC 1.4.1 not-color-alone via text token · SC 4.1.2 name DOM-determinable · SC 1.4.3 label 4.5:1 / SC 1.4.11 dot · reduced-motion preserved) · tokens design-token✓ (`--color-inset`/`-text-primary`/`-text-secondary`, `--font-code`/`-body`, `--spacing-*`, `--radius-sm`; no hardcoded hex)

## Deviations from intent
- **Label = bordered `--color-inset` chip, not plain text beside the dot** (the P4 mockup showed plain text). Justification: white text over a bright severity dot only reaches ~3.8:1; a solid inset chip guarantees SC 1.4.3 4.5:1 regardless of the dot hue behind it. On-brand (flat/matte, borders-only depth). p11 axe confirmed zero violations.
- **Fix #1 (operator live-verify catch):** the chip initially centered ON the dot (`translate(-50%,-50%)`, opaque) → hid its own dot. Re-anchored BELOW the dot (`top: calc(topPct% + 12%)`, clear of the ~0.14-norm glow). Justification: the dot (health hue) must remain visible ALONGSIDE the label. Confirmed fixed on a live boot (operator screenshot: dots visible, chips below). 693 vitest still green (position-agnostic assertions); p11 unaffected.
- **Live severity differentiation is INERT (all dots render "healthy") — UPSTREAM, not P-069.** Root cause: `incident_workspace_key` (`pulse-app/src/main.rs:686` = `data_dir.to_string_lossy()`) ≠ the incident producer's `digest.workspace` (the detected project root, `\\?\D:\dev\projects\andromeda-pulse` per the corpus). The per-service severity join (`services_router.rs:95` `incident_registry.list_active(&workspace_root)`) therefore finds zero active incidents → `priority_tier` stays null → every dot reads "healthy." Documented placeholder (main.rs:684-685 comment: "future chunks integrate workspace-detector for proper per-project keying"). P-069's render plumbing is correct + unit/p11 test-proven (tests use a populated `priority_tier`). Filed as a follow-up.

## Decisions & corrections
- **P4 dialogue (user-approved):** always-on labels (over hover-only / hybrid) + text severity token (over glyph / both).
- **Research-resolved (recorded, not asked):** DOM overlay over canvas-text (canvas text is SR-invisible + agent-untestable, a11y-plan §1 + tests §1); severity from `priority_tier` via the existing 4-tier `severityToHueFraction` model (no new TauRPC field).
- **Operator leave-running smoke-verify caught 3 things** the automated gates all passed: (a) labels hiding dots → Fix #1; (b) severity all-"healthy" → diagnosed to the upstream `incident_workspace_key` mismatch (main.rs:686); (c) traces "No traces yet" → `viz.query.traces` runs once at mount (row_count 0 before data), never re-polls.
- **Operator decision (option 1):** wrap P-069 now (Fix #1 + labels are done/correct); file the 2 upstream issues as follow-up route entries; widget per-dot labels = **CARRY** (leave aggregate-glance — quarter-screen surface, per layout-templates §Wireframe Compact widget).
- **Learnings surfaced:** (i) an all-green automated smoke (693 vitest + p11 axe + boot obs-log showing "webview rendering") CANNOT catch visual occlusion (label-over-dot) — the leave-running operator verify did; (ii) the a11y Playwright webServer serves an un-rebuilt `dist` (needs `npm run build` first, else the spec tests the stale bundle); (iii) `ServiceListItem["priority_tier"]` is `PriorityTier | null | undefined` (optional serde field) — type test annotations as `PriorityTier | null`; (iv) the concrete upstream root cause: `incident_workspace_key`=data_dir vs producer's `digest.workspace`=project-root → constellation severity + incidents panel both inert.

## Outcome
- **Acceptance criteria met** (P-069 render plumbing): each rendered dashboard dot exposes its service NAME (always-on DOM label) + a health/severity color + a non-color text token — asserted in the DOM (vitest) + p11 a11y (dashboard axe zero-violations + per-dot SC 1.4.1/4.1.2). Fix #1 confirmed on a live boot. Live severity DIFFERENTIATION is inert upstream (documented above; a follow-up) — the encoding itself is correct.
- **Gates green:** `npm run typecheck` · `npm run test` (vitest **693/693**) · `npm run lint` · `verify:contrast` (12/12) · p11 a11y Playwright (**4/4**). Rust workspace gates (`clippy --workspace` / `nextest --workspace` / `capability-drift`) DEFERRED — zero `.rs` delta (re-run at the next source-touching chunk); the pulse-app binary WAS rebuilt (`cargo build -p pulse-app`, green) for the boot re-embed.
- **Smoke:** warm re-embed boot (real WebView2, 54,538 frame metrics, 0 panics, all ticks, clean zero-orphan shutdown) + operator leave-running verify that caught & resolved Fix #1.
