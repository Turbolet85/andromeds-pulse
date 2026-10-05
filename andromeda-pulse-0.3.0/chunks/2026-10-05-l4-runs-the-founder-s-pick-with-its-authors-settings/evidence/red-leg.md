# RED leg — the chunk-base build runs the pick (step 1, measure first)

Host paths are placeholdered: `{data-dir}` (the leg's fresh data dir under the session temp dir), `{model}`
(the pick's GGUF), `{llama-cli}` (the b9305 CUDA build).

## Setup
- Chunk base `7663dd91c449`; no source edit before the build. `cargo build -p ingest --example inject_demo`
  exit 0, `cargo build -p pulse-app` exit 0 (gates 20 / 21, run by hand on the base tree).
- Slot precondition (gate 22's two reads, 2026-10-05 16:23 CEST, daytime): `ss -ltnH` → no `:4317` / `:4318`
  listener; `nvidia-smi --query-compute-apps` → one process, `voxtype-osd-gtk4` 10 MiB, no `llama-cli`.
- The pick: `sha256sum {model}` → `85a896a047553e842f25297ee5b031d64ff30147d9c4af17b1e4b394cd1fab87`
  (`gemma-4-E4B-it-Q4_K_M.gguf`).
- Boot: `target/debug/pulse-app` by path, on a fresh `ANDROMEDA_PULSE_DATA_DIR={data-dir}` exported once;
  `ANDROMEDA_PULSE_MODEL_PATH={model}`, `ANDROMEDA_PULSE_LLAMA_{CUDA,CPU}_BIN_PATH={llama-cli}`;
  `ANDROMEDA_PULSE_L4_DETERMINISTIC` and `ANDROMEDA_PULSE_HARDWARE_PROFILE` unset (measured: absent from the
  launch shell's env). `:4317` accepted within 1 s; pid = `pulse-app`.
- Boot records: `interpretation.hardware.detect` `profile: gpu_primary`; `interpretation.model.load`
  `inference_mode: real`, then `load_status: loaded` with `model_identity: gemma-4-E4B-it-Q4_K_M`.

## The round
`gate.py run --live-legs --entry 25` → `round: COMPLETE · legs fired 1/1`: gate 24 (liveness) green, gate 25
(`inject_demo`, arg-less) green, exit 0 after 301.52 s. Lines in `round-142920Z.txt`.

## Reads (all on the leg's own log family, `{data-dir}/logs/agent-latest.jsonl*`)
| gate | read | result |
|---|---|---|
| 26 | poll for `interpretation.inference.error` `json_parse_failed` | `3` on the first poll (14:24:30Z), exit 0 — green |
| 27 | `interpretation.json.parse` with `parse_outcome: ok` | `0`, exit 1 — matches `expect = ['exit 1', 'last line 0']` |
| 31 | `bash xtask/ci/l4-latency-p99.sh` (grader at the chunk base) | `l4-latency-p99: no metric.pipeline.l4.inference_latency_p99_milliseconds records found (INACTIVE state — pre-mistralrs-binding chunk; gate trivially passes)`, exit 0 |

Whole-leg counts after the storm:
- `interpretation.inference.error`: 4, every one `error_category: json_parse_failed`, `recovery_action: skip_digest`.
- `interpretation.json.parse`: 0 records of any outcome.
- `metric.pipeline.l4.inference_latency_p99_milliseconds`: 4 samples (1783 · 1787 · 1907 · 1995 ms), 0 of them
  carrying `hardware_profile`. That is why the grader found "no records": it keeps a sample only with that field.
- 0 `"level":"ERROR"` and 0 `app.panic.fatal`. The defect is BLOCK-shaped (exit 0, no JSON), so a clean log is
  expected here and the 0-ERROR assertion is not applied to this leg (test-plan §3, 2026-08-28).

## Second source for the absence — direct `llama-cli`, after the app stopped
The same argv shape outside the product: `-m {model} -ngl 99 -c 8192 -rea off -st --simple-io
--no-display-prompt --log-disable -n 64 … -p 'Reply with the JSON object only.'`.
- With `--json-schema-file crates/interpretation/src/schema.json`: exit 0, stdout carries `Failed to initialize
  samplers` (1×) and no `{`.
- With `--grammar-file pulse-app/src/l4-output.gbnf`: exit 0, no sampler-init failure, stdout carries `{`.

## Teardown
`kill -TERM` on the app pid: gone within 10 s; `app.exit` `exit_class: signal, signal: sigterm`; `:4317` /
`:4318` listeners 0; no `pulse-app`, `inject_demo` or `llama-cli` process left.
