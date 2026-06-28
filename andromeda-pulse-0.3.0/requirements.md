# Requirements — andromeda-pulse v0.3.0

_Derived from `andromeda-pulse-0.3.0/intent.md` §4 findings (OBSERVED→EXPECT). Capability ids continue the project scheme (v0.2.0 ended at **P-060** in `docs/v0_2_0/pulse-capability-spec.md`) → v0.3.0 = **P-061…P-077**. Each cap's EXPECT is its acceptance shape; the OBSERVED clause is carried into `verification-matrix.json` as `observed_gap`. ids are reused verbatim by the verification matrix._

## Theme 1 — Window & shell hygiene
- P-061 · Window geometry + movable shell — opens at a sane default size/position (centered or remembered); the frameless custom titlebar is a working drag region so the window moves (per intent F1).
- P-062 · Window size constraints — enforce a min-size + a sensible aspect-ratio constraint for the glance widget (per intent F2).
- P-063 · Predictable close + honest tray — window-close quits or minimizes-to-tray with a CLEAR "still running" indication; the dashboard is closable; the tray is the single honest running surface (per intent F3).
- P-064 · Suppress browser context menu — no default WebView2 menu (Back/Refresh/Save-as/Print/Inspect) in production; suppressed app-wide or replaced with an app menu (per intent F4).
- P-065 · Canvas not a browser image — the halo/canvas is not treated as a saveable/draggable browser image (per intent F5).
- P-066 · Widget→dashboard navigation — an explicit in-app affordance expands the glance widget into the full dashboard (not tray-only) (per intent F6).

## Theme 2 — State honesty & legibility
- P-067 · Live-only service truth — show only CURRENTLY-live services; persisted/stale `service_registry` entries are hidden or clearly marked historical/inactive; recency labels never imply liveness that is not there (per intent F7 — HIGH PAIN).
- P-068 · Anomaly surfacing — errors/anomalies surface to the TOP of the Traces table and/or are visually flagged + filterable; the user sees "what is wrong" first, not last (per intent F8 — HIGH PAIN).
- P-069 · Legible labeled constellation — each constellation dot is identifiable (name on/near it or on hover) with health/severity clearly encoded (per intent F9).
- P-070 · Plain-language status — a human-readable status states sources connected / telemetry arriving / buffer state (e.g. "Receiving from 5 services · ~540 spans/s · buffer 2 min / 10 min") (per intent F10).
- P-071 · Self-explaining empty states — Metrics/Logs (and other empty surfaces) explain themselves with an actionable hint (per intent F11).

## Theme 3 — AI-debug climax
- P-072 · Investigate actions functional — each Investigate action (diagnose latency outlier / find error correlation / trace failed request / summarize service health) runs a real LLM/MCP-backed analysis with visible progress + a result; failures surface, never silent (per intent F12; gated on P-073).
- P-073 · Deterministic env-gated L4 mode — an env/flag-gated deterministic L4 mode (canned `L4Output`, no GPU/3B; reuse the `StubInferenceRunner` pattern) so the digest→L4→incident path completes reproducibly for demos / tests / external verification (per intent F13a — KEYSTONE).
- P-074 · Tier1 incident-path reliability under load — coalesce repeated identical hard-signals into one digest + an elastic queue so a sustained storm yields ONE reliable incident (today `TIER1_QUEUE_CAP=3` + ~4 s L4 latency drops ~75% of storm digests) (per intent F13b — KEYSTONE).

## Theme 4 — External verification
- P-075 · Conductor e2e + delegated-timing verification (dynamic-external) — with the deterministic L4 mode (P-073), Conductor drives telemetry → a known incident → MCP read-back, proving end-to-end incident content-fidelity + the 4 delegated timing caps **P-025** (halo hue ≤2 s) / **P-027** (constellation discovery ≤5 s) / **P-037** (report render ≤2 s) / **P-045** (counter refresh ≤1 s) (per intent F14; covers P-025/P-027/P-037/P-045).

## Theme 5 — Test gap & housekeeping
- P-076 · Integration UX e2e test — an integration/e2e test exercises the REAL assembled product path (launch → telemetry → Traces render → storm → incident → Investigate) so integrated-UX breakage is caught going forward (per intent F15).
- P-077 · Demo telemetry injector formalized — `crates/ingest/examples/inject_demo.rs` becomes a supported, tracked dev/test tool (per intent §5 housekeeping). _Companion cleanup: retire `.andromeda/context/api-surface.md` once the code-graph `tree.db` is built._
