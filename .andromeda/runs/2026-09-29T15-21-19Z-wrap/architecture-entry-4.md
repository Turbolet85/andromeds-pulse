
## 2026-09-29-p-025-hue-shift-observable-made-gradable — agent-run CI env export + boot failure path; smoke:hue-shift registered
**Section:** §Occupied Resources → xtask CLI surfaces → `scripts/agent-run.{sh,ps1}` · `cargo xtask smoke:hue-shift` (new)
**Change:**
- agent-run: was "the three invocations share the workflow-level `ANDROMEDA_PULSE_DATA_DIR`, ci.yml:12"; now they run inside one `xvfb-run` and share the variable every job exports to `$GITHUB_ENV` right after harden-runner, because `runner.*` is unavailable in workflow- and job-level `env:`. On a failed readiness poll `boot` reports how the app ended (signal name, exit status, or still running) and runs `bash "$0" cleanup`.
- New registration: `cargo xtask smoke:hue-shift` (`xtask/src/hue_shift.rs`), the P-025 scenario leg — grades the rise against `interpretation.incident.created` and the fall against the first `triage.incident.auto_resolve.tick` with `resolved_count ≥ 1`, each on anchor error ≤ 1000 ms; exit 0 PASS · 1 FAIL · 2 INCONCLUSIVE; artifact under `target/hue-shift/`; dev-host only, not CI-wired.
**Why:** the workflow-level form never parsed, so CI had not run for two months; the leg is a new formalized xtask CLI contract, a registry item per the 2026-08-25 rule.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/
