# P2 fan-out — 7 drift-detectors (2026-07-07-traces-table-auto-refresh)

All 7 doc-agents returned `proposals: []` (zero drift). Webview-only, reuse-only chunk: zero new IPC/crate/dep/schema/env/port/telemetry; existing surfaces reused. Mirrors the P-080 webview-only outcome.

| doc | detectors | verdict | reason (cited from report) |
|---|---|---|---|
| arch | D-arch-resources, D-arch-decisions | `proposals: []` | Symbols/APIs NONE new; re-poll reuses existing `traces.query`; no new library/runtime; re-poll-a-query is distinct from the `pulse://stream/*` push contract (SSE/WebSocket rejected, polling is not that). |
| security-plan | D-security-input, D-security-auth, D-security-deps | `proposals: []` | No new input surface (reuses typed `TracesQueryArgs`, `cursor:null`); no auth/crypto/secret; Dependencies NONE. |
| design-system | D-design-tokens | `proposals: []` | Coverage `tokens n/a`; no hardcoded hex/px; `--color-accent` token used; Tables pattern unchanged. |
| layout-templates | D-layout-surface | `proposals: []` | The Full-dashboard Traces trace-table surface pre-exists (P1/P4); no new region. |
| test-plan | D-tests-coverage, D-tests-framework, D-tests-obs-harness | `proposals: []` | vitest at the mandated webview-unit tier; framework matches §2/§4; 5-command harness untouched (warm re-embed smoke is a per-chunk method per an existing rule, not a §3 contract change). |
| obs-plan | D-obs-instrumentation, D-obs-stack, D-obs-pii | `proposals: []` | Reuses existing `viz.query.traces` span (proven at runtime, 190 @1s); no new logger; no browser OTel SDK; PII scrubbed via the existing `viz` allowlist; `announce()` is a static a11y string. |
| a11y-plan | D-a11y-surface, D-a11y-obs-schema | `proposals: []` | No new interactive element; the polite empty→populated announcement is covered by the §4/§7 P4 live-region contract (SC 4.1.3); no schema change. |

**Result:** 0 amendments · 0 escalations · cascade no-op · drift = 0.
