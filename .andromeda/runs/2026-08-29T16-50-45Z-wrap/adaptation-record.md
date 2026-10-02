# Adaptation Record — 0-pending wrap (session 56)

**Wrapped:** 2026-08-29T17:04:05Z · **Branch:** `chore/migrate-pulse-to-v3` · **HEAD at entry:** `f045bdf`
**Path:** Setup step 6 no-op — 0 pending master records; tree dirty only with the expected-transient
bookkeeping pair (`.claude/session-handoff.md`, `.andromeda/friction-log.ndjson`).
**Trigger:** operator ADAPTATION RELAY, direction pre-ratified 2026-08-29, shape approved in-session.

No chunk ran. No P1 report, no P2 fan-out, no light/drift/coverage gate, no master write, no
flip-compaction (P7 does not execute on this path). Scope was the markerless tail plus its
cross-version residual store.

---

## Anchor (verified before any edit)

The six entries that STAY, in the operator's stated order, were already in that relative order in the
live tail. Removing one interleaved entry produced the anchor exactly — **no reorder was performed**:

1. Halo State Pulse — *(head; carries audit pin #22)*
2. Advisory backlog
3. npm advisory coverage
4. Diagnostics un-muting + harness-truth sweep
5. Staged-bindings assertion
6. ACL-rejection logging

---

## Disposition 1 — Metrics label surface · MOVED OUT of 0.3.0

**Action:** removed from `andromeda-pulse-0.3.0/working-route.md`; appended to `.andromeda/residuals.md`
as an `open` cross-version entry, origin `2026-08-23-metrics-points-labels`, **target `0.4.0`**.

**Ground:** the data path shipped — `metrics_points.labels` persists a scrubbed `key=value` set and both
`metrics.query` and MCP `query_metrics` return it. What remains is purely the CHART surface, ruled UI
material and post-0.3.0.

**Carried into the residual so the next route intake does not rediscover it:** the measurement
(`MetricsChart` aggregates only `ts_unix_nano`/`value`; no metrics table, legend, or per-row element
anywhere in the webview, measured 2026-08-23) · the unspecified-territory scope (layout-templates never
drew the Metrics wireframe; per-series breakdown changes what "a series" means in a bucketed chart;
design-system authorises exactly two EmptyState variants) · the do-not-re-derive note that the misleading
"No metrics received yet" surface came from a rejected batch `2026-08-23-metrics-points-identity` already
closed.

**Orphan check:** the entry carried CONTEXT / SCOPE / NOTE only — no `PREREQ`, no inbound `CARRY` pinned
onto it by another chunk — so its removal stranded no annotation. It was not the first markerless entry,
so no next-entry PREREQ needed re-pinning.

---

## Disposition 2 — Halo State Pulse · STAYS, fork COLLAPSED

**Action:** the markerless entry text was rewritten to the decided shape so its `/andromeda-phase` runs
decision-free. **Pin #22 rides it unchanged.**

- **Core (24 words, WHAT-not-HOW):** "Halo State Pulse signature deferred — the specs stop claiming a
  surface that does not render, and the design half moves to the next version".
- **`RULING` (new):** operator, 2026-08-29 — the signature does NOT render in 0.3.0; the work is the
  defer-disposition, not a build.
- **`CONTEXT` (kept, tail re-pointed):** the 2026-08-21 measurement stands; its closing clause changed
  from "this entry is load-bearing for those amendments" to "the defer DISCHARGES that ownership".
- **`SCOPE` (replaced — the fork is gone):** two doc-side halves — (a) the owning spec sites record the
  signature as DEFERRED, **including `layout-templates §Surface: desktop-native`**, whose tray-halo
  error-rate hue wording is still live; (b) the render-half is appended to `.andromeda/residuals.md`
  citing the `andromeda-pulse-0.4.0-incubator` signature-orb material. Both (a) and (b) are the CHUNK's
  work, not this wrap's.
- **`NOTE` (kept):** P-025 already measures the real surface; does not block Conductor's Epoch-4 return.
- **`CARRY` (kept, one premise corrected — see below).**
- **`PREREQ` (verbatim):** audit pin #22, origin preserved.

**Sub-part disposition stated explicitly, not inherited.** The desktop-native (tray) halo layer was never
probed by the 2026-08-21 chunk. It therefore does not INHERIT the webview ruling; it is named in the new
SCOPE and falls under the same defer **on its own ground** — nothing renders it in 0.3.0 either. (Per the
`A DISPOSITION IS NOT INHERITED` rule, CLAUDE.md 2026-08-25.)

---

## Premise corrected in place (factual → AUTO under the route-resolve gradient)

The Halo entry's `CARRY` half (b) asserted:

> `degraded_mode` is `Report.degraded_mode = parsed_l4.is_none()` **driven by Resolved-only persistence,
> so an Active incident renders degraded in real-model mode too and no fixture change can move it.**

**Re-derived at HEAD and measured FALSE.** `crates/triage/src/incident/registry.rs` carries TWO sibling
writers to `resolution_summary_text` — `attach_resolution_summary` rejects non-Resolved (`:338`) and
`attach_interpretation_summary` rejects Resolved (`:355`) — the latter added by
`2026-08-26-interpretation-brief-completeness`, three chunks before this wrap. A LIVE incident with a
clean parse renders `degraded_mode: false`.

**The CARRY's obligation survives and was NOT retired.** Its other premises re-verified green the same
day: `degraded_mode = parsed_l4.is_none()` is unchanged (`pulse-app/src/incidents_router.rs:444`);
`trace_id` / `span_ids` / `timestamps_unix_nano` remain hardcoded empty at the producer
(`pulse-app/src/inference_runtime.rs:871,873`); and half (a) holds — zero non-test
`DigestKind::ResolutionSummary` constructors, fixture still pinning `is_resolution_summary: false`. So
the correction NARROWED the claim rather than deleting the annotation.

Had the rewrite transcribed the entry forward verbatim, the false clause would have reached the chunk's
`/andromeda-phase` as a premise. No gate reads route prose — this was caught only because the rewrite
re-checked. Curated as a Tier-1 extension.

---

## Audit PREREQ (pin #22) — session 56 is a BETWEEN-POINT

Session 55 discharged the owed full-form probe. The pin's own text already names 56 and 57 as
between-points with **session 58 the next INTERVAL POINT**, so it remains true after this wrap and was
**not edited**.

No probe was run and none was owed. Basis/overlap re-verification is a structural no-op here: HEAD is
unchanged since the session-55 probe (no commits this session) and the tree is clean but for bookkeeping,
so `Cargo.lock` is provably byte-identical to the lockfile that probe read. Re-running would re-prove an
unchanged artifact. Recorded rather than silently skipped.

---

## Ledger

| | |
|---|---|
| Working-route | 7 markerless → **6**; 1 entry rewritten, 1 removed; **49 frozen markers untouched, no `[marker]` line in the diff** |
| residuals.md | +1 `open` entry (target 0.4.0) |
| Curation | T1 **1** (in-place extension) · T2 0 · T3 0 · 0 conflicts · 0 deferred · CLAUDE.md 156/200 (unchanged — inline extension) |
| Master-route | **untouched** (0 pending; no flip is owed on this path) |
| Verification matrix | **untouched** — 21/22 verified, P-075 pooled |
| Gates | none run — no chunk, no source delta |
| Evolve | 2 step records (`curation`, `route-resolve`) + 1 friction (`contract.premise-falsified`) |
