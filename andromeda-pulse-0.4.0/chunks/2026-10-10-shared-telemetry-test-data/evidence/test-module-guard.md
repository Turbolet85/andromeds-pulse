# The test-module-only guard on this host — 2026-10-10-shared-telemetry-test-data

Gate entry 4 of the plan's `## Test Commands` (`git diff -U0 50216d7e… -- {four files} | awk '…'`) reads which
hunks of the four co-located-test files start above the line where each file's test module opens. Its awk takes
the file name from a header line `+++ b/{path}`.

## What this host prints
The user-level git config of this host sets `diff.mnemonicPrefix = true`
(`git config --show-origin --get diff.mnemonicPrefix` names the user config file; value `true`). Under it a diff of
a commit against the work tree prints `+++ w/{path}`, and a diff of two commits prints `+++ b/{path}`.

Header lines of the entry's own diff, as printed here:

```
+++ w/crates/buffer/src/appender.rs
+++ w/crates/buffer/src/consumer.rs
+++ w/xtask/src/engine_cycle.rs
+++ w/xtask/src/harness_ready.rs
```

So the awk never sets its file name, and every hunk of every file is read against the fall-through line 360.

## Four readings, run by hand at /implement (git and awk only, the entry's awk text unchanged)
| form | exit | output |
|---|---|---|
| the entry as written, base against the work tree | 1 | `:288` `:289` `:309` `:319` `:355` (five lines, the file name empty) |
| the same with `--src-prefix=a/ --dst-prefix=b/` on `git diff` | 0 | none |
| the pinned form over the plan's known-positive range `514fbe57..50216d7e` | — | 25 lines |
| the entry as written over that range (commit to commit) | — | 25 lines |

The five printed lines are hunks of `crates/buffer/src/consumer.rs`, whose test module opens at line 284: every one
starts inside it. They print because 288 to 355 lie below 360, not because a product line changed.

## What follows
- With the prefixes pinned the property the entry states holds: no hunk of the four files starts above its test
  module's line.
- As written, on a host with this setting, the entry is wrong in both directions for a diff against the work
  tree: it reads red on a test-module hunk below line 360 of any file, and it would read green on a product-line
  hunk between line 360 and a module line above it (`appender.rs` 931, `harness_ready.rs` 477).
- The plan's baseline for this entry was taken on the untouched tree, where the diff is empty and the header lines
  never print; its known-positive control is a commit-to-commit diff, which prints `b/`.
- The plan is not edited by /implement. This reading goes to the operator.
