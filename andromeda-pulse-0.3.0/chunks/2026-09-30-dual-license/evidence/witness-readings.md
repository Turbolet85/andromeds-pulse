# Dual-license witness readings — 2026-09-30-dual-license

Selector: `cargo nextest run --workspace --profile ci -E 'test(/license_check::/)'` (the plan's `license_check::`
entry). Chunk base `1dfca7412b0026cfbb5221ebf460bb5665983f9e`.

## RED before (Step 1)
Taken with only `xtask/src/license_check.rs` + its `#[cfg(test)] mod license_check;` declaration in the tree; every
license surface at its base state. Exit 100.

- Summary: `4 tests run: 0 passed, 4 failed`
- `license_texts_present_at_root` — FAIL: panics in `read` (`LICENSE-MIT` absent)
- `license_workspace_value_is_dual_and_inherited` — FAIL: `left: ["license = \"MIT\""]` vs `right: ["license = \"MIT OR Apache-2.0\""]`
- `license_npm_manifest_and_lock_root_agree` — FAIL: `left: String("MIT")` vs `right: "MIT OR Apache-2.0"` (package.json)
- `license_channel_manifests_carry_dual_license` — FAIL: `update-channels.yml lacks "license any_of: [\"MIT\", \"Apache-2.0\"]"`
- `git diff --quiet 1dfca74 -- pulse-app/ui/src/bindings/index.ts` after the run: exit 0 (the selector does not
  regenerate the bindings).

## GREEN after (Step 6)
After Steps 2-5 (both texts copied from Conductor `cdb7082` blobs, md5 `0d9a205d780c21cbd7c042a596b7c446` /
`1836efb2eb779966696f473ee8540542`, 0 CR bytes; the four manifest edits). Exit 0.

- Summary: `4 tests run: 4 passed, 2429 skipped`

## Mutation check (Step 7, one-shot)
One surface reverted at a time, the selector run, the file restored from a backup and its sha256 asserted equal to
the pre-mutation bytes.

| mutation | exit | summary | the one failing test | its diagnostic |
|---|---|---|---|---|
| (a) `LICENSE-APACHE` deleted | 100 | `4 tests run: 3 passed, 1 failed` | `license_texts_present_at_root` | `read LICENSE-APACHE: The system cannot find the file specified. (os error 2)` |
| (b) `Cargo.toml` `[workspace.package]` back to `"MIT"` | 100 | `4 tests run: 3 passed, 1 failed` | `license_workspace_value_is_dual_and_inherited` | `left: ["license = \"MIT\""]` |
| (c) lock root `packages[""]` back to `"MIT"` | 100 | `4 tests run: 3 passed, 1 failed` | `license_npm_manifest_and_lock_root_agree` | `package-lock.json root entry license` · `left: String("MIT")` |
| (d) Scoop line back to `"license": "MIT",` | 100 | `4 tests run: 3 passed, 1 failed` | `license_channel_manifests_carry_dual_license` | `update-channels.yml lacks "\"license\": \"MIT\|Apache-2.0\""` |
| after the last restore | 0 | `4 tests run: 4 passed, 2429 skipped` | — | — |

Each mutation reddened exactly its own test; every restore returned to GREEN.
