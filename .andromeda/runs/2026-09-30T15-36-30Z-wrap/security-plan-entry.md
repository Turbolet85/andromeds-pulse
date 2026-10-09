
## 2026-09-30-perf-budget-gate-reads-real-samples — the boot-recorder state files join the harness-only class
**Section:** §Security Anti-Patterns → Input (the harness/xtask-only carve-out) · §Threat Model Summary → CLI input trust boundary
**Change:** the carve-out gains a sibling class — STATE FILES, not env vars: `run/andromeda-pulse.spawn` (app pid) and `run/andromeda-pulse.exit` (one-line end), written by the `scripts/agent-run.{sh,ps1}` boot waiting wrapper (ps1 since this chunk) and read only by the harness — `boot` polls the spawn record ≤ 5 s, and `cargo xtask harness:status` reads the exit record through `read_ended`'s bounded grammar (one line, ≤ 48 printable ASCII). The product never reads either, so canonicalize-and-confine has nothing to guard. Was: the trust-boundary line said "the harness-only tool-locator carve-out is unchanged".
**Why:** the ps1 recorder made these records a cross-shell harness surface; the sh records had never been named in this plan. Validated at the wrap as routine by actual class — no product boundary crossed, the read is grammar-bounded.
**Ref:** .andromeda/runs/2026-09-30T15-36-30Z-wrap/
