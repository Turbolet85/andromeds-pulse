# tests extract

## Relevance
relevant: the chunk changes L4 prompt and/or digest text that in-crate pins guard, extends the dev-only probe's pinned surface, and its PREREQ is a standard-gate member (clippy). The real-model series itself is evidence, never a test.

## Constraints
- The plan's `## Test Commands` carries the full standard gate set unconditionally, per test-plan §3 Per-chunk gate discipline. That set includes `cargo clippy --workspace --all-targets --all-features -- -D warnings`, which is the PREREQ this chunk closes. The section also fixes the order: `capability-drift` BEFORE the default-features workspace nextest; the `--features mcp-server` `emit_taurpc_bindings` regen as the LAST cargo-adjacent step; and, for a chunk that adds no TauRPC procedure, `git diff --quiet <chunk-base> -- pulse-app/ui/src/bindings/index.ts` as the close. The webview npm gates drop out only if no `pulse-app/ui/**` path is touched.
- A real-model generation is never a gate (per test-plan §1 `l4-decision-probe-arg-parse-unit-coverage`), and every test layer must be deterministic and agent-readable (per test-plan §2 Agent-runnable invariants). So the pre-registered series is recorded measurement in `evidence/`, while pass/fail for the gate set comes only from deterministic pins.
- Prompt-surface pins live in the interpretation crate (per test-plan §4 interpretation crate). The `PROMPT_VERSION_*` lineage (v2.4 primary, v1.3 fallback, v1.3 reflection) is asserted through the consts, never through duplicated literals, and the three versions bump together. `TRIGGER_FRAMING_INSTRUCTION` must appear exactly once in every tier's Output Instructions, after `CITING_INSTRUCTION` (`framing_instruction_present_in_every_tier_output_instructions`). ASCII cleanliness rides `composed_prompt_templates_are_ascii_clean_for_argv_transport`, and the schema property order is pinned by `primary_prompt_schema_lists_decision_and_severity_after_the_analysis`. Changed wording must keep each of these pins true, or update it deliberately with the reason. Whether a version bump is owed, and which test literals the sweep must catch, is research's question.
- Digest framing pins live in the triage crate (per test-plan §4 triage crate). There must be exactly one `TRIGGER: {cue_cause_label}` line, keyed on the FIRST cue, directly after `OVERALL:`, and none without a cue. The static corpus framing note sits under `CORPUS MATCHES:` only when matches exist. Five `render_payload_*` tests pin this across every `CueKind`. A change to the TRIGGER line's content must keep the first-cue keying (the Fault Identity tie). The two "omits" pins guard only as the negative half of their pairs.
- The probe `pulse-app/examples/l4_decision_probe.rs` declares `test = true` and carries 8 collected pins, per test-plan §1 `l4-decision-probe-arg-parse-unit-coverage`. They cover:
  - the closed `names_trigger` label set (`rank1` / `elsewhere` / `none` / `unparsed`);
  - the `nf` arm stripping exactly the three framing lines, with each removal count-checked;
  - the S4 framed-corpus-match rendering;
  - every arm × shape composing within the production bound.

  If the remedy adds or changes framing lines, the `nf` strip count and the composition-within-bound pin must follow. The flag parse, the INCONCLUSIVE exit 2, the A2/A4 transforms and `first_keys` stay OWED (they are not this chunk's to discharge unless it touches them).
- pulse-app unit probes live in `pulse-app/tests/*.rs` or in the `test = true` example, never in lib sources (`[lib] test = false`), per test-plan §4 Conventions. Library-crate tests are co-located `#[cfg(test)] mod tests`.
- Flakiness budget is zero and retry-once is banned, per test-plan §10 Zero-flakiness budget and §11 Quality. A deterministic pin that flakes is quarantined and root-caused, never retried.

## Patterns to follow
- Mutation-check every new or changed pin: revert the product change alone and show the pin reddens. This is per test-plan §4 interpretation crate (removing the framing instruction from the fallback builder alone reds its pin) and §4 triage crate (keying TRIGGER on the LAST cue reds the first-cue pin).
- Pair a presence pin with its absence pin, and use `#[rstest]` over every `CueKind`. The `render_payload_*` set in test-plan §4 triage crate is the model.
- Exit contract for a measuring tool is 0 PASS · 1 FAIL · 2 INCONCLUSIVE. An unmet precondition (unset or guard-rejected L4 path) reports INCONCLUSIVE, never PASS. This follows test-plan §3 Per-chunk gate discipline (the scenario-leg family) and the probe's INCONCLUSIVE arm in §1.
- The probe's L4 path reads reuse the already-pinned `validate_path_input`, per test-plan §1 `l4-decision-probe-arg-parse-unit-coverage`. Do not add a second path guard.

## Anti-patterns to avoid
- NEVER add retry-once policies, and never re-run a FAILing measurement until it passes, per test-plan §11 Quality. This mirrors the scope's run-once rule.
- NEVER lower a threshold to pass, per test-plan §11 CI. Here that means the pre-registered bar and the `names_trigger` grader are fixed before the first generation, never after.
- NEVER expose captured content in test output, per test-plan §11 Universal. Probe evidence carries bounded labels and verdict lines only, with no model text.

## Contract bindings
- tests ↔ security: the ASCII argv-transport pin `composed_prompt_templates_are_ascii_clean_for_argv_transport` and the probe's every-arm-within-production-bound pin (`MAX_PROMPT_BYTES`) are the test-side carriers of security.md's argv-transport and prompt-bound rules (per test-plan §4 interpretation crate; §1 `l4-decision-probe-arg-parse-unit-coverage`).
- tests ↔ arch §Established Decisions [Fault Identity]: the triage first-cue TRIGGER pin `render_payload_names_the_trigger_from_the_first_cue` is what holds "framing keyed on the SAME first cue as the identity" (per test-plan §4 triage crate).
- tests ↔ CI: the clippy PREREQ closes inside the standard gate set (per test-plan §3 Per-chunk gate discipline), and lint-test runs the same command in CI (per test-plan §9 Pipeline structure).

## Acceptance criteria contributions
- (tests) The standard gate set exits green in the documented order, including `cargo clippy --workspace --all-targets --all-features -- -D warnings`. The bindings close `git diff --quiet <chunk-base> -- pulse-app/ui/src/bindings/index.ts` exits 0 (per test-plan §3 Per-chunk gate discipline).
- (tests) Each changed prompt, schema-description or digest literal is pinned by an in-crate test that a revert-only mutation reddens. Any `PROMPT_VERSION_*` bump is asserted through the consts across all three tiers (per test-plan §4 interpretation crate / §4 triage crate).
- (tests) The probe's collected pins pass under `cargo nextest run --workspace --profile ci`, including the `nf` strip count and every arm × shape within the production bound for any new or changed arm (per test-plan §1 `l4-decision-probe-arg-parse-unit-coverage`).
- (tests) The real-model series is reported as measured evidence, never as a gate verdict. No deterministic pin depends on a generation (per test-plan §1 `l4-decision-probe-arg-parse-unit-coverage`; §2 Agent-runnable invariants).
