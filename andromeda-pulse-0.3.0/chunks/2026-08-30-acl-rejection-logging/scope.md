# Scope — 2026-08-30-acl-rejection-logging

**Working-entry intent (verbatim outcome):** a capability-rejected webview IPC leaves a record instead of
vanishing.

## Why now (CONTEXT folded from the working entry — coordinates are HYPOTHESES until research re-verifies them)

- Operator-selected residual at the `2026-08-27-report-window-copy-affordance` P5 review: that chunk shipped
  the one-line window grant and explicitly declined the machine-readable half.
- security-plan §Logging & Monitoring → "What to log" REQUIRES Tauri capability-rejected IPC calls to be
  logged, precisely because such rejections are SILENT at runtime — and nothing logs them today.
  (VERIFIED at research: security-plan.md:331-332 verbatim; 0 ACL-rejection emitters in the Rust tree.)
- Measured cost of the silence: the report window's Copy control was ACL-dead from `2026-07-10` until an
  operator pressed it on `2026-08-27`, with a clean obs log throughout; the webview's bare `catch` turned the
  rejection into component state and `agent-latest.jsonl` never saw it. (VERIFIED at research: the swallow
  shape holds at HEAD — `use-report.ts:93` `catch { setCopyState("error") }`, no binding, no report.)

## The design question (the entry says answer it FIRST, before any binding work)

The rejection surfaces only inside the webview, and the webview `catch` receives an OPAQUE error — it cannot
attribute the failure to the ACL. Two candidate shapes:

1. **Frontend record** via a `telemetry.frontend.*` TauRPC procedure (the only obs-plan §1/§3-sanctioned
   route for a frontend observable — VERIFIED at research: obs-plan :29/:45/:199/:279, the DECIDED
   mechanism). Honest limit [premise-corrected: see item 2 — the record CAN say "the ACL rejected it"]. Cost: the full 4-place binding (router registration +
   `EXPECTED_PROCEDURES` pin + `emit_taurpc_bindings` merge + arch §Occupied Resources entry) + an EXACT
   allowlist leaf — and since `2026-08-30-staged-bindings-assertion`, the pin edit must be STAGED with
   regenerated bindings or capability-drift reds.
2. **Tauri-runtime-side hook** that can genuinely attribute (the rejection text `not allowed by ACL` was
   observed at SOME layer during the 2026-08-27 probe — `ERR win=report :: plugin:clipboard-manager|write_text
   not allowed by ACL`). [premise-corrected: measured at research — Tauri 2.11's denial site
   (`webview/mod.rs:1820-1852`) rejects ONLY to the webview; no log, no event, no app-side hook fires, and
   `Builder::invoke_system` is script-only in 2.11 — so NO runtime-side hook exists at this version (arm 2
   DEAD here; revisitable on a Tauri upgrade). AND the attributable string IS the webview promise-rejection
   value itself — `Command {cmd} not allowed by ACL` (release) / a `resolve_access_message` text containing
   `not allowed` (debug) — so the fork COLLAPSES to arm 1 WITH genuine attribution: a bounded webview
   classifier (`not allowed` substring → `acl_rejected`, else `other`). What was opaque was the component
   STATE the catch collapsed to, never the rejection value it received.]

The decided arm is arm 1 (frontend record with genuine attribution), per the measurement above; the P5 review
presents this as the resolved fork.

## What it builds (boundaries)

In scope:
1. The decided rejection-record mechanism, emitting a bounded record at the wire: bounded `error_category` +
   the `sanitize_window_label`-treated window label + a byte count; NEVER the payload (security-plan
   §Anti-Patterns → Logging). (VERIFIED at research: `pulse-app/src/window.rs:107`, private — the DAG means
   the ui-bridge resolver carries its OWN bounded coercion mirroring the same 4+unknown set, the third copy
   beside the TS `sanitizeWindowLabel`.)
2. Its own EXACT allowlist leaf (a leaf-less target silently redacts every field — the documented
   most-deceptive fallback shape).
3. If the frontend arm wins: the full 4-place binding above, staged per the staged-artifacts gate.
4. Retro-fit the measured-cost catch site ONLY (the report Copy path, `use-report.ts:93`) so its rejection
   reaches the new record — via a reusable classifier+reporter helper so future catch sites join cheaply.
   (VERIFIED at research: the survey found ~25 catch sites, most deliberate jsdom-guard `.catch(() => {})`
   shapes; a sweep is out of scope — the entry names none.)
5. A discriminating verification: in a revoked-grant world (the manual revoke-cycle discipline from
   `2026-08-23-webview-self-verify`), exercising the control produces the NEW record at the wire with
   `error_category = acl_rejected` + `window_label = report`; in the granted world it produces none and the
   affordance works. Without this chunk the revoked world is silent — that silence is the defect being
   closed. (Refined at research: the live proof is the manual revoke cycle, NOT a new headful stage — the
   granted-world leg has nothing to assert, so a stage-borne wire half would be vacuous by construction.)
6. CARRY (a11y/design, pre-existing, unrelated to the ACL work): `pulse-app/ui/src/report/Report.tsx`
   `ErrorState` renders BODY-SIZE text in `var(--color-accent)` (~3.8:1 on Base) on the report LOAD-error
   path — the accent-as-error-text class the `2026-08-23-a11y-verification` pass swept at three OTHER sites
   and missed here; fix = the same treatment (text → `--color-text-primary`, accent kept for border/icon).
   (VERIFIED at research: `Report.tsx:111` sets `color: var(--color-accent)` on the ErrorState container at
   14px — the `<strong>` headline inherits it; the `<span>` detail is already `--color-text-secondary` and
   the accent border stays.)
7. PREREQ (pin #22, compact form — re-pinned here from `2026-08-30-staged-bindings-assertion`): this chunk's
   wrap is a BETWEEN-POINT (sessions 62/63 between; session 64 owes the next FULL-FORM probe) — re-verify
   basis + overlap first-hand (`cargo audit` exit + the byte-identical duplicate-id basis; `cargo deny check
   advisories` with the owned set re-enumerated from scratch as DISTINCT ids; `bans licenses sources`) and
   record `probe skipped per ratified interval (next: 64)` in the chunk report. No "Nth consecutive" ordinal.

Out of scope:
- Logging any payload, prompt, or attribute content — bounded fields only.
- Changing the ACL itself: no new capability grants, no permission widening (this chunk OBSERVES rejections).
- Automating the webview-drive revoke cycle (test-plan §1 trigger-owned; explicitly left there by
  `2026-08-30-staged-bindings-assertion`).
- Any per-procedure capability JSON (does not exist in this project — measured 2026-08-21).
- The `Dead lib-src test migration` entry's territory (next chunk).

## Surfaces / contracts expected to be touched

- `pulse-app/ui/src/report/use-report.ts` (+ a new classifier/reporter helper module) — the measured-cost
  catch site. (VERIFIED at research.)
- `crates/ui-bridge/src/telemetry.rs` + `xtask/src/main.rs` `EXPECTED_PROCEDURES` + regenerated staged
  bindings. [premise-corrected: `pulse-app/src/main.rs` needs NO edit — `TelemetryApiImpl` is already merged
  at both production router branches AND `emit_taurpc_bindings` (:1006/:1033/:2003), so a new method on the
  existing trait rides the existing wiring.]
- `pulse-app/src/observability.rs` allowlist (one new exact leaf, `ui.ipc.rejection`; no bare `ui` key
  exists — probe measured 0 — so leaf-less = fully redacted) + NEW pins in
  `pulse-app/tests/unit_observability_allowlist_ipc_rejection.rs`. (VERIFIED/refined at research.)
- `pulse-app/ui/src/report/Report.tsx` (the CARRY fix) + `use-report.test.ts` + a p9 a11y spec extension
  driving the LOAD-error path (VERIFIED at research: no a11y spec renders `ErrorState` today).
- Headful leg: [premise-corrected: NO new stage and no stage-borne wire half — the granted-world run has no
  rejection to assert (vacuous by construction); the wire proof lives in the manual revoke cycle + the
  committed test tiers. The 17-stage leg runs unchanged as a regression gate.]

## Anchors for validation-1

The plan must (a) decide the attribution fork FROM MEASURED EVIDENCE (not assumption), (b) deliver a bounded,
allowlisted, wire-verifiable rejection record reachable from the webview's real failure path, (c) land the
CARRY a11y fix, (d) discharge the pin #22 between-point at wrap, and (e) violate none of the Out-of-scope
lines above.
