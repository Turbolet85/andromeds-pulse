
## 2026-09-29-p-025-hue-shift-observable-made-gradable — wasmtime requirement 46 → 48.0.3
**Section:** §Stack and Technologies → Plugin runtime · §Established Decisions → [Plugin Runtime] · §Inherited Defaults → Plugin runtime
**Change:** Was requirement `"46"`, lockfile-resolved 46.0.3 (Cranelift 0.133.3); now requirement `"48.0.3"`, lockfile-resolved 48.0.3 as of 2026-09-29 (Cranelift 0.135.3, read from the lockfile at this wrap). The Stack row adds that the bump closed RUSTSEC-2026-0316 and that 49.x is out of reach while the toolchain is pinned at 1.95 (49.0.1 needs Rust 1.96). The 25+ family floor is unchanged.
**Why:** the chunk upgraded wasmtime in-chunk to clear its first real CI run's advisory (founder ruling, via the overseer); three body sites restated the old pin.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
