# P2 fan-out results — 2026-06-29-predictable-close-self-verify

7 drift-detector doc-agents (one per spec source), validated against the report + playbook.

| doc | detectors | verdict |
|---|---|---|
| arch | D-arch-resources, D-arch-decisions | `proposals: []` — no new IPC/port/env/crate/dep; `cargo xtask self-verify` conforms to the established cargo-xtask pattern; `tray.signpost.shown` is a tracing target, not an Occupied Resource. |
| security | D-security-input, D-security-auth, D-security-deps | `proposals: []` — no new input surface; notification uses the pre-declared outbound-only `pulse:notification`; no new deps. |
| design | D-design-tokens | `proposals: []` — OS-native notification + dev tool → no web UI tokens (consistent with design-system §desktop-native). |
| a11y | D-a11y-surface, D-a11y-obs-schema | `proposals: []` — OS-native notification → OS a11y; close-to-tray preserves the existing P5 focus contract; no schema change. |
| obs | D-obs-instrumentation, D-obs-stack, D-obs-pii | `proposals: []` — `tray.signpost.shown` is instrumented (message-only, aggregate-only); existing `tracing` stack; notification body never logged (by construction). |
| layouts | D-layout-surface | **1 proposal — APPLIED (routine).** Close-to-tray first-close OS-notification is a genuinely-new notification trigger this chunk added; layout-templates §Notifications listed only 3. → added trigger #4 + summary line + sidecar. Clean new-surface documentation (NEW in the report's Changes; design-confirmed; arch §OS-notification-policy explicit-decision clause). |
| tests | D-tests-coverage, D-tests-framework, D-tests-obs-harness | **1 proposal — ESCALATED → HANDOFF (resolved WITH user).** D-tests-obs-harness: test-plan §3 `logs` cmd globs `*.log` but obs sink produces `agent-latest.jsonl.<date>`. Real but PRE-EXISTING 3-way inconsistency (test-plan §3 spec ≠ agent-run.sh bare `agent-latest.jsonl` ≠ reality; + `smoke.rs::run_smoke` same bare bug) the chunk only EXPOSED. User chose handoff (fixing §3 alone is incomplete) → handoff note + new playbook rule codified. |

## Resolution
- **Amendments applied:** 1 (layout-templates §Notifications trigger #4 + Primary-screens summary; sidecar appended).
- **Escalations resolved:** 1 (tests/D-tests-obs-harness → handoff; playbook rule codified: "pre-existing cross-spec harness-bind inconsistency a chunk only EXPOSES → handoff, not its amendment").
- **Cascade:** no-op (no `.claude/docs/*-summary.md` or `.claude/rules/*` derives the notification-trigger list).
- **Drift = 0:** every proposal is {applied | escalated-and-resolved}; zero open.
