# Cascade dispositions — 2026-10-05-l4-runs-the-founder-s-pick-with-its-authors-settings

## The search
`cascade.py sweep --patterns-file cascade-patterns.toml` over the seven masters (pre-pass baseline `7663dd91`, the pre-CI
parent), `.andromeda/registries/**`, the three curation homes, the two judgment bases and every leaf. Patterns, each
controlled on a pre-pass hit (a ninth, `trivially pass`, never fired on the pre-pass masters and was dropped — the
masters never stated the grader's vacuity):

| id | what it retires | phrasings covered |
|---|---|---|
| `schema-file` | the `--json-schema-file` constraint mechanism | the flag token |
| `schema-constr` | the same mechanism in prose | `JSON-schema-constrained` · `schema-constrained GBNF` · `native JSON-schema` |
| `shipped-llama` | "the shipped model is / stays Llama-3.2-3B" | `shipped (GGUF|model) … Llama|stays` · `stays Llama-3.2` |
| `two-outside` | "TWO product-written locations outside the data dir" | `two/second product-written` · `TWO/SECOND deliberate` · `one of the two product-written` |
| `gpu-5s` | the 5000 ms / < 5 s gpu-primary budget | `5000 ms gpu-primary` · `gpu-primary budget` · `< 5 s on gpu-primary` · the matrix cell form |
| `zero-muted` | "the muted-diagnostic backlog is ZERO / no owner remains" | `ZERO targets remain muted` · `No owner remains` |
| `tests-101` | the 101 `pulse-app/tests/*.rs` count | `101 (top-level) pulse-app/tests` · `101 files` |
| `gbnf-flag` | the probe's `--gbnf` flag | the flag token |

A phrasing the patterns did not carry, found by a manual leaf grep (`data-dir escape|second product|two
product-written|written locations outside`): CLAUDE.md:31 "the second data-dir escape after `~/Downloads`" — re-derived.

## First listing (before step 3) — every row
- architecture.md:72 `schema-file` ×3 (@c350, @c3021, @c5037) — new text this pass (never `--json-schema-file`; the
  chunk-base argv parsed 0/4; the prefill trap) — true, no change.
- architecture.md:221 `schema-file` — the new temp-file bullet's history ("since chunk #84 it held the JSON schema for
  `--json-schema-file`") — true history, no change.
- test-plan.md:144 `schema-file` ×2 — @c2581 the 29-pin extension's record of the `gb` arm (history); @c5309 this
  pass's mutation note — no change.
- test-plan.md:144 `gbnf-flag` ×4 — @c2489 the 29-pin extension's history; @c4715/4784/4812 this pass's retirement text
  — no change.
- architecture.md:72/73 `shipped-llama` — this pass's "Before it the shipped GGUF was …" / "while the shipped model was
  still …" — true history, no change.
- architecture.md:72 `gpu-5s` — "sat above the then gpu-primary budget of 5000 ms, which the founder raised" — this
  pass's text, true; test-plan.md:145 — the new row's "gpu-primary budget 10000 ms" — true.
- `zero-muted`, `tests-101`, `schema-constr` — 0 rows after the pass (their sites amended: obs-plan.md:553/565,
  test-plan.md:355, architecture.md:30/71/367).
- LEAF rows → step 3: `.claude/docs/services/interpretation.md:30/31/35/41` (shipped model, argv, prefill trap, the
  probe's `gb`/`--gbnf`) RE-DERIVED · `.claude/docs/stack.md:28` (constraint mechanism) RE-DERIVED;
  `.claude/docs/stack.md:24` (the session-144 spike's "122.3 tok/sec under L4 `--json-schema-file` GBNF") — a dated
  historical measurement, kept · `.claude/rules/security.md:17` RE-DERIVED · `.claude/rules/security.md:56` ("a second
  deliberate NO-SCRUB … log boundary") — a different claim sharing the token, no change ·
  `.claude/docs/services/interpretation.md:30` `gpu-5s` RE-DERIVED · `.claude/rules/testing.md:25` RE-DERIVED.

## Step 3 — leaves re-derived (by the DAG table and provenance)
- architecture → CLAUDE.md `GENERATED:setup:overview` (Stack line: the shipped model, its sampling, the GBNF via
  `--grammar-file`, the vendored converter) and `GENERATED:setup:modules` (corpus line: three out-of-data-dir locations);
  `.claude/docs/stack.md` (§AI/LLM Inference: invocation, constraint mechanism, shipped model);
  `.claude/docs/services/interpretation.md` (trait accessor, shipped model, argv, prefill trap, grammar provenance,
  probe). `.claude/docs/conventions.md` / `gotchas.md` / `commands.md` — no amended arch section feeds them (grep for
  the patterns: 0 rows).
- security-plan → `.claude/rules/security.md` (path-var rule: three locations + first-party argv operands; full-path
  never-log set; §Supply chain: the vendored channel) · `.claude/docs/security-summary.md` (argv operands + three
  locations; the vendored channel). CLAUDE.md `GENERATED:setup:warnings` re-read — states no location count, no change.
- test-plan → `.claude/rules/testing.md` (102 files) · `.claude/docs/tests-summary.md` (§CI integration: the Python 3
  test-time requirement). Its pending-trigger list carries no L4 row — no change.
- obs-plan → `.claude/rules/observability.md` (backlog recurrence OPEN; §SLO the L4 latency budget) ·
  `.claude/docs/obs-summary.md` (an L4 row in the SLO table; backlog recurrence).

## Lateral binds
test-plan §3 ↔ obs-plan §3 (harness) — neither section amended. a11y schema ↔ obs schema — neither amended.

## Final listing (after step 3)
Every remaining row is current text or history: the four leaf `schema-file` rows read "never `--json-schema-file`" /
the dated spike measurement; `services/interpretation.md:34` `shipped-llama` is "the shipped model's published sampling"
(a false positive — no Llama claim); `:39` `gbnf-flag` is the retirement statement; `rules/security.md:56` is the
unrelated NO-SCRUB boundary. 0 stale rows.
