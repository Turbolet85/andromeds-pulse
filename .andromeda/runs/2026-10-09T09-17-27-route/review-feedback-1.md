# Review feedback 1 — andromeda-pulse-0.4.0 (Phase 4)

_Received 2026-10-09 through the operator (the pc overseer), relay
`pc-overseer/relays/pulse-route-phase4-answer-2026-10-09.md`, read whole. The founder was shown the eight epochs, the
21 requirements in summary, the list "Decisions for you and the founder", and four questions by dialog._

## Verdict: APPROVED AS IT STANDS

The question put to the founder, verbatim: «Утверждаете требования и маршрут Pulse 0.4.0?»

His answer, typed in his own words in place of an option: «Мы можем попробовать оставить как есть и в догонку 0 пендиг
врапом поднастроить что нам надо».

So the requirements P-083…P-103 and the 48-chunk route stand as printed at Phase 4. The draft was NOT edited after
this feedback: no Insert, Reorder, Rewrite or Remove. Phases 5 and 6 run as written. `intent.md` is not edited and
the route is not re-run from Phase A.

## Confirmed unchanged (the operator's relay, its §3)

- P-088 and P-093 stand as written; the approval of the route as it stands covers them.
- The classification of the 22 capabilities of 0.3.0 in `requirements.md` stands.
- Corpus encryption retired first, in Epoch 1: agreed.

## Scope guard — noted here and handed to the operator; NONE of it is built or written into the route by this run

Each item below is a capability decision or a control no finding states. Per the operator's directive for this run it
is neither sent to a re-run of the architecture skill nor to a specialist's skill: it is noted here and belongs to a
follow-up 0-pending adaptation wrap, whose relay the operator will send. It is not this run's work.

**The founder's rulings at the same dialog:**

- Three more things leave in 0.4.0 — his picks, all three: «Хост плагинов на WASM (Рекомендую убрать)», «Определение
  рабочей папки (Рекомендую убрать)», «Выгрузка для дообучения (Рекомендую убрать)»: the WASM plugin host, workspace
  detection, and the training export to `~/Downloads`. This decides critical paths P4 and P7 as named for retirement.
  No requirement of this run carries these three removals and no chunk of the written route names them.
- Raw telemetry is kept 7 days by default — his pick «7 дней (Рекомендую)»: configurable, with a ceiling on the
  file's size at which the oldest goes first; the size figure is set by measurement in the disk epoch. P-091 as
  written says "for days" and carries no number.

**The operator's own, for the same wrap:**

- Who terminates the encrypted channel and where its key lives (the deferred security Insert, channel key custody).
- The door is encrypted in transit, and its admission has a lifecycle — how a party is admitted and revoked (the
  deferred security Rewrite, and the first gap named at Phase 4).
- How the engine reaches a node: one Linux binary and a documented way to run it as a service; the Windows and macOS
  legs go, as the route already has it.
- The personal data of the real service: named by the founder together with the service, when P-101 is taken up (the
  deferred security Rewrite on `Real service watched for days`).

## State of the three deferred suggestions

`merge-decisions.md` records three security suggestions as deferred. They stay unapplied: the founder approved the
route as it stands and the operator took each for the follow-up wrap. Nothing in the route was removed at review, so
`[GATE_REACHABILITY]` has no removal to account for.
