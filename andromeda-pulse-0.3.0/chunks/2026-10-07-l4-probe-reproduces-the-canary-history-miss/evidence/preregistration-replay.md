# Pre-registration record — the replay reading

- **Chunk:** `2026-10-07-l4-probe-reproduces-the-canary-history-miss`
- **Chunk base:** `48714f09c95eaa0a2461d9a5ee26c96591983bb0`
- **Written:** 2026-10-07T13:13:30Z, at /implement Step 11, after the capture precondition entry and the capture section entry
  (neither spawns a model; `evidence/section-diff.md`) and before any replay generation. At this instant the builder
  had read no text of a captured prompt: both entries print hashes, byte and line counts and closed labels only.
- **Source:** `plan.md`, lines 73-97, the section copied byte for byte below
  (sha256 of the copied section: `35ee7efe4434a425761862f176c6d2d60c1558537d6c8f46c1ec0181fb7a6372`)
- **Amended before the run, on the operator's word** (inputs#I18, inputs#I19): the section "Amendments" at the end of
  this file. Where an amendment and the copied section differ, the amendment is the pre-registered rule.
- This file is never edited after the reading.

## The prompts, named before the run

No captured drive missed: Conductor graded all three drives of the capture run `Identified` (inputs#I20 `:274-276`).
So no prompt below is the prompt of a drive that missed. The label `miss` names a ROLE in this reading, the position
that missed in the fourth and fifth series (the third drive, six corpus candidates); it states nothing about what
this drive did.

| label | role | drive | directory name | sha256 of `prompt.txt` | bytes | rendered corpus lines |
|---|---|---|---|---|---|---|
| `miss` | candidate, the position that missed twice | d3 | `20261007T125156.990720047.348846` | `83b8a3ea7ccbcfbda4dfbc76f62ded32f7e6362d711d8ba9ad34bbb2ac63661e` | 8059 | 5 |
| `control` | control | d1 | `20261007T123304.212607066.88021` | `1d953dfbf29bfca03a3c6d4ef30188630a7f2469faab76e21d554783666e22ec` | 7616 | 1 |
| `d2` | candidate, read because of amendment 5 | d2 | `20261007T124239.786338234.251033` | `21c39d8cdf8e55191fb292168b8fb352a1ec61821cd2e882bbcc86ffa42409c8` | 7854 | 3 |

How each was identified: by the relay's table (inputs#I19 section 2), re-derived by this run from Conductor's own
records. Its ledger gives each drive's creating prompt-assembly instant and `token_count` (inputs#I20 `:183-185`:
12:33:04.207Z and 7616, 12:42:39.781Z and 7854, 12:51:56.985Z and 8059). Each directory's `started-utc` follows its
instant by 7 ms, and the byte count of its `prompt.txt` equals the logged count. In each drive's capture record the
attributed incident's `opened_at` is that same instant to the millisecond, and that incident is the drive's last
`created=true` record, logged one generation after the assembly. Bytes and corpus lines are the capture precondition
entry's readings.

---

## Pre-registration — the replay reading (fixed now, before any capture is read; /implement copies it to `evidence/preregistration-replay.md` in Step 11, the two prompts' sha256 beside it)
- **Instrument:** the probe as Steps 6-9 leave it. A replay passes a captured prompt's bytes unchanged as the `-p`
  operand of the product's argv (`build_llama_cli_args`), with the committed GBNF and the shipped model on the CUDA
  route (inputs#I9). Fired once; never re-run, re-ordered or re-thresholded.
- **Prompts:** two model invocations of the capture run against `f70be92`, each one directory the operator's wrapper
  wrote (inputs#I14 §2, inputs#I15).
  - `miss`: the creating prompt of the drive that missed, the third drive in both graded series.
  - `control`: the creating prompt of a passing drive, the second drive if it passed in the capture run, else the
    first.
  The evidence file names each by the sha256 of its `prompt.txt` and by how the relay identified it, before the run.
- **Arm:** `shipped`, the prompt verbatim. **n:** 20 per prompt, 40 generations.
- **Scope graded:** `conductor`, given on the command line. A prompt whose first cue line does not carry exactly that
  `scope_id` is INCONCLUSIVE before any spawn.
- **Label and per-prompt rule:** as in the first pre-registration: `identifies` over the first hypothesis statement, a
  service miss is `signal_only` or `neither`, `unparsed` is counted apart; REPRODUCES at 2 or more misses of 20, CLEAN
  when misses plus `unparsed` are 1 or fewer, UNREAD otherwise.
- **Verdict:**
  - REPRODUCED (exit 0) iff `miss` reproduces.
  - NOT REPRODUCED (exit 1) iff `miss` reads CLEAN. This is a second finding, never a pass: the miss is not determined
    by the prompt bytes. The chunk stops and reports, and no remedy is guessed (inputs#I14 §3).
  - INCONCLUSIVE (exit 2) iff `miss` is UNREAD.
- **Control in the verdict:** `control` is read by the same rule and printed. A control that reproduces is named in
  the report and voids the miss-against-control difference as an explanation; it does not change the verdict.
- **What is recorded only:** both prompts' label counts, the footprint line, the section differences of Step 11. At
  n = 20 a clean prompt shows that 20 generations did not miss on it.

---

## Amendments (the operator's, recorded before the run; inputs#I18, inputs#I19)

Whose word: the operator (the pc overseer), 2026-10-07, in this session's invocation (inputs#I18) and in the relay
file it names (inputs#I19 section 3: "a technical fork; the plan left it to the overseer when d3 does not miss").

1. **What the reading asks.** The plan's Step 11 stops when the third drive does not miss in the capture run, for the
   overseer's decision. It did not miss, and the decision is: replay all three scenario prompts. The reading therefore
   asks whether the real prompt of the position that missed in two of three live drives misses at a rate a single
   live drive can hide. It does not ask why a missing drive missed; no such drive's prompt was captured.
2. **Prompts:** three, as the table above names them, in place of the copied section's two. `control` is the first
   drive (the relay's choice), not the second.
3. **n:** 20 per prompt, 60 generations. Arm `shipped`, unchanged.
4. **Verdict**, by the copied section's per-prompt rule (REPRODUCES at 2 or more service misses of 20):
   - REPRODUCED (exit 0) iff `miss` or `d2` reproduces. The prompt that then takes the `miss` role in the remedy
     reading is `miss` (d3) when it reproduces, else `d2`.
   - NOT REPRODUCED (exit 1) iff `miss` and `d2` both read CLEAN. This is the second finding, never a pass: the chunk
     stops and reports, no remedy is guessed, and the overseer decides between a larger n on d3 and another capture
     run.
   - INCONCLUSIVE (exit 2) otherwise: neither reproduces and at least one of the two is UNREAD.
   - `control` is read by the same rule and printed, and moves no verdict. A control that reproduces is named in the
     report and voids the difference against it as an explanation. The relay does not rule the case of a control
     that reproduces while both candidates read CLEAN: the verdict is then NOT REPRODUCED, the control's count is
     reported beside it, and the chunk stops for the overseer as in every NOT REPRODUCED.
5. **A second count, recorded beside the bar. It selects nothing.** Per prompt, the generations whose FIRST
   hypothesis statement names `conductor-canary`: the closed label `names_other` (`named`, `not_named`, `unparsed`),
   read by `--count-naming conductor-canary`. The statement is ASCII-lowercased and the id must stand as a whole
   word, with the grader's own word rule; a `-`, `_` or space inside the id is read as any of the three. Stated
   limit: it reads no placement, so a statement that mentions the sibling in any way counts. It moves no verdict, no
   threshold and no exit. Why it is recorded (inputs#I19 section 4): a first hypothesis can quote the cue's scope and
   still place the storm in the sibling, the service bar reads that statement as naming the service, and a remedy
   that clears the bar while this count stays high has not fixed what was asked.
6. **Instrument.** The probe as it stood at the first gate block of this run plus two edits made before any capture
   entry fired: the replay verdict reads every replay but `control`, and the second count. Their pins and mutation
   checks 45-51 are in `evidence/mutation-checks.md`; the probe unit entry read 150 passed on that tree.
7. **The entry as it is fired:** the plan's replay reading entry with `,d2=$L4_REPLAY_D2` added to its `--replay`
   operand and `--count-naming conductor-canary` added after `--reproduce-misses 2`; every other token as the plan
   writes it. `L4_REPLAY_D2` is a third shell name of the leg, as the two the plan names are.
8. **Unchanged:** the scope graded (`conductor`), the label, the per-prompt rule and its threshold, fired once and
   never re-run, re-ordered or re-thresholded, and what is recorded only. At n = 20 a clean prompt shows that 20
   generations did not miss on it.
