# The product path against the `CX` arm, on the three captured scenario prompts (a dry run)

- **Asked by:** the operator's relay for this re-entry (inputs#I29, section 2): a real prompt composed by the product
  path must equal what the probe's `CX` arm composed on the same capture, for the three scenario prompts, by a dry
  run; if that cannot be shown, say so.
- **Reading: the same, on all three, byte for byte.** The comparison is made inside the probe and prints closed words
  and counts. It takes two things as given that no capture holds, named below with what each rests on.
- **Written:** 2026-10-07T18:30:23Z, at the re-entry of /implement that swaps the product change from `CO` to `CX`.
  No generation ran; no GPU was used.
- No text of a captured prompt is in this file. The probe read each prompt in place; the facts below are counts,
  sizes, positions, closed words and clock times.

## What "the product path" can mean for a capture

A capture is a finished prompt. The product composes one from a digest's inputs, and those are not in a capture: the
corpus incidents sit in the cell-encrypted corpus, which no step of this chunk opens (scope.md, Boundaries). So the
product cannot be run end to end over a capture. What can be run is the part the remedy changes:

1. Each line of the capture's corpus block is read back into an incident: its fingerprint, title, age and status.
   A line is accepted only when the product's `format_corpus_match_line` writes it back byte for byte.
2. Each incident is scoped: to the cue's service at the pre-registered own positions, to another service of the
   digest's table elsewhere.
3. The product's `select_corpus_matches` runs over them, with the digest's services as the active scopes and the
   prompt's citable fingerprints as the window's.
4. The product's line format and `render_payload` write the block, and it replaces the capture's block. Every other
   byte of the prompt is kept: production passes no corpus argument to the prompt builder
   (`pulse-app/src/inference_runtime.rs:423`), so the selection reaches a prompt through the digest's block alone.

It runs twice: with no triggering scope, compared with the capture itself, and with the cue's scope, compared with
the `CX` edit of the capture. The first is the control of the instrument: if the lines read back do not compose the
capture again, the second comparison says nothing.

## The runs

Fired 2026-10-07T18:26:57Z on the probe built from the tree of this re-entry, the model variables unset. Each is
`<probe> --arms shipped,CX --replay "miss=<capture dir>" --replay-scope conductor --own-lines <positions> --dry-run
--product-path`, the label `miss` a slot as in the third reading. Each exited 0.

| drive | sha256 of `prompt.txt` | own lines | `shipped` | `CX` arm | product path |
|---|---|---|---|---|---|
| d1 | `1d953dfbf29bfca03a3c6d4ef30188630a7f2469faab76e21d554783666e22ec` | `none` | 7616 B, 1 line | 7401 B, 0 lines | unremedied same · under the cue scope same as CX · kept 0 of 1 lines |
| d2 | `21c39d8cdf8e55191fb292168b8fb352a1ec61821cd2e882bbcc86ffa42409c8` | `2` | 7854 B, 3 lines | 7611 B, 1 line | unremedied same · under the cue scope same as CX · kept 1 of 3 lines |
| d3 | `83b8a3ea7ccbcfbda4dfbc76f62ded32f7e6362d711d8ba9ad34bbb2ac63661e` | `2,5` | 8059 B, 5 lines | 7732 B, 2 lines | unremedied same · under the cue scope same as CX · kept 2 of 5 lines |

The three hashes are the ones every earlier pre-registration of this chunk names, and the `CX` sizes are the sizes
the third reading generated from (`evidence/preregistration-third.md`, the cells).

- **The relay asks for hashes.** The probe carries no hash crate (the plan's lean, kept), and a composed prompt is
  never written anywhere a hash tool could read it. The comparison is the stronger one, equality of the bytes,
  made in the process that holds them.
- **`unremedied same` on all three** is the control reading: the block read back, selected with no triggering scope
  and rendered by the product, is the captured prompt again, byte for byte.

## The comparison can read `differs`, on a capture too

Fired 2026-10-07T18:27:36Z, the same call with `--own-lines none` on d2 and d3, so every line is taken as the
sibling's by scope:

| drive | product path | exit |
|---|---|---|
| d2 | unremedied same · under the cue scope differs as CX · kept 1 of 3 lines | 1 |
| d3 | unremedied same · under the cue scope differs as CX · kept 2 of 5 lines | 1 |

Told that no line is own, the arm drops the whole block; the product's selection still keeps the lines that carry
the cited fingerprint, so the two part and the line says so. Two things follow. The check is not a tautology of its
inputs. And the lines the product keeps on these prompts do not rest on the scopes given by position: the
fingerprint arm alone keeps the same 1 and 2 lines.

In the probe's own pins the same function reads `same` twice on all 17 shapes under `nb`, and `differs` on three
built cases; four mutation checks (71 to 74 in `evidence/mutation-checks.md`) redden those pins.

## What the comparison takes as given

1. **Each line's scope.** A rendered line carries none. The own positions are the third reading's
   (`evidence/preregistration-third.md`, "The own lines, by position"), read from Conductor's records and the
   captures' own cue lines. As shown above, the kept lines on these three prompts do not depend on them.
2. **The window's fingerprints.** The product matches a corpus incident against every exception fingerprint seen
   twice or more in the digest's window, across all services (`crates/triage/src/baseline/sql.rs:103-116`,
   `crates/triage/src/digest/assembler.rs:282-287`). A prompt holds one of them, the triggering cue's, as its
   citable id (`pulse-app/src/inference_runtime.rs:128-138`); the dry run uses that one. If the window had also held
   the fingerprint of a sibling's line, the product would keep that line where the arm removed it.
   **What the captures and Conductor's records say about that, none of it a title or a fingerprint:**
   - Every sibling line in the three blocks is an incident of the canary: its retry storms of the first, second and
     third drive, and one error-rate spike, whose fingerprint is empty and matches nothing.
   - The canary's storm is detected every 90 s in each drive, the last time 5 s before the scenario's creating
     digest is assembled (inputs#I30 `:486-491`, `:513`; inputs#I31 `:496-501`, `:527`; inputs#I32 `:518-523`,
     `:555`). The window is 60 s (each prompt's own `WINDOW:` line).
   - The capture root holds the canary's nine retry-storm prompts of those detections. Each cites one fingerprint,
     and the nine are nine different ones: every burst carries its own.
   - Each canary incident carries the fingerprint of the burst that created it, the first of its drive, 185 s before
     the scenario's digest. The burst inside the window, 5 s before, cites another. In no drive does a sibling line
     carry the fingerprint the canary cited inside the window.
   So on these three prompts the window's set held no sibling line's fingerprint, as far as the storm cues show it.
   The set itself was never logged and is not in a capture: this is a reading from timing and from nine cited
   fingerprints, not a read of the set.

## What it does not show

- **A live digest.** The product's pipeline was not driven. The reading says that the selection, the line format
  and the render compose the bytes the `CX` arm composed, from the lines a capture holds. Conductor's sixth series
  is the first run of the remedy through the product.
- **A prompt where the window holds a sibling's fingerprint.** There the product keeps that sibling's line by rule
  (the fingerprint arm is unchanged), and the `CX` arm as it was read removed it. A neighbour whose storm repeats
  one fingerprint, with an incident already opened for it, is that case. It is not among the three captures, and no
  arm read such a block.
- **More than five matches.** The product narrows before the five-line cap, the arm edited a block already capped.
  They part only when an own match stands sixth or later by age. d3 had six candidates and the sixth was the
  canary's, so they agree there; no capture holds the other case.
- **The counts themselves.** 0 service misses of 120 was measured on the arm's bytes. This file shows those are the
  bytes the product's selection composes from the same lines; it measures no generation.
