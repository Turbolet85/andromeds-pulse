# Curation — 2026-10-10-door-inside-the-engine-s-process

Scope: the second window of this wrap and the report's "Decisions & corrections" section. The first window's
conversation (the implement session, the operator pass and the report's authoring) was cleared before curation ran,
on the operator's directive; a correction only that conversation held and the report does not carry is not curated.

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + testing.md: "The test that proves a claimed capability is written to the ledger's acceptance, not to the plan step" (confidence 0.8)
                                              + testing.md: "A lock guard taken in an `if let` scrutinee lives through the then-block" (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 2 dup · 0 task-specific · 0 conflict · 0 deferred by the cap · 8 at or below the threshold
  No-other-home: "The test that proves a claimed capability is written to the ledger's acceptance, not to the plan step"
  No-other-home: "A lock guard taken in an `if let` scrutinee lives through the then-block"
```

## Applied

1. **testing.md — the ledger's acceptance before the proving test.** Signals: verified by measurement (+0.4), a
   specific technical detail with context (+0.2), reached no other durable home (+0.2) = 0.8.
   Proof: the report's "Mistakes of this session": the other-account arm was written to plan step 9, which says the
   owner's call "is answered", while P-126's acceptance in the ledger asks for an answer "with content the test sent
   over OTLP"; seen when the ledger was read after the first green block; it changed the arm's owner call from a
   `ping` to a `query_logs` (report, Deviation 3) and cost a second firing of the whole gate block.
2. **testing.md — a lock guard in an `if let` scrutinee.** Signals: proven by a real gate failure (+0.4), a specific
   technical detail (+0.2), reached no other durable home (+0.2) = 0.8.
   Proof: the report's "Mistakes of this session": the stand-in of `pulse-app/tests/unit_door.rs` took a value out of
   a mutex inside an `if let` scrutinee and waited in the then-block; the next call blocked on the lock; one red run,
   18 of 19, fixed by taking the value in a statement of its own.

## Corrected in place (outside the cap)

- `rules/security.md` Session Additions 2026-05-12, one clause: "currently mcp-server is the only optional crate" now
  reads "currently `mcp.*` is the only feature-gated TauRPC namespace", with a `[corrected 2026-10-11: …]` tag that
  quotes the old clause. On the operator's word at the route-resolve card (inputs#I8).
  Proof: the report's Dependencies bullet — `pulse-app/Cargo.toml`'s dependency `mcp-server-crate` is no longer
  `optional` and the feature reads `mcp-server = ["mcp-server-crate/mcp-server"]`; `pulse-app/src/main.rs` is not
  among the chunk's 16 changed files, so its cfg-gate points stand.

## Filtered

- dup — "a passing test's printed lines are readable by a step only under `--success-output immediate`": carried by
  `rules/testing.md` Session Additions 2026-10-04 (the clean-skip entry, which names the flag and the assertion). The
  chunk followed it; not a recurrence.
- dup — "reading a hosted job's log through `gh api --allow-escape-sequences`, stripping the escapes, a known-present
  line as the control": carried by `rules/testing.md` Session Additions 2026-10-10.
- at 0.6, rejected — "a Unix socket path must be shorter than 108 bytes; the session scratchpad's path is longer":
  this wrap amended the bound into architecture and test-plan, so neither conditional signal applies.
- at 0.6, rejected — "root is not held by a directory's or a socket's mode": amended into security-plan
  (§Authentication, the fourth ban) and re-derived into `rules/security.md`.
- at 0.6, rejected — "tell arms apart in a process-wide capture by the runtime's thread name": the test-plan surface
  row this wrap wrote carries it.
- at 0.6, rejected — "a plan's claim that serde structs refuse an unknown field is answered by a grep for
  `deny_unknown_fields`": amended into security-plan and architecture as measured, and carried on the route.
- below the threshold — "a blocking stand-in must be releasable, the runtime's drop waits for `spawn_blocking`
  tasks" (0.2) · "a caller sends a line and its terminator in one write and treats a failed write as closed" (0.2) ·
  "`git grep` over tracked files where a recursive grep walks `target/`" (a one-off, below 0.6) · "the PROVISIONAL
  sentence in full once per master, a pointer at the restating sites" (the operator's answer for this wrap's halt,
  not given as a standing convention).

## To the handoff

- No new `recurrence-despite-learning`. A command opening with `cd` out of the project root was refused by the Bash
  guard and re-issued once, in the first window (report) and again in this one: the guard held the rule each time,
  so neither is a recurrence; the entry the handoff already carries from prior wraps stands as it is.
- A deferred learning on the operator's word (inputs#I7): a new variable that is the switch of a write crossing a
  newly widened boundary matched `New env var registration` (routine) and `Boundary widening` (escalate); it went to
  the halt and no discriminator rule was minted, so the next such variable is judged again.
