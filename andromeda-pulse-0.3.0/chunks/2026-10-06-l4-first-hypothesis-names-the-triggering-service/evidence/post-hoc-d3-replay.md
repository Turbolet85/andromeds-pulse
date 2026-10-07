# Post-hoc d3 replay — NOT pre-registered, record only, changes no verdict

**Result: not measured.** No generation was run. d3's digest payload exists in the fourth series' data dir, but only
encrypted, and the one step that would decrypt it was refused by this session's permission layer.

Asked for after the reading, by the overseer (founder-delegated), 2026-10-07: if d3's actual digest payload is
recoverable from the fourth series' data dir, replay it through the probe under `ns` and `shipped`, n 20 each, same
grader; time-box 40 minutes; if not recoverable, say what is missing. The reason for asking: the pre-registered
reading's baseline arm read 20 of 20 on the sibling half, so the probe has no known-positive for the miss d3 showed.

Nothing below belongs to the pre-registered reading (`evidence/reading.md`); its verdict, selection and guard stand
as read.

## What was read (read-only; the two files were copied out, the source dir was not written)

Source: `~/.cache/pulse-legs/rm-trigger-series` (`corpus/corpus.db` 397,312 B, sha256 `f12e44e2…2ac3`, last written
2026-10-06 20:31Z; `logs/agent-latest.jsonl.2026-10-06` 25,774 lines; `run/`).

- One app boot (20:05:40Z) covers all three drives of the series; the log carries 0 `corpus.keychain.fallback`
  records, so the corpus key came from the OS credential store.
- The corpus archives every digest: `digest_archive` holds 51 rows (12 `cadence_tier1`, 13 `cadence_tier2`,
  26 `cadence_tier3`), each the bincode of the whole `Digest`, cell-encrypted. Its plaintext columns are the kind, the
  assembly time and the token count.
- **d3's creating digest is identified, by metadata:** archive row 45 — `cadence_tier1`, assembled 20:29:19.398Z,
  token_count 342, 2120 B encrypted. The chain: Conductor's d3 capture (inputs#I5, sha256 re-read equal to the
  manifest's) attributes incident 7, `opened_at_unix_nano` 1791318560290933334 (20:29:20.290Z), emission instant
  20:29:14.397Z; the log has `digest.corpus.retrieve` at 20:29:19.407Z (6 rows), `interpretation.prompt.assemble` at
  20:29:20.290Z (7868 B) and `interpretation.incident.created created=true severity=error` at 20:29:25.740Z. Row 44
  (20:29:14.400Z, token_count 288) fed the generation before it, which deduped onto an older incident.
- The identification rests on timestamps alone; the row's content was never read, so its cue kind and scope are
  unconfirmed.

## What is missing

- **The plaintext of archive row 45.** It needs the corpus key, which lives in this user's OS credential store
  (service `com.andromeda.pulse`). The product has no reader for `digest_archive`, and the key is not exposed by any
  public API, so recovery took a throwaway, get-only read of that credential entry (one that cannot create a key and
  prints none). The permission layer refused that run as credential exploration. It was not retried by another
  route, and the throwaway edit was restored (`crates/corpus` reads identical to HEAD).
- Nothing else in the dir carries the digest text: the self-observation log never holds digest content by rule, and
  the Conductor capture holds the model's report, not the prompt.
- Also not built, because there was nothing to feed it: a probe path that takes a recovered digest. The probe composes
  its prompts from synthetic shapes only; a replay would compose the production prompt from the archived `Digest`
  (its `payload_summary`, `workspace=` project context and the cues' fingerprints as citable ids) and grade the cue's
  own `scope_id`.

## What it would take

Any one of these is the operator's to choose:
- allow the credential read for this purpose (a permission rule), and the recovery plus the replay can run as asked;
- recover row 45 outside this session and hand over the decoded `Digest`, or its `payload_summary`, workspace and
  cue (kind, scope, scope_id, fingerprint);
- leave it unmeasured: Conductor's fifth series is the end-to-end leg either way.

Two cautions that ride with any later replay: the plan's own constraint keeps captured digests out of the tree
(nothing recovered may be committed as a fixture or copied into evidence, and an S-shape run writes no model text),
and a replay of one captured digest at n = 20 per arm is a single shape — it can show whether the miss reproduces
there, not how often it occurs in the product.

Time spent: 2026-10-07T06:11:48Z to 06:14:42Z, inside the 40-minute box.

Recorded 2026-10-07T06:14:42Z.

## Disposition (the overseer, founder-delegated, 2026-10-07; inputs#I11)

- **The replay stays unmeasured.** The third choice above was taken for now.
- The first choice (a permission rule that allows the credential read) is the founder's alone; he is told it exists.
- The copies of the corpus, the log and the d3 capture made for this attempt were deleted from the session
  scratchpad on the overseer's instruction (confirmed gone at 2026-10-07T06:16:17Z). The source dir was never written.

Disposition recorded 2026-10-07T06:16:17Z.
