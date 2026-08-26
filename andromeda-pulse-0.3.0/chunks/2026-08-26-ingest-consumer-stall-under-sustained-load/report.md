# Report — 2026-08-26-ingest-consumer-stall-under-sustained-load

**Chunk:** Ingest consumer stall under sustained load — the buffer keeps draining under a sustained feed, and a wedged consumer becomes visible instead of reading as healthy
**Date:** 2026-08-26
**Commits:** none yet since `last_wrap` 2026-08-26T06:40:00Z — this wrap authors the chunk commit

## Changes (structured — detectors read this)

- **Files:**
  - `crates/buffer/src/state.rs` — new `last_append_at_nanos` atomic + snapshot field + 2 in-crate tests
  - `crates/viz/src/query.rs` — new `pub fn read_connection` + 1 in-crate test
  - `pulse-app/src/viz_routers.rs` — three router constructors take the cloned read connection
  - `pulse-app/src/heartbeat.rs` — drain-progress classifier, 2 new `buffer.tick` fields, transition WARN, `run_buffer` signature
  - `pulse-app/src/observability.rs` — allowlist: 2 fields on the `buffer` set + a new exact leaf
  - `xtask/src/main.rs` — `mod ingest_progress` + `Cmd::CheckIngestProgress` + dispatch + `run_check_ingest_progress`
  - NEW `xtask/src/ingest_progress.rs` · NEW `pulse-app/tests/unit_consumer_stall_classification.rs` · NEW `pulse-app/tests/unit_observability_allowlist_consumer_stall.rs`
  - `pulse-app/ui/src/bindings/index.ts` — regenerated to canonical (content diff EMPTY vs HEAD; EOL artifact only). **No UI source change.**
  - **NOT touched:** `crates/triage/**` (see Reverted / negative API facts)

- **Symbols / APIs:**
  - NEW `pub fn viz::query::read_connection(&Arc<Mutex<Connection>>) -> Arc<Mutex<Connection>>` — reached via the existing `pub mod query` path; **no `crates/viz/src/lib.rs` re-export added**. Callers: `TracesApiImpl::new` / `MetricsApiImpl::new` / `LogsApiImpl::new` (`pulse-app/src/viz_routers.rs`). The three `query_traces` / `query_metrics` / `query_logs` signatures are UNCHANGED — all other callers keep working; MCP reaches them by its own path unaffected.
  - NEW `pub` in `pulse-app::heartbeat` (all `#[doc(hidden)]`, for integration-test reach under `[lib] test = false`): `TARGET_CONSUMER_STALLED` · `STALL_CONSECUTIVE_TICKS` · `enum DrainProgress` · `fn classify_drain_progress` · `fn last_append_age_seconds`
  - CHANGED (private) `heartbeat::run_buffer` +1 param `Arc<IngestSender>`; `heartbeat::emit_buffer_tick` +2 params, now returns `DrainProgress`. Both private — `spawn()` is the sole caller and already held both Arcs; **16 in-file `mod tests` call sites updated** (they compile under `clippy --all-targets` even though `[lib] test = false` stops them running).
  - CHANGED `buffer::BufferStateSnapshot` +1 field `last_append_at_nanos`; `BufferState::record_rows_appended` now also stores it. Single construction site (`BufferState::snapshot`).
  - NEW xtask command `check:ingest-progress`; NEW `pub` in `xtask::ingest_progress`: `Verdict` · `evaluate` · `parse_lines` · `TARGET_STALLED` · `TARGET_BUFFER_TICK`
  - **No TauRPC procedure added/changed** — `EXPECTED_PROCEDURES` untouched, `capability-drift` clean.
  - **No new ports, sockets, or env vars.**

- **Crates / modules:** none added, none removed. Changed: `buffer`, `viz`, `pulse-app`, `xtask`.

- **Dependencies:** none added, none bumped. `Cargo.toml` / `Cargo.lock` untouched.

- **Schema / config:** none. No DuckDB DDL change, no corpus schema change, no `config.toml` key, **no env var** (the stall threshold is a named constant — obs-plan §10 already fixes the tolerance numbers, so there is nothing for a user to tune, and this avoids an arch §Occupied Resources + security-plan §Input Validation registration for a value with one correct setting).

- **Spec-master edits:** none. (Expected amendments for THIS wrap are listed under Outcome.)

- **Counts / qualifiers moved:**
  - `buffer.tick` field set: **12 → 14** fields (adds `rows_ingested_delta`, `last_append_age_seconds`). Restated at obs-plan §1 (Heartbeat ticks), §5 (tick-aggregated Counter rows), §8 (`buffer` whitelist) — the triple-site rule applies.
  - Obs allowlist target count: **+1 exact leaf** (`buffer.consumer.stalled`), which also needs a §6 warn-row entry (dual-site rule).
  - `BufferStateSnapshot` fields: **9 → 10**.
  - Workspace test count: **1931 → 1950** (+19, accounted exactly: 6 classification + 4 allowlist + 2 buffer state + 6 xtask ingest_progress + 1 viz read_connection).
  - obs-plan §10 DuckDB-connection-isolation topology: the isolated set grows from {retention, L1a reads, Q7 primary} to include **viz's three production query fns**.

- **Dev-tool versions:** none.

- **Reverted / negative API facts:**
  - **Plan step 4 executed as a NO-OP — nothing was written to `crates/triage/src/baseline/sql.rs`.** The plan directed moving the Q7 fallback off the shared connection. Verification before editing showed there is nothing to move: `run_q7_with_timeout` takes `state.conn`, and `TriageSqlState::new` (`sql.rs:314`) already `try_clone`s it, so the fallback has always run on the L1a read clone. The in-code phrase "the shared connection" means *shared among L1a queries*, not the appender connection. No edit was invented to satisfy the step.
  - No `crates/viz/src/lib.rs` re-export was added (the `pub mod query` path made it unnecessary), so the crate's declared export list is unchanged.

- **Spec claims disproved by measurement:** *(recorded here as their durable home, per operator ruling 2026-08-26 — `research.md` and `scope.md` have no sanctioned writer at this phase and were NOT edited)*
  1. **The chunk's prime causal hypothesis is FALSE.** `research.md` §Findings 1 and `scope.md` §Premise closure name viz shared-appender-connection contention as the first-class candidate for the wedge. **Measured false:** the RED leg reproduced every stated precondition (405–780 `viz.query.traces`, all four windows navigated, real L4, sustained feed, 15 min) and the consumer never wedged — `rows_ingested` climbed linearly to 47,898 with the channel at 0.0 % throughout. Evidence: `legs/red-at-head` (283,902 lines, 0 ERROR, 0 panics, 62 ticks).
  2. **`research.md` §Findings 2 named a second obs-plan §10 violator that does not exist** (the Q7 fallback). Evidence: `sql.rs:300-330,448-455` + `pulse-app/src/main.rs:543`.
  3. **`plan.md` §Test Commands names the wrong binary** — `./target/release/andromeda-pulse.exe`; the cargo bin is `pulse-app.exe` (the former is the bundled artifact name). Evidence: `pulse-app/Cargo.toml:14-16`.
  4. **obs-plan §10's isolation invariant was VIOLATED in code at HEAD** by `crates/viz` (three production query fns on the shared appender connection, no `try_clone` in the crate, no timeout). The MANDATE was correct; the code diverged. **Fixed in the impl this chunk** — so this is a closed divergence, not an open disposition.

- **Coverage of new surfaces:**
  - `buffer.tick` +`rows_ingested_delta` +`last_append_age_seconds` → validation n/a · instrumentation ✓ (tick-aggregated, 15 s, never per-batch) · PII redacted✓ (aggregate numerics only; resolve through the `buffer` set, verified unredacted on the wire: 42/42 ticks) · tests ✓ (`unit_observability_allowlist_consumer_stall.rs` + 2 in-crate `state.rs` pins) · a11y n/a · tokens n/a
  - `buffer.consumer.stalled` (new WARN target) → validation n/a · instrumentation ✓ (once per healthy→stalled transition and once on recovery; never per-batch) · PII redacted✓ (own EXACT leaf; `reason`/`consequence` are bounded static tokens, `stalled_seconds`/`buffer_capacity_pct` numeric; no service/span/fingerprint identity) · tests ✓ (4 pins incl. a fallback-set discriminator proving the exact leaf is load-bearing) · a11y n/a · tokens n/a
  - `viz::query::read_connection` (new pub fn) → validation n/a · instrumentation ✓ (`viz.query` warn-on-fallback, mirroring `buffer.retention` / `triage.sql`) · PII n/a · tests ✓ (in-crate: distinct-Arc + same-database) · a11y n/a · tokens n/a
  - `cargo xtask check:ingest-progress` (new gate) → validation ✓ (bounded `Verdict`; NEUTRAL-tolerant on an absent stream) · instrumentation n/a (CLI gate) · PII n/a · tests ✓ (6 unit + a 4-arm end-to-end discrimination proof) · a11y n/a · tokens n/a
  - No UI element, route, region or interactive surface added → design-tokens / layout / a11y all n/a for this chunk.

## Deviations from intent

1. **Step 4 was a no-op** (see Reverted / negative API facts). Justification: the premise was measured false; fabricating an edit to satisfy a plan step would have shipped work with no subject.
2. **Step 5's fix was NOT applied — the attribution landed outside the chunk's touchpoints.** Justification: the measured root cause is a runaway in `crates/triage/src/cadence/`, outside `plan.md`'s Files-to-modify (fix-loop Trigger 3). The plan anticipated this verbatim ("If attribution lands outside those, say so plainly"). Consequence, stated plainly: **scope outcome 1 — "the buffer keeps draining" — is delivered by measurement, not by repair.** HEAD is healthy absent the runaway precondition; the runaway itself is un-fixed and gets its own route entry.
3. **Binary path corrected** in the smoke invocation (`pulse-app.exe`). Justification: plan fact wrong; `plan.md` is read-only at implement.
4. **WARN target named `buffer.consumer.stalled`**, not the P4 option-preview's `ingest.consumer.stalled`. Justification: the emitter is `buffer::run_consumer`, and in-repo targets name their owning module (`buffer.tick`, `buffer.retention`, `triage.sql`). Flagged at the P5 review before approval.
5. **`viz` helper reached via `viz::query::read_connection`** rather than a `lib.rs` re-export. Justification: keeps an unlisted file out of the modify set at no cost.
6. **One test added at wrap P1** (`read_connection_returns_a_distinct_usable_connection`). Justification: a new `pub` boundary with no direct test is a test-plan §2 gap; closed in-chunk rather than reported.

## Decisions & corrections

- **Operator ruling (2026-08-26, this wrap):** `research.md` is never mutated (ownership table) and `scope.md`'s amendment windows (P3 premise-closure, P5 val-1) are closed — so the falsifications above have **no sanctioned writer** at wrap. Their durable home is THIS report plus the drift/curation channels. Neither artifact was edited. *(This retracts the "needs premise-correction" item from the /implement P4 console report.)*
- **Operator ruling (2026-08-26):** the cadence-runaway route entry's research must own the **TRIGGER** question — what made that run's cue→cadence→digest→L1a loop spawn 122× while this chunk's 15-min RED leg stayed at baseline — i.e. the *conditions*, not only the mechanism.
- **Attribution verified first-hand by the overseer:** 26,821 `metric.pipeline.l1a.query_count_total` records in the predecessor's wedge log vs 220 in the healthy leg. The 122× runaway stands.
- **Operator decision (P4 review):** fix surface includes the §10 violation on its own merits, independent of causation; enforcement goes beyond "observable" to a failing gate.
- **Measure-first held:** the observable's threshold was chosen from obs-plan §10's stated in-spec bounds (450 s > the 420 s sustained-drain cap), not from the observed wedge.

## Outcome

**Acceptance criteria: met**, with deliverable A's *fix* explicitly out of scope (deviation 2).

- Outcome 1 (buffer keeps draining) — measured healthy at HEAD and after the change; **not** delivered by repair.
- Outcome 2 (wedged consumer visible) — delivered: field pair + transition WARN + exact allowlist leaf + failing gate.
- Attribution named on evidence: **tokio blocking-pool starvation driven by a 122× runaway in the cue→cadence→digest→L1a loop.** Discriminator: `viz` (appender connection) and L1a (a separate clone) use *different mutexes*, yet both `spawn_blocking` users stopped (`duckdb.append` 17:20:50, `viz.query.traces` 17:24:13) while every async task ran 16 minutes longer — a mutex takes one, only the pool takes both. Max append duration 11 ms in both runs and the final append succeeded in 2 ms, so DuckDB contention is excluded. Candidates ruled out, recorded: DuckDB append contention, viz shared-connection contention, retention/L1a clone fallback (0 fallback WARNs), Q7 timeout (0).

**Gates green** — `cargo fmt --check` · `cargo clippy --workspace --all-targets --all-features -- -D warnings` (0 warnings) · `cargo nextest run --workspace --profile ci` **1949/1949 + 1 skip** (pre-P1; +1 viz test since → 1950 expected at the light gate) · `cargo xtask capability-drift` clean · `cargo xtask capability-widening-check` clean · `cargo deny check bans licenses sources` ok · `cargo xtask check:ingest-progress` PASS. Webview gates excluded — zero `pulse-app/ui/**` source delta (bindings content-identical to HEAD).

`cargo deny check advisories` observed SEPARATELY as designed-red: exit 1 at exactly **8 DISTINCT** owned upgradeable IDs (0189/0190/0194/0195/0204/0222/0253/0258), set unchanged. **`cargo audit` PREREQ: probe skipped per ratified interval (next: 46)** — basis + overlap re-verified first-hand this session.

**Two mutation checks, both discriminating.** Gate: 4 arms — `stalled` → exit 1, `recovered` stays green (so it keys on `reason`, not target presence), `healthy` PASS, `empty` NEUTRAL. Classifier: collapsing `classify_drain_progress` to `Advancing` reddened exactly the 2 pins guarding the collapsed branches while the other 4 stayed green; restored, no residue.

**Smoke (boot-path changed — `pulse-app/src/observability.rs` is a declared test-plan §3 trigger): two Direct-binary legs**, own fresh `ANDROMEDA_PULSE_DATA_DIR` each, real OTLP, real L4 (registered GGUF + b9305 CUDA `llama-cli`).
- RED at HEAD (15 min): 283,902 lines · 0 ERROR · 0 panics · 62 ticks · 47,898 rows · channel 0.0 % · 0 Q7 timeouts · **wedge did not reproduce**.
- GREEN with changes (10 min): 189,672 lines · 0 ERROR · 0 panics · 42 ticks, **42/42 carrying both new fields unredacted** · 0 `viz.query` fallback WARNs (the `try_clone` took) · 405 `viz.query.traces` still flowing · gate PASS (longest zero-delta run 1, threshold 30).
- Clean shutdown both legs: no orphan processes, `:4317`/`:4318` released.

**Not observed live:** the `buffer.consumer.stalled` WARN never fired, because the condition did not reproduce. Its correctness rests on the mutation-checked branch pins and the 4-arm gate proof, not on a live firing — stated rather than implied.

**Expected amendments (wrap):**
- `obs-plan.md` §1 + §5 + §8 — the `buffer.tick` field set 12 → 14 (triple-site; a §8-only apply leaves §1 and §5 stale).
- `obs-plan.md` §6 + §8 — the new `buffer.consumer.stalled` WARN target (dual-site).
- `obs-plan.md` §10 — a progress-vs-liveness signal named alongside the tick-absence stall definition (the measured wedge satisfied tick-presence throughout), and the connection-isolation topology extended to include viz's three query fns.
- `test-plan.md` §3 — `cargo xtask check:ingest-progress` in the gate discipline.

**Surfaced for the route (not fixed here):** the cadence/digest runaway that starves the blocking pool — needs its own entry, whose research owns the TRIGGER question per the operator ruling above. Evidence preserved at `scratchpad/l4run-171923/logs/agent-latest.jsonl.2026-08-25`.
