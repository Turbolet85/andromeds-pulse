# `interpretation` — L4 LLM Interpretation Layer (trait surface + hardware profile)

## Responsibility
Hosts the L4 interpretation layer's **abstraction**, not its runtime (chunk #82 — Epoch 9 Foundation v0.2.0). Owns the `LlmInferenceRunner` trait that every inference backend implements, the model/hardware contract types, the cross-platform GPU probe, and the `pulse://stream/model-status` broadcast. It does **NOT** own any concrete inference implementation: per arch §Established Decisions [LLM Inference Runtime] bus-factor mitigation, the concrete impl lives at the binary boundary in `pulse-app/` so this crate never imports a heavy runtime.

## Key integrations

### Consumes from
- `triage::contract::HardwareProfileSource` — the chunk #80 trait this crate's `HardwareProfileDetector` implements.
- `ANDROMEDA_PULSE_HARDWARE_PROFILE` — env override for the detected profile (bounded parse).

### Publishes to
- `interpretation::contract` re-exports (the only `pub` surface): `LlmInferenceRunner` · `ModelTier` · `ModelStatus` · `ModelIdentity` · `ModelLoadEvent` · `InferenceError`.
- `ModelStatusBroadcast` → the `pulse://stream/model-status` Tauri IPC topic.
- Consumed by `pulse-app::model_router` (`model.current_profile`), `pulse-app::inference_runtime` (the incident producer) and `pulse-app::investigate_router` (`investigate.run_action`).

### Dependencies
- Workspace-inherited: `serde`, `serde_json`, `thiserror`, `tracing`, `tokio`, `chrono`; path deps `triage` (the `HardwareProfile` substrate) and `security` (the PII scrubber the shared report projection applies). Dev-only: `rstest`, `tokio` with `test-util`, `tempfile` (the probe pins' `TempDir` fixtures, since chunk 2026-10-04-l4-hardware-probe-finds-cuda-on-arch-layout-hosts). **No `llama.cpp`, no `mistralrs`, no `candle`** — deliberately.

## Internal conventions
- **Async trait without `async-trait`:** `LlmInferenceRunner` declares methods returning a manual `Pin<Box<dyn Future<Output = …> + Send + 'a>>`, mirroring the 2026-05-23 `SqlQueryRunner` precedent. Native `async fn` in traits is not dyn-compatible, and the workspace runs a "no new deps" discipline — so the boxed-future form is what keeps `Arc<dyn LlmInferenceRunner>` injectable at the binary boundary. Concrete impls wrap their body in `Box::pin(async move { … })`.
- **Cross-crate serde:** payload types that reference a non-serde enum from another crate store a **bounded `String` label** computed via that crate's label fn (e.g. `profile_label(...)`), never a forced derive on the source crate — preserves the arch DAG (2026-05-24 pattern).
- **`contract` module** is the ONLY `pub` surface; internals are `pub(crate)`.

## Service-specific gotchas
- **The GPU probe is a per-platform file-presence check, never a `dlopen`:** macOS assumes Metal; Windows checks `C:\Windows\System32\nvcuda.dll`; Linux checks `libcuda.so` or its soname `libcuda.so.1` under `/usr/lib/x86_64-linux-gnu`, `/usr/local/cuda/lib64`, `/usr/lib` or `/usr/lib64` (8 candidates, `Path::exists`, so a dangling symlink reads absent). The Linux set lives in two private constants behind the root-taking seam `cuda_driver_present_under(root)`, which the arm calls with `/`; the `cuda_probe_` pins run it over `TempDir` layouts and pin the constants exactly (chunk 2026-10-04-l4-hardware-probe-finds-cuda-on-arch-layout-hosts — before it, an Arch-layout host with only `/usr/lib/libcuda.so` read GPU-absent). No subprocess, no env input, no log of a path. A probe miss is a *tier downgrade*, never a panic — it resolves to a CPU tier and keeps booting.
- **Tier selects the binary, not just a flag:** GPU-primary / GPU-fallback → the CUDA `llama-cli` + `-ngl 99`; CPU-primary / CPU-fallback / Unknown → the CPU build + `-ngl 0`. The paths come from `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` / `_CPU_BIN_PATH`, consumed in `pulse-app/src/llamacli_inference.rs`. Since 2026-08-26 those two and `ANDROMEDA_PULSE_MODEL_PATH` pass `validate_path_input` — traversal-reject before canonicalize, 4096-byte bound, canonicalize, regular-file assert, plus confinement under the opt-in `ANDROMEDA_PULSE_L4_ALLOW_ROOT` (fail-closed if that root is set but unresolvable). A rejection degrades to `ModelStatus::Error` → `ModelNotConfigured` and emits `interpretation.model.load.error` with a basename only.
- **`interpretation.model.load` is UN-MUTED since chunk 2026-08-30-diagnostics-un-muting-harness-truth-sweep** — the leaf was COMPLETED to `{model_identity, tier, load_status, inference_mode}`: `inference_mode` added, the never-emitted `file_size_bytes` REMOVED (zero producers), and `model_identity` now carries the semantic name at the loaded emit. (History: recorded 2026-08-15 as "resolves to no allowlist leaf"; corrected 2026-08-26 to a PARTIAL leaf — the repair was complete-the-leaf, not add-one — and executed 2026-08-30, live-verified unredacted on the boot smoke.)
- **`interpretation.incident.created` needs its own EXACT leaf** and **no bare `interpretation` prefix key may exist** — a prefix key would widen every future `interpretation.*` target added without its own leaf to one field set. The invariant HOLDS in code since chunk 2026-08-30-diagnostics-un-muting-harness-truth-sweep, which removed the bare key; every `interpretation.*` registration in `AllowList::production()` is a dotted exact leaf (`interpretation.hardware.detect` among them). Its field-completeness guard runs in `pulse-app/tests/observability_pins.rs`, beside the PII half in `pulse-app/tests/unit_incident_producer.rs`.
- **Deterministic mode is contract-visible:** `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` swaps in a canned-`L4Output` runner at boot (`pulse-app/src/deterministic_inference.rs`). Its `evidence_refs` array is deliberately POPULATED — that is load-bearing for gradeability; empty arrays made downstream payload-identity assertions pass vacuously. The incident title is cue-grounded at the producer in every mode, so a deterministic storm incident reads `Retry storm: Deterministic verification incident`; the canned rank-1 hypothesis names a retry storm for EVERY incident, so only the title discriminates the cause in this mode.

## Entry points for modification
- **Public contract / trait:** `crates/interpretation/src/contract.rs`
- **Hardware probe:** `crates/interpretation/src/hardware.rs` (`HardwareProfileDetector`, `detect_gpu_present`, the Linux candidate constants)
- **Model-status broadcast:** `crates/interpretation/src/` (`ModelStatusBroadcast`)
- **Concrete impls (binary boundary, NOT this crate):** `pulse-app/src/llamacli_inference.rs` · `pulse-app/src/deterministic_inference.rs`
- **Tests:** co-located `#[cfg(test)] mod tests`; the runner is exercised end-to-end from `pulse-app/tests/`.

## Capabilities
P-053 (Fallback Model Tier — detection side) · P-054 (Hardware Profile Awareness).

## References
- `.andromeda/architecture.md` §Established Decisions [LLM Inference Runtime — L4 interpretation layer] (incl. the three documented sibling-impl swap paths)
- `.claude/docs/stack.md` — AI/ML serving row
- `.claude/rules/observability.md` — allowlist-leaf discipline
