# arch extract

## Relevance
relevant — adds an alternative `LlmInferenceRunner` impl selected at the binary boundary for deterministic digest→L4→incident completion.

## Constraints
- (arch §Stack) `LlmInferenceRunner: Send + Sync` trait boundary is the swap point; deterministic runner is another impl behind this trait (per chunk #82 design).
- (arch §Established Decisions) The trait was designed to anticipate impl swaps as the bus-factor mitigation pattern; only the concrete impl changes, never the trait signature.
- (arch §Occupied Resources §Environment variables) New `ANDROMEDA_PULSE_*` env var must register here; existing `_LLAMA_CUDA_BIN_PATH` + `_LLAMA_CPU_BIN_PATH` pin the b9305 baseline.
- (arch §Conventions) Env var naming `SCREAMING_SNAKE_CASE`; must not collide (`_OTLP_*`, `_LLAMA_*`, `_RETENTION_SECONDS`, `_LOG_LEVEL`, `_MCP_ENABLED`, `_PLUGIN_DIR`, `_PIDFILE`/`_LOGFILE`).
- (arch §Cross-cutting §Feature-gate hygiene) Deterministic mode is env-selected at runtime, NOT a `--features` compile-time flag — binary identical across configs.
- (arch §Inherited Defaults) Boundary error handling uses `anyhow::Result`; module-internal `thiserror`.

## Patterns to follow
- The existing `StubInferenceRunner` pattern (in `crates/interpretation/`) is the canned-output template; reuse its contract + error handling.
- Deterministic mode selected at the `pulse-app` binary boundary (same site where hardware-profile tier routing picks `LlamaCliInference`), per chunk #84 selection precedent.
- The `L4Output` produced must be schema-valid so the chunk #81 cadence/digest subscriber + chunk #92 incident-creation consume it unchanged.

## Anti-patterns to avoid
- Do not introduce a `--features deterministic-l4` flag — env-gate the selection (binary constant across test/prod).
- Do not bypass the real incident-creation path; the deterministic `L4Output` still flows through digest→L4→incident (chunk #92 producer).

## Contract bindings
- **arch ↔ interpretation**: `LlmInferenceRunner` trait — deterministic impl is a third option alongside `LlamaCliInference` + `StubInferenceRunner`.
- **arch ↔ triage (chunk #81)**: cadence/digest subscriber invokes the runner + consumes output; preserve `L4Output` schema.
- **arch ↔ chunk #92 incident-creation**: consumes the L4 output → creates incidents; deterministic mode produces valid incidents incl. deterministic severity.

## Acceptance criteria contributions
- (arch) New `ANDROMEDA_PULSE_*` env var registered in §Occupied Resources at wrap (phase-write-blocked; amendment-deferred).
- (arch) `LlmInferenceRunner` trait honored — deterministic impl `Send + Sync`, returns `L4Output` with no network/subprocess.
- (arch) Deterministic `L4Output` schema-valid + drives chunk #92 incident creation with deterministic severity (reproducible red-dot/Findings per P-073).

## Relevant amendment history
- **2026-05-25 — LLM runtime mistralrs → llama.cpp D1**: the swap-path design that makes this chunk possible; trait built to support multiple impls without signature change.
- **2026-05-24 — `interpretation` crate boundary**: this chunk adds a concrete impl within the existing crate.
- **2026-05-23 — `pulse://stream/digests`**: chunk #81 cadence/digest upstream; deterministic L4 invoked from the cadence subscriber on this stream.
