# security extract

## Relevance
partial — no new surface (no port / procedure / capability / env var / dependency), but the chunk drives the real L4 subprocess path through three product-consumed path env vars and commits evidence derived from model runs, so the L4 input-validation, argv and logging/hygiene rules bind.

## Constraints
- The three product-consumed L4 path vars that the leg env `inputs#I2` exports (`ANDROMEDA_PULSE_MODEL_PATH` · `_LLAMA_CUDA_BIN_PATH` · `_LLAMA_CPU_BIN_PATH`) must go through the always-on guard (trim/reject-empty, pre-canonicalize `..` rejection, `MAX_PATH_INPUT_BYTES` bound, canonicalize, regular-file assert), with confinement opt-in through `ANDROMEDA_PULSE_L4_ALLOW_ROOT` and fail-closed when set but unresolvable (per security-plan §Input Validation, L4 path row; §Security Anti-Patterns → Input, NARROWED EXCEPTION). The guard stays unchanged here. Whether `l4_decision_probe` in real mode reaches the binary/model through that same `validate_path_input` path or bypasses it is a question for research (P3).
- The `-p` prompt operand must pass `validate_prompt_bounded` (16384-byte ceiling, NUL/C0/C1 rejection with `\n`/`\r`/`\t` whitelisted, reject never truncate) before the `Command` is built (per security-plan §Input Validation, L4 inference argv prompt row; §Security Anti-Patterns → Code Patterns). The plan's measured ceiling basis is the 7,185 B v2.4 S4 dry-run composition. Research should check whether the real-model run's S1–S4 prompts stay under the ceiling at HEAD.
- Model output is never logged and never committed as text. Evidence carries only the probe's bounded labels and verdict lines. An inference failure surfaces as a bounded category on `interpretation.inference.error` and carries no output bytes and no prompt text (per security-plan §Input Validation, L4 argv row, the stdout-decode "no output bytes" guarantee; §Logging & Monitoring → What NEVER to log).
- No full product-consumed filesystem path (model GGUF, llama-cli binary, allow-root) may appear in any log record or in the committed evidence. Only the canonicalized basename plus the env-var NAME is allowed (per security-plan §Logging & Monitoring → What NEVER to log; §Security Anti-Patterns → Logging). Scope's `~`-relative spelling for committed evidence is a hygiene choice layered on top of this rule.
- The GGUF, the leg env file and any credential-class material stay out of the tree. The model is gitignored, and secret scanning (pre-commit + per-PR) is the verifier for anything committed (per security-plan §Secret Management → Secret scanning in CI; §Security Anti-Patterns → Secrets).
- Scope Boundaries declare no new env var, port, TauRPC procedure, capability JSON, corpus table, MCP tool or dependency. Any such addition would reopen §Input Validation / §API Security / §Dependency Security obligations that this chunk does not carry (per security-plan §Input Validation, CLI / env var inputs row; §Dependency Security).

## Patterns to follow
- The L4 guard primitive: `std` both-sides-canonicalize, following the `publish_workspace_key` precedent, at `pulse-app/src/llamacli_inference.rs::validate_path_input` (per security-plan §Security Anti-Patterns → Input).
- An unconfined L4 posture is announced once per boot at WARN on `interpretation.model.allow_root`. A real-model leg that leaves the allow-root unset should expect that WARN; the evidence can record it as the posture, not as a fault (per security-plan §Security Anti-Patterns → Input, NARROWED EXCEPTION "Consequence").
- Prompt rejection is reported only as a bounded category through `PromptRejection::label()`. Any probe-side failure reading follows the same bounded-category shape and carries no excerpt (per security-plan §Input Validation, L4 argv row).
- Scrubbing happens upstream: OTLP-derived prompt content is PII-scrubbed by `Digest::scrubbed_clone` before argv. The prompt bound is a shape-and-length control, not a confidentiality one (per security-plan §Security Anti-Patterns → Code Patterns).

## Anti-patterns to avoid
- Pasting raw model generations, prompt text or `llama-cli` stdout into `evidence/`, a report or any log, including "illustrative" excerpts (per security-plan §Logging & Monitoring → What NEVER to log; §Input Validation, L4 argv row).
- Recording a full host path (the `~/dev/tools/...` binary, the `AI-Model/...gguf` model or the leg env file) in committed evidence or log output (per security-plan §Security Anti-Patterns → Logging).
- Loosening the L4 path guard or the prompt bound to make a real-model run go through, for example raising `MAX_PROMPT_BYTES` or skipping the regular-file assert. Either is a security-plan amendment, not a chunk-local tweak (per security-plan §Security Anti-Patterns → Input and → Code Patterns).

## Contract bindings
- security ↔ obs: the no-model-output and basename-only rules bind to the obs-plan self-observation allowlist leaves `interpretation.inference.error`, `interpretation.model.allow_root` and `interpretation.prompt.assemble`. The evidence log review checks both domains at once.
- security ↔ tests/CI: committed evidence passes the run-dir hygiene check and the CI `secret-scan` job (per security-plan §Secret Management → Secret scanning in CI).

## Acceptance criteria contributions
- (security) Committed `evidence/` (stdout captures + `runs.json` for both arms) contains no model-generated text, no prompt text, and no absolute host path (no `/home/` prefix). The only path spellings are basenames or `~`-relative (per security-plan §Logging & Monitoring → What NEVER to log; §Security Anti-Patterns → Logging).
- (security) Any self-observation log produced by the real-model leg contains the L4 model/binary paths only as `path_basename` (the containing directory appears 0×), and every `interpretation.inference.error` record carries a bounded category with no output bytes (per security-plan §Security Anti-Patterns → Logging; §Input Validation, L4 argv row).
- (security) `git diff --stat` over the chunk shows no change to `pulse-app/src/llamacli_inference.rs`'s `validate_path_input` / `validate_prompt_bounded` / `MAX_PROMPT_BYTES` / `MAX_PATH_INPUT_BYTES`, and no new env var, dependency or capability JSON (per security-plan §Input Validation; §Dependency Security).
- (security) The CI `secret-scan` job is green on the chunk commit, and the GGUF stays untracked (per security-plan §Secret Management → Secret scanning in CI).
