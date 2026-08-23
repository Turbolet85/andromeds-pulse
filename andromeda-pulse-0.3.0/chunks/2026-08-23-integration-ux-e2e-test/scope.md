# Scope — 2026-08-23-integration-ux-e2e-test

**Working entry (verbatim intent):** Integration UX e2e test — real assembled path under
deterministic-L4 (launch, telemetry, real-time push, Traces, storm, incident, Investigate) guards
regressions (P-076 · intent F15).

**Claims:** P-076 · Integration UX e2e test (`verification-matrix.json#P-076`) — the first real claim
in six chunks; the cap has been pooled since `2026-08-15-corpus-key-persistence` declined it.

---

## 1. What this chunk builds

A **scripted multi-stage headful e2e** that drives the REAL assembled product path in the live Tauri
window under deterministic-L4, asserting at each stage against an observable the driver can read.

Today the headful leg is **one press**: `cargo xtask webview-drive` → `tests-e2e/webview-drive.mjs`
clicks one control (`button[aria-label="Close to tray"]`) and the xtask caller owns the verdict by
reading ONE obs record family (`ui.layout.transition`). Verified at HEAD: `webview-drive.mjs` (200
lines) and `xtask/src/webview_drive.rs` (297 lines, 9 unit tests), landed at
`2026-08-23-webview-self-verify`.

This chunk turns that one-press leg into a **staged path**. The existing script already carries the
substrate the staged path needs — tauri-driver spawn + port poll, WebdriverIO session, multi-window
handle enumeration (four surfaces: compact-widget / main / findings / report), obs-log tailing, and
the `pollUntil` bounded-poll discipline that replaced `sleep(N)`. The delta is **stages and
assertions**, not a new driver stack.

### The stages the cap names

`launch → telemetry → real-time push → Traces render → storm → incident → Investigate`

### The three acceptance assertions, and the observable each binds to

P-076's acceptance asserts **Traces render · incident creation · an Investigate result**. The
claim-exit invariant requires each bound to an observable the driver can actually read. Three
observable classes are available, and the binding is **research's call** (P3), not assumed here:

**[VERIFIED at P3 — each binding resolved, field-by-field, and none needs a new allowlist leaf]**

| # | Assertion | Bound observable | Allowlist resolution | Field |
|---|---|---|---|---|
| 1 | Traces render | `viz.query.traces` (`crates/viz/src/query.rs:184-192`) | prefix fallback → bare `"viz"` key (`observability.rs:358`); all 6 emitted fields admitted | `row_count` |
| 2 | Incident creation | `interpretation.incident.created` (`pulse-app/src/inference_runtime.rs:779-785`, a NON-test producer) | exact leaf (`observability.rs:2084`), 4/4 match | `created` |
| 3 | An Investigate result | `investigate.run_action.request` (`pulse-app/src/investigate_router.rs:157-164`) | exact leaf (`observability.rs:1946`), 5/5 match | `status` |

Each stage must fail the leg if it breaks — "the press returned without throwing" is not evidence
(the lesson the predecessor chunk's RED arm proved).

### Deterministic-L4 is the mode, and it CAN deliver this acceptance

Verified at HEAD, first-hand, not taken from the directive:

- `pulse-app/src/investigate_router.rs` honours the deterministic env-gated L4 mode — the doc
  comment at `:9`, the `deterministic_mode_enabled` import at `:25`, the `deterministic_mode: bool`
  field at `:153`, and the gate read at `:183` threaded through every emit arm to `:285`.
- The canned fixture carries **three** `evidence_refs` — `deterministic_inference.rs:68-71`
  (`det-span-9f2c4a7e1b6d0358` · `det-template-0007` ·
  `det-fingerprint-4a7f2b91c6e05d3849b1e7a2c5f08d63`).

So det-L4 is sufficient for the acceptance as written. CARRY #13 below names what det-L4 CANNOT
produce (`ResolutionSummary`) — a **different artifact the acceptance never names**, so it folds as
a scope boundary, never as a premise correction against the mode.

### The leg must RUN, not skip

`resolve_msedgedriver` (`xtask/src/webview_drive.rs:213`) returns `None` when
`ANDROMEDA_PULSE_MSEDGEDRIVER_PATH` is unset or not a file, and the leg then SKIPs clean at `:52`.
For a chunk whose acceptance IS the leg, **a clean skip is not an acceptable outcome**: the plan's
Test Commands must carry the env var in firing form, and the chunk's own gate evidence must come
from a run that actually drove the window.

---

## 2. Surfaces and contracts touched

- `pulse-app/ui/tests-e2e/webview-drive.mjs` — the driver script (extend, or a staged sibling).
- `xtask/src/webview_drive.rs` — the caller + verdict owner; `Cmd::WebviewDrive { expect_absent }`
  is the current CLI surface (`xtask/src/main.rs:114`, dispatched at `:177`).
- `crates/ingest/examples/inject_demo.rs` — the telemetry source for the path. **Measured
  correction:** intent F15 §Housekeeping calls it "currently untracked"; it IS tracked at HEAD, as
  are three sibling injectors (`inject_colliding_logs` · `inject_colliding_metrics` ·
  `inject_scrub_canaries`) and `load_profiles`. **[VERIFIED at P3]** — no new/extended injector is
  needed: `inject_demo.rs` is purpose-built for this exact chain (its header documents warmup →
  payment-service degradation → fixed-fingerprint exception storm → RetryStorm cue → digest → L4 →
  incident) and speaks real OTLP gRPC, satisfying arch §Test-time telemetry injection. Consumed
  as-is; formalizing it stays P-077's job.
- The obs log (`<data_dir>/logs/agent-latest.jsonl*`) — the app's own record, the precedent verdict
  channel. **[VERIFIED at P3 — no new leaf needed]** all three observables already resolve under the
  shipped allowlist. The 2026-08-21 delegated-timing lesson was checked and does NOT bite here:
  assertion 1 does ride `for_target`'s prefix fallback, but the bare `"viz"` key is a complete field
  vocabulary (11 fields, superset of the 6 emitted) rather than the `value`-only stub the bare
  `metric` key is.
- `ANDROMEDA_PULSE_L4_DETERMINISTIC` (P-073 gate) · `ANDROMEDA_PULSE_DATA_DIR` ·
  `ANDROMEDA_PULSE_MSEDGEDRIVER_PATH`.
- DOM selectors on the Traces / constellation / findings / Investigate surfaces. **[premise-corrected
  at P3: the selector convention is MIXED, not uniformly accessible-name]** — accessible names ship
  on the Investigate button (`Titlebar.tsx:128`), the per-row Investigate control
  (`TraceTable.tsx:314`) and `ConnectionDot` (`role="img"`), but the Traces table, `EmptyState` and
  the footer expose only `data-testid`. a11y-plan §1 P1's `region[aria-label="Telemetry traces
  chart"]` + `table` does NOT exist at HEAD — the same phantom-selector class as the
  `Minimize to tray` case. Resolution is a P4 operator decision.

**Expected NON-touches** — **[VERIFIED at P3]**: no new TauRPC procedure (so no `EXPECTED_PROCEDURES`
pin, no capability JSON edit, no `emit_taurpc_bindings` merge); no new workspace or npm dependency
(WebdriverIO + tauri-driver are already devDeps); no corpus DDL; **and no new obs allowlist leaf**.
Confirmed additionally: `pulse-app/capabilities/*.json` stays untouched — `core:window:allow-close`
is already granted and the leg presses only existing controls.

---

## 3. Annotation disposition (13 CARRY · 1 PREREQ · 1 NOTE)

Counted mechanically off the working entry (10171 bytes): `· CARRY:` ×13, `· PREREQ:` ×1,
`· NOTE:` ×1. No `BLOCKED-ON` at an annotation position. Every one is disposed below — an entry this
long is exactly where a silent drop hides.

> **AMENDED at P5 validation-1 (intent-incomplete — the scope as first written was, and the
> divergence is justified).** This section originally ABSORBED five CARRYs. P3 measured the cost:
> five absorbed CARRYs plus three acceptance stages is a wide leg on what test-plan §10 calls the
> suite's highest flake-risk surface, and two of them (#11, #12) are open INVESTIGATIONS whose
> outcome is unknown until driven — not assertions with a known shape. Surfaced to the operator at
> P4, who chose to land the three acceptance stages plus only the two CARRYs that cost no extra
> driver mechanic. **Net disposition: 2 absorbed (#5, #7) · 3 re-pinned (#6, #11, #12) · 7 boundary
> · 1 re-pinned non-headful (#10).** The three re-pinned CARRYs are DEFERRED, never dropped — wrap's
> route-resolve carries them onto the follow-up entry. The per-CARRY table below is annotated
> accordingly.

**Disposition principle** — **[VERIFIED at P3: the principle holds; the absorbed/boundary split
survived research unchanged, and the absorbed set's BREADTH is now a P4 operator question given
test-plan §10's zero-flakiness budget on the suite's highest-risk surface]** — ABSORB what rides the SAME assembled path at no new driver
mechanic; BOUNDARY what needs a driver mechanic the path does not use (pointer drag, window resize,
native-menu observation, multi-window geometry, production build); RE-PIN what is not this leg's
job at all. P3 closes the feasibility premises; a contestable residue goes to the operator at P5.

### Absorbed — asserted by this chunk (#5, #7) · RE-PINNED at P5 (#6, #11, #12)

| # | CARRY | Why it rides this path |
|---|---|---|
| 5 | **P-081 Traces auto-refresh** — table populates empty→populated on its own, no manual reload; filter/sort + focus survive a refresh | This IS assertion 1 done honestly. The path already drives telemetry→Traces; asserting the transition rather than a static snapshot costs no new mechanic |
| 6 | **P-070 connection-status footer** — worded connected-source count + spans/s + buffer fill, updating on its own; count matches constellation live-dots | ~~Same DOM, same window, same live stream the path already establishes~~ → **RE-PINNED at P5.** Cheap in mechanic but adds a sixth stage to the flake surface for no acceptance coverage; deferred to the follow-up. Selector measured and ready for it: `[data-testid="connection-status-line"]` (`ConnectionStatusLine.tsx:110`) |
| 7 | **P-071 empty states (settled half)** — a traces-only stream leaves Metrics/Logs empty → worded message + `:4318`/`:4317` exporter hint + Observatory glyph | The path injects traces only; reaching /metrics + /logs is DOM navigation, no new mechanic. *The query-failure half (distinct error line, NO hint) needs fault injection → boundary* |
| 11 | **intake #5** — a second incident invisible to the data-dir workspace key | ~~The entry's own text places it here~~ → **RE-PINNED at P5.** It is an open INVESTIGATION, not an assertion with a known shape — its outcome is unknown until driven, so it cannot be scoped inside the chunk that must claim P-076. Deferred whole to the follow-up |
| 12 | **intake #12** — corpus/in-app active-list divergence after auto-resolve (read-back 2 active vs in-app 1) | ~~Same class as #11~~ → **RE-PINNED at P5**, same reason: an open investigation. Note the leg's incident stage does make the divergence *reachable* — `incidents.list_active.request` → `item_count` (exact leaf, `observability.rs:2242`) is the count-shaped observable it would key on when taken up |

### Boundary — explicitly NOT asserted here, owner named

| # | CARRY | Mechanic it needs · owner |
|---|---|---|
| 1 | **P-061 drag-delta** — window-position-delta > 0 from a real titlebar drag | Real pointer drag on a native drag region. Owner: a headful **window-mechanics** leg (see §4) |
| 2 | **P-062 resize-clamp + aspect-band** — clamp below min-size; aspect snaps into 1.4–2.1 on release | Real window resize. Owner: same window-mechanics leg |
| 3 | **P-064/P-065 chrome suppression** — no native WebView2 menu on right-click; canvas not save/drag-able | Observation of a **native** menu (outside the DOM, so outside WebDriver's reach — **[NOT closed at P3: reasoned from the WebDriver model, deliberately NOT empirically probed. Its future owner must probe rather than inherit this assumption]**) **and** a production build. Owner: same window-mechanics leg |
| 4 | **P-066 toggle + P-061/P-063 corrections** — in-widget `Toggle dashboard` click AND Cmd/Ctrl+Shift+P from both windows; widget boots at the margin-inset corner; dashboard ✕ → collapse-to-widget, widget ✕ → app-to-tray + the every-time "still running" toast | Global shortcut + tray + multi-window lifecycle. Owner: same window-mechanics leg. *(The widget ✕ → hidden half already ships as the predecessor's one-press leg)* |
| 8 | **P-082 internal-scroll layout** — table body scrolls internally with hero/toolbar/sticky-header fixed, NO outer page scrollbar, **across a couple window sizes** | The multi-size clause needs the resize mechanic → window-mechanics leg. **[VERIFIED at P3: the single-default-size half DOES absorb cheaply — `trace-table-scroll` is a shipped `data-testid` (`TraceTable.tsx:112`), so a no-outer-scrollbar + internal-scroll read costs no new mechanic. Folded into the P4 breadth question rather than decided here]** |
| 9 | **incidents floating-window disclosure** — badge click docks `findings` BELOW the widget sized to count; row-select opens `report`; Esc/blur/mark-all-read dismiss with cross-window focus restore to the badge | Multi-window **docking geometry** + cross-window focus. Owner: same window-mechanics leg |
| 13 | **det-L4 blind spots** — (a) `DigestKind::ResolutionSummary` unreachable under det-L4 (zero non-test constructors; fixture pins `is_resolution_summary: false` at `deterministic_inference.rs:76` — **verified exactly at HEAD**); (b) `EvidenceRefs.trace_id`/`span_ids`/`timestamps_unix_nano` empty in EVERY mode, and `degraded_mode = Report.degraded_mode = parsed_l4.is_none()` driven by Resolved-only persistence | **Per directive, folded as a boundary, not a gap.** These name artifacts P-076's acceptance never mentions. Both halves are the same class — an absence check over them passes for the WRONG reason — so pinning them belongs to a chunk that owns the absence, not to the chunk asserting presence |

### Re-pinned — not this leg's job

| # | CARRY | Disposition |
|---|---|---|
| 10 | **MCP cross-process read-back guard** — spawn `andromeda-pulse-mcp`, assert `retrieve_telemetry_slice.fingerprint_refs` equals the seeded refs + the Report Evidence section renders them; needs a NEW test file (`e2e_p3_mcp_incident_tools.rs` is in-process by construction) | The entry itself calls this **"the one non-headful CARRY in this set"**. It is a committed Rust integration test, not a headful driver stage. Already tracked as `mcp-incident-read-back-cross-process-coverage` in test-plan §1 — that trigger is its owner and survives this chunk untouched. **[VERIFIED at P3: the trigger IS live — `test-plan.md:124` carries the §1 row and `:465` the matching §6 P3 Current-residual line. Re-pin disposition holds]** |

### PREREQ + NOTE

- **PREREQ · `cargo audit` re-check (pin #11)** — CARRIED, and **auto-satisfied this session**.
  Standing deferral since `2026-08-15-corpus-key-persistence`; every-3rd-wrap INTERVAL. Probe points
  ran at sessions 25/28/31/34/37; **point 37 fired in full form and discharged**, so the next point
  is **40**. `state.yaml` reads `session_count: 38`, making this session 39 — **no probe owed**.
  Between interval points the obligation is to re-verify basis + overlap and record
  `probe skipped per ratified interval (next: 40)` in the chunk report — never a silent skip. The
  pin rides this entry and must be re-pinned onward at wrap if still undischarged.
- **NOTE · the older `cargo audit` re-pin record** — states the PREREQ moved AWAY to
  `Webview self-verify on the Windows host`. **Superseded**: that entry was promoted and wrapped at
  `2026-08-23-webview-self-verify`, and the wrap re-pinned the PREREQ back onto this entry (origin
  `2026-08-15-corpus-key-persistence` preserved). The NOTE is stale history; the PREREQ above is the
  live obligation. Directive confirms: "Pin #11 carried (next: 40)."

---

## 4. Boundaries (what this chunk does NOT do)

- **No window-mechanics leg.** Pointer drag, window resize, native-context-menu observation,
  global-shortcut/tray lifecycle, and multi-window docking geometry are all out. Seven boundary
  CARRYs (1, 2, 3, 4, 8, 9, and #13) accumulate here; if no existing route entry owns them, wrap's
  route-resolve should mint one rather than re-pinning seven annotations onto whatever entry is next.
  **[VERIFIED at P3: no existing markerless entry owns window mechanics — the nine remaining entries
  are Integration-UX-e2e / A11y verification / Halo canvas / Advisory backlog / npm advisory /
  Diagnostics un-muting / Staged-bindings / Metrics label surface / Demo injector. Minting is
  therefore the correct disposition, and it is wrap's call, not this chunk's]**
- **No new capability claim beyond P-076.** The absorbed CARRYs re-verify caps already `verified`
  (P-070/P-071/P-081/P-082) at higher fidelity; that raises confidence, it does not re-open them.
- **No MCP cross-process test file** (CARRY #10 — owned by its test-plan trigger).
- **No production-build leg** — **[VERIFIED at P3]** `locate_pulse_binary` resolves under
  `target/{release,debug}` (`xtask/src/webview_drive.rs:41`), so the leg drives whichever binary is
  already built; it neither forces nor asserts a production build.
- **No det-L4 absence pins** (CARRY #13).
- **No `cargo audit` probe** — interval-deferred, next point 40.
- **No route/spec edits** — phase is read-only on the specs; any spec↔reality gap found in P3
  surfaces at wrap's reconcile.

---

## 5. Open questions for P3/P4

1. **Observable binding per assertion** — DOM vs obs-record vs read-back, for each of the three
   acceptance assertions. Research owns this; it is the claim-exit invariant's substance.
2. **Storm stage mechanism** — does the existing `inject_demo.rs` (or a sibling injector) produce a
   storm that reaches incident creation under det-L4, or does the stage need a new/extended
   injector? Does that make `inject_demo` a shipped dependency of the gate (P-077 adjacency)?
3. **Boundary-CARRY feasibility** — is a native WebView2 context menu observable to WebDriver at
   all? If it is structurally unobservable, CARRY #3 is not "deferred", it is
   **not-provable-by-this-method** and needs its owner told so.
4. **Absorbed-set size** — five absorbed CARRYs plus three acceptance assertions is a wide leg. If
   P3 shows the absorbed set materially inflates the chunk, the fork (narrow to the three acceptance
   assertions vs the full absorbed set) goes to the operator at P5.
5. **New obs allowlist leaves** — does any chosen observable need one? (A bare parent key keeps
   `value` and silently redacts every label.)
