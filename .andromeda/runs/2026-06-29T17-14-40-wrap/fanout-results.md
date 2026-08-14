# Fan-out drift results — 2026-06-29-window-geometry-movable-shell wrap

7 doc-agents (one per spec source), evaluated against report.md.

| doc | verdict | resolution |
|---|---|---|
| arch | `[]` (detector under-scoped) | orchestrator-judged: registered `window-geometry.json` in §Occupied Resources §Filesystem locations (routine completeness; precedent corpus.db #68 / export sink #95). D-arch-resources literal scope = IPC/port/env-var/crate; filesystem-file detector-growth flagged. |
| security-plan | 1 proposal — D-security-input (geometry-file boundary) | **routine** per the bounded-config-input playbook rule (report shows it validated + unit-tested → not an unvalidated-boundary HALT). Applied: new §Input Validation row. |
| design-system | `[]` | clean — no new UI element (Titlebar untouched; tokens n/a). |
| layout-templates | `[]` | clean — titlebar surface pre-existed chunk #24; agent self-applied the pre-existing-surface reject. |
| test-plan | `[]` | clean — new paths unit-tested; `cargo nextest` runner matches §2/§4; harness unchanged. |
| obs-plan | `[]` | clean — PII redacted (no coordinate logging), no new hot path, `tracing` stack unchanged. |
| a11y-plan | `[]` | clean — no new interactive element (capability enables existing titlebar affordances); schema unchanged. |

**Escalations:** 0. **Amendments applied:** 2 (arch §Occupied Resources, security-plan §Input Validation). **Cascade:** no-op (CLAUDE.md / stack.md / security.md rule / security-summary.md do not enumerate the amended registry rows — they reference/summarize at a higher level). **Drift = 0.**
