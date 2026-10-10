# Report — 2026-10-09-pre-push-check-native-on-linux

**Chunk:** Pre-push check native on Linux: the surviving stages run on the dev host; the wsl.exe hop and its distro clone leave
**Date:** 2026-10-10
**Commits:** since the last wrap (2026-10-09T20:19:20Z): `8f655cae chore(setup-project): upgrade andromeda-pulse — U02, U49`
(not this chunk's; the chunk base) · `cb8cc4dc chore(2026-10-09-pre-push-check-native-on-linux): operator pre-CI commit`
(the operator pass's one commit; `git log --format='%h %s' 8f655cae..HEAD`: 1 line).

`cargo xtask pre-push:linux` is a check that runs on the Linux dev host itself. It read `green` twice on this host,
once on the working tree and once on the committed tree `cb8cc4dc`. No product code changed: the diff is the verb's
module and one `about` string. The CI run of the pushed tip is red in one check, the boot smoke, with the folded
watch's own signature; by the operator's word (inputs#I2 item 1) that red is not a reading of P-103, whose subject is
the dev-host check.

## Changes (structured — detectors read this)

- **Files:** `xtask/src/pre_push.rs` (592 lines added in 54 ranges; `git diff --stat 8f655cae -- xtask`: 593
  insertions, 378 deletions over the two files) · `xtask/src/main.rs:263` (one line, the `about` string of
  `PrePushLinux`). Nothing else tracked outside the pipeline's own records differs from the chunk base
  (`gate.py scope`: changed 2 · listed 2).
- **Symbols / APIs** (all in `xtask/src/pre_push.rs`, a leaf module of the `xtask` binary; coordinates from the
  listing at the end of this report):
  - **Kept, signature unchanged:** `pub(crate) fn run() -> Result<ExitCode>` (sole caller: `main` at
    `xtask/src/main.rs:342`, `grep -n 'pre_push::' xtask/src/*.rs`: 2 hits) · `pub(crate) fn rust_channel` (one
    caller outside the module, `declared_floor_equals_the_pinned_channel` at `xtask/src/rust_floor.rs:69`) ·
    `node_major` · `script_mode` · `node_major_of` · the `Verdict` enum with its three exits (0 green · 1 red ·
    2 cannot-evaluate) · the `Stage` enum, its six names and their order · `SEED_LOG`.
  - **Removed:** the constants `WSL`, `DISTRO`, `CLONE_DIR`, `CACHE_CAP_BYTES`, `NODE_BIN`, `TOOLS`; the `Linux`
    struct and its `wsl.exe` command wrapper; `distro_home`; `provisioning`; `sync` and the patch file it wrote;
    `cache`; `wsl_path`; `apt_packages`; `installed_packages`; `remediation`. No caller outside the module existed
    for any of them (research's code-graph read).
  - **New:** `host_supported(os)` `:134-138` · `Host` `:187-194` (the working tree, the per-run area, the two
    variable sets) · `RunArea` with `under(root)` and `reset()` · `remove_tree` · `dir_holding(path, name)`
    `:328-331` · `stage_path(home, verb_path)` `:333-347` · `stage_env(home, verb_path, area, npm)` `:349-368` ·
    `command(env, dir, program, args)` `:370-381` · `succeeds` `:383-387` · `Probes` `:402-413` · `probe` ·
    `channel_listed` `:437-441` · `missing_pieces` · `found_version` `:477-489` · `provisioning_stop` `:491-495` ·
    `head_and_tree` · `restoring(path, action)` with `RestoreFailed` · `document(verdict, reason, doc)`.
  - **The host guard:** the verb evaluates on Linux alone (`host_supported(std::env::consts::OS)`); any other
    system reads `cannot-evaluate` / `not-linux`.
  - **The verdict's reasons, complete:** `all-stages-ok` · `not-linux` · `pins-unreadable` · `home-unset` ·
    `provisioning-missing` · `run-dir-unusable` · `tree-unreadable` · `stage-failed:{stage}` ·
    `restore-failed:bindings`. Gone: `not-windows`, `wsl-missing`, `distro-missing`, `sync-failed:{step}`,
    `sync-mismatch`. `home-unset`, `run-dir-unusable`, `tree-unreadable` and `restore-failed:bindings` are new.
  - **The order of a run:** host guard → the two pins read from the repository (the Rust channel from
    `rust-toolchain.toml`, the Node major from ci.yml; either unreadable is `pins-unreadable`) → `HOME` read
    (unset or empty is `home-unset`) → the provisioning probes, each in the stage environment → the per-run area
    removed and created again → `head` and `tree` → the six stages, first failure stops. The per-run area is reset
    AFTER the provisioning check passes, so a run that ends `provisioning-missing` rewrites only the report twin.
  - **The provisioning check, native.** Required, each named in `missing[]` when absent: the pinned channel in
    `rustup toolchain list` (`rust:{channel}`, no install hint), then `cargo +{channel} clippy --version`
    (`rust:clippy`) and `cargo +{channel} nextest --version` (`cargo-nextest`) — those two are probed only when the
    channel is listed, because a `cargo +{channel}` call against an absent channel could install it; `node
    --version` whose major equals ci.yml's pin (`node:{pin} (found {version})`, where the found text is the tool's
    version token, `none` when no `node` answered, `unreadable` when the output is not a short version token);
    `npm --version` answering (`tool:npm`); `git` and `cc` on the stage PATH (`tool:git`, `tool:cc`); a Python 3
    interpreter, `python3` else `python` printing `Python 3` (`tool:python3`). Gone: the read of ci.yml's apt list,
    the `dpkg-query` probe, `apt:{package}`, `tool:jq`, `tool:xvfb-run`, the one `sudo apt-get install` line. The
    verb installs nothing and prints no install command.
  - **Environment variables.** The verb READS two from its own environment, by value, to build the stage
    environment, and prints or writes neither: `HOME` and `PATH`. Every probe and stage child is spawned with a
    CLEARED environment plus exactly: `HOME` (the developer's home) · `PATH` (`{HOME}/.cargo/bin`, then the
    directory of the first `node` file found in an absolute directory of the verb's own PATH, then
    `/usr/local/bin:/usr/bin:/bin`; with no `node` there the directory is left out) · `ANDROMEDA_PULSE_DATA_DIR`
    (`target/pre-push/run/data` under the repository root, absolute) · for the `npm` stage only,
    `PUPPETEER_CACHE_DIR` (`target/pre-push/run/puppeteer`, absolute) · for the three git calls that compute
    `head` and `tree` only, `GIT_INDEX_FILE` (`target/pre-push/run/index`). `DBUS_SESSION_BUS_ADDRESS`,
    `XDG_RUNTIME_DIR`, `DISPLAY` and `WAYLAND_DISPLAY` are not in the set (pinned by set equality), so no stage
    reaches an OS credential store. `PUPPETEER_CACHE_DIR` is a variable no product code reads; `grep -r
    PUPPETEER_CACHE_DIR` over the seven masters and `.andromeda/registries/`: 0 hits.
  - **Filesystem.** The per-run area `target/pre-push/run/` belongs to the verb: `data/`, `puppeteer/` and the
    temporary index. The report twin stays at `target/pre-push/report.json`. No longer written:
    `target/pre-push/index`, `target/pre-push/tree.patch`, and the distro clone `~/andromeda-pulse-pre-push` with
    its 40 GiB `target/` cap. The verb touches nothing else under `target/pre-push/` and never
    `~/.cache/puppeteer`.
  - **The stages, native.** Same six names, same order, same commands, each run in the working tree, its output
    sent to stderr: `script-modes` (`git ls-files -s scripts/agent-run.sh`, green on mode `100755`) · `source-lint`
    (`cargo xtask check:english-sources`) · `npm` (`npm ci`, then `npm run build`, in `pulse-app/ui`) · `clippy`
    (`cargo clippy --workspace --all-targets --all-features -- -D warnings`) · `test` (`cargo xtask test`) ·
    `ci-gates` (the data dir removed and created again holding only the seed log, then `cargo xtask ci-gates`).
  - **The bindings restore.** The `test` stage rewrites the tracked `pulse-app/ui/src/bindings/index.ts`; the
    stage runs inside `restoring`, which reads the file's bytes (or notes it absent), runs the stage, and puts the
    file back whichever way the stage ended; a file absent before is removed again. A restore that fails makes the
    stage red with reason `restore-failed:bindings`. The restore runs when the stage's command returns, not on a
    signal: a verb killed in the middle of the `test` stage leaves the bindings rewritten.
  - **No port, no TauRPC procedure, no capability, no tracing target.** The verb binds no port and starts no
    `pulse-app`.
- **Crates / modules:** `xtask` changed (one module rewritten); none added or removed.
- **Dependencies:** none added, none bumped (`tempfile` was already a normal dependency of `xtask`). No manifest,
  lockfile or `deny.toml` line differs from the chunk base (the scope guard entry, green).
- **Schema / config:** the verb's verdict document. Members now, by set equality in a test: `verdict`, `reason`,
  `head`, `tree`, `stages[{name, ok, ms}]`, `missing[]` — six. `remediation` and `cache{bytes, cap, cleaned}`
  left with the apt line and the clone. No member carries a value of an environment variable or a path.
- **Spec-master edits:** none before this wrap.
- **Counts / qualifiers moved:**
  - **The verdict document's members:** 8 → 6 (architecture §Occupied Resources → xtask CLI surfaces and the
    test-plan key file `per-chunk-gate-discipline.md` both print the eight-member form).
  - **Where the check can run:** a Windows host with the WSL `Ubuntu` distro → the Linux dev host. The key file
    says "it cannot run on the dev host"; that qualifier is now false.
  - **The module's tests:** 10 → 20 (`cargo nextest run -p xtask -E 'test(pre_push)'`: 20). Five left with their
    helpers (`apt_packages_join_continuations_and_drop_flags`, `apt_packages_read_the_real_workflow`,
    `installed_packages_keep_only_installed`, `remediation_is_one_apt_line_or_none`, `wsl_path_maps_drive_paths`),
    five stayed, fifteen are new.
  - **Workspace test count:** 2826 → 2836 passed, 0 skipped (`cargo nextest run --workspace --profile ci` at
    implement, and 2836 test `ok` events in each of the verb's two green `test` stages). No master states the total.
  - **Boot-smoke readings since `b3ac58a`:** twelve `ci` runs, three red (was nine, two red, at the last wrap).
    Basis: `gh run list --workflow ci.yml -L 40`, the runs created from `b3ac58a`'s on: ten on the build branch
    (`b3ac58a9` red · `0b61bfbe` · `569604be` · `b3e58597` · `f36a2ac3` red · `277d65db` · `e2931127` · `8a89e368`
    · `8f655cae` · `cb8cc4dc` red), one push on `main` (`178ebac5`), one hotfix pull request (`5b307e4c`). The
    three reds are `ci#37924991598`, `ci#37961031489` and `ci#38010977166`; in each the one failed job is `boot
    smoke (ubuntu-22.04)` (read per run through the jobs API). On the build branch alone: three red in ten.
- **Dev-tool versions:** `node` (the runtime; the masters' word for the pin is ci.yml's `node-version` major) on
  the dev host: Node `v24.21.0` with npm `11.19.0` INSTALLED 2026-10-10T00:40Z by `mise install node@24`, a
  user-level install beside the host's default `v26.8.2` / npm `11.19.1`, which is unchanged and is still what
  `command -v node` resolves. On the operator's word (plan step 1). Not a lockfile-resolved package. The Viola
  repository's Node install inside the WSL distro is no longer read by anything in this repository.
- **Harness / gate surface:** the xtask verb `pre-push:linux` changed as above (host, provisioning list, stage
  environment, per-run area, bindings restore, verdict document, reasons); its `about` text. No `agent-run` verb,
  no CI step, no status or verdict shape of another verb changed. `.github/workflows/ci.yml` is read for one pin
  and not edited.
- **Cross-project / external claims:**
  - `I1 · ../additional/pc-overseer/relays/pulse-phase-pre-push-native-linux-2026-10-09.md · copy no-repo ·
    unchanged` — the operator's phase directive; cited by scope, research and plan.
  - `I2 · ../additional/pc-overseer/relays/pulse-wrap-pre-push-native-linux-2026-10-10.md · copy no-repo ·
    unchanged` — the operator's wrap directive, snapped at this wrap and cited here as inputs#I2 (`verify`, fired
    before this report existed, printed it `UNCITED`). `inputs: 2 entries — unchanged 2 · drifted 0`.
  - **The CI run this chunk's gates read:** `ci#38010977166`, a `pull_request` run on the pushed tip `cb8cc4dc`
    (the merge it built: `3a05394f`), conclusion **failure**: six checks `success` (a11y · coverage gate · gitleaks
    · lint / test · mcp-server tests · supply-chain), one `failure`, `boot smoke (ubuntu-22.04)`, job
    `114090652871`. Read once, not re-run. Record: `evidence/operator-pass.md`.
  - **What that boot job kept** (artifact `logs-boot-Linux` `11653875886`, expires 2026-10-24): the settle verdict
    `ended`, `ended: exit 1`, `app_exit_record: absent`, `windows_settled: 0`, `display: reachable`, `session_bus:
    reachable`; `xvfb.log` 0 B; the application log 53 records long, its last record 0.072 s after `ui-bridge.ready`
    (the webview's first call) and 0.907 s after its first; 0 `app.exit`, 0 `app.panic.fatal`, 0
    `app.boot.window.navigation`.
  - **The two green readings before it:** boot smoke green on `8a89e368` (`ci#37986543962`) and on `8f655cae`
    (`ci#37990081874`), each `settled` with four windows (their artifacts read at this wrap).
  - **inputs#I2's two questions, answered by a read:**
    - *Does `boot.log` hold the application's stderr and stdout whole?* By the harness script, yes:
      `scripts/agent-run.sh:79` spawns the app as `"$APP_BIN" > "$DATA_DIR/logs/boot.log" 2>&1`, so both streams of
      the app, and of any child that inherits them, go to that file from its first byte. In all four artifacts read
      (`ci#37979648967`, `ci#37986543962`, `ci#37990081874` green; `ci#38010977166` red) the file is 207 B and
      holds the same one line, the accessibility-bus warning. So the red run's process ended with exit status 1
      having written nothing more to either stream than a green run writes. Not measured: output still in a
      process buffer at the moment it ended.
    - *Do the green runs' application logs hold an `app.boot.window.navigation` record that this one lacks?* Yes.
      Each of the three green logs holds four, one per window, each `navigated: true`, written 4.696 s, 4.129 s
      and 4.382 s after that run's `ui-bridge.ready`; the red log holds none. The red run's last record stands
      0.072 s after its `ui-bridge.ready`, so its process ended about four seconds before the point at which a
      green run writes those records: their absence is the early end itself, not a second signal.
  - **What the job still does not keep that would name who ended the process.** Kept: the exit status (`exit 1`,
    recorded by the waiting wrapper as an exit and not as a signal), both output streams, the display server's
    error output, the app's own log. Not kept: any record of the call that ended the process (no stack or trace at
    the exit), the exit status or output of the webview's own child processes apart from what they write to the
    inherited streams, and anything the kernel or the session logged at that instant. The app's own log has no
    `app.exit` record, which is the record the product writes for an end it can see.
  - **The condition the last chunk reproduced locally is not what this run shows.** That chunk's display-lost
    control (its report, Outcome) produced this signature with `display: gone`. This run reads `display:
    reachable` and an empty `xvfb.log`.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none by this chunk. Standing from the last chunk and
  re-read here: the readiness and settle instruments it added did not close the boot smoke's early end, and were
  not claimed to; this run is the first red those instruments read.
- **Spec claims disproved by measurement:**
  1. **The check "cannot run on the dev host".** Stated in the test-plan key file
     `.andromeda/registries/contracts/test-plan/per-chunk-gate-discipline.md:32` ("it requires a Windows host with
     the WSL `Ubuntu` distro (`wsl.exe`), so since the dev host moved to Omarchy Linux (2026-10-03) it cannot run
     on the dev host") and in architecture §Occupied Resources → xtask CLI surfaces (`.andromeda/architecture.md:250`,
     "Windows host only"). Measured false after this chunk: two green runs on the dev host (below). This is the
     chunk's own change, carried by expected amendments 1 and 3.
  2. **"It … reads no new env var"** (`.andromeda/architecture.md:250`, the same row). The native verb reads `HOME`
     and `PATH` by value and sets `PUPPETEER_CACHE_DIR` for one stage. Carried by expected amendment 2.
  3. **The boot job's `ci-gates` line "on every run that reaches the settle verdict".** Stated at
     `.andromeda/obs-plan.md:545` ("the step's order makes that the expected line on every run that reaches the
     settle verdict"), `.andromeda/obs-plan.md:559` ("the boot-smoke line reads `… no WebGPU adapter
     (no_navigator_gpu)` on a run that reaches the settle verdict") and `.andromeda/test-plan.md:100` ("so the boot
     job's line reads `no WebGPU adapter (no_navigator_gpu)`"). Measured false for a run whose settle verdict reads
     `ended`: on `ci#38010977166` the smoke step printed the settle verdict and failed, the job's `cargo xtask
     ci-gates` step reads `skipped` (the jobs API, step 14 of job `114090652871`), and the job log holds 0
     `ci-gates` lines. The line is printed only on a run whose smoke step passes. Added to this report after the
     fan-out, on the obs-plan detector's note; raised by the orchestrator at Validate (check 6). The neighbouring
     claim, that the boot job's LOG holds the `ui.webgpu.adapter` record on every run that reaches the settle
     verdict (`architecture.md:180`, `security-plan.md:447`, `test-plan.md:142`, `obs-plan.md:132`, `:445`), holds
     on this run: 2 records, both `no_navigator_gpu`.
  No other master statement was measured false. The folded watch's reading is not a spec claim: the key file
  `.andromeda/registries/contracts/test-plan/5-command-implementation.md:5` says the cause "is not named, and stays
  with the route's P-129 record", which this run leaves true.
- **Expected amendments (from plan):** sites located by `grep -r -i -c` per pattern over the seven masters and
  every file under `.andromeda/registries/`, lines counted.
  1. architecture §Occupied Resources → xtask CLI surfaces, the `pre-push:linux` row — **carried** (Symbols / APIs,
     Schema / config, Counts). `pre-push`: architecture 1 line (`:250`, ten occurrences of the pattern in it) ·
     test-plan 1 (`:227`) · key file `per-chunk-gate-discipline.md` 1 (`:32`) · the other masters 0. The row's
     three uses of "clone" (`architecture.md:250`: the distro clone, the clone's `target/`, `check:english-sources`
     "in the clone") all sit inside the `pre-push:linux` row; the `check:english-sources` row itself names no
     clone (it names `pre-push:linux`'s `source-lint` stage, which stays true).
  2. architecture §Occupied Resources → Environment variables — **carried** (Symbols / APIs, Environment
     variables). `PUPPETEER_CACHE_DIR`: 0 hits in any master or key file, so the row is new.
  3. test-plan §3 Per-chunk gate discipline, the `pre-push:linux` paragraph — **carried**. The section is a key
     file: `.andromeda/registries/contracts/test-plan/per-chunk-gate-discipline.md:32` (`wsl`: 1 line there, 1 in
     architecture `:250`, 1 in test-plan `:227`; `remediation`: architecture 1 · key file 1, the three a11y-plan
     hits and one a11y key-file hit being the violation schema's own word, no change). The native reading is a new
     dated reading beside the WSL ones: green 2026-10-10 on `cb8cc4dc`, six stages, 89 s warm (`npm` 66.9 s, `test`
     18.1 s, 2836 tests), load average 4.61 before and 11.32 after; a timing under that load, not a budget.
  4. test-plan §4 Unit Test Strategy, the corpus crate's "native pre-push stage 5" clause — **carried**
     (`stage 5`: test-plan 2 lines, `:227` the clause and `:257` an unrelated "Stage 500 spans", no change). The
     stage is now the verb's own `test` stage under a constructed environment; that the credential-store legs
     clean-skip there is stated, not measured by this chunk (the stage's log cannot show a passing test's output).
  5. security-plan §Security Anti-Patterns → Input, the harness-only class — **carried** (Symbols / APIs,
     Environment variables and Filesystem). `pre-push` and `PUPPETEER`: 0 hits in security-plan; its one `distro`
     hit (`:275`) is the keychain fallback's "legacy distro", no change. The classification is the operator's
     reading (inputs#I2 item 6): the verb's tool-locating read of `PATH` and `HOME` and its per-run area under
     `target/` are routine harness evidence, not a widening — no stage reaches the credential store and no value is
     printed or written.
- **Coverage of new surfaces:**
  - `cargo xtask pre-push:linux` (native) → validation mechanism✓ (host guard, pins read from the repository,
    provisioning check; an unmet pin is `cannot-evaluate`) · instrumentation n/a (an xtask verdict verb: one JSON
    document and a twin, no tracing target) · PII redacted✓ for the verdict and the twin (no environment value, no
    path; the Node version text is a bounded token) — stage output on stderr is the tools' own and passes no
    scrubber, as before · tests unit (20, co-located) + live (two runs of the verb per green reading) · a11y n/a ·
    tokens n/a.

## Deviations from intent

Accepted as recorded by the operator, twice: in this session after the implement report ("the six deviations are
accepted as recorded - the wrap carries the home-unset reason and the reset-after-provisioning order into the amended
rows" — the operator, 2026-10-10) and in inputs#I2 item 5.

1. **An unset `HOME`** reads `cannot-evaluate` with a new reason, `home-unset`. Plan step 6 says `HOME` is read from
   the verb's own environment and does not say what an unset one reads.
2. **The per-run area is reset after the provisioning check passes**, not at the very start. Plan step 7 says "at
   the start of every run"; the cannot-evaluate gate entry's note says that run "rewrites only the report twin".
   The two cannot both hold for a `provisioning-missing` run; the code keeps the note true.
3. **`missing[]` names.** The new pieces are `tool:npm` and `tool:python3`; `rust:{channel}` lost its `(rustup
   toolchain install …)` hint because step 5 says no install command is printed; with no Node answering the entry
   reads `(found none)`; a version text that is not a short token reads `unreadable`.
4. **The PATH search skips a non-absolute entry** (an empty one would resolve against the working directory). Not
   stated by step 6.
5. **Three tests beyond step 12's list:** `reset_empties_the_per_run_area_and_nothing_beside_it`,
   `a_restore_that_cannot_write_is_a_failure`, `stage_path_leaves_the_node_directory_out_when_the_verb_has_none`.
6. **Stage children are spawned with a cleared environment directly**, not through `cargo_command()`. That helper
   exists to drop the package variables `cargo run` injects; a cleared environment carries none of them.

Scope record: none — `gate.py scope` clean, 0 recorded (`scope: clean — changed 2 · listed 2 · recorded 0 … ·
excluded 51`, base `8f655cae`, read at this wrap).

## Decisions & corrections

- **The operator's words of this chunk.** At phase: Node 24 as a user-level `mise` install the implement run may
  make; the Node pin kept as ci.yml states it; the bindings put back as found, no regen stage. After implement: the
  six deviations accepted; the operator pass ordered, with the stop at a boot-smoke-only red. At this wrap
  (inputs#I2): the operator pass stands with its red, no run is re-run for a green, and the red is not a reading of
  P-103; the watch's closing entry is minted first in the markerless tail; the harness-only reading; P-113 stays
  pooled with a dated note.
- **A green that looked too fast was read before it was believed.** The gate block returned the workspace suite in
  17 s and the six-stage run in 90 s. Both were real (2836 passed; 1.3 GB in the per-run area); the check was the
  entry's own log, the report twin, and the test count's arithmetic (2826 + 20 − 10 = 2836).
- **`pulse-app` alone is rebuilt by a run of the verb on the shared `target/`** (checked once by `clippy`,
  compiled once by `test`, in both green runs; 1.5 to 1.7 s and 3.2 to 4.1 s). The cause is not isolated: the `npm`
  stage rewrites `pulse-app/ui/dist`, which the crate embeds, just before.
- **Sweep hazard, met and routed around:** a test fixture's home literal shaped `/home/{user}` is quoted by a
  failing pin, and that shape in a committed evidence file trips the hygiene read. The fixture is spelled
  `/dev-home` and the mutation runs' raw output was kept out of evidence.
- **The CI read returns on the first failure while the run is open.** `ci.py conclusion` printed `verdict: red`
  with the coverage job still running; whether the boot smoke was the ONLY red was known 14 minutes later, from a
  jobs read after the run closed.
- **The reading count's denominator is every `ci` run since `b3ac58a`, on any branch** (twelve), not the build
  branch's alone (ten). The last wrap's "nine" was counted the same way.
- **A guard refusal of this session:** the Bash guard refused a call that carried a stray `cat` heredoc beside a
  python heredoc; the record was appended with an anchored edit instead.

## Outcome

**Acceptance criteria, each re-asserted against the diff:**

1. **(P-103)** MET. On this Linux dev host `cargo xtask pre-push:linux`, started with Node 24 first on PATH, exits 0
   and prints one JSON verdict reading `green` / `all-stages-ok`, its stages `script-modes`, `source-lint`, `npm`,
   `clippy`, `test`, `ci-gates` in that order, each `ok`; the same document stands at
   `target/pre-push/report.json` (compared after each run). Read twice: at implement on the working tree, and in the
   operator pass on the committed tree `cb8cc4dc`, whose verdict's `tree` equals the commit's tree. The two files
   that held the hop contain no `wsl` in any letter case (the census entry: exit 1, last line 0).
2. **(arch) three exits, one verdict, the twin, six members.** MET — the member set is pinned by set equality and
   read from the twin.
3. **(arch, tests) a missing tool or pin reads `cannot-evaluate`, names it, never green.** MET — live with no Node
   on PATH (`provisioning-missing`, `missing: ["node:24 (found none)", "tool:npm"]`, exit 2), and per arm by the
   provisioning pins.
4. **(arch, security) every stage child receives a constructed environment.** MET — pinned by set equality; the
   mutation that added a session variable turned the pin red.
5. **(security, obs) no stage resolves the developer's default data dir.** MET — pinned with the stage environment.
6. **(security) the verdict holds no environment value and no path.** MET — the twin of each green run holds no
   `/`; the member set is pinned.
7. **(tests) the check leaves the tracked bindings as it found them.** MET — the restore pinned for a succeeding
   action, a failing one and a file absent before; the second bindings close and the scope guard clean after the
   green run; after the committed-tree run `git status --short` read 0 rows.
8. **(tests) every decision carries a host-independent pin, each turned red once by its mutation.** MET — four
   mutations, each confirmed present, each red, all reverted (`evidence/mutation-checks.md`).
9. **(tests) the standard gate set passes in its order.** MET at implement.
10. **(obs) `ci-gates` over the seed prints `zero-panic PASS`, `heartbeat-gap PASS`, `perf-budget NEUTRAL`.** MET
    on both green runs.
11. **(obs) a recorded wall time states its load and is not a budget.** MET (`evidence/live-readings.md`,
    `evidence/operator-pass.md`).
12. **(security) the dependency graph is unchanged.** MET — the scope guard.
13. **(a11y) the diff holds no rendered-surface source, a11y spec, fixture or baseline.** MET — the scope guard.
14. **(a11y) CI's `a11y` job is green on the chunk's tip.** MET on the job itself (`a11y (ubuntu-22.04)` success,
    279 s, on `cb8cc4dc`). The plan reads this criterion through the CI entry, whose green needs every check of
    the run; that entry is red on the boot smoke alone (below).

**Gates** (the plan's entries by `run`; the first twenty from implement's one whole-block run, `entries 23 · green
18 · red 0 · recorded 2 · timeout 0 · not-run 3`, no fix iteration):

- `cargo fmt --check` — green · exit 0
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — green · exit 0
- `cargo xtask check:english-sources` — green · exit 0 · `"verdict": "clean"`
- `cargo xtask capability-widening-check` · `cargo xtask check:ingest-progress` · `cargo xtask
  check:staged-artifacts` · `cargo xtask capability-drift` · `cargo xtask verify:capability-matrix` — each green ·
  exit 0
- `cargo nextest run --workspace --profile ci` — green · exit 0 · 2836 passed, 0 skipped
- `cargo nextest run -p pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'` — green
- `git diff --quiet 8f655caef468e90844fef54dcc33cef7bd4e426c -- pulse-app/ui/src/bindings/index.ts` — green, both
  times it is listed (before the live runs and after the green one)
- `cat xtask/src/pre_push.rs xtask/src/main.rs | grep -i -c wsl` — green · exit 1 · last line 0
- `d="$(mise where node@24)" && "$d/bin/node" --version` — green · `v24.`
- `env PATH="$HOME/.cargo/bin:/usr/bin:/bin" cargo xtask pre-push:linux` — green · exit 2 · `cannot-evaluate` ·
  `provisioning-missing` · `node:24 (found` · no `green`
- `cat /proc/loadavg` (before) — recorded · `6.76 6.82 8.57`
- `d="$(mise where node@24)" && PATH="$d/bin:$PATH" cargo xtask pre-push:linux` — green · exit 0 · 90.25 s · the
  artifact fresh · every atom held. Run again by hand in the operator pass on the committed tree: exit 0, green,
  89 s, every atom held.
- `cat /proc/loadavg` (after) — recorded · `10.56 7.59 8.65`
- the report-twin read (`python -X utf8 -c "import json; raw = open('target/pre-push/report.json' …`) — green ·
  the six members, the six stages in order, each ok, no path
- the scope guard (`git diff --name-only 8f655caef468… -- crates pulse-app xtask scripts docs …`) — green · no output
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/gate.py hygiene` — `leg = 'operator'`, driven once by
  hand: exit 0 · `hygiene: clean` (`evidence/operator-pass.md`, Entry 21)
- `git diff --quiet && git diff --cached --quiet && git push origin build/andromeda-pulse-0.4.0` — `leg =
  'operator'`, driven once by hand on the operator's word: exit 0 · `8f655cae..cb8cc4dc` · the remote tip equal
  (Entry 22)
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/ci.py conclusion --sha HEAD --wait 2700` — `leg =
  'operator'`, driven once by hand: exit 0 · `cb8cc4dcb7e4 verdict: red · checks 7/7 · first-fail +563 s boot
  smoke (ubuntu-22.04)` (Entry 23). Its `contains verdict: green` atom did not hold. A red of the subject the folded
  watch names: a recurrence, returned to route-resolve's Recurrence-watch row before the flip. **red — not this
  chunk's: the same signature read red on `b3ac58a9` (`ci#37924991598`) and `f36a2ac3` (`ci#37961031489`), two trees
  without this chunk's edits, and the process that ended was built from sources equal to the chunk base → the
  markerless entry "Boot smoke's self-end named from a run" (P-129), minted first in the tail at this wrap's
  route-resolve.** The basis is the operator's word at the route-resolve card ("2 - yes, no control run", the
  operator, 2026-10-10), who added the second half as measured by him; re-derived here before it was written:
  - the process that ended is `pulse-app` (the settle verdict's pid 7444 is the pid `boot: ready` printed and the
    one `boot.log`'s line names);
  - no workspace package depends on `xtask` (`cargo tree -i xtask --workspace --edges normal,build,dev`: `xtask`
    alone, nothing above it). The operator's wording was that no manifest names `xtask`; one does, in a comment
    (`crates/ui-bridge/Cargo.toml:27`), and none as a dependency;
  - this chunk's diff against the base holds `xtask/src/pre_push.rs` and `xtask/src/main.rs` alone (`git diff
    --name-only 8f655cae -- crates pulse-app xtask scripts .github Cargo.toml Cargo.lock`: 2 lines), so the
    `pulse-app` binary of that run was built from sources equal to the base's;
  - the smoke step's first call is `bash scripts/agent-run.sh boot`, which builds `pulse-app` before it builds
    `xtask` (`scripts/agent-run.sh:48`, `:55`). One precision on "before any xtask verb": inside `boot`, after the app
    is spawned, the script polls `cargo xtask harness:ready` (`:110`), so an `xtask` binary built from this chunk's
    tree did run beside the app. That verb and `harness:settled` do not reach the changed module (`grep -n pre_push
    xtask/src/harness_ready.rs xtask/src/harness_status.rs`: 0 lines), and this chunk's base read green under the
    same verbs (`ci#37990081874`).
  No control run was made, on the operator's word; the run was not re-run.
- Smoke: skipped — no boot-path or UI-surface change; the changed verb itself ran live.

**Watches:**
- the CI boot smoke ending by itself after ready — **RECURRED** `ci#38010977166` on `cb8cc4dc` · settle verdict
  `ended`, `exit 1`, `app_exit_record: absent`, the app's last record 0.072 s after the webview's first call. Green
  readings since the last wrap, before it: `ci#37986543962` (`8a89e368`) and `ci#37990081874` (`8f655cae`); with
  `ci#37979648967` (`e2931127`) that is three green from the count's start, then this red. Three reds in twelve
  readings since `b3ac58a`.

**Outcome basis.** The operator pass ran: the gate verdicts rest on its final state — Setup's commit list (`cb8cc4dc`
alone), the committed-tree run of the verb, and the final HEAD's CI run as recorded in `evidence/operator-pass.md`.
Implement's report, given in this same session, stays the basis for what only it holds (the first green run, the
mutation controls, the cannot-evaluate reading). Between implement and this report: the operator's word accepting the
six deviations and ordering the pass, and the wrap directive inputs#I2. The implement conversation is present in this
window; the evolve records were not read as a basis.

**Process hygiene.** Implement's census: the gate block and its children terminated; a `cargo-mutants` run of another
project and its children left running, not this run's. Re-measured at the operator pass's end (`ps`, 2026-10-10
after 01:17Z): no `pre-push:linux`, `agent-run` or `pulse-app` process. The operator pass started the verb once and two
bounded waits on the CI run; all three returned.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 8f655cae (the parent of the oldest pre-CI commit cb8cc4dc) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### xtask/src/main.rs — added 1 line(s) in 1 range(s)
added: 263
### xtask/src/pre_push.rs — added 592 line(s) in 54 range(s)
added: 1-2 · 6-14 · 16 · 18 · 27-28 · 31 · 41-43 · 105-117 · 124 · 134-139 · 141-142 · 149 · 153-167 · 169 · 172-173
       179 · 181-182 · 187-195 · 227-231 · 233-239 · 249-262 · 273-276 · 278-286 · 291-298 · 301-309 · 312-317 · 319
       321-324 · 328-388 · 402-433 · 437-441 · 443-451 · 455-456 · 458-462 · 464-471 · 474 · 477-508 · 511 · 514 · 516
       520-524 · 527-550 · 605-608 · 611 · 616-649 · 653-712 · 726-732 · 782-788 · 790-792 · 794-800 · 804-816 · 818-819
       821-839 · 842-950
  - 107-109 «if fs::create_dir_all(&dir).is_ok() {»
- 134-138 @136 «fn host_supported(os: &str) -> bool {»
  - 153-155 «let Some(home) = std::env::var_os("HOME").filter(|home| !home.is_empty()) else {»
  - 158-163 «let host = Host {»
- 187-194 @189 «struct Host<'a> {»
- 328-331 @329 «fn dir_holding(path: &OsStr, name: &str) -> Option<PathBuf> {»
- 333-347 @337 «fn stage_path(home: &OsStr, verb_path: &OsStr) -> OsString {»
  - 340-343 «if let Some(node) = dir_holding(verb_path, "node") {»
- 349-368 @352 «fn stage_env(home: &OsStr, verb_path: &OsStr, area: &RunArea, npm: bool) -> Vec<Var> {»
  - 353-360 «let mut env = vec![»
  - 361-366 «if npm {»
- 370-381 @372 «fn command(env: &[Var], dir: &Path, program: &str, args: &[&str]) -> Command {»
  - 374-379 «cmd.args(args)»
- 383-387 «fn succeeds(env: &[Var], dir: &Path, program: &str, args: &[&str]) -> bool {»
  - 384-386 «command(env, dir, program, args)»
- 402-413 @404 «struct Probes {»
  - 416-417 «let answer =»
- 437-441 «fn channel_listed(toolchains: &str, channel: &str) -> bool {»
  - 438-440 «toolchains»
- 477-489 @479 «fn found_version(raw: &str) -> &str {»
  - 482-488 «if version.is_empty() {»
- 491-495 @493 «fn provisioning_stop(missing: &[String]) -> Option<Stop> {»
  - 522-524 «remove_tree(data_dir).is_ok()»
  - 535-539 «let before = match fs::read(path) {»
  - 541-546 «let restored = match &before {»
  - 616-621 «const SESSION_VARIABLES: [&str; 4] = [»
  - 623-636 «fn provisioned() -> Probes {»
  - 638-646 @639 «fn node_dirs(tmp: &TempDir) -> (PathBuf, PathBuf) {»
  - 660-665 @661 «fn nothing_is_missing_on_a_provisioned_host() {»
  - 667-698 @668 «fn each_required_piece_is_named_when_it_alone_is_missing() {»
  - 726-731 @727 «fn node_major_reads_the_real_workflow() {»
  - 843-851 @844 «fn stage_path_leaves_the_node_directory_out_when_the_verb_has_none() {»
  - 853-861 @854 «fn the_per_run_paths_sit_under_the_run_dir() {»
  - 863-879 @864 «fn reset_empties_the_per_run_area_and_nothing_beside_it() {»
  - 881-894 @882 «fn six_stages_in_the_registered_order() {»
  - 896-909 @897 «fn restoring_puts_the_file_back_however_the_action_ended() {»
  - 911-921 @912 «fn restoring_removes_a_file_that_was_absent_before() {»
  - 923-935 @924 «fn a_restore_that_cannot_write_is_a_failure() {»
  - 937-950 @938 «fn the_verdict_document_has_exactly_six_members() {»
