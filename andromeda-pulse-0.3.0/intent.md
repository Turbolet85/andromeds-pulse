# v0.3.0 Intent — andromeda-pulse

**Purpose & how to use this file.** This is the human-authored INTENT for version 0.3.0 — the single source
of truth that `/andromeda-route --version 0.3.0` derives `vision.md`, `requirements.md`, and
`working-route.md` from. It is deliberately detailed and unambiguous: every finding states what we OBSERVED
and what we EXPECT, so the route/phase pipeline produces precise requirements + chunks with no guesswork.
**Working pattern: we author the intent → Andromeda derives everything else from it.**

_Authored 2026-06-28 from a live dogfood of the running v0.2.0 build._

---

## 1. Context

Pulse reached **v0.2.0 feature-complete**: 60 capabilities (P-001..P-060), route 100/100, all gates green
(nextest 1676, webview 640, a11y 0, coverage 83%). The 7 design docs are v3-normalized; the project is
migrated to the v3 Andromeda pipeline (this `andromeda-pulse-0.3.0/` version folder).

**A live dogfood of the running app on 2026-06-28 revealed a hard truth:** the product is **green on paper
but illegible and partly non-functional in practice.** The backend data spine works; the user-facing product
— the window, the legibility, the AI-debug actions — does not let a user understand or act on anything.
v0.3.0 closes that gap.

## 2. The core problem (one sentence)

> Pulse is feature-complete, but **you cannot understand its state or use its core AI-debug value from the
> running UI** — and the test suite (component-level) never caught it.

## 3. What WORKS today — preserve, build on (do NOT rebuild)

The deterministic **data spine is solid**; v0.3.0 is fix / legibility / wiring on top of it, not a rewrite:

- OTLP ingest (`:4317` gRPC / `:4318` HTTP) → mpsc → DuckDB ring buffer → trace/metric/log query.
- The **Traces table populates correctly** with real services + latencies under live telemetry.
- The **snapshot / curation path works** (markdown + json, token-budgeted — 451 tokens from real data).
- The deterministic detection layers fire (hard-signal floor; retry-storm detection).

## 4. Findings — OBSERVED vs EXPECT

Each finding is a concrete observation from the live dogfood plus the expected v0.3.0 behavior. These are the
raw material for `requirements.md`.

### Theme 1 — Window & shell hygiene
- **F1 · Geometry/position.** OBSERVED: opens tiny, pinned to the top-left corner, cannot be moved (dragging
  the titlebar does nothing). EXPECT: opens at a sane default size + position (centered or remembered); the
  custom frameless titlebar is a drag region so the window is movable.
- **F2 · Resize.** OBSERVED: free-resizes to any dimensions — no min-size, no aspect lock → looks absurd when
  stretched. EXPECT: enforce a min-size + a sensible aspect-ratio constraint (it is a glance widget meant to
  hold fixed-ish proportions).
- **F3 · Close behavior.** OBSERVED: the expanded dashboard cannot be closed — the X and taskbar-close do not
  quit; the process keeps running (only Tray→Quit or a kill stops it). EXPECT: predictable, signposted close —
  window-close either quits or minimizes-to-tray with a CLEAR indication it is still running; the dashboard is
  closable; the tray is the single honest "still running" surface.
- **F4 · Browser context menu leaks.** OBSERVED: right-click anywhere shows the default WebView2 menu (Back /
  Refresh / Save as / Print / Inspect) — zero app meaning. EXPECT: suppress the browser context menu app-wide
  (or replace with an app menu); no Inspect/Print/Save-as in a production build.
- **F5 · Canvas image-save leaks.** OBSERVED: clicking the halo/canvas triggers the browser's save-image
  interaction. EXPECT: the canvas is not treated as a saveable/draggable browser image.
- **F6 · No widget→dashboard navigation.** OBSERVED: the full dashboard (Traces/Metrics/Logs/Snapshots/
  Settings) is reachable ONLY via the tray ("Open …"); the small glance widget has no affordance to open it.
  EXPECT: an explicit in-app way (button/click) to expand the widget into the dashboard.

### Theme 2 — State honesty & legibility
- **F7 · Phantom services (the UI lies) — HIGH PAIN.** OBSERVED: on a FRESH start with zero live telemetry
  (tray: "Ingest: 0 sp/s"; Traces: "No traces yet"), the constellation STILL renders ~6–7 service dots + a
  hover tooltip "Listening · last span · just now". This is almost certainly the **persisted `service_registry`
  (corpus SQLite) shown as if live** — the tool contradicts itself and shows services that are not currently
  present. For an observability tool this is the cardinal sin: you cannot trust what you see. EXPECT: show only
  CURRENTLY-live services; persisted/stale services are hidden or clearly marked historical/inactive; recency
  labels reflect reality and never imply liveness that is not there.
- **F8 · Anomalies buried — HIGH PAIN.** OBSERVED: with live telemetry, the Traces table sorts by latency
  ascending → the single erroring/anomalous service (`payment-service`, 2500 ms, 100% errors) sits at the very
  BOTTOM, off-screen; the visible top is all-healthy services with Error 0. For an incident/observability tool
  this is backwards. EXPECT: errors/anomalies surface to the TOP (and/or are visually flagged + filterable) —
  the user sees "what is wrong" first, not last.
- **F9 · Illegible constellation.** OBSERVED: the dots are fuzzy unlabeled blue blobs — no names, no glanceable
  health. EXPECT: each dot is identifiable (name on/near it or on hover) and its health/severity is clearly
  encoded; the user can tell at a glance what they are looking at.
- **F10 · No plain-language status.** OBSERVED: nowhere does the UI state, in words, how many sources are
  connected / whether telemetry is arriving / the buffer state. EXPECT: a clear human-readable status (e.g.
  "Receiving from 5 services · ~540 spans/s · buffer 2 min / 10 min").
- **F11 · Mute empty states.** OBSERVED: the Metrics and Logs tabs are blank with no explanation (the demo
  injector sent only traces, but the UI does not say so). EXPECT: empty states explain themselves ("No metrics
  received yet — point an OTLP metrics exporter at :4318/:4317").

### Theme 3 — AI-debug climax (the core value — currently dead)
- **F12 · Investigate actions dead — KEYSTONE-GATED.** OBSERVED: the Investigate modal opens and the snapshot
  generates correctly (451 tokens from real data — this path works), but the 4 action buttons (Diagnose latency
  outlier / Find error correlation / Trace failed request / Summarize service health) do NOTHING on click — no
  result, no feedback — even with live data. This is the product's CORE value and it is non-functional. EXPECT:
  each action runs a real LLM/MCP-backed analysis with visible progress + a result; failures surface, never
  silent.
- **F13 · No incident from a real storm — KEYSTONE.** OBSERVED: a real retry-storm (`payment-service`, 426
  identical-fingerprint exceptions over ~80 s) produced NO incident — the digest→L4→incident chain never
  completes because the L4/LLM step is degraded/fragile; the "incident → red dot → Investigate" climax never
  fires. EXPECT:
  - **(a) a deterministic, env-gated L4 mode** (canned `L4Output`, no GPU/3B — reuse the existing
    `StubInferenceRunner` pattern, exposed to the live app by env/flag) so the incident path completes
    reproducibly for demos / tests / external verification;
  - **(b) Tier1 incident-path reliability under load** — the current `TIER1_QUEUE_CAP=3` + ~4 s L4 latency
    drops ~75% of storm digests; coalesce repeated identical hard-signals into one digest + an elastic queue
    so a sustained storm yields ONE reliable incident.

### Theme 4 — External verification (Conductor)
- **F14 · Pulse delegated its proof to Conductor, but it is blocked.** Pulse's own spec delegates 4
  timing-bound capabilities to Conductor: **P-025** halo hue ≤2 s · **P-027** constellation discovery ≤5 s ·
  **P-037** report render ≤2 s · **P-045** counter refresh ≤1 s — plus the end-to-end incident
  content-fidelity. These cannot be verified while incidents are non-deterministic (LLM-gated). EXPECT: with the
  deterministic L4 mode (F13a) in place, Conductor can drive telemetry → a known incident → MCP read-back and
  finally PROVE the e2e + these 4 caps. This is the Pulse↔Conductor mutual-bootstrap unlock.

### Theme 5 — Test gap & housekeeping
- **F15 · Integration-test blind spot.** OBSERVED: 1676 nextest + 640 webview + a11y all green, yet the
  integrated live UX is broken/illegible — the suite tests components in isolation, never the assembled
  product. EXPECT: an integration/e2e UX test that exercises the REAL product path (launch → telemetry → Traces
  render → storm → incident → Investigate) so this class of breakage is caught going forward.
- **Housekeeping:** formalize the demo telemetry injector (`crates/ingest/examples/inject_demo.rs`, currently
  untracked) as a proper, supported dev/test tool — it proved its worth in this dogfood. Retire the markdown
  `.andromeda/context/api-surface.md` once the code-graph `tree.db` is built.

## 5. Scope, priorities, non-goals

- **Keystones (highest value):** F13 deterministic-L4 mode + Tier1 reliability — they make the AI-debug REAL
  and externally verifiable, and they unblock F12 (Investigate) and F14 (Conductor). Sequence these early.
- **Quick wins:** F1–F6 (window/chrome) — mostly standard Tauri/webview config; high visible payoff per effort.
- **Highest user-pain:** F7 (phantom services / the UI lies) + F8 (anomalies buried) — they destroy trust;
  rank them high.
- **Out of scope / deferred:** the 3B model's *judgment quality* (it dismisses real storms) is a separate,
  deeper problem — the deterministic-L4 mode sidesteps it for verification; real-model quality is NOT a v0.3.0
  goal. MCP `tools/call` `CallToolResult` compliance is deferred (Conductor already adapted to the raw shape;
  "fixing" it would re-break that adapter).

## 6. Definition of done (what "v0.3.0 works" means)

Launch Pulse → it opens as a proper, movable, closable window with no browser-chrome leaking. With zero
telemetry it honestly says "nothing connected" (no phantom services). Point telemetry at it → live services
appear, **labeled and legible**, with **errors surfaced first**, and a plain-language status. Drive a storm →
a **deterministic incident fires** (red dot + Findings) and the **Investigate actions produce real results**.
**Conductor proves** the e2e + the 4 delegated timing caps against the deterministic mode. An integration UX
test guards all of the above.

## 7. What we expect Andromeda to derive from this intent

- `vision.md` — the §1–2 framing (legible + trustworthy + AI-debug works + externally verifiable; on top of the
  working v0.2.0 data spine).
- `requirements.md` — the findings (§4) as numbered v0.3.0 capabilities (continuing the P-0xx scheme, e.g.
  P-061+), each carrying its OBSERVED→EXPECT acceptance shape.
- `working-route.md` — the capabilities sequenced into buildable chunks under epoch headers. Suggested ordering
  (route may resequence by dependency): keystone deterministic-L4 first (unblocks Investigate + Conductor) →
  window/chrome quick-wins → state-honesty (phantom services) + legibility (labels, status) → anomaly-surfacing
  → Tier1 reliability → Conductor closure + the integration UX test.
