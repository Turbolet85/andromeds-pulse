#!/usr/bin/env python
"""Evolve-diagnose stage analysis (Epoch 4 epoch-to-date).
Writes q-*.json evidence twins into this run dir; stdout = bounded summaries.
Subcommands: health | typed | cases <key> | untyped | chains | level
"""
import json, sys, os
from collections import Counter, defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
LEDGER = '.andromeda/friction-log.ndjson'
EPOCH_KEY = 'Epoch 4'
E3_KEY = 'Epoch 3'
ID_BOUNDARY = '2026-08-18'
IMPACT_KEYS = ['iterations', 'retries', 'reformulations', 'dialogue_rounds',
               'extra_reads', 'halted', 'soft_exit', 'deferred']
UNIVERSAL = {'tooling.host-shell', 'contract.narrow-basis-claim', 'contract.premise-falsified',
             'contract.structural-blind-spot', 'contract.token-proxy-check'}
EXPECTED = {'phase': 5, 'implement': 3, 'wrap-session': 5}


def dashfold(s):
    return (s or '').replace('–', '—').replace(' - ', ' — ')


def bare(s):
    s = s or ''
    return s[len('andromeda-'):] if s.startswith('andromeda-') else s


def load():
    recs, skips = [], 0
    with open(LEDGER, encoding='utf-8') as f:
        for i, line in enumerate(f):
            line = line.strip()
            if not line:
                continue
            try:
                r = json.loads(line)
                r['_line'] = i
                recs.append(r)
            except ValueError:
                skips += 1
    return recs, skips


def load_excl():
    p = os.path.join(HERE, 'q-retractions.json')
    if not os.path.exists(p):
        return set(), set()
    d = json.load(open(p, encoding='utf-8'))
    return set(d.get('retracted_ids', [])), set((t[0], t[1]) for t in d.get('retracted_problems', []))


def in_epoch(r):
    return dashfold(r.get('epoch')).startswith(EPOCH_KEY)


def in_e3(r):
    return dashfold(r.get('epoch')).startswith(E3_KEY)


def probs(r, excl_probs):
    p = r.get('problem')
    lst = [] if p is None else ([p] if isinstance(p, dict) else list(p))
    rid = r.get('id')
    out = []
    for idx, f in enumerate(lst):
        if isinstance(f, dict) and ((rid, None) in excl_probs or (rid, idx) in excl_probs):
            continue
        if isinstance(f, dict):
            out.append(f)
    return out


def short(s, n=150):
    s = (s or '').replace('\n', ' ').replace('|', '/')
    return s if len(s) <= n else s[:n - 1] + '~'


def imp_str(d):
    d = d or {}
    return ','.join(f"{k}={v}" for k, v in d.items() if v) or '-'


def dump(name, obj):
    with open(os.path.join(HERE, name), 'w', encoding='utf-8') as f:
        json.dump(obj, f, ensure_ascii=False, indent=1)


def group_weight(sums):
    return (1 + sums.get('iterations', 0) + sums.get('retries', 0) + sums.get('reformulations', 0)
            + 2 * sums.get('dialogue_rounds', 0) + 3 * sums.get('halted', 0) + 3 * sums.get('soft_exit', 0))


def case_row(r):
    return {'chunk': r.get('chunk'), 'ts': r.get('ts'), 'id': r.get('id'),
            'what': r.get('what'), 'impact': r.get('impact') or {},
            'artifacts': r.get('artifacts') or [], 'evidence': r.get('evidence')}


def cmd_health(recs, skips, excl_ids, excl_probs):
    e4 = [r for r in recs if in_epoch(r)]
    steps = [r for r in e4 if r.get('kind') == 'step']
    fric = [r for r in e4 if r.get('kind') == 'friction']
    nullep = [r for r in recs if not (r.get('epoch') or '').strip()]
    cov = defaultdict(Counter)
    stepids = defaultdict(Counter)
    for r in steps:
        sk = bare(r.get('skill'))
        stepids[sk][r.get('step')] += 1
        if r.get('chunk'):
            cov[r['chunk']][sk] += 1
    ns_runs = sum(1 for r in steps if bare(r.get('skill')) == 'new-session')
    chunk_null_steps = Counter(bare(r.get('skill')) for r in steps if not r.get('chunk'))
    deviations = {}
    exact = 0
    for c, sk in sorted(cov.items()):
        dev = {k: (sk.get(k, 0), v) for k, v in EXPECTED.items() if sk.get(k, 0) != v}
        extra = {k: n for k, n in sk.items() if k not in EXPECTED and k != 'new-session'}
        if dev or extra:
            deviations[c] = {'dev': dev, 'extra': extra, 'all': dict(sk)}
        else:
            exact += 1
    unt = defaultdict(lambda: [0, 0])
    for r in fric:
        k = f"{bare(r.get('skill'))}/{r.get('step')}"
        unt[k][1] += 1
        if r.get('untyped'):
            unt[k][0] += 1
    pfill = sum(1 for r in steps if probs(r, set()))
    post = [r for r in e4 if (r.get('ts') or '') >= ID_BOUNDARY]
    idfill = sum(1 for r in post if r.get('id'))
    out = {'epoch_records': len(e4), 'step': len(steps), 'friction': len(fric),
           'unparseable': skips, 'null_epoch_records': len(nullep),
           'null_epoch_detail': [{'ts': r.get('ts'), 'kind': r.get('kind'), 'skill': r.get('skill'),
                                  'step': r.get('step'), 'chunk': r.get('chunk')} for r in nullep],
           'session_starts_new_session': ns_runs,
           'chunk_null_step_records': dict(chunk_null_steps),
           'chunks_covered': len(cov), 'chunks_exact_expected': exact,
           'coverage_deviations': deviations,
           'step_ids_per_skill': {k: dict(v) for k, v in stepids.items()},
           'untyped_per_step': {k: {'untyped': v[0], 'friction': v[1]} for k, v in sorted(unt.items())},
           'problem_fact_fill': {'populated': pfill, 'step_records': len(steps)},
           'id_fill_post_boundary': {'with_id': idfill, 'post_boundary_records': len(post)},
           'retracted': {'ids': sorted(excl_ids), 'problems': sorted((list(t) for t in excl_probs), key=lambda t: (t[0], str(t[1])))}}
    dump('q-health.json', out)
    print(f"E4 records {len(e4)} ({len(steps)} step / {len(fric)} friction) | unparseable {skips} | null-epoch {len(nullep)}")
    print(f"session-starts(new-session) {ns_runs} | chunk-null step records {dict(chunk_null_steps)}")
    print(f"chunks {len(cov)} | exact 5/3/5 coverage {exact} | deviations {len(deviations)}")
    for c, d in deviations.items():
        print(f"  DEV {c}: {d['all']}")
    print("step-ids per skill:")
    for k, v in stepids.items():
        print(f"  {k}: {dict(v)}")
    print(f"problem-fact fill {pfill}/{len(steps)} | id fill (post {ID_BOUNDARY}) {idfill}/{len(post)}")
    print(f"retracted ids {len(excl_ids)} problem-facts {len(excl_probs)}")
    print("untyped per step (untyped/friction):")
    for k, v in sorted(unt.items()):
        if v[1]:
            print(f"  {k}: {v[0]}/{v[1]}")


def _typed_groups(recs, excl_ids):
    fric = [r for r in recs if in_epoch(r) and r.get('kind') == 'friction' and r.get('id') not in excl_ids]
    steps = [r for r in recs if in_epoch(r) and r.get('kind') == 'step']
    denom = Counter((bare(r.get('skill')), r.get('step')) for r in steps)
    groups = defaultdict(list)
    for r in fric:
        t = r.get('type') or 'UNTYPED'
        groups[(bare(r.get('skill')), r.get('step'), t)].append(r)
    rows = []
    for (sk, st, t), rs in groups.items():
        sums = {k: sum((x.get('impact') or {}).get(k, 0) for x in rs) for k in IMPACT_KEYS}
        w = group_weight(sums)
        rows.append({'key': f"{sk}/{st}/{t}", 'skill': sk, 'step': st, 'type': t, 'n': len(rs),
                     'sums': sums, 'weight': w, 'sort': len(rs) * w,
                     'rate_denom': denom.get((sk, st), 0),
                     'chunks': sorted(set(x.get('chunk') or '(null)' for x in rs)),
                     'cases': [case_row(x) for x in rs]})
    uni = defaultdict(list)
    for r in fric:
        t = r.get('type')
        if t in UNIVERSAL:
            uni[t].append(r)
    uni_rows = []
    for t, rs in uni.items():
        sums = {k: sum((x.get('impact') or {}).get(k, 0) for x in rs) for k in IMPACT_KEYS}
        w = group_weight(sums)
        uni_rows.append({'type': t, 'n': len(rs), 'sums': sums, 'weight': w, 'sort': len(rs) * w,
                         'steps': sorted(set(f"{bare(x.get('skill'))}/{x.get('step')}" for x in rs)),
                         'chunks': sorted(set(x.get('chunk') or '(null)' for x in rs)),
                         'cases': [case_row(x) for x in rs]})
    rows.sort(key=lambda r: -r['sort'])
    uni_rows.sort(key=lambda r: -r['sort'])
    return rows, uni_rows


def cand(row):
    halts = row['sums'].get('halted', 0) + row['sums'].get('soft_exit', 0)
    return row['n'] >= 3 or (row['n'] >= 2 and halts > 0)


def cmd_typed(recs, excl_ids):
    rows, uni_rows = _typed_groups(recs, excl_ids)
    dump('q-typed.json', {'groups': rows, 'universal_rollup': uni_rows})
    cands = [r for r in rows if r['type'] != 'UNTYPED' and cand(r)]
    below = [r for r in rows if r['type'] != 'UNTYPED' and not cand(r)]
    print(f"groups {len([r for r in rows if r['type']!='UNTYPED'])} typed | candidates {len(cands)} | below {len(below)}")
    print("== CANDIDATES (per-step) ==")
    for r in cands:
        print(f"[{r['n']}x w{r['weight']} rate {r['n']}/{r['rate_denom']}] {r['key']}  chunks={len(r['chunks'])}  sums={imp_str(r['sums'])}")
        for c in r['cases'][:14]:
            print(f"    - {short(c['chunk'] or '(null)',34)} | {short(c['what'],120)} | {imp_str(c['impact'])}")
        if r['n'] > 14:
            print(f"    ... {r['n']-14} more (q-typed.json / cases subcommand)")
    print("== UNIVERSAL ROLLUP (type alone, cross-step) ==")
    for r in uni_rows:
        halts = r['sums'].get('halted', 0) + r['sums'].get('soft_exit', 0)
        mark = 'CAND' if (r['n'] >= 3 or (r['n'] >= 2 and halts > 0)) else 'below'
        print(f"[{r['n']}x w{r['weight']} {mark}] {r['type']}  steps={r['steps']}  chunks={len(r['chunks'])}")
    print("== BELOW THRESHOLD ==")
    for r in below:
        print(f"[{r['n']}x] {r['key']}  {imp_str(r['sums'])}")


def cmd_cases(recs, excl_ids, key):
    rows, uni_rows = _typed_groups(recs, excl_ids)
    for r in rows + [{'key': u['type'], **u} for u in uni_rows]:
        if r.get('key') == key:
            for c in r['cases']:
                print(f"- {c['chunk'] or '(null)'} | {short(c['what'],200)} | {imp_str(c['impact'])} | ev={short(c.get('evidence') or '-',60)}")
            return
    print("key not found")


def cmd_untyped(recs, excl_ids):
    fric = [r for r in recs if in_epoch(r) and r.get('kind') == 'friction'
            and r.get('untyped') and r.get('id') not in excl_ids]
    e3 = [r for r in recs if in_e3(r) and r.get('kind') == 'friction' and r.get('untyped')]
    dump('q-untyped.json', {'epoch4': [case_row(r) | {'skill': bare(r.get('skill')), 'step': r.get('step')} for r in fric],
                            'epoch3_count': len(e3),
                            'epoch3': [case_row(r) | {'skill': bare(r.get('skill')), 'step': r.get('step')} for r in e3]})
    print(f"untyped E4: {len(fric)} (E3 lookback: {len(e3)})")
    for i, r in enumerate(fric):
        print(f"[{i}] {short(r.get('chunk') or '(null)',34)} | {bare(r.get('skill'))}/{r.get('step')} | {imp_str(r.get('impact'))}")
        print(f"     {short(r.get('what'),230)}")


def cmd_chains(recs, excl_ids, excl_probs):
    e4 = [r for r in recs if in_epoch(r)]
    by_chunk = defaultdict(list)
    for r in e4:
        if r.get('chunk'):
            by_chunk[r['chunk']].append(r)
    anchors = []
    for r in e4:
        if r.get('kind') == 'step':
            for c in (r.get('consumed') or []):
                if c.get('quality') in ('thin', 'wrong', 'missing'):
                    anchors.append({'kind': 'consumed-verdict', 'chunk': r.get('chunk'), 'line': r['_line'],
                                    'consumer': f"{bare(r.get('skill'))}/{r.get('step')}",
                                    'artifact': c.get('artifact'), 'quality': c.get('quality'),
                                    'note': c.get('note')})
        elif r.get('kind') == 'friction' and (r.get('type') or '').startswith('input.') and r.get('id') not in excl_ids:
            for a in (r.get('artifacts') or ['(unnamed)']):
                anchors.append({'kind': 'input-friction', 'chunk': r.get('chunk'), 'line': r['_line'],
                                'consumer': f"{bare(r.get('skill'))}/{r.get('step')}",
                                'artifact': a, 'quality': r.get('type'), 'note': r.get('what')})
    joins = []
    for a in anchors:
        if not a['chunk']:
            continue
        prods = [s for s in by_chunk[a['chunk']] if s.get('kind') == 'step' and s['_line'] < a['line']
                 and any(p.get('artifact') == a['artifact'] for p in (s.get('produced') or []))]
        if not prods:
            joins.append({**a, 'producer': None})
            continue
        p = prods[-1]
        pe = next(x for x in (p.get('produced') or []) if x.get('artifact') == a['artifact'])
        pfr = [x for x in by_chunk[a['chunk']] if x.get('kind') == 'friction'
               and bare(x.get('skill')) == bare(p.get('skill')) and x.get('step') == p.get('step')]
        joins.append({**a, 'producer': f"{bare(p.get('skill'))}/{p.get('step')}",
                      'producer_outcome': p.get('outcome'), 'producer_signals': pe.get('signals') or [],
                      'producer_note': pe.get('note'), 'producer_frictions': [short(x.get('what'), 90) for x in pfr]})
    shapes = Counter((j.get('producer'), j['artifact'], j['consumer']) for j in joins if j.get('producer'))
    dump('q-chains.json', {'anchors': len(anchors), 'joins': joins,
                           'shapes': [{'producer': k[0], 'artifact': k[1], 'consumer': k[2], 'chunks': v}
                                      for k, v in shapes.most_common()]})
    print(f"anchors {len(anchors)} (consumed-verdicts + input.* frictions)")
    for j in joins:
        p = j.get('producer')
        print(f"- {short(j['chunk'],34)} | {j['consumer']} <-[{j['artifact']}:{j['quality']}]- {p or 'NO-PRODUCER-FOUND'}"
              + (f" out={j.get('producer_outcome')} sig={j.get('producer_signals')}" if p else ''))
        print(f"    note: {short(j.get('note'),140)}")
        if p and j.get('producer_frictions'):
            print(f"    producer-frictions: {j['producer_frictions'][:3]}")
    print("== shapes across chunks ==")
    for s, v in shapes.most_common():
        print(f"  {v}x  {s[0]} -[{s[1]}]-> {s[2]}")


def cmd_level(recs, excl_ids, excl_probs):
    e4s = [r for r in recs if in_epoch(r) and r.get('kind') == 'step']
    e3s = [r for r in recs if in_e3(r) and r.get('kind') == 'step']
    pile, deferred, overridden, removed = [], [], [], []
    for r in e4s:
        for i, f in enumerate(probs(r, excl_probs)):
            row = {'chunk': r.get('chunk'), 'skill_step': f"{bare(r.get('skill'))}/{r.get('step')}",
                   'ts': r.get('ts'), 'id': r.get('id'), 'idx': i,
                   'nature': f.get('nature'), 'solution': f.get('solution'), 'note': f.get('note')}
            sol = f.get('solution')
            if sol in ('workaround', 'prohibition'):
                pile.append(row)
            elif sol == 'removed-cause':
                removed.append(row)
            elif sol == 'deferred':
                deferred.append(row)
            elif sol == 'overridden':
                overridden.append(row)
    over3 = []
    for r in e3s:
        for i, f in enumerate(probs(r, excl_probs)):
            if f.get('solution') == 'overridden':
                over3.append({'chunk': r.get('chunk'), 'skill_step': f"{bare(r.get('skill'))}/{r.get('step')}",
                              'note': f.get('note')})
    tool = defaultdict(lambda: Counter())
    for r in recs:
        if r.get('kind') == 'friction' and (r.get('type') or '').startswith('tooling.') and r.get('id') not in excl_ids:
            ep = 'E4' if in_epoch(r) else ('E3' if in_e3(r) else 'other')
            tool[r['type']][ep] += 1
    degraded = defaultdict(lambda: Counter())
    for r in recs:
        if r.get('kind') == 'step' and r.get('outcome') == 'ok-degraded':
            ep = 'E4' if in_epoch(r) else ('E3' if in_e3(r) else 'other')
            degraded[f"{bare(r.get('skill'))}/{r.get('step')}"][ep] += 1
    gates = []
    for r in e4s:
        if r.get('step') == 'gates':
            notes = ' ; '.join(short(c.get('note'), 110) for c in (r.get('consumed') or []) if c.get('note'))
            gates.append({'chunk': r.get('chunk'), 'counts': r.get('counts'), 'notes': notes})
    sig = [s for r in e4s for p in (r.get('produced') or []) for s in (p.get('signals') or []) if 'defer' in s]
    dump('q-level.json', {'passA_workaround_prohibition': pile, 'removed_cause': removed,
                          'deferred_facts': deferred, 'overridden_facts': overridden,
                          'overridden_e3_lookback': over3,
                          'tooling_type_by_epoch': {k: dict(v) for k, v in tool.items()},
                          'ok_degraded_by_step_epoch': {k: dict(v) for k, v in degraded.items()},
                          'gates_records': gates, 'defer_signals': sig})
    print(f"Pass-A pile: workaround/prohibition {len(pile)} | removed-cause {len(removed)} | deferred {len(deferred)} | overridden {len(overridden)} (E3 lookback {len(over3)})")
    print("== WORKAROUND / PROHIBITION facts ==")
    for f in pile:
        print(f"- {short(f['chunk'] or '(null)',34)} | {f['skill_step']} | {f['nature']}/{f['solution']}")
        print(f"    {short(f['note'],185)}")
    print("== REMOVED-CAUSE facts ==")
    for f in removed:
        print(f"- {short(f['chunk'] or '(null)',34)} | {f['skill_step']} | {f['nature']} | {short(f['note'],150)}")
    print("== DEFERRED facts ==")
    for f in deferred:
        print(f"- {short(f['chunk'] or '(null)',34)} | {f['skill_step']} | {short(f['note'],160)}")
    print("== OVERRIDDEN facts (E4) ==")
    for f in overridden:
        print(f"- {short(f['chunk'] or '(null)',34)} | {f['skill_step']} | {short(f['note'],170)}")
    print(f"== OVERRIDDEN (E3 lookback): {len(over3)} ==")
    for f in over3:
        print(f"- {short(f['chunk'] or '(null)',30)} | {f['skill_step']} | {short(f['note'],120)}")
    print("== tooling.* by epoch ==")
    for k, v in sorted(tool.items()):
        print(f"  {k}: {dict(v)}")
    print("== ok-degraded outcomes by step/epoch ==")
    for k, v in sorted(degraded.items()):
        print(f"  {k}: {dict(v)}")
    print(f"== defer-signals in E4 produced[]: {sig} ==")
    print("== gates records (deferral closure aid) ==")
    for g in gates:
        print(f"- {short(g['chunk'] or '(null)',34)} | counts={g['counts']}")
        if g['notes']:
            print(f"    {short(g['notes'],200)}")


def main():
    recs, skips = load()
    excl_ids, excl_probs = load_excl()
    cmd = sys.argv[1] if len(sys.argv) > 1 else 'health'
    if cmd == 'health':
        cmd_health(recs, skips, excl_ids, excl_probs)
    elif cmd == 'typed':
        cmd_typed(recs, excl_ids)
    elif cmd == 'cases':
        cmd_cases(recs, excl_ids, sys.argv[2])
    elif cmd == 'untyped':
        cmd_untyped(recs, excl_ids)
    elif cmd == 'chains':
        cmd_chains(recs, excl_ids, excl_probs)
    elif cmd == 'level':
        cmd_level(recs, excl_ids, excl_probs)
    else:
        print('unknown subcommand')


if __name__ == '__main__':
    main()
