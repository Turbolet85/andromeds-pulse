# Cascade dispositions — 2026-10-04-l4-framing-measured-on-the-real-model

One amendment this pass: architecture §Established Decisions → [Fault Identity]. The retired claim is "whether it
moves the model's rank-1 hypothesis is UNMEASURED: the pre-registered real-model series … is gated on a Linux llama.cpp
CUDA binary". It now says the series FAILS the pre-registered bar, with the counts, at its measured boundary.

## The search
- **Patterns:** `cascade-patterns.toml`, run by `cascade.py sweep` against baseline `4c9e05e1`, over the seven masters
  + `.andromeda/registries/**` + the three curation homes + the two judgment bases + the leaf bodies:
  - `gated-cuda` (fixed "gated on a Linux llama.cpp CUDA binary")
  - `rank1-unmeasured` (the UNMEASURED phrasing)
  - `min-rank1`
  - `trigger-framing` (`trigger framing` / `TRIGGER_FRAMING_INSTRUCTION`)
  - `names-trigger`
  - `linux-cuda`
  - `decision-probe` (`l4_decision_probe`)
- **Mechanism read, not only tokens.** The retired claim asserts two things: that the effect is not yet measured, and
  that a missing CUDA binary gates it. Both phrasings were searched (`gated-cuda`, `rank1-unmeasured`, `linux-cuda`).
  Each pattern's control fired on the pre-pass `architecture.md:72`; `names-trigger`'s fired on `test-plan.md:144`.
- **By hand**, for any leaf restating the effect as not yet measured:
  - `grep -rn -i 'unmeasured|not yet measured|pre-registered|gated on'` over the docs leaves (`stack.md`,
    `conventions.md`, `gotchas.md`, `services/triage.md`, `services/interpretation.md`): 0 hits.
  - `grep 'trigger|framing|TRIGGER'` over CLAUDE.md: 1 hit, line 50.

## Rows
- `gated-cuda`: 0 rows. The single pre-pass site was `architecture.md:72`, now amended.
- `rank1-unmeasured`: 0 rows. Same single site, amended.
- `architecture.md:72` carries `min-rank1`, `trigger-framing` and `decision-probe`, each marked `edited`. **Amended**:
  this is the new text. Re-read for an intra-line duplicate: the UNMEASURED clause stands nowhere else on the line
  (6848 chars; `gated-cuda` and `rank1-unmeasured` read 0).
- `test-plan.md:144` carries `min-rank1`, `names-trigger` and `decision-probe`. **No change**, because it is a true
  claim sharing the tokens. The `l4-decision-probe-arg-parse-unit-coverage` row names `--min-rank1` as a flag whose
  parse is still owed a test. This chunk edited no probe and added no pin, so the row stands.
- `test-plan.md:385` @c988 (`trigger-framing`). **No change**, a true claim: it names the unit pin that asserts
  `TRIGGER_FRAMING_INSTRUCTION` sits once per tier, which is unchanged. Read through `cascade.py window --at 988`.
- `test-plan.md:385` @c1999 (`linux-cuda`). **No change**, a true claim about a different subject: the hardware
  probe's CUDA candidate-set pins. Read through `cascade.py window --at 1999`. The line's other `real-model` hits
  (@c898, @c1936) are chunk-marker spellings.
- Curation homes 0 · judgment bases 0 · leaves 0, for every pattern.

## Leaves (step 3)
- **Enumerated by provenance** (playbook rule, arch leaf enumeration): `_Extracted from architecture.md_` heads
  `.claude/docs/stack.md` and `.claude/docs/conventions.md`. Add the CLAUDE.md `GENERATED:setup:*` blocks.
- **Recomputed from the amended §Established Decisions:**
  - CLAUDE.md `GENERATED:setup:warnings` line 50, the Fault-identity invariant. It states identity, the cue-kind title
    text and the TRIGGER line as framing, never identity. All still hold. The block states no measurement status, so
    it is unchanged.
  - `stack.md` and `conventions.md` carry no [Fault Identity] restatement and no series claim. Unchanged.
  - `services/triage.md:28` describes the TRIGGER line and the corpus-match framing note as rendered. That is still
    true, so it is unchanged.
- **Lateral binds:** test §3 ↔ obs §3 and a11y ↔ obs schema are not touched.
