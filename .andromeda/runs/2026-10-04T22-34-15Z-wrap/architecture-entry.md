
## 2026-10-04-declared-rust-floor-matches-the-code — the declared Rust floor is 1.95, equal to the pin
**Section:** §Stack and Technologies → Primary language / runtime · §Established Decisions → [Primary Language] · §Inherited Defaults → Language / runtime · §Standard Contracts → `app_info` example
**Change:**
- Was "the workspace's declared `rust-version = "1.85"` stale and owned by the route entry 'The declared Rust floor matches the code'", per "2026-10-04-corpus-key-creation-is-race-free — the stated Rust floor matches the code"; now the workspace declares `rust-version = "1.95"` once in `[workspace.package]`, all 16 members inherit it.
- The floor is set by the resolved dependency graph: 28 packages declare `rust-version = "1.95.0"` (wasmtime 48.0.5 and its internal crates, cranelift 0.135.5, pulley 48.0.5), so no toolchain below 1.95.0 builds the product. rustc ≥ 1.89 (`std::fs::File::lock`; let-chains 1.88) stays, restated as the workspace's OWN-code bound, never the floor.
- The xtask test `declared_floor_equals_the_pinned_channel` holds the declared floor equal to the pinned channel at major.minor, so every pinned-toolchain build proves it.
- `app_info` example `"rust_version": "1.84.0"` → `"1.95"` — the field is `env!("CARGO_PKG_RUST_VERSION")`, the declared string verbatim; this settles the value question that entry's Kept left to this route entry.
**Why:** This chunk is the route entry those clauses deferred to. The floor witness's strength ("declared == pinned channel", over "≥ every dependency" and "none") was chosen by the overseer (founder-delegated, 2026-10-04) at P4: an overstated floor costs nobody, because Pulse ships an app, not a published crate.
**Kept:** `rust-toolchain.toml` `channel = "1.95.0"` and every dependency unchanged (wasmtime 49 needs Rust 1.96).
**Ref:** .andromeda/runs/2026-10-04T22-34-15Z-wrap/
