# Report — 2026-08-23-headful-leg-extension

**Chunk:** Headful leg extension — the window-mechanics affordances a live drive can prove but the assembled path never touches
**Date:** 2026-08-24
**Commits:** none since last_wrap (this wrap's commit is the chunk's first)

## Changes (structured — detectors read this)

- **Files:**
  - `xtask/src/webview_drive.rs` (modified — the leg's assertion side)
  - `pulse-app/ui/tests-e2e/webview-drive.mjs` (modified — the driver side)
  - `pulse-app/src/window.rs` (modified — one obs field + two stale comments)
  - `pulse-app/src/observability.rs` (modified — one allowlist leaf + one dead-list line)
  - `pulse-app/tests/unit_observability_allowlist_close_signpost.rs` (NEW — live allowlist guard, 3 tests)
  - `pulse-app/ui/src/bindings/index.ts` (transient regen artifact, regenerated-with-mcp last; staged copy carries `"mcp":`)

- **Symbols / APIs:**
  - `STAGES` grew 7 → 13 entries: new ids `traces-scroll` · `connection-status` · `findings-window` · `report-window` · `dashboard-toggle` · `dashboard-close` (each automatically valid for `--expect-absent`; no CLI change — `xtask/src/main.rs` untouched).
  - New xtask predicates/readers: `is_dashboard_hidden_transition` · `is_close_signpost` · `stage_bool` · `dom_traces_scroll_ok` · `dom_connection_status_ok` · `dom_findings_window_ok` · `dom_report_window_ok` · `dom_dashboard_toggle_ok` · `dom_dashboard_close_ok`; constants `MAIN_LABEL` · `SIGNPOST_TARGET`. All crate-internal to `xtask` (graph-confirmed: zero external callers of the seam).
  - `stage_halves` **widget-close arm STRENGTHENED**: obs half now requires the transition AND the signpost record with `window_label == "compact-widget"` — bare record presence no longer satisfies (existing test fixture extended; new discriminating tests added).
  - `tray.signpost.shown` emission (`pulse-app/src/window.rs::maybe_show_close_signpost`) now carries ONE bounded field `window_label` (via the existing `sanitize_window_label`, 4-value set + `unknown`). Previously fieldless. Same target, same level, same call site — no new emission.
  - Driver-side helpers (harness-internal, node): `switchToLabel` · `windowInvoke` (via `__TAURI_INTERNALS__.invoke('plugin:window|…', {label})` — the SAME transport + grants the app's own `use-toggle-dashboard` uses cross-window; no new capability) · `recoverBlankWindows` · `logRecords`/`logCount` (refactor of `logMentions`) · `asPoint`/`asSize` · `isWindowVisible`.
  - **No new TauRPC procedure, no capability JSON change** (byte-identical after the revoke mutation was restored — hash-verified `29defd5ca1cccf0bf6e68c47ed11f096`), no new env var, no new port. `EXPECTED_PROCEDURES` untouched; capability-drift clean.

- **Crates / modules:** no crate added/removed. `xtask` + `pulse-app` (2 source files) + the ui `tests-e2e` driver + 1 new `pulse-app/tests/` file.

- **Dependencies:** none added, none bumped (Rust and npm both zero-delta; `webdriverio@^9.31.2` already present carried the new `Key` usage).

- **Schema / config:** none. No migration, no config key, no Settings field.

- **Spec-master edits:** none in-chunk (implement authors none; P2 owns them — expected list below).

- **Counts / qualifiers moved:**
  - Assembled-path stage count **7 → 13** — stated in arch §Stack → GUI verification harness (dev-only) Role cell, test-plan §6 (drivers row + P5 scenario), `.claude/rules/testing.md` §Framework E2E row ("7-stage"), `.claude/rules/verification-harness.md` 2026-08-23 entry ("SEVEN stages").
  - xtask colocated test count grew ~27 → 88 (no doc states the number — verified, no site to move).

- **Dev-tool versions:** none installed/upgraded. (Host msedgedriver 151.0.4129.101 confirmed matching the WebView2 runtime 151.0.4129.101 — located at `D:/dev/tools/edgedriver/msedgedriver.exe`, consumed via the registered env var; not a project artifact.)

- **Reverted / negative API facts:**
  - Mutation-check artifacts all reverted byte-identical: the capability revoke (`allow-show` + `allow-close`), the `TraceTable` scroll-style mutations (×2 forms), the widget-mounted `ConnectionStatusLine`. None shipped.
  - The launch stage's `mainNavigated` probe was first written as "label readable ⇒ navigated" and REPLACED mid-chunk: the Tauri label is init-script-injected into `about:blank` documents too, so label-readability is NOT a navigation signal (measured in the leg's own handle dump). The shipped probe reads `getUrl() !== 'about:blank'`.

- **Spec claims disproved by measurement:**
  1. **layout-templates §Component — Notifications trigger #4** states the close-to-tray signpost "fires ONCE per session". Measured false at HEAD: `should_show_close_signpost` (`pulse-app/src/window.rs:135`) gates on `notifications_enabled` alone — no once-per-process latch; the toast fires on EVERY widget close (the P-066-era correction; the in-file comments carried the same stale wording and were fixed in-chunk). Needs the layouts body amended.
  2. **test-plan §6's leg description** (and the §1 flake note): the `launch` flake's characterization is superseded by this chunk's root-cause: **each boot under the automation environment loses ~one of the four webviews' initial navigation** (victim ~random — measured victims across 7 boots: findings ×2, report ×2, main ×2, none ×1), and nothing re-navigates a victim (`show()` ≠ navigate — why toggle recovery measured 0/10). Remediated in the DRIVER by loud, recorded re-navigation of blank victims at launch (`recovered_from_blank` in the stage record). **Boundary honesty (operator directive at wrap):** the race is *measured under the automation environment; production exposure is unmeasured; no confining mechanism has been identified* — all four windows are declared in `tauri.conf.json` and every production boot creates and navigates them too, the remediation lives ONLY in the harness driver, and production absence rests on indirect evidence (no complaints; boot smokes read the obs log, not per-webview URLs — a blank hidden `findings`/`report` window would be invisible to every smoke shipped today). The open half — a plain-boot per-webview-URL measurement, or a product-side re-navigate-on-show guard — needs an owner (route-resolve, operator's call).
  3. **plan.md Test Commands' RED arm** listed `--expect-absent dashboard-toggle` WITHOUT `--no-inject` — measured wrong-polarity (a healthy app shows the stage, so the arm exits FAILURE by design); the firing form is `--no-inject --expect-absent {telemetry-dependent stage}`; ran as `--no-inject --expect-absent findings-window`. (A plan defect, not a spec-master defect — recorded for the P2 intent-consistency check.)

- **Coverage of new surfaces** (per new stage; all are HARNESS assertions over existing product surfaces — no new product UI element, no new external surface, no new hot-path op):
  - `traces-scroll` → validation n/a · instrumentation n/a (DOM-only; no obs half by design) · PII n/a · tests {xtask unit fixtures both-ways + live GREEN + `--no-inject` RED; source-mutation partial — see Deviations} · a11y n/a · tokens n/a
  - `connection-status` → validation n/a · instrumentation n/a (DOM-only) · PII n/a · tests {unit both-ways + live GREEN + widget-mount mutation RED + `--no-inject` RED} · a11y n/a · tokens n/a
  - `findings-window` → validation n/a · instrumentation n/a (JS `hide()/show()` end-to-end — no Rust record exists; DOM-only by construction) · PII {geometry recorded as derived booleans/deltas only — never raw coordinates} · tests {unit both-ways + live GREEN + capability-revoke RED (press landed, show dropped) + `--no-inject` RED} · a11y n/a · tokens n/a
  - `report-window` → same posture as findings-window; Esc chain asserts a11y-plan §5 focus restoration (SC 2.1.2) live; capability-revoke RED proven
  - `dashboard-toggle` → instrumentation n/a (toggle hides via JS `hide()` — no record either direction, measured; DOM-only via the window API's own `is_visible`) · tests {unit both-ways + live GREEN (all 4 flips) + capability-revoke RED (4 presses landed, show dropped, 4 flips red)} · rest n/a
  - `dashboard-close` → instrumentation ✓ (obs half = the `main → hidden` transition record, UNIQUE to the ✕ path since the toggle writes no record; silence-discriminator = `signpost_delta == 0`) · tests {unit both-ways incl. both-halves-required + live GREEN + capability-revoke RED} · rest n/a
  - `tray.signpost.shown.window_label` (the one product-source delta) → validation n/a (bounded by `sanitize_window_label`) · instrumentation ✓ (exact allowlist leaf; no bare `tray` key introduced) · PII ✓ (label-only; the banned content/coordinate fields asserted absent by the new guard) · tests {`unit_observability_allowlist_close_signpost.rs` 3 tests, mutation-checked RED (leaf removed → 3/3 fail) and GREEN; live wire-proof in the leg's widget-close stage (`signpost_seen:true` matching the label field)} · a11y n/a · tokens n/a

## Deviations from intent

1. **RED-arm polarity** (plan Test Commands defect): ran `--no-inject --expect-absent findings-window` instead of the listed `--expect-absent dashboard-toggle` — the listed form fails against a healthy app by design. Justification: the predecessor chunk's documented RED form pairs `--no-inject` with a telemetry-dependent stage; using a NEW stage also proves the new stage discriminates.
2. **Toggle stage is FLIP-based, not scripted hide→show directions** (plan Step 4.5). Run 2 measured the scripted directions drifting off the app's actual state (the dashboard self-mounts HIDDEN and is never shown unless launch presses the toggle — so the stage entered with `main` hidden, and every fixed-direction poll asserted the wrong direction). Flip semantics — each press must INVERT the visibility it found — assert the same P-066 contract (open-if-hidden/hide-if-shown, both routes, widget stays) robustly. The plan's "assert `ui.layout.transition` in both directions" half was premise-false anyway: the toggle path writes NO record (JS `hide()`/`show()`), measured; which is also what makes the `main → hidden` record unique to `dashboard-close` — its obs half rests on exactly that uniqueness.
3. **Blank-window recovery added to the launch stage** (not a planned step; it IS plan Step 1's "fix" outcome): the root-cause remediation for the flake — re-navigate `about:blank` victims at launch, before anything is asserted (the traces no-reload invariant starts at `traces-empty`), loudly recorded per-run. No wait, no retry (both plan-rejected): a mechanically different remediation aimed at the root cause (`show()` ≠ navigate).
4. **`traces-scroll` source-side mutation: reported partial.** Two honest mutation mechanisms (`overflowY: visible`; `minHeight: 0` removal — the canonical pre-P-082 defect cause) BOTH failed to redden the stage because the shipped containment is LAYERED (route column clamp + parent flex + region overflow) and each single-element mutation was absorbed by the next layer. Not escalated to a shell-level mutation (cost/risk beyond proportion). The predicate's discrimination stands proven by unit fixtures both ways (`a_scrolling_page_fails_traces_scroll`) + the `--no-inject` live RED. The layered-containment finding is itself recorded.
5. **Two vacuity traps found live and closed** (run-1 findings, not plan steps): the hidden-report dialog probe (a hidden `report` webview renders the fallback Report, so an ungated DOM probe is vacuously green — now gated on visibility) and click-focus masquerading as restored-focus (the badge is already focused by its own click — now gated on the dismissal actually happening).
6. **Plan's new-file location honored with one addition**: `unit_close_signpost.rs` needed NO change (its comments already said every-time) — a no-op reconciliation, as was `xtask/src/main.rs`.

## Decisions & corrections

- **Operator directive (wrap invocation): the race's boundary wording + ownership.** The lost-navigation race must be worded as *measured under the automation environment; production exposure unmeasured; no confining mechanism identified* — NOT "automation-only" (no confining mechanism was identified; all four windows exist in every production boot; the remediation is harness-side only; production absence is indirect evidence — smokes read no per-webview URLs). The open half (plain-boot per-webview URL measurement, or a product-side re-navigate-on-show guard in `window.rs`) needs an owner via route-resolve; owner and form are the operator's trajectory call; the deferred-CARRYs follow-up entry is a natural home.
- **Operator breadth ruling (P4, phase):** mechanic-free six + flake; defer #1 drag / #2 resize / #3 native-menu / #5's multi-size half behind a recorded mechanic probe; two consequent sub-part deferrals (every-time-toast repetition proof; P-061 boot geometry) recorded in plan §Constraints.
- **Terminal-rendering trap (near-miss, retracted before any code changed):** `JSON.stringify` of wdio's `Key.Shift`/`Key.Escape` printed as empty strings because private-use-area codepoints render invisibly — briefly read as broken key constants; a codepoint-level probe (`e008`/`e00c`) disproved it. Probe PUA-valued constants by codepoint, never by visual print.
- **Label injection ≠ navigation:** the Tauri window label is readable on `about:blank` (init-script injection) — a label read can NEVER serve as a navigation signal; only `getUrl()` leaving `about:blank` can.
- **The dashboard is hidden for most of the assembled path** (self-mount = the hidden webview's DOM mounts; nothing shows the window until the toggle stage). DOM/layout assertions against hidden WebView2 windows are real (layout is computed); visibility assertions must come from the window API, not DOM presence.
- **Hidden-window DOM is vacuity fuel:** a hidden window can render content (the report fallback) — any DOM probe on a maybe-hidden window must be gated on measured visibility first.

## Outcome

**Acceptance criteria: met** (with deviation 4's reported partial on one mutation arm). Gates all green:

- `cargo fmt --check` ✓ · `cargo clippy --workspace --all-targets --all-features -- -D warnings` ✓ · `cargo nextest run --workspace --profile ci` **1925/1925 + 1 skip** ✓ — **PREREQ A (Rust gate deferral since 2026-08-23-a11y-verification) CLOSED**
- `cargo xtask capability-drift` **clean** (after the regen-last discipline; staged bindings carry `"mcp":`) · `cargo xtask capability-widening-check` clean (0/3) ✓
- `npm run lint` ✓ · `npm run typecheck` ✓ · `npm test` **809/809** ✓ (`pulse-app/ui` touched → webview gates ran)
- `cargo deny check bans licenses sources` **ok**; `cargo deny check advisories` observed designed-red with the owned upgradeable set re-enumerated first-hand: **8 IDs** (0189/0190/0194/0195/0204/0222/0253/0258 — unchanged). **PREREQ B: point 41 — `probe skipped per ratified interval (next: 43)`**; basis re-verification rides the interval discipline (the overlap ran; the probe itself is owed at 43).
- `cargo xtask self-verify` **PASS** (a11y/contrast 7/7 surfaces, 0 new violations vs baseline)
- `cargo xtask webview-drive` GREEN: **PASS — all 13 stages**, clean log (0 ERROR / 0 `app.panic.fatal`) — twice on pristine source (runs 3 and 7). RED arm `--no-inject --expect-absent findings-window`: **PASS (discriminates)**.
- **Mutation checks:** allowlist leaf (RED 3/3-fail → GREEN 3/3-pass) · capability revoke `allow-show`+`allow-close` (5 stages red with every control still found and pressed — the dead-affordance mode reproduced live; restored byte-identical, hash-verified) · widget-mounted status line (stage red, dashboard half green; reverted) · traces-scroll source mutation (reported partial per deviation 4).
- **Smoke:** UI-surface + boot-path condition satisfied in P2 — self-verify PASS + the leg booted the real binary 7 times over real OTLP with the fresh embed (bundle hash matched `ui/dist` each cycle); the new obs field live-verified at the wire (`signpost_seen:true` on the label field). Zero orphan processes; ports released (TIME_WAIT residue only).
- **Flake tally (per-condition, per the attribution discipline):** 7 boots — victims: findings, report, main, none, report, findings, main (6/7 boots struck, one window each); recovery succeeded 6/6; both pristine GREEN runs passed end-to-end. Boundary per the operator directive above.

**Expected amendments (wrap P2)** — from plan §Implementation notes + this report:
- `architecture.md` §Stack → GUI verification harness (dev-only) Role cell — stage set 7 → 13 (same cell amended by both predecessor chunks).
- `obs-plan.md` §8 — the new exact `tray.signpost.shown` leaf (fields: `window_label`).
- `test-plan.md` §6 — selector-strategy migration DISCHARGED for the Traces anchors (region label + native `table` + `.trace-row`; `trace-table-empty` testid retained — no accessible anchor ships there); the leg description 7 → 13 stages; the `launch`-flake status → root-caused + driver-remediated, worded per the operator's boundary directive (measured under the automation environment; production exposure unmeasured; no confining mechanism identified).
- `layout-templates.md` §Component — Notifications trigger #4 — "fires ONCE per session" → every-time (gated on `notifications_enabled` alone; matches the shipped `should_show_close_signpost` + P-063/P-066 correction).
