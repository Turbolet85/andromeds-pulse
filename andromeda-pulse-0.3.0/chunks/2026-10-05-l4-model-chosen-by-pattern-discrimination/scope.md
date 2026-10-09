# Scope — L4 model chosen by pattern discrimination

**Marker:** `2026-10-05-l4-model-chosen-by-pattern-discrimination` · version `andromeda-pulse-0.3.0` · Epoch 4 — Polish & ship: verification
**Working entry (`working-route.md:172`):** "L4 model chosen by pattern discrimination — the L4 model is chosen by how well each
small candidate catches non-standard patterns and stays quiet on healthy traffic"

## Intent
The predecessor's naming series could not choose a model. Its shapes S1–S6 are all retry storms that Pulse had already caught,
so the model only had to name a cue that was in front of it. Every candidate read 36–40 of 40 (inputs#I1 §0). This chunk
replaces that saturated test with a discrimination test. The test asks two things of each small candidate: does it catch the
non-standard patterns that are in today's digest but that no detector flags, and does it stay quiet on healthy traffic? The
output is a quality-vs-footprint table over the six models, plus a recommendation made by a rule fixed before any run. The
founder decides from that table. The chunk ships no model change (inputs#I1 §5–§6). The founder's ruling: "the point of the
model is not only to print more nicely what Pulse already catches, but to catch non-standard patterns, so we need a balance
between lightness and dumbness" (inputs#I1, header, translated).

## What the chunk builds
- **Shape set, three families, each shape a synthetic digest** (inputs#I1 §2):
  - **A1–A7, catchable from today's digest.** No cue and `OVERALL` nominal, except A7, which carries a cue on the wrong
    service by design. The model must surface the pattern and name its cause.
  - **B1–B3, no-alarm controls.** The model must not raise an alarm (decision dismiss/watch).
  - **C1–C3, digest-limited.** Each runs in two renders: today's `render_payload` output and a probe-only "enriched" render
    that carries baselines and a short trend. C under today's render is expected to fail and is recorded as `digest-limited`.
- **Ground truth and keyword sets written BEFORE any run** (inputs#I1 §2–§3; directive inputs#I2). Per shape this records
  the expected decision class, the ground-truth service and the pre-declared cause-word set (e.g. A1
  deploy|release|commit|change). All of it sits in a pre-registration whose write time precedes every real-model leg, the
  predecessor's `preregistration.md` discipline.
- **The scorer.** Per generation it emits bounded labels only, from the model's JSON (inputs#I1 §3):
  - **detect** — the decision is `surface` for A and C, and is not `surface` for B;
  - **cause** — the rank-1 hypothesis names the ground-truth service AND a word from the shape's cause set;
  - **valid** — the output is schema-valid, carries no thinking, and lands inside the time budget.

  Per model it reports A-detect %, A-cause %, B-false-alarm %, C-today % and C-enriched %, plus GPU latency, RAM and VRAM from
  the existing footprint probe.
- **The blind audit hook** (inputs#I1 §3; inputs#I2). Each generation's synthetic digest text and raw output are kept in
  gitignored `target/` for audit only. 10 % of rows are drawn at random with the model name hidden, and the overseer grades
  them against the ground truth. If the audit disagrees with the keyword label on more than 10 % of the sample, the keyword
  sets are fixed and the WHOLE series is re-graded, not re-generated, before any recommendation.
- **The series, on the extended `pulse-app/examples/l4_decision_probe.rs`** (inputs#I1 §6, which places the work in that
  probe; the file exists at HEAD, 1948 lines). Six models: Llama-3.2-3B (baseline) · Qwen3.5-2B · Qwen3.5-4B · gemma-4-E2B ·
  gemma-4-E4B · Nemotron 3 Nano 4B. Each runs with its authors' sampling, a `--grammar-file` GBNF and thinking off, on the
  CUDA route (inputs#I1 §4; working-entry CONTEXT 3; inputs#I2).
- **The table and the recommendation.** The recommendation follows the rule fixed beforehand: *the lightest model whose
  A-cause is within 10 points of the best and whose B-false-alarm is no worse than the best + 10 points*. It is shown to the
  founder as a curve (A/B/C quality vs footprint). The founder decides, and the decision is the next entry's input, not this
  chunk's output (inputs#I1 §5).

## Boundaries
- **No product model, argv or sampling change.** The swap — model, sampling, GBNF — is the next working entry, "L4 runs the
  founder's pick with its authors' settings", and the GBNF needs the founder's own word at THAT entry's phase. Here the GBNF
  and sampling live in the probe only.
- **The enriched render is probe-only.** If C-enriched clearly beats C-today, the chunk proposes a separate entry, "the digest
  carries baselines and a short trend", and folds none of it into the product (inputs#I1 §6; working-entry CONTEXT 4).
- **Slow patterns (a horizon wider than 60 s) are out:** they are in the 0.4.0 incubator (working-entry CONTEXT 4; inputs#I1
  header).
- **No change to the shared L4 prompt framing or the predecessor's frozen S1–S6 grader.** The new scorer stands beside them.
- **No llama.cpp bump and no runtime swap:** b9305 stays (predecessor scope boundary; arch [LLM Inference Runtime]).
- **Real-model runs only in daytime and only inside the granted model slot** (inputs#I2).
- The synthetic texts are never committed. The committed evidence carries labels and aggregates only (inputs#I1 §3).

## Folded freight (the entry's four CONTEXT blocks and two CARRY blocks)
- **CONTEXT 1 — founder ruling 2026-10-05:** mint this entry first of three ahead of pre-push:linux, on the founder-approved
  design (approved whole), snapshotted verbatim at the wrap run dir and snapped here as inputs#I1. The snapshot's sha256 was
  re-derived at take-up as `9bd753447c656239885b32e2c4c5c7dbf54f55dc2d765b070a936405084fb879`, which matches the entry's
  `9bd753447c65…4fb879`.
- **CONTEXT 2 — the design's shapes and scoring:** A1–A7 · B1–B3 · C1–C3, ground truth written before any run, detect · cause
  · valid per generation, a 10 % blind grader audit, the recommendation rule fixed beforehand, and the founder deciding from
  the quality-vs-footprint curve. These are folded into §What the chunk builds above.
- **CONTEXT 3 — candidates and argv:** Llama-3.2-3B (baseline) · Qwen3.5-2B · Qwen3.5-4B · gemma-4-E2B · gemma-4-E4B · Nemotron
  3 Nano 4B, each with its authors' sampling, a `--grammar-file` GBNF and thinking off.
  - `[inferred]` "under the shipped json-schema argv qwen35 and gemma4 emit no JSON on b9305". Measured at
    `2026-10-05-l4-runs-on-a-small-current-model-chosen-by-measurement`, `evidence/series.md` (spot-read at take-up: §Leg 1
    §Diagnosis, `Failed to initialize samplers`, exit 0). This is a causal claim, so P3 re-verifies it.
  - That chunk's naming series read Qwen3.5-2B `PASS · rank1 37/40`, and gemma-4-E2B and Nemotron 40/40. The design (§0)
    reads that as saturated. Spot-read at take-up: series.md:203–211.
- **CONTEXT 4 — out of scope / follow-on:** slow patterns are in the 0.4.0 incubator. A C-enriched reading clearly above
  C-today proposes a separate "the digest carries baselines and a short trend" entry, never folded in. See §Boundaries.
- **CARRY 1 — the zero-generation trap:** "a zero-generation row reads `thinking absent` and a fast `elapsed_ms` in both
  worlds — record it as no reading, never as valid or a pass". Measured at the predecessor, report §Decisions & corrections.
  The scorer's `valid` label must therefore not pass a row that generated nothing. Such a row is `no reading`, never `valid`
  and never counted as a correct B dismissal.
- **CARRY 2 — the GPU-idle precondition:** "ports free and zero GPU compute processes" read red at the predecessor on the
  founder's own `voxtype-osd-gtk4` overlay (~10 MiB, not ours to close). The overseer ruled that the legs proceed, because the
  footprint is read per child PID (founder-delegated, 2026-10-05; the predecessor's `evidence/series.md` §Precondition,
  spot-read at take-up, lines 7–20). This plan states the precondition so that a foreign process is recorded, not read as red.

## Directive (inputs#I2) — folded
- The model slot and ports 4317/4318 are granted for this chunk, and conductor-builder stays idle. Daytime only.
- Each shape's ground truth and keyword set are written BEFORE any run.
- The synthetic texts are kept in gitignored `target/` for the blind 10 % audit (`target/` is ignored at `.gitignore:16`).
- All six models run under their authors' settings with GBNF and thinking off, "as in :170". **Coordinate re-verified:**
  `working-route.md:170` is the predecessor's entry, `2026-10-05-l4-runs-on-a-small-current-model-chosen-by-measurement`
  (inputs#I1 §6 also calls that chunk ":170"). "As in :170" therefore names that chunk's run settings: the `gb` arm,
  `--gbnf l4-output.gbnf`, per-model `--sampling` and `-c 8192 -rea off` (series.md:124–129). The directive does not mean a
  line of this entry.

## Premises closed at P3 (research.md)
- VERIFIED: the digest coordinates the design measured at `4e5b595` still hold at HEAD `2dc099a`.
  - `render_payload` is at `crates/triage/src/digest/assembler.rs:629`.
  - The per-row baselines are set equal to the current observation at `:584–598`.
  - The cueless cadences exist: `Tier3` (60 s window) and `Reflection` (1800 s), at `coordinator.rs:36-48,237-240`.
  - A cueless `cadence_tier3` digest reaches L4 (`process_digest` has no cue gate, `inference_runtime.rs:198-260`) and is
    composed with the primary prompt (`:395-425`).
- VERIFIED, with one structural extension: the probe can build every shape through the product's own `render_payload`.
  - Services, recent changes (`DigestProjectContext.recent_commits`, `assembler.rs:73-85`) and corpus matches
    (`format_corpus_match_line`, `retrieval.rs:136-148`) are all inputs `render_payload` already takes.
  - The probe's `Shape` carries exactly one cue (`l4_decision_probe.rs:168-173`), so the cue becomes optional.
  - A cueless shape renders under the `tier3` mode label (the 60 s cueless channel). A7's cued shape renders under `tier1`.
- VERIFIED: the enriched render is a probe-side text transform.
  - `DigestServiceRow` already carries `*_baseline` fields that `render_payload` never prints (`assembler.rs:684-694`).
  - The transform follows the probe's `truthful_overall` / `remove_lines` precedent (`l4_decision_probe.rs:362-380,620-636`).
  - No product seam is needed.
- VERIFIED: the six GGUFs are in the gitignored `AI-Model/`, listed by basename. `l4-output.gbnf` (gitignored, under
  `target/l4-decision-probe/`) reads sha256 `7b5cc276…ac03`, equal to the predecessor addendum. The sampling is reused:
  - the `samp()` table in `preregistration-addendum.md:73-96` covers Llama, Qwen and gemma;
  - Nemotron's row is in `preregistration-addendum-4.md:26-31`.

  Thinking-off per architecture:
  - qwen35 and nemotron_h: `-rea off` plus `--chat-template-kwargs {"enable_thinking":false}`;
  - gemma4 and Llama: `-rea off`;
  - and under `--grammar-file` the grammar applies from the first token.
- `[resolved at P4]` The `valid` atom's time budget. The design names none. The product holds two numbers:
  - the gpu-primary SLO, 5000 ms (`xtask/ci/l4-latency-p99.sh:9`);
  - the runner's wall-clock cut, 60 s (`llamacli_inference.rs:87`).

  The predecessor's two 4B-class models read GPU p50 above 5000 ms, so the choice is a P4 fork.
- VERIFIED (re-derived): the size is 13 shapes, with C's 3 in two renders, so 16 shape-renders × n=10 × 6 models = 960
  generations. At the predecessor's GPU p50 of 2.6–6.1 s that is about 1–1.6 h of GPU.
- Mechanism spot-checks:
  - CARRY 1: still true at the predecessor's `report.md:167-168`.
  - CARRY 2: still true at `evidence/series.md:7-20`.
  - CONTEXT 3, the grammar trap: still true at `series.md:49-70`. No real-model leg here passes `--json-schema-file`,
    so the trap is avoided by construction (the `gb` arm), not relied on.

## Founder-facing fact surfaced at P3 (not changed by this chunk)
- Today a cueless, non-reflection digest whose generation reads `surface` creates NO incident. It emits
  `interpretation.incident.skipped {skip_reason: no_cue}` and returns (`pulse-app/src/inference_runtime.rs:822-824`).
- So the table's A-detect measures what the MODEL catches. It becomes user-visible only after a product change that makes
  a cueless surface into an incident.
- That change is out of scope here (§Boundaries: no product change). If the founder's pick rests on A-detect, it is a
  candidate follow-on entry.

## Second fold source — the CI verdict read at Setup 5a
- `2dc099a` (the last wrap's flip, = HEAD): **`CI 2dc099a6: verdict not yet available`**. It read `in progress · checks 13/13`,
  with run `ci#37293411947` in progress (12 checks running; the oldest, `supply-chain (audit + deny + auditable)`, at 139 s)
  and `secret-scan#37293412073` completed/success. An unfinished run has no wall-clock yet. The run is not folded as green, and
  no red is dispositioned because none was read. P3/P5 re-read it.
- **Re-read at P3:** `CI 2dc099a6: verdict red`, first-fail +553 s, `boot smoke (ubuntu-22.04)` (job 111708927759,
  failed step "Boot pulse-app smoke"). Run `ci#37293411947` was still in progress when read (coverage gate and release
  build windows-latest running), so the job log was not yet readable. `secret-scan#37293412073` completed/success.
  - `git diff --name-only 5ac259e 2dc099a -- crates pulse-app xtask scripts Cargo.toml Cargo.lock deny.toml
    rust-toolchain.toml .github .cargo` prints nothing. The red sha differs from the green `5ac259e` (`verdict: green ·
    checks 13/13`) in docs, route and evidence files only.
  - Does the failing subject intersect what this chunk builds? **No.** The boot smoke boots the product binary, and
    this chunk touches only a dev example and evidence.
  - The disposition is the operator's word (promotion.md §Atomic order), asked at P4. **Answered: "Take up, red
    unowned"** (overseer, founder-delegated, 2026-10-05). The overseer's note: "measured now: run 37293411947 is still
    in progress, so its log is not readable yet. The diff 5ac259e..2dc099a touches no code path. Re-read the finished
    log at P5; on a runner flake re-run the failed job (gh run rerun --failed) and record the second reading."
  - So the red is recorded here as UNOWNED, not folded. P5 re-reads the finished job's log and names its cause.
- **Re-read at P5** (the job's log through the API, plus its uploaded artifact `logs-boot-Linux`). The cause, as read:
  - The failing step's harness printed `boot: ready (PID=6839 …)`, then `harness:status` read `"ended": "exit 1"` /
    `"verdict": "not-running"`.
  - The app's own log (51 lines) is clean through boot: both OTLP binds, the webview's first IPC, `ui-bridge.ready` and
    `viz.query.traces`.
  - It STOPS at 10:06:27.474 with no `app.exit`, no `app.panic.fatal` and no ERROR beyond the expected CI
    `corpus.open.error {KeyringUnavailable}`.
  - So the process ended about one second after the webview loaded, by an end the exit reporter cannot see (security.md
    `app.exit`: SIGKILL, `_exit`, pre-sink).
  - The tree is code-identical to the green `5ac259e`, so the reading is a runner-side flake candidate.
  - Per the overseer's word, the failed job is re-run once the run completes and the second reading is recorded below.

## Out of scope
- Shipping the chosen model (the next entry), "Without a GPU, L4 analysis is programmatic", and pre-push:linux.
- Moving the Conductor-side marker. Conductor v3-09 waits on the swap entry, not on this one.
- Turning the labelled shapes into Conductor "pattern plays" or a LoRA dataset (inputs#I1 §6, a later use).
