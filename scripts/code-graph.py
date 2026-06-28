#!/usr/bin/env python3
"""code-graph.py — Andromeda code-graph pipeline (rust-analyzer SCIP -> DuckDB).
Usage:
  python code-graph.py refresh
  python code-graph.py query <run_dir> <marker> "<sql>"
Deps: the `duckdb` + `protobuf` pip packages + the vendored scip_pb2.py (same dir);
rust-analyzer on PATH. A missing tool makes `refresh` write .refresh-stale and exit 0
(never blocks the pipeline). See .andromeda/ integrity-protocol.md."""
import sys, os, json, time, subprocess, shutil

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)  # vendored scip_pb2 + sibling code-graph-views.sql


def root():
    try:
        r = subprocess.run(["git", "rev-parse", "--show-toplevel"],
                           capture_output=True, text=True)
        if r.returncode == 0 and r.stdout.strip():
            return r.stdout.strip()
    except Exception:
        pass
    return os.getcwd()


def span(r):
    # SCIP range: [sl, sc, ec] (single-line) or [sl, sc, el, ec] (multi-line)
    if len(r) == 3:
        return r[0], r[0]
    if len(r) == 4:
        return r[0], r[2]
    return None, None


def _git(*a):
    return subprocess.run(["git", *a], capture_output=True, text=True).stdout.strip()


def refresh():
    rt = root()
    os.chdir(rt)
    cache = os.path.join(rt, ".andromeda", "cache")
    os.makedirs(cache, exist_ok=True)
    for s in (".refresh-done", ".refresh-stale"):
        try:
            os.remove(os.path.join(cache, s))
        except OSError:
            pass

    def stale(msg):
        sys.stderr.write(f"tree-refresh: STALE - {msg}\n")
        with open(os.path.join(cache, ".refresh-stale"), "w", encoding="utf-8") as f:
            f.write(msg + "\n")
        sys.exit(0)

    if not shutil.which("rust-analyzer"):
        stale("rust-analyzer not installed")
    try:
        import duckdb  # noqa: F401
    except Exception:
        stale("python 'duckdb' package not installed (pip install duckdb)")
    try:
        import scip_pb2
    except Exception as e:
        stale(f"scip_pb2/protobuf not importable ({e}); pip install protobuf")

    t0 = time.time()
    scip_path = os.path.join(cache, "index.scip")
    rc = subprocess.run(["rust-analyzer", "scip", rt, "--output", scip_path],
                        stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL).returncode
    if rc != 0 or not os.path.exists(scip_path):
        stale("rust-analyzer scip failed")

    idx = scip_pb2.Index()
    with open(scip_path, "rb") as f:
        idx.ParseFromString(f.read())
    defs, occs = [], []
    for d in idx.documents:
        p = d.relative_path.replace("\\", "/")
        for occ in d.occurrences:
            if occ.symbol.startswith("local "):  # skip function-local symbols (noise)
                continue
            sl, _ = span(occ.range)
            if sl is None:
                continue
            is_def = bool(occ.symbol_roles & 1)  # SymbolRole.Definition == 1
            occs.append({"symbol": occ.symbol, "file": p, "line": sl, "is_def": is_def})
            if is_def:
                es, ee = span(occ.enclosing_range) if occ.enclosing_range else (sl, sl)
                defs.append({"symbol": occ.symbol, "file": p,
                             "def_line": sl, "enc_start": es, "enc_end": ee})
    with open(os.path.join(cache, "defs.ndjson"), "w", encoding="utf-8") as f:
        for r in defs:
            f.write(json.dumps(r) + "\n")
    with open(os.path.join(cache, "occ.ndjson"), "w", encoding="utf-8") as f:
        for r in occs:
            f.write(json.dumps(r) + "\n")

    import duckdb
    db = os.path.join(cache, "tree.db")
    try:
        os.remove(db)
    except OSError:
        pass
    con = duckdb.connect(db)
    with open(os.path.join(HERE, "code-graph-views.sql"), encoding="utf-8") as f:
        con.execute(f.read())  # duckdb runs the whole multi-statement script (comments + ; handled)
    nodes = con.execute("SELECT count(*) FROM symbol").fetchone()[0]
    edges = con.execute("SELECT count(*) FROM refs").fetchone()[0]
    con.close()
    secs = int(time.time() - t0)
    print(f"tree-refresh: {nodes} nodes / {edges} edges - {secs}s")
    with open(os.path.join(cache, ".refresh-done"), "w", encoding="utf-8") as f:
        f.write(f"done {secs}s {nodes}/{edges}\n")


def _append_trace(trace, record):
    """Accumulate one query-record into the trace (read-append-rewrite).
    Tolerates an absent file and a legacy bare-result-array (migrates to []-of-records).
    Assumes SEQUENTIAL single-process invocation (the phase agent issues queries one at a
    time) — do NOT parallelize the `query` command, or concurrent appends race (re-opens #15)."""
    prior = []
    try:
        with open(trace, encoding="utf-8") as f:
            existing = json.load(f)
        if isinstance(existing, list):
            prior = [r for r in existing
                     if isinstance(r, dict) and "rows" in r and "result" in r and "db_state" in r]
    except (OSError, ValueError):
        prior = []  # absent / corrupt / legacy bare-array -> start fresh (audit artifact: never crash)
    prior.append(record)
    with open(trace, "w", encoding="utf-8") as f:
        json.dump(prior, f, default=str, indent=0)


def query(run_dir, marker, sql):
    rt = root()
    os.chdir(rt)
    cache = os.path.join(rt, ".andromeda", "cache")
    db = os.path.join(cache, "tree.db")
    trace = os.path.join(run_dir, f"tree-query-{marker}.json")

    # Freshness -> regenerate if: DB absent, stamped commit != HEAD, or uncommitted *.rs changes.
    head = _git("rev-parse", "HEAD") or "none"
    try:
        with open(os.path.join(cache, "tree.db.commit"), encoding="utf-8") as f:
            stamp = f.read().strip()
    except OSError:
        stamp = "none"
    dirty = bool(_git("status", "--porcelain", "--", "*.rs"))
    db_state = "fresh"
    if (not os.path.exists(db)) or stamp != head or dirty:
        sys.stderr.write("tree-query: DB absent/stale -> regenerating...\n")
        db_state = "regenerated"
        try:
            refresh()                 # refresh() sys.exit(0)s on tool-missing/stale; keep query alive
        except SystemExit:
            pass
        if os.path.exists(db):
            with open(os.path.join(cache, "tree.db.commit"), "w", encoding="utf-8") as f:
                f.write(head + "\n")
        else:
            db_state = "cold-start"

    # Cold-start / refresh-stale: no DB -> record an empty consultation (present-but-empty is valid, != missing).
    if not os.path.exists(db):
        _append_trace(trace, {"sql": sql, "rows": 0, "result": [], "db_state": "cold-start"})
        sys.stderr.write("tree-query: no DB (cold-start / refresh stale) - empty result recorded\n")
        return

    import duckdb
    con = duckdb.connect(db, read_only=True)
    rows = con.execute(sql).fetchall()
    cols = [c[0] for c in con.description] if con.description else []
    con.close()
    results = [dict(zip(cols, r)) for r in rows]
    _append_trace(trace, {"sql": sql, "rows": len(results), "result": results, "db_state": db_state})
    print(json.dumps(results, default=str, indent=2))


def main():
    if len(sys.argv) < 2:
        sys.exit("usage: code-graph.py {refresh | query <run_dir> <marker> \"<sql>\"}")
    cmd = sys.argv[1]
    if cmd == "refresh":
        refresh()
    elif cmd == "query":
        if len(sys.argv) < 5:
            sys.exit("usage: code-graph.py query <run_dir> <marker> \"<sql>\"")
        query(sys.argv[2], sys.argv[3], sys.argv[4])
    else:
        sys.exit(f"unknown subcommand: {cmd}")


if __name__ == "__main__":
    main()
