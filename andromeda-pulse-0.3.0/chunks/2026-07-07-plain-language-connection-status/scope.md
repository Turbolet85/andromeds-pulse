# Scope — Plain-language connection status (P-070)

**Marker:** `2026-07-07-plain-language-connection-status`
**Version:** andromeda-pulse-0.3.0 · Epoch 3 — State honesty & legibility
**Capability:** P-070 (intent F10) → `andromeda-pulse-0.3.0/verification-matrix.json#P-070`

**Working-route source (verbatim):**
> Plain-language connection status — a human-readable services-connected, spans-per-second, and
> buffer-state line using design-system typography (P-070 · intent F10) · CARRY: make the
> ConnectionDot recency honest — its "Listening · last span · just now" tooltip (chunk #89) can read
> "just now" on zero telemetry; deferred from P-067 (RESEARCH-CORRECTS-INTENT — that phrasing was the
> intent's F7 evidence but belongs to the connection-status surface, not the constellation, which
> P-067 fixed).

---

## Intent (what & why)

**Observed gap (P-070):** Nowhere does the UI state, *in words*, how many sources are connected, whether
telemetry is arriving, or the buffer state. The app has a minimal `ConnectionDot` (chunk #89) — a colored
dot + tooltip — and a live constellation, but no plain-language sentence a developer can read at a glance to
answer "is my telemetry actually flowing, and from how many services?"

**Requirement (P-070):** A human-readable status line states sources-connected / telemetry-arriving /
buffer-state — e.g. *"Receiving from 5 services · ~540 spans/s · buffer 2 min / 10 min"* — rendered with
design-system typography.

**Why now:** Epoch 3 is "State honesty & legibility." P-067 made the constellation live-only; P-068/P-069
made Traces + dots legible; P-081 made the table live. The one remaining honesty gap is the *worded* status —
the plain-language complement to the visual dot/constellation, so a user never has to infer connection health
from color alone.

## What this chunk builds

1. **A plain-language connection-status line** on the dashboard (design-system typography), reporting in words:
   - **connected-source count** — how many services are *currently live* (recency-gated, consistent with the
     P-067 live-only truth model — never counting stale/persisted registry entries as connected);
   - **ingest rate** — spans-per-second (a human-rounded rate, e.g. "~540 spans/s");
   - **buffer state** — fill vs. the configured retention window (e.g. "buffer 2 min / 10 min").
   - Honest zero/empty phrasing: with zero live telemetry the line says so plainly (e.g. "No telemetry yet —
     listening on :4317/:4318") rather than implying flow that is not there.

2. **Folded CARRY — ConnectionDot recency honesty.** Fix the chunk-#89 `ConnectionDot` tooltip so its recency
   phrasing (currently "Listening · last span · just now") cannot read "just now" when there has been **zero**
   telemetry / no span has ever arrived. On a fresh boot with no spans, the tooltip must state an honest
   "listening, no spans yet" rather than a recency ("just now") that implies a span just landed. This is the
   P-067 dual for the connection surface (P-067 fixed the constellation; the intent's F7 "just now" evidence
   actually belonged here, per the RESEARCH-CORRECTS-INTENT note carried from P-067).

## Boundaries

- **Primarily webview** (matrix `method: webview`). The status line is a rendered UI surface consuming existing
  telemetry/state contracts. A backend delta is IN scope **only if** research (P3) shows the three data points
  (live-source count · spans/s · buffer fill) are not already retrievable from an existing TauRPC/stream — in
  which case the minimal-cost path (extend an existing payload / reuse `services.list_with_states` +
  `connection.current_state` + a buffer/rate source) is preferred over a new TauRPC namespace, per the
  Settings-extension / reuse-first discipline in the session-learnings.
- **Placement:** the dashboard is the primary home for the worded line (the compact widget stays aggregate-glance
  per the P-069 layout boundary); whether a condensed form also belongs in the widget is a P4 plan question, not
  assumed here.
- **Consistency with live-only truth (P-067):** the "connected sources" count MUST use the same recency gate as
  the constellation's live-only visibility — the worded count and the dot count must not disagree.

## Surfaces / contracts likely touched (to be grounded in P3 research)

- Connection state: `connection.current_state` (`ConnectionApiImpl` → `ConnectionStatePayload`, chunk #59) +
  the `pulse://stream/connection-state` topic; the `ConnectionDot` component + its recency/tooltip hook
  (chunk #89) for the CARRY.
- Live services: `services.list_with_states` (`ServicesApiImpl` → `ServiceListPayload`, chunk #67) + the
  frontend recency gate added at P-067 (`constellation-types` visibleDots/recency).
- Ingest rate + buffer fill: source TBD in research — candidates are an existing metrics/health/ready surface
  (`ready`'s `ingest_mpsc_capacity_pct`, buffer retention config) or a viz/metrics query; identify the
  lowest-cost existing producer before proposing any new field.
- Design system: typography tokens for the status line; a11y — the line is visible text (screen-reader legible
  by construction); confirm it does not regress the Traces/constellation a11y surfaces.

## Open questions for research (P2 distill / P3 codebase)

1. Where does **spans-per-second** already exist (or the nearest counter to derive it from) without a new
   backend contract? Is there a rolling rate anywhere, or only raw counts?
2. Where does **buffer fill vs. retention** surface today (config `retention_seconds` + a "current span/oldest
   ts" readout)? Is `ready`/`health` enough, or is a viz query needed?
3. Exact **recency-gate reuse** — can the worded live-source count reuse the P-067 frontend recency helper
   directly, or is a shared selector needed so the dot and the line never disagree?
4. **ConnectionDot** — where is the "just now" phrasing produced (component vs. a formatting hook), and what is
   the honest zero-span branch?
5. **Placement** — dashboard-only, or dashboard + a condensed widget form?

## Out of scope

- Any new persistent setting or corpus schema; any new OTLP surface.
- Reworking the constellation or Traces table beyond consuming shared state.
- Historical/aggregated rate charts (this is a *current* status line, not a metrics view).
- The Metrics/Logs self-explaining empty states (P-071 — the next chunk).

## Acceptance (from verification-matrix.json#P-070)

A visible status line reports connected-source count, ingest rate, and buffer fill in plain language.

**P5 amendment (intent-incomplete, 2026-07-07):** the P1 note tentatively guessed "a11y/contrast-exempt" — the
a11y extract corrects this. The line is read-only with NO user-operated control → it IS **affordance-EXEMPT**
(no headful click needed; a DOM-shape render assertion + the /implement P3 operator visual verify suffice, per
verification-matrix-contract §Affordance honesty pure-visual carve-out). But it is NOT **contrast-exempt** — the
SC 1.4.3 text-contrast sweep on the full-dashboard axe pass applies to the rendered text (a11y-plan §6). The two
"exempt"s are distinct; the P1 note conflated them.
