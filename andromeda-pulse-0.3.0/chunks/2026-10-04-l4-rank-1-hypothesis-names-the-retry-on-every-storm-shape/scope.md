# Scope — L4 rank-1 hypothesis names the retry on every storm shape

**Marker:** `2026-10-04-l4-rank-1-hypothesis-names-the-retry-on-every-storm-shape` · **Version:** andromeda-pulse-0.3.0 ·
**Taken up:** 2026-10-04T23:3xZ (2026-10-05 local) · **Mode:** ordinary take-up (no `BLOCKED-ON` on the entry; not gated)

## Intent (working entry, verbatim title + hint)
L4 rank-1 hypothesis names the retry on every storm shape — a retry-storm incident's first hypothesis names the retry,
measured against the pre-registered bar.

## What this chunk builds
1. **A product remedy for the measured shortfall.** The shipped trigger framing (prompt `TRIGGER_FRAMING_INSTRUCTION`
   + the digest's `TRIGGER:` line + its corpus framing note, all shipped at
   `2026-10-04-l4-interpretation-names-its-triggering-cue`) read `FAIL · rank1 30/40` on the real model, the shortfall in
   S2 (6/10) and S3 (5/10). This chunk changes the L4 input (prompt text and/or digest rendering) so a retry-storm
   incident's FIRST hypothesis names the retry on those shapes too, without losing S1 (9/10) or S4 (10/10).
   Which lever is not stated by the entry; choosing it is a P4 fork. [verified at P3 as open, and narrowed: the
   TRIGGER line carries the closed kind label only, never `scope_id` (overseer ruling recorded at
   `2026-10-04-l4-interpretation-names-its-triggering-cue`). The candidates are therefore the framing instruction's
   wording, the `hypotheses` schema description and the conventions snippet. The framing instruction also reaches
   `investigate.run_action`, so any rewording stays conditional on a TRIGGER line (research.md §Scope premise
   closure)]
2. **A NEW pre-registration, written into `plan.md` before any real-model run.** It names the arms, the shapes, n per
   shape, the threshold and the run order, and fixes them before the first generation. The predecessor's series is
   spent and is never re-run as-is. "The pre-registered bar" in the entry is the predecessor's rule: rank-1 names the
   retry in >= 36/40, i.e. 90 %, over shapes S1–S4 at n = 10. [verified at P3: the predecessor's plan entry 13 and
   the arch [Fault Identity] measured clause]. Whether the new series keeps those four shapes alone or adds held-out
   storm shapes, and how candidate remedies are chosen, are P4 questions.
3. **The real-model measurement.** The new series runs on the granted model slot (`inputs#I1`) through the leg env
   `inputs#I2` (llama.cpp b9305 CUDA, `-ngl 99`, the same GGUF the predecessor used). Its stdout lines and `runs.json`
   go to this chunk's `evidence/`. A FAIL is recorded as measured, never as passed, and never re-run until it passes.
   This follows the P4 ruling that held at the predecessor (overseer, founder-delegated, 2026-10-04).
4. **PREREQ — close the rust gate deferral (clippy).** It has been deferred since
   `2026-10-04-l4-framing-measured-on-the-real-model` (record: that chunk's report). This chunk runs
   `cargo clippy --workspace --all-targets --all-features -- -D warnings` green, and its report names the deferral as
   closed.

## Folded freight (from the working entry)
- **CONTEXT (the measured FAIL)** — the remedy for the measured FAIL of
  `2026-10-04-l4-framing-measured-on-the-real-model` (`evidence/series.md`; Llama-3.2-3B-Instruct-Q4_K_M, llama-cli
  b9305 CUDA `-ngl 99`, n = 10 per shape, each arm run once). The shipped framing read `FAIL · rank1 30/40` against the
  pre-registered 36, per shape S1 9 · S2 6 · S3 5 · S4 10. The no-framing `nf` arm read 18/40 (S1 6 · S2 6 · S3 1 ·
  S4 5). Folded as items 1–3; the counts are the predecessor's committed evidence, re-read at take-up.
- **CONTEXT (measured-marked claim, kept verbatim):** "the shortfall lies in S2 6/10 and S3 5/10, which carry
  NO corpus match, while S4 (S1 plus a d3-like corpus match) reads 10/10 — so the corpus-match restatement hypothesis
  does not account for it (measured at that chunk; at n = 10 a smaller corpus effect is not excluded)". [verified at P3:
  a re-tally of the committed records gives shipped S1 9 · S2 6 · S3 5 · S4 10 = 30 and nf S1 6 · S2 6 · S3 1 · S4 5 =
  18]
- **[inferred] CONTEXT (hypothesis-marked claim, kept verbatim):** "hypothesis: what separates S2/S3 from S1 lies in
  the shapes' own storm content (`pulse-app/examples/l4_decision_probe.rs` shapes), unmeasured". [render half verified
  at P3, causal half UNRESOLVED and kept as a hypothesis]
  - At HEAD, S2's storm row renders `payment-service 9.0/s | 35.0% | 110ms`. S3's renders
    `inventory-service 7.0/s | 12.0% | 480ms`, against neighbours at 40–120 ms.
  - S1's storm row renders `100.0%` errors and reads 9/10, so "a competing elevated metric" alone does not separate
    the shapes.
  - A second candidate explanation, unmeasured, is the grader's stem gap: the retry term is the substring `retry`, which
    `retries` and `retried` do not contain (measured, research.md Open question 2). Which share of the S2/S3 shortfall
    is wording and which is grading cannot be read from the label-only evidence.
- **CONTEXT (rulings):** the FAIL is recorded, never as passed (P4 ruling, overseer, founder-delegated, 2026-10-04).
  This remedy stays inside 0.3.0 per the founder ruling of 2026-10-02 (wrap directive, overseer, founder-delegated,
  2026-10-05).
- **CONTEXT (spent series):** the series is spent. A re-measurement is a NEW pre-registration, written before any run.
  Folded as item 2.
- **CONTEXT (slot):** real-model runs need the operator's model slot (ports 4317/4318 shared with conductor-builder).
  GRANTED for this chunk: the 4317/4318 slot and the model slot, with conductor-builder idle until this chunk's wrap
  (`inputs#I1`, overseer, founder-delegated). The leg env is unchanged (`inputs#I2`, byte-identical to the predecessor's
  `inputs#I2` copy, measured with `cmp` at take-up).
- **CONTEXT (Conductor):** Conductor's fourth v3-09 series grades this framing. It waited on
  `2026-10-04-l4-framing-measured-on-the-real-model` and now waits on this entry (wrap directive, overseer,
  founder-delegated, 2026-10-05; placed ahead of pre-push:linux by the overseer so Conductor unblocks soonest — the
  founder may move it). On PASS, this chunk's report names that series as unblocked. On FAIL it names where the
  BLOCKED-ON moves. The Conductor-side marker is the overseer's to move, never this chunk's.
- **PREREQ:** close rust gate deferral (deferred since `2026-10-04-l4-framing-measured-on-the-real-model`: clippy —
  record: that chunk's report). Folded as item 4.

## Boundaries
- **Fault identity is untouched** (arch §Established Decisions [Fault Identity]). The cue KIND reaches the incident
  TEXT, never its identity. Incidents still coalesce per `(kind, scope, scope_id)`, and the producer's
  `{cue_cause_label}: {model title}` grounding stays. The TRIGGER line stays keyed on the SAME first cue the identity is
  taken from. It remains framing, never identity.
- **Prompt text stays ASCII** (security.md 2026-08-27, argv transport). Any changed template text, schema description
  or digest literal is pinned by `composed_prompt_templates_are_ascii_clean_for_argv_transport`.
- **Prompt size bound.** Every composed prompt stays under `MAX_PROMPT_BYTES` (16384). The predecessor read 6984–7185 B
  for the shipped arm.
- **Model output is never logged or committed as text.** The evidence carries the probe's bounded labels and verdict
  lines only. The probe records no title, symptom, hypothesis or raw output.
- **The grader is not moved to fit the result.** `names_trigger` (the `retry` term over the rank-1 statement) and its
  label set stay as the predecessor measured them, so the new series is comparable. A grader change would be its own
  pre-registered decision, made before any run, never after seeing one.
- **No new port, TauRPC procedure, capability JSON, corpus table, MCP tool or env var.** A prompt-text change bumps
  all three versions together (v2.5 / v1.4-fallback / v1.4-reflection), with the tests that pin the old literal.
  [verified at P3: the bare-literal sweep finds 16 hits, 15 of them this chunk's and 1 no-change (research.md Files to
  modify)]
- **The deterministic runner and its consumers stay green.** The canned L4 output, `degraded_mode` and the
  incident-producer pins are unaffected by a prompt or digest text change. [verified at P3: the canned output is a
  fixed literal (`prompt_version: "v2.1"`) that reads no prompt]
- **The real L4 path guard is unchanged** (`validate_path_input`, opt-in `ANDROMEDA_PULSE_L4_ALLOW_ROOT`). Committed
  evidence carries no full host path.
- **Runs.** Each pre-registered arm runs once, in the pre-registered order. Any further run is a new pre-registration.

## CI read at take-up (Setup 5a)
- `200312b` (the last wrap's flip commit = HEAD): **verdict not yet available**. ci#37243798555 (pull_request) was in
  progress with checks 13/13 registered, 12 running and the oldest a11y (ubuntu-22.04) at 167 s.
  secret-scan#37243798567 completed/success. Not folded, not read as green. No red to disposition.
