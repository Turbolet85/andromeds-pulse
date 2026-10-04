# Lockfile delta — 2026-10-04-supply-chain-advisories-on-wasmtime-resolved

Read 2026-10-04 with `git diff 5988a5f80b7280081fdeb12d9eca16bfa32f4de9 -- Cargo.lock` (the chunk base), after
`Cargo.toml` moved the wasmtime requirement `"48.0.3"` -> `"48.0.4"` and `cargo update -p wasmtime` ran (no
`--precise`). Cargo printed `Locking 35 packages to latest compatible versions` and
`wasmtime v48.0.3 -> v48.0.5 (available: v49.0.2, requires Rust 1.96.0)`.

## Shape
- `Cargo.lock | 180 lines, 90 insertions(+), 90 deletions(-)`.
- 70 `version` lines (35 packages x 2) plus 70 `checksum` lines: one version move and one checksum move per package.
- 20 more changed lines (10 out, 10 in) are dependency-list entries that name a version
  (`"wasmparser 0.254.0"` -> `"wasmparser 0.254.2"`, likewise `wasm-encoder` / `wasm-metadata` / `wit-component` /
  `wit-parser`). Cargo writes the version into a dependency entry when that crate appears at more than one version in
  the lockfile, so these lines follow the same package moves; none names a package outside the family.
- `name` lines added or removed: 0. No package was added to or removed from the lockfile.

## Packages (35, all in the wasmtime family)
| package | before | after |
|---|---|---|
| cranelift-assembler-x64 | 0.135.3 | 0.135.5 |
| cranelift-assembler-x64-meta | 0.135.3 | 0.135.5 |
| cranelift-bforest | 0.135.3 | 0.135.5 |
| cranelift-bitset | 0.135.3 | 0.135.5 |
| cranelift-codegen | 0.135.3 | 0.135.5 |
| cranelift-codegen-meta | 0.135.3 | 0.135.5 |
| cranelift-codegen-shared | 0.135.3 | 0.135.5 |
| cranelift-control | 0.135.3 | 0.135.5 |
| cranelift-entity | 0.135.3 | 0.135.5 |
| cranelift-frontend | 0.135.3 | 0.135.5 |
| cranelift-isle | 0.135.3 | 0.135.5 |
| cranelift-native | 0.135.3 | 0.135.5 |
| cranelift-srcgen | 0.135.3 | 0.135.5 |
| pulley-interpreter | 48.0.3 | 48.0.5 |
| pulley-macros | 48.0.3 | 48.0.5 |
| wasm-compose | 0.254.0 | 0.254.2 |
| wasm-encoder | 0.254.0 | 0.254.2 |
| wasm-metadata | 0.254.0 | 0.254.2 |
| wasmparser | 0.254.0 | 0.254.2 |
| wasmprinter | 0.254.0 | 0.254.2 |
| wasmtime | 48.0.3 | 48.0.5 |
| wasmtime-environ | 48.0.3 | 48.0.5 |
| wasmtime-internal-cache | 48.0.3 | 48.0.5 |
| wasmtime-internal-component-macro | 48.0.3 | 48.0.5 |
| wasmtime-internal-component-util | 48.0.3 | 48.0.5 |
| wasmtime-internal-core | 48.0.3 | 48.0.5 |
| wasmtime-internal-cranelift | 48.0.3 | 48.0.5 |
| wasmtime-internal-fiber | 48.0.3 | 48.0.5 |
| wasmtime-internal-jit-debug | 48.0.3 | 48.0.5 |
| wasmtime-internal-jit-icache-coherence | 48.0.3 | 48.0.5 |
| wasmtime-internal-unwinder | 48.0.3 | 48.0.5 |
| wasmtime-internal-versioned-export-macros | 48.0.3 | 48.0.5 |
| wasmtime-internal-wit-bindgen | 48.0.3 | 48.0.5 |
| wit-component | 0.254.0 | 0.254.2 |
| wit-parser | 0.254.0 | 0.254.2 |

The set is the one research measured in its scratch worktree (research.md §Measured), package for package. Five of these crates
sit in the lockfile at more than one version (after the move: `wasm-encoder` and `wasmparser` at 0.244.0 / 0.254.2 /
0.258.0; `wit-parser`, `wasm-metadata` and `wit-component` at 0.244.0 / 0.254.2). Only the 0.254.0 copy of each
moved; the other copies are untouched.

The plan's lockfile family guard and no-added-package guard read this same diff; their verdicts are in this
chunk's implement gate run.
