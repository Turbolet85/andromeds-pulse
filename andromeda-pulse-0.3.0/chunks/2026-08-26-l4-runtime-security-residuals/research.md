# Codebase Research — 2026-08-26-l4-runtime-security-residuals

## Scope
- **Depth:** moderate · **Reads:** 9 · **Globs/Greps:** 14 · **Graph queries:** 3 (rust plane, `db_state: fresh`)

## Files inspected
- `pulse-app/src/llamacli_inference.rs` (643 lines; read `:110-145`, `:210-235`, `:375-470`, `:565-615`) — the
  single production file this chunk changes. Holds all three env consts, both guard halves, the argv builder,
  and the two `interpretation.model.load` emit sites.
- `crates/interpretation/src/prompt.rs` (`:140-170`, `:630-640`) — the prompt builder and the test-only
  length assertion.
- `pulse-app/src/inference_runtime.rs` (`:300-345`) — the one production caller chain that assembles the
  prompt and hands it to `generate_constrained`.
- `crates/triage/src/contract.rs` (`:540`, `:580-590`) — `Digest::payload_summary` and its `scrubbed_clone`.
- `crates/workspace-detector/src/contract.rs` (`:90-126`) — the repo's ONLY canonicalize-and-confine guard
  plus its bounded-read sibling; the pattern both deliverables copy.
- `crates/plugins/src/loader.rs` (`:155-185`) — the second confinement precedent (traversal-component reject
  + canonicalize + NotFound clean-skip).
- `pulse-app/src/observability.rs` (`:1928-1985`) — the `interpretation` allowlist region.
- `pulse-app/tests/unit_llamacli_inference.rs` (562 lines, structure only) — the existing, correctly-located
  pin suite this chunk extends.
- `pulse-app/tests/integration_real_llama_cli.rs` (173 lines, structure only) — the env-gated real-runtime leg.

## Graph impact (rust plane, `db_state: fresh`; trace at `{run_dir}/tree-query-{marker}.json`)
- **`canonicalize_path`** — 7 rows. **2 production callers**, both `LlamaCliInference::new` at
  `llamacli_inference.rs:116` and `:117`; the other 5 are `pulse-app/tests/unit_llamacli_inference.rs`
  (`:32` import, plus `canonicalize_rejects_nonexistent_path` `:270`, `canonicalize_rejects_directory_path`
  `:278`, `canonicalize_accepts_existing_regular_file` `:287`, `canonicalize_rejects_traversal_payload` `:299`).
  Meaning: changing this function's contract is crate-local and its regression net already exists.
- **`build_llama_cli_args`** — 10 rows, **exactly ONE production caller**: `generate_constrained` at
  `llamacli_inference.rs:530`. The other 9 are spawn-arg pins in the same test file (`:46` max-tokens, `:55`
  single-turn, `:63` simple-io/no-display-prompt, `:78` model-path+schema-file, `:99`/`:106` ngl routing,
  `:119` prompt-via-`-p`, `:436` log-disable). Meaning: the argv shape is already pinned field-by-field, so a
  bound added here is immediately regression-covered — and `spawn_args_contain_prompt_via_p_arg` (`:119`) is
  the exact pin a relocation would have had to rewrite.
- **`read_env_path`** — 3 rows, all production, all in-file (`:116`, `:117`, `:140`). No external consumer.
- **Cross-crate:** none. No query returned a row outside `pulse-app`, so there is no crate-edge blast radius
  and no library crate can be made to depend on `pulse-app` by this change.
- **Truncation note:** the first read of Q1/Q2 through `tail` showed only test callers and would have
  supported a false "no production caller" conclusion; the `rows` field (7 and 10) is what surfaced the
  production callers. Recorded because the trap is live in this repo's tooling, exactly as the discipline warns.

## Patterns detected
- **Canonicalize-and-confine, `std` form** (`crates/workspace-detector/src/contract.rs:93-99`): canonicalize
  BOTH sides, then `starts_with`, then a typed `Error::PathTraversalRejected { reason }`. The repo's only
  working confinement.
- **Bounded cross-process read** (`crates/workspace-detector/src/contract.rs:115-126`): byte cap first, then
  UTF-8, then trim, then reject empty + `is_control()`; every failure yields `None` and falls back. This is
  the shape Deliverable B's prompt bound should mirror.
- **Traversal-component pre-check** (`crates/plugins/src/loader.rs:156-178`): reject `Component::ParentDir`
  BEFORE canonicalizing, so a traversal is refused even when the target resolves.
- **Graceful-degraded model boot** (`llamacli_inference.rs:118`, `:210-230`): a failed path resolve yields
  `None` at construction and `ModelStatus::Error` → `InferenceError::ModelNotConfigured` at load — never a
  panic, never a boot block. Any new rejection must join this path.
- **Structured error observable already in the file** (`llamacli_inference.rs:576-582`, `:605-611`): WARN on
  `interpretation.inference.error` carrying `error_category` + `recovery_action = "skip_digest"`. The
  in-file precedent for the rejection record's field shape.

## Conventions to follow
- **Tests live in `pulse-app/tests/*.rs`, never co-located** — `pulse-app`'s `[lib] test = false` compiles a
  `#[cfg(test)] mod tests` and never runs it. Both target files already exist
  (`unit_llamacli_inference.rs:562`, `integration_real_llama_cli.rs:173`), so this chunk EXTENDS, creating none.
- **Env consts are `pub` and imported by tests** (`unit_llamacli_inference.rs:32`,
  `integration_real_llama_cli.rs:44`) — a new env const follows the same `pub const` shape.
- **Bounded env parse with a stated default** — `read_env_path`'s trim + reject-empty is the in-file form;
  the registered project-wide form is `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` (resolve once at boot,
  every rejection falls to a default, one WARN when non-default).
- **The guard stays inside the concrete impl.** `LlmInferenceRunner` is the swap boundary; the deterministic
  runner (`deterministic_inference.rs:121`) and `investigate_router.rs:254` are separate implementations that
  must not be touched.

## New files to create
- (none) — every change lands in one existing production file and two existing test files.

## Files to modify
- `pulse-app/src/llamacli_inference.rs` — the path guard (extend `canonicalize_path` / its call sites at
  `:116-117`), the prompt bound (before `build_llama_cli_args` at `:530`, or inside it at `:408-409`), and the
  rejection emit.
- `pulse-app/src/observability.rs` — complete the `interpretation.model.load.error` leaf (`:1969-1979`) with
  the `*_basename` field; the leaf itself already exists.
- `pulse-app/tests/unit_llamacli_inference.rs` — extend; it already imports all three env consts (`:32`) and
  pins the canonicalize branches and every argv field.
- `pulse-app/tests/integration_real_llama_cli.rs` — extend only if the guard changes the env-gated leg's
  skip conditions (it reads all three vars at `:44`, `:70-85`).
- `Cargo.toml` / `pulse-app/Cargo.toml` — only if the P-2 fork resolves toward adopting `strict-path`; the
  workspace dep already exists at `:49` and `pulse-app` already declares it, so adoption needs **no manifest
  change at all** — only an `use`.
- **Registry/allowlist members that ride this list:** `pulse-app/src/observability.rs` is the code-side
  allowlist (above). The arch §Occupied Resources and security-plan entries are spec-master registry entries →
  Expected amendments at wrap, NOT touchpoints.
- **Matrix anchors:** `grep` of `verification-matrix.json` for `llamacli_inference` returns ONE hit, inside
  P-077's `notes` prose (naming `:216` / `:239` as emit sites). No `ref` grep-anchor points into this file, so
  `cargo xtask verify:capability-matrix` cannot break on the edit.

## The load-bearing equality
The design needs: **`strict-path`'s `PathBoundary` + `strict_join` yields "resolved path is under root R and is
a regular file" for THESE inputs** — an operator-declared root R that is NOT the data dir, holding a
user-managed GGUF and `llama-cli.exe`. Verified that the API expresses it (`PathBoundary::try_new`,
`strict_join`, `boundary_check`, `canonicalize`, `canonicalize_anchored`, `metadata`, plus the `VirtualRoot`
sibling). **Not verified, and it decides the implementation rather than the plan:** its behaviour on Windows
`\\?\` extended-length paths, which is the form `std::fs::canonicalize` already returns on this host — so a
naive `starts_with` between a `\\?\`-prefixed child and a bare-prefixed root would fail open or closed
depending on which side is canonicalized. The `publish_workspace_key` precedent avoids this by canonicalizing
BOTH sides; any `strict-path` adoption must be checked for the same property.

## Open questions
- Should the allow-root be a NEW env var, or should confinement be asserted against the *directory the
  supplied path already lives in* (a no-new-resource form)? → blocks: **plan-decision** (P4 resolves before
  synthesis; it changes whether arch §Occupied Resources gains an entry).
- Does adopting `strict-path` — its first use in the repo — belong in this chunk, given both paths owe an
  amendment (adopting costs an arch §Stack row; declining contradicts a named security mandate)? → blocks:
  **plan-decision**.
- Does the Windows `\\?\` form change the confinement comparison? → blocks: **implementation-scope** (the
  file list stands either way; /implement measures it).
