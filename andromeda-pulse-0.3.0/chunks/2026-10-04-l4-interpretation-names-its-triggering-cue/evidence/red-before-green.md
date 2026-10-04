# Red before green — Step 1

One-shot control, run 2026-10-04 on the tree with the pins and the three consts
(`TRIGGER_LINE_PREFIX`, `CORPUS_MATCHES_FRAMING_NOTE`, `TRIGGER_FRAMING_INSTRUCTION`) declared,
but before any emission code or lineage bump.

Command: `cargo nextest run --workspace --profile ci -E 'test(/render_payload_names_the_trigger|render_payload_omits_the_trigger|render_payload_frames_corpus|render_payload_omits_the_corpus_framing|render_payload_framing_lines|framing_instruction_present|primary_prompt_schema_lists_decision_and_severity_after_the_analysis/)' --no-fail-fast`

Exit 100 · `Summary 7 tests run: 2 passed, 5 failed`.

## RED (5)

| pin | assertion line |
|---|---|
| `render_payload_names_the_trigger_from_the_first_cue` | `assembler.rs:1175` — `exactly one TRIGGER line` · left `0`, right `1` |
| `render_payload_frames_corpus_matches_as_other_incidents` | `assembler.rs:1204` — `the framing note directly follows the header` · left `Some("  - [abcd] Error-rate spike: past incident - 3m ago, active")`, right `Some("  (other or past incidents - context only, not the signal this digest reports)")` |
| `render_payload_framing_lines_are_ascii` | `assembler.rs:1236` — `ErrorRateSpike: the render carries a TRIGGER line` |
| `framing_instruction_present_in_every_tier_output_instructions` | `prompt.rs:583` — `primary: the framing instruction appears exactly once in Output Instructions` · left `0`, right `1` |
| `primary_prompt_schema_lists_decision_and_severity_after_the_analysis` (the lineage pin, edited) | `prompt.rs:559` — left `"v2.3"`, right `"v2.4"` |

## Green on the base by construction (2)

The conditional-property asymmetry (testing.md 2026-08-17): the "omits … without" arms hold on a
tree that never emits either line, so they cannot be red here. They carry the guard only as the
negative half of their pair.

- `render_payload_omits_the_trigger_line_without_a_cue` — PASS
- `render_payload_omits_the_corpus_framing_note_without_matches` — PASS

The plan predicted the lineage pin green on the base; it measured RED, because the pin was moved
to the new versions before the bump. Recorded as measured.
