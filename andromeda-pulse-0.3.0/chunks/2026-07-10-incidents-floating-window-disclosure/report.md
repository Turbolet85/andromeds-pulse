# Report — 2026-07-10-incidents-floating-window-disclosure

**Chunk:** Incidents floating-window disclosure — separate borderless always-on-top Findings window docked below the compact widget (reuses P-080 FindingsDropdown content) + use-findings badge re-poll CARRY
**Date:** 2026-07-10
**Commits:** (uncommitted; wrap commits) — Incidents floating-window disclosure

## Changes (structured — detectors read this)
- **Files:**
  - New (frontend): `pulse-app/ui/src/widget/FindingsWindow.tsx` (+`.test.tsx`), `pulse-app/ui/src/widget/ReportWindow.tsx` (+`.test.tsx`), `pulse-app/ui/src/hooks/use-findings-window.ts` (+`.test.ts`)
  - Modified (frontend): `App.tsx` (window-label render branches), `hooks/use-window-label.ts` (+test), `hooks/use-findings.ts` (+test — CARRY re-poll), `widget/CompactWidget.tsx`, `widget/FindingsCounter.tsx` (+test), `components/Modal.tsx` (fill variant), `report/Report.tsx` (variant passthrough)
  - Modified (a11y specs): `tests-a11y/axe/p8-findings-dropdown.spec.ts` (→ findings-window audit), `tests-a11y/axe/p9-diagnostic-report-modal.spec.ts` (→ report-window audit), `tests-a11y/keyboard-focus/widget-and-modals.spec.ts` (findings-window ring; report-modal test → P-076 headful CARRY)
  - Modified (backend/config): `pulse-app/src/window.rs` (`sanitize_window_label` bounded enum + test), `pulse-app/tauri.conf.json` (2 new window declarations), `pulse-app/capabilities/default.json` (window scope + grants)
- **Symbols / APIs:**
  - **NO new TauRPC procedure, port, endpoint, env var, or workspace crate.** `incidents.*` TauRPC + `pulse://stream/incidents` consumed unchanged; `pulse-app/ui/src/bindings/index.ts` UNCHANGED.
  - New window labels (frontend + `window.rs::sanitize_window_label` bounded enum): `"findings"`, `"report"` (join `main`/`compact-widget`).
  - New capability grants (core-window, NOT TauRPC): `core:window:allow-set-position`, `core:window:allow-set-size`; capability `windows` scope extended `["compact-widget","main","findings","report"]`.
  - New frontend module `use-findings-window.ts` exports: `computeFindingsWindowPosition`, `computeReportWindowPosition`, `clampIntoMonitor`, `resizeFindingsWindow`, `openFindingsWindow`, `hideFindingsWindow`, `openReportWindow`, `closeReportWindow`, `dismissFindings`, `onFindingsDismissed`, `onReportOpen`, `onReportClosed` + event consts (`FINDINGS_DISMISSED_EVENT`, `REPORT_OPEN_EVENT`, `REPORT_CLOSED_EVENT`).
  - `Modal` (`components/Modal.tsx`) gains optional `variant?: "overlay" | "fill"` (default `"overlay"` — existing consumers unchanged); `Report` threads it through.
  - Cross-window Tauri events (via `@tauri-apps/api/event` emit/listen, `core:event` in `core:default`): `findings:dismissed`, `report:open` (`{incidentId}`), `report:closed`.
- **Crates / modules:** none added/removed. `pulse-app` binary only (`window.rs`). No library crate touched.
- **Dependencies:** none added/bumped.
- **Schema / config:**
  - `tauri.conf.json` `app.windows`: `findings` (380×220, hidden, decorations:false, alwaysOnTop, skipTaskbar, resizable:false) + `report` (600×680, hidden, decorations:false, alwaysOnTop, skipTaskbar, resizable:true).
  - `capabilities/default.json`: `windows` array + `core:window:allow-set-position` + `allow-set-size` (rationale appended to the description block).
  - No corpus/DuckDB schema change; no config.toml key.
- **Coverage of new surfaces:**
  - `findings` window (unread-incident disclosure list) → validation n/a · instrumentation: window-lifecycle bounded via `sanitize_window_label` obs allowlist (label = `findings`) · PII redacted✓ (no incident body logged; reuses P-080 content) · tests unit(vitest FindingsWindow + use-findings-window) + a11y(p8 axe + keyboard-focus) · a11y✓ (disclosure contract, cross-window focus/Esc restore, focus ring, SC 1.4.1/2.4.3/4.1.2/4.1.3) · tokens design-token✓ (`--color-raised-2` opaque popover, borders-only, custom scrollbar)
  - `report` window (Diagnostic Report, fill mode) → validation n/a · instrumentation: label `report` in obs bounded enum · PII redacted✓ (report content already scrubbed at chunk #88 resolver) · tests unit(vitest ReportWindow) + a11y(p9 axe) · a11y✓ (dialog role/name, focus trap, Esc, SC) · tokens design-token✓ (`--color-base` fill bg, `--color-raised-3` card)
  - `use-findings.ts` live re-poll (CARRY) → ~1s silent-background refresh (keep-last on error) + 0→N-edge announce (SC 4.1.3) · tests vitest fake-timer (re-poll, keep-last, announce-once)

## Deviations from intent
1. **Report is a SEPARATE `report` window** (not hosted in the findings window, not cross-window-in-widget). Plan left "where the Report opens once rows move to a window" implicit; through operator-driven live verify it resolved to: initially in the findings window → then a dedicated `report` window positioned relative to the findings dropdown (mirroring dropdown-vs-widget). **Justification:** hosting the report in the compact findings window stretched/moved the dropdown (operator-caught); a separate window keeps the dropdown compact + stable and gives the report a readable surface. Added the 4th window label + `allow-set-size`.
2. **Findings window sizes to its incident count** (`core:window:allow-set-size`, `setSize` from its own webview; caps ~8 rows then the list scrolls). Operator-directed (a fixed-size dropdown read as wrong for few incidents).
3. **`Modal` gains an opt-in `fill` variant** (`components/Modal.tsx` — NOT in the plan's file list). **Justification:** in a dedicated window the modal's translucent backdrop shows as a grey frame; `fill` makes the report fill its window (solid `--color-base`, no backdrop). Default `overlay` preserved → Settings/Investigation modals unchanged (44 tests pass). In-scope-by-extension (necessitated by the separate-report-window design).
4. **`FindingsCounter` `aria-controls` removed; `aria-haspopup` `"true"→"dialog"`.** The disclosed panel is now cross-document — a dangling `aria-controls` id-ref would be a new axe violation.
5. **a11y specs re-homed** (not in the plan's modify list): `p8` → findings-window surface, `p9` → report-window surface, keyboard-focus → findings-window ring (the report-modal-via-dropdown test dropped → P-076 headful CARRY). In-scope-by-extension (they tested the surface this chunk moved).
6. **Row-select is no longer a dismiss trigger** (scope said "row-select → hide"); it opens the report window (dropdown stays). Esc/blur/mark-all-read still dismiss. Justified by #1.
7. **Panel token = `--color-raised-2`** (scope said `--color-inset`; corrected at /phase val-1 per the a11y extract + design-tokens.md — dropdowns/popovers surface).

## Decisions & corrections
- **Window mechanism = frontend-side** (extends the P-066 `use-toggle-dashboard` precedent; no new TauRPC) — confirmed at /phase P4 AskUserQuestion.
- **Report as its own window** (operator directive, live verify): "make report as separate window so its not touching dropdown, same job we make for dropdown against widget."
- **Findings window sizes to content** (operator directive): stretch to the incident count, cap + scroll (dropdown best practice).
- **Report fill mode** (operator directive): the modal backdrop grey frame is wrong in a standalone window → fill.
- **Cross-window incident id delivery = Tauri event** (`report:open {incidentId}`), report window falls back to the first active incident when shown without an event (keeps it non-blank + a11y-testable).
- **Pre-existing follow-up surfaced (NOT this chunk):** the DuckDB **append-path stalls after ~10 min of sustained storm + deterministic-L4** — ingest keeps receiving (span_count climbs) but `duckdb.append` stops → `viz.query.traces` returns 0 + no new incidents; a fresh restart clears it. Documented chunk-#99 DuckDB-connection-contention class; this chunk touches none of ingest/buffer/viz/L4. Worth its own follow-up chunk (route carry).
- **Boot-smoke / operator-verify learning:** the automated obs-log smoke is layout-blind — the operator leave-running visual verify caught 3 real UX issues (fixed-size window · cramped in-window report · grey backdrop) the gates could not; 3 in-chunk fix-loop re-entries. Confirms operator visual verify is load-bearing for cross-window layout chunks.

## Outcome
- **Acceptance criteria met** (findings window docks + sizes to content · separate report window · cross-window dismiss/focus · a11y · CARRY re-poll · capability grants minimal). **No verification-matrix cap linked** (P-061..P-082 all claimed; operator-directed refinement beyond the numbered plan).
- **Gates green:** `npm run typecheck` · `npm run lint` · `npm run test` (vitest **788**) · `npm run test:a11y` (playwright **33** · lighthouse 7/7 · pa11y 7/7 · regression **0/0**) · `cargo fmt --check` · `cargo build -p pulse-app` (ACL compile-embed: `findings`+`report` windows + `allow-set-position`/`allow-set-size`) · `cargo clippy -p pulse-app --all-targets --all-features -D warnings` · `cargo xtask capability-drift` (clean) · `cargo xtask capability-widening-check` (clean).
- **Deferred (source-delta-proportional):** `cargo nextest run --workspace` RUN — after check-mode clippy, a build-mode cold ~30-binary test-link on a 26G-free disk (documented OOM/rlib risk); the isolated additive `window.rs` label enum is compile+lint+ACL+boot-smoke verified. Re-run after freeing disk.
- **Boot smoke (warm re-embed):** 0 `app.panic.fatal` · 0 ERROR · webview + frame metrics · storm→`interpretation.incident.created` 10 (deterministic L4) · `incidents.list_active` re-poll firing · `viz.query.traces` 100 rows. **Operator visual verify PASSED** (findings window docks + sizes to content; separate report window fills cleanly; dropdown stable).
