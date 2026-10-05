# Report — 2026-10-04-l4-rank-1-hypothesis-names-the-retry-on-every-storm-shape

**Chunk:** a retry-storm incident's first hypothesis names the retry on every storm shape, measured against a new
pre-registration
**Date:** 2026-10-05T06:1xZ
**Commits:** `febe375 chore(2026-10-04-l4-rank-1-hypothesis-names-the-retry-on-every-storm-shape): operator pre-CI
commit, for the run this chunk's verdict reads` (since `last_wrap` 2026-10-04T23:26:00Z. The basis is `200312b`, the
parent of the oldest pre-CI commit, from `git log --reverse --format=%H -F --grep …`)

## Changes (structured — detectors read this)

- **Files** (`git diff --name-only 200312b`, source subset; `gate.py scope` clean, changed 4 · listed 4):
  - `crates/interpretation/src/prompt.rs`
  - `crates/interpretation/src/schema.rs`
  - `pulse-app/examples/l4_decision_probe.rs`
  - `pulse-app/tests/unit_inference_runtime.rs`
  - Chunk folder: `scope.md` · `research.md` · `plan.md` · `inputs/` (I1, I2) · `evidence/` (`series.md`,
    `operator-pass.md`, `slot1-runs.json`, `slot2-runs.json`, `slot2-heldout-runs.json`) · this report.
  - `crates/interpretation/src/schema.json` is NOT changed: the R2 lever was not selected.
- **Symbols / APIs:**
  - `interpretation::prompt::TRIGGER_FRAMING_INSTRUCTION` (pub const, ASCII), reworded. It is byte-identical to the
    probe's measured `R1_FRAMING_INSTRUCTION`, 538 B (was 318 B), checked by a script comparing both constants.
    It keeps its v2.4 first sentence and its CORPUS MATCHES clause. It adds two obligations, kind-generic, no cue-kind
    word:
    - the first hypothesis statement names the TRIGGER line's signal in that line's own words;
    - any other abnormal metric on the same service is a cause or effect of that signal, never a separate first
      hypothesis.
    - Consumers are unchanged (code-graph refs at /phase): the three tier builders (`prompt.rs`), the probe's `nf`
      strip, and therefore `inference_runtime` L4 primary/fallback/reflection plus `investigate_router`
      (`investigate.run_action`). The instruction stays conditional ("When the digest carries a TRIGGER line"), so a
      TRIGGER-less investigation or reflection digest reads correctly.
  - Prompt lineage, all three bumped together:
    - `interpretation::schema::PROMPT_VERSION_PRIMARY` "v2.4" → "v2.5"
    - `PROMPT_VERSION_FALLBACK` "v1.3-fallback" → "v1.4-fallback"
    - `PROMPT_VERSION_REFLECTION` "v1.3-reflection" → "v1.4-reflection"
    - The `interpretation.prompt.assemble` field `prompt_version` now carries the new literals. The field set is
      unchanged and no emit site changed.
  - Probe (dev-only `[[example]]`, `test = true`), `l4_decision_probe`:
    - new arms `R1` · `R3` · `R2` · `R1R3` · `R1R2` · `R3R2` (each a single text transform over HEAD; idempotent once
      its text ships);
    - `--shapes` flag (default `S1,S2,S3,S4`; an unknown id is INCONCLUSIVE, exit 2);
    - held-out shapes `S5` (latency-only storm on auth-service) and `S6` (error rate + latency on checkout-api);
    - a record-only `names_trigger_stem` grader (`retry`/`retries`/`retried`) added to every `runs.json` row
      (`"names_trigger_stem"`) and one stdout line per arm. The verdict still reads the frozen `names_trigger` only,
      byte-unchanged.
    - The header line gains `· shapes {ids}`. `parse_args` is split into `parse_args_from(iter)`.
- **Crates / modules:** none added or removed. Changed: `interpretation` (prompt text plus lineage consts),
  `pulse-app` (example and test literals only).
- **Dependencies:** none.
- **Schema / config:**
  - The L4 output schema (`schema.json`) is unchanged.
  - No config key, env var, port, TauRPC procedure, capability, corpus table or MCP tool. The bindings stay
    byte-identical to `200312b`.
- **Spec-master edits:** none (no master was touched before this wrap's P2).
- **Counts / qualifiers moved:**
  - Workspace nextest count 2630 → **2639** (+8 probe pins, +1 `prompt.rs` pin). Basis: `cargo nextest run
    --workspace --profile ci` Summary lines at /implement (2638 before Step 7, 2639 after) and at pre-push stage 5
    (2639).
  - Probe collected pins 8 → **16** (`cargo nextest run -p pulse-app -E 'binary(l4_decision_probe)'` Summary).
    test-plan.md:144 states "8 collected pins".
  - `-p interpretation` 127 → **128**.
  - The prompt-size observed maximum (security-plan.md:139 and :461 state "7,185 B, the 2026-10-04 v2.4 dev-probe
    composition (synthetic S4)") moves to **7,405 B**, the v2.5 dev-probe composition of the same synthetic S4
    (`l4_decision_probe --arms shipped --shapes S1,…,S6 --dry-run` after Step 7). The ratio becomes
    16384 / 7405 ≈ **2.21×** (was ~2.28×), headroom **8,979 B ≈ 8.8 KiB** (was 9,199 B ≈ 9.0 KiB).
    - The largest composition any probe arm produced was 7,527 B (R1R2 S4, an arm not shipped).
    - Fallback and reflection compositions are NOT measured by the probe (primary builder only). Each carries the
      same instruction, so each grows by the same +220 B, and their absolute sizes are unmeasured this chunk.
- **Dev-tool versions:** none. llama-cli b9305 CUDA was re-read at fire time through the leg env (`inputs#I2`,
  unchanged). The npm stage reported the standing `10 high severity` line.
- **Harness / gate surface:** the probe's arm set, `--shapes` flag, held-out shapes and stem line (above). No xtask
  verb, agent-run script, CI step or status shape changed.
- **Cross-project / external claims:**
  - CI on `febe375` (the pre-CI commit), read with `ci.py conclusion --sha HEAD --wait 2400`: `verdict: green ·
    checks 13/13 · wall 1388 s`. Runs ci#37268608915 and secret-scan#37268608901, both completed/success.
    - The jobs, all `success`: lint/test ×3 OS, release build macOS + Windows, boot smoke, mcp-server tests,
      coverage gate, supply-chain, a11y ×3.
    - The sha is the record; this wrap's commit adds to that tree.
  - `I1 · message: overseer (founder-delegated), /andromeda-phase args, 2026-10-04T23:29Z · copy · n/a` (a message
    has no live source).
  - `I2 · ../additional/pc-overseer/l4-env.sh · copy · unchanged`. Its sha256 was also re-read equal at each slot's
    fire time.
  - No uncited entry, 0 `UNPARSED:`.
  - **Conductor:** its fourth v3-09 series grades this framing and waited on this entry. With the verdict FAIL, the
    BLOCKED-ON does not clear here. It moves to the model-replacement entry this wrap mints under the founder ruling
    (P5). The Conductor-side marker is the overseer's to move.
- **Reverted / negative API facts:** none. The candidate levers R3 (a conventions sentence) and R2 (a schema
  hypotheses-description sentence) were measured and NOT shipped. They exist only as probe constants.
- **Insufficient fixes (written, kept, not the remedy):**
  - **The defect:** the rank-1 hypothesis of a retry-storm incident does not name the retry on ≥ 36/40 of S1–S4.
  - **What the change resolved:** the shipped R1 rewording lifted strict rank1 from 28/40 (this chunk's in-series v2.4
    baseline) to **34/40** at Slot 2. Per shape: S1 8→9 · S2 7→9 · S3 5→7 · S4 8→9. The held-out S5/S6 read 20/20.
  - **Not resolved:** the pre-registered bar of 36. S3 (inventory-service, 12 % errors at 480 ms against 40–120 ms
    neighbours) stays the weakest at 7/10.
  - **Who owns the remainder:** the founder ruling of 2026-10-05 (this wrap's directive), which replaces the L4 model
    with a small current one chosen by measurement. That is the route entry minted at P5, ahead of pre-push:linux.
- **Spec claims disproved by measurement:** none against a master. Two chunk-level readings, recorded here and owed
  no amendment:
  - research.md Open question 2 / scope's `hypothesis:` CONTEXT asked how much of the predecessor's S2/S3 shortfall
    was the grader's stem gap. Measured on the same v2.4 composition and shape set (Slot 1 `shipped`): stem 29/40
    against strict 28/40, so the instrument accounts for **1 of 40**. The shortfall is wording, not grading. The
    predecessor's own stem reading stays unrecoverable (labels only). Slot 2 reads stem 36 against strict 34.
  - **Selection carry-over** (a measurement fact, not a falsified claim): the selected R1 read 37/40 in Slot 1 and the
    same text read 34/40 in Slot 2's fresh generations. At n = 10 per shape and one run per arm, a 3-count spread on
    the same composition is within run-to-run variation. The two v2.4 readings (the predecessor's 30, this chunk's 28)
    differ by 2. No measurement here separates selection optimism from variation.
- **Expected amendments (from plan):**
  - **architecture §Established Decisions [Fault Identity]** — carried (Symbols: lineage + R1 text; Insufficient
    fixes: Slot 2 34/40). Search `30/40|v2\.4\b|v1\.3-(fallback|reflection)` over the seven masters: architecture.md
    1 hit each at :72 (the measured clause and the lineage parenthetical), 0 elsewhere for `30/40`. The new clause:
    - Slot 1 arms `r(shipped) 28 · R1 37 · R3 36 · R2 33`;
    - rule branch: qualifiers R1, R3, so R1 ships (least blast radius); no combination fired;
    - Slot 2 `FAIL · rank1 34/40` (S1 9 · S2 9 · S3 7 · S4 9; elsewhere 2 · none 4 · unparsed 0);
    - held-out S5/S6 20/20;
    - stem readings: Slot 1 shipped 29 · R1 37 · R3 36 · R2 35; Slot 2 36; held-out 20;
    - lineage v2.5 / v1.4-fallback / v1.4-reflection; lever = the reworded `TRIGGER_FRAMING_INSTRUCTION`;
    - the remainder owned by the model-replacement route entry.
  - **test-plan §4 interpretation crate** — carried (Symbols: lineage; Counts: `-p interpretation` 128). Search
    `v2\.4\b|v1\.3-(fallback|reflection)|TRIGGER_FRAMING_INSTRUCTION`: test-plan.md 1 hit at :385. New pin:
    `every_tier_obliges_the_first_hypothesis_to_name_the_trigger_signal` (in `prompt.rs` `mod tests`) asserts the
    clause "the first hypothesis statement must name that signal in the TRIGGER line's own words" exactly once in
    each tier's composed prompt. Mutation-checked: reverting the product text alone read `left: 0, right: 1`.
  - **test-plan §1 `l4-decision-probe-arg-parse-unit-coverage`** — carried (Counts: probe pins 8 → 16). Search
    `l4-decision-probe-arg-parse-unit-coverage`: test-plan.md 1 hit at :144. The 8 new pins:
    - `--shapes`: `shapes_default_is_s1_through_s4`, `shapes_flag_selects_the_listed_shapes_in_shapes_order`,
      `shapes_flag_refuses_an_unknown_shape`;
    - the stem grader: `stem_grader_label_set_is_exactly_the_closed_set`,
      `stem_reads_rank1_for_retries_and_retried_where_the_strict_grader_does_not` (the asymmetric pair);
    - the candidate arms: `candidate_arms_carry_their_text_exactly_once_within_the_bound`,
      `candidate_transforms_are_identity_once_their_text_is_present`,
      `r2_schema_parses_and_changes_only_the_hypotheses_description`;
    - plus the existing `every_arm_and_shape_composes_within_the_production_bound`, extended to assert the S1–S6 set
      and the byte bound per prompt.
    - The flag-parse half for the PRE-EXISTING flags (`--arms`, `--n`, `--min`, `--min-rank1`, `--out`,
      `--dry-run`) stays owed. Only `--shapes` (and `--n`, incidentally, in one pin) is pinned.
  - **security-plan §Input Validation, L4 inference argv prompt row (:139), and its §Security Anti-Patterns → Code
    Patterns mirror (:461)** — carried (Counts: 7,185 → 7,405 B). The plan's condition "only if the largest
    production composition moves above 7,185 B" is MET. Search `7,185|7185`: security-plan.md 2 hits at :139 and
    :461, 0 elsewhere. It is a measurement note at both sites in lockstep, never a bound; `MAX_PROMPT_BYTES` 16384 and
    `validate_prompt_bounded` are unchanged in the diff. The fallback and reflection sizes are unmeasured (above).
- **Coverage of new surfaces:**
  - `TRIGGER_FRAMING_INSTRUCTION` v2.5 (changed L4 argv prompt text, all tiers) → validation `validate_prompt_bounded`
    16384 + ASCII pin ✓ · instrumentation n/a (no emit site changed; `prompt_version` rides the existing
    `interpretation.prompt.assemble` field) · PII n/a (static template text, no user data) · tests unit (obligation
    pin, mutation-checked; framing-once pin; ASCII pin; the three lineage pins) · a11y n/a · tokens n/a.
  - `l4_decision_probe --shapes` + the candidate arms + `names_trigger_stem` (dev-only example) → validation (unknown
    shape and arm refused, INCONCLUSIVE exit 2; exactly-once transform anchors) ✓ · instrumentation n/a (the probe
    installs no subscriber) · PII bounded labels only, no model text written ✓ · tests unit (8 new pins, 2
    mutation-checked) · a11y n/a · tokens n/a.

## Deviations from intent

- **The probe header gains `· shapes {ids}`** (in-intent; every atom the plan quotes still matches). Why: the routing
  witness records which shapes a run covered.
- **`parse_args` split into `parse_args_from(iter)`** (in-intent). Why: the `--shapes` pins need to drive parsing
  without the process argv.
- **"Extend the existing pin"** (Step 5) was done by asserting the shape set is S1–S6 and the byte bound per prompt.
  The arm and shape loops grew with the data.
- **The slot entries (15, 19, 20) were fired by hand** with their exact `run` text inside `bash -c`, wrapped by a
  scratchpad script that recorded START/END/EXIT. Entries 19 and 20 were fired consecutively from one script, in plan
  order. Combination entries 16–18 were `not fired — rule did not call it`.
- **The operator pass's pre-push stages** (stage commands, the stage 3 fresh `PUPPETEER_CACHE_DIR`, regen, base
  close) are not entries in this chunk's plan, which lists only the hygiene entry 21. They were fired in the exact
  `run` text of the precedent `2026-10-04-declared-rust-floor-matches-the-code` plan entries 25–33, with this chunk's
  base `200312b`, on the overseer's go (founder-delegated, 2026-10-05). Recorded in `evidence/operator-pass.md`.
- **Founder night pause:** relayed by the overseer after both Slot 2 entries had completed. No job was cut, nothing
  re-ran, and work resumed on the overseer's go.
- **Scope record:** none — `gate.py scope` reads `clean — changed 4 · listed 4 · recorded 0`.

## Decisions & corrections

- **Step 6 rule applied as fixed in the plan:** defect reproduced (28 < 36); qualifiers R1 (37) and R3 (36); R1
  selected by the least-blast-radius order; no combination run. No arm was re-run or reordered, and no text was tuned
  between Slot 1 and Step 7.
- **Overseer (founder-delegated), 2026-10-05, after recounting every slot from the copied `runs.json`:** "implement
  verified … they match (28/37/36/33, Slot 2 34/40, S5+S6 20/20)".
- **FOUNDER RULING 2026-10-05** (this wrap's directive, relayed by the overseer): replace the L4 model with a current
  SMALL efficient one chosen by measurement.
  - Pulse is a background helper: small footprint, no thinking mode. LoRA on Conductor scenarios comes later; bigger
    models are rejected.
  - Mint the entry next, before pre-push:linux. Candidates: Qwen3.5-4B, Qwen3.5-2B, Gemma 4 E4B, Gemma 4 E2B (unsloth
    GGUF Q4_K_M, Apache-2.0); baseline Llama 3.2 3B. b9305 knows qwen35 and gemma4. Conductor v3-09 waits on it.
- **Evidence form carried:** counts re-derived from the copied `runs.json`, never from stdout; `series.md` holds no
  candidate or prompt text, naming the constants instead (`R1_FRAMING_INSTRUCTION` etc.). This keeps the evidence
  hygiene that bans model and prompt text.
- **Sweep hazard confirmed:** the bare-literal version sweep `v2\.4\b|v1\.3-(fallback|reflection)` (word boundary,
  unquoted) found the probe fixture `prompt_version: "v2.4"` (`l4_decision_probe.rs` test helper), which stays
  unchanged as a parsed-output fixture. It also found the schema.rs lineage doc lines, which stay as history.
- **The predecessor's tokenizer-fetch plan defect is closed:** every slot entry sources the env before its build, and
  each build logged `using local tokenizer override` (no fetch).

## Outcome

- **Acceptance, re-asserted against the diff:**
  - *Slot 2 verdict recorded* — MET as measured: `l4-decision-probe: names-trigger verdict: FAIL · rank1 34/40` on
    the fixed tree. **The verdict is FAIL**, recorded as measured and never as passed (P4 ruling carried). Outcome
    word for the chunk: **FAIL rank1 34/40**.
  - *Slot 1 recorded and the rule applied as written* — MET (`evidence/series.md` §Slot 1 and §Step 6; every arm's
    strict and stem counts; the shipped text byte-identical to `R1_FRAMING_INSTRUCTION`).
  - *Stem reading reported* — MET: Slot 1 arms, Slot 2 S1–S4, held-out S5/S6. The Slot 1 shipped stem 29 sits beside
    the predecessor's recorded 30/40, whose stem reading is stated unrecoverable.
  - *Held-out S5/S6 recorded* — MET: 20/20 strict and stem, unthresholded.
  - *PREREQ closed* — MET: `cargo clippy --workspace --all-targets --all-features -- -D warnings` exit 0 at
    /implement (both runs) and at pre-push stage 4. **The clippy deferral from
    `2026-10-04-l4-framing-measured-on-the-real-model` is closed.**
  - *Standard gate set green in order* — MET: workspace nextest 2639 (= 2630 + 9 pins added); bindings close exit 0.
  - *Every new pin mutation-checked* — MET:
    - stem terms collapsed: 2 pins red;
    - default widened: 1 red;
    - product text reverted: obligation pin red;
    - each applied, grep-confirmed, then run (`series.md` §Mutation checks).
  - *Identity untouched* — MET: the diff touches no digest, producer, triage or identity code (scope guard: no output
    outside the five listed files; `gate.py scope` clean).
  - *No new resource* — MET (no port, procedure, capability, table, MCP tool or env var). Slots ran llama-cli b9305
    CUDA `-ngl 99`, detection-routed, the same GGUF.
  - *Bound and ASCII hold* — MET: every arm × S1–S6 at most 7,527 B < 16384; ASCII pin green; 0
    `stdout_utf8_invalid` rows (every decision is surface or watch across 220 rows).
  - *Evidence clean* — MET: `hygiene: clean` (four reads); no emit site changed.
- **Gates** (by `run`; /implement ran the block twice, before and after Step 7, then the operator pass on `febe375`):
  - `cargo fmt --check` — green
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings` — green (and native stage 4/6 green)
  - the scope guard `git diff --name-only 200312b… -- crates pulse-app … ':!…'` — green (no output)
  - `cargo build -p pulse-app --example l4_decision_probe` — green
  - the dry-run `… --arms shipped,nf,R1,R3,R2,R1R3,R1R2,R3R2 --shapes S1,…,S6 --dry-run` — green (all four atoms)
  - `cargo nextest run -p interpretation` — green (127, then 128)
  - `cargo nextest run -p pulse-app -E 'binary(l4_decision_probe)'` — green (16)
  - `cargo xtask capability-widening-check` — green
  - `cargo xtask check:ingest-progress` — green (NEUTRAL reading expected)
  - `cargo xtask check:staged-artifacts` — green
  - `cargo xtask capability-drift` — green
  - `cargo nextest run --workspace --profile ci` — green (2638, then 2639; native stage 5/6 2639)
  - the mcp-server `emit_taurpc_bindings` regen — green (re-fired after stage 5)
  - `git diff --quiet 200312b… -- pulse-app/ui/src/bindings/index.ts` — green (re-fired after the regen)
  - Slot 1 (leg operator) — exit 0, all six atoms held (`evidence/series.md`)
  - combination entries R1R3 / R1R2 / R3R2 (leg operator) — not fired, the rule did not call them
  - Slot 2 verdict (leg operator) — exit 1 = FAIL, a completed record; all five atoms held
  - Slot 2 held-out (leg operator) — exit 0, all four atoms held
  - `gate.py hygiene` (leg operator) — `hygiene: clean` (`evidence/series.md`, `evidence/operator-pass.md`)
  - Native pre-push 1/6–6/6 — all exit 0.
  - CI on `febe375`: `verdict: green · checks 13/13`.
  - Smoke: none owed (no boot-path or UI-surface change; P3 skipped with that reason).
- **Watches:** none folded.
- **Outcome basis:** the operator pass ran. Setup's commit list is `febe375` alone; that HEAD's CI run
  ci#37268608915 is green, recorded in `evidence/operator-pass.md` rows 10–12 and here. /implement's P4 report (this
  session's conversation) is the basis for the mutation readings, the pin counts and the smoke skip.
- **Process hygiene:**
  - /implement's census: gate runs ×2, slot runs ×3 (cargo build, `l4_decision_probe`, 220 `llama-cli` spawns under
    `kill_on_drop`) — all terminated.
  - The operator pass's cargo / npm / nextest children — terminated.
  - Re-measured at the operator pass (2026-10-05T05:3xZ, `ps -eo pid,comm,args` and `nvidia-smi`): 0 cargo,
    llama-cli or probe processes; GPU 0 %.
