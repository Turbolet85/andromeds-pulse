
## 2026-10-04-l4-hardware-probe-finds-cuda-on-arch-layout-hosts — the hardware-profile override in the CLI / env var row
**Section:** §Input Validation → "CLI / env var inputs" row
**Change:** the row lists `ANDROMEDA_PULSE_HARDWARE_PROFILE` and states its validation: `parse_profile_override` — trim + lowercase, then a closed match over the four profile labels in snake or kebab case; any other value rejected to real detection with one WARN on `interpretation.hardware.detect`, never a panic; not a path, never canonicalized, never logged by value (only the resolved profile label).
**Why:** registry completeness for a code-validated input that predates this chunk; no boundary widened (the var, its parse and its reader are unchanged). The detector's escalate severity is the detector's, not the finding's: applied by actual class on the plan's recorded expected amendment.
**Ref:** .andromeda/runs/2026-10-04T17-56-08Z-wrap/
