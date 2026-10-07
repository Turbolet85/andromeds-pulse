# The remedy in the product: `CX`, by the founder's choice (plan Step 15, done a second time)

- **What ships:** on a digest whose triggering cue carries a `scope_id`, the corpus block keeps the incidents scoped
  to that service and the ones the fingerprint arm keeps, and drops every other service's. A digest with no cue, or
  a cue with no `scope_id`, is selected and ordered as before.
- **Two facts, both true, neither replacing the other:**
  - The remedy reading's pre-registered order (`CR`, `CO`, `CC`, `CX`, least change first) selected `CO`
    (`evidence/reading-remedy.md`). That selection stands as measured and is not rewritten.
  - The founder chose `CX`, 2026-10-07 20:08 local, by dialog, relayed by the pc overseer (inputs#I29, section 1;
    the relay's sha256 `df69509d…5e13`). He chose on the third reading's counts (`evidence/reading-third.md`), which
    the order never saw: it read `CO` and `CX` on the second drive's prompt alone.
- **Written:** 2026-10-07T18:35:25Z, at the re-entry of /implement that replaced the `CO` product change. No
  generation ran in it and no GPU was used. No text of a captured prompt and no model text is in this file.

## The counts the choice rests on (recounted in this run from the third reading's three `runs.json`)

n = 40 in every cell. A service miss is a first hypothesis that does not name `conductor`; `named` is a first
hypothesis that names `conductor-canary` at all.

| prompt | `shipped` misses | `CO` misses | `CX` misses | `CX` named |
|---|---|---|---|---|
| d1 | 4 | 4 (it composes as `shipped` there: one line, the sibling's) | 0 | 1 |
| d2 | not read in that reading | 1 | 0 | 1 |
| d3 | 4 | 0 | 0 | 13 |
| the three | | 5 of 120 | 0 of 120 | 15 of 120 |

The files hash as the reading recorded them (`909b2873…`, `cdcae9e4…`, `758e8519…`).

## What is in the tree

- **Product:** `select_corpus_matches` (`crates/triage/src/digest/retrieval.rs`) narrows its scope arm to the
  triggering scope, before the five-line cap, and only ever removes; the fingerprint arm is as it was. `assemble`
  passes the cue's `scope_id`, as it did for `CO`. Two files under `crates/` differ from the chunk base, no other.
- **The `CO` change is gone:** its ordering, its five in-crate pins and the probe pins moved for it.
- **Probe:** the baseline arm `nb` (the block before the remedy: every active scope's lines, newest first); the four
  candidates are edits of `nb`, so each is the arm the readings measured; `CX` composes as `shipped` on all 17
  shapes, pinned; `--product-path` (`evidence/product-path-equality.md`).
- **`nb` is here although `S17` read CLEAN.** The plan adds it only when `S17` reproduces. The relay names it
  (inputs#I29, section 2), and with `CX` in the product it is needed for another reason: `shipped` no longer
  composes the blocks of `S4`, `S12`, `S14`, `S15`, `S16` and `S17`, and every pin of what the readings measured
  reads them under `nb`.
- **Pins and checks:** `evidence/mutation-checks.md`, the last section (five in-crate pins, three new probe pins and
  the flag's cases in a fourth, sixteen moved, twelve mutations, each red). Gate block of this run: 19 green, 0 red, 12 operator legs not run
  (triage 497 passed; the probe binary 157; the workspace 2799).
- **Companion sweep** (`grep -rln corpus_matches crates pulse-app/tests`): 21 files hit, 6 in the digest module, 15
  outside it · 0 changed · 15 no change. Of the 15, two reach the selection: `crates/triage/src/contract.rs` (the
  re-export) and `pulse-app/tests/integration_corpus_retrieval_two_session.rs`, whose four `assemble` calls pass no
  cue and so read the unchanged path.

## Two choices of the builder, inside what the relay left open

- **The narrowing runs before the cap.** The `CX` arm edited a block already capped at five. The product drops the
  other scopes' matches first and caps what is left, so an own match is not lost to lines that are then dropped.
  The two part only when an own match stands sixth or later by age; no shape and no capture holds that case (d3 had
  six candidates, the sixth the sibling's).
- **Only an active scope counts.** A cue whose service is not in the digest's services table adds no match: under a
  triggering scope the selection is a subset of what it was without one.

## What `CX` does not cover

- **On d3 a third of first hypotheses still name the sibling beside the right service: 13 of 40** (14 of 40 under
  `shipped`, 17 under `CO`). All 13 also name `conductor`, so the service bar reads them as `both`. The count reads
  no placement: a sentence that names the sibling to rule it out is counted with one that places the storm there.
- **One own line on d3 names the sibling.** The triggering service's own storm of the second drive carries a title,
  authored by the model when that incident was created, that names the sibling and not the triggering service apart
  from it. `CX` keeps that line by rule. No arm removed it, so the reading does not show that the 13 come from it.
- **Another service's line stays when its fingerprint is live in the window.** The fingerprint arm is unchanged by
  rule (scope.md, Boundaries). A neighbour whose storm repeats one fingerprint, with an incident already opened for
  it, keeps that incident's line in a digest cued for another service. In the captures every canary burst carried
  its own fingerprint, so no sibling line was held that way (`evidence/product-path-equality.md`); no arm read a
  block of that kind.
- **The probe keeps no durable known-positive.** `S17` read CLEAN under the unremedied block (1 miss of 20). The
  known-positive the readings found is a captured prompt, deleted when the chunk closes. After that nothing in the
  tree fails without the remedy and passes with it under a real model; the pins hold what the product composes, not
  what the model then says.
- **The numbers are three prompts of one capture run.** 0 of 40 on a prompt shows that 40 generations did not miss
  on it; `shipped` on d3 read 0 of 20, then 2 of 20, then 4 of 40 across the three readings. No prompt with other
  kinds in its block, or with several own lines ahead of many others, was read.
- **The counts were measured on the arm.** `CX` was read as an edit of each captured prompt's bytes. The dry run
  shows the product's selection composes those bytes from the same lines; the product's pipeline was not driven,
  and no generation ran on the tree that ships. Conductor's sixth series is the first reading of it live.
- **A digest without a cue scope.** The tier-3 and other cue-less digests keep every active scope's lines, as
  before; nothing here measures them.
