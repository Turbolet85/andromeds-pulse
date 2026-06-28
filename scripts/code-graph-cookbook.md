# Code-graph query cookbook

Quick-reference for querying this project's code-graph (DuckDB symbol graph) — the schema + canonical query shapes
so you don't reinvent them. The view definitions live in `scripts/code-graph-views.sql` (authoritative).

## Invocation
`python scripts/code-graph.py query <run_dir> <marker> "<sql>"` — runs the SQL, prints rows to stdout, and APPENDS
a `{sql, rows, result, db_state}` record to `<run_dir>/tree-query-<marker>.json` (the adoption trace; regenerates
the DB on miss/stale first). Cold-start / empty DB (early chunk, no symbols): skip the query, note it, derive from
arch + extracts.

## Schema (the query views)
| View | Columns | What it is |
|---|---|---|
| `symbol` | `symbol, crate, file, def_line` | every workspace-defined symbol + its crate + def site |
| `refs` | `callee, file, line` | every reference TO a workspace symbol (keys on **`callee`**) |
| `calls` | `caller, callee, file, line` | resolved caller→callee edges (innermost enclosing def) |
| `contains` | `parent, child` | definition nesting (method → impl/type) |
| `crate_edges` | `from_crate, to_crate` | usage-based crate deps (resolved, NOT Cargo.toml-declared) |
| `calls_m` | (= `calls`, materialized) | use in recursive blast-radius CTEs |

**`defs` / `occ` / `wsdef` are raw internal load tables** (NDJSON import + a distinct-symbol helper) — never query
them directly; always use the six views above.

SCIP symbols are fully-qualified strings — `<scheme> <manager> <crate> <version> <descriptors>` (e.g.
`rust-analyzer cargo conductor-core 0.1.0 run_record/RunRecord#`). Match by descriptor with `LIKE '%Name%'`;
`crate` is the 3rd space-delimited field. No `implements` view (the indexer emits no SCIP Relationships).

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

## Reading the result
- Non-empty → the impacted sites; cite `symbol @ file:line` in `plan.md` (the adoption numerator).
- Empty (`rows: 0`) → consulted-but-no-match = a REAL finding (leaf / additive / zero cross-crate blast), NOT
  "didn't query". Record it as such; do not re-probe.

<!-- Project-specific query learnings accumulate below via wrap curation. -->
