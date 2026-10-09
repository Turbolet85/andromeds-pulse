# security extract

## Relevance
partial — the chunk crosses the env-var/CLI-input boundary (reads, and possibly writes, process env before GTK init) and may add one boot log record. It touches no network, IPC, corpus, capability or secret surface. Only a "document" outcome in half 2 leaves it with no domain coverage.

## Constraints
- The threat model lists the reserved env vars plus the system `XDG_RUNTIME_DIR` as the CLI-input entry points. Any NEW product-read system env var used as a detection signal (e.g. a Wayland/session indicator or a GPU-vendor indicator) widens that entry-point list, and so does a product-written WebKit lever var. Record it as a wrap amendment beside the arch §Occupied Resources env-var entry the scope already names (per security-plan §Threat Model Summary → Attack surface "CLI input (env vars + binary launch)"; §Input Validation, "CLI / env var inputs" row).
- Every env input the remedy reads must be validated at the boundary, with a defined result for a malformed or blank value. Precedent: trim it, test it against a closed accepted shape, and give an unusable value a stated posture rather than a panic (per security-plan §Input Validation "CLI / env var inputs" row). Whether the boot path already reads any of these signals is research's question.
- If detection reads a filesystem location, it must be a fixed literal path such as a driver/sysfs node. Never take that path from an env var. An env-sourced path would fall under the canonicalize-and-confine rule and need its own carve-out (per security-plan §Security Anti-Patterns → Input, the product-binary `*_PATH` / `*_DIR` ban and its scoped carve-outs).
- A posture boot record must not log a full filesystem path or an env-var VALUE. It may carry closed labels plus the env-var NAME, which is safe (per security-plan §Logging & Monitoring "What NEVER to log"; §Security Anti-Patterns → Logging, the full-path ban, now categorical).
- If the record is a deliberate no-scrub boundary, it needs its OWN exact allowlist leaf with a closed field set, because field redaction is applied at the subscriber layer and fields not on the allowlist are redacted. Precedent: `app.exit`, `ui.webgpu.adapter` (per security-plan §Logging & Monitoring "Log format"; §Security Anti-Patterns → Logging, the `app.exit` / `ui.webgpu.adapter` NO-SCRUB entries).
- The env write is `unsafe` under Edition 2024. Its safety precondition, no other thread alive, must hold at the write site. The plan names threads the boot spawns at install, including the `pulse-exit-reporter` thread in the exit FFI surface. Whether the lever can be set before observability install and the tracing appender start is research's question (per security-plan §Security Anti-Patterns → Universal, Edition 2024 `unsafe_op_in_unsafe_fn` defaults and the atexit/thread-locals entry).
- If any new crate is added for GPU or session detection, it falls under the supply-chain gates: `cargo audit` pass/fail, `cargo deny check bans licenses sources` with `multiple-versions = "deny"` never relaxed, and a duplicate allowed only as an ID-scoped carve-out (per security-plan §Dependency Security → CI integration).

## Patterns to follow
- The `XDG_RUNTIME_DIR` read discipline is the nearest precedent for a product-read SYSTEM env var: trim, validate when set, and give a deterministic posture when unset or blank. Its stated residuals are written into the plan body (per security-plan §Security Anti-Patterns → Input, corpus-key lock dir carve-out).
- The `app.exit` record shape: closed labels only, an exact allowlist leaf, pin tests on the field set in both directions, and the record's blind spots stated in the body. The plan names `_exit` as one of those blind spots, which bears directly on the chunk's GTK `_exit(1)` hypothesis (per security-plan §Security Anti-Patterns → Logging, `app.exit` entry).
- `observability::log_basename` is the sanctioned emitter if any path-bearing field is ever needed (per security-plan §Security Anti-Patterns → Logging, full-path ban).
- Fail toward a stated posture and never crash on an unusable input, the same way an unusable store degrades the corpus rather than aborting boot (per security-plan §Secret Management → Runtime).

## Anti-patterns to avoid
- NEVER spawn a subprocess for detection (`lspci`, `nvidia-smi`, a shell) whose argv is built from env-sourced or otherwise external strings. Prefer an in-process fixed-path or env read (per security-plan §Security Anti-Patterns → Code Patterns, `Command::new(...).arg(user_input)` ban).
- NEVER log the lever's or a detection var's VALUE, or a full driver/sysfs/runtime path, in the posture record (per security-plan §Security Anti-Patterns → Logging).
- NEVER read an env-sourced path without canonicalize-and-confine or a scoped, ratified carve-out (per security-plan §Security Anti-Patterns → Input).

## Contract bindings
- security ↔ obs ↔ tests: a new posture boot record needs an exact allowlist leaf (obs schema) and an allowlist pin test, as the `pulse-app/tests/unit_observability_allowlist_*.rs` precedent shows. Without the leaf, its fields are redacted at the subscriber layer.
- security ↔ arch: a product env read or write is an arch §Occupied Resources env-var entry and also a security-plan §Threat Model CLI-input / §Input Validation env-row enumeration amendment. Both are owed at wrap, not at phase.
- security ↔ tests CI: if a dependency is added, it binds to the `supply-chain` CI job (`cargo audit`, `cargo deny check bans licenses sources`).

## Acceptance criteria contributions
- If a posture boot record ships, its emitted fields are closed labels plus at most the env-var NAME. A boot log read shows 0 full paths and 0 env-var values, and an allowlist pin test asserts the exact field set (per security-plan §Security Anti-Patterns → Logging).
- Every new env input read for detection has a defined result for unset, blank and malformed values, with no panic, and that result is stated in the chunk report (per security-plan §Input Validation, "CLI / env var inputs" row).
- No detection code path builds a `Command` from env-sourced or otherwise external text. A grep of the changed boot code for `Command::new` shows none, or only a fixed literal argv (per security-plan §Security Anti-Patterns → Code Patterns).
- If any dependency is added: `cargo audit` exits 0 and `cargo deny check bans licenses sources` passes, run as separate invocations from `cargo deny check advisories` (per security-plan §Dependency Security → CI integration).
