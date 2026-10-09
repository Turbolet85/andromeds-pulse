# The capture entries of Step 11 — the precondition and the section read

Both entries read the captures in place and spawn no model. Each was driven once by hand, at 2026-10-07T13:11:35Z,
on the binary built from the tree whose probe file hashes `0c0aa2d1…93cd` (the tree mutation checks 45-51 restored).
Nothing below is text of a captured prompt: the entries print hashes, byte and line counts and closed labels.

The plan's two entries name two prompts. The operator's decision is three (inputs#I18, inputs#I19), so each was fired
with the third added: one more `sha256sum`, and `,d2=$L4_REPLAY_D2` in the source list. `L4_REPLAY_D2` is a shell
name of these legs alone, like the two the plan names. Every other token is the plan's.

Which directory is which, and how each was identified: `evidence/preregistration-replay.md`. The label `miss` is a
role, the position that missed in the fourth and fifth series (d3). No captured drive missed.

## Entry 22 — the capture precondition

- **run** (the plan's text with the third prompt added):
  `sha256sum < "$L4_REPLAY_MISS/prompt.txt" && sha256sum < "$L4_REPLAY_CONTROL/prompt.txt" && sha256sum < "$L4_REPLAY_D2/prompt.txt" && ./target/debug/examples/l4_decision_probe --arms shipped --replay "miss=$L4_REPLAY_MISS,control=$L4_REPLAY_CONTROL,d2=$L4_REPLAY_D2" --replay-scope conductor --dry-run`
- **exit:** 0 · **atoms:** `exit 0` held · `contains dry-run: arm shipped replay:miss` held ·
  `contains dry-run: arm shipped replay:control` held · `lacks INCONCLUSIVE` held (0 hits) → **green**

```
83b8a3ea7ccbcfbda4dfbc76f62ded32f7e6362d711d8ba9ad34bbb2ac63661e  -
1d953dfbf29bfca03a3c6d4ef30188630a7f2469faab76e21d554783666e22ec  -
21c39d8cdf8e55191fb292168b8fb352a1ec61821cd2e882bbcc86ffa42409c8  -
dry-run: arm shipped replay:miss: prompt 8059 bytes (max 16384) · argv same · corpus lines 5
dry-run: arm shipped replay:control: prompt 7616 bytes (max 16384) · argv same · corpus lines 1
dry-run: arm shipped replay:d2: prompt 7854 bytes (max 16384) · argv same · corpus lines 3
```

What the green shows, per prompt: the directory loads, the prompt passes the production bound and carries a
`<DIGEST>` section, its first cue line names a known cue kind with `scope_id` `conductor`, and the recorded argv
equals the probe's own apart from the model and grammar paths. The rendered corpus blocks hold 5, 1 and 3 lines
against 6, 1 and 3 corpus candidates logged before selection (inputs#I20 `:274-276`): the third drive's block is at
the five-line cap.

## Entry 23 — the capture section read

- **run** (the plan's text with the third prompt added):
  `./target/debug/examples/l4_decision_probe --sections "miss=$L4_REPLAY_MISS,control=$L4_REPLAY_CONTROL,d2=$L4_REPLAY_D2,S14,S16"`
- **exit:** 0 · **atoms:** `exit 0` held · `contains sections: miss vs control: differs in` held ·
  `contains sections: miss vs S14: differs in` held · `contains sections: miss vs S16: differs in` held ·
  `lacks INCONCLUSIVE` held (0 hits) → **green**

```
sections: miss vs control: role same · miss 406 B 3 L · control 406 B 3 L
sections: miss vs control: conventions same · miss 478 B 3 L · control 478 B 3 L
sections: miss vs control: output-schema same · miss 4726 B 147 L · control 4726 B 147 L
sections: miss vs control: project-context same · miss 99 B 5 L · control 99 B 5 L
sections: miss vs control: digest.window same · miss 27 B 1 L · control 27 B 1 L
sections: miss vs control: digest.project same · miss 40 B 1 L · control 40 B 1 L
sections: miss vs control: digest.recent-changes same · miss 0 B 0 L · control 0 B 0 L
sections: miss vs control: digest.overall differs · miss 52 B 1 L · control 52 B 1 L
sections: miss vs control: digest.trigger same · miss 21 B 1 L · control 21 B 1 L
sections: miss vs control: digest.services differs · miss 124 B 3 L · control 124 B 3 L
sections: miss vs control: digest.attention-cues same · miss 78 B 2 L · control 78 B 2 L
sections: miss vs control: digest.corpus-matches differs · miss 658 B 7 L · control 215 B 3 L
sections: miss vs control: citable-evidence-ids same · miss 104 B 5 L · control 104 B 5 L
sections: miss vs control: corpus-retrieval same · miss 0 B 0 L · control 0 B 0 L
sections: miss vs control: output-instructions same · miss 1208 B 4 L · control 1208 B 4 L
sections: miss vs control: other same · miss 0 B 0 L · control 0 B 0 L
sections: miss vs control: differs in digest.overall,digest.services,digest.corpus-matches
sections: miss vs d2: role same · miss 406 B 3 L · d2 406 B 3 L
sections: miss vs d2: conventions same · miss 478 B 3 L · d2 478 B 3 L
sections: miss vs d2: output-schema same · miss 4726 B 147 L · d2 4726 B 147 L
sections: miss vs d2: project-context same · miss 99 B 5 L · d2 99 B 5 L
sections: miss vs d2: digest.window same · miss 27 B 1 L · d2 27 B 1 L
sections: miss vs d2: digest.project same · miss 40 B 1 L · d2 40 B 1 L
sections: miss vs d2: digest.recent-changes same · miss 0 B 0 L · d2 0 B 0 L
sections: miss vs d2: digest.overall differs · miss 52 B 1 L · d2 52 B 1 L
sections: miss vs d2: digest.trigger same · miss 21 B 1 L · d2 21 B 1 L
sections: miss vs d2: digest.services same · miss 124 B 3 L · d2 124 B 3 L
sections: miss vs d2: digest.attention-cues same · miss 78 B 2 L · d2 78 B 2 L
sections: miss vs d2: digest.corpus-matches differs · miss 658 B 7 L · d2 453 B 5 L
sections: miss vs d2: citable-evidence-ids same · miss 104 B 5 L · d2 104 B 5 L
sections: miss vs d2: corpus-retrieval same · miss 0 B 0 L · d2 0 B 0 L
sections: miss vs d2: output-instructions same · miss 1208 B 4 L · d2 1208 B 4 L
sections: miss vs d2: other same · miss 0 B 0 L · d2 0 B 0 L
sections: miss vs d2: differs in digest.overall,digest.corpus-matches
sections: miss vs S14: role same · miss 406 B 3 L · S14 406 B 3 L
sections: miss vs S14: conventions same · miss 478 B 3 L · S14 478 B 3 L
sections: miss vs S14: output-schema same · miss 4726 B 147 L · S14 4726 B 147 L
sections: miss vs S14: project-context differs · miss 99 B 5 L · S14 71 B 5 L
sections: miss vs S14: digest.window same · miss 27 B 1 L · S14 27 B 1 L
sections: miss vs S14: digest.project differs · miss 40 B 1 L · S14 29 B 1 L
sections: miss vs S14: digest.recent-changes same · miss 0 B 0 L · S14 0 B 0 L
sections: miss vs S14: digest.overall differs · miss 52 B 1 L · S14 52 B 1 L
sections: miss vs S14: digest.trigger same · miss 21 B 1 L · S14 21 B 1 L
sections: miss vs S14: digest.services differs · miss 124 B 3 L · S14 129 B 3 L
sections: miss vs S14: digest.attention-cues same · miss 78 B 2 L · S14 78 B 2 L
sections: miss vs S14: digest.corpus-matches differs · miss 658 B 7 L · S14 700 B 7 L
sections: miss vs S14: citable-evidence-ids differs · miss 104 B 5 L · S14 104 B 5 L
sections: miss vs S14: corpus-retrieval same · miss 0 B 0 L · S14 0 B 0 L
sections: miss vs S14: output-instructions same · miss 1208 B 4 L · S14 1208 B 4 L
sections: miss vs S14: other same · miss 0 B 0 L · S14 0 B 0 L
sections: miss vs S14: differs in project-context,digest.project,digest.overall,digest.services,digest.corpus-matches,citable-evidence-ids
sections: miss vs S16: role same · miss 406 B 3 L · S16 406 B 3 L
sections: miss vs S16: conventions same · miss 478 B 3 L · S16 478 B 3 L
sections: miss vs S16: output-schema same · miss 4726 B 147 L · S16 4726 B 147 L
sections: miss vs S16: project-context differs · miss 99 B 5 L · S16 71 B 5 L
sections: miss vs S16: digest.window same · miss 27 B 1 L · S16 27 B 1 L
sections: miss vs S16: digest.project differs · miss 40 B 1 L · S16 29 B 1 L
sections: miss vs S16: digest.recent-changes same · miss 0 B 0 L · S16 0 B 0 L
sections: miss vs S16: digest.overall differs · miss 52 B 1 L · S16 52 B 1 L
sections: miss vs S16: digest.trigger same · miss 21 B 1 L · S16 21 B 1 L
sections: miss vs S16: digest.services differs · miss 124 B 3 L · S16 129 B 3 L
sections: miss vs S16: digest.attention-cues same · miss 78 B 2 L · S16 78 B 2 L
sections: miss vs S16: digest.corpus-matches differs · miss 658 B 7 L · S16 679 B 7 L
sections: miss vs S16: citable-evidence-ids differs · miss 104 B 5 L · S16 104 B 5 L
sections: miss vs S16: corpus-retrieval same · miss 0 B 0 L · S16 0 B 0 L
sections: miss vs S16: output-instructions same · miss 1208 B 4 L · S16 1208 B 4 L
sections: miss vs S16: other same · miss 0 B 0 L · S16 0 B 0 L
sections: miss vs S16: differs in project-context,digest.project,digest.overall,digest.services,digest.corpus-matches,citable-evidence-ids
```

## What the rows show, as counts and labels

- **Among the three captured prompts** the prompt template, the project context, the cue lines, the `TRIGGER:` line
  and the citable ids are byte-identical. `miss` differs from `d2` in two sections, `digest.overall` and
  `digest.corpus-matches`, and from `control` in those two and `digest.services`.
- **The corpus block** is the one section that differs in size: 7, 5 and 3 lines for `miss`, `d2` and `control`
  (a header, the framing note and 5, 3 and 1 match lines), 658, 453 and 215 bytes.
- `digest.overall` and `digest.services` differ at equal byte and line counts, so a value on a line differs, not
  the line set.
- **Against the synthetic shapes** `S14` and `S16`, `miss` also differs in `project-context`, `digest.project` and
  `citable-evidence-ids`, and its services table differs in size as well (124 against 129 bytes, 3 lines each). The
  shapes hold the probe's fixed project context and synthetic ids, so those differences are expected by
  construction; what each carries is read at Step 13, only after a reproduction.
- No source holds a `RECENT CHANGES` section or a rendered `# Corpus Retrieval` section, and nothing fell under
  `other`: every byte of each prompt sits under a known header.

What the rows do not show: what any differing line says. No text of a section is printed, and no captured prompt
was read by the builder for this file.
