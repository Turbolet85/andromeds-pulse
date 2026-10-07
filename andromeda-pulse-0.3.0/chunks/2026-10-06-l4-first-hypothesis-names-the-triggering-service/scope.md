# Scope — The L4 first hypothesis names the triggering service

**Marker:** `2026-10-06-l4-first-hypothesis-names-the-triggering-service` · version `andromeda-pulse-0.3.0` · Epoch 4 — Polish & ship: verification
**Working entry (`working-route.md:178`):** "The L4 first hypothesis names the triggering service — an incident's first
hypothesis names the service its triggering cue is scoped to, measured on the real model"

## Intent
Today the L4 prompt tells the model WHICH SIGNAL the incident is about (the digest's `TRIGGER:` line and the framing
instruction) and never WHICH SERVICE. The shipped model then picks the service from whatever else the digest carries. On
Conductor's fourth real-model series it named the retry storm three times out of three and gave it to the right service
twice; the third time it gave the storm to the canary. This chunk makes the first hypothesis name the service its
triggering cue is scoped to, and proves it with a pre-registered reading on the real model.

## What the chunk builds
- **The triggering cue's service reaches the model as part of the trigger framing** — on the `TRIGGER:` line, in the
  framing instruction, or both. Which of the three is the phase's to settle (working-entry CONTEXT 4).
  - The settling evidence available tonight is everything that needs no model: the three fourth-series captures, the
    corpus-match hypothesis below, and the code. The directive bars a real-model probe in this phase (inputs#I1), so the
    form is a P4 fork decided on that evidence, and the real-model reading measures the shipped form beside its
    counterfactuals (verified at P3: the probe's arm machinery already takes subtractive arms, `nf` at
    `pulse-app/examples/l4_decision_probe.rs:478-493`).
  - Decided at P4 (inputs#I7; the overseer, delegate, provisional on the boundary part): the instruction-only form
    ships. The service on the `TRIGGER:` line stays out of the product, because the 2026-10-04 ruling that keeps that
    line to the kind label rests on a security ground and superseding it needs the founder's own word. The line-only
    and both forms are measured as harness renders in the one reading; the smallest lever that meets the bar ships, a
    line-bearing one only on the founder's own word.
  - Added at the P5 review, before any run (inputs#I8): a regression guard. If the shipped form reads below the
    pre-change baseline on either half of the bar, the sentence and the prompt lineage go back to v2.5 before the wrap
    and the reading is recorded; a form that misses the bar without regressing stays.
- **A pre-registered bar for the real-model reading**, written before any run and including the case d3 hit
  (inputs#I1): the digest's trigger is scoped to one service while the context names a hyphenated sibling of it
  (`conductor` beside `conductor-canary`), and the first hypothesis must still name the triggering service.
  - The bar grades with a rule at least as strict as the one Conductor's fifth series will apply (inputs#I2,
    `crates/conductor-run/tests/real_model_harvest.rs`): `identifies_cause` (`:191`) = `names_conductor` (`:197`, the
    service as a whole word, neither neighbour in `[a-z0-9_-]`, so `conductor-canary` alone fails) AND `names_retry`
    (`:209`, a run of ASCII letters equal to `retry`, `retries` or `retrying`). Verified at P3 against inputs#I2.
  - The rule's own stated limits ride with it (its doc comment, `:187-190`): d1's "on Conductor or Conductor-Canary"
    passes it, because the first `Conductor` stands as a whole word (inputs#I3 `:421`). Verified at P3.
  - The instrument is the dev probe `l4_decision_probe`: it renders through the product's own `render_payload` and
    `build_primary_tier_prompt`, spawns `llama-cli` with production's argv, and needs the GPU but neither port 4317 nor
    4318 (verified at P3: `l4_decision_probe.rs:18-25`, `:1509-1536`). Its present grader reads the signal only
    (`names_trigger`, a substring `retry`, `:675-712`); the service half is this chunk's to add.
- **The real-model reading itself, as a leg that asks the operator for the go** (inputs#I1): daytime only, inside a
  granted model slot, conductor-builder idle. Nothing in this phase or in the non-leg part of the implementation runs
  the model.
- **The pins that move with the framing** (verified at P3 by a sweep of the exact literal and of the version labels):
  the digest test that expects the trigger line to read `TRIGGER: Retry storm` alone
  (`crates/triage/src/digest/assembler.rs:1186`), the probe's S4 pin on the same literal
  (`pulse-app/examples/l4_decision_probe.rs:1919`), the three version consts
  (`crates/interpretation/src/schema.rs:34`, `:42`, `:55`), their literal asserts
  (`crates/interpretation/src/prompt.rs:574-576`), and the three literal `prompt_version=` pins in
  `pulse-app/tests/unit_inference_runtime.rs` (`:539`, `:561`, `:628`). The instruction's exactly-once pin
  (`prompt.rs:580`) rides the const; its obligation-clause pin (`prompt.rs:607`) holds while that clause's text stays.
  Under the form decided at P4 the two `TRIGGER: Retry storm` literals do not move (the line is unchanged); the
  version consts, their asserts and the three literal pins do.
- **The service part of the trigger line is conditional.** [premise-corrected: three conditions, not one — the cue's
  scope is `Service`, it carries a `scope_id`, and that `scope_id` is renderable on one line. A cue with no `scope_id`
  keeps the signal-only line (`cue_summary`, `assembler.rs:618-623`); so does an `Operation`-scoped cue, whose
  `scope_id` is an operation name (`crates/triage/src/cue/evaluate.rs:128-136`); and nothing between the scrubbed
  choke point (`crates/buffer/src/appender.rs:118-141`) and the digest removes a newline or bounds the length of a
  service name, so an unguarded service part could forge a second `TRIGGER:` line.] Under the form decided at P4 no
  service part is rendered by the product, so these conditions are owed only if a line-bearing lever ships later.

## Mechanism claims carried by the entry (closed at P3)
- "re-derived at Pulse `1124148` — the digest's trigger line carries the first cue's cause label and no scope
  (`crates/triage/src/digest/assembler.rs:680`, its prefix const at `:608`; the test at `:1186` expects the label
  `Retry storm` alone); the framing instruction (`crates/interpretation/src/prompt.rs:90`) obliges the first hypothesis
  to name that signal in the trigger line's own words and says nothing about the service; the service reaches the model
  on the cue line's `scope_id=` (`assembler.rs:621`) and on the services rows (`:685`)".
  - Verified at HEAD `b5138e2`: the prefix const at `:608`, the trigger push at `:677-683`, `cue_summary`'s `scope_id=`
    at `:621`, the cue line rendered from it at `:696-705`, the services rows at `:684-695`, the test's assert at
    `:1186`, the instruction const at `prompt.rs:90-98`.
- [premise-corrected: not established as the cause, and not the only route — in all three captures the creating
  digest's corpus retrieval returned rows (1, 3 and 6: inputs#I3 `:531`, inputs#I4 `:553`, inputs#I5 `:573`), and all
  three narratives cite a 100 % error rate on both `conductor` and `conductor-canary`, so the sibling's name reached the
  model by two routes in every drive, the services rows and the corpus matches; d1 and d2 passed with both present.]
  The entry's text: "hypothesis: d3's "Conductor-Canary" came from a corpus-match line of an earlier canary incident —
  d1's narrative says a similar "Retry storm: Conductor-Canary experiencing retry storm and high error rate" incident
  was active 3 minutes ago (`rm-capture-d1.txt:417`), which supports it and does not prove it, so the phase measures it
  before building on it".
  - What supports it: d3's statement repeats the title shape of that earlier canary incident almost word for word
    (inputs#I5 `:426` beside inputs#I3 `:417`), and a corpus line carries an incident's title verbatim
    (`crates/triage/src/digest/retrieval.rs:136-148`).
  - What it cannot carry: one failing drive of three does not separate the two routes. d3's own justification shows the
    model had read the cue's scope correctly ("an autonomous cue for the conductor scope", inputs#I5 `:427`).
  - Where it is measured: the real-model leg, by a sibling shape with and without corpus matches under the pre-change
    render. The chunk's lever does not depend on which route it was.
- "Conductor's fourth pre-registered real-model series read NOT MET, 2 of 3, against Pulse `5f77859`
  (2026-10-06 20:06Z-20:31Z, Conductor `fa6a374`; the first reading of the shipped gemma-4-E4B-it-Q4_K_M at prompt
  v2.5 on retry naming): all three first hypotheses name the retry storm and split on the service".
  - Verified at P3: d1 "Retry storm due to service instability on Conductor or Conductor-Canary." (inputs#I3 `:421`),
    d2 "A retry storm is actively occurring in the Conductor service." (inputs#I4 `:424`), d3 "Retry storm:
    Conductor-Canary experiencing a retry storm" (inputs#I5 `:426`); each capture reads `inference_mode: real` (inputs#I3 `:446`,
    inputs#I4 `:453`, inputs#I5 `:452`) and `prompt_version=v2.5` on every prompt-assembly line it holds (10 of 10,
    12 of 12, 16 of 16).

## Boundaries
- Conductor's rule is not relaxed and its `v3-09` is not deferred (founder ruling, working-entry CONTEXT 1). The fifth
  series, and the rule in inputs#I2, are Conductor's; this chunk changes nothing in that repository.
- No model, sampling, GBNF or llama.cpp change: the shipped gemma-4-E4B-it-Q4_K_M at its authors' settings stays.
- Fault identity is untouched: the trigger line is framing, never identity; incidents keep coalescing per
  `(kind, scope, scope_id)`.
- Prompt template text stays ASCII (argv transport); the service name is OTLP-derived content and already reaches the
  argv, scrubbed, on the cue line and the services rows.
- No real-model run at night; none without the operator's go; conductor-builder stays idle (inputs#I1).
- "L4 generation records render unredacted", "Model observations without a cue surface quietly" and "Without a GPU, L4
  analysis is programmatic" are the next entries, not this one.

## Folded freight (the entry's four CONTEXT blocks)
1. **Founder ruling 2026-10-06 ~22:50 «думаю а»** (in-repo snapshot
   `.andromeda/runs/2026-10-06T21-47-06Z-wrap/relay-1.md` §2): Pulse is fixed, then Conductor runs a fifth series.
   Rejected by him: relaxing Conductor's rule to the retry facet alone; deferring `v3-09` at Conductor's version close.
   The entry sits at the head of the tail because Conductor's fifth series and its version close wait on the sha that
   ships it; the placement is the overseer's, the founder can move it.
2. **The fourth series' verdict** → the third mechanism bullet above.
3. **The re-derivation at `1124148` and the corpus-match hypothesis** → the first two mechanism bullets above.
4. **The HOW is the phase's to settle by measurement; the acceptance is a pre-registered real-model reading; every model
   run waits for daytime or the founder's word** → the first three build bullets above.

## Directive (inputs#I1) — folded
- Take up the head entry: done by this promotion.
- No real-model probe tonight; research and the plan run now → the sub-bullet under the first build bullet.
- The reading is a leg that asks the overseer for the go, daytime → the third build bullet.
- The pre-registered bar includes the case d3 hit → the second build bullet.
- Coordinates, re-read at Conductor `29adafa` (inputs#I2): `names_conductor` at `:197` and `names_retry` at `:209`
  stand; `identifies_cause` is declared at `:191` (`:190`, the line the directive names, is the last line of its doc
  comment).

## Second fold source — the CI verdict read at Setup 5a
- `b5138e208b9e` (the last wrap's flip, = HEAD): **verdict not yet available** — `ci#37546012637` in progress at take-up
  (13/13 checks registered, 12 running, the oldest `mcp-server tests (ubuntu-22.04)` at 163 s);
  `secret-scan#37546012633` completed success. Not read as green; nothing to disposition yet. The predecessor's pre-CI
  commit `fc4ebdf` read green 13/13 (handoff).

## Gate
- none — the entry carries no `BLOCKED-ON`.
