# Mutations of the edge pin — 2026-10-10-shared-telemetry-test-data

Run once at /implement, by hand, on the working tree over chunk base `50216d7e`. Subject: the xtask test module
`test_support_edges` (`xtask/src/test_support_edges.rs`), five tests. Command for every reading:
`cargo nextest run -p xtask --profile ci --no-tests=fail -E 'test(test_support_edges)'`.

## Red first (plan step 2), before any member named the set
State: `crates/test-support` in the root member list with its manifest and `src/lib.rs`; no member's
`[dev-dependencies]` named it yet.

- exit 100 · `Summary … 5 tests run: 4 passed, 1 failed, 478 skipped`
- FAIL `test_support_edges::test_support_is_a_member_that_some_member_uses`, its panic line:
  `no member names test-support under [dev-dependencies]: the pins above would pass over a set nobody uses`
- PASS the other four (`no_member_names_test_support_outside_its_dev_dependencies`,
  `test_support_depends_on_ingest_alone`, and the two that read the module's own parsing).

## Mutation 1 — a member names the set outside its dev-dependencies (red on (a))
Edit: in `xtask/Cargo.toml` the line `test-support = { path = "../crates/test-support" }` moved from
`[dev-dependencies]` to the end of `[dependencies]`. The mutated tail was read back before the run.

- exit 100 · `Summary … 5 tests run: 4 passed, 1 failed, 478 skipped`
- FAIL `test_support_edges::no_member_names_test_support_outside_its_dev_dependencies`, its panic line:
  `xtask names test-support under "[dependencies]": "test-support = { path = \"../crates/test-support\" }"`
- PASS the other four. `test_support_is_a_member_that_some_member_uses` stays green: `pulse-app` and `buffer`
  still name the set under their dev-dependencies.

Restored by the reverse edit. `sha256(xtask/Cargo.toml)` before the mutation and after the restore: `5f0f4c08…f3d375`,
equal.

## Mutation 2 — the set gains a second dependency (red on (b))
Edit: `prost.workspace = true` added under `[dependencies]` of `crates/test-support/Cargo.toml`. The mutated table was
read back before the run.

- exit 100 · `Summary … 5 tests run: 4 passed, 1 failed, 478 skipped`
- FAIL `test_support_edges::test_support_depends_on_ingest_alone`, its panic lines:
  `crates/test-support depends on the crate whose types it builds, and on nothing else` ·
  `left: ["ingest", "prost"]` · `right: ["ingest"]`
- PASS the other four.

Restored by the reverse edit. `sha256(crates/test-support/Cargo.toml)` before and after: `7241629b…d62487`, equal.

## A third reading, not asked by the plan — the default-built pin reads the defaults
Edit: the factory's default log body in `crates/test-support/src/otlp.rs` changed from `test log record` to
`test log record password=hunter2`. Command:
`cargo nextest run -p buffer --profile ci --no-tests=fail -E 'test(default_built) | test(dispatch_folds_redactions_after_a_successful_append)'`.

- exit 100 · `Summary … 2 tests run: 1 passed, 1 failed, 187 skipped`
- FAIL `consumer::tests::a_default_built_batch_of_every_kind_lands_whole_with_no_redaction`:
  `no default is masked` · `left: 3` · `right: 0` (the batch holds three default log records)
- PASS `consumer::tests::dispatch_folds_redactions_after_a_successful_append` (the set canary, one redaction).

Restored by the reverse edit. `sha256(crates/test-support/src/otlp.rs)` before and after: `28d1e29c…57e109f`, equal.

## The tree after the three restores
- `sha256(Cargo.lock)` before the first mutation and after the last restore plus one more cargo run:
  `07850078…3bc64c1`, equal (mutation 2 made cargo add `prost` to the member's lock entry; the restore took it out).
- `grep -c 'hunter2\|^prost'` over the member's `otlp.rs` and `Cargo.toml`: 0 and 0.
- The pins on the restored tree, `cargo nextest run --workspace --profile ci --no-tests=fail -E 'package(test-support) |
  (package(buffer) & test(default_built)) | (package(xtask) & test(test_support_edges))'`: exit 0 ·
  `Summary … 20 tests run: 20 passed, 666 skipped` (14 of the member, 1 of `buffer`, 5 of xtask).

## Limits
- One run per reading. Each mutation changes one line; no mutation of (c) other than the red-first state was made.
- The pin reads manifests as text. A dependency reached through a `[target.…]` table of another member, or through a
  renamed `package = "test-support"` key under another name, names the set on its own line and is read; a manifest
  outside the root member list is not read.
