# Report — 2026-08-15-corpus-key-persistence

**Chunk:** Corpus key persistence — the AES-256-GCM cell key survives a process boundary (a real platform
credential-store backend linked, with the intended PassphraseFallback branch made reachable and warning
where no store exists), and pre-fix content orphaned under ephemeral keys is retired by an operator-chosen
disposition (recovery is cryptographically closed); carries the operator-gated arm-zero classification leg
**Date:** 2026-08-15
**Commits:** none this session (the chunk commits at this wrap; last commit was
`2961f4e feat(2026-08-14-workspace-key-alignment)`)

## Changes (structured — detectors read this)

- **Files:** 20 (16 modified · 4 new) — `Cargo.toml` · `Cargo.lock` · `deny.toml` ·
  `crates/corpus/Cargo.toml` · `crates/corpus/src/lib.rs` · `crates/corpus/src/keychain.rs` ·
  `crates/corpus/src/contract.rs` · **new** `crates/corpus/src/disposition.rs` ·
  `crates/mcp-server/src/tools.rs` · `pulse-app/src/main.rs` · `pulse-app/src/observability.rs` ·
  `pulse-app/tests/e2e_p3_mcp_incident_tools.rs` · **new**
  `pulse-app/tests/unit_observability_allowlist_corpus_key.rs` · `.andromeda/master-route.md` ·
  `andromeda-pulse-0.3.0/working-route.md` · `andromeda-pulse-0.3.0/verification-matrix.json` ·
  `.andromeda/friction-log.ndjson` · `.claude/session-handoff.md` · **new** chunk folder · **new** phase run-dir

- **Symbols / APIs:**
  - NEW public module `corpus::disposition` — `DispositionOutcome` (`Noop` / `Completed{inventory_path,
    rows_purged, tables_affected}` / `Failed{error_category}`) · `OrphanedIncident` · `OrphanScan` ·
    `scan_orphaned()` · `render_inventory()` · `dispose_orphaned_content()` · `emit_disposition_outcome()`
  - NEW public const `corpus::contract::CORPUS_PASSPHRASE_ENV` = `"ANDROMEDA_PULSE_CORPUS_PASSPHRASE"`
  - CHANGED `OsKeychainBackend` — gains interior `served_by` state; `backend_kind()` now reports the branch
    that actually served the key (was a static `cfg!` guess). `new()` signature UNCHANGED, so both
    production construction sites are untouched.
  - CHANGED `crates/mcp-server::dispatch_retrieve_report` — stamps `incident.id = row.id` after decode
    (P-038 parity fix; see Deviations §3).
  - UNCHANGED: `Corpus::open` / `open_in_memory` signatures · `cell_encrypt` / `cell_decrypt` · the 8 MCP
    `#[tool]` methods · every TauRPC procedure (no IPC surface added → no capability JSON, no
    `EXPECTED_PROCEDURES`, no bindings contract change).
  - **NEW env var:** `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` — opt-in fallback passphrase; bounded parse
    (non-empty, ≤1024 bytes), never logged. **Not yet registered in arch §Occupied Resources.**
  - **NEW filesystem path:** `<data_dir>/corpus/orphaned-inventory-{unix_nano}.md` — the pre-purge audit
    artifact, atomic `.tmp`+rename, under the data-dir root. **Not yet registered in arch §Occupied
    Resources.**
  - **NEW tracing targets:** `corpus.keychain.fallback` (WARN) · `corpus.read.undecryptable` (WARN) ·
    `corpus.orphan.disposition` (WARN). **REPAIRED existing target:** `corpus.open.error` — emitting since
    chunk #68 with no resolvable allowlist entry, so its `error_kind` was redacted.
  - No new ports/sockets.

- **Crates / modules:** `crates/corpus` gains the `disposition` module (`pub mod`). No crate added or
  removed; the workspace member list is unchanged at 16.

- **Dependencies:**
  - `keyring` `"3"` → `{ version = "3", features = ["apple-native", "windows-native",
    "sync-secret-service", "crypto-rust"] }`. keyring 3.x declares **no `default` feature**, so the bare
    form linked no platform store.
  - New transitive tree: `security-framework` 2.11.1 + 3.7.0 · `core-foundation` 0.9.4 + 0.10.1 ·
    `windows-sys` 0.60.2 · `byteorder` · `dbus-secret-service` (+ its pure-Rust `crypto-rust` cipher deps).
  - `crates/corpus` takes `blake3` (already a workspace-level dep since the fingerprint work) for the
    fallback KDF.
  - `deny.toml [bans] skip` += `core-foundation`, `security-framework` — ID-scoped, owner-named, reasoned
    (the duplicate is INTERNAL to keyring, which depends on both security-framework majors; not
    source-fixable without dropping macOS key persistence).

- **Schema / config:** **no** corpus DDL change · **no** `SCHEMA_VERSION` bump (still 1) · no new corpus
  table · no `config.toml` key. The 6 reserved corpus tables are unchanged.

- **Spec-master edits:** none applied yet — this wrap's P2 owns them. Queued from `plan.md` §Expected
  amendments: arch §Occupied Resources (at-rest posture + the 2 new resources above) · security-plan
  §Secret Management → Storage → Runtime (P-049 posture) · obs-plan §8 (allowlist leaves) · test-plan §1
  (the `workspace-key-cross-process-coverage` pending trigger's BLOCKED framing).

- **Counts / qualifiers moved:**
  - workspace `cargo nextest run --workspace --profile ci`: **1745 → 1768** passed (1 skipped) — stated in
    `.claude/session-handoff.md` and any doc restating the suite size.
  - `crates/corpus` crate suite: → **70** tests.
  - `pulse-app --features mcp-server` leg: **381/382 → 382/382** (1 skipped).
  - MCP tool count unchanged at 8.

- **Dev-tool versions:** none.

- **Reverted / negative API facts:** none. (An auto-generated on-disk fallback key file was CONSIDERED and
  rejected before implementation — security-plan §Data Protection forbids plaintext runtime state on disk
  — so no such surface ever existed.)

- **Spec claims disproved by measurement:**
  1. **Workspace `Cargo.toml` comment**: *"keyring 3 … Default features auto-select per target"* — measured
     FALSE (`cargo info keyring` lists no `default` feature). This is a code comment, corrected in-place in
     the same edit; recorded here because it is the recorded false belief that produced the defect.
  2. **security-plan §Secret Management → Storage → Runtime**: *"neither branch of the above is reached
     (open defect)"* — the keychain-primary branch is now reached and measured (Windows Credential Manager
     entry `corpus-key.com.andromeda.pulse` present; cross-process read-back proven). **Needs amendment.**
  3. **arch §Occupied Resources → Corpus SQLite → At-rest posture**: *"key sourcing is INTENDED … but
     UNIMPLEMENTED as of 2026-08-15 … per-process ephemeral"* — no longer true. **Needs amendment.**
  4. **The chunk's own working-route entry expectation**: *"expect the incident read to REACH decryption and
     fail there"* — measured: the read now completes (`tool dispatch ok`), so the expectation is discharged
     rather than confirmed. Evidence: `evidence/arm-zero-classification.md` §5.
  5. **LIVE product finding (outranks this chunk — see Deviations §5):** storms are detected
     (`storms_detected_total` 0→1) but `triage.cue.emit` = **0** and the `incidents` table holds **0 rows** —
     in the Conductor arm AND both smoke boots. The capture-chunk era demonstrably created incidents (10 in
     its run; 2 active independently read at the workspace-key before-evidence), so the storm→cue/incident
     path plausibly regressed within the last two chunks' window. Not a spec-master edit — a route matter.

- **Coverage of new surfaces:**
  - `OsKeychainBackend primary+fallback key resolution` → validation {bounded-parse ✓} · instrumentation
    {WARN `corpus.keychain.fallback`, once per boot, bounded static fields ✓} · PII {key/passphrase never
    logged ✓; `EncryptionKey` Debug prints `len` only} · tests {unit ✓ — 5 resolve/KDF cases + a real
    cross-process leg} · a11y {n/a} · tokens {n/a}
  - `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` (env input boundary) → validation {non-empty + ≤1024 bytes, unset
    ⇒ not-configured ✓} · instrumentation {n/a — presence is implied by the fallback WARN} · PII {never
    logged ✓} · tests {unit ✓} · a11y {n/a} · tokens {n/a}
  - `corpus orphan disposition (boot pass + inventory artifact)` → validation {n/a — self-written, table
    names from a private const list, ids bound as params} · instrumentation {WARN
    `corpus.orphan.disposition`, aggregate counts only ✓} · PII {artifact carries only columns already
    plaintext in the corpus; log carries counts only ✓} · tests {unit ✓ — 6 cases incl.
    preserve-on-failed-write and idempotence} · a11y {n/a} · tokens {n/a}
  - `corpus read-path skip-and-count` → validation {n/a} · instrumentation {WARN
    `corpus.read.undecryptable`, one aggregate per query, bounded `query_id` ✓} · PII {count only, no row
    identity ✓} · tests {unit ✓ — mixed-corpus + by-id not-found} · a11y {n/a} · tokens {n/a}
  - `MCP retrieve_report incident-id stamp` → validation {n/a} · instrumentation {existing
    `mcp.tools.call.*` ✓} · PII {n/a — an i64 rowid} · tests {integration ✓ — 4/4 in
    `e2e_p3_mcp_incident_tools`} · a11y {n/a} · tokens {n/a}
  - **No UI surface** was added or changed (zero `pulse-app/ui/**` paths touched) → design-token, layout,
    and a11y coverage are `n/a` for this chunk.

## Deviations from intent

1. **Passphrase SOURCE was underspecified by the plan** (it said "a passphrase-derived key" without naming
   the source). Resolved WITHOUT dialogue because security-plan answers it three times: *"derived
   per-user"*, *"secrets MUST come from environment"*, *"No plaintext runtime state on disk"*. Chose an
   env-supplied passphrase with BLAKE3 derivation; nothing is written to disk. The rejected alternative
   (auto-generated on-disk secret) would have added a registered data-dir secret and changed the at-rest
   posture.
2. **The fallback is opt-in, not automatic.** It engages only when a passphrase is configured; an
   unconfigured host surfaces the store's error instead. Justification: a transient store failure silently
   switching keys would re-create this very defect. Consequence, stated plainly: on a host with no
   credential store AND no passphrase, the corpus degrades to absent — which is strictly better than
   today's silent non-persisting key, but it does mean the acceptance clause "neither … nor a hard failure"
   holds for the CONFIGURED case only.
3. **A product fix in another crate, operator-sanctioned mid-wrap-prep.** The sanctioned inherited-fixture
   repair (`e2e_p3_mcp_incident_tools.rs` ms-scale literals → now-relative) was premised on being "one
   file, test-only". That premise was falsified: repairing the 30-day window let the test reach its real
   assertion **for the first time**, which exposed a genuine P-038 parity bug — `dispatch_retrieve_report`
   decoded the incident from the bincode payload and never stamped `row.id`, so the sidecar rendered
   `Incident ID: 0` and the incident appeared as its own prior (`load_previously_seen` in the same file
   already stamped). Surfaced, not absorbed; the operator chose the product fix. Landed: one line in
   `crates/mcp-server/src/tools.rs` + both test expectations. Leg now 382/382.
4. **Three edits just outside the plan's Files-to-modify**, each judged an in-scope helper spawned by the
   chunk's own work: `crates/corpus/Cargo.toml` (the blake3 line), `crates/corpus/src/lib.rs` (the `mod`
   line for the module the plan's New-files entry anticipated), and a **fifth** `cell_decrypt` site at
   `contract.rs:467` (`load_pipeline_metric`) that research enumerated only four of — hardening four and
   leaving one fail-fast would have shipped an incoherent mixed posture.
5. **Step 0 (arm-zero) RAN** rather than deferring a third time — see Outcome. Its result is a finding that
   outranks this chunk (Changes §Spec claims disproved #5) and is carried to route-resolve, not actioned
   here.

## Decisions & corrections

- **Operator, at /phase P4:** retire-HOW → **inventory-then-purge** (a third option research surfaced that
  dominates both named poles: preserves the audit record, leaves the DB clean, needs no DDL or schema
  bump). Read path → **skip-and-count** hardening, independent of the disposition.
- **Operator, pre-wrap:** sanctioned the inherited-fixture fix + running Step 0 now; then, when the
  "test-only" premise proved false, chose the product parity fix over encoding the defect into the test.
- **Correction to the /implement report (mine):** I reported "605 incident events written" in the P3 smoke.
  That grep matched `incidents.list_active.request` — the webview's IPC poll — not incident creation. An
  independent read-only sqlite3 count showed the `incidents` table held **0 rows** in both smoke boots. The
  key-persistence proof is unaffected: it rests on `digest_archive` (15 rows) + `pipeline_metrics` (10
  rows) encrypted payloads written by boot 1 and decrypted by boot 2.
- **Host-tooling correction (mine):** `cmdkey /list` run from Git Bash never executes — MSYS path
  conversion rewrites the `/list` switch into a path, so it prints its usage banner. Piped to `grep` that
  reads exactly like a truthful "no entries" answer, and I used it once as evidence of test hygiene.
  Re-run via `powershell -NoProfile` it showed the real list. Any Windows credential-store assertion made
  from Git Bash without `MSYS_NO_PATHCONV` is unfalsifiable the same way.
- **Muted-diagnostic class, grown:** beyond `corpus.open.error` (repaired here), the arm-zero
  classification was actively obstructed by `incidents.list_active.request → item_count`,
  `triage.incident.persist → incident_count`/`persist_kind`, and `triage.incident.corpus_restore →
  kind`/`restored_incident_count`, all `<redacted>`. Locating the zero required an out-of-band sqlite3
  read. These are `triage.*` / `incidents.*` targets whose intended field sets are not this chunk's to
  choose — carried to P2.

## Outcome

**Acceptance criteria — met, with two qualified:**

- ✅ **Cross-process key persistence.** Two sequential boots against one data dir: boot 1 wrote encrypted
  `digest_archive` (15) + `pipeline_metrics` (10) rows; boot 2 ran three decrypting corpus restores
  (`restored_fingerprint_count: 1` from the encrypted `pipeline_metrics` payload) with **0
  `decryption_failed`**, 0 ERROR, 0 panics — against the measured 13 failures that defined the defect.
  Credential entry `corpus-key.com.andromeda.pulse` confirmed present (via powershell, per the correction
  above). Unit leg `corpus_key_survives_a_real_process_boundary` re-executes the test binary for a genuine
  process boundary — the in-process two-instance form would have PASSED under the defect, since keyring's
  mock store was process-global.
- ⚠️ **Fallback branch** — the branch is reachable, warns, and is unit-proven in both directions
  (`resolve_key` with store-Err + passphrase → `PassphraseFallback` + WARN; without passphrase → the
  store's error). Qualified per Deviations §2: it engages on configuration, not automatically.
- ✅ **Orphan disposition** — inventory written before deletion, `service_registry` untouched, idempotent,
  rows preserved on a failed inventory write. 6 unit cases.
- ✅ **`cargo deny check bans licenses sources` green** with ID-scoped, owner-named, reasoned entries; no
  ignore widened, no `multiple-versions` relaxation.
- ⚠️ **`cargo audit`** cannot run at all: `error loading advisory database: parse error: duplicate advisory
  ID: RUSTSEC-2026-0244`, reproduced on retry. Upstream RustSec data fault — no code change clears it; the
  same upstream event Conductor hit (one advisory DB, two projects). Substitute signal
  `cargo deny check advisories`: 11 findings, **none** tracing through
  keyring/security-framework/core-foundation/dbus-secret-service (verified by grepping the dependency
  trees), so the new tree introduced no advisory. Disposition is this wrap's directive-2/3 business.
- ✅ **Obs allowlist** — `corpus.open.error` repaired plus 3 new explicit leaves; 6 probes in a new
  pulse-app integration test, including the invariant that **no bare `corpus` prefix key exists** (a prefix
  key would silently widen every future `corpus.*` target).
- ✅ **Arm-zero classification RECORDED** — `evidence/arm-zero-classification.md`, verbatim counters.

**Gates (all green):** `cargo fmt --check` · `cargo clippy --workspace --all-targets --all-features -D
warnings` · `cargo build --workspace --tests --jobs 4` · `cargo nextest run -p corpus` **70/70** ·
`cargo nextest run --workspace --profile ci` **1768 passed / 1 skipped** · `cargo deny check bans licenses
sources` · `cargo build -p mcp-server --features mcp-server --bin andromeda-pulse-mcp` ·
`cargo nextest run -p pulse-app --features mcp-server` **382/382** · `cargo xtask capability-drift` clean ·
`cargo xtask capability-widening-check` clean · `bash scripts/agent-run.sh status`. Bindings regenerated as
the last cargo op. `cargo audit` is the one non-green — environmental, see above.

**Smoke (boot-path changed → required):** two-boot warm smoke on a fresh temp data dir, described above;
both boots stopped by specific PID, ports released, zero orphan processes.

**Arm-zero (Step 0, operator window):** Conductor preflight from its own repo root, pulse-app launched
outside it, `ANDROMEDA_PULSE_DATA_DIR` propagated, all evidence in this chunk's folder, nothing written
into Conductor's tree. Feed counters `span_events_seen 6 / fingerprints_computed 6 /
observer_invocations 6` → the recipe's fourth row: **the feed is healthy; the arm's zero came from
elsewhere.** Narrowed to at-or-before cue emission (Changes §Spec claims disproved #5).
