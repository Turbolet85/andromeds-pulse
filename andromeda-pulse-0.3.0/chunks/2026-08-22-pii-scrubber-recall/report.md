# Report — 2026-08-22-pii-scrubber-recall

**Chunk:** PII scrubber recall — a bare provider key in telemetry is redacted before it reaches stored fields, not only its keyed form (P-047); measure-first, end-to-end to the stored field
**Date:** 2026-08-22
**Commits:** (uncommitted at authoring; prior since last_wrap: `chore(setup-project): absorb the per-plane code-graph template; retire dead pointers` · `feat(2026-08-21-delegated-timing-observables): three delegated timing bounds became measurable at the wire`)

## Changes (structured — detectors read this)

- **Files:** **7 modified source files** + **2 new test files** (+ route/bookkeeping).
  Modified: `crates/security/src/scrubber.rs` · `crates/buffer/src/appender.rs` · `crates/buffer/src/state.rs` · `crates/buffer/src/contract.rs` · `crates/buffer/src/consumer.rs` · `pulse-app/src/heartbeat.rs` · `pulse-app/src/observability.rs`.
  New: `pulse-app/tests/e2e_pii_bare_credential_stored_field.rs` · `pulse-app/tests/unit_observability_allowlist_redaction_counter.rs`.
  Diffstat: 83 insertions, 11 deletions across the 7 modified.

- **Symbols / APIs:**
  - `security::scrubber` pattern catalog gains an **eighth arm**, category label **`provider_key`** — anchored-prefix + length-floor alternation over published issuer prefixes (`[sr]k_(live|test)_` · `sk-` · `gh[pousr]_` · `AKIA` · `xox[baprs]-` · `AIza`). Ordered AFTER the keyed arms (`api_key` / `secret_kv` keep their categories for keyed forms) and BEFORE `credit_card` (so a digit-heavy key is not mis-labelled). `scrub_attribute` signature UNCHANGED; `ScrubbedValue` contract UNCHANGED.
  - `buffer::BufferState::record_redactions(&self, n: u64)` — **NEW pub fn**. Deliberately separate from `record_feed_counts` because the log-record path applies redactions while producing no fingerprint-feed counts.
  - `buffer::BufferStateSnapshot.redactions_applied: u64` — **NEW pub field**.
  - `buffer::contract::BufferHeartbeat.redactions_applied: u64` — **NEW pub field**, populated in `heartbeat_payload`.
  - `buffer::appender::build_logs_record_batch` — **signature widened** with a third param `state: &BufferState`. `pub(crate)`, so the change is crate-internal. **Remaining-caller fact: 3 call sites existed and ALL 3 were updated** — `crates/buffer/src/consumer.rs:139` (the production path, now passes the live `state`), `crates/buffer/src/appender.rs:564` (a `#[cfg(test)]` helper), `crates/buffer/src/appender.rs:1172` (a colocated test). No caller outside the `buffer` crate — verified by the code-graph impact query at /phase (26 rows) plus this chunk's compile.
  - `buffer::appender::scrub_otlp_field` / `extract_log_body` — private fns, each gains a `&mut u64` redaction-tally out-param. 3 internal call sites (`appender.rs:291` log body, `:368`/`:369` exception message + stacktrace), all updated.
  - **No TauRPC procedure added or changed** — `EXPECTED_PROCEDURES` untouched; `capability-drift` clean.

- **Crates / modules:** none added, none removed. Changed: `security` (catalog), `buffer` (counter plumbing), `pulse-app` (emit site + allowlist).

- **Dependencies:** **none added, none bumped.** No new workspace dep; the arm is a `regex` pattern in the existing `OnceLock` catalog.

- **Schema / config:** **no DuckDB schema change, no corpus DDL, no `SCHEMA_VERSION` bump, no config keys, no env vars.** The counter is in-memory `AtomicU64` state surfaced on an existing heartbeat event.

- **Spec-master edits:** none at authoring (P2 reconcile owns any).

- **Counts / qualifiers moved:** **YES — the P-047 category count moved 7 → 8.**
  - `crates/security/src/scrubber.rs:3` module doc updated in-chunk ("the 8 P-047 categories"), with the literal `P-047` grep anchor preserved (the legacy-ledger gate depends on it).
  - **`CLAUDE.md:32` still states "7 P-047 categories (JWT / bearer token / API key / secret KV / email / credit card / SSN)"** — a GENERATED distillation, so it moves via the P2 cascade, not a body edit.
  - **No `.andromeda/` spec master states the count** (verified by grep across all seven). Hits in `docs/v0_2_0/pulse-v0_2_0-{route,capability-audit-2026-06-12,consolidation-audit-2026-05-19}.md` are **dated historical audit records** — correct as of their dates; NOT living specs, do not rewrite.

- **Dev-tool versions:** none.

- **Reverted / negative API facts:** none shipped-then-reverted. (A deliberate temporary mutation — removing the `redactions_applied` allowlist entry — was applied and restored as the mutation check; see Outcome.)

- **Spec claims disproved by measurement:**
  1. **Five client-controlled columns never reach `scrub_attribute` at all** — `spans.service_name`, `span_events.name`, `metrics_points.metric_name`, `log_records.severity_text`, `instrumentation_scopes.scope_name`/`scope_version`. Measured at source: `appender.rs:45` `extract_service_name` → `:58` push → `:74` `StringArray` → `:83` the column, with **no `scrub_otlp_field` anywhere on that path**. This is a **coverage** gap, distinct from this chunk's **recall** claim, and it sits inside P-047's own subject ("PII Scrubbing at Ingestion"). It stands against a plain reading of security-plan §Security Anti-Patterns → Logging's uniform-scrubber-coverage paragraph, which enumerates covered call sites without stating that these columns are outside the mandate. **Needs an owner — carried to P5 as a trajectory decision.**
  2. **The chunk plan's own Test Commands claim is false**: it stated "none of this chunk's paths are on test-plan §3's boot-smoke trigger list", but step 5 modifies `pulse-app/src/observability.rs`, which that list names literally. The smoke was run rather than taking the exemption (see Deviations).
  3. **Not a disproof, recorded for precision:** `span_events.exception_type` is deliberately left raw (`appender.rs:365–367`, "class identifier, not user content") — a documented decision, not a gap.

- **Coverage of new surfaces:**
  - `security::scrubber` `provider_key` arm → validation n/a · instrumentation n/a · PII **redacted✓** (it IS the redaction) · tests **unit✓** (4 recall cases + 5 false-positive guards in the existing `#[rstest]` corpora) · a11y n/a · tokens n/a
  - `buffer.tick` field `redactions_applied` → validation n/a · instrumentation **log✓** (aggregate count on the existing 15s heartbeat, folded once per batch, zero per-field emissions) · PII **redacted✓** (aggregate count only; no matched value, category, attribute key, or `service_name` label — guarded by an explicit never-admit test) · tests **unit✓ + integ✓ + e2e✓** (3 allowlist guards + counter-advance assertions in the OTLP→DuckDB e2e + live wire observation) · a11y n/a · tokens n/a
  - OTLP→DuckDB bare-credential stored-field path (`log_records.body`, `span_events.exception_message`) → validation n/a · instrumentation **log✓** · PII **redacted✓** · tests **e2e✓** (real OTLP gRPC on ephemeral loopback → real consumer → real DuckDB read-back) · a11y n/a · tokens n/a

## Deviations from intent

1. **Ran the boot smoke the plan excused.** Plan §Test Commands asserted no boot-smoke gate. **Justification:** step 5 edits `pulse-app/src/observability.rs` (allowlist init) and step 4 edits `pulse-app/src/heartbeat.rs` (emit site) — both boot-path, and `observability.rs` is named literally on test-plan §3's boot-smoke trigger list. Running it was load-bearing: it produced the only wire-level proof that the counter reaches the log unredacted, which the plan's own CARRY #10 criterion requires.

2. **Widened `build_logs_record_batch` to take `&BufferState`** (3 call sites updated). **Justification:** plan §Implementation notes required the counter be folded from BOTH batch paths or have its coverage stated honestly. The logs builder had no state handle. Threading it was the honest option; a counter covering one path while named `redactions_applied` is the "reads as if it counted all" failure the note warned against.

3. **The live canary was hand-encoded OTLP protobuf.** **Justification:** OTLP/HTTP returned **415** for `application/json` — this build is protobuf-only (`crates/ingest/src/http.rs:37,205`), correct behavior, not a defect. A scratchpad encoder produced a valid payload so the counter's advance could be observed at the wire rather than only in-process.

4. **Counter field named `redactions_applied`, not `redactions_total` as planned.** The plan's step 4 — and the option preview the operator selected at the P4 CARRY-#10 decision — both named it `redactions_total`. **Justification:** the shipped name matches the participle style of its immediate siblings on the same event (`fingerprints_computed`, `observer_invocations`, `span_events_seen`), which is the local convention at the emit site. **This is a divergence from an operator-visible preview, not just from plan prose** — flagged here rather than silently kept; `_total` is also live in this project (`storms_detected_total`), so the planned name was equally defensible and a rename is cheap if preferred. Every artifact (code, guard test, allowlist leaf, report) uses `redactions_applied` consistently.

5. **`cargo clean -p pulse-app` mid-gate (environmental).** The D: dev drive hit 100% (49 MB free of 300 GB); `rust-lld` failed with `LLVM ERROR: IO failure on output stream: no space on device`. The hygiene dry-run measured only ~13 GB reclaimable, so the pre-authorized targeted clean was used — **93.2 GiB freed**. Not a code deviation; recorded because it interrupted the gate chain.

## Decisions & corrections

- **Design tension decided: anchored-prefix + length floor, entropy scoring REJECTED.** Grounded in obs-plan §8 (which names the shape family `sk-…{40,}` / `ghp_…{36}` / `AKIA…{16}` and warns over-broad patterns false-positive) and the catalog's own header (`scrubber.rs:55–58`, "recall over precision — false positives are acceptable; false negatives leak secrets"). Honest qualifier recorded in scope: obs §8 was authored for **grep-based CI heuristics**, not the runtime catalog, so it supplies vocabulary and a strong analogical lean, not a direct mandate.
- **Measure-first was made mechanical, not procedural.** The post-fix assertion was written and run BEFORE the catalog change; its failure output IS the measurement (`stored_len=40`, no marker, on both routes). A premise correction would have been an equally good outcome; the premise held.
- **`redactions_applied` scoped to the OTLP persistence path only** — the drain/template path (`drain.rs:374`/`:634`) calls `scrub_attribute` directly and already has its own tick fields. Scope stated in the field's doc comment rather than left implicit.
- **Operator correction accepted:** the /implement console header said "Modified (6)" while listing seven. `git status` confirms **seven**. Corrected here — the report is the detectors' only input, so a header disagreeing with its own list can mislead a file-counting detector.
- **`cargo audit` PREREQ:** **probe skipped per ratified interval (next: 34)** — this wrap is session 32. Never a silent skip. The overlap half WAS re-verified this session: `cargo deny check advisories` reports exactly **8 owned upgradeable IDs** — the standing seven (0189/0190/0194/0195/0204/0222/0253) plus **RUSTSEC-2026-0258** (`h2` 0.4.14 → ≥0.4.16), owned by the Advisory-backlog entry and never ignore-listed. Basis unchanged and upstream.
- **Cross-project, RECORD ONLY (no edit made to the Conductor repo):** Conductor's `PiiCategory` is a seven-variant enum whose doc comment defines it as "the seven P-047 PII categories Pulse's scrubber must detect and redact" — **derived from this catalog**, so its verified P-047 proof is *structurally incapable* of seeing the class this chunk just closed. An eighth category is now warranted there, or Conductor's claim stays narrower than Pulse's implementation. Its own route owns that work.

## Outcome

**Acceptance criteria: met.** The measurement ran RED before the change (both canaries stored verbatim, `stored_len=40`, no redaction marker) and GREEN after — on the real OTLP→DuckDB path, not a regex assertion.

**Gates green** (the chunk's plan `## Test Commands`, all run):
`cargo fmt --check` ✓ · `cargo clippy --workspace --all-targets --all-features -- -D warnings` exit 0 ✓ · `cargo nextest run -p security` 23/23 ✓ (14 → 23 cases) · `cargo nextest run --workspace --profile ci` **1833 passed, 1 skipped** ✓ (baseline 1819 → +14) · `cargo nextest run --workspace -E 'binary(e2e_pii_bare_credential_stored_field)'` 2/2 ✓ · `cargo xtask capability-drift` clean ✓ · `cargo xtask capability-widening-check` 0 violations / 3 inspected ✓ · `cargo xtask verify:capability-matrix` **60/60, 0 violations** ✓ (the `P-047` grep anchor survived the count edit) · `cargo deny check bans licenses sources` ok ✓ · `cargo deny check advisories` **designed-RED at exactly the 8 owned IDs**, no new finding (observed separately, never folded into the pass/fail invocation).

**Mutation check DISCHARGED.** Neutralizing the `redactions_applied` allowlist leaf turned **2 of 3** guards RED (the third is the never-admit ban, correctly insensitive to that mutation); restoring returned **3/3**. The pins discriminate.

**Boot smoke: PASS** (direct-binary variant, fresh `ANDROMEDA_PULSE_DATA_DIR`, prebuilt injector invoked by path — never `cargo run` inside a timeout):
- **Feed precondition asserted first:** `rows_ingested = 2403` — so no counter reading could be misread as a dead feed.
- **0 `app.panic.fatal` · 0 ERROR lines · 12 tick families present.**
- `redactions_applied` **present and UNREDACTED** on `buffer.tick`, and it advanced **0 → 1** on the tick after a hand-encoded bare-credential canary was ingested (`rows_ingested` 2403 → 2404). Value was legitimately 0 before — `inject_demo` carries no credentials, and a present-but-zero field is exactly the "passes for the wrong reason" shape, so a real canary was driven rather than accepting the zero.
- **Canary literal appears 0 times in the obs log** (obs-plan §1 Vector 1 satisfied).
- Clean shutdown by specific PID; `:4317` and `:4318` both released; 0 processes remaining.

**Verification matrix: no write.** Zero entries carry `chunk == 2026-08-22-pii-scrubber-recall`. P-047 has no entry in `andromeda-pulse-0.3.0/verification-matrix.json` (22 caps, P-061…P-082) — it was closed in the v0.2.0 era and lives in the legacy ledger `docs/v0_2_0/capability-verification-matrix.json`. The coverage gate correctly no-ops; proof lives in the tests plus this report.
