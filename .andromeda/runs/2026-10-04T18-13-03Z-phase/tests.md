# tests extract

## Relevance
partial — items 1–3 (prompt framing in `crates/interpretation`, the bounded probe label in `pulse-app/examples/l4_decision_probe.rs`) are unit-testable now; item 4 (the real-model measurement) is a non-deterministic dev-probe run that per test-plan §1 cannot be a test or a gate, and it is gated anyway.

## Constraints
- The chunk plan's `## Test Commands` MUST carry the unconditional standard gate set, in the plan's order: capability-drift BEFORE the default-features workspace nextest, the `--features mcp-server` `emit_taurpc_bindings` regen as the LAST cargo-adjacent step, then the `git diff --quiet <chunk-base> -- pulse-app/ui/src/bindings/index.ts` close (this chunk adds no procedure); the webview npm gates drop out only if zero `pulse-app/ui/**` paths are touched (per test-plan §3 Per-chunk gate discipline).
- The prompt-surface changes belong to the interpretation crate's co-located `#[cfg(test)] mod tests` pins in `prompt.rs`; test-plan §4 requires the delimited-section discipline already specified for the citable-evidence surface (rendered in ALL THREE tier builders, section ORDER pinned, explicit none-instruction when empty rather than an omitted section) — whether the corpus block and a trigger-naming section are already pinned this way per tier is research's question (per test-plan §4 "What unit tests cover" → interpretation crate).
- Prompt versioning pins ride the `PROMPT_VERSION_*` consts so a lineage bump is asserted without literal duplication; if this chunk changes template text that the lineage tracks, the version consts and their pins move together across the three tiers (per test-plan §4 interpretation crate bullet).
- `l4_decision_probe` is an `[[example]]` that is "neither a test nor a gate"; any new label logic tested there needs `[[example]] … test = true` in the manifest or nextest collects nothing — the open coverage trigger `l4-decision-probe-arg-parse-unit-coverage` names its deterministic halves as owed per-branch tests (per test-plan §1 Pending coverage triggers, rows `l4-decision-probe-arg-parse-unit-coverage` and the `inject-demo-arg-parse-unit-coverage` correction).
- Coverage thresholds stand at ≥75% line / ≥70% branch / ≥85% function (xtask excluded temporarily); new deterministic code (label classifier, framing builders) must not drag them down (per test-plan §10 Coverage thresholds).
- No real-model generation enters the deterministic suite: tests use no real network/time/host state, and fixtures are generated at runtime (per test-plan §11 Universal and §7 Self-bootstrapping requirement); the real-GGUF run of item 4 is evidence, not a test verdict (per test-plan §1 `l4-decision-probe-arg-parse-unit-coverage`).
- Pure functions are tested directly, never mocked; traits only via constructor-injected `Arc<dyn Trait>` (per test-plan §8 What NOT to mock / Anti-monkey-patching).

## Patterns to follow
- The citable-evidence prompt pins (marker-delimited section rendered verbatim between markers in all three builders; order-after-Current-Digest / before-Corpus-Retrieval pin; empty-input none-instruction pin) as the template for a "past or other incidents" corpus-framing pin and a "triggering cue" section pin (per test-plan §4 interpretation crate bullet).
- Schema/order pins that encode a measured model behaviour with a cited evidence file, e.g. `primary_prompt_schema_lists_decision_and_severity_after_the_analysis` (per test-plan §4 interpretation crate bullet).
- `#[rstest]` case tables for closed-set classifiers — the 8-case `cuda_probe_` matrix plus exact-set pin on the production constants is the nearest precedent for a closed names-the-trigger label set (per test-plan §4 interpretation crate bullet and §7 Fixture library).
- RED-before-green against the pre-chunk prompt plus mutation checks (drop the framing / the cue name and confirm specific pins redden), recorded as evidence (per test-plan §4 interpretation crate bullet, the RED 8/13 + mutation-checked precedent).
- Dev-tool bounded-input boundaries get a per-branch test (valid / unknown / out-of-set) (per test-plan §1 `inject-demo-arg-parse-unit-coverage` precedent row).

## Anti-patterns to avoid
- NEVER assert on fixture-internal details or private internals — assert the public contract (marker presence/order, closed label value) (per test-plan §11 Unit).
- NEVER expose model output or raw attribute text in test output or probe artifacts — the names-the-trigger label is a closed label, never title text (per test-plan §11 Universal "NEVER expose secrets in test output").
- NEVER add retry-once or `#[ignore]` without an issue to paper over a non-deterministic real-model leg (per test-plan §11 Quality).

## Contract bindings
- tests ↔ security: the ASCII-only template constraint is enforced by an existing interpretation-crate pin (`composed_prompt_templates_are_ascii_clean_for_argv_transport`, named in the scope's Boundaries); any added framing text must keep it green — test-plan does not itself anchor this pin, so the binding is to security-plan's argv rule.
- tests ↔ obs: the probe's bounded-label output is the bounded-label/NEVER-log discipline (obs/security side); the test side owes a closed-set pin on the label values (per test-plan §1 `l4-decision-probe-arg-parse-unit-coverage`).
- tests ↔ verification matrix: real-model behaviour sits in the env-gated-runtime mode of `cargo xtask verify:capability-matrix`, not in nextest (per test-plan §9 capability-matrix paragraph).

## Acceptance criteria contributions
- `cargo nextest run -p interpretation` passes with new pins proving the corpus block is framed as past/other incidents and the triggering cue is named, in every tier builder that carries the corpus section, each RED against the pre-chunk prompt (per test-plan §4 interpretation crate bullet).
- If the probe gains classifier logic, its `[[example]]` declares `test = true` and the workspace nextest count rises by exactly the new probe pins, covering every label in the closed set plus an unknown/absent arm (per test-plan §1 `l4-decision-probe-arg-parse-unit-coverage`).
- The full standard gate set passes in the mandated order and the bindings close `git diff --quiet <chunk-base> -- pulse-app/ui/src/bindings/index.ts` exits 0 (per test-plan §3 Per-chunk gate discipline).
- Workspace coverage stays ≥75% line / ≥70% branch / ≥85% function (per test-plan §10 Coverage thresholds).
