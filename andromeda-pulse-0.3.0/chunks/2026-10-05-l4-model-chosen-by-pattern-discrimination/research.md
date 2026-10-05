# Codebase Research — 2026-10-05-l4-model-chosen-by-pattern-discrimination

## Scope
- **Depth:** moderate. The modify-set is one dev-only `[[example]]` plus probe-only evidence. The depth goes into the probe,
  the digest renderer it drives, and the product path the cueless digests take.
- **Reads:** 14. **Globs/Greps:** 19. **Code-graph queries:** 1 (rust plane).
- **Harness rules consulted:** `.claude/rules/verification-harness.md`, by structural read: the 15-section header index,
  the 22 Session Additions introducers (`awk` over `^- 20` from line 111), and §Scenario legs (92–98) read in full.
  - Two additions apply: 2026-08-28 (a leg reproducing a measurement is a scenario, never a gate) and 2026-08-29 (a
    harness's negative finding needs a second source).
  - The predecessor's leg form carries a third practice: the build is done outside the timed run.
  - No real-model probe leg is registered as a scenario or a gate. The probe's legs stay `leg = 'operator'`, as at the
    predecessor.
- **Platform issues consulted:** none. No runner-only bullet is folded here. The CI red read at P3 is dispositioned in
  scope §Second fold source, and its log could not be read yet (the run is still in progress, `gh run view
  --log-failed` refuses).
- **External inputs:**
  - `inputs#I1` — the founder-approved design: shapes, scoring, audit, the recommendation rule, the six models.
  - `inputs#I2` — the phase directive: the slot and ports granted, daytime only, ground truth before any run, texts in
    `target/`, the six models under their authors' settings with GBNF and thinking off.
  - `inputs#I3` — the overseer's `l4-env.sh`, which every real-model leg sources. It sets the b9305 CUDA llama-cli for
    both routes and the default model path.

## Files inspected
- `pulse-app/examples/l4_decision_probe.rs` (read in full, 1948 lines).
  - It is the series engine. Shapes are built in-process through the real `render_payload` and
    `build_primary_tier_prompt` (`:229-301`, `:413-499`).
  - Each generation is one llama-cli spawn with the production argv (`compose_argv` `:862-890`, `generate`
    `:892-968`), the production timeout (`LLAMA_CLI_TIMEOUT`), the bounded extractor and `parse_bounded`
    (`:1248-1273`).
  - Per-row labels go to `{out}/runs.json` (`:1307-1323`, `:1389-1393`).
  - The header states its discipline: "No title, symptom, hypothesis, justification or raw output is written anywhere"
    (`:22-27`). The audit hook changes exactly this.
  - `Shape` holds exactly ONE `cue: AttentionCue` (`:168-173`), and `prepare` always renders one cue and passes
    `STORM_FINGERPRINT` as the only citable id (`:419-448`). A cueless shape needs that made optional.
  - 29 `#[test]` fns (`grep -c '#\[test\]'` = 29).
  - `parse_args_from` (`:1013-1079`) is the pinned flag seam. `--sampling` is allowlisted (`:120-127`, `:985-1007`), and
    `gb` swaps the schema file for `--grammar-file` (`:463`, `:883-888`).
- `pulse-app/Cargo.toml:19-23` — `[[example]] name = "l4_decision_probe" … test = true`. Its pins are collected by the
  workspace nextest.
- `crates/triage/src/digest/assembler.rs`:
  - `:584-598` — `compose_services_from_q1` sets every `*_baseline` equal to the current observation, re-derived at
    HEAD `2dc099a`.
  - `:604-614` — the `TRIGGER_LINE_PREFIX` and `CORPUS_MATCHES_FRAMING_NOTE` constants.
  - `:618-623` — `cue_summary`.
  - `:629-715` — `render_payload`:
    - the WINDOW/mode line;
    - RECENT CHANGES, at most 5 commits as `{m}m ago | {basename} | {n} files`;
    - OVERALL: `nominal` without a cue, `anomalous` with one, `degraded` on the active-incident bypass;
    - TRIGGER, only with a cue;
    - SERVICES: rate, error% and p99 only. The header says "vs baselines" but no baseline is printed;
    - ATTENTION CUES;
    - CORPUS MATCHES with the framing note, at most `DIGEST_CORPUS_RETRIEVAL_LIMIT` = 5 (`crates/triage/src/digest/mod.rs:67`).
- `crates/triage/src/digest/assembler.rs:73-85` — `DigestProjectContext.recent_commits: Vec<DigestRecentCommit
  {basename, age_seconds, files_changed_count}>`. This is how A1 and B3 carry a recent change.
- `crates/triage/src/digest/retrieval.rs:136-148` — `format_corpus_match_line` renders `[{fingerprint}] {title} — {age}m
  ago, {status}`. A6's recurrence is several such lines carrying ONE fingerprint.
- `crates/triage/src/cadence/coordinator.rs`:
  - `:36-48` — the `Tier3` / `Reflection` modes with labels `tier3` / `reflection`;
  - `:237-240` — the window: Tier1/2/3 at 60 s, Reflection at 1800 s.
- `pulse-app/src/inference_runtime.rs`:
  - `:198-260` — `process_digest` has NO cue gate. A cueless `cadence_tier3` digest reaches L4, gated only by backoff
    and the unchanged-digest damper.
  - `:395-425` — the primary tier composes `build_primary_tier_prompt` for every non-reflection kind, including Tier3.
  - `:790-825` — **a cueless, non-reflection digest whose generation says `surface` creates NO incident**: it emits
    `interpretation.incident.skipped {skip_reason: no_cue}` and returns.
- `pulse-app/src/llamacli_inference.rs`:
  - `:79` `MAX_PROMPT_BYTES` 16384;
  - `:87` `LLAMA_CLI_TIMEOUT` 60 s;
  - `:93` `LLAMA_CLI_MAX_OUTPUT_BYTES` 64 KiB;
  - `:97` `DEFAULT_MAX_TOKENS` 1024;
  - `:104` and `:110` — `-c 8192` and `-rea off`.
- `crates/interpretation/src/schema.rs`:
  - `:149-152` — `Hypothesis {statement, confidence, justification}`;
  - `:170-184` — `L4Output` with `decision`, `severity`, `title`, `symptom` and `hypotheses`;
  - `:341` — `parse_bounded` runs `validate`, so "schema-valid" is `parse_bounded` Ok.
- `xtask/ci/l4-latency-p99.sh:7-12,35-39` — the product's per-profile L4 budgets: `gpu-primary 5000ms` (Tier-1 SLO),
  `gpu-fallback 3000ms`, `cpu-primary 30000ms` and `cpu-fallback 15000ms`.
- The predecessor's evidence:
  - `evidence/series.md` — the selection slot's GPU p50 per model: Llama 2598 ms, Qwen3.5-2B 3595 ms, Qwen3.5-4B
    5518 ms, gemma-4-E2B 4246 ms, gemma-4-E4B 6072 ms, and Nemotron 3755 ms. Peak VRAM: 3382, 2128, 3716, 2308 and
    3898 MiB. Peak RSS: 2.37, 1.68, 3.10, 3.49 and 5.32 GB (`:176-182`, `:209`). The two 4B-class models' p50 is above
    the 5000 ms gpu-primary budget.
  - `preregistration-addendum.md:73-96` — the per-model authors' sampling table and the `samp()` mapping.
  - `preregistration-addendum-4.md:26-31` — Nemotron's sampling and thinking-off.
  - `plan.md:130-451` — the operator leg firing form.
  - `report.md:167-168` — the CARRY-1 process correction.
- `AI-Model/` (gitignored, `.gitignore:119`), listed by basename only. It holds the six GGUFs
  (Llama-3.2-3B-Instruct, Qwen3.5-2B, Qwen3.5-4B, gemma-4-E2B-it, gemma-4-E4B-it, NVIDIA-Nemotron-3-Nano-4B, all
  Q4_K_M) and `llama-b9305-cuda`.
- `target/l4-decision-probe/l4-output.gbnf` (gitignored): sha256 `7b5cc276…ac03`, equal to the predecessor addendum's
  recorded value.

## Graph impact (rust plane, `tree-query-2026-10-05-l4-model-chosen-by-pattern-discrimination.json`, 67 rows)
- **render_payload** has five production callers:
  - `assemble` at `crates/triage/src/digest/assembler.rs:338,362,377,393`;
  - the `digest` module re-export at `crates/triage/src/digest/mod.rs:36`.

  Outside triage, only the probe calls it (`prepare` at `l4_decision_probe.rs:428`) along with the
  `unit_digest_runtime_scrub` pins. The chunk calls it unchanged, so the blast radius is zero.
- **build_primary_tier_prompt** has two production callers: `handle_digest_outcome` (`inference_runtime.rs:421`) and
  `run_action` (`investigate_router.rs:248`). The probe calls it at `l4_decision_probe.rs:448`. Unchanged.
- **build_llama_cli_args** has one production caller, `generate_constrained` (`llamacli_inference.rs:784`), and the
  probe's `compose_argv` (`:869`). Unchanged; its signature is untouched.
- **validate_prompt_bounded** has one production caller, `generate_constrained` (`:769`), and the probe's `prepare`
  (`:487`). Unchanged.
- **Reading:** the chunk is a pure consumer of four product functions. It changes no signature, and no caller threading
  is owed.

## Patterns detected
- **In-process synthetic digest** (`l4_decision_probe.rs:413-499`). The shape data feeds `render_payload`, then
  `build_primary_tier_prompt`, then `validate_prompt_bounded`, never OTLP. The obs extract's "no own-port dial" holds by
  construction.
- **Text transform on a rendered payload** (`truthful_overall` `:362-380`, `remove_lines` `:620-636`). This is the
  probe-side precedent for the enriched render: a pure function over the rendered text with an exactly-once anchor check.
- **Closed label sets read in-process from model JSON** (`names_trigger` `:653-676`, `names_trigger_stem` `:693-716`,
  pinned by "label set is exactly the closed set" tests `:1527`, `:1646`). The new `detect` / `cause` / `valid` labels
  follow the same form.
- **Per-row bounded record plus per-arm summary line** (`:1300-1386`). The new scorer adds per-family summary lines in
  the same `  arm {arm}: …` grammar.
- **Footprint per child pid** (`parse_vram_mib` `:818-824`, `read_vm_hwm_kib` `:826-835`). A foreign GPU process never
  enters a reading, which is what CARRY 2 relies on.
- **Basename-only printing** (`basename` `:1090-1094`, the binary line `:1181-1188`).

## Conventions to follow
- **Example module directory**: a non-`mod.rs` file `examples/l4_decision_probe.rs` resolves `mod patterns;` to
  `examples/l4_decision_probe/patterns.rs`. Cargo auto-discovers only `examples/*/main.rs`, so the directory adds no
  example target and the `[[example]]` declaration (`pulse-app/Cargo.toml:19-23`) is unchanged.
- **ASCII-only sources** (`cargo xtask check:english-sources`; predecessor plan gate `:238-242`).
- **Flags through `parse_args_from`** with a pin per branch (test-plan §1 `l4-decision-probe-arg-parse-unit-coverage`).
- **Real-model legs are `leg = 'operator'`, fired once, in daytime only.** They take the predecessor's firing form
  (`plan.md:288-336`):
  - a `build` entry first;
  - unset the three overrides, then source `l4-env.sh` (inputs#I3);
  - a per-model `ANDROMEDA_PULSE_MODEL_PATH` loop with `--arms gb --gbnf … --sampling "$(samp $m)" --footprint`;
  - a fresh UTC-stamped out dir under `target/l4-decision-probe/`.
- **The GPU-idle precondition** (`plan.md:280-286`), restated per CARRY 2 so that a foreign compute process is RECORDED
  rather than red.

## New files to create
- `pulse-app/examples/l4_decision_probe/patterns.rs` — the A/B/C pattern family:
  - the shapes and their pre-declared ground truth and cause-word sets;
  - the probe-side enriched render;
  - the detect / cause / valid scorer;
  - the seeded blind-audit draw and the re-grade over stored outputs;
  - their pins.
- `andromeda-pulse-0.3.0/chunks/2026-10-05-l4-model-chosen-by-pattern-discrimination/evidence/` — the evidence records:
  - the pre-registration, written before any real-model run;
  - the series record and the quality-vs-footprint table with the rule's recommendation;
  - the audit record and the mutation checks;
  - the operator-pass record.

## Files to modify
- `pulse-app/examples/l4_decision_probe.rs` — the `patterns` module wiring:
  - a cueless or optional cue on `Shape`, with the mode label per shape;
  - the new flags;
  - per-family summary lines;
  - audit-text storage under the out dir;
  - the header doc's discipline paragraph updated to name what is now written and where.

## Open questions
- What is the `valid` atom's time budget, and are invalid rows counted in the A/B/C denominators? → blocks:
  plan-decision. The design names no budget (inputs#I1 §3). The product holds two numbers:
  - the gpu-primary SLO of 5000 ms (`l4-latency-p99.sh:9`);
  - the runner's wall-clock cut of 60 s (`llamacli_inference.rs:87`).

  The predecessor measured the two 4B-class models' GPU p50 above 5000 ms, so the choice moves A/B quality for exactly
  the heavier models.
- What does "the lightest model" order by in the recommendation rule? → blocks: plan-decision. The CPU route is
  founder-retired, so the predecessor's order (CPU max `peak_rss_kib`) no longer applies. Peak VRAM and peak RSS order
  the six models differently: gemma-4-E2B has lower VRAM than Llama but higher RSS (series.md `:178-182`).
- Founder-facing fact, not a question: the table measures the model, and today's product acts on a cueless `surface`
  by recording a `no_cue` skip, never an incident (`inference_runtime.rs:822-824`). A model's A-detect therefore
  becomes user-visible only after a product change that this chunk does not make. It is surfaced in scope and in the P5
  review.
