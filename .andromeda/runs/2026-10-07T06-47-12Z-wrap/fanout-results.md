# Fan-out results — the 2026-10-07T06-47-12Z wrap

Chunk `2026-10-06-l4-first-hypothesis-names-the-triggering-service`. Seven doc-agents, one parallel batch, each given
its doc, the report and its detectors (19 detector ids over 7 prompts: arch 2 · security-plan 4 · design-system 2 ·
layout-templates 2 · test-plan 3 · obs-plan 4 · a11y-plan 2; the four migrated docs also their keyed-contract render).
Entity probe on every return: 0 HTML entities. Stripped from each return: YAML comments only (the detector's own
no-drift basis); their substance is in the verdict lines below.

## Verdicts

- **architecture** — 3 proposals (D-arch-decisions ×3). D-arch-resources: no drift (no IPC route, port, env var,
  crate or table added; the probe's flags, shapes, arms and row key are a dev example's, not registry-class).
- **security-plan** — 4 proposals (D-security-input ×4, raised at `warning`: the invariant holds, the measured basis
  it states moved). D-security-auth, D-security-deps, D-security-logging: no drift.
- **design-system** — `proposals: []`. No UI element added (both coverage rows `tokens n/a`); no status claim of the
  doc touched.
- **layout-templates** — `proposals: []`. No surface or region added; the doc states nothing about the L4 prompt.
- **test-plan** — 4 proposals (D-tests-coverage ×4). D-tests-framework, D-tests-obs-harness: no drift.
- **obs-plan** — `proposals: []`. No hot-path operation, logger, emit or §10 narrative touched; the reading's
  `elapsed_ms` is a reading beside the §10 budget, not its sample.
- **a11y-plan** — `proposals: []`. No interactive element; neither schema changed.

## architecture — parsed proposals and dispositions

- **A1** · D-arch-decisions · warning · §Established Decisions → [Fault Identity] (arch:73, the lineage clause)
  - change: the current-lineage clause reads `v2.6 / v1.5-fallback / v1.5-reflection` (moved together at this chunk;
    `v2.5 / v1.4-*` was the lineage of the two reworded obligations from the 2026-10-04 rank-1 chunk).
  - basis as returned: `crates/interpretation/src/schema.rs:35, :44, :59` · report · arch:73@c6028
  - **disposition: REJECTED as returned, then RAISED by the orchestrator and APPLIED.** The basis cites source line
    numbers the report does not carry — the re-derivation tell (amendment-flow §Validate, before the checks). The
    fact is in the report (Symbols / APIs; Counts) and the entry is on the plan's expected-amendments list, so it
    enters through check 5 as routine (playbook: Accurate this-chunk addition) and is applied from the report alone.
- **A2** · D-arch-decisions · warning · §Established Decisions → [Fault Identity] (arch:73, the instruction's
  obligation list)
  - change: the list of what `TRIGGER_FRAMING_INSTRUCTION` tells the model gains the scope obligation — when the cue
    line under ATTENTION CUES carries a `scope_id`, the first hypothesis statement names that value exactly as
    written and attributes the signal to nothing else, a service whose name merely contains it included (one static
    ASCII sentence; conditional; no cue kind, no service; the `scope_id` by reference; 787 B, was 538 B).
  - basis as returned: `crates/interpretation/src/prompt.rs:93` · report · arch:73@c5509
  - **disposition: REJECTED as returned (the same tell), RAISED under check 5 and APPLIED** from the report's
    Symbols / APIs bullet; routine (Accurate this-chunk addition).
- **A3** · D-arch-decisions · warning · §Established Decisions → [Fault Identity] (arch:73, the `TRIGGER:` line
  statement and the close of the measurement account)
  - change: (1) at the `TRIGGER:` line statement, the line carries the kind label only, never the `scope_id` or a
    service name (overseer ruling 2026-10-04, founder-delegated; unchanged at this chunk; a service-bearing line was
    measured as two harness-only renders and never written into the product; the superseding question moot because
    the smallest lever met the bar); (2) after the model-ship sentence, the pre-registered reading as measured at
    this chunk — model, runtime, n, grader, `shipped` 40/40 + 20/20, `ns` 38/40 + 20/20, `L` / `LI` 40/40 + 20/20,
    selection `shipped`, PASS, guard HOLDS — with its limit beside it: the baseline met the bar too, no
    known-positive for d3, the measured effect two generations on S3, the d3 remedy unmeasured (Conductor's fifth
    series the only end-to-end reading).
  - basis: report · arch:73@c5173
  - **disposition: APPLY.** Check 1: routine (Accurate this-chunk addition; it records a standing ruling and a
    measurement, it reverses no locked decision and widens no boundary). Check 5: it is the plan's own entry. The
    existing "still FAILS the pre-registered bar of 36/40" sentence is a dated reading of another model and grader
    and stays.

## security-plan — parsed proposals and dispositions

- **S1** · D-security-input · warning · §Input Validation → row `L4 inference argv prompt`
  - change: a dated v2.6 note after the 7,575 B note (the same synthetic A6 at v2.6, +249 B, `--dry-run`, primary
    builder only, 7,824 B) and the derived clause re-based: ~2.09× above the 7,824 B observed maximum, 8,560 B of
    headroom; 7,575 B stays as the prior note.
  - **disposition: APPLY.** Check 1: the detector carries `escalate`, the finding is a stale measured figure — the
    2026-08-23 rule (apply by actual class) governs: (a) the escalate condition is affirmatively absent (no new
    external-input surface; the prompt passes the unchanged validator), (b) the actual class has its own disposition
    (Accurate this-chunk addition). The `≈ 8.4 KiB` rendering is arithmetic on the report's 8,560 B.
- **S2** · dependent-of S1 · §Security Anti-Patterns → Code Patterns (the mirror)
  - change: the mirror re-based — ~2.09× the observed maximum, 7,824 B, the 2026-10-07 v2.6 composition of A6;
    before it 7,575 B (A6 at v2.5); the rest of the before-chain unchanged.
  - **disposition: APPLY** with S1 (one group).
- **S3** · D-security-input · warning · §Input Validation → the same row
  - change: the clause "the fallback and reflection compositions … are unmeasured" dated to v2.5, and the first
    measurements stated: at v2.6 on an EMPTY digest primary 7,011 B, fallback 7,200 B (its pin asserts under
    8,000 B), reflection 7,549 B; the reading's largest product composition `shipped` S8 at 7,693 B.
  - **disposition: APPLY** (same rule as S1). Scope kept as measured: empty-digest readings only.
- **S4** · dependent-of S3 · §Security Anti-Patterns → Code Patterns (the mirror)
  - change: the mirror clause dated to v2.5 and the empty-digest readings added.
  - **disposition: APPLY** with S3 (one group).

## test-plan — parsed proposals and dispositions

- **T1** · D-tests-coverage · warning · §4 → interpretation crate (the instruction's clause)
  - change: the instruction's parenthetical gains the scope obligation and its pin
    `every_tier_obliges_the_first_hypothesis_to_name_the_cue_scope` (exactly once per tier; red before green;
    mutation-checked).
  - basis as returned: test-plan:229 · `crates/interpretation/src/prompt.rs:639` · report
  - **disposition: REJECTED as returned (a source line the report does not carry), RAISED under check 5 and
    APPLIED** from the report's Coverage row; routine (Accurate this-chunk addition).
- **T2** · dependent-of T1 · §4 → interpretation crate (the trigger-signal pin's parenthetical)
  - change: the undated "`-p interpretation` 128 pins" retired; the count stated on the new pin's clause: 130 pins at
    this chunk (129 at its base).
  - basis: test-plan:229 · report
  - **disposition: APPLY.** It rests on the report and the doc alone, and it corrected the report: the Counts bullet
    said no master states the count, on a pattern that could not match `128 pins` (check 4 — an absence claim whose
    search could not have found the hit). The report's bullet is corrected. Its primary T1 is applied through
    check 5, so the group stands.
- **T3** · D-tests-coverage · warning · §4 → interpretation crate (the lineage clause)
  - change: the lineage reads `v2.6 / v1.5-fallback / v1.5-reflection` (bumped together at this chunk), the prior
    steps kept as dated.
  - basis as returned: test-plan:229 · `crates/interpretation/src/schema.rs:35,44,59` · report
  - **disposition: REJECTED as returned (the same tell), RAISED under check 5 and APPLIED** from the report.
- **T4** · D-tests-coverage · warning · §1 → `l4-decision-probe-arg-parse-unit-coverage`
  - change: one dated clause appended — 91 collected pins (30 new), `ARMS` 15 → 18 with `ns` / `L` / `LI`, shapes
    S1–S8, the `identifies` label set and its pins, the arm pins, the bar flags, the selection / verdict / guard
    rules and their printed lines, the no-model-text pin, three mutation checks; in STILL OWED the two bar flags
    join the "every flag but" list.
  - basis as returned: test-plan:144 · `pulse-app/examples/l4_decision_probe.rs:160, :2380, :2886-3375` · report
  - **disposition: REJECTED as returned (the same tell), RAISED under check 5 and APPLIED** from the report's
    Symbols / APIs, Counts and Coverage bullets.

## Validate — the six checks over the set

1. **Playbook:** every applied amendment is routine (Accurate this-chunk addition; the 2026-08-23 actual-class rule
   for the four security ones). No boundary widening: the product `TRIGGER:` line is unchanged and the added
   instruction text is static first-party text. No rule collision.
2. **Cross-contradiction:** none. A1 and T3 state the same lineage; A2 and T1 the same obligation; S1/S2 and S3/S4
   edit different clauses of the same two sites.
3. **Intent-consistency:** the report does not diverge from the entry or the plan's acceptance. Its deviations are
   justified; the post-hoc replay attempt and the operator pass rest on the overseer's recorded word (inputs#I10,
   I11). Scope record: none (`gate.py scope` clean, 0 recorded).
4. **Absence needs evidence:** one absence claim of the report was false (the 128-pins site, corrected above). The
   others: "no master states a workspace total" — pattern `\b2[, ]?[5-7]\d\d (tests|passed|pins)|nextest … 2[5-7]\d\d`
   over the masters and registries, 0 hits; the restating sites of the two security claims — `observed maximum`
   2 hits and `are unmeasured` 2 hits, both at security-plan:139 and :463, all four amended. Line profile: every
   site sits on a line over 2,000 chars (arch:73 8,645; security-plan:139 3,953, :463 2,367; test-plan:144 6,340,
   :229 3,950); each was read whole or by offset window before its disposition.
5. **Expected amendments:** all three listed entries are covered — architecture (A1 + A2 + A3), security-plan both
   sites (S1–S4), test-plan §4 and §1 (T1–T4). obs-plan: none expected, none proposed.
6. **Disproved claims:** no spec-master claim. The chunk-artifact premise (the sibling shapes reproduce d3's miss)
   is recorded in the report and `evidence/reading.md`, with no amendment owed (its artifacts are closed), and its
   limit is carried into the architecture body by A3; the open corpus-route hypothesis and the two predictions are
   recorded as read. All DISPOSED.

**Escalations: 0.** Apply set: 11 amendments (architecture 3 · security-plan 4 · test-plan 4).
