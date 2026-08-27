# Codebase Research — 2026-08-27-report-window-copy-affordance

## Scope
- **Depth:** moderate · **Reads:** 12 · **Globs/Greps:** 11 · **Graph queries:** 5 (ts ×3, rust ×2; all `db_state: fresh`)

## Files inspected
- `pulse-app/capabilities/clipboard.json` (full) — one permission `clipboard-manager:allow-write-text`; `"windows": ["compact-widget", "main"]`. The `report` label is absent. The `description` already carries a rationale + the NEVER-add-read ban, so the widening extends an existing rationale rather than inventing one.
- `pulse-app/tauri.conf.json` (`app.windows`) — four declared labels: `compact-widget`, `main`, `findings`, `report`; all `visible: false`, all `url: null`.
- `pulse-app/src/window.rs:22-25` — `COMPACT_WIDGET_LABEL` / `FINDINGS_WINDOW_LABEL` / `REPORT_WINDOW_LABEL = "report"`; `sanitize_window_label` at `:111-112` maps them.
- `pulse-app/src/main.rs:1083` — `.plugin(tauri_plugin_clipboard_manager::init())`; `:1094` `.invoke_handler(invoke_router.into_handler())`. The plugin is registered app-wide; the capability is what scopes it per window.
- `pulse-app/ui/src/report/use-report.ts` (full, 105 lines) — `writeText` imported at `:14`; `copyMarkdown` at `:84-96` sets `copying` → awaits `writeText(report.markdown)` at `:91` → `copied`, with a bare `catch { setCopyState("error") }` at `:93-95`. `AUTO_RESET_MS = 2_000` auto-resets `copied`/`error` back to `idle` (`:72-82`) — **a 2-second observation window for any assertion.**
- `pulse-app/ui/src/report/Report.tsx` (full, 120 lines) — sole consumer of `useReport`. **`:29-31` computes `liveMessage` from `COPY_LIVE_REGION_MESSAGES[copyState]` and `:58-59` passes it to `Modal` as `liveRegionLevel="polite"` + `liveMessage`.**
- `pulse-app/ui/src/report/report-types.ts:126-133` — `COPY_LIVE_REGION_MESSAGES`: `idle: ""` · `copying: "Copying report to clipboard…"` · `copied: "Report copied to clipboard."` · `error: "Failed to copy report to clipboard."` The doc comment already cites a11y-plan §7.
- `pulse-app/ui/src/report/ReportRenderer.tsx:425-460` — the Copy button: `data-testid="report-copy-markdown"`, **`data-copy-state={copyState}`**, `disabled={copyState === "copying"}`, a `✓` glyph (`aria-hidden`) in the `copied` state, and `copyButtonLabel()` at `:449-459` returning `Copy markdown` / `Copying…` / `Copied` / `Copy failed — retry`.
- `pulse-app/ui/src/components/Modal.tsx:44-45, 63-64, 128, 205-220` — `liveRegionLevel`/`liveMessage` props; `aria-busy={busy}` on the dialog (fed by `loading`, not by copy state); the live region is `role="status"` + `aria-live={liveRegionLevel}` rendering `{liveMessage}`.
- `pulse-app/src/snapshot_runtime.rs:24, 245-290` — the OTHER clipboard path: `handle.clipboard().write_text(report.markdown.clone()).is_ok()` at `:254-257`, then `tracing::info!(target: "snapshot.clipboard.write", byte_count, success, dual_file_paths_basenames)` at `:283-288`.
- `pulse-app/src/observability.rs:1156-1165` — the exact allowlist leaf for `snapshot.clipboard.write` (`byte_count`, `success`, `dual_file_paths_basenames`), with guards at `:4409` (field-set) and `:4423` (redaction of `clipboard_content` / `raw_markdown`).
- `xtask/src/webview_drive.rs:46-127, 244-295, 350-362, 570-600, 1270-1300, 1375-1420` — the `STAGES` table (15), `dom_stage` lookup, `stage_halves` dispatch, `dom_report_window_ok`, and the 5 stage-level unit tests.
- `pulse-app/ui/tests-e2e/webview-drive.mjs:44-59, 898-985` — driver selectors and the full `report-window` stage body.

## Graph impact (trace: `.andromeda/runs/2026-08-27T20-41-09Z-phase/tree-query-2026-08-27-report-window-copy-affordance.json`)
- **`useReport()`** (ts) — 2 refs, BOTH in `pulse-app/ui/src/report/Report.tsx` (`:10` import, `:23` call). Single consumer; changing the hook's returned shape has a one-file blast radius.
- **`CopyState`** (ts) — 7 refs: `ReportRenderer.tsx:20,39,448` · `report-types.ts:127` · `use-report.ts:15,21,31`. Any new copy state (e.g. a distinct `acl-rejected`) touches exactly these three files.
- **`copyMarkdown`** (ts) — 0 refs; re-probed per the cookbook's zero-callers precondition: the symbol IS indexed (`UseReportResult#copyMarkdown.` @ `use-report.ts:22`, plus `ReportRendererProps#onCopyMarkdown.` @ `ReportRenderer.tsx:38`). Zero refs is a destructuring artifact (`Report.tsx:24` destructures it, `:47` calls the local binding), **not** dead code.
- **`STAGES` / `stage_ids`** (rust) — 10 caller sites: `stage_ids()` `:133`; `run_webview_drive()` `:138,142,244,270`; and 5 unit tests `:1381,1388,1395,1402,1412`. **All five tests iterate `STAGES` generically — none hardcodes an id list** (the two `--expect-absent` tests name `"storm-incident"` only as a subject), so adding a stage requires no edit to any of them.
- **`Stage` / `StageHalves` / `dom_report_window_ok`** (rust) — `Stage#` `:47`, `StageHalves#{obs,dom}` `:307-309`, `satisfied()` `:313`, `dom_report_window_ok()` `:576`, and its two pins `report_window_requires_the_full_escape_chain` `:1275` / `a_lost_focus_restore_fails_report_window` `:1286`.
- Cross-plane note: this chunk's seam is a **capability ACL**, not an IPC procedure — no name-bridge join was needed, because no symbol crosses the boundary (the ACL is data, evaluated in the Tauri runtime).

## Patterns detected
- **DOM-only stage verdict is the shipped design for this exact window** (`xtask/src/webview_drive.rs:352-361`): `findings-window` and `report-window` are both `StageHalves { obs: None, dom: Some(..) }`, with the comment "the findings/report lifecycle is JS `hide()`/`show()` end to end — no Rust hook, no obs record — so these two are DOM-only by construction". A DOM-only copy verdict is therefore consistent, not a concession.
- **Stage lookup is id-keyed, not order-keyed** (`dom_stage` `:291-295`; `stage_halves` matches on `stage.id`). A `record()` call for a new stage id can be emitted from anywhere in the driver — including mid-way through the `report-window` block, while the report is open — without reordering `STAGES`.
- **All-required-fields predicate** (`dom_report_window_ok` `:577-599`): seven named booleans, `.all(|f| stage_bool(&stage, f) == Some(true))`. The established shape for a new predicate.
- **Vacuity gating is a hard-won convention here** — the driver comments at `:918-921` and `:962-964` record two measured vacuous-green incidents (a hidden window's dialog satisfied a probe; click-focus was indistinguishable from restored focus). Any copy probe must be gated on the report window being visible AND focused.
- **Backend clipboard write, obs-recorded, no payload** (`snapshot_runtime.rs:254-288` + `observability.rs:1159-1165`): outcome as `success: bool` + `byte_count`, never the text. The template if the machine-readable half is bought.
- **Auto-reset window** (`use-report.ts:72-82`): `copied` and `error` decay to `idle` after 2 s. An assertion that polls too slowly reads `idle` and cannot tell success from failure — a real vacuity risk unique to this stage.

## Conventions to follow
- **Selector binding**: `data-testid` is correct here despite the accessible-name-first rule (test-plan §6). The Copy button's accessible name is its label, which **changes with state** (`copyButtonLabel`, `ReportRenderer.tsx:449-459`), so a name-bound selector would be unstable by construction. `data-testid="report-copy-markdown"` already ships (`:429`) and is the stable anchor; the state is read from the sibling attribute `data-copy-state` (`:430`).
- **Capability rationale in-file** (`clipboard.json` `description`): the existing text names its chunk-#43 origin and the never-add-read ban; the widening appends its reason in the same field (arch §Webview IPC capability policy; security-plan §Anti-Patterns → API).
- **Predicate + two pins**: a new `dom_*_ok` gets a positive pin and a negated pin, per `:1275` / `:1286`.
- **Obs leaf + two guards** if a new target lands: field-set resolution (`observability.rs:4409` shape) and non-allowlisted-field redaction (`:4423` shape), both under a test module that actually runs.

## New files to create
- `pulse-app/ui/src/report/use-report.test.ts` — **no unit test exists for the copy state machine today** (the directory holds only `SendToAgentButton.test.tsx` + `use-mcp-delivery.test.ts`). Pins: `writeText` rejection → `copyState === "error"`; resolution → `"copied"`; `report === null` → `"error"` without calling `writeText`; the 2 s auto-reset.

## Files to modify
- `pulse-app/capabilities/clipboard.json` — add `"report"` to `windows`; extend `description` with the rationale.
- `pulse-app/ui/tests-e2e/webview-drive.mjs` — press `[data-testid="report-copy-markdown"]` while the report window is visible and focused (inside the existing `report-window` block, **before** its Escape chain), poll `data-copy-state` within the 2 s window, and `record()` the observation.
- `xtask/src/webview_drive.rs` — `STAGES` entry (if a separate stage id is chosen), the `stage_halves` arm, a `dom_*_ok` predicate, and its two unit pins.
- `pulse-app/ui/src/report/use-report.ts` — **only if** the P4 fork buys the machine-readable diagnostic (the `catch` would gain a reporting call).
- `pulse-app/ui/src/report/report-types.ts` + `ReportRenderer.tsx` — **only if** the fork adds a distinct copy state (the `CopyState` ref set above is the exact threading list).
- `pulse-app/ui/tests-a11y/` — the a11y extract requires the p9 report spec to cover the rejected/in-flight copy states; **no `tests-a11y` file mentions copy today** (grep: 0 hits), so this is a coverage addition, and its exact spec file is an implement-scope detail.
- Not modified, confirmed from the graph: the five `webview_drive` stage tests (they iterate `STAGES`), `EXPECTED_PROCEDURES`, `capabilities/default.json`, and the three NEVER-widen capability files.

## Open questions
1. **Does scope item 4's machine-readable diagnostic get bought, and at what cost?** Security-plan §Logging & Monitoring → What to log requires ACL-rejected IPC calls to be logged; obs-plan §1/§3 admit a frontend observable only through a `telemetry.frontend.*` TauRPC procedure (a new procedure + `EXPECTED_PROCEDURES` pin + the emit-bindings merge + an arch §Occupied Resources entry), which the scope explicitly fences out. A third path exists and is architecturally cleaner — route the report copy through a **backend** procedure like `snapshot.generate` already does, which sidesteps the webview ACL entirely and gets `snapshot.clipboard.write`-shaped obs for free — but that is the same procedure cost plus a behaviour change, and it would make the ACL widening unnecessary rather than complementary. → blocks: **plan-decision** (P4 must resolve before synthesis).
2. **One stage or two?** Extending `report-window`'s existing predicate keeps the count at 15 (no arch §Stack Role-cell amendment) but makes the copy behaviour inseparable from that stage's `--expect-absent` arm; a new id (e.g. `report-copy`) gives a separable RED arm at the cost of 15 → 16 and the Role-cell amendment. → blocks: **plan-decision**.
3. **Is the ACL the proven cause, or the strongly-inferred one?** Verified first-hand: the capability configuration (report absent from `windows`), the call site, the swallowed rejection, the operator's live symptom, and that the working sibling path writes from Rust (ACL-exempt). NOT yet executed: a press observed failing *because of* the ACL. The chunk's own RED arm — press before the widening, observe `data-copy-state="error"`; press after, observe `"copied"` — is what converts this to measurement, and it should be sequenced BEFORE the fix. → blocks: **implementation-scope** (ordering, not file set).

## Scope premise closure
Three `[inferred]` bullets were closed against these findings and `scope.md` was amended in place:
1. **Item 3's undecided observable — VERIFIED, tag dropped.** Two discriminating observables already ship (`data-copy-state`, and the live-region text); neither costs a procedure, an obs leaf, or a read permission.
2. **Item 4's "is a diagnostic needed" — PREMISE-CORRECTED.** The user-facing half already ships (label + live-region announcement), so the `catch` is less silent than assumed; the machine-readable half is genuinely absent and structurally costed. Item 4 is a P4 fork, not a research call.
3. **The second clipboard surface — PREMISE-CORRECTED.** The conclusion (out of scope) holds; the mechanism does not. It mounts in both `main` and `compact-widget`, but that is moot: it never touches the webview clipboard plugin — the write is Rust-side via `AppHandle::clipboard()`, which the ACL does not gate.

No extract leaned on a falsified premise in a way that poisons synthesis: the a11y extract explicitly deferred the live-region question to research ("whether the code already exposes any live region for the Copy result is research's question"), and this research answers it YES — which **satisfies** rather than contradicts its acceptance contribution.
