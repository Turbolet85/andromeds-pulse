
## 2026-10-10-shared-telemetry-test-data — `test-support` at the three library-crate enumerations; seventeen members
**Section:** §2 Test Strategy (Test pyramid, Unit row) · §4 Unit Test Strategy (Framework; Test file location; What unit tests cover)
**Change:**
- §2 Unit row: the per-crate list gains the dev-only `test-support`.
- §4 Framework: was "inherited by all 16 members"; now 17.
- §4 Test file location: the co-located LIBRARY-crate list gains `test-support` (`crates/test-support/src/otlp.rs` and `ports.rs`).
- §4 What unit tests cover: a new `test-support` bullet — the `otlp` factories (10 tests: equal builds, the fixed default instant and service, id lengths and positional span ids, set values read back), the `ports` claim (4 tests: the range, two live claims differ, a live claim's lock file refuses a second lock, a claimed port binds), its edges pinned from `xtask` (`test_support_edges`, 5 tests), and the write-boundary reading of its defaults in `buffer`'s `consumer::tests::a_default_built_batch_of_every_kind_lands_whole_with_no_redaction` (`rows_ingested` equal to the rows sent, `redactions_applied` 0, `append_rejections` 0).
**Why:** The chunk added a library crate that carries unit tests; the three enumerations and the member count did not name it.
**Kept:** The §2 and §4 lists still omit `curation` and `config-watcher`, as they did before this chunk; not this chunk's.
**Ref:** .andromeda/runs/2026-10-10T22-04-24Z-wrap/
