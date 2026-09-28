-- code-graph-views.sql — load NDJSON + build the code-graph query relations.
-- Idempotent (CREATE OR REPLACE). Executed by code-graph.py with CWD at the project root.

CREATE OR REPLACE TABLE defs AS SELECT * FROM read_json_auto('.andromeda/cache/defs.ndjson');
CREATE OR REPLACE TABLE occ  AS SELECT * FROM read_json_auto('.andromeda/cache/occ.ndjson');

-- workspace-defined symbols (the indexed documents ARE the workspace)
CREATE OR REPLACE TABLE wsdef AS SELECT DISTINCT symbol FROM defs;

-- Symbol NAME + KIND, derived once for every plane from the SCIP descriptor tail:
-- SCIP symbol = "<scheme> <manager> <package> <version> <descriptors>" -> crate = field 3 (a cargo crate on the
-- rust plane, an npm package on the ts plane); the LAST descriptor is <name><marker> with marker '().' fn/method
-- · '#' type · '.' term · '/' module-or-file (TS files are backticked) · '!' macro · ':' meta · ')' parameter
-- · ']' type parameter. Build-time only: the relations below are TABLES, so no query depends on these macros.
CREATE OR REPLACE MACRO sym_name(s) AS
  replace(regexp_extract(s, '(?:^|[/#\]\.:\s\)\(\[])(\x60[^\x60]+\x60|[A-Za-z_\$][A-Za-z0-9_\$]*)(\(\)\.|#|\.|/|!|:|\)|\])$', 1), chr(96), '');
CREATE OR REPLACE MACRO sym_kind(s) AS
  CASE regexp_extract(s, '(?:^|[/#\]\.:\s\)\(\[])(\x60[^\x60]+\x60|[A-Za-z_\$][A-Za-z0-9_\$]*)(\(\)\.|#|\.|/|!|:|\)|\])$', 2)
    WHEN '().' THEN 'fn' WHEN '#' THEN 'type' WHEN '.' THEN 'term' WHEN '/' THEN 'module'
    WHEN '!' THEN 'macro' WHEN ':' THEN 'meta' WHEN ')' THEN 'param' WHEN ']' THEN 'typeparam' ELSE '' END;

-- symbol: the node relation — symbol + parsed crate + derived name/kind + its definition file:line
CREATE OR REPLACE TABLE symbol AS
  SELECT symbol, split_part(symbol, ' ', 3) AS crate, sym_name(symbol) AS name, sym_kind(symbol) AS kind,
         file, def_line
  FROM defs;

-- refs: non-def occurrences pointing at a workspace-defined symbol (resolved intra-workspace edges),
-- carrying the callee's derived columns + its definition file (one row per symbol: a symbol may define in
-- several files, e.g. a module root)
CREATE OR REPLACE TABLE refs AS
  SELECT o.symbol AS callee, s.name AS callee_name, s.kind AS callee_kind, s.crate AS callee_crate,
         s.file AS callee_file, o.file, o.line
  FROM occ o JOIN (SELECT symbol, any_value(crate) AS crate, any_value(name) AS name,
                          any_value(kind) AS kind, min(file) AS file
                   FROM symbol GROUP BY symbol) s ON s.symbol = o.symbol
  WHERE o.is_def = FALSE;

-- calls: a ref contained in a definition's enclosing range (innermost def wins) -> caller -> callee
CREATE OR REPLACE TABLE calls AS
  SELECT d.symbol AS caller, sym_name(d.symbol) AS caller_name, r.callee, r.callee_name, r.callee_kind,
         r.callee_crate, r.callee_file, r.file, r.line, (d.enc_end - d.enc_start) AS span
  FROM refs r JOIN defs d
    ON r.file = d.file AND r.line >= d.enc_start AND r.line <= d.enc_end
  QUALIFY ROW_NUMBER() OVER (PARTITION BY r.file, r.line, r.callee ORDER BY span ASC) = 1;

-- calls_m: the same table under its contract name (recursive blast-radius CTEs)
CREATE OR REPLACE TABLE calls_m AS SELECT * FROM calls;

-- contains: definition nesting via range containment (innermost parent), e.g. method -> impl/type
CREATE OR REPLACE VIEW contains AS
  SELECT p.symbol AS parent, c.symbol AS child
  FROM defs p JOIN defs c
    ON p.file = c.file AND p.symbol <> c.symbol
   AND c.enc_start >= p.enc_start AND c.enc_end <= p.enc_end
   AND (p.enc_end - p.enc_start) > (c.enc_end - c.enc_start)
  QUALIFY ROW_NUMBER() OVER (PARTITION BY c.symbol ORDER BY (p.enc_end - p.enc_start) ASC) = 1;

-- crate_edges: usage-based crate dependency graph (resolved, not Cargo.toml-declared)
CREATE OR REPLACE VIEW crate_edges AS
  SELECT DISTINCT split_part(caller, ' ', 3) AS from_crate,
                  split_part(callee, ' ', 3) AS to_crate
  FROM calls_m
  WHERE split_part(caller, ' ', 3) <> split_part(callee, ' ', 3);
