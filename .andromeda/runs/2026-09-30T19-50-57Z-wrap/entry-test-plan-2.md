
## 2026-09-30-perf-instruments-measure-their-budgets — ipc-rejection-wire-coverage widened to the adapter wire
**Section:** §1 ipc-rejection-wire-coverage row
**Change:** Widened to the second fire-and-forget webview → Rust wire (`reportAdapterOutcome` → `telemetry.frontend.record_webgpu_adapter` → `ui.webgpu.adapter`): its committed pins (ui-bridge ×5, allowlist ×3, vitest 9 + 6) stay green if the reporter silently no-ops, and its only witness is the dev-host frame leg — no CI job can witness it. Owed: one committed cross-boundary assertion for BOTH procedures.
**Why:** the same one-time-proof class: the stale-bundle silent no-op recurred in this chunk before its live leg.
**Ref:** .andromeda/runs/2026-09-30T19-50-57Z-wrap/
