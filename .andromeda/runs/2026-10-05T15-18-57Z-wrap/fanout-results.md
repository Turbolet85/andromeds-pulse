# Fan-out results — 2026-10-05-l4-runs-the-founder-s-pick-with-its-authors-settings

Seven Explore doc-agents, one parallel batch, verbatim prompt (contracts line dropped: `registry.py contracts` → NOT
MIGRATED for arch · tests · obs · a11y). Detector counts per prompt: arch 2 · security 4 · design 2 · layout 2 ·
tests 3 · obs 4 · a11y 2 = 19 = the drift-base's `doc:` names. No return needed stripping beyond its trailing
`#` commentary (kept as the doc's verdict note below); no entity was HTML-escaped.

## Verdicts
- design-system — `proposals: []` (D-design-tokens: every Coverage row `tokens n/a`; D-design-status-narrative: no
  status claim the report touches).
- layout-templates — `proposals: []` (no UI surface; no status claim the report touches).
- a11y-plan — `proposals: []` (no interactive element; the obs envelope unchanged).
- obs-plan — 3 proposals (D-obs-defect-narrative ×3).
- test-plan — 6 proposals (D-tests-coverage ×4, D-tests-framework ×2); D-tests-obs-harness no drift.
- architecture — 10 proposals (D-arch-decisions ×7, D-arch-resources ×3).
- security-plan — 5 proposals (D-security-deps ×1, D-security-logging ×4); D-security-input / D-security-auth no drift.

## Proposals and dispositions

### architecture
| # | detector · section | change (short) | disposition |
|---|---|---|---|
| A1 | D-arch-decisions · §Established Decisions [LLM Inference Runtime] | constraint `--json-schema-file` → committed GBNF via `--grammar-file`; `grammar_for_schema`; `GrammarTempFile`; the authors' sampling after `-n`; `-p` last; the prefill trap kept as the reason | ESCALATE — Boundary widening (playbook `Boundary widening`, never routine): the subprocess boundary gains the `--grammar-file` crossing and four sampling operands |
| A2 | dependent-of A1 · §Stack AI/ML row | the same mechanism + sampling | rides A1 (atomic group) |
| A3 | dependent-of A1 · §Inherited Defaults LLM line | the same | rides A1 |
| A4 | D-arch-decisions · [LLM Inference Runtime] Model clause | shipped GGUF = the founder's pick `gemma-4-E4B-it-Q4_K_M` (unsloth); the "until it lands, Llama-3.2-3B" clause retired | apply — `Accurate this-chunk addition` (routine); also plan Expected amendment 1 |
| A5 | dependent-of A4 · [Fault Identity] closing clause | "shipped model stays Llama-3.2-3B / owed to the next entry" retired; the historical series kept | apply (with A4) |
| A6 | dependent-of · [LLM Inference Runtime] gpu-primary comparison | budget 5000 → 10000 ms p99 nearest-rank; live p99 6716 ms (n = 7) | apply — Accurate this-chunk addition; plan Expected amendment 1 |
| A7 | D-arch-decisions · §Stack | a test-only row: vendored llama.cpp b9305 converter (MIT, stdlib Python 3) + the test-time Python 3 interpreter | apply — Accurate this-chunk addition (a new test-time runtime is a §Stack fact); text re-derived from the report's Dependencies |
| A8 | D-arch-resources · §Occupied Resources → Filesystem locations | register the per-spawn L4 grammar temp file in `std::env::temp_dir()` (since chunk #84) as the THIRD out-of-data-dir product write | apply — `Accurate this-chunk addition` + the PRE-EXISTING-reality correction rule (impl correct, doc alone wrong); plan Expected amendment 3; report Spec claim disproved 1 |
| A9 | dependent-of A8 · export-sink bullet | "one of TWO exceptions" → three | apply (with A8) |
| A10 | dependent-of A8 · lock-file bullet | "the SECOND exception" → one of three (the temp file predates it) | apply (with A8) |

Note: A1, A4 and A6 carry `basis:` file:line in architecture.md itself (the doc under check), not a source location —
not the re-derivation tell.

### security-plan
| # | detector · section | change (short) | disposition |
|---|---|---|---|
| S1 | D-security-deps (escalate) · §Dependency Security | record the vendored, test-only, MIT, hash-pinned converter + the test-time Python 3 requirement; outside cargo-deny / audit / Dependabot / the npm gate | apply — no playbook match; the operator's RECORDED direction settles it (plan Expected amendment 6 names the change; overseer P4 fork 1 "record the MIT provenance next to the vendored file"); its class guard (an unvetted dep shipping) does not hold: test-only, never shipped, hash- and license-pinned. Still surfaced at the HALT card for visibility |
| S2 | D-security-logging · §Data Protection → At rest (lock-file bullet, :167) | "the second product-written artifact outside the data dir" → one of three (the L4 grammar temp file) | apply — PRE-EXISTING-reality correction (impl correct, doc wrong); Spec claim disproved 1 |
| S3 | dependent-of S2 · §Security Anti-Patterns → Input (:393) | "one of the two product-written locations" → three | apply (with S2) |
| S4 | D-security-logging · §Security Anti-Patterns → Logging (full-path never-log bullet) | the grammar temp path joins the enumerated "Covers" set; measured 0 in the GREEN log | apply — Accurate this-chunk addition (the file was renamed this chunk; GREEN-log measurement in the report) |
| S5 | dependent-of S4 · §Logging & Monitoring → What NEVER to log | the same enumeration | apply (with S4) |
| S6 (raised, check 5) | orchestrator · §Input Validation L4 argv row (:139) + §Security Anti-Patterns → Code Patterns `-p` bullet (:461) | the argv gains the first-party `--grammar-file {temp path}` and four first-party sampling operands; `-p` stays the ONE OTLP-derived operand | ESCALATE — Boundary widening (plan Expected amendment 4 names the pattern itself); D-security-input found the invariant holding, the record is what is owed |

### test-plan
| # | detector · section | disposition |
|---|---|---|
| T1 | D-tests-coverage · §1 `l4-decision-probe-arg-parse-unit-coverage` (gb / --gbnf retired, ARMS 16→15, 61 pins, replace-or-append pinned, `--gbnf` dropped from STILL OWED) | apply — Accurate this-chunk addition; Expected amendment 9 |
| T2 | D-tests-coverage · §4 Test file location (Rust) 101 → 102 | apply — Expected amendment 10 |
| T3 | D-tests-coverage · §1 `llamacli-inference-error-emission-coverage` gains the `grammar_schema_mismatch` WARN (emission not asserted; only the helper's arms pinned) | apply — accurate: the report's Coverage pins only `grammar_for_schema`'s return arms |
| T4 | D-tests-coverage · §1 new row `l4-latency-p99-ps1-run-coverage` (the `.ps1` never run; owed over the three fixtures) | apply — carries the overseer's owed item 1 to an owned pending row |
| T5 | D-tests-framework · §9 `lint-test` row: Python 3 on all three runners (`unit_l4_grammar`; fails, never skips; `ci#37327846820`) | apply — Expected amendment 11 |
| T6 | D-tests-framework · §4 Framework (Rust crates): the one non-Rust test-time runtime | apply — Expected amendment 11 (§2/§9 named; §4's framework line is the restating site) |

### obs-plan
| # | detector · section | disposition |
|---|---|---|
| O1 | D-obs-defect-narrative · §10 Performance budgets: an L4 gpu-primary row | REJECTED as written (re-derivation tell: `basis` cites `xtask/ci/l4-latency-p99.sh:40` and `inference_runtime.rs:65`, source locations the report does not carry) → RE-RAISED by the orchestrator under check 5 (Expected amendment 7) from the report's facts: routine, apply |
| O2 | D-obs-defect-narrative · §8 muted-diagnostic backlog re-opened (`interpretation.constrained.generate` partly redacted) | REJECTED as written (re-derivation tell: `basis` cites `llamacli_inference.rs:974-979`, `observability.rs:2131`) → RE-RAISED by the orchestrator under check 6 from the report's Coverage row: routine (PRE-EXISTING-reality correction — the §8 "ZERO / no owner remains" claim is falsified). The OWNER pointer is P5's (overseer directive: ask at the route resolve); the body names it after P5, before the sidecar entries land |
| O3 | dependent-of O2 · §8 closing paragraph | re-raised with O2 |

## Validate checks
1. Playbook — A1/A2/A3 + S6 → `Boundary widening` (escalate). S1 → no match; settled by the plan's recorded
   direction (shown on the card anyway). All other applies → `Accurate this-chunk addition` or the PRE-EXISTING-reality
   correction rule (`Drift proposal ACCURATELY correcting a doc claim that a PRE-EXISTING reality falsifies`).
2. Cross-contradiction — none: A8–A10 (arch) and S2/S3 (security) state the same three-location count.
3. Intent-consistency — the report's deviations (replace-or-append generalised, a third probe test re-pointed, the
   vendored README copied) are within the intent; scope record: none (gate.py scope clean, 0 recorded).
4. Absence-needs-evidence — O2's "the sweep guards did not catch it" is the agent's inference; the applied text states
   only what the report measured (7 records, 14 fields, 0 in the RED log), not a guard-coverage claim.
5. Expected amendments — all 11 entries covered: 1 (A1/A4/A6), 2 (A2/A3), 3 (A8–A10), 4 (S6, raised), 5 (S2/S3),
   6 (S1), 7 (O1, raised), 8 (obs §8 `grammar_schema_mismatch` — not carried: obs-plan enumerates no
   `interpretation.inference.error` categories, `grep -c` → 0; the latency leaf's field set is unchanged), 9 (T1),
   10 (T2), 11 (T5/T6).
6. Disproved claims — 1 → A8–A10 + S2/S3; 2 → O1 (the vacuity's fix recorded as the §10 row); 3 → O1.

## Escalations (HALT)
- E1 — Boundary widening: A1 (+A2/A3) and S6 — the L4 subprocess argv gains `--grammar-file {per-spawn temp path}` and
  four first-party sampling operands. RESOLVED: ratified by the founder's own word, live 2026-10-05 ~17:45, relayed
  verbatim by the overseer — asked «Утверждаете это расширение целиком?» (grammar file + `--temp 1.0 --top-p 0.95
  --top-k 64 --min-p 0`), answered «Утверждаю целиком» — beside inputs#I1 (the phase-time ruling 1). Covers the
  grammar crossing AND the four sampling operands. A1–A3 and S6 → apply.
- S1 (shown for visibility) — RESOLVED: apply as directed; the overseer (founder-delegated): "the hash pin and the
  LICENSE beside it are the vetting; a CI sha256 step is not owed now."
