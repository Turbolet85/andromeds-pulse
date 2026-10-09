# security extract

## Relevance
partial — the chunk adds no new external surface of its own. Part B may carry a webview adapter record across the TauRPC trust boundary and into the self-observation log. Part C edits `ci.yml`. Part A only re-scopes a timer, but its sample must not carry snapshot content.

## Constraints
- If Part B crosses the bridge through a NEW TauRPC procedure, security-plan §API Security (TauRPC capability authorization row) and §Security Anti-Patterns → API require two things in the same change: router registration in the owning crate, and the procedure's `EXPECTED_PROCEDURES` pin in `xtask/src/main.rs`. `capability-drift` and `check:staged-artifacts` diff the worktree AND the staged bindings. No per-procedure `pulse-app/capabilities/` entry is needed. Whether an existing procedure can carry the record instead (no new pin) is research's question.
- Per security-plan §Input Validation (TauRPC bridge row) and §Threat Model Summary (IPC vector), the procedure argument that carries the adapter result MUST be a `serde`-deserialized, validated struct. The cause must be a closed enum, and any auxiliary field must be bounded, following the `telemetry.frontend.*` precedent (`validate_*` helpers, closed `HueSeverityTier` enum, each rejection unit-tested by name). A rejection returns `AppError::Validation { field, reason }`.
- Per security-plan §Error Handling and §Security Anti-Patterns → Code Patterns / Logging, any new or reused procedure MUST return `Result<T, AppError>`. It must never serialize `anyhow::Error`, and `Internal { message }` carries no stack traces, file paths, library versions or Rust struct names.
- Per security-plan §Logging & Monitoring ("What NEVER to log", log format) and the `ui.ipc.rejection` no-scrub-boundary precedent in §Security Anti-Patterns → Logging, the adapter record's tracing emission MUST sit behind its own exact allowlist leaf with a bounded field set. Free-form strings from the webview (raw `requestAdapter` error text, adapter vendor/description strings) do not cross; a closed cause enum and bounded labels do. The emission runs once per event, not per frame.
- Per security-plan §Logging & Monitoring ("What NEVER to log": snapshot file contents) and §Security Anti-Patterns → Logging (first NEVER bullet), a whole-generation snapshot timer and its perf sample MUST carry only duration and count fields. They must never carry snapshot markdown, attribute values or a full path. The snapshot resolver arm of this ban is currently UNVERIFIED (test-plan §1 `snapshot-resolver-level-coverage`), so a new emission point on that path does not inherit an executed canary.
- Per security-plan §Bootstrap phases `dep-security-ci-gate` and §Security Anti-Patterns → Secrets, the Part C `ci.yml` edit MUST keep:
  - workflow-level `permissions: contents: read`;
  - every third-party Action pinned by a 40-char SHA with a version comment;
  - `step-security/harden-runner` as the first step of the touched `release` job.

  Adding `cache-on-failure: true` to the existing pinned rust-cache step must not repin it to a tag.
- Per security-plan §Security Anti-Patterns → Logging (full-path ban, categorical) and §Security Anti-Patterns → Input (harness-only state-file class), any new harness or CI read of the app log that derives the `frame: cannot-evaluate` cause MUST NOT echo full product paths. The product binary must not read any new harness-only file.

## Patterns to follow
- `telemetry.frontend.record_ipc_rejection` / `ui.ipc.rejection` (security-plan §Logging & Monitoring, §Security Anti-Patterns → Logging): a webview-originated record crosses as a bounded triple. It uses a closed enum category, a coerced window label, a bounded numeric field and its own exact allowlist leaf, with a banned-field pin test.
- The `telemetry.frontend.*` delegated-timing procedures (security-plan §Input Validation, TauRPC bridge row) are the in-namespace precedent for validated argument structs, with a named rejection test per validator.
- For the cause vocabulary: webview-side classification maps the raw result to a closed enum before `invoke`, so the backend never parses free text (the `IpcRejectionCategory` shape in §Security Anti-Patterns → Logging).
- Basename-only path logging through `pulse_app::observability::log_basename` (security-plan §Security Anti-Patterns → Logging, full-path bullet), if any new record references a file.

## Anti-patterns to avoid
- NEVER add a TauRPC procedure without its `EXPECTED_PROCEDURES` pin and a validated argument struct (security-plan §Security Anti-Patterns → API).
- NEVER log snapshot contents, clipboard contents, raw OTLP attribute values or free-form webview-supplied strings on the new timer or adapter records (security-plan §Security Anti-Patterns → Logging).
- NEVER reference a third-party GitHub Action by floating tag, and never grant workflow-level `contents: write` (security-plan §Security Anti-Patterns → Secrets).

## Contract bindings
- security ↔ obs: the adapter record's allowlist leaf, field set and emit cadence are shared with obs-plan's tracing schema and allowlist. The bounded-field and no-free-text rule is security's. The event name and level are obs's.
- security ↔ tests: a new procedure's validator rejections, the allowlist leaf pin (banned-field check) and the `capability-drift` / `check:staged-artifacts` CI gates are test-plan-owned carriers of the §API Security and §Logging mandates.
- security ↔ arch: a new procedure also lands in arch §Occupied Resources (Tauri IPC routes), with the `emit_taurpc_bindings` regen and a staged-copy check before commit.

## Acceptance criteria contributions
- `cargo xtask capability-drift` and `cargo xtask check:staged-artifacts` exit 0 at commit, whether Part B reuses a procedure or adds one (per security-plan §API Security, TauRPC capability authorization row).
- If a procedure is added or extended, every new argument field is validated by a closed enum or bounded numeric validator. Each rejection arm is unit-tested by name and returns `AppError::Validation` (per security-plan §Input Validation, TauRPC bridge row).
- A pin test asserts that the adapter-record allowlist leaf carries only its bounded fields, with no raw adapter strings or error text. A wire or log grep of a run shows no snapshot markdown in the perf-sample or snapshot-timer records (per security-plan §Logging & Monitoring, "What NEVER to log").
- The `ci.yml` diff keeps workflow-level `permissions: contents: read`, harden-runner first in `release`, and the 40-char SHA pin on the touched rust-cache step (per security-plan §Bootstrap phases, `dep-security-ci-gate`).
