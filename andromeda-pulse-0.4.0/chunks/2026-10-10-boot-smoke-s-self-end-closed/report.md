# Report — 2026-10-10-boot-smoke-s-self-end-closed

**Chunk:** Boot smoke's self-end closed: every self-end counted with its label; the cause behind the ending call closed; equal source reads the same (P-129)
**Date:** 2026-10-10
**Commits:** `4e61553b chore(2026-10-10-boot-smoke-s-self-end-closed): operator pre-CI commit` (parent `95d17008`, the chunk base; `git log --format='%h %s' 95d17008..HEAD`: 1 row)

## Changes (structured — detectors read this)
- **Files:** five, all inside research's two lists (`git diff --numstat 95d17008 -- pulse-app xtask`: 5 rows).
  - `pulse-app/src/xlib_threads.rs` — new, 71 lines.
  - `pulse-app/tests/unit_xlib_threads.rs` — new, 105 lines.
  - `pulse-app/src/main.rs` — +4 −1: the import list (48-50) and one statement (282).
  - `pulse-app/src/lib.rs` — +1: `pub mod xlib_threads;` (42).
  - `xtask/src/harness_series.rs` — +358 −28.
  - Not touched (the plan's scope-guard entry, green at /implement and in the pre-push check): `pulse-app/src/render_posture.rs`,
    `pulse-app/src/observability.rs`, `xtask/src/harness_ready.rs`, `xtask/src/harness_witness.rs`,
    `xtask/src/harness_status.rs`, `scripts/agent-run.sh`, `scripts/agent-run.ps1`, `scripts/exit-witness.c`,
    `.github/workflows/ci.yml`, `pulse-app/tests/quality_gate_workflow.rs`, every manifest, `Cargo.lock`, every
    capability file, every `pulse-app/ui/**` path.
- **Symbols / APIs:**
  - **`pulse_app::xlib_threads`** (new public module of the `pulse-app` lib): `XLIB_LIBRARY: &CStr` = `libX11.so.6`;
    `enum XlibThreads { Initialised, LibraryNotFound, SymbolNotFound, Refused, NotApplicable }` (19-28); `pub fn
    init() -> XlibThreads` (30-37); `#[doc(hidden)] pub fn init_from(library: &CStr) -> XlibThreads`, the Linux body
    (39-65) and the other-platform body returning `NotApplicable` (67-71). On Linux `init_from` calls `libc::dlopen`
    on the library name with `RTLD_NOW`, `libc::dlsym` for `XInitThreads`, and calls the resolved function once
    through a transmuted `unsafe extern "C" fn() -> c_int`; a null handle is `LibraryNotFound`, a null symbol
    `SymbolNotFound`, a return of 0 `Refused`. The handle is never closed. Four `unsafe` blocks, each under a
    `// SAFETY:` comment. It reads and sets no environment variable, emits no log record, starts no thread, takes
    no input from outside the binary (the one library name is a compile-time constant).
  - **One call site:** `xlib_threads::init();` is the SECOND statement of `main()` (`pulse-app/src/main.rs:282`),
    directly after `let render_posture = render_posture::apply_linux_default();` (281, unchanged, still the first)
    and before `tokio::runtime::Builder` builds any thread. Its return value is not used and not logged.
  - **`xtask/src/harness_series.rs`**, all private to the module: `struct EndRead { ended: String, exit_witness:
    &'static str }` (71-76); `BootRecord` gained `end: Option<EndRead>`; `fn ended_by_itself(Option<&str>) -> bool`
    (285-294): true for any exit record but `signal 15 (TERM)` and `signal 9 (KILL)`, false for none; `classify`
    gained a third parameter (the exit record of a boot with no settle verdict) and one arm, `None if
    ended_by_itself(ended) => Ended`; `fn exit_record` (314-319); `fn read_end(data_dir) -> Option<EndRead>`
    (378-389); `fn smoke_record(data_dir) -> Option<BootRecord>` (391-405); `boot_entry` gained a fourth parameter
    (`Option<&EndRead>`); `series_payload`'s third parameter changed from `Option<&Value>` to
    `Option<&BootRecord>`. Two constants: `SPAWN_RECORD_FILE` = `andromeda-pulse.spawn`, `SMOKE_CYCLE` = `smoke`.
  - **New callers of existing readers, their signatures unchanged:** `harness_series::read_end` calls
    `harness_status::{end_file, read_ended, read_pid}` and `harness_witness::label`. `harness_witness::label` now
    has two production callers (`harness_ready::run_settled` and `harness_series::read_end`); before this chunk it
    had one (research.md, Graph impact). `read_pid` is called on the SPAWN record
    (`run/andromeda-pulse.spawn`), not on the pid file, which the boot verb's cleanup removes on a failed boot.
  - No TauRPC procedure, IPC route, port, socket, environment variable, capability or log target added or changed.
- **Crates / modules:** `pulse-app` gained the lib module `xlib_threads` and the test binary `unit_xlib_threads`.
  No workspace member added, removed or renamed.
- **Dependencies:** none added, none bumped. `libc` was already a direct dependency of `pulse-app`
  (`pulse-app/Cargo.toml`, `libc = "0.2.186"`); no manifest and no lockfile line changed. `tao` stays 0.35.0.
- **Schema / config:** `logs/boot-series.json` keeps its member sets (six top members, seven per-boot members), its
  closed `cycle` labels, its verdict set and its exit codes; the member-set pin passed unedited. What a value can
  now be:
  - a per-boot entry whose `verdict`, `app_exit_record` and `windows_settled` are null (no settle read ran) may
    carry a non-null `ended` (the boot's exit record) and a non-null `exit_witness` (one of the six closed labels),
    read from the boot's own data dir; before, all five were null for such a boot;
  - the count `ended` now includes a boot with no settle verdict whose exit record is an end the app made itself;
    such a boot was counted `other`, so the series verdict for it moves from `not-all-settled` to `self-ended`
    (both exit 1);
  - ordinal 1 (`cycle` `smoke`) is listed from the smoke's own exit record and witness label when the top data
    dir holds no `logs/harness-settled.json` and that exit record is an own end; before, ordinal 1 was listed only
    from a settle verdict. It stays out of the counts.
  - Both read values pass through the existing 48-byte printable-ASCII bound. No member, label, kept file or
    witness line kind was added; `KEPT_FILES` and `CYCLE_SCRIPT` are unchanged.
- **Spec-master edits:** none.
- **Counts / qualifiers moved:**
  - `harness_series::tests`: 16 pins → 25 (`grep -c '#\[test\]' xtask/src/harness_series.rs` at the base and now).
    Stated at test-plan §1 `harness-cleanup-verdict-and-boot-spawn-shell-coverage` ("carries 16 unit pins").
  - `pulse-app/tests/*.rs`: 102 files → 103 (`ls pulse-app/tests | grep -c '\.rs$'`). Stated, dated 2026-10-05, in
    the `rules/testing.md` leaf.
  - Production callers of `harness_witness::label`: 1 → 2 (above).
  - Production call sites sound only as an opening statement of `main()`: 1 → 2 (`render_posture::apply_linux_default`
    first, `xlib_threads::init` second). architecture §Occupied Resources → Environment variables states the first
    as "the FIRST statement of `main()`"; that wording stands (inputs#I4) and nothing in it is moved.
  - The workspace suite at /implement: 2891 tests run, 2891 passed, 0 skipped (`cargo nextest run --workspace
    --profile ci`); 31 of them are this chunk's two sets (25 + 6). The base's count was not run in this chunk.
- **Dev-tool versions:** none — no host tool installed, upgraded or read changed. The runner's Xlib package
  (`libx11-6 1.7.5-1ubuntu0.3`) and the dev host's Xlib (1.8.13) are library readings, carried under Cross-project.
- **Harness / gate surface:** `cargo xtask harness:boot-series` — no flag, exit code, verdict word or member
  changed; what it counts and lists changed as under Schema / config. Its former measured limit ("a boot that ends
  before the boot verb reads ready takes no settle verdict, so it is counted `other` with a null `exit_witness`")
  no longer describes it. No xtask verb added; `scripts/agent-run.{sh,ps1}`, `harness:settled`, `harness:ready`,
  `harness:status`, the exit witness and `.github/workflows/ci.yml` are unchanged. The `Boot series (equal source)`
  step stays a gating step of the `boot` job.
  - **A host need of the workspace tests, new with this chunk** (added to this report after the fan-out, on the
    test-plan detector's note that the report did not state it): `unit_xlib_threads`'s found arm
    (`the_found_library_initialises_and_a_second_call_does_too`, `pulse-app/tests/unit_xlib_threads.rs` 20-26) loads
    the host's `libX11.so.6` through `init_from`; a host without the library FAILS that pin, never skips it (the
    comment above the test; the rule the `cc` controls follow). No setup step installs it; green in the
    `lint / test (ubuntu-22.04)` job of `ci#38026637514` attempt 1, on the dev host and in `pre-push:linux`'s `test`
    stage. The symbol arm loads `libc.so.6`. All three arm pins are compiled on Linux only.
- **Cross-project / external claims:**
  - **The CI run this chunk's verdict reads:** `ci#38026637514` (pull_request) on the pushed tip `4e61553b`, merge
    commit `69c81cb339a9` (read from each attempt's kept log), **success on attempts 1, 2 and 3**;
    `secret-scan#38026637098` success. Attempt 1 ran all six jobs, each `success` (`a11y` among them, its two
    artifacts present); attempts 2 and 3 re-ran `boot smoke (ubuntu-22.04)` alone. Per attempt: 8 of 8 boots
    settled, `boot-series.json` `all-settled` with `"settled": 7`, the smoke's settle verdict `settled`, 8 witness
    files of one `loaded` line each. 24 boots, 0 self-ended. Artifacts `logs-boot-Linux` `11660322584`,
    `11660874804`, `11660978789`, expiring 2026-10-24. Record: `evidence/operator-pass.md`. The verdict was taken
    on `4e61553b`; this wrap's commit adds records to that tree and no source.
  - **The two runs before the change (the without arm):** `ci#38019133294` (tip `925be35f`, merge `ab6a1ae6ef0d`),
    7 of 8 boots self-ended; `ci#38022477393` (tip `95d17008`, merge `71ce5340f2ad`), 5 of 8. 12 of 16. Read at
    phase P3 from each run's kept files (`.andromeda/runs/2026-10-10T04-02-31Z-phase/p3-measurements.md`).
  - **Ubuntu jammy packages, read from Ubuntu's own pool:** `libx11-6_1.7.5-1ubuntu0.3_amd64.deb` (sha256
    `d382a306…7d547`), `libx11-xcb1_1.7.5-1ubuntu0.3_amd64.deb` (`02c7db5d…6806`), `libgtk-3-0
    3.24.33-1ubuntu2.2`. The runner's Xlib 1.7.5 does not initialise its thread support at load; the dev host's
    1.8.13 does (a ten-line probe of `_Xglobal_lock`, phase P3).
  - **`tao 0.35.0`** (the locked version, read in the cargo registry at phase P3): it starts a device-event thread
    on X11 that opens its own display and loops on `XNextEvent`; its one `XInitThreads` call is not on the start
    path; 0.35.3 and 0.37.1 are the same.
  - **The defect is in the app as it was published before this chunk** (inputs#I5 item 6, the operator's fact for
    the record): the app used Xlib from two threads without the call on every start, so on a host whose Xlib is
    older than 1.8 it can end by itself in its first second. Measured in this chunk on the base `95d17008` alone
    (26 of 32 boots under the runner's Xlib on the dev host; 12 of 16 on the runner). No published build was
    booted here. What is done about anything published is the founder's; the operator carries it.
  - **Inputs** (`inputs.py verify`, this wrap): `I1 · ../additional/pc-overseer/relays/pulse-phase-boot-smoke-self-end-closed-2026-10-10.md · copy · unchanged` ·
    `I2 · message (the /andromeda-phase invocation) · copy · n/a` · `I3 · message (the P4 ruling) · copy · n/a` ·
    `I4 · message (the P5 review) · copy · n/a` ·
    `I5 · ../additional/pc-overseer/relays/pulse-wrap-boot-smoke-self-end-closed-2026-10-10.md · copy · unchanged`
    (snapped at this wrap and cited here: inputs#I5). No drift, nothing vanished or broken, no `UNPARSED:` row.
- **Reverted / negative API facts:** none shipped and removed. Three mutations were applied and removed by hand at
  /implement (`evidence/mutation-checks.md`); none is in the tree.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - obs-plan §7 Error classes captured → Process-end cause: "Not measured: why the X connection's read failed".
    Measured at phase P3 under a debugger: at the stop in `_XIOError` the connection reports no error
    (`xcb_connection_has_error` reads 0), so no read failed; Xlib raised the I/O error because its own bookkeeping
    held no reply for the request while a second thread (`tao`'s device-event thread) was inside Xlib. One capture
    of one ending; the variation (0 of 8 with the initialisation against 26 of 32 without) is what shows the cause.
  - Same sentence: "and the runner's own library build (… recorded from a source read, not from the runner)". Read
    at phase P3 from Ubuntu's packages: the three Xlib frames of every `end` line are exact return addresses in
    `libx11-6 1.7.5-1ubuntu0.3`, and the GDK function is `gdk_x_io_error` in `libgtk-3-0 3.24.33-1ubuntu2.2`.
  - Same paragraph: the end line's `errno` 11 is stated as a member of the ending call. It is a stale value: no
    system call failed at the stop (inputs#I1 item 5, verified at P3).
  - Same paragraph: "The end is named, not closed; its owner is the route's closing entry." Closed by this chunk
    (Outcome).
  - architecture §Occupied Resources → xtask CLI surfaces, the `harness:boot-series` row, and test-plan §9 Boot
    smoke (harness): the "measured limit" sentence, and "exit … 1 `self-ended` (a boot's settle verdict read
    `ended`)" / "a boot whose settle verdict reads `ended` fails the job" as the only way to `self-ended`. Both are
    superseded by the change (Schema / config), not by a measurement.
  - plan.md, Implementation step 2: "with the own-end decision made to return false, the first and third pins go
    red". Four went red (the smoke's listing and the still-`Ended` half of the unreadable-witness pin pass through
    the same decision). A forecast in a chunk artifact; reported here, no edit.
- **Expected amendments (from plan):**
  - architecture §Occupied Resources → xtask CLI surfaces, the `harness:boot-series` row — **carried** (Schema /
    config; Harness / gate surface; Symbols, the second caller of the witness reader). Sites: `harness:boot-series`
    6 hits in `architecture.md` (218, 244, 245, 249, 253, 258) and 1 in
    `registries/contracts/architecture/ci-cd-approach.md` (3); `[Mm]easured limit` 1 hit (258).
  - architecture §Stack and Technologies, the Xlib thread initialisation — **carried** (Symbols / APIs; the one
    call site). Sites: `XInitThreads|Xlib|libX11` 0 hits in `architecture.md`; the fact is new there. The
    `__NV_DISABLE_EXPLICIT_SYNC` row (`architecture.md:242`, "the FIRST statement of `main()`") is not amended
    (inputs#I4).
  - architecture §Infrastructure Patterns → CI/CD approach, the readings — **carried** (Cross-project). Site: the
    key file `registries/contracts/architecture/ci-cd-approach.md` (`ci#38019133294` 1 hit, line 3);
    `ci#38022477393` 0 hits anywhere.
  - test-plan §9 Pipeline structure → Boot smoke (harness), the measured-limit sentence — **carried**. Site:
    `test-plan.md:496` (`[Mm]easured limit` 1 hit).
  - test-plan §1 Pending coverage triggers → `harness-cleanup-verdict-and-boot-spawn-shell-coverage` — **carried**
    (Counts; Schema / config; Outcome for what stays owed). Site: `test-plan.md:134` (1 hit of the trigger's name);
    the key file `registries/contracts/test-plan/5-command-implementation.md` names the trigger 3 times (4, 6, 34).
  - test-plan §3 `boot`, "stays with the route's closing entry" — **carried**. Site: `closing entry` 1 hit in
    `test-plan.md` (134) and 1 in the key file `registries/contracts/test-plan/5-command-implementation.md` (5),
    which also holds the one `P-129` hit and one `XInitThreads|Xlib|libX11` hit of the registries.
  - obs-plan §7 Error classes captured → Process-end cause — **carried** (Spec claims disproved; Outcome). Site:
    `obs-plan.md:396` (`Process-end cause` 1 hit; `closing entry` 1 hit).
  - obs-plan §10 CI gates, the readings — **carried** (Cross-project). Sites: `harness:boot-series` 2 hits in
    `obs-plan.md` (499, 545); `ci#38019133294` 2 hits (396, 499).
  - security-plan §Security Anti-Patterns → Input and its two restating sites — **carried** (Symbols: the series
    reads the spawn record, the exit record and the witness label through the same bounded readers). Sites:
    `exit[- ]witness` 3 hits in `security-plan.md` (85, 138, 395). The classification stays PROVISIONAL
    (inputs#I5 item 7); nothing in this chunk changes what it classifies: no member, no kept file, nothing new
    read from inside the app's process.
  - Search basis for every line above: one script over the seven masters and all 33 files under
    `.andromeda/registries/`, per pattern, hits with line numbers (this wrap, 2026-10-10).
- **Coverage of new surfaces:**
  - `pulse_app::xlib_threads::init` (a start-path call into a system library) → validation n/a (no external input:
    the library name is a constant; a missing library or symbol is inert) · instrumentation n/a (no record by
    ruling, inputs#I1 item 8; no subscriber exists at that point) · PII n/a · tests unit (`unit_xlib_threads`, 6 on
    Linux: three arms, the soname, the place in `main`, the one call site) + the local close leg + three CI
    attempts · a11y n/a · tokens n/a
  - `harness:boot-series`, the before-ready read (`read_end`, `smoke_record`) → validation mechanism✓ (the exit
    record through `read_ended`'s 48-byte grammar, the pid through `read_pid`, the witness file through the
    bounded reader; both listed values through `bounded_label`) · instrumentation n/a (a harness verdict) · PII
    n/a (closed labels and a bounded exit record; no path, no environment value) · tests unit (9 new pins, a
    `TempDir` fixture among them) + the local repair leg on a real process · a11y n/a · tokens n/a

## Deviations from intent
- **`series_payload`'s smoke parameter became `Option<&BootRecord>`.** The plan requires the member-set pin, which
  calls it with three arguments, to pass unedited, and names no parameter shape. The older smoke pin was
  re-pointed to build a record; `classify` and `boot_entry` each gained an argument, so their existing pins gained
  a `None`. Accepted by the operator: "Deviations accepted as recorded".
- **Two pins beyond the plan's list:** a settle verdict outranks what the data dir held; a top dir whose exit
  record is the verb's own `TERM` is not listed. Both are arms of the code step 1 describes. Accepted, same word.
- **The one-call-site pin scans `pulse-app/src/*.rs` and is not marked `andromeda:walks-tree`,** like the unmarked
  ratchet beside it (`pulse_app_src_carries_no_new_dead_test_attributes`). The operator: "the one-call-site pin
  stays unmarked like the ratchet beside it".
- **Phase 3 of /implement was recorded from Phase 2.** The three smoke entries ran as gate entries on the built
  binary and were not driven again by hand. Accepted, same word.
- **The pre-push check before the push was run by hand, not through the gate tool,** so the committed gate trail
  was not rewritten ahead of the push's clean-tree guard (`evidence/operator-pass.md`).
- scope record: none — `gate.py scope` clean, 0 recorded (`changed 5 · listed 5`, base `95d17008`, read at this
  wrap).

## Decisions & corrections
- **The operator's word after the implement report** (2026-10-10, quoted whole in `evidence/operator-pass.md`):
  deviations accepted; the pin stays unmarked; run the operator pass in the plan's order; a self-end in any
  attempt stops the pass; read every kept witness file whole; on three green attempts write the P-129 ref; start
  no skill.
- **The wrap directive** (inputs#I5): the standing rule "until closed the boot job reads red on most runs" is
  consumed by this chunk, and no line of the tail or the handoff tells a later chunk to expect a red boot job; the
  series stays a gating step and a self-end in it is a new reading, brought to the operator. Leaving owners are
  pinned: the module, its call and its test leave with `Window retired`; the carry on `Window's gates retired` is
  read against what changed in the series. The series' first boot building again is presented at the route card.
  A proposal graded `escalate` halts at the card. The witness record's classification stays PROVISIONAL.
- **A harness tool's own trail is tracked once committed.** Between the pre-CI commit and the push, running a
  listed entry through the gate tool with the run dir rewrites a committed file and trips the push's clean-tree
  guard. The pre-push check was run by its plain command for that reason.
- **A guard refused a heredoc append to an evidence file** during the pass (one probe, then an anchored edit). The
  host rule already states it.
- **The evolve playbook for the smoke step was read in the same tool batch as the fix-loop record,** before the
  Phase 3 verdict was stated; recorded in that step's own record. No boot ran after the read.
- **Sweep hazard:** `git diff --quiet` does not see untracked files, so a new evidence file does not trip the
  push's clean-tree guard while a modified tracked one does.

## Outcome
- **Acceptance criteria, each against the diff:**
  - The series counts a before-ready self-end; `TERM` and `KILL` are not self-ends; each arm pinned; the mutation
    check recorded — **met** (25 pins green; `evidence/mutation-checks.md`).
  - On a real process, ordinal 1 with `"ended": "exit 101"` and `"exit_witness": "runtime-exit"`, the series
    `all-settled` for its own boot — **met** (the repair leg, red at the base).
  - `logs/boot-series.json` keeps its registered shape; the member-set pin passes unedited — **met**.
  - No member added to a witness line, no interposed call, no kept file, no product source reads a witness
    variable; a witness file outside the grammar reads `unreadable` and the boot stays red — **met** (the scope
    guard prints nothing; `grep -rn ANDROMEDA_PULSE_EXIT_WITNESS pulse-app/src crates | wc -l` reads 0; pinned).
  - `main` calls the initialisation second, after the render-posture step and before the runtime; `render_posture.rs`
    byte-identical to the base; inert where the library or symbol is not found; pinned, the two mutation checks
    recorded; no environment variable, log record, dependency, member, procedure, port or capability added —
    **met**.
  - The close on the dev host under the runner's Xlib: `ldd` under the variable reads 2, the close leg reads
    `all-settled`, `"settled": 8`; the swap read undone at 0 — **met** (7 of 8 self-ended at the base under the same
    command).
  - The host leg reads `all-settled` with its kept files — **met**.
  - The standard gate set in its order, the bindings close exit 0, `pre-push:linux` green — **met** (at /implement
    and again on the committed tree).
  - **P-129:** three consecutive green attempts of one pull-request run of equal source, each with its kept files
    as stated — **met** (`ci#38026637514`, attempts 1 to 3, one merge commit; `evidence/operator-pass.md`). Ref
    written on the operator's word; `planned → implemented`.
  - a11y: the `a11y` job concluded `success` on the pass's run with its artifacts present; `ci.yml` untouched —
    **met**.
  - obs: no product log record added or changed; `observability.rs` untouched — **met**.
  - security: every witness file read in this chunk is recorded as read whole with its count beyond the shape —
    **met** (local legs: 23 files, 24 lines, 0; the pass: 24 files, 24 lines, 0; phase P3: 24 files, 42 lines, 0).
- **Gates** (the plan's entries by `run`; /implement's block, 24 green of 24 on its first full run, 0 fix
  iterations):
  - `cargo fmt --check` — green · `cargo clippy --workspace --all-targets --all-features -- -D warnings` — green
  - `git diff --name-only 95d17008… -- crates pulse-app xtask scripts docs …` (the scope guard) — green, no output
  - `grep -rn ANDROMEDA_PULSE_EXIT_WITNESS pulse-app/src crates | wc -l` — green, exit 1, last line 0
  - `cargo nextest run -p xtask --profile ci -E 'test(/harness_series::/)'` — green, 25 passed
  - `cargo nextest run --workspace --profile ci -E 'binary(unit_xlib_threads)'` — green, 6 passed
  - the witness build, `cargo build --bin pulse-app --release`, the jammy fetch (both package checksums OK) — green
  - `LD_LIBRARY_PATH=… ldd … | grep -c 'target/jammy-x11/lib/libX11'` — green, last line 2
  - the close leg (`harness:boot-series --count 8` under the swap) — green: `all-settled`, `"settled": 8`
  - the repair leg (`DISPLAY=:987`, then `--count 1`) — green
  - the host leg (`--count 2`) — green
  - `ldd target/release/pulse-app | grep -c 'jammy-x11'` — green, exit 1, last line 0
  - `cargo xtask check:english-sources` · `capability-widening-check` · `check:ingest-progress` ·
    `check:staged-artifacts` · `capability-drift` · `verify:capability-matrix` — green
  - `cargo nextest run --workspace --profile ci` — green, 2891 passed, 0 skipped
  - the `--features mcp-server` bindings regen — green · `git diff --quiet 95d17008… -- pulse-app/ui/src/bindings/index.ts` — green
  - `d="$(mise where node@24)" && PATH="$d/bin:$PATH" cargo xtask pre-push:linux` — green, six stages
  - **`leg = 'operator'` entries, driven once by hand at the operator pass, each recorded in
    `evidence/operator-pass.md`:** `gate.py hygiene` — green (`hygiene: clean`) · the push — green
    (`95d17008..4e61553b`) · `ci.py conclusion --sha HEAD --wait 2700` ×3 — green, `verdict: green`, then with
    `attempt 2` and `attempt 3` · the jobs read of attempt 1 — recorded: six jobs, each `success` (the chunk's CI
    verdict is green; nothing to disposition) · the boot job log read ×3 — green (the frame line present, no
    `app ended:`, no `Process completed with exit code`) · the artifact read ×3 — green (all six atoms) · the
    two `gh run rerun … --job` entries — green (the run read attempt 2, then 3).
  - Smoke: the boot path changed (`main.rs`); the three legs above are that smoke in the harness `boot` form.
- **Watches:** none folded on this entry. The entry's own standing rule (its fourth `CARRY:`: "until this entry
  closes the cause the boot job reads red on most runs") is consumed: three attempts, 24 boots, none self-ended.
- **Outcome basis:** the operator pass ran, so the verdicts rest on its final state: the one commit `4e61553b` and
  the final HEAD's run `ci#38026637514` recorded in `evidence/operator-pass.md`. /implement's report, given in
  this session, is the basis for what only it holds (the block's 24 entries, the mutation checks, the local legs,
  the deviations). The operator's word after it changed nothing in the tree. Post-implement artifacts:
  `evidence/operator-pass.md`, the P-129 ref in `verification-matrix.json`, the ledger tool's trail in the
  implement run dir.
- **Limits, stated as they stand** (inputs#I5 item 5):
  - The before-ready read never ran on a runner: all 24 boots of the three attempts reached ready. It is read on
    a real process by the local repair leg (the smoke's own boot) and pinned per arm for a series boot.
  - The step inside Xlib by which the reply is lost is not traced.
  - The close leg takes the mapped library from `ldd` under the same variable and from the base's result under
    the same command, not from the running process's own map.
  - One series of eight on the built change under the swapped library; the base read four.
  - Attempts 2 and 3 ran the `boot` job alone; the other five jobs were read once.
- **Found and not owned by this chunk:** the series' first boot builds again on the runner in every attempt
  (3 min 3 s, 3 min 34 s and 3 min 33 s between the smoke's `boot: ready` line and boot 2's; later boots about
  8 s apart), as in the two runs before. Cause unmeasured: no series boot keeps its `build.log`. On the dev host a
  shell-launched build after four series compiled nothing (phase P3). For disposition at the route card
  (inputs#I5 item 4).
- **Process hygiene:** /implement's census — 12 `pulse-app` boots, each under its own `xvfb-run`, started by gate
  entries 11 to 13, each cycle `cleanup: clean`; `cargo` and `nextest` children of the gate and mutation runs —
  all terminated, measured by `ps` after the block (no `pulse-app`, `Xvfb`, `xvfb-run`, `agent-run`). The operator
  pass started no app process; `ps -eo pid,comm` read 0 rows for `pulse-app` and `Xvfb` after it. This wrap has
  started one process so far, the code-graph refresh, which returned (exit 0). No listener on 14317, 14318, 4317
  or 4318 was read after /implement's block. Left under `target/` (ignored): `target/jammy-x11/`,
  `target/exit-witness/exit-witness.so`, three local data dirs `target/boot-smoke/20261010T05*`, and the three
  downloads `target/boot-smoke/ci-38026637514-attempt-{1,2,3}`.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 95d17008 (the parent of the oldest pre-CI commit 4e61553b) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### pulse-app/src/lib.rs — added 1 line(s) in 1 range(s)
added: 42
### pulse-app/src/main.rs — added 4 line(s) in 2 range(s)
added: 48-50 · 282
- 48-50 «use pulse_app::{»
### pulse-app/src/xlib_threads.rs — new file · 71 line(s)
- 19-28 @20 «pub enum XlibThreads {»
- 30-37 @35 «pub fn init() -> XlibThreads {»
- 39-65 @41 «pub fn init_from(library: &CStr) -> XlibThreads {»
  - 45-47 «if handle.is_null() {»
  - 51-53 «if symbol.is_null() {»
  - 54-58 @57 «let init_threads: unsafe extern "C" fn() -> libc::c_int =»
  - 59-64 @61 «match unsafe { init_threads() } {»
- 67-71 @69 «pub fn init_from(_library: &CStr) -> XlibThreads {»
### pulse-app/tests/unit_xlib_threads.rs — new file · 105 line(s)
- 15-18 @16 «fn the_product_library_is_the_xlib_soname() {»
- 20-26 @23 «fn the_found_library_initialises_and_a_second_call_does_too() {»
- 28-35 @30 «fn a_library_that_is_not_there_is_inert() {»
  - 31-34 «assert_eq!(»
- 37-41 @39 «fn a_library_without_the_symbol_is_inert() {»
- 43-47 @45 «fn other_platforms_are_not_applicable() {»
- 49-51 «fn src_dir() -> std::path::PathBuf {»
- 53-68 @54 «fn main_body_lines() -> Vec<String> {»
  - 57-60 «let opening = lines»
  - 61-67 «lines[opening + 1..]»
- 70-80 @71 «fn main_calls_it_second_after_the_render_posture_step_and_before_the_runtime() {»
  - 75-78 «let runtime = body»
- 82-105 @83 «fn the_product_entry_has_one_call_site_outside_its_module() {»
  - 85-103 «for entry in std::fs::read_dir(src_dir()).expect("read pulse-app/src") {»
### xtask/src/harness_series.rs — added 358 line(s) in 32 range(s)
added: 11-14 · 23 · 36-39 · 71-77 · 83-84 · 92 · 153 · 159-163 · 167-168 · 285-297 · 300 · 314-320 · 322 · 378-406
       420-423 · 425-434 · 439 · 442 · 446-449 · 458-466 · 509 · 513-554 · 629-633 · 636 · 638 · 642 · 645-772 · 870
       884-945 · 955 · 963 · 966-972
- 71-76 @73 «struct EndRead {»
  - 160-163 «let end = match settle {»
- 285-294 @289 «fn ended_by_itself(ended: Option<&str>) -> bool {»
  - 290-293 «!matches!(»
- 314-319 «fn exit_record(record: &BootRecord) -> Option<&str> {»
  - 315-318 «match &record.settle {»
- 378-389 @380 «fn read_end(data_dir: &Path) -> Option<EndRead> {»
  - 383-384 «let exit_witness =»
  - 385-388 «Some(EndRead {»
- 391-405 @393 «fn smoke_record(data_dir: &Path) -> Option<BootRecord> {»
  - 395-398 «let end = match settle {»
  - 399-404 «(settle.is_some() || end.is_some()).then_some(BootRecord {»
  - 425-434 «let (ended, exit_witness) = match (settle, end) {»
  - 513-529 @514 «fn unsettled(»
  - 533-547 @535 «fn left_by_a_boot(spawn: bool, ended: &str, witness: &str) -> tempfile::TempDir {»
  - 549-553 «fn witness_of_an_exit_call() -> String {»
  - 652-675 @653 «fn a_boot_that_ended_by_itself_before_ready_is_ended_with_its_record_and_label() {»
  - 677-693 @678 «fn the_boot_verbs_own_term_and_kill_are_not_self_ends() {»
  - 695-700 @696 «fn another_signal_is_an_end_the_app_made_itself() {»
  - 702-718 @703 «fn no_exit_record_is_other_with_null_members() {»
  - 720-734 @721 «fn a_settle_verdict_outranks_what_the_data_dir_held() {»
  - 736-758 @737 «fn a_boot_dir_is_read_to_its_exit_record_and_witness_label() {»
  - 884-918 @885 «fn the_smokes_own_end_is_listed_as_ordinal_1_and_stays_out_of_the_counts() {»
  - 920-944 @921 «fn a_top_dir_is_listed_from_its_settle_verdict_and_not_from_the_verbs_own_term() {»
