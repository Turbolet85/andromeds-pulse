# P-075 round binary — 2026-10-02-incident-events-readable-through-mcp

## S2 — the binary the round runs against (2026-10-03, Linux host)
- **S2 = `cdb6c1ed572761ae384597a7ed437222e3a1d1fc`** —
  `chore(2026-10-02-incident-events-readable-through-mcp): operator pre-CI commit, for the run this chunk's verdict reads`,
  on `chore/migrate-pulse-to-v3`, parent `4a26ad8` (S). Printed by plan gate 19 (`git rev-parse HEAD`) on 2026-10-03.
- Pushed by plan gate 20 (`git diff --quiet && git diff --cached --quiet && git push origin chore/migrate-pulse-to-v3`):
  exit 0, `4a26ad8..cdb6c1e`; origin head = S2.
- What S2 adds over S: the shared `incident_events` vocabulary in `triage::contract` (`created` plus the three status
  labels), written by the producer and coerced against by the sidecar; the corrected `tools/list` description; the
  real-producer cross-process leg (`premise-correction.md`, `scope-record.md`).
- Build command: `cargo build --workspace --release --features mcp-server` (plan gate 16), on Omarchy Linux
  (x86_64-unknown-linux-gnu), re-run after the native `npm` stage rebuilt `ui/dist` so the embedded frontend is the one
  at S2: exit 0, `Finished release … in 1m 25s` (pulse-app re-linked; the bundle is `index-DmVOp_0t.js`).
- Artifacts (Linux host), sha256:
  - `target/release/pulse-app` — `23f6ef2bd0b854d9697a4d203b3aea1bf6f40b146420de362112556914976ada`
  - `target/release/andromeda-pulse-mcp` — `e64f3688ec14144910ab25d461ec4ea99c161fda2e1b8824a3d480f17021e02e`
- Source identity: the tree built is S2's — `git status` read 0 changes after the commit, and the build ran after it.
- CI on S2 (plan gate 21): recorded in `operator-pass.md` §Gate 21 (S2).
- Relay: to conductor-builder by the overseer, with `round-request.md` (rewritten for S2: the seven graded assertions —
  the six of `verification-matrix.json#P-075` plus the incident events read-back over the four-kind vocabulary — the
  deterministic mode, the grading posture). The round runs ONCE, against S2.

## S — superseded, never relayed (2026-10-02, Windows host)
- S = `4a26ad8f1d9cc913f3f369d37ba4bb0bb641e1e4`, parent `a69030a`; pushed `a69030a..4a26ad8`.
- CI on S: `ci#37069724167` 11/12 jobs success, red only on `supply-chain` — three `wasmtime 48.0.3` advisories published
  2026-10-02 (RUSTSEC-2026-0325 / -0326 / -0327), external decay with an unchanged lockfile.
- Artifacts (Windows dev host), sha256: `pulse-app.exe` `2141d524…ac35`, `andromeda-pulse-mcp.exe` `ba8d5c3b…cd44`
  (gate 16, run `implement-2026-10-02T15-37-29Z`).
- Superseded because assertion 7's vocabulary bullet failed on S: the producer's `created` event read back as `unknown`
  (relayed by the overseer 2026-10-03). Conductor held its drive; no round ran on S.
