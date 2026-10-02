=== architecture 1
## 2026-09-29-p-025-hue-shift-observable-made-gradable — wasmtime requirement 46 → 48.0.3
**Section:** §Stack and Technologies → Plugin runtime · §Established Decisions → [Plugin Runtime] · §Inherited Defaults → Plugin runtime
**Change:** Was requirement `"46"`, lockfile-resolved 46.0.3 (Cranelift 0.133.3); now requirement `"48.0.3"`, lockfile-resolved 48.0.3 as of 2026-09-29 (Cranelift 0.135.3, read from the lockfile at this wrap). The Stack row adds that the bump closed RUSTSEC-2026-0316 and that 49.x is out of reach while the toolchain is pinned at 1.95 (49.0.1 needs Rust 1.96). The 25+ family floor is unchanged.
**Why:** the chunk upgraded wasmtime in-chunk to clear its first real CI run's advisory (founder ruling, via the overseer); three body sites restated the old pin.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
=== architecture 2
## 2026-09-29-p-025-hue-shift-observable-made-gradable — P-025 hue interval stated at the IPC route
**Section:** §Occupied Resources → Tauri IPC routes → `telemetry.frontend.record_*` delegated-timing entry
**Change:** The `metric.constellation.hue_update_ms` clause now states its `duration_ms`: the paint instant minus the service's `ServiceListItem.tier_effective_at_unix_nano` (replayed by `triage::contract::tier_effective_at` — rise = the opening incident's `opened_at_unix_nano`, fall = the last max-tier holder's `resolved_at_unix_nano`), one record per service whose tier changed, emitted by `hueShiftSamples` only for changes witnessed after mount, no service id. Leaf name and fields unchanged.
**Why:** the chunk re-anchored the observable to the interval the P-025 budget bounds, per Conductor's measurement contract.
**Kept:** the `services.list_with_states` entry — the new `tier_effective_at_unix_nano` payload field is a field inside an already-registered procedure, which this registry does not enumerate (chunk #91's `priority_tier` join was never registered either).
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
=== architecture 3
## 2026-09-29-p-025-hue-shift-observable-made-gradable — harness:status derives liveness from the pid
**Section:** §Occupied Resources → xtask CLI surfaces → `cargo xtask harness:status` · §Occupied Resources → Filesystem locations → `run/andromeda-pulse.pid`
**Change:** Was "derived from the pidfile plus the log family's mtime"; now from the pidfile, the liveness of the pid it holds, and the log family's mtime — a dead pid is `not-running` whatever the log says (`ps -o stat=` on Unix, a zombie counts as dead; `tasklist` on Windows). Before it, a crashed app read `running-healthy` for up to 60 s. The verdict JSON and its four arms are unchanged.
**Why:** measured on a CI boot smoke whose app panicked in 12 ms and still read healthy; fixed in-chunk on a founder ruling.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
=== architecture 4
## 2026-09-29-p-025-hue-shift-observable-made-gradable — agent-run CI env export + boot failure path; smoke:hue-shift registered
**Section:** §Occupied Resources → xtask CLI surfaces → `scripts/agent-run.{sh,ps1}` · `cargo xtask smoke:hue-shift` (new)
**Change:**
- agent-run: was "the three invocations share the workflow-level `ANDROMEDA_PULSE_DATA_DIR`, ci.yml:12"; now they run inside one `xvfb-run` and share the variable every job exports to `$GITHUB_ENV` right after harden-runner, because `runner.*` is unavailable in workflow- and job-level `env:`. On a failed readiness poll `boot` reports how the app ended (signal name, exit status, or still running) and runs `bash "$0" cleanup`.
- New registration: `cargo xtask smoke:hue-shift` (`xtask/src/hue_shift.rs`), the P-025 scenario leg — grades the rise against `interpretation.incident.created` and the fall against the first `triage.incident.auto_resolve.tick` with `resolved_count ≥ 1`, each on anchor error ≤ 1000 ms; exit 0 PASS · 1 FAIL · 2 INCONCLUSIVE; artifact under `target/hue-shift/`; dev-host only, not CI-wired.
**Why:** the workflow-level form never parsed, so CI had not run for two months; the leg is a new formalized xtask CLI contract, a registry item per the 2026-08-25 rule.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
=== security-plan 1
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
=== security-plan 2
## 2026-09-29-p-025-hue-shift-observable-made-gradable — npm residuals: GHSA-ggr8 pruned, GHSA-7pqw accepted
**Section:** §Dependency Security → npm channel (`pulse-app/ui`) → current state
**Change:** Was residual roots GHSA-ggr8-5vv4-36mx (deepmerge-ts, "pinned <8 by the entire webdriverio 9 line") and GHSA-jmr9-qjv8-65gv (extract-zip). Now both advisory exceptions are extract-zip — GHSA-jmr9 and GHSA-7pqw-9j4j-h8q3 (range `*`, 2.0.1 the latest release, no fixed release anywhere; npm's remedy a rejected pa11y-ci downgrade; dev-only via the puppeteer chains) — and GHSA-7pqw is the one residual accepted at this chunk. GHSA-ggr8 was pruned: its closing condition fired (webdriverio 9.32.0 brings deepmerge-ts 8.0.2). The chunk's dev-only bumps are recorded: vitest 4.1.11 (GHSA-82fw), qs 6.16.0, undici 6.29.0, webdriverio 9.32.0. Exception counts unchanged (2 advisory + 2 license).
**Why:** the no-safe-upgrade class is exactly what the exception form exists for; the operator relay named GHSA-7pqw the one accepted residual.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
=== obs-plan 1
## 2026-09-29-p-025-hue-shift-observable-made-gradable — hue_update_ms measures tier-effective → paint
**Section:** §8 PII Scrubbing → DELEGATED TIMING leaves → `metric.constellation.hue_update_ms`
**Change:** Was "measures span-arrival → the severity-driven hue the constellation DOT renders"; now `duration_ms` is the paint instant minus the service's `ServiceListItem.tier_effective_at_unix_nano` (rise = the opening incident's `opened_at_unix_nano`, fall = the last max-tier holder's `resolved_at_unix_nano`, Acknowledged keeps its tier), one record per service whose tier changed, emitted by `hueShiftSamples` only for changes witnessed after mount, graded by `cargo xtask smoke:hue-shift`. Fields, the dot-hue / not-a-Halo clause and the metric-fallback clause unchanged.
**Why:** the old interval did not measure the quantity the P-025 ≤ 2 s budget bounds (Conductor's measurement contract).
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
=== obs-plan 2
## 2026-09-29-p-025-hue-shift-observable-made-gradable — the CI perf-budget gates are recorded VACUOUS, owned
**Section:** §1 Telemetry triggers → perf-budget-instruments (WebGPU row) · §10 Performance budgets (WebGPU frame row; Buffer memory row) · §10 CI gates (perf-budget p99 bullet; snapshot p99 bullet)
**Change:** Each site keeps its intended gate and now states that it is measured VACUOUS in CI: the `ci-gates` step feeds `perf-slo-check` only the boot-smoke log (52 records, 0 frame / memory / snapshot samples), so it reads NEUTRAL and cannot fail, and the p99 SLOs are not CI-enforced today. Owner at every site: the working-route entry "Perf-budget gate reads real samples".
**Why:** measured at the chunk's first real CI run; recorded as an OPEN defect with a named owner (playbook 2026-08-28) rather than leaving the sections reading as enforced.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
=== test-plan 1
## 2026-09-29-p-025-hue-shift-observable-made-gradable — webview runner vitest 4; wasmtime row 48.x
**Section:** §1 Surfaces under test (desktop-webview unit row) · §4 Framework (webview unit tests) · §5 plugins → runtime row
**Change:** Was `vitest` 3.x; now `vitest` 4.x (4.1.11 with `@vitest/mocker` 4.1.11 — the vitest 3 mocker is vulnerable, GHSA-82fw). The §5 plugins row was `wasmtime` 46.x / 46.0.3; now 48.x / 48.0.3 as of 2026-09-29 (floor 25+ unchanged).
**Why:** both upgrades landed in-chunk to clear advisories on the first real CI run.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
=== test-plan 2
## 2026-09-29-p-025-hue-shift-observable-made-gradable — harness:status liveness; boot names how a failed app ended
**Section:** §1 Test harness requirements · §3 `boot` (Exit code) · §3 `status` (Command body; Exit code semantics) · §3 PID file → Lifecycle
**Change:**
- status: was derived from the pidfile + log-family mtime; now also a liveness probe of the pid (`ps -o stat=`, a zombie `Z` counts as dead; `tasklist` on Windows) — a dead pid is `not-running` whatever the log says. `classify(pid, alive, newest_log)` is pinned per arm incl. dead-pid and exited-child (was "7 unit pins").
- boot: on a failed poll the sh verb names how the app ended (signal name, exit status, or still running) and runs `bash "$0" cleanup`; `scripts/agent-run.sh` is git mode 100755 (at 100644 the branch died with exit 126).
**Why:** a CI boot smoke whose app panicked in 12 ms read `running-healthy`; fixed in-chunk on a founder ruling.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
=== test-plan 3
## 2026-09-29-p-025-hue-shift-observable-made-gradable — CI rows: per-job DATA_DIR export, xvfb, a11y measured green
**Section:** §9 Pipeline structure → Boot smoke (harness) row · A11y suite row
**Change:**
- Boot smoke: was "sharing the workflow-level `ANDROMEDA_PULSE_DATA_DIR` (ci.yml:12)"; now the cycle runs inside one `xvfb-run` after the Linux system libraries install, and every job exports the variable to `$GITHUB_ENV` right after harden-runner (`runner.*` is unavailable in workflow/job `env:`), pinned by two workflow self-lint guards.
- A11y suite: "runs in CI today" is now measured green on run `ci#36574279289` (6/6, `464f2a3`), with the Playwright chromium browser installed before it.
**Why:** the workflow-level form never parsed; the chunk's CI rehabilitation produced the first real green run.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
=== test-plan 4
## 2026-09-29-p-025-hue-shift-observable-made-gradable — smoke:hue-shift registered as the third scenario leg
**Section:** §3 Per-chunk gate discipline → scenario legs
**Change:** New: `cargo xtask smoke:hue-shift` (`xtask/src/hue_shift.rs`) — not a standard gate, dev-host only, not CI-wired. Grades `metric.constellation.hue_update_ms`: the rise against `interpretation.incident.created`, the fall against the first `triage.incident.auto_resolve.tick` with `resolved_count ≥ 1`, each on anchor error ≤ 1000 ms (the ≤ 2000 ms budget is context; Conductor grades P-025). The storm stops once the rise paints; both samples are graded. Exit 0 PASS · 1 FAIL · 2 INCONCLUSIVE (a precondition unmet); artifact under `target/hue-shift/`.
**Why:** the chunk's live leg, RED at the pre-fix HEAD and GREEN after.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
=== test-plan 5
## 2026-09-29-p-025-hue-shift-observable-made-gradable — coverage measure excludes xtask TEMPORARILY
**Section:** §4 Coverage target · §9 Coverage report row · §10 Cumulative across workspace
**Change:** New: `xtask/` is excluded from the coverage measure via `COVERAGE_IGNORE_FILENAME_REGEX` in `cargo xtask test:coverage` — TEMPORARILY. Measured on the CI lcov: line 84.3 % / function 83.95 % with xtask, 88.6 % / 87.80 % without (the function gate failed with it). Review point: the next epoch-boundary code audit, covering coverage quality, thresholds above 85 % and xtask's re-inclusion. The thresholds (75 / 70 / 85) are unchanged.
**Why:** founder ruling 2026-09-29 — exclude now, but only temporarily; the 85 % bar itself is to be revisited upward.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
=== test-plan 6
## 2026-09-29-p-025-hue-shift-observable-made-gradable — pending coverage triggers: boot failure branch, perf-slo-check arms
**Section:** §1 Pending coverage triggers → `harness-cleanup-verdict-and-boot-spawn-shell-coverage` · `perf-slo-check-arm-coverage` (new)
**Change:**
- Widened: the sh `boot` failure-diagnosis branch also ships with no committed test; it is the instrument of the open Linux-boot watch. Owed now: an assertion per verdict arm and per boot failure-reason arm, plus the ps1 boot leg.
- New row: `xtask/ci/perf-slo-check.sh` reads an empty metric stream as NEUTRAL (it died under `pipefail` on an empty `grep | sort`), and none of its arms has a committed test. Owed: empty ⇒ NEUTRAL, in-budget ⇒ PASS, over-budget ⇒ FAIL. It points to the CI vacuity owned by "Perf-budget gate reads real samples".
**Why:** both branches changed in the chunk's operator pass with CI exit behaviour as their only evidence.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
