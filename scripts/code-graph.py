#!/usr/bin/env python3
"""code-graph.py — Andromeda code-graph pipeline (SCIP -> DuckDB, one DB per language plane).
Usage:
  python code-graph.py refresh [plane]
  python code-graph.py query <run_dir> <marker> "<sql>" [plane]
Planes are manifest-detected at run time: rust (root Cargo.toml; rust-analyzer) ·
ts (tracked tsconfig.json; scip-typescript). Each plane builds an independent
.andromeda/cache/<plane>/tree.db. Deps: the `duckdb` + `protobuf` pip packages + the
vendored scip_pb2.py (same dir). A missing indexer SKIPS that plane (recipe printed);
refresh writes .refresh-stale only when NO plane can build — exit 0 either way (never
blocks the pipeline). See integrity-protocol.md."""
import sys, os, json, time, hashlib, subprocess, shutil

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)  # vendored scip_pb2 + sibling code-graph-views.sql

# Plane registry: tool + install recipe + dirty-globs + manifest detection + indexer argv.
# Project args resolve against the PROCESS cwd (measured), so refresh chdir()s to the root
# and passes cwd= explicitly; indexer exe is the RESOLVED which() path (npm .cmd shims never
# launch via bare names under Windows CreateProcess).
PLANES = {
    "rust": {
        "tool": "rust-analyzer",
        "recipe": "install rust-analyzer (rustup component add rust-analyzer, or a release binary on PATH)",
        "globs": ["*.rs"],
        "scip": "index.scip",
        "argv": lambda exe, rt, out, roots: [exe, "scip", rt, "--output", out],
    },
    "ts": {
        "tool": "scip-typescript",
        "recipe": "npm i -g @sourcegraph/scip-typescript",
        "globs": ["*.ts", "*.tsx"],
        "scip": "index-ts.scip",
        "argv": lambda exe, rt, out, roots: [exe, "index", "--cwd", rt, "--output", out,
                                             "--no-progress-bar", *roots],
    },
}


def root():
    try:
        r = subprocess.run(["git", "rev-parse", "--show-toplevel"],
                           capture_output=True, text=True)
        if r.returncode == 0 and r.stdout.strip():
            return r.stdout.strip()
    except Exception:
        pass
    return os.getcwd()


def _git(*a):
    return subprocess.run(["git", *a], capture_output=True, text=True).stdout.strip()


def detect_planes(rt):
    """Manifest presence only. rust: root Cargo.toml (nested-Rust repos are a documented
    non-goal). ts: any TRACKED-or-untracked-unignored tsconfig.json (git ls-files — fast,
    gitignore-aware; roots = their dirs, exact filename only so tsconfig.node.json companions
    never become false roots; the indexer recurses project references itself)."""
    planes = {}
    if os.path.exists(os.path.join(rt, "Cargo.toml")):
        planes["rust"] = [rt]
    out = _git("ls-files", "--cached", "--others", "--exclude-standard",
               "--", "tsconfig.json", "*/tsconfig.json")
    roots = sorted({os.path.dirname(p) or "." for p in out.splitlines() if p.strip()})
    if roots:
        planes["ts"] = roots
    return planes


def span(r):
    # SCIP range: [sl, sc, ec] (single-line) or [sl, sc, el, ec] (multi-line)
    if len(r) == 3:
        return r[0], r[0]
    if len(r) == 4:
        return r[0], r[2]
    return None, None


def dirty_fp(rt, plane):
    """Fingerprint of this plane's uncommitted state (git pathspec wildcards match across
    directories — verified). Empty porcelain -> the CLEAN fingerprint."""
    out = _git("status", "--porcelain", "--", *PLANES[plane]["globs"])
    return hashlib.sha256(out.encode("utf-8")).hexdigest()[:16]


CLEAN_FP = hashlib.sha256(b"").hexdigest()[:16]


def _plane_dir(cache, plane):
    d = os.path.join(cache, plane)
    os.makedirs(d, exist_ok=True)
    return d


def _lock(pd):
    """Single-builder lock per plane dir. Stale (>15 min) locks are broken; a held lock
    returns False (caller decides: refresh skips the plane with a note, query cold-starts)."""
    lock = os.path.join(pd, ".refresh-running")
    try:
        if os.path.exists(lock) and time.time() - os.path.getmtime(lock) > 900:
            os.remove(lock)
        fd = os.open(lock, os.O_CREAT | os.O_EXCL | os.O_WRONLY)
        os.write(fd, f"{os.getpid()} {int(time.time())}".encode())
        os.close(fd)
        return True
    except OSError:
        return False


def _unlock(pd):
    try:
        os.remove(os.path.join(pd, ".refresh-running"))
    except OSError:
        pass


def build_plane(rt, cache, plane, roots):
    """Build ONE plane. Returns a status line for the aggregate sentinel."""
    spec = PLANES[plane]
    exe = shutil.which(spec["tool"])
    if not exe:
        return f"{plane} SKIPPED ({spec['tool']} not installed - {spec['recipe']})"
    pd = _plane_dir(cache, plane)
    if not _lock(pd):
        return f"{plane} SKIPPED (another build holds the lock)"
    try:
        t0 = time.time()
        scip_path = os.path.join(pd, spec["scip"])
        for f in os.listdir(pd):          # pre-delete transients: parse only THIS run's file
            if f.endswith(".scip"):
                os.remove(os.path.join(pd, f))
        r = subprocess.run(spec["argv"](exe, rt, scip_path, roots), cwd=rt,
                           stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        if r.returncode != 0 or not os.path.exists(scip_path):
            return f"{plane} SKIPPED (indexer failed rc={r.returncode})"

        import scip_pb2
        idx = scip_pb2.Index()
        with open(scip_path, "rb") as f:
            idx.ParseFromString(f.read())
        if not idx.documents:
            return f"{plane} SKIPPED (indexer produced 0 documents)"
        defs, occs, seen = [], [], set()
        for d in idx.documents:
            p = d.relative_path.replace("\\", "/")
            for occ in d.occurrences:
                if occ.symbol.startswith("local "):  # function-local noise
                    continue
                sl, _ = span(occ.range)
                if sl is None:
                    continue
                is_def = bool(occ.symbol_roles & 1)  # SymbolRole.Definition == 1
                key = (occ.symbol, p, sl, is_def)    # dedupe: overlapping tsconfig includes
                if key in seen:
                    continue
                seen.add(key)
                occs.append({"symbol": occ.symbol, "file": p, "line": sl, "is_def": is_def})
                if is_def:
                    es, ee = span(occ.enclosing_range) if occ.enclosing_range else (sl, sl)
                    defs.append({"symbol": occ.symbol, "file": p,
                                 "def_line": sl, "enc_start": es, "enc_end": ee})
        for name, rows in (("defs.ndjson", defs), ("occ.ndjson", occs)):
            with open(os.path.join(pd, name), "w", encoding="utf-8") as f:
                for rec in rows:
                    f.write(json.dumps(rec) + "\n")

        import duckdb
        db = os.path.join(pd, "tree.db")
        try:
            os.remove(db)
        except OSError:
            pass
        con = duckdb.connect(db)
        with open(os.path.join(HERE, "code-graph-views.sql"), encoding="utf-8") as f:
            sql = f.read().replace(".andromeda/cache/", f".andromeda/cache/{plane}/")
        con.execute(sql)  # duckdb runs the whole multi-statement script
        nodes = con.execute("SELECT count(*) FROM symbol").fetchone()[0]
        edges = con.execute("SELECT count(*) FROM refs").fetchone()[0]
        con.close()
        with open(os.path.join(pd, "built.head"), "w", encoding="utf-8") as f:
            f.write((_git("rev-parse", "HEAD") or "none") + "\n")
        with open(os.path.join(pd, "built.fp"), "w", encoding="utf-8") as f:
            f.write(dirty_fp(rt, plane) + "\n")
        secs = int(time.time() - t0)
        print(f"tree-refresh[{plane}]: {nodes} nodes / {edges} edges - {secs}s")
        return f"{plane} ok {secs}s {nodes}/{edges}"
    finally:
        _unlock(pd)


def refresh(only=None):
    rt = root()
    os.chdir(rt)
    cache = os.path.join(rt, ".andromeda", "cache")
    os.makedirs(cache, exist_ok=True)
    for s in (".refresh-done", ".refresh-stale"):
        try:
            os.remove(os.path.join(cache, s))
        except OSError:
            pass
    for legacy in ("tree.db", "index.scip", "defs.ndjson", "occ.ndjson"):  # pre-plane layout
        try:
            os.remove(os.path.join(cache, legacy))
        except OSError:
            pass

    def stale(msg):
        sys.stderr.write(f"tree-refresh: STALE - {msg}\n")
        with open(os.path.join(cache, ".refresh-stale"), "w", encoding="utf-8") as f:
            f.write(msg + "\n")
        sys.exit(0)

    try:
        import duckdb  # noqa: F401
    except Exception:
        stale("python 'duckdb' package not installed (pip install duckdb)")
    try:
        import scip_pb2  # noqa: F401
    except Exception as e:
        stale(f"scip_pb2/protobuf not importable ({e}); pip install protobuf")

    planes = detect_planes(rt)
    if only:
        if only not in planes:
            stale(f"plane '{only}' not detected (manifests found for: {', '.join(planes) or 'none'})")
        planes = {only: planes[only]}
    if not planes:
        stale("no indexable plane detected (no root Cargo.toml, no tracked tsconfig.json)")

    lines = [build_plane(rt, cache, plane, roots) for plane, roots in planes.items()]
    built = [ln for ln in lines if " ok " in ln]
    for ln in lines:
        if " ok " not in ln:
            sys.stderr.write(f"tree-refresh: {ln}\n")
    if not built:
        stale("; ".join(lines))
    with open(os.path.join(cache, ".refresh-done"), "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")


def _append_trace(trace, record):
    """Accumulate one query-record into the trace (read-append-rewrite).
    Tolerates an absent file and a legacy bare-result-array (migrates to []-of-records).
    Assumes SEQUENTIAL single-process invocation (the phase agent issues queries one at a
    time) — do NOT parallelize the `query` command, or concurrent appends race."""
    prior = []
    try:
        with open(trace, encoding="utf-8") as f:
            existing = json.load(f)
        if isinstance(existing, list):
            prior = [r for r in existing
                     if isinstance(r, dict) and "rows" in r and "result" in r and "db_state" in r]
    except (OSError, ValueError):
        prior = []  # absent / corrupt / legacy -> start fresh (audit artifact: never crash)
    prior.append(record)
    with open(trace, "w", encoding="utf-8") as f:
        json.dump(prior, f, default=str, indent=0)


def query(run_dir, marker, sql, plane=None):
    rt = root()
    os.chdir(rt)
    cache = os.path.join(rt, ".andromeda", "cache")
    trace = os.path.join(run_dir, f"tree-query-{marker}.json")
    planes = detect_planes(rt)
    if plane is None:
        if len(planes) == 1:
            plane = next(iter(planes))
        elif not planes:
            plane = "rust"  # cold-start path below records honestly
        else:
            sys.exit(f"multiple planes detected ({', '.join(sorted(planes))}) - "
                     f"name one: code-graph.py query <run_dir> <marker> \"<sql>\" <plane>")
    elif planes and plane not in planes:
        sys.exit(f"plane '{plane}' not detected (found: {', '.join(sorted(planes))})")
    pd = os.path.join(cache, plane)
    db = os.path.join(pd, "tree.db")
    for other in planes:
        if other != plane and not os.path.exists(os.path.join(cache, other, "tree.db")):
            sys.stderr.write(f"tree-query: note - plane '{other}' is detected but has no DB "
                             f"(unbuilt or its indexer is missing)\n")

    # Per-plane freshness: OWN (built.head == HEAD and built.fp == current) or
    # VOUCHED (root stamp == HEAD and the plane's current fingerprint is clean).
    head = _git("rev-parse", "HEAD") or "none"
    fp = dirty_fp(rt, plane)

    def _read(p):
        try:
            with open(p, encoding="utf-8") as f:
                return f.read().strip()
        except OSError:
            return "none"

    own = _read(os.path.join(pd, "built.head")) == head and _read(os.path.join(pd, "built.fp")) == fp
    vouched = _read(os.path.join(cache, "tree.db.commit")) == head and fp == CLEAN_FP
    db_state = "fresh"
    if (not os.path.exists(db)) or not (own or vouched):
        sys.stderr.write(f"tree-query: {plane} DB absent/stale -> regenerating...\n")
        db_state = "regenerated"
        try:
            refresh(only=plane)       # exit(0)s on tool-missing/stale; keep query alive
        except SystemExit:
            pass
        if os.path.exists(db):
            if fp == CLEAN_FP:        # self-stamp only what the stamp can honestly mean
                with open(os.path.join(cache, "tree.db.commit"), "w", encoding="utf-8") as f:
                    f.write(head + "\n")
        else:
            db_state = "cold-start"

    # Cold-start / plane unbuilt: record the empty consultation (present-but-empty is valid).
    if not os.path.exists(db):
        _append_trace(trace, {"sql": sql, "rows": 0, "result": [],
                              "db_state": "cold-start", "plane": plane})
        sys.stderr.write(f"tree-query: no {plane} DB (cold-start / indexer missing) - "
                         f"empty result recorded\n")
        return

    import duckdb
    con = duckdb.connect(db, read_only=True)
    rows = con.execute(sql).fetchall()
    cols = [c[0] for c in con.description] if con.description else []
    con.close()
    results = [dict(zip(cols, r)) for r in rows]
    _append_trace(trace, {"sql": sql, "rows": len(results), "result": results,
                          "db_state": db_state, "plane": plane})
    print(json.dumps(results, default=str, indent=2))


def main():
    if len(sys.argv) < 2:
        sys.exit("usage: code-graph.py {refresh [plane] | query <run_dir> <marker> \"<sql>\" [plane]}")
    cmd = sys.argv[1]
    if cmd == "refresh":
        refresh(only=sys.argv[2] if len(sys.argv) > 2 else None)
    elif cmd == "query":
        if len(sys.argv) < 5:
            sys.exit("usage: code-graph.py query <run_dir> <marker> \"<sql>\" [plane]")
        query(sys.argv[2], sys.argv[3], sys.argv[4],
              sys.argv[5] if len(sys.argv) > 5 else None)
    else:
        sys.exit(f"unknown subcommand: {cmd}")


if __name__ == "__main__":
    main()
