# security extract

## Relevance
partial — the chunk widens a read-only, fixed-path filesystem-presence probe; it adds no input boundary, IPC, dependency or secret, so the domain contributes guard-rails (no subprocess, no new env/path input, no path logging, no new dep) rather than new controls.

## Constraints
- The probe's candidate locations stay first-party compile-time constants (system-level paths); no candidate may be derived from an env var, config value, OTLP attribute, MCP argument or workspace-detector output. Any path that is product-consumed from an env var falls under the canonicalize-and-confine rule (per security-plan §Input Validation, CLI / env var inputs row; §Security Anti-Patterns → Input), so introducing one here would be a boundary widening, not a probe change. The scope declares no new env var; whether `ANDROMEDA_PULSE_HARDWARE_PROFILE` is listed in that row today is research's question — the chunk leaves that override unchanged either way.
- No `dlopen`, no `nvidia-smi` / `ldconfig` or any other subprocess: security-plan §Security Anti-Patterns → Code Patterns bans `Command::new(...).arg(...)` over untrusted strings and admits exactly one argv path (the L4 `-p` prompt, behind `validate_prompt_bounded`); a new probe subprocess would be an unvetted second process-spawn site. Presence-only `exists()` (or equivalent metadata read) over fixed paths is the shape that stays inside the plan.
- No full filesystem path reaches the log: security-plan §Logging & Monitoring → "What NEVER to log" makes the full-path ban categorical (basename + env-var NAME only). The scope expects the probe to log no path at all; if any probe-outcome record is added, it carries a bounded label (e.g. GPU present/absent), never the matched path.
- No new dependency: if one were added it would enter the `cargo audit` + `cargo deny check bans licenses sources` gates and the `multiple-versions = "deny"` posture (per security-plan §Dependency Security → CI integration); the scope's "no new dependency" boundary keeps those gates unchanged.
- Fail-safe direction preserved: a probe miss must yield GPU-absent (CPU profile), never a false GPU-present — the widening only adds candidates. This keeps the downstream L4 binary choice on the side the security plan already vets; the CUDA/CPU `llama-cli` paths themselves stay governed by the `validate_path_input` guard (per security-plan §Input Validation, PRODUCT-CONSUMED path boundary, `ANDROMEDA_PULSE_LLAMA_{CUDA,CPU}_BIN_PATH`), which this chunk must not touch. Whether the detected profile alters which bin-path var is read is research's question.
- Any testability seam (injectable candidate list / existence predicate) is a test-only or pure-fn parameter, never a runtime-configurable input: a production-reachable override of the candidate set would be a new untrusted path input under security-plan §Threat Model Summary (filesystem-reads vector) and §Input Validation.

## Patterns to follow
- System-signal-only hardware detection with an explicit safe default (the fn's own security note, scope §Boundaries) — the same fail-safe shape as the plan's fail-closed guards (per security-plan §Input Validation, `ANDROMEDA_PULSE_L4_ALLOW_ROOT` / `XDG_RUNTIME_DIR` rows): an unexpected or unreadable location degrades to the conservative outcome, never panics, never blocks boot.
- Basename-or-nothing logging via `pulse_app::observability::log_basename` if a path ever must appear (per security-plan §Logging & Monitoring); the expected design logs none.
- Pure fn over an injected predicate, with production calling it over the real constant set — mirrors the plan's preference for validation logic that is unit-testable without host state (per security-plan §Input Validation, L4 argv prompt row's bounded pure validator precedent).

## Anti-patterns to avoid
- Spawning a probe process (`nvidia-smi`, `ldconfig -p`, `lspci`) or `dlopen`-ing `libcuda` to "confirm" the GPU (per security-plan §Security Anti-Patterns → Code Patterns; scope §Boundaries).
- Logging the resolved / matched `libcuda` path in full (per security-plan §Security Anti-Patterns → Logging; §Logging & Monitoring).
- Reading the candidate set from a new `ANDROMEDA_PULSE_*_PATH` / `*_DIR` env var without the canonicalize-and-confine guard (per security-plan §Security Anti-Patterns → Input).

## Contract bindings
- security ↔ obs: if a probe-outcome field is added to an existing record (e.g. the `ModelLoadEvent` / `interpretation.*` targets), it is a bounded label only and stays inside obs-plan's allowlist discipline; the scope declares no new log target, so the expected binding is "none added".
- security ↔ tests: the injectable-seam test fixtures use synthetic candidate paths (no host-real paths asserted as present), and the supply-chain CI job (`cargo deny check bans licenses sources`) stays green with an unchanged lockfile (per security-plan §Dependency Security → CI integration).

## Acceptance criteria contributions
- `crates/interpretation/src/hardware.rs` diff introduces no `std::process::Command` / `tokio::process::Command`, no `libloading` / `dlopen`, and no new `std::env::var` read (grep over the diff) (per security-plan §Security Anti-Patterns → Code Patterns).
- No tracing call in the changed code emits a full path; any added field is a bounded label (grep the diff for `tracing::` / `info!` / `warn!` with a path argument) (per security-plan §Logging & Monitoring).
- `Cargo.lock` unchanged and `cargo deny check bans licenses sources` passes (per security-plan §Dependency Security → CI integration).
- A unit test pins the safe default: with no candidate present the probe returns GPU-absent (per security-plan §Threat Model Summary, filesystem-reads vector — conservative outcome on an unexpected host).
