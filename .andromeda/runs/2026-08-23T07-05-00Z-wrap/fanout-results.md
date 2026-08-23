# Fan-out results — 2026-08-23-ingestion-scrub-coverage

7 Explore doc-agents, one per spec source, run in one parallel batch against
`chunks/2026-08-23-ingestion-scrub-coverage/report.md`.

**Note on raw twins:** the only stripping applied to any return was removal of the fenced ` ```yaml `
wrapper and the agents' trailing evaluation prose. Both the verbatim proposal YAML and those evaluation
notes are preserved below per doc, so this consolidated file IS the audit record for all seven; no separate
`.raw-fanout-{doc}.md` carries content beyond what is reproduced here.

## Verdicts

| doc | detectors run | proposals | severity | disposition |
|---|---|---|---|---|
| arch | D-arch-resources · D-arch-decisions | **2** (1 primary + 1 dependent) | warning | routine-APPLY-AS-MEASURED |
| security-plan | D-security-input · D-security-auth · D-security-deps | **3** (1 primary + 2 dependent) | escalate (detector) → **routine** (playbook) | routine-APPLY |
| design-system | D-design-tokens | 0 | — | clean |
| layout-templates | D-layout-surface | 0 | — | clean |
| test-plan | D-tests-coverage · D-tests-framework · D-tests-obs-harness | **3** (1 primary + 2 dependent) | warning | routine-APPLY |
| obs-plan | D-obs-instrumentation · D-obs-stack · D-obs-pii | **1** | warning | routine-APPLY |
| a11y-plan | D-a11y-surface · D-a11y-obs-schema | 0 | — | clean |

**Total: 9 proposals across 4 masters. 0 escalations.**

## Validation (orchestrator)

1. **Playbook check** — all 9 match an existing routine rule:
   - security (D-security-input, escalate severity) → the **2026-08-14 routine-APPLY** rule. Its
     discriminator is "is there an impl half to fix?" — NO, this chunk shipped it, so the fix completes in
     one artifact. Consistent with the two standing precedents that scope an escalate-severity detector to
     its actual hazard class (2026-06-28 new-env-var; 2026-08-16 D-obs-pii): D-security-input's escalate
     severity exists for ACTUALLY-unvalidated boundaries, and this is a posture flip recording a gap
     CLOSED — the opposite of a hole.
   - arch → the **2026-08-15 routine-APPLY-AS-MEASURED** rule: an impl half exists (retiring two dead
     tables + their dead retention DELETEs) but is too large to ride this chunk, so the amendment records
     measured truth AND names the route entry that owns the fix (the `Diagnostics un-muting +
     harness-truth sweep` CARRY landed at P5).
   - tests + obs → the **2026-07-08 routine-APPLY** rule: accurate this-chunk corrections inside an
     already-documented structure.
2. **Cross-contradiction** — none. The 9 proposals touch 9 distinct sections across 4 masters, all in the
   same direction.
3. **Intent-consistency** — the report matches the chunk's intent. The one divergence (four live columns,
   not five) is JUSTIFIED, so intent was incomplete: it was already amended at /andromeda-phase P3, where
   `scope.md` took a `[premise-corrected]` tag on the write-path premise and a Finding-5 entry on the
   vacuous fifth column. The working-route entry itself is frozen at promotion and correctly left as the
   historical record.
4. **Absence needs evidence** — the producer-absence claims cite the search that establishes them:
   `append_record_batch_to_table` is the only DuckDB write path in `crates/buffer` and is called for
   exactly four tables (`appender.rs:534/557/580/604` + the generic `consumer.rs:182`); re-verified
   first-hand at this wrap, together with the seven retention DELETEs at `retention.rs:23-29`.
5. **Expected-amendments reconciliation** — the plan's list is the coverage floor; all four entries are
   covered, and three of them came in WIDER than predicted:
   - `security-plan` §Logging → proposed, **+2 sites the plan did not name** (lines 48, 165).
   - `test-plan` §1 → proposed, **+2 sites the plan did not name** (line 126 contrast clause, line 332 §4
     prose).
   - `architecture.md` §Occupied Resources → proposed, **+1 site** (§Conventions → Database entity naming).
   - `obs-plan` §5 (+§1/§8) → proposed at **§5 only**. The plan predicted three sites; measured at HEAD,
     `scrub_otlp_field` (the retired scope wording) appears at exactly ONE line (356). Lines 101 (§1) and
     517 (§8) name the field `redactions_applied` without any scope claim, so they are not duplicate
     occurrences. **Orchestrator-verified independently** rather than accepted from the detector — the
     narrower set is correct and the plan over-predicted.
6. **Disproved-claims disposition** — all three report entries end disposed:
   - "five client-controlled columns" → the security-plan proposal set (3 sites).
   - "`spans.service_name` has a single write path" → DISPOSED AT /phase, not here: the claim lives in the
     chunk's own working-route entry CONTEXT, which froze at promotion and is historical by contract; the
     correction was applied to `scope.md` as a `[premise-corrected 2026-08-23: …]` tag. No spec master
     restates it.
   - "`resources` is also never written" → the arch proposal set (2 sites).

---

## arch — 2 proposals

```yaml
proposals:
  - detector: D-arch-resources
    severity: warning
    section: Occupied Resources → DuckDB database / schema names → Reserved tables
    change: Annotate the reserved-table list so `resources` and `instrumentation_scopes` read as declared-and-retention-swept but PRODUCER-LESS — declared in `crates/buffer/src/schema.rs`, swept by `crates/buffer/src/retention.rs`, never written (measured 2026-08-23), so two of the seven retention DELETEs sweep permanently-empty tables and neither table can leak nor be scrubbed; the four live client-controlled write targets are `spans`, `metrics_points`, `log_records`, `span_events`.
    sidecar: "2026-08-23 — Occupied Resources: `resources` + `instrumentation_scopes` recorded as declared/retention-swept but never written (no producer); reserved-table enumeration no longer implies a live write path (chunk 2026-08-23-ingestion-scrub-coverage)."
    rationale: Report §Changes → "Spec claims disproved by measurement" #1 and #3 measure that `append_record_batch_to_table` fires for exactly four tables, that `instrumentation_scopes` has no producer, and that `resources` is declared/swept/never-written; the report names arch §Occupied Resources as the unqualified site that "should say it".
  - detector: D-arch-resources
    severity: warning
    section: Conventions → Database entity naming
    change: The inline reserved-set restatement must not read as seven live tables — mark `resources` and `instrumentation_scopes` as producer-less in the parenthetical, and qualify the periodic `DELETE FROM <table>` sweep clause as covering two permanently-empty tables.
    sidecar: "2026-08-23 — Conventions → Database entity naming: inline reserved-set restatement + retention-sweep clause aligned with the Occupied Resources producer-less qualification."
    rationale: Second occurrence of the retired wording — line 84 restates the seven-table set unqualified AND asserts the periodic DELETE cutoff task, so a single-site apply would leave arch claiming both dead tables as ordinary swept entities.
    dependent-of: D-arch-resources
```

**D-arch-decisions — no drift.** Dependencies "none added, none bumped"; crates none added/removed; the
changed symbols are `pub(crate)` DuckDB/Arrow internals consistent with §Stack and §Established Decisions;
the new example injects over OTLP `:4317`/`:4318`, matching §Cross-cutting Patterns "Test-time telemetry
injection" rather than contradicting it.

---

## security-plan — 3 proposals

```yaml
proposals:
  - detector: D-security-input
    severity: escalate
    section: "Security Anti-Patterns → Logging (MEASURED reality clause, line 427)"
    change: "Flip the MEASURED-reality clause from 'does NOT yet hold for five columns' to holding for the four live client-controlled columns — `spans.service_name`, `span_events.name`, `metrics_points.metric_name`, `log_records.severity_text` now pass through `scrub_attribute` (retiring both the five-count and 'The `spans` table receives NO scrub at all') — and enumerate the residue as four-live-plus-one-vacuous with `instrumentation_scopes` recorded as having no producer (measured 2026-08-23) and therefore unable to leak; mark the owning working-route entry closed, and keep unchanged both the 8-category catalog and the fact that the ring buffer remains unencrypted."
    sidecar: "2026-08-23-ingestion-scrub-coverage — §Security Anti-Patterns → Logging: DuckDB ring-buffer scrub coverage flipped from an open five-column gap to covered (four live columns scrubbed at the write boundary; `instrumentation_scopes` dispositioned documented-vacuous, zero code)."
    rationale: "Report Changes → 'Counts / qualifiers moved: YES' names security-plan.md:427 as the five-count site, and 'Spec claims disproved by measurement' #1 measures it FALSE; Coverage marks all four write boundaries PII redacted✓ with unit+wire tests, and the smoke records `redactions_applied` 0 → 4 with the canary literal and bare `sk_live_` at 0 occurrences across 21,846 log lines."
  - detector: D-security-input
    severity: escalate
    section: "Threat Model Summary → Data classification → user-content, Sensitivity note (line 48)"
    change: "Retire the trailing cross-reference 'see §Security Anti-Patterns → Logging for the five DuckDB ring-buffer columns that are NOT, measured 2026-08-22' — restate as the now-covered ring-buffer posture plus the producer-less `instrumentation_scopes`."
    sidecar: "2026-08-23-ingestion-scrub-coverage — §Threat Model Summary corpus sensitivity note: five-column DuckDB gap cross-reference updated to the covered posture."
    rationale: "Duplicate-occurrence of the wording retired by the primary; the report listed only line 427, so this restatement would otherwise survive a single-site apply and keep a measurably-false claim in the threat model."
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "Data Protection → At rest — per medium → Persistent incident corpus bullet (line 165)"
    change: "Retire 'the DuckDB ring buffer is a separate substrate with a measured five-column gap' and restate: the ring buffer is a separate unencrypted substrate whose four live client-controlled columns now scrub pre-write (measured 2026-08-23), with `instrumentation_scopes` producer-less — preserving the unencrypted-substrate fact, which this chunk does not change."
    sidecar: "2026-08-23-ingestion-scrub-coverage — §Data Protection at-rest corpus bullet: 'measured five-column gap' phrasing replaced by the covered four-column posture."
    rationale: "Duplicate-occurrence; the report did not list this site, and leaving it would contradict the flipped §Logging clause it explicitly cross-references."
    dependent-of: D-security-input
```

**D-security-auth — clean** (no identity/session/token/key path touched). **D-security-deps — clean**
(no dependency added or bumped).

---

## design-system — 0 proposals

`proposals: []`. Every Coverage row carries `tokens n/a`; no `hardcoded✗` anywhere; Files lists only Rust
paths. §Color Palette / §Spacing / §Typography need no amendment.

---

## layout-templates — 0 proposals

`proposals: []`. No user-facing surface or region added — all four touched paths are Rust backend/dev-tool,
all changed signatures are `pub(crate)`, every Coverage row carries `a11y n/a · tokens n/a`, and the report
confirms zero `pulse-app/ui/**` paths touched. Nothing to map to a §Wireframe entry.

---

## test-plan — 3 proposals

```yaml
proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Pending coverage triggers → `buffer-redaction-counter-unit-coverage` row"
    change: Mark the row LANDED, no longer pending (retained for audit trail per the `capability-widening-check` row precedent) — the once-per-batch fold arithmetic now has an in-crate assertion, `redaction_counter_folds_once_per_batch_across_all_four_builders` in `crates/buffer/src/appender.rs`, pinning the fold arithmetic, that ALL FOUR appender builders contribute to one tally, and that the fold stays separate from `record_feed_counts`; drop the "proven cross-crate only" / "stays OPEN" clauses.
    sidecar: "2026-08-23-ingestion-scrub-coverage — `buffer-redaction-counter-unit-coverage` CLOSED: in-crate fold assertion landed; scope widened from two batch builders to all four appender builders."
    rationale: "Report Outcome names the new pin and the mutation check shows it going RED when the four scrub sites are neutralized (so it discriminates); Expected amendments explicitly lists this closure. The row's current text ('no test inside the `buffer` crate', 'proven cross-crate only', 'stays OPEN', 'BOTH batch builders') is now false at HEAD."
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Pending coverage triggers → `buffer-log-seq-allocator-unit-coverage` row (contrast clause)"
    change: Rewrite the closing contrast so it no longer asserts the retired claim — reference the redaction-counter trigger as CLOSED, leaving the allocator as the sole remaining in-crate `buffer` gap.
    sidecar: "2026-08-23-ingestion-scrub-coverage — seq-allocator trigger contrast clause updated; the allocator is now the sole open in-crate `buffer` gap."
    rationale: "This row restates the retired 'counter proven cross-crate only' wording verbatim, so a single-site apply would leave the false claim alive here."
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy → buffer crate bullet"
    change: "'TWO in-crate gaps remain' becomes ONE — delete the redaction-counter clause and list the covered surface (fold across all four appender builders, in-crate since this chunk), leaving `BufferState::reserve_log_seq_block` as the only remaining gap."
    sidecar: "2026-08-23-ingestion-scrub-coverage — §4 buffer-crate coverage prose: in-crate gap count TWO → ONE."
    rationale: "§4 duplicates the §1 claim in prose including the now-retired two-builder framing; without this edit the gap survives at a second site and the count is wrong."
    dependent-of: D-tests-coverage
```

**D-tests-framework — clean** (every runner matches §2/§4: nextest, rstest, the §3 standard gate set;
the Direct-binary smoke is the form §3 sanctions by name; the `cargo build … --jobs 4` prefix is a build
step, not a runner swap). **D-tests-obs-harness — clean** (no harness, status-endpoint or log-format
change; the removed `SCENARIO=` usage deleted a non-existent harness usage rather than changing the
harness).

---

## obs-plan — 1 proposal

```yaml
proposals:
  - detector: D-obs-instrumentation
    severity: warning
    section: "§5 Metric Coverage — Conceptual instrument types table, the `redactions_applied` row"
    change: "Widen the registered scope from the OTLP persistence path (`scrub_otlp_field`: log body + exception message + exception stacktrace) to all four `buffer::appender` record-batch builders — the `spans.service_name`, `span_events.name`, `metrics_points.metric_name` and `log_records.severity_text` write boundaries — still folded once per batch via `BufferState::record_redactions`, and state that the counter counts redactions to PERSISTED cells only, so a scrub applied purely for identity consistency on a non-stored path is deliberately uncounted (field set and label posture unchanged)."
    sidecar: "obs-plan §5: `redactions_applied` scope widened from `scrub_otlp_field` log/exception paths to all four appender builders; persisted-cells-only counting rule recorded (chunk 2026-08-23-ingestion-scrub-coverage)."
    rationale: "Report Changes → Symbols: both spans and metrics builders gained `state: &BufferState` and `extract_service_name` now scrubs its return; Coverage records `instrumentation ✓ (redactions_applied folded once per batch)` for all four write boundaries. §5 today registers the counter as scoped to `scrub_otlp_field` only. The persisted-cells-only rule is why the wire smoke reads 0 → 4 rather than 5 (the two non-storing callers pass `None`). No other obs-plan site restates the retired scope wording — §1 line 101 and §8 line 517 name the field only."
```

**D-obs-stack — clean** (no dependency added; no telemetry symbol outside the §3 `tracing` harness; no OTel
SDK). **D-obs-pii — clean** (aggregate count only, no new tick field, no label; the smoke measured the
canary literal and the bare `sk_live_` format at 0 occurrences across 21,846 log lines).

---

## a11y-plan — 0 proposals

`proposals: []`. **D-a11y-surface** — no interactive UI element added; every Coverage row carries
`a11y n/a`; zero `pulse-app/ui/**` paths. **D-a11y-obs-schema** — Schema/config is "none … no
violation-schema change"; the obs amendment is §5 (instrumentation registry), not §6 (structured-log
schema), so a11y-plan's four bindings to obs §6 remain consistent.
