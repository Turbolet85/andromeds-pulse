# Code-graph query cookbook

Quick-reference for querying this project's code-graph (DuckDB symbol graph) — the schema + canonical query shapes
so you don't reinvent them. The relation definitions live in `scripts/code-graph-views.sql` (authoritative).
(Setup re-seeds this file by replacing everything ABOVE the learnings marker at the bottom; the tail is preserved.)

## Invocation
`python scripts/code-graph.py query <run_dir> <marker> "<sql>" [plane]` — runs the SQL against ONE plane's DB,
prints rows to stdout, and APPENDS a `{sql, rows, result, db_state, plane, probe_hits}` record to
`<run_dir>/tree-query-<marker>.json` (the adoption trace; regenerates that plane on miss/stale/changed-views
first). `probe_hits` is filled on a 0-row `calls`/`calls_m`/`refs`/`contains` query — see Reading the result.
**Planes:** the graph is one independent DB per indexed language (`rust` · `ts`), manifest-detected. With one
plane the arg is optional; with several it is REQUIRED — pick the plane your modify-set touches, and for a seam
question query BOTH (see the name-bridge below). Cold-start / empty DB (early chunk, no symbols): skip the query,
note it, derive from arch + extracts.

## Schema (the six query relations — identical on every plane)
| Relation | Columns | What it is |
|---|---|---|
| `symbol` | `symbol, crate, name, kind, file, def_line` | every workspace-defined symbol + its package, derived name/kind, def site |
| `refs` | `callee, callee_name, callee_kind, callee_crate, callee_file, file, line` | every reference TO a workspace symbol (key on **`callee_name`**) |
| `calls` | `caller, caller_name, callee, callee_name, callee_kind, callee_crate, callee_file, file, line` | resolved caller→callee edges (innermost enclosing def) |
| `calls_m` | (= `calls`) | the contract name for recursive blast-radius CTEs |
| `contains` | `parent, child` | definition nesting (method → impl/type) |
| `crate_edges` | `from_crate, to_crate` | usage-based package deps (resolved, NOT manifest-declared) |

**`defs` / `occ` / `wsdef` are raw internal load tables** (NDJSON import + a distinct-symbol helper) — never query
them directly; always use the six relations above. In ad-hoc SQL an alias needs `AS` (`sym AS name`, never
`sym name`).

**Query by NAME, never by a plane's path grammar.** `name` / `callee_name` / `caller_name` are the last
descriptor's identifier, derived by ONE rule for every plane; `kind` is the SCIP marker class, not the language's:
`fn` (function / method) · `type` (also an enum variant) · `term` (field / variable / constant) · `module` (a
Rust module or a TS file — file names keep their extension, so module names are the one class NOT unified across
planes) · `macro` · `meta` (scip-typescript's anonymous object / type-literal members, named `color0`, `margin4` —
41–54 % of a ts plane; exclude them from a name search) · `param` · `typeparam`. A name may carry quotes
(`'sr-error'0`, `"aria-label"`) — double the quote in a SQL literal. The raw `symbol` string is the fully-qualified
SCIP form `<scheme> <manager> <package> <version> <descriptors>` — a cargo crate or an npm package in field 3
(`crate`) — e.g. `rust-analyzer cargo conductor-core 0.1.0 lamp/impl#[Lamp]for_record().` or
``scip-typescript npm conductor-ui 0.1.0 src/`lamp.ts`/lampForRecord().``. Its path grammar DIFFERS per plane
(rust: `/` module · `]` impl method · `#` type member · nothing at crate root; ts: the file path, directory outside
the backticks, filename inside) — never the KEY of an impact query (measured: seven "leaf" answers were path
patterns shaped for the other plane); a rust module-path filter (`AND callee LIKE '%<module>/%'`) is the last
rung of the collision ladder below, after `callee_kind` and `callee_file`.

## Canonical queries (copy + adapt — `<name>` is the bare identifier)
```sql
-- 1. IMPACT — who calls a symbol I'm about to change
SELECT caller, file, line FROM calls WHERE callee_name = '<name>' AND callee_kind = 'fn' ORDER BY file, line;
--    name collision (new / resolve / default …): AND callee_file LIKE '%<file>'   (the DEFINITION's path,
--    same semantics on every plane)  ·  then AND callee_crate = '<crate>'  ·  rust only: AND callee LIKE '%<module>/%'
--    all references incl. non-call uses: SELECT callee, file, line FROM refs WHERE callee_name = '<name>'

-- 2. CRATE EDGES — does my crate have cross-crate consumers (inbound) or deps (outbound)?
SELECT from_crate, to_crate FROM crate_edges WHERE to_crate = '<crate>' OR from_crate = '<crate>';
--    no inbound rows = leaf crate; an additive change has zero cross-crate blast radius

-- 3. RECURSIVE BLAST-RADIUS — transitive callers up to N hops (base case RESOLVES the name to exact symbols;
--    guard a collision with kind + file, or the union is meaningless)
WITH RECURSIVE b(sym, depth) AS (
  SELECT symbol, 0 FROM symbol WHERE name = '<name>' AND kind = 'fn'
  UNION
  SELECT c.caller, b.depth + 1 FROM calls_m c JOIN b ON c.callee = b.sym WHERE b.depth < 3)
SELECT DISTINCT sym, depth FROM b ORDER BY depth;

-- 4. EXISTENCE / COLLISION-CHECK — is a name free before I create a new module/type? which symbols carry it?
SELECT symbol, crate, kind, file FROM symbol WHERE name = '<name>' AND kind <> 'meta';
--    partial name: WHERE name LIKE '<prefix>%'

-- 5. EXTERNAL SURFACE — a package's pub items reached from OUTSIDE its source dir (other packages, its own
--    tests/): the byte-stability oracle before a split or rename. callee_file is the DEFINITION's path, so
--    "outside its src/" derives from it, never from the package name; `src/` · `tests/` are the cargo layout —
--    substitute the plane's (ts: the package's root dir); the package-root `module` row is every `use` path
SELECT callee_name, callee_kind, callee_file, COUNT(*) AS sites, COUNT(DISTINCT file) AS files
FROM refs WHERE callee_crate = '<crate>' AND callee_file LIKE '%/src/%'
  AND file NOT LIKE regexp_replace(callee_file, '/src/.*$', '/src/%') AND callee_kind <> 'module'
GROUP BY 1, 2, 3 ORDER BY sites DESC, callee_name;
--    who reaches it, by directory: SELECT regexp_extract(file, '^(.*/(src|tests|benches|examples))/', 1) AS from_dir,
--    COUNT(*) FROM refs WHERE <the same three predicates> GROUP BY 1 ORDER BY 2 DESC
```

## The cross-plane seam (IPC / TauRPC / bindings)
Planes are disjoint subgraphs — no edge crosses a language boundary. But generated bindings (e.g.
`ui/src/bindings/index.ts`) ARE indexed on the ts plane, so frontend callers of an IPC procedure are visible up
to the binding symbol; only the binding↔backend hop is missing. For seam impact run the **name-bridge**: query
BOTH planes with the SAME `callee_name = '<name>'` (fn / type names are plane-agnostic; module names are not) and
join the two result sets by hand. Corollary: **zero callers on an IPC-facing or runtime-invoked symbol is NOT
dead-code evidence** — the caller may live on the other plane or in a runtime dispatch the graph cannot see.

## Reading the result
- Non-empty → the impacted sites; cite `symbol @ file:line` in `plan.md` (the adoption numerator) — `line` / `def_line` are the SCIP range's 0-INDEXED start (code-graph.py `span()`): the editor line is `line + 1`, cite THAT (measured: three citations off by one in one plan, operator-caught; the same slip in three more chunks this epoch).
- Read the result from the trace file or the whole stdout — never through `| head` / `| tail`: a clipped view has
  produced a false count twice; the trace's `rows` is the authority and IS the record.
- Empty (`rows: 0`) → consulted-but-no-match is a REAL finding (leaf / additive / zero cross-crate blast), NOT
  "didn't query" — but ONLY after two preconditions: (a) the plane you queried actually BUILT (`db_state` was
  not `cold-start`, and no skipped-plane notice named it), and (b) every name / pattern the query used exists on
  this plane — the script probes each against `symbol` and records `probe_hits` (`{"names": {"<name>": N},
  "patterns": {"<pattern>": N}}`) in the trace: all > 0 → a genuine no-callers; any 0 → NOT a leaf (the symbol
  lives on another plane, under another spelling, only at runtime — or the index MISSED its definition: a name
  whose definition a grep finds in this plane's own source is an index gap (measured on one indexer:
  rust-analyzer's SCIP left 6 of 79 async fns without a `symbol` row while `occ` held their def-site occurrence,
  0 of 1069 non-async — the cause unmeasured; two such fns returned 0 rows against grepped production callers);
  a warning names it; grep the def site for its callers before concluding). Record which.
- A skipped/unbuilt plane in play → the research proceeds file-first and records `derived-without-graph`, never
  "leaf".

<!-- Project-specific query learnings accumulate below via wrap curation. -->
