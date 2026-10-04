
## 2026-10-04-linux-launch-stays-up-on-nvidia-wayland — render-posture placement owed; pulse-app test-file count
**Section:** §1 Pending coverage triggers (new row `render-posture-main-placement-coverage`) · §4 Unit Test Strategy → Conventions → Test file location (Rust)
**Change:** §1 gains `render-posture-main-placement-coverage`: the lib is pinned (per-arm decision, re-exec `set_var` children, leaf + emit capture) but `main()`'s order — the apply call first, ahead of every runtime/thread creation, and the emit after `observability::init` — is proven only by that chunk's gate-time placement probe and its two dev-host operator legs; owed: a committed assertion over that order. §4: was "92 targets"; now 101 top-level `pulse-app/tests/*.rs` files (99 at the chunk's base).
**Why:** the same one-time-proof-for-a-gate class as `discovery-observer-wiring-coverage` / `exit-hook-main-composition-coverage`; the count moved with this chunk's two new test files and had gone stale before it.
**Ref:** .andromeda/runs/2026-10-04T12-30-22Z-wrap/
