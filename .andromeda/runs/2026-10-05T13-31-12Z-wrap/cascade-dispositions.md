# Cascade dispositions — 2026-10-05-l4-model-chosen-by-pattern-discrimination

The search: `cascade.py sweep --patterns-file cascade-patterns.toml` (baseline `2dc099a6`, the pre-CI parent), run
after the last body amendment of the pass. Ten patterns, from every amendment:
- architecture (Model clause + [Fault Identity] tail): `chosen by the pattern-discrimination` · `which chooses the
  model|chooses the (L4 )?model` · `not automatically Qwen3\.5-2B` · `pattern[- ]discrimination` ·
  `Llama-3.2-3B-Instruct-Q4_K_M` — the claim's own verb (chooses / is chosen) and its actor (the route entry), not
  only the entry's name;
- security-plan (lockstep re-base): `7,?405` · `2\.21` · `8,?979|8\.8 KiB`;
- test-plan §1 probe row: `29 collected|to 29 |\b29 pins|pins 16 . 29` · the old parse-pinned flag list.
Every pattern's known-positive control fired on the pre-pass masters (architecture.md:71/:72, security-plan.md:139,
test-plan.md:144). Three patterns returned 0 rows after the pass (`arch-chooses`, `sec-ratio`, `sec-headroom`): their
controls fired, so the retired ratio, headroom and "which chooses the model" wording are gone from every master,
leaf, curation home and judgment base.

## Rows

| row | disposition |
|---|---|
| `.claude/docs/services/interpretation.md:30` arch-chosen-by · arch-not-auto · arch-pattern ×2 · arch-llama (leaf) | STALE — re-derived in step 3 from the amended architecture: the founder's pick (gemma-4-E4B-it-Q4_K_M) and how it was chosen; Llama stays shipped until the next entry |
| `architecture.md:71` arch-pattern ×2 (standing, edited) | this pass's own text (the Model clause) — true |
| `architecture.md:71` arch-llama ×2 @c1656, @c3115 (standing, edited) | true: "the shipped GGUF is Llama-3.2-3B-Instruct-Q4_K_M" and "until it lands the shipped GGUF stays Llama-3.2-3B-Instruct-Q4_K_M" |
| `architecture.md:72` arch-pattern ×2 (new) | this pass's own text ([Fault Identity] tail) — true |
| `architecture.md:72` arch-llama @c6523 (standing, edited) | true: the v2.5 framing series' measured model, a past measurement (window read by offset at Validate check 4) |
| `security-plan.md:139` arch-pattern ×2 · `:461` arch-pattern (new) | this pass's own text (the 7,575 B entries) — true |
| `security-plan.md:139` sec-7405 · `:461` sec-7405 (standing, edited) | true: kept as the prior entry of the measurement chain ("measured 7,405 B" / "before it 7,405 B"), no longer the maximum |
| `test-plan.md:144` tests-29 (standing, edited) | true: "Extended at chunk 2026-10-05-l4-runs-on-a-small-current-model-chosen-by-measurement to 29 collected pins" — an earlier extension, followed now by the 61-pin one |
| `test-plan.md:144` tests-flags (new) | this pass's own text: the widened parse-pinned list begins with the old four flags |
| `test-plan.md:144` arch-pattern (new) | this pass's own text — true |

Curation homes: 0 rows for every pattern. Judgment bases (`playbook.md`, `drift-base.md`): 0 rows.

## Leaves (step 3)

- architecture → CLAUDE.md `GENERATED:setup:*` recomputed from arch's sections: the overview / modules / stack lines
  name the llama.cpp runtime and the `interpretation` crate, never a chosen or shipped model → no change.
  `docs/stack.md` (§Stack leaf): states the runtime, argv constants and tiers, no model choice → no change.
  `docs/services/interpretation.md` (the [LLM Inference Runtime] module leaf, provenance "Extracted from
  architecture.md"): line 30 re-derived (above); line 40 ("Measurement tool: `pulse-app/examples/l4_decision_probe.rs`
  reuses the product argv builder") still true.
- security-plan → `.claude/docs/security-summary.md`, `.claude/rules/security.md`, CLAUDE.md
  `GENERATED:setup:warnings`: none states the observed prompt maximum, its ratio or its headroom (grep `observed
  max|headroom|16384|16,384|MAX_PROMPT` over CLAUDE.md, `.claude/rules/*`, `.claude/docs/**`: 0 leaf hits) → no
  change.
- test-plan → `.claude/docs/tests-summary.md`, `.claude/rules/testing.md`, CLAUDE.md warnings: none states the
  probe's pin count or its parse-pinned flag list (grep `decision.probe|l4_decision`: only
  `docs/services/interpretation.md:40`, above) → no change.
- Lateral binds: test-plan §3 ↔ obs-plan §3 and a11y ↔ obs schema untouched by this pass.
