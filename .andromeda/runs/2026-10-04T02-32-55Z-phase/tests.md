# tests extract

## Relevance
relevant: the chunk's main deliverable is a cross-process race witness that must go RED at HEAD, and it changes a `corpus` crate boundary that the plan's corpus unit-strategy row and its feature-gated cross-process MCP legs already cover.

## Constraints
- Cross-process key behaviour MUST be witnessed by a test that re-executes its own test binary, never by an in-process two-instance check. Per test-plan §4 Unit Test Strategy → "What unit tests cover", corpus crate row: an in-process check passed while the 2026-08-15 defect was live, because keyring's non-persisting store was process-global. The same row requires the leg to skip cleanly where no credential store exists, and any real entry it mints to use a test-scoped service name plus cleanup, so the operator's store is never polluted. Applied here, the witness's barrier-synchronized children each resolve the key against one test-scoped service whose entry is ABSENT at the start.
- The re-exec shape follows test-plan §3 Test Harness Contract → "Process-end witness form (integration tier)". The parent re-runs the binary with `--exact <fn>` and passes each child's arm and role through the CHILD's environment. Proof that a child ran must be IN-ARM, because child stdout is discarded. Each child therefore reports through an artifact the parent reads after the child ends, such as its exit code or a file under its own TempDir. A missing child artifact fails the arm and never counts as agreement.
- Decide what the RED leg produces BEFORE choosing its evidence (test-plan §3 → "Direct-binary smoke variant", shape rule). Divergent minted keys emit no ERROR and no panic, so a clean log proves nothing. The evidence of record is the count of distinct keys the children return, plus whether each one equals the key the store holds afterwards. How often HEAD actually reproduces more than one key is P3's premise-closure question. The plan cannot assert it.
- The witness must be deterministic. Test-plan §2 Test Strategy → "Agent-runnable invariants" bans flaky retries and real-time synchronization, and §11 → E2E bans `sleep(N)` for sync. Children line up on an explicit barrier, such as a file or pipe rendezvous the parent releases, and never on a timed delay. The GREEN arm must pass on every run, not most runs.
- Test placement:
  - Tests for `corpus` changes are co-located `#[cfg(test)] mod tests` inside the library crate (test-plan §2 → "Test directory + naming conventions"; §4 → Conventions).
  - Any `pulse-app` leg goes in `pulse-app/tests/*.rs` and never in `pulse-app/src/`: the lib-src `#[test]` ratchet `pulse_app_src_carries_no_new_dead_test_attributes` is a flat zero.
- The standard gate set is unconditional (test-plan §3 → "Per-chunk gate discipline"). Three conditions for this chunk:
  - The webview gates drop only if no `pulse-app/ui/**` path is touched.
  - With no procedure added, the sequence closes on `git diff --quiet <chunk-base> -- pulse-app/ui/src/bindings/index.ts`, and `capability-drift` runs before the workspace nextest.
  - If the constructor change reaches `pulse-app/src/main.rs` (boot builds `OsKeychainBackend`), the conditional boot-smoke gate is also required. Whether boot is touched is P3/P4's call.
- The cross-process MCP legs that open the corpus with `OsKeychainBackend::new("com.andromeda.pulse")`, `e2e_p3_mcp_resolve_content` and `e2e_p3_mcp_incident_events_content`, are FEATURE-GATED. They sit outside the default `--workspace` gate and clean-skip on every CI runner, which has no OS credential store (test-plan §1 → Pending coverage triggers, `mcp-incident-read-back-cross-process-coverage`; §9 CI Integration → `mcp-test` job). A green CI therefore says nothing about the race. The evidence is a run on the Linux dev host where the output contains no `[skip]` line.

## Patterns to follow
- **Re-exec child arms.** `pulse-app/tests/integration_exit_cause_record.rs` (test-plan §3 → "Process-end witness form") is the shape: `--exact` plus an env-selected arm, a per-child TempDir, child-ran proof inside the arm, and the parent asserting only after the children end. The existing corpus cross-process key-persistence test (§4, corpus row; chunk 2026-08-15-corpus-key-persistence) is the in-crate precedent this witness extends from "persists across processes" to "concurrent creators agree".
- **Clean skip that names itself.** Where no store exists, the leg prints a `[skip]` line and returns. The positive proof on a host that has a store is that the output contains no `[skip]` (test-plan §1, `mcp-incident-read-back-cross-process-coverage`, which records "`lacks [skip]` held, 16/16" on the Linux dev host).
- **Real filesystem, owned traits.** A file lock is exercised on a real `tempfile::TempDir`, never a mocked filesystem (test-plan §8 → "What to mock", Filesystem). If the test uses a store it controls instead of the OS store, the substitute sits behind a trait the project owns and is injected through the constructor (§8 → Anti-monkey-patching; §11 → Mocking: "mock only traits you own").
- **Mutation-check the witness.** Restoring HEAD's unlocked get-then-set must turn the witness RED. A witness that stays green under that mutation passes for the wrong reason. The plan applies this practice to every shipped guard; see the §4 pins described as "mutation-checked".

## Anti-patterns to avoid
- Never share mutable state across parallel tests (test-plan §11 → Test Data). The real OS credential store under the production service id `com.andromeda.pulse` is exactly that shared state, and it is the measured order-sensitivity on a fresh store. Every new leg uses its own test-scoped service name and cleans up after itself.
- No retry-once policy and no ignored flaky tests (test-plan §11 → Quality and CI). A race witness that is green "most runs" under the fix is a real bug, not a flake to quarantine.
- Never `std::env::set_var` in the parent mid-test without cleanup (test-plan §11 → Mocking). Arm, role and service selection go into the CHILD's `Command` environment.

## Contract bindings
- **tests ↔ security:**
  - The witness compares keys without printing key material. A test may assert equality, or compare a digest, but never echo the 32-byte key or its hex (test-plan §11 → Universal: "NEVER expose secrets in test output"; security rules §Logging & redaction).
  - A lock-file path, if one appears in output, appears by basename only.
  - The fail-closed arm is a required negative leg: a store error with no passphrase, or a lock that cannot be acquired, surfaces an error and never yields a key. This is test-plan §4's corpus-row "store's error surfaces when no passphrase is configured" pin, extended to the lock.
- **tests ↔ obs:** if P4 adds a tracing record for lock acquisition or failure, the `pulse-app/tests/unit_observability_allowlist_*` sweep is where it is pinned. Whether such a record is added belongs to obs and P4.
- **tests ↔ CI:**
  - The feature-gated MCP legs run only in the `mcp-test` job, `cargo nextest run --workspace --features mcp-server` (test-plan §9).
  - CI runners have no credential store, so the race witness's non-skip proof is dev-host evidence, not CI evidence.

## Acceptance criteria contributions
- (tests) A committed re-exec witness, in which barrier-synchronized child processes resolve the key against one initially empty, test-scoped store:
  - It goes RED at the chunk base with more than one distinct key, and GREEN after the fix: every child returns the same key, and that key equals the stored key.
  - Every arm carries in-arm child-ran proof.
  - It prints `[skip]` where no store exists and runs on the Linux dev host with no `[skip]` line. Restoring the unlocked get-then-set turns it RED.
  - Per test-plan §4 (corpus row) and §3 ("Process-end witness form").
- (tests) `cargo nextest run -p corpus` passes, and so does the full standard gate set, including `cargo nextest run --workspace --profile ci`. The run closes on bindings byte-identical to the chunk base (per test-plan §3, "Per-chunk gate discipline").
- (tests) `cargo test -p pulse-app --features mcp-server --test e2e_p3_mcp_resolve_content --test e2e_p3_mcp_incident_events_content` passes on the dev host against a FRESH credential store, with no `[skip]` line. It also passes in either run order, which removes the order-sensitivity the scope names (per test-plan §1, `mcp-incident-read-back-cross-process-coverage`). Which legs share the store is P3's to confirm.
- (tests) Workspace coverage holds at ≥ 75% line, ≥ 70% branch and ≥ 85% function, with the new lock path's failure branch covered (per test-plan §10, Coverage thresholds).
