# Codebase Research — 2026-08-22-pii-scrubber-recall

## Scope
- **Depth:** moderate-deep · **Reads:** 9 files (2 full, 7 targeted ranges/greps) · **Globs/Greps:** 12 · **Graph queries:** 1 (rust plane, `db_state: regenerated`, 26 rows)

## Files inspected
- `crates/security/src/scrubber.rs` (full, 180 lines) — the catalog, its entry point, and both test corpora. The module header states the governing posture and the `patterns()` comment restates it.
- `crates/buffer/src/appender.rs` (`:107`, `:275–310`, `:355–415`) — the DuckDB persistence path: every `scrub_otlp_field` call site, the `exception_type`-stays-raw decision, and the `record_feed_counts` tick fold.
- `crates/buffer/src/schema.rs` (grep, both schema copies) — every stored `VARCHAR` column; establishes which fields can carry a credential at all.
- `pulse-app/tests/e2e_pii_scrubber_persistence_coverage.rs` (grep, imports + all 5 test fns) — the legacy-ledger-referenced P-047 e2e file; its actual shape.
- `xtask/src/main.rs` (`:140–143`, `:624–674`, `:814`) — what `verify:capability-matrix` reads and asserts.
- `pulse-app/src/observability.rs` (`:176–190`, `:3858–3872`) — the `buffer` allowlist leaf and its existing guard.
- `.andromeda/obs-plan.md` (`:543–544`) — the §8 UI-vocabulary exemption + secret-format regex family.
- `.andromeda/test-plan.md` (`:136`, `:307`) — the two crate enumerations the tests extract predicted a gap in.
- `crates/buffer/src/state.rs` (`:21–80`) + `crates/buffer/src/contract.rs` (`:47–69`) — the tick-counter field plumbing.

## Graph impact (rust plane; 26 rows, `db_state: regenerated`)
- **`scrub_attribute`** — 13 distinct call sites (each row-paired with its `use` import). Production consumers:
  `crates/buffer/src/appender.rs:303` (`scrub_otlp_field`) · `crates/buffer/src/drain.rs:374` (`snapshot_state`), `:634` (`write_template_to_table`) · `crates/interpretation/src/markdown.rs:336` (`scrub_string`) · `pulse-app/src/baseline_persistence.rs:108` · `digest_runtime.rs:54` · `inference_runtime.rs:537`, `:560` · `investigate_router.rs:113` · `lifecycle_persistence.rs:102` · `storm_persistence.rs:87` · `training_export.rs:85`. Test consumers: `scrubber.rs:130/144/172`, `pulse-app/tests/unit_deterministic_inference.rs:95`.
- **Meaning for the change:** the catalog is reached through ONE function from twelve production sites. Widening the catalog changes behaviour at every one of them simultaneously — including `training_export` (the `~/Downloads` egress sink) and `investigate_router`. A recall arm is therefore additive-but-global; the false-positive corpus is what bounds the blast radius.

## Patterns detected
- **Recall-over-precision is the catalog's stated design posture** (`crates/security/src/scrubber.rs:55–58`): "Patterns chosen for recall over precision — false positives are acceptable (over-redaction); false negatives leak secrets." The existing `credit_card` arm (`:97`, `\b(?:\d[ \-]?){13,19}\b`) is that posture in practice — it matches any 13–19 digit run and does not check Luhn.
- **First-match-wins, ordered catalog** (`:38–48`): `scrub_attribute` returns on the first matching category, so a new arm's POSITION decides its category label when patterns overlap.
- **`ScrubbedValue::Redacted` carries only a `&'static str` category — never value content** (`:12–21`), enforced by a proptest invariant (`:170–179`). Any new arm inherits this by construction.
- **Tick-aggregated counter fold** (`appender.rs:392–413` → `state.rs:21/80` `AtomicU64` → `contract.rs:47/69` snapshot): the shipped template for a hot-path-safe count, exactly as the obs extract described.
- **`[REDACTED:{category}]` stable-marker rendering** (`appender.rs:303–310`, mirroring `drain.rs:600–603`): the persistence-side convention for what replaces a redacted value.

## Conventions to follow
- **pulse-app tests live in `pulse-app/tests/*.rs`** — `[lib] test = false` (test-plan.md `:307`); a co-located `mod tests` there would compile and never run.
- **`exception_type` is deliberately NOT scrubbed** (`appender.rs:365–367`): "class identifier, not user content." This is a documented decision, not a coverage gap — do not "fix" it.
- **Scrub renders to a marker, never drops the field** (`appender.rs:303–310`).

## The claim's actual surface (this is what makes the measurement designable)
The DuckDB schema has **no generic `attributes` column** — OTLP span attributes are not persisted wholesale. Every client-controlled string that reaches DuckDB is one of these columns (`crates/buffer/src/schema.rs`, both copies):

| Column | Scrubbed? | Path |
|---|---|---|
| `log_records.body` | **yes** | `extract_log_body` → `scrub_otlp_field` (`appender.rs:291`) |
| `span_events.exception_message` / `.exception_stacktrace` | **yes** | `appender.rs:368–369` |
| `log_records.exception_message` / `.exception_stacktrace` | **yes** | same helper |
| `log_templates.template_content` | **yes** | `drain.rs:634` |
| `span_events.exception_type` | no — **documented decision** | `appender.rs:365–367` |
| `spans.service_name` (+ `span_events.name`, `metrics_points.metric_name`, `log_records.severity_text`, `instrumentation_scopes.scope_name`/`scope_version`) | no | `extract_service_name` (`appender.rs:107`) and siblings — never reach `scrub_attribute` |

**Consequence:** the chunk's claim is precisely testable and narrow. A bare provider key placed in a **log body** or an **exception message** DOES pass through `scrub_attribute` — so if it lands in DuckDB verbatim, the defect is unambiguously the CATALOG's recall, with the boundary coverage already proven. That is the canary route the measurement should take.

**Separate observation (record, do not action):** the unscrubbed client-controlled columns in the last row above are a *coverage* question, not a recall question — a different defect class from this chunk's claim, and outside its stated boundaries. `service_name` is the notable one (it is client-supplied and reaches several persisted surfaces).

## Live gate discovered — `verify:capability-matrix` DOES read the legacy ledger
`xtask/src/main.rs:637` joins `docs/v0_2_0/capability-verification-matrix.json` and asserts (`:141`) "all 60 P-001..P-060 ids present exactly once, every scenario file ref exists, every `contains` anchor greps non-empty." The P-047 row refs two files with `contains: "P-047"`:
- `crates/security/src/scrubber.rs` — anchor present twice (`:3` "the 7 P-047 categories", `:38` "P-047 pattern catalog")
- `pulse-app/tests/e2e_pii_scrubber_persistence_coverage.rs` — anchor present once (`:6`)

**Binding constraints for the plan:** (a) neither file may be renamed or moved without updating the ledger; (b) the literal string `P-047` must survive every edit — note that `:3` reads "the **7** P-047 categories", so an eighth arm makes that sentence stale while its anchor still greps; (c) the gate is 60/60 today and is a per-PR CI step, so it is a real pass/fail, not a dormant artifact.

## The existing P-047 e2e file is NOT the home the scope assumed
`pulse-app/tests/e2e_pii_scrubber_persistence_coverage.rs` is **in-process and corpus-oriented**: 5 tests over the four CORPUS persistence adapters (`drain_persistence`, `lifecycle_persistence` ×2, `storm_persistence`, `baseline_persistence`), asserting `[REDACTED:email]` markers inside corpus bincode bytes. It never speaks OTLP, never touches DuckDB, and uses `email` as its canary category throughout. It is a legitimate sibling but **not** an end-to-end OTLP→DuckDB harness — so the stored-field leg needs a different (probably new) home, and moving/renaming this file would break the ledger gate above.

## Files to modify
- `crates/security/src/scrubber.rs` — the catalog arm + its `#[rstest]` recall case + false-positive guard cases. The header comment at `:3` ("the 7 P-047 categories") needs its count updated while keeping the `P-047` anchor.
- *(conditional, if CARRY #10 lands)* `crates/buffer/src/state.rs` + `crates/buffer/src/contract.rs` + `crates/buffer/src/appender.rs` — the counter field, following `record_feed_counts`; and `pulse-app/src/observability.rs:176` — the `buffer` allowlist leaf, which today enumerates no redaction field.
- *(conditional)* `docs/v0_2_0/capability-verification-matrix.json` — only if a new test file must be referenced; otherwise untouched.

## New files to create
- `pulse-app/tests/{name}.rs` — the OTLP→DuckDB bare-key stored-field measurement (new file; the existing P-047 e2e file is corpus-shaped and ledger-pinned, so extending it in place would conflate two harnesses).

## Expected amendments (wrap-owned, not touchpoints)
- **test-plan.md** — `security` is absent from all three crate enumerations (`grep -c 'crates/security'` = **0**; the §2 pyramid Unit row at `:136` and the §4 test-location convention at `:307` both list "ingest, buffer, viz, ui-bridge, snapshot, workspace-detector, plugins, mcp-server, corpus, triage" without it). The tests extract predicted exactly this; it is the same routine-APPLY shape as the `corpus` and `triage` amendments.
- **security-plan.md §Security Anti-Patterns → Logging** — no amendment has ever enumerated a scrubber *pattern*; if an eighth arm lands, the coverage paragraph is the site of record.
- **obs-plan.md §5/§8** — only if the CARRY #10 counter lands.

## Scope premise closure
Both `[inferred]` bullets in `scope.md` are **VERIFIED** and their tags dropped (scope.md amended):
1. *Anchored-prefix lean* — **VERIFIED** twice over: `obs-plan.md:543–544` names the exact family (`sk-[a-zA-Z0-9]{40,}`, `ghp_[a-zA-Z0-9]{36}`, `AKIA[0-9A-Z]{16}`, `Bearer [a-zA-Z0-9]{40,}`, `password=[^\s]+`) and warns that over-broad patterns false-positive; and `scrubber.rs:55–58` independently declares recall-over-precision as the catalog's posture. **Honest nuance recorded:** the obs §8 passage was written for *grep-based CI heuristics* (the a11y gate), not the runtime catalog — so it is a strong analogical lean plus a named shape vocabulary, not a direct mandate on `scrub_attribute`.
2. *CARRY #10 pairs with the measurement leg* — **VERIFIED**: `record_feed_counts` (`appender.rs:411`) sits in the same file and flow as the scrub call sites, and the `buffer` allowlist leaf (`observability.rs:176`) exists but enumerates no redaction field, so the fold point and the registration point are both identified and adjacent.

Additionally **sharpened** (not falsified): scope described the boundary as "stored fields" generally; research narrows it to the exact column table above, and corrects the call-site coordinate — the helper is `appender.rs:303`, invoked at `:291`, `:368`, `:369`.

## Open questions
- Should the CARRY #10 redaction counter land in THIS chunk or be recorded as a pending trigger? → blocks: **plan-decision** (P4 resolves before synthesis; the scope explicitly leaves it to the plan).
- Does the measurement leg ride a committed test only, or a committed test PLUS a direct-binary smoke run? → blocks: **plan-decision** (test-plan §3 makes boot-smoke conditional and this chunk's paths are not on its trigger list, so a committed integration test is the default).
