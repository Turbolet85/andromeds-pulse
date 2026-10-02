# Scope — Live-only service truth

**Marker:** `2026-07-01-live-only-service-truth` · **Capability:** P-067 · **Intent:** F7 (HIGH PAIN) · **Epoch 3 — State honesty & legibility**

## Capability statement
Show only CURRENTLY-live services in the constellation / service surfaces; persisted/stale `service_registry` (corpus SQLite) entries are hidden or clearly marked historical/inactive; recency labels never imply liveness that is not there.

## Observed gap (from the 2026-06-28 live dogfood)
On a FRESH start with zero live telemetry (tray "Ingest: 0 sp/s"; Traces "No traces yet"), the constellation STILL renders ~6–7 service dots plus a hover tooltip "Listening · last span · just now". For an observability tool this is the cardinal sin: the UI contradicts itself and shows services that are not currently present. Two mechanisms compound:

1. **Persisted registry shown as if live.** The corpus `service_registry` is restored into `InMemoryServiceRegistry` at boot (chunk #71 `set_state_on_corpus_restore`); `services.list_with_states` → `registry.list_all()` returns EVERY tracked entry regardless of lifecycle state, and the webview renders each as a live dot.
2. **Recency label lies.** `set_state_on_corpus_restore` stamps `last_seen_unix_nano = restored_at_unix_nano` (boot time), so a service last actually seen in a PRIOR session reads "last span · just now" — the recency label itself implies liveness that is not there.

## What this chunk builds
- **A single "live" truth for the service surfaces.** Only currently-live services (currently receiving / recently-active per the 7-state lifecycle model) are shown as live constellation dots; persisted/restored/stale services are either hidden or explicitly rendered as historical/inactive — never as live dots.
- **Recency-label honesty.** The recency / "last span" label reflects the service's ACTUAL last-observed time, never a boot-time-reset value; a restored-but-not-re-observed service must not read "just now".
- **Zero-telemetry honesty (the acceptance condition).** With zero live telemetry the constellation/list shows no live services.

## Surfaces / contracts it touches (candidates — confirmed at /implement)
- `pulse-app/src/services_router.rs` — the `services.list_with_states` resolver (`ServiceListPayload` / `ServiceListItem`).
- `crates/triage/src/lifecycle/registry.rs` — `InMemoryServiceRegistry::list_all` + `set_state_on_corpus_restore` last-seen semantics; `ServiceListItem` already carries `state: ServiceLifecycleState`.
- `crates/triage/src/lifecycle/{state_machine,persistence,broadcast}.rs` — liveness classification over the 7 states (Active/Bootstrapping/Quiet vs Silent/Dormant/Archived/Unknown).
- Webview: `pulse-app/ui/src/hooks/use-service-constellation.ts` (polls the resolver) · `dashboard/routes/traces/ConstellationCanvas.tsx` · widget `constellation-pipeline.ts` / `constellation-types.ts` (dot construction + recency tooltip).

## Boundaries (explicitly NOT in this chunk)
- NOT a rebuild of the lifecycle state machine or the corpus `service_registry` schema — the 7-state model + the table stay; this chunk changes only what is SHOWN as live + the recency-label semantics.
- NOT the labeled/legible constellation (P-069 · F9 — dot names + health encoding), NOT the plain-language status line (P-070 · F10), NOT anomaly surfacing (P-068 · F8). Those are later Epoch 3 chunks.
- Preserve the working data spine (ingest → buffer → baseline tick → registry) untouched.

## Open questions (resolved at /phase P4 via AskUserQuestion)
1. **Liveness criterion** — which `ServiceLifecycleState`s count as "live" (Active only? Active+Bootstrapping+Quiet? plus a last-seen recency threshold?).
2. **Hide vs mark-historical** — persisted/stale services fully hidden from the constellation, OR rendered with a visually-distinct historical/inactive treatment.
3. **Filter site** — backend (resolver/registry filters or flags liveness) vs frontend (webview filters `ServiceListItem[]` by `state`) vs hybrid.
4. **Recency-label fix** — stop stamping `last_seen = boot_time` on corpus restore, vs. carry a separate "restored / never-re-seen this session" marker so the label reads "last seen: earlier session" rather than "just now".

## Acceptance (verification-matrix P-067 · method: integration)
With zero live telemetry the constellation/list shows no live services (persisted entries hidden or labeled historical); no recency label implies current liveness.

## Decisions (P4 — user-approved 2026-07-01)
The four open questions above were resolved at /phase P4 via AskUserQuestion (all recommendations accepted):
1. **Liveness criterion → recency of honest `last_seen`.** A service is live only while its last span is within
   the 60 s live window (mirrors the FSM `ACTIVE_TO_QUIET_THRESHOLD_SECONDS`). Corpus-restored/stale and
   long-quiet services fall out by recency.
2. **Treatment → HIDE entirely.** Non-live services get no dot and are excluded from the aria summary (extends
   the existing `archived`-hide). Chosen over mark-historical because the canvas has no per-dot labels until
   P-069, so marking would be color-only → SC 1.4.1 violation this chunk can't satisfy yet.
3. **Filter site → backend fixes `last_seen` (honest truth) + frontend recency gate.** The single consumer is
   the constellation; `services.list_with_states` is left unchanged (no new field/filter) — the recency
   predicate lives at the display site.
4. **Recency-label fix → required, not optional.** Stop clobbering `last_seen` on corpus restore
   (`registry.rs::set_state_on_corpus_restore`) — obs/security/a11y/arch converge on this.

**Premise correction (RESEARCH-CORRECTS-INTENT).** The intent's "Listening · last span · just now" tooltip
evidence was proven (via `research.md`) to be the whole-app **ConnectionDot** (chunk #89), NOT a per-service
constellation label — the chunk-#93 constellation has no per-dot tooltip. The falsified premise is the
mechanism (a per-service recency tooltip); the OUTCOME (UI shows services that aren't live) stands. This chunk
therefore targets the phantom constellation dots + the registry `last_seen` honesty (the true root cause);
the connection-dot "just now" phrasing is a distinct surface deferred to **P-070** (plain-language status).
Working-route line + human intent F7 left as historical record; correction recorded here + in the matrix
`notes`.
