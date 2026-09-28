#!/usr/bin/env python
"""Summarize cargo-mutants outputs -> c-mutation.json (pinned: counts + score formula + FULL survivors)."""
import json, os, re, shutil

HERE = os.path.dirname(os.path.abspath(__file__))
UNITS = ['curation', 'security', 'snapshot', 'workspace-detector', 'interpretation', 'viz', 'config-watcher']

progress = {}
ppath = os.path.join(HERE, 'mutants-progress.txt')
if os.path.exists(ppath):
    for line in open(ppath, encoding='utf-8', errors='replace'):
        m = re.match(r'^(\S+) rc=(\d+) dur=(\d+)s', line.strip())
        if m:
            progress[m.group(1)] = {'rc': int(m.group(2)), 'dur_s': int(m.group(3))}

result = {'scoped_units': [], 'scores': {}, 'counts': {}, 'durations_s': {},
          'score_formula': 'caught/(caught+missed)', 'survivors': [], 'budget_exhausted': [], 'notes': []}

for u in UNITS:
    mdir = os.path.join(HERE, f'mutants-{u}', 'mutants.out')
    oj = os.path.join(mdir, 'outcomes.json')
    pr = progress.get(u, {})
    if pr:
        result['durations_s'][u] = pr.get('dur_s')
    if os.path.exists(oj):
        d = json.load(open(oj, encoding='utf-8'))
        caught = d.get('caught')
        missed = d.get('missed')
        timeout = d.get('timeout', 0)
        unviable = d.get('unviable', 0)
        total = d.get('total_mutants')
        if caught is None:  # fallback tally from outcomes[]
            tally = {'CaughtMutant': 0, 'MissedMutant': 0, 'Timeout': 0, 'Unviable': 0}
            for o in d.get('outcomes', []):
                s = o.get('summary')
                if s in tally:
                    tally[s] += 1
            caught, missed, timeout, unviable = tally['CaughtMutant'], tally['MissedMutant'], tally['Timeout'], tally['Unviable']
            total = caught + missed + timeout + unviable
        score = round(caught / (caught + missed) * 100, 1) if (caught + missed) else None
        result['scoped_units'].append(u)
        result['scores'][u] = score
        result['counts'][u] = {'mutants': total, 'caught': caught, 'missed': missed,
                               'timeout': timeout, 'unviable': unviable}
        mt = os.path.join(mdir, 'missed.txt')
        if os.path.exists(mt):
            for line in open(mt, encoding='utf-8', errors='replace'):
                line = line.strip()
                if line:
                    m = re.match(r'^(\S+?\.rs:\d+)(?::\d+)?:\s*(.*)$', line)
                    if m:
                        result['survivors'].append([u, m.group(1), m.group(2)])
                    else:
                        result['survivors'].append([u, '', line])
        if pr.get('rc') == 124:
            result['budget_exhausted'].append(u)
            result['notes'].append(f'{u}: outcomes.json present but rc=124 - treat counts as partial')
    else:
        # partial or never-ran
        if pr.get('rc') == 124:
            partial = {}
            for kind in ('caught', 'missed', 'timeout', 'unviable'):
                f = os.path.join(mdir, f'{kind}.txt')
                partial[kind] = sum(1 for l in open(f, encoding='utf-8', errors='replace') if l.strip()) if os.path.exists(f) else 0
            result['budget_exhausted'].append(u)
            result['counts'][u] = {'mutants': None, **partial}
            c, m_ = partial['caught'], partial['missed']
            result['scores'][u] = round(c / (c + m_) * 100, 1) if (c + m_) else None
            result['notes'].append(f'{u}: budget-exhausted at 25min ceiling - PARTIAL counts from txt lists')
            if os.path.exists(os.path.join(mdir, 'missed.txt')):
                for line in open(os.path.join(mdir, 'missed.txt'), encoding='utf-8', errors='replace'):
                    line = line.strip()
                    if line:
                        result['survivors'].append([u, 'partial', line])
        else:
            result['notes'].append(f'{u}: no artifacts (rc={pr.get("rc")}) - see mutants-{u}.log')

with open(os.path.join(HERE, 'c-mutation.json'), 'w', encoding='utf-8') as f:
    json.dump(result, f, ensure_ascii=False, indent=1)
print('units run:', result['scoped_units'])
print('scores:', result['scores'])
print('counts:', json.dumps(result['counts']))
print('durations_s:', result['durations_s'])
print('budget_exhausted:', result['budget_exhausted'])
print('survivors:', len(result['survivors']))
for s in result['survivors']:
    print('  MISSED', s[0], '|', s[1], '|', s[2][:130])
for n in result['notes']:
    print('  note:', n)

# cleanup: drop the heavy mutants dirs after summarizing (logs kept until record assembly)
for u in UNITS:
    d = os.path.join(HERE, f'mutants-{u}')
    if os.path.isdir(d):
        shutil.rmtree(d, ignore_errors=True)
print('cleanup: mutants-* dirs removed (summary + survivors kept in c-mutation.json)')
