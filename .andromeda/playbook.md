# Playbook — andromeda-pulse

<!--
Amendment-validation rules consulted by /andromeda-wrap-session's main agent when it validates the
amendments its fan-out proposed. This file GROWS from dogfood — it starts near-empty. One rule per entry:

  - pattern: {the class of amendment this matches}
    verdict: routine | escalate       # routine → apply silently; escalate → halt + ask the user
    note: {why}

"Main is uneasy" (no rule matches but it looks strange) → escalate too; a confirmed escalation pattern
becomes a new rule here. Format owned by /andromeda-wrap-session (`references/amendment-flow.md`).
-->

## Rules
- pattern: Foundation-sequencing deferral — a drift-detector flags a gap (a tool not yet installed, a capability not yet hardened, a dependent not yet wired) whose resolution is a LATER, still-pending chunk's defined job — e.g. `cargo-audit` used before the audit-gate chunk; a struct landed before its validation chunk; an endpoint before its auth chunk.
  verdict: routine
  note: greenfield builds land capabilities first and harden / depend on them in later chunks — expected bootstrap ordering, NOT spec drift (the spec is right; the resolving chunk just hasn't run yet). Apply silently. Auditable caution — it is sequencing ONLY if a later chunk genuinely owns the resolution; a gap with NO resolving chunk anywhere in the route is real drift → escalate.

- pattern: New env var (or similar bounded config input) registration — a drift detector flags that a new env var the chunk added is absent from a spec registry (arch §Occupied Resources env vars; security-plan §Input Validation table), AND the report shows the input is code-validated (bounded enum / truthy / `TryFrom` parse) + unit-tested.
  verdict: routine
  note: a code-validated + unit-tested env var is registry-completeness drift, NOT an unvalidated-boundary security hole — apply the registration silently. D-security-input's escalate-severity is for ACTUALLY-unvalidated boundaries; an env var the report shows validated + tested does not need a HALT. (Codified 2026-06-28 from the P-073 deterministic-L4 env-var registration; escalated once, then ruled routine WITH the user.)

_(more grow from escalations + resolved cases)_
