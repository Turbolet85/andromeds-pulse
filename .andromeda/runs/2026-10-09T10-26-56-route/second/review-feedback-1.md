# Review feedback 1 — Phase 4, andromeda-pulse-0.4.0 (second derivation)

_Source: the operator's feedback `pc-overseer/relays/pulse-route-phase4-feedback-second-2026-10-09.md`, read whole on
2026-10-09. It is feedback, not the approval: the founder's word on the requirements comes separately. Draft before
this round: 69 entries; after: 71 (one entry added, one entry split). No Phase 5 was run._

## Applied to the draft

**1. A new entry at the head of Epoch 4 (the security validator's first deferred insert, now applied).**
`Security posture restated for a networked engine — tier, auth model, attack surface re-read against a token-gated
network receiver and stores on disk (P-116, P-087, P-088)`. No new capability: P-116 already requires that no record
read first describes the product this version stops being, and the tier is such a record. Re-read here: the tier's
justification at `.andromeda/security-plan.md:22-24` (no persistent user data store · no internet-exposed network
surface · no user authentication surface) and its "single source of truth" sentence at `:109-111`. The chunk's own
wrap amends the master; no specialist re-run. Whatever tier the plan's own criteria give, the chunk states it and the
entries after it are built to it — that sentence did not fit the 25-word line and stands here. `merge-decisions.md`
keeps the insert as "deferred" (Phase 3's record); this file is where it became applied.

**2. What reads a finding before the door exists, named on three checks.**
The draft named no reader. Re-derived at HEAD `7f99c38`: before `Door inside the engine's process` (Epoch 5) the only
reader of an incident's content outside the engine's process is the stdio sidecar's incident tools (test-plan §1
critical path P3; `xtask/src/external_resolve.rs` already drives a real `andromeda-pulse-mcp` subprocess). Its four
telemetry tools read an empty database until the door (`.andromeda/residuals.md:31`), so "finding" on these lines
means an incident or its report, not telemetry. The engine's own log shows that an incident formed, not what it says.
- `Engine end-to-end gate reachable`: "finding read back" → "finding read through stdio sidecar". For the word cap
  the line lost "service" and three articles; it still carries the memory cap, the loopback sender, the kept log, the
  second host reaching a stub and the recorded green.
- `Theme 0 checked`: the reader did not fit one 25-word line beside the engine-gate half, so the entry is SPLIT:
  `Detection parity after the removals` (the engine gate's half, P-086, with "critical paths restated") and `Theme 0
  checked by the external harness` (read through the stdio sidecar on the engine's host, P-102). This undoes a merge
  the Phase 1 draft made of the two; every theme now has a check entry of the same shape.
- `Theme 1 checked by the external harness`: "findings out" → "findings read through the stdio sidecar on the
  engine's host"; "telemetry from another host in" became "another host sends" and "path pinned in the engine gate"
  became "path pinned" for the word cap.

**Does a chunk break that read path before Epoch 5?** None was found; no stop. Five entries pass through the sidecar
and each carries its working state as an obligation (coordinates read at `7f99c38`):
- `Corpus encryption at rest retired` (P-085) — the sidecar opens the corpus with the credential-store backend
  (`crates/mcp-server/src/bin/andromeda-pulse-mcp.rs:66-67`).
- `One place on a node` (P-118) — the sidecar carries one of the data-directory resolver copies.
- `Incident is the engine's own record` (P-112) — the sidecar decodes the stored incident
  (`crates/mcp-server/src/tools.rs:336`, `:413`) and parses the model's answer out of it (`:380-383`).
- `Local model retired` (P-084) — the sidecar imports the old renderer and schema from the interpretation crate
  (`tools.rs:25-28`); the plain report of the entry before it is what lets that crate leave.
- `Workspace detection retired` (P-105) — the sidecar reads the published key file
  (`andromeda-pulse-mcp.rs:78`).
The operator's list named the last three; the first two are from this probe. Not a break, stated for the external
harness: a stdio sidecar on the engine's host is driven from that host, so at the Theme 1 check a harness on another
host needs a process there.

## Applied outside the draft (the feedback's §4, on the operator's word)

- `andromeda-pulse-0.4.0/requirements.md`, carried residual "Stored metrics feed no check": added that in 0.4.0 the
  stored metrics' consumer is the door's metric query (P-092, P-095) and that no finding expects a check over metrics;
  it stays visible.

## Noted, not applied — nothing built for these

- **Security validator's second deferred insert** (the named service's personal data bounded): a control no
  requirement states. It arrives, if at all, as a capability line through a 0-pending adaptation wrap after this route
  is written.
- **Five requirements the intent seems to be missing** — owner-only stores · the engine's own log bounded · where the
  door's credential lives on the developer's machine · the integrity of the delivered binary · records stamped in the
  future: same path, nothing added now. "owner-only" stays in `One place on a node` as written.
- **Tests validator's deferred insert** (coverage gate re-based): no entry. The operator's measurement, re-read here:
  `.github/workflows/ci.yml:544-626` is the `coverage` job, named "coverage gate (line ≥75% / branch ≥70% / function
  ≥85%)", with its own "Enforce coverage thresholds" step — so a removal that takes the workspace under a bar reads
  red at that chunk's own CI read. The `xtask` exclusion keeps its recorded revisit point (`.andromeda/test-plan.md:194`,
  the next epoch-boundary code audit).

## The operator's answers, recorded

- **Order:** `Desktop distribution retired` before `Engine delivered to a node` stands as drafted — nothing is
  delivered from this branch between the two, and the engine's Linux build in CI precedes the removal.
- **Theme 0 holds sixteen findings.** "Fifteen" in the directive was the operator's miscount. The sentence in
  `requirements.md` "Notes for the reader" that records the disagreement stands as written.
- **P-120's witness** stays a push-event run; how this branch gets one is that chunk's to read from the workflow.
- **`experiments/`** is a local, git-ignored directory: P-084's chunk shows it gone by a listing on its card, and its
  deletion on the dev host is asked of the operator there.
- **The two channel repositories** (`turbolet85/homebrew-andromeda-pulse`, `turbolet85/scoop-andromeda-pulse`) are
  noted for the founder.
- **The five differing OBSERVED readings** stand as recorded; the EXPECT lines are unaffected.

## Scope guard

No request in this round was a capability change to the intent, specialist content or an implementation step. No
re-run of arch or of a specialist was suggested.
