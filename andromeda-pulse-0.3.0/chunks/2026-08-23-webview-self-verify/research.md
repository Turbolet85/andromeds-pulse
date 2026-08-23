# Codebase Research — 2026-08-23-webview-self-verify

## Scope
- **Depth:** moderate-deep · **Reads:** 14 · **Globs/Greps:** 12 · **Graph queries:** 2 (ts plane)

## Files inspected
- `xtask/src/self_verify.rs` (438 lines; structure + `launch_pulse` 180-190, `headless_reason` 279-292, `read_log_lines` 324) — **the extension point.** Already boots the real dev binary under a fresh `ANDROMEDA_PULSE_DATA_DIR` + `ANDROMEDA_PULSE_LOG_LEVEL=debug` with `kill_on_drop(true)`, polls `:4317`/`:4318` for readiness, globs the date-suffixed log family, composes the a11y/contrast harness, and cleans up with a zero-orphan check. It has every piece except the ability to press a control.
- `xtask/src/main.rs` (Cmd enum + dispatch 155-177; `capability_drift` body) — 18 existing subcommands incl. `SelfVerify`. **`capability_drift` reads `bindings.ts`, parses `ARGS_MAP`, and diffs against `EXPECTED_PROCEDURES` — it never reads capability GRANTS.**
- `pulse-app/src/window.rs` (110-200) — the close path in full.
- `pulse-app/src/observability.rs` (927-1005 allowlist leaves; `for_target` 2271-2291) — the leaf set + fallback semantics.
- `pulse-app/src/tray.rs:102` — `tray.visibility.toggle` emit site.
- `pulse-app/ui/src/components/WindowControls.tsx` (full) · `hooks/use-window-controls.ts` · `components/Titlebar.tsx:166` · `dashboard/router.tsx:150` · `widget/CompactWidget.tsx:73` — the control and its mount chain.
- `pulse-app/capabilities/default.json` — grant set + rationale text.
- `pulse-app/tauri.conf.json` — window definitions.
- `pulse-app/ui/playwright.config.ts` + `playwright-a11y.config.ts` + `package.json` — the existing browser harness.
- `.andromeda/test-plan.md` (148, 292, 294, 497) · `.andromeda/a11y-plan.md:228` — the governing clauses.

## Graph impact
- **`WindowControls#close` / `#minimize` / `#maximize`** (`pulse-app/ui/src/hooks/use-window-controls.ts:9-18`) — the hook's returned closures; `close` at `:18` is `getCurrentWindow().close()`.
- **`CONTROL_STYLE`** referenced at `components/WindowControls.tsx:60,71` — the shared button style; confirms three sibling buttons.
- Graph note: the ts plane resolves the hook symbols but not the `WindowControls` *component* export by name — a descriptor-tail artifact of scip-typescript, not evidence of absence. The production render site was confirmed file-first (`Titlebar.tsx:166`), per the cookbook's "zero callers is not dead-code evidence" rule.

## Patterns detected
- **The leg's observable already exists, already carries the window identity, and is already allowlisted** (`pulse-app/src/window.rs:119-129`). `handle_close_to_tray` emits `target: "ui.layout.transition"` with `layout_mode_from` (the sanitized window label), `layout_mode_to = "hidden"`, `tray_visible = hide_ok`, on EVERY close of either window. The allowlist leaf at `observability.rs:975` permits exactly `layout_mode_from` / `layout_mode_to` / `tray_visible` / `always_on_top` / `duration_ms` — so all three emitted fields survive un-redacted. This is the deterministic signal the leg asserts on; no new emission, no new leaf, no `sleep(N)`.
- **Primary/secondary close model** (`window.rs:139-199`): `CloseRequested` → `api.prevent_close()` → geometry flush → if the label is `compact-widget`, also hide `main` and fire the signpost → `handle_close_to_tray`. Closing the WIDGET sends the app to the tray; closing the DASHBOARD collapses to the widget. `layout_mode_from` distinguishes the two in the log, satisfying the layouts extract's "record which surface" requirement for free.
- **`tray.signpost.shown` is field-less** (`window.rs:166`) and has **no allowlist leaf**; `for_target` (`observability.rs:2271`) falls back to `.tick`-strip, then the `.`-prefix, then `::`-prefix — and no bare `"tray"` key exists, so it resolves `None`. Because the event emits zero fields there is nothing to redact: target + message still reach the log. Usable as a presence signal for the widget-close path only — weaker than `ui.layout.transition`, which should be the primary assertion.
- **Existing harness launch discipline** (`self_verify.rs:180-190`) matches test-plan §3's Direct-binary variant exactly — per-run fresh data dir, env override, `kill_on_drop`, port-based readiness.

## Conventions to follow
- **Headless skip is Linux-only** (`self_verify.rs:279-292`): `headless_reason` returns a skip only when `is_linux && DISPLAY.is_none() && WAYLAND_DISPLAY.is_none()`. A Windows host never skips — the new leg runs for real here.
- **Log reads glob the date-suffixed family** (`self_verify.rs:320-324`, comment cites the `rolling::daily` sink). Mandatory, per `verification-harness.md` 2026-06-29.
- **Control selection by accessible name**: the ✕ is `<button type="button" aria-label="Close to tray" class="window-control window-control--close">` with `CONTROL_STYLE` binding `--target-input-min` (the chunk #99 SC 2.5.8 fix). Its siblings are `aria-label="Minimize"` and `aria-label="Maximize"`. Role+name selection satisfies test-plan §11's xpath/hashed-CSS ban.
- **All four windows are `visible: false, decorations: false`** in `tauri.conf.json` (`compact-widget` · `main` · `findings` · `report`) — the driver must resolve/await the right window rather than assume one is showing.

## New files to create
- A driver spec/config under the harness surface the probe selects — either a WebdriverIO conf + spec (shape a) or a Playwright CDP spec under `pulse-app/ui/tests-e2e/` (shape b). **Path is probe-determined; not fixed here.**

## Files to modify
- `xtask/src/self_verify.rs` — add the affordance-press leg (or a sibling `xtask/src/webview_drive.rs` behind a new `Cmd`); this is where launch/readiness/log-read/cleanup already live.
- `xtask/src/main.rs` — a new `Cmd` variant + dispatch arm if the driver gets its own subcommand (the `Cmd` enum and its `match` are the two threading sites; both in this one file).
- `pulse-app/ui/package.json` — devDeps, if the driver lands JS-side (shape a's wdio stack or shape b's spec).
- `pulse-app/ui/playwright.config.ts` — **currently inert**: `testDir: "./tests-e2e"` pointing at a directory that does not exist, with `testIgnore: ["**/*"]`. A reserved-but-disabled E2E surface; shape (b) would activate it.
- `pulse-app/capabilities/default.json` — **transient mutation target only** (revoke → rebuild → observe RED → restore). Must end byte-identical.
- No `crates/**` or `pulse-app/src/**` production change is indicated by research.

## Premise corrections to the extracts (P4 must override the extract text where these conflict)
1. **obs extract — FALSIFIED.** It states "the whitelist enumerates no `tray`, `ui`, `window` or `app.boot.webview.*` entry". All of them exist: `app.boot.webview.init` (`:927`), `app.boot.tray.init` (`:966`), `ui.layout.transition` (`:975`), `tray.visibility.toggle` (`:988`), `tray.menu.interaction` (`:992`), `tray.notification.dismiss` (`:996`), `tray.notification.action` (`:1000`). The chunk therefore needs **no new obs target and no new allowlist leaf** — the obs extract's 4th acceptance bullet is conditional and does not fire.
2. **obs extract — LIKELY FALSE for the RED arm.** It requires the revoked arm to "surface the rejected close as a WARN/ERROR module-boundary event rather than silently". The historical record is the opposite: `verification-harness.md:110` records that with the grant missing the ✕ "did NOTHING" and "1719 nextest + the boot-quit self-verify ALL passed" — the ACL drops the IPC in the webview before any Rust boundary is reached, so no Rust-side error exists to log. **The RED leg's assertion must be the ABSENCE of the `ui.layout.transition` / `layout_mode_to = "hidden"` line, not the presence of an error.** Confirm at implement; do not plan an error assertion.
3. **a11y extract question — RESOLVED, and the spec is stale.** `a11y-plan.md:228` (§1 P5), `.claude/rules/a11y.md:75`, and `.claude/docs/a11y-summary.md:30` all name `button[aria-label="Minimize to tray"]`. That string has **zero occurrences anywhere in the codebase**. The shipped labels are `"Minimize"` and `"Close to tray"`. A three-site spec↔reality divergence — a wrap-time amendment, not a phase edit.
4. **tests extract question — RESOLVED both ways.** (a) `pulse-app/capabilities/*.json` IS in the boot-smoke trigger list verbatim (`test-plan.md:292`), so the mutation check owes the gate; the Direct-binary variant (`:294`) is the accepted form and already carves out the RED leg ("the clean-log assertion binds the GREEN leg only"). (b) `test-plan.md:148` bans "Playwright headful mode" in the same clause as human-in-loop verification and manual screenshot comparison — a literal reading is adverse to shape (b).
5. **No gate guards the capability restore.** `capability_drift` diffs procedures via `bindings.ts` ARGS_MAP vs `EXPECTED_PROCEDURES` and never inspects grants; `capability_widening_check` covers only the three NEVER-widen caps and only in the widening direction. **A revoked `core:window:allow-close` left in the tree would pass both gates** — so the restore needs an explicit byte-level assertion in the plan, not gate reliance.

## Open questions
- **Which driver shape survives contact with this host?** → blocks: implementation-scope. Both are unmeasured; the directive orders one probe each. Research supplies the standing lean, not the answer: shape (a) is the stack **both** plans pin (`test-plan.md:54`, `:395`, amendments:18) and is proven working in the companion repo (`conductor/crates/conductor-tauri/ui/` — `@crabnebula/tauri-driver ^2.0.9` + `webdriverio ^9.29.1` installed, `wdio.conf.ts` present, `a11y` script wired), but needs `tauri-driver` + an msedgedriver matching WebView2 **151.0.4129.101** — both absent from PATH. Shape (b) reuses the installed Playwright stack but has **zero CDP path at HEAD** (no `connectOverCDP` / `remote-debugging` / `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS` anywhere) and runs against `test-plan.md:148`. If (b) wins on measurement it owes a test-plan amendment; (a) does not.
- **Does the posture clause need amending regardless of shape?** → blocks: plan-decision (resolved in P4). `test-plan.md:148` ("no Playwright headful mode") and `:497` ("headful tauri-driver … not agent-driven") both encode the pre-2026-08-22 posture that headful GUI verification is out of agent-driven scope. The operator ruling this entry records supersedes that. The amendment is owed at wrap either way; the plan records it as an Expected amendment rather than editing the spec here.
