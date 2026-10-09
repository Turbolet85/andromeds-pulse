# arch extract

## Relevance
partial — a measure-and-record chunk with no planned product code; arch binds through the L4 runtime decision, the pre-registration it names, the three L4 path env vars, and the shared OTLP port slot.

## Constraints
- The L4 runtime is `llama.cpp` b9305-pinned prebuilt binaries invoked D1 spawn-per-generation, CUDA build + `-ngl 99` for the gpu tiers, CPU build + `-ngl 0` for the cpu tiers, with bounded `-st` + `-n {max_tokens}` + outer wall-clock timeout (per arch §Established Decisions [LLM Inference Runtime — L4 interpretation layer]; §Inherited Defaults → LLM inference runtime). The Linux build the series uses must be the same pinned tag (b9305); a different tag would be a chunk-scoped bump event per the same entry's Pin discipline, not drift accepted in passing. Whether the binary the leg env names is b9305 is a P3 re-check.
- Arch records the trigger framing as framing, not identity, and its rank-1 effect as UNMEASURED, with the pre-registered series named verbatim (`--arms shipped --n 10 --min-rank1 36`, ≥ 36/40, `nf` recorded beside it) (per arch §Established Decisions [Fault Identity]). The chunk must run that series as written; the tuple `(kind, scope, scope_id)`, the coalesce predicate and title grounding stay untouched.
- Routing from hardware profile to the CUDA binary goes through the `HardwareProfileSource` tier. Real Linux detection is a fixed-path `libcuda.so`/`libcuda.so.1` presence check, and `ANDROMEDA_PULSE_HARDWARE_PROFILE` is an override that is never logged (per arch §Occupied Resources → env vars `ANDROMEDA_PULSE_HARDWARE_PROFILE`). P3 must answer whether the probe's real-mode path resolves gpu-tier → `_LLAMA_CUDA_BIN_PATH` on this host at HEAD; arch states only the target.
- The three L4 path inputs (`ANDROMEDA_PULSE_MODEL_PATH`, `_LLAMA_CUDA_BIN_PATH`, `_LLAMA_CPU_BIN_PATH`) are product-consumed and keep the always-on guard: traversal reject before canonicalize, a 4096-byte bound, canonicalize, and a regular-file check. Confinement is opt-in via `ANDROMEDA_PULSE_L4_ALLOW_ROOT`, an unconfined posture logs one WARN per boot, and a set-but-unresolvable root fails closed (per arch §Occupied Resources → env vars `ANDROMEDA_PULSE_MODEL_PATH` / `ANDROMEDA_PULSE_L4_ALLOW_ROOT`). The leg env feeds these vars and does not change their guard.
- `:4317`/`:4318` are loopback-only, spec-fixed resources (per arch §Occupied Resources → Network ports). Any probe or app run that binds them occupies the slot granted to this chunk; no alternate port binding is introduced.
- The chunk adds nothing to the registry: no new port, TauRPC procedure, env var, corpus table or crate (per arch §Occupied Resources; §Inherited Defaults → Module boundaries). Under that rule, a zero §Occupied Resources delta at wrap is the expected outcome.

## Patterns to follow
- Swap boundary: the `LlmInferenceRunner` trait in `crates/interpretation` is the seam, and only the concrete `LlamaCliInference` at the `pulse-app` binary boundary touches the runtime (per arch §Established Decisions [LLM Inference Runtime]; §Cross-cutting Patterns → Module dependency direction). A probe under `pulse-app/examples/` sits on the binary side of that seam.
- Agent-driven harness shape: deterministic invocation, machine-parseable output (the probe's one-line verdict + `runs.json`) (per arch §Cross-cutting Patterns → Development Style).
- Config precedence for the leg: process env > `config.toml` > defaults, so the leg env file is the whole configuration of the run (per arch §Cross-cutting Patterns → Config management).

## Anti-patterns to avoid
- Re-tuning the framing, digest, probe arms or the ≥ 36/40 threshold after seeing a result would turn a recorded decision input into a moving target. Arch records the measurement as the open half of the [Fault Identity] framing entry (per arch §Established Decisions [Fault Identity]).
- Logging or committing a full product-consumed L4 path. The L4 guard's records are basename-only by design (per arch §Occupied Resources → env vars `ANDROMEDA_PULSE_L4_ALLOW_ROOT`).
- Floating the llama.cpp build ("latest tag") for the measurement leg (per arch §Established Decisions [LLM Inference Runtime] → Pin discipline).

## Contract bindings
- arch ↔ tests: the pre-registered rule and its verdict line are the tests-domain acceptance; arch's [Fault Identity] entry carries "UNMEASURED … gated on a Linux llama.cpp CUDA binary". Wrap owes an arch amendment of that clause to the measured outcome (PASS or FAIL, with the `nf` reading), with no change to the identity semantics.
- arch ↔ security: the L4 path-env guard and the basename-only logging (arch §Occupied Resources env vars) bind to security-plan §Security Anti-Patterns → Input's narrowed exception and to committed-evidence hygiene.
- arch ↔ obs: `interpretation.model.allow_root` / `interpretation.model.load.error` / `interpretation.hardware.detect` are the boot records that attest the posture and tier the run took (per arch §Occupied Resources → env vars).

## Acceptance criteria contributions
- The series runs on the b9305-pinned `llama-cli` through the CUDA tier path (gpu profile → `_LLAMA_CUDA_BIN_PATH`, `-ngl 99`), as attested by the run's tier/binary evidence and not assumed (per arch §Established Decisions [LLM Inference Runtime — L4 interpretation layer]).
- §Occupied Resources delta is zero: no new port, TauRPC procedure, env var, corpus table or crate, and `cargo xtask capability-drift` stays clean (per arch §Occupied Resources).
- At wrap, the [Fault Identity] entry's "whether it moves the model's rank-1 hypothesis is UNMEASURED … gated on a Linux llama.cpp CUDA binary" clause is amended to the recorded verdict (`rank1 r/40` + the `nf` distribution beside it), and the identity key, coalesce predicate and title grounding text are left unchanged (per arch §Established Decisions [Fault Identity]).
- Committed evidence carries no full L4 path, only basenames or `~`-relative spellings (per arch §Occupied Resources → env vars `ANDROMEDA_PULSE_MODEL_PATH` / `ANDROMEDA_PULSE_L4_ALLOW_ROOT`).
