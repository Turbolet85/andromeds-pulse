# Codebase Research — 2026-08-30-acl-rejection-logging

## Scope
- **Depth:** deep · **Reads:** 13 files/sections · **Globs/Greps:** ~22 (incl. Tauri registry source)

## Files inspected
- `pulse-app/ui/src/report/Report.tsx` (full) — the CARRY defect CONFIRMED at HEAD: `ErrorState` (:98-119)
  sets `color: "var(--color-accent)"` on the container at `fontSize: 14px` body size, so the `<strong>`
  headline inherits accent; the `<span>` detail is already `--color-text-secondary`; the border is accent
  (correct to keep). `message` renders the hook's error string.
- `pulse-app/ui/src/report/use-report.ts` (full) — BOTH measured shapes hold at HEAD: `copyMarkdown`'s catch
  (:93) discards the rejection entirely (`catch { setCopyState("error") }` — no binding, no report); the
  LOAD catch (:57-64) renders `err.message` only when `err instanceof Error`, else the static
  "Failed to load report" (an invoke rejection is typically a string/object → static in practice).
- `crates/ui-bridge/src/telemetry.rs` (:1-300) — the frontend-arm pattern in full: bounded serde enums with
  `as_str()`, per-field `validate_*` → `AppError::Validation`, `#[taurpc::procedures(path =
  "telemetry.frontend")]` trait with 4 methods, `#[taurpc::resolvers]` impl emitting level-gated
  `tracing::info!`, colocated `CapturingSubscriber` tests that RUN (library crate — no `[lib] test = false`).
- `pulse-app/src/observability.rs` (leaf region ~:990-1075 + probes) — exact leaves exist for
  `ui.layout.transition` / `tray.signpost.shown` / `app.boot.window.navigation` /
  `metric.findings.counter_refresh_ms`; **NO ACL/IPC-rejection leaf or emitter exists anywhere in the Rust
  tree** (grep: 0 hits) — the "nothing logs them today" premise VERIFIED. Bare-key probe (two-line window per
  observability.md 2026-08-21, control `buffer`→1): `ui`→0, `telemetry`→0, `app`→0 — a `ui.*` target without
  its own exact leaf resolves to NOTHING and every field redacts (shape (a)).
- `pulse-app/src/window.rs` (:107-120, :589-595) — `sanitize_window_label` exists (PRIVATE `fn`; bounded
  `main`/`compact-widget`/`findings`/`report` + `unknown`); all callers in `window.rs`. `ui-bridge` cannot
  reach it (DAG roots at pulse-app), so the resolver needs its OWN bounded label coercion mirroring the set —
  the third bounded copy, exactly how the TS side already mirrors it (`use-window-label.ts`, frontend.md
  2026-05-09).
- `pulse-app/ui/tests-e2e/webview-drive.mjs` (:54, :955-1035) — `report-copy` stage is DOM-only
  (`data-copy-state` + live region). Its FAILURE-ONLY probe runs a raw `__TAURI_INTERNALS__.invoke` **inside
  the webview** and catches the rejection — the layer question answered by that chunk's own instrument.
- `xtask/src/main.rs` (:1116-1163, :1531) — `EXPECTED_PROCEDURES` with the four `telemetry.frontend.*` pins;
  xtask's own test asserts pin↔bindings set equality, so a new pin is auto-covered.
- `pulse-app/src/main.rs` (:43, :1006, :1033, :2003) — `TelemetryApiImpl` is ALREADY merged at both
  production router branches AND in `emit_taurpc_bindings` — **a new method on the existing trait needs NO
  `main.rs` edit** (the 2026-06-01 already-exists class: the plan must not add wiring that already ships).
- **Tauri 2.11.0 source** (lockfile version; `D:/dev/rust/cargo/registry/src/.../tauri-2.11.0/`):
  - `src/webview/mod.rs:1820-1852` — THE denial site. Release builds reject with
    `format!("Command {} not allowed by ACL", request.cmd)`; debug builds reject with
    `resolve_access_message(...)`. The rejection flows ONLY through `invoke.resolver.reject(...)` back to
    the webview — **no log, no event, no app-visible hook fires on the Rust side** (the `tracing::error!` at
    :1750 is the invoke-key mismatch path, feature-gated, unrelated).
  - `src/app.rs:1677` — `Builder::invoke_system(initialization_script)` takes ONLY a script in 2.11; no
    `invoke_responder` / `on_invoke` builder hook exists anywhere in the crate (grep: 0 hits).
  - `src/ipc/authority.rs:229-275` — the debug message variants all carry the substring `not allowed`
    ("… not allowed. …", "command not allowed on any window/webview/URL context"), so ONE substring covers
    both build profiles for a classifier.
- `pulse-app/ui/tests-a11y/axe/p9-*.spec.ts` — `p9-report-copy-states.spec.ts` drives the COPY states via
  `__mockReject`; `p9-diagnostic-report-modal.spec.ts` has ZERO `__mockReject`/`get_report`/error hits — the
  LOAD-error path (`ErrorState`) is NOT rendered by any a11y spec today (the a11y extract's open question,
  answered: genuinely new coverage).
- `andromeda-pulse-0.3.0/chunks/2026-08-27-report-window-copy-affordance/{plan,report}.md` — provenance of
  the "opaque rejection" premise: the plan's §Constraints REJECTED the telemetry option on it; the same
  chunk's REPORT records the probe reading `copy_rejection: "…not allowed by ACL"` from the webview.

## Graph impact (traced at `tree-query-2026-08-30-acl-rejection-logging.json`, rust ×1 + ts ×1)
- **`copyMarkdown`** (ts) — consumers: `Report.tsx:23` + `use-report.test.ts` ×4 + self. The catch edit fans
  out to the existing co-located test file; no other production consumer.
- **`record_findings_counter_refresh`** (rust) — callers: 2 colocated tests only. Zero non-test callers is
  the documented IPC-seam shape (cookbook name-bridge corollary), not dead code; the webview reaches it via
  the generated bindings. Threading set for a sibling procedure: `telemetry.rs` + the `EXPECTED_PROCEDURES`
  pin + regenerated STAGED bindings — nothing else.

## Patterns detected
- **Frontend telemetry procedure** (`telemetry.rs:166-254`): bounded input struct → `validate_*` →
  level-gated `tracing::` emit; rejection unit-tested by name (security-plan §Input Validation row).
- **Exact-leaf + pins under `pulse-app/tests/`** (`unit_observability_allowlist_window_navigation.rs`
  shape): set equality both directions + fallback discriminator; src-level `mod tests` never run in
  pulse-app (ratchet reds new ones).
- **Fire-and-forget webview reporting** with `.catch(() => {})` (`use-findings.ts:18` comment) so a reporter
  can never throw or loop — the shape the rejection reporter must take (a failed report of a failure must
  not recurse).
- **Bounded label mirroring** across layers: `window.rs::sanitize_window_label` (Rust) ↔
  `use-window-label.ts::sanitizeWindowLabel` (TS) — the resolver-side coercion is the third copy by the same
  rule (coerce to `unknown`, never reject on label).

## Conventions to follow
- **Emit level/discipline**: WARN, once per rejection event, off any hot path — `interpretation.model.load.error`
  precedent (obs-plan §6 warn row; obs extract).
- **Field set**: `error_category` (bounded static) + `window_label` (bounded 4+unknown) + `payload_bytes`
  (aggregate numeric) — NEVER the payload, the command string, or the raw rejection text (security-plan
  §Logging & Monitoring verified at :331-332; §Anti-Patterns → Logging).
- **Gate order**: full standard set + webview gates; `capability-drift` LAST; regen staged with the pin
  (testing.md 2026-06-12/2026-08-15; the staged gate now asserts it mechanically).

## New files to create
- `pulse-app/tests/unit_observability_allowlist_ipc_rejection.rs` — the leaf pins (set equality both ways +
  no-bare-`ui`-key discriminator).
- `pulse-app/ui/src/report/ipc-rejection.ts` (name per plan) — the bounded classifier + fire-and-forget
  reporter helper, exported for future catch sites.

## Files to modify
- `crates/ui-bridge/src/telemetry.rs` — `IpcRejectionInput` + validators + 5th trait method
  `record_ipc_rejection` + resolver + colocated tests (incl. rejection-by-name + emit-target tests).
- `xtask/src/main.rs` — `EXPECTED_PROCEDURES` +1 (`telemetry.frontend.record_ipc_rejection`).
- `pulse-app/ui/src/bindings/index.ts` — REGENERATED via the mcp-feature nextest, STAGED with the pin.
- `pulse-app/src/observability.rs` — exact leaf `ui.ipc.rejection` (3 fields).
- `pulse-app/ui/src/report/use-report.ts` — `copyMarkdown` catch classifies + reports (fire-and-forget).
- `pulse-app/ui/src/report/use-report.test.ts` — reporter-fires-on-rejection + classifier cases (both
  measured message forms + non-matching → other).
- `pulse-app/ui/src/report/Report.tsx` — CARRY fix: container `color` → `var(--color-text-primary)`
  (headline inherits ≥4.5:1); accent stays on the border; span detail stays secondary.
- `pulse-app/ui/tests-a11y/axe/p9-diagnostic-report-modal.spec.ts` (or a sibling spec) — drive the
  LOAD-error path via `__mockReject` with a populated fixture so the fixed `ErrorState` is actually audited.
- (Wrap amendments, NOT touchpoints: arch §Occupied Resources procedure entry; obs-plan §6+§8 dual-site
  registration; design-system §Color Palette "migration COMPLETE" wording; possibly security-plan pin-#22
  trailing pointer staleness flagged by the security extract.)

## Open questions
- none blocking plan-decision — the fork is resolved by measurement (below).
- Whether `Report.tsx` has its own co-located test file (vs. coverage via `use-report.test.ts` +
  `ReportRenderer` tests) → blocks: implementation-scope only (file list provisional for /implement).

## The fork, resolved by measurement
1. **No Tauri-runtime-side hook exists** at the workspace's Tauri 2.11: the denial site rejects straight to
   the webview; `invoke_system` is script-only; nothing app-side can see the denial without forking the
   runtime. Arm 2 is DEAD at this version (recorded, revisitable on a Tauri upgrade).
2. **The webview-side rejection IS attributable**: its value carries `not allowed by ACL` (release) /
   `not allowed` (debug) — measured live by the 2026-08-27 probe in the webview context and confirmed at the
   Tauri source. The entry's "honest limit" premise (the catch "cannot attribute it to the ACL") is
   PREMISE-CORRECTED: what was opaque was the COMPONENT STATE the catch collapsed to, not the rejection
   value it received. A bounded classifier (`String(err?.message ?? err)` contains `not allowed` →
   `acl_rejected`, else `other`) attributes genuinely, with the caveat that the string is a Tauri
   implementation detail, not a contract — the classifier treats non-match as `other` and its pin cites both
   measured forms.
→ The frontend arm (`telemetry.frontend.record_ipc_rejection`) is the decided shape, and it satisfies the
security-plan mandate semantically ("the ACL rejected it"), not merely approximately.
