# Scope — 2026-07-05-anomaly-surfacing (P-068)

**Working-route entry:** Anomaly surfacing — errors and anomalies sorted to the top of Traces, flagged with semantic error tokens, filterable (P-068 · intent F8 · design-system)

**Capability:** P-068 · Anomaly surfacing (Theme 2 — State honesty & legibility; **HIGH PAIN**)

**Annotations on the working entry:** none (no `PREREQ:` / `CARRY:` to fold in — the Epoch-3 CARRY rides P-070, not this chunk).

## Intent F8 (OBSERVED → EXPECT)
- **OBSERVED:** with live telemetry, the Traces table sorts by latency **ascending** → the single erroring/anomalous service (`payment-service`, 2500 ms, 100% errors) sits at the very **BOTTOM**, off-screen; the visible top is all-healthy services with Error 0. For an incident/observability tool this is backwards.
- **EXPECT:** errors/anomalies surface to the **TOP** (and/or are visually **flagged** + **filterable**) — the user sees "what is wrong" **first**, not last.

## Matrix acceptance (P-068, method `webview`)
> With a mixed dataset, erroring/anomalous services appear at the top (or are flagged) and are filterable; the top row is not all-healthy when errors exist.

## What it builds
1. **Anomaly-first ordering** — the Traces table ranks/sorts erroring/anomalous rows to the TOP so they are visible without scrolling; replaces (or overrides) the current latency-ascending order that buries the erroring service off-screen.
2. **Semantic error flagging** — anomalous/erroring rows carry a **design-system semantic error token** (color + badge/icon/text, never color-only) so "what is wrong" is glanceable, not merely positionally-first.
3. **Filterable** — a user-operable control (e.g. an "errors only" / "anomalies only" toggle or chip) narrows the table to anomalous rows; reversible back to all rows.

## Delta confirmed at /phase P4 (2026-06-01 inverse-of-hybrid — intent-incomplete refinement)
Research found P-068 is **partially already built**: item 2 (**semantic error flagging**) already ships in `pulse-app/ui/src/dashboard/routes/traces/TraceTable.tsx` — erroring rows carry a `--color-accent` left-border + a `✗` glyph + the count (non-color-only). So this chunk does **not build flagging**; it keeps it as-is. The **user-confirmed delta** (AskUserQuestion at P4 → "Ordering + filter") is **item 1 (default anomaly-first ordering)** + **item 3 (errors-only filter)** only. Also: intent F8's stated mechanism "sorts by latency ascending" is falsified by `crates/viz/src/query.rs` (`ORDER BY ts_unix_nano DESC` + no default error-hoist); the OBSERVED "errors buried" outcome holds and the fix is unaffected (premise-correction; intent.md left as the historical source).

## Boundaries
- **Frontend / webview surface.** Primary work is presentation: row ordering, the semantic error token, and the filter control + its state — in the dashboard's Traces table component.
- **Reuse the existing `traces.*` query path.** No new TauRPC procedure expected. IF the row data does not already carry an error-count / status / anomaly signal, surfacing that signal is in scope (a viz-query field addition, threaded through the established bindings), but the PREFERRED path is to consume whatever error/status the row already exposes — research resolves which.
- **Design-system governed.** Use existing NASA Deep Space palette semantic error tokens; do not invent new colors. Contrast + non-color-only encoding per a11y-plan (SC 1.4.1). No new motion (nothing new to gate under `prefers-reduced-motion`).
- **Non-goals:** constellation labeling (P-069), plain-language connection status (P-070), self-explaining empty states (P-071). No change to anomaly/error **detection** logic — the error/anomaly signal already exists in the data spine; this chunk **surfaces** it. No change to ingest/buffer retention or the OTLP contract.

## Surfaces / contracts touched (confirm in research)
- The webview **Traces table** component (`pulse-app/ui/src/…`) — the sort/order + row rendering + the filter control.
- Possibly the **trace row DTO / viz query** — only if error-count / status is not already present on the row.
- **Design-system** semantic error token(s) — color + badge/label.

## Acceptance anchor (val-1)
Matrix **P-068** (webview): mixed dataset (healthy + erroring services) → erroring/anomalous services appear at the TOP (or are flagged) **and** are filterable; the top row is not all-healthy when errors exist. **Affordance honesty:** the "filterable" control is exercised by a REAL DOM event in the webview test (event dispatch on the actual control), not a programmatic/state-only proxy — a dead filter control (e.g. a handler never wired) must fail the test.
