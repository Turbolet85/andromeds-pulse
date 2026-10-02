# Codebase Research — 2026-08-23-integration-ux-e2e-test

## Scope
- **Depth:** deep · **Reads:** 14 · **Globs/Greps:** 16 · **Graph queries:** 2 (rust + ts planes)

## Files inspected
- `xtask/src/webview_drive.rs` (1-140 + surface grep) — the verdict owner. Structure to extend: headless/binary/msedgedriver SKIP guards → `TempDir` data dir → `run_driver` (240s timeout) → read log lines → GREEN-only clean-log check → single `is_widget_hidden_transition` predicate → `report(observed, expect_absent)`. The two-arm `--expect-absent` inversion is a whole-leg boolean today.
- `pulse-app/ui/tests-e2e/webview-drive.mjs` (full, 200 lines) — the press side. Already carries every substrate a staged path needs: tauri-driver spawn, `portAccepts` poll, WebdriverIO session, 4-window handle enumeration with per-handle URL + control probe, `pollUntil` bounded polling, obs-log tailing, `finally`-guarded `deleteSession`.
- `pulse-app/src/observability.rs` (allowlist regions :352-375, :1944-1970, :2082-2095, :2240-2262; `for_target` :2271-2291) — the default-deny allowlist and its resolution order.
- `crates/viz/src/query.rs` (:178-196) — the `viz.query.traces` emit site.
- `pulse-app/src/investigate_router.rs` (:1-40, :145-190, :285) — det-L4 gate + `emit_outcome`.
- `pulse-app/src/inference_runtime.rs` (:86, :779-790) — the incident-created emit site.
- `pulse-app/src/incidents_router.rs` (:215-232) — `incidents.list_active.request`.
- `pulse-app/src/deterministic_inference.rs` (:55-95) — the canned fixture.
- `crates/ingest/examples/inject_demo.rs` (:1-40) — the storm injector.
- `pulse-app/ui/src/dashboard/routes/traces/TraceTable.tsx` (selector grep, 13 hits) · `components/EmptyState.tsx` (full) · `components/Titlebar.tsx` (:124-134) · `dashboard/ConnectionStatusLine.tsx` (:110) · `dashboard/FooterStatusBar.tsx` (:13) · `components/ConnectionDot.tsx` (:111-125) · `dashboard/routes/MetricsRoute.tsx` (:35-52) — the DOM handles.
- `pulse-app/ui/eslint.config.mjs` (:55) · `pulse-app/ui/tsconfig.json` (:22-23) — gate coverage of `tests-e2e/**`.
- `.andromeda/test-plan.md` (:124, :465) — CARRY #10's owning trigger.

## Graph impact
- **rust plane — `run_action`:** **9** caller rows (`db_state: fresh`), ALL in `pulse-app/tests/integration_investigate_actions.rs` (:68, :84, :89, :104, :113, :126, :158, :190, :271) — re-derived from the trace's `rows` field, not a clipped console view. Zero production callers, which is expected and NOT dead-code evidence: this is an IPC-facing TauRPC resolver whose real caller is the webview across the plane boundary (cookbook §cross-plane seam). The name-bridge confirms the other half — the webview presses `button[aria-label="Investigate"]` at `Titlebar.tsx:128`.
- **ts plane — driven components:** **71** symbol rows (`db_state: fresh`) across 5 files — `ConnectionStatusLine.tsx` (14) · `EmptyState.tsx` (6) · `TraceTable.tsx` (4) plus two colocated test files. All three components the leg selects against resolve on the indexed plane. Both planes fresh; no regenerate, no cold-start.

## Patterns detected
- **Press-JS / assert-Rust split** (`webview_drive.rs:6-9`): the `.mjs` presses and returns; the xtask side reads the app's own record and owns the verdict. The leg's whole rationale is written in that header — "a press that returned without throwing proves nothing".
- **Bounded-poll synchronization** (`webview-drive.mjs:56-66` `pollUntil`): deadline + interval + explicit probe, replacing `sleep(N)`. Both stages and teardown already use it.
- **Multi-window handle discovery** (`webview-drive.mjs:126-142` `findWidgetHandle`): enumerates handles, switches, reads URL + probes for a control, then picks. Four surfaces exist (compact-widget / main / findings / report) and the session lands on an arbitrary one — the staged path needs this per stage, not once.
- **Aggregate-only, bounded-cardinality emission** (`investigate_router.rs:145-150`): `action_id` is always a validated id or the literal `"unknown"`, never a webview-supplied string.
- **Storm-by-construction fixture** (`inject_demo.rs:1-10`): one FIXED `exception.type` + `exception.stacktrace` repeated so the buffer hashes one identical fingerprint — "storm = many of the SAME fingerprint".

## Conventions to follow
- **Allowlist resolution order** (`observability.rs:2271-2291`): exact target → `.tick` suffix strip → first `.` segment → first `::` segment → `None`. The prefix fallback is the third rung.
- **Selector convention is MIXED at HEAD** — accessible names ship on some surfaces, `data-testid` on others. Measured, per surface, below.
- **`tests-e2e/**` is lint-covered but not typecheck-covered** (`eslint.config.mjs:55` includes `tests-e2e/**/*.{js,mjs}`; `tsconfig.json:22` includes only `src/**/*`). Consistent — the driver is `.mjs`, outside tsc's remit — but it means `npm run typecheck` is not a guard on driver changes; `npm run lint` is.

## The three acceptance observables — each bound and VERIFIED at HEAD

The claim-exit invariant needs each assertion bound to an observable the driver can read. All three resolve to obs records, and **none needs a new allowlist leaf.**

| # | Assertion | Target | Emit site | Allowlist resolution | Assertable field |
|---|---|---|---|---|---|
| 1 | Traces render | `viz.query.traces` | `crates/viz/src/query.rs:184-192` | **prefix fallback** → bare `"viz"` key, `observability.rs:358` | `row_count` |
| 2 | Incident creation | `interpretation.incident.created` | `pulse-app/src/inference_runtime.rs:779-785` | **exact leaf**, `observability.rs:2084` | `created` (+ `severity`, `priority_tier`) |
| 3 | Investigate result | `investigate.run_action.request` | `pulse-app/src/investigate_router.rs:157-164` | **exact leaf**, `observability.rs:1946` | `status` (+ `deterministic_mode`) |

**The load-bearing equality, verified field-by-field (not "the mechanism exists" — whether it decides THIS case):**

- **Assertion 1 is the one that could have failed and does not.** `viz.query.traces` has NO exact leaf; it resolves through `for_target`'s first-segment fallback to the bare `"viz"` key. That is the same rung the 2026-08-21 delegated-timing lesson warns about — but the outcome is the opposite here, because the bare `viz` key enumerates a SUPERSET rather than a stub. Emit site emits 6 fields: `query_id` ✓ · `param_count` ✓ · `param_types` ✓ · `time_window_seconds` ✓ · `row_count` ✓ · `latency_ms` ✓ — all six appear in the `viz` set (which also carries `row_count_returned`, `query_latency_ms`, `subscribers_active`, `traceparent`, `filter_count`). **`row_count` survives scrubbing today.** The `metric.*` failure mode does not transfer: the bare `metric` key is a stub carrying `value`; the bare `viz` key is a complete field vocabulary.
- **Assertion 2:** exact leaf `["created","deduped","severity","priority_tier"]` vs emit site `created` / `deduped` / `severity` / `priority_tier` — exact 4/4 match. **Producer-existence check passes:** `TARGET_L4_INCIDENT_CREATED` has exactly 2 occurrences, the const at `:86` and a real `tracing::info!` at `:779` in production code — not under `#[cfg(test)]`.
- **Assertion 3:** exact leaf `["action_id","status","duration_ms","deterministic_mode","model_tier"]` vs emit site's 5 fields — exact 5/5 match. `status = "success"` is set on the success path (`:285`); `deterministic_mode` independently proves the leg ran under det-L4.
- **Secondary for #2:** `incidents.list_active.request` → `item_count` (exact leaf `["item_count"]`, emit at `incidents_router.rs:221`) gives a count-shaped cross-check for absorbed CARRY #12.

## The storm stage — mechanism resolved, no new injector

`crates/ingest/examples/inject_demo.rs` is purpose-built for exactly this path. Its own header documents the chain: *"Short healthy warmup, then payment-service degrades: every error span carries an OTLP `exception` span event with a FIXED `exception.type` + `exception.stacktrace` … At ~10 error spans/s, the chunk-#66 RetryStormDetector crosses its `count >= 10 / 60s` Autonomous threshold within ~1s -> RetryStorm cue -> accelerated cadence -> rich digest -> L4 -> incident -> payment dot red."*

It speaks real OTLP gRPC (`TraceServiceClient`), satisfying arch §Test-time telemetry injection's no-back-door mandate. It covers launch → telemetry → storm → incident in one process. **No new injector is needed**, and the scope's P-077 adjacency question resolves to "the gate consumes an existing tracked example" — it does NOT formalize it (that stays P-077's job).

## DOM selectors — measured at HEAD, convention is MIXED

Per the a11y amendment lesson (a spec-pinned name absent from the product left two real controls uncovered), every selector was measured before being considered.

**Accessible names DO ship (prefer these — a name regression then fails the stage):**
- `button[aria-label="Investigate"]` — `Titlebar.tsx:128` (also `data-testid="titlebar-investigate"`)
- `button[aria-label="Investigate trace {label}"]` — `TraceTable.tsx:314`
- `[role="img"]` + aria-label — `ConnectionDot.tsx:111-113`
- `button[aria-label="Close to tray"]` / `"Minimize"` — the predecessor leg's, still live

**Only `data-testid` ships (no accessible name exists to bind to):**
- Traces: `trace-table` · `trace-row` · `trace-table-empty` · `trace-table-loading` · `trace-table-scroll` · `trace-table-toolbar` · `trace-errors-only-filter` · `trace-row-error`/`-ok` (`TraceTable.tsx`, 13 test-ids)
- Empty states: `metrics-empty-state` / `metrics-error-state` / `{testId}-hint` (`EmptyState.tsx:16,38`; `MetricsRoute.tsx:38-51`)
- Footer: `connection-status-line` (`ConnectionStatusLine.tsx:110`) · `dashboard-footer` (`FooterStatusBar.tsx:13`)

**The conflict this creates (plan-decision):** a11y-plan §1 P1 mandates `region[aria-label="Telemetry traces chart"]` + `table`, and the a11y extract's acceptance contribution requires every driven element be located "by accessible name or role". **Neither exists on the Traces surface at HEAD** — it is `data-testid` throughout. This is the same phantom-selector class as the `Minimize to tray` case, one surface over. The EmptyState glyph is deliberately `aria-hidden` (decorative, SC 1.1.1) with meaning carried by message + hint text, so the a11y "worded text carrier" rule is already satisfied by design there — the message/hint ARE the assertion targets.

## Files to modify
- `pulse-app/ui/tests-e2e/webview-drive.mjs` — from one press to a staged sequence; per-stage handle resolution; keep the press/assert split.
- `xtask/src/webview_drive.rs` — from one boolean predicate to per-stage predicates + a staged verdict; extend `Cmd::WebviewDrive`'s arm model beyond the whole-leg `--expect-absent`.
- `xtask/src/main.rs` — `Cmd::WebviewDrive` (:114) + dispatch (:177) if the CLI surface gains a stage selector.
- **Caller threading:** `xtask/src/self_verify.rs` exports `headless_skip_reason` / `locate_pulse_binary` / `read_log_lines` / `workspace_root`, all four consumed by `webview_drive.rs:24-26`. Any signature change there threads into `self_verify.rs`'s own callers — the graph shows the helper set is shared, so prefer additive helpers over re-shaping these.
- Colocated `#[cfg(test)] mod tests` in `xtask/src/webview_drive.rs` (9 tests today) — each new stage predicate needs its own, per test-plan §4.

## New files to create
- None required. (A separate staged driver script is possible but the existing `.mjs` already carries the substrate; the plan should extend rather than fork, per arch's "extend the registered stack, not replace it".)

## Files NOT to modify (confirmed non-touches)
- `pulse-app/capabilities/*.json` — no new TauRPC procedure; the leg presses existing controls, and `core:window:allow-close` is already granted.
- `xtask/src/main.rs::EXPECTED_PROCEDURES` — no procedure added.
- `pulse-app/ui/package.json` — no new npm dependency (WebdriverIO + tauri-driver already devDeps).
- `pulse-app/src/observability.rs` — **no new allowlist leaf needed**, all three observables already resolve.
- `crates/ingest/examples/inject_demo.rs` — consumed as-is.

## Open questions
1. **Selector convention for the Traces stage** → blocks: **plan-decision**. Bind to shipped `data-testid`, or add the a11y-mandated `aria-label`/`role` to the Traces surface in this chunk? The neighbouring markerless entry "A11y verification — v0.3.0 interactive surfaces" is the natural owner of the second option. Resolved with the operator at P4.
2. **Absorbed-set breadth** → blocks: **plan-decision**. Five absorbed CARRYs plus three acceptance assertions is a wide leg for the highest flake-risk surface in the suite (test-plan §10 zero-flakiness budget). Whether to land the three acceptance stages first and re-pin the five absorbed CARRYs, or land all eight, is the operator's call at P4.
3. **Native context-menu observability (boundary CARRY #3)** → blocks: **nothing in this chunk**. NOT empirically probed here — WebDriver addresses the DOM and a WebView2 native context menu is an OS-level popup outside the document, so it is *reasoned* unobservable, not *measured* unobservable. Recorded so the boundary's future owner probes rather than inherits the assumption.
