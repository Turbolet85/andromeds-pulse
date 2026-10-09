# Cascade dispositions — the 2026-10-09T08-07-22Z 0-pending wrap

Five amendments in this pass, each recording a fact this wrap itself produced or measured (the 0-pending door; no
report, no fan-out):

1. `design-system.md` §Brand Identity, the Signature element paragraph (line 14) — the sketch folder
   `andromeda-pulse-0.4.0-incubator/signature-orb/` is gone (measured at this wrap: the folder holds four files and no
   `signature-orb/`).
2. `layout-templates.md`, the Halo canvas status note (line 100) — the same fact.
3. `architecture.md`, four sites (lines 30, 72, 232, 296) — the CPU-route retirement no longer has a route entry: this
   wrap removed "Without a GPU, L4 analysis is programmatic" from the route and carried it as a residual.
4. `test-plan.md` §1 `l4-latency-p99-ps1-run-coverage` (line 145) and the key file
   `registries/contracts/test-plan/per-chunk-gate-discipline.md` (line 32) — the `.ps1` grader run is dropped, and the
   native pre-push port left the route for the residuals.
5. `obs-plan.md` §8 (line 464) and §10's L4 row (line 548) — the owner entry "L4 generation records render unredacted"
   was retired unbuilt; the `.ps1` run is dropped.

## The search

`cascade.py sweep` (`cascade v1.1 · 01b568a2`), patterns in `cascade-patterns.toml`, baseline `18a872d0`, over the
seven masters, every `.andromeda/registries/**` file, the three curation homes, the two judgment bases and the leaf
bodies. Each control fired on the pre-pass text.

| id | pattern | what it looks for |
|---|---|---|
| `sketch-folder` | fixed `signature-orb` | the removed folder by name |
| `sketch-gathers` | regex `design direction gather`, case-insensitive | the retired claim's verb, any tense |
| `progl4-entry` | fixed `programmatic-L4 route entry` | the retired owner by the name the masters used |
| `still-until` | regex `still (routed\|consumed) until`, case-insensitive | the "until an entry lands" mechanism, whatever it names |
| `progl4-title` | fixed `analysis is programmatic`, case-insensitive | the retired entry by its title |
| `ps1-run-owed` | regex `run (itself )?(is )?owed`, case-insensitive | the owed-run claim |
| `owned-by-carry` | fixed `owned by the CARRY` | the retired ownership clause |
| `owner-pinned` | fixed `owner is pinned on the working route` | the retired owner clause of obs-plan §8 |
| `unredacted-title` | fixed `records render unredacted`, case-insensitive | the retired entry by its title |
| `own-route-entry` | fixed `is its own route entry` | the native-port ownership clause |
| `prepush-title` | fixed `runs natively on Linux`, case-insensitive | the carried entry by its title |
| `ps1-half` | regex `ps1.{0,3} half has (not\|never) run`, case-insensitive | the unrun-half statement beside the owed run |

Two retired entry titles have no pre-pass master hit, so the tool would refuse them as patterns (no control) and they
were searched by hand instead: `boot smoke is deterministic` and `surface quietly`, case-insensitive, over the seven
masters, `.andromeda/registries/`, CLAUDE.md, `.claude/rules/`, `.claude/docs/`, `playbook.md`, `drift-base.md`,
`requirements.md` and the verification matrix — 0 hits in each.

Not looked for: a paraphrase that names none of these (for example "the CPU path awaits its replacement"); any
mention outside the swept populations (chunk artifacts, sidecars and their archives, run dirs, the route archive,
the master route's frozen descs, the friction log).

## Rows and dispositions

| row | disposition |
|---|---|
| `design-system.md:14` `sketch-folder` standing, edited | amended — the folder's name stands in the sentence that now says it was removed |
| `layout-templates.md:100` `sketch-folder` standing, edited | amended — the same |
| `.claude/docs/design-summary.md:15` `sketch-folder` + `sketch-gathers` leaf | re-derived — the sentence now reads as design-system's does |
| `.claude/docs/services/interpretation.md:27` `progl4-entry` + `still-until` leaf | re-derived from architecture's amended sentence |
| `.claude/docs/stack.md:27` `progl4-entry` + `still-until` leaf | re-derived, the same |
| `architecture.md:72` `progl4-title` standing, edited, ×2 | no change to either match — the first is the founder's 2026-10-05 ruling as recorded, which stands; the second is this pass's sentence naming the removed entry by its title |
| `.claude/rules/observability.md:99` `ps1-run-owed` + `ps1-half` leaf | re-derived from obs-plan §10's amended row (the rule file's body section `SLO invariants`, not its Session Additions) |
| `.claude/docs/obs-summary.md:82` `ps1-run-owed` + `ps1-half` leaf | re-derived, the same |
| `.claude/rules/observability.md:75` `owner-pinned` leaf | re-derived from obs-plan §8's amended sentence (body section `PII scrubbing`) |
| `obs-plan.md:464` `unredacted-title` new | this pass's own text |
| `.claude/rules/verification-harness.md:95` `own-route-entry` leaf | re-derived from the amended key file (body section `Scenario legs`) |
| `.claude/docs/tests-summary.md:89` `own-route-entry` leaf | re-derived, the same |
| `registries/contracts/test-plan/per-chunk-gate-discipline.md:32` `prepush-title` standing, edited | amended — the title stands in the sentence that now says the entry was removed and carried |
| `test-plan.md:145` `ps1-half` standing, edited | amended — "has never run" stays true; the clause after it now says the run was dropped |
| `obs-plan.md:548` `ps1-half` standing, edited | amended — the same |
| `owned-by-carry` 0 rows | the clause stood only at `test-plan.md:145` and was replaced there; a statement about this pattern |

Curation rows: 0. Judgment-base rows: 0.

## Leaves

Eight leaf passages in seven files were rewritten, each from the amended master sentence it distils:
`.claude/docs/design-summary.md:15`, `.claude/docs/services/interpretation.md:27`, `.claude/docs/stack.md:27`,
`.claude/docs/obs-summary.md:82`, `.claude/docs/tests-summary.md:89`, `.claude/rules/observability.md:75` and `:99`,
`.claude/rules/verification-harness.md:95`. CLAUDE.md's generated blocks read 0 rows on every pattern and are not
changed. No preserve-verbatim home was touched.

## Other masters, the binds

`security-plan.md` and `a11y-plan.md` read 0 rows. The tests↔obs bind on the `.ps1` grader is kept consistent: both
sides now say the run was dropped at the 2026-10-09 version close. `registry.py check --project . --master
.andromeda/test-plan.md` prints 0 defects after the key-file edit.

## Seen and left

`architecture.md:73` ([Fault Identity]) still says "Conductor's sixth series is the first live reading" in the future
sense. That series has since run (relayed, not read here); the sentence is not this pass's subject and no amendment
was made on a relayed reading.
