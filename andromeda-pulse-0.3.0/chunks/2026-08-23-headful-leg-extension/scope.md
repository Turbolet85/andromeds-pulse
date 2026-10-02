# Scope — 2026-08-23-headful-leg-extension

**Chunk:** Headful leg extension — the window-mechanics affordances a live drive can prove but the
assembled path never touches.
**Version:** andromeda-pulse-0.3.0 · **Epoch 4 — Polish & ship: verification**
**Working entry:** `andromeda-pulse-0.3.0/working-route.md` (frozen this promotion)

---

## What this chunk builds

The 7-stage `cargo xtask webview-drive` leg shipped at `2026-08-23-integration-ux-e2e-test` drives one
assembled path: launch → traces-empty → traces-populate → storm-incident → investigate → empty-states →
widget-close. Seven residual affordances were boundaried OUT of it because each needs a driver MECHANIC
that path never exercises — pointer drag, window resize, native-menu observation, global shortcut / tray
lifecycle, multi-window docking geometry — or, for the footer line, purely to bound that chunk's flake
surface.

This chunk EXTENDS that leg with those affordances, so the residuals stop resting on unit/config proofs
plus operator eyeballs and start resting on a real headful drive of the real control. It also owns the
leg's measured `launch` flake, which currently has no owner, and re-points the Traces stage selectors onto
the accessible anchors the surface now ships.

**Verified extension seam** — the leg is genuinely extensible, and the mechanism is TWO-SIDED (the working
entry says "a `STAGES` table with per-stage `obs`/`dom` predicates"; that is correct but under-specified,
so the coordinate is recorded here):
- `xtask/src/webview_drive.rs:51` — `pub const STAGES: &[Stage]`, each `Stage { id, what }`; `id` is what
  `--expect-absent` names.
- `xtask/src/webview_drive.rs:270` — `stage_halves(id, records, dom) -> StageHalves`, the per-stage
  predicate pair: the **obs half** reads the app's own observability log, the **dom half** reads the
  driver's `webview-drive-stages.json` report. `stage_observed` (`:311`) requires both applicable halves.
- `pulse-app/ui/tests-e2e/webview-drive.mjs` — the WebdriverIO driver that presses controls and calls
  `record(stage, observed, detail)` (`:82`) into that report.

So each CARRY below lands as: a `STAGES` entry + its `stage_halves` arm + the driver steps that press the
real control.

## The verdict discipline this leg inherits (non-negotiable)

A press that "returned without throwing" proves nothing — the Tauri ACL drops a webview IPC silently, which
is the dead-affordance mode this whole leg exists to catch. Every stage's verdict is the **app's own
record** (obs log) and/or the driver's DOM report, never the absence of an exception. Every new stage must
also be **mutation-checked in both directions** (GREEN observes; RED with the capability/handler
neutralised still finds and still presses the control while the record never appears).

---

## Folded annotations — each verified at HEAD before it shaped this scope

Per `promotion.md`, the working entry's CARRY / PREREQ annotations fold here as HYPOTHESES and every named
coordinate was re-derived first-hand. Results:

| # | CARRY | Named coordinate | Verified at HEAD |
|---|---|---|---|
| 1 | Titlebar drag → window-position-delta > 0 (P-061 residual) | drag region | ✅ `WIDGET_EDGE_MARGIN = 24` + `compute_snap_position` (`pulse-app/src/window.rs:313`, `:343`) |
| 2 | Live resize-clamp + aspect band on release (P-062 residual) | band 1.4–2.1 | ✅ `WIDGET_MIN_ASPECT = 1.4` / `WIDGET_MAX_ASPECT = 2.1` (`window.rs:33`,`:35`); `clamp_to_aspect_bounds` (`:290`) fires on a settle-delayed `Resized` (`:226`); min-size `400×225` in `tauri.conf.json` |
| 3 | Prod browser-chrome suppression via real right-click + canvas drag (P-064/P-065) | handlers | ✅ `use-suppress-browser-chrome.ts:38-39` (`contextmenu` + `dragstart`). **The entry's NOT-YET-PROBED question stands unprobed** — see Open questions |
| 4 | Widget↔dashboard toggle + boot geometry + per-window close + toast (P-066/P-061/P-063) | `button[aria-label="Toggle dashboard"]` · Cmd/Ctrl+Shift+P | ✅ `Titlebar.tsx:105`; shortcut via `useDashboardToggleShortcut` (`hooks/use-toggle-dashboard.ts:41`); per-window close `close_sends_app_to_tray` (`window.rs:144`) |
| 5 | Traces INTERNAL-SCROLL live across a couple of window sizes (P-082 residual) | `trace-table-scroll` | ✅ `TraceTable.tsx:173` |
| 6 | Incidents floating-window disclosure live, multi-window (2026-07-10 residual) | window labels | ✅ `findings` / `report` / `compact-widget` / `main` (`window.rs:22-25`); all four configured in `tauri.conf.json`, `visible: false`, `decorations: false` |
| 7 | Plain-language connection-status FOOTER LINE (P-070 residual) | `[data-testid="connection-status-line"]` at `ConnectionStatusLine.tsx:110` | ✅ **exact line match** |
| 8 | The `launch` flake — needs an owner | measured 1/5, 1/4, 2/5 over 14 runs | ✅ leg appears in NO `.github/workflows/` file, so "gates nothing today" holds |
| 9 | Re-point Traces stage selectors onto accessible anchors | `region[aria-label=…]` · native `table` · `.trace-row` | ✅ `ConstellationCanvas.tsx:250`; native `<table>` `TraceTable.tsx:176`; `className="trace-row"` `:324` |

**One premise correction found (#4).** The entry calls for the "every-time" `still running` toast. That IS
the shipped behavior — `should_show_close_signpost(notifications_enabled) -> notifications_enabled`
(`window.rs:135`) gates on the setting alone, with no once-per-process latch. But the surrounding prose
still says "the **one-time** … signpost" (`:148`) and "fire the **first-close** … signpost **once**"
(`:187`) — stale comments from before the P-066 correction. The behavior to assert is EVERY-TIME; the
stale comments are a cleanup this chunk may take in passing. **[verified P3]**

**One premise refinement found (#9).** The re-point is a **preference change, not a repair**: the current
testids (`trace-table`, `trace-table-empty`, `trace-row` — `webview-drive.mjs:36-38`) still exist in the
markup (`TraceTable.tsx:325` carries `data-testid="trace-row"` right beside `className="trace-row"`), so
today's selectors are not broken. The value is anchoring the leg on the accessible name/role the a11y
chunk made load-bearing, so a future a11y regression reddens the leg. **[verified P3]** — `TraceTable.tsx:325` carries the testid beside the class at `:324`; the re-point is an edit to the driver's selector-constant block (`webview-drive.mjs:36-50`) plus `countRows`.

### PREREQs

- **PREREQ A — Rust gate deferral (origin `2026-08-23-a11y-verification`).** `cargo clippy --workspace
  --all-targets --all-features` + `cargo nextest run --workspace --profile ci` were deferred there on a
  mechanically-confirmed zero compiled-source delta. This chunk touches `xtask/` (Rust), so the deferral
  closes here and both gates run for real.
- **PREREQ B — `cargo audit` standing deferral (pin #13).** Ratified every-3rd-wrap INTERVAL; probe points
  ran at sessions 25/28/31/34/37/**40 (discharged in FULL form)**. **The next point is 43.** `state.yaml`
  reads `session_count: 40`, so this chunk's wrap is **point 41 and owes NO probe** — it must record
  `probe skipped per ratified interval (next: 43)` in the chunk report rather than skipping silently. The
  overlap signal (`cargo deny check advisories`) still runs and re-enumerates its owned set (**8** IDs at
  the last probe — re-enumerate, never carry the count forward).

---

## Boundaries — what this chunk does NOT do

- Does **not** re-verify the 7 stages the assembled leg already covers; those stay as shipped.
- Does **not** wire the leg into CI. It gates nothing today, and wiring a leg with a live `launch` flake
  into a workflow with a zero-flakiness budget (test-plan §10) would be a knowingly-red gate. **[verified P3]** — no `.github/workflows/` file references `webview-drive`.
- Does **not** adopt a second driver (CDP + Playwright was probed, recorded and DECLINED at
  `2026-08-23-webview-self-verify`; `playwright.config.ts` stays inert for this purpose). **[verified P3]** —
  both configs exist, but only `playwright-a11y.config.ts` is referenced (by `test:a11y`); the base config is
  referenced by nothing in `package.json`, `.github/workflows/` or `xtask/src/`.
- Does **not** change product behavior for the affordances it asserts — these are residual VERIFICATION
  gaps on shipped capabilities, not repairs. Any product-side change is a finding to surface, not silent
  in-scope work (excepting the stale-comment cleanup above).
- Does **not** re-open `role="status"` on the constellation summary — ratified OFF by the operator
  2026-08-23.

---

## Open scope questions — ALL RESOLVED AT P4 (recorded here so this file stays the val-1 anchor)

**Resolution summary.** The operator resolved Q1 at the P4 dialogue in favour of the boundary research
measured: **land the six CARRYs reachable with mechanics the leg already uses** (#4 toggle + per-window close,
#5 at default size, #6 multi-window, #7 footer, #8 flake, #9 re-point), and **defer the three that need a
mechanic never measured on this host** (#1 pointer drag, #2 window resize, #3 native-menu observation) plus
#5's multi-window-size half. Q2 is therefore deferred WITH CARRY #3 rather than answered. Q3 is answered:
research located the flake's mechanism (below), and the plan's Step 1 owns it.

Two sub-parts were additionally deferred as a consequence, each recorded in `plan.md`
§Constraints & rejected approaches: the **every-time** claim on the close signpost (proving repetition needs a
second widget close, hence the tray-restore mechanic, which is in the deferred set — this chunk asserts the
single close), and **P-061 boot geometry** (the other half of the same residual whose drag half is deferred).

The three deferred CARRYs + the two sub-parts need a follow-up route entry; wrap's route-resolve is the
channel that homes them.

---

### The questions as originally posed (for the record)

1. **Breadth.** This entry carries 7 stage-CARRYs + the flake + the selector re-point + 2 PREREQs. Both the
   session handoff and a prior evolve diagnosis flagged the accretion and suggested the entry "may deserve
   splitting at its promotion". The fork — land all seven stages in one chunk, or scope to a coherent
   subset and carry the remainder — is contestable and materially changes the work, so it resolves with the
   operator at P4 with a marked recommendation.
2. **Is CARRY #3 even expressible?** The entry records the question verbatim and unprobed: *whether a
   NATIVE WebView2 context menu is observable to WebDriver at all — it is reasoned unobservable, never
   measured*. P3 must probe this before the plan commits to a stage. If it is unobservable, the honest
   alternatives are asserting the *suppression* side only (the menu never appears / `defaultPrevented`) or
   declining the stage with a recorded reason — never a stage that passes vacuously.
3. **What owning the flake means.** The measurement is unusually complete (perfectly bimodal across 14
   runs: `observed=true` always coincides with `toggle_pressed=false`; toggle recovery 0/10; 50s already
   elapses in failing runs) and the operative diagnosis is a **startup race**, not a short wait. Owning it
   could mean root-causing the race, or making the leg deterministic around it. Which of those this chunk
   commits to depends on what P3 finds. **[verified P3 — mechanism located]** — `webview-drive.mjs:345-364`
   reproduces the bimodality exactly (self-mount poll wins ⇒ `toggle_pressed=false`; else the toggle path is
   taken and still fails). The toggle cannot recover it because the recorded failure is `main`'s webview never
   leaving `about:blank` — `show()`+`setFocus()` cannot mount an SPA that never navigated. `dumpHandles`
   already captures each handle's label + URL, so the diagnostic substrate exists.

---

## Surfaces + contracts touched

- `xtask/src/webview_drive.rs` — `STAGES` table, `stage_halves` predicate arms, CLI arms.
- `pulse-app/ui/tests-e2e/webview-drive.mjs` — driver steps, selectors, stage records.
- **No product-source change expected** beyond the optional stale-comment cleanup in `pulse-app/src/window.rs`.
- **No new TauRPC procedure**, so no `EXPECTED_PROCEDURES` pin, no bindings regen obligation from this
  chunk's own surface. **[premise-corrected P3: the "no capability JSON edit" half needed narrowing —
  true for the GREEN path, but for the reason that every needed `core:window:*` grant is ALREADY in
  `default.json` (`allow-start-dragging` / `allow-close` / `allow-show` / `allow-set-focus` / `allow-hide` /
  `allow-set-position` / `allow-set-size`), not because procedures imply no grants. Security and arch were
  right to flag it. The RED arms revoke-and-restore those grants; a user-driven OS resize is NOT
  capability-gated at all, so CARRY #2's RED arm must neutralise `clamp_to_aspect_bounds` instead.]**
- **Env vars** — reuses the registered harness-only set: `ANDROMEDA_PULSE_MSEDGEDRIVER_PATH` (xtask-side)
  and the xtask→node relay `PULSE_BIN` · `PULSE_DATA_DIR` · `MSEDGEDRIVER_PATH` · `PULSE_INJECTOR`. A new
  relay var would need registering in arch §Occupied Resources. **[verified P3]** — the existing set covers
  the leg; no new var is implied unless the plan adds one.
- **Observability** — **[premise-corrected P3: sharper and more expensive than "wherever possible".]**
  `pulse-app/src/window.rs` emits NOTHING on the `Moved` path and nothing on a SUCCESSFUL `Resized`/clamp
  (only a `warn!` when the clamp fails), so **CARRY #1 and CARRY #2 have no obs half at HEAD** — each needs
  a DOM-only verdict via the already-granted Tauri geometry getters, or a new target + exact allowlist leaf
  (which obs bars from emitting per-event and security bars from carrying coordinates). Separately,
  `tray.signpost.shown` is emitted with NO fields and resolves to `None` through `for_target`'s fallback
  chain (there is no bare `tray` key), so CARRY #4's toast half can currently assert only record presence —
  which test-plan §6's effect-FIELD rule disallows.
