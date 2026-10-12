# Live readings of the gate block — 2026-10-10-engine-harness-proofs-completed

The plan's `## Test Commands` block was fired whole through the gate tool (`gate v1.14`) three times at implement on
2026-10-10, and the lines below are the tool's own, without its header. Entries 20 to 25 carry `leg = 'operator'`
and were not fired: they are the operator pass's.

## First firing — the tree before the port-claim change

      1 lint        green · exit 0 · 0.66s · 0 B → 1.log · cargo fmt --check
      2 lint        green · exit 0 · 2.8s · 236 B → 2.log · cargo clippy --workspace --all-targets --all-features -- -… (68 chars)
      3 probe       green · exit 0 · 0.01s · 0 B → 3.log · git diff --name-only 514fbe575d0446decebc98296192c52f30019… (524 chars)
      4 integration green · exit 0 · 8.99s · 6246 B → 4.log · cargo nextest run --workspace --profile ci --no-tests=fail… (120 chars)
      5 integration green · exit 0 · 16.81s · 2349 B → 5.log · cargo nextest run --workspace --profile ci --no-tests=fail… (99 chars)
      6 smoke       green · exit 0 · 97.46s · 2195 B → 6.log · d="target/boot-smoke/$(date -u +%Y%m%dT%H%M%SZ)-series" &&… (290 chars)
      7 e2e         green · exit 0 · 18.54s · 1591 B → 7.log · mkdir -p target/exit-witness && cc -shared -fPIC -O2 -o ta… (226 chars)
      8 lint        green · exit 0 · 0.4s · 100 B → 8.log · cargo xtask check:english-sources
      9 probe       green · exit 0 · 0.32s · 69 B → 9.log · cargo xtask quarantine-tracking
     10 probe       green · exit 0 · 0.3s · 394 B → 10.log · cargo xtask capability-widening-check
     11 probe       green · exit 0 · 0.29s · 99 B → 11.log · cargo xtask check:ingest-progress
     12 probe       green · exit 0 · 0.32s · 342 B → 12.log · cargo xtask check:staged-artifacts
     13 probe       green · exit 0 · 0.31s · 714 B → 13.log · cargo xtask capability-drift
     14 probe       green · exit 0 · 0.29s · 447 B → 14.log · cargo xtask verify:capability-matrix
     15 unit        green · exit 0 · 18.38s · 750559 B → 15.log · cargo nextest run --workspace --profile ci --no-tests=fail
     16 build       green · exit 0 · 2.68s · 493 B → 16.log · cargo nextest run -p pulse-app --features mcp-server --bin… (101 chars)
     17 probe       green · exit 0 · 0.01s · 0 B → 17.log · git diff --quiet 514fbe575d0446decebc98296192c52f30019428 … (95 chars)
     18 integration green · exit 0 · 108.13s · 1648428 B → 18.log · d="$(mise where node@24)" && PATH="$d/bin:$PATH" cargo xta… (75 chars)
     19 probe       green · exit 0 · 0.0s · 0 B → 19.log · test ! -e target/pre-push/run/data/run/andromeda-pulse.pid… (162 chars)
     20 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ga… (71 chars)
     21 probe       not run — leg operator (the letter drives it) · m="$(git ls-remote origin refs/heads/main | cut -f1)" && t… (111 chars)
     22 probe       not run — leg operator (the letter drives it) · git diff --quiet && git diff --cached --quiet && git push … (92 chars)
     23 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci… (95 chars)
     24 probe       not run — leg operator (the letter drives it) · d="target/boot-smoke/ci-<id>-attempt-1" && gh run download… (379 chars)
     25 probe       not run — leg operator (the letter drives it) · d="target/engine-cycle/ci-<id>-attempt-1" && gh run downlo… (196 chars)
    entries 25 · green 19 · red 0 · recorded 0 · timeout 0 · not-run 6

## Second firing — after the port-claim change

      1 lint        green · exit 0 · 0.68s · 0 B → 1.2.log · cargo fmt --check
      2 lint        green · exit 0 · 1.91s · 236 B → 2.2.log · cargo clippy --workspace --all-targets --all-features -- -… (68 chars)
      3 probe       green · exit 0 · 0.01s · 0 B → 3.2.log · git diff --name-only 514fbe575d0446decebc98296192c52f30019… (524 chars)
      4 integration green · exit 0 · 8.03s · 6246 B → 4.2.log · cargo nextest run --workspace --profile ci --no-tests=fail… (120 chars)
      5 integration green · exit 0 · 16.89s · 2349 B → 5.2.log · cargo nextest run --workspace --profile ci --no-tests=fail… (99 chars)
      6 smoke       green · exit 0 · 100.84s · 2195 B → 6.2.log · d="target/boot-smoke/$(date -u +%Y%m%dT%H%M%SZ)-series" &&… (290 chars)
      7 e2e         green · exit 0 · 18.6s · 1591 B → 7.2.log · mkdir -p target/exit-witness && cc -shared -fPIC -O2 -o ta… (226 chars)
      8 lint        green · exit 0 · 0.4s · 100 B → 8.2.log · cargo xtask check:english-sources
      9 probe       green · exit 0 · 0.32s · 69 B → 9.2.log · cargo xtask quarantine-tracking
     10 probe       green · exit 0 · 0.31s · 394 B → 10.2.log · cargo xtask capability-widening-check
     11 probe       green · exit 0 · 0.32s · 99 B → 11.2.log · cargo xtask check:ingest-progress
     12 probe       green · exit 0 · 0.33s · 342 B → 12.2.log · cargo xtask check:staged-artifacts
     13 probe       green · exit 0 · 0.32s · 714 B → 13.2.log · cargo xtask capability-drift
     14 probe       green · exit 0 · 0.3s · 447 B → 14.2.log · cargo xtask verify:capability-matrix
     15 unit        green · exit 0 · 18.45s · 750559 B → 15.2.log · cargo nextest run --workspace --profile ci --no-tests=fail
     16 build       green · exit 0 · 2.78s · 493 B → 16.2.log · cargo nextest run -p pulse-app --features mcp-server --bin… (101 chars)
     17 probe       green · exit 0 · 0.01s · 0 B → 17.2.log · git diff --quiet 514fbe575d0446decebc98296192c52f30019428 … (95 chars)
     18 integration green · exit 0 · 107.04s · 1648404 B → 18.2.log · d="$(mise where node@24)" && PATH="$d/bin:$PATH" cargo xta… (75 chars)
     19 probe       green · exit 0 · 0.0s · 0 B → 19.2.log · test ! -e target/pre-push/run/data/run/andromeda-pulse.pid… (162 chars)
     20 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ga… (71 chars)
     21 probe       not run — leg operator (the letter drives it) · m="$(git ls-remote origin refs/heads/main | cut -f1)" && t… (111 chars)
     22 probe       not run — leg operator (the letter drives it) · git diff --quiet && git diff --cached --quiet && git push … (92 chars)
     23 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci… (95 chars)
     24 probe       not run — leg operator (the letter drives it) · d="target/boot-smoke/ci-<id>-attempt-1" && gh run download… (379 chars)
     25 probe       not run — leg operator (the letter drives it) · d="target/engine-cycle/ci-<id>-attempt-1" && gh run downlo… (196 chars)
    entries 25 · green 19 · red 0 · recorded 0 · timeout 0 · not-run 6

## Third firing — the final tree

      1 lint        green · exit 0 · 0.71s · 0 B → 1.3.log · cargo fmt --check
      2 lint        green · exit 0 · 2.26s · 202 B → 2.3.log · cargo clippy --workspace --all-targets --all-features -- -… (68 chars)
      3 probe       green · exit 0 · 0.01s · 0 B → 3.3.log · git diff --name-only 514fbe575d0446decebc98296192c52f30019… (524 chars)
      4 integration green · exit 0 · 10.96s · 6246 B → 4.3.log · cargo nextest run --workspace --profile ci --no-tests=fail… (120 chars)
      5 integration green · exit 0 · 17.19s · 2349 B → 5.3.log · cargo nextest run --workspace --profile ci --no-tests=fail… (99 chars)
      6 smoke       green · exit 0 · 117.36s · 2195 B → 6.3.log · d="target/boot-smoke/$(date -u +%Y%m%dT%H%M%SZ)-series" &&… (290 chars)
      7 e2e         green · exit 0 · 18.64s · 1591 B → 7.3.log · mkdir -p target/exit-witness && cc -shared -fPIC -O2 -o ta… (226 chars)
      8 lint        green · exit 0 · 0.44s · 100 B → 8.3.log · cargo xtask check:english-sources
      9 probe       green · exit 0 · 0.33s · 69 B → 9.3.log · cargo xtask quarantine-tracking
     10 probe       green · exit 0 · 0.29s · 394 B → 10.3.log · cargo xtask capability-widening-check
     11 probe       green · exit 0 · 0.29s · 99 B → 11.3.log · cargo xtask check:ingest-progress
     12 probe       green · exit 0 · 0.34s · 342 B → 12.3.log · cargo xtask check:staged-artifacts
     13 probe       green · exit 0 · 0.32s · 714 B → 13.3.log · cargo xtask capability-drift
     14 probe       green · exit 0 · 0.3s · 447 B → 14.3.log · cargo xtask verify:capability-matrix
     15 unit        green · exit 0 · 19.09s · 750827 B → 15.3.log · cargo nextest run --workspace --profile ci --no-tests=fail
     16 build       green · exit 0 · 3.81s · 493 B → 16.3.log · cargo nextest run -p pulse-app --features mcp-server --bin… (101 chars)
     17 probe       green · exit 0 · 0.01s · 0 B → 17.3.log · git diff --quiet 514fbe575d0446decebc98296192c52f30019428 … (95 chars)
     18 integration green · exit 0 · 109.07s · 1648990 B → 18.3.log · d="$(mise where node@24)" && PATH="$d/bin:$PATH" cargo xta… (75 chars)
     19 probe       green · exit 0 · 0.0s · 0 B → 19.3.log · test ! -e target/pre-push/run/data/run/andromeda-pulse.pid… (162 chars)
     20 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ga… (71 chars)
     21 probe       not run — leg operator (the letter drives it) · m="$(git ls-remote origin refs/heads/main | cut -f1)" && t… (111 chars)
     22 probe       not run — leg operator (the letter drives it) · git diff --quiet && git diff --cached --quiet && git push … (92 chars)
     23 probe       not run — leg operator (the letter drives it) · python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci… (95 chars)
     24 probe       not run — leg operator (the letter drives it) · d="target/boot-smoke/ci-<id>-attempt-1" && gh run download… (379 chars)
     25 probe       not run — leg operator (the letter drives it) · d="target/engine-cycle/ci-<id>-attempt-1" && gh run downlo… (196 chars)
    entries 25 · green 19 · red 0 · recorded 0 · timeout 0 · not-run 6

Between the first two firings the tree changed in two test files only (`pulse-app/tests/integration_console_engine.rs`
and `xtask/tests/common/mod.rs`): the three spawned arms gained the lines that print their readings, and the
port picker was replaced (below). `xtask/src/engine_log.rs` was mutated seven times and reverted between the
firings (`mutations.md`). Between the second and the third, one pin was added in `xtask/src/pre_push.rs`
(`end_cycle_ends_the_process_the_stage_data_dir_s_pid_file_names`; `red-readings.md`). Entry 3, the scope guard,
read no output in all three.

## What the entries read (from each entry's log)

- **Entry 4** (the two new xtask test binaries): `25 tests run: 25 passed, 0 skipped`.
- **Entry 5** (the spawned-program suite): `7 tests run: 7 passed, 0 skipped` (four arms before this chunk).
- **Entry 6** (the window smoke, two boots under their own virtual displays, receivers on 14317 and 14318), the
  same in all three firings: each
  boot `series-cycle boot=0 settled=0 status=0 cleanup=0`, `cleanup: clean`; the series verdict
  `"boots": 2`, `"settled": 2`, ordinals 2 and 3 each `"verdict": "settled"`, `"verdict": "all-settled"`. The
  window program reads ready under the changed decision.
- **Entry 7** (the engine cycle on 24317 and 24318), second firing: `boot: ready`, `"verdict": "settled"`,
  `"verdict": "running-healthy"`, `cleanup: clean`, the seven arm lines all PASS (335 records; heartbeat max gap
  15001 ms, n=2 per target; largest `rows_ingested` 810; memory max 138240 B, populated 1), `engine-log: PASS`,
  and the verdict `"verdict": "pass"`, `"error_records": 0`, `"witness_file": "absent"`. Its data dir is
  `target/engine-cycle/20261010T190250Z`; the first firing's is `target/engine-cycle/20261010T184357Z`, the log
  the mutations were read on; the third firing's is `target/engine-cycle/20261010T191125Z`, with the same
  verdict (`"verdict": "pass"`, `"error_records": 0`, `"witness_file": "absent"`, `engine-log: PASS`,
  `cleanup: clean`).
- **Entry 15** (the workspace suite): `3111 tests run: 3111 passed, 0 skipped` in the first two firings,
  `3112 tests run: 3112 passed, 0 skipped` in the third (the added pin).
- **Entry 18** (the pre-push check): below.
- **Entry 19** (the teardown probe): exit 0, so no pid file under the check's data dir and both harness ports
  refusing after the check returned.

## The changed pre-push check — first measured wall-clock

`cargo xtask pre-push:linux` with Node 24 first on PATH, the dev host warm, another project's builds sharing the
CPU. Each reading is n = 1.

| | First firing | Second firing | Third firing |
|---|---|---|---|
| The entry, as the gate tool timed it | 108.13 s | 107.04 s | 109.07 s |
| `script-modes` | 2 ms | 2 ms | 2 ms |
| `source-lint` | 410 ms | 401 ms | 403 ms |
| `npm` | 65 798 ms | 64 251 ms | 65 432 ms |
| `clippy` | 1 026 ms | 1 025 ms | 1 122 ms |
| `test` | 21 097 ms | 21 412 ms | 21 771 ms |
| `ci-gates` (one engine cycle, then `ci-gates` over its log) | 18 764 ms | 18 957 ms | 18 977 ms |
| Verdict | `green` / `all-stages-ok`, six stages each `ok` | the same | the same |

The reading on record before this chunk was 89 s for the whole check (one reading, 2026-10-10, stage six over a
seed). The engine was already built in release on this host, so no firing includes a cold release build;
that cost is not measured here.

What stage six printed in the second firing (the cycle's own lines, then the gate's):

    cleanup: clean
    engine-log: family PASS (335 records across 1 file(s))
    engine-log: program PASS (one app.boot.engine record, reading console)
    engine-log: panic PASS (no app.panic.fatal record at ERROR)
    engine-log: heartbeat-gap PASS (ingest.tick max gap 15000 ms (n=2); buffer.tick max gap 15000 ms (n=2); connection.tick max gap 15000 ms (n=2))
    engine-log: progress PASS (2 buffer.tick record(s), largest rows_ingested 810)
    engine-log: process-end PASS (one app.exit record, the last of the family)
    engine-log: budget PASS (memory max 138240 B <= 512000000 B, n=2, populated 1)
    engine-log: budget frame and snapshot not graded (a console engine has no producer for either)
    engine-log: PASS
    ci-gates: zero-spans PASS (335 log records across 1 file(s))
    ci-gates: zero-panic PASS

## A run that did not read the same on one tree, and what was changed

After the first firing, the three port-using test binaries (`agent_run_shell`, `engine_cycle_refusals`,
`integration_console_engine`; 32 tests) were run by hand six times on the unchanged tree:
`cargo nextest run --workspace --profile ci --no-tests=fail -E 'binary(agent_run_shell) | binary(engine_cycle_refusals) | binary(integration_console_engine)'`.

- Five runs: `32 tests run: 32 passed`. One run: `31 passed, 1 failed`:
  `a_rejected_grpc_port_leaves_that_receiver_unstarted_and_the_http_receiver_alone_listening`, on
  `no gRPC receiver was started`: a port the arm had picked and left unbound accepted a connection. Its census
  in that run read the engine's one HTTP listener and nothing else, so the listener was not the engine's.
- The same arm alone, 60 runs of its test binary: 0 failed.
- Measured on this host: 20 000 binds to port 0 returned 10 667 distinct ports, every one odd, inside
  32768 to 60999; that is a uniform draw over about 14 100 numbers. The 32 tests start together and made about
  70 such draws, each released at once and bound again later (or never), so two tests can be handed one number.
- The first firing's entries 4, 5, 15 and 18 had read green on that tree.

The change: every port these three binaries use is claimed, not picked. A claim is a number from 20000 to 31999
(below the ephemeral range, the harness's own 24317 and 24318 left out), held by an advisory lock on a file
under `target/test-ports/` for as long as the test holds it, and shown bindable when claimed. The four existing
arms of the spawned-program suite take their ports the same way now; what the engine is handed is unchanged in
kind (two loopback port numbers).

After the change: 14 hand runs of the same command, `32 tests run: 32 passed` each, and the second and third
firings above. This is a count, not an exclusion: 14 green runs do not bound a rare failure; the claim removes the one
mechanism that was measured.
