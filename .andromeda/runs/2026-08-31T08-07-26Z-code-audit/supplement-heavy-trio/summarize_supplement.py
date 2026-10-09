#!/usr/bin/env python
"""Summarize the heavy-trio mutation supplement -> c-mutation-heavy-trio.json + supplement.md.
Same pinned shape as audit record #1's B1: {caught/missed/unviable/score/duration} + FULL survivor list.
Handles budget-exhausted units by parsing the incremental txt lists as PARTIAL."""
import json, os, re, shutil, datetime

HERE = os.path.dirname(os.path.abspath(__file__))
AUDIT = os.path.dirname(HERE)
UNITS = ['buffer', 'triage', 'pulse-app']
CEIL_S = 6000

sizing = {u: n for u, n in json.load(open(os.path.join(AUDIT, 'c-mutation-sizing.json'), encoding='utf-8'))['per_unit']}
rec1 = json.load(open(os.path.join(AUDIT, 'record.json'), encoding='utf-8'))

progress, starts = {}, {}
ppath = os.path.join(HERE, 'progress.txt')
if os.path.exists(ppath):
    for line in open(ppath, encoding='utf-8', errors='replace'):
        m = re.match(r'^(\S+) rc=(\d+) dur=(\d+)s', line.strip())
        if m:
            progress[m.group(1)] = {'rc': int(m.group(2)), 'dur_s': int(m.group(3))}
        m2 = re.match(r'^(\S+) START (\S+) D_free=(\S+)GB', line.strip())
        if m2:
            starts[m2.group(1)] = {'start': m2.group(2), 'd_free_gb': m2.group(3)}


def count_lines(p):
    return sum(1 for l in open(p, encoding='utf-8', errors='replace') if l.strip()) if os.path.exists(p) else 0


def survivors_from(mdir, unit, partial):
    out = []
    mt = os.path.join(mdir, 'missed.txt')
    if not os.path.exists(mt):
        return out
    for line in open(mt, encoding='utf-8', errors='replace'):
        line = line.strip()
        if not line:
            continue
        m = re.match(r'^(\S+?\.rs:\d+)(?::\d+)?:\s*(.*)$', line)
        site, mut = (m.group(1), m.group(2)) if m else ('partial' if partial else '', line)
        out.append([unit, site, mut])
    return out


res = {'supplement_of': 'audit record #1 (.andromeda/code-metrics.ndjson, sha 83d4060)',
       'sha': rec1['sha'], 'epoch': rec1['epoch'],
       'ledger': 'UNTOUCHED by design — record #1 already declares these three as skips[declined]; '
                 'the next boundary cites this supplement as first-absolute for these units',
       'recipe': 'timeout 6000 cargo mutants -p {unit} --test-tool=nextest -o {supplement_dir}/mutants-{unit} '
                 '(pinned collectors.md C1: nextest pass-through, jobs=1, scratch copy; 100-min operator ceiling)',
       'score_formula': 'caught/(caught+missed)',
       'units': {}, 'survivors': [], 'budget_exhausted': [], 'notes': []}

for u in UNITS:
    mdir = os.path.join(HERE, f'mutants-{u}', 'mutants.out')
    pr = progress.get(u, {})
    dur = pr.get('dur_s')
    oj = os.path.join(mdir, 'outcomes.json')
    complete = os.path.exists(oj) and pr.get('rc') != 124
    if os.path.exists(oj):
        d = json.load(open(oj, encoding='utf-8'))
        caught, missed = d.get('caught'), d.get('missed')
        timeout_n, unviable = d.get('timeout', 0), d.get('unviable', 0)
        total = d.get('total_mutants')
        if caught is None:
            tally = {'CaughtMutant': 0, 'MissedMutant': 0, 'Timeout': 0, 'Unviable': 0}
            for o in d.get('outcomes', []):
                s = o.get('summary')
                if s in tally:
                    tally[s] += 1
            caught, missed = tally['CaughtMutant'], tally['MissedMutant']
            timeout_n, unviable = tally['Timeout'], tally['Unviable']
            total = caught + missed + timeout_n + unviable
    elif pr:
        caught, missed = count_lines(os.path.join(mdir, 'caught.txt')), count_lines(os.path.join(mdir, 'missed.txt'))
        timeout_n, unviable = count_lines(os.path.join(mdir, 'timeout.txt')), count_lines(os.path.join(mdir, 'unviable.txt'))
        total = None
    else:
        res['notes'].append(f'{u}: never ran (no progress entry)')
        continue

    tested = caught + missed + timeout_n + unviable
    score = round(caught / (caught + missed) * 100, 1) if (caught + missed) else None

    # CANNOT-EVALUATE: the unmutated baseline never built, so zero mutants ran.
    # Guarded explicitly — a 0/0 row must never render as a score (the inverse of the
    # collectors.md "Found 0 mutants is a NO-OP, never a pass" hazard).
    if tested == 0:
        res['cannot_evaluate'] = res.get('cannot_evaluate', []) + [u]
        res['units'][u] = {
            'status': 'cannot-evaluate', 'score': None, 'rc': pr.get('rc'), 'duration_s': dur,
            'mutants_declared_at_head': sizing.get(u), 'mutants_tested': 0,
            'caught': 0, 'missed': 0, 'timeout': 0, 'unviable': 0,
            'started_utc': starts.get(u, {}).get('start'),
            'mechanism': "cargo-mutants' baseline build is PACKAGE-SCOPED "
                         "(`cargo nextest run --no-run --package=pulse-app@0.1.0`) and failed in a PRISTINE scratch "
                         "tree with `crate {libduckdb_sys, wasmtime, cranelift_codegen} required to be available in "
                         "rlib format, but was not found in this form` — across the bin target and ~25 integration "
                         "test targets. No mutant was ever built or run.",
            'not_a_score': 'total_mutants=0 with a single Baseline/Failure outcome — absence of data, not 0%'}
        res['notes'].append(f'{u}: CANNOT-EVALUATE under the pinned recipe — baseline build failed in {dur}s; '
                            f'0 of {sizing.get(u)} mutants tested')
        continue
    entry = {'complete': complete, 'rc': pr.get('rc'), 'duration_s': dur,
             'mutants_tested': tested, 'mutants_declared_at_head': sizing.get(u),
             'caught': caught, 'missed': missed, 'timeout': timeout_n, 'unviable': unviable,
             'score': score, 'started_utc': starts.get(u, {}).get('start'),
             'd_free_gb_at_start': starts.get(u, {}).get('d_free_gb')}
    if total is not None:
        entry['mutants_total_reported'] = total
    if pr.get('rc') == 124 or (not complete and pr):
        res['budget_exhausted'].append(u)
        entry['partial'] = True
        pct = round(tested / sizing[u] * 100, 1) if sizing.get(u) else None
        entry['coverage_of_declared'] = f'{tested}/{sizing.get(u)} mutants ({pct}%)'
        res['notes'].append(f'{u}: budget-exhausted at the {CEIL_S//60}-min ceiling — PARTIAL: {tested}/{sizing.get(u)} '
                            f'mutants tested; score {score}% is over the tested subset only, NOT the unit')
    elif complete and pr.get('rc') not in (0, 2, 3):
        res['notes'].append(f'{u}: rc={pr.get("rc")} with outcomes.json present — see mutants-{u}.log')
    if complete and timeout_n:
        res['notes'].append(f'{u}: completed with rc={pr.get("rc")} (cargo-mutants signals timeouts, not failure) — '
                            f'{timeout_n} NON-TERMINATING mutants, excluded from the score by the formula; listed separately')
    if entry.get('partial'):
        mjp, ocp = os.path.join(mdir, 'mutants.json'), os.path.join(mdir, 'outcomes.json')
        if os.path.exists(mjp) and os.path.exists(ocp):
            declared, reached = {}, {}
            for m in json.load(open(mjp, encoding='utf-8')):
                declared[m['file']] = declared.get(m['file'], 0) + 1
            for o in json.load(open(ocp, encoding='utf-8')).get('outcomes', []):
                sc = o.get('scenario')
                if isinstance(sc, dict) and 'Mutant' in sc:
                    f = sc['Mutant']['file']
                    reached[f] = reached.get(f, 0) + 1
            entry['files_declared'] = len(declared)
            entry['files_unreached'] = sorted((f, n) for f, n in declared.items() if f not in reached)
            entry['files_partial'] = sorted((f, reached[f], n) for f, n in declared.items()
                                            if f in reached and reached[f] < n)
    tl = os.path.join(mdir, 'timeout.txt')
    if os.path.exists(tl):
        entry['timeout_sites'] = [l.strip() for l in open(tl, encoding='utf-8', errors='replace') if l.strip()]
    res['units'][u] = entry
    res['survivors'] += survivors_from(mdir, u, entry.get('partial', False))

json.dump(res, open(os.path.join(HERE, 'c-mutation-heavy-trio.json'), 'w', encoding='utf-8'),
          ensure_ascii=False, indent=1)

# ---------- render supplement.md ----------
L = []
A = L.append
A("# Mutation supplement — heavy trio · andromeda-pulse · Epoch 4 — Polish & ship: verification")
A(f"supplement of **audit record #1** · HEAD `{res['sha'][:7]}` · same boundary, source tree verified clean · "
  f"{datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%d')}")
A("")
A(f"> Completes the three units record #1 declares as `skips[declined]` (buffer / triage / pulse-app — 375 / 908 / 903 "
  f"mutants, 40–95 min each). **The ledger is untouched by design**: record #1 already carries the declaration, and the "
  f"next boundary cites this file as the first-absolute reading for these units. Recipe pinned to collectors.md C1 — "
  f"`--test-tool=nextest` pass-through, jobs=1, scratch copy, {CEIL_S//60}-min operator ceiling per unit. Obligation-free: "
  f"nothing here is applied, and no source was edited.")
A("")
A("## Scores")
A("")
A("| unit | mutants tested | caught | missed | unviable | timeout | score | duration | status |")
A("|---|---|---|---|---|---|---|---|---|")
for u in UNITS:
    e = res['units'].get(u)
    if not e:
        A(f"| {u} | — | — | — | — | — | — | — | never ran |")
        continue
    if e.get('status') == 'cannot-evaluate':
        A(f"| {u} | **0** of {e['mutants_declared_at_head']} | — | — | — | — | **no score** | "
          f"{e['duration_s']}s | **cannot-evaluate** (baseline build failed) |")
        continue
    st = 'complete' if e['complete'] else f"**budget-exhausted** ({e.get('coverage_of_declared','partial')})"
    sc = f"**{e['score']}%**" if e['score'] is not None else '—'
    dur = f"{e['duration_s']}s ({round(e['duration_s']/60)}m)" if e['duration_s'] else '—'
    A(f"| {u} | {e['mutants_tested']} of {e['mutants_declared_at_head']} | {e['caught']} | {e['missed']} | "
      f"{e['unviable']} | {e['timeout']} | {sc} | {dur} | {st} |")
A("")
done = [u for u in UNITS if u in res['units'] and res['units'][u].get('status') != 'cannot-evaluate']
tc = sum(res['units'][u]['caught'] for u in done)
tm = sum(res['units'][u]['missed'] for u in done)
if tc + tm:
    A(f"**Weighted across the trio:** {tc} caught / {tm} missed = **{round(tc/(tc+tm)*100,1)}%** "
      f"(formula `caught/(caught+missed)`; a partial unit's contribution is its tested subset only).")
    A("")
    A(f"**Against record #1's 7 light units** (64.2% weighted, line coverage 84.35%): the trio adds the three largest "
      f"units in the workspace — buffer (the DuckDB ring buffer), triage (the L1 distillation pipeline) and pulse-app "
      f"(the binary + its routers). Read the two together as the epoch's mutation picture; neither half is the whole.")
A("")
for u in UNITS:
    e = res['units'].get(u, {})
    if e.get('status') == 'cannot-evaluate':
        A(f"## {u} — cannot-evaluate (no score exists for this unit)")
        A("")
        A(f"**What happened:** the unmutated baseline failed to build after {e['duration_s']}s, so **0 of "
          f"{e['mutants_declared_at_head']} declared mutants** were built or run. `outcomes.json` carries "
          f"`total_mutants: 0` and a single `Baseline / Failure` outcome.")
        A("")
        A(f"**Mechanism (measured, not inferred):** {e['mechanism']}")
        A("")
        A("**Why this is not a stale-target artifact:** cargo-mutants builds in a pristine scratch copy with its own "
          "fresh target dir (`%TEMP%\\cargo-mutants-andromeda-pulse-*.tmp`), so there is no inherited "
          "`target/.fingerprint` state — and the same source at this same sha built and ran 2364/2364 tests green "
          "under workspace-scoped `cargo llvm-cov nextest --workspace` earlier in this session. The failure tracks "
          "the **package-scoped** build shape, not the tree's condition. `.claude/rules/testing.md` documents this "
          "error string but attributes it to \"environmental (target/.fingerprint inconsistency between feature "
          "combinations)\" with a clean-and-retry / `--bin`-scoping remedy — that characterization does not explain "
          "this occurrence, and neither remedy is reachable from inside cargo-mutants' baseline invocation. Recorded "
          "as an observation about the documented root cause; no artifact was edited.")
        A("")
        A("**Options for the next boundary** (costed, operator's call — none taken here):")
        A("- `--test-workspace=true`: replaces the package-scoped test command with the workspace-scoped one that is "
          "known to work. Cost: every mutant re-runs the full 2364-test suite (~19s) on top of its rebuild — "
          "903 mutants ≈ 7–15 h, far past any per-unit ceiling. A 100-min slice would cover only the first ~5% of "
          "mutants in cargo-mutants' deterministic file order, which is a biased sample, not a unit score.")
        A("- Scope pulse-app by file (`--file`) into several bounded runs across boundaries, accepting per-file "
          "rather than per-unit scores.")
        A("- Leave pulse-app unmeasured and say so: its 903 mutants stay a declared gap in the record, which is the "
          "status quo record #1 already carries.")
        A("")
A(f"## Survivors — complete list ({len(res['survivors'])} rows)")
A("")
if res['budget_exhausted']:
    A(f"_Partial for: {', '.join(res['budget_exhausted'])} — the survivor list covers the tested subset, so ABSENCE "
      f"of a site here is not evidence it is killed._")
    A("")
    for u in res['budget_exhausted']:
        e = res['units'][u]
        if e.get('files_unreached') is not None:
            A(f"**{u} partial boundary** — cargo-mutants proceeds in deterministic file order, so the untested "
              f"remainder is a contiguous alphabetical tail, NOT a random sample. "
              f"{e['files_declared'] - len(e['files_unreached'])} of {e['files_declared']} files reached; "
              f"**{len(e['files_unreached'])} files never reached** ({sum(n for _, n in e['files_unreached'])} "
              f"mutants declared, 0 tested):")
            A("")
            A("| file never reached | mutants declared |")
            A("|---|---|")
            for f, n in e['files_unreached']:
                A(f"| `{f}` | {n} |")
            if e.get('files_partial'):
                A("")
                A("Partially reached: " + " · ".join(f"`{f}` {r}/{n}" for f, r, n in e['files_partial']))
            A("")
A("| unit | site | mutation |")
A("|---|---|---|")
for u, site, m in res['survivors']:
    A(f"| {u} | `{site}` | {m.replace('|','/')} |")
A("")
tsites = [(u, s) for u in UNITS for s in res['units'].get(u, {}).get('timeout_sites', [])]
if tsites:
    A(f"## Non-terminating mutants ({len(tsites)}) — timeouts, neither caught nor survived")
    A("")
    A("_Given a slot deliberately: the collector table records that a timeout with no home stays invisible across "
      "boundaries. These are excluded from `caught/(caught+missed)` — a hang is evidence the mutation breaks "
      "termination, not evidence a test asserts the behaviour._")
    A("")
    A("| unit | site |")
    A("|---|---|")
    for u, s in tsites:
        A(f"| {u} | `{s.replace('|','/')}` |")
    A("")
if res['notes']:
    A("## Notes")
    for n in res['notes']:
        A(f"- {n}")
    A("")
A("## Provenance")
A(f"- Recipe: `{res['recipe']}`")
A(f"- Ledger: {res['ledger']}")
A("- Evidence twin: `c-mutation-heavy-trio.json` beside this file; per-unit logs `mutants-{unit}.log`; timing/disk trail `progress.txt`.")
open(os.path.join(HERE, 'supplement.md'), 'w', encoding='utf-8').write("\n".join(L))

print('units:', {u: (res['units'][u]['score'], res['units'][u]['mutants_tested']) for u in done})
print('budget_exhausted:', res['budget_exhausted'])
print('survivors:', len(res['survivors']))
for n in res['notes']:
    print('  note:', n)
print('written supplement.md + c-mutation-heavy-trio.json')

for u in UNITS:
    d = os.path.join(HERE, f'mutants-{u}')
    if os.path.isdir(d):
        shutil.rmtree(d, ignore_errors=True)
print('cleanup: mutants-* dirs removed (summary + full survivor list retained)')
