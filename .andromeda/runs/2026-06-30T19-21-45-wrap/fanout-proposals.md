# Fan-out proposals — 2026-06-30-browser-chrome-suppression wrap

7 doc-agents (one per spec source), each scoped to its detectors + the chunk report.

| doc | verdict |
|---|---|
| arch | `proposals: []` — no new IPC/endpoint/event/port/env-var/crate; no new dep/decision |
| security-plan | `proposals: []` — no external-input surface (preventDefault-only DOM handler); no auth/crypto/secret; no deps |
| design-system | `proposals: []` — no UI rendered; canvas CSS is behavioral (interaction-suppression), not a color/spacing/typography token |
| layout-templates | `proposals: []` — behavioral suppression on existing surfaces; no new wireframe region |
| test-plan | `proposals: []` — 4 vitest cases cover the new hook; vitest framework matches §4; harness unchanged |
| obs-plan | **1 proposal** (D-obs-instrumentation, warning) — see below |
| a11y-plan | `proposals: []` — no new interactive element; no schema change; editable-exempt SC 3.3.2/4.1.2 + 0 new violations |

## obs proposal (D-obs-instrumentation, warning) — ESCALATED

- **section:** obs §10 (SLO Invariants & Telemetry Budgets)
- **change (proposed):** add a subsection clarifying that simple browser-UI suppression handlers (DOM event listeners that only call `preventDefault()`, no state mutation / data processing) are exempt from instrumentation when scoped to non-critical-path user-interaction filtering.
- **rationale (agent):** the report cites "obs §10 frame-budget" to justify the hook's zero telemetry, but §10's budgets cover snapshot/WebGPU-frame/buffer SLOs, not event-handler suppression; the exemption is "defensible but not yet formally documented."

### Main's validation
- **Over-reach / false-positive.** `useSuppressBrowserChrome` is a `preventDefault()`-only contextmenu/dragstart suppressor: not a hot path, carries no data flow, is not a §4 desktop-webview instrumentation target (web-vitals / lifecycle / frame-timing / counters). Instrumenting it would VIOLATE obs's own anti-pattern ("NEVER emit high-frequency telemetry inside DOM event handlers — frame-budget cardinality"). My code is correct; the obs-plan is not wrong (it never required instrumenting chrome-suppression handlers). The detector read the report's slightly-imprecise "§10 frame-budget" citation as a missing exemption.
- **No clean playbook-rule match** — the 2026-06-28 over-reach rules are scoped to PRE-EXISTING operations; this hook is new this chunk → escalate (per playbook "no match + structural → escalate") + propose codifying a new rule.
