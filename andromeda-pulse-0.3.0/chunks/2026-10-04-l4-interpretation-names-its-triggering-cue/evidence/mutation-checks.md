# Mutation checks — Step 12

One-shot controls, 2026-10-04. Each mutation was applied with the anchored Edit tool, the named
pins were run with `cargo nextest run --workspace --profile ci -E …`, and the mutation was then
restored. Restoration was confirmed by grep: `cues.first()` ×1 in assembler.rs, the note push ×1,
`prompt.push_str(TRIGGER_FRAMING_INSTRUCTION)` ×3 in prompt.rs, no title-first branch in the probe.

| | mutation | pins run | result |
|---|---|---|---|
| (a) | `render_payload` keys the TRIGGER line on `cues.last()` | `render_payload_names_the_trigger_from_the_first_cue` | RED — left `"TRIGGER: Error-rate spike"`, right `"TRIGGER: Retry storm"` (exit 100, 1 run 0 passed) |
| (b) | the note push replaced by `let _ = CORPUS_MATCHES_FRAMING_NOTE;` | `render_payload_frames_corpus_matches_as_other_incidents`, `s4_renders_a_framed_corpus_match_beside_the_trigger` | both RED (exit 100, 2 run 0 passed) |
| (c) | `TRIGGER_FRAMING_INSTRUCTION` removed from the fallback builder only | `framing_instruction_present_in_every_tier_output_instructions` | RED — `fallback: the framing instruction appears exactly once in Output Instructions` (exit 100) |
| (d) | `names_trigger` returns `elsewhere` when the title names the term, before the rank-1 read | `names_trigger_reads_rank1_…`, `names_trigger_reads_elsewhere_…` | rank1 pin RED — left `"elsewhere"`, right `"rank1"`; elsewhere pin GREEN (exit 100, 1 passed 1 failed) |

## Notes

- (d): the plan expected both the rank1 and the elsewhere pins to go red. The elsewhere pin stays
  green by construction: none of its three fixtures names the retry in the first hypothesis, so
  reading the title first cannot change their label. The rank1 pin's fixture names the retry in both
  the title and the first hypothesis, and it is the one that discriminates title-first ordering.
- (d) was first attempted against a pre-format anchor. The Edit errored (`String to replace not
  found`), and the run that followed was a green on the UNMUTATED tree. It was discarded as a failed
  mutation (testing.md 2026-08-17), the anchor was re-read, and the mutation was re-applied and
  confirmed by grep before the recorded run.
