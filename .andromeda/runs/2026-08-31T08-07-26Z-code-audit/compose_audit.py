#!/usr/bin/env python
"""Render baseline proposals.md from the c-*.json twins. Row counts asserted programmatically."""
import json, os

HERE = os.path.dirname(os.path.abspath(__file__))
load = lambda n: json.load(open(os.path.join(HERE, n), encoding='utf-8'))
sizes, dup, comp = load('c-sizes.json'), load('c-duplication.json'), load('c-complexity.json')
graph, dead, mut, cov = load('c-graph.json'), load('c-dead.json'), load('c-mutation.json'), load('c-coverage.json')
rec = load('record.json')

# dead top-40 classing (by index, judged from symbol shape)
CLS = {0: 'fp-runtime-invoked (TauRPC)', 1: 'fp-trait-dispatch (Visit)', 2: 'fp-trait-dispatch (Visit)',
       3: 'CANDIDATE', 4: 'fp-trait-dispatch (TauRPC impl)', 5: 'fp-trait-dispatch', 6: 'fp-trait-dispatch',
       7: 'fp-runtime-invoked (TauRPC)', 8: 'fp-serde-default', 9: 'fp-anchor (deliberate)',
       10: 'fp-trait-dispatch (CorpusWriter)', 11: 'fp-trait-dispatch (FromStr)', 12: 'fp-runtime-invoked (TauRPC)',
       13: 'fp-runtime-invoked (TauRPC)', 14: 'fp-runtime-invoked (TauRPC)', 15: 'fp-trait-dispatch',
       16: 'fp-trait-dispatch (TauRPC impl)', 17: 'fp-trait-dispatch', 18: 'CANDIDATE', 19: 'fp-serde-default',
       20: 'fp-trait-dispatch (Visit)', 21: 'CANDIDATE', 22: 'CANDIDATE (unread field)',
       23: 'fp-runtime-invoked (TauRPC)', 24: 'fp-entry-point (build.rs)', 25: 'fp-entry-point (example)',
       26: 'fp-entry-point (example)', 27: 'fp-entry-point (example)', 28: 'fp-entry-point (example)',
       29: 'fp-entry-point (example)', 30: 'CANDIDATE (unconstructed error variant)',
       31: 'CANDIDATE (unread field)', 32: 'CANDIDATE', 33: 'fp-trait-dispatch (Default)',
       34: 'fp-runtime-invoked (tonic service)', 35: 'CANDIDATE (unused const)',
       36: 'fp-trait-dispatch (Default)', 37: 'fp-trait-dispatch (TauRPC impl)',
       38: 'fp-runtime-invoked (TauRPC)', 39: 'fp-runtime-invoked (TauRPC)'}

L = []
A = L.append
A("# Code Audit — andromeda-pulse · Epoch 4 — Polish & ship: verification · 2026-08-31")
A("mode **baseline** · HEAD `83d4060` · baseline none · span — · ledger record #1 appended")
A("")
A("> Operator-ruled epoch boundary: the one markerless route entry (Conductor return) is administrative/external with no Pulse code surface, so HEAD is Epoch 4's final code state. **Trend judgments begin at the next boundary** — everything below is absolute-only. Founder-facing and obligation-free; fixes route through the normal route/intent channels. Evidence twins: the `c-*.json` files beside this document; reproducibility = {sha, tool_versions, commands} in the ledger record.")
A("")
A("## Baseline findings")
A("")
# B1 mutation vs coverage
sc = mut['scores']; ct = mut['counts']; surv = mut['survivors']
assert len(surv) == 153, f"survivor count {len(surv)} != 153"
tot_caught = sum(c['caught'] for c in ct.values()); tot_missed = sum(c['missed'] for c in ct.values())
A(f"### B1 — mutation score vs line coverage — the hollow-test gap, quantified")
A(f"**Numbers:** line coverage **{cov['line']}%** / function {cov['function']}% (2364 tests, all green) against a weighted mutation score of **{round(tot_caught/(tot_caught+tot_missed)*100,1)}%** over the 7 scoped units ({tot_caught} caught / {tot_missed} missed / 582 total mutants incl. unviable). Formula: caught/(caught+missed).")
A("")
A("| unit | mutants | caught | missed | unviable | score | duration |")
A("|---|---|---|---|---|---|---|")
for u in mut['scoped_units']:
    c = ct[u]
    A(f"| {u} | {c['mutants']} | {c['caught']} | {c['missed']} | {c['unviable']} | **{sc[u]}%** | {mut['durations_s'][u]}s |")
A("")
A("**Suspected shapes (survivor families):** curation's `anomaly.rs` arithmetic (operator swaps in `detect_latency_outliers` / `detect_error_correlation` / `detect_cardinality_spikes` survive — tests assert presence, not values); snapshot's `markdown.rs` rendering (whole render fns replaced with `()` survive — structure asserted, content not); interpretation's `schema.rs::validate` bounds (~27 `>`→`>=`/`==` swaps survive — the L4 output validator is barely pinned, and the same fn is a complexity top-offender at cognitive 34); config-watcher lifecycle (`apply`, `record_rejection`, `emit_event`, `ConfigWatchTask::run` survive whole-body deletion at 54.5%).")
A(f"**Direction:** killing tests for the four families above, highest-leverage first (schema.rs `validate`; curation arithmetic with value-level asserts). Complete survivor list ({len(surv)} rows):")
A("")
A("| unit | site | mutation |")
A("|---|---|---|")
for u, site, m in surv:
    A(f"| {u} | `{site}` | {m.replace('|','/')} |")
A("")
# B2 dead
top40 = dead['top']
assert len(top40) == 40, f"dead top {len(top40)} != 40"
cands = [i for i in range(40) if CLS[i].startswith('CANDIDATE')]
A("### B2 — dead-code candidates — 162 zero-ref symbols (upper bound), FP classes dominate")
A(f"**Numbers:** 2949 raw zero-ref symbols → 1125 after symbol-path `tests/` exclusion → **162** after the DUAL symbol+file filter (pinned this baseline: the symbol-only recipe missed integration-test-target fns — 1125 vs 162, the filter-drift class the collector table warns about). Counts are **candidates, never \"dead\"**: the FP classes below are structural.")
A(f"**Top-40 classed** ({len(cands)}/40 candidate-rate — extrapolated, most of the 162 are FP classes):")
A("")
A("| # | symbol | file | class |")
A("|---|---|---|---|")
for i, r in enumerate(top40):
    A(f"| {i} | `{r['symbol'].replace('|','/')}` | {r['file']} | {CLS[i]} |")
A("")
A("**Corroborated candidates** (two independent signals): `ConfigWatchHandle::status_snapshot` (zero-ref AND its mutation survived); `LwwQueue::active_incident_depth_for` (sibling of the graph-proven-dead `drain_all` removed at 2026-08-25 — the rest of the queue's read surface may be following it); `MetricHistoryPoint#snapshot_unix_nano` (unread field of the `diagnostics.history` validated stub); `FormatError::AnchorEncodingFailed` (unconstructed variant); `STREAM_NAME_CONNECTION_STATE` (unused const); `LlamaCliInference::{configured_model_path, binary_kind}`.")
A("**Direction:** a sweep chunk over the corroborated set (delete or wire a consumer), leaving the FP classes untouched; the 162 number is the trend anchor, not a work list.")
A("")
# B3 machete
deps = dead['unused_deps']
assert len(deps) == 12, f"deps {len(deps)} != 12"
A("### B3 — unused dependencies — 12 declarations across 8 manifests (cargo-machete)")
A("")
for d in deps:
    A(f"- `{d}`")
A("")
A("**FP caveat:** machete misses macro-only usage — `ui-bridge: tauri` is plausibly taurpc-macro-referenced; verify before removing. **Suspected real:** `xtask: ui-bridge` (the 2026-08-30 harness-truth chunk replaced the in-xtask `ui_bridge::health` call with `harness_status`); `corpus: security` (resolver-side scrubbing moved to pulse-app per the 2026-05-26 defense-in-depth pattern); `ingest: {bytes, tonic-prost, tower}`.")
A("**Direction:** per-dep remove or `[package.metadata.cargo-machete] ignored` with provenance — the same disposition discipline the deny/advisory gates use.")
A("")
# B4 duplication
topc = dup['top']
assert len(topc) == 10
A(f"### B4 — duplication starting table — {round(dup['pct'],2)}% of lines ({dup['clones']} clones: {dup['split']['src']} src / {dup['split']['test']} test / {dup['split']['mixed']} mixed)")
A(f"**Numbers:** {dup['duplicated_lines']} duplicated lines / {dup['total_lines']} scanned (466 sources; min-tokens 50; token-level 9.19%).")
A("")
A("| lines | class | file A | file B |")
A("|---|---|---|---|")
for a, b, l, c in topc:
    A(f"| {l} | {c} | {a} | {b} |")
A("")
A("**Suspected shape:** the 158-line src clone `MetricsChart.tsx` ↔ `ConstellationCanvas.tsx` is a shared-chart-primitive extraction waiting to happen; the `triage` trio (`baseline/mod.rs` ↔ `pattern/storm.rs` ↔ `pattern/detector.rs`, 70–74L) duplicates window/EWMA plumbing across the detector family.")
A("**Direction:** extract the shared webview chart primitive; consider a shared window-state helper for the triage trio. Test-class clones (293) are the usual fixture repetition — cheaper to leave.")
A("")
# B5 complexity
topo = comp['top']
assert len(topo) == 10
A(f"### B5 — complexity starting table — {comp['over_ceiling']} functions over ceiling (cognitive>15 rust · cyclomatic>15 ts)")
A(f"**Numbers:** {comp['functions_rust']} rust + {comp['functions_ts']} ts functions; cyclomatic p50/p90 {comp['cyclomatic_p50']}/{comp['cyclomatic_p90']}; cognitive p50/p90 {comp['cognitive_p50']}/{comp['cognitive_p90']} (rust only — lizard carries no cognitive).")
A("")
A("| value | metric | function | file |")
A("|---|---|---|---|")
for o in topo:
    A(f"| {o['value']} | {o['metric']} | `{o['fn']}` | {o['file']} |")
A("")
A("**Suspected shape:** the top is boot wiring and harness verbs (`main` 90 — the known God-boot; `verify_capability_matrix` 52, `parse_bindings` 37 in xtask; `inject_demo::main` 49) — high-fan-in glue, not algorithmic rot. The one corroborated hotspot is `interpretation/schema.rs::validate` (cognitive 34 AND the largest mutation-survivor family).")
A("**Direction:** none urgent at baseline; if `main.rs` keeps growing (1547 lines, cog 90) a boot-module split is the natural cut. Watch `validate` via B1's killing tests rather than a refactor.")
A("")
# B6 sizes
tops = sizes['top']
assert len(tops) == 10
A(f"### B6 — size starting table — {sizes['totals']['loc']} LOC / {sizes['totals']['files']} files / 16 units · {sizes['over_800']} files >800 lines")
A(f"**Numbers:** file p50/p90/max {sizes['file_p50']}/{sizes['file_p90']}/{sizes['file_max']}.")
A("")
A("| code lines | file |")
A("|---|---|")
for f, v in tops:
    A(f"| {v} | {f} |")
A("")
A("**Note:** the max (`observability_pins.rs`, 3057) is the deliberate 2026-08-30 dead-test migration target; `appender.rs` 2366 / `observability.rs` 2157 / `contract.rs` 2115 are the src heavyweights to watch for growth.")
A("")
# B7 graph
fi = graph['fan_in_top']
assert len(fi) == 20
A(f"### B7 — dependency graph — **0 cycles** · {graph['cross_unit_edges']} cross-unit edges")
A("**The headline absolute check passes plainly: no crate cycle exists.** Starting tables for the trend:")
A("")
A("fan-out per unit (instability proxy): " + " · ".join(f"{u} {n}" for u, n in graph['fan_out']))
A("")
A("| fan-in (distinct callers) | symbol |")
A("|---|---|")
for s, n in fi:
    A(f"| {n} | `{s.replace('|','/')}` |")
A("")
A(f"_{graph['ts_plane_note']}_")
A("")
# B8 coverage vs gates
A("### B8 — coverage vs the project's own stated gates")
A(f"**Numbers:** line **{cov['line']}%** (stated floor ≥75% — comfortably above) · function **{cov['function']}%** against the stated ≥85% function floor — **0.94pt under**, stated without verdict (the gate's own tooling basis may differ) · branch **unmeasurable on this toolchain**: `--branch` maps to `-Z coverage-options=branch`, nightly-only on stable 1.95.0 — which also means the test-plan's ≥70% branch floor cannot currently be produced by the documented `cargo llvm-cov` command on this host. That last fact is a founder-facing observation about the GATE, not the code.")
A("")
A("## Informational")
A("- `crates/triage-experimental/` exists on disk **untracked** (src/lib.rs + src/main.rs; not a workspace member, not git-tracked) — fs-walk collectors leaked it into complexity until population-filtered; surfaced for disposition (adopt, relocate, or delete — operator's call).")
A("- `interpretation` mutation ran 951s — over the 15-min budget, under the 25-min ceiling; complete, not truncated.")
A("- Tool-substitution note for the trend: this host's `jscpd` is the **Rust `cpd 5.0.16` reimplementation** (flags differ from node jscpd); the recorded command is the firing form.")
A("- Disk: pre-authorized `cargo clean` freed 291.0 GiB in 78s before the instrumented build; ~187 GB free at run end.")
A("- Coverage suite: 2364/2364 passed + 1 skip under instrumentation — matches the known suite exactly.")
A("- Audit side effect, caught by the exit self-check: the coverage run's default-features workspace suite CLOBBERED `pulse-app/ui/src/bindings/index.ts` to the no-mcp shape (the documented taurpc dev-mode export class, worktree 0 vs HEAD 1 `\"mcp\":`); restored to HEAD byte-identical before exit — the tree gained nothing outside the run dir + ledger.")
A("")
A("## Below threshold — no action")
A("Baseline mode: no prior record exists, so no movements — this section starts reporting at the next boundary.")
A("")
A("## Skips")
for s in rec['skips']:
    A(f"- **{s['metric']}** — {s['reason']}" + (f" ({s['note']})" if s.get('note') else ""))
A("")
A("_Generated by compose_audit.py from the c-*.json evidence twins; ledger record #1 in `.andromeda/code-metrics.ndjson` is the trend anchor._")

out = os.path.join(HERE, 'proposals.md')
open(out, 'w', encoding='utf-8').write("\n".join(L))
print(f"written {out} · lines {len(L)} · findings 8 · informational 5 · skips {len(rec['skips'])}")
