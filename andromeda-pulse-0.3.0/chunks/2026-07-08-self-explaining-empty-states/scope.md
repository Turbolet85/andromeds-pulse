# Scope — Self-explaining empty states

**Marker:** `2026-07-08-self-explaining-empty-states`
**Version:** andromeda-pulse-0.3.0 · Epoch 3 — State honesty & legibility
**Capability:** P-071 · intent F11 (Mute empty states)
**Promoted:** 2026-07-08

## Working-route entry (verbatim)
> Self-explaining empty states — Metrics and Logs empty surfaces explain themselves with an actionable hint and design-system iconography (P-071 · intent F11)

No `PREREQ:` / `CARRY:` annotations on the taken-up entry — nothing to fold in.

## What it builds
When the Metrics or Logs dashboard route has **no data to show and is not loading**, render a
**self-explaining empty state**: an explanatory line + an **actionable hint** naming the concrete
next step (point an OTLP exporter at the receiver port), with **design-system iconography**
(Observatory custom glyph) — instead of a blank/silent panel.

Intent F11 verbatim EXPECT: empty states explain themselves — e.g.
*"No metrics received yet — point an OTLP metrics exporter at :4318/:4317"*.

The two named surfaces:
- **Metrics** (`MetricsRoute.tsx` → `MetricsChart`) — today renders the chart unconditionally; a
  fresh boot with only traces flowing shows an unexplained blank chart area.
- **Logs** (`LogsRoute.tsx` → `LogFilter` + `LogTable`) — today renders the filter + table; with
  zero log records the table body is empty with no explanation of why or what to do.

Likely shape (HOW is /andromeda-implement's to finalize): a small shared presentational
`EmptyState` component (message + hint + Observatory glyph, muted `font-body` per the frontend
rule "empty → font-body muted message") consumed by both routes behind a
`rows.length === 0 && !isLoading` render branch, so the two surfaces read as one system and a
future surface can reuse it.

## Boundaries / non-goals
- **Frontend-only (webview).** No backend/IPC/TauRPC/capability delta — the routes already fetch
  via `useMetrics` / `useLogs`; the empty state is a pure render branch over their existing
  return shape (`rows` + loading flag). No new `pulse://` topic, no `ready`/`health` field.
- **Four honest states** (frontend rule "handle loading / error / empty explicitly"; P4 user
  decision — "minimal honest error line"): loading (quiet — unchanged), populated (chart/table —
  unchanged), the NEW self-explaining empty (gated on `error === null`), and a distinct static error
  message (no exporter hint, no raw `AppError`) on a genuine query failure so it never masquerades as
  "no data." The empty state must NOT flash during the initial fetch (only after a settled zero-result
  load), mirroring the silent-background-refresh discipline.
- **Metrics + Logs are the F11 surfaces; Snapshots adopts the shared component too** (P4 user
  decision — "promote & adopt"): the reusable `EmptyState` is promoted from SnapshotsRoute's existing
  local copy and adopted by all three routes, de-duplicating and fixing Snapshots' latent
  tertiary→secondary contrast in passing. Traces' "No traces yet" is owned by P-081 (auto-refresh)
  and stays out of scope; the requirement's "(and other empty surfaces)" is satisfied by the reusable
  component, not by re-theming Traces.
- **Copy names the real receiver ports** — `:4318` (OTLP/HTTP) and `:4317` (OTLP/gRPC), the
  spec-fixed loopback receivers (arch §Occupied Resources). Metrics vs Logs get surface-appropriate
  wording ("metrics exporter" / "logs exporter").
- Not the compact widget (aggregate-glance; no per-route empty panels).

## Surfaces / contracts touched
- `pulse-app/ui/src/dashboard/routes/MetricsRoute.tsx` — empty-state render branch.
- `pulse-app/ui/src/dashboard/routes/LogsRoute.tsx` — empty-state render branch.
- (likely NEW) a shared `EmptyState` presentational component + its co-located test.
- (likely NEW/edit) an Observatory glyph under `src/components/icons/` (reuse existing —
  `telescope` / `aperture` / `constellation-grid` — before adding one).
- Co-located `*.test.tsx` for the routes + component; an a11y axe spec if a new persistent
  surface warrants it (p-series template).
- Design tokens: `--color-text-tertiary`/`-muted` for the message, `--font-body`, spacing scale,
  custom glyph at 24px hero — no new tokens.

## Acceptance (from verification-matrix P-071)
Empty Metrics/Logs surfaces render an explanatory message with an actionable hint (e.g. point an
OTLP exporter at the port). `method: webview`. Pure-visual/render (read-only, no user-operated
control) → affordance-EXEMPT; proven by vitest DOM-shape assertions (the message + hint text +
glyph present on the zero-data, not-loading branch; absent when populated; not flashed while
loading) + the a11y sweep stays green + the /implement P3 operator warm-boot visual verify (the
Metrics/Logs tabs show the self-explaining copy on a traces-only boot).

## Open questions — RESOLVED at P4
1. **Shared component** — RESOLVED: a single shared `components/EmptyState.tsx`, promoted from
   SnapshotsRoute's existing local copy and adopted by Metrics/Logs/Snapshots (user decision
   "promote & adopt"; de-dup + fixes Snapshots' latent tertiary contrast).
2. **Icon choice** — RESOLVED: reuse the existing Observatory `telescope` glyph ("looking for
   signal"), per design + layouts.
3. **a11y role** — RESOLVED: plain semantic text, NOT `role="status"`/`aria-live` (a11y extract —
   persistent route content, not a live update; also avoids a mistimed announce during initial fetch).
4. **Error-state handling** — RESOLVED (added at P4): a minimal honest error line (distinct static
   message, no exporter hint, no raw `AppError`), gating the empty state on `error === null`.
