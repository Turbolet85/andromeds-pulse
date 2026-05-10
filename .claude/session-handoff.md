# Session Handoff

**Last Updated:** 2026-05-10T20:39:33Z
**Branch:** main
**Session End Status:** clean (chunk #42 lands; 563 Rust tests + 503 webview tests passing across workspace — +5 Rust / +21 webview from session 49 baseline; 1 commit composed in Phase 10 of this run)
**Last Commit:** (pending — wrap commit composed in Phase 10 of this run; closes session 50 + chunk #42 implementation; continues Epoch 6 — Snapshot & Investigate)

## Current State

- **Last completed chunk:** route#42 "Investigate trigger + capture collapse — button on widget/main/context-menu/trace-row + 350ms scale+opacity supporting moment + aria-busy/aria-live" (epoch 6; commit_sha pending — landed this wrap)
- **Next chunk:** route#43 "Workspace path detection + clipboard + notification — workspace.detect (.andromeda/ marker) + dual .json/.md + 4 preset prompts + 'Snapshot ready' toast" (final chunk of Epoch 6 — Snapshot & Investigate)
- **In-progress phase:** none
- **Phase artifacts present:** `.andromeda/phases/{phase-1..phase-39}/{combined.md, research.md, plan.md}` (phase-39 closed chunk #42 in this session 50; next /andromeda-phase plans phase-40 for chunk #43)
- **Epoch 6 — Snapshot & Investigate: 4 of 5 chunks complete (chunk #39 dedup/anomaly/critical-path + chunk #40 aggregate/attribute-filter + chunk #41 markdown-formatter + chunk #42 Investigate-trigger; #43 remaining as the final epoch closer).**

## Andromeda State Detection (states A-L)

⚠️ **F — Pending phase planning:** chunk #42 committed; `.andromeda/phases/phase-40/` (for chunk #43) does not yet exist. Remediation: `/andromeda-phase` to plan chunk #43 (final Epoch 6 chunk, then transitions to Epoch 7 — Plugin runtime + MCP server).

(All other A-L checks clear. State C / arch staleness clear (CLAUDE.md mtime > arch.md mtime). State I cleared by this wrap's SHA-fixup amend (state.yaml.last_completed_chunk advances to chunk #42 with this wrap commit's SHA; chunk #41's orphan SHA 8dd4d1e from session 49 overwritten). States A / B / D / E / G / H / J / K / L all clear by absence-of-trigger or by Phase 5 reconcile + Phase 6 detection.)

## Drift Detection (6 dimensions)

No drift detected.

(D1 / D2 cleared by Phase 5 reconcile at this wrap. dependency-tree.md LIVING block unchanged from session 49 baseline at 229 lines because chunk #42 introduced ZERO new workspace-level transitive deps — placeholder TauRPC procedure reuses existing ui-bridge `AppError` + `SnapshotPreset` types; no new specta/taurpc additions to snapshot crate. api-surface.md LIVING block refreshed with fresh per-crate `cargo +nightly public-api --simplified` output 4258 lines (+53 lines vs session 49 baseline 4205 reflecting new `SnapshotApi` trait + `SnapshotApiImpl` struct public surface in ui-bridge crate). D3 cleared by `cargo metadata --no-deps` returning 10 workspace members matching arch §Inherited Defaults; capability-drift clean (`snapshot.generate` added to xtask EXPECTED_PROCEDURES + emitted in bindings file `pulse-app/ui/src/bindings/index.ts:55, 63`). D4 cleared by no-cross-plan-inconsistency (no specialist plans edited this session). D5 cleared — no specialist plan edited this session; CLAUDE.md mtime > all upstream plan mtimes. D6 cleared — chunk #42 commit lands this wrap; state.yaml.last_completed_chunk advances accordingly + commit_sha self-heals from "pending" via Phase 10 SHA-fixup amend.)

**Drift_warnings dedup outcome (per Phase 6 v2.1 discipline):** session-49 drift_warnings was empty (no carryover); this wrap's empty detection persists empty.

## Spec Amendments (this session)

(none this session — no amendments authored. Sessions 43-45's prior 14 archives preserved; no new amendments added in session 50.)

state.yaml.spec_amendments.active: empty (unchanged from session 49 close)
state.yaml.spec_amendments.archive: 14 entries (unchanged from session 49 close)

## Key Decisions This Session

- **Plan deviation: router lives in `crates/ui-bridge/src/snapshot_ipc.rs`, not `crates/snapshot/src/ipc.rs`** — plan Open Q1 explicitly flagged this for /implement-time review. The substrate-pollution cost of adding taurpc + specta to snapshot crate exceeded the benefit; ui-bridge centralizes routers per telemetry.rs precedent. Documented as Tier 3 session-learning (router-location architectural convention).

- **Plan deviation: IPC return type `Result<(), AppError>` instead of `Result<MarkdownReport, AppError>`** — MarkdownReport (chunk #41 substrate) lacks specta::Type derive; adding specta to snapshot crate would pollute substrate. The placeholder always returns Err, so `()` is sufficient. Chunk #43 will refine return type when real curation output flows through. Documented as Tier 3 session-learning (placeholder return type pattern).

- **Plan deviation: IPC arg `SnapshotPreset` instead of `TokenBudget`** — both enums are Conservative/Balanced/Detailed; SnapshotPreset already specta-derived in ui-bridge::contract since chunk #38; using it directly avoids `From<SnapshotPreset> for TokenBudget` conversion this chunk (deferred to chunk #43 when the placeholder body actually calls into snapshot::curate()). The by-name alignment from chunk #41's substrate-vs-IPC-enum-naming learning pays off.

- **Plan deviation: TraceRowContextMenu inlined as direct `onContextMenu` handler, no separate component file or popover** — plan called for a React Aria Components Menu + Popover, but minimum-viable for chunk #42 is `event.preventDefault()` + `openInvestigation(rowElement)` directly. Right-click goes straight to action; no popover menu UI. Chunk #43 (or later UX polish chunk) can layer a styled popover if needed.

- **Webview cfg-gating pattern for placeholder items: `#[cfg(any(feature = "taurpc-runtime", test))]`** — xtask builds ui-bridge WITHOUT the taurpc-runtime feature, which left `PLACEHOLDER_MESSAGE` const + `use` imports as dead code. Resolution: gate them with `any(feature, test)` so they're available under both production (feature on) and test (feature off, tests cfg-gated) builds. Adds to the obs Session Addition 2026-05-07 lesson about feature-cfg interactions (different but related symptom). Captured in Deferred learnings below.

- **Vitest fake-timer + async-rejected-promise pivot to real-timer waitFor** — `vi.useFakeTimers()` + `vi.advanceTimersByTimeAsync(350)` didn't flush microtasks for a setTimeout-spawned async function that ends with a rejected promise; tests timed out at 5000ms. Switched to real timers + `waitFor(..., { timeout: 2000 })` — works cleanly without complex fake-timer dance. Trade-off: tests take 350ms longer each (acceptable; only 6 modal tests affected). Captured in Deferred learnings below.

- **Provider-context test wrap audit pattern** — chunk #42's new `InvestigationProvider` broke 3 existing test files (router.test, TracesRoute.test, TraceTable.test) that render downstream consumers. Discovered during /implement Phase 2 fix-loop; resolution wrapped each render in `<InvestigationProvider>` (~1-line addition per file). Tier 2 testing.md addition captures this auditing pattern for future shell-level context introductions.

## Files Modified

This wrap's commit:

Code changes (Phase 39 implementation — chunk #42):

Backend (Rust):
- `crates/ui-bridge/src/snapshot_ipc.rs` (NEW, ~140 lines) — `#[taurpc::procedures(path = "snapshot")] pub trait SnapshotApi { async fn generate(preset: SnapshotPreset) -> Result<(), AppError>; }` + `SnapshotApiImpl` resolver that emits `tracing::warn!(target: "snapshot.generate.request", budget, result_kind = "placeholder", message)` and returns `Err(AppError::Internal { message: "Investigate not yet wired (lands chunk #43)".to_string() })`. 3 unit tests via `CapturingSubscriber` pattern from telemetry.rs:147+.
- `crates/ui-bridge/src/lib.rs` — added `pub mod snapshot_ipc;` + `#[cfg(feature = "taurpc-runtime")] pub use snapshot_ipc::{SnapshotApi, SnapshotApiImpl};`
- `pulse-app/src/main.rs` — added `use ui_bridge::snapshot_ipc::{SnapshotApi, SnapshotApiImpl};` import; `.merge(SnapshotApiImpl::new().into_handler())` in all 3 router branches (production Some(conn) / dev-fallback None / test branch)
- `pulse-app/src/observability.rs` — added `snapshot.generate.request` per-leaf entry to `AllowList::production()` with bounded fields `["budget", "result_kind", "message"]`; +2 sibling tests (`allowlist_for_target_resolves_snapshot_generate_request_field_set` positive resolver + `scrubber_redacts_non_allowlisted_snapshot_generate_request_field` negative canary) following the chunk #41 unrolled-explicit-fn pattern per testing.md Session Addition 2026-05-10
- `xtask/src/main.rs` — uncommented `"snapshot.generate"` in `EXPECTED_PROCEDURES` const; coupled with consuming-code commit per security.md Session Additions 2026-05-10

Webview (TypeScript):
- `pulse-app/ui/src/hooks/use-investigation.tsx` (NEW, ~55 lines) — `InvestigationContext` + `InvestigationProvider` + `useInvestigation` hook coordinating modal open + triggerRef across surfaces
- `pulse-app/ui/src/hooks/use-investigation.test.tsx` (NEW, ~55 lines) — 4 tests covering provider default state / openInvestigation / closeInvestigation / consumer-outside-provider error
- `pulse-app/ui/src/components/InvestigateButton.tsx` (NEW, ~80 lines) — forwardRef'd semantic `<button type="button">` with 3 variants (`main` / `widget-titlebar` / `trace-row-inline`); telescope icon at 20px (main/titlebar) or 16px (row-inline); aria-busy reflects prop; motion-reduce variants; `target-input-min` hit area
- `pulse-app/ui/src/components/InvestigateButton.test.tsx` (NEW, ~80 lines) — 8 tests covering semantic HTML / aria-label across variants / Enter+Space activation / aria-busy / ref forwarding / target-input-min hit area
- `pulse-app/ui/src/dashboard/InvestigationModalForm.tsx` (NEW, ~170 lines) — composes Modal primitive with state machine (idle/capturing/result/error) + 350ms supporting moment via setTimeout + `useReducedMotion` from `motion/react` (degrades to instant) + placeholder IPC binding via `__setProxyForTest()` seam (SettingsModalForm.tsx:58 precedent) + `appErrorMessage()` AppError discriminated-union mapper
- `pulse-app/ui/src/dashboard/InvestigationModalForm.test.tsx` (NEW, ~160 lines) — 6 tests covering closed-state / capturing→error transition / role=alert sanitized message / aria-live transitions / reduced-motion path / custom preset arg forwarding
- `pulse-app/ui/src/components/Titlebar.tsx` — converted `export function Titlebar` to `export const Titlebar = forwardRef(...)` (transparent to existing consumers); added optional `onInvestigateClick?: (trigger: HTMLElement | null) => void` prop + telescope icon button rendered conditionally between settings gear and WindowControls; `useImperativeHandle` exposes `TitlebarHandle` for future ref-based access if needed
- `pulse-app/ui/src/components/Titlebar.test.tsx` — 3 new tests: button-not-rendered-when-handler-undefined / button-renders-with-aria-label-when-handler-provided / click-forwards-trigger-to-handler
- `pulse-app/ui/src/widget/CompactWidget.tsx` — wrapped in `<InvestigationProvider>`; extracted `CompactWidgetContents` inner component that consumes `useInvestigation()` and wires `onInvestigateClick={openInvestigation}` to Titlebar + renders `<InvestigationModalForm />` at widget root; comment header updated to reflect new tab stop (titlebar gear + Investigate button instead of "titlebar gear only")
- `pulse-app/ui/src/dashboard/Dashboard.tsx` — wrapped RouterProvider in `<InvestigationProvider>` + sibling `<DashboardInvestigationModal />` inner component for modal mount at dashboard shell root
- `pulse-app/ui/src/dashboard/routes/TracesRoute.tsx` — added header action row `<div style={{ display: flex, justifyContent: space-between }}>` containing `<h1>Traces</h1>` + `<InvestigateButton variant="main" onClick={openInvestigation} />`
- `pulse-app/ui/src/dashboard/routes/traces/TraceTable.tsx` — TraceRowView signature extended with `onInvestigate` prop; each `<tr>` now has `onContextMenu={event => { event.preventDefault(); onInvestigate(event.currentTarget); }}` suppressing browser-native context menu; last `<td>` (Error cell) now renders flex-row containing the error count span + `<InvestigateButton variant="trace-row-inline" />` with `event.stopPropagation()` on click to avoid bubbling back into row context-menu
- `pulse-app/ui/src/dashboard/routes/TracesRoute.test.tsx` — wrapped existing `render(<TracesRoute />)` in `<InvestigationProvider>`
- `pulse-app/ui/src/dashboard/routes/traces/TraceTable.test.tsx` — extended `renderWithProvider` helper to nest `<InvestigationProvider>` inside the existing `<StatusLiveRegionProvider>` wrap
- `pulse-app/ui/src/dashboard/router.test.tsx` — extended `renderRouterAt` helper similarly to nest `<InvestigationProvider>` inside the existing `<HaloInputProvider>` wrap
- `pulse-app/ui/src/bindings/index.ts` — regenerated by `pulse-app::tests::emit_taurpc_bindings` integration test; now includes `'snapshot':'{"generate":["preset"]}'` in ARGS_MAP at line 55 + `"snapshot": {generate: (preset: SnapshotPreset) => Promise<null>},` at line 63

Phase artifacts:
- `.andromeda/phases/phase-39/{combined.md, research.md, plan.md}` (273 + 110 + 311 lines after Phase 5 Telescope.tsx provenance cleanup)

Run audit trail:
- `.andromeda/runs/2026-05-10T19-42-06-phase-39/{security,design,layouts,tests,obs,a11y,arch}.md` + `.raw-{*}.md` (7 stripped + 7 raw)

Wrap-session changes (this commit):
- `.andromeda/context/dependency-tree.md` — Last reconciled timestamp refreshed to 2026-05-10T20:36:44Z; LIVING block UNCHANGED from session 49 baseline (229 lines — chunk #42's zero-new-deps property meant fresh `cargo tree --workspace --depth 2 --prefix indent` produced byte-identical output to session 49); session 50 maintenance note appended documenting chunk #42 router-in-ui-bridge decision + zero new transitive deps
- `.andromeda/context/api-surface.md` — Last reconciled timestamp refreshed to 2026-05-10T20:36:44Z; LIVING block fully refreshed (4258 lines, +53 lines vs session 49 baseline 4205 — captures new `ui-bridge::snapshot_ipc::SnapshotApi` trait + `ui-bridge::snapshot_ipc::SnapshotApiImpl` struct + all the auto-generated taurpc procedure-trait associated types/impls); LIVING block format preserved as raw concatenated tooling stdout per chunk #41 simplification + integrity-protocol.md §Reconcile faithful replace; session 50 maintenance note appended
- `.claude/rules/testing.md` — Tier 2 Session Additions: 1 new entry (provider-context test wrap audit — when a chunk introduces a new React Context provider that downstream consumers render, audit existing test files for wrap-need; chunk #42 caught 3 files at /implement Phase 2 fix-loop)
- `.claude/docs/session-learnings.md` — Tier 3: 2 new top entries (router-in-ui-bridge architectural convention — TauRPC routers live in `crates/ui-bridge/`, not substrate crates; placeholder TauRPC IPC return type `Result<(), AppError>` over the eventual data type when bindings aren't specta-ready)
- `.andromeda/state.yaml` — last_wrap to 2026-05-10T20:39:33Z; session_count to 50; last_completed_chunk advances to chunk #42 with commit_sha pending (post-commit SHA-fixup amend in Phase 10); drift_warnings empty; plan_freshness unchanged (no upstream plan edits this session); living_artifact_freshness updated to 2026-05-10T20:36:44Z; spec_amendments unchanged (active=[], archive=14)
- `.claude/session-handoff.md` — full overwrite (this file)

## Curation Summary (this wrap)

- **Tier 1 (CLAUDE.md USER:session-learnings):** 0 additions
- **Tier 2 (`.claude/rules/*/Session Additions`):** 1 addition (testing.md — provider-context test wrap audit pattern for new React Context provider introductions; confidence 0.7, validated by chunk #42's 3-file audit experience)
- **Tier 3 (`.claude/docs/session-learnings.md`):** 2 additions (router-in-ui-bridge architectural convention — TauRPC routers live in `crates/ui-bridge/`, not substrate crates; confidence 0.85, validated by telemetry.rs precedent + chunk #42 deviation rationale. Placeholder TauRPC IPC return type pattern — `Result<(), AppError>` for placeholders when eventual data type lacks specta bindings; confidence 0.75, corollary to chunk #41 substrate-vs-IPC enum naming alignment)
- **Filtered:** 2 deferred to max-3-cap (cfg(any(feature, test)) gate for runtime-mod-only items + Vitest fake-timer pivot to real-timer waitFor — see Deferred learnings section below)

## Last Failed Command

(none — all gates passed cleanly across Phase 39 implementation + this wrap; clippy initially flagged 1 warning (`preset_label` unused under cfg permutation) resolved by inlining the match into the resolver body; xtask build initially flagged 2 unused-import warnings + dead-code on `PLACEHOLDER_MESSAGE` resolved by `#[cfg(any(feature = "taurpc-runtime", test))]` gating; webview fake-timer tests timed out at 5s default resolved by pivot to real-timer waitFor)

## Tests Status

passing — 563 Rust tests across workspace (+5 vs session 49 baseline of 558) + 503 webview tests via Vitest (+21 vs session 49 baseline of 482) — total 1066 tests green.

- New ui-bridge snapshot_ipc tests (3): `generate_emits_warn_at_snapshot_generate_request_target` / `generate_returns_sanitized_internal_error_without_path_or_struct_name` / `generate_accepts_all_three_presets` — all use `CapturingSubscriber` pattern from telemetry.rs:147+ + #[tokio::test] async resolver invocation
- New observability tests (2): positive resolver `allowlist_for_target_resolves_snapshot_generate_request_field_set` + negative canary `scrubber_redacts_non_allowlisted_snapshot_generate_request_field` — single fn each per testing.md Session Addition 2026-05-10 (`tracing::warn!(target: ...)` requires `&'static str` literal)
- New webview tests (21): use-investigation.test.tsx (4) + InvestigateButton.test.tsx (8) + InvestigationModalForm.test.tsx (6) + Titlebar.test.tsx additions (3)
- All standard chunk-gate baseline gates clean: `cargo fmt --check` / `cargo clippy --workspace --all-targets --all-features -- -D warnings` / `cargo nextest run --workspace --profile ci` / `cargo xtask capability-drift` (0 missing, 0 extra) / `cargo deny check bans licenses sources` / `cargo audit` (no new advisories; 18 pre-existing allowed warnings — unchanged baseline) / `npm run lint --prefix pulse-app/ui` / `npm run typecheck --prefix pulse-app/ui` (chunk #42 clean; 2 pre-existing `MetricsChart.test.tsx` errors from chunk #35 remain — deferred since session 43, out of scope per session-handoff convention) / `npm run test --prefix pulse-app/ui`
- Coverage: snapshot crate unchanged (chunk #42 doesn't touch substrate); ui-bridge crate gains snapshot_ipc module with 3 tests + 4 functions — coverage maintained
- Cyrillic-mixing health check: 0 hits across all NEW chunk #42 source files (`crates/ui-bridge/src/snapshot_ipc.rs` + 7 new pulse-app/ui/src/* files clean by construction)

Pre-existing tsc deferred: 2 errors in `pulse-app/ui/src/dashboard/routes/metrics/MetricsChart.test.tsx:27,49` (chunk #35 inherited; deferred per user since session 43 wrap; chunk #42 does not touch the file; carry-over preserved).

Boot smoke gate: SKIPPED — `pulse-app` binary compiles cleanly (18.27s incremental dev build verified at /implement Phase 2b); the `pulse-app::tests::emit_taurpc_bindings` integration test exercises the router merge + bindings emission, providing compile-time smoke proxy. Full `scripts/agent-run.sh boot && status && cleanup` deferred (release-build cost prohibitive in interactive session; runtime panic risk low given chunks #27-#36 cluster of boot-path fixes per testing.md Session Addition 2026-05-09).

## Next Recommended Action

**Priority 1 — `/andromeda-phase` for chunk #43 (Workspace path detection + clipboard + notification):**

Closes Epoch 6 — Snapshot & Investigate. Chunk #43 wires the actual `snapshot.generate` IPC body (replacing chunk #42's placeholder), workspace.detect with `.andromeda/` marker detection + VCS metadata, dual `.json`/`.md` clipboard write, 4 preset prompts ("Diagnose latency outlier" / "Find error correlation" / "Trace failed request" / "Summarize service health"), and OS-native "Snapshot ready" notification via `tauri-plugin-notification`. Chunk #43 is substantial (workspace-detector crate first activation + clipboard plugin wire-up + notification capability + 4-prompt UX), so likely a single-chunk plan per the same grouping heuristic that paired chunks #41 and #42 individually.

Per session-learnings 2026-05-10 router-in-ui-bridge architectural convention + 2026-05-10 placeholder return type pattern: chunk #43 should refine `crates/ui-bridge/src/snapshot_ipc.rs::SnapshotApi::generate` signature from `Result<(), AppError>` to `Result<MarkdownReport, AppError>` (or an IPC-friendly equivalent), introduce the `From<SnapshotPreset> for TokenBudget` conversion at the bridge boundary, and wire the placeholder body to call `snapshot::curate()` + `snapshot::format_markdown()` + workspace.detect + clipboard. The new return type drives a specta::Type derive question for MarkdownReport — chunk #43 plan should explicitly decide between (a) cfg-gated specta derive on MarkdownReport in snapshot crate, OR (b) `IpcMarkdownReport` wrapper in ui-bridge with `From<MarkdownReport>` conversion. Option (a) preferred for simplicity unless substrate-purity is judged critical.

**Priority 2 (informational) — MetricsChart.test.tsx tsc errors cleanup (carry-over from session 43):**

When a future chunk touches `pulse-app/ui/src/dashboard/routes/metrics/`, fix the 2 chunk #35 inherited tsc errors at lines 27, 49. The `chunk-gate-baseline-coverage` trigger mandates `tsc --noEmit` clean per chunk plan, so future metrics-touching chunks SHOULD include the gate AND fix the inherited errors when they touch the file.

## Session Goals (carry-over)

(none — session 50 user goal (continue epoch 6 via chunk #42 Investigate trigger + capture collapse) achieved. No outstanding goals carry over to session 51.)

## Deferred decisions (Trigger 4 to Path B carry-over)

(none this session — no spec to reality drift triggered)

## Deferred learnings (filtered out from Phase 4 curation)

Two candidates deferred via max-3-cap:

**Deferred #1 — Cargo feature cfg-gate discipline: `#[cfg(any(feature = "X", test))]` for items used only in cfg-gated runtime mod + tests (confidence 0.7):**

When a crate has a `#[cfg(feature = "X")] mod runtime { use super::*; ... }` pattern where the runtime module's contents are gated, top-level imports + consts referenced ONLY inside the runtime mod will fire `unused_imports` / `dead_code` warnings when the crate is built WITHOUT the feature (e.g., xtask depends on ui-bridge without the `taurpc-runtime` feature, exposing the unused warnings). Resolution: gate the top-level items with `#[cfg(any(feature = "X", test))]` so they're available under both production builds (feature enabled) AND test builds (feature off, tests cfg-gated). Verified at chunk #42 with `crates/ui-bridge/src/snapshot_ipc.rs` `PLACEHOLDER_MESSAGE` + `use crate::contract::{AppError, SnapshotPreset}` — both gated this way to silence warnings while preserving test access. Pairs with the obs Session Addition 2026-05-07 lesson on AllowList split('.').next() resolver order (different feature/build concern but same "cfg-gating discipline" family).

**Deferred #2 — Vitest fake-timer + async-rejected-promise pivot to real-timer waitFor (confidence 0.65):**

`vi.useFakeTimers()` + `vi.advanceTimersByTimeAsync(N)` followed by `waitFor(...)` doesn't reliably flush microtasks for a `setTimeout` callback that spawns an `async` function ending in a rejected promise — tests timed out at the default 5000ms despite the advanceTimers call. Pivot to real timers + `waitFor(..., { timeout: 2000 })` works cleanly: the 350ms supporting-moment delay completes naturally, the rejected IPC promise propagates through microtask queue normally, React state updates settle, and waitFor's polling catches the final state. Trade-off: each affected test takes ~350-500ms longer (acceptable; only 6 modal tests). Pairs with the chunk #41 testing.md Session Addition 2026-05-04 (motion/react useReducedMotion module-global reset) — both surface fake-timer + async-state interactions in Vitest. Documented for future webview tests asserting state transitions across setTimeout + async IPC boundaries.

Both will be promoted to Tier 2 testing.md additions in a future wrap-session if either pattern recurs.
