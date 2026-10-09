# Research — Live-only service truth (P-067)

_Grounded via direct reads (code-graph `refs` unavailable for this symbol set; direct reads used). Extracts in `.andromeda/runs/2026-07-01T18-13-37Z-phase/`._

## Data flow (current)
- **Resolver** `services.list_with_states` (`pulse-app/src/services_router.rs:93`) → `registry.list_all()` returns EVERY `ServiceListItem` in the `InMemoryServiceRegistry` DashMap, then joins the active-incident registry to enrich `priority_tier`. **No liveness filter.**
- **Contract** `ServiceListItem` (`crates/triage/src/lifecycle/registry.rs:54`): `service`, `state: ServiceLifecycleState`, `last_seen_unix_nano`, `manual_override`, `priority_tier`. Crosses the bridge via specta.
- **Webview** `use-service-constellation.ts` polls the resolver every 1s → `ServiceListItem[]` → `ConstellationCanvas` (used by BOTH the dashboard Traces hero and the compact widget, `CompactWidget.tsx:78`). `visibleDots()` (`widget/constellation-types.ts:86`) ALREADY drops `archived` and dims by state (`BRIGHTNESS_BY_STATE`: active 1.0, bootstrapping 0.85, quiet 0.6, silent 0.4, unknown 0.5, dormant 0.25, archived 0). `constellationSummary()` returns "no active services" only when ALL are archived.
- **7-state FSM** (`state_machine.rs:22`): Unknown → Bootstrapping → Active ↔ Quiet → Silent (↔ Quiet) → Dormant → Archived. Docstring calls `Unknown` "services seen via prior corpus but not yet observed in the current session (chunk #69 corpus restore territory)" — but restore does NOT use it (see below).

## Root cause of F7 (phantom services on a fresh boot)
1. **Boot restore surfaces persisted services.** `main.rs:623-637`: `lifecycle_persistence.load_all()` → `InMemoryServiceRegistry::from_entries(entries)` (preserves the persisted `ServiceRegistryEntry`, incl. the REAL prior-session `last_seen`), THEN a loop calls `reg.set_state_on_corpus_restore(&service, state, restored_at)` per service (to emit `pulse://stream/service-lifecycle` CorpusRestore events).
2. **The restore clobbers `last_seen` to boot time.** `set_state_on_corpus_restore` (`registry.rs:264-294`) does `self.entries.insert(... last_seen_unix_nano: restored_at_unix_nano ...)` — OVERWRITING the real last_seen (+ first_seen + last_transition) with boot time, and keeping the persisted state (e.g. Active/Quiet). Net: every restored service ends up looking recently-seen and live-ish.
3. **No demotion on a quiet boot.** `tick_all` only transitions services that have baseline snapshots; with zero telemetry the baseline is empty → restored Active/Quiet services stay live-looking forever → phantom dots. **Neither `state` (persisted live-ish) nor `last_seen` (clobbered to now) can currently distinguish restored-stale from live.**

## The intent's "Listening · last span · just now" tooltip — CORRECTED
- Grep pins this to the **ConnectionDot** titlebar tooltip: `components/ConnectionDot.tsx:144` renders "last span {lastSpan}"; `formatLastSpanAgo` (`:67`) returns "just now" when `last_span_ago_ms < 1000`; `connectionStateLabel` (`:48`) returns "Listening". Driven by `connection.current_state` (the ingest CONNECTION FSM — Listening/Receiving/Idle/Stalled/ReceiverFailed, chunk #89), **not** the per-service registry.
- The current `ConstellationCanvas` (chunk #93) has NO per-dot hover tooltip (canvas + aria summary only).
- **RESEARCH-CORRECTS-INTENT (per session-learnings 2026-06-28):** the intent's tooltip evidence conflates the whole-app connection dot with the constellation. P-067's real, still-present defect is the **phantom constellation dots on a fresh boot** (mechanism above). The connection-dot recency phrasing ("Listening · just now") is a distinct legibility concern that belongs to **P-070 (plain-language status)** — flagged here, NOT addressed in this chunk. The falsified premise is the mechanism (a per-service recency tooltip), not the outcome (UI shows services that aren't live).

## Candidate fix sites
- **Backend (authoritative truth):** `registry.rs::set_state_on_corpus_restore` (stop clobbering `last_seen`) + a liveness predicate over `(state, last_seen, now)` in `triage`; `main.rs` restore loop (unchanged shape); `services_router.rs` (supply `now` if the resolver filters).
- **Frontend (render gate):** `constellation-types.ts::visibleDots` + `constellationSummary` (extend the existing `archived` hide to a liveness gate — canvas has no per-service DOM, so "hidden" = not drawn + excluded from the aria summary count).

## Tests to mirror
- **Triage unit:** `registry.rs` `#[cfg(test)]` (fresh_registry / tick_all / set_state_on_corpus_restore) — add liveness-predicate + last_seen-preservation tests here.
- **Rust integration:** `pulse-app/tests/unit_services_router.rs` (`#[tokio::test]`, `ServicesApiImpl::new`, manual-override fixtures) — add a "restored-stale hidden / live shown" case.
- **Webview:** `widget/constellation-types.test.ts` (visibleDots/summary) + `dashboard/routes/traces/ConstellationCanvas.test.tsx` — assert live shown, stale hidden, zero-telemetry → empty summary.
- **Zero-telemetry scenario** (obs + tests extracts): boot with a persisted registry + zero live spans → constellation shows no live services.

## Cross-domain constraints (from the 7 extracts)
- **obs / a11y / security converge:** DON'T reset `last_seen` on corpus restore; hidden services excluded from the aria summary + focus order intact; log only aggregate counts (`service_count_live` / `_historical`), never the registry payload.
- **design:** hide OR mark-historical (if marked, pair color with label/icon per SC 1.4.1 — but the constellation has no per-dot labels until P-069, so marking would be color-only → hide is the a11y-clean choice this chunk); live keeps Halo breathing, stale static/dim; `prefers-reduced-motion` respected.
- **arch:** no new IPC procedure or broadcast event (`services.list_with_states` + `pulse://stream/service-lifecycle` locked); no corpus schema change; code stays in `triage` + `pulse-app/src/services_router.rs` per the module DAG.
- **tests:** unconditional standard gate set (fmt / clippy / nextest / capability-drift + webview lint/typecheck/test) since `pulse-app/ui/**` is touched.
