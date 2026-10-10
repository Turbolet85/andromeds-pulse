# Fan-out results — 2026-10-10-boot-smoke-s-self-end-named-from-a-run

Seven doc-agents, one batch. Each return is the agent's own YAML, collected by script from its hand-back
(entity-decoded; the probe counts below are of the decoded text). Dispositions follow the lists.

## architecture — proposals 17 · entities after decode 0 · nothing stripped

```yaml
proposals:
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → xtask CLI surfaces'
    change: 'Register `cargo xtask harness:boot-series --count N` (`xtask/src/harness_series.rs`, `Cmd::HarnessBootSeries`; chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run; same formalized-CLI-contract rule): it boots the release app N more times, ordinals 2 to N+1 (the CI smoke is ordinal 1), one boot at a time, each one cycle `boot` → `harness:settled` → `status` → `cleanup` (after a failed `boot` the two middle verbs are skipped, `cleanup` always runs) under its own `xvfb-run`, on its own data dir `series/boot-{ordinal}/` under the resolved data dir, with `XDG_DATA_HOME` / `XDG_CACHE_HOME` set to empty directories inside it and `ANDROMEDA_PULSE_PIDFILE` / `_LOGFILE` removed from the boot environment, a cycle bounded at 1200 s; one pretty-JSON verdict on stdout with a twin at `logs/boot-series.json`: `{verdict, boots, settled, ended, other, per_boot}`, each `per_boot` entry `{ordinal, cycle, verdict, ended, app_exit_record, windows_settled, exit_witness}` reduced to labels of at most 48 printable-ASCII bytes and a count; exit 0 `all-settled` · 1 `self-ended` (a settle verdict read `ended`) · 1 `not-all-settled` · 2 `cannot-evaluate` (a count outside 1 to 16, not Linux, no data dir or one already holding `series/`, no `xvfb-run` on `PATH`, no boot run, or a cycle labelled `cleanup-not-clean` / `timed-out` / `no-record`, after which the series stops); `cycle` is closed: `complete` · `boot-failed` · `status-not-healthy` · `cleanup-not-clean` · `timed-out` · `no-record` · `smoke` (ordinal 1, listed from the smoke `logs/harness-settled.json` when the data dir holds one, outside the counts); it binds no port itself (each boot binds the two resolved OTLP ports) and adds no TauRPC procedure, IPC route or capability; wired as the `Boot series (equal source)` step of the ci.yml `boot` job with `--count 7`; measured limit: a boot that ends before the boot verb reads ready takes no settle verdict, so it counts `other` with a null `exit_witness` (`ci#38019133294`: `ended` 2, `other` 4 for seven self-ends); the timed-out path never ran live.'
    sidecar: '2026-10-10 boot-smoke-s-self-end-named-from-a-run: registered the new xtask verb `harness:boot-series` (contract, verdict shape, exit codes, cycle labels, CI wiring, measured undercount) under xtask CLI surfaces.'
    rationale: 'Report Changes → Symbols / APIs lands a new xtask verb with a 0 / 1 / 2 contract and a JSON verdict; Expected amendments records `boot-series` at 0 hits in every master, so the CLI surface is unregistered. The undercount is from Changes → Harness / gate surface and Insufficient fixes.'
    basis: 'xtask/src/main.rs:72-75'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → xtask CLI surfaces'
    change: 'The `cargo xtask harness:settled` Contract object reads eight members, not seven: `{verdict, pid, ended, app_exit_record (present | absent), windows_settled (0 to 4), display, session_bus (each reachable | gone | unset | unknown), exit_witness}`, where `exit_witness` is one closed label from `harness_witness::label` — `unset` (no witness file) · `unreadable` · `loaded` · `exit-call` · `runtime-exit` · `no-record` — read from `logs/exit-witness.jsonl` by a reader that takes at most 64 lines of at most 4096 printable-ASCII bytes carrying one of the three kinds `loaded` / `end` / `runtime-exit` and a `u32` pid, anything else reading `unreadable`, never a partial record; the verdict set, the exit codes and the other seven members are unchanged, and `logs/harness-settled.json` holds the same eight (chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run).'
    sidecar: '2026-10-10 boot-smoke-s-self-end-named-from-a-run: `harness:settled` object seven members → eight (`exit_witness`, a closed six-label set); verdicts and exit codes unchanged.'
    rationale: 'Report Changes → Symbols / APIs: the settle verdict gained an eighth member; Counts / qualifiers moved names the stale seven-member list at architecture §Occupied Resources → xtask CLI surfaces.'
    basis: '.andromeda/architecture.md:252'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → xtask CLI surfaces'
    change: 'The `harness:settled` caller sentence names two callers instead of one: the "Boot pulse-app smoke" step of ci.yml, between `boot` and `status`, and each cycle of `cargo xtask harness:boot-series`, which runs it between the same two verbs and skips it after a failed `boot`.'
    sidecar: '2026-10-10 boot-smoke-s-self-end-named-from-a-run: `harness:settled` gained a second caller, the `harness:boot-series` cycle.'
    rationale: 'Report Changes → Symbols / APIs: each series boot runs one cycle `boot`, `harness:settled`, `status`, `cleanup`; the row still says its caller is the smoke step alone, a restatement the boot-series registration retires.'
    basis: 'xtask/src/harness_series.rs:70-94'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → xtask CLI surfaces'
    change: 'In the `scripts/agent-run.{sh,ps1}` row, add the sh-only witness arm of `boot` and qualify "sh+ps1 in lockstep" by it: `agent-run.sh boot` reads `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`, trimmed — unset or blank, the app is spawned as before; a regular file, the one spawn command runs with `LD_PRELOAD` set to it and `ANDROMEDA_PULSE_EXIT_WITNESS_FILE` set to `{data dir}/logs/exit-witness.jsonl`, neither exported, so the pre-build and every other verb run without them; set and not a regular file, `boot: exit witness library not found` on stderr and exit 1 before the pre-build; the verb also removes a stale `logs/exit-witness.jsonl` of the same data dir before the spawn; `agent-run.ps1` is unchanged and the 5-verb set holds, no sixth verb (chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run).'
    sidecar: '2026-10-10 boot-smoke-s-self-end-named-from-a-run: `agent-run.sh boot` gained the exit-witness arm (sh only; fail-closed on a missing library); ps1 unchanged, verb set unchanged.'
    rationale: 'Report Changes → Symbols / APIs: the boot verb witness arm lands in `scripts/agent-run.sh` alone, with `scripts/agent-run.ps1` unchanged; the registered driver-pair contract carries neither the arm nor the sh / ps1 difference.'
    basis: 'scripts/agent-run.sh:44-56'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → xtask CLI surfaces'
    change: 'In the same `scripts/agent-run.{sh,ps1}` row, where the "Boot pulse-app smoke" step is described, add its two neighbours in the `boot` job (the smoke step lines themselves unchanged): `Build the exit witness` before it, after the release build (`cc` builds `scripts/exit-witness.c` into `$RUNNER_TEMP` and names it through `$GITHUB_ENV` as `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`), and `Boot series (equal source)` after it and before `ci-gates` (`if: always()`, a plain `run: cargo xtask harness:boot-series --count 7`, no soft-fail key); three more workflow pins in `pulse-app/tests/quality_gate_workflow.rs` hold the pair, and the four smoke pins are untouched.'
    sidecar: '2026-10-10 boot-smoke-s-self-end-named-from-a-run: the smoke step row now names its two neighbouring `boot` job steps (witness build before, boot series after) and their three pins.'
    rationale: 'Report Changes → Harness / gate surface: the `boot` job gains two steps around the unchanged smoke step, pinned by three workflow tests; the row describes the smoke wiring as the job harness content with no neighbour.'
    basis: '.github/workflows/ci.yml:375-382'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → Environment variables'
    change: 'Add a row: `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` — harness-only path to the built exit-witness library, read solely by `scripts/agent-run.sh boot` (sh only; `agent-run.ps1` does not read it), trimmed: unset or blank ⇒ the app is spawned as before; a regular file ⇒ loaded into the app spawn alone through `LD_PRELOAD`; set and not a regular file ⇒ `boot: exit witness library not found`, exit 1, before the pre-build; the value is never printed; in CI set through `$GITHUB_ENV` by the `Build the exit witness` step; NOT consumed by the production binary (0 lines in `pulse-app/src` and `crates`) (chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run).'
    sidecar: '2026-10-10 boot-smoke-s-self-end-named-from-a-run: registered harness-only `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`.'
    rationale: 'Report Changes → Symbols / APIs → Environment variables lists it as new; Expected amendments records `EXIT_WITNESS` at 0 hits in all masters and key files, beside the harness-only rows.'
    basis: '.andromeda/architecture.md:242-246'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → Environment variables'
    change: 'Add a row: `ANDROMEDA_PULSE_EXIT_WITNESS_FILE` — harness-only, SET (never exported) by `scripts/agent-run.sh boot` on the app spawn line to `{data dir}/logs/exit-witness.jsonl` and read only by the preloaded library `scripts/exit-witness.c`, which at load opens that file for append, close-on-exec, removes this variable and `LD_PRELOAD` from the process environment, writes a `loaded` line and registers an exit handler; unset ⇒ the library does nothing; file not openable ⇒ it removes the two variables and stays inert; NOT consumed by the production binary own code (0 lines in `pulse-app/src` and `crates`) (chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run).'
    sidecar: '2026-10-10 boot-smoke-s-self-end-named-from-a-run: registered harness-only `ANDROMEDA_PULSE_EXIT_WITNESS_FILE` (set on the spawn line, read by the library).'
    rationale: 'Report Changes → Symbols / APIs → Environment variables lists it as new, set by the boot verb and read by the library at load; it appears in no architecture registry row.'
    basis: 'scripts/exit-witness.c:63-92'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → Environment variables'
    change: 'Add a row: `LD_PRELOAD` — SYSTEM variable, NOT an `ANDROMEDA_PULSE_*` input, SET harness-only by `scripts/agent-run.sh boot` on the app spawn line alone (never exported) when `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` names a regular file, and removed from the process environment by the library at load; the `Build the exit witness` workflow step carries no `LD_PRELOAD` (pinned); read by no `pulse-app` or `crates` code (0 lines) (chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run).'
    sidecar: '2026-10-10 boot-smoke-s-self-end-named-from-a-run: registered the system variable `LD_PRELOAD` as set harness-only on the app spawn line by `agent-run.sh boot`.'
    rationale: 'Report Changes → Symbols / APIs → Environment variables: `LD_PRELOAD` is set on the app spawn line by the boot verb; Expected amendments records `LD_PRELOAD` at 0 hits in all masters.'
    basis: 'scripts/agent-run.sh:92-99'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → Environment variables'
    change: 'Add a row: `XDG_DATA_HOME` · `XDG_CACHE_HOME` — SYSTEM variables, NOT `ANDROMEDA_PULSE_*` inputs, SET harness-only by `cargo xtask harness:boot-series` on each of its boots to empty directories inside that boot data dir (`xdg-data`, `xdg-cache`), so the webview state of that boot lands there; read by no `pulse-app` or `crates` code (0 lines) (chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run).'
    sidecar: '2026-10-10 boot-smoke-s-self-end-named-from-a-run: registered `XDG_DATA_HOME` / `XDG_CACHE_HOME` as set harness-only by `harness:boot-series` on each of its boots.'
    rationale: 'Report Changes → Symbols / APIs → Environment variables: the series sets both on each of its boots; Expected amendments records `XDG_DATA_HOME` and `XDG_CACHE_HOME` at 0 hits in all masters.'
    basis: 'xtask/src/harness_series.rs:152-191'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → Environment variables'
    change: 'In the `HOME` · `PATH` row, add a second harness-only reader of `PATH`: `cargo xtask harness:boot-series` reads it by value only to find `xvfb-run` (none on `PATH` reads `cannot-evaluate`) and never prints it (chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run).'
    sidecar: '2026-10-10 boot-smoke-s-self-end-named-from-a-run: `PATH` gained a second harness-only by-value reader, `harness:boot-series` (to find `xvfb-run`).'
    rationale: 'Report Changes → Symbols / APIs → Environment variables: `PATH` is read by the series by value, only to find `xvfb-run`, never printed; the row names `pre-push:linux` as the reader.'
    basis: 'xtask/src/harness_series.rs:108-111'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → Environment variables'
    change: 'In the `ANDROMEDA_PULSE_PIDFILE` row, add: `cargo xtask harness:boot-series` removes the variable from the environment of each of its boots, so each boot resolves its PID file under its own data dir `series/boot-{ordinal}/` (chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run).'
    sidecar: '2026-10-10 boot-smoke-s-self-end-named-from-a-run: `ANDROMEDA_PULSE_PIDFILE` is removed from each `harness:boot-series` boot environment.'
    rationale: 'Report Changes → Symbols / APIs → Environment variables: `ANDROMEDA_PULSE_PIDFILE` is removed from each series boot environment so it resolves under that boot data dir; the row enumerates who handles the variable and omits this.'
    basis: '.andromeda/architecture.md:242'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → Environment variables'
    change: 'In the `ANDROMEDA_PULSE_LOGFILE` row, add: `cargo xtask harness:boot-series` removes the variable from the environment of each of its boots, so each boot resolves its log family under its own data dir `series/boot-{ordinal}/` (chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run).'
    sidecar: '2026-10-10 boot-smoke-s-self-end-named-from-a-run: `ANDROMEDA_PULSE_LOGFILE` is removed from each `harness:boot-series` boot environment.'
    rationale: 'Report Changes → Symbols / APIs → Environment variables: `ANDROMEDA_PULSE_LOGFILE` is removed from each series boot environment so it resolves under that boot data dir; the row enumerates who handles the variable and omits this.'
    basis: '.andromeda/architecture.md:243'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → Filesystem locations'
    change: 'In the `logs/` subpath entry, the harness-written set the production binary neither writes nor reads is no longer two files: beside `logs/harness-settled.json` and `logs/xvfb.log` it holds `logs/exit-witness.jsonl` (written by the preloaded library, one JSON object per line of at most 4096 bytes, kinds `loaded` / `end` / `runtime-exit`; a stale one of the same data dir is removed by `agent-run.sh boot` before the spawn), `logs/boot-series.json` (the `cargo xtask harness:boot-series` verdict) and `logs/series/boot-{ordinal}/` (copies of that boot log family and of its `boot.log`, `harness-settled.json`, `xvfb.log`, `exit-witness.jsonl`); all ride the `logs-boot-Linux` artifact, which beyond its earlier contents now holds `exit-witness.jsonl`, `boot-series.json` and `series/boot-{2..8}/` with five files each, four where a boot took no settle verdict (chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run).'
    sidecar: '2026-10-10 boot-smoke-s-self-end-named-from-a-run: `logs/` harness-written set grew from two files by `exit-witness.jsonl`, `boot-series.json` and `series/boot-{ordinal}/`; the `logs-boot-Linux` artifact contents updated.'
    rationale: 'Report Changes → Symbols / APIs → Filesystem locations and Harness / gate surface: three new harness-written paths under `logs/` and new artifact contents; the entry states that `logs/` holds two harness-written files; Expected amendments records `exit-witness` at 0 hits in all masters.'
    basis: '.andromeda/architecture.md:217'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → Filesystem locations'
    change: 'Add a subpath entry under the resolved root: `series/boot-{ordinal}/` — the per-boot data dirs `cargo xtask harness:boot-series` creates (ordinals 2 to N+1), each holding `logs`, `run`, `xdg-data`, `xdg-cache` and the product own `corpus`; harness-made, not under `logs/` and not uploaded; a root that already holds `series/` makes the verb read `cannot-evaluate` (chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run).'
    sidecar: '2026-10-10 boot-smoke-s-self-end-named-from-a-run: registered the `series/boot-{ordinal}/` per-boot data dirs of `harness:boot-series` under the data-dir root.'
    rationale: 'Report Changes → Symbols / APIs → Filesystem locations: the series data dirs `series/boot-{ordinal}/` with `logs`, `run`, `xdg-data`, `xdg-cache` and `corpus` are a new subpath under the resolved data dir, absent from the subpath list.'
    basis: 'xtask/src/harness_series.rs:120-150'
  - detector: D-arch-resources
    severity: warning
    section: '§Occupied Resources → Filesystem locations'
    change: 'Add a harness-only entry outside the data dir: the exit-witness source `scripts/exit-witness.c` and its build outputs — `target/exit-witness/exit-witness.so` on a dev host (ignored) and `$RUNNER_TEMP/exit-witness.so` in CI — built by `cc -shared -fPIC -O2`, never product-written, so the THREE product-written out-of-data-dir exceptions are unchanged (chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run).'
    sidecar: '2026-10-10 boot-smoke-s-self-end-named-from-a-run: registered `scripts/exit-witness.c` and its two build-output locations as harness-only paths outside the data dir.'
    rationale: 'Report Changes → Symbols / APIs → Filesystem locations names the source and the local and CI build outputs; Expected amendments lists `scripts/exit-witness.c` for this section with `exit-witness` at 0 hits.'
    basis: 'scripts/exit-witness.c'
  - detector: D-arch-resources
    severity: warning
    section: '§Infrastructure Patterns → CI/CD approach'
    change: 'The `boot` job description reads: the mcp-feature release build → `Build the exit witness` (`cc` builds `scripts/exit-witness.c` into `$RUNNER_TEMP`, named through `$GITHUB_ENV` as `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`) → the agent-harness boot smoke (its lines unchanged) → `Boot series (equal source)` (`if: always()`, a plain `run: cargo xtask harness:boot-series --count 7`, no soft-fail key) → `cargo xtask ci-gates`; one run of the job reads eight boots, a boot whose settle verdict reads `ended` fails the job, `ci-gates` is skipped when the smoke or the series fails and the upload still runs; pinned by three workflow tests in `pulse-app/tests/quality_gate_workflow.rs`; still six jobs on `ubuntu-22.04`, no matrix, no job edge; as measured on `ci#38019133294` (attempt 1, merge commit `ab6a1ae6ef0d…`, chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run): red on the `boot` job alone, seven of eight boots ended by themselves, the three with a witness label reading `exit-call` (`_exit`, code 1, from `libgdk-3.so.0` under Xlib `_XIOError`), the series step 4 min 27 s; the cause is not closed and that red belongs to the closing entry.'
    sidecar: '2026-10-10 boot-smoke-s-self-end-named-from-a-run: CI/CD approach `boot` job gained the witness-build and boot-series steps (a self-ended boot fails the job); recorded the `ci#38019133294` reading.'
    rationale: 'Report Changes → Harness / gate surface: the `boot` job has two added steps and a non-zero series fails it; the keyed contract still describes `boot` as release build, boot smoke, then `ci-gates`. Expected amendments names this key file, whose line 3 describes the six jobs. Readings from Cross-project / external claims and Spec claims disproved by measurement.'
    basis: '.github/workflows/ci.yml:400-407'
    dependent-of: D-arch-resources
  - detector: D-arch-decisions
    severity: warning
    section: '§Stack and Technologies'
    change: 'Add a row — Layer: Boot-smoke exit witness (harness-only, Linux) · Technology: C, the single file `scripts/exit-witness.c`, built by the host `cc -shared -fPIC -O2` with no dependency beyond the C library (dev-host `cc` read at GCC 16.2.1 20260810 on 2026-10-10; the runner compiler version unread) · Role: a shared library `scripts/agent-run.sh boot` preloads into the app spawn alone to record which call ended the process (`exit`, `_exit`, `_Exit`, `quick_exit`, `abort` interposed), built in the ci.yml `boot` job and by the xtask tests, whose seven built-library controls are Linux only and FAIL on a missing `cc`; never in the shipped binary, no Cargo dependency, no manifest or lockfile change (chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run).'
    sidecar: '2026-10-10 boot-smoke-s-self-end-named-from-a-run: §Stack gained a harness-only row for the C exit-witness library built by the host `cc` (a `cc` requirement for the Linux xtask tests).'
    rationale: 'Report Changes → Symbols / APIs: the library is C, built by `cc -shared -fPIC -O2`; Harness / gate surface: seven controls on the built library, Linux only, a missing `cc` fails them; Dev-tool versions gives the `cc` reading; Dependencies: none added. §Stack names Rust as the single language and carries test-only rows for its other non-Rust tooling, but no row allows a C source or a `cc` requirement of the workspace suite. No locked decision is contradicted: the library is harness-only and outside the product.'
    basis: 'scripts/exit-witness.c:124-162'
```

## security-plan — proposals 3 · entities after decode 0 · nothing stripped

```yaml
proposals:
  - detector: D-security-input
    severity: escalate
    section: "§Security Anti-Patterns → Input (the product-binary path env var ban, its harness-only class)"
    change: >-
      After the `2026-10-09-pre-push-check-native-on-linux` entry of the harness-only class, add "The same class since chunk `2026-10-10-boot-smoke-s-self-end-named-from-a-run`" (classified routine harness evidence, the app's exit record — the operator's reading, the pc overseer, 2026-10-10; not the founder's word), and retire the count in "(b) two harness-WRITTEN files … Both ride the boot job's `logs-boot-Linux` artifact": (a) `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`, read by `scripts/agent-run.sh boot` alone (sh only; `agent-run.ps1` unchanged), trimmed — unset or blank spawns the app as before; a regular file puts `LD_PRELOAD` = that file and `ANDROMEDA_PULSE_EXIT_WITNESS_FILE` = `{data dir}/logs/exit-witness.jsonl` on the one spawn command, neither exported, so the pre-build and every other verb run without them; set and not a regular file reads `boot: exit witness library not found`, exit 1, before the pre-build — a FAIL-CLOSED arm, not the class's clean skip on absence; the value is never printed; the arm ships with no committed shell-level test; (b) the library `scripts/exit-witness.c` (first-party C, no dependency beyond the C library, built by `cc -shared -fPIC -O2`; in CI into `$RUNNER_TEMP` and named through `$GITHUB_ENV`, with no `LD_PRELOAD` in that step) — unlike every other member of the class it is harness code that RUNS INSIDE the app's process: it interposes `exit`, `_exit`, `_Exit`, `quick_exit`, `abort`; at load it opens the file named by `ANDROMEDA_PULSE_EXIT_WITNESS_FILE` for append, close-on-exec, removes `LD_PRELOAD` and that variable from the process environment (so the processes the app starts do not load it), and with the file not openable removes the two variables and stays inert; it applies no check of its own to that path (the boot verb sets it); no `pulse-app` code reads either variable (grep over `pulse-app/src` and `crates`: 0 lines) and no file of the shipped binary changed; (c) the harness-written files under `logs/`, never read by the product, are now four named files plus per-boot copies, all riding `logs-boot-Linux` — `harness-settled.json` (eight members, the eighth the closed label `exit_witness`: `unset` · `unreadable` · `loaded` · `exit-call` · `runtime-exit` · `no-record`), `xvfb.log`, `exit-witness.jsonl` (three closed line shapes `loaded` / `end` / `runtime-exit`, each one write of at most 4096 bytes, names reduced to printable ASCII without `"` and `\`, frames as module basename, exported symbol and hex offset, at most 23; it passes no scrubber; read by `harness:settled` bounded at 64 lines of at most 4096 printable-ASCII bytes with one of the three kinds and a `u32` pid, anything else reading `unreadable`, only the label leaving the reader), `boot-series.json`, and `series/boot-{ordinal}/` copies of each series boot's log family with `boot.log`, `harness-settled.json`, `xvfb.log`, `exit-witness.jsonl`; as measured on `ci#38019133294`: eight witness files, 15 lines, each read whole, 0 lines beyond the shape, and all eight `xvfb.log` 0 B; the operator's standing read: every kept witness file read whole, stop on anything beyond its shape; (d) `cargo xtask harness:boot-series --count N` (an xtask verb, never product-consumed): a count outside 1 to 16, a non-Linux host, no data dir or one that already holds `series/`, or no `xvfb-run` on `PATH` reads `cannot-evaluate`; it reads `PATH` by value only to find `xvfb-run`, never printed; it sets `XDG_DATA_HOME` and `XDG_CACHE_HOME` to empty directories inside each boot's own data dir `series/boot-{ordinal}/` and removes `ANDROMEDA_PULSE_PIDFILE` / `_LOGFILE` from each boot's environment; its verdict carries no environment value and no path, per-boot values reduced to labels of at most 48 printable-ASCII bytes and a count; the series' data dirs (`logs`, `run`, `xdg-data`, `xdg-cache`, the product's own `corpus`) are not uploaded, and `run/` is still not uploaded.
    sidecar: >-
      2026-10-10 · chunk `2026-10-10-boot-smoke-s-self-end-named-from-a-run` · §Security Anti-Patterns → Input: the harness-only class gains the exit-witness arm (`ANDROMEDA_PULSE_EXIT_WITNESS_LIB` at `agent-run.sh boot`, fail-closed; the preloaded `scripts/exit-witness.c`, harness code inside the app's process; `ANDROMEDA_PULSE_EXIT_WITNESS_FILE`), the `harness:boot-series` verb and the kept files (`exit-witness.jsonl`, `boot-series.json`, `series/boot-{ordinal}/`); "two harness-written files" retired; classified routine harness evidence by the operator, the pc overseer, 2026-10-10, not the founder's word.
    rationale: >-
      The report's Changes (Symbols / APIs; Schema / config; Coverage of new surfaces) add external-input surfaces security-plan names nowhere — the report's own search reads `EXIT_WITNESS\|LD_PRELOAD\|XDG_DATA_HOME\|XDG_CACHE_HOME` 0 in all masters — while security-plan line 395 still states "two harness-WRITTEN files" with "Both ride the boot job's `logs-boot-Linux` artifact" and a class guard of "trim + `is_file()` existence check with a clean skip on absence". Validation is stated present for the count, the `_LIB` read (trim, regular-file check, fail-closed exit 1), the witness reader and the series verdict; for the library's own read of `ANDROMEDA_PULSE_EXIT_WITNESS_FILE` the report records `validation {n/a — it takes one file path, set by the boot verb}`, and that read happens inside the app's process, which the class's standing reason ("the var never reaches the shipped binary") does not describe as worded. Graded `escalate` by the detector; the classification is the operator's reading and not the founder's word (inputs#I3), and the report's Expected amendments names this site with its two restating sites.
    basis: "scripts/exit-witness.c:63-92 (the load-time open and the environment removal); scripts/agent-run.sh:44-56 (the `_LIB` read and its fail-closed arm); xtask/src/harness_witness.rs:88-135 (the bounded reader)"
  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → the CLI / env var inputs row"
    change: >-
      After the third harness-only reader (`cargo xtask pre-push:linux`) add a fourth harness-only boundary (chunk `2026-10-10-boot-smoke-s-self-end-named-from-a-run`; not product-consumed — no `pulse-app` code reads any of it): `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`, read by `scripts/agent-run.sh boot` alone (sh only), validated by trim + regular-file check — unset or blank spawns the app as before, set and not a regular file reads `boot: exit witness library not found` and exits 1 before the pre-build (fail closed, not a skip), the value never printed; a regular file is loaded into the app's process on the one spawn line (`LD_PRELOAD`), with `ANDROMEDA_PULSE_EXIT_WITNESS_FILE` = `{data dir}/logs/exit-witness.jsonl` set beside it and read by the library alone, which opens it for append, removes both variables from the process environment and checks nothing more of the path; the witness reader of `harness:settled` (`xtask::harness_witness`) takes at most 64 lines of at most 4096 printable-ASCII bytes with one of three kinds and a `u32` pid, anything else reading `unreadable`, never a partial record, and only one closed label (`unset` · `unreadable` · `loaded` · `exit-call` · `runtime-exit` · `no-record`) leaves it as the settle verdict's eighth member; `cargo xtask harness:boot-series --count N` reads `cannot-evaluate` on a count outside 1 to 16, a non-Linux host, no data dir or one already holding `series/`, or no `xvfb-run` on `PATH` (read by value only for that search, never printed), sets `XDG_DATA_HOME` / `XDG_CACHE_HOME` on each of its boots, and its verdict document carries no value of an environment variable and no path (per-boot values as labels of at most 48 printable-ASCII bytes and a count). Point to §Security Anti-Patterns → Input for the class and its classification.
    sidecar: >-
      2026-10-10 · chunk `2026-10-10-boot-smoke-s-self-end-named-from-a-run` · §Input Validation, CLI / env var inputs row: a fourth harness-only boundary registered — the boot verb's fail-closed `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` read, the library's `ANDROMEDA_PULSE_EXIT_WITNESS_FILE`, the bounded witness reader behind `exit_witness`, and `harness:boot-series --count` (1 to 16).
    rationale: >-
      Restating site of the primary: security-plan line 138 enumerates the harness-only readers one by one ("A second harness-only reader … A third harness-only reader …") and stops at `pre-push:linux`, so the roster is short by the surfaces the report's Symbols / APIs and Coverage of new surfaces bullets add (the `_LIB` validation `trim, regular-file check, fail-closed exit 1`; the reader's `64 lines, 4096 bytes, printable ASCII, three kinds, a u32 pid; else unreadable`; the count's `cannot-evaluate` arms). The report's Expected amendments names this row as a restating site.
    basis: "scripts/agent-run.sh:44-56; xtask/src/harness_witness.rs:88-135; xtask/src/harness_series.rs:334-359"
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: "§Threat Model Summary → Attack surface → CLI input (env vars + binary launch) → Trust boundary"
    change: >-
      In the harness-only carve-out enumeration replace "the two harness-written `logs/harness-settled.json` / `logs/xvfb.log` files" with the current set — the harness-written `logs/harness-settled.json`, `logs/xvfb.log`, `logs/exit-witness.jsonl`, `logs/boot-series.json` and the `logs/series/boot-{ordinal}/` per-boot copies — and add the two members of chunk `2026-10-10-boot-smoke-s-self-end-named-from-a-run`: the exit-witness arm of `agent-run.sh boot` (sh only; `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`, trim + regular-file check, fail closed; the named library loaded into the app's process on the spawn line through `LD_PRELOAD`, with `ANDROMEDA_PULSE_EXIT_WITNESS_FILE` read by that library alone and both variables removed from the process environment at load) and the `harness:boot-series` xtask verb's by-value read of `PATH` with `XDG_DATA_HOME` / `XDG_CACHE_HOME` set on each of its boots; qualify "none product-consumed" so it reads true for the new member: no `pulse-app` code reads any of them, and the witness library is the one member that runs inside the app's process (classified routine harness evidence by the operator, the pc overseer, 2026-10-10, not the founder's word; §Security Anti-Patterns → Input).
    sidecar: >-
      2026-10-10 · chunk `2026-10-10-boot-smoke-s-self-end-named-from-a-run` · §Threat Model Summary, CLI input trust boundary: the harness-only carve-out list gains the exit-witness arm, the `harness:boot-series` verb and the kept files; "the two harness-written … files" retired; "none product-consumed" qualified for the library that runs inside the app's process.
    rationale: >-
      Restating site of the primary: security-plan line 85 carries the same count ("the two harness-written `logs/harness-settled.json` / `logs/xvfb.log` files") and closes the enumeration with "none product-consumed", which the report's Changes outgrow — Filesystem locations lists `logs/exit-witness.jsonl`, `logs/boot-series.json` and `logs/series/boot-{ordinal}/` as harness-written, and Symbols / APIs states the library is loaded on the app's spawn line and reads its variable inside that process. The report's Expected amendments names §Threat Model Summary as a restating site (`harness-settled`: security-plan lines 85, 395).
    basis: "scripts/agent-run.sh:44-56; scripts/exit-witness.c:63-92; xtask/src/harness_series.rs:308-310"
    dependent-of: D-security-input
```

## design-system — proposals 0 · entities after decode 0 · nothing stripped

## layout-templates — proposals 0 · entities after decode 0 · nothing stripped

## test-plan — proposals 12 · entities after decode 0 · nothing stripped

```yaml
proposals:
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Pending coverage triggers → `harness-cleanup-verdict-and-boot-spawn-shell-coverage`"
    change: >-
      Append a widening: **Widened 2026-10-10-boot-smoke-s-self-end-named-from-a-run:** the sh `boot` verb's witness arm (it reads `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`, trimmed; unset or blank spawns as before; a regular file runs the one spawn command with `LD_PRELOAD` set to it and `ANDROMEDA_PULSE_EXIT_WITNESS_FILE` set to `{data dir}/logs/exit-witness.jsonl`; set and not a regular file prints `boot: exit witness library not found` and exits 1 before the pre-build; a stale `logs/exit-witness.jsonl` is removed before the spawn) ships with NO committed shell-level test — the set arm's evidence is three local legs and the CI `boot` job, the refusal arm was driven once by hand; `agent-run.ps1` has no witness arm. What is pinned is the Rust side: the witness reader and decision (14 pins) and seven controls on the built library in `xtask/src/harness_witness.rs`, and the `exit_witness` member pin in `harness_ready`. Owed: a harness-level assertion per witness-arm branch (unset, set, refused).
    sidecar: "2026-10-10-boot-smoke-s-self-end-named-from-a-run — §1 trigger `harness-cleanup-verdict-and-boot-spawn-shell-coverage` widened: the sh `boot` witness arm (`ANDROMEDA_PULSE_EXIT_WITNESS_LIB`) has no committed shell-level test; set arm driven by three local legs and CI, refusal arm once by hand."
    rationale: >-
      Report, Coverage of new surfaces: "`ANDROMEDA_PULSE_EXIT_WITNESS_LIB` at `agent-run.sh boot` → … tests {✗ no committed shell-level test; the set arm is driven by the three local legs and the CI job, the refusal arm was driven once by hand}". A new path with no test at the harness tier this trigger row tracks; the plan's Expected amendments name this row (`test-plan.md:134`). The row's last widening stops at chunk 2026-10-09-boot-smoke-s-early-exit-found-and-closed.
    basis: "scripts/agent-run.sh:44-56"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Pending coverage triggers → `harness-cleanup-verdict-and-boot-spawn-shell-coverage`"
    change: >-
      In the same widening, record the series verb's unexercised half: `cargo xtask harness:boot-series` carries 16 unit pins on its pure decisions (`xtask/src/harness_series.rs`), one local leg (two boots, `all-settled`) and one CI run (`self-ended`); its `timed-out` path (a cycle bounded at 1200 s) is pinned as a label and never ran live; and the verdict under-reads a boot that ends before the boot verb reads ready — such a boot runs no `harness:settled`, so it is counted `other` with `verdict` null and `exit_witness` null (as measured on `ci#38019133294`: seven boots ended by themselves, `boot-series.json` reads `ended` 2, `other` 4; the step still failed and each witness file is kept). Owed: the before-ready boot recorded with its exit record and witness label (the closing entry's first repair), and a live reading of the `timed-out` path.
    sidecar: "2026-10-10-boot-smoke-s-self-end-named-from-a-run — §1 trigger row records `harness:boot-series` gaps: `timed-out` path pinned, never run live; a boot ending before ready takes no settle verdict and is counted `other` (`ci#38019133294`: `ended` 2, `other` 4 of seven self-ends)."
    rationale: >-
      Report, Coverage of new surfaces: "`cargo xtask harness:boot-series` → … tests {16 unit pins; one local leg (two boots, `all-settled`); one CI run (`self-ended`); the timed-out path never ran live}"; Harness / gate surface, "The measured defect of the series verdict"; Insufficient fixes: "four of that run's seven self-ends carry `verdict` null and `exit_witness` null, and `boot-series.json` reads `ended` 2, `other` 4", owner "the closing entry minted at this wrap, as the first thing it repairs". Outcome marks the matching acceptance "UNMET in part". The row already carries the same class of note for `harness:settled`'s `not-settled` arm ("pinned and never read live").
    basis: "xtask/src/harness_series.rs:432-755"
  - detector: D-tests-framework
    severity: warning
    section: "§4 Unit Test Strategy → Framework (Rust crates)"
    change: >-
      "One unit binary shells out to a non-Rust runtime" becomes two shell-outs: keep `unit_l4_grammar` (Python 3) and add that since chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run the `xtask` unit binary shells out to the host C compiler — `harness_witness::tests::built_library` (Linux only) builds `scripts/exit-witness.c` with `cc` and runs seven controls on the built library; a missing `cc` FAILS them, never skips them.
    sidecar: "2026-10-10-boot-smoke-s-self-end-named-from-a-run — §4 Framework: a second unit binary shells out of Rust — xtask's `harness_witness::tests::built_library` builds `scripts/exit-witness.c` with `cc` (Linux only; a missing `cc` fails the tests)."
    rationale: >-
      Report, Harness / gate surface → xtask tests: "seven controls on the built library, `mod built_library`, `:383-598`, Linux only, a missing `cc` fails them"; Symbols / APIs: the library is "C, no dependency beyond the C library; built by `cc -shared -fPIC -O2`"; Dev-tool versions: "`cc` was read on the dev host at `GCC 16.2.1 20260810`". The runner is unchanged (`cargo nextest run -p xtask --profile ci …`, `cargo nextest run --workspace --profile ci`, both on-spec); what moved is §4's count of unit binaries that leave Rust, which the section states as exactly one.
    basis: "xtask/src/harness_witness.rs:383-598"
  - detector: D-tests-framework
    severity: warning
    section: "§9 CI Integration → Pipeline structure → Lint + tests (`lint-test` job)"
    change: >-
      Beside "The workspace tests need a Python 3 interpreter on the runner", state that since chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run they also need a C compiler (`cc`) on the runner: the xtask `harness_witness` built-library controls build `scripts/exit-witness.c` with the runner's own `cc` — no setup step; a missing `cc` fails the tests; green in the `lint / test` job of `ci#38019133294`, the runner compiler's version not read from that run.
    sidecar: "2026-10-10-boot-smoke-s-self-end-named-from-a-run — §9 `lint-test` row: the workspace tests also need `cc` on the runner (xtask `harness_witness` built-library controls; green on `ci#38019133294`)."
    rationale: >-
      The restating site of the §4 claim: the `lint-test` row names the one non-Rust tool the workspace tests need on the runner. Report, Coverage of new surfaces: the seven controls are "green on the dev host and in the runner's `lint / test` job"; Dev-tool versions: "the runner's compiler built the library and the controls in `ci#38019133294` and its version was not read from that run"; Harness / gate surface: the `boot` job gained "two added steps and nothing else", so `lint-test` has no setup step for it.
    basis: "xtask/src/harness_witness.rs:383-598"
    dependent-of: D-tests-framework
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 → 5-command implementation (`boot` Command body)"
    change: >-
      Add the witness arm, sh only (since chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run; `scripts/agent-run.ps1` unchanged): the sh verb reads the harness-only `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`, trimmed — unset or blank, the app is spawned as before; a regular file, the one spawn command runs with `LD_PRELOAD` set to it and `ANDROMEDA_PULSE_EXIT_WITNESS_FILE` set to `{data dir}/logs/exit-witness.jsonl`, neither exported, so the pre-build and every other verb run without them; a stale `logs/exit-witness.jsonl` of the same data dir is removed before the spawn; set and not a regular file is refused before the pre-build (Exit code, below). The library is `scripts/exit-witness.c` (built by `cc -shared -fPIC -O2`; it interposes `exit`, `_exit`, `_Exit`, `quick_exit`, `abort`, removes `LD_PRELOAD` and its file variable from the process environment at load, and is inert when the file variable is unset). Neither variable is read by `pulse-app`'s own code. The arm has no committed shell-level test (the §1 `harness-cleanup-verdict-and-boot-spawn-shell-coverage` trigger). The 5-command set is unchanged: no sixth verb.
    sidecar: "2026-10-10-boot-smoke-s-self-end-named-from-a-run — §3 `boot` Command body: sh-only witness arm (`ANDROMEDA_PULSE_EXIT_WITNESS_LIB` → `LD_PRELOAD` + `ANDROMEDA_PULSE_EXIT_WITNESS_FILE` on the spawn line only; stale witness file removed); ps1 unchanged; no sixth verb."
    rationale: >-
      Report, Symbols / APIs: "The boot verb's witness arm, sh only (`scripts/agent-run.sh:44-56, 90, 92-99`) … `scripts/agent-run.ps1` is unchanged (plan step 3). The 5-command set is unchanged: no sixth verb"; Environment variables: both new variables harness-only, "never read by `pulse-app`'s own code". The key file's `boot` Command body (its line 4) describes the spawn and closes "Env vars unchanged: …" with no witness arm, so §3 no longer describes the shipped verb. The report changes neither the status verdict shape nor the app log format, and obs-plan §3's keyed contracts name no harness verb, so the §3 ↔ obs-plan §3 pair holds no disagreement — the drift is test-plan §3's own `boot` body.
    basis: "scripts/agent-run.sh:44-56"
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 → 5-command implementation (`boot` Exit code)"
    change: >-
      "0 on `ready`; non-zero when no `ready` verdict arrives inside the window" gains a second non-zero path: with `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` set and not a regular file the sh verb prints `boot: exit witness library not found` on stderr and exits 1 before the pre-build (chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run; driven once by hand, no committed test — the §1 `harness-cleanup-verdict-and-boot-spawn-shell-coverage` trigger).
    sidecar: "2026-10-10-boot-smoke-s-self-end-named-from-a-run — §3 `boot` Exit code: fail-closed exit 1 (`boot: exit witness library not found`) before the pre-build when the witness library variable names no regular file."
    rationale: >-
      Report, Symbols / APIs: "Set and not a regular file: `boot: exit witness library not found` on stderr, exit 1, before the pre-build"; Deviations: "The missing-library check sits before the pre-build"; Coverage: "the refusal arm was driven once by hand". The key file's Exit code label (its line 6) names the readiness failure as the verb's non-zero path; the plan's Expected amendments name this label.
    basis: "scripts/agent-run.sh:44-56"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 → 5-command implementation (`boot` Readiness signal)"
    change: >-
      Replace "the cause of the app ending by itself shortly after ready on a runner is not named, and stays with the route's P-129 record" with: who ends the app is named from one run — on `ci#38019133294` seven of eight boots ended by themselves (three after `boot: ready`, four before ready), and each of the seven kept witness lines reads `_exit(1)`, `errno` 11, on the main thread, called from a static function of `libgdk-3.so.0` entered from Xlib's `_XIOError` (under `_XReply`, `XGetWindowProperty`, `gdk_x11_screen_supports_net_wm_hint`) — an end that cannot be logged, hence no `app.exit`; why the X connection's read failed is not named, the runner's GTK build was not read, and whether the witness library or the series contributes to the rate is not measured; P-129 is not claimed and the remainder stays with the route's closing entry.
    sidecar: "2026-10-10-boot-smoke-s-self-end-named-from-a-run — §3 `boot` Readiness signal: the self-end's ending call is named from `ci#38019133294` (`_exit(1)`, errno 11, GDK's X IO-error path under `_XIOError`); why the X read failed is not named; P-129 not claimed."
    rationale: >-
      Report, Cross-project / external claims: "seven ended by themselves, one settled"; "Ordinals 1, 2 and 8 ended after `boot: ready` … Ordinals 3, 4, 5 and 7 ended before ready"; the witness lines "`\"call\":\"_exit\",\"code\":1,\"errno\":11`, 23 frames — … `libgdk-3.so.0` +0x76ccc (a static function); `libX11.so.6` `_XIOError`, `_XReply`, `XGetWindowProperty`"; Limits: "the lines do not say why the X connection's read failed"; header: "P-129 is not claimed". The key file's sentence says the cause "is not named" and places the end "shortly after ready"; both halves are overtaken by this run's reading. No other site of test-plan or its key files carries the claim (`P-129`, "not named": this label only).
  - detector: D-tests-obs-harness
    severity: warning
    section: "§9 CI Integration → Pipeline structure → Boot smoke (harness)"
    change: >-
      The job's step sequence becomes: `cargo build --workspace --release --features mcp-server` → `Build the exit witness` (`cc` builds `scripts/exit-witness.c` into `$RUNNER_TEMP` and names it through `$GITHUB_ENV` as `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`; no `LD_PRELOAD` in the step) → the "Boot pulse-app smoke" step (its lines unchanged) → `Boot series (equal source)` (`if: always()`, a plain `run: cargo xtask harness:boot-series --count 7`, no soft-fail key) → `cargo xtask ci-gates` (skipped when the smoke or the series fails) → the `if: always()` upload. State the verb: `cargo xtask harness:boot-series --count N` (1 to 16) boots the release app N more times, ordinals 2 to N+1 (the smoke is ordinal 1), each one cycle (`boot`, `harness:settled`, `status`, `cleanup`; after a failed `boot` the two middle verbs are skipped, `cleanup` always runs) under its own `xvfb-run` on its own data dir `series/boot-{ordinal}/`, a cycle bounded at 1200 s; one pretty-JSON verdict `{verdict, boots, settled, ended, other, per_boot}`, also written to `logs/boot-series.json`; `all-settled` exit 0 · `self-ended` 1 · `not-all-settled` 1 · `cannot-evaluate` 2. So one run of the job reads eight boots and a boot whose settle verdict reads `ended` fails the job. Pins: the four smoke pins unchanged plus three (`pulse-app/tests/quality_gate_workflow.rs:587-657`: the library built before the smoke and named through `$GITHUB_ENV`; the series after the smoke and before `ci-gates` under `if: always()`; no `continue-on-error`, `retry` or `|| true` on the series step). Measured limit: a boot that ends before ready takes no settle verdict and is counted `other`. Reading: `ci#38019133294` red — seven of eight boots ended by themselves, the series step 4 min 27 s.
    sidecar: "2026-10-10-boot-smoke-s-self-end-named-from-a-run — §9 Boot smoke row: the `boot` job gains `Build the exit witness` before the smoke and `Boot series (equal source)` (`cargo xtask harness:boot-series --count 7`, `if: always()`) before `ci-gates`; three new workflow pins; `ci#38019133294` red on seven self-ends of eight boots."
    rationale: >-
      Report, Harness / gate surface: "CI, the `boot` job, two added steps and nothing else … `Build the exit witness` (`ci.yml:375-382`), after the release build and before the smoke … `Boot series (equal source)` (`ci.yml:400-407`), after the smoke and before `ci-gates`, `if: always()`, a plain `run: cargo xtask harness:boot-series --count 7` with no soft-fail key. So one run of the job reads eight boots, and a boot whose settle verdict reads `ended` fails the job … `ci-gates` is skipped when the smoke or the series fails; the upload still runs"; "Three workflow pins (`pulse-app/tests/quality_gate_workflow.rs:587-657`) … The four smoke pins are untouched"; Symbols / APIs for the verb's contract; Spec claims disproved: "the series step ran 4 min 27 s". The row (`test-plan.md:496`) gives the job as build → smoke → `ci-gates` → upload, with no witness build and no series.
    basis: ".github/workflows/ci.yml:400-407"
  - detector: D-tests-obs-harness
    severity: warning
    section: "§9 CI Integration → Pipeline structure → Boot smoke (harness)"
    change: >-
      `harness:settled`'s printed object gains an eighth member: `{verdict, pid, ended, app_exit_record, windows_settled, display, session_bus, exit_witness}`, `exit_witness` one closed label — `unset` (no witness file) · `unreadable` · `loaded` · `exit-call` · `runtime-exit` · `no-record`; `logs/harness-settled.json` holds the same eight; the verdict set, the exit codes and the other seven members are unchanged (chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run).
    sidecar: "2026-10-10-boot-smoke-s-self-end-named-from-a-run — §9 Boot smoke row: `harness:settled`'s object seven members → eight (`exit_witness`, a closed six-label set); verdicts and exit codes unchanged."
    rationale: >-
      Report, Symbols / APIs: "The settle verdict gained an eighth member, `exit_witness` (`settled_payload`, seventh parameter, `xtask/src/harness_ready.rs:359, 369`; read in `run_settled` at `:112-114`) … one closed label from `harness_witness::label` (`xtask/src/harness_witness.rs:40-55`): `unset` (no file) · `unreadable` · `loaded` · `exit-call` · `runtime-exit` · `no-record`. The verdict set, the exit codes and the other seven members are unchanged"; Counts / qualifiers moved: "`harness:settled`'s object: seven members → eight". The row (`test-plan.md:496`) prints the seven-member list; it is the one site in test-plan and its key files that states the member list.
    basis: "xtask/src/harness_ready.rs:359"
  - detector: D-tests-obs-harness
    severity: warning
    section: "§9 CI Integration → Pipeline structure → Boot smoke (harness)"
    change: >-
      "the uploaded `logs-boot-Linux` holds five files" becomes: it still holds the smoke's five (`agent-latest.jsonl.{date}`, `boot.log`, `build.log`, `harness-settled.json`, `xvfb.log`) and now also `exit-witness.jsonl`, `boot-series.json` and `series/boot-{2..8}/` with five files each (four where a boot took no settle verdict) — all harness-written, none read by the product; the series' own data dirs (`series/boot-{ordinal}/`, not under `logs/`) are not uploaded.
    sidecar: "2026-10-10-boot-smoke-s-self-end-named-from-a-run — §9 Boot smoke row: the `logs-boot-Linux` artifact now also holds `exit-witness.jsonl`, `boot-series.json` and `series/boot-{2..8}/`."
    rationale: >-
      Report, Harness / gate surface: "What the job's artifact (`logs-boot-Linux`, the whole `logs/` dir) now holds beyond before: `exit-witness.jsonl`, `boot-series.json`, `series/boot-{2..8}/` with five files each (four where a boot took no settle verdict). It held `agent-latest.jsonl*`, `boot.log`, `build.log`, `harness-settled.json`, `xvfb.log` before and still does"; Filesystem locations: the series' data dirs are "not under `logs/`, not uploaded". The row's "holds five files" is the one `logs-boot` site in test-plan.
    basis: "xtask/src/harness_series.rs:308-310"
  - detector: D-tests-obs-harness
    severity: warning
    section: "§9 CI Integration → Build failure conditions"
    change: >-
      Beside the boot-smoke harness-cycle bullet, add what else now fails the `boot` job: the `Build the exit witness` step failing, and `cargo xtask harness:boot-series --count 7` non-zero — `self-ended` (a boot's settle verdict read `ended`) or `not-all-settled` exit 1, `cannot-evaluate` exit 2; the series runs `if: always()` after the smoke, with no retry and no soft-fail key, and `ci-gates` is skipped when the smoke or the series fails.
    sidecar: "2026-10-10-boot-smoke-s-self-end-named-from-a-run — §9 Build failure conditions: a non-zero `harness:boot-series` (`self-ended` / `not-all-settled` 1, `cannot-evaluate` 2) fails the `boot` job."
    rationale: >-
      The restating site of the Boot smoke row's claim of what the `boot` job runs and fails on (`test-plan.md:509`, named by the plan's Expected amendments). Report, Symbols / APIs: "Verdicts and exits: `all-settled` 0 · `self-ended` 1 … · `not-all-settled` 1 · `cannot-evaluate` 2"; Harness / gate surface: "a boot whose settle verdict reads `ended` fails the job"; the third workflow pin: "the series step holds no `continue-on-error`, `retry` or `|| true`". The bullet lists the smoke cycle's three exits as the job's whole boot-smoke failure condition.
    basis: ".github/workflows/ci.yml:400-407"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§1 Coverage triggers → performance-budget: WebGPU canvas throughput"
    change: >-
      "on a run whose smoke step passes the boot job's line reads `no WebGPU adapter (no_navigator_gpu)` … on a run whose smoke step fails the job skips its `ci-gates` step and prints no such line" becomes: the line is printed only on a run whose smoke step AND `Boot series (equal source)` step both pass; since chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run the job skips `ci-gates`, and prints no such line, when either fails.
    sidecar: "2026-10-10-boot-smoke-s-self-end-named-from-a-run — §1 performance-budget (WebGPU canvas) row: the boot job's `ci-gates` frame line now needs the smoke and the boot series both green; `ci-gates` is skipped when either fails."
    rationale: >-
      A second restating site of the boot job's step order: this row conditions the `ci-gates` frame line on the smoke step alone. Report, Harness / gate surface: the series sits "after the smoke and before `ci-gates`" and "`ci-gates` is skipped when the smoke or the series fails; the upload still runs". The report gives no line for this row; it is the `ci-gates` mention in the §1 Coverage triggers table.
    basis: ".github/workflows/ci.yml:400-407"
    dependent-of: D-tests-obs-harness
```

## obs-plan — proposals 4 · entities after decode 0 · nothing stripped

```yaml
proposals:
  - detector: D-obs-defect-narrative
    severity: warning
    section: "§10 SLO Invariants & Telemetry Budgets → Performance budgets, the WebGPU canvas frame row"
    change: >-
      Replace "the step's order makes that the expected line on every run whose smoke step passes (the settle verdict reads `settled`). On a run whose smoke step fails the job skips its `ci-gates` step and prints no such line" with: since chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run a `Boot series (equal source)` step (`cargo xtask harness:boot-series --count 7`, `if: always()`, no soft-fail key) sits after the smoke and before `ci-gates`, so `frame: cannot-evaluate: 0 samples, no WebGPU adapter (no_navigator_gpu)` is the expected `ci-gates` line only on a run whose smoke step AND series step both pass; `ci-gates` is skipped, and the line not printed, when the smoke or the series fails (a series boot whose settle verdict reads `ended` fails the job). Keep the `ci#37979648967` and `ci#38010977166` readings and the closing log-side sentence (the boot job's log as a witness of the WARN arm on a run that reaches the settle verdict) as they stand.
    sidecar: >-
      §10 frame row: the boot job's `ci-gates` frame line is printed only when the smoke step and the new `Boot series (equal source)` step both pass; was "on every run whose smoke step passes".
    rationale: >-
      The row states a step-order mechanism as current ("the step's order makes that the expected line on every run whose smoke step passes"). The report's Harness / gate surface bullet adds a step between the smoke and `ci-gates` and states "`ci-gates` is skipped when the smoke or the series fails; the upload still runs", and "a boot whose settle verdict reads `ended` fails the job". A run whose smoke passes and whose series reads `self-ended`, `not-all-settled` or `cannot-evaluate` therefore prints no such line, so the row's condition is no longer sufficient. The report records no `ci-gates` reading for `ci#38019133294`, so none is added.
    basis: ".github/workflows/ci.yml:400-407 (report Changes → Harness / gate surface)"
  - detector: D-obs-defect-narrative
    severity: warning
    section: "§10 SLO Invariants & Telemetry Budgets → CI gates, the bullet \"Build fails if a perf-budget arm exceeds its SLO or cannot be read\""
    change: >-
      Replace "the boot-smoke line reads `… no WebGPU adapter (no_navigator_gpu)` on a run whose smoke step passes (as measured on one run, `ci#37979648967`; §10 frame row) and is not printed on a run whose smoke step fails, the job skipping its `ci-gates` step (as measured on `ci#38010977166`)" with: the boot-smoke line reads `… no WebGPU adapter (no_navigator_gpu)` on a run whose smoke step and `Boot series (equal source)` step both pass (the smoke-passing reading: one run, `ci#37979648967`, before the series step existed; §10 frame row) and is not printed on a run where either fails, the job skipping its `ci-gates` step (a failed smoke as measured on `ci#38010977166`; the series step added at chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run).
    sidecar: >-
      §10 CI gates, perf-budget bullet: the boot-smoke frame line is printed when the smoke step and the series step both pass, and not when either fails; was "on a run whose smoke step passes".
    rationale: >-
      Second occurrence of the claim the frame-row proposal retires: this bullet restates that the boot-smoke `ci-gates` line follows from the smoke step passing alone. The report's Harness / gate surface bullet: "`ci-gates` is skipped when the smoke or the series fails".
    basis: ".github/workflows/ci.yml:400-407 (report Changes → Harness / gate surface)"
    dependent-of: D-obs-defect-narrative
  - detector: D-obs-defect-narrative
    severity: warning
    section: "§9 CI Integration → Telemetry artifact handling, the Log file row"
    change: >-
      In the `logs-boot-${{ runner.os }}` parenthetical, replace "the artifact holds five files, as read on `ci#37979648967`: …" with: the artifact held five files as read on `ci#37979648967` (`agent-latest.jsonl.{date}`, `boot.log`, `build.log`, `harness-settled.json`, `xvfb.log`) and still holds them; since chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run it also holds three more harness-written entries the product never reads, as read on `ci#38019133294` (34178 B) — `exit-witness.jsonl` (the preloaded exit witness's lines for the smoke boot, a closed line shape of three kinds `loaded` · `end` · `runtime-exit`, names reduced to printable ASCII, module basenames only), `boot-series.json` (the `cargo xtask harness:boot-series` verdict: closed labels of at most 48 printable-ASCII bytes and counts, no environment value and no path) and `series/boot-{2..8}/` with five files each (that boot's log family, `boot.log`, `harness-settled.json`, `xvfb.log`, `exit-witness.jsonl`; four where a boot took no settle verdict). `harness-settled.json` now holds eight members, the eighth `exit_witness`, one closed label (`unset` · `unreadable` · `loaded` · `exit-call` · `runtime-exit` · `no-record`). The series' own data dirs (`series/boot-{ordinal}/` under the data dir, with `run`, `xdg-data`, `xdg-cache`, `corpus`) are not under `logs/` and are not uploaded, as `run/` is not.
    sidecar: >-
      §9 Log file row: `logs-boot-Linux` additionally holds `exit-witness.jsonl`, `boot-series.json` and `series/boot-{2..8}/` (five files each, four where a boot took no settle verdict); `harness-settled.json` carries an eighth member, `exit_witness`; was "the artifact holds five files".
    rationale: >-
      The row states the artifact's content as current ("holds five files") and describes the settle-verdict file. The report's Harness / gate surface bullet "What the job's artifact (`logs-boot-Linux`, the whole `logs/` dir) now holds beyond before" lists the three new entries and says the earlier five are still held; Symbols / APIs gives the eighth member and its closed label set; Filesystem locations gives the kept-file list and that the series' data dirs are not uploaded. The plan's Expected amendments name this row (`obs-plan.md:499`). It is the file set by which a run's self-end is read, so it is carried under the narrative detector, as the same row was at the 2026-10-09 boot-smoke wrap.
    basis: "obs-plan.md:499; xtask/src/harness_series.rs:308-310 (`is_kept`); xtask/src/harness_witness.rs:40-55 (the label set); report Changes → Harness / gate surface, Symbols / APIs → Filesystem locations"
  - detector: D-obs-defect-narrative
    severity: warning
    section: "§7 Error Capture & Reporting → Error classes captured, the Process-end cause bullet"
    change: >-
      Keep the loggable / unloggable partition as written and add after the unloggable list: (a) a native GDK end is not always the C `exit()` the loggable class names — as measured on `ci#38019133294` (chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run), the Linux CI boot job's self-end is `_exit(1)` with `errno` 11 on the main thread, called from a static function of `libgdk-3.so.0` entered from Xlib's `_XIOError`, reached from `_XReply` under `XGetWindowProperty` under `gdk_x11_screen_supports_net_wm_hint`; all seven `end` lines of that run are equal in every member but `pid` and `tid`; that is an unloggable end, which is why those boots hold no `app.exit` record and no `app.panic.fatal`; (b) "(stderr / `boot.log` at most)" describes the product's own surfaces: when the harness's boot verb runs with `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` naming the library built from `scripts/exit-witness.c` (Linux, `agent-run.sh` only; the CI `boot` job sets it), an interposed `exit`, `_exit`, `_Exit`, `quick_exit` or `abort` also leaves one `end` line (call, code, errno, at most 23 frames of module basename · exported symbol · offset) in the harness-written `logs/exit-witness.jsonl`, read by `cargo xtask harness:settled` as its `exit_witness` label — `pulse-app`'s own code reads neither variable and writes neither file; (c) not measured: why the X connection's read failed, and the runner's own library build (research read the GTK branch head's `gdk_x_io_error`, which calls `_exit(1)`). The end is named, not closed; the cause's owner is the closing entry minted at this chunk's wrap.
    sidecar: >-
      §7 Process-end cause: the CI boot smoke's self-end is recorded as an unloggable `_exit(1)` from GDK's X I/O-error path (`ci#38019133294`, seven equal `end` lines); the harness's exit witness is named as what keeps the ending call for such an end; the cause stays open with its owner.
    rationale: >-
      The report's `Spec claims disproved by measurement` bullet cites this bullet (`obs-plan.md:396`) as holding ("lists `_exit` among the ends that cannot be logged; the measured end is that one"), and the Outcome states "Against obs-plan §7's partition this is an end that cannot be logged (`_exit`), which is why no `app.exit` record exists for it". Nothing is measured false, but two readings of the bullet are now stale as a current account: its loggable class names "a C `exit()`, incl. a native GTK/GDK/WebKitGTK one" with no counter-case, while the one measured native GDK end is `_exit`; and its unloggable class says such an end leaves "stderr / `boot.log` at most", while the job now keeps a line naming the call and its caller (Outcome, first obs criterion: met, on the dev host by the control leg and on the runner by seven `end` lines). The plan's Expected amendments name this bullet ("what a run's witness showed of the end with no `app.exit`"). The limits and the owner are the report's own (Cross-project → Limits of that reading; Decisions → the wrap relay, inputs#I4; "P-129 is not claimed").
    basis: "obs-plan.md:396; scripts/exit-witness.c:124-162 (the five interposed calls); scripts/agent-run.sh:44-56 (the witness arm); xtask/src/harness_ready.rs:112-114 (the label read in `run_settled`); report Changes → Cross-project / external claims → The witness lines"
```

## a11y-plan — proposals 0 · entities after decode 0 · nothing stripped


## Validate — dispositions (before Escalate)

Opening rule: every source coordinate in a `change`, `rationale` or `basis` was checked against the report's bullets
and its last section (a row's range, a span of rows, an added range as printed). 36 of 36 hold; rejected for a
source the report does not carry: 0.

- **architecture 1 to 17 — apply · check 1** (the playbook's "Accurate this-chunk addition"; "Registry over-reach"
  does not govern: §Occupied Resources → xtask CLI surfaces enumerates each xtask verb with its contract, and the
  plan's expected amendments name the row). Check 5: the plan's four architecture entries are each matched
  (xtask CLI surfaces: 1 to 5 · Environment variables: 6 to 12 · Filesystem locations: 13 to 15 · CI/CD approach:
  16). Proposals on one row land as one edit. 6, 7 and 8 (the two witness variables and `LD_PRELOAD`) register the
  crossing the security group escalates: they are applied after that group's resolution and worded by it. 17 (a
  §Stack row for the C library) matches the same rule; the plan the operator approved names the library, and the
  host compiler is already a provisioning item of the pre-push check; shown to the operator beside the escalation.
- **security-plan 1 to 3 — escalate · check 1.** The detector's severity is `escalate`; the playbook's "Boundary
  widening" governs (harness code now runs inside the app's process, loaded on its spawn line, and reads a file
  variable there with no check of its own); "New env var registration" (routine) does not govern, its
  `unit-tested` clause failing for the boot verb's arm, which ships with no committed shell-level test; the
  operator's wrap relay holds that an `escalate` proposal halts for the operator's word (inputs#I4 item 7). One
  `dependent-of` group: it applies or is rejected whole. Check 5: the plan's security entry and its two restating
  sites are matched by the three.
- **test-plan 1 to 12 — apply · check 1** ("Accurate this-chunk addition"; 7 and the two §1 trigger widenings also
  under "Amendment RECORDING a measured OPEN defect … owned by a NAMED route entry"). Check 5: the plan's two
  test-plan entries are matched (§9 Boot smoke and Build failure conditions: 8 to 11 · §3 `boot` and the §1
  trigger: 1, 2, 5, 6). 3, 4, 7 and 12 go beyond the list and rest on the report's Changes.
- **obs-plan 1 to 4 — apply · check 1** (the same two rules). Check 5: the plan's obs entry is matched (§9 Log file
  row: 3 · §7 Process-end cause: 4). 1 and 2 go beyond the list and rest on Harness / gate surface.
- **design-system · layout-templates · a11y-plan — no proposal.**

Check 2: no two proposals edit one section in opposing directions (architecture, test-plan and obs-plan state the
same step order and the same skip of `ci-gates`). Check 3: the report follows the plan, which names and does not
close on the operator's P4 ruling (inputs#I3); the master record's desc ("cause closed; equal source reads the
same") is rewritten to what the chunk did at the flip. Check 4: each absence the proposals rest on ("0 hits in all
masters") is the report's own grep; the cascade sweep re-reads the restating sites. Check 6: the two disproved
plan claims are chunk-artifact claims, recorded in the report (the series' added time; the per-boot label), the
second routed to the closing entry at P5; no master claim was measured false.

## Escalate — resolved with the operator, 2026-10-10

One escalation: the security-plan group (three sites). Put to the operator with three options (apply on the operator's
reading · apply the facts with the classification provisional · hold for the founder). The answer, verbatim:

> "Apply facts, classification provisional" — notes: "Operator (pc overseer): a library that runs inside the app
> process is not mine to class for good. Apply the three sites with the surfaces and their measured behaviour; write
> the classification as PROVISIONAL - the operator reading of 2026-10-10 (routine harness evidence, the app exit
> record), awaiting the founder own word at the epoch boundary - and keep the standing stop that every kept witness
> file is read whole. Carry it in the handoff beside P-129. The architecture Stack row for the C library: apply."
> — the operator, the pc overseer, 2026-10-10, given here.

Applied as answered: the three sites state the surfaces and their measured behaviour, the classification PROVISIONAL
at each, the standing stop kept; architecture 6, 7, 8 worded by it; architecture 17 applied. No playbook rule is
proposed: the class is the never-routine one.

## Apply — what landed

36 of 36 applied: architecture 17 (12 body edits and 1 key-file edit; proposals on one row landed as one edit),
security-plan 3 (4 body edits), test-plan 12 (7 body edits and 3 key-file edits), obs-plan 4 (4 body edits). Two
further wording fixes followed the sweep (`architecture.md:218`, `obs-plan.md:499`; `cascade-dispositions.md`). Six
sidecar entries. Open escalations: 0.
