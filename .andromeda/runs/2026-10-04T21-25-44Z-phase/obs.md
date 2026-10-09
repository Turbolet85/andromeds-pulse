# obs extract

## Relevance
partial — a build-declaration chunk with no planned instrumentation; obs is touched only where the declared floor feeds a self-observation or identity value (`CARGO_PKG_RUST_VERSION` → `app_info.rust_version`, and the `rust_version` field named at `pulse-app/src/observability.rs:505`), and by the corpus-key lock path the CARRY reopens.

## Constraints
- Service identity is compile-time single-source: `service.version` comes from `env!("CARGO_PKG_VERSION")` and is registered once as a default subscriber field (per obs-plan §3 Service identity; §11 Universal "NEVER hardcode service identity"). The plan does not list a `rust_version` field on any record. Whether the `rust_version` field at `observability.rs:505` is emitted on a log record (and if so on which target and allowlist leaf), or is only a struct field, is research's question. If it is emitted, the raise changes its value and nothing else. It must keep deriving from `env!("CARGO_PKG_RUST_VERSION")`. Never hand-write it as a literal.
- Field-allowlist exactness: every emitted target resolves to an EXACT allowlist leaf whose field set equals the emit site's fields, and no bare prefix key (`app`, `corpus`) may exist (per obs-plan §8 PII Scrubbing, the `app.boot.*` and corpus key-custody leaves). If research finds `rust_version` on an `app.boot.*` record, it must already sit in that record's leaf, or the value is redacted on the wire. This chunk must not add, rename or widen a leaf.
- Corpus key-custody records stay bounded: `corpus.open.error` carries `error_kind` only, and `corpus.keychain.fallback` carries bounded static strings only (per obs-plan §8 corpus key-custody leaves; §6 warn row `corpus.keychain.fallback`). The CARRY's skip-arm cleanup in `fetch_with_lock_dir`'s test must not introduce a new emit, and must not carry the lock-dir or lock-file path into any record (per obs-plan §11 Logs "full paths (Vector 3)").
- Version strings stay off the error boundary: `AppError` sanitization strips library versions (per obs-plan §7 Error Capture & Reporting, line "Sanitize `AppError` stack traces — remove Rust struct names, library versions"; §2 logging-sensitive row). The `app_info` envelope's `rust_version` is an arch §Standard Contracts field, not an error path. It is out of obs's remit, provided it never leaks into an error variant.
- Lint telemetry: `fmt` / `clippy` output is consumed as structured CI annotations (per obs-plan §9 CI Integration, Pipeline integration table). Removing the `incompatible_msrv` allow must leave the clippy `-D warnings` stage green, which keeps that signal free of new findings.
- CI default fields: the `deployment.environment` / `ci.run.id` / `git.commit.sha` default fields are boot-time subscriber fields (per obs-plan §9 CI-specific default fields). Whether a floor-witness CI job, if P4 adds one, inherits the log-artifact upload convention is a P4/research question (per obs-plan §9 Telemetry artifact handling, distinct artifact names per job).

## Patterns to follow
- Compile-time `env!()` identity sourcing with a single registration point, as the plan prescribes for `service.version` (per obs-plan §3 Service identity). The `CARGO_PKG_RUST_VERSION` → `app_info.rust_version` path follows the same shape, so the raise flows through without a code edit at the emit or envelope site.
- Emit-site field set and allowlist leaf asserted by set equality, with the test placed under `pulse-app/tests/` because `[lib] test = false` (per obs-plan §8, the `app.boot.render.posture` and `app.boot.window.navigation` leaf entries). This pattern applies only if research finds `rust_version` riding a boot record and a pin needs to follow the value.
- Lock-path failures surface only as unit `KeychainError` variants, with no path and no OS text (the posture the corpus key-custody leaves encode; per obs-plan §8). Keep this posture when restructuring the skip arm.

## Anti-patterns to avoid
- NEVER log a full path. The corpus-key lock dir and lock file are basename-or-nothing (per obs-plan §11 Logs, Vector 3). A cleanup diagnostic added "for debuggability" to the skip arm is the likely place for this to slip.
- NEVER hardcode service or runtime identity as a literal (per obs-plan §11 Universal). A test fixture `"1.85"` (`crates/ui-bridge/src/contract.rs:2344`) is a fixture, not an emit. Whether it mirrors the live value or is an arbitrary sample is research's question. Either way, no production emit may compare against or carry a hand-typed floor.

## Contract bindings
- obs ↔ arch §Standard Contracts: the `app_info` envelope's `rust_version` (from `CARGO_PKG_RUST_VERSION`) is the arch-owned identity value. Its example reads `"1.84.0"` (`architecture.md:103`), and correcting that is a wrap amendment. obs consumes it only if it also rides a log record (see Constraints 1).
- obs ↔ security §Logging & redaction: the corpus-key lock dir/file never-log-path rule is shared. The CARRY's skip-arm change sits on that boundary (per obs-plan §8; security rules §Logging & redaction, the lock-dir clause).
- obs ↔ tests §3 harness: the boot smoke and `ci-gates` read the JSON log artifact (per obs-plan §9 Telemetry artifact handling). The raise must leave every `app.boot.*` record's field set unchanged, so the harness reads an identical shape.

## Acceptance criteria contributions
- If research finds a `rust_version` field emitted on any log record, that field's value equals the raised declared floor after the chunk, sourced from `env!("CARGO_PKG_RUST_VERSION")` with no literal, and its target's allowlist leaf still resolves it unredacted (per obs-plan §3 Service identity; §8 PII Scrubbing exact-leaf entries)
- No new log target, allowlist leaf or emitted field is introduced by the chunk. The skip-arm cleanup emits nothing, and no record carries the lock-dir or lock-file path (per obs-plan §11 Logs, Vector 3; §8 corpus key-custody leaves)
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` stays green with the `incompatible_msrv` allow removed, so the CI lint annotation stream gains no finding (per obs-plan §9 CI Integration, Pipeline integration `fmt` / `clippy` row)
