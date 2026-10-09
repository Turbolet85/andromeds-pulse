
## 2026-09-30-dual-license — [License] decision; crate counts; the capability-drift slot
**Section:** §Established Decisions → [License] (new) · §Design Philosophy → Single-process modular monolith · §Infrastructure Patterns → runtime topology · §Occupied Resources → xtask CLI surfaces (the staged-assertion sentence)
**Change:**
- New [License] decision: `MIT OR Apache-2.0`, holder Turbolet85. `LICENSE-MIT` + `LICENSE-APACHE` at the root, LF. Set once at `Cargo.toml [workspace.package] license`; every member inherits via `license.workspace = true`, none overrides. The npm manifest and its lock root carry it. Homebrew `license any_of: ["MIT", "Apache-2.0"]`, Scoop `"license": "MIT|Apache-2.0"`. Tauri `bundle.license` inherits the Cargo value. cargo-deny checks own crates. Pinned by the test-only `xtask` module `license_check`. Out of the decision: per-file headers, a NOTICE bundle, `authors`.
- Counts: was "twelve library crates … fourteen workspace members total"; now fourteen library crates, sixteen members (§Occupied Resources stays canonical). Runtime topology: was "all twelve library crates"; now fourteen.
- xtask CLI surfaces: was "rides the slot that runs LAST in every gate list"; now the `capability-drift` slot, which runs BEFORE the default-features workspace nextest since 2026-09-30.
**Why:** the founder's license decision of 2026-09-29 (the repository public that day), in the Conductor `cdb7082` shape; the counts had gone stale against the 16-member list; the gate order moved by the overseer's founder-delegated directive (test-plan §3).
**Kept:** "12-module" inside the [Backend Framework] / [Tauri IPC Bridge] / [Module Boundaries] rationales — decision-time reasoning, not a current count.
**Ref:** .andromeda/runs/2026-09-30T07-44-36Z-wrap/
