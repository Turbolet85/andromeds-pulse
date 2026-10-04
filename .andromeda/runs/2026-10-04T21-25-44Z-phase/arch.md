# arch extract

## Relevance
relevant: architecture.md names this route entry as the owner of the stale declared floor in three places (§Stack and Technologies, Primary language row; §Established Decisions [Primary Language]; §Inherited Defaults, Language / runtime). §Standard Contracts `app_info` carries the `rust_version` field, and §Occupied Resources → Filesystem locations governs the lock file the CARRY touches.

## Constraints
- The toolchain stays pinned at rustc 1.95.0 (`rust-toolchain.toml`), and the raised declared floor must sit at or below that pin. Per arch §Stack and Technologies (Primary language row) and §Infrastructure Patterns → Project directory structure (`rust-toolchain.toml  # pin rustc 1.95.0`), this chunk is not a toolchain bump.
- Arch records the code floor as rustc ≥ 1.89 (`std::fs::File::lock`; let-chains need 1.88) and calls `rust-version = "1.85"` stale, per arch §Stack and Technologies, §Established Decisions [Primary Language] and §Inherited Defaults. Arch got that floor from a clippy `incompatible_msrv` measurement. Whether ≥ 1.89 is the TRUE floor is research's question, because it also depends on the newest language feature the workspace uses and on every resolved dependency's declared `rust-version`.
- No dependency upgrade comes with the raise. wasmtime stays at requirement `"48.0.4"` (resolved 48.0.5), and 49.x is out of reach while the toolchain is pinned at 1.95, because 49.0.2 needs Rust 1.96. Per arch §Stack and Technologies (Plugin runtime row) and §Inherited Defaults (Plugin runtime).
- The `app_info` envelope carries `rust_version`. Its example reads `"1.84.0"`, a three-component string, while the declared floor is two-component (`"1.85"`). Per arch §Standard Contracts (Tauri IPC introspection — `app_info`), the example is a wrap amendment. Which value the code actually feeds into that field, and in what shape, is research's question.
- The corpus-key lock file is content-free, held only for the get → generate → set → read-back sequence, and NEVER deleted, because deleting it after unlock would let a later opener lock a different inode than a waiter. Its location is `$XDG_RUNTIME_DIR` on Linux and otherwise the canonicalized `std::env::temp_dir()`. Per arch §Occupied Resources → Filesystem locations → Out-of-data-dir lock file, the CARRY ("the skip arm leaves no lock file") must be met without weakening the product's never-delete rule.
- The lock-file bullet records that "No `ANDROMEDA_PULSE_*` variable was added". A new lock-dir override variable would be a new reserved resource and would come under the path-env-var canonicalize-and-confine rule. Per arch §Occupied Resources → Environment variables and → Filesystem locations → Out-of-data-dir lock file.
- The lint gate stays `cargo clippy --workspace --all-targets --all-features -- -D warnings`. Removing the `incompatible_msrv` allow must keep that gate green with every feature enabled, including `mcp-server`. Per arch §Infrastructure Patterns → Build system.

## Patterns to follow
- All 16 reserved workspace members inherit one declared floor from the workspace manifest (the scope's `rust-version.workspace = true`), so the raise is a single workspace-level edit and no member gets a per-crate floor. Per arch §Occupied Resources → Cargo workspace crate names and §Inherited Defaults (Module boundaries).
- Any floor witness that is added goes into the `xtask` crate, so contributors run the same command locally as `cargo xtask <task>`. Per arch §Infrastructure Patterns → CI/CD approach ("All shared CI logic that needs Rust lives in the `xtask` crate") and §Cross-cutting Patterns → Development Style (deterministic, machine-parseable harness invocations).
- A new xtask verb follows the formalized CLI contract: a stated exit-code contract (0 / 1 / 2 shape), named verdict arms, and an entry in the xtask CLI surfaces list. Per arch §Occupied Resources → xtask CLI surfaces.
- A CI-wired witness changes the documented job roster, which is "seven independent jobs" with per-job cache ownership under the 10 GB cap that arch records as a watch. A new job, or new cache keys, must be weighed against that headroom. Per arch §Infrastructure Patterns → CI/CD approach.

## Anti-patterns to avoid
- Do not delete the corpus-key lock file on any production path, including error and skip paths, to satisfy the CARRY. Arch bans deleting it (§Occupied Resources → Filesystem locations → Out-of-data-dir lock file). The fix belongs to the test's own lock-dir footprint, and how that is done is research's and P4's question.
- Do not change `rust-toolchain.toml`'s channel or bump any dependency, wasmtime included, as part of the raise (arch §Stack and Technologies; §Inherited Defaults).
- Do not declare a floor above what the pinned 1.95.0 toolchain satisfies, or above what has been measured. Arch's ≥ 1.89 is a lower bound that clippy measured, not a derived value (arch §Established Decisions [Primary Language]).

## Contract bindings
- arch §Standard Contracts `app_info.rust_version` ↔ obs: the scope's inferred `pulse-app/src/observability.rs` `rust_version` field.
- arch §Standard Contracts `app_info.rust_version` ↔ tests: the scope's inferred `"1.85"` fixture in `crates/ui-bridge/src/contract.rs`. If the envelope value derives from the declared floor, the raise flows into both.
- arch §Occupied Resources → Out-of-data-dir lock file ↔ security: the ratified XDG_RUNTIME_DIR / temp-dir carve-out in security-plan §Security Anti-Patterns → Input and `.claude/rules/security.md`.
- arch §Occupied Resources → Out-of-data-dir lock file ↔ tests: the skip arm of `corpus_key_survives_a_real_process_boundary`.
- The arch floor statement ↔ security: security-plan §Security Anti-Patterns → Universal ("NEVER let the rust-toolchain drift below 1.85.0") and `.claude/rules/security.md` §Rust toolchain ("MUST pin to `1.85.0` minimum"). These must move in lockstep with arch at wrap.
- A floor witness, if added ↔ tests/CI: arch §Infrastructure Patterns → CI/CD approach (the job roster) and §Occupied Resources → xtask CLI surfaces.

## Acceptance criteria contributions
- The workspace `rust-version` equals the floor derived at research (≥ the newest std API or language feature the code uses, ≥ every resolved dependency's declared `rust-version`, and ≤ 1.95.0). `rust-toolchain.toml` stays `channel = "1.95.0"`, and no dependency requirement changes. (per arch §Stack and Technologies, Primary language row; §Established Decisions [Primary Language])
- After the raise, the workspace builds and `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes with the `incompatible_msrv` allow removed. (per arch §Infrastructure Patterns → Build system)
- The skip arm of `corpus_key_survives_a_real_process_boundary` leaves no lock file in the system lock dir. The production lock-file contract does not change: same name, same location resolution, still never deleted. (per arch §Occupied Resources → Filesystem locations → Out-of-data-dir lock file)
- Wrap amends arch to the raised floor in four places: the Stack row, [Primary Language] and Inherited Defaults lose their "declared 1.85 is stale" clause, and the §Standard Contracts `app_info` example `rust_version` is updated. Any new xtask verb is registered under §Occupied Resources → xtask CLI surfaces. (per arch §Stack and Technologies; §Established Decisions; §Inherited Defaults; §Standard Contracts; §Occupied Resources)
