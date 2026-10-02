# Operator pass — 2026-10-02-incident-events-readable-through-mcp

Run by the session on the overseer's word (2026-10-02: "Run the operator pass in the approved plan: entries 17-22
(hygiene, pre-push:linux, the pre-CI commit, S = rev-parse HEAD, push, CI read). Keep the disk guard. Then stop and report
S"), after /implement closed green + smoke. The disk guard (stop below 15 GB free on D:) held throughout. No pulse-app
process was launched; 4317/4318 stayed closed.

## Gate 17 — hygiene
`python -X utf8 …/gate.py hygiene`: exit 0, `hygiene: clean — read 29 (runs 27 · evidence 2)`; re-fired after this file
was written (see below).

## Gate 18 — `cargo xtask pre-push:linux`
Exit 0, verdict document `"verdict": "green"`, six stages (script-modes · source-lint · npm · clippy · test ·
ci-gates). The WSL `test` stage ran the workspace suite at 2573/2573 passed — the Windows 2571 plus the two Unix-only
exit-cause arms — including this chunk's 11 feature-free pins (5 corpus `load_incident_events_*`, 6 dispatcher
`retrieve_incident_events_*`).

## Gate 19 — S
Printed after the pre-CI commit; recorded in `round-binary.md`.

## Gate 20 — push
Recorded in `round-binary.md` with S.

## Gate 21 — CI on S
Recorded in `round-binary.md` with S.

## Gate 22 — Conductor evidence for S
Not run here: it reads Conductor's committed round, which follows the overseer's relay of S and `round-request.md`.
