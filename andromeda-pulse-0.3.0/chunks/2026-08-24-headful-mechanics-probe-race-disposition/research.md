# Codebase Research — 2026-08-24-headful-mechanics-probe-race-disposition

## Scope
- **Depth:** moderate · **Reads:** 11 · **Globs/Greps:** 12 · **Code-graph queries:** 4 (rust ×2, ts ×2; trace `tree-query-2026-08-24-headful-mechanics-probe-race-disposition.json`, all `db_state: fresh`)

## Files inspected
- `pulse-app/src/window.rs` (1–60, 100–300 + tests tail) — the whole window-event surface: which events are handled, which windows they are guarded to, and what each emits.
- `pulse-app/src/tray.rs` (255–285) — `focus_or_show_window`, the tray restore path.
- `pulse-app/tauri.conf.json` (windows array) — all four windows declared, `visible:false`, no explicit `url`.
- `pulse-app/capabilities/default.json` (permissions array) — the nine granted `core:window:*` permissions.
- `xtask/src/webview_drive.rs` (40–115, 296–380) — the `STAGES` table, `stage_ids`, `stage_halves` verdict readers.
- `pulse-app/ui/tests-e2e/webview-drive.mjs` (500–580) — launch stage, navigation signal, blank-victim recovery.
- `pulse-app/ui/src/hooks/use-findings-window.ts` — `openFindingsWindow` / `openReportWindow` show sites.
- `pulse-app/ui/src/hooks/use-toggle-dashboard.ts` — `main.show()`.
- `pulse-app/ui/src/components/Titlebar.tsx` — the drag-region spans + the `Toggle dashboard` control.
- `pulse-app/ui/src/components/WindowControls.tsx` — the shipped accessible names.
- `pulse-app/ui/src/hooks/use-suppress-browser-chrome.ts` — the app-wide contextmenu suppression.
- `.andromeda/test-plan.md` §3 — boot-smoke trigger paths + the Direct-binary smoke variant.

## Graph impact
- **`clamp_to_aspect_bounds`** — 10 rows (rust). Exactly ONE production caller: `pulse-app/src/window.rs:230`, inside the debounced `WindowEvent::Resized` arm; the other 9 are `pulse-app/tests/unit_window_constraints.rs`. CARRY 3's named neutralization lever is therefore precise and single-site — mutating it cannot silently disturb another caller.
- **`stage_halves` / `stage_observed` / `stage_ids`** — 26 rows (rust), every one inside `xtask/src/webview_drive.rs` (production arms + colocated tests, e.g. `:1022`, `:1162-1166`, `:1233`). The stage surface is self-contained: adding or declining a stage touches one Rust file plus the node driver, with no cross-crate blast radius.
- **`openFindingsWindow()` @ `pulse-app/ui/src/hooks/use-findings-window.ts:129`** and **`openReportWindow()` @ `:181`** (ts) — consumers are `pulse-app/ui/src/widget/ReportWindow.tsx:9,17,26` plus the hook's own test. These are the show sites for the two windows Half B names as the visible blank-window exposure.

## Patterns detected
- **Two-half stage verdict** (`xtask/src/webview_drive.rs:296-380`): `stage_halves(id, records, dom)` returns `StageHalves{obs, dom}` with `None` meaning inapplicable; 13 stages are registered in the `STAGES` const (`:53`) with an `id` + a `what` sentence. Adding a stage = one `Stage` entry + one arm in `stage_halves` + a driver step that `record(...)`s the DOM observation.
- **Debounced settle, not per-event** (`window.rs:211-243`): `Resized` bumps a generation counter, spawns a task that sleeps `ASPECT_DEBOUNCE` (150ms) and clamps only if no newer resize arrived. This is the sanctioned quiet boundary the obs extract requires any resize-attached emission to ride.
- **Navigation signal is `getUrl()` leaving `about:blank`** (`webview-drive.mjs:500-547`): the driver polls that predicate, re-navigates victims, and records `recovered_from_blank` in the launch stage record. The Tauri label is init-script-injected into `about:blank` too, so label-readability is explicitly not a navigation signal.
- **Frontend-owned window presentation**: `findings.show()` (`use-findings-window.ts:162`), `report.show()` (`:208`), `main.show()` (`use-toggle-dashboard.ts:28`) — all via `@tauri-apps/api/webviewWindow`, each preceded by a `setPosition`.
- **Warn-only-on-failure emission**: every Rust window operation (`show_failed`, `set_size_failed`, `set_position_failed`) emits a `warn!` on the error path under `app.boot.window.show`; the SUCCESS paths emit nothing.

## Conventions to follow
- **Bounded window label at every emit site**: `sanitize_window_label` (`window.rs:107`) collapses to the 4-label set + `unknown`; `tray.signpost.shown` (`window.rs:169`) carries exactly `window_label`.
- **Accessible-name-first selectors, measured at HEAD**: the shipped set is `aria-label="Minimize"` / `"Maximize"` / `"Close to tray"` (`WindowControls.tsx:45,56,67`) and `"Toggle dashboard"` (`Titlebar.tsx:105`). Note `Maximize` ships alongside the pair the a11y extract named — any stage binding must use the measured set, not a remembered one.
- **Drag region spans the titlebar minus buttons**: `data-tauri-drag-region` at `Titlebar.tsx:67, 83, 90` plus the `titlebar__grow` flex spacer at `:101` — matching design-system and layout-templates, so a press point on the grow span is inside the region and clear of every control.
- **Single-file stage extension**: `xtask/src/webview_drive.rs` + `pulse-app/ui/tests-e2e/webview-drive.mjs`, the shape all six predecessor stages used.

## New files to create
- (none expected) — every landed stage extends the two existing harness files. A Half B path that ships an obs guard would add one allowlist-guard test under `pulse-app/tests/` (obs-plan §8 requires guards live there, never in `observability.rs`'s own `mod tests`).

## Files to modify
- `xtask/src/webview_drive.rs` — probe result handling, `STAGES` entries for landed stages, `stage_halves` arms, colocated tests.
- `pulse-app/ui/tests-e2e/webview-drive.mjs` — the probe itself plus a driver step per landed stage.
- `.andromeda/test-plan.md` §6 — **NOT a touchpoint** (spec master); recorded as an Expected amendment instead.
- Half B path-dependent: `pulse-app/src/window.rs` and `pulse-app/src/observability.rs` only if an instrument/guard path is chosen; `pulse-app/tests/` for its allowlist guard.
- `pulse-app/capabilities/default.json` — transiently mutated by the RED arm and restored byte-identical; not a shipped modification.

## Open questions
- **Half B path selection — measure (a) vs guard (b) vs instrument-then-measure (c)** → blocks: **plan-decision**. Research establishes that (a) as literally worded ("read a plain boot's per-webview URLs, no driver attached") has no existing mechanism: with no WebDriver attached, nothing in the shipped product reports a webview's URL, and attaching a driver is precisely what makes it not a plain boot. A third shape therefore exists — a one-shot Rust boot-time navigation check emitting a bounded record (WARN when a window is still `about:blank`) — which measures production exposure AND leaves a durable detector, at the cost of a new obs target + exact allowlist leaf + guard test. P4 resolves this with the operator.
- **Whether the drag/resize stages may land with a DOM-only verdict** → blocks: **implementation-scope**. Both mechanics emit nothing on their success paths (see premise 2 below), so their obs half is `None` unless this chunk adds an emission. The predecessor already landed five of six stages DOM-half-only, so precedent permits it; the file list stays provisional until the probe decides which stages land at all.

---

## Scope premise closure

1. **VERIFIED** — no prior in-repo measurement of W3C Actions / `setWindowRect` exists. `pulse-app/ui/tests-e2e/webview-drive.mjs` contains zero `action(` / `performActions` / `setWindowRect` occurrences; its entire interaction surface is `.click()`, `.keys()` and `.execute()`. `webdriverio ^9.31.2` ships the APIs, so the open question is transport reach (msedgedriver → WRY/WebView2), not availability. The `[inferred]` tag is dropped.

2. **FALSIFIED** — `[premise-corrected: WindowEvent::Moved is guarded to the MAIN window only and emits no tracing record; it calls store.record_move_throttled into window-geometry.json (window.rs:204-210). The compact widget is deliberately excluded — "the compact widget always boots to its fixed corner (P-061 correction — no widget remembered geometry to restore or drift off-screen)". The Resized clamp likewise emits only on FAILURE (set_size_failed, window.rs:236). So neither mechanic has an obs half at HEAD: a drag stage on the widget produces no handled event at all, a drag on the dashboard produces a file write and no record, and a successful aspect clamp is silent.]* Consequences: the drag stage's verdict is DOM-half-only unless this chunk adds an emission at the debounced settle point; and the security extract's open question ("does the Moved handler log coordinates?") is answered NO — nothing is logged, so the coordinate-logging ban is already satisfied.

3. **VERIFIED** — tray restore runs through `focus_or_show_window` (`pulse-app/src/tray.rs:273`), reached from the tray menu/icon, an OS-native surface with no WebDriver reach (design-system §Surface: desktop-native states tray interaction is OS-controlled). The repetition proof's *restore* leg is therefore the declinable part; the second *close* press remains a real affordance press, and `core:window:allow-show` is granted, so a programmatic re-show is available as SETUP between the two closes without weakening the assertion.

4. **VERIFIED, and confirmed against the owning code** (per the a11y precedent that an absence must be checked against the component that owns the surface) — `handle_window_event` handles exactly `CloseRequested` (`:183`), `Moved` (`:204`) and `Resized` (`:211`); Tauri 2 exposes no Shown variant. Window presentation for the three race-victim-capable windows is owned by the FRONTEND: `use-findings-window.ts:162` / `:208` and `use-toggle-dashboard.ts:28`. Rust's only `show()` sites are the widget at boot (`window.rs:253`), the close signpost path (`:167`), tray restore (`tray.rs:273`) and `snapshot_runtime.rs:278`. The annotation's named guard site has no hook; a guard would need a different Rust mechanism or a frontend pre-show check. The `[inferred]` tag is dropped and the correction stands.

5. **VERIFIED** — `verification-matrix.json` carries 22 capabilities: 20 `verified`, 2 `planned`. Every capability this chunk's CARRYs touch (P-061, P-062, P-063, P-064, P-065, P-066, P-082) is already `verified`; the two unclaimed (P-075 `dynamic-external`, P-077 `by-construction`) are owned by other route entries. The expected matrix outcome is *link nothing*.

**Additional facts measured (not previously in scope):**
- The contextmenu suppression CARRY 4 would fall back to **ships**: `use-suppress-browser-chrome.ts:27,38` installs a document-level `contextmenu` listener, so the `defaultPrevented` fallback is a real, observable assertion rather than a hypothetical.
- `pulse-app/capabilities/*.json` is on test-plan §3's **boot-smoke trigger path list**; the RED arm touches it transiently, and the headful leg boots the real binary regardless, so a smoke gate belongs in Test Commands either way.
- test-plan §3 codifies the **Direct-binary smoke variant for MEASURE-FIRST chunks**: two legs, a separate fresh `ANDROMEDA_PULSE_DATA_DIR` each, with the 0-ERROR clean-log assertion binding the GREEN leg only.
