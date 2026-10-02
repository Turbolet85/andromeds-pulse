# P2 fan-out results — 2026-07-08-self-explaining-empty-states

7 Explore doc-agents, one per spec source. Report is the single source each read.

## Clean (proposals: [])
- **arch** (D-arch-resources, D-arch-decisions) — new symbol is a webview `EmptyState` component (not an arch resource); no new IPC/port/socket/env/crate; no new dependency; no §Stack/§Established-Decisions contradiction.
- **security** (D-security-input, D-security-auth, D-security-deps) — no new external-input surface (webview render branch, no IPC/HTTP/deserialized struct); no identity/session/token/keys; no new dependency.
- **design** (D-design-tokens) — every new UI element `tokens design-token✓`; no `hardcoded✗`. NOTED but declined as out-of-invariant-scope: the doc's §Loading/Empty-States prose says message color `#7D8697 (Tertiary)` while the chunk uses `--color-text-secondary` — a token-VALUE/contrast concern, not token-vs-hardcoded → surfaced to the orchestrator as an observation, not a D-design-tokens proposal.
- **tests** (D-tests-coverage, D-tests-framework, D-tests-obs-harness) — every new path tested (vitest 732 + p13 axe); runner is vitest (matches §4); no harness/status/log-format change.
- **obs** (D-obs-instrumentation, D-obs-stack, D-obs-pii) — no hot-path operation (render is not instrumentable; obs no-sink); no telemetry library; no logging → no PII path.
- **a11y** (D-a11y-surface, D-a11y-obs-schema) — the new `EmptyState` is NON-interactive (no focusable, affordance-EXEMPT); the perceivable coverage (SC 1.4.3/1.1.1/4.1.3) + the new p13 axe spec are present; no violation-schema change.

## Proposal (1)
- **layouts · D-layout-surface · warning** — section "Surface: desktop-webview → new Component — Empty / error state (data views)". change: add a §Component entry documenting the shared empty-state region (Metrics/Logs/Snapshots) — centered decorative Observatory glyph + message + exporter hint (:4318/:4317) in `--color-text-secondary`/`font-body`, plus the distinct honest-error variant. rationale: the report adds these empty/error states + a new shared `EmptyState` component, but no §Wireframe/§Primary-screen depicts a zero-data/error region for the Metrics/Logs data views (only Traces is wireframed; zero-data documented elsewhere = canvas fallback, tray Silent).

## Orchestrator validation → escalate
Both the layouts proposal AND the design observation sit on the genuinely-ambiguous apply-vs-reject-vs-handoff boundary (accurate this-chunk current-truth addition ↔ 2026-06-28 over-reach-reject ↔ 2026-06-29 pre-existing-handoff), with no codified playbook rule (the "apply-side within-existing-structure" candidate was deferred-to-recurrence at the P-070 wrap). Escalated to the user with a marked recommendation (apply both + codify).
