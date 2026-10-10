# Commands Reference

Full command surface for andromeda-pulse development. Read on demand.

## Lint + format
```bash
cargo fmt --check                                              # Verify formatting (CI gate)
cargo fmt                                                      # Apply formatting
cargo clippy --workspace --all-targets --all-features -- -D warnings   # Lint gate (CI denies warnings)
```

## Typecheck
```bash
cargo check --workspace --all-targets                          # Implicit typecheck
cd pulse-app/ui && tsc --noEmit                                # Webview TypeScript bindings (TauRPC-generated .d.ts)
```

## Tests
```bash
# Single crate
cargo nextest run --filter-expr 'package(ingest)'

# All workspace
cargo nextest run --workspace --profile ci --message-format libtest-json

# Coverage (LCOV)
cargo llvm-cov nextest --workspace --lcov --output-path lcov.info --summary-only

# Coverage (Cobertura for CI tooling)
cargo llvm-cov nextest --workspace --cobertura --output-path cobertura.xml

# Integration only
cargo nextest run --workspace --profile ci --filter-expr 'integration'

# E2E only
cargo nextest run --workspace --profile ci --filter-expr 'e2e'

# Property tests with regression replay
cargo nextest run --workspace --filter-expr 'proptest'

# A11y suite
cargo xtask test:a11y                                          # Reuses E2E driver per binding contract
# OR
cd pulse-app/ui && npm run test:a11y
```

## Build
```bash
# Dev build
cargo build --workspace

# Release smoke build
cargo build --workspace --release

# Tauri desktop bundle (current platform)
cargo tauri build

# xtask shortcuts
cargo xtask release            # tauri-action invocation: builds .msi/.dmg/.AppImage/.deb
cargo xtask sign               # Code signing (Azure Key Vault EV + Apple Developer ID)
cargo xtask notarize           # macOS notarization
```

## Run locally
```bash
# Run pulse-app (Tauri dev mode with hot reload)
cargo tauri dev

# Or direct
cargo run --bin pulse-app

# With MCP feature
cargo run --bin pulse-app --features mcp-server

# Console engine — the package's second program: same engine boot, no window, no display, no model runner
cargo build --bin andromeda-pulse-engine --release
target/release/andromeda-pulse-engine run        # stays up until SIGTERM / SIGINT; binds 4317 / 4318 by default and writes the same pid file as pulse-app
target/release/andromeda-pulse-engine version    # one line, exit 0; any other command line is usage, exit 2

# Override env
ANDROMEDA_PULSE_DATA_DIR=$TMPDIR/dev \
ANDROMEDA_PULSE_LOG_LEVEL=debug \
ANDROMEDA_PULSE_OTLP_GRPC_PORT=14317 \
ANDROMEDA_PULSE_OTLP_HTTP_PORT=14318 \
cargo run --bin pulse-app
```

## Agent-run harness (5-command discipline)
```bash
./scripts/agent-run.sh boot       # Start the window app (boot engine, sh only: the console engine andromeda-pulse-engine run; any other word is usage, exit 2), await ready via harness:ready verdict — status running-healthy for the program asked AND both OTLP ports accepting (10s default; HARNESS_STATUS_TIMEOUT overrides)
./scripts/agent-run.sh run        # Execute test suite
./scripts/agent-run.sh status     # cargo xtask harness:status --program window (status engine, sh only: --program console) — real-process verdict JSON, exit 0 running-healthy / 1 stale, wrong-program or not-running / 2 cannot-evaluate
./scripts/agent-run.sh cleanup    # SIGTERM + verify ports released
./scripts/agent-run.sh logs       # Tail JSON log file
```
PowerShell variant: `.\scripts\agent-run.ps1 boot|run|status|cleanup|logs` (it takes no `engine` word and passes no `--program` flag).

## Supply chain + security gates
```bash
cargo audit                                                    # RustSec advisory check
cargo deny check bans licenses sources                         # Duplicate / license / source policy
cargo deny check advisories                                    # Same as cargo audit but via deny
cargo xtask check:npm-supply-chain                             # npm advisory/license/ban gate (pulse-app/ui; policy npm-policy.json; lockfile-only)
cargo xtask harness:status [--program window|console]          # Real-process status verdict JSON {verdict,pid,ended,program,log_file_basename,last_write_age_seconds,stale_after_seconds}; program = the log family's last app.boot.engine record (window | console | unknown); with --program, a run of the other program, or of none, reads wrong-program; exit 0 running-healthy / 1 stale, wrong-program or not-running / 2 cannot-evaluate
cargo xtask harness:ready [--program window|console]           # Boot readiness verdict JSON {verdict,pid,ended,program,otlp_grpc,otlp_http}: status running-healthy for the program asked AND a TCP connection accepted on both OTLP ports; exit 0 ready / 1 not-ready, wrong-program or ended / 2 cannot-evaluate
cargo xtask harness:settled [--timeout-seconds N]              # Settle verdict JSON {verdict,pid,ended,app_exit_record,windows_settled,display,session_bus,exit_witness}, also written to logs/harness-settled.json; exit_witness is one label (unset, unreadable, loaded, exit-call, runtime-exit, no-record) read from logs/exit-witness.jsonl; exit 0 settled / 1 ended or not-settled / 2 cannot-evaluate (default 30 s, refused below 8)
cargo xtask harness:boot-series --count N                      # Linux: N more boots (1 to 16) after the CI smoke, each on its own data dir series/boot-{ordinal}/ and display server (xvfb-run), through boot, harness:settled, status, cleanup; verdict JSON {verdict,boots,settled,ended,other,per_boot}, also written to logs/boot-series.json; exit 0 all-settled / 1 self-ended or not-all-settled / 2 cannot-evaluate. A boot that ended by itself before ready (no settle verdict) is counted ended with its exit record and witness label, read from its own data dir; the smoke's own boot is listed the same way as ordinal 1. Binds the two OTLP ports the environment resolves: on the dev host set ANDROMEDA_PULSE_OTLP_GRPC_PORT / _HTTP_PORT off 4317 / 4318 first
cargo xtask harness:engine-settled [--timeout-seconds N]       # Console engine settle verdict JSON {verdict,pid,program,ingest_ticks,buffer_ticks,connection_ticks,memory_samples_populated}: boot record reads console, pid alive, two records of each of ingest.tick / buffer.tick / connection.tick and one non-zero metric.buffer.memory_bytes sample; exit 0 settled / 1 ended, wrong-program or not-settled / 2 cannot-evaluate (default 60 s, refused below 20); writes no file; harness:settled stays the window's
cargo xtask check:engine-log                                   # Console engine log check, no argument (paths resolved as harness:status does): seven arms family, program, panic, heartbeat-gap (over 45 000 ms between consecutive ingest.tick / buffer.tick / connection.tick records), progress, process-end, budget (memory max 512 000 000 B); one line per arm, then engine-log: PASS | FAIL | cannot-evaluate; exit 0 / 1 / 2, a FAIL on any arm outranks a cannot-evaluate
cargo xtask harness:engine-cycle [--data-dir DIR] [--grpc-port N] [--http-port N]   # Linux: one whole run of the console engine — injector build, agent-run.sh boot engine, inject_demo --sustained --error-pct=0, harness:engine-settled, status engine, cleanup, check:engine-log — every child under a cleared environment; defaults target/engine-cycle/{UTC second} and harness-only ports 24317 / 24318 (4317 / 4318 refused: shared-port); verdict JSON {verdict,boot,settled,status,cleanup,check,error_records,witness_file}; exit 0 pass / 1 fail / 2 cannot-evaluate; CI: the boot job's Console engine cycle step
cargo xtask pre-push:linux                                     # Linux dev host, native: six Linux-reachable stages (script-modes, source-lint, npm, clippy, test, ci-gates) in the working tree under a constructed environment; needs ci.yml's Node major first on PATH (dev host: d="$(mise where node@24)" && PATH="$d/bin:$PATH" …); exit 0 green / 1 red / 2 cannot-evaluate
cargo xtask check:english-sources                              # English-only source lint (crates, pulse-app/src+tests+ui/src, xtask/src); ASCII ::error annotations; exit 0 clean / 1 findings / 2 cannot-evaluate
cargo xtask check:staged-artifacts                             # Staged git-index bindings + capability grants vs EXPECTED_PROCEDURES/EXPECTED_GRANTS; exit 0 staged-clean / 1 staged-drift / 2 cannot-evaluate
cargo xtask verify:capability-matrix                           # The capability record's gate: docs/capability-record.json (82 ids, each claimed or retired) against the working route; exit 0 clean / 1 findings / 2 cannot-evaluate
cargo nextest run --workspace --profile perf-samples --no-tests=fail   # The in-process perf-sample producer; CI's spelling: a selection that matches no test fails (writes target/tmp/perf-budget-samples/)
cargo xtask perf:budget --data-dir target/tmp/perf-budget-samples --require memory,snapshot  # Perf-budget grader over <DIR>/logs/agent-latest.jsonl*; exit 0 PASS / 1 FAIL / 2 cannot-evaluate
cargo xtask perf:frame-sample                                  # Windows dev host only: frame p99 ≤ 33 ms under a software WebGPU adapter; exit 0 PASS / 1 FAIL / 2 INCONCLUSIVE (opens a window)

# CI-side
gitleaks detect --redact                                       # Secret scanning (pre-commit + CI)

# xtask wrappers
cargo xtask audit                                              # Wraps cargo audit
cargo xtask deny-bans                                          # Wraps cargo deny check bans
cargo xtask capability-drift                                   # Diff TauRPC procedures (worktree bindings) vs EXPECTED_PROCEDURES + run the staged-artifacts assertion (since 2026-08-30)
```

## Tooling install (Bootstrap phase install commands)
```bash
# Rust toolchain: rust-toolchain.toml pins 1.95.0 — rustup installs and selects it automatically on first cargo call
rustup show active-toolchain

# Test runner
cargo install cargo-nextest

# Coverage
cargo install cargo-llvm-cov

# Supply chain
cargo install cargo-audit cargo-deny cargo-auditable

# Dependency tree (for living artifact reconcile)
cargo install cargo-modules

# API surface (for living artifact reconcile)
cargo install cargo-public-api

# Tauri CLI (driver + bundler)
cargo install tauri-cli --version "^2"

# A11y stack (webview-side)
cd pulse-app/ui && npm install --save-dev \
  @axe-core/playwright@4.11 \
  lighthouse@13 \
  pa11y@10 pa11y-ci@4 \
  eslint-plugin-jsx-a11y@6.10 \
  colorjs.io@0.6
```

## Performance + profiling
```bash
# No bench suite exists (no criterion dependency, no `xtask bench` verb; measured 2026-10-10).
# The perf budgets are graded over log samples:
cargo xtask perf:budget --data-dir <DIR> --require memory,snapshot

# Profiling
cargo flamegraph --bin pulse-app
samply record cargo run --bin pulse-app --release   # macOS / Linux
```

## Release pipeline (CI-only — manual approval gated)
- `release.yml` runs on tag push (`v*`): tauri-action builds matrix → Azure Key Vault EV signing → Apple Developer ID notarization → upload bundles + `latest.json` to GitHub Releases.
- `update-channels.yml` runs on `release.yml` completion: updates Homebrew tap + Scoop manifest with new version + sha256.

## Git
- See `.claude/docs/workflow.md` for branch / commit / PR conventions.

## Code-graph (symbol graph — replaces the retired markdown living-trees)
Derived + gitignored under `.andromeda/cache/`, **one DuckDB database per language PLANE** (`rust` · `ts`), each at `cache/{plane}/tree.db`. Planes are detected from manifests at run time: `rust` = root `Cargo.toml` (+ `rust-analyzer`); `ts` = tracked `tsconfig.json` (+ `scip-typescript`). Both are live in this project.

```bash
pip install -r scripts/requirements.txt          # one-time: duckdb + protobuf
python scripts/code-graph.py refresh             # build EVERY detected plane
python scripts/code-graph.py refresh ts          # or just one plane
python scripts/code-graph.py query <run_dir> <marker> "<sql>" <plane>
```
- `refresh` is backgrounded by `/wrap-session`; `/andromeda-phase` calls `query`, which regenerates only the plane it needs on a miss. **Do not run `refresh` by hand** — the next wrap or a phase query does it.
- `plane` is REQUIRED on `query` when several planes are detected (both are here) — the chunk's modify-set says which.
- A missing indexer skips that plane (recipe in `.andromeda/cache/.refresh-done`) and never blocks; if no plane can build you get `.refresh-stale`, exit 0.
- Views: `symbol` / `refs` / `calls` / `contains` / `crate_edges` / `calls_m`. Schema + canonical query shapes: `scripts/code-graph-cookbook.md` (authoritative definitions in `scripts/code-graph-views.sql`).
