# Codebase Research — 2026-10-04-corpus-key-creation-is-race-free

## Scope
- **Depth:** moderate · **Reads:** 9 · **Globs/Greps:** 8 (plus 2 measurement probes)
- **Harness rules consulted:** none — the chunk has no `agent-run` / scenario leg. Its live legs are `cargo test` / `cargo nextest` invocations that re-execute the test binary. The design leaned on below leaves `pulse-app/src/main.rs` untouched, so the conditional boot-smoke gate does not fire. If P4 chooses a design that changes the constructor at `main.rs:339-341`, that gate and `.claude/rules/verification-harness.md` come back into scope.
- **Platform issues consulted:** none — no runner-only bullet. The one CI row folded at Setup is `in progress`, not red.

## Files inspected
- `crates/corpus/src/keychain.rs` (1-260, 380-460):
  - `KeychainBackend` trait (`:55-64`): `fetch_or_create_key(&self, service_id)` takes no path.
  - `FakeKeychainBackend` (`:69-101`).
  - `OsKeychainBackend { service, served_by }` (`:126-129`); `new(service)` (`:132`).
  - `fetch_from_os_store` (`:155-172`): get → on `NoEntry` generate → `set_password`, no read-back. Any other error → `Failed`.
  - `resolve_key` (`:175-202`): passphrase fallback only on store error, fail-closed otherwise.
  - Existing cross-process test, built on the re-exec child `child_writes_its_resolved_key` (`:393-406`, env `ANDROMEDA_PULSE_TEST_KEYCHAIN_SERVICE` / `ANDROMEDA_PULSE_TEST_KEY_SINK`):
    - `corpus_key_survives_a_real_process_boundary` (`:413-456`) is sequential: the parent creates the key, then one child reads it.
    - It uses a test-scoped service `andromeda-pulse-test-{pid}-…` and deletes the entry before asserting.
- `crates/corpus/src/contract.rs` (125-160): `Corpus::open(path, keychain)` and `open_in_memory` call `fetch_or_create_key("corpus-key")` and map ANY `KeychainError` to `Error::KeyringUnavailable`. So a new lock-failure arm surfaces through the existing error with no new variant at the `Corpus` boundary.
- `crates/corpus/Cargo.toml` (9-25): deps already include `blake3`, `keyring`, `rand`, `thiserror`, `tracing`; dev-deps `tempfile`. No new dependency is needed for a std file lock.
- `rust-toolchain.toml`: `channel = "1.95.0"`.
- `pulse-app/src/main.rs`:
  - `:183-204` `resolve_data_dir`: `ANDROMEDA_PULSE_DATA_DIR`, else the platform default; on Linux `$XDG_CONFIG_HOME/andromeda-pulse`, else `~/.andromeda-pulse`, else `temp_dir`.
  - `:339-352` the app constructs `OsKeychainBackend::new("com.andromeda.pulse")` and opens `data_dir/corpus/corpus.db`. On error it emits `corpus.open.error error_kind=?e` (non-fatal).
- `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs`:
  - `:28-50` has its own `resolve_data_dir`, the same shape as the app's.
  - `:63-93` constructs the same backend and service. On error it emits `mcp.boot.corpus.init` `reason=corpus_open_failed` (warn) and serves without incident tools.
- `pulse-app/tests/e2e_p3_mcp_resolve_content.rs` (1-30, 70-110):
  - Feature-gated `mcp-server`.
  - `seed_incident(data_dir, …)` opens the corpus with `OsKeychainBackend::new("com.andromeda.pulse")` under a PER-TEST `data_dir`; the sidecar is pointed at it through `ANDROMEDA_PULSE_DATA_DIR`.
  - `pulse-app/tests/e2e_p3_mcp_incident_events_content.rs:112,134` and `crates/mcp-server/tests/incident_events_subprocess.rs:79,110` take the same shape and carry `SKIP_LINE = "[skip] no OS credential store; cross-process corpus unavailable"` (`:35` / `:38`).

## Measured
- **The race reproduces, deterministically enough to be a RED witness (mechanism re-derived at HEAD).**
  - Probe: `scratchpad/race_probe.py`, run against `target/debug/deps/corpus-e815b7eb702ea2c6` (the HEAD `corpus` lib test binary). Each round spawns 8 concurrent `--exact keychain::tests::child_writes_its_resolved_key` children against one fresh test-scoped service, then one sequential child reads what the store holds. It compares key DIGESTS only, never the keys.
  - Result, 5 of 5 rounds: `children 8 exit0 8 sinks 8 distinct 8 returned-key-not-stored 7`. Every child minted its own key and 7 of 8 returned a key the store no longer holds: last write wins. There was no barrier, so plain concurrent spawn is enough.
  - Cleanup: `secret-tool clear service {service} username corpus-key` exit 0 each round. `secret-tool search --all username corpus-key | grep -c race-probe` → 0.
  - The CONTEXT's "mechanism by code reading, not measured as a race" is now measured.
- **The lock primitive decides the case (equality verified).**
  - `rustc 1.95.0`: a probe (`scratchpad/lockprobe.rs`) opened ONE path through TWO handles in one process. With the first holding `File::lock()`, the second handle's `try_lock()` returned `Err` (`true`); after `unlock()` it returned `Ok`.
  - So an exclusive std file lock held across get → generate → set serializes every opener of the same path. Because the exclusion is per handle, not per process, the same lock covers two threads in one process and separate processes alike (Linux `flock`, Windows `LockFileEx`).
  - `File::lock` / `try_lock` / `unlock` are stable on the pinned toolchain.
- **The data dir is NOT the identity the race is keyed on.**
  - The credential entry is `(service = "com.andromeda.pulse", account = "corpus-key")`, one per OS user (`keychain.rs:156`, `contract.rs:137`).
  - Every cross-process test leg opens that SAME entry from a DIFFERENT per-test `data_dir` (the files above). The measured symptom (2026-10-02 `evidence/mutation-checks.md:115-127`) was concurrent seeding processes with distinct data dirs.
  - In production, the app and the sidecar share a data dir only when both resolve it the same way. `ANDROMEDA_PULSE_DATA_DIR` set on one and not the other still leaves them on one credential entry.
  - So the equality "N concurrent first-run processes end on one key" holds ONLY for a lock whose path is a function of the credential entry. A lock under the resolved data dir closes the same-data-dir pair, and leaves open both the measured case and any two processes that resolve different data dirs.

## Graph impact (from the code-graph query; "cold-start — empty DB" if early)
- **derived-without-graph (rust plane):** python `duckdb` is not importable on this host (health check 11; the handoff records the same), so `scripts/code-graph.py query` cannot run. The caller set below comes from name greps over `crates/`, `pulse-app/`, `xtask/`.
- **`OsKeychainBackend::new`** — 2 production construction sites: `pulse-app/src/main.rs:340` and `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs:66`. Test sites:
  - `crates/mcp-server/tests/incident_events_subprocess.rs:79`
  - `pulse-app/tests/e2e_p3_mcp_resolve_content.rs:88`
  - `pulse-app/tests/e2e_p3_mcp_incident_events_content.rs:112`
  - `keychain.rs` tests `:318/:330/:348/:356/:363/:401/:420`
  
  Derivation: `grep -rn 'OsKeychainBackend' --include=*.rs .` minus `target/`. A design that keeps `new(service)` unchanged touches none of them.
- **`KeychainBackend::fetch_or_create_key`** — production callers are `Corpus::open` / `open_in_memory` only (`contract.rs:137,153`). Every other hit is `FakeKeychainBackend` test wiring (`grep -rn 'fetch_or_create_key\|KeychainBackend'`). The trait signature need not change.
- **`Error::KeyringUnavailable`** — 14 files consume it (`grep -rln 'Error::KeyringUnavailable'`: `contract.rs` plus 8 `pulse-app/src/*` adapters plus 5 `pulse-app/tests/unit_*`). It is untouched if the lock failure maps into the existing `KeychainError` → `KeyringUnavailable` path.

## Patterns detected
- **Re-exec child for a real process boundary** (`keychain.rs:393-456`): an env-gated child test that is inert in an ordinary run, a parent that re-runs `current_exe()` with `--exact`, a per-run TempDir sink, the credential entry deleted before the assert, and `[skip]` when the store answers with an error. The concurrent witness extends exactly this, with N children and a barrier instead of 1 sequential child.
- **Fail-closed custody** (`keychain.rs:118-123`, `:175-202`): a store error without a passphrase returns the store's error and never a key. A lock acquire/open failure belongs on this side, mapped into `KeychainError`.
- **One boot record per failed open** already exists: app `corpus.open.error` (`main.rs:347-351`) and sidecar `mcp.boot.corpus.init` (`andromeda-pulse-mcp.rs:84-90`). A lock failure surfaces through them as `KeyringUnavailable`, so no new `corpus.*` target is needed (the obs extract's no-new-target arm).
- **Clean skip that names itself** (`SKIP_LINE` in the two incident-events legs; `eprintln!("[skip] …")` in `keychain.rs:423`).

## Conventions to follow
- **Co-located corpus tests**: `#[cfg(test)] mod tests` in `crates/corpus/src/keychain.rs` (`:280+`), no `pulse-app` lib-src test (ratchet).
- **ASCII-only source**: `cargo xtask check:english-sources` (CI lint-test plus pre-push).
- **No key bytes in output**: the existing child writes the hex to a TempDir sink the parent reads, never to stdout. A new witness compares in-process and prints at most counts.
- **Path in logs is basename only**: any lock path in an error or log uses a basename. The error enum carries no path today (`KeychainError` has unit variants, `keychain.rs:15-23`).

## New files to create
- none

## Files to modify
- `crates/corpus/src/keychain.rs` — the locked create-or-read inside `OsKeychainBackend` (lock path derivation + acquire around get → generate → set → read-back) and the co-located concurrent re-exec witness plus its fail-closed negative arm

## Open questions
- Lock location: per-user, keyed on the credential entry identity (closes every measured and production case, but is a second on-disk artifact OUTSIDE the resolved data dir after the `~/Downloads` export sink; Linux `$XDG_RUNTIME_DIR` is per-user 0700, with `temp_dir` as the fallback, which is shared `/tmp` on Linux and per-user on macOS and Windows) vs under the resolved data dir (arch's default home for new artifacts; does not close the measured case) → blocks: plan-decision
