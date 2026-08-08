# Session Handoff

**Last Updated:** 2026-07-10T16:21:37Z
**Branch:** chore/migrate-pulse-to-v3
**Status:** clean (wrapped)
**Last Commit:** `2026-07-10-incidents-floating-window-disclosure` — separate `findings` + `report` floating windows (incidents disclosure + diagnostic report), cross-window-event coordination, `use-findings` re-poll CARRY

## Position
- Done: `2026-07-10-incidents-floating-window-disclosure` — unread incidents surface in a **separate borderless `findings` window docked below the compact widget** (sizes to the incident count, caps ~8 rows → scrolls), and a row-select opens a **separate `report` window** positioned relative to it (Modal `fill` variant, no backdrop). Cross-window dismiss + focus restore via Tauri events; the `use-findings` badge re-poll CARRY landed (silent 1s refresh + 0→N announce). **Completes Epoch 3 — State honesty & legibility.** Operator visual verify PASSED. No matrix cap (operator-directed refinement beyond P-061..P-082).
- Next (first markerless): **Conductor e2e verification closure (P-075)** — Epoch 4 (Polish & ship: verification) → `/andromeda-phase`. (Then P-076 Integration UX e2e / A11y verification / P-077 demo-injector.)

## Work done
Frontend (new): `FindingsWindow.tsx` · `ReportWindow.tsx` · `use-findings-window.ts` (+tests). Frontend (mod): `App.tsx` (window-label branches) · `use-window-label.ts` · `use-findings.ts` (CARRY re-poll) · `CompactWidget.tsx` · `FindingsCounter.tsx` (aria-controls dropped, haspopup→dialog) · `components/Modal.tsx` (opt-in `fill` variant) · `Report.tsx` (variant passthrough) · p8/p9/keyboard-focus a11y specs (re-homed to the window surfaces). Backend/config: `window.rs` (`sanitize_window_label` +`findings`/`report`) · `tauri.conf.json` (2 window decls) · `capabilities/default.json` (window scope + `allow-set-position`/`allow-set-size`). **Gates green:** typecheck · lint · vitest **788** · a11y (playwright **33** · lighthouse 7/7 · pa11y 7/7 · regression **0/0**) · fmt · `cargo build -p pulse-app` (ACL) · clippy -p pulse-app · capability-drift/widening clean. Boot smoke (warm re-embed): 0 panics · storm→10 incidents (deterministic L4) · 100 traces.

## Drift resolved
3 amendments (**arch** — register the cross-window events `findings:dismissed`/`report:open`/`report:closed` in §Occupied Resources as a distinct class; **layout-templates** — add the `findings` + `report` windows to desktop-webview Primary screens; **a11y-plan** — §5 findings/report focus order + cross-window restoration) + 3 sidecars. **0 escalations.** Cascade: a11y.md rule got the cross-window focus + `aria-controls`-dangling note; arch→CLAUDE.md/stack.md + layouts→frontend.md cascades no-op (granular). 4 detectors clean (security/design/tests/obs).

## Notes
- **Curation:** Tier 2 ×1 (`frontend.md` — CROSS-WINDOW DISCLOSURE: separate-window-positioned-relative-to-parent + Tauri-event coordination + cross-window focus restore/dismiss-suppression + Modal `fill` variant + `setSize` size-to-content + data-via-event-with-fallback). Tier 3 ×1 (`session-learnings.md` — live-verify gotchas: incidents need `ANDROMEDA_PULSE_L4_DETERMINISTIC=true`; the DuckDB append-path stalls after ~10 min sustained storm+L4, restart clears). Filters: 1 dup (operator-visual-verify-for-layout — dup of 2026-07-05) · 0 conflict · 0 deferred.
- **Route:** 2 CARRYs — headful residual (live below-widget dock + cross-window Esc→badge focus) → **P-076**; findings/report window a11y coverage → **A11y verification**.
- **Known limitation (user-chose handoff-note, NOT a route chunk):** the **DuckDB append-path stalls after ~10 min of sustained 10k/s storm + deterministic-L4** — ingest keeps receiving (`ingest.tick` span_count climbs) but `duckdb.append` stops → `viz.query.traces` 0 rows + no new incidents; a fresh restart clears the in-memory ring buffer. Pre-existing chunk-#99 DuckDB-connection-contention class (NOT this chunk — it touches no ingest/buffer/viz/L4); the chunk-#99 load-profile/reliability suite is the owner. It's a stress-test edge (extreme sustained load), not normal dev usage. Live-verify recipe: fresh app + `L4_DETERMINISTIC=true` + fresh data dir + pump inject_demo + glance within a few minutes.
- **Deferred gate (source-delta-proportional):** `cargo nextest run --workspace` RUN — the isolated additive `window.rs` label enum is compile+lint+ACL+boot-smoke verified; the cold ~30-binary test-link was deferred as OOM/disk-risky on 26G free. **DISK CONSTRAINT LIFTED 2026-07-25** — the D: Dev Drive was expanded 200 → 300 GB (**124.7 GB free**), so this deferral MUST be CLOSED at the next `.rs`-touching chunk; the `free-disk.ps1 -Execute` prerequisite no longer applies. Memory remains a real constraint (the ~30-binary parallel compile can rustc-OOM): `cargo build --workspace --tests --jobs 4` first under `CARGO_INCREMENTAL=0`, then `cargo nextest run --workspace --profile ci` (per `.claude/rules/testing.md` 2026-06-28 + 2026-07-25).
- Branch **pushed 2026-07-25** — `chore/migrate-pulse-to-v3` now tracks `origin/chore/migrate-pulse-to-v3`. Last failed command: none.

## Session End Status
Completed normally at 2026-07-25 13:45:01
