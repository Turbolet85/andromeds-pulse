# tests extract

## Relevance
partial — a dependency bump with no new test surface; the tests domain gives it the standard per-chunk gate set, the plugins regression net, and the CI supply-chain / Cranelift gates it must pass.

## Constraints
- The chunk plan's `## Test Commands` MUST carry the unconditional standard gate set (fmt · clippy `--workspace --all-targets --all-features -D warnings` · `capability-widening-check` · `check:ingest-progress` · `check:staged-artifacts` · `capability-drift` BEFORE the workspace nextest · `cargo nextest run --workspace --profile ci` · the `--features mcp-server` `emit_taurpc_bindings` regen as the last cargo-adjacent step), per test-plan §3 Per-chunk gate discipline. The webview gates drop out only if the Files-to-modify / New-files lists touch zero `pulse-app/ui/**` paths (same section); the boot-smoke gate is conditional on a boot/setup path being touched, and none is in scope (same section, Boot-smoke gate).
- A chunk adding no TauRPC procedure closes the bindings sequence with `git diff --quiet <chunk-base> -- pulse-app/ui/src/bindings/index.ts` (exit 0 = byte-identical to base), per test-plan §3 Per-chunk gate discipline (the capability-drift ordering paragraph).
- `cargo deny check bans` is never skipped (the tonic 0.14 / 0.13 canary), per test-plan §11 Test Strategy; a deny `bans` violation is a CI build-failure condition, per test-plan §9 Build failure conditions and §9 Cargo deny enforcement (`multiple-versions = "deny"`).
- The Cranelift-only WASM backend assertion (`cargo tree -p wasmtime | grep -q cranelift`) is a named `run:` step of the ci.yml `supply-chain` job and must stay green across the wasmtime family move, per test-plan §9 Cranelift-only WASM enforcement and the §1 Pending coverage triggers row `security-vector-coverage: Cranelift-only WASM`. Whether that step runs before or after `rustsec/audit-check` inside the job (and so whether the red run measured it at all) is research's question.
- The plugins crate's regression net is its unit tests (Component Model loading, WIT binding validation, the disallowed-import negative test) per test-plan §4 (plugins crate bullet), plus the plugins → runtime integration boundary (fixture WASM from `tests/fixtures/plugins/`, `wasmtime::component`, unauthorized-import negative test) per test-plan §5 Boundary types covered. Whether those tests exist and pass at 48.0.4 is research's / implement's question.
- Coverage stays at or above Standard tier (≥ 75 % line / ≥ 70 % branch / ≥ 85 % function, `xtask/` excluded temporarily) and is never lowered to pass, per test-plan §10 Coverage thresholds and §11 CI.
- Match the scope of a narrowed run to the scope of the build: a `-p` selection must not be assumed compatible with artifacts a `--workspace` build produced (measured on `pulse-app` with `wasmtime_internal_cache` among the rlib-format failures), per test-plan §11 Project-specific (the `--filter-expr` under `--workspace` bullet).

## Patterns to follow
- Narrow to the plugins tests with `cargo nextest run --workspace --profile ci -E 'package(plugins)'` (filter under `--workspace`) rather than a bare `-p` against a workspace build, per test-plan §11 Project-specific.
- WASM fixtures are pre-compiled minimal Component Model `.wasm` binaries committed under `tests/fixtures/plugins/`, shared read-only per suite — the bump reuses them unchanged, per test-plan §7 (WASM fixtures row).
- Run `capability-drift` before the default-features workspace nextest, then the mcp-server regen, then the base-diff close — the sequence for a chunk that adds no procedure, per test-plan §3 Per-chunk gate discipline.
- The CI `supply-chain` job carries the Linux workspace release build (`cargo auditable build --workspace --release`), so the bumped family must build there in release, per test-plan §9 Pipeline structure (Supply chain row and Release build row).

## Anti-patterns to avoid
- NEVER skip `cargo deny check bans`, per test-plan §11 Test Strategy.
- NEVER lower a coverage threshold or quarantine a plugins test to get green; a flake is a real bug, per test-plan §11 CI and §11 Quality (no `#[ignore]` without an open issue, no retry-once).
- NEVER ship with a known test failure; all gates green, per test-plan §11 Quality.

## Contract bindings
- tests ↔ security: the CI `supply-chain` job (audit + deny + auditable + the Cranelift assertion) is where test-plan §9 Pipeline structure (Supply chain row) and §9 Cargo deny enforcement meet security-plan §Dependency Security. The scope's outcome gate (`cargo audit` exit 0, `cargo deny check bans licenses sources` green, `advisories` naming no `wasmtime` id) is security's to define; the tests plan owns only the CI wiring and the build-failure conditions.
- tests ↔ plan text: test-plan §5 Boundary types covered (plugins → runtime row) carries the coordinate "`wasmtime` 48.x (lockfile-resolved 48.0.3 as of 2026-09-29)"; a move to 48.0.4 makes that coordinate stale, so it is a test-plan amendment candidate at wrap (the row's "48.x" family statement still holds).

## Acceptance criteria contributions
- `cargo nextest run --workspace --profile ci` passes on the bumped lockfile, including the plugins unit tests and the plugins → runtime integration tests with their unauthorized-import negative arms (per test-plan §3 Per-chunk gate discipline; §4 plugins crate; §5 Boundary types covered).
- The standard gate set is green, ending with the bindings close `git diff --quiet <chunk-base> -- pulse-app/ui/src/bindings/index.ts` at exit 0 (per test-plan §3 Per-chunk gate discipline).
- The Cranelift-only WASM assertion `cargo tree -p wasmtime | grep -q cranelift` passes locally on the bumped graph and as its ci.yml `supply-chain` step on the chunk's own run (per test-plan §9 Cranelift-only WASM enforcement).
- Coverage on the chunk's CI `coverage` job stays at or above 75 % line / 70 % branch / 85 % function (per test-plan §10 Coverage thresholds).
