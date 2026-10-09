# Scope — 2026-08-24-headful-mechanics-probe-race-disposition

**Chunk (working-route entry, title clause verbatim):**
> Headful mechanics probe + navigation-race disposition — the deferred window-mechanic stages land or are
> declined on a measured driver probe, and the race's production exposure is measured or guarded

Version: `andromeda-pulse-0.3.0` · Epoch 4 — Polish & ship: verification · promoted 2026-08-24.
Predecessor: `2026-08-23-headful-leg-extension` (13-stage leg; the breadth split that deferred these mechanics).

---

## 1. What this chunk does

Two independent halves, joined because both are residue of the same predecessor chunk and both are gated on a
measurement rather than an assumption:

**Half A — the mechanics probe, then stages.** Four window-mechanic stages (drag · resize · native-menu ·
every-time-toast repetition) were deferred out of the 13-stage headful leg because the driver capability they
need has never been exercised on this host. This chunk **measures that capability FIRST**, then lands each
stage that the measurement supports and **declines, with a recorded reason, each stage it does not** — never a
vacuous stage that asserts something it cannot observe.

**Half B — the navigation-race disposition.** The per-boot lost-initial-navigation race is remediated in the
harness driver only; its production exposure is unmeasured and no confining mechanism has been identified.
This chunk closes that open half by **either** measuring a plain (no-driver) boot's per-webview URLs **or**
shipping the product-side guard — and records which, with the boundary language at test-plan §6 updated to
match what was actually established.

The gating discipline is the point of the chunk: the probe's result is an INPUT to what ships, so a
"declined stage with a recorded reason" is a first-class success outcome here, not a shortfall.

---

## 2. Half A — the probe (CARRY 1)

**Folded annotation (verbatim):** _"the probe comes FIRST — W3C pointer Actions + `setWindowRect` against a
WRY window have never been measured on this host; stages land or are declined on that measurement, never
assumed (2026-08-23-headful-leg-extension breadth split)"_

Measure, against the live Tauri window through the existing `cargo xtask webview-drive` transport
(tauri-driver → msedgedriver → WRY/WebView2):

- **W3C pointer Actions** — whether a synthesized pointer down → move → up sequence reaches the window at all,
  and whether it produces a real OS-level window move when performed inside the `data-tauri-drag-region`.
- **`setWindowRect` / window-rect manipulation** — whether the driver can resize the window, which is the only
  mechanism by which a driver-side resize stage could exercise the clamp.

**Verified at HEAD (2026-08-24):** the driver uses **zero** `action(` / `performActions` / `setWindowRect`
calls — its entire interaction surface today is `.click()`, `.keys()` and `.execute()`
(`pulse-app/ui/tests-e2e/webview-drive.mjs`). The annotation's "never been measured" is therefore accurate as
written. The APIs themselves exist in the installed `webdriverio ^9.31.2` (with `@crabnebula/tauri-driver
^2.0.9`, `pulse-app/ui/package.json`); what is unmeasured is whether they REACH a WRY window through
msedgedriver. **[P3-VERIFIED]** No prior in-repo measurement exists; the distinction between "the API exists
in wdio" and "the API reaches this window" is confirmed as the real open question the probe must settle.

**Probe outcome drives the rest of Half A.** Each stage below lands only if the probe supports its mechanism.

---

## 3. Half A — the deferred stages (CARRYs 2–5)

Each stage, if landed, follows the leg's established shape: a real press/gesture, a **field-level two-half
verdict** (`StageHalves{obs, dom}`), `--expect-absent`-eligible, and a RED mutation arm that proves the pin
discriminates. Each is independently declinable on the probe.

### 3.1 Drag stage — P-061 residual (CARRY 2)
**Folded:** _"window-position-delta > 0 via a real titlebar drag inside the drag region; pairs with the P-061
boot-geometry sub-part (widget boots at the margin-inset corner) deferred with it; RED lever = revoke
`core:window:allow-start-dragging`"_

- Assert a real titlebar drag inside the drag region yields **window-position delta > 0**.
- **Boot-geometry sub-part:** the widget boots at the margin-inset corner. Verified at HEAD:
  `WIDGET_EDGE_MARGIN = 24` (`pulse-app/src/window.rs:317`), `compute_snap_position` insets by that margin
  (`:320-347`), applied via `set_position` (`:411`, `:459`), unit-tested
  (`compute_snap_position_top_left_insets_by_margin_from_origin`, `:557`).
- **[premise-corrected: `WindowEvent::Moved` (`window.rs:204`) is guarded to the MAIN window ONLY and emits NO
  tracing record — it calls `store.record_move_throttled` into `window-geometry.json`. The compact widget is
  deliberately excluded ("the compact widget always boots to its fixed corner — P-061 correction"). A drag on
  the WIDGET produces no handled event at all; a drag on the DASHBOARD produces a file write and no record.]**
  Consequence: the drag stage has **no obs half at HEAD** and is DOM-half-only unless this chunk adds an
  emission — precedent permits that (the predecessor landed five of six stages DOM-half-only). Side effect:
  the security extract's open question "does the `Moved` handler log coordinates?" is answered NO, so the
  coordinate-logging ban is already satisfied by construction.
- **RED lever verified available:** `core:window:allow-start-dragging` IS granted today
  (`pulse-app/capabilities/default.json:9`), so revoking it is a live mutation arm — the same shape the
  predecessor used, where a revoke reddened five stages while every control was still found and pressed.

### 3.2 Resize stage — P-062 residual (CARRY 3)
**Folded:** _"live resize-clamp + the 1.4–2.1 aspect band on release, plus P-082's multi-window-size
internal-scroll half (same mechanic); the RED arm CANNOT be a capability revoke — a user/OS resize is not
capability-gated (measured 2026-08-23-headful-leg-extension) — neutralise `clamp_to_aspect_bounds` instead;
NOTE the shipped P-082 containment is LAYERED (two single-element mutations were absorbed — route column
clamp + parent flex + region overflow), so a defect-reproducing mutation must break the top of the chain"_

- Assert the live resize clamp and the **1.4–2.1 aspect band on release**. Verified at HEAD:
  `WIDGET_MIN_ASPECT = 1.4` / `WIDGET_MAX_ASPECT = 2.1` (`pulse-app/src/window.rs:33-35`),
  `clamp_to_aspect_bounds` (`:294`) applied from the `WindowEvent::Resized` arm (`:211`, call at `:230`) after
  a `ASPECT_DEBOUNCE` of 150ms — i.e. **clamping is debounced to release**, matching the annotation's "on
  release" wording. Unit coverage exists (`pulse-app/tests/unit_window_constraints.rs`).
- **P-082's multi-window-size internal-scroll half** rides the same mechanic (resize the window, assert the
  traces region still bounds itself and the page does not outer-scroll).
- **RED arm is NOT a capability revoke** (a user/OS resize is not capability-gated — measured by the
  predecessor); the lever is neutralising `clamp_to_aspect_bounds`.
- **Layered-containment warning carried:** a defect-reproducing P-082 mutation must break the TOP of the
  chain — the predecessor's two single-element mutations (route column clamp · parent flex · region overflow)
  were absorbed and produced a *reported partial*, which is the recorded precedent this stage must not repeat.

### 3.3 Native-menu stage — P-064 / P-065 residual (CARRY 4)
**Folded:** _"whether a NATIVE WebView2 context menu is observable to WebDriver at all remains
NOT-YET-PROBED (reasoned unobservable, never measured); if unobservable, assert the suppression side
(`defaultPrevented`) or decline with a recorded reason — never a vacuous stage"_

- Probe observability of a native WebView2 context menu to WebDriver.
- If unobservable: assert the **suppression side** (`defaultPrevented`) instead, or **decline with a recorded
  reason**. A stage that "passes" because it can see nothing either way is explicitly out of bounds.
- Verified at HEAD: P-064 and P-065 both already carry the note _"Live headful right-click folded into
  P-076"_ in their matrix acceptance, and P-076 is `verified` — so this stage **strengthens existing
  evidence**; it does not unblock a pending claim.

### 3.4 Every-time-toast repetition proof — P-063 residual (CARRY 5)
**Folded:** _"proving the signpost fires on EVERY widget close needs two closes, hence the tray-restore
mechanic; the single-close half is proven (widget-close requires the labelled `tray.signpost.shown` record
since 2026-08-23-headful-leg-extension)"_

- The single-close half is already proven; what is owed is **repetition** — two closes, which requires the
  **tray-restore mechanic** (restore from tray between the closes).
- Verified at HEAD: `tray.signpost.shown` is emitted at `pulse-app/src/window.rs:169` carrying the bounded
  `window_label` field, has its exact allowlist leaf (`pulse-app/src/observability.rs:1007`) and is already
  the driver's `SIGNPOST_TARGET` (`xtask/src/webview_drive.rs:42`), asserted in the widget-close stage.
- **[P3-VERIFIED]** Tray restore runs through `focus_or_show_window` (`pulse-app/src/tray.rs:273`), reached
  from the tray icon/menu — an OS-native surface with no WebDriver reach (design-system §Surface:
  desktop-native puts tray interaction under OS control). So the **restore leg** is the declinable part, not
  the second close press. `core:window:allow-show` IS granted, so a programmatic re-show remains available as
  SETUP between the two closes while the assertion stays on a real affordance press — a recorded decline of
  the tray-driven form is acceptable by the same rule as 3.3.

---

## 4. Half B — navigation-race disposition (CARRY 6, operator directive 2026-08-24)

**Folded:** _"the per-boot lost-initial-navigation race (measured under the automation environment: 6/7 boots
struck, victim ~random across findings/report/main; driver-remediated by loud recorded re-navigation,
`recovered_from_blank`) has UNMEASURED production exposure and NO confining mechanism identified — every
production boot creates and navigates the same four windows, and a victim `findings`/`report` window would
surface as a blank window on first open (`show()` ≠ navigate); either MEASURE a plain boot's per-webview URLs
(no driver attached) or ship the product-side mirror (a re-navigate-on-show guard in
`pulse-app/src/window.rs`); boundary wording lives at test-plan §6"_

**Verified at HEAD:**
- The four windows ARE declared in `pulse-app/tauri.conf.json` (`compact-widget` · `main` · `findings` ·
  `report`), all `visible: false`, none with an explicit `url` — so every production boot does create and
  navigate the same four webviews the automation environment does. The annotation's premise holds.
- The remediation `recovered_from_blank` exists only driver-side
  (`pulse-app/ui/tests-e2e/webview-drive.mjs:547`).
- test-plan §6 carries the operator boundary **verbatim** ("measured under the automation environment;
  production exposure UNMEASURED; no confining mechanism identified") and explicitly names the working route
  as owner of the open half. The cross-reference is accurate; no correction owed.

**Disposition — one of the two named paths must be established (not both required):**
- **(a) Measure** a plain boot (no driver attached) reading per-webview URLs, establishing whether the race
  reproduces outside the automation environment; **or**
- **(b) Guard** — ship the product-side mirror so a blank victim re-navigates.

**[P3-VERIFIED — the dictated coordinate is wrong, and the correction stands].** The annotation names "a
re-navigate-on-show guard in `pulse-app/src/window.rs`", but `handle_window_event` handles exactly three
events — `CloseRequested` (`:183`), `Moved` (`:204`), `Resized` (`:211`) — and Tauri 2's `WindowEvent` exposes
**no Shown/Visible variant**. Confirmed against the code that actually OWNS window presentation (per the a11y
precedent that an absence must be checked against the owning component): `findings.show()`
(`use-findings-window.ts:162`), `report.show()` (`:208`) and `main.show()` (`use-toggle-dashboard.ts:28`) are
all FRONTEND calls via `@tauri-apps/api/webviewWindow`; Rust's only `show()` sites are the widget at boot
(`window.rs:253`), the signpost path (`:167`), tray restore (`tray.rs:273`) and `snapshot_runtime.rs:278`.
**There is no on-show hook at the named site** — a guard needs a different Rust mechanism or a frontend
pre-show check, which is a live input to the path decision.

**A THIRD path surfaced at P3.** Path (a) as literally worded has no existing mechanism: with no driver
attached, nothing in the shipped product reports a webview's URL, and attaching a driver is precisely what
makes the boot not-plain. The real options are therefore **(a) measure** via some added read-back, **(b)
guard** (re-navigate a blank victim), or **(c) instrument-then-measure** — a one-shot boot-time navigation
check emitting a bounded record (WARN when a window is still `about:blank`), which measures production
exposure AND leaves a durable detector, at the cost of a new obs target + exact allowlist leaf + a guard test
under `pulse-app/tests/`. P4 resolves this with the operator rather than silently picking one.

Whichever path is taken, the **durable text is corrected to what was established** — the test-plan §6
boundary must end this chunk stating a measured scope with a named confining mechanism, or stating the guard
now confines it. Leaving the boundary as-is while claiming the half closed is out of bounds.

---

## 5. PREREQ — `cargo audit` (pin #14)

**Folded (compact form):** standing deferral since `2026-08-15-corpus-key-persistence`, ratified at the
2026-08-16 0-pending adaptation wrap; **every-3rd-wrap re-run INTERVAL**; point 40 discharged in FULL form at
the `2026-08-23-a11y-verification` wrap; **next probe point 43**.

**Verified:** `state.yaml` `session_count: 41`, so the predecessor's wrap was point 41 and **this chunk's wrap
is point 42 — it owes NO probe.** The obligation is to re-verify the basis (RustSec DB unloadable —
`parse error: duplicate advisory ID: RUSTSEC-2026-0244`, upstream) plus the overlap signal
(`cargo deny check bans licenses sources` green as the pass/fail gate, `cargo deny check advisories` observed
SEPARATELY as designed-red), re-enumerate the owned upgradeable ID set first-hand
(0189/0190/0194/0195/0204/0222/0253/0258 — eight at last count, and the set has GROWN historically so it is
re-counted, never carried forward), and record `probe skipped per ratified interval (next: 43)` in the chunk
report. A silent skip is not permitted.

---

## 6. Boundaries — what this chunk does NOT do

- **No capability claim is assumed.** Every capability these CARRYs touch is ALREADY `verified` in
  `verification-matrix.json` — P-061, P-062, P-063, P-064, P-065, P-066, P-082 — and the only two unclaimed
  entries in the version (P-075 `dynamic-external`, P-077 `by-construction`) are owned elsewhere on the route
  (P-077 by the Demo-injector entry; P-075 declined at `2026-08-17-conductor-e2e-verification-closure`). This
  chunk **strengthens evidence for already-verified caps**, exactly as its predecessor did. **[P3-VERIFIED]**
  The matrix carries 22 capabilities — 20 `verified`, 2 `planned` — so the expected outcome is *link nothing*;
  P5 decides against the contract, and a decision to claim nothing is recorded rather than silent.
- **Does not fix the advisory backlog** (its own route entry) — the PREREQ is an interval re-check only.
- **Does not touch the Halo canvas disposition, npm advisory coverage, diagnostics un-muting, the
  staged-bindings assertion, the metrics label surface, or the demo injector** — each is its own markerless
  route entry.
- **Does not widen the `webview-drive` CLI shape** unless a landed stage requires it; stage ids stay
  `--expect-absent`-eligible by construction.
- **Does not CI-wire the leg** (test-plan §6 records it gates nothing today; that remains true after this
  chunk unless explicitly taken up).
- **No vacuous stage.** A stage whose assertion cannot fail is a defect, not coverage — decline and record.

---

## 7. Surfaces and contracts touched (expected)

| Surface | Expected involvement |
|---|---|
| `xtask/src/webview_drive.rs` | stage table (`STAGES`, currently **13**), `stage_halves` verdict readers, report |
| `pulse-app/ui/tests-e2e/webview-drive.mjs` | probe + new stage drivers; DOM observation records |
| `pulse-app/src/window.rs` | Half B site (**if** path (b)/(c)) — NO on-show hook exists; a different mechanism is required, see §4 |
| `pulse-app/src/observability.rs` | only if a new obs target/leaf is needed (predecessor needed none) |
| `pulse-app/capabilities/default.json` | RED mutation arm only — revoke-and-restore, byte-identical after |
| `.andromeda/test-plan.md` §6 | boundary text corrected to what this chunk establishes |
| `verification-matrix.json` | expected no-op (see §6) |

**Invariants that bind this chunk:** stage assertions match an effect FIELD, never the target alone; both
verdict halves stay independent (`StageHalves{obs, dom}`); the RED arm must be proven to discriminate; the
capability JSON must be restored byte-identical (hash-verified) after any revoke mutation — a discipline no
shipped gate enforces (owned by the Staged-bindings assertion entry).

---

## 8. Premise closure (P3, 2026-08-24)

All five premises are closed against `research.md`; the tags above are updated in place.

1. **VERIFIED** — no prior in-repo measurement of W3C Actions / `setWindowRect` exists (§2).
2. **FALSIFIED → corrected** — `WindowEvent::Moved` is MAIN-only and emits no record; the widget is
   deliberately excluded. Neither drag nor a successful resize clamp has an obs half at HEAD, so both stages
   are DOM-half-only unless this chunk adds an emission (§3.1, §3.2).
3. **VERIFIED** — tray restore is OS-native (`tray.rs:273`) and out of WebDriver reach, so the RESTORE leg is
   the declinable part; the second close stays a real affordance press (§3.4).
4. **VERIFIED** — no on-show hook at the named site; presentation of `findings`/`report`/`main` is
   frontend-owned. A THIRD path (instrument-then-measure) surfaced and goes to the operator at P4 (§4).
5. **VERIFIED** — 20 `verified` / 2 `planned`, both planned ones owned elsewhere ⇒ link nothing (§6).
