# Codebase Research — 2026-10-04-l4-interpretation-names-its-triggering-cue

## Scope
- **Depth:** deep · **Reads:** 14 · **Globs/Greps:** 16
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — structural extraction (`grep -n` of the 10
  `##` headers + the 22 Session Additions introducers, 60,568 B); 0 additions applied — none concerns the
  `l4_decision_probe` example or a llama-cli run. This chunk's only live leg is the GATED real-model probe run, which
  launches no app, binds no port and reads no `agent-latest.jsonl`.
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg (Setup
  5a read `5d6e344` as `in progress`, not red).

## Files inspected
- `crates/interpretation/src/prompt.rs` (1-460, 515-595, 795-835; test index 436-958) — the three tier builders
  (`:182`, `:272`, `:360`) share one section order: Role → Conventions → Schema → Project Context → Current Digest
  → Citable Evidence Ids → optional `# Corpus Retrieval (past similar incidents)` → Output Instructions. The corpus
  block renders only when `corpus_retrieval` is non-empty (`:239`, `:329`, `:417`). ROLE_DEFINITION (`:85-91`),
  CONVENTIONS_SNIPPET (`:96-103`) and OUTPUT_REMINDER (`:108-112`) say nothing about which digest line the
  interpretation is ABOUT. 40 co-located tests, including the ASCII pin
  `composed_prompt_templates_are_ascii_clean_for_argv_transport` (`:799`) and the version pin
  `assert_eq!(PROMPT_VERSION_PRIMARY, "v2.3")` (`:548`).
- `pulse-app/src/inference_runtime.rs` (100-135, 370-440, 790-840) — the production composer. `handle_digest_outcome`
  passes `""` as `corpus_retrieval` to EVERY tier builder (`:412-418`, `:421`, `:427`). The prompt's input is
  `digest.payload_summary` + `workspace=…` + `citable_evidence_ids(digest)` (the cues' full-hex fingerprints,
  `:126-135`). The incident identity's triggering cue is `digest.attention_cues.first()` (`:815`); a reflection digest
  takes the synthetic `ReflectionTrend` identity and carries no cue (`:813-814`). The title grounding
  `grounded_output`/`grounded_title` (`:677-687`) prefixes `cue_cause_label(kind)` onto the MODEL title only. Symptom,
  timeline and hypotheses stay model-authored.
- `crates/triage/src/digest/assembler.rs` (1-30, 240-300, 600-700, 1020-1130) — a cue-triggered digest carries
  exactly ONE `DigestCueRef`, built from the triggering cue (`:259-272`). `render_payload` (`:616-694`) renders it as
  `  [{tier}] {cue_kind_label} — {summary}` under `ATTENTION CUES:` (`:676-686`), e.g.
  `[autonomous] retry_storm — retry_storm scope_id=S`. Corpus matches render as `  - {line}` under a bare
  `CORPUS MATCHES:` header (`:687-692`), with nothing marking them as other or past incidents. Selection runs on the
  window's Q3 fingerprints and ANY Q1 service scope (`:276-287`), so a sibling service's active incident can appear.
  `Digest::scrubbed_clone` skips `corpus_matches`, so each line is scrubbed at this egress (`:276-281`). Tests pin
  `OVERALL:` lines by exact string (`:1061-1121`) and `CORPUS MATCHES:` by presence and absence (`:903`, `:961`).
- `crates/triage/src/digest/retrieval.rs` (120-160) — `format_corpus_match_line` renders
  `[{fingerprint}] {title} — {age}m ago, {outcome}` (`:136-148`), and `{outcome}` can read `active`. Since
  `2026-10-04-retry-storm-interpretation-names-its-cause`, `{title}` carries a `{cue_cause_label}: ` prefix for new
  rows (`Error-rate spike: …`).
- `crates/triage/src/contract.rs` (185-225) — `cue_cause_label(kind)` (`:203-212`) is the closed ASCII vocabulary:
  Error-rate spike · Latency regression · Restart event · Service went silent · Retry storm · Reflection trend.
  `cue_summary` and `render_payload` are re-exported `#[doc(hidden)]` for the probe (`:152`).
- `crates/interpretation/src/schema.rs` (20-60, 120-180) — `PROMPT_VERSION_PRIMARY = "v2.3"`,
  `PROMPT_VERSION_FALLBACK = "v1.2-fallback"`, `PROMPT_VERSION_REFLECTION = "v1.2-reflection"` (`:31`, `:37`, `:48`).
  `L4Output` carries `title`, `symptom`, `timeline` and `hypotheses: Vec<Hypothesis { statement, … }>`
  (`:161-176`). `prompt_version` is model-emitted and only length-validated (`:196`), so a version bump changes no
  parse path.
- `crates/interpretation/Cargo.toml` (18-38) — `interpretation → triage` is a declared normal dependency, so reusing
  `triage::contract::{cue_cause_label, CueKind}` from `interpretation` adds no crate edge.
- `pulse-app/src/investigate_router.rs` (236-262) — `investigate.run_action` calls `build_primary_tier_prompt` with an
  `INVESTIGATION FOCUS:` payload, empty project context, `""` corpus and no citable ids. It carries no digest cue and
  no corpus matches. A static framing sentence about a digest it does not have would reach this caller too.
- `pulse-app/examples/l4_decision_probe.rs` (whole, 663 lines) — renders synthetic Tier1 retry-storm digests through
  the real `render_payload` with `corpus_matches = &[]` (`:241-250`), so NO probe shape carries a corpus line today.
  It composes with the real `build_primary_tier_prompt` (`:255-256`) and validates the production bound (`:276`). Per
  generation it keeps bounded labels only (decision, severity, `is_resolution_summary`, `would_create`, first three
  keys, an output hash: `:543-600`) and never model text. `--dry-run` composes every arm and prints prompt bytes
  without spawning (`:469-488`). The binary comes from `binary_target_for_profile(profile)` (`:490-491`), so a
  GPU-present host needs `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`.
- `pulse-app/Cargo.toml` (18-21) — the probe is declared `[[example]] name/path` with no `test = true`. A
  `#[cfg(test)]` block added to it would never be collected by the workspace nextest (testing.md 2026-08-29).
- `pulse-app/src/llamacli_inference.rs` (`:76`) — `MAX_PROMPT_BYTES = 16 * 1024`, checked at `:662`.
- `andromeda-pulse-0.3.0/chunks/2026-10-04-retry-storm-interpretation-names-its-cause/research.md` (11-50) — the
  half-1 analysis the CONTEXT hypothesis cites. The d3 Timeline `Error Rate Spike in <workspace-key>: 3m ago,
  active` has the corpus-line shape, and the timestamps fit incident 7 (an active error-rate-spike incident 203.75 s
  older). That incident's title is not in the capture, so "the model copied it" is fitted to text shape and
  timestamps, n = 1. The d3 primary prompt measured `token_count=6979`, which is `prompt.len()` in bytes
  (`inference_runtime.rs:433-436`).
- `AI-Model/` (listing + `file`) — the GGUF is present. `llama-b9305-cuda/` holds Windows PE32+ images only
  (`llama-cli.exe`, `ggml-cuda.dll`, `cublas64_13.dll`). There is no Linux binary, and `llama-cli` is not on PATH.

## Graph impact
Plane `rust`, `db_state: fresh`, 86 rows (trace `.andromeda/runs/2026-10-04T18-13-03Z-phase/tree-query-2026-10-04-l4-interpretation-names-its-triggering-cue.json`):
- **render_payload** — production callers are 4, all in `Assembler::assemble` (`crates/triage/src/digest/assembler.rs:337,361,376,392`).
  Non-production callers: 4 in-crate tests (`:1029,1071,1089,1107`), `pulse-app/examples/l4_decision_probe.rs:241`
  and `pulse-app/tests/unit_digest_runtime_scrub.rs:33,51`. A SIGNATURE change ripples to all of these. A change to
  the rendered TEXT ripples only to the presence and absence pins on `CORPUS MATCHES:` (sweep below).
- **build_primary_tier_prompt** — 3 production callers: `inference_runtime.rs:421`, `investigate_router.rs:248` and
  the probe `:256`. Also `pulse-app/tests/integration_real_llama_cli.rs:134` and 18 in-crate tests.
  **build_fallback_tier_prompt**: `inference_runtime.rs:427` + 16 in-crate tests. **build_reflection_tier_prompt**:
  `inference_runtime.rs:412` + 12 in-crate tests. A new PARAMETER on the builders touches ~50 call sites. Template-text
  changes touch none.
- **cue_cause_label** — production caller `grounded_title` (`inference_runtime.rs:677`) plus 1 test
  (`contract.rs:711`). It is reachable from `interpretation` over the existing dependency edge.
- **format_corpus_match_line** — production caller `assemble` (`assembler.rs:314`) plus 1 test (`retrieval.rs:262`).

## Patterns detected
- **Delimited prompt section + explicit none-instruction** (`prompt.rs:57-74`, `push_citable_ids_section`): this is
  the template for a new prompt section. The builder takes the payload, renders between markers in all three tiers,
  and makes the empty case explicit.
- **Shared instruction appended in every tier** (`prompt.rs:78-81`, `CITING_INSTRUCTION` pushed at `:252`, `:342`,
  `:430`): this is the shape for a framing sentence that must hold in all three tiers.
- **Probe arm = one factor varied** (`l4_decision_probe.rs:30-41`, `prepare` `:226-285`): each arm is a transform of
  A0 that differs in exactly one factor, and `shipped` is the tree as it stands. Bounded labels only (`:22-26`).
- **Real-function reuse in the probe** (`:16-20`): the probe renders through the product's own `render_payload`,
  `cue_summary` and `build_primary_tier_prompt`, so a framing change in those functions reaches the probe unchanged.

## Conventions to follow
- **Template text is ASCII** (`prompt.rs:799-831`): every literal this chunk adds to a template, and any digest
  header or framing literal, stays ASCII. The digest's ATTENTION CUES and CORPUS MATCHES lines already carry a U+2014
  em-dash (`assembler.rs:680`, `retrieval.rs:146`). That is digest content, outside the template pin, and this chunk
  does not change it. A new literal must not add another.
- **Lineage bump in lockstep** (`schema.rs:27-48`; test-plan §4): a template-text change bumps the
  `PROMPT_VERSION_*` const of every tier it touches, and the version pin (`prompt.rs:548`) moves with it.
- **Probe output is bounded labels only** (`l4_decision_probe.rs:22-26`): a names-the-trigger label is a closed set
  computed in-process, and no title, symptom or statement text is printed or written.
- **Example tests need `test = true`** (`pulse-app/Cargo.toml:19-21`; testing.md 2026-08-29): any pin added inside
  the probe needs the manifest declaration, and the workspace count must move by exactly the number of pins added.

## Sweeps
- `CORPUS MATCHES` literal (`grep -rn 'CORPUS MATCHES' --include=*.rs crates pulse-app xtask`): 7 hits. Code
  `assembler.rs:688`; tests `assembler.rs:903` (presence), `:961` (absence) and
  `pulse-app/tests/integration_corpus_retrieval_two_session.rs:223` (presence); doc comments `assembler.rs:12`,
  `contract.rs:573`, `digest/mod.rs:55`. A header RENAME changes the 3 test pins and the 3 comments. Keeping the header
  and adding a framing line changes none of them.
- Prompt-version literals (`grep -rn '"v2\.3"\|"v1\.2-fallback"\|"v1\.2-reflection"'` over crates, pulse-app, xtask
  and docs): 5 hits. The 3 const definitions; the pin `prompt.rs:548`; and
  `pulse-app/tests/integration_generation_damper.rs:195`, which is fixture DATA for an `L4Output` (no change: the
  damper does not read the version).

## Scope premise closure
- Item 1 `[inferred]` surfaces → **FALSIFIED.** Production passes `""` for `corpus_retrieval` at all three tier calls
  (`inference_runtime.rs:412-418,421,427`) and at `investigate_router.rs:248-253`, so the prompt's
  `# Corpus Retrieval` block never renders on a production path. Corpus matches reach the model ONLY as the digest's
  `CORPUS MATCHES:` section inside `payload_summary` (`assembler.rs:687-692`). The framing therefore acts on that
  digest text and on prompt text that describes it, not on the three dead `prompt.rs` corpus sites. scope.md is
  amended.
- Item 2 `[inferred]` triggering cue → **VERIFIED.** The triggering cue is `digest.attention_cues.first()`
  (`inference_runtime.rs:815`), and a cue-triggered digest carries exactly that one cue (`assembler.rs:259-272`). The
  composer receives it only as text inside `digest_payload` (the ATTENTION CUES line, snake_case kind label) plus its
  fingerprint in the citable ids. There is no structured cue parameter. Reflection digests carry no cue. Tag dropped.
- Boundaries `[inferred]` no new resource → **VERIFIED for both remedy branches.** Template text, digest text and a
  probe label need no port, TauRPC procedure, capability JSON, table, MCP tool or env var. Tag dropped.
- Boundaries `[inferred]` bounds still hold → **VERIFIED with the measurement owed.** The ceiling is 16,384 B
  (`llamacli_inference.rs:76`) and the largest recorded primary prompt is 6,979 B (d3). A framing of a few hundred
  bytes leaves headroom above 9 KiB. The exact delta is measured at implement through the probe's `--dry-run`. The
  security-plan's recorded "observed maximum 6,932 B" is itself below d3's 6,979 B, which is a note for the wrap's
  re-base and not a bound breach. Tag dropped.
- CONTEXT hypothesis `[inferred]` → **re-derived and stays a HYPOTHESIS.** Its verified halves still hold at HEAD:
  the digest carries the retry (`assembler.rs:259-272,676-686`), and production's corpus argument is empty
  (`inference_runtime.rs:412-427`). The causal claim (the model restated a corpus line) is unmeasured, n = 1, and is
  what the gated item 4 tests. It keeps its tag and marker text.

## New files to create
- none

## Files to modify
- `crates/interpretation/src/prompt.rs` — framing text: corpus matches are other or past incidents, and the
  ATTENTION CUES entry is the subject. Lineage pins and new framing pins in all three tiers.
- `crates/interpretation/src/schema.rs` — `PROMPT_VERSION_*` bump for every tier whose template changes.
- `crates/triage/src/digest/assembler.rs` — CORPUS MATCHES framing in `render_payload` (digest branch), plus its
  in-crate pins.
- `pulse-app/tests/integration_corpus_retrieval_two_session.rs` — only if the `CORPUS MATCHES:` header text changes
  (digest branch, rename form).
- `pulse-app/examples/l4_decision_probe.rs` — a bounded names-the-trigger label and a corpus-carrying shape or arm.
- `pulse-app/Cargo.toml` — `test = true` on the probe's `[[example]]` when pins land in the example.

## Open questions
- Which carrier marks corpus matches as other or past incidents: digest text (`render_payload`, which reaches every
  consumer of `payload_summary`, including the probe and the CONTEXT counterfactual), prompt text (a static
  instruction in the tier templates), or both? → blocks: plan-decision.
- How the prompt names the triggering cue: a static instruction pointing at the ATTENTION CUES entry (no signature
  change, ~0 call sites), or an explicit `# Triggering Cue` section carrying `cue_cause_label(kind)` through a new
  builder parameter (~50 call sites, including `investigate_router` with no cue)? → blocks: plan-decision.
- Where the names-the-trigger classifier lives: in the probe example (`test = true`, beside the open trigger
  `l4-decision-probe-arg-parse-unit-coverage`) or in `interpretation` as a pub fn whose only caller is the dev probe?
  → blocks: implementation-scope.
