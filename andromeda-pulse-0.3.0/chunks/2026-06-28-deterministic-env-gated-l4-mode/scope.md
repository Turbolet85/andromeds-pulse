# Scope — Deterministic env-gated L4 mode

**Marker:** `2026-06-28-deterministic-env-gated-l4-mode`
**Capability:** P-073 (intent F13a) · **KEYSTONE**
**Working entry:** "Deterministic env-gated L4 mode — canned `L4Output` via `StubInferenceRunner`, env/flag-selected, so the incident path completes without GPU/3B (P-073 · intent F13a)"

## What this chunk builds
An **env/flag-gated deterministic L4 mode**: when the gate is set, the L4 interpretation step returns a **canned, deterministic `L4Output`** (reusing the existing `StubInferenceRunner` pattern) instead of invoking the real `llama-cli` subprocess — so the **digest → L4 → incident** chain completes **reproducibly**, with no GPU / model / 3B dependency, for demos, tests, and external (Conductor) verification.

## Why (the gap)
OBSERVED (intent F13a): a real retry-storm produced NO incident — the digest→L4→incident chain never completes because the real L4/LLM step is degraded/fragile. The deterministic mode makes the chain's completion reproducible, decoupled from model quality (explicitly out of scope for v0.3.0 per intent §5).

## Boundaries
**In scope:**
- A new env/flag gate (an `ANDROMEDA_PULSE_*` variable) that selects the deterministic L4 runner at the `pulse-app` binary boundary (where the `LlmInferenceRunner` impl is chosen per hardware tier).
- A deterministic canned `L4Output` that is schema-valid and drives the EXISTING incident-creation path to a real incident — including a deterministic severity so the red-dot / Findings climax fires.
- Honoring the existing `LlmInferenceRunner` trait contract unchanged — the deterministic runner is another impl behind it, selected by the gate.

**Out of scope:**
- The real `LlamaCliInference` behavior + the 3B model's judgment quality (deferred per intent §5 — the deterministic mode sidesteps it for verification).
- The incident-creation producer itself (chunk #92, already landed) — this chunk only makes the L4 step deterministic so the EXISTING chain completes.
- Investigate-action wiring (P-072), Conductor verification (P-075), the integration UX test (P-076) — separate chunks that CONSUME this deterministic mode.

## Surfaces / contracts touched
- `interpretation` crate — `LlmInferenceRunner` trait + the existing `StubInferenceRunner` pattern (the canned-output source).
- `pulse-app` binary boundary — the runner-selection site (currently picks `LlamaCliInference` per chunk #80 `HardwareProfileSource` tier); add the deterministic-gate branch.
- A new `ANDROMEDA_PULSE_*` env var → arch §Occupied Resources registration (surfaced for a wrap amendment; phase is read-only on arch).
- The `L4Output` contract (the canned value must be valid) + the chunk #81 cadence/digest subscriber that invokes L4.

## Acceptance shape (from P-073 EXPECT)
With the deterministic L4 env/flag set, a seeded digest yields a deterministic `L4Output` and a created incident, **reproducibly**, with no GPU/model. (Verification: integration — `verification-matrix.json#P-073`.)
