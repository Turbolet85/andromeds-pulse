
## 2026-09-30-perf-budget-gate-reads-real-samples — app.boot.gpu.check carries wgpu_backend only
**Section:** §1 multi-platform-exporter-compat row
**Change:** `app.boot.gpu.check` carries `wgpu_backend` only — the compile-target default backend (`detect_wgpu_backend()`, a `cfg!(target_os)` switch), not an adapter probe. Was: `gpu_available: true | false` beside it; the removed field was a hardcoded `false`. No truthful adapter-state record exists yet; owner "Perf instruments measure what their budgets name".
**Why:** the field read `false` on a 578-frame GPU control and on CI alike, so no reader could treat it as adapter evidence; the chunk removed it from code and allowlist rather than leave a known-false boot record.
**Ref:** .andromeda/runs/2026-09-30T15-36-30Z-wrap/
