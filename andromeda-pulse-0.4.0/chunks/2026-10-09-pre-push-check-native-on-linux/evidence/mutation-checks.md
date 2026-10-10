# Mutation controls (plan step 13)

One-shot, by hand, at /andromeda-implement, before the first graded use. Finished 2026-10-10T00:44:48Z.

Method, the same for each: the mutation is applied with an anchored edit to `xtask/src/pre_push.rs`; a grep confirms
it is present in the file; then `cargo nextest run -p xtask -E 'test(pre_push)' --no-fail-fast` is run (20 tests, the
module's own); the mutation is reverted. The edit and the run were never fired in one batch. The file's sha256 before
the first mutation and after the last revert reads the same (`c02e9d05eeaa4582…`), `cargo fmt --check` exits 0, and
the same nextest call reads `20 tests run: 20 passed` on the reverted file.

The raw nextest output is not kept here: a failing pin quotes its fixture paths. What is kept is each red run's
exit, its summary line, the failing tests and the first diagnostic line of each.

| # | Decision | Mutation (confirmed present) | Run | Pins that went red |
|---|---|---|---|---|
| 1 | the host guard | `os == "linux"` → `os != "linux"` in `host_supported` | exit 100 · `20 tests run: 19 passed, 1 failed` | `only_a_linux_host_is_supported` — `assertion failed: host_supported("linux")` |
| 2 | the Node comparison | `if node_major_of(&probes.node) != Some(node)` → `if false` in `missing_pieces` | exit 100 · `20 tests run: 18 passed, 2 failed` | `a_node_major_off_the_pin_is_named_with_what_was_found` — ``assertion `left == right` failed`` (the list came back empty) · `each_required_piece_is_named_when_it_alone_is_missing` (the Node arm) |
| 3 | the stage environment | one variable added to `stage_env`'s set: `DBUS_SESSION_BUS_ADDRESS` = `mutation` | exit 100 · `20 tests run: 19 passed, 1 failed` | `stage_env_is_exactly_the_constructed_set` — ``assertion `left == right` failed`` at the plain-stage set equality |
| 4 | the bindings restore | the restore skipped: `restored` computed without reading, writing or removing the file | exit 100 · `20 tests run: 17 passed, 3 failed` | `restoring_puts_the_file_back_however_the_action_ended` · `restoring_removes_a_file_that_was_absent_before` · `a_restore_that_cannot_write_is_a_failure` |

Each mutation turned at least one pin red; no run read green under a mutation.

What stayed green under each mutation, as a reading of what those pins do not cover:
- under 1, 2 and 3 the restore pins, the per-run pins, the stage-list pin and the document pin stayed green: each
  decision has its own pin and none leans on another's;
- under 2, `nothing_is_missing_on_a_provisioned_host` stayed green, as it must: it is the positive half and cannot
  tell a neutralised comparison from a working one. The two pins that went red are the half that carries the guard.

Not mutation-checked, and said so: the per-run paths and the stage order (plan step 13 lists four decisions: the
guard, the Node comparison, the stage environment, the restore). Their pins are value pins against literals.
