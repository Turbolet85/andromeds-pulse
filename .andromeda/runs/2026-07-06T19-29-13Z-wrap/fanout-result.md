# Fan-out result — 2026-07-06-incidents-panel-dropdown-layout-bug wrap P2

7 Explore doc-agents (one per spec source), 15 detectors. **All returned `proposals: []` — drift = 0.**

| doc | detectors | verdict |
|---|---|---|
| arch | D-arch-resources, D-arch-decisions | `[]` — no new IPC/endpoint/event/crate/env/port; no new lib/runtime/decision. (`incidents.list_active` is existing, already registered.) |
| security-plan | D-security-input, D-security-auth, D-security-deps | `[]` — no new external-input surface; no auth/crypto/secret; no new deps. |
| design-system | D-design-tokens | `[]` — Coverage tokens flag = `design-token✓` (`--color-raised-2`/`--spacing-xs`/`--spacing-lg`/`--radius-md`/`--duration-standard`); the lone `32px` is the documented titlebar constant. (overflowY scrollbar-ban is a different invariant + pre-existing gap — out of D-design-tokens scope.) |
| layout-templates | D-layout-surface | `[]` — no NEW surface (existing FindingsDropdown, chunk #87; layout-only). Floating window is deferred future-work, not in Changes. |
| test-plan | D-tests-coverage, D-tests-framework, D-tests-obs-harness | `[]` — layout path covered (vitest DOM-shape + p8 axe); vitest/playwright match §2/§4; no harness/status/log-format change. |
| obs-plan | D-obs-instrumentation, D-obs-stack, D-obs-pii | `[]` — no new operation/telemetry/logging; instrumentation n/a; no raw PII (no new DOM sink). |
| a11y-plan | D-a11y-surface, D-a11y-obs-schema | `[]` — no new interactive element (existing, preserved); no a11y/obs schema change. |

**Validate:** 0 escalations · 0 cross-contradictions · intent-consistent (P-080 met its acceptance; floating window + badge-reactivity fix are additive future work). **Apply + cascade: no-op** (no spec body amended; no distillation re-derive). **drift = 0 on exit.**
