# tests extract

## Relevance
relevant — the chunk's deliverable is a scrubber-arm precision change whose correctness is carried almost entirely by the `security` crate's recall + false-positive test corpora, plus one end-to-end pin of the workspace-key form.

## Constraints
- Test tier is Standard; unit level carries the weight — the `security` crate's unit coverage is where the arm change must be proven (per test-plan §1 Test Scope Summary; §2 Test Strategy test pyramid).
- test-plan §4 (security crate row) requires the P-047 catalog to be pinned by co-located `#[rstest]` recall AND false-positive corpora in `crates/security/src/scrubber.rs`, with the false-positive half treated as a "first-class guard", and `scrub_attribute` + the `ScrubbedValue` contract asserted unchanged; this chunk's new pins land in those corpora, not in a new file. Whether the existing corpus already contains a date-time-stamp or path-shaped digit run is research's question.
- Every plan's `## Test Commands` MUST carry the unconditional standard gate set (fmt · clippy `--all-targets --all-features -D warnings` · `nextest --workspace --profile ci` · `capability-widening-check` · `check:ingest-progress` · `check:staged-artifacts` · `capability-drift` LAST); webview npm gates only if `pulse-app/ui/**` is touched (per test-plan §3 Per-chunk gate discipline).
- Any `pulse-app` test narrowing uses `--workspace` + `-E`/`--filter-expr`, not `-p pulse-app` (per test-plan §3 `run`); pulse-app unit-shaped probes live in `pulse-app/tests/*.rs`, never as `#[test]` in `pulse-app/src/` (the zero-baseline ratchet reds the build) (per test-plan §2 Test directory + naming conventions; §4 Conventions).
- Coverage gate is ≥75% line / ≥70% branch / ≥85% function (xtask temporarily excluded); new arm logic (e.g. a Luhn/length/separator check) needs branch coverage of each reject path, not just the accept path (per test-plan §10 Quality Gates & Coverage Targets; §4 Coverage target).
- If property-based tests are used for the card check, generators MUST be seeded deterministically with regression files in `proptest-regressions/` (per test-plan §7 Test Data & Fixtures — Randomized data; §11 Test Data). Property testing is selective per trigger, not mandated here (per test-plan §2 Test Strategy).
- Boot-smoke gate is conditional on touching a boot/setup path; a change confined to `crates/security/src/scrubber.rs` does not trigger it — whether the end-to-end workspace-key pin requires touching one is research's question (per test-plan §3 Per-chunk gate discipline — Boot-smoke gate).

## Patterns to follow
- Paired recall/selectivity pins at a scrub site: the `buffer` label precedent pins a credential canary as redacted AND a benign + high-entropy control (drawn from the scrubber's own false-positive corpus) as `Allowed` byte-identical — the same two-direction shape the scope asks for (per test-plan §4 buffer crate row, `metrics_points.labels` clause).
- `rstest` parameterized cases (`#[rstest]` + `#[case]`/fixtures) for corpus rows rather than one test per literal (per test-plan §4 Fixture pattern at unit level; §7 Fixture library).
- Consumer-inherited change verified at the consuming write boundary: `buffer`'s `appender::tests` per-column redaction pins are the existing downstream carriers that must stay green when the arm changes (per test-plan §4 buffer crate row).
- Workspace-key surface is already a pinned contract in `workspace-detector` (derivation, `publish_workspace_key` round-trip, bounded-read validation); an end-to-end pin of the key's scrubbed form should reuse that derivation rather than hand-build a key string (per test-plan §4 workspace-detector crate row; §5 cross-process key handoff boundary row).

## Anti-patterns to avoid
- NEVER test private crate-internal helpers (e.g. a private Luhn fn) in isolation as the only proof — assert through the public `scrub_attribute` boundary and its `ScrubbedValue` contract (per test-plan §11 Unit).
- NEVER use production telemetry dumps as fixtures — the measured Conductor value `rm-20260923-093840` is a synthetic literal and belongs in the corpus as such, never a captured log/corpus extract (per test-plan §11 Test Data); and NEVER expose real-shaped secrets in test output beyond the committed canary literals (per test-plan §11 Universal).
- NEVER lower a coverage threshold or add `#[ignore]` without a tracked issue to land the chunk (per test-plan §11 CI; §11 Quality).

## Contract bindings
- tests ↔ security: the false-positive corpus in `scrubber.rs` is the bound on catalog precision named by the security rule (§Logging & redaction); test-plan §4 security row is the test-side statement of the same contract — a wrap amendment to security-plan P-047's card-arm description would pair with a test-plan §4 security-row update (crate suite count).
- tests ↔ obs/harness: the end-to-end PROJECT-line form, if verified live, runs under the direct-binary smoke form with `ANDROMEDA_PULSE_L4_DETERMINISTIC` and fresh `ANDROMEDA_PULSE_DATA_DIR`, evidence read from `<data_dir>/logs/agent-latest.jsonl*` (per test-plan §3 Direct-binary smoke variant).
- Hazard for research: if the scrub site on the workspace-key → PROJECT-line path sits in a resolver that links the tauri clipboard/notification chain, a test binary linking it aborts at load — the documented `snapshot-resolver-level-coverage` gap (per test-plan §1 Pending coverage triggers); the pin may have to target the library-level function instead.

## Acceptance criteria contributions
- `cargo nextest run --workspace --profile ci` green, with the `security` crate's recall corpus unchanged-and-passing (incl. `4532-1234-5678-9010` → `credit_card`) and new false-positive `#[rstest]` cases for `rm-20260923-093840` and its workspace-key-embedded form returning `ScrubbedValue::Allowed` (per test-plan §4 security crate row).
- Each new rejection branch of the card check (length / checksum / separator / context, whichever research selects) is discriminating: a mutation removing it reds at least one committed pin, not only the accept-side pins (per test-plan §10 Quality Gates & Coverage Targets; §11 Quality).
- The full standard gate set passes, `cargo xtask capability-drift` last (per test-plan §3 Per-chunk gate discipline).
- Workspace coverage stays at or above ≥75% line / ≥70% branch / ≥85% function (per test-plan §10 Quality Gates & Coverage Targets).
