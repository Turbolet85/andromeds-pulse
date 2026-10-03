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
After the pre-CI commit (`git add -A` over the chunk's work: source, tests, chunk folder, the phase and implement run
dirs, the /phase route/handoff/ledger edits): `git rev-parse HEAD` exit 0, **S = `4a26ad8f1d9cc913f3f369d37ba4bb0bb641e1e4`**,
parent `a69030a`. Recorded in `round-binary.md`.

## Gate 20 — push
Clean-tree guard held (`git status --short` empty after the commit); `git push origin chore/migrate-pulse-to-v3` exit 0,
`a69030a..4a26ad8`; origin head = S.

## Gate 21 — CI on S — RED, external advisory decay
`ci.py conclusion --sha HEAD --wait 2400`: exit 0, `4a26ad8f1d9c verdict: red` — first fail +87 s, `supply-chain (audit +
deny + auditable)`. The entry's `contains verdict: green` atom does NOT hold. The run was then read to completion
(`gh run view 37069724167`, 21:56:14Z → 22:22:36Z): `ci#37069724167` conclusion failure — 11 of 12 jobs success (lint /
test ×3, boot smoke, mcp-server tests, coverage gate, a11y ×3, release build ×2), 1 failure (supply-chain);
`secret-scan#37069723897` success.

**The failure.** `cargo audit` (advisory DB `f8dee89e1b2f`, updated 2026-10-02T22:27:46+02:00, 1288 advisories) found 3
vulnerabilities, all `wasmtime 48.0.3`, all dated **2026-10-02**, all with a stated safe upgrade (`>=48.0.4, <49.0.0` or
`>=49.0.2`): RUSTSEC-2026-0325 (mis-typed tag imports → GC heap corruption), RUSTSEC-2026-0326 (GC rooting across
`try_call`), RUSTSEC-2026-0327 (component async-lifted callback result count → native stack buffer overflow). Distinct
ids, not error blocks. The informational set (8 unmaintained, 2 unsound) is the standing one.

**Not this chunk's — the basis.** `Cargo.lock` is byte-identical between `a69030a` and S (plan gate 6 printed nothing for
`git diff --name-only a69030a -- Cargo.lock …`), and `cargo audit`'s verdict is a function of the lockfile and the
advisory DB alone; the base's own CI (`ci#37012645910`, green) read a DB that predates these three advisories. So the
same three findings land on the base against today's DB, by construction. Disposition is not this pass's: per
security-plan §Dependency Security a finding with a stated safe upgrade is never ignore-listed — it takes a named owner
and stays red until upgraded (a lockfile bump, `wasmtime` 48.0.3 → 48.0.4, outside this chunk's scope guard).

## Gate 22 — Conductor evidence for S
Not run here: it reads Conductor's committed round, which follows the overseer's relay of S and `round-request.md`.
