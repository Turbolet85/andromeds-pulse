# Cascade dispositions — the 2026-10-07T09-50-55Z 0-pending wrap

One amendment in this pass: `security-plan.md` §Dependency Security → CI integration, the `Critical CVE response SLA`
clause (line 226), scoped by the founder's ruling of 2026-10-07 to bind from the first tagged release.

## The search

`cascade.py sweep` (`cascade v1.1 · b56ded6b`), patterns in `cascade-patterns.toml`, baseline `f70be92c`, over the
seven masters, every `.andromeda/registries/**` file, the three curation homes, the two judgment bases and the leaf
bodies. Each control fired on the pre-pass text.

| id | pattern | what it looks for |
|---|---|---|
| `sla-name` | fixed `Critical CVE response SLA` | the clause by its name |
| `sla-72h` | regex `72 ?h(ours?)?\b`, case-insensitive | the window by its figure, any spelling |
| `sla-patch-rel` | fixed `tagged patch release`, case-insensitive | the clause's end condition |
| `sla-window` | regex `within the same window\|advisory disclosure`, case-insensitive | the clause's own verbs and phrasings |
| `sla-ghsa` | fixed `GHSA-p6vx-979v-rg4c` | the one advisory the amended clause now names |

Not looked for: a paraphrase of the SLA that uses none of these words (for example "three days"), and any mention
outside the swept populations (chunk reports, the handoff, the friction log, the route).

## Rows and dispositions

| row | disposition |
|---|---|
| `security-plan.md:226` `sla-name` standing, edited | amended — this is the clause; its name stands by design |
| `security-plan.md:226` `sla-72h` standing, edited, ×2 | amended — the first match is the standing SLA figure, kept; the second is the new sentence that scopes it ("the 72h window starts to bind with the first tagged release"). No retired wording: the ruling scopes the clause, it does not replace it |
| `security-plan.md:226` `sla-patch-rel` standing, edited | amended — kept, as above |
| `security-plan.md:226` `sla-window` standing, edited, ×2 | amended — both matches are the standing clause's own phrasing, kept |
| `security-plan.md:222` `sla-ghsa` standing | no change — the npm channel paragraph's record of the advisory and its in-range lockfile close; a true claim sharing the token, and it agrees with the amended clause (fix on the build branch) |
| `security-plan.md:226` `sla-ghsa` new | this pass's own text |

Leaf rows: 0. Curation rows: 0. Judgment-base rows: 0. Registry rows: 0.

## Leaves

No leaf carries the SLA clause in any swept spelling: `.claude/docs/security-summary.md`, `.claude/rules/security.md`
and CLAUDE.md's warnings block read 0 rows on all five patterns, before and after the amendment. The clause was never
distilled, so no leaf is stale and none is rewritten. Whether the summary SHOULD carry the SLA is setup's derivation
and is not decided here.

## Other masters

No other master cites the SLA: 0 rows outside `security-plan.md`. The tests↔obs and a11y↔obs binds are untouched.
