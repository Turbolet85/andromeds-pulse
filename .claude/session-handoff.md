# Session Handoff

**Last Updated:** 2026-08-23T07:35:00Z
**Branch:** chore/migrate-pulse-to-v3 (tracks `origin/chore/migrate-pulse-to-v3`; 17 ahead before this commit — unpushed)
**Status:** clean (wrapped)
**Last Commit:** `2026-08-23-ingestion-scrub-coverage` — the client-controlled ring-buffer columns stop storing host PII verbatim

## Position
- Done: `2026-08-23-ingestion-scrub-coverage` — the four client-controlled DuckDB columns (`spans.service_name` · `span_events.name` · `metrics_points.metric_name` · `log_records.severity_text`) now scrub at the write boundary. `service_name` scrubs INSIDE `extract_service_name` — the choke point for three consumers — because scrubbing the column alone would have desynced DuckDB identity from the baseline registry. **Measured RED first** via a mutation check (5 pins red pre-fix), GREEN after, on the real OTLP path.
- Next (first markerless): **metrics_points identity** — the direct sibling; PK at 3 sites, no label column. Carries the re-pinned `cargo audit` PREREQ (**pin #7**, next point **37**).

## Work done
4 source files (3 modified + 1 new dev-only producer example); +318/−12 in `appender.rs` alone. The scrub landed at one choke point + three push sites; the spans and metrics builders gained the `redactions_applied` fold they never had. 8 in-crate `buffer` tests + a 16-case `security` identity-preservation corpus.

**Gates green:** fmt · clippy `-D warnings` · nextest **1859** + 1 skip (1835 → +24, exactly the tests added) · `-p buffer -p security` **207/207** · capability-drift clean · capability-widening 0/3 · `cargo deny check bans licenses sources` ok · advisories designed-RED at exactly the 8 owned IDs. **`cargo audit` PREREQ DISCHARGED at point 34** — probe RAN, exit read DIRECTLY as **1** (an earlier piped read had reported 0), first diagnostic line byte-identical upstream.

**Mutation check:** neutralizing all four scrub sites turned **5 pins RED** (four column recall pins + the counter fold); the identity-preservation pins correctly stayed green — they guard over-redaction, not under-redaction.

**Smoke PASS and discriminating** — feed precondition asserted BEFORE any absence claim (`rows_ingested` 0→6, `redactions_applied` 0→**4**, `span_events_seen` 0→2); all four tables appended 2 rows each so every canary AND control landed with no PK collision; canary literal and bare `sk_live_` format each **0 times across 21,846 log lines**; 0 panics, 0 ERROR, 0 PK violations; 6 tick families; clean specific-PID shutdown, both ports released, no orphan. The counter reading **4 and not 5** is what proves the persisted-cells-only rule held.

## Drift resolved
**9 amendments across 4 masters · 0 escalations · drift = 0.** security-plan ×3 (§Logging posture flip — five→four-live-plus-one-vacuous — plus 2 restating sites the plan never named) · arch ×2 (§Occupied Resources + §Conventions: three reserved tables are producer-less) · test-plan ×3 (`buffer-redaction-counter-unit-coverage` CLOSED + 2 restating sites) · obs-plan ×1 (§5 counter scope widened to all four builders). Cascade re-derived **5 leaves**: CLAUDE.md modules block · `rules/security.md` · `rules/observability.md` · `docs/conventions.md` · `docs/stack.md`. 3 of 7 detectors clean; 0 false positives; 0 detector re-spawns.

## Notes

- **A third dead table found mid-wrap.** The chunk measured `instrumentation_scopes` producer-less; the arch amendment sweep then found `resources` AND `span_links` in the same state — declared in `schema.rs`, swept by `retention.rs` (`:25`/`:28`/`:29`), written by nothing. **Three of the seven retention DELETEs sweep permanently-empty tables.** I wrote "two of seven" into arch before measuring `span_links` and corrected it before the sidecar — the claim went into a spec master first and was verified second, which is the inverse of the discipline this chunk otherwise enforced. CARRY'd onto the Diagnostics sweep entry, which arch now names as owner.

- **The bindings defect recurred a SIXTH time, with a NEW trigger.** This chunk touched **no TauRPC at all** — running the plan-mandated `nextest --workspace` gate is itself what rewrote `bindings/index.ts`. So an assertion scoped to "chunks that touch TauRPC" would have missed it entirely. What HELD was the staged-copy half: HEAD and the index were both correct, only the worktree was clobbered. CARRY'd onto `Staged-bindings assertion` — the fix must cover both triggers and must run unconditionally.

- **Cascade near-miss, caught by a second sweep.** The cross-master grep is only as good as the pattern set derived from the amendments, and the first pass under-derived it: patterns came from the security/test-plan wordings but not obs's (`scoped to the OTLP persistence path`), so `rules/observability.md:73` reported clean initially and surfaced only on a widened re-grep. Nothing shipped stale.

- **The `metric_name` namespace trap, resolved.** The phase directive argued scrubbing `metrics_points.metric_name` was risky because it is "the lookup key in corpus". Measured false: that corpus column is a product-internal namespace (`save_pipeline_metric("drain_template_tree","l1c")`) in a *different database*. Curated as Tier 3.

- **`cargo audit`:** probe **RAN** and DISCHARGED point 34. Re-pinned compact as **pin #7**, next point **37**, now carrying a probe-auto-satisfy SIGNATURE (exit 1 + the exact diagnostic line + overlap green at the eight owned IDs) so a future re-check that reproduces it byte-identically satisfies the pin without re-authoring the basis.

- Last failed command: none.

## Curation
T1 0 · T2 0 · **T3 2** · corrections 0. Filtered 5 — four by dedup against content the cascade had just written into the spec ecosystem, one (pipeline friction) correctly out of curation scope. CLAUDE.md **153/200**.
