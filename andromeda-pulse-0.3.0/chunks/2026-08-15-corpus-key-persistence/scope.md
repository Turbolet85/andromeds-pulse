# Scope — Corpus key persistence

**Marker:** `2026-08-15-corpus-key-persistence`
**Version:** andromeda-pulse-0.3.0 · **Epoch:** 4 — Polish & ship: verification
**Working-route entry:** "Corpus key persistence — the incident corpus stays readable across process
boundaries, and content written under ephemeral keys is recovered or retired" (operator-directed 2026-08-15)

---

## The defect, as measured

Measured at `2026-08-14-workspace-key-alignment`, not designed:

- `Cargo.lock` resolves `keyring 3.6.3` with dependencies `[log, zeroize]` only — **no platform
  credential-store backend is linked**, so keyring's non-persisting mock store supplies the key.
- Consequence: the corpus AES-256-GCM cell key is **per-process ephemeral**.
- Evidence: first app process — 0 decryption failures; second app process — **13 `decryption_failed`
  warnings against rows its own predecessor wrote**; `cmdkey /list` lists **zero** entries.
- Data-loss implication: every corpus payload written under an ephemeral key is unreadable by any later
  process. Historical encrypted incident / digest content is **orphaned**.
- Blocking relation: the e2e read-back path now REACHES decryption and fails there, so **P-075 and P-076
  sit behind this**.

This defect is **pre-existing**; `2026-08-14-workspace-key-alignment` only unmasked it (before that chunk,
the workspace-key mismatch meant the `WHERE` filter never returned a row to decrypt).

---

## What this chunk builds

### A. Key persistence across the process boundary (the primary fix)

Make the corpus cell key survive a process boundary on a dev host, by linking a real platform
credential-store backend into `keyring`.

- Likely shape: `keyring = { version = "3", features = [<platform-native set>] }` in the workspace
  manifest. `[premise-open: the exact feature names are NOT verifiable offline — `~/.cargo/registry/src/`
  is empty and no `keyring-*.crate` is cached, consistent with `scripts/free-disk.ps1` reclaiming
  `registry/{cache,src}`. Carried to research.md Open Question 1 (implementation-scope): the authoritative
  list comes from the crate at implement time.]`
  **Also in scope, found by research:** the workspace manifest's own comment asserts *"Default features
  auto-select per target"* — the recorded false belief that produced the defect. It is corrected in the
  same edit.
- The consumer surface (`crates/corpus/src/keychain.rs::OsKeychainBackend`, `Corpus::open(path,
  Arc<dyn KeychainBackend>)`) needs **no signature change** — VERIFIED: `contract.rs:101-113` fetches the
  key once through the injected `Arc<dyn KeychainBackend>`, and the graph shows only 2 production
  construction sites (`pulse-app/src/main.rs:327`, `andromeda-pulse-mcp.rs:65`), both already passing the
  identical service name `"com.andromeda.pulse"`. Holds **provided** the fallback composes at the backend
  layer rather than at the `Corpus::open` seam (research.md Open Question 2 — a P4 plan-decision).

### B. Both intended branches reachable, not just the primary (directive 2)

The chunk-#73 P-049 posture is **keychain-primary + `PassphraseFallback`-with-warning**. Today
**neither branch executes**: the primary is served by keyring's mock store, and `BackendKind::
PassphraseFallback` exists as a bounded-enumeration variant with an `as_str()` label but **no backend
produces it** (`crates/corpus/src/keychain.rs` — `OsKeychainBackend::backend_kind()` returns only
Macos/Windows/Linux; `FakeKeychainBackend` returns `InMemoryFake`).

- Platform features make the **primary** real on dev hosts.
- The **fallback** needs its own reachable path AND its own test leg: headless / CI hosts have no
  credential store, and that is exactly where `PassphraseFallback` must engage and emit its warning.
- **Test hygiene constraint (directive 2):** any test that mints a REAL keychain entry must use a
  test-scoped service name and/or clean up after itself. **Never pollute the operator's actual
  credential store.**

### C. Orphaned-content disposition — the fork is "retire HOW", not "recover vs retire" (directive 1)

The working entry's "recovered **or** retired" wording is **narrower in reality**: recovery of PRE-FIX
content is **cryptographically closed**. The mock store is process-local, so every earlier key died with
its process — the 13-failures evidence IS that fact. There is no key to recover with.

The **real design fork** is therefore *how* to retire:

- **Silent purge** — delete undecryptable rows outright.
- **Inventory-preserving tombstones** — table/column metadata stays plaintext by design (arch §Corpus
  SQLite → At-rest posture), so an inventory of orphaned rows is buildable **without any key** (row
  identity, timestamps, workspace, status remain readable; only the BLOB cell payloads are opaque).

Research frames the operator question **that way**; the choice itself is an operator decision surfaced at
P4/P5, not one this scope pre-empts.

**Two research findings sharpen the fork (research.md F1 + F2):**

- **F1 — the disposition is REQUIRED, not optional cleanup.** `cell_decrypt(...)?` propagates inside the
  row loops (`contract.rs:697`, `:741`, `:650`, `:775`), so **a single undecryptable row makes the entire
  query return `Err(DecryptionFailed)`**. Fixing the key alone does **not** restore the read path on an
  existing data dir — which is why P-075/P-076 sit behind the whole chunk, not just the key fix.
- **F2 — the keyless inventory is richer than assumed, and one table is not orphaned at all.** Of the 6
  reserved tables, **5 carry `payload BLOB`**; `service_registry` (`schema.rs:44-53`) has **no BLOB** and
  is fully readable. `incidents` keeps `workspace`, `status`, and four timestamps in plaintext beside the
  opaque payload — so a tombstone inventory is concretely buildable with no key, not a hypothetical.

### C-bis. The fork as RESOLVED at P4 (operator decisions — scope amended per val-1 intent-incomplete)

Both forks were put to the operator at P4 with the research findings above as their basis; both
recommendations were accepted:

1. **Retire-HOW → inventory, then purge.** A third point research surfaced, which dominates both named
   poles: write the keyless audit artifact FIRST (per-table counts plus, for `incidents`, the plaintext
   workspace/status/timestamps that survive without a key), THEN delete the orphaned rows. It preserves
   the inventory the directive wanted, leaves the DB clean so the read paths need no `WHERE` clause, and
   needs **no DDL and no `SCHEMA_VERSION` bump** — unlike in-place tombstones, which would need a marker
   column on all 5 payload-bearing tables. Silent purge was rejected for discarding the only surviving
   evidence of what the ephemeral-key window destroyed.
2. **Read path → harden to skip-and-count** (NEW work, not in the original scope; justified by F1). The
   `?` propagation at `contract.rs:650, :697, :741, :775` becomes skip-the-row + **one aggregate warn per
   query carrying a count only** (no row identity — obs cardinality discipline). This addresses causes the
   disposition does not: key rotation, partial writes, disk corruption. The corpus degrades instead of
   going dark.

### C-ter. Obs allowlist repair (NEW work, not in the original scope; justified by research F3)

`corpus.open.error` — the EXISTING corpus boot-failure diagnostic — has no resolvable allowlist entry, so
its `error_kind` field is redacted today. The one diagnostic that would have named this defect at boot was
itself muted. Repairing it is **required work in this chunk**, independent of the new targets it adds.

### D. Migration / disposition path

Whatever the fork resolves to, the orphaned rows must be dealt with deterministically on a later boot
(not left to fail decryption forever). `[inferred]` — whether this is a one-shot migration at open, a
lazy per-read disposition, or an operator-invoked action is research's question. The 2026-05-20 universal
rule applies: **preserve source data on failure; only remove on FULL success** (session-learnings
2026-05-20 — partial migration emits degraded-success WITHOUT removing the legacy source).

**Research found the precedent to copy:** `pulse-app/src/baseline_persistence.rs:176
migrate_legacy_baseline_if_present`, called once at `main.rs:395` — boot-time, once, outcome-typed
(Completed with a `legacy_file_deleted` flag vs Failed preserving the source). A schema-version hook also
already exists (`crates/corpus/src/schema.rs:113 apply_migrations` over `PRAGMA user_version`) if the
disposition needs a DDL bump; a row-level DELETE/UPDATE disposition does not.

### E. Supply-chain consequences, anticipated not discovered (directive 3)

Platform-native keyring features pull new dependency trees (`windows-sys` class). Any `cargo deny`
finding routes through the **owned-channel discipline** — ID-scoped, reasoned entries naming an owner —
**never silent ignore-widening** (the 2026-05-03 / 2026-05-20 `deny.toml` skip-list-with-provenance
discipline). The plan's Test Commands already carry `cargo deny`; the plan must **anticipate** the churn
rather than discover it at implement.

### F. FIRST STEP — arm-zero classification (folded from the working entry; directive 4)

Operator-gated and **pre-fix by design**. The counter read is log-side and needs **no decryption**, so it
runs BEFORE the fix.

- One Conductor arm re-run under the `buffer.tick` feed counters landed by
  `2026-08-14-fingerprint-feed-capture-repair`.
- The counters discriminate the failure class on sight: `span_events_seen = 0` → nothing reached the
  appender · `fingerprints_computed = 0` with `span_events_seen > 0` → attributes lost in ingest ·
  `observer_invocations = 0` with `fingerprints_computed > 0` → wiring · all three non-zero → the feed is
  healthy and the arm's zero came from elsewhere.
- Expected reading: the incident read **REACHES decryption and fails there** — which is itself the
  confirmation that the `2026-08-14-workspace-key-alignment` publication landed.
- A **non-reproducing arm is a result, not a skip** — record it verbatim if the arm now reads non-zero.
- Recipe: `andromeda-pulse-0.3.0/chunks/2026-08-14-workspace-key-alignment/evidence/arm-zero-classification.md`.
- It discharges the unmet acceptance criterion carried from that chunk.
- **Instrument discipline:** Conductor is the INSTRUMENT, not a session. Run its preflight from its own
  repo root; launch `pulse-app` from OUTSIDE that repo; propagate `ANDROMEDA_PULSE_DATA_DIR` to the
  sidecar. **ALL evidence lands in THIS chunk's folder. Never write into Conductor's tree.**
- Schedule the headful half **with the operator** (the agent cannot schedule it alone).

---

## Boundaries — explicitly NOT in this chunk

- **Not** the workspace-key derivation — that landed at `2026-08-14-workspace-key-alignment` and is
  correct; this chunk does not revisit it.
- **Not** P-075 (Conductor e2e verification closure) or P-076 (integration UX e2e) — they are unblocked
  BY this chunk but remain their own route entries. This chunk **partially advances** them; it does not
  claim them.
- **Not** Conductor-side changes. Conductor's `architecture.md:174` still encodes the now-false
  "incidents `workspace` column = `data_dir`" belief; that rides the operator's Conductor-side directive.
  Nothing in their tree is touched.
- **Not** a re-encryption scheme change — AES-256-GCM cell-level encryption stays as designed
  (`crates/corpus/src/encryption.rs`); only the **key SOURCE** and the orphaned-row disposition change.
- **Not** the deferred `test-plan §3` warm-boot doc-fix family, nor the `agent-latest.jsonl*` bare-name
  family — both are waiting on their own targeted cleanup chunk.

---

## Surfaces + contracts touched (expected)

| Surface | Expectation |
|---|---|
| `Cargo.toml` (workspace) + `Cargo.lock` | `keyring` feature set; new transitive tree |
| `crates/corpus/src/keychain.rs` | `OsKeychainBackend` real backend; `PassphraseFallback` made reachable + its warning |
| `crates/corpus/src/contract.rs` | `Corpus::open` / `open_in_memory` — unchanged **only if** the fallback composes at the backend layer (Open Question 2); note `open` today collapses EVERY keychain error to `Error::KeyringUnavailable`, so there is no fallback branch at this seam |
| `crates/corpus/src/encryption.rs` | **unchanged** — VERIFIED: `cell_encrypt`/`cell_decrypt` take `&EncryptionKey`; the key source is entirely upstream |
| `pulse-app/src/main.rs:327` | keychain-backend construction at boot — **need not change** (graph: 1 of only 2 production construction sites, already passing the right service name); touched only if a boot-time disposition pass is wired, mirroring `migrate_legacy_baseline_if_present` at `:395` |
| `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs:65-66` | the sidecar's `OsKeychainBackend` construction — **the cross-process consumer this chunk exists to make work**. VERIFIED already aligned: same service `"com.andromeda.pulse"`, same account `"corpus-key"` — unlike the workspace key, no identity reconciliation is needed here |
| `deny.toml` | anticipated ID-scoped provenance entries for new transitive deps; the dated-reason + revisit-condition convention is already in force (`deny.toml:7-13`, `:94-98`) |
| obs allowlist (`pulse-app/src/observability.rs`) | **REQUIRED, and broader than assumed** — VERIFIED (research F3): `corpus.open.error` has **no resolvable allowlist entry today** (no exact key, and no bare `"corpus"` key — only exact `"corpus.pipeline_metrics.purge"`), so `for_target` returns `None` and its `error_kind` field **is redacted right now**. The diagnostic that would have explained this defect is itself muted. Sidecar side is fine (bare `"mcp"` key covers `mcp.boot.corpus.init`) |

**Invariant:** the key value itself is NEVER logged (security-plan §Logging; the 2026-08-14
`app.boot.workspace_key` precedent logs `key_bytes` + basename only, never the value).

---

## Capability linkage

The 0.3.0 verification matrix (P-061…P-082) holds **no capability for corpus key persistence** — this is
a defect-fix chunk. It **partially advances** P-075 and P-076 (both `planned`, `chunk:null`) by removing
the blocker, but does not satisfy either acceptance, so both stay `chunk:null` per the matrix contract's
multi-chunk rule. The P-049 posture named in directive 2 belongs to the v0.2.0 capability spec
(`docs/v0_2_0/pulse-capability-spec.md`), not to this version's matrix. **VERIFIED** — the 0.3.0 matrix
holds ids P-061…P-082; 19 `verified`, and exactly 3 `planned`/`chunk:null` (P-075, P-076, P-077). None
names corpus key custody.

---

## Acceptance shape (outcome level; P4 concretizes)

1. A key written by one process is **read back successfully by a later, separate process** — the 13
   `decryption_failed` warnings do not reproduce.
2. The **fallback branch executes and warns** where no credential store exists (headless/CI), rather than
   silently succeeding through a mock.
3. Orphaned pre-fix content is **deterministically disposed of** per the operator's retire-HOW choice —
   never left to fail decryption indefinitely, never silently deleted if tombstones are chosen.
4. `cargo deny` is **green with reasoned, ID-scoped, owner-named entries** for any new finding — no
   blanket ignore-widening.
5. The arm-zero classification is **recorded with its verbatim counter reading** in this chunk's
   `evidence/` folder.
