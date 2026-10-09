# Adaptation record — the 2026-10-06T22-04-35Z 0-pending wrap

0 pending · 0 gated at Setup (`records 80 · complete 80`). Tree at Setup: HEAD `ce5857b`, 0 ahead of
`origin/chore/migrate-pulse-to-v3`, dirty only with expected-transient bookkeeping (`friction-log.ndjson`, the
handoff's `Session End Status` trailer) and the untracked `.andromeda/runs/2026-10-06T22-00-53Z-phase/`.

Directive: the overseer (founder-delegated), in this session's conversation, 2026-10-06 — snapshotted verbatim as
`relay-1.md` (§1 the answer at `/andromeda-phase`'s CI-red halt, §2 this wrap's directive).

## 1. Route adaptation (operator-requested)

The directive names the entry, its disposition and its placement, so the trajectory gate was satisfied by a
recorded pre-direction — no halt. Authority: `word: "mint ONE entry at the head of the tail, AHEAD of the L4
first-hypothesis entry" — delegate the overseer (founder-delegated), 2026-10-06`. The placement is a trajectory
fork the founder owns, so the answer is PROVISIONAL by rule: the founder's own later word supersedes it.

| Item | Disposition |
|---|---|
| Mint "The npm supply-chain gate is green again" | APPLIED — inserted at `working-route.md:176`, ahead of "The L4 first hypothesis names the triggering service" (now `:178`); title + scope hint + three `CONTEXT:` blocks (the direction · the CI measurements · the zero in-repo diff, the hypothesis and the not-measured half) |
| Re-pin next-entry `PREREQ` / `WATCH` onto the new head | NOTHING TO MOVE — the previous head carried four `CONTEXT:` blocks and no `PREREQ` or `WATCH` (`route.py pins`) |
| Everything else on the tail | UNCHANGED, per "Nothing else changes" |

Read-back: `route.py markerless` → `186 lines · 86 entries · 80 stamped · 6 markerless · 80 separators`; the new
entry reads `176 · … · freight 3` and `route.py pins` lists its three blocks (561 · 1125 · 708 chars); `git diff`
on the working route is 2 insertions, 0 deletions, no `[{marker}]`-frozen line touched; the file stays `i/lf w/lf`.

## 2. What the entry's CONTEXT rests on (measured this session, 2026-10-06 ~22:00Z)

- `ci.py conclusion` over the four shas from the last flip through HEAD (trail
  `.andromeda/runs/2026-10-06T22-00-53Z-phase/ci-no-marker.json`): `ce5857b` red · `373b576` red · `1124148` red ·
  `5f77859` green (ci#37340298405).
- The failed check on all three reds is the job `supply-chain (audit + deny + auditable)`: run 37536230886
  (job 112517747768), run 37536948756 (job 112520275174), run 37537455637 (job 112521977519).
- Failed step: step 14 `cargo xtask check:npm-supply-chain` — read from the step list of job 112521977519.
- The step's output — read from the log of job 112517747768 only: arm `findings-red`, 5 distinct advisories, 2
  excepted (`extract-zip` GHSA-7pqw-9j4j-h8q3, GHSA-jmr9-qjv8-65gv), 3 not: `seroval` GHSA-p6vx-979v-rg4c
  (critical), `seroval` GHSA-jp82-f5mq-hwhp (high), `source-map-js` GHSA-68fv-2mgg-jv7q (high).
- Locked versions (`pulse-app/ui/package-lock.json`): `seroval` 1.5.4, `seroval-plugins` 1.5.4, `source-map-js` 1.2.1.
- `git diff 5f77859 ce5857b` over the lockfile, `package.json`, `npm-policy.json`, `xtask/` and `.github/`: 0 lines.
- Limits, stated on the entry too: the other jobs of runs 37536948756 and 37537455637 had not completed when
  read; the advisories' publication dates were not read (their arrival between the runs is a hypothesis); whether
  a forward fix exists for either package was not measured.

## 3. Other duties of this path

- **Gated premise re-check:** no `gated` record stands — nothing to re-verify.
- **Curation (P3):** not run — the conversation carried no operator correction.
- **Amendments:** none — this wrap produced or measured no fact a master states.
- **Epoch growth:** Epoch 4 reads 68 entries after the mint (6 markerless); the no-split ruling stands.
- **Not applied, kept:** the overseer's phase directive for the L4 entry (`relay-1.md` §3) — it binds nothing
  until re-sent with that entry's `/andromeda-phase`.
