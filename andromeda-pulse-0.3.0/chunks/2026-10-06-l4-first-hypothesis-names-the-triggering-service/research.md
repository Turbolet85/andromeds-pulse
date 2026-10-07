# Codebase Research — 2026-10-06-l4-first-hypothesis-names-the-triggering-service

## Scope
- **Depth:** deep · **Reads:** 21 · **Globs/Greps:** 19
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — 0 additions applied (a name grep for
  `l4_decision_probe`, `l4-env` and `real-model` over it returns 0 hits; the probe is not a harness verb). The leg's
  firing form is taken from the two recorded probe series instead:
  `andromeda-pulse-0.3.0/chunks/2026-10-04-l4-rank-1-hypothesis-names-the-retry-on-every-storm-shape/plan.md:262-309`
  and `andromeda-pulse-0.3.0/chunks/2026-10-05-l4-model-chosen-by-pattern-discrimination/plan.md:339-346`.
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg
- **External inputs:**
  - `inputs#I1` — the phase directive: no real-model probe tonight, the reading as a leg that asks for the go, the bar
    includes the case d3 hit.
  - `inputs#I2` — the rule Conductor's fifth series grades (`real_model_harvest.rs`): `identifies_cause` `:191` =
    `names_conductor` `:197` AND `names_retry` `:209`; its stated limits at `:187-190`.
  - `inputs#I3`, `inputs#I4`, `inputs#I5` — the fourth series' captures d1, d2, d3: the three first hypotheses, the
    corpus-retrieval row counts, the real-model and prompt-version readings.
  - `inputs#I6` — the overseer's `l4-env.sh`: it exports `ANDROMEDA_PULSE_MODEL_PATH` (the gemma-4-E4B-it-Q4_K_M GGUF)
    and `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`. The handoff's note that it still names Llama-3.2-3B is out of date.
  - `inputs#I7` — the answer at the P4 fork: the instruction-only form ships; one daytime reading with three arms on
    one bar; the smallest lever that meets it ships, a line-bearing one only on the founder's own word.
  - `inputs#I8` — the P5 review: the plan's five confirm points stand as written, and a regression guard joins the
    pre-registration (the shipped arm below the pre-change baseline on either half reverts the sentence and the lineage).

## Files inspected
- `crates/triage/src/digest/assembler.rs` (228-427, 560-770, 1090-1259) — `render_payload` writes the trigger line at
  `:677-683` from `cues.first()`: the cause label and nothing else. The cue line (`:696-705`) carries
  `cue_summary`'s `{kind} scope_id={s}` (`:618-623`); the services rows are at `:684-695`. `assemble` builds the one
  `DigestCueRef` from the triggering cue with its `scope` and `scope_id` (`:262-273`), so the service is already on
  the struct `render_payload` receives. Corpus lines are scrubbed one by one (`:314-316`).
- `crates/triage/src/digest/retrieval.rs` (128-150) — `format_corpus_match_line` writes
  `[{fingerprint}] {title} — {age}m ago, {outcome}`: the incident's title verbatim.
- `crates/triage/src/digest/damper.rs` (1-40) — the damper's condition projection excludes `payload_summary`, so the
  trigger line's text takes no part in the unchanged-digest comparison.
- `crates/triage/src/contract.rs` (223-227, 589-640) — `CueScope` is `Service` | `Operation` | `Global`;
  `Digest::scrubbed_clone` scrubs the whole `payload_summary`.
- `crates/triage/src/pattern/storm.rs` (296-322) and `crates/triage/src/cue/evaluate.rs` (118-138) — a storm cue's
  `scope_id` is the service name; a latency-regression cue is `Operation`-scoped and its `scope_id` is an operation name.
- `crates/buffer/src/appender.rs` (106-141) — `extract_service_name` returns the scrubbed `service.name`. It masks
  secrets and does nothing else: no control-character filter, no length bound.
- `crates/interpretation/src/prompt.rs` (80-100, 196-276, 556-665) — `TRIGGER_FRAMING_INSTRUCTION` is one line, pushed
  after `CITING_INSTRUCTION` in all three builders (`:272`, `:365`, `:456`).
- `crates/interpretation/src/schema.rs` (31-55) — the three version consts: `v2.5`, `v1.4-fallback`, `v1.4-reflection`.
- `pulse-app/src/inference_runtime.rs` (404-432, 795-830) — the runtime picks the builder by tier and digest kind; the
  producer takes the incident identity from `digest.attention_cues.first()` (`:826`), the same cue the trigger line reads.
- `pulse-app/src/investigate_router.rs` (248) — a second caller of `build_primary_tier_prompt`; its digest carries no
  trigger line, so any added instruction text must stay conditional.
- `pulse-app/examples/l4_decision_probe.rs` (1-340, 341-540, 536-760, 1453-1750, 1880-1945) — the instrument. Arms are
  transforms over the shipped composition (`prepare`, `:450-534`); `nf` is subtractive (`:478-493`); the R candidates
  are additive and compose as identity once their text is in the tree (`apply_r1`, `:549-576`, a whole-line equality).
  The grader reads the signal only (`names_trigger`, `:689-712`). Verdict lines and exits at `:1727-1749`.
- `pulse-app/examples/l4_decision_probe/patterns.rs` (690-720, 1534-1551) — pattern shapes render through
  `render_payload`; their one trigger pin counts lines by prefix.
- `pulse-app/tests/unit_digest_runtime_scrub.rs` (20-60) — the scrub fixture pin compares the `OVERALL:` line only.
- `pulse-app/tests/unit_inference_runtime.rs` (534-542, 558-563, 625-630) — three literal `prompt_version=` pins.
- `.andromeda/architecture-amendments.md` (655-668) — the 2026-10-04 ruling in its own words: "the TRIGGER line carries
  the kind label only, never `scope_id`" (overseer, founder-delegated).
- `.andromeda/playbook.md` (114-116) — the one `verdict: escalate` pattern that bears: "Boundary widening — a chunk
  WIDENS what crosses an already-hardened boundary (a read-only channel gains a write, a validated surface admits a
  new input class, a subprocess/IPC boundary gains a new crossing) and the proposal records it."
- The three Conductor captures and the harvest rule, through `inputs#I2`-`inputs#I5`.

## Graph impact (from the code-graph query; rust plane, trace `tree-query-{marker}.json`)
- **render_payload** — 17 reference rows (`calls WHERE callee_name = 'render_payload' AND callee_kind = 'fn'`): 4 in
  `assemble` (`assembler.rs:338`, `:362`, `:377`, `:393`), 5 in the crate's own tests, 2 re-exports, 4 in the probe and
  its pattern module, 2 in `pulse-app/tests/unit_digest_runtime_scrub.rs:33`, `:51`. The signature does not change, so
  no caller threads a new value: the service is already on `DigestCueRef`.
- **cue_summary** — 8 rows (same query shape): `assemble` at `assembler.rs:267`, the probe at
  `l4_decision_probe.rs:392`, `:454`, `patterns.rs:706`, plus re-exports. Unchanged by this chunk.
- **TRIGGER_FRAMING_INSTRUCTION** — 9 rows (`refs WHERE callee_name = ...`): the three builders' capacity and push
  sites (`prompt.rs:218`, `:272`, `:311`, `:365`, `:402`, `:456`), one pin (`:599`), and the probe (`:119`, `:651`).
- **TRIGGER_LINE_PREFIX** — 7 rows: the definition's re-exports, three pins in `assembler.rs`, the probe (`:132`, `:643`).
- **Crate edges** — `interpretation → triage` and `interpretation → security` already exist; `pulse-app` and
  `mcp-server` depend on both (`crate_edges` for the two crates, 6 rows). No new edge is needed in any form.

## Patterns detected
- **The service is already in the digest, twice** (`assembler.rs:696-705`, `:684-695`): on the cue line and in the
  services rows. d3's justification quotes the cue's scope correctly (inputs#I5 `:427`) while its statement names the
  sibling (`:426`). Presence in the digest is therefore not enough; what is missing is the service in the words the
  first hypothesis is told to repeat.
- **The first hypothesis repeats the trigger line** (`prompt.rs:92-94`): the instruction obliges it to name the signal
  "in the TRIGGER line's own words". All three fourth-series statements open with the line's words, "Retry storm" or
  "A retry storm" (inputs#I3 `:421`, inputs#I4 `:424`, inputs#I5 `:426`).
- **Two routes carried the sibling's name in every drive** — corpus rows 1, 3, 6 (inputs#I3 `:531`, inputs#I4 `:553`,
  inputs#I5 `:573`) and a 100 % error rate on both services in each narrative. One failing drive of three cannot say
  which route d3 took.
- **Subtractive arms give counterfactuals on a fixed tree** (`l4_decision_probe.rs:478-493`, pinned at `:1885-1911`):
  the `nf` arm is the shipped composition minus exactly three lines. The same shape yields the pre-change render, an
  instruction-only render and a line-only render from one shipped tree, in one sitting.
- **A two-armed real-model verdict completes its chunk either way** (the 2026-10-04 slot-2 entry,
  `…names-the-retry-on-every-storm-shape/plan.md:302-309`): exit 0 PASS and exit 1 FAIL are both a recorded verdict,
  exit 2 measured nothing; a FAIL is never reported as passed.
- **An equality the plan will lean on, verified:** for a `Service`-scoped first cue, the cue `render_payload` reads
  for the trigger line and the cue the producer takes the identity from are the same element — both are index 0 of the
  digest's cue list (`assembler.rs:678`, `inference_runtime.rs:826`), and `assemble` builds that list with at most one
  element (`assembler.rs:262-273`).
- **The pattern scorer's service match is a substring** (`l4_decision_probe/patterns.rs:755-757`, `names_service`:
  `lower.contains(service)`): it would read `conductor-canary` as naming `conductor`. The bar cannot reuse it; the
  whole-word rule of inputs#I2 needs its own function.
- **The R1 candidate arm matches the instruction as a whole line** (`l4_decision_probe.rs:549-558`). Once the shipped
  instruction gains text, R1 stops being the identity and replaces the line with the v2.5 text; its identity pin
  (`candidate_transforms_are_identity_once_their_text_is_present`, `:2058`) then needs the candidate text to follow
  the shipped one.

## Conventions to follow
- **Render pins go through the real renderer** (`assembler.rs:1155-1166`, `render_with`): build the cue refs in the
  test, assert on the rendered lines; an "omits" pin guards only beside its positive twin (`:1169-1200`).
- **Version labels move together and are asserted through the consts** (`prompt.rs:574-576`,
  `unit_inference_runtime.rs:539`, `:561`, `:628`): an instruction-text change bumps all three.
- **Template text is ASCII and names no cue kind** (`prompt.rs:880`, the argv-transport pin; the instruction's own doc
  comment at `:83-89`). By the same rule it names no service.
- **Probe labels are closed sets, pinned with an asymmetric pair** (`l4_decision_probe.rs:1859`, `:1978`, `:2004`), and
  an S-shape run writes no model text anywhere (`:27-33`).
- **The series out dir is minted at invocation** under the gitignored `target/` (`O="target/l4-decision-probe/…-$(date
  -u …)"`, the 2026-10-04 form).
- **Sources are ASCII and English** (`cargo xtask check:english-sources`): the founder's ruling is never copied into a
  source file or a test.

## New files to create
- none

## Files to modify
- `crates/interpretation/src/prompt.rs` — the framing instruction and its pins
- `crates/interpretation/src/schema.rs` — the three prompt version labels and their doc lines
- `pulse-app/tests/unit_inference_runtime.rs` — the three literal version pins
- `pulse-app/examples/l4_decision_probe.rs` — the sibling shapes, the counterfactual arms, the service grader, the verdict
<!-- Rewritten at P4 to the decided fork (inputs#I7): the instruction-only form ships, so the digest renderer and its
     pins are not written. Before the answer this list also named crates/triage/src/digest/assembler.rs and
     pulse-app/tests/unit_digest_runtime_scrub.rs, the files of the line-bearing branches. -->

## Open questions
- Which form ships: the service on the trigger line, an instruction sentence, or both → blocks: plan-decision.
  ANSWERED at P4 (inputs#I7): the instruction-only form ships; the line-only and both forms are harness renders in the
  one daytime reading, never in the product. The line form would reverse the recorded 2026-10-04 ruling (kind label
  only), whose ground was security, and that is a boundary question for the founder's own word.
- Sweep record, for the plan: `"TRIGGER: Retry storm"` as an exact literal — 2 hits · 2 changed (`assembler.rs:1186`,
  `l4_decision_probe.rs:1919`); the version labels as bare words (`grep -rnw 'v2\.5\|v1\.4-fallback\|v1\.4-reflection'`
  over `crates`, `pulse-app/{src,tests,examples,ui/src}`, `xtask`, `scripts`) — 15 hits in 3 files · all 3 files
  changed; `TRIGGER` (`grep -rn 'TRIGGER'` over the same trees) — 11 files hit · 4 changed · 7 no-change (the re-export lines in
  `contract.rs` and `digest/mod.rs`, a prefix count in `patterns.rs`, and four files where the word is
  `CADENCE_TRIGGER`). This is a note, not a question.
- Outside this chunk, recorded so it is not lost: `format_corpus_match_line` writes a U+2014 dash into the prompt
  (`retrieval.rs:145`), which is non-ASCII text on the argv. It predates this chunk and no step here touches it.
