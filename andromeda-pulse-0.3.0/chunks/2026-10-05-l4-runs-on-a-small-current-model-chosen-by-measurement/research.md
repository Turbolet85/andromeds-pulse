# Codebase Research — 2026-10-05-l4-runs-on-a-small-current-model-chosen-by-measurement

## Scope
- **Depth:** deep on the L4 runtime boundary, the probe and the pinned llama.cpp tree; shallow elsewhere · **Reads:** 14 · **Globs/Greps:** 19
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full as a structural extraction (60,568 B: lines 1–112 and 113–157), all 22 Session Additions. The applicable ones: build a producer outside the timed section and run it by path (2026-08-23); a scenario leg is not a gate and proves its own preconditions, INCONCLUSIVE never PASS (2026-08-28); a harness's negative finding needs a second source (2026-08-29); a leg's window must outlast the threshold it reads (2026-08-28). No addition names the L4 probe itself.
- **Platform issues consulted:** none. There is no runner-only bullet: Setup 5a's run for `4e5b595` was in progress, not red.
- **External inputs:**
  - `inputs#I1` — the overseer's phase directive: the model slot and :4317/:4318 are granted, the b9305 arch-name grep is the overseer's own measurement, each load must be proven, the criteria order is footprint, then no thinking, then quality, and heavy runs are daytime only.
  - `inputs#I2` — b9305 `src/llama-arch.cpp` names `qwen35` (:42) and `gemma4` (:59). These are table entries, not loads.
  - `inputs#I3` — the Hugging Face API read: the four unsloth repos exist, each card reads `license: apache-2.0`, plus each Q4_K_M file name, size and LFS sha256 and each repo sha.
  - `inputs#I4` — the overseer's `l4-env.sh`, which the prior series sourced. `ANDROMEDA_PULSE_MODEL_PATH` is pinned to the Llama-3.2-3B GGUF, and both bin vars point at the b9305 CUDA build (the CPU route is that build at `-ngl 0`).

## Files inspected
- `pulse-app/src/llamacli_inference.rs` (1–895) — the whole runtime boundary.
  - The env consts are at `:45-64`.
  - The bounds: `MAX_PROMPT_BYTES` 16 KiB (`:76`), `LLAMA_CLI_TIMEOUT` 60 s for every tier (`:84`), `LLAMA_CLI_MAX_OUTPUT_BYTES` 64 KiB (`:90`) and `DEFAULT_MAX_TOKENS` 1024 (`:94`).
  - `LlamaCliInference::new` (`:136-157`) resolves both paths through `resolve_guarded_path` (`:595`).
  - `load_from_env_if_configured` (`:229-269`) derives `ModelIdentity.semantic_name` from the GGUF **file stem** (`:254-257`).
  - The argv is `build_llama_cli_args` (`:405-435`): `-m -ngl -st --simple-io --no-display-prompt --log-disable -n --json-schema-file -p`. It carries no `-c`, no `--reasoning` and no `--jinja`.
  - `generate_constrained` (`:730-894`) extracts the FIRST top-level JSON object from stdout (`extract_json_object_bounded`, `:326`).
- `pulse-app/examples/l4_decision_probe.rs` (1–130, 930–1080).
  - It reuses the product argv, timeout, output cap and guard (`:81-85`) and takes the model only from `ANDROMEDA_PULSE_MODEL_PATH`.
  - Each per-generation row (`:1027-1039`) carries `arm`, `shape`, `run`, `decision`, `severity`, `is_resolution_summary`, `would_create`, `first_keys`, `output_hash`, `names_trigger` and `names_trigger_stem`. It carries no latency, memory or thinking reading.
  - The only timing is per-arm wall seconds (`:943`, `:1071`).
  - A load failure surfaces as `Spawned::Failed(..)`. Only `spawn_failed` exits INCONCLUSIVE (`:963-965`); any other failure counts as an unparsed row.
- `~/dev/tools/llama.cpp-b9305/tools/cli/cli.cpp` (80–225) — b9305 `llama-cli` drives the server engine.
  - Reasoning content is printed to STDOUT between `[Start thinking]` and `[End thinking]` (`:158-171`).
  - The chat template is applied with `reasoning_format = DEEPSEEK` and `enable_thinking = chat_params.enable_thinking && template supports it` (`:217-219`).
  - `inputs.json_schema` and `inputs.grammar` are empty (`:212-213`). The grammar reaches sampling only through `defaults.sampling` (`:88`), which is where `--json-schema-file` lands.
- `~/dev/tools/llama.cpp-b9305/common/arg.cpp` (1278–1286, 2029, 2294, 2425–2445, 2511, 3014–3200).
  - `-rea/--reasoning [on|off|auto]`: `off` sets `enable_thinking=false` in the template kwargs, and the default is `auto`, detected from the template (`:3159-3175`).
  - `--reasoning-budget N`, where 0 means end immediately (`:3177`); `--chat-template-kwargs` (`:3014`, its `enable_thinking` key deprecated); `--jinja` (`:3140`).
  - `-c/--ctx-size`, 0 = loaded from the model (`:1278`); `-fit [on|off]` (`:2431`); `-nkvo` (`:2029`); `-dev` (`:2294`); `--no-op-offload` (`:2511`).
- `~/dev/tools/llama.cpp-b9305/common/common.h` — the defaults are `n_ctx = 0`, meaning the model's training context (`:427`), `fit_params = true` (`:451`), `fit_params_min_ctx = 4096` (`:453`), a fit target of 1 GiB free per device (`:456`), `no_kv_offload = false` (`:553`) and `no_op_offload = false` (`:556`).
- `andromeda-pulse-0.3.0/chunks/2026-10-04-l4-rank-1-hypothesis-names-the-retry-on-every-storm-shape/evidence/series.md` (grep) — the baseline standing, spot-checked at HEAD (see the closure below).
- `.../plan.md` of that chunk (302–310) — the firing form of the prior real-model legs. It unsets `ANDROMEDA_PULSE_{HARDWARE_PROFILE,L4_ALLOW_ROOT,L4_DETERMINISTIC}`, sources `inputs#I4`, builds the example, and runs `./target/debug/examples/l4_decision_probe --arms shipped --shapes S1,S2,S3,S4 --n 10 --min-rank1 36 --out target/l4-decision-probe/…`. Its `expect` atoms come from the probe's own print sites (`:681`, `:827`, `:864`).
- `xtask/ci/l4-latency-p99.sh` (1–120) — per-profile budgets of 5000 ms (gpu_primary), 3000 (gpu_fallback), 30000 (cpu_primary) and 15000 (cpu_fallback). They grade `metric.pipeline.l4.inference_latency_p99_milliseconds` (emitted at `pulse-app/src/inference_runtime.rs:63-64`, timed at `:431-457`). The gate is not in the standard set.
- `crates/triage/src/digest/assembler.rs` (22–56, 150) — the build-time Llama-3 `tokenizer.json` (`crates/triage/build.rs:39-41`) sizes the DIGEST against its 3000-token cap. It sizes neither the model's context nor the argv.
- `crates/interpretation/src/contract.rs` (66–67, 207–220) — `ModelIdentity { semantic_name }`. The `"llama-3.2-3b-instruct-q4_k_m"` literal at `:212` is a serialization-shape test fixture, not a product default.
- `.claude/docs/services/interpretation.md` (1–40) and `.claude/docs/stack.md:32` — tier routing and the binary-distribution flag. No doc in `.claude/docs` names the Llama model; the only planning doc that does is `docs/v0_2_0/pulse-vision-and-backlog.md` (historical).

## Graph impact (rust plane; trace `../../../.andromeda/runs/2026-10-05T06-18-42Z-phase/tree-query-2026-10-05-l4-runs-on-a-small-current-model-chosen-by-measurement.json`)
- **`build_llama_cli_args`** has 3 non-test callers: `generate_constrained` @ `pulse-app/src/llamacli_inference.rs:759`, `generate()` @ `pulse-app/examples/l4_decision_probe.rs:747`, and the import @ `:83`. It also has 8 pin call sites in `pulse-app/tests/unit_llamacli_inference.rs` (`:49, :58, :66, :81, :102, :109, :122, :417`; query 1, `rows: 12`, editor lines = SCIP line + 1). Adding argv CONSTANTS leaves the signature unchanged, so no caller threads. The probe inherits any argv change by construction.
- **`LLAMA_CLI_TIMEOUT` / `DEFAULT_MAX_TOKENS`** are referenced at `llamacli_inference.rs:762, :784`, `l4_decision_probe.rs:82, :750, :765` and six pins in `unit_llamacli_inference.rs` (refs query, `rows: 12`). A timeout change reaches the probe automatically.

## Patterns detected
- **Model identity is data, not code** (`llamacli_inference.rs:254-257`). The identity is the GGUF file stem, so swapping `ANDROMEDA_PULSE_MODEL_PATH` changes `interpretation.model.load`'s `model_identity` value with no code change and no field-set change. There is no compiled-in default model path. With the var unset, the runner boots degraded (`ModelNotConfigured`, `:233-235`).
- **Thinking would break extraction, not just cost tokens.** A thinking pass prints `[Start thinking]…` on stdout BEFORE the content (`cli.cpp:160-170`), and the extractor takes the first `{` it finds (`llamacli_inference.rs:328`). Reasoning text that contains a brace would be cut as the "JSON". `-rea off` is b9305's documented lever, and it reaches only templates that support `enable_thinking` (`cli.cpp:219`). Llama 3.2's template does not, so the flag is a no-op for the baseline. That is a measurable equality the plan must verify per candidate, not assume.
- **Footprint is set by the context, not only the weights.** No `-c` is passed, so b9305 runs the model's TRAINING context (`common.h:427`), shrunk only to fit free device memory less 1 GiB (`:451-456`). A per-model RAM/VRAM reading under the shipped argv therefore measures training-context KV cache × fit, so models with different training contexts are not compared on weights. The ruling's footprint criterion is unmeasurable as a model property until the context is held fixed.
- **The "CPU route" on this host is not CPU-only.** It is the CUDA build at `-ngl 0` (`inputs#I4`), with `no_kv_offload = false` and `no_op_offload = false` (`common.h:553,556`). The KV cache and large-batch ops still go to the GPU, so a CPU-only latency read must hide the device (`CUDA_VISIBLE_DEVICES=` empty, or `-dev none`), or it measures a GPU-assisted CPU route.
- **One timeout for every tier** (`llamacli_inference.rs:84`, 60 s). A CPU-only generation that exceeds it surfaces as `timeout` (`:802-813`) and counts as a failed row in the probe, so CPU-only latency doubles as a hard viability bound.

## Conventions to follow
- **Probe additions are pinned through `parse_args_from`** with mutation-checked pins, collected because the example declares `test = true` (test-plan §1 `l4-decision-probe-arg-parse-unit-coverage`; `pulse-app/Cargo.toml` `[[example]]`).
- **Real-model readings are evidence, never assertions.** The probe prints bounded labels only: no raw output, no paths beyond basenames (`l4_decision_probe.rs:10-28`).
- **Argv additions are first-party constants** inside `build_llama_cli_args`, each with a contains-pin in `pulse-app/tests/unit_llamacli_inference.rs`, in the shape of `spawn_args_contain_single_turn_flag` (`:57`) and `spawn_args_contain_log_disable_flag` (`:416`).
- **The real-model firing form** of the prior chunk's plan (302–310): unset the three overrides, source the env (`inputs#I4`), build the example outside the run, run by path, `--out target/l4-decision-probe/{slot}-{UTC}`, `leg = 'operator'`, the atoms from the probe's print sites.

## New files to create
- `andromeda-pulse-0.3.0/chunks/2026-10-05-l4-runs-on-a-small-current-model-chosen-by-measurement/evidence/` — the pre-registration (written before the first run), the load and no-thinking legs, the footprint table and the quality series records

## Files to modify
- `pulse-app/src/llamacli_inference.rs` — argv constants decided at P4 (a fixed context size, the reasoning switch)
- `pulse-app/tests/unit_llamacli_inference.rs` — contains-pins for each new argv constant
- `pulse-app/examples/l4_decision_probe.rs` — per-generation readings the series needs (thinking marker present, per-generation latency, peak memory as decided at P4), each pinned through its seam
- `.claude/docs/services/interpretation.md` — the shipped L4 model and the argv it passes

## Open questions
- Does the shipped argv hold the context fixed (a `-c` constant) so that footprint compares the models, and if so at what size? Prompts measure ≤ 7,405 B (≈ 2k tokens) plus a 1024-token generation cap. → blocks: plan-decision
- Does the selection rule rank the candidates lexicographically in the ruling's order (footprint bounds as a gate, no-thinking as a gate, then strict rank1)? What ships when no candidate clears the baseline's measured standing? → blocks: plan-decision
- How is CPU-only latency taken: GPU hidden (`CUDA_VISIBLE_DEVICES=`), or the shipped CPU route as configured, and does the 60 s timeout stand for that route? → blocks: plan-decision
