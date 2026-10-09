#!/usr/bin/env python
"""Recompute c-complexity.json filtered to the ONE population (git-tracked manifest).
Fixes the fs-walk leak: crates/triage-experimental (untracked) + tests-e2e/*.mjs."""
import json, os, csv, math, shutil

HERE = os.path.dirname(os.path.abspath(__file__))


def norm(p):
    p = (p or '').replace('\\', '/')
    return p[2:] if p.startswith('./') else p


def nearest_rank(vals, q):
    if not vals:
        return 0
    s = sorted(vals)
    return s[max(1, math.ceil(q * len(s))) - 1]


population = set(norm(l.strip()) for l in open(os.path.join(HERE, 'population.txt'), encoding='utf-8') if l.strip())
rust_fns, excluded = [], set()


def walk(space, file):
    for sp in space.get('spaces', []):
        if sp.get('kind') == 'function':
            m = sp.get('metrics', {})
            rust_fns.append({'fn': sp.get('name') or '<closure>', 'file': file,
                             'cyclo': (m.get('cyclomatic') or {}).get('sum') or 0,
                             'cog': (m.get('cognitive') or {}).get('sum') or 0})
        walk(sp, file)


for root, _, files in os.walk(os.path.join(HERE, 'rca')):
    for fn in files:
        if fn.endswith('.json'):
            try:
                j = json.load(open(os.path.join(root, fn), encoding='utf-8'))
            except ValueError:
                continue
            f = norm(j.get('name', fn))
            if f in population:
                walk(j, f)
            else:
                excluded.add(f)

ts_fns = []
ts_excluded = set()
with open(os.path.join(HERE, 'lizard.csv'), encoding='utf-8', errors='replace') as fh:
    for row in csv.reader(fh):
        if len(row) >= 8:
            try:
                ccn = int(row[1])
            except ValueError:
                continue
            f = norm(row[6])
            if f in population:
                ts_fns.append({'fn': row[7], 'file': f, 'ccn': ccn})
            else:
                ts_excluded.add(f)

cyc_all = [f['cyclo'] for f in rust_fns] + [f['ccn'] for f in ts_fns]
cog_rust = [f['cog'] for f in rust_fns]
over = sum(1 for f in rust_fns if f['cog'] > 15) + sum(1 for f in ts_fns if f['ccn'] > 15)
off = ([{'fn': f['fn'], 'file': f['file'], 'value': f['cog'], 'metric': 'cognitive'} for f in rust_fns]
       + [{'fn': f['fn'], 'file': f['file'], 'value': f['ccn'], 'metric': 'cyclomatic'} for f in ts_fns])
off.sort(key=lambda o: -o['value'])
comp = {'functions_rust': len(rust_fns), 'functions_ts': len(ts_fns),
        'cyclomatic_p50': nearest_rank(cyc_all, .5), 'cyclomatic_p90': nearest_rank(cyc_all, .9),
        'cyclomatic_max': max(cyc_all) if cyc_all else 0,
        'cognitive_p50': nearest_rank(cog_rust, .5), 'cognitive_p90': nearest_rank(cog_rust, .9),
        'cognitive_max': max(cog_rust) if cog_rust else 0,
        'cognitive_population': 'rust functions only (lizard has no cognitive — noted)',
        'population': 'FILTERED to population.txt (git-tracked) — fs-walk had leaked untracked crates/triage-experimental + tests-e2e/*.mjs',
        'excluded_offpopulation_files': sorted(x for x in (excluded | ts_excluded) if 'triage-experimental' in x or x.endswith('.mjs'))[:10],
        'over_ceiling': over, 'ceiling': 'cognitive>15 (rust) + cyclomatic>15 (ts)',
        'max': {'fn': off[0]['fn'], 'file': off[0]['file'], 'val': off[0]['value']} if off else None,
        'top': off[:10]}
json.dump(comp, open(os.path.join(HERE, 'c-complexity.json'), 'w', encoding='utf-8'), ensure_ascii=False, indent=1)
print(f"complexity(filtered): fns rust {len(rust_fns)} / ts {len(ts_fns)} · cyclo p50/p90/max {comp['cyclomatic_p50']}/{comp['cyclomatic_p90']}/{comp['cyclomatic_max']} · cog p50/p90/max {comp['cognitive_p50']}/{comp['cognitive_p90']}/{comp['cognitive_max']} · over ceiling {over}")
print("off-population excluded:", comp['excluded_offpopulation_files'])
for o in off[:10]:
    print(f"   top: {o['value']:>4} {o['metric'][:3]} {o['fn']}  ({o['file']})")
shutil.rmtree(os.path.join(HERE, 'rca'), ignore_errors=True)
for f in ['lizard.csv', 'lizard.err', 'rca.log']:
    try:
        os.remove(os.path.join(HERE, f))
    except OSError:
        pass
print("cleanup done")
