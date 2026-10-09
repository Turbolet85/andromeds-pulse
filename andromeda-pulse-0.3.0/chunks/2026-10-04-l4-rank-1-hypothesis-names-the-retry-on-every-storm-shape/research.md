# Codebase Research — 2026-10-04-l4-rank-1-hypothesis-names-the-retry-on-every-storm-shape

## Scope
- **Depth:** moderate · **Reads:** 11 · **Globs/Greps:** 10 (+1 code-graph query, rust plane, `db_state: fresh`)
- **Harness rules consulted:** `.claude/rules/verification-harness.md`. It is unchanged since the predecessor's
  full read (`git log -1` on it: `76d6cca 2026-10-04 04:30`, before the predecessor's 22:55 take-up), so that read
  stands: §Scenario legs (lines 92-98) re-read here. The probe is neither an `agent-run` verb nor an xtask leg, so no
  firing-form recipe names it. The six newest Session-Additions introducers (lines 146-156) were re-read and none bears
  on a real-model probe run. `.claude/rules/testing.md` (auto-loaded) applies:
  - 2026-08-16: prebuild outside a timed run.
  - 2026-08-29: an `[[example]]` needs `test = true`.
  - 2026-09-30 [ext. 2026-10-04]: evidence hygiene.
  - 2026-09-30 sweep hazard: grep the bare version literal with a word boundary.
  - 2026-10-04: diff probes name the chunk base sha.
  - 2026-10-05: zero Rust delta never defers nextest; this chunk has a Rust delta anyway.
  - 2026-10-04 [ext.]: never batch a mutation Edit with its test run.
- **Platform issues consulted:** none. There is no runner-only bullet (the take-up CI read was in progress, not red),
  and no CI-reading entry exists outside the operator leg.
- **External inputs:** two snapshots, each used without a repeat read.
  - `inputs#I1` — the overseer's phase directive: the 4317/4318 slot and the model slot are granted for this chunk,
    and conductor-builder stays idle until its wrap.
  - `inputs#I2` — the leg env file. It is byte-identical to the predecessor's `I2` copy (`cmp`, P1). It exports
    `ANDROMEDA_PULSE_MODEL_PATH`, `_LLAMA_CUDA_BIN_PATH` and `_LLAMA_CPU_BIN_PATH`, and sets no
    `ANDROMEDA_PULSE_HARDWARE_PROFILE`, `_L4_ALLOW_ROOT` or `_L4_DETERMINISTIC`.

## Files inspected
- `pulse-app/examples/l4_decision_probe.rs` (1-460, 560-880 of 1064). This is the probe.
  - Shapes S1-S4 sit at `:149-199`.
  - `prepare()` at `:303-376` composes each arm through the real `render_payload` + `build_primary_tier_prompt`, and
    applies an arm's transform as an exactly-once text edit (`A4_ANCHOR`, `remove_lines`).
  - The grader `names_trigger` sits at `:425-448`, with its terms at `:411-419`.
  - The verdict print sites are `:681` (header), `:827` (per-arm `names_trigger` line) and `:864` (names-trigger
    verdict).
  - A test fixture carries `prompt_version: "v2.4"` at `:885` (test data, not a version pin).
- `crates/triage/src/digest/assembler.rs` (552-710). `render_payload` emits the lines in this order:
  `WINDOW` → `PROJECT` → `OVERALL` → `TRIGGER: {cue_cause_label(first cue)}` (`:678-683`) → `SERVICES` rows
  (`{rate}/s | {err%} | {p99}ms`, `:684-695`) → `ATTENTION CUES` (`[{tier}] {kind_label} — {summary}`, `:696-706`) →
  `CORPUS MATCHES` with its framing note (`:707-`).
- `crates/interpretation/src/prompt.rs` (1-280). The levers live here:
  - `TRIGGER_FRAMING_INSTRUCTION` at `:87-92` (one const, pushed into the Output Instructions of all three tiers at
    `:266/:359/:450`);
  - `CONVENTIONS_SNIPPET` (`:107-114`, "Hypotheses ranked highest-confidence-first");
  - `ROLE_DEFINITION` (`:96-102`).
  - The version pin `assert_eq!(PROMPT_VERSION_*, …)` sits at `:568-570`.
- `crates/interpretation/src/schema.rs` (31-52). `PROMPT_VERSION_PRIMARY = "v2.4"`, `_FALLBACK = "v1.3-fallback"`,
  `_REFLECTION = "v1.3-reflection"`, each with a doc line per bump.
- `crates/interpretation/src/schema.json` (52-78). The `hypotheses` description is "Ranked hypothesis list per P-033
  (primary tier emits up to 5; fallback tier emits 1)". The `statement` item carries no description.
- `crates/triage/src/contract.rs:205-214`. `cue_cause_label(RetryStorm) = "Retry storm"`.
- `pulse-app/src/deterministic_inference.rs:40`. The canned output carries `"prompt_version": "v2.1"`, a fixed
  literal. It reads no prompt, so a prompt or digest change cannot reach it.
- The predecessor's `evidence/{shipped,nf}-runs.json` (re-tallied here) and `evidence/series.md`.
- `andromeda-pulse-0.3.0/chunks/2026-10-01-real-model-incident-surfacing/plan.md` (20-35, 168-180, 536-543) is the
  project's precedent for choosing a real-model remedy. It ran an arm matrix on the untouched base, applied a decision
  rule fixed before the run (qualify by threshold, select the least-blast-radius qualifier), then re-measured the
  shipped prompt at a pre-registered count.

## Graph impact (from the code-graph query; rust plane)
Query: `refs` for the 8 lever symbols (126 rows, trace `tree-query-{marker}.json`).
- **TRIGGER_FRAMING_INSTRUCTION** — 9 refs: `prompt.rs:211/265/304/358/395/449` (three builders, size + push),
  `prompt.rs:592` (the per-tier exactly-once pin), and `l4_decision_probe.rs:64/386` (the `nf` arm strips it by exact
  line equality). A rewording therefore reaches every tier at once, and the `nf` strip keeps working because it keys on
  the const.
- **build_primary_tier_prompt** — 28 refs. The production callers are `inference_runtime.rs:420` (L4 primary) and
  `investigate_router.rs:247` (a second consumer, `investigate.run_action`). The rest are `integration_real_llama_cli.rs:133`,
  the probe at `:337`, and 20 in-crate test sites. So a framing-text change also reaches `investigate.run_action`.
  The instruction is conditional ("When the digest carries a TRIGGER line"), so a digest without one reads correctly.
- **render_payload** — 15 refs: 9 in `assembler.rs` (4 production paths `:337/361/376/392` + 5 pins), `contract.rs:152`
  and `digest/mod.rs:35` (re-exports), `unit_digest_runtime_scrub.rs:32/50`, and the probe at `:77/:317`.
- **PROMPT_VERSION_PRIMARY** — 7 refs: `prompt.rs:227` (the composed version line) and `:540/:567` (pins),
  `schema.rs:357`, and `inference_runtime.rs:421` (the `interpretation.prompt.assemble` field).
- **L4_OUTPUT_JSON_SCHEMA** — 29 refs: every builder; `investigate_router.rs:256` (the `--json-schema-file`
  copy); `inference_runtime.rs:442`; and the probe's A5 transform (`:274/346/357/361`). A schema-description edit
  reaches all three tiers plus investigate, and A5's exact-text reorder transform.
- **cue_cause_label** — 5 refs: the TRIGGER line (`assembler.rs:680`) and the producer's title grounding
  (`inference_runtime.rs:676`). Neither is a lever this chunk may move, per the kind-label-only ruling below.

## Patterns detected
- **Exactly-once probe transforms** (`l4_decision_probe.rs:351-363`, `:390-408`). An arm edits the HEAD composition
  with an anchored replacement or removal and fails INCONCLUSIVE (exit 2) when its anchor is not found exactly once.
  A candidate-remedy arm follows this shape, so every candidate is measured as "HEAD plus one change".
- **Pre-registered two-slot remedy selection** (2026-10-01 plan `:20-28`, `:168-180`). This is the project's
  established design. Slot 1 measures the candidate arms on the untouched base, baseline first. A decision rule fixed
  before the run qualifies arms by the bar and selects the least-blast-radius qualifier. The fix is then applied to the
  tree. Slot 2 re-measures `shipped` with fresh generations at the pre-registered count, and that reading is the
  acceptance.
- **Prompt lineage bumps together** (`schema.rs:31-52` doc lines; the arch and tests histories). Any template change
  moves all three tiers: v2.5 / v1.4-fallback / v1.4-reflection. The pins ride the consts.

## Conventions to follow
- **ASCII template text** (`prompt.rs` consts; pinned by `composed_prompt_templates_are_ascii_clean_for_argv_transport`).
- **The framing instruction stays exactly once per tier, after `CITING_INSTRUCTION`**
  (`framing_instruction_present_in_every_tier_output_instructions`, `prompt.rs:592`).
- **The TRIGGER line carries the closed kind label only, never `scope_id`.** This is an overseer scope ruling recorded
  at `2026-10-04-l4-interpretation-names-its-triggering-cue` (architecture-amendments, surfaced by the arch history).
  The security history adds that non-closed storm content in that line would need its own boundary disposition.
- **The probe records bounded labels only** (`:22-28`, `:773-784`). No model text is ever written.

## New files to create
- `andromeda-pulse-0.3.0/chunks/2026-10-04-l4-rank-1-hypothesis-names-the-retry-on-every-storm-shape/evidence/`

## Files to modify
- `pulse-app/examples/l4_decision_probe.rs` — candidate-remedy arms and any pre-registered grader reading (P4 fork), with their pins
- `crates/interpretation/src/prompt.rs` — the selected framing text, if the selected arm is a prompt-text lever, and its pins
- `crates/interpretation/src/schema.json` — the hypotheses description, only if the selected arm is the schema lever
- `crates/interpretation/src/schema.rs` — the three `PROMPT_VERSION_*` bumps with their doc lines
- `pulse-app/tests/unit_inference_runtime.rs` — the three `prompt_version=` literals at `:539/:561/:628` (version sweep)
<!-- Version sweep: `grep -rnE 'v2\.4\b|v1\.3-(fallback|reflection)'` over crates, pulse-app/{src,tests,examples}
and xtask (*.rs, *.json, *.ts, *.tsx): 16 hits. 3 changed consts (schema.rs:33/40/52) + their 3 doc lines
(schema.rs:31/39/50) + 3 pin asserts (prompt.rs:568-570) + 6 literal asserts/messages (unit_inference_runtime.rs:
539/540/561/562/628/629) are this chunk's. 1 no-change: l4_decision_probe.rs:885 (a parsed-output test fixture; its
value is the model's echoed field, not the shipped version). Canned output v2.1 (deterministic_inference.rs:40)
does not match the pattern and is unaffected. -->

## Open questions
- **How is the remedy chosen and measured?** A single remedy, picked by reasoning and measured once against the bar?
  Or the precedent's two-slot design: a candidate arm matrix on the untouched base under a fixed rule, then a fresh
  confirmation of the shipped tree? And does the confirmation add held-out storm shapes the selection never saw? →
  blocks: plan-decision (the series design, the arm set, the decision rule, the acceptance).
- **What does the grader record?** `names_trigger`'s retry term is the substring `retry`. `"retries"` and `"retried"`
  do not contain it; `"retrying"`, `"retry_storm"` and `"retry storm"` do (measured:
  `['retry' in w for w in ('retries','retried','retrying','retry_storm','retry storm')]` →
  `[False, False, True, True, True]`). A rank-1 statement naming "excessive retries" therefore grades
  `elsewhere`/`none`. How much of the predecessor's S2/S3 shortfall is this is NOT measurable from the committed
  evidence, which records labels only. Keep the verdict grader frozen and add a record-only stem reading, or change the
  grader before any run? → blocks: plan-decision (the grader, the evidence form, comparability with 30/40).
- **Which levers are candidates?** Given the kind-label-only TRIGGER ruling, they are:
  - the framing instruction's wording;
  - the `hypotheses` schema description;
  - the conventions snippet.

  Digest re-ordering is a larger-blast-radius digest change. → blocks: plan-decision (resolved inside the series-design
  fork).

## Live-leg invocation (the series)
- Preconditions, measured at P3:
  - `cargo build -p pulse-app --example l4_decision_probe` exits 0. It was warm here (0.37 s); the build printed the
    triage tokenizer download warning because the leg env was not sourced, which matches the predecessor's plan defect.
  - `l4_decision_probe --arms shipped,nf --dry-run` exits 0 with shipped S1-S4 at 6984 / 6987 / 6991 / 7185 B and nf
    S1-S4 at 6644 / 6647 / 6651 / 6766 B (max 16384). These are identical to the predecessor's reading.
  - Disk: 692 G free; `target/` is 70 G. No flycheck cargo was running (`ps` selected by `comm == cargo` +
    `--message-format=json`: none).
- Firing form, carried from the predecessor's entries:
  - `unset ANDROMEDA_PULSE_HARDWARE_PROFILE ANDROMEDA_PULSE_L4_ALLOW_ROOT ANDROMEDA_PULSE_L4_DETERMINISTIC`
  - `. "$HOME/dev/projects/additional/pc-overseer/l4-env.sh"` (`inputs#I2`), sourced BEFORE any cargo build, so the
    tokenizer is not fetched.
  - The probe argv, with `--out` minted per run under `target/l4-decision-probe/`.
- Verdict atoms, from the print sites: `:681` `binary llama-cli (cuda, -ngl 99)`; `:827` `arm {arm}: names_trigger
  rank1`; `:864` `l4-decision-probe: names-trigger verdict: {PASS|FAIL} · rank1 r/n`.
- Time: the predecessor measured 112 s and 119 s wall per 40-generation arm (`evidence/series.md`).

## Scope premise closure
- **Item 1** (`[inferred]` the lever is not stated by the entry) — VERIFIED as open. The candidate set is narrowed by
  two recorded constraints: the overseer's kind-label-only TRIGGER ruling (arch history) and the security history's
  rule that non-closed content needs its own boundary disposition. The choice stays a P4 fork (Open question 1).
- **Item 2** (`[inferred]` "the pre-registered bar" = the predecessor's rule, >= 36/40 over S1-S4 at n = 10) —
  VERIFIED. That is the rule in the predecessor's plan entry 13 and in the arch [Fault Identity] measured clause.
  Held-out shapes stay the P4 fork the scope names.
- **Measured-marked claim** (S2 6/10, S3 5/10, no corpus match; S4 10/10) — spot-checked still true. A re-tally of the
  committed `shipped-runs.json` gives S1 9 · S2 6 · S3 5 · S4 10 (30), and `nf-runs.json` gives S1 6 · S2 6 · S3 1 ·
  S4 5 (18). The caveat stands: at n = 10, a smaller corpus effect is not excluded.
- **Hypothesis-marked claim** ("what separates S2/S3 from S1 lies in the shapes' own storm content") — the RENDER half
  is VERIFIED at HEAD; the causal half is UNRESOLVED.
  - From `:149-199` through `render_payload`, S2's storm service row renders `payment-service 9.0/s | 35.0% | 110ms`.
    S3's renders `inventory-service 7.0/s | 12.0% | 480ms`, against neighbours at `40-120ms`.
  - S1's storm service renders `100.0%` errors, and S1 reads 9/10. So "a competing elevated metric" alone does not
    separate the shapes: S1 has the starkest metric of all.
  - A second, unmeasured candidate explanation is the grader's stem gap (Open question 2). The causal half stays
    `hypothesis:` and is not stated as fact in the plan.
- **`[inferred]` prompt version bump with its sweep** — VERIFIED. 16 sweep hits are listed under Files to modify
  (15 this chunk's, 1 no-change).
- **Boundary: the deterministic runner stays green** — VERIFIED. The canned output is a fixed JSON literal with
  `prompt_version: "v2.1"` (`deterministic_inference.rs:40`) and reads no prompt. The producer pins
  (`unit_incident_producer.rs`) consume the canned output, not prompt text.
- **One finding the scope did not state** — the framing instruction also reaches `investigate.run_action`
  (`investigate_router.rs:247`). Its conditional wording keeps a TRIGGER-less investigation digest correct; any
  rewording must stay conditional.
