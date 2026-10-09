# Series — L4 rank-1 hypothesis names the retry on every storm shape

Pre-registered in `plan.md` (Steps 6 and 8) before any generation. Every slot entry was fired ONCE, verbatim, on the
model slot granted in `inputs#I1`, through the leg env `inputs#I2`. Bounded labels and counts only: no model, prompt or
digest text, and no absolute host path. Every count below was re-derived from the copied `*-runs.json` files in this
folder, not from stdout. It agrees with the stdout record line for line.

## Verdict

**Slot 2: `FAIL · rank1 34/40`** against the pre-registered bar of 36 (strict `names_trigger`, frozen grader).
Per shape: S1 9 · S2 9 · S3 7 · S4 9. This is recorded as measured, never as passed.

## Pre-flight (2026-10-04T23:5xZ, before Slot 1)

- GPU: RTX 3090 at 0 % utilisation, 971 MiB used. The only compute client was a 10 MiB desktop overlay
  (`voxtype-osd-gtk4`); no llama-cli was running.
- 0 `ANDROMEDA_PULSE_*` exports in the launching shell. 0 listeners on 4317/4318.
- The leg env file's sha256 was equal to `inputs#I2`'s (`4a28d0fd…f77647`) when read at fire time.
- The product tree was untouched against the chunk base `200312b` (`git diff --stat 200312b -- crates pulse-app/src`
  was empty) when Slot 1 fired. Only the probe example had changed.
- Dry-run (plan entry 5, every arm × S1–S6): exit 0; every composition was at most 7,527 B of the 16,384 bound.
  - Before Step 7: shipped S1–S6 read 6984 / 6987 / 6991 / 7185 / 6983 / 6983 B. The S1–S4 figures are identical to
    the predecessor's.
  - After Step 7: shipped S1–S6 read 7204 / 7207 / 7211 / 7405 / 7203 / 7203 B, equal to the R1 arm byte for byte
    (R1 composes as identity).
- Build: each slot entry built AFTER sourcing the env. The triage build used the local tokenizer override, with no
  fetch. This closes the predecessor's plan defect.

## Slot 1 — selection, untouched product tree (plan entry 15)

Fired 2026-10-04T23:53:59Z, ended 2026-10-05T00:01:51Z (7 m 52 s), exit 0, 160 rows (4 arms × 4 shapes × 10).

```
series out target/l4-decision-probe/slot1-20261004T235359Z
l4-decision-probe: binary llama-cli (cuda, -ngl 99) · model Llama-3.2-3B-Instruct-Q4_K_M.gguf · arms shipped,R1,R3,R2 · shapes S1,S2,S3,S4 · n 10 per shape
arm shipped: would_create 40/40 · decision 37/0/3 · severity_none 0 · resolution_summary 0
  arm shipped: per shape would_create S1 10 S2 10 S3 10 S4 10 · failed 0 · first_keys schema_version>prompt_version>title=40 · distinct outputs S1 10 S2 10 S3 10 S4 10 · wall 116s
  arm shipped: names_trigger rank1 28/40 · elsewhere 8 · none 4 · unparsed 0 · per shape rank1 S1 8 S2 7 S3 5 S4 8
  arm shipped: names_trigger_stem rank1 29/40 · elsewhere 7 · none 4 · unparsed 0 · per shape rank1 S1 8 S2 7 S3 5 S4 9
arm R1: would_create 40/40 · decision 39/0/1 · severity_none 0 · resolution_summary 0
  arm R1: per shape would_create S1 10 S2 10 S3 10 S4 10 · failed 0 · first_keys schema_version>prompt_version>title=40 · distinct outputs S1 10 S2 10 S3 10 S4 10 · wall 116s
  arm R1: names_trigger rank1 37/40 · elsewhere 3 · none 0 · unparsed 0 · per shape rank1 S1 8 S2 10 S3 9 S4 10
  arm R1: names_trigger_stem rank1 37/40 · elsewhere 3 · none 0 · unparsed 0 · per shape rank1 S1 8 S2 10 S3 9 S4 10
arm R3: would_create 40/40 · decision 38/0/2 · severity_none 0 · resolution_summary 0
  arm R3: per shape would_create S1 10 S2 10 S3 10 S4 10 · failed 0 · first_keys schema_version>prompt_version>title=40 · distinct outputs S1 10 S2 10 S3 10 S4 10 · wall 119s
  arm R3: names_trigger rank1 36/40 · elsewhere 4 · none 0 · unparsed 0 · per shape rank1 S1 10 S2 9 S3 7 S4 10
  arm R3: names_trigger_stem rank1 36/40 · elsewhere 4 · none 0 · unparsed 0 · per shape rank1 S1 10 S2 9 S3 7 S4 10
arm R2: would_create 39/40 · decision 31/0/9 · severity_none 1 · resolution_summary 0
  arm R2: per shape would_create S1 10 S2 10 S3 9 S4 10 · failed 0 · first_keys schema_version>prompt_version>title=40 · distinct outputs S1 10 S2 10 S3 10 S4 10 · wall 116s
  arm R2: names_trigger rank1 33/40 · elsewhere 6 · none 1 · unparsed 0 · per shape rank1 S1 6 S2 8 S3 9 S4 10
  arm R2: names_trigger_stem rank1 35/40 · elsewhere 4 · none 1 · unparsed 0 · per shape rank1 S1 7 S2 9 S3 9 S4 10
```

Combination entries 16 (R1R3), 17 (R1R2) and 18 (R3R2): **not fired — rule did not call it.**

## Step 6 — the decision rule, every branch taken

`r(shipped) = 28` · `r(R1) = 37` · `r(R3) = 36` · `r(R2) = 33` (strict `names_trigger` rank1 out of 40).

1. *Defect not reproduced?* `r(shipped) = 28 < 36`, so no. The defect reproduced.
2. *Qualifiers* (`r >= 36` among R1, R3, R2): **R1 (37), R3 (36)**. R2 (33) does not qualify.
3. *Selection*, the first qualifier in least-blast-radius order R1 → R3 → R2: **R1 ships.**
4. *None qualifies* / *Still none*: not reached. The combine-once rule did not fire.

No arm was re-run or reordered. The stem readings did not enter the rule.

## Step 7 — the applied text

- The product's `TRIGGER_FRAMING_INSTRUCTION` was replaced by the probe constant `R1_FRAMING_INSTRUCTION`. A script
  confirmed the two constants equal byte for byte (538 bytes, ASCII, no cue-kind word), and the R1 arm now composes
  as identity.
- Lineage bumped together: `PROMPT_VERSION_PRIMARY` v2.4 → v2.5, `_FALLBACK` v1.3-fallback → v1.4-fallback,
  `_REFLECTION` v1.3-reflection → v1.4-reflection.

## Slot 2 — confirmation, fixed tree (plan entries 19 and 20)

**Verdict entry (19).** Fired 2026-10-05T00:05:26Z, ended 00:07:25Z (1 m 59 s), exit 1 (FAIL, a completed record),
40 rows.

```
series out target/l4-decision-probe/slot2-20261005T000526Z
l4-decision-probe: binary llama-cli (cuda, -ngl 99) · model Llama-3.2-3B-Instruct-Q4_K_M.gguf · arms shipped · shapes S1,S2,S3,S4 · n 10 per shape
arm shipped: would_create 40/40 · decision 38/0/2 · severity_none 0 · resolution_summary 0
  arm shipped: per shape would_create S1 10 S2 10 S3 10 S4 10 · failed 0 · first_keys schema_version>prompt_version>title=40 · distinct outputs S1 10 S2 10 S3 10 S4 10 · wall 114s
  arm shipped: names_trigger rank1 34/40 · elsewhere 2 · none 4 · unparsed 0 · per shape rank1 S1 9 S2 9 S3 7 S4 9
  arm shipped: names_trigger_stem rank1 36/40 · elsewhere 1 · none 3 · unparsed 0 · per shape rank1 S1 10 S2 9 S3 8 S4 9
l4-decision-probe: names-trigger verdict: FAIL · rank1 34/40
```

**Held-out entry (20)**, record-only and unthresholded. Fired 00:07:25Z, ended 00:08:25Z (1 m 00 s), exit 0, 20 rows.
S5 and S6 were authored at P4, never ran in Slot 1 and never fed the rule.

```
series out target/l4-decision-probe/slot2-heldout-20261005T000725Z
l4-decision-probe: binary llama-cli (cuda, -ngl 99) · model Llama-3.2-3B-Instruct-Q4_K_M.gguf · arms shipped · shapes S5,S6 · n 10 per shape
arm shipped: would_create 20/20 · decision 18/0/2 · severity_none 0 · resolution_summary 0
  arm shipped: per shape would_create S5 10 S6 10 · failed 0 · first_keys schema_version>prompt_version>title=20 · distinct outputs S5 10 S6 10 · wall 59s
  arm shipped: names_trigger rank1 20/20 · elsewhere 0 · none 0 · unparsed 0 · per shape rank1 S5 10 S6 10
  arm shipped: names_trigger_stem rank1 20/20 · elsewhere 0 · none 0 · unparsed 0 · per shape rank1 S5 10 S6 10
```

## Strict vs stem, per shape per arm (rank1 out of 10)

| run · arm | reading | S1 | S2 | S3 | S4 | S5 | S6 | total |
|---|---|---|---|---|---|---|---|---|
| Slot 1 · shipped (v2.4) | strict | 8 | 7 | 5 | 8 | — | — | 28/40 |
| | stem | 8 | 7 | 5 | 9 | — | — | 29/40 |
| Slot 1 · R1 | strict | 8 | 10 | 9 | 10 | — | — | 37/40 |
| | stem | 8 | 10 | 9 | 10 | — | — | 37/40 |
| Slot 1 · R3 | strict | 10 | 9 | 7 | 10 | — | — | 36/40 |
| | stem | 10 | 9 | 7 | 10 | — | — | 36/40 |
| Slot 1 · R2 | strict | 6 | 8 | 9 | 10 | — | — | 33/40 |
| | stem | 7 | 9 | 9 | 10 | — | — | 35/40 |
| Slot 2 · shipped (v2.5 = R1) | strict | 9 | 9 | 7 | 9 | — | — | **34/40** |
| | stem | 10 | 9 | 8 | 9 | — | — | 36/40 |
| Slot 2 held-out · shipped (v2.5) | strict | — | — | — | — | 10 | 10 | 20/20 |
| | stem | — | — | — | — | 10 | 10 | 20/20 |

Rows where stem and strict differ, read per row from the copies: Slot 1 shipped 1 (`elsewhere`→`rank1`), R2 2
(`elsewhere`→`rank1`), Slot 2 shipped 2 (one `elsewhere`→`rank1`, one `none`→`rank1`). Every other row is equal under
both readings.

## The stem reading beside the recorded 30/40

- The predecessor's v2.4 series recorded strict `rank1 30/40` (S1 9 · S2 6 · S3 5 · S4 10). Its stem reading is
  **unrecoverable**: its 40 rows kept labels only.
- The measured counterpart is this chunk's Slot 1 `shipped` arm, the same v2.4 composition on the same S1–S4. It reads
  strict **28/40**, stem **29/40**. On that arm the instrument (substring `retry`, missing `retries` / `retried`)
  accounts for **1 of 40** generations; the remaining shortfall is wording.
- Slot 2's stem reading is 36/40 against strict 34/40. That is a record-only figure and never the verdict. The bar is
  defined on the strict grader.
- At n = 10 per shape, a two-count gap between readings is within run-to-run variation. The predecessor's 30 and
  Slot 1's 28 on the same composition differ by two.

## Mutation checks (plan Steps 5 and 7; each applied, grep-confirmed, then run, never batched)

| mutation | pin(s) red under it | reading | reverted, green after |
|---|---|---|---|
| stem terms collapsed to `["retry"]` | `stem_reads_rank1_for_retries_and_retried_where_the_strict_grader_does_not` · `stem_grader_label_set_is_exactly_the_closed_set` | 14 passed, 2 failed (`left: "none"` vs `right: "rank1"`) | yes, 16/16 |
| `DEFAULT_SHAPES` widened to S1–S5 | `shapes_default_is_s1_through_s4` | 15 passed, 1 failed | yes, 16/16 |
| product framing text reverted alone to its v2.4 wording | `every_tier_obliges_the_first_hypothesis_to_name_the_trigger_signal` | 127 passed, 1 failed (`left: 0` vs `right: 1`) | yes, 128/128 |

## Routing witness

The probe installs no tracing subscriber, so no self-observation log exists for these runs and routing is not
attested by one. The witness is the probe header in all three runs: `binary llama-cli (cuda, -ngl 99) · model
Llama-3.2-3B-Instruct-Q4_K_M.gguf`, routed by detection (`ANDROMEDA_PULSE_HARDWARE_PROFILE` unset). The obs counts
"0 `<redacted>`" and "0 `app.panic.fatal`" have no log family to read and are not applicable.

## Process census after the slots

Read with `ps` and `nvidia-smi` at the stop-time census (2026-10-05, after 00:08:25Z):

| process | started by | final state |
|---|---|---|
| `cargo build` (Slot 1, Slot 2 verdict entries) | this run, via the slot entries | terminated (exited inside its entry) |
| `l4_decision_probe` (×3 runs) | this run, via the slot entries | terminated (exits 0 / 1 / 0) |
| `llama-cli` (220 spawns) | the probe, `kill_on_drop` | terminated; 0 llama-cli in the census |
| GPU | — | 0 % utilisation, 971 MiB, only the desktop overlay as a client |

## Copied records (sha256: source under `target/l4-decision-probe/{run}/runs.json` = copy here)

| copy | rows | sha256 (source = copy) |
|---|---|---|
| `slot1-runs.json` | 160 | `ba72d00e68cc8d20bd8d2c373ff340e89bfad2abd70ca4069ee28464767540c8` |
| `slot2-runs.json` | 40 | `4ae8e1628f0f3959eab40c00183536cbe9a94e50538d9cd05908f728d0479f39` |
| `slot2-heldout-runs.json` | 20 | `dc293f3a8a41f3a593a9d1e212ff081d044409af76dac25e8b3a4d14baf0cdd4` |

No `slot1-combo-runs.json` exists: the combine-once rule did not fire.

## Hygiene entry (plan entry 21), driven by hand after this file was written

Summary line, verbatim (the header lines carry host paths and are left out):

```
hygiene: clean — read 36 (runs 28 · evidence 4 · inputs 4) · trails 14 not read · copies 2 not read by P1 — 0 host paths kept · binary 0 not read by P1
```

Exit 0. Re-fired after this section was added (an evidence edit). The re-fire's summary is in the implement report.
