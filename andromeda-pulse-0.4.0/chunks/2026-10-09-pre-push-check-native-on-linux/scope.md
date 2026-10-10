# Scope — Pre-push check native on Linux

**Marker:** `2026-10-09-pre-push-check-native-on-linux` · version `andromeda-pulse-0.4.0` · Epoch 1 — Foundation: base CI, the capability record, a console engine with its door, its gates
**Working entry (`working-route.md:17`):** "Pre-push check native on Linux — surviving stages run on the dev host; the
second-system hop and its distro clone leave (P-103, P-113)"
**Chunk base:** `8f655ca` (HEAD at take-up). Every diff-shaped probe of this chunk names this sha, never HEAD: the
operator pre-CI commit moves HEAD before the wrap re-runs them.

## Intent
Intent F16 (`andromeda-pulse-0.4.0/intent.md:396-399`) observes that `cargo xtask pre-push:linux` needs `wsl.exe` and
that on the Linux dev host its stages are run by hand, and expects "the check a developer runs before a push runs
natively on the Linux dev host over the stages that survive this version's removals". Requirement P-103 repeats it
(`requirements.md:73`), carried from 0.3.0 by the founder's ruling (`.andromeda/residuals.md:21`). This is the fourth
chunk of 0.4.0 and the second of the four entries that carry P-113.

## What the chunk builds
- **The check runs natively on the Linux dev host and gives a verdict.** Read at take-up, at `8f655ca`:
  `xtask/src/pre_push.rs::drive` returns `cannot-evaluate` with reason `not-windows` on any host but Windows
  (`pre_push.rs:146-148`), so on this host the verb can only print that. After the chunk one invocation runs the
  stages on the host it is started on, with no second operating system involved.
- **The second-system hop and its distro clone leave** (the entry's words). Verified at P3, by reading
  `xtask/src/pre_push.rs` whole; the hop and the clone are:
  - the hop: the constants `WSL` and `DISTRO` (`:23-24`), `distro_home` (`:315-328`), and `Linux::cmd`, which wraps
    every stage command and every probe in `wsl.exe -d Ubuntu --exec /usr/bin/env -i` (`:274-292`);
  - the clone: `CLONE_DIR` (`:25`), the distro-side half of `sync` (the clone, fetch, reset, clean, patch apply and the
    tree-id comparison, `:433-455`; its host-side half computes the tree id of HEAD plus the working tree through a
    temporary index, `:396-431`, and needs no second system), `cache`, the clone's own `target/` against a 40 GiB cap
    (`:26`, `:458-469`), and `wsl_path` with its test (`:487-501`, `:720-735`);
  - `NODE_BIN` (`:29-32`), the Viola repository's user-local Node install inside the distro, which the spec records
    as a cross-project fragility (`.claude/rules/verification-harness.md`, the `pre-push:linux` row).
  - `rust_channel` stays whatever happens to the rest. Verified at P3 from the code graph (`tree-query` trace, rust
    plane, 2 rows): the module has two callers outside itself, `main` at `xtask/src/main.rs:342` (`run`) and the test
    `declared_floor_equals_the_pinned_channel` at `xtask/src/rust_floor.rs:69` (`rust_channel`).
- **The surviving stages run on the dev host** (the entry's words). The verb has six stages today, in order
  (`pre_push.rs:196-203`): `script-modes` · `source-lint` (`cargo xtask check:english-sources`) · `npm` (`npm ci`,
  then `npm run build`, in `pulse-app/ui`) · `clippy` · `test` (`cargo xtask test`) · `ci-gates` (over a seeded fresh
  data dir). First failure stops.
  - The chunk makes the check native for what stands TODAY and says, stage by stage, which later entry removes it; it
    removes no stage for a surface that still exists (inputs#I1 item 2).
  - [premise-corrected: inputs#I1 item 2 says "several of the six exist for the window and leave in Epoch 2
    (`Window's gates retired`)"; read from the stages and the route, no stage's subject is named by one entry alone,
    and three later entries share them.] Stage by stage, as read at P3 (`working-route.md:35`, `:42`, `:44`, `:46`,
    each entry's own words before its freight):
    - `script-modes` asserts `scripts/agent-run.sh` is git mode 100755, because CI runs it by path. The only workflow
      lines that do are the boot job's (`ci.yml:385-388`; `grep -n 'agent-run' .github/workflows/*.yml`: 3 lines), and
      "a11y and boot CI jobs … leave" at `Window's gates retired` (`:42`, P-083).
    - `source-lint` scans five roots (`xtask/src/source_lint.rs:24`). One of them, `pulse-app/ui/src`, is the webview
      interface that leaves at `Window retired` (`:44`, P-083). The stage has no entry that removes it.
    - `npm` installs the UI package and builds `ui/dist`, which Tauri embeds (`pulse-app/tauri.conf.json:7`). The
      "webview interface" leaves at `Window retired` (`:44`); the "npm tree" is named by `Desktop distribution
      retired` (`:46`, P-114).
    - `clippy` covers the whole workspace under `--all-features`. No entry removes it.
    - `test` runs the default-features workspace suite. Its credential-store legs leave with the "credential-store
      key" at `Corpus encryption at rest retired` (`:35`, P-085); its rewrite of the generated bindings leaves with
      the "IPC routers" at `Window retired` (`:44`), and the "bindings" checks at `Window's gates retired` (`:42`).
    - `ci-gates` grades a seeded log with the unchanged `cargo xtask ci-gates`. Its CI counterpart is a step of the
      boot job (`:42`), and its "frame" arm is named there too. No entry removes the verb itself.
- **What the hand-run of the six stages had to do on this host**, each "a fact for the plan and none a design"
  (inputs#I1 item 1; the same facts ride `.andromeda/residuals.md:21` as its items (a), (b), (c), (e)). Each evidence
  pointer was spot-checked at P3; none was re-run.
  - "stage 3 needed a fresh per-run `PUPPETEER_CACHE_DIR` under `target/pre-push/` (the browser's extraction stopped
    partway even in a fresh cache)". Verified at P3 against
    `andromeda-pulse-0.3.0/chunks/2026-10-04-supply-chain-advisories-on-wasmtime-resolved/evidence/operator-pass.md`
    (rows 28: red on the shared cache, green on a fresh one) and against the host: `target/pre-push/` holds ten such
    caches, 963 MB each (`du -sh target/pre-push/puppeteer*`), 9.4 GB with their data dirs. The founder's ruling of
    2026-10-04 rides the residual: isolate the cache under `target/`, never delete the shared one. Why extraction
    stops is unmeasured. Item (e): plain `npm ci` exits 1 against the shared cache, `PUPPETEER_SKIP_DOWNLOAD=1 npm ci`
    exits 0 (measured at 2026-10-06-npm-supply-chain-gate-is-green-again; not re-run).
  - "the bindings regen had to be re-run after stage 5". Verified at P3: the default-features workspace run rewrites
    `pulse-app/ui/src/bindings/index.ts` to the no-mcp shape (test-plan §3 Per-chunk gate discipline states the
    mechanism; the same operator pass records the clobber), and that file is tracked
    (`git check-ignore -q`: exit 1). The clone used to absorb the rewrite; natively it lands in the working tree.
  - "stage 5's `env -i` drops the session bus, so the credential-store tests skip there". Verified at P3 by pointer:
    the skip line stands at seven sites in four files (`grep -rn -c 'no OS credential store' crates pulse-app/tests`),
    and a clean skip is a pass that nextest's counts do not show (test-plan amendment
    `2026-10-04-corpus-key-creation-is-race-free`).
- **The provisioning check is written for an apt distro, and its package list is not what this host needs.**
  Verified at P3. It reads its pins from the repository (the Rust channel, ci.yml's `node-version` major, ci.yml's
  first `apt-get install` list) and probes `rustup`, `cargo clippy`, `cargo nextest`, `node --version`, `which` for
  four tools and `dpkg-query` for the apt list (`pre_push.rs:332-389`); a missing piece is `cannot-evaluate` with one
  `sudo apt-get install` line (`:591-598`). Measured on this host (`ID=omarchy`):
  - `dpkg-query` and `apt-get` are not on PATH; `pacman` is.
  - `libxdo` is absent (no `/usr/lib/libxdo*`, `pacman -Q xdotool`: not found) while ci.yml's list names
    `libxdo-dev`; the workspace builds and tests green on this host without it: `Cargo.lock` names no `libxdo` crate
    (`grep -n 'name = "libxdo' Cargo.lock`: no line), the release binary built today links none (`ldd … | grep -c
    xdo`: 0), and the previous chunk's report records clippy and the workspace suite green here (2826 passed,
    `chunks/2026-10-09-boot-smoke-s-early-exit-found-and-closed/report.md:301`, `:318`). The eight other libraries of
    the list answer `pkg-config --exists`.
  - Of the four probed tools, no stage calls `jq` or `xvfb-run` (`grep -rn -l -w jq` and `grep -rn xvfb-run` over
    `xtask`, `scripts`, `.github`: the only users are `pre_push.rs` itself, a criterion script and the CI boot step).
  - The `test` stage needs a Python 3 interpreter (`pulse-app/tests/unit_l4_grammar.rs` fails without one), which
    the check does not probe. `python3` is at `/usr/bin/python3`.
  - `node --version` prints `v26.8.2`, the only Node the host's `mise` holds, with npm 11.19.1; every job of `ci.yml`
    pins `node-version: '24'` (`ci.yml:65` and five more). The Rust channel 1.95.0 is installed and active.
  What the native check requires of the host, and how it names a missing piece, is the plan's.
- **The stages ran in a clone under `env -i`; natively they run where the developer works.** Verified at P3. Today
  every stage command gets `HOME`, a fixed `PATH` and one data dir, and "nothing of this host's environment crosses"
  (`pre_push.rs:271-273`, `:216-223`); the hand-runs kept that form with `env -i` (the last plan that carried them:
  `andromeda-pulse-0.3.0/chunks/2026-10-05-l4-runs-on-a-small-current-model-chosen-by-measurement/plan.md:382-415`).
  With the clone gone the stages read and write the working tree itself:
  - `npm ci` deletes and reinstalls `pulse-app/ui/node_modules`, and a failed run leaves it empty (the same operator
    pass: "The first stage-3 run removed `pulse-app/ui/node_modules` before failing"); `npm run build` rewrites
    `pulse-app/ui/dist`. Both are gitignored.
  - `clippy` and `test` build into the developer's `target/`.
  - `test` rewrites the one tracked file named above.
  - `ci-gates` reads the data dir the verb seeds: `seed` removes and recreates it, so the stage grades the three seed
    records alone (`pre_push.rs:472-483`, `:44-51`).
  The developer's shell on this host carries `DBUS_SESSION_BUS_ADDRESS`, `XDG_RUNTIME_DIR`, `DISPLAY` and
  `WAYLAND_DISPLAY`, and no `RUST*`, `CARGO*`, `CC` or `PUPPETEER*` variable (`env`, names only).
- **The verdict contract is kept**: one pretty-JSON verdict on stdout with a twin at
  `target/pre-push/report.json`, exit 0 green · 1 red · 2 cannot-evaluate, a missing tool `cannot-evaluate` and never
  green, stage output on stderr, no stage starting `pulse-app` or binding a port (`pre_push.rs:1-11`). Verified at P3
  against architecture §Occupied Resources → xtask CLI surfaces (the arch extract). Two of the verdict's members
  describe the clone (`cache{…}`, and `remediation` as one apt line); whether each stays is the plan's to state.
- [premise-corrected: the module holds ten tests, not nine — `grep -c '#\[test\]' xtask/src/pre_push.rs` reads 10.]
  **The module's own tests follow the code.** Those of a helper that leaves leave with it, and what the native path
  adds is pinned where `xtask`'s tests run (co-located, test-plan §2).
- **A timing the check takes on this host says under what load it was taken** (inputs#I1 item 5: other builders run
  heavy cargo here).
- **The records that describe a WSL-hosted check are corrected at the wrap, not here.** Phase amends no spec source;
  the plan carries them as expected amendments.

## Capability
- **P-103 is this entry's own.** Its acceptance reads "On the Linux dev host one check runs natively before a push,
  over every stage that survives this version's removals, with no second operating system involved, and gives a
  verdict." Whether this chunk alone proves it is decided at P4 and P5.
- **P-113 is advanced, not claimed.** Its observed gap lists "the `wsl.exe` hop in `xtask/src/pre_push.rs`" among
  nine items; this chunk removes that one. Two later entries carry the rest: `One place on a node`
  (`working-route.md:33`) and `Other operating systems retired from the code` (`:56`). The ledger's note on P-113
  (2026-10-09-ci-on-linux-alone, phase P5) already names this split.

## Boundaries
- **CI's own steps are not this chunk's** (inputs#I1 item 4). `No CI step reads nothing` (`working-route.md:19`,
  P-128) is the next entry. `.github/workflows/ci.yml` is read for its pins and not edited.
- **No stage leaves for a surface that still exists** (inputs#I1 item 2). The window, its bridge and the credential
  store stand today; their stages stay until the entries that remove them.
- **No other retirement of another operating system.** The `.ps1` scripts, the `pwsh` spawns, the Windows path
  handling, the link hints, the linker override and the library-test workaround stay; they are
  `Other operating systems retired from the code`'s (`working-route.md:56`).
- **No sixth `agent-run` verb.** The check stays an `xtask` verb (test-plan §3's five-command discipline).
- **A boundary widening halts for the founder's own word** (inputs#I1 item 6). A stage that would reach something
  the check does not reach today (the session bus and through it the OS credential store, the developer's own data
  dir, a port) is a question, not a decision of this chunk.
- No run binds 4317 or 4318 without the operator's word (the handoff's host note). The check binds no port today.
- At P5 the plan card is printed and the phase stops for the operator's `yes` (inputs#I1 item 6).

## Folded freight (the entry's one block)
- watch: the CI boot smoke ending by itself after ready — exit 1, no `app.exit` record, a fraction of a second after
  the webview's first calls; no cause named (1/11; since 2026-10-09-boot-smoke-s-early-exit-found-and-closed). An
  observation: no acceptance criterion, no plan task, no gate entry. The block's own terms stand as written on the
  entry: on a recurrence read what the job kept (`harness-settled.json`, `xvfb.log`) and mint the entry that closes
  the named cause; retirement by count puts P-129 to the operator at that wrap's card; a reading is of equal source
  only while both tips stand; the two red runs' artifacts `11613618010` and `11632850540` expire 2026-10-23.
  - Readings at this take-up (inputs#I1 item 3), each from the boot job's own log, fetched to a file through
    `gh api --allow-escape-sequences …/actions/jobs/{id}/logs`: two more, both green and both `settled` (below). No
    recurrence. The tally is the wrap's to re-author.

## Second fold source — the CI verdict read at Setup 5a
Every commit from the last master flip (`8a89e36`) through HEAD, read through `ci.py conclusion`, 7 checks each. Both
are pull-request runs; each built the branch tip merged with `main` at `178ebac5`.
- `8f655caef468` (HEAD; the setup upgrade, no source file): **`in progress` at Setup — CI `8f655ca`: verdict not yet
  available** (`ci#37990081874`, `secret-scan#37990081871` success). Read at take-up: five of the six `ci` jobs
  complete, all `success`; `coverage` still running. Re-read before the P5 review, the run complete: green ·
  checks 7/7 · wall 1363 s.
  - its `boot smoke (ubuntu-22.04)` check: green · job `114021702149`, 634 s · `harness:settled` printed
    `verdict: settled`, `windows_settled: 4`, `display: reachable`, `session_bus: reachable`,
    `app_exit_record: absent`; `status` printed `running-healthy`; `cleanup: clean`. The merge it built: `3e4b750`.
- `8a89e3681c94` (the last flip): green · checks 7/7 · wall 1592 s (`ci#37986543962`, `secret-scan#37986543964`).
  - its `boot smoke (ubuntu-22.04)` check: green · job `114009833232`, 639 s · `verdict: settled`,
    `windows_settled: 4`, `display: reachable`, `session_bus: reachable`, `app_exit_record: absent`;
    `running-healthy`; `cleanup: clean`. The merge it built: `b131a4d`.

No red and no `not green` among them: nothing is dispositioned here.

## Gate
- none — the entry carries no `BLOCKED-ON` (`route.py blocked`: 0 blocks on a pending, gated or markerless line).
