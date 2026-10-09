# Codebase Research — 2026-10-04-l4-framing-measured-on-the-real-model

## Scope
- **Depth:** moderate · **Reads:** 9 · **Globs/Greps:** 9 (+2 code-graph queries)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read §5-command discipline through §Anti-patterns
  in full (lines 1-110) and the Session-Additions index (lines 111-156) with the two entries that bear on a real-model
  measurement read whole: 2026-08-28 (a leg reproducing an open question is a SCENARIO, never a gate; it proves its own
  preconditions) and 2026-08-29 (an absence a harness observed needs a second source). The probe is not an `agent-run`
  verb or an xtask leg, so no firing-form recipe there names it; its invocation is the pre-registered argv below.
  `.claude/rules/testing.md` (auto-loaded) — 2026-06-05 [ext. 2026-10-04] (`grep | wc -l` under pipefail), 2026-08-16
  (prebuild outside a timed run), 2026-08-29 (`[[example]]` needs `test = true`), 2026-09-30 [ext. 2026-10-04]
  (evidence hygiene: a transcript naming a temp path or the repo root trips it), 2026-10-04 (diff probes name the base
  sha; scope guard lists new files).
- **Platform issues consulted:** none — no runner-only bullet (the CI read at take-up was in progress, not red) and no
  CI-reading entry outside the operator leg.
- **External inputs:** `inputs#I1` — the overseer's phase directive (the 4317/4318 slot granted, conductor-builder idle;
  binary `version: 9305 (63248fc)`); `inputs#I2` — the leg env file (exports `ANDROMEDA_PULSE_MODEL_PATH`,
  `_LLAMA_CUDA_BIN_PATH`, `_LLAMA_CPU_BIN_PATH`, `ANDROMEDA_LLAMA3_TOKENIZER_PATH`; sets no
  `ANDROMEDA_PULSE_HARDWARE_PROFILE`, no `ANDROMEDA_PULSE_L4_ALLOW_ROOT`, no `ANDROMEDA_PULSE_L4_DETERMINISTIC`).

## Files inspected
- `pulse-app/examples/l4_decision_probe.rs` (1-80, 95-223, 300-560, 569-873) — the whole measuring path. Arms list
  `ARMS` carries `nf` and `shipped` (`:81`); `parse_args` takes `--arms` · `--n` · `--min` · `--min-rank1` · `--out` ·
  `--dry-run` (`:569-611`); shapes S1-S3 are retry storms on one service each, S4 = S1 plus one rendered corpus match
  of an older active error-rate-spike incident on `payment-service` (`:149-223`); `names_trigger` reads the rank-1
  hypothesis statement for the trigger kind's terms (`retry` for `RetryStorm`) and labels `rank1` / `elsewhere` /
  `none`, `unparsed` when the output did not parse (`:410-452`); `generate` spawns `llama-cli` with production's argv,
  `kill_on_drop`, `LLAMA_CLI_TIMEOUT` (60 s), output cap, stderr to null (`:521-567`); `main` resolves the binary by
  profile and validates both paths through `validate_path_input` (`:662-686`), prints the header line with BASENAMES
  only, one per-generation stderr line of bounded labels (`:767-772`), the per-arm summary lines (`:797-841`), writes
  `{out}/runs.json` of bounded labels plus an output HASH (`:773-784`, `:844-848`), and with `--min-rank1 K` prints
  `l4-decision-probe: names-trigger verdict: {PASS|FAIL} · rank1 {total_rank1}/{total}` and exits 1 on FAIL
  (`:860-872`). Default `--out` is `target/l4-decision-probe/{UTC stamp}` (`:599-602`), where it also writes
  `schema-{arm}.json`.
- `pulse-app/src/llamacli_inference.rs` (76-97, 404-453) — `MAX_PROMPT_BYTES` 16 KiB (`:76`), `LLAMA_CLI_TIMEOUT` 60 s
  (`:84`), `DEFAULT_MAX_TOKENS` 1024 (`:94`), `NGL_GPU` 99 (`:97`); `build_llama_cli_args` (`-m -ngl -st --simple-io
  --no-display-prompt --log-disable -n --json-schema-file -p`); `binary_target_for_profile` maps `GpuPrimary` /
  `GpuFallback` → `(ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH, 99, "cuda")` (`:444-453`).
- `crates/interpretation/src/hardware.rs` (40-189) — `HardwareProfileDetector::new` reads
  `ANDROMEDA_PULSE_HARDWARE_PROFILE` first, else `detect_real()`; Linux GPU presence = any of
  `{usr/lib/x86_64-linux-gnu, usr/local/cuda/lib64, usr/lib, usr/lib64} × {libcuda.so, libcuda.so.1}` exists
  (`:165-189`); GPU present → `GpuPrimary` (`:107-108`).
- `pulse-app/Cargo.toml` (21-23) — `[[example]] name = "l4_decision_probe" … test = true`.
- `andromeda-pulse-0.3.0/chunks/2026-10-04-l4-interpretation-names-its-triggering-cue/plan.md` (150-474) — the
  pre-registered entries (`:328-343`): `cargo build -p pulse-app --example l4_decision_probe &&
  ./target/debug/examples/l4_decision_probe --arms shipped --n 10 --min-rank1 36`, expect `exit 0` +
  `contains names-trigger verdict: PASS`; then `./target/debug/examples/l4_decision_probe --arms nf --n 10`, expect none
  (record-only); both with `env = ['ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH', 'ANDROMEDA_PULSE_MODEL_PATH']`, timeout 7200.
  Its standard gate block (`:216-431`) is the shape this chunk's gate set copies.

## Graph impact (from the code-graph query)
- **binary_target_for_profile** — 9 rows (trace `tree-query-{marker}.json`): the probe's `main` (`l4_decision_probe.rs:663`),
  production's `LlamaCliInference::new` (`llamacli_inference.rs:137`) and five routing pins in
  `pulse-app/tests/unit_llamacli_inference.rs:133-165` (one per profile). The probe routes through the SAME function as
  the shipped runner — the probe measures production's routing, not a copy.
- **TRIGGER_FRAMING_INSTRUCTION · CORPUS_MATCHES_FRAMING_NOTE · TRIGGER_LINE_PREFIX** — referenced in the three prompt
  tiers (`prompt.rs`), the digest assembler and contract re-export, and the probe's `nf` transforms
  (`l4_decision_probe.rs:379,383,387`): the counterfactual removes exactly the shipped framing, no other text.
- No symbol is changed by this chunk: no impact set to thread.

## Patterns detected
- **Bounded-label measurement record** (`l4_decision_probe.rs:773-784`): `runs.json` rows carry `arm` · `shape` · `run`
  · `decision` · `severity` · `is_resolution_summary` · `would_create` · `first_keys` · `output_hash` · `names_trigger`
  — no model text, so the file is committable evidence as written.
- **Self-proving exit grammar** (`:619-622`, `:868-872`): exit 2 INCONCLUSIVE (unset or guard-rejected path, an arm that
  will not compose, an unwritable out dir, a spawn failure) · exit 1 a FAIL verdict · exit 0 PASS — the scenario-leg
  shape of verification-harness.md §Scenario legs.
- **Per-generation failures stay labels** (`:741-743`): a timeout / non-zero exit / bad UTF-8 / unparsable output is a
  `decision` label (`timeout`, `exit_failure`, `stdout_utf8_invalid`, `parse_failed`) and `names_trigger: unparsed` —
  counted in the denominator, never dropped, never retried.

## Conventions to follow
- **The pre-registered argv is the record** (predecessor plan `:328`, `:337`): the series runs exactly
  `--arms shipped --n 10 --min-rank1 36`, then `--arms nf --n 10` — no added `--out`, so the run is the pre-registered
  one byte for byte; `runs.json` is copied from the default out dir the header names.
- **Prebuild outside the timed run** (testing.md 2026-08-16): the example build precedes the series (the pre-registered
  shipped entry chains it with `&&`).
- **Evidence hygiene** (testing.md 2026-09-30 [ext. 2026-10-04]): a committed capture names no repo root, no `/tmp`
  path and no home path — the probe's header prints basenames only; a hand-written transcript uses placeholders.

## New files to create
- none

## Files to modify
- none

## Live-leg invocation (the series)
- Preconditions, measured at P3: the example builds (`cargo build -p pulse-app --example l4_decision_probe`, exit 0,
  11.4 s warm); `l4_decision_probe --arms shipped,nf --dry-run` exits 0 with every arm × shape under the bound —
  shipped S1-S4 6984 / 6987 / 6991 / 7185 B, nf S1-S4 6644 / 6647 / 6651 / 6766 B (max 16384); `nvidia-smi` reads an
  RTX 3090 with 702 / 24576 MiB in use; this shell exports no `ANDROMEDA_PULSE_*` variable (`env | grep -c` = 0).
- Firing form: `. ~/dev/projects/additional/pc-overseer/l4-env.sh` (`inputs#I2`) in the leg's own shell, then the
  pre-registered argv. The leg binds no port and starts no `pulse-app`; the model slot (`inputs#I1`) is the GPU and the
  operator's window, not a socket.
- Verdict atoms, from the print site (`l4_decision_probe.rs:864`): `l4-decision-probe: names-trigger verdict: PASS ·
  rank1 r/40` or `… FAIL · rank1 r/40`; routing atom from `:681`: `binary llama-cli (cuda, -ngl 99)`.
- Time budget: 40 generations per arm, each bounded by `LLAMA_CLI_TIMEOUT` 60 s (worst case 40 min per arm); the
  overseer's smoke measured 155 t/s on the GPU (`inputs#I1` context, CONTEXT block).

## Scope premise closure
- Item 4 (`[inferred]` no product code changes) — VERIFIED: the probe at HEAD carries every flag, arm and shape the
  series names (`:81`, `:569-611`, `:149-223`), and both arms compose (the dry-run above). Real mode on this host
  routes to the CUDA binary: `/usr/lib/libcuda.so` → `libcuda.so.1` → `libcuda.so.610.57.04` exists, so
  `detect_real()` returns `GpuPrimary` and `binary_target_for_profile` selects `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` at
  `-ngl 99`; the leg env sets no `ANDROMEDA_PULSE_HARDWARE_PROFILE` override, so detection — not an override — routes
  the run, and the probe's header line attests it.
- Item 5 (`[inferred]` a FAIL is a recorded outcome, never re-run) — VERIFIED as a premise (the probe records a FAIL as
  exit 1 with its verdict line; test-plan §10/§11 and arch [Fault Identity] forbid re-thresholding or a repeat); its
  disposition (complete with the FAIL recorded vs held) stays the P4 fork the scope names.
- CONTEXT (block cleared, routing) — the routing half re-derived above (VERIFIED); the binary half re-measured at P1
  (`version: 9305 (63248fc)`, Linux ELF, `libggml-cuda.so` beside it).
- Mechanism claim the chunk measures (`[inferred]`, verbatim marker kept in scope): "the d3 model restated a
  corpus-match title". Not re-derivable at HEAD — it is what the `nf` vs `shipped` comparison measures; only S4
  carries a corpus match, so the hypothesis' direct reading is S4's per-shape rank1 in each arm.
- One finding the scope did not state: the probe installs NO tracing subscriber (`grep -n 'tracing\|observability\|subscriber'`
  on the example: 0 hits), so `interpretation.hardware.detect` / `interpretation.model.allow_root` /
  `interpretation.model.load` emit nowhere during the series. The routing witness is the probe's own header line,
  never a self-observation record (obs extract's acceptance item 3: recorded as "routing not attested by
  self-observation; attested by the probe header").
- One finding the arch history raised, re-derived: the deterministic runner cannot reach the series — the example
  never reads `ANDROMEDA_PULSE_L4_DETERMINISTIC` (`grep -n DETERMINISTIC` on it: 0 hits) and spawns `llama-cli`
  directly; the leg env does not set it either.

## Open questions
- On a FAIL, does the chunk complete with the FAIL recorded (a remedy becomes a new route entry), or does it hold
  without completing? → blocks: plan-decision (the acceptance line and the shipped entry's `expect`).
- Does the chunk discharge the probe's still-owed flag-parse pins (test-plan §1
  `l4-decision-probe-arg-parse-unit-coverage`)? → blocks: plan-decision (leaned at P4: no — the scope plans no code, and
  the Boundaries freeze the probe before the run).
