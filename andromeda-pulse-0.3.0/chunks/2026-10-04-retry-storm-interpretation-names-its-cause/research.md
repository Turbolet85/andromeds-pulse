# Codebase Research — 2026-10-04-retry-storm-interpretation-names-its-cause

## Scope
- **Depth:** deep (the L4 interpretation path end to end: cue → digest → prompt → producer → report) · **Reads:** 14 · **Globs/Greps:** 16
- **Harness rules consulted:** none — no live leg in this chunk as researched; the half-1 answer rests on the
  recorded Conductor d3 series plus code at HEAD, and every remedy branch below is provable in-process. If P4 adds a
  live leg, `.claude/rules/verification-harness.md` is read in full then.
- **Platform issues consulted:** none — no runner-only bullet folded (CI on `e71dba5` was in progress at Setup and
  still in progress at this re-read; no red to disposition).

## Half 1 — where the retry fact is lost (the measurement)

The four candidate layers from scope.md, each closed against code at HEAD (`e71dba5`) and against the recorded d3
capture (Conductor `conductor-0.3.0/chunks/2026-10-01-interpretation-re-proven-after-the-incident-surfacing-fix/evidence/rm-capture-d3.txt`, read-only):

1. **The digest CARRIES the retry — verified.** For a Tier1 digest triggered by a `RetryStorm` cue with
   `scope_id = S`, the assembler builds exactly one `DigestCueRef` from the triggering cue
   (`crates/triage/src/digest/assembler.rs:261-272`), and `render_payload` emits it under `ATTENTION CUES:` as the line
   `  [autonomous] retry_storm — retry_storm scope_id=S` (`assembler.rs:676-686`, kind label `:551-560`, summary
   `cue_summary` `:605-610`), plus `OVERALL: anomalous` (`:652-663`). The cue leaves the digest only when the token
   hard cap forces truncation (`:372-387`); the d3 prompt that fed incident 10 measured `token_count=6979`
   (capture line for `interpretation.prompt.assemble t=2026-10-01T20:04:53.604Z`), under the cap.
2. **The prompt presents it as one line among others — verified.** `build_primary_tier_prompt`
   (`crates/interpretation/src/prompt.rs:182-256`) inserts the digest verbatim between `<DIGEST>` markers. Nothing in
   the role (`:85-91`, "severity classifier"), the conventions (`:96-103`) or the output reminder (`:108-112`) ties
   `title` / `symptom` / the rank-1 hypothesis to the triggering cue, and the schema's own descriptions are generic
   (`title`: "Short human-readable incident title"; `symptom`: "Observable behavior summary" —
   `crates/interpretation/src/schema.json`, read by `json.load` at HEAD). The prompt's `corpus_retrieval` argument is
   always `""` (`pulse-app/src/inference_runtime.rs:420,426`); past incidents reach the model INSIDE the digest, as its
   `CORPUS MATCHES:` section (`assembler.rs:687-692`).
3. **The model is where it was lost — inferred from recorded evidence, strongly supported, NOT measured.**
   - The d3 report's Timeline reads `Error Rate Spike in <workspace-key>: 3m ago, active`. That is the shape of a
     digest CORPUS MATCHES line, `[{fingerprint}] {title} — {age}m ago, {outcome}`
     (`crates/triage/src/digest/retrieval.rs:136-148`), with the title and the age/outcome tail carried over.
   - Timestamps fit exactly one prior incident: incident 7 opened `1790884889849546600`, 203.75 s before incident 10
     (`1790885093604447300`) — "3m ago" — and is the only one the capture marks `seen_active=true`. Its open instant
     equals the 20:01:29.849Z `interpretation.prompt.assemble` of the Tier2 digest whose
     `digest.assemble.request` carried `cue_kind=error_rate_spike` (capture `:539`, `:557`), created at 20:01:33.908Z
     with `severity=error priority_tier=autonomous`. An error-rate-spike incident, active, 3 minutes older.
   - Why it could reach incident 10's digest but not its Previously Seen: the digest's selection keeps a candidate
     whose `scope_id` matches ANY current Q1 service (`assembler.rs:286-287`; `select_corpus_matches`
     `retrieval.rs`), while the report's Previously Seen keeps only the incident's OWN scope
     (`select_previously_seen`; incidents_router `get_report` `pulse-app/src/incidents_router.rs:460-468`). The d3
     report lists only #2 there — consistent with incident 7 sitting on a sibling scope.
   - So: the retry fact reached the model (layers 1-2), and the model's Title / Symptom / Timeline / rank-1 hypothesis
     restate a competing, differently-kinded active incident's title. **Incident 7's title is not in the capture, so
     "the model copied it" is a hypothesis fitted to text shape and timestamps, not a measurement.** Whether the model
     would name the retry WITHOUT that corpus line (the counterfactual) needs the real model — not measurable on this
     host (overseer note 2026-10-04: no GGUF, no llama-cli under `~`).
4. **The render has NO deterministic carrier of the cause — verified.** `assemble_report`
   (`crates/interpretation/src/markdown.rs:249-329`) takes `title` / `symptom` / `timeline` / `hypotheses` from the
   parsed `L4Output` (all model-authored); without one, from `incident.title` / `incident.detail` — which the producer
   also set from the model (`inference_runtime.rs:883-884`). `incident.kind` (the cue kind, `RetryStorm` here, set from
   `digest.attention_cues.first()` at `inference_runtime.rs:794-800`) is rendered NOWHERE in the report
   (`serialize_report`, `markdown.rs:115-237`). The Findings rows show `incident.title` (`incidents_router.rs:60`;
   `pulse-app/ui/src/widget/FindingsWindow.tsx:236`, `FindingsDropdown.tsx:215`).

**Verdict for half 1:** the retry is not lost in the digest or the prompt's inputs; it is lost at the model, and the
product has no deterministic field that would carry it past a model that drops it. The second half is a product fact
at HEAD, independent of the model; the first half is inference over one recorded generation (n = 1 incident).

**A deterministic-path trap (load-bearing for P4):** the canned `L4Output` of the deterministic runner
(`pulse-app/src/deterministic_inference.rs:43-53`) carries the rank-1 statement "Deterministic fixture hypothesis: the
retry storm originates in the synthetic verification scope." for EVERY incident, whatever its cue. Any check that a
deterministic-mode report "names the retry" passes vacuously — it must also show that a non-retry incident does NOT
name it.

## Files inspected
- `crates/triage/src/digest/assembler.rs` (195-495, 545-715) — Tier1 digest = one triggering cue; ATTENTION CUES line shape; CORPUS MATCHES scope selection over all Q1 services; truncation order.
- `crates/triage/src/digest/retrieval.rs` (1-150) — `select_corpus_matches` (any current scope) vs `select_previously_seen` (own scope); `format_corpus_match_line` shape.
- `crates/interpretation/src/prompt.rs` (1-270) — section order; role/conventions/reminder text; `corpus_retrieval` slot.
- `crates/interpretation/src/schema.json` (title/symptom/timeline/hypotheses descriptions) — generic field guidance.
- `crates/interpretation/src/markdown.rs` (80-330) — `Report` struct, `serialize_report`, `assemble_report` (single source for TauRPC + MCP, P-038).
- `pulse-app/src/inference_runtime.rs` (755-900) — identity from the first cue; `title`/`detail` from the model.
- `pulse-app/src/incidents_router.rs` (50-65, 440-480) — list title source; report assembly + Previously Seen.
- `pulse-app/src/deterministic_inference.rs` (43-53) — canned title / symptom / timeline / hypotheses.
- `crates/triage/src/cue/classify.rs` (45-75) — `cue_kind_label` exists (pub in `cue`) but is NOT re-exported through `triage::contract` (`contract.rs:39-52`).
- `crates/triage/src/cue/evaluate.rs` (120-140) — cue `scope_id` per family (service name; operation name for latency).
- `pulse-app/examples/l4_decision_probe.rs` (1-40) — the dev-only real-model probe; it records bounded labels only and deliberately writes no title/symptom/hypothesis text.
- Conductor `…/evidence/rm-capture-d3.txt` (395-560) and `report.md` (155-175) — the d3 report body, incident table and the log window.

## Graph impact (from the code-graph query; trace `tree-query-2026-10-04-retry-storm-interpretation-names-its-cause.json`)
- **assemble_report** — production callers `pulse-app/src/incidents_router.rs:479` (`get_report`) and `crates/mcp-server/src/tools.rs:390` (`dispatch_retrieve_report`); test callers `crates/interpretation/src/markdown.rs:538,551,558`, `pulse-app/tests/e2e_p3_mcp_incident_tools.rs:125`, `pulse-app/tests/e2e_security_negative_canaries.rs:297`, `pulse-app/tests/integration_interpretation_attach.rs:234,261` — a render-side change reaches the webview report AND MCP `retrieve_report` byte-identically, with no signature change needed.
- **create_incident_from_l4_output** — one production caller `pulse-app/src/inference_runtime.rs:266`; 50 test call sites across 8 `pulse-app/tests/` files (unit_incident_producer.rs carries 31) — a producer-side title change is pinned in `unit_incident_producer.rs`.
- **render_payload / cue_summary** — production `assembler.rs:337-392`; dev probe `pulse-app/examples/l4_decision_probe.rs:62,241`; pins `assembler.rs:1029-1107` (`overall_line_*`), `pulse-app/tests/unit_digest_runtime_scrub.rs:33,51` — a digest-side change moves these pins and the probe's rendering.
- **build_primary_tier_prompt / build_fallback_tier_prompt** — production `inference_runtime.rs:420,426`, `investigate_router.rs:248`; dev `l4_decision_probe.rs:256`, `integration_real_llama_cli.rs:134`; 35 in-crate pins in `prompt.rs` — a prompt-side change is version-bumped through `PROMPT_VERSION_*` and moves the investigate path too.
- (Line numbers are the trace's 0-indexed `line` + 1.)

## Patterns detected
- **Ground a model-adjacent field from cue-borne data at the producer join** (`inference_runtime.rs:860-866`, `grounded_fingerprint_hashes`): the cue's real fingerprint is unioned with the model's refs, parsed refs first — the precedent for adding a deterministic fact without displacing model content.
- **Single-source report projection** (`markdown.rs:239-253`): TauRPC `get_report` and MCP `retrieve_report` both call `assemble_report`, so a projection change cannot diverge between the two egresses.
- **Scrub at projection** (`markdown.rs:262-286`): every text field passes `scrub_string`; a closed cue-kind label needs none, but any text joined with model text still rides the scrub.

## Conventions to follow
- **Cross-crate exposure only through `contract`** (arch §Conventions): `cue_kind_label` is `pub` in `triage::cue` but absent from `triage::contract` (`contract.rs:39-52`); a human-readable cue label used by `interpretation` either joins the contract or lives in `interpretation`.
- **Prompt template text is ASCII** (`prompt.rs:799` `composed_prompt_templates_are_ascii_clean_for_argv_transport`), and any prompt change bumps through `PROMPT_VERSION_*` with the schema property-order pin held (`prompt.rs:529`).
- **pulse-app tests live in `pulse-app/tests/`** (`[lib] test = false`; flat-zero ratchet).

## New files to create
- none

## Files to modify
- `crates/interpretation/src/markdown.rs` — render-side grounding of the triggering cue into the report (branch A), with its inline pins.
- `pulse-app/src/inference_runtime.rs` — producer-side grounding of the incident title from the cue kind (branch A).
- `crates/interpretation/src/prompt.rs` — prompt-side framing of the triggering cue and of corpus matches (branch B), with its inline pins.
- `crates/interpretation/src/schema.rs` — the `PROMPT_VERSION_*` bump if branch B ships.
- `crates/triage/src/contract.rs` — re-export of a cue-kind label if the render needs it across the crate boundary.
- `pulse-app/tests/unit_incident_producer.rs` — producer pins for a grounded title.
- `pulse-app/tests/integration_interpretation_attach.rs` — report pins over the real producer + `assemble_report`.
- `pulse-app/tests/e2e_p3_mcp_incident_tools.rs` — the MCP `retrieve_report` content pin over the same projection.

## Open questions
- Which remedy ships — A (deterministic grounding of the cause from `incident.kind` into the report and/or the incident title), B (prompt framing: name the triggering cue, mark corpus matches as other/past incidents), or both? → blocks: plan-decision. A is provable on this host and fixes the structural gap (no deterministic carrier); B targets the model layer where d3 lost it but CANNOT be proven here without the real model (founder desk act), and per the founder ruling "the fix follows the measurement" it rests on an inferred, not measured, mechanism.
- If A: which fields carry the cause — the report title (reaches Findings rows only if the producer also grounds `incident.title`), a symptom prefix, or a new labelled line in the report header block (a `Report` field → TauRPC payload shape → bindings regen)? → blocks: plan-decision. Conductor's v3-09 grades the rank-1 HYPOTHESIS statement, which only branch B (or a fabricated hypothesis, which no branch should do) can move.
- If any branch changes the incident title or report text under the deterministic runner, Conductor's harvest fixture `conductor/crates/conductor-run/tests/lifecycle_harvest.rs:77` pins `"title":"Deterministic verification incident"` (a static JSON fixture, cross-repo) → blocks: implementation-scope (record as a cross-repo note, not a Pulse edit).
