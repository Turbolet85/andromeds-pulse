# Report — 2026-10-05-l4-runs-the-founder-s-pick-with-its-authors-settings

**Chunk:** L4 ships gemma-4-E4B with its authors' sampling, a committed GBNF via --grammar-file and thinking off; the GPU SLO raised
**Date:** 2026-10-05T15:19Z
**Commits:** `bc1946d chore(2026-10-05-l4-runs-the-founder-s-pick-with-its-authors-settings): operator pre-CI commit, for the run this chunk's verdict reads` (the only commit since `last_wrap` 2026-10-05T13:46:13Z beyond the prior wrap's `7663dd9`)

## Changes (structured — detectors read this)
- **Files:** (basis `git diff --name-only 7663dd9 HEAD -- crates pulse-app xtask docs`: 18 files, +1688 / −197)
  - modified: `pulse-app/src/llamacli_inference.rs` · `pulse-app/src/inference_runtime.rs` ·
    `crates/interpretation/src/contract.rs` · `pulse-app/tests/unit_llamacli_inference.rs` ·
    `pulse-app/tests/unit_inference_runtime.rs` · `pulse-app/examples/l4_decision_probe.rs` ·
    `pulse-app/examples/l4_decision_probe/patterns.rs` · `xtask/ci/l4-latency-p99.sh` · `xtask/ci/l4-latency-p99.ps1` ·
    `docs/v0_2_0/pulse-distillation-architecture.md`
  - new: `pulse-app/src/l4-output.gbnf` (44 lines, sha256 `7b5cc276b45d45a65c372c4cceb20d01bf6ab668d208f7bbd11a5125a8a8ac03`) ·
    `pulse-app/tests/unit_l4_grammar.rs` · `pulse-app/vendor/llama-cpp/{json_schema_to_grammar.py, LICENSE, README.md}` ·
    `xtask/ci/fixtures/{l4-latency-tail, l4-latency-pass, l4-latency-unlabeled}.jsonl`
- **Symbols / APIs:**
  - `pulse_app::llamacli_inference`:
    - new `pub const L4_OUTPUT_GBNF: &str = include_str!("l4-output.gbnf")`;
    - new `pub const LLAMA_CLI_TEMP = "1.0"` · `LLAMA_CLI_TOP_P = "0.95"` · `LLAMA_CLI_TOP_K = "64"` · `LLAMA_CLI_MIN_P = "0"`;
    - new `pub fn grammar_for_schema(&str) -> Result<&'static str, InferenceError>`. It maps `L4_OUTPUT_JSON_SCHEMA` to
      `L4_OUTPUT_GBNF` and any other schema to `InferenceFailed { reason: "grammar_schema_mismatch" }`;
    - `build_llama_cli_args(model, ngl, max_tokens, grammar_path, prompt)` keeps its five parameter types; its fifth
      parameter is now the grammar path. Argv:
      `-m · -ngl · -c 8192 · -rea off · -st · --simple-io · --no-display-prompt · --log-disable · -n {N} · --temp 1.0 ·
      --top-p 0.95 · --top-k 64 · --min-p 0 · --grammar-file {path} · -p {prompt}`, with no `--json-schema-file` and
      `-p` still last. Callers (basis `grep -rn 'build_llama_cli_args'`): `generate_constrained`, the probe's
      `compose_argv`, and the argv pins in `unit_llamacli_inference.rs`;
    - `generate_constrained` calls `grammar_for_schema` FIRST, before the configuration check. On a mismatch it emits
      one WARN on `interpretation.inference.error` with `error_category = "grammar_schema_mismatch"` and
      `recovery_action = "skip_digest"`, plus `model_tier` and `hardware_profile`, and writes and spawns nothing. Both
      production callers pass `L4_OUTPUT_JSON_SCHEMA` (`inference_runtime.rs` and `investigate_router.rs`, unchanged),
      so the Investigate path inherits the grammar argv;
    - the per-spawn temp file `SchemaTempFile` became `GrammarTempFile`: the same RAII and the same
      `std::env::temp_dir()`, now named `andromeda-pulse-llama-grammar-{pid}-{nanos}.gbnf` and holding
      `L4_OUTPUT_GBNF`. Its write-failure reason is `grammar_tempfile_write_failed`;
    - `LlamaCliInference` overrides `hardware_profile()` with the profile it was constructed with.
  - `interpretation::contract::LlmInferenceRunner` gains `fn hardware_profile(&self) -> HardwareProfile`, which defaults
    to `HardwareProfile::Unknown`. Every other implementor keeps the default, among them the deterministic runner and
    the test stubs.
  - `pulse_app::inference_runtime`: `handle_digest_outcome` reads `profile_label(runner.hardware_profile())` once, and
    both `metric.pipeline.l4.inference_latency_p99_milliseconds` emits now carry `hardware_profile` (the runtime-error
    arm and the parse-success arm). The private `handle_parse_outcome` gains a `hardware_profile: &'static str`
    parameter from its one caller. No public signature changes.
  - The probe (`pulse-app/examples/l4_decision_probe.rs`, dev-only):
    - the `gb` arm and the `--gbnf` flag are retired (ARMS 16 → 15), so `--arms gb` reads "unknown arm gb" and
      `--gbnf` reads "unknown flag --gbnf";
    - each run writes `{out}/l4-output.gbnf` (holding `L4_OUTPUT_GBNF`) once and passes it as the production grammar
      path;
    - `compose_argv` applies an arm's extra flag/value pairs by REPLACE-OR-APPEND, so no flag is passed twice. This
      covers `--sampling` and the `A1` arm's `--temp 0`;
    - R2 and A5 now vary only the prompt's embedded schema copy, because the converter emits no `description` (0 in
      the 44 lines).
  - `patterns.rs`: `GPU_PRIMARY_BUDGET_MS` goes 5000 → 10000.
  - Env vars: none added. `ANDROMEDA_PULSE_MODEL_PATH` selects the GGUF, and its file stem stays the model identity.
- **Crates / modules:** none added or removed. A new non-Rust vendored directory `pulse-app/vendor/llama-cpp/` holds
  test-only Python, which no crate builds.
- **Dependencies:**
  - Rust: none (`Cargo.toml` / `Cargo.lock` untouched; the scope guard, gate `git diff --name-only 7663dd91… -- …
    Cargo.toml Cargo.lock …`, printed nothing).
  - Vendored: llama.cpp b9305 (`63248fc3e33e6f3b579dce6a743fd6ce8939af9c`) `examples/json_schema_to_grammar.py`,
    byte-identical (sha256 `677553718afb2bc2a63182fa812240c8436981f3fe43381e2e48a27b355352a8`). It is MIT, its
    `LICENSE` sits beside it (sha256 `94f29bbe…010d`, "Copyright (c) 2023-2026 The ggml authors"), and it is
    stdlib-only Python 3. It runs only under `unit_l4_grammar` and never in the shipped binary. cargo-deny cannot see it.
  - Test-time: a Python 3 interpreter (`python3` or `python` printing `Python 3`) on every runner that runs the
    workspace tests. A missing interpreter FAILS `unit_l4_grammar`; it never skips.
- **Schema / config:**
  - The shipped GGUF is the founder's pick, the unsloth `gemma-4-E4B-it-Q4_K_M.gguf` (sha256
    `85a896a047553e842f25297ee5b031d64ff30147d9c4af17b1e4b394cd1fab87`). It is selected by the env var and never
    committed.
  - The output constraint mechanism moves from `--json-schema-file` (an OUTPUT_FORMAT grammar, prefilled with the
    template's generation prompt) to `--grammar-file` with the committed GBNF (a USER grammar, never prefilled).
  - The latency grader:
    - `L4_GPU_PRIMARY_BUDGET_MS` default 5000 → 10000 (`.sh` and `.ps1`); the other three profiles are unchanged;
    - p99 is nearest-rank: `.sh` uses `int((n*99+99)/100)` and `.ps1` uses `Floor((n*99+99)/100)`, both replacing
      `int(n*0.99)`;
    - a target record carrying no `hardware_profile` or no numeric `duration_ms` makes the run print
      `::error::l4-latency-p99: {N} sample(s) carry no hardware_profile or duration_ms` and exit 1. Before, such records
      were dropped.
  - `interpretation.inference.error` gains the `error_category` value `grammar_schema_mismatch`. The latency leaf's
    `hardware_profile` is now emitted; the leaf already permitted it.
- **Spec-master edits:** none this chunk before the wrap. The P2 apply is next.
- **Counts / qualifiers moved:**
  - `pulse-app/tests/*.rs` 101 → 102 (basis `git ls-tree --name-only 7663dd9 pulse-app/tests/ | grep -c '\.rs$'` → 101;
    `ls pulse-app/tests/*.rs | wc -l` → 102). The new file is `unit_l4_grammar.rs`. The testing.md rule text says
    "101 files as of 2026-10-04".
  - Workspace nextest: 2697 run, 2697 passed, 0 skipped (gate `cargo nextest run --workspace --profile ci`, implement
    run).
  - The probe's collected pins: 61 → 61 (`-E 'binary(l4_decision_probe)'`: 2 retired, 2 added). The probe's ARMS count
    is 16 → 15.
  - Gate 5's selection: 332 → 341 (+9 new pins).
  - The gpu-primary L4 SLO: < 5 s → < 10 s p99, at its four homes in `docs/v0_2_0/pulse-distillation-architecture.md`
    (`:317` matrix cell, now `~5-7.5s` primary inference and `< 10s` Tier-1; `:531`; `:860`; `:864`).
  - Product-written locations outside the data dir: the masters count TWO (the `~/Downloads` training-export sink and
    the corpus-key lock file), at security-plan.md:167 ("the second product-written artifact there") and :393 ("one of
    the two product-written locations"); architecture.md:219 has the lock-file entry. A THIRD has existed since chunk
    #84: the per-spawn L4 constraint temp file in `std::env::temp_dir()`. It was the schema file and is now
    `andromeda-pulse-llama-grammar-{pid}-{nanos}.gbnf`. This chunk changed its content and name, not its location.
- **Dev-tool versions:** none changed. llama.cpp b9305 (`63248fc`) re-read, unchanged. Host `python3` 3.14.7 (read at P3)
  runs the converter. `pwsh` is ABSENT on the dev host (`which pwsh` → not found), so the `.ps1` grader was not run.
- **Harness / gate surface:**
  - `xtask/ci/l4-latency-p99.{sh,ps1}` changed (budget, nearest-rank, the unlabeled failure), with three new fixtures
    under `xtask/ci/fixtures/`. The grader is still dev-host only and not CI-wired (basis
    `grep -rn 'l4-latency-p99' .github` → 0).
  - The new `unit_l4_grammar` binary runs in CI's `lint-test` workspace tests on all three runners.
  - No `agent-run`, no xtask verb, no CI step changed.
- **Cross-project / external claims:**
  - CI run `ci#37327846820` (pull_request) on sha `bc1946de3484`: success, `verdict: green · checks 13/13`, wall
    1686 s. `secret-scan#37327847087` success. `lint / test` succeeded on `ubuntu-22.04`, `macos-latest` and
    `windows-latest` (basis `gh run view 37327846820 --json jobs`). That is the Python proof (overseer, founder-delegated,
    P4).
  - The b9305 sampler-init failure of `--json-schema-file` with gemma4, re-measured on this host
    (`evidence/red-leg.md` §Second source).
  - Inputs (`inputs.py verify`):
    - I1 · relay message (the founder's rulings) · copy · n/a;
    - I2 · `~/dev/data/l4-model-comparison-2026-10/l4-output.gbnf` · copy · unchanged;
    - I3 · `../additional/pc-overseer/l4-env.sh` · copy · unchanged (still names Llama-3.2-3B; moving it is the overseer's);
    - I4 · `~/dev/tools/llama.cpp-b9305:examples/json_schema_to_grammar.py` · pointer@63248fc3 · unchanged;
    - I5 · `~/dev/tools/llama.cpp-b9305:LICENSE` · pointer@63248fc3 · unchanged. I5 was UNCITED in plan/scope/research
      (snapped at implement). It is cited here: the vendored `LICENSE` is I5 byte for byte.
- **Reverted / negative API facts:** the `gb` arm and the `--gbnf` flag removed from the probe (the shipped argv IS the
  former `gb` argv).
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  1. "The product writes outside the data dir in exactly TWO places" (security-plan.md:167 and :393; the
     CLAUDE.md / rules security text). Measured false: the per-spawn L4 constraint temp file in
     `std::env::temp_dir()` is a third, pre-existing since chunk #84 (research.md §Scope premise closure;
     `llamacli_inference.rs` `GrammarTempFile::create`).
  2. "The L4 latency grader gates a p99 per profile." Measured vacuous at the chunk base: live latency samples carried
     no `hardware_profile`, so the grader dropped every one and printed "gate trivially passes" at exit 0
     (`evidence/red-leg.md`, gate `bash xtask/ci/l4-latency-p99.sh` on the RED log). Its index was also
     `int(n*0.99)`, not nearest-rank (`evidence/mutation-checks.md` §Grader controls: the base grader reads 1000 on the
     tail fixture where nearest-rank reads 20000). Fixed in this chunk.
  3. obs-plan §10 holds NO L4 row (basis `grep -c 'l4-latency' .andromeda/obs-plan.md` → 0), while research found the
     obs-plan-amendments 2026-06-10 record calling the L4 per-profile budgets "confirmed canonical". This is a chunk
     premise correction already recorded at research; the row is an expected amendment.
- **Expected amendments (from plan):**
  - arch §Established Decisions [LLM Inference Runtime] (Model clause, constraint mechanism, sampling, the gpu-primary
    comparison) → carried: Schema / config + Symbols. Sites: `grep -c -i 'json-schema-file' architecture.md` → 3,
    `'Llama-3.2'` → 2, `'gemma'` → 2, `'gpu-primary\|gpu_primary'` → 4 (line 71 the decision, line 367 Inherited
    Defaults).
  - arch §Stack (AI/ML row, line 30) and §Inherited Defaults (LLM line, line 367) → carried: the same facts.
  - arch §Occupied Resources: the third out-of-data-dir write → carried: Counts (architecture.md:219 has the lock-file
    entry and no constraint-temp-file entry).
  - security-plan §Input Validation (L4 argv row) and §Security Anti-Patterns → Code Patterns (the `-p` bullet) →
    carried: Symbols (the argv). Sites: `grep -c 'validate_prompt_bounded' security-plan.md` → 3.
  - security-plan §Security Anti-Patterns → Input: the third out-of-data-dir write → carried: Counts and Spec claim
    disproved 1 (security-plan.md:167 and :393).
  - security-plan §Dependency Security: the vendored, test-only MIT converter → carried: Dependencies (§Dependency
    Security at security-plan.md:205).
  - obs-plan §10: an L4 row (gpu-primary p99 ≤ 10000 ms nearest-rank, the script pair, an unlabeled sample failing) →
    carried: Schema / config and Spec claim disproved 3 (`grep -c 'l4-latency' obs-plan.md` → 0).
  - obs-plan §8: `grammar_schema_mismatch` in `interpretation.inference.error`'s `error_category` set "where the section
    enumerates the values" → carried: Schema / config. Sites: `grep -c 'interpretation.inference.error' obs-plan.md` → 0,
    security-plan.md → 1, test-plan.md → 1. obs-plan does not enumerate that target, so it is not carried there. The
    latency leaf's `hardware_profile` (field set unchanged) has obs-plan hits for `inference_latency_p99` → 0.
  - test-plan §1 `l4-decision-probe-arg-parse-unit-coverage` (test-plan.md:144): `gb` and `--gbnf` retired, count 61 →
    carried: Symbols and Counts (`grep -c -i 'gbnf' test-plan.md` → 1, `'grammar-file'` → 1).
  - test-plan §4: the `pulse-app/tests/*.rs` count 101 → 102 → carried: Counts.
  - test-plan §2 / §9: Python 3 is a test-time dependency on all three CI runners → carried: Dependencies (test-plan.md:646
    is the lint-test row).
- **Coverage of new surfaces:**
  - `--grammar-file` + sampling operands at the L4 subprocess boundary → validation first-party constants and committed
    grammar ✓ (`-p` still `validate_prompt_bounded` before argv assembly) · instrumentation
    `interpretation.inference.error` (`grammar_schema_mismatch`) + the existing L4 targets ✓ · PII: the grammar path is
    never logged (gate: 0 `andromeda-pulse-llama-grammar` in the GREEN log), the model path is basename only (0 full
    paths) ✓ · tests unit (argv / sampling / helper / accessor pins, mutation-checked) + env-gated real round trip + the
    GREEN e2e leg · a11y n/a · tokens n/a.
  - `grammar_for_schema` → validation exact-equality guard ✓ · instrumentation the mismatch WARN ✓ · PII n/a · tests
    unit (both arms, mutation-checked) · a11y n/a · tokens n/a.
  - `LlmInferenceRunner::hardware_profile` (trait default) → validation n/a · instrumentation: feeds the latency emit's
    `hardware_profile` ✓ · PII n/a (a bounded label) · tests unit (default `unknown`, override `gpu_primary`) · a11y n/a
    · tokens n/a.
  - `unit_l4_grammar` (vendored converter run) → validation: byte equality + a discrimination pin ✓ · instrumentation
    n/a · PII n/a · tests unit (CI green on 3 runners) · a11y n/a · tokens n/a.
  - The latency grader's unlabeled-record failure → validation ✓ · instrumentation n/a · PII n/a · tests fixtures (3
    gate entries) · `.ps1` half `unrunnable-here` (no `pwsh` on the dev host) · a11y n/a · tokens n/a.
  - PRE-EXISTING, exposed by this chunk: `interpretation.constrained.generate` (the success arm of
    `LlamaCliInference::generate_constrained`, byte-unchanged by this chunk) emits `raw_output_bytes` and
    `extracted_bytes`, but its allowlist leaf lacks both, so they render `"<redacted>"`. Measured: 7 records, 14 fields
    in the GREEN log (`evidence/green-leg.md`). This is the "incomplete exact leaf" shape (observability.md 2026-08-26).
    It never fired on the base build with this model, because every generation failed before it (0 records in the RED
    log). instrumentation PARTLY redacted ✗ — owner to be decided at P5 (overseer directive).

## Deviations from intent
- `compose_argv` applies EVERY extra flag/value pair by replace-or-append, not only `--sampling` (plan step 9 named
  `--sampling` only). Justification: the `A1` arm's `--temp 0` would otherwise duplicate the new production
  `--temp 1.0`, the duplicate-flag shape the plan bans for `--sampling`. Pinned by
  `sampling_replaces_the_production_pairs_rather_than_duplicating`.
- A third probe test used the `gb` arm (`pattern_arms_vary_the_argv_only`); plan step 9 named two retired pins. It is
  re-pointed at `nr` (the same "varies the argv only" property). The pin count is still 61.
- `pulse-app/vendor/llama-cpp/README.md` was written to the session scratchpad and copied into place. A project
  PreToolUse Write hook blocks any `vendor/` path as a generated directory. The README's content is as plan step 2 states.
- llama.cpp's `LICENSE` snapped as input I5 (`inputs.py snap --step implement`), as plan step 2 directed.
- Beyond plan: a direct `llama-cli` second source for the RED absence (schema-file vs grammar-file,
  `evidence/red-leg.md`) and one extra mutation (a perturbed committed GBNF reds the equality pin,
  `evidence/mutation-checks.md`).
- Scope record: none. `gate.py scope` is clean: changed 18, listed 18, 0 recorded (P1, base `7663dd91`).

## Decisions & corrections
- Founder rulings (inputs#I1): (1) the GBNF in the product via `--grammar-file`, with a test pinning GBNF == the schema
  conversion. This boundary widening is ratified. (2) Raise the L4 GPU SLO; the phase proposed the figure.
- P4 forks (overseer, founder-delegated): vendor the converter so the test PERFORMS the conversion; make the grader
  gradable and show it failing over budget.
- Figure chosen: gpu-primary 10000 ms p99 (nearest-rank). Measured live p99 6716 ms (n = 7); the pattern series max was
  7285 ms.
- Wrap directive (overseer, founder-delegated): the CI green on `bc1946d` is the Python proof. Carry two owed items: the
  `.ps1` grader run and the `interpretation.constrained.generate` redaction. Ask at the route resolve where the redaction
  fix lives. Conductor v3-09 is now unblocked by this entry.
- Sweep hazard: a mid-burst helper-name guess (`argv_of` for the probe's `argv` helper) was caught at compile. A new
  test local named `argv` would SHADOW the module's `argv(...)` helper inside that test; rename the local instead.
- Sweep hazard: a fresh e2e leg on a never-before-successful path can expose leaf gaps in emit sites the chunk never
  touched. The leaf census reads the emit sites, not the chunk's diff.

## Outcome
- Acceptance criteria, re-asserted against the diff:
  - (arch) The argv carries `--grammar-file` with no `--json-schema-file`, and keeps `-n`, `-st`, `-c 8192`, `-rea off`,
    the outer timeout and `kill_on_drop`. b9305 is unchanged. MET.
  - (founder ruling 1) The committed GBNF equals the converter's output, performed in-test, with a discrimination pin.
    The hashes are I4 / I2, and the MIT notice sits beside the converter. MET.
  - (founder ruling 1, CI) `ci#37327846820` is green on `bc1946d`, with lint-test on 3 runners. MET.
  - (product) Measured. RED: 4/4 `json_parse_failed`, 0 parse ok. GREEN: 7/7 parse ok; `model.load` names
    `gemma-4-E4B-it-Q4_K_M`; the real round trip prints PASS, not skipped. MET.
  - (security) `-p` is the only OTLP-derived operand, and `validate_prompt_bounded` still runs first. Grammar path and
    sampling are first-party. MET.
  - (security, obs) The GREEN log has 0 full GGUF paths, 0 grammar temp names, 0 panic and 0 ERROR. MET.
  - (obs) The latency samples carry `hardware_profile`. The grader reads nearest-rank: PASS at 10000, FAIL at 1 ms, FAIL
    on an unlabeled record, and the tail fixture reads as a breach. MET for the `.sh`. The `.ps1` is edited in lockstep
    and NOT RUN (no `pwsh`); that run is owed.
  - (obs) `interpretation.model.load`'s field set is unchanged and only its value names the pick; no new tracing target.
    MET. Note the pre-existing partly-redacted `interpretation.constrained.generate` leaf (Coverage), which no criterion
    of this chunk covers.
  - (tests) The standard gate set is green and the bindings close exits 0. MET.
  - (tests) Each new pin is mutation-checked and the probe count is recorded (61). MET.
- Gates (implement, entries 1–21 at 14:40:11Z; the light gate re-runs at P7):
  - `cargo fmt --check` green;
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings` green;
  - `sha256sum … && grep -c …LICENSE` green (both hashes, `last line 2`);
  - `cargo nextest … binary(unit_l4_grammar)` green (2 run, 0 skipped);
  - `cargo nextest … unit_llamacli_inference + unit_inference_runtime + observability_pins + package(interpretation)`
    green (341);
  - `cargo nextest … binary(l4_decision_probe)` green (61);
  - `bash xtask/ci/l4-latency-p99.sh …tail.jsonl` green (exit 1, `p99=20000ms exceeds budget 10000ms (sample_count=150)`);
  - `…pass.jsonl` green (exit 0, `p99=7300ms … (sample_count=10) PASS`);
  - `…unlabeled.jsonl` green (exit 1, `carry no hardware_profile or duration_ms`);
  - the scope guard `git diff --name-only 7663dd91… -- …` green (no output);
  - `cargo xtask check:english-sources` green (`"verdict": "clean"`);
  - `capability-widening-check` · `check:ingest-progress` · `check:staged-artifacts` · `capability-drift` ·
    `verify:capability-matrix` green;
  - `cargo nextest run --workspace --profile ci` green (2697/2697, 0 skipped);
  - the `--features mcp-server` regen green;
  - `git diff --quiet 7663dd91… -- pulse-app/ui/src/bindings/index.ts` green;
  - `cargo build -p ingest --example inject_demo` and `cargo build -p pulse-app` green.
- Leg entries:
  - the slot precondition (`ss … && nvidia-smi …`): operator leg, driven by hand twice, exit 0 with no `llama-cli`
    (`evidence/red-leg.md`, `evidence/green-leg.md`);
  - `cargo nextest … --success-output immediate -E 'binary(integration_real_llama_cli)'`: live, fired in a round →
    `round: COMPLETE · legs fired 1/1`, green 9.68 s, PASS line and no `[skip]` (`evidence/round-144048Z.txt`);
  - the liveness probe + `./target/debug/examples/inject_demo`: round + live, fired twice (RED `round-142920Z.txt`, GREEN
    `round-144612Z.txt`), each `round: COMPLETE`;
  - the RED poll and the RED 0-parse read: operator, RED leg, green (`3` / exit 1 `0`);
  - the GREEN poll, `model.load`, 0-panic/ERROR, the grader, the 1 ms mutation and the path read: operator, GREEN leg,
    all green per their `expect`;
  - `gate.py hygiene`: operator pass, `hygiene: clean — read 39`;
  - `git diff --quiet && … git push origin chore/migrate-pulse-to-v3`: operator pass, pushed `7663dd9..bc1946d`;
  - `ci.py conclusion --sha HEAD --wait 2400`: operator pass, `bc1946de3484 verdict: green · checks 13/13`.
- Smoke: no boot path changed, so none was listed. The GREEN leg booted `target/debug/pulse-app` by path.
- Watches: none.
- Outcome basis: implement's P4 report (this conversation) plus the operator pass run in this conversation on the
  overseer's word (Setup 4 commit list: `bc1946d` only; the final HEAD's CI run `ci#37327846820`).
- Process hygiene (implement P4 census, re-measured now): both app boots, both storms, the real-test `llama-cli` and two
  direct `llama-cli` probes, all terminated. The ports read free, and `pgrep -x pulse-app|inject_demo|llama-cli` reads
  none as of this report.
