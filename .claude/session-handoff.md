# Session Handoff

**Last Updated:** 2026-08-23T11:35:41Z
**Branch:** chore/migrate-pulse-to-v3 (tracks `origin/chore/migrate-pulse-to-v3`; 19 ahead before this commit — unpushed)
**Status:** clean (wrapped)
**Last Commit:** `2026-08-23-metrics-points-labels` — a metric point's label set survives ingestion and reads back

## Position
- Done: `2026-08-23-metrics-points-labels` — `metrics_points` gained a scrubbed `labels VARCHAR NOT NULL DEFAULT ''` column across all three DDL representations, deliberately OUTSIDE the 4-column PK. Labels reach `SELECT_METRICS`, `MetricRow`, the `metrics.*` IPC response, MCP `query_metrics` and `pulse://stream/metrics`. **Measured RED first at HEAD**, then GREEN.
- Next (first markerless): **Integration UX e2e test** (P-076). Carries the re-pinned `cargo audit` PREREQ (**pin #9**) — and **the next wrap IS probe point 37**, so it fires in FULL form with the exit status read directly, never through a pipe.

## Work done
8 files, **+403/−6**. The scrub input is the **JOINED `key=value` form unioned with the bare value** — an operator correction at the phase P5 review, and load-bearing: `scrub_attribute` takes one `&str` while `secret_kv` and `api_key` are key-name-anchored, so a value-only scrub sees `hunter2` and matches nothing, leaking the entire keyed class. Keys are stored verbatim; only values become `[REDACTED:{category}]`, so two distinct keys never collapse into one placeholder. Ingest already bounded attributes (128/point · 256 B/key · 4096 B/value), so the bound half was inherited, not built. `consumer.rs` and `mcp-server/tools.rs` were verified unchanged, as the plan predicted.

**Gates green:** fmt · clippy `-D warnings` · nextest **1877** + 1 skip (1870 → +7, exactly the pins added) · `-p buffer -p viz` **225/225** · ui lint/typecheck/vitest **801** · capability-drift clean · capability-widening 0/3 · `cargo deny check bans licenses sources` ok · advisories designed-RED at exactly the 8 owned IDs, no new findings. **Zero gate deferrals.**

**RED → GREEN, real OTLP path, separate fresh data dirs:** RED at HEAD gave `rows_ingested` **8** with `redactions_applied` **2** (metric names only — zero label redactions); GREEN gave **8** / **4**. The +2 is exactly the two label canaries; the benign `http.method=GET` did not increment, proving selectivity at the wire. Both legs 0 ERROR / 0 panics — this defect was information loss, not batch rejection, so the RED evidence is the counter, not a `duckdb.append` failure. 0 canary literals across 1417 lines. Clean shutdown by specific PID, both ports released, 0 orphans.

**Mutation check — both families discriminate.** Neutralising the label scrub reddened the redaction pin with the defect verbatim (`left: "password=hunter2"`); dropping `labels` from the read reddened the distinguishability pin while the empty-labels pin stayed green — the expected asymmetry for a conditional property.

## Drift resolved
**9 amendments across 4 masters · 0 escalations · drift = 0.** security-plan ×5 (2 primaries — coverage 4→5 cells, and a THIRD scrub shape with its failure mode — plus 3 restating sites, one of which needed the two different fives disambiguated rather than incremented) · arch ×1 (§Conventions primary-key convention; §Occupied Resources explicitly disposed as tables-not-columns) · obs-plan ×1 (§5 scope 4→5 sites AND the counting unit refined to per-redacted-label-PAIR) · test-plan ×2 (§4 pin set + the §1 audit row). Cascade re-derived **5 leaves**; closure grep over six retired forms: **0 stale claims**.

## Notes

- **The cascade table is still wrong, and the playbook rule saved it again.** `amendment-flow.md` names only `stack.md` as architecture.md's leaf, but the provenance header shows TWO — `conventions.md` distills §Conventions verbatim and carried the retired "stores no attributes column" claim. The 2026-08-22 enumerate-by-provenance rule, codified after this exact leak one chunk ago, caught it a second time. The rule works; the reference it compensates for has not been fixed.

- **`D-security-logging` fired correctly on its first live run** — added one chunk ago after the section needed amendment at three consecutive chunks. It produced the richest proposal set (5 of 9) and caught the two-different-fives ambiguity no floor entry had anticipated. Its report-fact needs went beyond the stock Changes bullets, so a dedicated Scrub-posture subsection was written; the agent cited it by name in all five proposals.

- **A mutation check can report the INVERSE finding.** A heredoc-quoted Python edit script whose target line ends in a line-continuation backslash raises `SyntaxError` and changes nothing, while the following test run prints its usual PASS lines — indistinguishable from a pin that fails to discriminate. It happened twice this session; the second was the mutation check itself. Curated to `rules/testing.md` as a facet on the 2026-08-17 entry.

- **`agent-run.sh run` is `cargo nextest run --workspace`, not an OTLP send.** The first Test Commands draft used it as the live leg and also called `mktemp -d` twice per leg, which would have split boot and the log read across different data dirs. Both caught at the phase P5 review. Curated to `rules/verification-harness.md`.

- **`cargo audit`:** session 36 is BETWEEN ratified probe points (25/28/31/34, DISCHARGED at 34) → **skipped in the ratified form, next point 37**. Basis re-verified rather than echoed: the upstream RustSec load failure cannot be cleared by a repo change, which holds a fortiori here since the chunk added and bumped zero dependencies. Overlap re-derived this wrap: stable at the 8 owned IDs.

- Last failed command: none.

## Deferred learnings
None — curation landed exactly at the Filter-5 cap of 3 with no overflow.

## Curation
T1 0 · **T2 3** · T3 0 · corrections 0. At the cap, nothing deferred. Two landed as new `## Session Additions` entries (`rules/security.md` — a regex catalog's arms can require the COMPOSED form, so scrubbing a decomposed value silently disables the key-anchored subset; `rules/verification-harness.md` — the live-leg invocation form) and one as an in-place additive facet on the 2026-08-17 mutation-check entry in `rules/testing.md`. Filtered 2 as duplicates — the conditional-property pin asymmetry (a third confirming instance of an entry already extended 2026-08-23) and the provenance-vs-table cascade (the playbook rule worked, so not a recurrence-despite-learning). CLAUDE.md **153/200**.
