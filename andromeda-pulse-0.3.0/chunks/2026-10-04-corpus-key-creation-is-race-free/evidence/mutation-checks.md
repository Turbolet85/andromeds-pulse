# Mutation checks — 2026-10-04-corpus-key-creation-is-race-free

One-shot controls run at /implement (plan Step 5). They are not listed gates. Each one ran
`cargo nextest run -p corpus --profile ci --no-fail-fast -E 'test(/race_free/)'` on this Linux
dev host (Secret Service answering, `XDG_RUNTIME_DIR` set). The witness spawns 8 barrier-released
children against one empty test-scoped entry `andromeda-pulse-test-{pid}-race-free`. Keys were
compared in-process and never printed; only counts are recorded here.

| Run | `fetch_from_os_store` body | distinct keys | returned-not-stored | witness | nextest exit |
|---|---|---|---|---|---|
| RED-at-base | HEAD's unlocked get → generate → set (76d6cca) | 8 | 7 | FAIL `left: (8, 7) right: (1, 0)` | 100 |
| GREEN | locked create-or-read (`fetch_with_lock_dir(service_id, default_lock_dir())`) | 1 | 0 | PASS (lock file existed, length 0) | 0 |
| Mutation | HEAD's unlocked body restored verbatim over the fix | 8 | 7 | FAIL `left: (8, 7) right: (1, 0)` | 100 |
| Restored | locked create-or-read | 1 | 0 | PASS | 0 (the listed targeted gate) |

Per run, the four sibling `race_free` pins (lock-name, Linux dir rule, fail-closed, the child) passed
in every row. They do not depend on the fetch body, except fail-closed, which calls
`fetch_with_lock_dir` directly.

Cleanup after each run: `secret-tool search --all username corpus-key | grep -c race-free` → 0.
The witness deletes the entry and its lock file before asserting.

RED-at-base matches research.md §Measured (8 distinct, 7 returned-not-stored, 5 of 5 rounds), so
the witness discriminates in both directions.
