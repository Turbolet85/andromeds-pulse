# GREEN legs — the rebuilt tree runs the pick (step 12)

Host paths are placeholdered: `{data-dir}` (the leg's fresh data dir under the session temp dir), `{model}`
(the pick's GGUF), `{llama-cli}` (the b9305 CUDA build).

## Preconditions
- The full non-leg gate block (entries 1–21) green on the rebuilt tree at 14:40:11Z; `target/debug/pulse-app`
  built 16:39:36 local, after the last source write (16:38:02).
- Gate 22, by hand at 16:40:31 CEST (daytime): `test "$(ss -ltnH | grep -cE ':(4317|4318) ')" = 0 && nvidia-smi
  --query-compute-apps=…` → exit 0, the one compute process `voxtype-osd-gtk4` 10 MiB, no `llama-cli`
  (`lacks llama` holds).

## (a) The env-gated real round trip — gate 23
`gate.py run --live-legs --entry 23` (env: `ANDROMEDA_PULSE_MODEL_PATH={model}`,
`ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH={llama-cli}`) → `round: COMPLETE · legs fired 1/1`, gate 23 green, 9.68 s
(`round-144048Z.txt`). The test ran, not skipped (`1 test run: 1 passed, 0 skipped`, no `[skip]` line):
```
[integration-real] profile=GpuPrimary
[integration-real] binary={llama-cli}
[integration-real] model={model}
[integration-real] load_from_env_if_configured -> Loaded (0 ms)
[integration-real] prompt bytes: 7322
[integration-real] generate_constrained -> 2409 bytes in 8775 ms (real subprocess)
[integration-real] L4Output: decision=Surface severity=Suggested tier=primary profile=gpu_primary hypotheses=2 steps=2
[integration-real] PASS — runtime swap functionally live end-to-end
```
It drives the production `generate_constrained` (the grammar argv and the authors' sampling) that the
Investigate path shares.

## (b) The end-to-end leg
- Boot: `target/debug/pulse-app` by path on a fresh `ANDROMEDA_PULSE_DATA_DIR={data-dir}` exported once,
  `ANDROMEDA_PULSE_MODEL_PATH={model}`, `ANDROMEDA_PULSE_LLAMA_{CUDA,CPU}_BIN_PATH={llama-cli}`;
  `ANDROMEDA_PULSE_L4_DETERMINISTIC` / `ANDROMEDA_PULSE_HARDWARE_PROFILE` unset. `:4317` accepted within 1 s;
  `interpretation.hardware.detect` `profile: gpu_primary`.
- The round: `gate.py run --live-legs --entry 25` → `round: COMPLETE · legs fired 1/1`, gate 24 green, gate 25
  (`inject_demo`, arg-less) green, exit 0 after 301.47 s (`round-144612Z.txt`).

| gate | read | result |
|---|---|---|
| 28 | poll for `interpretation.json.parse` `parse_outcome: ok` | `1` on the second poll (14:41:27Z), exit 0 — green |
| 29 | `interpretation.model.load` naming `gemma-4-E4B-it-Q4_K_M` | `1`, exit 0 |
| 30 | `app.panic.fatal` or `"level":"ERROR"` | `0`, exit 1 — matches `expect = ['exit 1', 'last line 0']` |
| 31 | `bash xtask/ci/l4-latency-p99.sh` (the leg family) | `l4-latency-p99: profile=gpu_primary p99=6716ms ≤ 10000ms (sample_count=7) PASS`, exit 0 |
| 32 | `L4_GPU_PRIMARY_BUDGET_MS=1 bash xtask/ci/l4-latency-p99.sh` | `::error::l4-latency-p99: profile=gpu_primary p99=6716ms exceeds budget 1ms (sample_count=7)`, exit 1 |
| 33 | the full model path or `andromeda-pulse-llama-grammar` in the log | `0`, exit 1 — matches `expect = ['exit 1', 'last line 0']` |

Whole-leg counts:
- `interpretation.json.parse`: 7 records, all `parse_outcome: ok`. `interpretation.inference.error`: 0.
- `metric.pipeline.l4.inference_latency_p99_milliseconds`: 7 samples, every one `hardware_profile: gpu_primary`:
  5642 · 5685 · 5995 · 6215 · 6230 · 6549 · 6716 ms (nearest-rank p99 of n = 7 is the 7th smallest, 6716).
- `interpretation.incident.created`: 3; `interpretation.incident.skipped`: 4.
- `interpretation.model.load` field sets: `{inference_mode}` · `{load_status, tier}` · `{load_status,
  model_identity, tier}` — inside the leaf `{model_identity, tier, load_status, inference_mode}`; only the
  identity's value names the pick.
- WARN records: 1 `interpretation.model.allow_root` (the once-per-boot unconfined posture) and 2
  `ui.webgpu.adapter`. 0 ERROR, 0 panic.

### A pre-existing redaction the leg exposed (not this chunk's change)
7 records carry `"<redacted>"`, all on `interpretation.constrained.generate`: its `raw_output_bytes` and
`extracted_bytes` fields are absent from that target's allowlist leaf. The emit site (the success arm of
`LlamaCliInference::generate_constrained`) is byte-unchanged by this chunk. It never fired on the chunk base
with this model, since every generation failed before it (the RED leg's log carries 0 such records). Reported
for the wrap, not fixed here.

## Teardown
`kill -TERM` on the app pid: gone within 10 s; `app.exit` `exit_class: signal, signal: sigterm`; `:4317` /
`:4318` listeners 0; no `pulse-app`, `inject_demo` or `llama-cli` process left; 0
`andromeda-pulse-llama-grammar-*` files left in the temp dir (the RAII guard removed every per-spawn grammar).
