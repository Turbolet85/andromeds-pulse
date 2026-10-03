# P-075 round binary — 2026-10-02-incident-events-readable-through-mcp

- **S = `4a26ad8f1d9cc913f3f369d37ba4bb0bb641e1e4`** —
  `chore(2026-10-02-incident-events-readable-through-mcp): operator pre-CI commit, for the run this chunk's verdict reads`,
  on `chore/migrate-pulse-to-v3`, parent `a69030a`. Printed by plan gate 19 (`git rev-parse HEAD`) on 2026-10-02.
- Pushed by plan gate 20 (`git diff --quiet && git diff --cached --quiet && git push origin chore/migrate-pulse-to-v3`):
  exit 0, `a69030a..4a26ad8`; origin head = S.
- CI on S (plan gate 21): `ci#37069724167` 11/12 jobs success, red only on `supply-chain` — three `wasmtime 48.0.3`
  advisories published 2026-10-02 (RUSTSEC-2026-0325 / -0326 / -0327), external decay with an unchanged lockfile; see
  `operator-pass.md` §Gate 21. The shipped binaries link `wasmtime 48.0.3` (the plugin host).
- Build command: `cargo build --workspace --release --features mcp-server` (plan gate 16, green, exit 0, 740.43 s, gate run
  `implement-2026-10-02T15-37-29Z`).
- Artifacts (Windows dev host), sha256:
  - `target/release/pulse-app.exe` — `2141d524bd4d0d2f2e5378e95b9acad45c32ef91b55b6964b1886277a724ac35`
  - `target/release/andromeda-pulse-mcp.exe` — `ba8d5c3be3de88dbe8d95eedf83ca988fee09b5e8dff8db965b99cac9d30cd44`
- Source identity: gate 16 built the working tree whose source equals S. After that build, /implement ran two mutation
  checks (`evidence/mutation-checks.md`) — debug builds only, never a release build — and restored both lines by anchored
  Edit; the source diff vs `a69030a` re-read 5 files, +379 / -17, identical to the gated state, and the restored source
  passed its 11 lib pins. Between the restoration and the S commit only evidence documents, run-dir trails and the
  friction ledger changed. A release rebuild at S is a no-op for these binaries.
- The release sidecar was smoked at /implement P3 against a fresh data dir: `tools/list` 9 tools including
  `retrieve_incident_events`; an unknown id → -32603 `incident not found`; exit 0 on stdin EOF.
- Relay: to conductor-builder by the overseer, with `round-request.md` (the seven graded assertions — the six of
  `verification-matrix.json#P-075` plus the incident events read-back — the deterministic mode, the grading posture).
