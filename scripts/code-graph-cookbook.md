# Code-graph query cookbook

Quick-reference for querying this project's code-graph (DuckDB symbol graph) — the schema + canonical query shapes
so you don't reinvent them. The view definitions live in `scripts/code-graph-views.sql` (authoritative).
(Setup re-seeds this file by replacing everything ABOVE the learnings marker at the bottom; the tail is preserved.)

## Invocation
`python scripts/code-graph.py query <run_dir> <marker> "<sql>" [plane]` — runs the SQL against ONE plane's DB,
prints rows to stdout, and APPENDS a `{sql, rows, result, db_state, plane}` record to
`<run_dir>/tree-query-<marker>.json` (the adoption trace; regenerates that plane on miss/stale first).
**Planes:** the graph is one independent DB per indexed language (`rust` · `ts`), manifest-detected. With one
plane the arg is optional; with several it is REQUIRED — pick the plane your modify-set touches, and for a seam
question query BOTH (see the name-bridge below). Cold-start / empty DB (early chunk, no symbols): skip the query,
note it, derive from arch + extracts.

## Schema (the query views — identical per plane)
| View | Columns | What it is |
|---|---|---|
| `symbol` | `symbol, crate, file, def_line` | every workspace-defined symbol + its package + def site |
| `refs` | `callee, file, line` | every reference TO a workspace symbol (keys on **`callee`**) |
| `calls` | `caller, callee, file, line` | resolved caller→callee edges (innermost enclosing def) |
| `contains` | `parent, child` | definition nesting (method → impl/type) |
| `crate_edges` | `from_crate, to_crate` | usage-based package deps (resolved, NOT manifest-declared) |
| `calls_m` | (= `calls`, materialized) | use in recursive blast-radius CTEs |

**`defs` / `occ` / `wsdef` are raw internal load tables** (NDJSON import + a distinct-symbol helper) — never query
them directly; always use the six views above.

SCIP symbols are fully-qualified strings — `<scheme> <manager> <package> <version> <descriptors>`; the `crate`
column is the 3rd field (a cargo crate OR an npm package):
- rust: `rust-analyzer cargo conductor-core 0.1.0 run_record/RunRecord#`
- ts:   `scip-typescript npm conductor-ui 0.1.0 `src/lamp.ts`/lampFor().`

**TS matching is noisier than Rust — descriptors EMBED the file path.** `LIKE '%index%'` matches every symbol in
every `index.ts`. Anchor on the descriptor tail: `LIKE '%/lampFor().%'` (function) · `LIKE '%RunReport#%'`
(type/component). No `implements` view (the indexers emit no SCIP Relationships).

## Canonical queries (copy + adapt the predicate)
```sql
-- 1. IMPACT — who references / calls a symbol I'm about to change
SELECT caller, file, line FROM calls WHERE callee LIKE '%<Symbol>%' ORDER BY file, line;
--    all references incl. non-call uses: SELECT callee, file, line FROM refs WHERE callee LIKE '%<Symbol>%'

-- 2. CRATE EDGES — does my crate have cross-crate consumers (inbound) or deps (outbound)?
SELECT from_crate, to_crate FROM crate_edges WHERE to_crate = '<crate>' OR from_crate = '<crate>';
--    no inbound rows = leaf crate; an additive change has zero cross-crate blast radius

-- 3. RECURSIVE BLAST-RADIUS — transitive callers up to N hops (base case RESOLVES the descriptor to exact symbols)
WITH RECURSIVE b(sym, depth) AS (
  SELECT symbol, 0 FROM symbol WHERE symbol LIKE '%<Symbol>%'
  UNION
  SELECT c.caller, b.depth + 1 FROM calls_m c JOIN b ON c.callee = b.sym WHERE b.depth < 3)
SELECT DISTINCT sym, depth FROM b ORDER BY depth;

-- 4. EXISTENCE / COLLISION-CHECK — is a name free before I create a new module/type?
SELECT symbol, crate, file FROM symbol WHERE symbol LIKE '%<Name>%';
```

## The cross-plane seam (IPC / TauRPC / bindings)
Planes are disjoint subgraphs — no edge crosses a language boundary. But generated bindings (e.g.
`ui/src/bindings/index.ts`) ARE indexed on the ts plane, so frontend callers of an IPC procedure are visible up
to the binding symbol; only the binding↔backend hop is missing. For seam impact run the **name-bridge**: query
BOTH planes for the procedure/type name and join the two result sets by hand. Corollary: **zero callers on an
IPC-facing or runtime-invoked symbol is NOT dead-code evidence** — the caller may live on the other plane or in
a runtime dispatch the graph cannot see.

## Reading the result
- Non-empty → the impacted sites; cite `symbol @ file:line` in `plan.md` (the adoption numerator).
- Empty (`rows: 0`) → consulted-but-no-match is a REAL finding (leaf / additive / zero cross-crate blast), NOT
  "didn't query" — but ONLY after two preconditions: (a) the plane you queried actually BUILT (`db_state` was
  not `cold-start`, and no skipped-plane notice named it), and (b) the symbol is confirmed INDEXED — re-probe
  once via query 4 on the bare name; a symbol with no `symbol`-view row at all means the graph cannot see it
  (wrong plane, runtime-invoked, or macro-generated), which is a different fact than "no callers". Record which.
- A skipped/unbuilt plane in play → the research proceeds file-first and records `derived-without-graph`, never
  "leaf".

<!-- Project-specific query learnings accumulate below via wrap curation. -->
