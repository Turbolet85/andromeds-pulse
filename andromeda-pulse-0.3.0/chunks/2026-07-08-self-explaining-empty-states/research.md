# Codebase Research — 2026-07-08-self-explaining-empty-states

## Scope
- **Depth:** moderate · **Reads:** 11 · **Globs/Greps:** 4
- **Graph query:** N/A — the code-graph `tree.db` is a **Rust-SCIP** index (`rust-analyzer cargo <crate>` symbols per `code-graph-cookbook.md`); this chunk is 100% webview TypeScript (`pulse-app/ui/**`), which the indexer does not cover. Cold-start for this surface; grounded by targeted TS reads instead (the correct tool here).

## Files inspected
- `pulse-app/ui/src/dashboard/routes/MetricsRoute.tsx` (full) — renders `<MetricsChart rows={rows} …/>` unconditionally; destructures only `{ rows }` from `useMetrics` (ignores `isLoading`/`error`). **The F11 gap.**
- `pulse-app/ui/src/dashboard/routes/metrics/MetricsChart.tsx` (full) — on empty `rows`, `aggregateIntoBuckets` returns `[]` and `renderChart` does `clearRect` then `if (list.length === 0) return;` → **draws a blank canvas, no text**. Confirms "blank with no explanation."
- `pulse-app/ui/src/dashboard/routes/LogsRoute.tsx` (full) — renders `<LogFilter/>` + `<LogTable rows={filter.filteredRows} isLoading/>`; destructures `{ rows, isLoading }` (ignores `error`).
- `pulse-app/ui/src/dashboard/routes/logs/LogTable.tsx` (full) — **already has** a loading branch (`isLoading && rows.length===0` → "Loading…") and an empty branch (`sorted.length===0` → **"No logs match filter"**). Uses `--color-text-secondary` with the comment `// text-secondary, not tertiary: small body text needs 4.5:1 (chunk #99 pa11y finding 3.92:1)` — the codebase has already internalized the a11y+design contrast finding. **Caveat:** its empty message is FILTER-empty; it also (mis)fires when there's simply no data at all — so the no-data case must be handled at the ROUTE level (it has the pre-filter `rows`).
- `pulse-app/ui/src/dashboard/routes/metrics/use-metrics.ts` + `logs/use-logs.ts` (full) — both return `{ rows, total, isLoading, error }`; on rejection they set `rows:[]` + `error:<AppError>`. So routes have all four states available; `error` is set on failure (must be excluded from the empty branch).
- `pulse-app/ui/src/canvas/Fallback.tsx` (full) — the muted-message precedent: centered `--font-body` 14px message; uses `--color-text-tertiary` + `role="alert"`/`aria-live="assertive"` (that alert role is specific to the WebGPU-failure surface — NOT wanted for a persistent empty panel).
- `pulse-app/ui/src/dashboard/routes/SnapshotsRoute.tsx` (full) — **already contains a local `EmptyState` component** (non-exported): a centered flex `<div data-testid="route-empty-state">` with `<Icon glyph="telescope" size={24} />` (decorative) + a `--font-body` 14px `<p>{message}</p>`. **Uses `--color-text-tertiary`** (the latent contrast issue the extracts flag; not yet caught because no axe spec audits this route).
- `pulse-app/ui/src/components/icons/BaseIcon.tsx` (full) — `<Icon glyph size/>` → `<svg stroke="currentColor" aria-hidden={true when unlabeled} role={"img" only when aria-label present}>`. Decorative-by-default + inherits text color via `currentColor`. Confirms the a11y decorative-icon contract holds by construction.
- `pulse-app/ui/src/dashboard/routes/SnapshotsRoute.test.tsx` (full) — asserts `getByTestId("route-empty-state")` + the message text. Promoting the local `EmptyState` to shared is safe iff the shared component keeps that testid + renders the message.
- `pulse-app/ui/src/dashboard/routes/MetricsRoute.test.tsx` (full) — mocks `MetricsChart` as a stub; existing tests assert section/heading (survive) or use non-empty `sampleRows` (chart path survives). No existing test breaks; empty-branch tests are added.
- `pulse-app/ui/tests-a11y/axe/p10-diagnostics-view.spec.ts` (full) — the axe-spec model: `installTauriIpcMock(page, v02WidgetOverrides, "main")` → `runAxeSweep(page, {surface, url, setup})` navigating a real route path; a 2nd test checks headings + testids. **No p-spec covers /metrics or /logs today** (p1–p12 = traces/investigation/settings/live-trace/widget/snapshot-completion/settings-form/findings/diagnostic-report/diagnostics-view/constellation/export-preview) → Metrics/Logs are un-audited surfaces.

## Graph impact
- **Cold-start for this surface** — Rust-SCIP DB does not index `pulse-app/ui/**`. Blast radius established by TS reads: the shared `EmptyState` will have 3 consumers (MetricsRoute, LogsRoute, SnapshotsRoute); no cross-crate/Rust impact; zero backend/IPC/capability delta.

## Patterns detected
- **Existing local `EmptyState`** (`SnapshotsRoute.tsx:41-70`): centered flex, decorative `<Icon glyph="telescope" size={24}/>`, `--font-body` message, `data-testid="route-empty-state"`. This IS the shape to promote to a shared reusable component (resolves scope OQ1).
- **Contrast discipline already applied** (`LogTable.tsx:127,143`): `--color-text-secondary` (6.8:1) NOT tertiary for body-size empty/loading text, with an explicit `chunk #99 pa11y 3.92:1` comment — the shared component must adopt secondary (SnapshotsRoute's local copy at tertiary is the latent bug this promotion fixes).
- **Decorative icon** (`BaseIcon.tsx:11,23-24`): omit `aria-label` → `aria-hidden="true"` automatically; meaning lives in text (SC 1.1.1/1.4.1 by construction).
- **Route-level axe audit** (`p10-diagnostics-view.spec.ts`): install IPC mock → visit route URL → `runAxeSweep` + heading/testid checks; regression baseline 0 tuples.

## Conventions to follow
- **Reuse-first, one copy** (`.claude/rules/frontend.md` 2026-07-02 two-copies entry): promote the single `EmptyState` rather than leave two copies; enumerate all consumers (Metrics/Logs/Snapshots) in one pass.
- **Three-state honesty** (`.claude/rules/frontend.md` "handle loading / error / empty explicitly"): empty branch gated on `!isLoading && error===null && rows.length===0` so it neither flashes during load nor masquerades over a query error.
- **Tokens only** (`design-tokens.md`): `--color-text-secondary` message/hint, `--font-body`, `--font-code` for inline port literals, 4px spacing scale, `<Icon>` 24px — no hardcoded hex/px.
- **Port literals are arch-locked** (`architecture.md §Occupied Resources`): `:4318` (OTLP/HTTP) + `:4317` (OTLP/gRPC) — copy names them exactly.
- **New surface ⇒ new axe spec** (`a11y.md` §Harness): add an axe spec + zero-data fixture; `npx playwright test --list` stays healthy; 0 critical/serious, 0 new regression tuple.

## New files to create
- `pulse-app/ui/src/components/EmptyState.tsx` — shared presentational empty-state (`message`, optional `hint?: ReactNode`, optional `glyph`, optional `testId`); `--color-text-secondary`, decorative `<Icon>`, plain semantic text (no `role="status"`).
- `pulse-app/ui/src/components/EmptyState.test.tsx` — DOM-shape: message + hint (incl. `:4318`/`:4317`) + decorative glyph node; contrast token; no live-region/interactive element.
- `pulse-app/ui/tests-a11y/axe/p13-empty-states.spec.ts` — axe audit of /metrics + /logs in the zero-data empty state (0 critical/serious; regression 0 new).

## Files to modify
- `pulse-app/ui/src/dashboard/routes/MetricsRoute.tsx` — pull `{ rows, isLoading, error }`; branch `!isLoading && !error && rows.length===0` → `<EmptyState message="No metrics received yet" hint=…:4318/:4317… testId="metrics-empty-state"/>`, else `<MetricsChart/>`.
- `pulse-app/ui/src/dashboard/routes/LogsRoute.tsx` — pull `error`; branch pre-filter `!isLoading && !error && rows.length===0` → `<EmptyState … testId="logs-empty-state"/>`, else `<LogFilter/>`+`<LogTable/>` (LogTable keeps its own "No logs match filter" for the filter-empty case).
- `pulse-app/ui/src/dashboard/routes/MetricsRoute.test.tsx` + `LogsRoute.test.tsx` — add empty-branch + not-flashed-while-loading + populated-hides-empty cases.
- `pulse-app/ui/src/dashboard/routes/SnapshotsRoute.tsx` (+ `.test.tsx` stays green) — **[pending P4 decision]** import the shared `EmptyState`, delete the local copy (de-dupe + fixes its latent tertiary contrast).
- `pulse-app/ui/tests-a11y/helpers/v02-fixtures.ts` — **[if needed]** ensure the axe mock resolves `metrics.query`/`logs.query` empty so the empty state renders.

## Open questions
- **OQ-A (Snapshots adoption):** promote-and-adopt the shared `EmptyState` in SnapshotsRoute (DRY, fixes its latent tertiary→secondary contrast; touches a 3rd route + its test) vs create a new shared component and leave SnapshotsRoute's local copy (narrower diff, keeps two copies + the latent bug). → **AskUserQuestion at P4** (recommend promote-and-adopt).
- **OQ-B (error state):** default = gate the empty branch on `error===null` and leave the error case as current behavior (Metrics blank chart / Logs "No logs match filter"), noting error-surface polish as a follow-up — F11 is the no-data case, not query failure. (In-plan decision; not a user question unless the reviewer wants error UI folded in.)
