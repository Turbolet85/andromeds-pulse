# Codebase Research — 2026-08-15-corpus-key-persistence

## Scope
- **Depth:** moderate · **Reads:** 8 · **Globs/Greps:** 11 · **Graph queries:** 4

---

## Files inspected

- `Cargo.toml` (workspace, lines 150–180) — the `keyring = "3"` declaration and its comment block. **The comment states the false belief that produced the defect**: *"Default features auto-select per target"*. That is the root cause in one line — keyring 3.x does NOT ship platform backends in its default feature set, so the declaration resolves to the crate's non-persisting fallback store.
- `crates/corpus/Cargo.toml` — `keyring.workspace = true`; dev-deps already carry `tempfile` + `rstest` + `proptest` (no new dev-dep needed for the test legs).
- `crates/corpus/src/keychain.rs` (full, 256 lines) — `KeychainBackend` trait (`fetch_or_create_key` + `backend_kind`), `FakeKeychainBackend`, `OsKeychainBackend`, `BackendKind` (5 variants incl. `PassphraseFallback`), hex `encode_key`/`decode_key`.
- `crates/corpus/src/contract.rs` (85–154, 706–755) — `Corpus::open` / `open_in_memory`; the decrypting read paths.
- `crates/corpus/src/encryption.rs` (1–75) — `EncryptionKey` wrapper, `cell_encrypt`, `cell_decrypt`.
- `crates/corpus/src/schema.rs` (20–99) — the 6 reserved tables' DDL; which columns are BLOB.
- `pulse-app/src/main.rs` (315–360) — boot-time keychain construction + non-fatal corpus open.
- `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs` (45–89) — the sidecar's mirror construction + non-fatal warn.
- `pulse-app/src/observability.rs` (`for_target` resolver + allowlist key survey) — see Graph/finding below.
- `crates/mcp-server/src/tracing_setup.rs` (50–80) — the sidecar's `"mcp"` allowlist entry.
- `deny.toml` (section survey) — `[graph] [advisories] [licenses] [bans] [sources]`, each with a dated, reasoned provenance comment block.

---

## Graph impact (`.andromeda/runs/2026-08-15T11-43-46Z-phase/tree-query-2026-08-15-corpus-key-persistence.json`)

- **`OsKeychainBackend`** — exactly **2 production construction sites**, and they are the two processes at issue:
  `pulse-app/src/main.rs:327` and `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs:65` (plus its line-5 import). **Both pass the identical service name `"com.andromeda.pulse"`.** The blast radius of swapping the backend is these two lines, or zero lines if the swap happens inside `keychain.rs`.
- **`fetch_or_create_key`** — production callers are exactly **`crates/corpus/src/contract.rs:102` and `:118`** (`Corpus::open` / `open_in_memory`); every other reference is inside `keychain.rs`'s own test module. The key is fetched **once per `Corpus::open`** with the account id `"corpus-key"` and lives for the corpus lifetime.
- **`crate_edges` for `corpus`** — inbound `mcp-server → corpus` and `pulse-app → corpus`; **no outbound edge to a sibling** beyond its declared deps. The DAG stays intact for a change confined to `corpus`.

---

## Patterns detected

- **Trait-injected backend, single fetch point** (`crates/corpus/src/contract.rs:101-113`): `Corpus::open(path, Arc<dyn KeychainBackend>)` calls `fetch_or_create_key("corpus-key")` and maps **every** keychain error to `Error::KeyringUnavailable`. There is **no fallback branch anywhere** — `BackendKind::PassphraseFallback` is a label with no producer (`keychain.rs:45` defines its string; `OsKeychainBackend::backend_kind()` at `:139-147` returns only Macos/Windows/Linux by `cfg!`, and `FakeKeychainBackend` returns `InMemoryFake`). This confirms the directive-2 premise exactly: **neither intended branch executes.**
- **Non-fatal corpus open in both processes** (`pulse-app/src/main.rs:336-343` `tracing::error!(target: "corpus.open.error", error_kind = ?e, …)`; `andromeda-pulse-mcp.rs:85-90` `tracing::warn!(target: "mcp.boot.corpus.init", reason = "corpus_open_failed", …)`). A fallback that returns `Err` degrades to "corpus absent", not a crash — so the fallback must **succeed with a warning**, not fail.
- **Migration precedent** (`pulse-app/src/baseline_persistence.rs:176 migrate_legacy_baseline_if_present`, called once at `main.rs:395`): boot-time, once, outcome-typed (Completed with a `legacy_file_deleted` flag vs Failed preserving the source). This is the shape any disposition pass should copy, and it is already the project's session-learnings 2026-05-20 rule.
- **Schema versioning hook exists** (`crates/corpus/src/schema.rs:113 apply_migrations`, `PRAGMA user_version`, `SCHEMA_VERSION = 1`, called from `db.rs:27,36`): a disposition that needs a schema bump has a home; a disposition that only DELETEs or UPDATEs rows does not need one.
- **`deny.toml` provenance convention** (`deny.toml:7-13, 94-98`): every ignore/skip carries a dated reason, a revisit-condition, and an explicit note about which finding is deliberately NOT listed. This is the "owned channel" the directive-3 discipline names, already in force.

---

## Conventions to follow

- **Bounded-enum log labels**: `BackendKind::as_str()` (`keychain.rs:40-48`) is the existing cardinality-safe label surface; extend the enum's use, not free-form strings.
- **Key never in Debug**: `EncryptionKey`'s hand-written `Debug` prints `len: 32` only (`encryption.rs:42-48`); `Corpus`'s `Debug` prints `path_basename` only (`contract.rs:87-94`). Any new type holding key material follows this.
- **pulse-app probes go in `pulse-app/tests/*.rs`** (`[lib] test = false`) — a `mod tests` in `pulse-app/src/` compiles under clippy but never runs.
- **corpus tests are co-located** `#[cfg(test)] mod tests` under `crates/corpus/src/` (the existing `keychain.rs:183-256` block).

---

## Findings that change the plan

### F1 — One orphaned row fails the WHOLE read, not just its own
`cell_decrypt(...)?` propagates inside the row loops: `contract.rs:697` (multi-row active-incident load, inside the loop before `result.push`), `:741` (`load_incident_by_id`), `:650`, `:775`. **A single undecryptable row makes the entire query return `Err(DecryptionFailed)`** — the caller then logs its sanitized `"decryption_failed"` string (`pulse-app/src/corpus_retrieval.rs:138`, `digest_runtime.rs:284`, `drain_persistence.rs:96`, `storage_router.rs:267`).

Consequence: the orphaned-content disposition is **not optional cleanup** — until orphaned rows are gone or skipped, every read touching them fails, so **fixing the key alone does not restore the read path** on an existing data dir. This is why P-075/P-076 sit behind the whole chunk, not just behind the key fix.

### F2 — A keyless orphan inventory is richer than assumed, and one table is not orphaned at all
Of the 6 reserved tables, **5 carry `payload BLOB`** (`baseline_state`, `pipeline_metrics`, `incidents`, `incident_events`, `digest_archive`) — `service_registry` (`schema.rs:44-53`) has **no BLOB at all**, so it is fully readable and **not orphaned**. And `incidents` (`schema.rs:64-74`) keeps `workspace`, `status`, and four timestamps in **plaintext** columns alongside the opaque payload.

Consequence: the tombstone/inventory option is concretely buildable with **no key** — id, workspace, status, created/updated/resolved/read timestamps all survive. This is a real, measured basis for the operator's retire-HOW choice rather than a hypothetical.

### F3 — The existing corpus boot-failure diagnostic is being silently redacted TODAY
`AllowList::for_target` (`pulse-app/src/observability.rs`) resolves exact → strip `.tick` → `split('.').next()` → `split("::").next()` → `None`. For `corpus.open.error`: no exact key, no `.tick`, prefix `"corpus"` — and **there is no bare `"corpus"` allowlist key** (only exact `"corpus.pipeline_metrics.purge"` at :2143). So it returns `None` and **`error_kind` is redacted in the log today** — the precise `for_target` prefix-fallback trap the obs extract and the 2026-08-14 `app.boot.workspace_key` amendment both warn about. The sidecar side is fine: `tracing_setup.rs:56-58` has a bare `"mcp"` key whose set includes `reason`, covering `mcp.boot.corpus.init`.

Consequence: this chunk must add the allowlist leaf for `corpus.open.error` regardless of what new targets it introduces — the diagnostic that would have explained the defect is itself muted.

### F4 — Service + account identity is already aligned across processes
Both constructions pass `"com.andromeda.pulse"` (`main.rs:328`, `andromeda-pulse-mcp.rs:66`) and `Corpus::open` uses account `"corpus-key"` for both. So **no identity reconciliation is needed** — unlike the workspace key, this half is already correct. Only the backend is missing.

### F5 — The exact `keyring` feature names cannot be verified offline
`~/.cargo/registry/src/` is empty and no `keyring-*.crate` is cached (consistent with the `scripts/free-disk.ps1` routine reclaiming `registry/{cache,src}`). The feature names in directive 3 (`windows-native` / `apple-native` / `sync-secret-service` class) are therefore **plausible but unverified at plan time**; the authoritative list comes from the crate at implement time. See Open Questions.

---

## Files to modify

- `Cargo.toml` (workspace) — `keyring` gains an explicit platform feature set; **the stale "Default features auto-select per target" comment must be corrected in the same edit** (it is the recorded false belief).
- `Cargo.lock` — new transitive tree (regenerated, committed).
- `crates/corpus/src/keychain.rs` — make the primary real and `PassphraseFallback` reachable. Graph says this is the only file that must change if the fallback composes at the backend layer (see Open Question 2).
- `pulse-app/src/observability.rs` — allowlist leaf for `corpus.open.error` (F3) plus any new keychain/fallback target.
- `deny.toml` — anticipated ID-scoped provenance entries for the new transitive tree (F3-adjacent; directive 3).
- `crates/corpus/src/contract.rs` — **only if** the disposition or the fallback needs a change at the open/read seam (Open Question 2 + the disposition design).
- `pulse-app/src/main.rs` — **only if** a boot-time disposition pass is wired (mirroring `migrate_legacy_baseline_if_present` at `:395`); the keychain construction at `:327-329` itself need not change.

**Caller threading:** the graph's caller set for the changed surface is exactly the two `OsKeychainBackend` construction sites (`main.rs:327`, `andromeda-pulse-mcp.rs:65`) and the two `fetch_or_create_key` call sites (`contract.rs:102`, `:118`). All four are enumerated above; there is no wider plumbing to thread. `keyring` is a **normal** (not dev) workspace dependency of `corpus` (`crates/corpus/Cargo.toml:16`), so it can back a shipped signature.

## New files to create
- A `crates/corpus/src/` module or an added backend type inside `keychain.rs` for the fallback — shape decided at P4; no new crate (arch extract's ban).
- `andromeda-pulse-0.3.0/chunks/2026-08-15-corpus-key-persistence/evidence/` — the arm-zero classification output (scope §F).

---

## Open questions

1. **Exact `keyring` 3.x platform feature names + whether they are additive to defaults or require `default-features = false`** → blocks: **implementation-scope**. Unverifiable offline (F5); resolved at implement by the crate's own manifest/`cargo tree`. The plan's file list is firm; only the literal feature strings are provisional.
2. **Does the fallback compose inside `keychain.rs` (a wrapping backend that tries OS then passphrase, so `contract.rs` is untouched), or at the `Corpus::open` seam (which today collapses every error to `KeyringUnavailable`)?** → blocks: **plan-decision**. P4 resolves; the backend-layer composition is the lower-blast-radius option and keeps the arch extract's "no signature change" expectation true.
3. **Retire-HOW: silent purge vs inventory-preserving tombstones** → blocks: **plan-decision**, and it is the operator's call per directive 1. F1 (one bad row fails the whole read) and F2 (a rich keyless inventory exists) are the two facts that decide it.
