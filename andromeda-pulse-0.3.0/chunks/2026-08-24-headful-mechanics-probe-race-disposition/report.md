# Report — 2026-08-24-headful-mechanics-probe-race-disposition

**Chunk:** Headful mechanics probe + navigation-race disposition — the four deferred window-mechanic stages land or are DECLINED on a measured driver probe; the lost-initial-navigation race's production exposure is measured or closed.
**Date:** 2026-08-25
**Commits:** (none yet — this wrap authors the chunk commit)

## Changes (structured — detectors read this)

- **Files:**
  - `xtask/src/webview_drive.rs` (M) — 2 new `STAGES` entries + their `stage_halves` verdict arms; `print_mechanics_probe`; 4 new colocated tests
  - `pulse-app/ui/tests-e2e/webview-drive.mjs` (M) — `probeWindowMechanics`; 2 new stage drivers; `DRAG_REGION` selector + 4 probe-displacement consts
  - `pulse-app/src/window.rs` (M) — `spawn_navigation_check` + `ALL_WINDOW_LABELS` / `BLANK_URL` / `NAVIGATION_SETTLE`
  - `pulse-app/src/observability.rs` (M) — one exact allowlist leaf
  - `pulse-app/src/main.rs` (M) — one boot-boundary registration line
  - `pulse-app/tests/unit_observability_allowlist_window_navigation.rs` (NEW) — 4 allowlist guard tests

- **Symbols / APIs:**
  - NEW `pulse_app::window::spawn_navigation_check<R: tauri::Runtime>(app: tauri::AppHandle<R>)` — sole caller is `pulse-app/src/main.rs` setup closure (one call site; no other callers)
  - NEW `xtask::webview_drive::print_mechanics_probe(dom: &[Value])` — sole caller is `run_webview_drive`
  - NEW obs target `app.boot.window.navigation` (fields `window_label` · `navigated` · `reason`) — INFO on navigated, WARN on blank; once per window per boot
  - NEW stage ids `native-menu-suppressed`, `signpost-repeat` (both `--expect-absent`-eligible; CLI shape unchanged)
  - CHANGED `STAGES` — 13 → 15 entries; `widget-close` is no longer terminal (`signpost-repeat` follows it deliberately)
  - **No TauRPC procedure added/changed** · **no `pulse://stream/*` topic added** · **no port, socket, or env var added** · capability JSON untouched (the drag RED lever was never exercised — its stage declined)

- **Crates / modules:** none added, none removed. Changed: `pulse-app` (bin), `xtask`.

- **Dependencies:** none added, none bumped. (`webdriverio` 9.31.2 / `@crabnebula/tauri-driver` 2.0.9 already present; the probe uses their existing Actions + window-rect APIs.)

- **Schema / config:** no migrations, no config keys, no violation-schema change. One new obs record shape (`app.boot.window.navigation`) with its exact allowlist leaf.

- **Spec-master edits:** none (wrap's amendment flow owns them; see Expected amendments below).

- **Counts / qualifiers moved:**
  - headful leg stages **13 → 15** — stated at arch §Stack (GUI verification harness Role cell), test-plan §6 (Drivers per surface + Scenario P5), `.claude/rules/testing.md` §E2E, `.claude/docs/tests-summary.md`
  - `xtask` colocated tests **88 → 92** — stated at test-plan §1 (msedgedriver-resolver trigger)
  - workspace nextest **1925 → 1933** (+8: 4 xtask + 4 pulse-app allowlist)
  - vitest 809 unchanged; owned advisory ID set **8, unchanged**

- **Dev-tool versions:** none installed or upgraded. (`msedgedriver` 151.0.4129.101 was already on the host and matches the installed WebView2 Runtime 151.0.4129.101 — located, not installed.)

- **Reverted / negative API facts:**
  - `DRIVE_TIMEOUT` raised 420 → 660 → 1200 during a misdiagnosis, then **fully reverted to 420s**. The ceiling is unchanged from HEAD; the leg costs 183s on the RED arm vs 184s pristine.
  - A `clickTab(browser, 'traces')` route change inside the new observer stage was written, measured harmful, and **removed** (see Deviations).

- **Spec claims disproved by measurement:**
  1. **The route CARRY's drag-stage obs-half premise.** It assumed `WindowEvent::Moved` could supply a drag verdict. Measured: `Moved` is guarded to the MAIN window only (the widget is deliberately excluded per a P-061 correction) and emits **no tracing record at all** — it calls `store.record_move_throttled` into `window-geometry.json`. `pulse-app/src/window.rs:204-210`.
  2. **The CARRY-dictated guard site.** "A re-navigate-on-show guard in `pulse-app/src/window.rs`" has no hook: `handle_window_event` handles only `CloseRequested`/`Moved`/`Resized`, and Tauri 2's `WindowEvent` enum has **no Shown/Visible variant** (source-verified against `tauri-2.11.0/src/app.rs`: Resized · Moved · CloseRequested · Destroyed · Focused · ScaleFactorChanged · DragDrop · ThemeChanged · Suspended · Resumed). `findings`/`report`/`main` are shown from the FRONTEND.
  3. **Half B path (a) as worded** ("measure a plain boot's per-webview URLs, no driver attached") had **no mechanism** — nothing shipped reported a webview URL, and attaching a driver is what makes a boot not-plain. Closed by shipping the instrument.
  4. **test-plan §6's navigation-race boundary** ("production exposure UNMEASURED; no confining mechanism identified") is now MEASURED: 0 blank windows / 36 windows / 9 plain boots.
  5. **W3C pointer Actions + `setWindowRect` against a WRY window** — measured UNSUPPORTED on this host: both execute without throwing and produce zero delta.
  6. **P-064/P-065 matrix acceptance** states "Live headful right-click folded into P-076"; the live suppression evidence landed HERE instead (P-076's leg has no such stage).

- **Coverage of new surfaces:**
  - `app.boot.window.navigation` (obs record) → validation n/a · instrumentation ✓ (INFO/WARN once per window per boot, post-settle, off any hot path) · PII **redacted✓** (bounded `sanitize_window_label` set + bounded static `reason`; no URL, title, or coordinates — guard test asserts the bans) · tests **unit✓** (`pulse-app/tests/unit_observability_allowlist_window_navigation.rs`, 4 tests, mutation-checked) · a11y n/a · tokens n/a
  - `native-menu-suppressed` (harness stage) → validation n/a · instrumentation n/a (DOM-only half by construction) · PII n/a · tests **e2e✓** (GREEN + RED arms; verdict arm unit-pinned + mutation-checked) · a11y n/a · tokens n/a
  - `signpost-repeat` (harness stage) → validation n/a · instrumentation ✓ (reads the existing `tray.signpost.shown` leaf; no new emission) · PII n/a · tests **e2e✓** + 3 unit pins, mutation-checked · a11y n/a · tokens n/a
  - **No product UI element added** — the chunk adds observation, not surface.

## Deviations from intent

1. **Two of four stages DECLINED, on measurement.** The plan's decision rule produced this; it is the designed outcome, not a shortfall. Drag and resize decline because W3C pointer Actions and `setWindowRect` both reach the window and move nothing (`drag_delta {dx:0,dy:0}`, `resize_delta {dw:0,dh:0}`, no error field). P-082's multi-window-size half rides the resize mechanic and declines with it.

2. **`pulse-app/src/main.rs` touched, though not in research's Files-to-modify.** One line registering `spawn_navigation_check` at the boot boundary, beside the existing `window::*` calls. Judged in-scope-by-extension per the gray-area rule — the plan directed a boot-time check in `pulse-app`, and the alternative was an uncalled function (dead code).

3. **Half B shipped the instrument and NO guard.** The plan made the guard conditional on the measurement; the measurement (0/36) says the race does not reproduce on plain boots, so shipping a repair would have been unwarranted product code.

4. **The `native-menu-suppressed` stage moved from late in the leg to immediately after `traces-scroll`.** Its first draft sat after `empty-states` and clicked back to `/traces` to make a canvas present; that route change left downstream window state unlike the pristine flow and killed the RED arm's WebDriver session at `dashboard-close`. Attribution was measured, not assumed: pristine 13-stage RED arm **184s PASS**, the same arm with the route change **no completion at 1200s**. Re-homed to where the leg is already on `/traces`, so it observes without perturbing; RED arm now **183s PASS**.

5. **`DRIVE_TIMEOUT` inflated three times, then reverted.** Diagnosed the hang as a slow leg and raised the ceiling 420→660→1200 before the evidence arrived. Reverted to 420s once the real cause was found; the shipped value is unchanged from HEAD.

6. **Boot-geometry sub-part — operator correction at this wrap (item 1).** The plan folded "widget boots at the margin-inset corner" into the drag stage, so it would have declined silently under drag's reason. That reason does not apply: the sub-part needs only a geometry READ, and this chunk's own probe measured `geometry_readable=true`. It is **not** declined here; it is re-homed with its own disposition (see route-resolve).

## Decisions & corrections

- **Operator directive (this wrap, item 1):** a sub-part folded into a declined stage must not inherit that stage's decline reason — give it its own disposition or its own stated reason. Applied to the boot-geometry sub-part.
- **Operator directive (this wrap, item 2):** P-061/P-062 hollow-`verified` gets an operator ruling at this wrap; the probe sharpened it because driver-side affordance evidence can never arrive on this host.
- **Buffered stdout can invert a hang diagnosis.** Two runs stopping at the same visible stage read as "it hangs there"; node block-buffers stdout to a file, so identical prefixes carry no location information. The real signature (`ERROR webdriver: Couldn't resolve command with id 4077` after `dashboard-close`) appeared only once the run went long enough to flush.
- **Attribution before blame:** three failures under my change with zero baseline samples is not attribution. One pristine-baseline run settled it in 184s.
- **A read-only observer stage must not perturb what follows it.** Navigating to make an assertion cheap cost the whole RED arm.
- **A detector that has never fired proves plumbing, not behaviour.** The navigation check was mutation-validated by collapsing its settle window until it reported `blank=1 victims: report`.
- **P-061/P-062 read as hollow-`verified`** (measured at P5 of the planning phase, sharpened here): both are affordance caps whose `ref` defers the affordance-level evidence to P-075 (planned/declined) and P-076 (verified, but its leg contains no drag/resize stage).

## Outcome

**Acceptance criteria: met.** Probe ran first and its result gated stage authoring; each declined stage carries a measured reason; each landed stage reports both halves independently with field-level predicates; every landed pin was mutation-checked; no capability was claimed.

**Gates (final source state):**
- `cargo fmt --check` clean · `cargo clippy --workspace --all-targets --all-features -- -D warnings` clean
- `cargo nextest run --workspace --profile ci` → **1933/1933 + 1 skip**, exit 0
- `cargo xtask capability-drift` clean · `cargo xtask capability-widening-check` clean (0/3)
- `npm run lint|typecheck|test --prefix pulse-app/ui` → **vitest 809/809**
- `cargo deny check bans licenses sources` ok · `cargo deny check advisories` designed-red at the **same 8 owned IDs** (0189/0190/0194/0195/0204/0222/0253/0258), re-enumerated first-hand
- `cargo xtask self-verify` **PASS** (incl. a11y/contrast; pa11y 7/7, 0 new violation tuples)
- `cargo xtask webview-drive` **GREEN — 15/15 stages, clean log (0 ERROR, 0 app.panic.fatal)**
- `cargo xtask webview-drive --no-inject --expect-absent traces-populate` **PASS (RED arm), 183s**

**Mutation checks (4, each reddened exactly the intended pins):** repeat-count `>=2`→`>=1` (2 pins red, press-half + native-menu green) · native-menu field-read → stage flag (1 pin red) · allowlist leaf key renamed (3 of 4 red; the fallback-leak pin correctly green) · `NAVIGATION_SETTLE` 5s→0 (detector fires, `blank=1 victims: report`).

**Smoke:** direct-binary variant (test-plan §3), 9 plain boots + 2 mutation boots, each with a fresh `ANDROMEDA_PULSE_DATA_DIR` and shutdown by the specific pid from the app's own pid file. `self-verify` ran as a P2 gate.

**PREREQ (`cargo audit`, pin #14):** this wrap is interval point **42** — owes no probe (point 40 discharged, next **43**). Basis unchanged (RustSec DB unloadable, upstream); overlap re-derived first-hand at the same 8 owned IDs. Record: `probe skipped per ratified interval (next: 43)`.

**Re-observed, pre-existing (not this chunk's):** `xtask::self_verify::launch_pulse` sets no CWD, so a stray root-level `ui/src/bindings/index.ts` is written on every self-verify run (gitignored; already CARRY'd on the Diagnostics un-muting entry).

**Expected amendments (wrap):**
- `.andromeda/test-plan.md` §6 — stage count/list 13 → 15 at every stating site; the two declined stages with their measured reasons; the navigation-race boundary corrected from UNMEASURED to the measured result (0/36 across 9 plain boots) with the shipped detector named as the standing mechanism.
- `.andromeda/architecture.md` §Stack → GUI verification harness Role cell — stage count/list 13 → 15.
- `.andromeda/obs-plan.md` §6 warn row + §8 whitelist — the `app.boot.window.navigation` leaf (dual-site rule).
- `.andromeda/test-plan.md` §1 — `xtask` colocated test count 88 → 92.
