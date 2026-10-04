# arch extract

## Relevance
partial — a dependency-version move inside the locked plugin runtime; no new surface, crate, contract or resource, but three arch rows pin the version this chunk changes.

## Constraints
- The plugin runtime stays `wasmtime` with the WASM Component Model + WIT, capability-scoped. The move is a version change inside that locked decision, not a runtime swap (per architecture §Established Decisions → [Plugin Runtime]; §Stack and Technologies → Plugin runtime row).
- The dependency lives where arch places it: the workspace manifest pins the version, and the `plugins` crate (a reserved workspace crate name) is the only consumer arch assigns to the wasmtime Component Model host. The chunk adds no crate, renames none, and adds no new sibling-dependency edge (per architecture §Occupied Resources → Cargo workspace crate names; §Infrastructure Patterns → Project directory structure; §Cross-cutting Patterns → Module dependency direction).
- 49.x is out of reach while the toolchain is pinned at 1.95. Arch records that "49.0.1 needs Rust 1.96" as the reason, so the target stays inside 48.x and the toolchain is not moved (per architecture §Stack and Technologies → Plugin runtime row). Arch states 49.0.1, while the scope's patched range starts at `>=49.0.2`. That does not change the outcome, but the rustc requirement of 49.0.2 specifically is research's to confirm, not arch's.
- Feature-gate hygiene holds. The bump must not pull optional or flag-specific dependencies into the default build graph, and the `component-model` feature set on the pin carries forward unchanged (per architecture §Cross-cutting Patterns → Feature-gate hygiene).
- The CI topology is fixed. `supply-chain` is one of the seven independent `ci.yml` jobs, and its `cargo auditable build --workspace --release` is the Linux release smoke, so the bump must build in release under `cargo auditable` as well as under the test profile (per architecture §Infrastructure Patterns → CI/CD approach).
- The chunk adds no TauRPC procedure, port, env var, corpus table or capability, so §Occupied Resources gains no entry (per architecture §Occupied Resources; §Cross-cutting Patterns → Webview IPC capability policy).

## Patterns to follow
- **The version-pin record shape.** Arch records each heavy dependency as `requirement "X"`, lockfile-resolved **Y** as of {date}, plus the advisory the bump closed and the reason the next major is out of reach. The wasmtime row's predecessor entry ("the bump from 46.0.3 closed RUSTSEC-2026-0316") is the template for recording 48.0.3 → 48.0.4 closing RUSTSEC-2026-0325 / -0326 / -0327 (per architecture §Stack and Technologies → Plugin runtime row; the DuckDB and `rmcp` rows follow the same shape).
- **Dependency upgrades go through workspace inheritance.** The version moves at the workspace manifest and the consuming crate keeps `workspace = true`. Per-crate version overrides are not the pattern (per architecture §Inherited Defaults → Module boundaries / Build).
- **Shared CI logic is reproducible locally.** The gate verdicts the chunk claims (audit / deny / auditable build) should be reproducible on the dev host before the CI round reads them (per architecture §Infrastructure Patterns → CI/CD approach, the final bullet on `xtask` local parity).

## Anti-patterns to avoid
- Moving the plugin runtime off Component Model/WIT, for example to Extism, or bumping the toolchain to reach 49.x. Both contradict a locked decision and the recorded out-of-reach reason (per architecture §Established Decisions → [Plugin Runtime]; §Stack and Technologies → Plugin runtime row).
- Adding a per-crate wasmtime version or a second wasmtime consumer outside `plugins`, which breaks single-source workspace pinning and the module DAG (per architecture §Cross-cutting Patterns → Module dependency direction).

## Contract bindings
- **arch ↔ security (supply chain).** Security owns the gate semantics: no advisory ignore, `cargo audit` pass/fail, `multiple-versions = "deny"`, and the wasmtime sandbox posture (Cranelift, `epoch_interruption`, `ResourceLimiter`). Arch owns the recorded version pin. Each wrap must leave the two agreeing on the resolved version.
- **arch ↔ wrap amendment.** Three arch sites state requirement `"48.0.3"` / lockfile-resolved 48.0.3 as fact: §Stack and Technologies (Plugin runtime row), §Established Decisions ([Plugin Runtime] heading), and §Inherited Defaults (Plugin runtime bullet). The repo-level `CLAUDE.md` Stack paragraph repeats the same pin. All of them become stale the moment the lockfile moves, so they are a wrap-time amendment obligation for this chunk.
- **arch ↔ tests (CI).** The `supply-chain` job's release smoke is the arch-recorded CI topology that the chunk's own CI round must read green.

## Acceptance criteria contributions
- `wasmtime` remains the Component Model plugin runtime, consumed only by the `plugins` crate via workspace inheritance. No new crate, crate rename or sibling-dependency edge is introduced (per architecture §Established Decisions → [Plugin Runtime]; §Cross-cutting Patterns → Module dependency direction).
- The resolved wasmtime version stays inside the 48.x major and the toolchain pin is untouched (per architecture §Stack and Technologies → Plugin runtime row).
- §Occupied Resources gains no entry: no new procedure, port, env var, table or capability (per architecture §Occupied Resources).
- At wrap, all three arch pin sites read the new requirement and lockfile-resolved version with its date, and the Stack row names the three RUSTSEC ids the bump closed in the existing "the bump from X closed Y" shape. No site is left reading 48.0.3 (per architecture §Stack and Technologies; §Established Decisions → [Plugin Runtime]; §Inherited Defaults → Plugin runtime).
