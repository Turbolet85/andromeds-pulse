#!/usr/bin/env python
"""Pinned summarizers -> c-{metric}.json (code-audit baseline). Run by path."""
import json, os, csv, math, shutil, re
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))


def norm(p):
    p = (p or '').replace('\\', '/')
    if p.startswith('./'):
        p = p[2:]
    return p


def nearest_rank(vals, q):
    if not vals:
        return 0
    s = sorted(vals)
    k = max(1, math.ceil(q * len(s)))
    return s[k - 1]


def is_test_path(p):
    p = norm(p)
    return ('/tests/' in p or p.startswith('tests/') or '/tests-' in p
            or '.test.' in p or '.spec.' in p)


def dump(name, obj):
    with open(os.path.join(HERE, name), 'w', encoding='utf-8') as f:
        json.dump(obj, f, ensure_ascii=False, indent=1)


population = set(norm(l.strip()) for l in open(os.path.join(HERE, 'population.txt'), encoding='utf-8') if l.strip())

# ---------- A3 sizes (tokei) ----------
tk = json.load(open(os.path.join(HERE, 'tokei.json'), encoding='utf-8'))
per_file = {}
for lang, data in tk.items():
    if lang == 'Total' or not isinstance(data, dict):
        continue
    for rep in data.get('reports', []):
        p = norm(rep.get('name'))
        if p in population:
            per_file[p] = per_file.get(p, 0) + rep['stats']['code']
vals = list(per_file.values())
top10 = sorted(per_file.items(), key=lambda t: -t[1])[:10]
sizes = {'file_p50': nearest_rank(vals, .5), 'file_p90': nearest_rank(vals, .9),
         'file_max': max(vals) if vals else 0, 'over_800': sum(1 for v in vals if v > 800),
         'top': [[f, v] for f, v in top10],
         'totals': {'loc': sum(vals), 'files': len(vals)},
         'population': 'git-tracked *.rs/*.ts/*.tsx under crates/ pulse-app/ xtask/ minus pulse-app/ui/src/bindings/ (generated)'}
units = sorted(set(['crates/' + p.split('/')[1] if p.startswith('crates/') else p.split('/')[0] for p in population]))
sizes['units'] = len(units)
dump('c-sizes.json', sizes)
print(f"sizes: {sizes['totals']['files']} files · {sizes['totals']['loc']} loc · p50 {sizes['file_p50']} p90 {sizes['file_p90']} max {sizes['file_max']} · >800: {sizes['over_800']} · units {sizes['units']}")
for f, v in top10[:5]:
    print(f"   top: {f} {v}")

# ---------- A1 duplication (jscpd2) ----------
rep = json.load(open(os.path.join(HERE, 'jscpd2', 'jscpd-report.json'), encoding='utf-8'))
dups = rep.get('duplicates', [])
st = (rep.get('statistics') or {}).get('total') or {}
clone_rows = []
split = {'src': 0, 'test': 0, 'mixed': 0}
for d in dups:
    a, b = norm(d['firstFile']['name']), norm(d['secondFile']['name'])
    lines = d.get('lines') or (d['firstFile'].get('end', 0) - d['firstFile'].get('start', 0) + 1)
    ta, tb = is_test_path(a), is_test_path(b)
    cls = 'test' if (ta and tb) else ('src' if (not ta and not tb) else 'mixed')
    split[cls] += 1
    clone_rows.append((a, b, lines, cls))
clone_rows.sort(key=lambda t: -t[2])
dup = {'pct': st.get('percentage'), 'duplicated_lines': st.get('duplicatedLines'),
       'total_lines': st.get('lines'), 'clones': st.get('clones', len(dups)),
       'stats_derived': not bool(st),
       'top': [[a, b, l, c] for a, b, l, c in clone_rows[:10]], 'split': split}
dump('c-duplication.json', dup)
print(f"duplication: pct {dup['pct']} · clones {dup['clones']} · dup_lines {dup['duplicated_lines']}/{dup['total_lines']} · split {split}")
for a, b, l, c in clone_rows[:5]:
    print(f"   clone[{c}] {l}L  {a} <> {b}")

# ---------- A2 complexity (rca + lizard) ----------
rust_fns = []


def walk(space, file):
    for sp in space.get('spaces', []):
        if sp.get('kind') == 'function':
            m = sp.get('metrics', {})
            cy = (m.get('cyclomatic') or {}).get('sum') or 0
            cog = (m.get('cognitive') or {}).get('sum') or 0
            rust_fns.append({'fn': sp.get('name') or '<closure>', 'file': file,
                             'cyclo': cy, 'cog': cog})
        walk(sp, file)


rca_dir = os.path.join(HERE, 'rca')
for root, _, files in os.walk(rca_dir):
    for fn in files:
        if fn.endswith('.json'):
            try:
                j = json.load(open(os.path.join(root, fn), encoding='utf-8'))
            except ValueError:
                continue
            walk(j, norm(j.get('name', fn)))

ts_fns = []
with open(os.path.join(HERE, 'lizard.csv'), encoding='utf-8', errors='replace') as f:
    for row in csv.reader(f):
        if len(row) >= 8:
            try:
                ccn = int(row[1])
            except ValueError:
                continue
            ts_fns.append({'fn': row[7] if len(row) > 7 else row[6], 'file': norm(row[6]), 'ccn': ccn})

cyc_all = [f['cyclo'] for f in rust_fns] + [f['ccn'] for f in ts_fns]
cog_rust = [f['cog'] for f in rust_fns]
over = sum(1 for f in rust_fns if f['cog'] > 15) + sum(1 for f in ts_fns if f['ccn'] > 15)
offenders = ([{'fn': f['fn'], 'file': f['file'], 'value': f['cog'], 'metric': 'cognitive'} for f in rust_fns]
             + [{'fn': f['fn'], 'file': f['file'], 'value': f['ccn'], 'metric': 'cyclomatic'} for f in ts_fns])
offenders.sort(key=lambda o: -o['value'])
comp = {'functions_rust': len(rust_fns), 'functions_ts': len(ts_fns),
        'cyclomatic_p50': nearest_rank(cyc_all, .5), 'cyclomatic_p90': nearest_rank(cyc_all, .9),
        'cyclomatic_max': max(cyc_all) if cyc_all else 0,
        'cognitive_p50': nearest_rank(cog_rust, .5), 'cognitive_p90': nearest_rank(cog_rust, .9),
        'cognitive_max': max(cog_rust) if cog_rust else 0,
        'cognitive_population': 'rust functions only (lizard has no cognitive — noted)',
        'over_ceiling': over, 'ceiling': 'cognitive>15 (rust) + cyclomatic>15 (ts)',
        'max': {'fn': offenders[0]['fn'], 'file': offenders[0]['file'], 'val': offenders[0]['value']} if offenders else None,
        'top': offenders[:10]}
dump('c-complexity.json', comp)
print(f"complexity: fns rust {len(rust_fns)} / ts {len(ts_fns)} · cyclo p50/p90/max {comp['cyclomatic_p50']}/{comp['cyclomatic_p90']}/{comp['cyclomatic_max']} · cog p50/p90/max {comp['cognitive_p50']}/{comp['cognitive_p90']}/{comp['cognitive_max']} · over ceiling {over}")
for o in offenders[:8]:
    print(f"   top: {o['value']:>3} {o['metric'][:3]} {o['fn']}  ({o['file']})")

# ---------- A4 graph ----------
def gread(name):
    p = os.path.join(HERE, name)
    try:
        return json.load(open(p, encoding='utf-8'))
    except Exception:
        return None


def short(sym):
    s = sym.replace('rust-analyzer cargo ', '')
    return re.sub(r' 0\.\d+\.\d+ ', '::', s)


cycles = gread('g-cycles-rust.txt') or []
fanin = gread('g-fanin-rust.txt') or []
fanout = gread('g-fanout-rust.txt') or []
edges = (gread('g-edges-rust.txt') or [{}])[0].get('edges', 0)
ts_edges = (gread('g-edges-ts.txt') or [{}])[0].get('edges', 0)
graph = {'cycles': len(cycles), 'cycle_paths': cycles,
         'fan_in_top': [[short(r['callee']), r['n']] for r in fanin][:20],
         'fan_out': [[r['from_crate'], r['n']] for r in fanout],
         'cross_unit_edges': edges,
         'ts_plane_note': f'ts plane crate_edges = {ts_edges} (single-package plane — module-level cycle metric not applicable)'}
dump('c-graph.json', graph)
print(f"graph: cycles {graph['cycles']} · cross-unit edges {edges} · fan-out head {graph['fan_out'][:3]} · ts edges {ts_edges}")

# ---------- A5 dead ----------
total_zr = (gread('g-deadcount-rust.txt') or [{}])[0].get('total_zero_ref', 0)
post_sym = (gread('g-deadcount2-rust.txt') or [{}])[0].get('post_test_zero_ref', 0)
cand = (gread('g-deadcount3-rust.txt') or [{}])[0].get('candidates', 0)
top40 = [{'symbol': short(r['symbol']), 'file': r['file']} for r in (gread('g-deadtop2-rust.txt') or [])]
machete = {}
cur = None
for line in open(os.path.join(HERE, 'machete.txt'), encoding='utf-8', errors='replace'):
    line = line.rstrip('\n')
    m = re.match(r'^(\S+) -- .*Cargo\.toml:$', line.strip())
    if m:
        cur = m.group(1)
        machete[cur] = []
    elif line.startswith('\t') and cur:
        machete[cur].append(line.strip())
    elif line.strip() == '' or line.startswith('If you believe'):
        cur = None
dead = {'total_zero_ref': total_zr, 'post_symbol_test_filter': post_sym,
        'zero_ref_candidates': cand,
        'filter': "zero-ref via refs view; excluded: instr(symbol,'tests/')>0 OR instr(file,'tests/')>0 (dual filter — symbol-only missed integration-test-target fns, measured 1125 vs 162)",
        'fp_classes': ['entry points (main/build.rs)', 'runtime-invoked surfaces (TauRPC procedures, MCP tools — dispatched via one invoke handler)',
                       'trait-impl methods reached by dispatch', 'derive/attr-invoked fns (serde defaults)', 'deliberate anchors (_force_serde_imports_used)'],
        'top': top40,
        'unused_deps': sorted(f"{c}: {d}" for c, ds in machete.items() for d in ds)}
dump('c-dead.json', dead)
print(f"dead: zero-ref total {total_zr} · post-symbol-filter {post_sym} · CANDIDATES (dual filter) {cand} · machete unused deps {len(dead['unused_deps'])} across {len(machete)} manifests")

# ---------- C1 sizing (mutants --list) ----------
ml = json.load(open(os.path.join(HERE, 'mutants-list.json'), encoding='utf-8'))
per_unit = defaultdict(int)
for m in ml:
    p = norm(m.get('file', ''))
    if p.startswith('crates/'):
        per_unit[p.split('/')[1]] += 1
    elif p.startswith('pulse-app/'):
        per_unit['pulse-app'] += 1
    elif p.startswith('xtask/'):
        per_unit['xtask'] += 1
sizing = sorted(per_unit.items(), key=lambda t: -t[1])
dump('c-mutation-sizing.json', {'total_mutants': len(ml), 'per_unit': sizing,
                                'note': 'from cargo mutants --list --json at HEAD; runtime estimate ~ mutants x 4-8s + ~2min baseline per unit (scratch build)'})
print(f"mutation sizing: {len(ml)} mutants total")
for u, n in sizing:
    est = round((n * 6 + 120) / 60)
    print(f"   {u:<22} {n:>5} mutants   est ~{est} min")

# ---------- cleanup (inside run_dir only; scripted per host rm policy) ----------
for d in ['rca', 'jscpd', 'jscpd2']:
    shutil.rmtree(os.path.join(HERE, d), ignore_errors=True)
for f in ['tokei.json', 'tokei.err', 'lizard.csv', 'lizard.err', 'machete.txt',
          'mutants-list.json', 'mutants-list.err', 'rca.log', 'jscpd.log', 'jscpd2.log']:
    try:
        os.remove(os.path.join(HERE, f))
    except OSError:
        pass
print("cleanup: raw tool outputs removed (c-*.json + g-*.txt + population.txt + traces kept)")
