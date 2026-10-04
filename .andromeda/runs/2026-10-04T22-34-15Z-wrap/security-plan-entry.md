
## 2026-10-04-declared-rust-floor-matches-the-code — the Edition-2024 minimum beside the declared build floor
**Section:** §Security Anti-Patterns → Universal (the rust-toolchain minimum ban)
**Change:** "NEVER let the rust-toolchain drift below 1.85.0" stands as the Edition-2024 minimum. Beside it the body now records the build floor: the workspace declares `rust-version = "1.95"`, equal to the pinned 1.95.0 channel and set by the dependency graph (wasmtime / cranelift / pulley declare 1.95.0), held equal by the xtask test `declared_floor_equals_the_pinned_channel`. The raise this ban's Kept assigned to the route entry "The declared Rust floor matches the code" (per "2026-10-04-corpus-key-creation-is-race-free — XDG_RUNTIME_DIR lock dir and the content-free lock file, outside the data dir") has landed.
**Why:** The plan's recorded expected amendment; no detector covers a toolchain-floor statement, so the orchestrator raised it. A minimum and a floor are different claims, and both are true.
**Kept:** The corpus-key lock file's never-deleted product contract (§Data Protection): this chunk's only new unlink is in `mod tests`, on the test's own pid-scoped lock name.
**Ref:** .andromeda/runs/2026-10-04T22-34-15Z-wrap/
