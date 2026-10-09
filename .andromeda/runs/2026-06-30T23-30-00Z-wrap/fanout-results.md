# Fan-out results — 2026-06-30-widget-to-dashboard-navigation wrap

7 doc-agents (one per spec source), each given its doc + the chunk report + its scoped detectors.

| doc | proposals | verdict |
|---|---|---|
| arch | `[]` | clean — no new IPC/event/port/env-var/crate; core:window perms granted in default.json are NOT arch §225 entries (agent correctly did not register them) |
| security-plan | `[]` | clean — no new external-input boundary (webview→core window ops), no auth, no deps |
| design-system | `[]` | clean — button uses design tokens ✓ |
| layout-templates | 1 (D-layout-surface: titlebar "Toggle dashboard" button not in §Custom titlebar component) | **routine-REJECT** — within-titlebar element; the Investigate button (chunk #42) set the precedent that titlebar action buttons aren't individually wireframed (playbook within-surface over-reach rule). Handoff note: the titlebar action-button enumeration gap (Investigate + Toggle) is a pre-existing layout-templates completeness gap for a future targeted touch-up. |
| test-plan | 1 (D-tests-framework: amend §3 to permit deferring `nextest --workspace`) | **routine-REJECT + codify** (user-approved) — false drift: the full nextest RAN green at the wrap P7 light gate, so §3 is satisfied (gate ran, just later). New playbook rule appended; testing.md §3 unchanged (always-run). |
| obs-plan | `[]` | clean — no new span (webview→core op; close path reuses existing `ui.layout.transition` + `tray.signpost.shown`), no new telemetry lib, no PII (signpost body static, never logged) |
| a11y-plan | 1 (D-a11y-surface: §P5 label "Expand to dashboard" stale) | **APPLY** — body amended to "Toggle dashboard" + focus note (widget stays; Cmd/Ctrl+Shift+P). Cascaded to a11y.md + a11y-summary.md + 3 SR scripts. Sidecar appended. |

**Summary:** 1 amendment applied (a11y §P5) · 1 escalation resolved (test-plan gate-deferral → reject+codify) · 1 routine-REJECT (layout, → handoff note) · 4 clean. Drift = 0.
