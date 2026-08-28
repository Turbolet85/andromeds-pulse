# Report — 2026-08-27-incident-persist-vs-resolve-write-race

**Chunk:** Incident persist-vs-resolve write race — a corpus resolution stays resolved: the stale-snapshot
persist loop stops silently reverting direct writers
**Date:** 2026-08-28T15:57Z
**Commits:** none since `last_wrap` (2026-08-27T22:11:01Z) — this chunk's work is uncommitted at report time

## Changes (structured — detectors read this)

- **Files:** 16 modified + 1 new.
  - `crates/corpus/src/contract.rs` · `crates/triage/src/incident/persistence.rs` ·
    `crates/triage/src/incident/mod.rs` · `crates/triage/src/contract.rs` ·
    `crates/mcp-server/src/tools.rs` · `pulse-app/src/incident_persistence.rs` ·
    `pulse-app/src/observability.rs`
  - tests: `pulse-app/tests/unit_observability_allowlist_incident_diagnostics.rs` ·
    `unit_incident_producer.rs` · `e2e_p3_mcp_incident_tools.rs` ·
    `integration_constellation_severity_workspace_key.rs` · `integration_deterministic_l4_mode.rs` ·
    `integration_generation_damper.rs` · `integration_interpretation_attach.rs` ·
    `integration_resolution_summary_attachment.rs` · `integration_tier1_storm_one_incident.rs`
  - NEW: `pulse-app/tests/integration_incident_write_guard.rs`

- **Symbols / APIs:**
  - NEW `pub enum corpus::contract::IncidentWriteOutcome { Applied, DeclinedStale }`.
  - NEW `pub enum triage::contract::IncidentWriteOutcome { Applied, DeclinedStale }` — a deliberate parallel
    type, not a re-export: `triage` sits below `corpus` in the DAG, so the adapter maps between them.
  - CHANGED `CorpusWriter::update_incident_status` — return `Result<(), Error>` →
    `Result<IncidentWriteOutcome, Error>`. **Callers KEPT, not sole:** `crates/mcp-server/src/tools.rs:461`
    (cross-process sidecar) + the `pulse-app` adapter + one in-crate corpus test.
  - CHANGED `IncidentPersistence::update_incident_status` — return `Result<(), IncidentError>` →
    `Result<IncidentWriteOutcome, IncidentError>`. **Six production callers KEPT and still compiling
    unchanged** (they bind only the `Err` arm): `crates/triage/src/incident/persistence.rs:170` ·
    `pulse-app/src/incident_observer.rs:73` · `pulse-app/src/incidents_router.rs:238` · `:309` ·
    `pulse-app/src/inference_runtime.rs:638` · `:825`. Nine test doubles updated to the new signature.
  - NEW tracing FIELD `declined_count` on the existing target `triage.incident.persist` (no new target).
  - **No new IPC procedure, no endpoint, no port, no env var, no CLI flag.**

- **Crates / modules:** none added, none removed. Four touched: `corpus`, `triage`, `mcp-server`,
  `pulse-app`. No new dependency edge — `mcp-server → corpus` and `pulse-app → corpus` already existed
  (graph-confirmed).

- **Dependencies:** none added · none bumped. `Cargo.toml` / `Cargo.lock` untouched.

- **Schema / config:** **no DDL, no `SCHEMA_VERSION` bump, no config key.** One SQL predicate added to the
  existing `incidents` UPDATE: `AND updated_unix_nano <= ?2`, a bound parameter (`status` and
  `updated_unix_nano` are already plaintext scalar columns; nothing was demoted from the encrypted cell).
  A second bound statement `SELECT 1 FROM incidents WHERE id = ?1` classifies a zero-row result.

- **Spec-master edits:** none at report time — the three expected amendments are listed under
  *Expected amendments* below and are P2's to apply.

- **Counts / qualifiers moved:** **YES, one.** The `triage.incident.persist` allowlist leaf goes
  **3 fields → 4** (`incident_count` · `persist_kind` · `duration_ms` · **`declined_count`**). Stated in:
  `.andromeda/obs-plan.md` §8 (Incident-path diagnostic leaves) and `.claude/rules/observability.md`
  (PII scrubbing bullet, which enumerates the three by name and argues why `duration_ms` belongs).

- **Dev-tool versions:** none.

- **Reverted / negative API facts:** none shipped. (The guard predicate was removed and restored as the
  mutation check; the restore is verified in-tree.)

- **Spec claims disproved by measurement:**
  1. **The working-route entry's "two writers" framing.** Measured: **seven** production call sites reach
     `update_incident_status`, and the seventh (`crates/mcp-server/src/tools.rs:461`) is in a **different
     process** and bypasses the `IncidentPersistence` trait entirely. Evidence: the P3 code-graph impact
     query (`{run_dir}/tree-query-{marker}.json`, 11 rows). **Disposition: recorded here and already
     premise-corrected in `scope.md`; the route entry is not a spec master, so no amendment is owed —
     this record IS the disposition.**
  2. **`plan.md` step 5's edit surface.** It directed threading the outcome through six named call sites;
     five needed **zero** edits because they bind only the `Err` arm, so a decline never reached
     `triage.incident.persist.error`. Measured by compiling. **Disposition: recorded here; a chunk-artifact
     claim, no amendment owed.**
  3. **The chunk's own GREEN-leg wait condition.** It polled `resolved_count` on
     `triage.incident.auto_resolve.tick`, a target with **no allowlist leaf**, so the field renders
     `"<redacted>"` and the condition was **vacuous by construction**. Confirms (does not contradict)
     obs-plan §8's standing record that this target resolves to nothing. **Disposition: recorded here;
     the un-muting is owned by the "Diagnostics un-muting + harness-truth sweep" entry.**

- **Coverage of new surfaces:**
  - `CorpusWriter::update_incident_status` (guarded corpus write boundary) → validation
    **n/a** (no external input; both new predicates are bound `?N` parameters, no interpolation) ·
    instrumentation **✓** (`declined_count` folded once per cycle on the existing `triage.incident.persist`
    record; no per-row emission) · PII **redacted✓** (aggregate count only — no incident id, scope_id,
    workspace, payload, SQL text, or bound values) · tests **unit + integration** (4 corpus in-crate
    incl. 3 new; 4 new integration in `integration_incident_write_guard.rs`) · a11y **n/a** (no UI
    surface) · tokens **n/a** (no UI element).
  - `dispatch_mark_incident_resolved` honest not-applied result (MCP tool response shape) → validation
    **n/a** · instrumentation **n/a** (no new emission) · PII **redacted✓** (bounded static reason string,
    no SQL, no payload) · tests **✗ at the cross-process tier** — covered only by the in-process
    signature/compile path; the cross-process form remains owned by the standing trigger
    `mcp-incident-read-back-cross-process-coverage` · a11y **n/a** · tokens **n/a**.

## Deviations from intent

1. **Plan step 5 needed one edit, not six** — five of its six named sites already bind only the `Err` arm,
   so a decline never reached the error target. *Justification:* the step's intent was satisfied by the
   existing code shape; inventing edits to match the step's count would have been churn. Only the MCP site
   needed work (it returned `{"resolved": true}` unconditionally; it now reports not-applied).
2. **Plan step 8 (sibling-violator sweep) executed as an explicit NO-OP** — no edit made. *Justification:*
   both sibling loops are whole-state single-call saves with exactly one production writer each
   (graph-confirmed at P3), so neither can exhibit a two-writer stale-snapshot race. Precedent:
   `2026-08-26-ingest-consumer-stall-under-sustained-load`, which likewise did not invent an edit.
3. **The RED *boot* leg named in Test Commands was not run; RED came from the mutation check instead.**
   *Justification:* the fix was already implemented when the smoke fired, and a boot-level RED cannot
   reliably reproduce the race inside a bounded leg — the collision needs a resolution to land inside the
   persist cycle's write span, which is precisely the nondeterminism the seam-based test removes. The
   mutation reddened 4 pins including **both** defect shapes; the accept-arm pins correctly stayed green.
4. **Eight files beyond the plan's explicit modify-list** (nine test doubles + two re-export lines).
   *Justification:* in scope — the plan and `research.md` named this class ("test companions that PIN the
   changed artifact", "the crate's lib.rs re-export"); the compiler enumerated the members.

## Decisions & corrections

- **Operator decision (P4 fork 1):** guard at the **corpus** choke point with a **monotonic**
  `updated_unix_nano` predicate — chosen over a triage-layer guard (covers 6 of 7 writers, misses the
  cross-process one) and over delta-persistence (larger, and does nothing for the cross-process path).
  A status-only predicate was ruled out by measurement: `attach_resolution_summary` *requires*
  `status == Resolved` and is the documented final write, so `status != 'resolved'` would break it.
- **Operator decision (P4 fork 2):** the app-registry staleness after an external (MCP) resolve is
  **accepted as a documented residual**, with reconciliation owned by a new route entry — not fixed here.
- **Correction, coordinate:** a CARRY on the *Diagnostics un-muting* entry cites the
  `incidents.list_active.request` leaf at `observability.rs:2242`; it is at **`:2321`**. Claim true,
  pointer stale — corrected in `scope.md` at fold time.
- **Correction, own measurement:** the GREEN-leg wait condition was vacuous (see Changes above). Caught by
  reading the records rather than trusting the loop's silence.
- **Recurring lesson observed twice this session:** a token-presence grep standing in for a semantic
  property produced two false reds (an "unanchored bullet" check keyed on the literal word `per`; an
  `[inferred]`-residue check that matched prose *about* the tag). Both resolved by reading the hits.

## Expected amendments (P2 to apply)

- `architecture.md` §Established Decisions — a **NEW** entry for corpus write arbitration (monotonic
  last-writer guard at the `CorpusWriter` choke point). The arch extract grep-verified that no
  write-arbitration / optimistic-locking / transaction-scope decision exists today, so this is an addition.
- `obs-plan.md` §8 — the `triage.incident.persist` leaf 3 fields → 4 (`declined_count`).
- Working-route — a NEW tail entry owning app-registry reconciliation with externally-resolved corpus rows.

## Outcome

**All acceptance criteria met.** Gates, all green and named:
`cargo fmt --check` · `cargo clippy --workspace --all-targets --all-features -- -D warnings` ·
`cargo nextest run --workspace --profile ci` → **2034 passed, 1 skipped** (+8 vs the 2026 baseline = exactly
the 8 pins added) · `cargo xtask capability-widening-check` (clean, 0/3) ·
`cargo xtask check:ingest-progress` (PASS) · `cargo deny check bans licenses sources` (exit 0) ·
`bash scripts/agent-run.sh status` · `cargo xtask capability-drift` (clean, run LAST after the bindings
regen). **0 fix-loop iterations.** Webview + a11y gates omitted — zero `pulse-app/ui/**` delta
(`bindings/index.ts` verified byte-identical to HEAD after regen).

**Supply chain:** `cargo deny check advisories` designed-red at the same **8 DISTINCT** ids —
0189 / 0190 / 0194 / 0195 / 0204 / 0222 / 0253 / 0258, seventh consecutive identical set.
**Audit PREREQ (session 51, a between-point):** probe NOT re-run per the ratified interval; basis + overlap
re-verified first-hand. Record: `probe skipped per ratified interval (next: 52)`.

**Mutation check (the RED measurement):** removing the predicate reddened **4** pins —
`a_resolution_landing_inside_the_persist_write_span_survives` (the race),
`an_externally_resolved_row_is_not_reverted_by_the_next_persist_cycle` (the cross-process deterministic
loss), `a_stale_write_declines_rather_than_erroring`, and the corpus-level decline pin. The three
accept-arm pins (`a_fresh_write_still_applies_through_the_adapter`,
`update_incident_status_applies_an_equal_timestamp_write`,
`update_incident_status_still_errors_for_a_missing_row`) correctly stayed **green in both worlds** — the
conditional-property asymmetry: the accept pins cannot discriminate alone, the decline pins carry the
guard. Predicate restored and verified in-tree.

**Boot smoke — GREEN leg (direct-binary variant; `observability.rs` is a declared boot-path trigger).**
Fresh data dir, deterministic L4, real OTLP seed via `inject_demo`. Feed precondition **`rows_ingested:
16092`** · 3 incidents created · `incidents.list_active.request` → `item_count` ran 0 → 5 → 5 → 5 → **0** ·
corpus read-back (plaintext `status`/`workspace` columns only — no key, no payload, no PII) shows
**0 non-resolved rows for the app's own workspace key**, both rows `resolved`. **The two halves AGREE —
zero zombie-actives**, which is the working entry's VERIFY clause. **0 `ERROR`, 0 `app.panic.fatal`.**
Clean stop by specific pid; `:4317` / `:4318` confirmed released; zero orphans.

**Stated rather than implied:** `declined_count` read **0** on all 13 persist records in the leg. Its
*plumbing* is proven at the wire (it renders **unredacted**, so the leaf extension works), but no stale
write raced during the run — the counter's **arithmetic** was exercised only by the tests, not live.

**Capabilities claimed: none** (P-075, the version's one unverified entry, is Conductor's external e2e plus
the four delegated timing caps — untouched here), so the coverage gate is a no-op for this chunk.
