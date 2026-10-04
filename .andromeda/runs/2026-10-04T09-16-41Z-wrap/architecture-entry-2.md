
## 2026-10-04-corpus-key-creation-is-race-free — the stated Rust floor matches the code
**Section:** §Stack and Technologies → Primary language / runtime · §Established Decisions → [Primary Language] · §Infrastructure Patterns → directory tree (`rust-toolchain.toml` comment) · §Inherited Defaults → Language / runtime
**Change:** was "Rust 2024 edition (rustc 1.84+)" at all four sites; now: toolchain pinned to rustc 1.95.0 (`rust-toolchain.toml`), code floor rustc ≥ 1.89 (`std::fs::File::lock`; let-chains already needed 1.88), the workspace's declared `rust-version = "1.85"` stale and owned by the route entry "The declared Rust floor matches the code".
**Why:** clippy `incompatible_msrv` measured the declared floor false at this chunk's first gate run; the founder ruled the Cargo.toml raise its own route entry (2026-10-04), so the masters state the truth and its owner meanwhile.
**Kept:** §Standard Contracts `app_info` example `"rust_version": "1.84.0"` — illustrative; what `app_info` reports was not measured, so the value question rides the same route entry.
**Ref:** .andromeda/runs/2026-10-04T09-16-41Z-wrap/
