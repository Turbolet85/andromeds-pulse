# /tmp corpus-key lock residue — removal record (plan Step 8)

`{tmp}` below is a placeholder for the system temp directory (the lock dir under `env -i`), written so
this record carries no host path.

Overseer review, founder-delegated, 2026-10-04: "remove it inside the chunk and record it". Re-derived at
/implement, after Step 5 landed; nothing copied from the plan.

## Pre-state (2026-10-04T21:55:14Z)
- `ls -l --time-style=full-iso {tmp}/andromeda-pulse-corpus-key-*.lock` listed exactly one file:
  `-rw------- 1 turbolet turbolet 0 2026-10-04 22:36:55.768059461 +0200 {tmp}/andromeda-pulse-corpus-key-c2875066785cd2b7.lock`
- `stat -c '%a %s %U'`: `600 0 turbolet` (mode 0o600, 0 bytes).
- `$XDG_RUNTIME_DIR` held no `andromeda-pulse-corpus-key-*.lock`.
- Holders: a scan of every readable `/proc/[0-9]*/fd/*` symlink for the exact path counted **0**; `lsof` on the
  path printed 0 lines. (The scan sees this user's processes; the file is this user's, mode 0600.)
- Its mtime (20:36:55Z) predates this /implement run (started 21:49:56Z), and this run's own CARRY-witness and
  mutation runs wrote only into their TempDirs, so it is not this run's residue.

## Removal (2026-10-04T21:55:24Z)
- `rm -- {tmp}/andromeda-pulse-corpus-key-c2875066785cd2b7.lock`, the exact name, nothing globbed: exit 0.

## Post-state (2026-10-04T21:55:24Z)
- The same `ls` printed `No such file or directory`: no corpus-key lock file stands in `/tmp`.

## Origin reading
Owed by the operator pass: the report-only residue census entry after native pre-push stage 5. The same name
(`andromeda-pulse-corpus-key-c2875066785cd2b7.lock`) reappearing reads as the product's own lock, re-created
by the production-service MCP legs under `env -i`; any other name, or none, is recorded here as read.

- Post-stage-5 census listing (plan entry 31, fired by hand at 2026-10-04T22:04:09Z, after native stage 5
  `cargo xtask test` under `env -i` exited 0 with `2630 tests run: 2630 passed` at 22:03:34Z, and stage 6):
  `ls -l --time-style=full-iso {tmp}/andromeda-pulse-corpus-key-*.lock` printed
  `No such file or directory` (exit 2; the entry is report-only, `expect = []`). **No corpus-key lock file of
  any name stands in `{tmp}`.**
- Reading, as read:
  - The removed name did NOT reappear, so the name-recurrence test does not identify the file as the product's
    own lock. Its origin stays undetermined: nothing in this census distinguishes an older test-skip residue from
    a product lock whose creating leg did not run, or did not create it, in this stage 5.
  - No pid-named test lock appeared either. Stage 5's `env -i` has no `XDG_RUNTIME_DIR` and no session bus, so
    `corpus_key_survives_a_real_process_boundary` is expected to take its skip arm with the lock dir in `{tmp}`.
    That is an INFERENCE, not an observed line: nextest captures a passing test's output, so the `[skip]` line is
    absent from stage 5's stdout (0 matches). Supporting it: the test passed in 0.007 s, consistent with the skip
    (the store arm spawns a child process); and `corpus_key_skip_arm_leaves_no_lock_file`, which asserts the
    `[skip]` line in its own cleared-env child, passed in the same stage. With the arm taken, it left nothing in
    `{tmp}`. This is the CARRY property in its real setting, the one the handoff measured leaving residue
    before this chunk.
  - Nothing was deleted at this census.
