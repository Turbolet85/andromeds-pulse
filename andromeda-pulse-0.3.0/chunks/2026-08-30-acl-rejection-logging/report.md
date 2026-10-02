# Report — 2026-08-30-acl-rejection-logging

**Chunk:** ACL-rejection logging — a capability-rejected webview IPC leaves a record instead of vanishing
**Date:** 2026-08-30
**Commits:** (none since last_wrap — this wrap's commit is the chunk commit; last: `feat(2026-08-30-staged-bindings-assertion)`)

## Changes (structured — detectors read this)
- **Files:** modified `crates/ui-bridge/src/telemetry.rs` · `xtask/src/main.rs` · `pulse-app/src/observability.rs` ·
  `pulse-app/ui/src/report/use-report.ts` · `pulse-app/ui/src/report/use-report.test.ts` ·
  `pulse-app/ui/src/report/Report.tsx` · `pulse-app/ui/src/bindings/index.ts` (REGENERATED via the
  mcp-feature emit test, staged beside the pin — never hand-edited). New:
  `pulse-app/ui/src/report/ipc-rejection.ts` · `pulse-app/ui/src/report/ipc-rejection.test.ts` ·
  `pulse-app/ui/src/report/Report.test.tsx` · `pulse-app/tests/unit_observability_allowlist_ipc_rejection.rs` ·
  `pulse-app/ui/tests-a11y/axe/p9-report-load-error.spec.ts`. Plus the chunk folder
  (scope/research/plan/report) + the phase run dir.
- **Symbols / APIs:**
  - NEW TauRPC procedure `telemetry.frontend.record_ipc_rejection` — a 5th method on the EXISTING
    `TelemetryApi` trait (`crates/ui-bridge/src/telemetry.rs`); rides the existing router merges in
    `main.rs` (:1006/:1033/:2003 untouched — measured pre-satisfied); `EXPECTED_PROCEDURES` +1
    (`xtask/src/main.rs`); NO capability-JSON change (grants byte-identical, staged gate green).
  - NEW ui-bridge pub items: `IpcRejectionCategory` (closed serde enum `acl_rejected`|`other`),
    `IpcRejectionInput { error_category, window_label: String, payload_bytes: u32 }`,
    `validate_payload_bytes` (bound 100_000_000, rejection unit-tested by name),
    `coerce_window_label` (the THIRD bounded window-label copy, mirroring
    `window.rs::sanitize_window_label` + `use-window-label.ts` — coerces to `unknown`, never rejects).
  - NEW self-observation target `ui.ipc.rejection` (WARN, once per rejection event, no hot path) with its
    OWN exact allowlist leaf `{error_category, window_label, payload_bytes}`
    (`pulse-app/src/observability.rs`); NO bare `ui` key exists (measured; pinned ×4 in
    `pulse-app/tests/unit_observability_allowlist_ipc_rejection.rs` incl. the fallback discriminator).
  - NEW webview exports (`ipc-rejection.ts`): `classifyIpcRejection` (substring `not allowed` →
    `acl_rejected`, else `other`; pinned against BOTH measured Tauri forms — release
    `Command {cmd} not allowed by ACL` at `webview/mod.rs:1850`, debug `resolve_access_message` texts),
    `reportIpcRejection` (fire-and-forget; swallow-by-design), `clampPayloadBytes`.
  - CHANGED: `use-report.ts::copyMarkdown`'s catch binds the error, classifies, reports
    (state-first — UX byte-identical); consumers unchanged (`Report.tsx` + tests — the full ts-plane
    caller set, graph-enumerated at research). `Report.tsx::ErrorState` container `color` accent →
    `--color-text-primary` (existing element restyled; NO new interactive element, NO new
    screen/region/window).
  - No new ports, sockets, events, env vars.
- **Crates / modules:** none added/removed; changed: `ui-bridge` (telemetry), `pulse-app` (allowlist),
  `xtask` (pin).
- **Dependencies:** none added, none bumped (affirmative).
- **Schema / config:** none.
- **Spec-master edits:** none at implement (specs untouched by the chunk); this wrap's P2 owes the plan's
  Expected-amendments floor: arch §Occupied Resources (`telemetry.frontend.*` roster + the new leaf note) ·
  obs-plan §6 warn row + §8 leaf (dual-site) · design-system §Color Palette (the "accent-as-error-text
  migration COMPLETE as of 2026-08-23" claim gains the measured fourth site, now fixed) · security-plan
  §Dependency Security trailing pointer (stale "next interval point" — see disproved bullet).
- **Counts / qualifiers moved:** the `telemetry.frontend.*` procedure roster 4 → 5 (arch §Occupied
  Resources entry enumerates the procedures; obs-plan §3/§1 name the family). Otherwise none — verified
  (test counts move per-wrap, not spec-stated).
- **Dev-tool versions:** none.
- **Reverted / negative API facts:** none.
- **Spec claims disproved by measurement:**
  - "The webview `catch` receives an OPAQUE rejection and cannot know the ACL was the cause" — stated in
    the 2026-08-27-report-window-copy-affordance chunk's `plan.md` (§Constraints, the ground on which
    option A was rejected) and echoed in THIS chunk's working-route entry SHAPE annotation. MEASURED
    FALSE: the rejection VALUE reaching the webview carries `not allowed by ACL` (release,
    tauri 2.11 `src/webview/mod.rs:1850`) / `not allowed` (debug, `src/ipc/authority.rs`) — proven live
    in the revoked-arm run (driver probe + the classifier producing `acl_rejected` at the wire). What was
    opaque was the component STATE the catch collapsed to. Disposition: both homes are CLOSED chunk
    artifacts / frozen route prose — recorded HERE (scope.md carries the premise-correction from phase P3;
    the frozen entry's annotations archive verbatim at flip-compaction); no master states the claim, so no
    amendment owed on it.
  - Tauri 2.11 exposes NO app-side hook at the ACL denial site (`invoke_system` is script-only;
    no invoke_responder) — measured at the registry source; this is the working entry's open design
    question ANSWERED, not a master claim; recorded in scope/research.
  - security-plan §Dependency Security §Standing-deferral trailing pointer reads "next interval point
    session 61" while session 61's FULL-FORM probe was already discharged (2026-08-30-staged-bindings-assertion)
    and the pinned next point is 64 — the stale-trailing-pointer class of the 2026-08-28 precedent;
    amendment owed at P2.
- **Coverage of new surfaces:**
  - `telemetry.frontend.record_ipc_rejection` (IPC boundary) → validation ✓ (closed serde enum rejects
    unknown category; `payload_bytes` bound rejected by name; label coerced) · instrumentation ✓ (it IS
    the instrument; WARN once per rejection) · PII redacted✓ (bounded triple only; payload/command/raw
    text never sent — wire-proven: markdown 0× across 6,033 revoked-run lines) · tests unit(8 by name) +
    live wire (revoked arm) · a11y n/a · tokens n/a
  - `ui.ipc.rejection` leaf → instrumentation ✓ · PII redacted✓ (exact leaf; fields rendered unredacted
    at the wire; banned-field pins) · tests unit(4 pins by name) · a11y/tokens n/a
  - `ipc-rejection.ts` reporter/classifier (webview, non-UI) → validation ✓ (clamp) · PII ✓ · tests
    unit(8) · a11y n/a · tokens n/a
  - `Report.tsx::ErrorState` (existing element, recolored) → tokens design-token✓ (`--color-text-primary`
    text, accent border only) · a11y ✓ (new p9 load-error axe spec RENDERS the state via `__mockReject`;
    role=alert + not-color-alone pinned in `Report.test.tsx`; 0 new violation tuples; Lighthouse 7/7 ≥90)
    · tests unit(3 DOM pins) + a11y(1 spec) · validation/instrumentation n/a

## Deviations from intent
1. **Bindings regen re-sequenced BEFORE the P3 revoke smoke** (plan step 8 said last-cargo-adjacent).
   Justification: the taurpc proxy resolves methods from the ARGS_MAP **bundled into the dist**, so a
   same-chunk procedure + caller is runtime-invisible until regen→dist→embed — the first revoked-arm run
   read 0 wire records through the reporter's designed silent skip. Re-sequenced (regen → dist → release →
   leg); the staged-gate constraint still held because nothing after the regen emits bindings (release
   builds/runs never export) — proven by `staged-clean` + capability-drift green run LAST.
2. **`ipc-rejection.test.ts` added** beyond the plan's new-file list (co-location convention; homes the
   both-build-forms classifier pins the acceptance requires).
3. **Granted-world injector stopped at batch 581/600** by the invoking shell's 5-min ceiling — partial
   seed accepted; the asserted properties (absence, cleanliness, feed-advanced) are valid on it.
4. **Boot smoke ceiling + cleanup orphan** (harness, surfaced not fixed): `agent-run.sh boot`'s
   `cargo run --release` re-fingerprints under the script's exported env → a full thin-LTO relink of an
   already-built binary that outran a 180s `HARNESS_STATUS_TIMEOUT` override (the script's own default was
   NOT measured this session; `HARNESS_STATUS_TIMEOUT=900` succeeded); `cleanup` exited 0 with ports open
   and the app child alive (pidfile names the wrapper) — terminated by specific PID.
5. **Matrix claims 0 capabilities** — no `ref`/`implemented`/`verified` writes owed (per contract; P-075
   stays pooled for the Conductor return).

## Decisions & corrections
- **Same-chunk frontend procedure + caller: the BUNDLED ARGS_MAP decides runtime existence.** Regen →
  `npm run build` → re-embed must precede any live leg exercising the caller; a fire-and-forget reporter
  makes the miss silent (0 records, no error) — the wire assert is the only discriminator. (Candidate for
  curation; extends the bindings-regen family with a runtime facet.)
- **Premise-correction (operator-visible):** ACL attribution IS available webview-side in both build
  profiles; arm-2 (runtime hook) measured absent at Tauri 2.11. The classifier treats the substring as an
  implementation detail (non-match → `other`; named pins cite the Tauri coordinates).
- **Leg-artifact preservation technique:** webview-drive prints `data_dir` then deletes its TempDir at
  exit — a background watcher that polls the printed line and re-copies `logs/` every 2s preserves the
  app's own record for post-run wire assertions. (Candidate for the verification-harness rules.)
- **Revoke-cycle staged-gate interplay held by design:** a worktree-only revoke never touches the index,
  so `check:staged-artifacts` stays green through the whole cycle; restore via `git checkout --` is
  byte-identical by construction.
- **Harness frictions recorded** (deviation 4) for a future harness chunk: wrapper-PID cleanup miss +
  env-triggered release re-fingerprint vs the 180s default.

## Outcome
All 16 acceptance criteria met. Gates (all green, exits read direct): `cargo fmt --check` ·
`cargo clippy --workspace --all-targets --all-features -- -D warnings` ·
`cargo build --workspace --tests --jobs 4` + `cargo nextest run --workspace --profile ci`
(**2274/2274 + 1 skip**; +12 = exactly the tests added, each collected by name) ·
`npm run lint/typecheck/test --prefix pulse-app/ui` (**vitest 835/835**, +14 by name) ·
`cargo xtask test:a11y` (**41 passed** incl. the new p9 load-error spec observed running; Lighthouse 7/7
≥90; regression-detector 0 new tuples) · `cargo xtask capability-widening-check` (clean, 3 inspected) ·
`cargo xtask check:ingest-progress` (PASS) · `cargo xtask check:staged-artifacts` (**staged-clean**) ·
`cargo build -p pulse-app --release` (embed hash verified per build: `index-CH8yg2Sm.js` →
`index-BhmyDc09.js` post-regen) · `cargo xtask webview-drive` (**17/17 PASS**, twice — pre- and
post-revoke-cycle) · `cargo xtask capability-drift` (green, run LAST).
Smokes: **granted world** — harness boot on a fresh data dir, 95,496 log lines, 0 ERROR, 0 panics,
`ui.ipc.rejection` 0×, rows_ingested 15,849, 23 buffer ticks, clean specific-PID shutdown. **Revoked
world (the live RED)** — grant removed worktree-only + rebuilt: pressing the real Copy control produced
EXACTLY ONE `WARN ui.ipc.rejection {error_category: acl_rejected, window_label: report, payload_bytes:
1598}`, all fields unredacted, the copied markdown 0× in 6,033 lines, `copy_state: "error"` (UX
unchanged), only `report-copy` red; the driver probe read the ACL string webview-side in the same run.
**Restore** — grant back byte-identical, rebuilt, 17/17 green.
**Pin #22 (session-62 BETWEEN-POINT, discharged at this wrap):** probe skipped per ratified interval
(next: 64) — basis + overlap re-verified first-hand anyway: `cargo audit` true exit 1 read directly,
basis byte-identical (`error loading advisory database: parse error: duplicate advisory ID:
RUSTSEC-2026-0244`); `cargo deny check advisories` exit 0 ("advisories ok") with the owned upgradeable
set EMPTY re-enumerated from scratch (0 DISTINCT `RUSTSEC-` ids); `cargo deny check bans licenses
sources` exit 0.
