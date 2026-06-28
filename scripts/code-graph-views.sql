-- code-graph-views.sql — load NDJSON + build the code-graph query views.
-- Idempotent (CREATE OR REPLACE). Executed by code-graph.py with CWD at the project root.

CREATE OR REPLACE TABLE defs AS SELECT * FROM read_json_auto('.andromeda/cache/defs.ndjson');
CREATE OR REPLACE TABLE occ  AS SELECT * FROM read_json_auto('.andromeda/cache/occ.ndjson');

-- workspace-defined symbols (the indexed documents ARE the workspace)
CREATE OR REPLACE TABLE wsdef AS SELECT DISTINCT symbol FROM defs;

-- symbol: the node view — symbol + parsed crate + its definition file:line
-- SCIP symbol = "<scheme> <manager> <package> <version> <descriptors>" -> crate = field 3.
CREATE OR REPLACE VIEW symbol AS
  SELECT symbol, split_part(symbol, ' ', 3) AS crate, file, def_line
  FROM defs;

-- refs: non-def occurrences pointing at a workspace-defined symbol (resolved intra-workspace edges)
CREATE OR REPLACE VIEW refs AS
  SELECT o.symbol AS callee, o.file, o.line
  FROM occ o
  WHERE o.is_def = FALSE AND o.symbol IN (SELECT symbol FROM wsdef);

-- calls: a ref contained in a definition's enclosing range (innermost def wins) -> caller -> callee
CREATE OR REPLACE VIEW calls AS
  SELECT d.symbol AS caller, r.callee, r.file, r.line, (d.enc_end - d.enc_start) AS span
  FROM refs r JOIN defs d
    ON r.file = d.file AND r.line >= d.enc_start AND r.line <= d.enc_end
  QUALIFY ROW_NUMBER() OVER (PARTITION BY r.file, r.line, r.callee ORDER BY span ASC) = 1;

-- materialize calls for fast recursive blast-radius CTEs
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
