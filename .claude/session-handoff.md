# Session Handoff

**Last Updated:** 2026-08-22T18:38:52Z
**Branch:** chore/migrate-pulse-to-v3 (tracks `origin/chore/migrate-pulse-to-v3`; 15 ahead before this commit — unpushed)
**Status:** clean (wrapped)
**Last Commit:** `2026-08-22-pii-scrubber-recall` — a bare provider key is redacted before it reaches stored fields; the counter that makes P-047 gradeable from outside the process rode along

## Position
- Done: `2026-08-22-pii-scrubber-recall` — the catalog's **eighth** category `provider_key` closes the BARE-credential recall gap (the four credential arms before it are key-name-anchored, so a standalone `sk_live_…`-shape token matched nothing). Measured RED before the fix on both routes, GREEN after, on the real OTLP→DuckDB path. CARRY #10 landed: `redactions_applied` on `buffer.tick`.
- Next (first markerless): **log_records identity** — two log records in the same tick at the same severity collide on the PK. Carries the re-pinned `cargo audit` PREREQ (**pin #5**).
- **New entry at tail position 2: "Ingestion scrub coverage"** — see below; it is load-bearing for a spec amendment applied this wrap.

## Work done
7 modified source files + 2 new test files (83 insertions / 11 deletions). The eighth catalog arm + 4 recall cases + 5 false-positive guards; the redaction counter threaded through `state`/`contract`/`appender` (both batch builders) + its `buffer.tick` emit site + its exact allowlist leaf.

**Gates green:** fmt · clippy `-D warnings` · nextest **1833** + 1 skip (1819 → +14) · `-p security` 23/23 (14 → 23) · capability-drift clean · capability-widening 0/3 · **verify:capability-matrix 60/60** · `cargo deny check bans licenses sources` ok · advisories designed-RED at exactly the 8 owned IDs. **Mutation check DISCHARGED** (neutralizing the leaf → 2 of 3 guards RED; restore → 3/3). **Boot smoke PASS** — feed precondition `rows_ingested` 2403 asserted first, 0 panics, 0 ERROR, 12 tick families; `redactions_applied` present UNREDACTED and advancing **0 → 1** after a hand-encoded canary; canary literal 0 times in the log; clean shutdown, both ports released.

## Drift resolved
**13 amendments across 3 masters + 4 leaves · 2 escalations resolved · drift = 0.** obs-plan ×3 (all three sites restating the `buffer.tick` field set — a §8-only apply would have left §1 and §5 stale) · test-plan ×9 (`security` added to all three enumerations it was missing from; new pending trigger; `buffer` bullet qualified; **5 JSON sites corrected** — the receiver is protobuf-only, `application/json` returns 415) · security-plan ×3 (catalog now 8 categories; the falsified `spans` coverage claim applied AS MEASURED with its owner named; 2 same-master duplicates qualified). Cascade: CLAUDE.md `GENERATED:setup:modules`, `rules/observability.md`, `rules/security.md`. Curation: **T1 0 · T2 2 (+2 in-place extensions) · T3 1**, 0 conflicts, 0 deferred.

## Notes

- **Un-scrubbed columns now OWNED.** Five client-controlled DuckDB columns never reach `scrub_attribute` at all — `spans.service_name` · `span_events.name` · `metrics_points.metric_name` · `log_records.severity_text` · `instrumentation_scopes.*`. A COVERAGE gap, distinct from the RECALL gap this chunk closed; the ring buffer is unencrypted, so PII there is stored and read in plaintext. security-plan §Anti-Patterns Logging was amended AS MEASURED (its old wording claimed `spans` was covered — false) and **names the new "Ingestion scrub coverage" route entry as owner**. Operator-decided as a standalone entry, not a sweep CARRY.

- **INTAKE #8 — NOT dispositioned; its premise does not reproduce at HEAD.** The disposition chosen was a deliberate-note, but the item states "the α-ratio equals the relative multiplier at shipped constants, so the `relative_magnitude` label is unreachable" and **there is no `relative_magnitude` label anywhere in the code** — it survives only as a doc-comment word in one test (`crates/triage/src/pattern/suppression.rs:322`) and as an example string in the capability spec. The live enum is `BypassReason::{Relative, Absolute}` (labels `"relative"`/`"absolute"`), and `Relative` is reachable by **two** paths (`suppression.rs:163` and the `:185` inconsistent-caller fallback). The only α at HEAD is the **EWMA smoothing alpha** (`crates/triage/src/baseline/ewma.rs`), a different subsystem from `magnitude_bypass_multiplier` (`crates/triage/src/cue/thresholds.rs`). Writing the note would have injected a false claim about a nonexistent label into a spec. **Needs the intake item re-checked against HEAD (or its real subject identified) before any disposition.**

- **INTAKE #13 — left open deliberately.** Its item text is not recoverable from any Pulse artifact (working route, chunk reports, the observables chunk that minted it, the overseer handoff — the handoff carried only the disposition LEAN, not the item). Dispositioning an item nobody can quote is worse than leaving it open, and a CARRY whose subject is unknown becomes a ghost obligation its absorbing chunk cannot act on. **Its text must be recovered from the operator's own intake record before it is dispositioned.**

- **`cargo audit`:** **probe SKIPPED per ratified interval (next: 34)** — this wrap is session 32. Never silent; recorded in the chunk report and re-pinned in compact form (**pin #5**, origin `2026-08-15-corpus-key-persistence` and chain age preserved). Basis UNCHANGED and upstream. Overlap re-derived and **STABLE at eight** owned upgradeable IDs (0189/0190/0194/0195/0204/0222/0253 + RUSTSEC-2026-0258 `h2` 0.4.14 → ≥0.4.16) — no new finding.

- **Cross-project, RECORD ONLY (no Conductor edit made).** Conductor's `PiiCategory` is a seven-variant enum documented as "the seven P-047 PII categories Pulse's scrubber must detect and redact" — **derived from this catalog**, so its verified P-047 proof is structurally incapable of seeing the class this chunk closed. An eighth category is warranted there, or its claim stays narrower than Pulse's implementation. Its own route owns that work.

- **A real coverage gap in this chunk's own work, recorded not hidden:** the new public `buffer` surface (`record_redactions` + both fields) shipped with **zero buffer-crate tests** — proven cross-crate only. The test-plan detector caught it by arithmetic (1819 → 1833 is fully accounted for by `security` +9 and the two pulse-app files +5, with no `-p buffer` in the gate set). Filed as pending trigger `buffer-redaction-counter-unit-coverage`.

- **Pre-existing doc gaps observed, NOT actioned** (no route obligation placed): `curation`, `interpretation`, `config-watcher` are also absent from test-plan's three crate enumerations (only `security`, this chunk's, was added); and arch's "twelve library crates" prose (`:4`, `:250`, `:333`) disagrees with the fourteen enumerated at `:200` — covered by arch's own "any count word defers to §Occupied Resources" disclaimer.

- **Field naming:** shipped as `redactions_applied`, not the planned `redactions_total` — operator-confirmed to keep, matching its siblings on the same event (`fingerprints_computed` / `observer_invocations` / `span_events_seen`).

- Last failed command: none.
