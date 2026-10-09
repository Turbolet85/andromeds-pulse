# L4 pattern-discrimination test — design draft for the founder (pc overseer, 2026-10-05)

**Founder, 2026-10-05: approved whole — «Да мне все нравится».** Slow patterns (a horizon wider than 60 s) → 0.4.0 incubator.

Founder, 2026-10-05: «смысл модели не только более красиво напечатать то что пульс и так ловит, а отловить и всякие нестандартные
патерны, и по этому надо баланс между легкостью и тупостью» → «Да придумай как это качественно потестить».

## 0. Why the current test cannot choose

S1–S6 are all retry storms that Pulse already caught (a cue is present); the model only has to NAME it. Every candidate scores 36–40
of 40 — the test is saturated and narrow, so "smallest that holds quality" picks the lightest model that can repeat the obvious.

## 1. What the model can see today (measured at Pulse `4e5b595`)

`render_payload` (`crates/triage/src/digest/assembler.rs:629`) gives ONE 60 s window:
- `WINDOW`, `PROJECT`, `RECENT CHANGES` (last 5 commits: age, basename, files changed);
- `OVERALL` nominal/anomalous/degraded, `TRIGGER` (first cue);
- `SERVICES`: per service rate/s · error% · p99 — the header says "vs baselines" but NO baseline is printed, and the baselines are
  set equal to the current observation (`assembler.rs:585-598`, "until chunk #82+ integrates L1b baseline state");
- `ATTENTION CUES` (5 detector kinds: error-rate spike, latency regression, restart, service went silent, retry storm);
- `CORPUS MATCHES` (similar past incidents).
Not in it: history/trend inside or across windows, call topology (who calls whom), per-endpoint breakdown, log/exception text.

The model also sees digests WITHOUT a cue: the Tier-3 baseline cadence (`crates/triage/src/cadence/coordinator.rs`) and the 1800 s
reflection cadence. That is the channel where "non-standard patterns" can be caught at all.

Consequence: some patterns are expressible today, some are not. A good test must separate "the model is too dumb" from "the digest
does not carry it" — otherwise we would blame the model for a missing input.

## 2. Shape families (synthetic digests, each with a ground truth written BEFORE any run)

**A. Catchable from today's digest — the model must surface it and name the cause (no cue, OVERALL nominal):**
- A1 Deploy correlation: one service at elevated errors, `RECENT CHANGES` 3 min ago touching it → cause: the recent change.
- A2 Blame the quiet one: checkout 30 % errors; payment 2 % errors but p99 5 s → cause: payment slowness (timeouts upstream).
- A3 Co-elevation: two unrelated services both slow at once, small errors → cause: a shared dependency / common downstream.
- A4 Low-volume failure: a 0.2/s service at 50 % errors, everything else fine → surface it despite tiny traffic.
- A5 Slow without errors: one service p99 2.4 s, 0 % errors, below a detector threshold → saturation / slow dependency.
- A6 Recurrence: corpus shows the same fingerprint several times this week → name it as recurring.
- A7 Wrong-trigger trap: a cue fires on service X, but the numbers point at Y as the origin → rank-1 should name Y.

**B. Controls — the model must NOT raise an alarm (decision dismiss/watch):**
- B1 Healthy: normal numbers, no cue.
- B2 Benign noise: 0.3 % errors at high rate, a batch job with a naturally high p99.
- B3 Deploy, no effect: recent commits, all numbers healthy → must not blame the deploy.
For a background helper a false alarm costs more than a miss; B measures it directly.

**C. Needs a richer digest — run in TWO renders, today's and an "enriched" one with baselines and a short trend:**
- C1 Traffic drop without errors (a silently dead consumer) — invisible without a baseline.
- C2 Slow creep (p99 rising window over window) — invisible without a trend.
- C3 Periodic spike (cron/GC) — invisible without history.
C under today's render is EXPECTED to fail and is recorded as "digest-limited"; C under the enriched render shows whether enriching
the digest (a separate product decision) would let a small model catch it. This tells us where the next investment belongs.

## 3. What is scored (labels only, decided before any run)

Per generation, from the model's JSON (bounded labels; the synthetic text is kept in gitignored `target/` for audit only):
- **detect** — decision is `surface` (A, C) / is not `surface` (B);
- **cause** — the rank-1 hypothesis names the ground-truth service AND a cause word from the shape's pre-declared set (e.g. A1:
  deploy|release|commit|change; A2: payment + latency|slow|timeout; A3: shared|common|dependency|downstream);
- **valid** — schema-valid, no thinking, inside the time budget.
Per model: A-detect %, A-cause %, B-false-alarm %, C-today %, C-enriched %, plus GPU latency / RAM / VRAM from the existing probe.

**Grader audit (quality guard):** keyword grading is crude, so 10 % of rows, drawn at random and with the model name hidden, are
read by the overseer against the ground truth; if the audit disagrees with the keyword label on more than 10 % of the sample, the
keyword sets are fixed and the WHOLE series is re-graded (not re-generated) before any choice.

## 4. Size and cost

~13 shapes × n=10 × 6 models (Llama baseline, Qwen3.5-2B, Qwen3.5-4B, Gemma 4 E2B, Gemma 4 E4B, Nemotron 3 Nano 4B) ≈ 800
generations at 3–7 s on the RTX 3090 → about 1 hour of GPU, daytime only. Same per-model authors' sampling, GBNF, thinking off.

## 5. How the choice is made

Not an automatic "lightest that passes". The table is shown to the founder as a curve (quality on A/B/C vs footprint), with an
overseer recommendation by a rule fixed beforehand: *the lightest model whose A-cause is within 10 points of the best and whose
B-false-alarm is no worse than the best + 10 points*. The founder decides the balance; the rule only makes the recommendation honest.

## 6. Where it lives

- The current chunk :170 closes with its results as recorded (naming test, confirmation, Nemotron rows).
- A new Pulse entry BEFORE the model swap: "L4 model chosen by pattern discrimination" — extends `l4_decision_probe` with the
  A/B/C shapes, the enriched render (probe-only), the scorer and the audit hook; runs the series; produces the table.
- The swap entry (model + sampling + GBNF) ships the founder's pick.
- If C-enriched beats C-today clearly, a separate entry: "the digest carries baselines and a short trend".
- The labelled shapes become Conductor's first ground-truth "pattern plays" later (the `scenario-flywheel` idea) and the seed of
  the LoRA dataset.

## 7. For the founder

1. Is this the right set of patterns, or are there failures from your own experience that matter more?
2. Is the false-alarm weight right (a background helper should rather stay quiet)?
3. Approve the new entry before the swap.
