# security extract

## Relevance
partial — the PREREQ adds new self-observation log emission on exit paths (security-plan §Logging & Monitoring / §Security Anti-Patterns → Logging bind directly); the P-075 round crosses the MCP stdio and loopback OTLP trust boundaries but adds no new surface Pulse-side; the evidence-citation and matrix work are out of domain.

## Constraints
- The exit-cause record is internal logging and may carry error detail, but only stack-local context, and always subject to the NEVER-log list (per security-plan §Error Handling "Internal logging"; §Logging & Monitoring "What NEVER to log"). A cause derived from a panic payload, an error `Display` chain or a native message is free text of unknown origin. Whether the existing `app.panic.fatal` path already bounds or sanitizes its payload is research's question. The new exit classes must not pass free text through unbounded.
- Any filesystem path in an exit-cause record is basename-only, and this rule is categorical (per security-plan §Security Anti-Patterns → Logging, the full-path bullet, `log_basename` precedent). A data-dir, log-dir or binary path in an exit or init failure is the likely leak site.
- The exit hook must not echo secret-class material. `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` and the derived corpus key are secret-class: never logged, never exported (per security-plan §Secret Management "What counts as secret"). This is load-bearing because the observed death's only ERROR was the keyring line (`KeyringUnavailable`), so a keyring- or corpus-adjacent exit cause is in the candidate set.
- Exit-cause reporting stays local and `tracing`-only. There is no third-party crash reporter (a future opt-in one would require scrubbing plus explicit consent), and no OTLP exporter points at the product's own `:4317`/`:4318` (per security-plan §Error Handling "Error reporting integration"; §Security Anti-Patterns → API, self-observation ban).
- An `atexit`-style native hook needs FFI, which must respect the Edition 2024 security defaults (`unsafe extern`, `unsafe_op_in_unsafe_fn`) (per security-plan §Security Anti-Patterns → Universal, the toolchain bullet). A new direct dependency for it (e.g. `libc`) goes through the dependency gates (per security-plan §Dependency Security → CI integration).
- The P-075 MCP readback runs only under the declared double-gate: compile-time `--features mcp-server` AND runtime `ANDROMEDA_PULSE_MCP_ENABLED=true`. Pulse must not relax either gate, or default it on, to suit an external round (per security-plan §Security Anti-Patterns → Code Patterns, the sidecar double-gate bullet; §Threat Model Summary → MCP stdio surface).
- The external driver reaches OTLP over loopback only. No binding change to `0.0.0.0` or a non-loopback interface is admissible for the round (per security-plan §Security Anti-Patterns → API, the loopback bullet; §Threat Model Summary → Attack surface, public API).

## Patterns to follow
- Basename emission through `pulse_app::observability::log_basename`, with allowlist leaves named `*_basename` (per security-plan §Security Anti-Patterns → Logging, the full-path bullet: the `app.boot.tracing.init` / `app.boot.pid` precedent).
- A deliberate bounded-field log boundary behind its own exact allowlist leaf, carrying only closed-enum or integer fields with free text excluded by construction, pinned by tests that include a banned-field check. Precedents are `ui.ipc.rejection` and `ui.webgpu.adapter` (per security-plan §Security Anti-Patterns → Logging). An exit-cause record shaped as `{exit_class (closed enum), code (integer), ...basename}` fits this pattern.
- A bounded category in place of offending text. The argv-prompt rejection carries a category with no prompt text, excerpt or substring (per security-plan §Security Anti-Patterns → Logging, the full-path bullet).
- The harness-side `run/andromeda-pulse.exit` boot-recorder file is harness-only and read through `read_ended`'s bounded grammar. The product binary never reads it (per security-plan §Security Anti-Patterns → Input, the boot-recorder state-file carve-out). The in-product exit-cause log is a separate channel and must not turn that file into a product-read input.

## Anti-patterns to avoid
- NEVER log raw OTLP attribute values, payload content, MCP tool response bodies or clipboard contents (per security-plan §Security Anti-Patterns → Logging, first bullet). This applies to exit-cause text, and to any MCP response observed during the P-075 round that reaches Pulse's own log.
- NEVER log a full product-consumed filesystem path (per security-plan §Security Anti-Patterns → Logging).
- NEVER expose internal hostnames or IPs in MCP error objects (per security-plan §Security Anti-Patterns → Logging, last bullet). This is relevant if the round exercises MCP error paths.

## Contract bindings
- security ↔ obs: field redaction is applied at the subscriber layer (per security-plan §Logging & Monitoring "Log format"). A new exit-cause tracing target therefore needs its exact allowlist leaf (obs-plan / `observability.md`), or its cause fields are redacted on the wire. Whether that redaction is the default for an unlisted target is research's question.
- security ↔ tests: an exit-cause witness should include a no-full-path / no-canary check alongside the "record survives exit" check (test-plan harness). Any added dependency rides the CI `supply-chain` job (per security-plan §Dependency Security → CI integration).
- security ↔ arch (MCP / corpus key): the sidecar decodes corpus BLOBs, so P-075's MCP readback needs the corpus key in the sidecar process. That key comes from the OS credential store, or the opt-in passphrase fallback where no store exists, for example headless CI (per security-plan §Secret Management → Runtime). Whether Conductor's round supplies a key source on its host is research's question.

## Acceptance criteria contributions
- Each exit-cause record induced by the witness contains 0 full filesystem paths (basename only) and 0 occurrences of a planted canary (a passphrase-env value and an OTLP-attribute string), read from the flushed log (per security-plan §Security Anti-Patterns → Logging).
- The new exit-cause target's field set is bounded (a closed-enum class, an integer code and basenames, with no free-text payload field) and pinned by an exact-allowlist-leaf test that includes a banned-field check (per security-plan §Security Anti-Patterns → Logging, the `ui.ipc.rejection` precedent).
- If the chunk adds a dependency (e.g. for an `atexit` hook): `cargo deny check bans licenses sources` and `cargo audit` both pass, run as separate invocations (per security-plan §Dependency Security → CI integration).
- No diff relaxes the MCP double-gate or changes an OTLP bind address. A grep over the chunk diff finds no new `0.0.0.0` bind and no default-on for `mcp_server_enabled` / `ANDROMEDA_PULSE_MCP_ENABLED` (per security-plan §Security Anti-Patterns → Code Patterns; §Security Anti-Patterns → API).
