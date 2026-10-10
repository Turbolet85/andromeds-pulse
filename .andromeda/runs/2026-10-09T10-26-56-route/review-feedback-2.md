# Review feedback 2 — Phase 4, andromeda-pulse-0.4.0 (third pass): five edits, then approved

_Source: the operator's answer `pc-overseer/relays/pulse-route-phase4-approval-third-2026-10-09.md`, read whole on
2026-10-09. The 39 requirements stand as `requirements.md` holds them; the route is approved with the five edits
below applied first. Numbered 2 because it is the run's second feedback round (round 1 is `second/review-feedback-1.md`,
on the second pass). Draft before this round: 76 entries (kept as `working-route-draft.merged.md`); after: 77._

## The five edits

1. **`Desktop distribution retired` — the release credentials. APPLIED.** The line now ends "release environment,
   secret names listed for the founder". The chunk names on its card the release environment and every secret name
   the two workflows read, for the founder to retire by hand — the treatment P-114 gives the two channel
   repositories; it deletes nothing outside the tree. For the 25-word cap the list before it was shortened:
   "release and updater-key runbooks" → "runbooks", "channel-manifest test" → "manifest test", "its bot watch and
   supply-chain gate" → "its watch, gate". This supersedes the merge record's "deferred" on the security rewrite.
2. **`Network OTLP receiver behind the token` — renewal. APPLIED.** "termination and key custody stated" →
   "termination, key custody, renewal stated". Supersedes the merge record's "deferred".
3. **The tests validator's deferred insert. APPLIED** in Epoch 6 directly after `Telemetry store on disk`, under
   P-091: `Disk store opened across builds — a telemetry store written by another build is recognised, carried forward
   or refused by a stated rule, never misread (P-091)`. Supersedes the merge record's "deferred".
4. **`Scenario legs driven against the console engine` — MOVED** to directly after `Door inside the engine's
   process`. Checked what stood between the two positions: `One place on a node`, `Corpus encryption at rest retired`
   and `Engine end-to-end gate reachable`. The legs need none of them — they are dev-host legs that boot the engine
   on a fresh data directory of their own, the dev host has a credential store, and they are not run by CI. Nothing
   kept it where it was.
5. **The obs validator's deferred insert. NO ENTRY.** The engine's graded log is the artifact a failed gate keeps.
   Noted against `Curated snapshot through the door`: that chunk's wrap corrects the obs plan's undelivered line (§9,
   a snapshot on test failure), and the entry keeps its "the snapshot code's one consumer" clause.

## Standing as applied (the operator's word)

"owner only, own host" on the door's first form, on P-118's authority · Foundation at thirteen entries · `Desktop
distribution retired` ahead of `Engine delivered to a node`.

## Scope guard

No request in this round was a capability change to the intent, specialist content or an implementation step.
