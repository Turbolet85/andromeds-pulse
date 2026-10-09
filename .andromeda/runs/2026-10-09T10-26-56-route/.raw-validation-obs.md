# Obs validation — route draft

## Insert
- Between `Curated snapshot through the door` and `Theme 2 checked by the external harness`: **"Failed engine gate keeps a snapshot — a failing engine-gate run in CI leaves the curated snapshot beside its log"** (epoch: `Epoch 6`)
  Reason: per obs-plan §9 Telemetry artifact handling ("Snapshot markdown (on test failure)") and §11 CI ("NEVER skip snapshot generation on test failure"), this artifact is a plan bootstrap item that no draft chunk carries and that `.github/workflows/ci.yml` does not upload at HEAD (logs, junit, criterion and coverage only).

## Rewrite
- `Door inside the engine's process`: "own host only;" → "own host only, calls logged unredacted;"
  Reason: per obs-plan §8 default-deny allowlist and §4 scenario P3 required fields, the door's `mcp.*` targets resolve only in the sidecar's own allowlist (bare `mcp` key, `crates/mcp-server/src/tracing_setup.rs`) while the engine's carries only `mcp-server`, so in-process door records fall to no entry and redact wholesale. Epoch 1's engine gate and detection baseline both read through the door two epochs before `Log allowlist describes this engine`.
- `Engine end-to-end gate reachable`: "log kept" → "log graded and kept"
  Reason: per obs-plan §10 CI gates, the zero-span, zero-panic and heartbeat-gap gates run in CI only as the boot job's `cargo xtask ci-gates` step over the boot-smoke log, and `Window's gates retired` removes that job in Epoch 2, so the engine gate's log must be the graded one before then.
- `Log allowlist describes this engine`: "every allowed target one the engine emits" → "every allowed target emitted, every emitted field allowed"
  Reason: per obs-plan §8 muted-diagnostic backlog (one recurrence still open), the measured failure is the reverse direction — an emitted field with an absent or partial leaf rendering `<redacted>` — which a dead-leaf sweep alone does not close.
- `Engine memory measured`: "replaces the buffer's row-count estimate" → "replaces the buffer's row-count estimate in the budget gate"
  Reason: per obs-plan §10 "Buffer memory bounded" and §11 SLO ("NEVER define soft SLO budgets"), the memory budget is the CI-required `perf:budget --require memory` arm grading `metric.buffer.memory_bytes`, so a figure that is only recorded would leave that arm grading a retired gauge or nothing.
