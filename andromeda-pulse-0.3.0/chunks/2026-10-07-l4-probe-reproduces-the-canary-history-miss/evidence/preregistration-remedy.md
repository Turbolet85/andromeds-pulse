# Pre-registration record — the remedy reading

- **Chunk:** `2026-10-07-l4-probe-reproduces-the-canary-history-miss`
- **Chunk base:** `48714f09c95eaa0a2461d9a5ee26c96591983bb0`
- **Written:** 2026-10-07T13:31:14Z, at /implement Step 13, after the replay reading (`evidence/reading-replay.md`, REPRODUCED
  on `d2`) and before any remedy generation.
- **Source:** `plan.md`, lines 99-153, the section copied byte for byte below
  (sha256 of the copied section: `bd8a744aed617c2abf69db40025ed6e7b39f68936f552c906e249bab94eabd4d`)
- **Filled in and amended before the run:** the sections after the copy. Where they and the copied section differ,
  they are the pre-registered rule; each difference names whose word it rests on.
- **Amended:** 2026-10-07T13:34:30Z, before the slot read and before any remedy generation, on the operator's word
  given with the go (inputs#I23): `S17`'s titles are reworded in the builder's own sentence and re-pinned, and the
  third drive's prompt is read as a recorded-only third replay. The paragraphs those two changes touch state the
  amended form; nothing else was changed.
- This file is never edited after the reading.
- No text of a captured prompt is in this file. The builder read the three captured digests at Step 13, in its own
  tool calls (inputs#I19 section 5); what follows states derived facts in the builder's words.

---

## Pre-registration — the remedy reading (its rules fixed now; /implement writes `evidence/preregistration-remedy.md` in Step 13, after the replay reading and before the remedy run, with the measured numbers filled in)
- **Runs only after REPRODUCED**, on the same model and route. `shipped` is measured again in this run as the
  known-positive (inputs#I10). Fired once.
- **Remedy prompt:** the `miss` replay. With m its service misses of 20 in the replay reading, n = max(20, ⌈100 / m⌉):
  m = 2 → 50, m = 3 → 34, m = 4 → 25, m ≥ 5 → 20, so the baseline is expected to miss at least 5 times
  (inputs#I10). The file states m, n and the expected misses n × m / 20, and that this is the reason for n.
- **Arms:** `shipped` (the replay verbatim) and four candidates, each `shipped` with ONE edit of the prompt's corpus
  block, the lines from `CORPUS MATCHES:` to the end of the digest:
  - `CR` (restate) — one static line appended after the last corpus line:
    `(end of other incidents - the signal this digest reports is the cue line under ATTENTION CUES, for the scope_id written there)`
  - `CO` (reorder) — the own lines first, then the others; the order inside each group kept.
  - `CC` (cap) — the first 2 lines kept (the count S8 passes with).
  - `CX` (exclude) — the own lines kept and the others removed; when no line is left the header and its note go too.
  The **own lines** are those of incidents scoped to the cue's `scope_id`, plus any line the fingerprint arm keeps on
  its own, one whose fingerprint is in the window's set (`crates/triage/src/digest/retrieval.rs:94-102`; in the
  probe's shapes that set is the cue's fingerprint). It is the set the product keeps when its scope arm is narrowed
  to the triggering scope. For a synthetic shape the probe derives the set from the shape's incidents. A rendered line
  carries no scope, so for the replay the set is pre-registered by line position, with the reading it rests on; where
  the capture and its drive record do not settle a line, the set is `unknown`, `CO` and `CX` are not run, and the
  file says so. A candidate that composes byte for byte as `shipped` on the remedy prompt is skipped, printed as
  skipped, and is never selectable.
- **Guards:** `S7`, `S8` and `S1`-`S4`, 20 generations each. Where a candidate composes byte for byte as `shipped` on
  a guard shape, or on `S17`, it takes `shipped`'s counts there and is not generated again.
- **Control prompt:** the `control` replay, 20 generations per arm, recorded only.
- **Derived shape** (inputs#I16, inputs#I17): `S17`, ONE synthetic shape derived from what the section reader shows
  the miss prompt holds and `S14` / `S16` lack, written with no capture text. `shipped` and every candidate read it
  at 20 generations in this run, and every count is recorded. It is no part of the bar, the guard, the known-positive
  rule or the selection, and cannot make the reading INCONCLUSIVE: the ground truth for shipping is the real prompt.
  By the first reading's per-shape rule it REPRODUCES iff `shipped` reads 2 or more service misses of 20 on it.
  - REPRODUCES: `S17` is committed as the probe's standing known-positive. Then, read after the selection:
    - COVERED iff the selected candidate's generations that are not `both` are 1 or fewer of 20 on `S17`. `S17` is
      then the standing guard for the remedy, the baseline arm `nb` its failing side (Step 15).
    - NOT COVERED otherwise: `S17` is a known-positive the remedy does not cover, and the report names that as an
      open finding.
    - Selection `none`: neither; `S17` stays the known-positive under `shipped`.
  - CLEAN or UNREAD: the report says the probe keeps no durable known-positive, and why.
  - No shape can be derived from the difference, or it is not built and green when the go is asked: the remedy
    reading runs without one, and the report says why.
  The file describes `S17` as the first Pre-registration's table describes `S9`-`S16` and names the section
  differences it carries. It is derived once and read once, and is never re-derived after a reading.
- **Allowance:** ⌊n / 20⌋ generations that are not `both` on the remedy prompt (1 at 20, 25 and 34; 2 at 50).
- **Known-positive, in this run** (inputs#I11): HELD iff `shipped` reads at least the allowance plus 2 service misses
  on the remedy prompt: 3 or more at n = 20, 25 and 34, 4 or more at n = 50. Otherwise LOST, INCONCLUSIVE (exit 2):
  the run holds nothing to measure a remedy against and the chunk stops for the founder. The accepted price: at an
  expected 5 misses the baseline falls short of 3 about one time in eight and short of 4 about one time in four
  (Poisson at 5: 0.125 and 0.265).
- **Bar, per candidate:** MET iff its generations that are not `both` are within the allowance on the remedy prompt,
  AND `both` is 38 or more of 40 over `S7` + `S8`, AND 72 or more of 80 over `S1`-`S4`. An `unparsed` generation is
  not `both`.
- **Regression guard, per candidate:** TRIPPED iff its `both` count is lower than `shipped`'s in this run over
  `S7` + `S8` or over `S1`-`S4`; an equal count holds. A tripped candidate is not selectable.
- **Selection:** in the order `CR`, `CO`, `CC`, `CX`, the first candidate whose bar is MET and whose guard HOLDS;
  `none` if none. The order is least change first. No combination is run.
- **Verdict:** SELECTED (exit 0) with the arm named, or NONE (exit 1). NONE is recorded as measured: no product
  change ships, and the report says the miss reproduces on the replayed prompt and no listed candidate meets the bar.

---

## The remedy prompt, and the numbers

- **The prompt in the `miss` role is the second drive's** (the replay pre-registration's amendment 4, confirmed with
  the go, inputs#I21): the `miss` label's own prompt (d3) read CLEAN in the replay reading and `d2` reproduced. In
  this run the label `miss` therefore names directory `20261007T124239.786338234.251033`, `prompt.txt` sha256
  `21c39d8cdf8e55191fb292168b8fb352a1ec61821cd2e882bbcc86ffa42409c8`, 7854 bytes, 3 rendered corpus lines. The
  replay reading read that prompt under the label `d2`, so the entry passes `--remedy-read-as d2`.
- **`control`** is the first drive's prompt, as in the replay reading: directory `20261007T123304.212607066.88021`,
  sha256 `1d953dfbf29bfca03a3c6d4ef30188630a7f2469faab76e21d554783666e22ec`, 7616 bytes, 1 line. It read 19 of 20
  `both` in the replay reading, so the plan's stop on a control under 19 does not fire.
- **m = 11** (the replay reading's service misses of 20 on that prompt). **n = max(20, ceil(100 / 11)) = 20.**
  **Expected baseline misses: n x m / 20 = 11**, above the 5 the sizing asks for, which is why n stays at 20.
- **Allowance:** floor(20 / 20) = 1 generation that is not `both`. **Known-positive HELD** iff `shipped` reads 3 or
  more service misses on the remedy prompt in this run.
- **Bar, guard, selection order, verdict:** as the copied section states them, unchanged.

## The own lines of the remedy prompt: `2`

Pre-registered by position, `--own-lines 2`: of the block's three lines the second alone is the triggering scope's
own. The reading it rests on, none of it a line's title:

- Line 2 carries the fingerprint that is the prompt's one citable evidence id. The citable ids are the window's
  fingerprint set (`pulse-app/src/inference_runtime.rs:411`), the set the selection's fingerprint arm reads
  (`crates/triage/src/digest/retrieval.rs:94-102`), so the product keeps that line when its scope arm is narrowed.
  Conductor's record of the drive names that id's incident as the one earlier incident carrying the scenario's
  fingerprint, the first drive's own (inputs#I22 `:407-412`).
- Lines 1 and 3 each carry another fingerprint, neither a citable id. Conductor's record lists the other incidents
  opened before this digest as not carrying the scenario's fingerprint, and records the canary's own retry-storm
  cue creating an incident in each of the first two drives (inputs#I22 `:549`; its first-drive record `:530`). The
  scenario's cue is the only one scoped to `conductor` in the run.

So `CO` moves line 2 first, `CC` keeps lines 1 and 2, `CX` keeps line 2 alone, and `CR` appends its line after
line 3. `control` has no pre-registered own lines: `CO` and `CX` are skipped on it and printed as skipped.

## What the difference shows the synthetic shapes lacked

Read from the three captured digests beside `evidence/section-diff.md`, `S14` and `S16`:

1. **The services table.** In the reproducing prompt the sibling's row stands before the triggering service's, and
   the two rows read the same values: the same low rate, a 100 % error rate, a zero p99. Every sibling shape puts
   the triggering service first and gives the rows different rates and latencies. The third drive's table has the
   reproducing prompt's order; the control's has the triggering service first.
2. **`OVERALL:`** counts one active incident (the third drive's counts two, the control's one). Every shape counts
   none.
3. **The corpus block** holds three lines, all retry storms: the sibling's newest (3 m, active), the triggering
   service's own earlier storm (9 m, resolved), the sibling's again (12 m, resolved). Each line has its own
   fingerprint, and only the own line's is the cue's. Each title reads as a detection of the kind in a named
   service and ends on that service; the sibling's scope_id stands in two of them, and the own line names the
   triggering service as a display name. `S16` holds the first two kinds of line but with titles that read as a
   hypothesis statement, five lines, two of them error-rate spikes, and two fingerprints in all.
4. **Not in the block:** the cue line, the `TRIGGER:` line, the window line and the prompt template are the same
   bytes in all three captured prompts and in the shapes.

What this does not settle: which of 1 to 3 carries the miss. The third drive's prompt has the same services order
and read 20 of 20; it differs from the reproducing prompt in `OVERALL:` and the corpus block alone.

Nothing in the reading shows that only the service on the `TRIGGER:` line would fix the miss, and it suggests no
candidate beyond the four: the services order is not an edit of the corpus block and is no remedy candidate here.

## The derived shape `S17`

| shape | what it holds | differs from S16 in |
|---|---|---|
| `S17` | S7's cue. Two services rows, the sibling's first, both at the same low rate, a 100 % error rate and a zero p99. One active incident counted on `OVERALL:`. Three retry-storm corpus lines whose titles state, in the builder's own sentence, that a named service is caught in a retry storm: the sibling's (3 m, active, a fingerprint other than the cue's), the triggering service's own (9 m, resolved, the cue's fingerprint), the sibling's again (12 m, resolved, a third fingerprint) | services order and values, the active-incident count, the block's count, kinds, fingerprints and title form |

- **Section differences it carries against `S16`:** `digest.overall`, `digest.services`, `digest.corpus-matches`,
  and no other (pinned). Its `shipped` composition is 7824 bytes; its own line is the second.
- **How it was written.** The rows, the count and the block are built by the probe's builders through the product's
  render. The rate is the builder's value, not the captured one, and the fingerprints are synthetic. The titles as
  first written differed from the captured titles in their verb alone; the operator ruled that this is the text and
  not a form (inputs#I23), and they were reworded before any reading. Each title now keeps what the captured title
  at its position names and nothing of its wording: the service (the sibling's scope_id in two, the triggering
  service as a display name in one), the signal kind, a one-clause statement, a length of the same class, and the
  three letter cases the block holds. The sentence frame is the builder's: the service stands first and the kind
  last. They are never changed after the reading.
- **Against the captured prompt it was derived from**, `S17` still differs in `project-context`, `digest.project`
  and `citable-evidence-ids` (the probe's fixed project and id), in `digest.services` (the rate) and in
  `digest.corpus-matches` (the fingerprints and the titles; 462 bytes against 453, 5 lines each); its `OVERALL:`,
  `TRIGGER:` and cue lines are the same bytes, each a line the product renders from the same counts and kinds.
- It is read under `shipped` and every candidate at 20 generations, selects nothing, and is never re-derived.

## Amendments and additions (each with whose word it rests on)

1. **The second count is recorded in this reading too** (the operator, inputs#I19 section 4, inputs#I21): the entry
   passes `--count-naming conductor-canary`, and the report gives the `names_other` count beside every arm's
   counts on the remedy prompt, the control and `S17`. It selects nothing and moves no bar, guard or exit.
2. **The entry as it is fired:** the plan's remedy reading entry with `,d3=$L4_REPLAY_D3` added to its `--replay`
   operand (amendment 5), `--remedy-read-as d2` after `--remedy-from <id>` and `--count-naming conductor-canary`
   after `--bar-ordinary 72`; `<id>` is `target/l4-decision-probe/replay-20261007T131536Z`; `L4_REPLAY_MISS` names
   the second drive's directory, `L4_REPLAY_D3` the third drive's, and `L4_REPLAY_OWN_LINES` is `2`. Every other
   token as the plan writes it.
3. **Instrument:** the probe as the replay reading used it plus the Step 13 edits: the shape field for the
   active-incident count, `S17`, and `--remedy-read-as`. Their pins and mutation checks 52-60 are in
   `evidence/mutation-checks.md` (58-60 after the titles were reworded); `S1`-`S16` compose byte for byte as before
   under all 22 arms (the dry-run and the section rows of the builds compared equal). The gate block read 19 green,
   0 red on the tree the reading uses (probe file `e9b409d7…b2a7`; entry 5: 153 passed).
4. **What the run generates** (the entry's dry run, 2026-10-07T13:34:25Z, no model): 25 arm-and-source cells of 20
   generations, 500 in all, predicted about 52 minutes at the replay reading's 6.2 s each.
   - `shipped`: the remedy prompt, the control, `d3`, `S1`-`S4`, `S7`, `S8`, `S17`.
   - `CR`: the remedy prompt, the control, `d3`, `S4`, `S8`, `S17`; it takes `shipped`'s counts on `S1`-`S3` and
     `S7`.
   - `CO`: the remedy prompt and `S17`; `shipped`'s counts on `S1`-`S4`, `S7`, `S8`; skipped on the control and on
     `d3`.
   - `CC`: the remedy prompt, the control, `d3` and `S17`; `shipped`'s counts on `S1`-`S4`, `S7`, `S8`. On the
     control it composes as `shipped` and is generated again there, recorded only.
   - `CX`: the remedy prompt, `S4`, `S17`; `shipped`'s counts on `S1`-`S3`, `S7`, `S8`; skipped on the control and
     on `d3`.
   - No candidate composes as `shipped` on the remedy prompt, so none is skipped whole.
5. **The third drive's prompt is read as a recorded-only third replay, labelled `d3`** (the operator, inputs#I23;
   the builder's proposal until then). Directory `20261007T125156.990720047.348846`, `prompt.txt` sha256
   `83b8a3ea7ccbcfbda4dfbc76f62ded32f7e6362d711d8ba9ad34bbb2ac63661e`, 8059 bytes, 5 rendered corpus lines. It is
   the position the series missed on and read 20 of 20 in the replay reading; under `shipped`, `CR` and `CC` it
   shows whether a candidate disturbs it. `CO` and `CX` are skipped on it, its own lines not being pre-registered,
   so a selection of `CO` or `CX` leaves it unread under the selected arm, and the report says so. `d3` enters no
   bar, no guard and no selection.
