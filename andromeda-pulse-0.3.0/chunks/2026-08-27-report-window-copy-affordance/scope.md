# Scope — 2026-08-27-report-window-copy-affordance

**Working-route entry (verbatim intent):** Report-window Copy affordance — the report's Copy control copies
instead of failing, and a gate presses it.

## Outcome

A user pressing **Copy markdown** in the Diagnostic Report window gets the report on their clipboard, and a
shipped gate presses that real control so the dead-affordance class stays caught. Today the press produces
`Copy failed — retry` and nothing reaches the clipboard.

## The defect — mechanism re-verified first-hand at HEAD (2026-08-27, this promotion)

Every coordinate the working entry names was re-derived against the artifact before it entered this scope
(promotion.md: annotations fold as hypotheses). All six held:

| Claim (working entry) | Re-derived at HEAD | Verdict |
|---|---|---|
| `capabilities/clipboard.json` grants `clipboard-manager:allow-write-text` to `["compact-widget","main"]` only | `pulse-app/capabilities/clipboard.json` — `"windows": ["compact-widget", "main"]`, one permission `clipboard-manager:allow-write-text` | ✓ exact |
| The Copy control lives in the REPORT window | `pulse-app/src/window.rs:25` `REPORT_WINDOW_LABEL = "report"`; `tauri.conf.json` declares four windows `compact-widget` / `main` / `findings` / `report`; the control is `pulse-app/ui/src/report/ReportRenderer.tsx:427-443` | ✓ exact |
| The plugin IS registered at `main.rs:1083` | `pulse-app/src/main.rs:1083` — `.plugin(tauri_plugin_clipboard_manager::init())` | ✓ exact line |
| The `writeText` IPC is silently ACL-rejected | `pulse-app/ui/src/report/use-report.ts:14` imports `writeText` from `@tauri-apps/plugin-clipboard-manager`; `:91` awaits it inside a `try`; the bare `catch { setCopyState("error") }` at `:93-95` discards the rejection with no diagnostic | ✓ exact |
| No headful stage presses Copy today | `xtask/src/webview_drive.rs::STAGES` — 15 stages; `report-window` opens the report and unwinds with Esc, and **no stage presses `report-copy-markdown`** | ✓ exact |
| The report moved to its own window at chunk 2026-07-10; the capability predates it (chunk #43) and its window list was never widened | `clipboard.json`'s own description names chunk #43 snapshot copy-to-clipboard; the window list carries no `report` | ✓ |

The failure is the textbook negative-default dead affordance this project's own capability policy describes:
the markup exists, the plugin is registered, the press runs, the IPC is dropped by the ACL, and the only
user-visible trace is the button relabelling itself `Copy failed — retry` (`ReportRenderer.tsx:458`).

## In scope

1. **Grant the report window clipboard write.** Widen `clipboard.json`'s `windows` list to include `report`,
   with a stated rationale in the file's own `description` (arch §Webview IPC capability policy requires a
   rationale on every core-API / capability grant). The file's NEVER-add-`allow-read-text` /
   `allow-read-image` posture is preserved verbatim — this chunk adds a WINDOW to an existing outbound
   write-text grant, never a permission.
2. **Press the real control in a shipped gate.** A headful assertion that exercises the actual Copy button
   (not a programmatic `writeText` proxy) so a future ACL regression reddens a gate rather than shipping.
   The `report-window` stage already opens the report window and already has the surface in hand.
3. **Verify the press actually copied** — the assertion must observe a COPY-SUCCEEDED outcome, not merely
   that the button was found and clicked without throwing. `copyMarkdown` swallows the rejection into
   component state, so a click-did-not-throw check passes identically in both worlds. **The
   discriminating observable ALREADY SHIPS, in two forms** — `data-copy-state={copyState}` on
   `[data-testid="report-copy-markdown"]` (`ReportRenderer.tsx:430`) and the live-region text
   (`COPY_LIVE_REGION_MESSAGES` → `Modal`'s `role="status"` + `aria-live="polite"`;
   `report-types.ts:127-133` → `Report.tsx:29-31,58-59` → `Modal.tsx:205-220`). Both discriminate
   `copied` from `error`, and neither needs a new procedure, an obs leaf, or clipboard-read permission.
   *(P3-verified; `[inferred]` dropped.)*
4. **Keep the failure legible** — split by audience, and only half of it is missing.
   *[premise-corrected: the USER-facing half already ships — the button relabels to `Copy failed —
   retry` AND the live region announces "Failed to copy report to clipboard.", so the bare `catch` is
   not as silent as the scope assumed. What is genuinely absent is a MACHINE-readable record: nothing
   reaches `agent-latest.jsonl`.]* That gap is structural, not an oversight: the rejection surfaces only
   inside the webview, and obs-plan §1/§3 route every frontend observable through a
   `telemetry.frontend.*` TauRPC procedure — which this chunk's own boundary fences out — while
   security-plan §Logging & Monitoring → What to log requires capability-rejected IPC calls to be
   logged. **This is therefore a costed fork for P4, not a free research call**, and it is the one open
   design decision in the chunk.

5. **`aria-busy` on the Copy button.** *[scope amended at P5 validation-1 as INTENT-INCOMPLETE — the
   original scope missed it.]* a11y-plan §4 ARIA Patterns (Button row) requires the control to expose
   `aria-busy` during the in-flight write; it is absent today (`Modal.tsx:128` carries `aria-busy` for the
   dialog's LOAD, not for the copy). One attribute on the exact control this chunk exists to repair, plus a
   pin in the vitest file the chunk already adds — taken in-chunk per the operator's standing fix-in-chunk
   preference, and surfaced on the P5 review card so it can be declined without disturbing items 1–4.

## Boundaries — explicitly NOT in scope

- **No clipboard READ.** `clipboard-manager:allow-read-text` / `allow-read-image` stay ungranted; the
  capability description's ban stands untouched.
- **No new capability file, no new TauRPC procedure, no permission addition.** One window label joins an
  existing grant, so `EXPECTED_PROCEDURES` / `capability-drift` / the emit-bindings test are all untouched.
- **`pulse:notification` / `pulse:tray` / `pulse:plugin-fs` are not touched** — `capability-widening-check`
  guards those three and must stay green; `clipboard` is not one of them.
- **The second clipboard surface is out of scope — for a stronger reason than the scope guessed.**
  *[premise-corrected: the window question is moot. `InvestigationModalForm` mounts in BOTH `main`
  (`Dashboard.tsx:31`) and `compact-widget` (`CompactWidget.tsx:130`), but neither matters, because that
  path does not use the webview clipboard plugin at all: it calls `snapshot.generate`, and the clipboard
  write happens in RUST via `AppHandle::clipboard().write_text(...)` (`snapshot_runtime.rs:254-255`),
  which the webview capability ACL does not gate. Its "Snapshot copied to clipboard" message is
  therefore truthful and unaffected by this chunk.]* That path is also the in-repo precedent for a
  clipboard-outcome obs record — `snapshot.clipboard.write {byte_count, success,
  dual_file_paths_basenames}` with an exact allowlist leaf at `observability.rs:1159-1165` — which is
  the shape scope item 4's fork would follow if the machine-readable half is bought.
- **Report CONTENT quality is out of scope.** The 3B Timeline self-incoherence nuance recorded at
  `2026-08-26-interpretation-brief-completeness` is a separate concern; this chunk moves bytes, it does not
  judge them.
- **No route reordering, no other markerless entry absorbed.**

## Folded annotations

**PREREQ (from the working entry): re-check `cargo audit`.** Standing deferral since
`2026-08-15-corpus-key-persistence`; ratified pin **#16**, every-3rd-wrap re-run INTERVAL, re-pinned onto this
entry from `2026-08-27-idle-observer-generation-damper` with origin preserved.
- Basis: upstream RustSec DB duplicate-id parse error (`RUSTSEC-2026-0244`), external decay.
- Overlap signal: `cargo deny check advisories`, designed-red at the same eight owned IDs
  0189 / 0190 / 0194 / 0195 / 0204 / 0222 / 0253 / 0258; `cargo deny check bans licenses sources` exit 0.
- **Interval position: session 49 RAN the probe full-form (fifth consecutive identical 8-DISTINCT-id set);
  the next interval point is 52.** This chunk's wrap is session **50** — a BETWEEN-point session. The owed
  action is therefore: re-verify basis + overlap and record `probe skipped per ratified interval (next: 52)`
  in the chunk report. Not a full probe. Never a silent skip.
- Re-derive the owned-ID count first-hand at each probe (it read 7 at session 28, 8 at session 40) and count
  DISTINCT `RUSTSEC-` ids, never error blocks (10 blocks for 8 ids at session 43).

No `CARRY:` and no `BLOCKED-ON:` annotation on this entry (scanned at promotion — `BLOCKED-ON` absent).

## Why this is worth a chunk

A user-visible broken control on the flagship report surface, hit by the operator personally, with a
one-line fix — and a gate gap that let it live for weeks. Placement ahead of `Ingest consumer initiating
freeze` was the route-resolve call under operator delegation (2026-08-27): initiating-freeze waits on its
armed detector by design; this does not.
