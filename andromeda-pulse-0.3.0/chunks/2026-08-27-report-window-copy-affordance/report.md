# Report — 2026-08-27-report-window-copy-affordance

**Chunk:** Report-window Copy affordance — the report Copy control copies instead of failing, and a headful gate presses the real control
**Date:** 2026-08-27
**Commits:** (none yet — this wrap authors the chunk commit)

## Changes (structured — detectors read this)

- **Files:**
  - `pulse-app/capabilities/clipboard.json` (M) — `windows` gains `"report"`; `description` gains the rationale
  - `pulse-app/ui/tests-e2e/webview-drive.mjs` (M) — `report-copy` press + DOM observation + failure-only rejection probe
  - `xtask/src/webview_drive.rs` (M) — `STAGES` entry, `stage_halves` arm, `dom_report_copy_ok`, 2 unit pins
  - `pulse-app/ui/src/report/ReportRenderer.tsx` (M) — `aria-busy` on the Copy button
  - `pulse-app/ui/src/report/use-report.test.ts` (NEW) — copy state machine, 4 pins
  - `pulse-app/ui/src/report/ReportRenderer.test.tsx` (NEW) — Copy control `aria-busy` + `data-copy-state`, 8 pins

- **Symbols / APIs:**
  - NEW `xtask::webview_drive::dom_report_copy_ok(dom: &[Value]) -> bool` — private to the module; sole caller is the `"report-copy"` arm of `stage_halves` (`xtask/src/webview_drive.rs`).
  - CHANGED `xtask::webview_drive::STAGES` — one entry appended (`report-copy`), between `report-window` and `investigate`. Its existing consumers are UNCHANGED and still compile against it: `stage_ids()` @`:133`, `run_webview_drive()` @`:138,142,244,270`, and 5 unit tests @`:1381,1388,1395,1402,1412` — all iterate `STAGES` generically, none enumerates ids, so none needed an edit.
  - CHANGED `ReportRenderer` Copy button DOM surface — `aria-busy` added. `ReportRendererProps` is UNCHANGED (no prop added/removed); `Report.tsx` remains its sole production caller.
  - **NO** TauRPC procedure added/changed · **NO** `EXPECTED_PROCEDURES` delta · **NO** new capability identifier · **NO** new env var · **NO** port/socket change.

- **Crates / modules:** none added, removed, or renamed.

- **Dependencies:** none added, none bumped (no `Cargo.toml`, `Cargo.lock`, or `package.json` change).

- **Schema / config:** `pulse-app/capabilities/clipboard.json` — the `windows` array widens from `["compact-widget","main"]` to `["compact-widget","main","report"]`. The `permissions` array is UNCHANGED (`clipboard-manager:allow-write-text` only); `clipboard-manager:allow-read-text` / `allow-read-image` remain absent, and the file's NEVER-add-read clause is preserved verbatim.

- **Spec-master edits:** none applied by /implement (spec masters are read-only there). One is EXPECTED at this wrap — see *Spec claims disproved by measurement* and the plan's `Expected amendments (wrap)`.

- **Counts / qualifiers moved:** **YES — the assembled headful stage count moved 15 → 16** (new id `report-copy`). Documents stating the old count: `.andromeda/architecture.md` §Stack and Technologies → *GUI verification harness (dev-only)* Role cell (arch's only statement of that path); `.andromeda/test-plan.md` §4 *E2E desktop-webview* and §6 *P5* row; `.claude/rules/testing.md` §Framework (E2E desktop-webview) and the 2026-08-23 verification-harness Session Addition. No other derived count, tally, or qualifier moved.

- **Dev-tool versions:** none installed or upgraded.

- **Reverted / negative API facts:**
  - Driver field `copy_aria_busy_while_copying` — added, then REMOVED. The clipboard write settles faster than the 250 ms poll, so the transient `copying` state is never sampled and the field was structurally always `null`; a permanently-null field would read as "aria-busy is broken". The property is pinned at the vitest tier instead (`ReportRenderer.test.tsx`).
  - Driver constant `MODAL_LIVE_REGION` — added, then REMOVED. `browser.execute` serialises its body to the webview and cannot close over module scope, so the selector literal must live inline; the constant was dead and failed `no-unused-vars`.
  - Driver field `widget_probe` — added for diagnosis, then REMOVED once it had answered its question (see *Deviations*).

- **Spec claims disproved by measurement:**
  1. **`.claude/rules/testing.md` §Session Additions 2026-07-05 (boot-smoke coverage) prescribes `npm run build --prefix pulse-app/ui` + `cargo build -p pulse-app` to re-embed the frontend before a smoke.** MEASURED FALSE for the two headful harnesses: `xtask::self_verify::pulse_binary_candidates` returns `target/release/pulse-app.exe` BEFORE `target/debug/…` (`xtask/src/self_verify.rs:303-310`, pinned by its own test `pulse_binary_candidates_prefers_release_then_debug` @`:432`), and both `cargo xtask self-verify` and `cargo xtask webview-drive` launch through it. Evidence: three consecutive GREEN-arm legs reported `copy_state: "error"` with `copy_rejection: "…not allowed by ACL"` while the generated `capabilities.json` already listed `report`; `cargo build -p pulse-app --release` flipped the stage to `copied` with no other change. Disposition owed: a `rules/testing.md` correction (leaf, via the cascade) — the recipe must name the release build for these two legs, or the resolver must prefer the freshest binary.
  2. **The plan's own step-6/7 sequencing inherited that same false premise** (a chunk-artifact claim, not a spec-master claim). Per the 2026-08-26 ownership rule this is recorded HERE and owes no amendment — `plan.md`'s amendment windows are closed at wrap. DISPOSED: recorded, no amendment owed.

- **Coverage of new surfaces:**
  - `report-copy` headful stage (`xtask` + driver) → validation n/a · instrumentation n/a (DOM-only by construction, matching the sibling `report-window` / `findings-window` arms — the findings/report lifecycle is JS `hide()`/`show()` end to end with no Rust hook) · PII n/a (records a bounded state token `idle|copying|copied|error` and a fixed live-region string; never the copied markdown) · tests unit (2 predicate pins, both directions) + e2e (the stage itself, GREEN + RED arms) · a11y n/a (harness, not a rendered surface) · tokens n/a
  - Copy button `aria-busy` (`ReportRenderer.tsx`) → validation n/a · instrumentation n/a · PII n/a · tests unit (`ReportRenderer.test.tsx`, busy in `copying` + not-busy in all three settled states) · a11y WCAG✓ (a11y-plan §4 ARIA Patterns Button row; SC 4.1.2 name/role/state; the outcome was already announced via the pre-existing `Modal` `role="status"` + `aria-live="polite"` live region, unchanged by this chunk) · tokens n/a (no style value added)
  - `clipboard.json` `report` window grant → validation n/a · instrumentation n/a · PII n/a (outbound write-text only; no read permission) · tests e2e (the `report-copy` stage is its regression guard; `capability-widening-check` clean, 0 violations across 3 inspected) · a11y n/a · tokens n/a

## Deviations from intent

1. **Three post-fix headful legs measured a stale binary** — the plan's step 6→7 assumed `cargo build -p pulse-app` puts the new ACL in front of the harness; it does not (see *Spec claims disproved* #1). Justification for the extra legs: each round added a MEASUREMENT rather than repeating a fix — leg 2 introduced a rejection probe that named the ACL, leg 3 added an already-granted-window contrast (`report` → `ERR`, `compact-widget` → `OK` in the same run) that localised the fault away from the configuration. A `cargo clean -p pulse-app` (142.9 GiB) was spent en route on a staleness hypothesis that was not the cause. Soft-exit Trigger 1 near-fired (identical failure after three distinct fixes) and continuation was chosen deliberately on that basis.

2. **A failure-only raw-invoke probe was retained in the driver**, which brushes the plan's `Constraints & rejected approaches` entry rejecting a `writeText` proxy. Justification: the rejection is a DIAGNOSTIC field, not the verification — it fires only when the stage has already failed, and the stage verdict keys exclusively on the real control press (`copy_pressed && copy_state === "copied"` plus the live-region text). Without it a red stage reports `copy_state: "error"` and nothing else, which is precisely the opacity that let this defect live from 2026-07-10 to a live operator press.

3. **A second new test file was added beyond the plan's touchpoints** — `ReportRenderer.test.tsx`. The plan's step 9 said to pin `aria-busy` "in the new vitest file", but that file (`use-report.test.ts`) tests the HOOK; `aria-busy` is a component-render property. Pinning it there would have mixed subjects. Justification: the acceptance criterion requires the attribute to be exposed, and an implemented-but-unpinned attribute is exactly the vacuous-green shape this project's learnings warn about. Caught at wrap by re-checking each criterion rather than assuming.

4. **`aria-busy` itself is a scope addition** — declared and approved at the P5 review card as an INTENT-INCOMPLETE amendment to `scope.md` (a11y-plan §4 requires it on the exact control this chunk repairs).

## Decisions & corrections

- **The harness launches `target/release/`, not `target/debug/`.** `cargo build -p pulse-app` alone leaves `self-verify` and `webview-drive` measuring whatever release binary is on disk — here one from 13:52 against a 23:27 debug build. Any chunk whose acceptance depends on those legs must build release.
- **`strings <exe> | grep <capability description>` is NOT an ACL-freshness probe.** Tauri strips `description` from the runtime ACL, so it returns 0 whether the build is fresh or stale. Its zero was briefly read as evidence of staleness — accidentally the right conclusion for the wrong reason, and after a clean rebuild the same zero would have read as still-stale. The behavioural probe (raw invoke returning the ACL rejection, plus an already-granted window returning OK in the same run) is what actually discriminated.
- **Soft-exit Trigger 1 as written cannot separate "stuck" from "narrowing with new evidence."** The usable discriminator was whether each round produced a new FACT, not whether the failure output changed.
- **`browser.execute` bodies are serialised to the webview** and cannot close over module-scope constants — selectors used only inside one must be inline literals.
- **A `data-testid` anchor was chosen over the accessible name, deliberately.** The Copy button's accessible name changes with state ("Copy markdown" / "Copying…" / "Copied" / "Copy failed — retry"), so a name-bound selector is unstable by construction. This is the sanctioned test-plan §6 exception, recorded at the selector's definition.
- **Two research premise-corrections landed at /phase P3** (already applied to `scope.md` in its sanctioned window, no disposition owed here): the discriminating copy observable already shipped in two forms, and the second clipboard surface is out of scope because it writes from Rust via `AppHandle::clipboard()` (ACL-exempt), not because of which window it mounts in.

## Outcome

**All 12 acceptance criteria MET.** The chunk's central claim is proven red→green on the real path, with the same stage reading both ways:

| Leg | `report-copy` observation |
|---|---|
| RED at HEAD (pre-widening) | `copy_pressed:true` · `copy_state:"error"` · live region "Failed to copy report to clipboard." |
| GREEN (post-widening) | `copy_pressed:true` · `copy_state:"copied"` · live region "Report copied to clipboard." |

The RED reading is the dead-affordance signature the stage exists to catch: the control was FOUND and PRESSED successfully, and only the effect field reveals the write was dropped — a "button exists, click did not throw" check passes there.

**Gates run (all green):**
- `cargo fmt --check` · `cargo clippy --workspace --all-targets --all-features -- -D warnings` → 0 warnings
- `cargo nextest run --workspace --profile ci` → **2026/2026 + 1 skip** (+2 vs the 2024 baseline = the two new xtask predicate pins exactly)
- `npm run lint / typecheck / test --prefix pulse-app/ui` → **821/821 across 79 files** (+8 vs 813 = the 4 hook pins + the 4-state aria-busy/data-state pins)
- `cargo xtask capability-widening-check` → clean, 0 violations across 3 inspected
- `cargo xtask check:ingest-progress` → PASS, 128 buffer ticks, longest zero-delta run 0
- `cargo deny check bans licenses sources` → exit 0 · `cargo deny check advisories` → designed-red at the same **8 DISTINCT ids** (0189/0190/0194/0195/0204/0222/0253/0258), unchanged for a sixth consecutive probe
- `cargo xtask test:a11y` → 37 passed · Lighthouse 7 surfaces all ≥90 · pa11y 7/7 · **0 new violation tuples vs baseline**
- `cargo xtask self-verify` → PASS (a11y/contrast PASS)
- `bash scripts/agent-run.sh status` → exit 0 with full payload (`uptime_ms: 0` — the documented not-evidence-of-this-run shape)
- `cargo xtask webview-drive` → **PASS, 16/16 stages, exit 0**, clean log (0 ERROR, 0 `app.panic.fatal`)
- `cargo xtask webview-drive --no-inject --expect-absent report-copy` → **PASS (RED arm), exit 0** — `copy_pressed:false`, confirming the stage is telemetry-dependent and discriminates
- `cargo xtask capability-drift` → clean (0 missing, 0 extra), run LAST after the mcp-server bindings regen

**Boot smoke (required — `pulse-app/capabilities/*.json` is on the test-plan §3 boot-path trigger list):** direct-binary variant on a fresh `ANDROMEDA_PULSE_DATA_DIR`, release binary. Both OTLP ports bound on `127.0.0.1` only. Feed precondition asserted BEFORE the verdict: `rows_ingested` advanced **0 → 3996** across 7 `buffer.tick` records. **0 `app.panic.fatal`, 0 ERROR.** Present: `app.boot.webview.init`, `viz.query.traces`, `duckdb.append`, `ingest`/`buffer`/`viz`/`connection` ticks, `triage.pattern.storm.detected`. `interpretation.incident.created` absent as expected — this leg sets no deterministic-L4 gate; incident creation is proven instead by the webview-drive `storm-incident` stage (obs:yes). Injector exited rc=124 at its 75 s bound mid-storm (batch 141/600) — the bounded-producer shape, not a failure. Clean shutdown by specific Windows pid 65264; `:4317`/`:4318` confirmed released.

**No gate deferrals.** All changed-surface gates plus the full workspace Rust gates ran (the latter on a cold rebuild after the diagnostic `cargo clean -p pulse-app`).

**Audit PREREQ (folded from the working-route entry):** this is a BETWEEN-point session (point 49 ran full-form; next point is **52**), so the owed action is basis + overlap re-verification, not a probe. Both re-derived first-hand this session: `bans licenses sources` exit 0, `advisories` exit 1 at the same eight DISTINCT owned ids, set unchanged. Record: `probe skipped per ratified interval (next: 52)`.

**Capabilities claimed: 0.** The version's single unverified matrix entry (P-075) is Conductor-owned `dynamic-external` and unaffected by this chunk; the coverage gate is a correct no-op.
