# Codebase Research — 2026-08-23-headful-leg-extension

## Scope
- **Depth:** moderate · **Reads:** 9 · **Globs/Greps:** 14 · **Code-graph queries:** 1 (rust plane)

## Files inspected
- `xtask/src/webview_drive.rs` (`44-90`, `256-320`) — the `Stage` struct, the 7-entry `STAGES` table, and `stage_halves`, the per-stage predicate pair every new stage must extend.
- `xtask/src/main.rs` (`105-125`, `175-190`) — the `Cmd::WebviewDrive { expect_absent, no_inject }` clap arm and its dispatch; the CLI seam for any new flag.
- `pulse-app/ui/tests-e2e/webview-drive.mjs` (`28-80`, `210-420`) — selector constants, the window-handle helpers, and the `launch` stage whose flake this chunk owns.
- `pulse-app/capabilities/default.json` (full) — the granted `core:window:*` permission set; the RED-arm lever inventory.
- `pulse-app/src/window.rs` (`85-200`, `280-320`, `340-400`) — the close/geometry/aspect seam and every tracing emission on it.
- `pulse-app/src/observability.rs` (`963-1005`, `2271-2291`) — the allowlist leaves for `ui.*` / `tray.*` / `app.boot.*` and the `for_target` fallback chain.
- `pulse-app/ui/package.json` (full) — the devDependency surface available to the driver.
- `pulse-app/ui/src/dashboard/routes/traces/TraceTable.tsx` (`170-180`, `320-330`) — the scroll region and the row's class/testid pair.
- `pulse-app/ui/src/dashboard/routes/traces/ConstellationCanvas.tsx` (`~250`) — the landmark region and its accessible name.

## Graph impact (rust plane, `tree-query-2026-08-23-headful-leg-extension.json`)
- **`stage_halves` / `stage_observed` / `STAGES`** — every caller is inside `xtask/src/webview_drive.rs` itself, overwhelmingly its own `#[cfg(test)] mod tests` (hits at `:709`, `:715`, `:776`, `:783`, `:790`, `:797`, …). **Blast radius is one file.** Adding stages changes no other crate; the cost lands entirely in that module's colocated test suite, which is the documented place to extend it.
- Consequence for the plan: no cross-crate threading, no re-export, no manifest change. The caller-threading concern that usually widens a modify-list does not apply here.

## Patterns detected
- **Two-sided stage verdict with an inapplicable half** (`xtask/src/webview_drive.rs:264-268`): `StageHalves::satisfied` is `obs.unwrap_or(true) && dom.unwrap_or(true)`, so `None` = not applicable. `launch` and `traces-empty` already ship **DOM-only** (`obs: None`) — a DOM-only new stage is precedented, not a weakening.
- **The DOM half can read a named `detail` FIELD, not just `observed`** (`dom_empty_states_have_hints`, `:317`): the empty-states stage discriminates on the hint's presence inside the stage record because "a message appeared" would pass on a query failure. This is the shape every new stage's predicate should take — a field, per test-plan §6.
- **Window identity is readable from the driver** (`webview-drive.mjs:250-262`): `dumpHandles` already reads each webview's Tauri window label via `globalThis.__TAURI_INTERNALS__.metadata.currentWindow.label` and its URL. Multi-window stages have a working identity mechanism already.
- **Aspect clamp is settle-delayed, not per-event** (`pulse-app/src/window.rs:~47`, `:226`): the `Resized` handler waits for the resize to settle before calling `clamp_to_aspect_bounds`. That settle point is the natural single fold site if a gesture signal is ever emitted, satisfying obs-plan §11's hot-path ban by construction.

## Conventions to follow
- **New xtask predicates get colocated `#[cfg(test)] mod tests`** — the module already carries ~27 such tests around the stage predicates and the `dom_*` readers (`xtask/src/webview_drive.rs:533-800`).
- **Selector constants live at the top of the driver** (`webview-drive.mjs:36-50`) — the re-point is an edit to that block plus `countRows`, not a scattered change.
- **A stage id is the `--expect-absent` vocabulary** (`xtask/src/webview_drive.rs:44`, validated at `:90-95` against `stage_ids()`), so every new stage automatically gains a code-driven RED arm name.

## Decisive findings

### 1. Every `core:window:*` grant the CARRYs need is ALREADY present — so each RED arm has a real lever
`pulse-app/capabilities/default.json` grants `allow-start-dragging`, `allow-minimize`, `allow-toggle-maximize`, `allow-close`, `allow-show`, `allow-set-focus`, `allow-hide`, `allow-set-position`, `allow-set-size`. The arch / security / tests extracts each independently raised this as research's question; the answer is that **no capability JSON edit is required for the GREEN path**, and the revoke-one-grant RED arm is available per stage (drag → `allow-start-dragging`; toggle → `allow-show`/`allow-set-focus`; findings/report docking → `allow-set-position`/`allow-set-size`).

**Important asymmetry:** a **user-driven OS resize** (CARRY #2) is NOT capability-gated — it is an OS window-manager operation whose `Resized` event the Rust side clamps. Its RED arm cannot be a capability revoke; it must neutralise `clamp_to_aspect_bounds` or its call site.

### 2. Two CARRYs have NO obs producer at HEAD — this is the chunk's main cost driver
`pulse-app/src/window.rs` emits exactly: `app.boot.{webview.init,gpu.check,tray.init}`, `ui.layout.transition` (×2, `:122` + `:433`), `tray.signpost.shown` (`:165`), and `app.boot.window.show` (warn/error paths only). **There is no emission on the `Moved` path and none on the successful `Resized`/clamp path** — the clamp only `warn!`s when it *fails* (`:229`).

So CARRY #1 (drag → position delta) and CARRY #2 (resize clamp + aspect band) have no obs half available. Three routes, in ascending cost:
- **(a) DOM-only stage** reading geometry through the Tauri getter API from the driver (`browser.execute` → `getCurrentWindow().outerPosition()/outerSize()`). These getters live in `core:window:default`, bundled in the already-granted `core:default`, so they need no new permission. Record a **derived** field (`moved: true`, `delta_px`, `aspect_in_band`) rather than raw coordinates.
- **(b) New obs target + exact allowlist leaf** — costs a leaf enumerating every emitted field, a guard under `pulse-app/tests/`, and it must fold once per gesture (obs-plan §11 bars per-event `info`). Security additionally bars logging coordinate values, so the emitted field must be derived, not x/y.
- **(c) Read `window-geometry.json`** — **only viable for the dashboard.** `pulse-app/src/window.rs:~198` records that only the dashboard's position is remembered; the compact widget always boots to its fixed corner, so a widget drag persists nothing.

### 3. `tray.signpost.shown` is emitted with NO fields and has NO allowlist leaf
The emission (`window.rs:165-168`) carries only a message. `for_target` (`observability.rs:2271-2291`) tries exact → `.tick`-strip → first `.`-prefix → `::`-prefix; there is **no bare `tray` key**, so it resolves to `None`. Harmless today (nothing to redact), but it means CARRY #4's toast half can assert only record *presence* — which is exactly what test-plan §6's effect-FIELD rule disallows. Making the toast gradeable needs a field on that emission plus its leaf.

### 4. The `launch` flake's mechanism is visible in the driver's control flow
`webview-drive.mjs:345-364`: poll ≤20s for a dashboard **self-mount** (identified by `TAB_NAV` existing); only if absent, press the toggle once, then poll ≤30s again. This reproduces the measured perfect bimodality exactly — `observed=true` ⟺ the self-mount won (`toggle_pressed=false`); `observed=false` ⟺ the toggle path was taken and still failed.

Why the toggle never recovers it (0/10): the toggle calls `show()`+`setFocus()` on `main`, but the failure mode recorded is that **`main`'s webview never leaves `about:blank`** — showing a window whose SPA never mounted cannot make `TAB_NAV` appear. So the press fires correctly and is simply not the missing ingredient; the wait length is not the variable either. `dumpHandles` already captures each handle's `label` + `url`, so a failing run's console output distinguishes "stuck at about:blank" from "wrong window" — the diagnostic substrate exists.

### 5. No resize / pointer-action mechanic exists in the driver today
`grep` for `setWindowRect|getWindowRect|performActions|setWindowSize` in `webview-drive.mjs` returns nothing. `webdriverio@^9.31.2` is present and supports both W3C Actions and window-rect commands, but **whether msedgedriver honors them against a WRY window is unmeasured** — the same class of unprobed premise as CARRY #3's native-menu question.

### 6. The base `playwright.config.ts` is genuinely inert; the a11y one is not
Both `playwright.config.ts` and `playwright-a11y.config.ts` exist. Only the a11y config is referenced (`package.json` `test:a11y`). Nothing in `package.json`, `.github/workflows/`, or `xtask/src/` references the base config — the scope's boundary is accurate as written.

## Files to modify
- `xtask/src/webview_drive.rs` — `STAGES` entries + `stage_halves` arms + per-stage `detail`-field readers + colocated tests. (Graph-confirmed: no external callers.)
- `pulse-app/ui/tests-e2e/webview-drive.mjs` — selector constants (the re-point), new driver steps, new stage records.
- `xtask/src/main.rs` — only if the plan adds a CLI flag; the existing `--expect-absent` already covers every new stage id for free.
- `pulse-app/src/window.rs` — ONLY if the plan chooses route (b) for CARRY #1/#2 or adds a field to the signpost; plus the optional stale-comment cleanup at `:148` / `:187`.
- `pulse-app/src/observability.rs` + a guard under `pulse-app/tests/` — ONLY if a new obs target lands.

## Open questions
1. **Does msedgedriver honor W3C Actions / `setWindowRect` against a WRY window?** → blocks: **plan-decision** for CARRY #1, #2 and #5's multi-size half. If it does not, those stages need a different mechanic (e.g. driving resize through the Tauri API from inside the webview) or must be declined with a recorded reason. This must be probed before the plan commits, exactly as the entry already demands for CARRY #3.
2. **Is a NATIVE WebView2 context menu observable to WebDriver at all?** (carried verbatim from the working entry, still unprobed) → blocks: **plan-decision** for CARRY #3. The honest fallback is asserting the suppression side (`defaultPrevented` on the dispatched `contextmenu`) rather than the menu's absence.
3. **Breadth: all seven stages in one chunk, or a scoped subset?** → blocks: **plan-decision**. Finding 2 materially raises the cost of CARRY #1/#2 above the others (they need a mechanic probe *and* a verdict-route decision), while CARRY #7 and #9 cost almost nothing. This is the operator fork the scope flagged, now with measured cost asymmetry behind it.
