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
- Workspace-inherited: `serde`, `thiserror`, `tracing`, `tokio` (sync primitives only). **No `llama.cpp`, no `mistralrs`, no `candle`** — deliberately.

## Internal conventions
- **Async trait without `async-trait`:** `LlmInferenceRunner` declares methods returning a manual `Pin<Box<dyn Future<Output = …> + Send + 'a>>`, mirroring the 2026-05-23 `SqlQueryRunner` precedent. Native `async fn` in traits is not dyn-compatible, and the workspace runs a "no new deps" discipline — so the boxed-future form is what keeps `Arc<dyn LlmInferenceRunner>` injectable at the binary boundary. Concrete impls wrap their body in `Box::pin(async move { … })`.
- **Cross-crate serde:** payload types that reference a non-serde enum from another crate store a **bounded `String` label** computed via that crate's label fn (e.g. `profile_label(...)`), never a forced derive on the source crate — preserves the arch DAG (2026-05-24 pattern).
- **`contract` module** is the ONLY `pub` surface; internals are `pub(crate)`.

## Service-specific gotchas
- **The GPU probe is per-platform and dlopen-shaped:** Metal on macOS, `libcuda.so` on Linux, `nvcuda.dll` on Windows. A probe failure is a *tier downgrade*, never a panic — it must resolve to a CPU tier and keep booting.
- **Tier selects the binary, not just a flag:** GPU-primary / GPU-fallback → the CUDA `llama-cli` + `-ngl 99`; CPU-primary / CPU-fallback / Unknown → the CPU build + `-ngl 0`. The paths come from `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` / `_CPU_BIN_PATH`, consumed in `pulse-app/src/llamacli_inference.rs`.
- **`interpretation.model.load` is a MUTED target** — its `inference_mode` field resolves to no obs allowlist leaf and is redacted in production today (measured 2026-08-15). Do not read its silence as evidence. Owner: the "Diagnostics un-muting + harness-truth sweep" route entry.
- **`interpretation.incident.created` needs its own EXACT leaf** and **no bare `interpretation` prefix key may exist** — a prefix key would widen the muted sibling above and every future `interpretation.*` target to one field set. Its field-completeness guard currently sits in `observability.rs`'s own `mod tests`, which `[lib] test = false` compiles but never runs — only the PII half is live-guarded.
- **Deterministic mode is contract-visible:** `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` swaps in a canned-`L4Output` runner at boot (`pulse-app/src/deterministic_inference.rs`). Its `evidence_refs` array is deliberately POPULATED — that is load-bearing for gradeability; empty arrays made downstream payload-identity assertions pass vacuously.

## Entry points for modification
- **Public contract / trait:** `crates/interpretation/src/contract.rs`
- **Hardware probe:** `crates/interpretation/src/` (`HardwareProfileDetector`)
- **Model-status broadcast:** `crates/interpretation/src/` (`ModelStatusBroadcast`)
- **Concrete impls (binary boundary, NOT this crate):** `pulse-app/src/llamacli_inference.rs` · `pulse-app/src/deterministic_inference.rs`
- **Tests:** co-located `#[cfg(test)] mod tests`; the runner is exercised end-to-end from `pulse-app/tests/`.

## Capabilities
P-053 (Fallback Model Tier — detection side) · P-054 (Hardware Profile Awareness).

## References
- `.andromeda/architecture.md` §Established Decisions [LLM Inference Runtime — L4 interpretation layer] (incl. the three documented sibling-impl swap paths)
- `.claude/docs/stack.md` — AI/ML serving row
- `.claude/rules/observability.md` — allowlist-leaf discipline
