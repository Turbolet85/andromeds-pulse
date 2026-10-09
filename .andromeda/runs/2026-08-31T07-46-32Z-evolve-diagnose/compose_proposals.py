#!/usr/bin/env python
"""Render proposals.md from the q-*.json evidence twins + authored narratives.
Tables are generated from q-typed.json / q-untyped.json so every case is cited (F-3)."""
import json, os

HERE = os.path.dirname(os.path.abspath(__file__))

typed = json.load(open(os.path.join(HERE, 'q-typed.json'), encoding='utf-8'))
untyped = json.load(open(os.path.join(HERE, 'q-untyped.json'), encoding='utf-8'))
health = json.load(open(os.path.join(HERE, 'q-health.json'), encoding='utf-8'))
retr = json.load(open(os.path.join(HERE, 'q-retractions.json'), encoding='utf-8'))
chains = json.load(open(os.path.join(HERE, 'q-chains.json'), encoding='utf-8'))

GROUPS = {g['key']: g for g in typed['groups']}
UNI = {u['type']: u for u in typed['universal_rollup']}
UNIVERSAL = {'tooling.host-shell', 'contract.narrow-basis-claim', 'contract.premise-falsified',
             'contract.structural-blind-spot', 'contract.token-proxy-check'}


def clean(s, n=200):
    s = (s or '').replace('\n', ' ').replace('|', '/').strip()
    return s if len(s) <= n else s[:n - 1] + '…'


def imp(d):
    d = d or {}
    return ' '.join(f"{k}={v}" for k, v in d.items() if v) or '—'


def table(cases):
    out = ['| chunk | what | impact | evidence |', '|---|---|---|---|']
    for c in cases:
        ev = c.get('evidence') or c.get('id') or '—'
        out.append(f"| {c.get('chunk') or '(session-scope)'} | {clean(c.get('what'))} | {imp(c.get('impact'))} | {clean(str(ev), 80)} |")
    return '\n'.join(out)


def sums_str(s):
    return ' '.join(f"{k}={v}" for k, v in s.items() if v) or '—'


def merged(keys):
    rows = [GROUPS[k] for k in keys]
    n = sum(r['n'] for r in rows)
    sums = {}
    cases = []
    chunks = set()
    for r in rows:
        for k, v in r['sums'].items():
            sums[k] = sums.get(k, 0) + v
        cases += r['cases']
        chunks |= set(r['chunks'])
    w = (1 + sums.get('iterations', 0) + sums.get('retries', 0) + sums.get('reformulations', 0)
         + 2 * sums.get('dialogue_rounds', 0) + 3 * sums.get('halted', 0) + 3 * sums.get('soft_exit', 0))
    return n, w, sums, cases, chunks


# ---- typed proposals: (keys, pattern, proposal) — order = sort weight, merges noted ----
P = [
 (['phase/distill/contract.extract-format', 'wrap-session/reconcile/contract.proposal-format'],
  "The sub-agent return transport HTML-entity-escapes `<` `>` `&` inside code spans — 3–7 of 7 distiller returns per batch, in 24 of 39 chunks at phase/distill plus 3 wrap fan-outs (27 cases; 23 summed decode iterations). Every occurrence was remedied by the documented decode-on-save; the expensive halves were the per-batch re-verification and the raw-twin duty (twins were repeatedly RECONSTRUCTED by inverse-encode and judged sanctioned — see level candidate L7).",
  "Make entity-decode a MECHANICAL default of the fan-out persist path (scripted decode + `entities=0` probe, which several chunks already ran ad hoc), and codify reconstructed-by-inverse-encode raw twins in fan-out.md as the sanctioned form. The transport itself is harness-level — not reachable from skill text — so the skill-side goal is zero per-batch judgment cost."),
 (['implement/fix-loop/tooling.environmental', 'implement/smoke/tooling.environmental'],
  "One environment family dominates: parallel-link memory pressure (rust-lld IO failure · os 1453 · 0xc000012d/os 1455) and target/ disk accumulation (up to 268G; `cargo clean` recovered 62k files/225G/87G/180G/290G across the epoch — see the six removed-cause facts), plus regen/fingerprint rebuild bursts (CARGO_INCREMENTAL, bindings). 17 cases across 15 chunks, all recovered per-instance.",
  "Partly absorbed project-side (`--jobs 4` + build-first de-race curated; cargo-clean pre-authorized). Remaining pipeline generalization: promote the remedy from remembered rules to a mechanical hygiene surface — e.g. a target/-size + free-disk line in the new-session health checks (warn at threshold) and a documented clean cadence — so the family stops being rediscovered mid-gate."),
 (['phase/validate/contract.mechanical-check'],
  "P5's mechanical pass fired in 15 chunks — almost every case the gate WORKING as designed. The recurring sub-causes it keeps catching: acceptance criteria with no producing invocation in Test Commands, and Expected-amendments lists naming leaves or immutable artifacts (the masters-only rule re-learned per chunk).",
  "Move the two recurring sub-causes upstream into the P4 synthesis template as explicit authoring checks (per-criterion producing-invocation; masters-only Expected-amendments), so validate stops being their first reader. The check itself is healthy — keep it as the backstop."),
 (['implement/fix-loop/contract.spec-reality-gap'],
  "Fix-loop measurement falsified spec/plan premises in 10 chunks (2 soft-exits): dead surfaces, wrong causal mechanisms, recipes that do not reach the harness. Each was resolved by the designed paths (soft-exit, premise-closure, operator fork).",
  "No separate mechanism change — this group is the fix-loop face of the cross-step `contract.premise-falsified` class; see PU1 for the aggregate direction. Evidence kept here for the per-step picture."),
 (['wrap-session/reconcile/ambiguity.playbook-no-match'],
  "Five reconcile passes hit dispositions the drift playbook could not decide — two rules matching with opposing verdicts, a class with no rule, and two unease-escalations — costing 4 halts and 6 dialogue rounds.",
  "Each halt minted a rule at the time (drift-base grew), which is the designed loop. Direction worth judging: a standing tiebreaker for the two-rules-oppose shape (most-specific-wins, or escalate-with-both-cited) would remove the commonest recurrence."),
 (['wrap-session/route-resolve/ambiguity.trajectory-halt'],
  "The new-chunk-ahead trajectory class halted (or formally required a halt) in 8 chunks; in several the operator had ALREADY directed the entry in a prior directive, making the halt a one-round formality.",
  "Let a recorded operator pre-direction (a directive naming the entry/disposition) satisfy the trajectory gate without a fresh halt; the gradient keeps its force for genuinely new trajectory. See also level candidate L4 (the override signature on this same rule)."),
 (['phase/plan/ambiguity.scope-question'],
  "8 plan dialogues; 7 were genuinely the operator's call (placement forks, verification depth, guard shape) and resolved in one round each. One bundled a decisive lean as a question.",
  "None — the dialogue is doing its job at ~1 round per chunk. Minor direction: frame decisive leans as leans (recommend-first) per the one bundled case."),
 (['implement/fix-loop/retry.fix-iterations'],
  "Six chunks logged iteration counts at or above the protocol's typical band (17 summed; worst case 6 iterations, every one a distinct root cause in the headful harness), plus one deliberate near-soft-exit continuation.",
  "None mechanism-side — the counts contextualize harness-heavy chunks (webview legs dominate the tail). The soft-exit triggers behaved; the one near-fire chose continuation deliberately and recorded it."),
 (['implement/smoke/tooling.harness-friction'],
  "Seven chunks routed around the agent-run harness: `boot` forced a cold `cargo run --release` rebuild, `status` returned a well-formed envelope about no process (exit 0 with no app), `cleanup` killed the wrapper and reported done. The smokes fell back to direct-binary launches every time.",
  "ABSORBED BY THE PROJECT at epoch end: `harness:status` real-process verdict (2026-08-30-diagnostics-un-muting) + boot pre-build/direct-spawn + independent-probe cleanup (2026-08-30-agent-harness-teardown-truth). Nothing further proposed; residual: the ps1 arm's live drive is owed via the test-plan §1 trigger."),
 (['wrap-session/gates/tooling.commit-mechanics'],
  "The TauRPC bindings-clobber at the P7 light gate recurred across 9 wraps (any default-features cargo run rewrites bindings.ts to the no-mcp shape; the ordering rule then caught it each time). Eight more untyped fix-loop records carry the same event mid-implement — see extension candidate U-A.",
  "ABSORBED: the playbook ordering rule (regen last, verify staged) plus `check:staged-artifacts` (2026-08-30-staged-bindings-assertion) made the staged copy the gated truth. Remaining generalization: the wrap SKILL could sequence regen-last/verify-staged natively — the cause (taurpc dev-mode export on any default-features run) will exist in any TauRPC project, not just this one."),
 (['wrap-session/reconcile/contract.false-positive-proposal'],
  "Six detector proposals asserted wrong facts (overclaimed universals, false premises about what a chunk introduced, a trigger premise the chunk itself falsified); each was caught by the validate-before-apply posture at the cost of dialogue rounds and reads.",
  "Keep the posture. Direction: detector returns could carry a basis pointer (file+line measured) per claim, so the orchestrator's re-derivation is one read instead of a search."),
 (['new-session/orientation/tooling.output-cap-overflow'],
  "14 of 42 session starts overflowed the tool-result cap reading working-route.md (single-line multi-KB entries) or the skill-reference bundle; recovery was structural re-extraction each time (31 summed extra reads). Recurred at other steps too — see extension candidate U-F.",
  "Prescribe the structural read as the DOCUMENTED default in the new-session skill body: grammar-aware tail extraction (markers + first markerless line) and per-file reference reads. The entry-length half is project-side (see U-B long-line-edit)."),
 (['implement/fix-loop/contract.test-expectation'],
  "Six chunks hit pre-existing tests that pinned the defect being fixed, or expectations needing strengthen-not-relax treatment; all were resolved per the discipline (strengthened, never relaxed), plus one plan-listed health check probing an inert config.",
  "None — the discipline held everywhere. The pattern documents the standing cost of pinned-defect tests; the inert-config case was amended into test-plan §1 at its chunk."),
 (['implement/code/input.plan-step-ambiguous'],
  "Eleven chunks had a plan step underdetermined or self-contradictory at code time — placement rules instead of locations, two options one structurally impossible, steps contradicting their own touchpoints.",
  "Add a P4 synthesis check: every step names a concrete location and admits a single executable interpretation (pairs with P3's upstream move). Several cases trace to research under-enumeration — see chain X1."),
 (['phase/validate/contract.intent-divergence'],
  "Val-1 classified 8 intent-incomplete / falsified-premise cases the P3 closure had missed — mostly working as designed (2 dialogue rounds total). Two consecutive chunks showed the same sub-shape: premise closure corrected the premise BULLET but left sections DERIVED from it standing.",
  "Have the premise-closure instruction name DERIVED sections explicitly (not just tagged bullets). Otherwise healthy."),
 (['wrap-session/reconcile/contract.cascade-miss'],
  "Ten chunks found amendment leaves the cascade DAG table does not name (conventions.md and docs/services/* as architecture/plan leaves; duplicate-claim sites) — each caught only by the orchestrator's own grep, costing extra reads per wrap.",
  "Amend the cascade table to the measured leaf set once (conventions.md + services docs as architecture.md/plan leaves) — an Andromeda reference fix that has been re-derived at least four times."),
 (['wrap-session/gates/tooling.light-gate-red'],
  "Four first-run light-gate reds: the rlib-format-mismatch family twice, one designed-red-by-plan command, one load-sensitive race the identical implement run passed.",
  "Make the build-first de-race prefix the light gate's documented default on this host class; the designed-red case fed the P-shape rule already curated (never a combined designed-red invocation)."),
 (['wrap-session/curation/ambiguity.filter-borderline'],
  "Nineteen wraps hit curation borderlines; the dominant shape is candidates scoring EXACTLY the Filter-4 threshold (0.6 = verified-by-measurement 0.4 + specific-technical-detail 0.2) — the boundary sits precisely where real candidates mass — plus Filter-1 edges against content the same wrap's cascade had just written.",
  "Move the threshold off the mass point (0.55 or 0.65), or add a documented exact-threshold disposition; separately codify the dedup-vs-wrap-own-cascade clause (it recurred 3×). Low per-case cost, but it is the highest-frequency judgment sink in the loop."),
 (['wrap-session/curation/ambiguity.tier-routing'],
  "Fifteen wraps spent judgment routing candidates between two defensible homes (testing vs verification-harness vs security; standalone vs in-place extension), with the written tiebreakers not adjudicating.",
  "Add a one-line home-registry to curation-tier-decision.md (which rule file owns which class); it would decide the commonest contested pairs mechanically."),
 (['wrap-session/route-resolve/contract.carry-no-owner'],
  "Five wraps held carries/amendments with no owning route entry (deferred twice, dialogue twice) — the mint-an-owner path resolved each but at halt cost.",
  "Direction for judgment: sanction a standing per-version residuals pool entry that orphan carries pin to by default, so a missing owner stops forcing a mint-or-defer dialogue each time. (Changes route semantics — founder call.)"),
 (['phase/validate/contract.matrix-claim'],
  "Six claim-gate frictions: affordance caps verified via process proxies (found and corrected — the affordance-honesty clause landed mid-epoch), an acceptance passable without the subject running, and zero-claim chunks still warranting full matrix reads.",
  "The affordance-honesty clause covers the class going forward; direction: a mechanical claim-time check that the acceptance names the subject's real invocation."),
 (['phase/plan/input.extracts-conflict'],
  "Four plan-time conflicts between extracts/authorities that P2 aggregate check C caught late or not at all; three more untyped cases were intra-extract or extract-vs-skill-contract conflicts check C structurally cannot see (see U-G).",
  "Extend aggregate check C's classes to intra-extract tension and extract-vs-skill-contract disagreement."),
 (['implement/code/input.research-files-wrong'],
  "Four modify-lists under-enumerated the real edit surface (callers of reshaped fns, boundary files carrying transit types).",
  "Research playbook: require a graph caller-enumeration for every signature-changing symbol before the boundary list closes (chain X1's producer-side fix)."),
 (['phase/plan/input.research-thin'],
  "Three plans were starved by research omissions — the live-leg producer never inspected, a needed convention unrecorded, the decisive external-integration fork unexamined.",
  "Research checklist: when the plan will name a live leg, the leg's invocation form is a required research object (X1)."),
 (['phase/plan/retry.synthesis-rework'],
  "Three first-draft plans needed rework for self-inflicted defects (criterion without a producer; a matrix id that does not exist).",
  "Covered by P3's upstream checks; the matrix-id case suggests the P4 template link ids to the read matrix rather than free-typing."),
 (['wrap-session/reconcile/input.report-insufficient'],
  "Four reports missed entries or precision that reconcile's detectors then caught (a missing disproved-claim, a wrong number attributed to a default).",
  "Generalize the 2026-08-27 lesson: the report's disproved-claims and deviation bullets get a mechanical re-derivation pass against the chunk's own measurements before the fan-out reads them."),
 (['wrap-session/report/contract.detector-fact-gap'],
  "Nine wraps found that facts the detectors bind to have no home in the stock report bullets (harness/status-shape changes, scrub shapes, external-repo claims, disposition-less disproofs).",
  "Add the missing fact-slots to the report template — the detectors' input contract IS the report; each gap found here was a detector reading between bullets."),
 (['phase/research/input.extract-signal-gap'],
  "Six research passes found the decisive fact at a location no extract signalled (found by unsignalled repo-wide greps); one muted diagnostic blocked a question outright.",
  "Keep repo-wide greps mandatory (already the discipline); direction: distillers could emit a sections-silent-on-X signal so silence is distinguishable from absence."),
 (['implement/code/tooling.hook-friction'],
  "Four PostToolUse hook events: edition-2015 rustfmt parse gaps, stale contradictory diagnostics against untouched files, one rule pointing the wrong way once.",
  "Known host behavior, low cost; direction: a conventions note on which hook diagnostics are expected noise mid-burst (partially exists) and a debounce if available."),
 (['wrap-session/gates/tooling.gate-deferral'],
  "Four sanctioned deferral events at gates (audit-pin probes, proportional live-leg deferrals) — designed behavior, recorded by this type as intended.",
  "None — visibility only; the deferral machinery behaved (see also L8 for closure tracking)."),
 (['phase/take-up/input.working-entry-thin'],
  "Four take-ups needed extra reads because the entry omitted context the reader cannot cheaply re-derive (prior-version cap ids, the first table of a two-table class, a signature phrased as an existing reading).",
  "Route-resolve authoring guidance: name coordinates the NEXT reader cannot re-derive cheaply (ids' version, both tables, whether a signature exists yet)."),
 (['phase/take-up/input.carry-context-gap'],
  "Four carries contradicted themselves or lacked the arithmetic that changes what they demand (the interval-point count; a guard site in a file with no such seam).",
  "Same authoring-checklist direction; the interval-arithmetic case is now curated (confirm the session count at each wrap)."),
 (['phase/distill/input.spec-source-gap'],
  "Three distills surfaced contradictions INSIDE a master (stale deferral state, crate missing from its own enumerations) — the distiller doing its job.",
  "None — routes to wrap amendments as designed; kept for the record."),
 (['implement/smoke/tooling.subprocess-bounds'],
  "Three smokes paid interpretation cost on bounded-subprocess semantics (SIGTERM grace never honored → SIGKILL, rc=124 at the injector bound, Stop-Process exit codes read as failure).",
  "Largely closed by the teardown-truth chunk's probe-based verdicts; the rc-interpretation guidance is curated in test-plan §3."),
 (['phase/distill/contract.binding-contradiction'],
  "Three check-C contradictions between masters (halo hue driver design vs layouts; a11y CI binding; a scope-inferred claim restated).",
  "None — the check discriminated each time; the halo case was resolved by the deferral chunk."),
]

PU = [
 ('contract.premise-falsified',
  "The epoch's largest class: verification falsified premises stated by authored artifacts at NINE steps across 26 chunks — route-entry EVIDENCE clauses stating causal mechanisms as fact (the cadence-runaway central premise; the gap-resume mechanism chain), stale coordinates, capability claims, in-repo comments. The pipeline absorbs each instance by design (research-corrects-intent, premise closure, report bullets); the VOLUME is the signal: 57+ extra reads at the research/take-up faces alone.",
  "The largest sub-source is route entries and specs stating mechanism claims as fact. Direction: entry/amendment authoring marks causal-mechanism claims as hypotheses-until-measured (the CARRY grammar already distinguishes annotation kinds), and the verify-at-HEAD discipline — already curated project-side — becomes a named line in the take-up/research playbooks. That converts churn into one cheap re-derivation per claim."),
 ('contract.narrow-basis-claim',
  "38 cases at 11 steps across 22 chunks: counts/absences/availability claimed from a basis narrower than the claim — tail-clipped pipes, single-manifest globs, host-target cargo tree, anchors composed from memory, a stated-authority trace disagreeing with working memory. Every case was caught by re-derivation before or shortly after landing.",
  "The basis disciplines are curated project-side in several rule entries; the pipeline generalization is one line in the research/validate playbooks — 'a claim's basis must span the claim's full scope; re-derive before writing' — plus U3's shell recipes for the pipe-exit-code masking shape that manufactures narrow bases."),
 ('tooling.host-shell',
  "36 cases at 11 steps across 15 chunks — the Windows/MSYS semantics family: grep -P locale-gated, printf backslash collapse, MSYS path conversion (cmdkey /list, tasklist /FI, /tmp invisibility), msys≠Windows pid spaces, heredoc quoting failures, && chains dying on designed non-zero, Bash-tool cwd persistence. Every instance was worked around locally; the same knowledge is scattered across project rule files. See level candidate L1 — this is the band-aid theme's typed face.",
  "Consolidate a win32 host-recipe annex into the pipeline's shell-discipline references (python-by-path default, no -P, no printf-for-JSON, granular commands, PowerShell probes for pids/ports) so each new project stops rediscovering the family. Most content already exists as scattered curated rules here."),
 ('contract.structural-blind-spot',
  "15 cases at 9 steps across 8 chunks: documented mechanisms that could not reach a defect BY CONSTRUCTION — no detector owning falsified narrative mechanisms in spec bodies (3 reconcile cases), a deferred-gate age trigger that never fired across five re-pins, guards homed where [lib] test=false means they never run, per-procedure sweeps under-running fixed lists.",
  "Each case minted its own fix mid-epoch (D-obs-defect-narrative and siblings landed). Direction: fold the recurring class — 'a falsified narrative/mechanism claim in a spec body' — into drift-base as a first-class detector, since three independent cases converged on exactly that gap."),
 ('contract.token-proxy-check',
  "8 cases at 6 steps: checks written as literal-token tests where the property is semantic (the anchor check matching 'per ', placeholder-lookalike notation, epoch-diagnosed greps) — both failure directions occurred (spurious HALT and suppressed signal), and the truth came from reading the hits each time.",
  "Re-express the named checks (fan-out per-extract check 3; validate placeholder scan) in parse-based form; add the failure-direction note to their criteria so authors test both ways."),
]

EXT = [
 ('U-A', 'implement/fix-loop', 'tooling.generated-artifact-clobber', [1, 28, 32, 52, 57, 70, 3, 29],
  "A generated artifact (TauRPC bindings.ts) is rewritten as a side effect of running a documented gate, reddening an unrelated gate downstream.",
  "8 untyped cases (6 at fix-loop, 2 at gates beside the typed commit-mechanics group). Project-absorbed by the staged gate, but the TYPE is worth having: any codegen-emitting test run can clobber a committed artifact."),
 ('U-B', 'phase/take-up + wrap-session/route-resolve', 'tooling.long-line-edit', [35, 42, 49, 64],
  "A single-line multi-KB route/master entry defeats the anchored-Edit / Read path (anchor-match failures, consumed line breaks, forced duplicate reads).",
  "4 cases; sibling of the orientation output-cap group (P12). The recovery recipes (python read-modify-write by path, structural extraction) recurred unprompted — a type would let them converge."),
 ('U-C', 'wrap-session/route-resolve', 'contract.standing-pin-carriage', [14, 21, 34, 40, 61],
  "A standing-deferral pin's migration across frozen entries hits an edge (verbatim-migration impossible after the satisfying wrap; compact form blocked by a changed owned set; between-point half-satisfied upstream).",
  "5 cases on the cargo-audit pin alone, including one positive first-exercise. The machinery works; the type would make its recurring edges visible."),
 ('U-D', 'phase/validate + phase/take-up', 'input.out-of-pipeline-source', [6, 33, 43],
  "The decisive fact lives outside every artifact the skill's input contract reads (a sibling repo, a gitignored side-note, a repo phase cannot read).",
  "3 cases. Criteria line: record when a step's conclusion required (or was blocked by) a source outside the skill's declared inputs."),
 ('U-E', 'cross-step', 'contract.skill-reference-drift', [4, 16, 45, 50],
  "An Andromeda skill body/reference states a path, enumeration, or rule that mismatches the deployed reality or its sibling reference (flat tree.db path vs per-plane; the nudge needing a read the evolve contract forbids; a 3-form fold list vs a 4th form in the wild; the version-dir glob aimed at the wrong root).",
  "4 cases. These are pipeline defects invisible to project drift detection; a type routes them to the founder."),
 ('U-F', 'all steps', 'tooling.output-cap-overflow → promote to Universal', [74],
  "The orientation playbook already types this; it fired untyped at phase/research (a -A6 grep) and appears as problem-facts at several other steps.",
  "Promote the existing type to the Universal list so any step can use it; the criteria line already exists in the orientation playbook."),
 ('U-G', 'phase/plan', 'input.extracts-conflict — broaden criteria', [11, 36, 44],
  "3 untyped conflicts were INTRA-extract or extract-vs-skill-contract — shapes the existing type's extract-vs-extract wording does not cover and P2 aggregate check C structurally cannot see.",
  "Broaden the type's criteria line to any authority-conflict surfacing at synthesis, and extend check C's classes to match (pairs with P22)."),
 ('U-H', 'wrap-session/route-resolve', 'contract.carry-no-owner — broaden to placement', [9, 10, 69],
  "3 untyped cases were placement/slot/owner ambiguity (a forced-early trajectory decision; unpinned slots after a partial operator direction; a non-obvious owner) — adjacent to the typed carry-no-owner group.",
  "Broaden the criteria to carry/entry PLACEMENT ambiguity, or add ambiguity.carry-placement beside it."),
 ('U-I', 'implement/fix-loop + implement/smoke', 'contract.vacuous-check-found', [18, 19, 67],
  "The mutation protocol keeps surfacing vacuous or insensitive checks (a pin iterating an empty vector; four pre-existing tests insensitive to the neutralized fixture; a verdict half true in both worlds).",
  "3 cases, all positive yield. A type would let the diagnosis COUNT the mutation protocol's catch rate instead of finding it in prose."),
]

CHAINS_MD = """### X1 — phase/research —[research]→ implement/code · implement/fix-loop · phase/plan — 13 joins, ≥10 chunks
The dominant lineage: research returns `outcome: ok` carrying the honest `unresolved-questions` signal, and downstream consumers grade the artifact `thin`/`wrong` — under-enumerated caller/boundary sets (fingerprint-feed, baseline-family, a11y ×3, interpretation-brief), uninspected live-leg forms (metrics-labels), one wrong-violator claim (ingest-stall). Both ends' records are in q-chains.json.
**Direction:** the producer-side checks in P23/P24 (graph caller-enumeration per signature change; live-leg invocation as a required research object). Consumers could also treat `unresolved-questions` as "boundary list provisional" rather than final.

### X2 — phase/plan —[plan]→ implement/fix-loop · implement/smoke · wrap-session/gates · phase/validate · implement/code — 13 joins, ≥9 chunks
Plan Test-Commands defects surface downstream: a combined deny invocation that can never be green, a wrong playwright config, a binary path that exists on no host, a SCENARIO= flag that does not exist, an omitted smoke on a boot-path touchpoint. Validate's producing-invocation rule catches some; the rest land at implement or the light gate.
**Direction:** a P5 mechanical executability pass over Test Commands (cheap --help/--list dry-runs; path existence), pairing with P3's upstream authoring checks.

### X3 — wrap-session/report —[report]→ wrap-session/reconcile — 2 chunks
Reports sufficient for six of seven detectors while the seventh finds a missed disproof; a wrong number attributed to a default. **Direction:** P26/P27 (report re-derivation pass + fact-slots).

Single-occurrence shapes (distill→research extracts; take-up self-joins; plan→report) are in q-chains.json; no aggregation claimed. 176 anchors total; most NO-PRODUCER joins reflect artifact-name granularity (file paths vs artifact names) — a known limit of the starting heuristics."""

LEVEL_MD = """### L1 — band-aid · environment · Windows host-shell semantics — 36 typed cases + ~25 workaround facts
**Facts:** the U3 table plus the workaround pile (grep -P locale ×6+, MSYS path conversion, printf collapse, heredoc EOFs, pid spaces, /tmp invisibility, && chain aborts, cwd persistence) — every fix a local workaround or a project rule entry.
**Level hypothesis:** the cause lives in the host+pipeline recipe layer (skills prescribe POSIX-shaped probes this host breaks); the fixes have accumulated in ONE project's rule files.
**Proposal:** the U3 win32 recipe annex at pipeline level.

### L2 — band-aid · environment · compound-command permission denials — ~11 facts
**Facts:** rm -rf + mkdir + GUI-launch compounds denied by the permission layer in ≥9 chunks; every chunk re-shaped to granular steps + fresh unique dirs (q-level.json pile).
**Level hypothesis:** the deny class is stable and the skills' smoke recipes keep producing the denied shape.
**Proposal:** smoke recipes prescribe granular steps + unique-dir-no-delete natively (no rm in the launch path).

### L3 — CLOSED band-aid · process/environment · harness verbs unusable for smokes — ~10 facts
**Facts:** direct-binary launches replacing agent-run boot/status across 9+ chunks (pile), plus the typed P9 group.
**Level hypothesis → outcome:** absorbed chunk-by-chunk until two dedicated project chunks removed the cause (harness:status; teardown-truth). The epoch's textbook band-aid→root-fix arc — recorded as closure, no proposal.

### L4 — override · process · trajectory-halt fires on pre-directed entries — 4+ facts (+ P6's 8 cases)
**Facts:** operator pre-directions repeatedly satisfying new-chunk-ahead halts (workspace-key wrap, ingest-stall, incident-persist, mechanics-probe re-home; E3 lookback ×2 same shape).
**Level hypothesis:** the RULE is miscalibrated for the already-directed case, not the project.
**Proposal:** pre-direction satisfies the gate (P6).

### L5 — rule-friction · process · inline-python vs file-by-path discipline — ~14 facts, both directions
**Facts:** ~9 deliberate inline uses noting "no mangling occurred" beside ~5 transport failures that vindicate the rule (heredoc EOFs, printf collapse, $TMPDIR-unset root write).
**Level hypothesis:** the rule's scope does not match the risk profile — small quote-free payloads are safe inline; document-sized or quote-bearing payloads are not.
**Proposal:** make the discipline size/content-conditional so compliance matches risk, instead of a blanket rule that is routinely and harmlessly violated until it isn't.

### L6 — chronic-degrade · resources · parallel-link memory + target/ disk accumulation — E3:1 → E4:17 typed + 6 removed-cause events
**Facts:** the P2 table; cargo-clean reclaims of 225G/290G/180G/87G; two new presentations of the same family (os 1455, 0xc000012d) folded into the testing.md family entry mid-epoch.
**Level hypothesis:** recurs across epochs, never halts, remedied per-instance — the halt policy structurally never surfaces it.
**Proposal:** P2's mechanical hygiene surface (health-check line + clean cadence).

### L7 — band-aid · process · raw-twin duty under entity escapes — ~8 facts
**Facts:** twins skipped/reconstructed-by-inverse-encode at ≥8 distills, each with an in-the-moment justification note (pile).
**Level hypothesis:** the reference's twin rule predates the entity-escape reality; reconstruction IS the de-facto sanctioned form.
**Proposal:** codify it (P1).

### L8 — deferred-forever scan · residue after closure-tracing
**Facts:** 16 deferred facts + 5 `deferral-open` signals. Closures TRACED for the major ones (muted diagnostics → 2026-08-30 sweep; strict-path → advisory-backlog DROP; injector counter → replay salt; cadence runaway → its chunk; flip-compaction regex → later flips ran).
**Residue:** the 2026-08-15 Suggested-storm drop asymmetry has no closure visible in-ledger; the msedgedriver-path unit-test gap is trigger-owned (open by design); TWO deferred facts carry EMPTY notes (idle-observer + diagnostics-sweep gates records) — a capture defect: a deferral fact with no note is untraceable. Worth one mechanism-health eye."""


def build():
    L = []
    A = L.append
    A("# Evolve Diagnosis — andromeda-pulse · Epoch 4 — Polish & ship: verification (epoch-to-date) · 2026-08-31T07:46Z")
    A("")
    A("> Every count below is grounded in `.andromeda/friction-log.ndjson` (Epoch-4 slice, retraction-filtered); raw query outputs are the `q-*.json` twins beside this file. Epoch-to-date: the Conductor-return entry is still markerless — its future records are not in this slice. Obligation-free throughout: accept / reject / defer / modify, no mechanism-side consequence.")
    A("")
    A("## Mechanism health")
    A("")
    h = health
    A(f"- **Records:** {h['epoch_records']} in-epoch ({h['step']} step / {h['friction']} friction) · 39 chunks · unparseable {h['unparseable']} · retracted 1 (0 unresolvable; id `2026-08-23T22:09:53Z-a`) · no retraction-of-retraction.")
    nd = h['null_epoch_detail']
    A(f"- **Null-epoch records:** {h['null_epoch_records']} ({nd[0]['ts'][:10]}, {nd[0]['skill']}/{nd[0]['step']} + {nd[1]['skill']}/{nd[1]['step']}, chunk:null — the 0-pending adaptation-wrap era; excluded from the slice by the epoch filter, era-legitimate).")
    A(f"- **Step coverage:** {h['chunks_exact_expected']}/39 chunks at the exact expected 5/3/5. Deviations (3): "
      + "; ".join(f"`{c}` {d['all']}" for c, d in h['coverage_deviations'].items())
      + " — one MISSING phase/plan checkpoint (cadence-runaway: a checkpoint that did not fire), two extra-run cases (delegated-timing implement 4; duplicate-span-replay implement 6 — the mid-implement pivot re-entered steps).")
    A(f"- **Session starts:** {h['session_starts_new_session']} new-session orientation records; 12 chunk-null wrap step records (adaptation / 0-pending wraps).")
    pf = h['problem_fact_fill']
    idf = h['id_fill_post_boundary']
    A(f"- **Problem-fact fill:** {pf['populated']}/{pf['step_records']} step records carry ≥1 deviation fact (fill is expected <100%). **Id fill:** {idf['with_id']}/{idf['post_boundary_records']} post-2026-08-18 records carry ids (100%).")
    unt_tot = sum(v['untyped'] for v in h['untyped_per_step'].values())
    A(f"- **Untyped rate:** {unt_tot}/{h['friction']} overall (14%). Hotspots: route-resolve 12/36, smoke 8/37, orientation 7/42, fix-loop 11/63, gates 6/32 — route-resolve's third is the strongest playbook-coverage signal (see the extension candidates).")
    A("- **Calibration boundaries in range:** `id` required from 2026-08-18 (E4 spans it; 246 earlier records read against their era). Deviation scan + universal types live for the whole epoch. Two deferred problem-facts carry EMPTY notes (capture defect — see L8).")
    A("- **Self-reference note:** this diagnosis session's own 2 orientation records (2026-08-31) are in the slice by design.")
    A("")
    A("## Proposals (typed patterns)")
    A("")
    pn = 0
    for keys, pattern, proposal in P:
        pn += 1
        n, w, sums, cases, chunks = merged(keys)
        key_disp = keys[0]
        if len(keys) > 1:
            key_disp += "  (+ merged: " + ", ".join(keys[1:]) + ")"
        A(f"### P{pn} — {key_disp} — {n} cases · weight {w} · {len(chunks)} chunk(s)")
        A(f"**Pattern:** {pattern}")
        A(f"**Evidence:** all {n} cases · summed impact: {sums_str(sums)}")
        A("")
        A(table(cases))
        A("")
        A(f"**Proposal:** {proposal}")
        A("")
    A("### Cross-step universal-type patterns")
    A("_These rollups absorb their per-step subgroups (the per-step split is shown in each table's rows); nothing below is double-counted in P1–P35._")
    A("")
    un = 0
    for t, pattern, proposal in PU:
        un += 1
        u = UNI[t]
        A(f"### PU{un} — cross-step · `{t}` — {u['n']} cases · weight {u['weight']} · {len(u['chunks'])} chunk(s) · steps: {', '.join(u['steps'])}")
        A(f"**Pattern:** {pattern}")
        A(f"**Evidence:** all {u['n']} cases · summed impact: {sums_str(u['sums'])}")
        A("")
        A(table(u['cases']))
        A("")
        A(f"**Proposal:** {proposal}")
        A("")
    A("## Cross-step chains (starting heuristics)")
    A("")
    A(CHAINS_MD)
    A("")
    A("## Level candidates (systemic-masked-as-project)")
    A("_Facts and clusters in `q-level.json` (Pass-A pile: 172 workaround/prohibition · 39 removed-cause · 16 deferred · 13 overridden + 5 E3-lookback). No verdict that any level call was wrong — that judgment is the founder's._")
    A("")
    A(LEVEL_MD)
    A("")
    A("## Playbook-extension candidates (untyped patterns, F-4)")
    A(f"_75 untyped records in-epoch (E3 lookback: {untyped['epoch3_count']}); clusters below meet F-4 (n≥3, or promoted-type evidence). Extending a playbook is an Andromeda change only the founder applies._")
    A("")
    e4u = untyped['epoch4']
    for uid, owner, ptype, idxs, criteria, note in EXT:
        A(f"### {uid} — {owner} — {len(idxs)} cases → proposed `{ptype}`")
        A(f"**Draft criteria line:** {criteria}")
        A(f"**Note:** {note}")
        A("")
        A(table([e4u[i] for i in idxs]))
        A("")
    A("## Below threshold — no action")
    A("_Visible for the founder's eye only; nothing here is proposed._")
    A("")
    below = [g for g in typed['groups'] if g['type'] not in ('UNTYPED',) and g['type'] not in UNIVERSAL
             and not (g['n'] >= 3 or (g['n'] >= 2 and (g['sums'].get('halted', 0) + g['sums'].get('soft_exit', 0)) > 0))]
    for g in sorted(below, key=lambda r: -r['n']):
        A(f"- [{g['n']}×] {g['key']} — {sums_str(g['sums'])}")
    A("")
    A("Emerging untyped singletons/pairs (watch next epoch): AskUserQuestion rejects preview:null options (1) · parse-mutate-json.dumps destroys authored JSON layout (1) · report-prose defects no gate reads (CJK stray char; bullet disposition) (2) · rotated-log bare-name trap re-hit (1) · same-chunk procedure+caller needs regen→dist→re-embed before any live leg (1, curated) · sidecar MCP double-gate applies to the sidecar binary itself (1) · code-graph calls view mis-attributed a caller (1) · smoke self-check catching harness-sourced false claims (accessor typo; total-0 misread) (2) · permission-policy denial has no covering type (1 — theme owned by L2).")
    A("")
    A(f"_Chains twin: q-chains.json ({chains['anchors']} anchors). Retractions twin: q-retractions.json. Generated by compose_proposals.py from the q-*.json evidence twins._")
    return "\n".join(L)


md = build()
out = os.path.join(HERE, 'proposals.md')
open(out, 'w', encoding='utf-8').write(md)
below_n = len([g for g in typed['groups'] if g['type'] not in ('UNTYPED',) and g['type'] not in UNIVERSAL
               and not (g['n'] >= 3 or (g['n'] >= 2 and (g['sums'].get('halted', 0) + g['sums'].get('soft_exit', 0)) > 0))])
print(f"written {out}")
print(f"lines {md.count(chr(10))+1} · proposals {len(P)+len(PU)} ({len(P)} typed + {len(PU)} universal) · extension candidates {len(EXT)} · level candidates 8 · below-threshold {below_n} typed groups + 9 emerging untyped")
