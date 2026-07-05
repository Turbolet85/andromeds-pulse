# Session Handoff

**Last Updated:** 2026-07-05T23:46:22Z
**Branch:** chore/migrate-pulse-to-v3
**Status:** clean
**Last Commit:** `2026-07-05-constellation-severity-live-wiring` — constellation severity live-wiring (workspace-key single-source; P-079)

## Position
- Done: `2026-07-05-constellation-severity-live-wiring` (P-079) — single-sourced the incident workspace key so the FILTER key = the producer's STAMPED workspace (detected project root). Per-service severity + the incidents panel now light up under a live storm. **Operator live-verify PASSED** (dots color-differentiate: payment-service red/autonomous, others blue/healthy; incidents panel + unread badge populate). **P-079 verified → 14/19 v0.3.0 caps.**
- Next: **Incidents-panel dropdown layout bug** (first markerless, Epoch 3 — a focused frontend chunk: bounded popover, no window stretch, design-token background) → `/andromeda-phase`.

## Work done
Backend-only (4 files): `digest_runtime.rs` +`resolve_workspace_for_incidents(detected, data_dir) -> (String, DigestProjectContext)` (single-source parity by one destructure) · `main.rs` hoist `detect(current_dir())` + derive both halves from the resolver · new `pulse-app/tests/integration_constellation_severity_workspace_key.rs` (5 tests: parity incl. `\\?\` / fallback / storm→`list_active(key)`≥1 / zero-state) · matrix P-079 → verified. Gates: fmt · nextest --workspace 1727/1727+1skip · clippy --all-features · integration 5/5 · storm boot-smoke (0 panics, 1 incident severity=error, `active_incident_queue_depth=5`, both resolvers querying, zero orphan).

## Drift resolved
none — all 7 spec-source detectors returned `proposals: []` (backend chunk: no new arch resources/deps/APIs/crates/schema/UI; PII redacted✓ per §8; tests present). 0 amendments · 0 escalations · cascade no-op.

## Notes
- **2 pre-existing frontend bugs EXPOSED by lighting up the dead surfaces (NOT P-079 regressions — P-079 is 100% backend)** → filed as route entries (operator-directed): (1) **Incidents-panel dropdown layout bug** (~chunk #91 — stretches window, white/mis-clipped bg; now first markerless); (2) **Traces auto-refresh** (`viz.query.traces` runs once at mount, never re-polls — priority-BUMPED to second markerless; originally filed at P-069's wrap). Do NOT conflate with P-079.
- **Gate-deferral closure:** P-079's `nextest --workspace` re-run CLOSED the source-delta-proportional deferral P-069 (webview-only) left open. self-verify release/a11y half deferred again (backend chunk, zero frontend delta) → re-runs at the next frontend-touching chunk.
- **Curation:** Tier 3 ×1 (single-source `(key, context)` parity pattern) · filtered 1 dup (backend boot-smoke) + 1 low-confidence.
- **Follow-up (obs, deferred — a future obs chunk):** `incidents.list_active.request` logs `item_count` but the default-deny AllowList redacts it, so the "row_count_returned ≥ 1" agent-verifiable signal isn't visible; allowlist the aggregate-safe count for that target. (Verified this chunk via `active_incident_queue_depth`=5 + `incidents_created_total`=1 + the integration test.)
- **Carried deferred (pre-existing, still pending):** the a11y Playwright webServer serves an un-rebuilt `dist` — run `npm run build --prefix pulse-app/ui` before the p11 spec (apply via `/andromeda-wrap-session --review`).
- Branch local-only — **NOT pushed**. Last failed command: none.
