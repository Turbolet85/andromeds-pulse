# Phase 2 fan-out results — 2026-06-28-deterministic-env-gated-l4-mode

7 doc-agents, scoped detectors against report.md.

- **arch** — `D-arch-resources` (warning): `ANDROMEDA_PULSE_L4_DETERMINISTIC` absent from §Occupied Resources §Environment variables → register it. `D-arch-decisions`: clean (third `LlmInferenceRunner` impl behind the unchanged trait — allowed per §Established Decisions [LLM Inference Runtime]).
- **security-plan** — `D-security-input` (ESCALATE): `ANDROMEDA_PULSE_L4_DETERMINISTIC` absent from §Input Validation env-var table → register it. NOTE: the env var IS validated in code (bounded truthy-parse, unit-tested `env_gate_truthy_parse_table`) — registry-completeness, not a missing validation. `D-security-auth`: clean. `D-security-deps`: clean (no deps added).
- **design-system** — `proposals: []` (no UI; tokens n/a).
- **layout-templates** — `proposals: []` (no surface).
- **test-plan** — `proposals: []` (unit+integration present; nextest framework matches; harness unchanged).
- **obs-plan** — `proposals: []` (boot `inference_mode` log + existing L4 tracing; tracing stack unchanged; no PII logged).
- **a11y-plan** — `proposals: []` (no interactive UI; no violation-schema change).

**Validation:** both proposals = register the new env var in spec registries. arch = routine (warning). security = escalate-severity → resolve WITH user. No cross-contradiction. Intent-consistent (the env var IS the chunk's intent; the specs simply lag).
