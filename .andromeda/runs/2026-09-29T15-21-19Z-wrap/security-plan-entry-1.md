
## 2026-09-29-p-025-hue-shift-observable-made-gradable — the `cargo audit` standing deferral ENDED (pin #22 discharged)
**Section:** §Dependency Security → CI integration → the standing-deferral bullet
**Change:** Was a standing deferral — `cargo audit` cannot load the RustSec DB (`duplicate advisory ID: RUSTSEC-2026-0244`), overlap `cargo deny check advisories`, probe every 3rd wrap, a running list of discharged interval points, next point session 67. Now the bullet records the deferral ENDED: `cargo audit` loads the DB (1273 advisories) and exits 0 after the rustls 0.23.45 / quinn-proto 0.11.15 / wasmtime 48.0.3 bumps (RUSTSEC-2026-0285 / -0185 / -0316); it is a plain pass/fail gate again. New: it reads the advisory-db under `$CARGO_HOME`, not `$HOME/.cargo/advisory-db` (a probe of the latter measured a copy 359 commits behind). Kept in the body: the distinct-ids counting rule, the no-ordinal rule, visible advisory dispositions. The probe-point history leaves the body; the entries below and the origin report hold it. Partial retirement of the 2026-08-16 re-ratification, the 2026-08-25 counting-rules and the 2026-08-28 ordinal entries: their rules stand, their probe points do not.
**Why:** the bullet's own terminating clause ("ends the first time `cargo audit` loads") fired at this chunk's gates; the operator relay directed the retirement.
**Supersedes:** 2026-08-17-incident-fingerprint-producer-repaired — `cargo audit` standing deferral: interval point DISCHARGED, re-pinned
2026-08-26-cadence-runaway-blocking-pool — cargo audit interval point 46 discharged
2026-08-27-idle-observer-generation-damper — standing-deferral point 49 discharged
2026-08-29-app-registry-reconciliation — cargo-audit deferral, session-55 interval point discharged
2026-08-30-agent-harness-teardown-truth — pin #22 session-64 FULL-FORM discharge recorded; pointer → session 67
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
