#!/usr/bin/env python
"""Assemble the baseline ledger record from the c-*.json twins -> record.json (schema per audit-pass.md)."""
import json, os, datetime

HERE = os.path.dirname(os.path.abspath(__file__))
RUN_DIR_REL = '.andromeda/runs/2026-08-31T08-07-26Z-code-audit'


def load(name, default=None):
    p = os.path.join(HERE, name)
    if not os.path.exists(p):
        return default
    return json.load(open(p, encoding='utf-8'))


sizes = load('c-sizes.json')
dup = load('c-duplication.json')
comp = load('c-complexity.json')
graph = load('c-graph.json')
dead = load('c-dead.json')
mut = load('c-mutation.json', {})
cov = load('coverage.json')

line_pct = func_pct = branch_pct = None
if cov:
    totals = cov['data'][0]['totals']
    line_pct = round(totals['lines']['percent'], 2)
    func_pct = round(totals['functions']['percent'], 2)
    if totals.get('branches') and totals['branches'].get('count'):
        branch_pct = round(totals['branches']['percent'], 2)
    json.dump({'line': line_pct, 'function': func_pct, 'branch': branch_pct,
               'note': 'branch null: -Z coverage-options=branch is nightly-only on stable 1.95.0; function% kept in this twin'},
              open(os.path.join(HERE, 'c-coverage.json'), 'w', encoding='utf-8'), indent=1)

skips = [
    {'metric': 'churn', 'reason': 'no-baseline'},
    {'metric': 'hotspots', 'reason': 'no-baseline'},
    {'metric': 'coverage-branch', 'reason': 'tool-missing',
     'note': '-Z coverage-options=branch nightly-only on stable 1.95.0; line+function recorded'},
    {'metric': 'mutation:buffer', 'reason': 'declined',
     'note': 'operator-declared: 375 mutants ~40 min/unit - overnight-run / next-boundary candidate'},
    {'metric': 'mutation:triage', 'reason': 'declined',
     'note': 'operator-declared: 908 mutants ~93 min - overnight-run / next-boundary candidate'},
    {'metric': 'mutation:pulse-app', 'reason': 'declined',
     'note': 'operator-declared: 903 mutants ~92 min - overnight-run / next-boundary candidate'},
    {'metric': 'mutation:xtask,ingest,mcp-server,ui-bridge,corpus,plugins', 'reason': 'declined',
     'note': 'outside the operator-chosen 7-unit subset (56-725 mutants, 8-74 min each)'},
]
for u in mut.get('budget_exhausted', []):
    skips.append({'metric': f'mutation:{u}', 'reason': 'budget-exhausted',
                  'note': '25-min per-unit ceiling hit; partial counts recorded'})

record = {
    'ts': datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ'),
    'epoch': 'Epoch 4 — Polish & ship: verification',
    'mode': 'baseline',
    'sha': '83d40601fdb3758ec82650a9d06a22f4b7c94387',
    'baseline_sha': None, 'span': None, 'ancestry_broken': False,
    'boundary_note': 'operator-ruled epoch boundary: the one markerless entry (Conductor return) is administrative/external with no code surface; HEAD is Epoch 4 final code state',
    'tool_versions': {'jscpd': 'cpd 5.0.16 (rust jscpd reimplementation)', 'tokei': '14.0.0',
                      'rust-code-analysis': '0.0.25', 'lizard': '1.24.0', 'cargo-mutants': '27.1.0',
                      'cargo-machete': '0.9.2', 'cargo-llvm-cov': '0.8.5', 'duckdb-python': '1.5.3',
                      'rustc': '1.95.0 stable'},
    'totals': {'loc': sizes['totals']['loc'], 'files': sizes['totals']['files'], 'units': sizes['units']},
    'duplication': {'pct': round(dup['pct'], 2), 'clones': dup['clones'],
                    'duplicated_lines': dup['duplicated_lines'], 'total_lines': dup['total_lines'],
                    'split': dup['split'], 'top': [[a, b, l] for a, b, l, c in dup['top']]},
    'complexity': {'cyclomatic_p50': comp['cyclomatic_p50'], 'cyclomatic_p90': comp['cyclomatic_p90'],
                   'cognitive_p50': comp['cognitive_p50'], 'cognitive_p90': comp['cognitive_p90'],
                   'over_ceiling': comp['over_ceiling'], 'max': comp['max'], 'top': comp['top'],
                   'functions': {'rust': comp['functions_rust'], 'ts': comp['functions_ts']}},
    'sizes': {'file_p50': sizes['file_p50'], 'file_p90': sizes['file_p90'], 'file_max': sizes['file_max'],
              'over_800': sizes['over_800'], 'top': sizes['top']},
    'graph': {'cycles': graph['cycles'], 'cycle_paths': graph['cycle_paths'],
              'fan_in_top': graph['fan_in_top'], 'fan_out': graph['fan_out'],
              'cross_unit_edges': graph['cross_unit_edges'], 'ts_plane_note': graph['ts_plane_note']},
    'dead': {'unused_deps': dead['unused_deps'], 'zero_ref_candidates': dead['zero_ref_candidates'],
             'filter': dead['filter'], 'top': dead['top'][:20]},
    'coverage': {'line': line_pct, 'branch': branch_pct, 'function': func_pct},
    'churn': {'pct': None, 'files_churned': 0},
    'hotspots': [],
    'mutation': {'scoped_units': mut.get('scoped_units', []), 'scores': mut.get('scores', {}),
                 'counts': mut.get('counts', {}), 'durations_s': mut.get('durations_s', {}),
                 'score_formula': 'caught/(caught+missed)', 'survivors': mut.get('survivors', [])},
    'commands': {
        'population': "git ls-files | grep -E '\\.(rs|ts|tsx)$' | grep -E '^(crates|pulse-app|xtask)/' | grep -v '^pulse-app/ui/src/bindings/'",
        'sizes': 'tokei crates pulse-app xtask --files --output json; per-file rows filtered to the population (ONE population for totals+percentiles+top)',
        'duplication': 'jscpd -f "rust,typescript,tsx" -r json,silent -o {run_dir}/jscpd2 -i "**/src/bindings/**,**/dist/**,**/node_modules/**,**/target/**,**/.andromeda/**,**/.claude/**,**/scripts/**,**/docs/**,**/refs/**" .  (single-root scan; min-tokens default 50)',
        'complexity': 'rust-code-analysis-cli --metrics -O json -o {run_dir}/rca -p crates -p pulse-app/src -p pulse-app/tests -p xtask/src  +  python -m lizard pulse-app/ui/src pulse-app/ui/tests-a11y pulse-app/ui/tests-e2e -x "*/bindings/*" --csv; per-function rows filtered to the population (fs-walk leaked untracked crates/triage-experimental + *.mjs)',
        'graph': 'python scripts/code-graph.py query {run_dir} code-audit "<canonical cycles/fan-in/fan-out/edges SQL per audit-pass.md>" rust',
        'dead': "cargo machete  +  zero-ref: SELECT ... FROM symbol s LEFT JOIN refs r ON r.callee = s.symbol WHERE r.callee IS NULL AND instr(s.symbol,'tests/') = 0 AND instr(s.file,'tests/') = 0  (DUAL filter pinned this baseline: symbol-only missed integration-test-target fns, 1125 vs 162)",
        'coverage': 'CARGO_BUILD_JOBS=4 cargo llvm-cov nextest --workspace --summary-only --json --output-path {run_dir}/coverage.json  (--branch dropped: nightly-gated on stable 1.95.0)',
        'mutation': 'timeout 1500 cargo mutants -p {unit} --test-tool=nextest -o {run_dir}/mutants-{unit}  (sequential; jobs=1; 25-min/unit ceiling = 15-min budget + cold-build allowance post cargo-clean)',
    },
    'skips': skips,
}

with open(os.path.join(HERE, 'record.json'), 'w', encoding='utf-8') as f:
    json.dump(record, f, ensure_ascii=False, indent=1)
print('record.json written')
print('coverage:', record['coverage'])
print('mutation scores:', record['mutation']['scores'])
print('skips:', len(skips))
