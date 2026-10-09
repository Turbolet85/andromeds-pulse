# Fan-out results — 2026-10-04-l4-hardware-probe-finds-cuda-on-arch-layout-hosts

Report: `andromeda-pulse-0.3.0/chunks/2026-10-04-l4-hardware-probe-finds-cuda-on-arch-layout-hosts/report.md`.
Detectors: 19 entries, prompt counts arch 2 · security-plan 4 · design-system 2 · layout-templates 2 · test-plan 3 ·
obs-plan 4 · a11y-plan 2 = 19 (each entry names one doc). Keyed contracts: arch · tests · obs · a11y `NOT MIGRATED`
(exit 3) — the contracts line was dropped. Every return was plain YAML plus `#` comments; stripping removed only the
comment lines (their substance below); no HTML entities (entities=0), no raw twin warranted.

## Verdicts
- architecture — 1 proposal. Stripped: D-arch-decisions no drift (dev-only `tempfile`, already locked; probe keeps the
  [LLM Inference Runtime] routing rule).
- security-plan — 1 proposal. Stripped: D-security-auth / -deps / -logging no drift (no key/secret touch; `tempfile`
  not banned; no scrub/log boundary changed).
- design-system — `proposals: []` (no UI; no status ruling touches its surfaces; the restored
  `andromeda-pulse-0.4.0-incubator/` matches design-system.md:14's pointer and changes no status).
- layout-templates — `proposals: []` (no UI surface; GPU mentions are the WebGPU Halo layer).
- test-plan — 1 proposal. Stripped: D-tests-framework / -obs-harness no drift (rstest + nextest + TempDir on spec;
  only comment marks in xtask).
- obs-plan — `proposals: []`. Stripped note: obs-plan §8 has no `interpretation.hardware.detect` leaf row; the emit
  sites are unchanged by this chunk, so the agent made no proposal (pre-existing; see disposition below).
- a11y-plan — `proposals: []`.

## Proposals and dispositions

### 1. architecture — D-arch-resources (warning)
- section: §Occupied Resources → Environment variables (reserved at arch level)
- change: register `ANDROMEDA_PULSE_HARDWARE_PROFILE` — product-consumed override read at `HardwareProfileDetector::new()`
  (`ENV_HARDWARE_PROFILE_OVERRIDE`, `crates/interpretation/src/hardware.rs`); trim + lowercase + closed match over the four
  profile labels (snake or kebab); other values → WARN on `interpretation.hardware.detect` + real detection; never logged
  by value; registry completeness (the var unchanged by this chunk).
- basis: architecture.md:220 (block), siblings :229/:230/:236; `grep -c HARDWARE_PROFILE` → 0.
- **Disposition: APPLY** (check 1, recorded-direction branch). The playbook's env-var registration rule (`playbook.md:20`)
  does NOT govern — its precondition "a new env var the chunk added" is false (the var predates the chunk). No other
  rule matches. The plan's P5-approved `Expected amendments (wrap)` entry names this exact change (register the var in
  §Occupied Resources → Environment variables), so the recorded direction settles it; a rule for the class is proposed
  at the wrap card. Checks 2–6 clear (no opposing proposal; consistent with intent; absence cites its grep; expected
  entry 1 matched; not a disproved claim).

### 2. security-plan — D-security-input (escalate)
- section: §Input Validation → `CLI / env var inputs` row (security-plan.md:138)
- change: add `ANDROMEDA_PULSE_HARDWARE_PROFILE` to the row's list and its validation (trim + lowercase + closed
  four-label match; other values rejected to real detection with one WARN; not a path, never canonicalized, never logged
  by value).
- **Disposition: APPLY** (check 1). Escalate severity is the detector's, not the finding's: `playbook.md:82`
  routine-APPLY-BY-ACTUAL-CLASS governs — (a) the escalate condition is affirmatively absent: the report states the input
  is code-validated by a closed parse (Expected amendments, entry 2) and its parse pins pass in the regression gate
  (`package(interpretation)`, the existing `parse_profile_override_*` tests); no unvalidated boundary; (b) the actual
  class (registry completeness) has its own disposition — the recorded direction of the plan's expected entry 2, which
  names this change. Not a boundary widening: no new input class crosses (the var, its parse and its reader are
  unchanged). Checks 2–6 clear.

### 3. test-plan — D-tests-coverage (warning)
- section: §4 → What unit tests cover → `interpretation crate` bullet (test-plan.md:385)
- change: extend the bullet with the hardware-probe candidate-set pins.
- **Disposition: APPLY** (check 1, `playbook.md:106` accurate this-chunk addition — the pins are this chunk's). The
  applied text is re-derived from the report, NOT the change line, which mis-states the mutation readings ("widening the
  set gives 4 RED, narrowing it gives 5 RED"); per the report: dropping `usr/lib` → 4 RED, dropping `libcuda.so.1` → 5 RED,
  reverting the arm → clippy dead-code RED. Expected entry 3 matched.

## Check 5 — expected amendments
- arch env-var registration → proposal 1 · security-plan row → proposal 2 · test-plan §4 bullet → proposal 3. Floor met.

## Check 6 — disproved claims
- `.claude/docs/services/interpretation.md:26` "dlopen-shaped" → DISPOSED: routed to the cascade's leaf re-derive of
  `docs/services/interpretation.md` (an arch §Occupied Resources amendment for the `interpretation` module's input; the
  leaf is re-computed from source at step 3).

## Not drift, recorded
- obs-plan §8 lacks an `interpretation.hardware.detect` row — the target, its leaf in `observability.rs` and its emit
  sites predate this chunk and are unchanged; `playbook.md:98` (not this chunk's drift). Recorded in the handoff as an
  observation, not actioned.

## Escalations
- none. Every proposal applies; nothing staged to escalate.
