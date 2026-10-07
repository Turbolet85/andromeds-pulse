# security extract

## Relevance
partial — the chunk adds no port, IPC procedure, env var, capability or dependency; it touches security through the L4 argv prompt bound (larger corpus blocks, a possible prompt-text remedy), the scrub posture of corpus-derived prompt text (a possible corpus-block remedy), and the corpus at-rest boundary its two CARRYs stand on.

## Constraints
- Every composition this chunk produces — each new probe shape (up to five corpus lines) and each remedy arm (a restated obligation adds bytes) — is required to pass `validate_prompt_bounded` before a `Command` is built: the `MAX_PROMPT_BYTES` ceiling plus the control-character rejection with its `\n` / `\r` / `\t` whitelist; over-long means reject, never truncate (per security-plan §Input Validation, row "L4 inference argv prompt"). The ceiling is not raised to fit a shape. Whether the probe's real-model arm routes through the product's `validate_prompt_bounded` for the new shapes is research's question.
- `-p` is required to stay the ONE OTLP-derived argv operand and the last; every other operand is first-party (per security-plan §Security Anti-Patterns → Code Patterns). A remedy lands in prompt text or in the corpus block, never as a new operand carrying digest- or corpus-derived text.
- A "cap the corpus block" remedy is required to cap at selection or render, not by cutting the assembled prompt: silent truncation of model input is the stated reason the bound rejects (per security-plan §Input Validation, row "L4 inference argv prompt").
- Corpus-derived text reaching the prompt is required to be scrubbed text: any attribute value crossing into the corpus passes `mask_secret_spans` at its write boundary (per security-plan §Security Anti-Patterns → Logging, "INTENDED posture"), and the prompt's OTLP-derived substring is stated as scrubbed upstream by `Digest::scrubbed_clone` (per security-plan §Security Anti-Patterns → Code Patterns). A remedy that reorders, caps or filters corpus lines must not open a path around that scrub. Where the corpus block is rendered relative to `scrubbed_clone` at HEAD is research's question.
- The d3 creating digest is cell-encrypted corpus content, and the corpus key is secret-class: never exported, never logged, never shared across installations (per security-plan §Secret Management → "What counts as secret"). No probe shape, test or evidence step may obtain it by decrypting a real corpus through a side path; the credential-store read is the only key source (per security-plan §Secret Management → Storage → Runtime).
- Telemetry and the corpus are user-content that can hold incidentally captured secrets (per security-plan §Threat Model Summary → Data classification). New shapes and committed evidence are therefore required to be synthetic literals; no captured digest, payload or recovered corpus line enters the tree.
- Any log record the chunk adds or changes (a selection count, a remedy's cap/exclusion tally) is required to carry counts and closed labels only — no corpus line text, no prompt text, no full path; an argv-prompt rejection surfaces as a bounded category with no excerpt (per security-plan §Security Anti-Patterns → Logging).

## Patterns to follow
- Measure each composition through the dev probe's `--dry-run` over the product's unchanged render → prompt builder → `validate_prompt_bounded` path, the way the prior L4 chunks' readings are recorded (per security-plan §Input Validation, row "L4 inference argv prompt").
- Record prompt sizes as session measurement notes, never as bounds; the row's headroom statement rests on the observed maximum, so a composition above the recorded maximum is a fact the wrap owes that row (per security-plan §Input Validation, row "L4 inference argv prompt").
- A prompt rejection reports on the existing `interpretation.inference.error` leaf as a bounded `PromptRejection` label, and the digest is skipped (per security-plan §Input Validation, row "L4 inference argv prompt").
- Scrub at the write boundary and mask a JSON projection per string leaf, never as serialized text; a synthetic corpus incident that carries a summary projection follows that shape (per security-plan §Security Anti-Patterns → Logging, "MEASURED reality").
- Paths in any record are basename plus the env-var NAME (per security-plan §Logging & Monitoring → "What NEVER to log"). Whether the probe's own stdout or evidence files print a full model or binary path is research's question.

## Anti-patterns to avoid
- NEVER pass a string sourced from an OTLP attribute to `Command` argv outside the bounded `-p` path (per security-plan §Security Anti-Patterns → Code Patterns).
- NEVER log raw OTLP attribute values, content payloads or prompt text; a count or a closed label stands in (per security-plan §Security Anti-Patterns → Logging).
- NEVER export, log or route around the corpus encryption key or its credential-store read (per security-plan §Secret Management → "What counts as secret").

## Contract bindings
- security ↔ arch: the kind-label-only `TRIGGER:` ruling (scope CARRY a) is owned by arch §Established Decisions [Fault Identity]. security-plan.md carries no rule for it — it names the TRIGGER line only inside a prompt-size measurement note (§Input Validation, row "L4 inference argv prompt") — so this extract adds no authority there; the stop-and-ask boundary stands on arch and the founder.
- security ↔ tests: the scope's "prompt template text stays ASCII (argv transport)" boundary has no anchor in security-plan.md; its pin and the prompt-lineage pins a text remedy moves belong to the tests domain.
- security ↔ tests: fixtures carry no real captured content — the synthetic-only constraint above binds the probe shapes and any digest test data (security-plan §Threat Model Summary → Data classification).
- security ↔ obs: `interpretation.inference.error` and the prompt-assembly record are obs-schema leaves; the no-prompt-text / bounded-category rule binds any field added to them (security-plan §Security Anti-Patterns → Logging).
- security ↔ tests CI: `cargo audit` and `cargo deny check bans licenses sources` bind only if the chunk adds a dependency, which the scope does not plan (security-plan §Dependency Security → CI integration).

## Acceptance criteria contributions
- Each new probe shape and each remedy arm has a measured composition size at or under `MAX_PROMPT_BYTES`, with the constant and the control-character whitelist unchanged in the diff (per security-plan §Input Validation, row "L4 inference argv prompt")
- The diff adds no argv operand sourced from digest or corpus text; `-p` remains the one OTLP-derived operand and no model, sampling or grammar operand changes (per security-plan §Security Anti-Patterns → Code Patterns)
- No file added by the chunk holds decrypted corpus or digest content, and no added code reads the corpus key or decrypts a real corpus for the probe; new shapes are built from source literals (per security-plan §Secret Management → "What counts as secret")
- Every log record added or changed by the chunk carries only counts or closed labels — a grep of the diff's `tracing` sites shows no corpus line text, prompt text or full path field (per security-plan §Security Anti-Patterns → Logging)
