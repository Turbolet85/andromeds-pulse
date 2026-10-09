# Mutation checks and RED readings — 2026-10-04-declared-rust-floor-matches-the-code

One-shot controls run by /implement (plan Steps 1 and 7). Each mutation was applied, confirmed present by
grep before its run, run alone (never in the same batch as its Edit), then reverted and the revert confirmed
(`git diff --stat` on the file, plus a grep). Test runs used `cargo nextest … --profile ci` on the Linux dev host.

## Step 1 — RED at base (the CARRY witness against the untouched skip arm)
- State: `corpus_key_skip_arm_leaves_no_lock_file` written; the skip arm of
  `corpus_key_survives_a_real_process_boundary` unchanged (no cleanup).
- Command: `cargo nextest run -p corpus --profile ci -E 'test(corpus_key_skip_arm_leaves_no_lock_file)'`
- Exit 100. `FAIL keychain::tests::corpus_key_skip_arm_leaves_no_lock_file`,
  `left: (true, 1)` / `right: (true, 0)` — the child reached the `[skip]` arm (first element true) and left
  exactly one `andromeda-pulse-corpus-key-*.lock` in its `XDG_RUNTIME_DIR` TempDir. Matches research.md's
  predicted RED leg.

## Step 7 (a) — declared floor `1.89`
- Mutation: `Cargo.toml` `rust-version = "1.95"` -> `"1.89"` (grep: `25:rust-version = "1.89"`).
- Command: `cargo nextest run -p xtask --profile ci -E 'test(declared_floor_equals_the_pinned_channel)'`
- Exit 100. `FAIL rust_floor::declared_floor_equals_the_pinned_channel`:
  `declared rust-version "1.89" vs pinned channel "1.95.0" (major.minor)`, `left: Some((1, 89))`,
  `right: Some((1, 95))`.
- Reverted: grep `25:rust-version = "1.95"`.

## Step 7 (b) — a member declares its own floor
- Mutation: `crates/viz/Cargo.toml` `rust-version.workspace = true` -> `rust-version = "1.95"`
  (grep: `5:rust-version = "1.95"`).
- Command: as (a).
- Exit 100. `FAIL rust_floor::declared_floor_equals_the_pinned_channel`:
  `crates/viz does not inherit the workspace rust-version` (the inheritance assertion).
- Reverted: grep `5:rust-version.workspace = true`; `git diff --stat -- crates/viz/Cargo.toml` empty.

## Step 7 (c) — skip-arm cleanup deleted
- Mutation: the three-line `if let Ok(dir) = default_lock_dir() { … remove_file … }` removed from the skip arm
  (read back: the `else` block holds only the comment, the `eprintln!` and `return`).
- Command: `cargo nextest run -p corpus --profile ci -E 'test(corpus_key_skip_arm_leaves_no_lock_file)'`
- Exit 100. `FAIL keychain::tests::corpus_key_skip_arm_leaves_no_lock_file`, `left: (true, 1)` /
  `right: (true, 0)`.
- Reverted: the cleanup line counts 2 in `keychain.rs` (success arm + skip arm).

## After all reverts
- `cargo nextest run --workspace --profile ci -E 'test(/declared_floor_equals_the_pinned_channel|corpus_key_skip_arm_leaves_no_lock_file/)'`
  exit 0: `2 tests run: 2 passed, 2628 skipped`.
