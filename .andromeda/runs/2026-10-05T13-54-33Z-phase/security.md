# security extract

## Relevance
partial — the chunk changes the argv of the L4 product subprocess (`build_llama_cli_args`) and may add a grammar file that reaches disk at spawn. It adds no IPC, OTLP, MCP or capability surface.

## Constraints
- The `-p` prompt stays the ONE OTLP-derived argv operand, and it is admitted only through `validate_prompt_bounded`, which runs before the `Command` is built. That check is the 16384 B ceiling plus NUL/C0/C1 rejection, with `\n`/`\r`/`\t` whitelisted. The swap from `--json-schema-file` to `--grammar-file` and the new sampling flags must not move, bypass or weaken it. Per security-plan §Input Validation (the L4 inference argv prompt row) and §Security Anti-Patterns → Code Patterns (the `Command::new(...).arg(user_input)` ban and its single bounded exception).
- Every new argv operand must be first-party:
  - the `--grammar-file` path;
  - the `--temp`/`--top-p`/`--top-k`/`--min-p` values.

  None may come from an OTLP attribute, an MCP tool argument or workspace-detector output. Per security-plan §Security Anti-Patterns → Code Patterns (the MCP STDIO command-injection cluster ban). Research should confirm the grammar path is built the way `schema_path` is today, from a first-party constant and never from telemetry.
- The model stays selected by `ANDROMEDA_PULSE_MODEL_PATH`, and its guard must not regress. That guard is trim plus reject-empty, `..` rejected before canonicalizing, the 4096 B length bound, `canonicalize()`, the regular-file assert, and opt-in `ANDROMEDA_PULSE_L4_ALLOW_ROOT` confinement that fails closed. Per security-plan §Security Anti-Patterns → Input (NARROWED EXCEPTION). If the chunk adds any NEW product-read `ANDROMEDA_PULSE_*_PATH` var (for example a grammar-path override), that var falls under the categorical canonicalize-plus-data-dir-confinement rule, not under the L4 exception, per the same §Input ban.
- Where the GBNF is written is a filesystem boundary question:
  - The plan names exactly two product-written locations outside the data dir: the `~/Downloads` training-export sink and the corpus-key lock dir. Per security-plan §Data Protection (the corpus-key lock file entry) and §Security Anti-Patterns → Input.
  - If the GBNF is materialized at spawn outside the data dir (for example in the temp dir), that is a third such location and a boundary widening that needs the founder's word.
  - Whether today's schema file already lands there, and so whether the GBNF only follows that precedent, is research's question.
- The re-based observed prompt maximum (7,575 B) and the ~2.16× ceiling ratio are written at two sites in security-plan, and they must move together. They are the §Input Validation L4 argv row and the §Security Anti-Patterns → Code Patterns bullet. The scope says the prompt text and its embedded schema are unchanged, so any re-measured composition under the pick is a measurement note, never a new bound.
- Any L4 failure under the new argv must be logged as a bounded category on `interpretation.inference.error`, carrying no prompt text and no model-output bytes. This covers sampler-init failure, a grammar rejection, and exit-0 output with no JSON. Per security-plan §Input Validation (the L4 argv row's rejection and `InferenceFailed` categories) and §Logging & Monitoring → What NEVER to log.
- Any log record that names the grammar file or the model GGUF names its basename only, never the full path. Per security-plan §Logging & Monitoring → What NEVER to log (full product-consumed filesystem paths) and §Security Anti-Patterns → Logging.

## Patterns to follow
- Validate before spawn: the bounded-parse helper runs ahead of `Command` assembly, and its bounded `label()` is the only text that reaches the log. Per security-plan §Input Validation (the L4 argv prompt row).
- Use `std` both-sides-canonicalize on the `publish_workspace_key` precedent for any path the product reads. Per security-plan §Security Anti-Patterns → Input.
- New argv operands are compile-time or first-party constants, so the subprocess boundary stays shape-and-length controlled. Per security-plan §Security Anti-Patterns → Code Patterns.

## Anti-patterns to avoid
- Never build an argv operand from OTLP-, MCP- or workspace-detector-sourced text without a bound. Per security-plan §Security Anti-Patterns → Code Patterns.
- Never log the full path of the GGUF, the llama binary, the allow-root or a materialized grammar/schema file. Never log prompt text or model stdout bytes on an inference failure. Per security-plan §Security Anti-Patterns → Logging and §Logging & Monitoring.
- Never commit the GGUF, and never add a new path env var that skips canonicalization and confinement. Per security-plan §Security Anti-Patterns → Input.

## Contract bindings
- security ↔ obs: the bounded failure categories on `interpretation.inference.error` are a security no-leak guarantee and also obs-plan schema leaves. A new failure category, such as a grammar or sampler-init failure, binds both plans. Per security-plan §Input Validation and §Logging & Monitoring.
- security ↔ tests: the GBNF==schema equality test and any ASCII/argv-transport pin on template or grammar text are owned by the test plan. The supply-chain CI gates bind here only if the chunk vendors the llama.cpp converter or adds a dependency. If it does, `cargo deny check bans licenses sources` and `cargo audit` apply, and a vendored third-party script needs license provenance. Per security-plan §Dependency Security → CI integration.

## Acceptance criteria contributions
- `validate_prompt_bounded` still runs before `build_llama_cli_args` on the production L4 path. Its tests stay green: the reject arms and the accepts-multi-line arm. Per security-plan §Input Validation (L4 inference argv prompt row).
- A grep of `build_llama_cli_args` shows `--grammar-file` and the sampling flags take only first-party values, and `-p` is the only OTLP-derived operand. Per security-plan §Security Anti-Patterns → Code Patterns.
- The real-model leg's log shows no full path for the GGUF, the binary or the grammar file. It shows no prompt or output text on any `interpretation.inference.error` record. Basename occurrences only. Per security-plan §Logging & Monitoring → What NEVER to log.
- If the chunk adds a dependency or vendors the converter, `cargo deny check bans licenses sources` and `cargo audit` both pass, run as separate invocations. Per security-plan §Dependency Security → CI integration.
