# Leaf recompute — the eight comparison returns (verbatim)

Each return is a read-only comparer's hand-back, taken from its transcript by script. The orchestrator verified
findings and wrote the edits; `cascade-dispositions.md` in this directory names each result. A root-prefixed
path in a return is respelled repo-relative here and nothing else is changed.


## security-plan → rules/security.md · docs/security-summary.md · CLAUDE.md warnings

finding blocks in full: 21 · chars 29818

~~~
## Findings: security-plan master vs its three leaves (read-only; nothing edited)

Master line numbers refer to `.andromeda/security-plan.md`. Most findings sit in the three copies of one long "product-binary path env vars" bullet (`security.md:17`, `security-summary.md:43`, `CLAUDE.md:44`), which the console-engine and harness chunks left behind.

### A. `.claude/rules/security.md` (generated body, lines 1-94)

```
- leaf: .claude/rules/security.md:17            class: stale
  leaf-text: «`scripts/agent-run.sh boot` reads `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` (sh only; trimmed; unset or blank spawns the app as before; a regular file is loaded onto the app's one spawn command through `LD_PRELOAD`,»
  master: .andromeda/security-plan.md:396 «is read and validated by bare `scripts/agent-run.sh boot`, the window program's boot, alone among the script's paths»
  now: The script no longer has one spawn command: "since chunk `2026-10-10-agent-harness-drives-the-console-engine` the script holds a second spawn line, the console engine's under `boot engine`, which never reads the variable, sets no preload and neither removes nor creates the witness file"; a regular file goes "on the window app's spawn command".
  replace-with: «bare `scripts/agent-run.sh boot`, the window program's boot, alone among the script's paths reads `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` (sh only; since chunk `2026-10-10-agent-harness-drives-the-console-engine` the script holds a second spawn line, the console engine's under `boot engine`, which never reads the variable, sets no preload and neither removes nor creates the witness file; trimmed; unset or blank spawns the app as before; a regular file is loaded onto the window app's spawn command through `LD_PRELOAD`,»
```

```
- leaf: .claude/rules/security.md:17            class: omission
  leaf-text: «whose six members carry no environment value and no path.»
  master: .andromeda/security-plan.md:396 «**The same class since chunk `2026-10-10-agent-harness-drives-the-console-engine` (read as routine harness evidence, not a boundary widening — the pc overseer's own reading as operator»
  now: The master's harness-only class has a further dated member, items (a)-(e), which the leaf's member-by-member enumeration lacks entirely (also master :85 and :138 "A fifth harness-only boundary, the console-engine harness").
  replace-with: «whose six members carry no environment value and no path. The same class since chunk `2026-10-10-agent-harness-drives-the-console-engine` (read as routine harness evidence, not a boundary widening — the pc overseer's own reading as operator, 2026-10-10; not the founder's word): `scripts/agent-run.sh` `boot` / `status` take one optional word from a closed set (none is the window app, `engine` the console engine, any other word prints usage and exits 2 before a directory is made; sh only), and `boot engine` spawns `target/release/andromeda-pulse-engine run` by path under the same waiting wrapper, writing the spawn record and the exit record as for the window app; `cargo xtask harness:status` / `harness:ready` take `--program window|console` (a clap value enum), their `program` member one of three closed labels (`window` · `console` · `unknown`), a run of the other program or of none reading `wrong-program`; `cargo xtask check:engine-log` (no argument; reads the exit record through `read_ended`; no line of the check holds a record's text or a path) and `cargo xtask harness:engine-settled --timeout-seconds N` (`cannot-evaluate` below 20) resolve their paths as `harness:status` does; `cargo xtask harness:engine-cycle [--data-dir DIR] [--grpc-port N] [--http-port N]` (Linux alone) reads the system `HOME` and `PATH` by value, only to build its children's environment, and prints or writes neither; every child gets a CLEARED environment plus exactly `HOME`, `PATH`, `ANDROMEDA_PULSE_DATA_DIR`, `ANDROMEDA_PULSE_OTLP_GRPC_PORT`, `ANDROMEDA_PULSE_OTLP_HTTP_PORT` (24317 / 24318 by default; 4317 and 4318 refused, `shared-port`), `ANDROMEDA_PULSE_RETENTION_SECONDS` = 60, `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` (made per run, never printed or written) and `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` only when its own environment holds it — the session bus address, the runtime dir and the display variables are outside the set (pinned by set equality); it writes only its per-cycle data dir (default `target/engine-cycle/{UTC second}/`, git-ignored), and its eight-member verdict carries no environment value and no path, while its stderr passes through the boot verb's `boot: ready (PID=…, data_dir=…)` line, which prints the data dir; the `boot` job's `Console engine cycle` step uploads that dir's whole `logs/` as `logs-engine-Linux` — the engine's log family plus `boot.log` and `build.log`, the last holding the runner's checkout paths and passing no scrubber; the dev example `inject_demo` (never shipped) reads `ANDROMEDA_PULSE_OTLP_GRPC_PORT` to pick its port, the host fixed at `127.0.0.1`, a value that is no port refused with exit 2 and never repeated.»
```

```
- leaf: .claude/rules/security.md:17            class: stale
  leaf-text: «read only by `boot` and by the xtask verbs `harness:status`, `harness:ready` and `harness:settled` through `read_ended`'s bounded grammar (one line, ≤ 48 printable ASCII); the product binary never reads them.»
  master: .andromeda/security-plan.md:396 «(a) the boot-recorder state files gain a writer and two readers»
  now: The reader set is larger and there is a second writer: "`agent-run.sh boot engine` … writes the spawn record and the exit record as for the window app; `cargo xtask check:engine-log` reads the exit record through `read_ended`, and it and `cargo xtask harness:engine-settled` resolve their paths as `harness:status` does"; and "since chunk `2026-10-10-boot-smoke-s-self-end-closed` `cargo xtask harness:boot-series` reads both as well".
  replace-with: «read only by the harness — `boot` and the xtask verbs `harness:status`, `harness:ready`, `harness:settled`, `harness:boot-series` (since chunk `2026-10-10-boot-smoke-s-self-end-closed`, for a boot that took no settle verdict) and, since chunk `2026-10-10-agent-harness-drives-the-console-engine`, `check:engine-log` and `harness:engine-settled` — the exit record through `read_ended`'s bounded grammar (one line, ≤ 48 printable ASCII); since that chunk `agent-run.sh boot engine` writes both records for the console engine as for the window app; the product binary never reads them.»
```

```
- leaf: .claude/rules/security.md:17            class: omission
  leaf-text: «is inert without its file, and applies no check of its own to the path; no `pulse-app` code reads either variable.»
  master: .andromeda/security-plan.md:396 «One more harness member reads the variable since chunk `2026-10-10-agent-harness-drives-the-console-engine`: `cargo xtask harness:engine-cycle`»
  now: `harness:engine-cycle` "reads it by value from its own environment, only to pass it on into its children's cleared environment when present, prints and writes nothing of it, reports `logs/exit-witness.jsonl` of the cycle's data dir as the closed label `witness_file` = `absent` | `present`, and reads `fail` on `present`"; "The library still runs inside the window app's process only, never the console engine's."
  replace-with: «is inert without its file, and applies no check of its own to the path; no `pulse-app` code reads either variable. It runs inside the window app's process only, never the console engine's. One more harness member reads the variable since chunk `2026-10-10-agent-harness-drives-the-console-engine`: `cargo xtask harness:engine-cycle` reads it by value only to pass it on into its children's cleared environment when present, prints and writes nothing of it, reports the cycle data dir's `logs/exit-witness.jsonl` as the closed label `witness_file` = `absent` | `present`, and reads `fail` on `present`.»
```

```
- leaf: .claude/rules/security.md:17            class: stale
  leaf-text: «the console program `andromeda-pulse-engine` reads none of them and builds no inference runner»
  master: .andromeda/security-plan.md:398 «reads none of them and no `ANDROMEDA_PULSE_L4_ALLOW_ROOT`, constructs no `LlamaCliInference` and emits no `interpretation.model.*` record»
  now: The master does not say "no inference runner": it says no `LlamaCliInference`, and at :138 that under a truthy `ANDROMEDA_PULSE_L4_DETERMINISTIC` the console program "seats the canned runner; unset seats NOTHING".
  replace-with: «the console program `andromeda-pulse-engine` reads none of them and no `ANDROMEDA_PULSE_L4_ALLOW_ROOT`, constructs no `LlamaCliInference` and emits no `interpretation.model.*` record, so the residual below does not reach it»
```

```
- leaf: .claude/rules/security.md:28            class: contradicted
  leaf-text: «currently rely on manual review and runtime IPC rejection (`xtask capability-drift` chunk #27 verifies TauRPC ↔ capability JSON sync only — it does NOT detect permission widening on the 3 named capabilities).»
  master: .andromeda/security-plan.md:197 «additionally asserts the staged `pulse-app/capabilities/*.json` against the in-code `staged_gate::EXPECTED_GRANTS` pin (`{identifier, windows, permissions}` × 6 files … set equality in BOTH directions: a REVOKED and an ADDED grant each red»
  now: The master states no "static analysis test gap" anywhere. It says `capability-drift` diffs TauRPC procedures against `EXPECTED_PROCEDURES` (there is no TauRPC-to-capability-JSON sync, and no per-procedure entries exist) and that an added grant in any pinned capability file is red: ":414 «a left-REVOKED or silently-ADDED grant is also caught mechanically by the staged-grants assertion — silent runtime rejection is no longer the sole stated enforcement»".
  replace-with: «are held mechanically, not by manual review alone: `xtask capability-drift` asserts the staged `pulse-app/capabilities/*.json` against `staged_gate::EXPECTED_GRANTS` (`{identifier, windows, permissions}` × 6 files, set equality in both directions — a REVOKED and an ADDED grant each red, an unpinned capability file red), so a silently widened grant is caught in CI, not only by Tauri's silent runtime rejection.»
  note: The whole bullet (heading "Static analysis test gap (documented; not yet implemented)", the "next `/andromeda-tests` re-run" trigger) should go or be rewritten. Outside my master, `.andromeda/test-plan.md:122` and `.claude/rules/testing.md:83` both mark this trigger LANDED as `cargo xtask capability-widening-check` (chunk #77); the orchestrator should source that half from test-plan. `pulse-app/capabilities/` holds exactly six files (clipboard, default, notification, plugin-fs, tray, updater), matching the master's "× 6 files".
```

```
- leaf: .claude/rules/security.md:32            class: stale
  leaf-text: «Per security-plan.md §Security Decisions Log 2026-05-11 —»
  master: .andromeda/security-plan.md:12 «Amendment history — the externalized Security Decisions Log plus superseded guidance … lives in the sibling `security-plan-amendments.md`»
  now: The plan body has no §Security Decisions Log; the log is externalized to the amendments sidecar. The fact itself still stands in the body at :133 and :199 («NOT a `wasmtime::Config` method»).
  replace-with: «Per security-plan §Input Validation / §API Security (the 2026-05-11 decision is in the externalized Security Decisions Log, `security-plan-amendments.md`) —»
```

```
- leaf: .claude/rules/security.md:5            class: omission
  leaf-text: «(tier=Minimal — local-first Tauri 2 desktop app, loopback-only OTLP, in-memory DuckDB)»
  master: .andromeda/security-plan.md:14 «and, since chunk `2026-10-10-console-engine-entry-point`, a console program (`andromeda-pulse-engine`) over the same engine boot that names no window framework»
  now: The plan covers two engine programs (three product binaries with the MCP sidecar, :100), not the Tauri app alone.
  replace-with: «(tier=Minimal — local-first Tauri 2 desktop app plus, since chunk `2026-10-10-console-engine-entry-point`, the console program `andromeda-pulse-engine` over the same engine boot; loopback-only OTLP, in-memory DuckDB)»
```

One-line item, low confidence:
- `.claude/rules/security.md:36` — stale (value) — «without an 8 MB size cap»: the master (:142, :400) says only "a size cap" for plugin-returned Arrow IPC and names no figure; the 8 MB in the master belongs to the OTLP body and decode limits. Suggested: «without a size cap».

### B. `.claude/docs/security-summary.md` (whole file, 64 lines)

```
- leaf: .claude/docs/security-summary.md:43            class: stale
  leaf-text: «`agent-run.sh boot` reads `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` (sh only; trim + regular-file check, fail closed with exit 1 before the pre-build) and loads the named library onto the app's one spawn command through `LD_PRELOAD`; the library (`scripts/exit-witness.c`) is the one harness member that runs inside the app's process»
  master: .andromeda/security-plan.md:85 «bare `agent-run.sh boot`, the window program's boot, reads `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` … and loads the named library into the window app's process on its spawn line through `LD_PRELOAD` (… the script has a second spawn line, the console engine's under `boot engine`, which never reads the variable and carries no preload»
  now: Two spawn lines exist; only the bare (window) boot reads the variable; `harness:engine-cycle` "passes the variable on by value to its children only when its own environment holds it"; the library runs "inside the window app's process only, never the console engine's" (:396).
  replace-with: «bare `agent-run.sh boot`, the window program's boot, reads `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` (sh only; trim + regular-file check, fail closed with exit 1 before the pre-build) and loads the named library onto the window app's spawn line through `LD_PRELOAD` (the script's second spawn line, the console engine's under `boot engine`, never reads the variable and carries no preload; `cargo xtask harness:engine-cycle` passes the variable on by value to its children only when its own environment holds it); the library (`scripts/exit-witness.c`) is the one harness member that runs inside the app's process — the window app's only, never the console engine's»
```

```
- leaf: .claude/docs/security-summary.md:43            class: omission
  leaf-text: «prints or writes neither, and writes only its per-run area `target/pre-push/run/` and the report twin, which hold no environment value and no path.»
  master: .andromeda/security-plan.md:85 «since chunk `2026-10-10-agent-harness-drives-the-console-engine`, the console-engine harness (the `engine` word of `agent-run.sh boot` / `status`, sh only, a closed word set; the `--program window|console` flag of `harness:status` / `harness:ready`; the `harness:engine-cycle` xtask verb's …»
  now: The harness-only class has a newer member the summary's enumeration lacks (detail at :396 (a)-(e)).
  replace-with: «prints or writes neither, and writes only its per-run area `target/pre-push/run/` and the report twin, which hold no environment value and no path. The same class since chunk `2026-10-10-agent-harness-drives-the-console-engine` (the operator's reading, not the founder's word): the console-engine harness — the `engine` word of `agent-run.sh boot` / `status` (sh only, a closed word set; any other word is usage, exit 2), which spawns `andromeda-pulse-engine run` under the same waiting wrapper and writes the same spawn and exit records; the `--program window|console` flag of `harness:status` / `harness:ready` (`program` one of `window` · `console` · `unknown`; the other program or none reads `wrong-program`); `check:engine-log` and `harness:engine-settled`; `harness:engine-cycle`, which reads `HOME` / `PATH` by value, gives every child a cleared, pinned environment (ports 24317 / 24318 by default, 4317 and 4318 refused; a per-run corpus passphrase never printed or written) and writes only its per-cycle data dir under `target/engine-cycle/`, its verdict holding no environment value and no path; the `boot` job's `logs-engine-Linux` upload of that dir's `logs/` (`build.log` holds the runner's checkout paths, passing no scrubber); and the dev example `inject_demo`'s read of `ANDROMEDA_PULSE_OTLP_GRPC_PORT` — none product-consumed.»
```

```
- leaf: .claude/docs/security-summary.md:43            class: stale
  leaf-text: «read only by `boot` and the xtask verbs `harness:status` / `harness:ready` / `harness:settled` through a bounded one-line grammar, never by the product)»
  master: .andromeda/security-plan.md:396 «(a) the boot-recorder state files gain a writer and two readers»
  now: `boot engine` also writes the records; `check:engine-log` reads the exit record through `read_ended`, `harness:engine-settled` resolves the same paths, and `harness:boot-series` "reads both as well".
  replace-with: «read only by `boot` and the xtask verbs `harness:status` / `harness:ready` / `harness:settled` / `harness:boot-series` and, since 2026-10-10, `check:engine-log` / `harness:engine-settled` through a bounded one-line grammar, written for the console engine too under `boot engine`, never read by the product)»
```

```
- leaf: .claude/docs/security-summary.md:43            class: omission
  leaf-text: «**Narrowed exception (guard landed 2026-08-26):** three PRODUCT-consumed vars — `ANDROMEDA_PULSE_MODEL_PATH` / `_LLAMA_CUDA_BIN_PATH` / `_LLAMA_CPU_BIN_PATH` — reject traversal»
  master: .andromeda/security-plan.md:398 «read by a SHIPPED binary — the window app `pulse-app` alone among the three product binaries: the console program `andromeda-pulse-engine` reads none of them and no `ANDROMEDA_PULSE_L4_ALLOW_ROOT`»
  now: The guard and its residual belong to the window app only: "so the residual consequence stated here does not reach it" (the console program).
  replace-with: «**Narrowed exception (guard landed 2026-08-26):** three PRODUCT-consumed vars — `ANDROMEDA_PULSE_MODEL_PATH` / `_LLAMA_CUDA_BIN_PATH` / `_LLAMA_CPU_BIN_PATH`, read by the window app `pulse-app` alone (the console program `andromeda-pulse-engine` reads none of them and no `ANDROMEDA_PULSE_L4_ALLOW_ROOT`, so the residual does not reach it) — reject traversal»
```

```
- leaf: .claude/docs/security-summary.md:26            class: omission
  leaf-text: «8. **CLI / env vars** — `serde` + `TryFrom<u16>` validation;»
  master: .andromeda/security-plan.md:84 «Since chunk `2026-10-10-console-engine-entry-point` also the command line of the console program `andromeda-pulse-engine` (`run` or `version`).»
  now: The CLI vector now has a real command line: :85 "the console program's command line is a closed two-word grammar — any other command line prints usage on stderr and exits 2, creating nothing"; :139 "No word takes a value, a path or an address, and no `stop` command exists."
  replace-with: «8. **CLI / env vars** — `serde` + `TryFrom<u16>` validation; since chunk `2026-10-10-console-engine-entry-point` also the command line of the console program `andromeda-pulse-engine`, a closed two-word grammar (`run` | `version`; any other command line prints usage on stderr and exits 2, creating nothing; no word takes a value, a path or an address);»
```

```
- leaf: .claude/docs/security-summary.md:26            class: stale
  leaf-text: «set to `1` as `main()`'s first statement when absent, before any thread exists;»
  master: .andromeda/security-plan.md:138 «the first statement of the window app's `main()` in `pulse-app/src/main.rs`, before any thread exists; the console program's `main` only hands its arguments to `pulse_app::console::main`, and its files name no render posture»
  now: There are two product `main`s; only the window app's sets the variable (:85 "the window app alone writes the process environment").
  replace-with: «set to `1` as the first statement of the window app's `main()` when absent, before any thread exists — the window app alone writes the process environment, and the console program's files name no render posture;»
```

```
- leaf: .claude/docs/security-summary.md:36            class: retired
  leaf-text: «(canonical self-observation surface per obs-plan §3 `2026-05-02 — Phase 3.5 pivot to tracing-only self-observation`; legacy `opentelemetry-stdout` references in security-plan.md §Data Protection / §Bootstrap phases / §Logging & Monitoring bodies are obsolete-but-equivalent»  [the passage runs from this parenthesis to the end of line 36, through «(body annotations at security-plan.md lines 154/239/330).»]
  master: .andromeda/security-plan.md:12 «This body holds ONLY current truth. Amendment history — … superseded guidance (e.g., the pre-pivot `opentelemetry-stdout` self-observation references) — lives in the sibling `security-plan-amendments.md`»
  now: The plan body holds no legacy `opentelemetry-stdout` reference and no `DEPRECATED (2026-05-08)` blockquote (0 hits for `DEPRECATED`; the only `opentelemetry-stdout` mention is line 12 itself). Lines 154 / 239 / 330 are now the outbound-TLS bullet, the `cargo-geiger` bullet and a blank line. The phase bullet (:259) reads "the `tracing-subscriber` JSON formatter writing to `~/.andromeda-pulse/logs/agent-latest.jsonl` (tracing-only — no OTel SDK)".
  replace-with: «6. `logging-redaction-wire` — the `tracing-subscriber` JSON formatter writing to `~/.andromeda-pulse/logs/agent-latest.jsonl` (tracing-only — no OTel SDK). Snapshot/clipboard/MCP-tool-response paths apply attribute-value redaction for incidentally captured secrets. (The pre-pivot `opentelemetry-stdout` references are no longer in the plan body; superseded guidance lives in `security-plan-amendments.md`.)»  [replaces the whole of line 36]
```

```
- leaf: .claude/docs/security-summary.md:37            class: omission
  leaf-text: «`Cargo.lock` integrity + `xtask capability-drift`»
  master: .andromeda/security-plan.md:260 «plus, since chunk `2026-08-30-npm-advisory-coverage`, `cargo xtask check:npm-supply-chain` in the `supply-chain` job (SHA-pinned setup-node, no `npm ci`)»
  now: The `dep-security-ci-gate` phase includes the npm gate; :219 adds that the build fails on its exit 1 or 2, "so the fail-condition roster is no longer Rust-boundary-only".
  replace-with: «`Cargo.lock` integrity + `cargo xtask check:npm-supply-chain` (the `supply-chain` job, SHA-pinned setup-node, no `npm ci`, since 2026-08-30) + `xtask capability-drift`»
```

```
- leaf: .claude/docs/security-summary.md:3            class: omission
  leaf-text: «local-first Tauri 2 desktop app, loopback-only OTLP, in-memory DuckDB, no user accounts, no compliance triggers.»
  master: .andromeda/security-plan.md:14 «and, since chunk `2026-10-10-console-engine-entry-point`, a console program (`andromeda-pulse-engine`) over the same engine boot that names no window framework»
  now: Same as `security.md:5`; :100 lists three product binaries (`pulse-app`, `andromeda-pulse-engine`, `andromeda-pulse-mcp`).
  replace-with: «local-first Tauri 2 desktop app plus, since chunk `2026-10-10-console-engine-entry-point`, the console program `andromeda-pulse-engine` over the same engine boot (no window framework); loopback-only OTLP, in-memory DuckDB, no user accounts, no compliance triggers.»
```

One-line items:
- `.claude/docs/security-summary.md:54` — stale (path) — heading «## Open residual risks (Decisions Log)»: the Decisions Log is externalized to `security-plan-amendments.md` (master :12); the three risks still stand in the body (:166, :356-359, :285).
- `.claude/docs/security-summary.md:10-16` — omission, low — the Data classifications list has no entry for the master's «user-content (persistent incident corpus …)» type (:45-48: `corpus.db`, 5 tables, cell-level AES-256-GCM, scrubbed projections). Line 13 covers only the key, and line 11 reads as if telemetry-derived data persists only in snapshots.

### C. `CLAUDE.md` lines 40-51 (security-plan-sourced statements only)

```
- leaf: CLAUDE.md:44            class: stale
  leaf-text: «`agent-run.sh boot` reads `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` (trim + regular-file check, fail closed) and loads `scripts/exit-witness.c`'s built library onto the app's one spawn command through `LD_PRELOAD`, the one harness member that runs inside the app's process»
  master: .andromeda/security-plan.md:85 «bare `agent-run.sh boot`, the window program's boot, reads `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` (sh only; trim + regular-file check, fail closed) and loads the named library into the window app's process on its spawn line through `LD_PRELOAD`»
  now: "the script has a second spawn line, the console engine's under `boot engine`, which never reads the variable and carries no preload"; the library runs inside the window app's process only (:396).
  replace-with: «bare `agent-run.sh boot`, the window program's boot, reads `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` (trim + regular-file check, fail closed) and loads `scripts/exit-witness.c`'s built library onto the window app's spawn line through `LD_PRELOAD` (the second spawn line, the console engine's under `boot engine`, never reads the variable and carries no preload), the one harness member that runs inside the app's process — the window app's only»
```

```
- leaf: CLAUDE.md:44            class: omission
  leaf-text: «and `pre-push:linux`'s by-value read of `HOME` / `PATH` (no value printed or written).»
  master: .andromeda/security-plan.md:85 «and, since chunk `2026-10-10-agent-harness-drives-the-console-engine`, the console-engine harness (the `engine` word of `agent-run.sh boot` / `status`, sh only, a closed word set; the `--program window|console` flag of `harness:status` / `harness:ready`; the `harness:engine-cycle` xtask verb's by-value read of `HOME` / `PATH` with its cleared child environment and its per-cycle data dir under `target/engine-cycle/`; the dev example `inject_demo`'s read of `ANDROMEDA_PULSE_OTLP_GRPC_PORT`), none product-consumed»
  now: The carve-out enumeration this bullet mirrors has one more member.
  replace-with: «and `pre-push:linux`'s by-value read of `HOME` / `PATH` (no value printed or written), and since 2026-10-10 the console-engine harness: the `engine` word of `agent-run.sh boot` / `status` (sh only, a closed word set), the `--program window|console` flag of `harness:status` / `harness:ready`, `harness:engine-cycle`'s by-value read of `HOME` / `PATH` with its cleared child environment and its per-cycle data dir under `target/engine-cycle/`, and the dev example `inject_demo`'s read of `ANDROMEDA_PULSE_OTLP_GRPC_PORT` — none product-consumed.»
```

```
- leaf: CLAUDE.md:44            class: omission
  leaf-text: «NARROWED EXCEPTION (guard landed 2026-08-26): the PRODUCT-consumed `ANDROMEDA_PULSE_MODEL_PATH` / `_LLAMA_{CUDA,CPU}_BIN_PATH` reject traversal»
  master: .andromeda/security-plan.md:138 «all read by the shipped window app `pulse-app` alone (the console program reads none of them)»
  now: Of the three product binaries only the window app reads these vars or `ANDROMEDA_PULSE_L4_ALLOW_ROOT`; the residual "does not reach" the console program (:398).
  replace-with: «NARROWED EXCEPTION (guard landed 2026-08-26): the PRODUCT-consumed `ANDROMEDA_PULSE_MODEL_PATH` / `_LLAMA_{CUDA,CPU}_BIN_PATH` — read by the window app `pulse-app` alone; the console program `andromeda-pulse-engine` reads none of them and no `ANDROMEDA_PULSE_L4_ALLOW_ROOT` — reject traversal»
```

```
- leaf: CLAUDE.md:47            class: stale
  leaf-text: «full plugin paths»
  master: .andromeda/security-plan.md:457 «NEVER log a full product-consumed filesystem path — basename of the canonicalized path only. Covers the plugin dir, the three L4 model/binary path vars, the `ANDROMEDA_PULSE_L4_ALLOW_ROOT` root, and the corpus-key lock dir / lock-file path … **The rule is now CATEGORICAL**»
  now: The ban covers every product-consumed filesystem path (the plugin dir is one of five named), categorical since chunk `2026-08-30-diagnostics-un-muting-harness-truth-sweep`.
  replace-with: «full product-consumed filesystem paths (basename only — plugin dir, L4 model/binary paths, the L4 allow root, the corpus-key lock dir, the L4 grammar temp file)»
```

### Closing

- `.claude/rules/security.md`: generated body read whole (94 lines; lines 17 and 62 confirmed complete against their tails) · findings 8 (+1 one-line)
- `.claude/docs/security-summary.md`: generated body read whole (64 lines; line 43 confirmed complete) · findings 9 (+2 one-line)
- `CLAUDE.md`: lines 40-51 read whole (line 44 confirmed complete) · findings 4
- `.andromeda/security-plan.md`: read whole · 475 lines · long lines read by window: no. The Read tool returned every long line unclipped; I confirmed this by matching the tail 160 characters of lines 85, 138, 140, 223, 276, 396, 398, 432 and 464 against an `awk substr` extract. No `substr` windowing was needed. It has no key files.
- Nothing was left unread.

Facts the master holds that fall outside the omission rule because no existing leaf bullet hosts them (not reported as findings):
- `.claude/rules/security.md` §"Input validation at every boundary" has no bullet for the console program's command-line boundary (master :126, :139: closed `run` | `version` grammar, exit 2, creating nothing).
- The same section has no bullet for the two readers of `ANDROMEDA_PULSE_L4_DETERMINISTIC` (master :138: the console program with the gate unset seats nothing, `interpretation = none`, `reason = deterministic_gate_unset`).
- Neither leaf carries the `incident_events` NO-SCRUB boundary (master :444) alongside the three NO-SCRUB bullets `security.md` does carry (lines 55-57).

The line-28 finding rests partly on `.andromeda/test-plan.md:122`, which is outside my assigned master; I checked it only by grep.
~~~


## test-plan → docs/tests-summary.md · rules/verification-harness.md

finding blocks in full: 18 · chars 29554

~~~
Leaf recompute: test-plan master + 7 key files vs `.claude/docs/tests-summary.md` and `.claude/rules/verification-harness.md` (lines 1-112). Read-only; nothing edited.

Headline: neither generated body contains the word `engine` at all (grep count 0 in both). Every console-engine / `--program` / `wrong-program` / engine-cycle fact from the two 2026-10-10 chunks is missing from both leaves. 17 finding blocks follow, most consequential first, then one-liners, then two places where the master (not the leaf) is behind the code.

Paths: masters under `.andromeda/`, key files under `.../.andromeda/registries/contracts/test-plan/`, leaves under `.claude/`. Every `leaf-text` below was checked to occur exactly once in its file.

## Findings

```
- leaf: .claude/rules/verification-harness.md:24            class: stale
  leaf-text: «(`xtask/src/harness_status.rs`): one verdict JSON on stdout `{verdict, pid, ended, log_file_basename, last_write_age_seconds, stale_after_seconds}`»
  master: registries/contracts/test-plan/5-command-implementation.md:17 «`cargo xtask harness:status [--program window|console]`» and :30 «The object has seven members since that chunk, pinned by set equality»
  now: The verb takes `--program window|console`; the sh `status` takes one optional word (none → `--program window`, `engine` → `--program console`, any other word usage exit 2), ps1 takes no word and passes no flag. The object has seven members, with `program`: `window` | `console` | `unknown`.
  replace-with: «(`xtask/src/harness_status.rs`; `[--program window|console]` — the sh `status` passes `--program window` bare and `--program console` under the word `engine`, any other word is usage, exit 2; `agent-run.ps1 status` takes no word and passes no flag): one verdict JSON on stdout, seven members since chunk 2026-10-10-agent-harness-drives-the-console-engine, `{verdict, pid, ended, program, log_file_basename, last_write_age_seconds, stale_after_seconds}` — `program` is the label of the LAST `app.boot.engine` record of the log family (`window` | `console` | `unknown`)»
```

```
- leaf: .claude/rules/verification-harness.md:24            class: stale
  leaf-text: «arms `running-healthy`(0) / `stale`(1) / `not-running`(1) / `cannot-evaluate`(2), derived out-of-process from the PID file + that pid's liveness + log-family mtime (`STALE_AFTER_SECONDS` 60)»
  master: 5-command-implementation.md:21 «"verdict": "running-healthy" | "stale" | "wrong-program" | "not-running" | "cannot-evaluate"» and :30 «1 `wrong-program` (since chunk 2026-10-10-agent-harness-drives-the-console-engine: with `--program`, a `running-healthy` or `stale` run whose log family records another program, or none)»
  now: Five arms, exits 0/1/1/1/2. `not-running` and `cannot-evaluate` stand whatever program was asked; with no flag the verdict is as before plus the `program` member.
  replace-with: «arms `running-healthy`(0) / `stale`(1) / `wrong-program`(1) / `not-running`(1) / `cannot-evaluate`(2), derived out-of-process from the PID file + that pid's liveness + log-family mtime (`STALE_AFTER_SECONDS` 60) + the family's last `app.boot.engine` record — `wrong-program` is, with `--program`, a `running-healthy` or `stale` run whose log family records the other program, or none; `not-running` and `cannot-evaluate` stand whatever program was asked»
```

```
- leaf: .claude/rules/verification-harness.md:22            class: stale
  leaf-text: «one JSON object `{verdict: ready | not-ready | ended | cannot-evaluate, pid, ended, otlp_grpc, otlp_http}`, exit 0 / 1 / 1 / 2»
  master: 5-command-implementation.md:5 «`{verdict: ready | not-ready | wrong-program | ended | cannot-evaluate, pid, ended, program, otlp_grpc, otlp_http}`, six members since chunk 2026-10-10-agent-harness-drives-the-console-engine (pinned by set equality)»
  now: Six members, five verdicts; «exit 0 `ready` · 1 `not-ready`, `wrong-program` or `ended` · 2 `cannot-evaluate`». The sh script asks `--program window` under bare `boot` and `--program console` under `boot engine`; a dead pid is `ended` whatever was asked.
  replace-with: «one JSON object, six members since chunk 2026-10-10-agent-harness-drives-the-console-engine, `{verdict: ready | not-ready | wrong-program | ended | cannot-evaluate, pid, ended, program, otlp_grpc, otlp_http}`, exit 0 `ready` · 1 `not-ready`, `wrong-program` or `ended` · 2 `cannot-evaluate` — the sh script asks `--program window` under bare `boot` and `--program console` under `boot engine`; `wrong-program` is the status verdict for the program asked (the log family's last `app.boot.engine` record names the other program, or none) and holds whatever the receivers answer; a dead pid is `ended` whatever was asked»
```

```
- leaf: .claude/rules/verification-harness.md:22            class: omission
  leaf-text: «`ANDROMEDA_PULSE_DATA_DIR=$TMPDIR/agent-run-$$` + `RUST_LOG=debug` — the `cargo run` wrapper is GONE;»
  master: 5-command-implementation.md:4 «Since chunk 2026-10-10-agent-harness-drives-the-console-engine the sh `boot` takes one optional word — none is the window app, as above; `engine` is the console engine; any other word is usage, exit 2, before a directory is made»
  now: `boot engine` pre-builds `cargo build --bin andromeda-pulse-engine --release` and xtask, spawns `target/release/andromeda-pulse-engine run` BY PATH under the same waiting wrapper, and polls `cargo xtask harness:ready --program console`. The word is sh only and has no committed shell-level test.
  replace-with: «`ANDROMEDA_PULSE_DATA_DIR=$TMPDIR/agent-run-$$` + `RUST_LOG=debug` — the `cargo run` wrapper is GONE; since chunk 2026-10-10-agent-harness-drives-the-console-engine the sh `boot` takes one optional word: none is the window app; `engine` is the console engine (pre-builds `cargo build --bin andromeda-pulse-engine --release` and xtask, spawns `target/release/andromeda-pulse-engine run` BY PATH under the same waiting wrapper with the same spawn and exit records, polls `cargo xtask harness:ready --program console`); any other word is usage, exit 2, before a directory is made — sh only (`agent-run.ps1` is unchanged), no committed shell-level test, read end to end by `cargo xtask harness:engine-cycle`;»
```

```
- leaf: .claude/rules/verification-harness.md:22            class: stale
  leaf-text: «the sh verb alone carries the exit-witness arm since chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run — it reads `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`, trimmed: unset or blank spawns as before; a regular file runs the one spawn command with `LD_PRELOAD` set to it»
  master: 5-command-implementation.md:4 «The exit-witness arm, sh only and the window program's alone (…; `boot engine` never reads the variable, sets no preload, and neither removes nor creates the witness file)»
  now: The script has two spawn commands. The preload goes on «the window app's spawn command» only; test-plan.md:135 says it was «the script's one spawn command until chunk 2026-10-10-agent-harness-drives-the-console-engine added the console engine's, which carries no preload».
  replace-with: «the sh verb alone carries the exit-witness arm since chunk 2026-10-10-boot-smoke-s-self-end-named-from-a-run, and it is the window program's alone (`boot engine` never reads the variable, sets no preload, and neither removes nor creates the witness file) — bare `boot` reads `ANDROMEDA_PULSE_EXIT_WITNESS_LIB`, trimmed: unset or blank spawns as before; a regular file runs the window app's spawn command with `LD_PRELOAD` set to it»
```

```
- leaf: .claude/rules/verification-harness.md:66            class: contradicted
  leaf-text: «No CI step runs it since chunk 2026-10-10-no-gate-stands-while-reading-nothing (`cargo xtask ci-gates` left the gap check; `cargo xtask perf:load-profiles`, which no workflow runs, is its one caller); its CI enforcement stands unmet and is carried on the working route.»
  master: test-plan.md:501 «since chunk 2026-10-10-agent-harness-drives-the-console-engine the gap check over the console engine's log is the cycle step's, next, and none is made over the window's log»
  now: The `boot` job's `Console engine cycle` step runs `check:engine-log`, whose `heartbeat-gap` arm is «a gap over 45 000 ms between consecutive `ingest.tick` / `buffer.tick` / `connection.tick` records». So the gap check is CI-enforced for a console engine's log and unmet only for the window's.
  replace-with: «No CI step runs that script since chunk 2026-10-10-no-gate-stands-while-reading-nothing (`cargo xtask ci-gates` left the gap check; `cargo xtask perf:load-profiles`, which no workflow runs, is its one caller). Since chunk 2026-10-10-agent-harness-drives-the-console-engine the gap check is CI-enforced over a console engine's log: the `boot` job's `Console engine cycle` step runs `cargo xtask check:engine-log`, whose `heartbeat-gap` arm fails on a gap over 45 000 ms between consecutive `ingest.tick` / `buffer.tick` / `connection.tick` records; none is made over the window's log.»
  note: the script-caller clause and "carried on the working route" are not test-plan statements (test-plan and its keys never name `heartbeat-gap-check`; obs-plan.md does, once). I kept the caller clause as it stood and dropped the working-route clause; the obs-plan comparer should confirm what remains carried for the window's log.
```

```
- leaf: .claude/rules/verification-harness.md:85            class: omission
  leaf-text: «It binds the resolved OTLP ports, one boot at a time: on the dev host move them off 4317 / 4318 first.»
  master: per-chunk-gate-discipline.md:54 «**Engine cycle gate form (runtime, console engine).** One whole run of the console engine under the harness is read by `cargo xtask harness:engine-cycle [--data-dir DIR] [--grpc-port N] [--http-port N]`»
  now: Three new xtask verbs, one of them a gating step of the same CI `boot` job whose other verbs this list carries (`harness:settled`, `harness:boot-series`). The list has no entry for any of them.
  replace-with: «It binds the resolved OTLP ports, one boot at a time: on the dev host move them off 4317 / 4318 first.
- `xtask harness:engine-cycle [--data-dir DIR] [--grpc-port N] [--http-port N]` (added 2026-10-10, `xtask/src/engine_cycle.rs`) — one whole run of the console engine under the harness: the CI `boot` job's `Console engine cycle` step (`if: always()`, after `ci-gates`, `--data-dir "$RUNNER_TEMP/andromeda-pulse-engine-data"`, no soft-fail key) and a dev-host gate form; not a member of the standard gate set. Defaults: a fresh `target/engine-cycle/{UTC second}` and ports 24317 / 24318. Refused before anything starts (`cannot-evaluate`, exit 2) on `not-linux` · `shared-port` (4317 or 4318 in either flag) · `data-dir-holds-a-log-family` · `home-unset` · `path-unset` · `data-dir-unusable` · `injector-build-failed`. Order: the injector build → `bash scripts/agent-run.sh boot engine` → `inject_demo --sustained --error-pct=0` by path → `cargo xtask harness:engine-settled` → the injector's end → `agent-run.sh status engine` → `agent-run.sh cleanup` → `cargo xtask check:engine-log` (after a failed boot the settle and status verbs are skipped; cleanup and the check always run), every child under a cleared environment with no session bus, runtime dir or display variable. One JSON object, eight members (`verdict`: `pass` 0 · `fail` 1 · `cannot-evaluate` 2; `boot`, `settled`, `status`, `cleanup`, `check`; `error_records`; `witness_file`). The gate reading: exit 0 with `"verdict": "pass"`, `"error_records": 0`, `"witness_file": "absent"`, `engine-log: PASS` and `cleanup: clean`.
- `xtask harness:engine-settled [--timeout-seconds N]` (default 60, below 20 `cannot-evaluate`) — holds the run until the log holds two records of each of `ingest.tick`, `buffer.tick`, `connection.tick` and one non-zero `metric.buffer.memory_bytes` sample; `settled` 0 · `ended` 1 · `wrong-program` 1 · `not-settled` 1 · `cannot-evaluate` 2.
- `xtask check:engine-log` — seven arms, `family` · `program` · `panic` · `heartbeat-gap` · `progress` · `process-end` · `budget`; exit 0 · 1 · 2, a FAIL on any arm outranking a cannot-evaluate; no arm reads PASS over an input it cannot grade. The `budget` arm grades the memory arm as required and prints `engine-log: budget frame and snapshot not graded (a console engine has no producer for either)`; `process-end` reads `run/andromeda-pulse.exit`. Full contracts: architecture §Occupied Resources → xtask CLI surfaces.»
  note: this adds bullets to the leaf's own verb list rather than text inside one bullet; the orchestrator decides whether it fits the omission rule. The `budget` and `process-end` sentences come from test-plan.md:101 and pid-file.md:5.
```

```
- leaf: .claude/docs/tests-summary.md:146            class: retired
  leaf-text: «**Living-artifact tooling re-run discipline gap** (wrap-session Phase 5 currently allows skipping tooling rerun on webview-only chunks;»
  master: test-plan.md:123 «`living-artifact-tooling-rerun-coverage` | **RETIRED 2026-08-26 — its subject no longer exists.**»
  now: Both living-doc artifacts (`.andromeda/context/api-surface.md`, `dependency-tree.md`) «were DELETED at commit `5a83771` (2026-06-28 …), and no `context/` directory exists at any path today»; the code-graph `tree.db` replaced them. Leaf lines 146-148 still state the trigger as a live MUST for `/andromeda-wrap-session` Phase 5.
  replace-with (for the whole of lines 146-148): «**Living-artifact tooling re-run discipline — RETIRED 2026-08-26, its subject no longer exists.** `living-artifact-tooling-rerun-coverage` required wrap-session Phase 5 to run the Tooling command of each `.andromeda/context/{artifact}.md`; both artifacts (`api-surface.md`, `dependency-tree.md`) were deleted at commit `5a83771` (2026-06-28) and no `context/` directory exists. The code-graph (`.andromeda/cache/{plane}/tree.db`, derived, gitignored, rebuilt on demand) replaced them and has no LIVING block to reconcile; the lesson survives in the code-graph refresh being unconditional at wrap Setup. The row is kept in test-plan §1 for audit.»
```

```
- leaf: .claude/docs/tests-summary.md:14            class: stale
  leaf-text: «`cargo xtask harness:status` — real-process verdict JSON `{verdict, pid, ended, log_file_basename, last_write_age_seconds, stale_after_seconds}`»
  master: test-plan.md:67 «`status` (`cargo xtask harness:status [--program window|console]`, the sh verb taking the same `engine` word — real-process verdict JSON `{verdict, pid, ended, program, log_file_basename, last_write_age_seconds, stale_after_seconds}`»
  now: Seven members with `program`; the verb takes `--program`; the sh `status` takes the word `engine`.
  replace-with: «`cargo xtask harness:status [--program window|console]` (the sh verb takes the word `engine` → `--program console`, none → `--program window`; ps1 takes no word and passes no flag) — real-process verdict JSON of seven members `{verdict, pid, ended, program, log_file_basename, last_write_age_seconds, stale_after_seconds}`»
```

```
- leaf: .claude/docs/tests-summary.md:14            class: stale
  leaf-text: «exits 0/1/1/2 from PID file + pid liveness (a dead pid is `not-running`, since 2026-09-29) + log-family mtime»
  master: test-plan.md:67 «from PID file + that pid's liveness + log-family mtime + the family's last `app.boot.engine` record — a dead pid is `not-running`, a run of the other program or one with no boot record is `wrong-program` — exits 0/1/1/1/2»
  now: Five verdicts, exits 0/1/1/1/2, with the `app.boot.engine` record as a fourth input.
  replace-with: «exits 0/1/1/1/2 (`running-healthy` / `stale` / `wrong-program` / `not-running` / `cannot-evaluate`) from PID file + pid liveness (a dead pid is `not-running`, since 2026-09-29) + log-family mtime + the family's last `app.boot.engine` record (a run of the other program, or one with no boot record, is `wrong-program`, since chunk 2026-10-10-agent-harness-drives-the-console-engine)»
```

```
- leaf: .claude/docs/tests-summary.md:12            class: omission
  leaf-text: «then spawn `target/release/pulse-app[.exe]` BY PATH (no `cargo run` wrapper;»
  master: test-plan.md:67 «`boot` (the window app `pulse-app`, or with the word `engine` — sh only, since chunk 2026-10-10-agent-harness-drives-the-console-engine — the console engine `andromeda-pulse-engine run`»
  now: `boot` starts either program; the row describes `pulse-app` only. Details as in 5-command-implementation.md:4.
  replace-with: «then spawn `target/release/pulse-app[.exe]` BY PATH — or, with the word `engine` (sh only, since chunk 2026-10-10-agent-harness-drives-the-console-engine; any other word is usage, exit 2), pre-build `cargo build --bin andromeda-pulse-engine --release` + xtask and spawn `target/release/andromeda-pulse-engine run` under the same waiting wrapper, polling `harness:ready --program console` (no `cargo run` wrapper;»
```

```
- leaf: .claude/docs/tests-summary.md:12            class: stale
  leaf-text: «exit 0 `ready` · 1 `not-ready` / `ended` · 2 `cannot-evaluate`»
  master: 5-command-implementation.md:5 «exit 0 `ready` · 1 `not-ready`, `wrong-program` or `ended` · 2 `cannot-evaluate`»
  now: `wrong-program` is a third exit-1 verdict of `harness:ready`; the object has six members with `program`.
  replace-with: «exit 0 `ready` · 1 `not-ready` / `wrong-program` / `ended` · 2 `cannot-evaluate`»
```

```
- leaf: .claude/docs/tests-summary.md:86            class: stale
  leaf-text: «each of its six uploads fails its step when it finds no file (since 2026-10-10;»
  master: test-plan.md:518 «each of the seven uploads (the seventh, `logs-engine-${{ runner.os }}`, since chunk 2026-10-10-agent-harness-drives-the-console-engine) carries `if-no-files-found: error`, the first six since chunk 2026-10-10-no-ci-step-reads-nothing»
  now: Seven uploads (ci.yml itself holds seven `upload-artifact` steps).
  replace-with: «each of its seven uploads fails its step when it finds no file (the first six since chunk 2026-10-10-no-ci-step-reads-nothing, the seventh, `logs-engine-${{ runner.os }}`, since chunk 2026-10-10-agent-harness-drives-the-console-engine;»
```

```
- leaf: .claude/docs/tests-summary.md:90            class: omission
  leaf-text: «`Boot series (equal source)` (`cargo xtask harness:boot-series --count 7`, `if: always()`) → `ci-gates`.»
  master: test-plan.md:501 «→ `Console engine cycle` (since that chunk: `if: always()`, a plain `run: cargo xtask harness:engine-cycle --data-dir "$RUNNER_TEMP/andromeda-pulse-engine-data"`, no soft-fail key» and :514 «the job also fails when `cargo xtask harness:engine-cycle` is non-zero»
  now: The job's chain no longer ends at `ci-gates`: the cycle step follows it, then the uploads `logs-boot-…` and `logs-engine-…`. The cycle runs whatever the smoke, the series and `ci-gates` returned, and fails the job on `fail` exit 1 or `cannot-evaluate` exit 2.
  replace-with: «`Boot series (equal source)` (`cargo xtask harness:boot-series --count 7`, `if: always()`) → `ci-gates` → `Console engine cycle` (`cargo xtask harness:engine-cycle --data-dir "$RUNNER_TEMP/andromeda-pulse-engine-data"`, `if: always()`, no soft-fail key, a data dir of its own; since chunk 2026-10-10-agent-harness-drives-the-console-engine) → the uploads `logs-boot-…` and `logs-engine-…`. The cycle fails the job when non-zero — `fail` exit 1 (any verb exit other than 0 or 2, an ERROR record in the engine's log family, or a witness file) or `cannot-evaluate` exit 2; its `check:engine-log` grades the engine's log on seven arms, the `heartbeat-gap` arm (over 45 000 ms between consecutive `ingest.tick` / `buffer.tick` / `connection.tick` records) among them, and no gap check is made over the window's log.»
```

```
- leaf: .claude/docs/tests-summary.md:139            class: stale
  leaf-text: «chunks touching `pulse-app/src/main.rs`, `crates/ui-bridge/src/`, `pulse-app/src/observability.rs` (boot-time obs init),»
  master: per-chunk-gate-discipline.md:46 «`pulse-app/src/main.rs`, `pulse-app/src/engine_boot.rs` (since chunk 2026-10-10-console-engine-entry-point the process start, `init_process`, and the whole engine composition, `start`, that both programs boot through, moved out of `main.rs`), `crates/ui-bridge/src/`»
  now: The boot/setup path list has six members; `pulse-app/src/engine_boot.rs` is one. per-chunk-gate-discipline.md:52 adds: «A chunk touching `pulse-app/src/console.rs` or `pulse-app/src/bin/andromeda-pulse-engine.rs` takes this test as its runtime gate; the window smoke boots `pulse-app` alone».
  replace-with: «chunks touching `pulse-app/src/main.rs`, `pulse-app/src/engine_boot.rs` (since chunk 2026-10-10-console-engine-entry-point the process start `init_process` and the whole engine composition `start`, which both programs boot through, live there), `crates/ui-bridge/src/`, `pulse-app/src/observability.rs` (boot-time obs init),»
  also add, after the leaf sentence ending «— test-plan §3.» in the same bullet: «A chunk touching `pulse-app/src/console.rs` or `pulse-app/src/bin/andromeda-pulse-engine.rs` takes the spawned-program test `pulse-app/tests/integration_console_engine.rs` as its runtime gate; the window smoke boots `pulse-app` alone. Since chunk 2026-10-10-agent-harness-drives-the-console-engine `boot engine` (sh only) is the same direct-binary form over `target/release/andromeda-pulse-engine run`.»
```

```
- leaf: .claude/rules/verification-harness.md:103            class: stale
  leaf-text: «Both expose identical 5 commands with identical exit semantics.»
  master: 5-command-implementation.md:4 «The word is sh only (`agent-run.ps1` is unchanged, so the two scripts no longer take the same words)»
  now: Same five verbs, different words: sh `boot` / `status` take an optional word with a usage exit 2 arm; ps1 takes none and passes no `--program` flag (:17). The exit-witness arm and its exit-1 refusal are sh only (:6).
  replace-with: «Both expose the same 5 commands, but they no longer take the same words (since chunk 2026-10-10-agent-harness-drives-the-console-engine): sh `boot` / `status` take one optional word (`engine` → the console engine; any other word → usage, exit 2), `agent-run.ps1` takes none and passes no `--program` flag; the exit-witness arm is sh only.»
```

```
- leaf: .claude/rules/verification-harness.md:59            class: omission
  leaf-text: «`cleanup` terminates the pid the FILE holds (canonical) across both pid spaces — the wrapper-pid defect is CLOSED (chunk 2026-08-30-agent-harness-teardown-truth).»
  master: registries/contracts/test-plan/pid-file.md:8 «BOTH product programs call `write_pid_file` — the window app `pulse-app` and the console program `andromeda-pulse-engine run` — at the same path, so the file holds the pid of whichever program started last on that data dir»
  now: The Lifecycle bullet reads as one app owning the file. The master adds that `cleanup` «still terminates whatever pid the file holds, so what keeps it on the right program is the data dir, one per run».
  replace-with: «`cleanup` terminates the pid the FILE holds (canonical) across both pid spaces — the wrapper-pid defect is CLOSED (chunk 2026-08-30-agent-harness-teardown-truth). Since chunk 2026-10-10-console-engine-entry-point BOTH product programs call `write_pid_file` (the shared boot's, through `engine_boot::init_process`) at the same path, so the file holds the pid of whichever program started last on that data dir; `status` and `boot`'s readiness poll ask for a program and read `wrong-program` when the log family's last `app.boot.engine` record names the other one, or none; `cleanup` still terminates whatever pid the file holds, so what keeps it on the right program is the data dir, one per run.»
```

```
- leaf: .claude/docs/tests-summary.md:118            class: retired
  leaf-text: «Body sites in test-plan.md §1 Coverage triggers (lines 48 + 255) + §Anti-Patterns Project-specific bullet (line 866) are annotated with `**[DEPRECATED 2026-05-08]**` first-column prefixes / `> **DEPRECATED (2026-05-08)**` indented blockquote (content preserved verbatim for audit trail).»
  master: test-plan.md:668 «(Previously a "NEVER self-dial / negative test must assert" rule; deprecation history in test-plan-amendments.md.)»
  now: test-plan.md is 671 lines, so lines 255 and 866 do not address those sites, and it holds 0 occurrences of `DEPRECATED`. The three sites (:48 entity row, :98 trigger row, :668 Project-specific bullet) are rewritten as `by-construction-satisfied`, with the history moved to test-plan-amendments.md.
  replace-with: «The test-plan body sites — the §1 entity row `Self-observation loop prevention`, the §1 trigger row `No self-OTLP dialing` and the §11 Project-specific bullet — now state it as `by-construction-satisfied` (no test generated, no negative test required); the deprecation history is in test-plan-amendments.md.»
```

## Lesser items, one line each

- `.claude/docs/tests-summary.md:34` — omission — the PID file sentence reads as one app; pid-file.md:8 says both programs write `write_pid_file` at the same path and the file holds whichever started last on that data dir.
- `.claude/docs/tests-summary.md:102-107` — omission — the Process-end witness form gained two arms over the production process start: children of `pulse-app/tests/integration_engine_boot.rs` call `engine_boot::init_process` and prove they ran by `app.boot.tracing.init`, `app.boot.pid` and a pid file holding their own pid (per-chunk-gate-discipline.md:50).
- `.claude/docs/tests-summary.md:93-107` — omission, no existing bullet — the master's two new forms, Spawned-program witness (per-chunk-gate-discipline.md:52) and Engine cycle gate (:54), have no home in "Scenario legs vs gates". The leaf's "FOUR scenario legs … all dev-host-only" stays true as written, since the engine cycle is a gate form and CI-wired.
- `.claude/docs/tests-summary.md:87-89` — omission, no existing bullet — the leaf lists `node`, Python 3, `cc` and `libX11.so.6` as test-time requirements but not `bash` and `awk` on PATH (test-plan.md:495, since chunk 2026-10-10-no-gate-stands-while-reading-nothing; a missing tool fails, never skips).
- `.claude/rules/verification-harness.md:62` — not judged — "15s for ingest/buffer/viz/plugins" is an obs-plan statement; test-plan.md:62 says a console engine's log holds `ingest.tick` / `buffer.tick`, never `viz.tick` / `plugins.tick`. For the obs-plan comparer.

## Leaf differs from master, but the code agrees with the leaf

These are not leaf edits toward the master; the master side looks behind.

1. **Status endpoint shape.** `.claude/rules/verification-harness.md:30-45` shows six subsystems (adds `viz`, `plugins`) with `last_tick_at` on each, plus an agent-read on `last_tick_at`. The key file `status-endpoint-shape.md:5-24` shows four subsystems and no `last_tick_at`, and `tests-summary.md:19-31` mirrors the key file. The code (`crates/ui-bridge/src/health.rs:134`, `:150-155`) has six subsystems and `last_tick_at`. `last_tick_at` appears in no `.andromeda/*.md` file and no key file under `registries/contracts/`. So the key file and `tests-summary.md` are behind the code; `verification-harness.md` matches it.
2. **Boot data-dir name.** Three values are in play. The script uses `$TMP_BASE/agent-run-$$` (`scripts/agent-run.sh:31`), and `verification-harness.md:22` and `:69` agree with it. The master's boot body says `$TMPDIR/andromeda-test-$$` (5-command-implementation.md:4). `tests-summary.md:12` (boot row) says `$TMPDIR/test-$$`, which is the master's per-test isolation value (test-data-bootstrap.md:7, correctly used at `tests-summary.md:67`) and matches neither the master's boot body nor the script. `tests-summary.md:12` is stale either way; which value to write depends on whether the master is corrected first.

## Closing

- `.claude/docs/tests-summary.md`: generated body read whole (150 lines) · findings 9 blocks (+4 one-liners)
- `.claude/rules/verification-harness.md`: generated body read whole (112 lines; `## Session Additions` at 113 not read for findings) · findings 9 blocks (+1 one-liner)

- `.andromeda/test-plan.md`: read whole · 671 lines · long lines read by window: no — read whole through the Read tool, which returned them unclipped; line tails cross-checked with `awk` for 135, 145, 226, 232, 233, 234, 289, 495, 501. Its §3 (line 189) is only a pointer to the key files.
- `5-command-implementation.md`: read whole · 42 lines · long lines (4, 5) read whole, tails cross-checked
- `status-endpoint-shape.md`: read whole · 24 lines · no long lines
- `log-format.md`: read whole · 17 lines · no long lines
- `pid-file.md`: read whole · 9 lines · no long lines
- `test-data-bootstrap.md`: read whole · 8 lines · no long lines
- `bootstrap-phases.md`: read whole · 10 lines · no long lines
- `per-chunk-gate-discipline.md`: read whole · 56 lines · long lines (22, 24, 32, 34, 48, 50, 54) read whole through the Read tool, tails of 32 and 54 cross-checked
- Leaf long lines (`tests-summary.md` 98, 143; `verification-harness.md` 22, 85, 97, 98, 99) read whole; tails of 98, 143, 22, 97, 99 cross-checked.

Nothing was left unread. Not judged, because their source is not test-plan or its keys (0 hits there): `tests-summary.md:59` ("live injector `crates/ingest/examples/load_profiles.rs`; no bench suite exists"), `verification-harness.md:52` (required `fields.service.*`), `:62-65` (tick intervals), `:87-88` (`xtask deny-bans`, `xtask audit`), `:102` (`set -euo pipefail`). These belong to the obs-plan / architecture comparers.
~~~


## test-plan → rules/testing.md

finding blocks in full: 10 · chars 15854

~~~
Leaf recompute: `.claude/rules/testing.md` (lines 1-109) against `.andromeda/test-plan.md` and its seven key files under `.andromeda/registries/contracts/test-plan/`.

Short names below: `testing.md` = the leaf, `test-plan.md` = the master, `{key}.md` = a key file in the contracts directory.

The leaf's generated body has no harness-verb bullet, so none of the `--program` / member-count / `wrong-program` / new-verb / seventh-upload facts is misstated there. Nine findings follow, then three one-liners.

## Findings (most consequential first)

```
- leaf: testing.md:98-99            class: retired
  leaf-text: «  - `living-artifact-tooling-rerun-coverage: unconditional reconcile at every wrap` — `/andromeda-wrap-session` Phase 5 living-artifacts reconcile MUST execute the recorded Tooling command from each `.andromeda/context/{artifact}.md` METADATA at every wrap regardless of chunk scope.»
  master: test-plan.md:123 «`living-artifact-tooling-rerun-coverage` | **RETIRED 2026-08-26 — its subject no longer exists.**»
  now: The trigger is retired. "Both living-doc artifacts — `.andromeda/context/api-surface.md` and `dependency-tree.md` — were DELETED at commit `5a83771` (2026-06-28 …), and no `context/` directory exists at any path today"; the code-graph (`.andromeda/cache/{plane}/tree.db`) replaced them and "has no LIVING block to reconcile". The leaf still states it as a live MUST.
  replace-with (replaces the header line 98 and the whole sub-bullet line 99; line 100, the amendment pointer, can stay): «- **RETIRED 2026-08-26 — Living-artifact tooling re-run discipline** (its subject no longer exists):
  - `living-artifact-tooling-rerun-coverage` — RETIRED, kept for the audit trail. It required wrap-session Phase 5 to run the Tooling command from each `.andromeda/context/{artifact}.md` METADATA at every wrap (skip-on-webview-only forbidden). Both living-doc artifacts (`.andromeda/context/api-surface.md`, `dependency-tree.md`) were deleted at commit `5a83771` (2026-06-28) and no `context/` directory exists; the code-graph (`.andromeda/cache/{plane}/tree.db`, derived, gitignored, rebuilt on demand by /andromeda-phase + /andromeda-wrap-session) replaced them and has no LIVING block to reconcile. The lesson survives in the code-graph refresh being unconditional at wrap Setup (test-plan §1 Pending coverage triggers).»
```

```
- leaf: testing.md:91            class: stale (member set)
  leaf-text: «chunks touching `pulse-app/src/main.rs`, `crates/ui-bridge/src/`, `pulse-app/src/observability.rs` (boot-time subscriber/allowlist init, before the OTLP bind),»
  master: per-chunk-gate-discipline.md:46 «`pulse-app/src/main.rs`, `pulse-app/src/engine_boot.rs` (since chunk 2026-10-10-console-engine-entry-point the process start, `init_process`, and the whole engine composition, `start`, that both programs boot through, moved out of `main.rs`)»
  now: The boot/setup path set that triggers the boot-smoke gate has six members; `pulse-app/src/engine_boot.rs` is one of them. The leaf lists five, so a chunk touching only `engine_boot.rs` reads as needing no smoke.
  replace-with: «chunks touching `pulse-app/src/main.rs`, `pulse-app/src/engine_boot.rs` (since chunk 2026-10-10-console-engine-entry-point the process start `init_process` and the whole engine composition `start`, which both programs boot through, moved out of `main.rs`), `crates/ui-bridge/src/`, `pulse-app/src/observability.rs` (boot-time subscriber/allowlist init, before the OTLP bind),»
```

```
- leaf: testing.md:91            class: omission
  leaf-text: «the direct-binary smoke variant is the accepted alternative — see test-plan §3 "Direct-binary smoke variant".»
  master: per-chunk-gate-discipline.md:52 «A chunk touching `pulse-app/src/console.rs` or `pulse-app/src/bin/andromeda-pulse-engine.rs` takes this test as its runtime gate; the window smoke boots `pulse-app` alone.»
  now: The window smoke does not boot the console program. Its runtime gate is the spawned-program test `pulse-app/tests/integration_console_engine.rs`. Line 48 adds that since chunk 2026-10-10-agent-harness-drives-the-console-engine "`boot engine`, sh only, is the same form over `target/release/andromeda-pulse-engine run`".
  replace-with: «the direct-binary smoke variant is the accepted alternative — see test-plan §3 "Direct-binary smoke variant" (the harness `boot` verb is that form over `target/release/pulse-app`; `boot engine`, sh only, is the same form over `target/release/andromeda-pulse-engine run`). The window smoke boots `pulse-app` alone: a chunk touching `pulse-app/src/console.rs` or `pulse-app/src/bin/andromeda-pulse-engine.rs` takes the spawned-program test `pulse-app/tests/integration_console_engine.rs` as its runtime gate (test-plan §3 "Spawned-program witness form").»
```

```
- leaf: testing.md:64            class: retired
  leaf-text: «poll `health` TauRPC (`subsystems.buffer.rows_ingested >= N`), Channel event subscription, or PID-file existence check.»
  master: test-plan.md:625 «NEVER use `sleep(N)` for synchronization — wait for explicit signal via `health` status polling or Channel event subscription»
  now: The master names no PID-file existence check as a sync signal anywhere (0 hits for "existence" in the master and all seven key files). Readiness is "Poll `cargo xtask harness:ready` … until its verdict is `ready`: the … status verdict … reads `running-healthy` AND a TCP connection is accepted on `127.0.0.1` at each port" (5-command-implementation.md:5). pid-file.md:8 says both programs write the same path, "so the file holds the pid of whichever program started last on that data dir". per-chunk-gate-discipline.md:52 gives the spawned-program form: "readiness read from both ports accepting, and records awaited by target under a bound, never a sleep for a count".
  replace-with: «poll `health` TauRPC (`subsystems.buffer.rows_ingested >= N`), Channel event subscription, or — for a booted product process — the `cargo xtask harness:ready` verdict for the program asked (status `running-healthy` AND a TCP connection accepted on both resolved OTLP ports); a spawned program is read ready from both ports accepting, its records awaited by target under a bound. The pid file is not a readiness signal: both programs write the same path.»
```

```
- leaf: testing.md:68            class: stale
  leaf-text (a): «Body sites in test-plan.md (§1 trigger rows + §Anti-Patterns bullet) are annotated with `**[DEPRECATED 2026-05-08]**` table-row prefixes / `> **DEPRECATED (2026-05-08)**` blockquotes (content preserved for audit trail).»
  leaf-text (b): «body annotations at test-plan.md lines 48/255/866»
  master: test-plan.md:668 «is prevented **by construction** … No negative test required. (Previously a "NEVER self-dial / negative test must assert" rule; deprecation history in test-plan-amendments.md.)»
  now: The master holds no `DEPRECATED` annotation at all (0 hits). The three body sites (lines 48, 98, 668) read `by-construction-satisfied` / "No test generated" / "No negative test required", each pointing to test-plan-amendments.md for the history. The file is 671 lines, so "866" names no line, and line 255 is the `**Boundary types covered:**` heading.
  replace-with (a): «The body sites in test-plan.md (§1 entity row `Self-observation loop prevention`, §1 trigger row `No self-OTLP dialing`, §11 Project-specific bullet) now read `by-construction-satisfied` — no test generated, no negative test required; the deprecation history lives in test-plan-amendments.md.»
  replace-with (b): «the by-construction text stands at test-plan.md §1 (entity row + trigger row) and §11 Project-specific»
```

```
- leaf: testing.md:74            class: stale
  leaf-text: «The following test triggers are documented as required-but-not-yet-implemented; they are deferred to the next `/andromeda-tests` re-run with this rule file as input.»
  master: test-plan.md:157 «(The `chunk-gate-baseline-coverage` and `boot-smoke-coverage` triggers are already active disciplines — see §3 "Per-chunk gate discipline".)»
  now: test-plan.md:114 says "most are not yet wired into the suite above (rows explicitly marked **LANDED** have since shipped and are retained for audit trail)", and line 157 makes two of the leaf's groups active disciplines. The leaf's intro calls every group below it not-yet-implemented and deferred, although its own bullets are LANDED ×3, active MUST ×2 and (per the first finding) retired ×1.
  replace-with: «The following triggers are decided-and-recommended. Groups marked **LANDED** have shipped and are kept for the audit trail; `boot-smoke-coverage` and `chunk-gate-baseline-coverage` are already active disciplines (test-plan §3 "Per-chunk gate discipline"); a RETIRED group is kept for audit only; anything else stays deferred to the next `/andromeda-tests` re-run with this rule file as input.»
```

```
- leaf: testing.md:107            class: stale
  leaf-text: «`cargo xtask perf:load-profiles` (four-profile suite + run-window-scoped obs gates; ~17 min — release/tag cadence, not per-PR)»
  master: test-plan.md:571 «orchestrated by `cargo xtask perf:load-profiles` (which then applies the in-process `xtask::perf_budget` grader with no arm required and prints `perf:load-profiles: perf-budget {verdict}`; the check scripts it once called were deleted at chunk 2026-09-30-perf-budget-gate-reads-real-samples)»
  now: The verb runs the four-profile suite and then the in-process `perf_budget` grader with no arm required; the obs check scripts are deleted. The master states no "~17 min" figure (0 hits).
  replace-with: «`cargo xtask perf:load-profiles` (four-profile suite, then the in-process `xtask::perf_budget` grader with no arm required, printing `perf:load-profiles: perf-budget {verdict}` — release/tag cadence, not per-PR)»
```

```
- leaf: testing.md:57            class: stale (pointer)
  leaf-text: «no test asserts an fps number, per test-plan §12 2026-06-10»
  master: test-plan.md:567 «> **Frame-budget assertion (authority: obs-plan §10).** The WebGPU fps figures above are descriptive intent only.»
  now: The master has no §12; its last section is §11 (line 601) and "§12" has 0 hits. The statement lives in §10, which adds: the frame arm asserts the ms-form "with a PASS only from the dev-host `perf:frame-sample` (on CI the frame arm reads 0 samples and prints the named cannot-evaluate line, never PASS)".
  replace-with: «no test asserts an fps number, and a frame PASS comes only from the dev-host `cargo xtask perf:frame-sample` — on CI the frame arm reads 0 samples and prints the named cannot-evaluate line, never PASS (test-plan §10 Frame-budget assertion)»
```

```
- leaf: testing.md:51            class: stale (qualifier) — the master is not uniform here
  leaf-text: «window resize and tray-icon repaint remain IPC-surrogate, deferred to the headful suite.»
  master: test-plan.md:391 «**Resize is no longer "deferred" — it is DECLINED on measurement:** `setWindowRect` reaches the WRY window and returns cleanly with `resize_delta {dw:0,dh:0}`»
  now: Line 391 (the dated correction) says "What still defers is the tray-repaint / notification surface, not the headful mode itself". Lines 79 and 383 of the same master still carry the older wording ("window resize, tray-icon repaint and OS-notification delivery … stay deferred to a headful suite"), which the leaf matches. The leaf's own line 20 already says the resize stage was DECLINED. Orchestrator's call which master passage governs.
  replace-with: «a driver-side resize stage is DECLINED on measurement (`setWindowRect` returns cleanly with zero delta); tray-icon repaint and OS-notification delivery remain IPC-surrogate, deferred to the headful suite.»
```

```
- leaf: testing.md:76-80            class: contradicted — but the master looks like the stale side
  leaf-text: «- **LANDED (chunk #77 — 2026-05-22):** PII vector test gaps (security plan §Logging vectors 2/3/4/6 lack explicit triggers; test plan currently only covers vector 5):»
  master: test-plan.md:118 «| `security-vector-coverage: AppError sanitization` | IPC error response must not contain stack traces, file paths, Rust struct names, or library versions; assert via grep on `agent-latest.jsonl`. (security-plan §Logging vector 2) |»
  now: Master rows 118-121 list the four PII-vector triggers in the pending table with no LANDED mark (line 114: only "rows explicitly marked **LANDED** have since shipped"), so as written the master says they are pending. On disk all four files the leaf names exist: `pulse-app/tests/security_apperror_sanitization.rs`, `security_plugin_path_basename_only.rs`, `security_mcp_response_body_redaction.rs`, `security_path_env_var_canonicalization.rs`.
  replace-with: no leaf edit proposed. The correction, if any, belongs in master rows 118-121 (mark LANDED, as row 122 is).
```

## One line each

- testing.md:87-88 — retired (master silent) — the `drain-golden-corpus-coverage` group has no counterpart in the master or key files (0 hits for "golden"); `crates/buffer/tests/drain_golden_corpus.rs` and `crates/buffer/tests/fixtures/drain/` exist on disk, so this is the same master-side gap as rows 118-121.
- testing.md:36 — retired (master silent) — «AAA pattern visible per test; one assertion concept per test.» is stated nowhere in the master or key files (0 hits for "AAA", "Arrange", "one assertion"); the nearest master text is line 206, "flat functions per module".
- testing.md:22 — omission, judgement call — the Framework driver list has no entry for the third product surface: test-plan.md:62 `console engine program (andromeda-pulse-engine run / version)`, driven as "the built binary spawned by `std::process::Command` from `pulse-app/tests/integration_console_engine.rs` under a cleared environment with no display variable and no session bus, its own TempDir data dir and two picked loopback ports", command grammar in process via `pulse-app/tests/unit_console_commands.rs`, and the harness path `cargo xtask harness:engine-cycle` on ports 24317 / 24318. No existing bullet is made false by its absence; the runtime-gate rule is covered by the second line-91 finding.

Checked and left alone: line 58's "runs in CI `lint-test` after `capability-widening-check`" (the master says "after capability-drift"; `.github/workflows/ci.yml` runs capability-drift → check:staged-artifacts → capability-widening-check → verify:capability-matrix, so both hold), line 25's 109-file count (master line 203 and the directory agree), and line 108's six jobs.

## Closing

- `.claude/rules/testing.md`: generated body read whole (109 lines; line 95, 2 664 chars, read to its end by window) · findings 10 in full + 3 one-liners
- `.andromeda/test-plan.md`: read whole · 671 lines · long lines read by window: yes (all 26 lines over 1 800 chars printed unclipped with `awk` in line ranges of at most 26 000 chars, each range ending on its last expected line)
- `5-command-implementation.md`: read whole · 42 lines · long lines read by window: yes (lines 4, 5 unclipped)
- `status-endpoint-shape.md`: read whole · 24 lines · long lines read by window: no (none)
- `log-format.md`: read whole · 17 lines · long lines read by window: no (none)
- `pid-file.md`: read whole · 9 lines · long lines read by window: no (none)
- `test-data-bootstrap.md`: read whole · 8 lines · long lines read by window: no (none)
- `bootstrap-phases.md`: read whole · 10 lines · long lines read by window: no (none)
- `per-chunk-gate-discipline.md`: read whole · 56 lines · long lines read by window: yes (lines 22, 24, 32, 34, 48, 50, 54 unclipped, in two ranges)
- Could not read whole: nothing. Not read, by instruction: `testing.md` line 110 onward (`## Session Additions`). The file `test-plan-amendments.md` the master points to was not in the assignment and was not read. No file was edited.
~~~


## obs-plan → rules/observability.md · docs/obs-summary.md

finding blocks in full: 25 · chars 29394

~~~
## Findings — obs-plan leaves vs master (read-only; nothing edited)

Scope note for the orchestrator: the obs master has NO §12 any more (headings stop at §11; the Decisions Log lives in `.andromeda/obs-plan-amendments.md`), and it does not mention `harness:status` / `harness:ready` / `wrong-program` / ports 24317-24318 / `inject_demo` at all. The second chunk reaches the obs master through `check:engine-log`, `harness:engine-cycle`, the `Console engine cycle` step and `logs-engine-Linux`. That is where most findings sit.

Where a passage is longer than 300 chars I give exact start and end anchors and say "whole bullet/cell".

---

### A. Contradicted / stale — CI enforcement readings (most consequential)

```
- leaf: .claude/rules/observability.md:98            class: contradicted
  leaf-text: whole bullet at line 98 — starts «- **Heartbeat-stall detection (liveness):** a `{module}.tick` gap >45s is the stall signal; the gap analysis is the script `xtask/ci/heartbeat-gap-check.sh`. Since chunk 2026-10-10-no-gate-stands-while-reading-nothing NO CI step makes the check» … ends «Its CI enforcement stands unmet and is carried on the working route.»
  master: .andromeda/obs-plan.md:559 «Since chunk 2026-10-10-agent-harness-drives-the-console-engine the gap check is made in CI for a console engine's log: the `heartbeat-gap` arm of `cargo xtask check:engine-log`»
  now: The gap check IS made in CI for a console engine's log, by the `boot` job's `Console engine cycle` step; "No CI step makes the gap check over the WINDOW app's log … so the window's ticks stay ungraded in CI". The script is `xtask/ci/heartbeat-gap-check.{sh,ps1}`, "called by `cargo xtask perf:load-profiles` alone". The master no longer says "carried on the working route".
  replace-with: «- **Heartbeat-stall detection (liveness):** a `{module}.tick` gap >45s is the stall signal — `ingest.tick` / `buffer.tick` / `connection.tick` in either program, `viz.tick` / `plugins.tick` in the window app only. In CI the gap is graded for a console engine's log (since chunk 2026-10-10-agent-harness-drives-the-console-engine): the `heartbeat-gap` arm of `cargo xtask check:engine-log`, run last by the `boot` job's `Console engine cycle` step (`cargo xtask harness:engine-cycle`, `if: always()`, directly after `ci-gates`) over the cycle's own log family — per target of the three engine ticks a gap over 45 000 ms is FAIL; fewer than two records, or a record with no readable timestamp, is cannot-evaluate, never PASS; `viz.tick` / `plugins.tick` are not read. No CI step grades the WINDOW app's ticks: `ci-gates` has had no heartbeat arm since chunk 2026-10-10-no-gate-stands-while-reading-nothing (before it, it ran over a boot log holding one tick per target, where it could not fail). The script `xtask/ci/heartbeat-gap-check.{sh,ps1}` stays, called by `cargo xtask perf:load-profiles` alone, which no workflow runs.»

- leaf: .claude/docs/obs-summary.md:66            class: contradicted
  leaf-text: whole bullet at line 66 — starts «- **Stall threshold (liveness):** missing tick for >45s = stall signal; the gap analysis is `xtask/ci/heartbeat-gap-check.sh`. No CI step runs it since chunk» … ends «its CI enforcement stands unmet and is carried on the working route.»
  master: .andromeda/obs-plan.md:559 (same anchor) + .andromeda/registries/contracts/obs-plan/heartbeat-ticks.md:5 «for a log whose `app.boot.engine` record reads `console` the gap is graded by the `heartbeat-gap` arm of `cargo xtask check:engine-log`»
  now: As above — CI-graded for a console engine's log; window ticks ungraded in CI.
  replace-with: «- **Stall threshold (liveness):** missing tick for >45s = stall signal. In CI the gap is graded for a console engine's log only (chunk 2026-10-10-agent-harness-drives-the-console-engine): the `heartbeat-gap` arm of `cargo xtask check:engine-log`, run by the `boot` job's `Console engine cycle` step, over `ingest.tick`, `buffer.tick` and `connection.tick` — a gap over 45 000 ms is FAIL; fewer than two records of a target, or a record with no readable timestamp, is cannot-evaluate, never PASS; `viz.tick` / `plugins.tick` are never read. No CI step grades the window app's ticks (`ci-gates` has had no heartbeat arm since chunk 2026-10-10-no-gate-stands-while-reading-nothing). The script `xtask/ci/heartbeat-gap-check.{sh,ps1}` stays, called by `cargo xtask perf:load-profiles` alone, which no workflow runs.»

- leaf: .claude/docs/obs-summary.md:77            class: contradicted
  leaf-text: «assert max ≤45000ms — made by no CI step since chunk 2026-10-10-no-gate-stands-while-reading-nothing (`perf:load-profiles` is the script's one caller); its CI enforcement is unmet and carried on the working route»
  master: .andromeda/obs-plan.md:559 «the gap check is made in CI for a console engine's log»
  now: Same as above.
  replace-with: «assert max ≤45000ms — in CI made for a console engine's log by the `heartbeat-gap` arm of `cargo xtask check:engine-log` (`boot` job, `Console engine cycle` step; the three engine ticks; fewer than two records of a target is cannot-evaluate, never PASS); no CI step makes it over the window app's log since chunk 2026-10-10-no-gate-stands-while-reading-nothing (`perf:load-profiles`, which no workflow runs, is the `heartbeat-gap-check.{sh,ps1}` script's one caller)»

- leaf: .claude/rules/observability.md:87            class: stale
  leaf-text: «(a) LIVENESS — missing tick for >45s = stall signal; CI fails build if any tick gap >45s during the test run.»
  master: .andromeda/obs-plan.md:559 «No CI step makes the gap check over the WINDOW app's log» · heartbeat-ticks.md:5
  now: No CI step reads a test run's ticks; the gap is CI-graded only for a console engine's log, over the three engine ticks. This sentence also contradicts the leaf's own line 98.
  replace-with: «(a) LIVENESS — missing tick for >45s = stall signal, read against the ticks the program emits; in CI the gap is graded for a console engine's log only, by the `heartbeat-gap` arm of `cargo xtask check:engine-log` over `ingest.tick` / `buffer.tick` / `connection.tick` (fewer than two records of a target is cannot-evaluate) — the window app's ticks are ungraded in CI.»

- leaf: .claude/rules/observability.md:102            class: stale
  leaf-text: «The older reading, a TEST RUN that produces zero spans failing the build as missing instrumentation, is made by no CI step (`cargo xtask test` reads no log); it stands unmet and is carried on the working route.»
  master: .andromeda/obs-plan.md:557 «is made for the console engine since chunk 2026-10-10-agent-harness-drives-the-console-engine»
  now: "`cargo xtask check:engine-log`, in the `boot` job's `Console engine cycle` step, FAILs a log family whose members hold no record (the `family` arm …) and FAILs a cycle in which no span landed (the `progress` arm: a largest `rows_ingested` of 0 …)". Only "The TEST-RUN form is made by no step". No "carried on the working route".
  replace-with: «The older reading — a run that produces zero spans fails the build as missing instrumentation — is made for the console engine since chunk 2026-10-10-agent-harness-drives-the-console-engine: `cargo xtask check:engine-log`, in the `boot` job's `Console engine cycle` step, FAILs a log family whose members hold no record (the `family` arm; no member at all is cannot-evaluate) and FAILs a cycle in which no span landed (the `progress` arm: a largest `rows_ingested` of 0; no `buffer.tick`, or no numeric `rows_ingested`, is cannot-evaluate, never PASS). The TEST-RUN form is made by no step (`cargo xtask test` returns nextest's own status and reads no log).»
```

### B. Contradicted — other causes

```
- leaf: .claude/docs/obs-summary.md:89            class: contradicted
  leaf-text: «(three production defects found+fixed at 50k spans/s by the load suite — **plus a FOURTH that is still OPEN**, below)»
  master: .andromeda/obs-plan.md:564 «A fourth was OPEN and is now CLOSED (2026-08-28) — closed by a dependency bump, not by a code change in this repo»
  now: The fourth defect is CLOSED and was "NOT … a connection-isolation violation". The leaf's own next bullet (line 90) already says CLOSED.
  replace-with: «(three production defects found+fixed at 50k spans/s by the load suite — plus a FOURTH, never a violation of this invariant, that was OPEN and is CLOSED since 2026-08-28 by a dependency bump, below)»

- leaf: .claude/rules/observability.md:71            class: contradicted
  leaf-text: «Its own EXACT leaf — the bare `interpretation` key exists and is POPULATED, so without one this target would RESOLVE while every field vanished (the most deceptive fallback shape).»
  master: .andromeda/obs-plan.md:454 «the bare `interpretation` key (measured VIOLATED 2026-08-26 at `observability.rs:1934` …) was REMOVED, and the removed-key truth is asserted by committed discriminators STRENGTHENED to `for_target("interpretation").is_none()`»
  now: The bare key has not existed since chunk 2026-08-30-diagnostics-un-muting-harness-truth-sweep. The leaf's own line 69 says so too.
  replace-with: «Its own EXACT leaf under the no-bare-`interpretation` invariant: the bare key was REMOVED at chunk 2026-08-30-diagnostics-un-muting-harness-truth-sweep (`for_target("interpretation").is_none()` is pinned), so without the leaf both fields redact. While the bare key existed, a missing leaf RESOLVED while every field vanished — the most deceptive fallback shape, and why the guard asserts the field SET.»
```

### C. Stale — two programs over one engine boot (obs-summary was only partly re-derived)

```
- leaf: .claude/docs/obs-summary.md:63            class: stale
  leaf-text: «- 15s interval for ingest/buffer/viz/plugins via `tokio::time::interval(Duration::from_secs(15))`.»
  master: .andromeda/registries/contracts/obs-plan/heartbeat-ticks.md:3 «spawned in two groups since chunk 2026-10-10-console-engine-entry-point: the engine's three ticks (`ingest.tick`, `buffer.tick`, `connection.tick`)»
  now: Five ticks in two groups; "The window app emits all five; the console program (`andromeda-pulse-engine run`) emits the three and never `viz.tick` or `plugins.tick`, so in a log whose `app.boot.engine` record reads `program: console` their absence is by design and not a stall".
  replace-with: «- 15s interval via `tokio::time::interval(Duration::from_secs(15))`, in two groups: the engine's three (`ingest.tick`, `buffer.tick`, `connection.tick`; `heartbeat::spawn_engine_ticks`, called from `engine_boot::start`) are emitted by both programs; the window's two (`viz.tick`, `plugins.tick`; `heartbeat::spawn_window_ticks`) by the window app alone — in a log whose `app.boot.engine` record reads `program: console` their absence is by design, not a stall.»

- leaf: .claude/docs/obs-summary.md:16            class: stale
  leaf-text: «  4. Spawn Tauri + bind OTLP receivers»
  master: .andromeda/registries/contracts/obs-plan/tracing-init.md:4 «(4) `engine_boot::start` spawns the engine tasks and binds the two OTLP receivers (tonic + axum) — in the window app before the Tauri builder»
  now: Step 4 is `engine_boot::start` in both programs; "the window app alone then runs the Tauri app". The receivers bind before the Tauri builder, and the console program spawns no Tauri app.
  replace-with: «  4. `engine_boot::start` spawns the engine tasks and binds the two OTLP receivers (both programs; in the window app before the Tauri builder); the window app alone then runs the Tauri app»

- leaf: .claude/docs/obs-summary.md:18            class: stale
  leaf-text: «(`install_exit_hook`) and, on Unix, the SIGTERM/SIGINT listener; the app runs through `run_return`, whose code»
  master: tracing-init.md:4 «installs … the SIGTERM/SIGINT listener (`install_signal_listener`), then records the start instant and writes the pid file. … The window app runs through `App::run_return`»
  now: `init_process` is steps (1)–(3) plus the installs ("ONE function"), ending with the start instant and the pid file. Only the window app runs through `run_return`; the console program ends by signal with `app.exit` class `signal`.
  replace-with: «(`install_exit_hook`) and, on Unix, the SIGTERM/SIGINT listener, then records the start instant and writes the pid file (steps 1–3 and these installs are that one function); the window app alone runs through `run_return`, whose code»

- leaf: .claude/docs/obs-summary.md:135            class: stale
  leaf-text: «Once per boot, emitted after `observability::init` (decided at the head of»
  master: .andromeda/obs-plan.md:329 «`warn!(target: "app.boot.render.posture", posture, lever)` — EXACTLY ONCE per window-app boot (the console program emits none, asserted on a spawned run), emitted by `main` after `engine_boot::init_process` has initialised the sink»
  now: Window-app boots only; the sink is initialised by `engine_boot::init_process`. The rules leaf (line 67) already carries this.
  replace-with: «EXACTLY ONCE per window-app boot (the console program emits none), emitted by `main` after `engine_boot::init_process` has initialised the sink (decided at the head of»
```

### D. Stale — owner / qualifier / path

```
- leaf: .claude/docs/obs-summary.md:165            class: stale
  leaf-text: «the emit site had never fired before; repair: complete the leaf; owner pinned on the working route)»
  master: .andromeda/obs-plan.md:465 «was retired unbuilt at the 2026-10-09 version close on the founder's ruling»
  now: "so in 0.3.0 as shipped both fields render `"<redacted>"` and nothing owns the repair". The rules leaf (line 76) already says this.
  replace-with: «the emit site had never fired before; repair: complete the leaf; its owner entry was retired unbuilt at the 2026-10-09 version close on the founder's ruling, so in 0.3.0 as shipped both fields render `"<redacted>"` and nothing owns the repair)»

- leaf: .claude/rules/observability.md:100            class: stale
  leaf-text: «`cargo xtask ci-gates` grades no perf arm, so the boot job prints no frame line on any run.»
  master: .andromeda/obs-plan.md:546 «the boot job prints no perf-budget `frame:` line on any run (since chunk 2026-10-10-agent-harness-drives-the-console-engine its `Console engine cycle` step prints one `engine-log: budget frame and snapshot not graded …` line, the engine check's own)»
  now: The boot job does print one line naming the frame arm, from the engine check; and master :560 names a second CI perf gate, "the `budget` arm of `cargo xtask check:engine-log` in the `boot` job's `Console engine cycle` step" (memory alone, required).
  replace-with: «`cargo xtask ci-gates` grades no perf arm, so the boot job prints no perf-budget `frame:` line on any run — its `Console engine cycle` step prints one `engine-log: budget frame and snapshot not graded …` line, the engine check's own (that check's `budget` arm grades memory alone, required, through the same grader, since chunk 2026-10-10-agent-harness-drives-the-console-engine).»

- leaf: .claude/docs/obs-summary.md:80            class: stale
  leaf-text: «`cargo xtask ci-gates` grades no perf arm and the boot job prints no frame line on any run.»
  master: .andromeda/obs-plan.md:546 (same anchor)
  now: Same as above.
  replace-with: «`cargo xtask ci-gates` grades no perf arm and the boot job prints no perf-budget `frame:` line on any run (its `Console engine cycle` step prints one `engine-log: budget frame and snapshot not graded …` line, the engine check's own).»

- leaf: .claude/rules/observability.md:36            class: stale
  leaf-text: «- **Path:** `~/.andromeda-pulse/logs/agent-latest.jsonl` (per-platform per arch §Occupied Resources Filesystem locations).»
  master: .andromeda/registries/contracts/obs-plan/log-file-location.md:3 «`<data_dir>/logs/agent-latest.jsonl` — `<data_dir>` from `resolve_data_dir()`: `ANDROMEDA_PULSE_DATA_DIR` when set»
  now: The path is data-dir-relative; `~/.andromeda-pulse/` is only the Linux fallback when `XDG_CONFIG_HOME` is unset. Every CI log in the master sits under an overridden data dir. The master itself keeps `~/.andromeda-pulse/logs/agent-latest.jsonl` as shorthand elsewhere (e.g. logging-stack.md:5), so only this "Path" bullet is stale.
  replace-with: «- **Path:** `<data_dir>/logs/agent-latest.jsonl`, `<data_dir>` from `resolve_data_dir()` — `ANDROMEDA_PULSE_DATA_DIR` when set, else `%APPDATA%\andromeda-pulse\` (Windows), `~/Library/Application Support/com.andromeda.pulse/` (macOS), `$XDG_CONFIG_HOME/andromeda-pulse/` else `~/.andromeda-pulse/` (Linux) — per arch §Filesystem locations.»

- leaf: .claude/docs/obs-summary.md:54            class: stale
  leaf-text: «- **Path:** `~/.andromeda-pulse/logs/agent-latest.jsonl` (per-platform per arch §Filesystem locations).»
  master: log-file-location.md:3 (same anchor)
  now: Same as above.
  replace-with: same replacement as rules:36.
```

### E. Omissions inside existing bullets (the bullet misleads as a rule today)

```
- leaf: .claude/rules/observability.md:101            class: omission
  leaf-text: «via the same required arm (a 0 sample is uninformative).»
  master: .andromeda/obs-plan.md:547 «and, since chunk 2026-10-10-agent-harness-drives-the-console-engine, over a console engine cycle's log by the `budget` arm of `cargo xtask check:engine-log` in the `boot` job (the same grader, memory required; no sample, or only zero samples, fails)»
  now: The memory budget has two CI readings: `perf:budget --require memory,snapshot` in lint-test, and the engine check's `budget` arm in `boot`. "a required arm holding only zeros fails".
  replace-with: «via the same required arm (a 0 sample is uninformative; a required arm holding only zeros fails) — and, since chunk 2026-10-10-agent-harness-drives-the-console-engine, over a console engine cycle's log by the `budget` arm of `cargo xtask check:engine-log` in the `boot` job (the same grader, memory required; no sample, or only zero samples, fails).»

- leaf: .claude/docs/obs-summary.md:81            class: omission
  leaf-text: «CI-enforced by the same `perf:budget` required arm (a 0 sample is uninformative).»
  master: .andromeda/obs-plan.md:547 (same anchor)
  now: Same as above.
  replace-with: «CI-enforced by the same `perf:budget` required arm (a 0 sample is uninformative; only zeros fails) and, since chunk 2026-10-10-agent-harness-drives-the-console-engine, over a console engine cycle's log by the `budget` arm of `cargo xtask check:engine-log` in the `boot` job (memory required; no sample or only zero samples fails).»

- leaf: .claude/rules/observability.md:99            class: omission
  leaf-text: «NEUTRAL-tolerant — no `buffer.tick` in the stream is not a failure — so it is safe to run unconditionally.»
  master: .andromeda/obs-plan.md:537 «a verb that prints NEUTRAL and exits 0 over a log family holding no `buffer.tick`, so over such a log it asserts nothing. The readings that cannot be neutral are `cargo xtask check:engine-log`'s»
  now: The master frames the tolerance as a limit, and names the reading that cannot be neutral: the engine check's `progress` arm ("a stall announcement that is not a recovery is FAIL; no `buffer.tick`, or no numeric `rows_ingested`, is cannot-evaluate; a largest `rows_ingested` of 0 is FAIL").
  replace-with: «It prints NEUTRAL and exits 0 over a log family holding no `buffer.tick` — so over such a log it asserts nothing. The reading that cannot be neutral is the `progress` arm of `cargo xtask check:engine-log` over a console engine cycle's log: a stall announcement that is not a recovery is FAIL; no `buffer.tick`, or no numeric `rows_ingested`, is cannot-evaluate; a largest `rows_ingested` of 0 is FAIL.»

- leaf: .claude/rules/observability.md:103            class: omission
  leaf-text: «Any future check script over `agent-latest.jsonl` MUST adopt the same posture.»
  master: .andromeda/obs-plan.md:580 «the tolerance above is the perf grader's, for an arm no caller requires, and is not a licence for a gate to pass over an absent input»
  now: "`perf:load-profiles` alone grades with no arm required"; `ci-gates` left the grader ("an absent log family is exit 2 `cannot-evaluate`, never NEUTRAL or PASS"); `check:engine-log` "is a third caller of the grader … has no NEUTRAL reading … and never passes over an input it cannot grade". As written, the leaf would direct a new gate to be NEUTRAL over absent input.
  replace-with: «Any future xtask check over `agent-latest.jsonl` MUST be NEUTRAL-tolerant for an UNREQUIRED arm only — the tolerance is the perf grader's, for an arm no caller requires, and is not a licence for a gate to pass over an absent input: `perf:load-profiles` alone grades with no arm required; `ci-gates` left the grader (an absent log family is exit 2 `cannot-evaluate`, never NEUTRAL or PASS); `cargo xtask check:engine-log` is a third caller — it requires the memory arm in-process through `perf_budget::grade_arm`, has no NEUTRAL reading, and never passes over an input it cannot grade.»

- leaf: .claude/docs/obs-summary.md:87            class: omission
  leaf-text: «absent metric stream → NEUTRAL, not FAIL — one grader serves headless CI and booted-app (ACTIVE) sessions.»
  master: .andromeda/obs-plan.md:580 (same anchor)
  now: Same as above. "headless CI" is also off: on CI the lint-test caller requires its arms and `ci-gates` no longer calls the grader.
  replace-with: «absent metric stream → NEUTRAL, not FAIL — one grader serves headless verification and booted-app (ACTIVE) sessions. The tolerance is the perf grader's, for an arm no caller requires, never a licence for a gate to pass over an absent input: `perf:load-profiles` alone grades with no arm required, `ci-gates` left the grader (absent log family = exit 2 `cannot-evaluate`), and `cargo xtask check:engine-log` (memory arm required through `perf_budget::grade_arm`) has no NEUTRAL reading.»
```

### F. Retired / stale pointers (lower consequence)

```
- leaf: .claude/rules/observability.md:19            class: retired
  leaf-text: «Body sites in security-plan.md are annotated with `> **DEPRECATED (2026-05-08)**` blockquotes (preserved verbatim for audit trail; new readers see deprecation notice first).»
  master: .andromeda/security-plan.md:12 «This body holds ONLY current truth. Amendment history — … superseded guidance (e.g., the pre-pivot `opentelemetry-stdout` self-observation references) — lives in the sibling `security-plan-amendments.md`»
  now: `security-plan.md` holds 0 occurrences of `DEPRECATED`, and `opentelemetry-stdout` only on line 12; no annotated body sites exist at lines 154/239/330. The same bullet's tail «body annotations at security-plan.md lines 154/239/330» and its "(e.g., `security-plan.md` §Data Protection / §Bootstrap phases … / §Logging & Monitoring …)" list are retired with it. Checked by grep against a master outside my assignment (security-plan), not a whole read — please verify.
  replace-with: «The security-plan body holds current truth only; its pre-pivot `opentelemetry-stdout` references left the body and live in `.andromeda/security-plan-amendments.md`.» (and drop the «; body annotations at security-plan.md lines 154/239/330» tail)

- leaf: .claude/rules/observability.md:92            class: stale
  leaf-text: «per obs-plan §12 2026-06-10»
  master: .andromeda/obs-plan.md:531 «## 10. SLO Invariants & Telemetry Budgets» (no §12 exists; `.andromeda/obs-plan-amendments.md:39` «2026-06-10 — Chunk #99 tag gate … → folded to §10»)
  now: The frame budget's home is §10 (frame row, :546, "SLO p99 ≤ 33ms"). test-plan states the fps figures are "descriptive intent only" with "authority: obs-plan §10".
  replace-with: «per obs-plan §10»

- leaf: .claude/docs/obs-summary.md:86            class: stale
  leaf-text: «(chunk #99, per obs-plan §12 2026-06-10)»
  master: same as above
  now: Same as above.
  replace-with: «(chunk #99, obs-plan §10)»

- leaf: .claude/docs/obs-summary.md:171            class: stale
  leaf-text: «(per obs-plan §12 Decisions Log entry 2026-05-04 "Clarify PII grep heuristic UI-vocabulary exemption")»
  master: .andromeda/obs-plan.md:485-486 «#### UI-vocabulary exemption»
  now: The exemption is §8 body text. Its member sets also differ from the leaf: the master lists FIVE regexes (`Bearer [a-zA-Z0-9]{40,}`, `AKIA[0-9A-Z]{16}`, `sk-[a-zA-Z0-9]{40,}`, `ghp_[a-zA-Z0-9]{36}`, `password=[^\s]+`) — the leaf's sixth, `xox[baprs]-[0-9a-zA-Z-]{10,}`, is in no part of the obs master. The master forbids leakage of "real OTLP attribute values, real plugin paths, real secrets, and real query parameters", and attributes the "Token budget" label to design-system, not "a11y-plan §3 P2".
  replace-with: «(per obs-plan §8 "UI-vocabulary exemption")» — and, in the same blockquote, drop «, `xox[baprs]-[0-9a-zA-Z-]{10,}`» from the regex list.
```

### G. One line each

- `.claude/rules/observability.md:87` — omission — clause (b) ends «asserted by `cargo xtask check:ingest-progress`» without the heartbeat-ticks.md:5 qualifier ("reads NEUTRAL, exit 0, over a family with no `buffer.tick`; the reading that cannot be neutral is the `progress` arm of `cargo xtask check:engine-log` over a console engine's log"). Same fix as rules:99.
- `.claude/docs/obs-summary.md:67` — omission — same missing qualifier after «asserted by `cargo xtask check:ingest-progress`».
- `.claude/docs/obs-summary.md:78` — omission — the Drain-progress row says «NEUTRAL when the stream carries no `buffer.tick`» and omits that over such a log it asserts nothing, and that the non-neutral reading is `check:engine-log`'s `progress` arm (obs-plan.md:537).
- `.claude/rules/observability.md:103` — retired — «ACTIVE evidence (session 183, 900,500 spans at 10k/s into the live app): … heartbeat max-gap 15.0s …» is in no master body. "900,500" matches nothing under `.andromeda/*.md`; the amendments sidecar (`obs-plan-amendments.md:39`) keeps only "frame p99 = 27.3ms over 56,642 real frames, buffer max 152.4MB". The master body's current frame reading is p99 2.6 ms, n = 7971 (:546). Orchestrator's call whether to keep it as history.
- `.claude/docs/obs-summary.md:88` — not judged — the «ACTIVE evidence flow: `scripts/agent-run boot` … `load_profiles -- custom <rate> <secs>`» recipe is nowhere in the obs master or its key files; it was not checked against test-plan.
- `.claude/docs/obs-summary.md` (PII section, lines 102-169) — outside the strict omission class (no existing bullet) — the section has a paragraph for every other `app.*` leaf but none for `app.boot.engine` (obs-plan.md:448; fields `program` / `interpretation` / `reason`, once per `engine_boot::start`, WARN when `interpretation` is `none`). The tick fix at summary:63 names that record, so the summary would cite a record it never defines.

Checked and found true as written, so not reported: the rules leaf's init order (line 20), tick split (85), `app.boot.render.posture` (67), `app.boot.engine` (68) and `interpretation.model.allow_root` "once per window-app boot" (71, first half) — all re-derived correctly. The `harness:status` / `harness:ready` parentheticals (rules:88, summary:68) still match architecture.md as written; `--program window|console` is an addition there, not a contradiction. The clipboard and updater-URL rules (rules:80-81) are held by security-plan.md:347-358, not the obs master.

---

- `.claude/rules/observability.md`: generated body read whole (123 lines) · findings 13 (11 full blocks + 2 one-liners)
- `.claude/docs/obs-summary.md`: generated body read whole (197 lines) · findings 16 (13 full blocks + 3 one-liners; plus 2 notes not classed)

- `.andromeda/obs-plan.md`: read whole · 662 lines · long lines read by window: yes — 329, 396, 546, 560 by `awk substr` windows covering their full length; 454, 465, 500, 559, 568 by full-line `awk` print; 63, 72, 132, 267 through the read tool, each confirmed unclipped by its closing characters
- `.andromeda/registries/contracts/obs-plan/tracing-init.md`: read whole · 14 lines · long lines read by window: yes (line 4, full-line print, unclipped)
- `.andromeda/registries/contracts/obs-plan/service-identity.md`: read whole · 6 lines · long lines read by window: no (none)
- `.andromeda/registries/contracts/obs-plan/logging-stack.md`: read whole · 6 lines · no (none)
- `.andromeda/registries/contracts/obs-plan/log-format-json-schema.md`: read whole · 21 lines · no (none)
- `.andromeda/registries/contracts/obs-plan/log-file-location.md`: read whole · 4 lines · no (none)
- `.andromeda/registries/contracts/obs-plan/snapshot-paste-to-ai-integration.md`: read whole · 16 lines · no (none)
- `.andromeda/registries/contracts/obs-plan/trace-context-propagation.md`: read whole · 7 lines · no (none)
- `.andromeda/registries/contracts/obs-plan/heartbeat-ticks.md`: read whole · 9 lines · no (none)

Could not read whole / not read:
- Leaf long lines (rules 77, 86, 100, 104; summary 165, 169) were read through the read tool, which returned them unclipped (each ends on its bullet's closing sentence); they were not re-windowed with awk.
- `security-plan.md`, `architecture.md`, `test-plan.md` and `obs-plan-amendments.md` were only grepped for specific strings (finding F-1, the §12 pointer, the harness parentheticals, the fps ruling), never read whole. Anything resting on them is marked above.
- `ci.yml` and source code were not opened. Every finding is leaf against master text only.
~~~


## architecture → CLAUDE.md generated blocks

finding blocks in full: 9 · chars 14682

~~~
## Findings — CLAUDE.md generated blocks (lines 3-130) vs `.andromeda/architecture.md` + its four key files

Most consequential first. Nothing was edited.

```
- leaf: CLAUDE.md:33            class: contradicted
  leaf-text: «NO direct `mistralrs` import at this crate level — concrete `MistralRsInference` impl lives at the binary boundary in `pulse-app/` per arch §Established Decisions [LLM Inference Runtime — L4 interpretation layer] bus factor mitigation entry.»
  master: .andromeda/architecture.md:76 «the `pub trait LlmInferenceRunner: Send + Sync` (`crates/interpretation/src/contract.rs`) is the swap boundary — only the concrete impl changes»
  now: The runtime is "`llama.cpp` prebuilt binaries (b9305-pinned series) invoked via subprocess", consumed by `pulse-app/src/llamacli_inference.rs` (arch:242, :245); arch:315 says it "Superseded the Pre-D1 `mistralrs = "=0.8.0"` choice". No `MistralRsInference` exists (tree: only `pub struct LlamaCliInference` at `pulse-app/src/llamacli_inference.rs:145`). The leaf's own line 9 already says the impl swapped to `LlamaCliInference`.
  replace-with: «NO inference runtime is linked at this crate level — the concrete `LlamaCliInference` impl (the `llama.cpp` `llama-cli` subprocess runner, `pulse-app/src/llamacli_inference.rs`) lives at the binary boundary in `pulse-app/`; the `LlmInferenceRunner` trait is the swap boundary per arch §Established Decisions [LLM Inference Runtime — L4 interpretation layer] (the `mistralrs` choice is superseded).»

- leaf: CLAUDE.md:27            class: contradicted
  leaf-text: «L1 streaming distillation scaffold (chunk #60 — Epoch 9 Foundation v0.2.0): 7 module skeletons (`baseline` / `pattern` / `cue` / `digest` / `interpretation` / `incident` / `lifecycle`) + 10 contract types in `triage::contract`»  …and, same line: «Empty implementations; consumed by chunks #61+ for streaming baseline trackers + attention cue emitter + restart event detector per pulse v0.2.0 plan Phase 2.»
  master: .andromeda/architecture.md:77 «`triage::digest::retrieval::select_corpus_matches` takes `triggering_scope: Option<&str>` … and its one production caller, `assemble`, passes the triggering cue's `scope_id`»
  now: The master describes `triage` as live production code, not empty skeletons: `triage::digest::render_payload` and `select_corpus_matches` (arch:77), `triage::contract::IncidentRegistry` + `IncidentPersistence` + `IncidentLifecycleBroadcast` (arch:202), `IncidentWriteOutcome` and `DurableActiveIncidents::active_incident_ids` (arch:78), `triage::lifecycle::InMemoryServiceRegistry::list` (arch:193), the cadence coordinator `crates/triage/src/cadence/broadcast.rs` (arch:207), `Thresholds::from_env()` at `crates/triage/src/cue/thresholds.rs` (arch:249), `cue_cause_label` / `incident_event_kinds()` / `hex_lower` (arch:77, :220, :250). The contract surface is far larger than the ten types listed.
  replace-with: «L1 streaming distillation (scaffolded at chunk #60 — Epoch 9 Foundation v0.2.0; live since): modules `baseline` / `cadence` / `pattern` / `cue` / `digest` / `interpretation` / `incident` / `lifecycle`, all `pub(crate)` behind `triage::contract` — attention cues and their closed cause labels (`cue_cause_label`), the cadence coordinator (`pulse://stream/cadence-events`), the digest assembler and its corpus retrieval (`render_payload`, `select_corpus_matches`), the incident registry and its persistence ports (`IncidentRegistry` / `IncidentPersistence` / `DurableActiveIncidents`, `IncidentWriteOutcome`), the service lifecycle registry (`services.list_with_states`) and the env-resolved bootstrap window (`Thresholds::from_env`).»  …and drop the "Empty implementations; consumed by chunks #61+ …" sentence. (The eight-module list is read from `crates/triage/src/lib.rs`; the master names `cadence` only by path. Keep or drop the ten-type list as you judge — the master names many more contract items.)

- leaf: CLAUDE.md:36            class: omission
  leaf-text: «agent-run 5-command harness (boot/run/status/cleanup/logs)»
  master: .andromeda/architecture.md:268 «sh+ps1 in lockstep but for two sh-only parts, the exit-witness arm of `boot` and the `engine` word of `boot` / `status`; 5 unchanged verbs»
  now: Still five verbs, but the sh `boot` / `status` "take one optional word — none is the window app, `engine` is the console engine, any other word is usage, exit 2"; `harness:status` / `harness:ready` take `--program window|console` with a `wrong-program` arm (exit 1); and xtask gained `check:engine-log`, `harness:engine-settled` and `harness:engine-cycle` ("one whole run of the console engine under the harness", default ports 24317 / 24318, called by the `Console engine cycle` step of ci.yml's `boot` job). The xtask bullet lists the gate verbs one by one and names none of these, so it reads as if the harness drives the window app only.
  replace-with: «agent-run 5-command harness (boot/run/status/cleanup/logs; the sh `boot` / `status` take the word `engine` for the console program) + the harness verdict verbs `harness:status` / `harness:ready` (`--program window|console`; a `wrong-program` arm, exit 1), `harness:settled` (the window's alone), `harness:boot-series` + the console engine's `check:engine-log` / `harness:engine-settled` / `harness:engine-cycle` (one whole console-engine run on harness-only ports 24317 / 24318; the `Console engine cycle` step of ci.yml's `boot` job)»

- leaf: CLAUDE.md:25            class: stale
  leaf-text: «`snapshot.{generate,list_recent,copy_to_clipboard}`»
  master: .andromeda/architecture.md:187 «`snapshot.list_recent` and `snapshot.copy_to_clipboard` deferred — no runtime emitter as of chunk #74; only `snapshot.generate` implemented at `pulse-app/src/snapshot_runtime.rs`»
  now: Only `snapshot.generate` is implemented; the other two are deferred. The tree agrees: `xtask/src/main.rs:1091` pins `"snapshot.generate"` and `:1108` holds the other two commented out.
  replace-with: «`snapshot.generate` (`snapshot.list_recent` / `snapshot.copy_to_clipboard` deferred — no runtime emitter)»

- leaf: CLAUDE.md:28            class: stale
  leaf-text: «`workspace.{detect,list}`»
  master: .andromeda/architecture.md:191 «`workspace.detect` — workspace-detector crate (`workspace.list` deferred — no runtime emitter as of chunk #74; only `workspace.detect` implemented at `crates/ui-bridge/src/workspace_ipc.rs:47`)»
  now: Only `workspace.detect` is implemented; `workspace.list` is deferred (tree: `xtask/src/main.rs:1106` pinned, `:1109` commented out).
  replace-with: «`workspace.detect` (`workspace.list` deferred — no runtime emitter)»

- leaf: CLAUDE.md:44            class: stale   (warning cites security-plan §Input, so this may duplicate the security comparer's; the deciding text is arch's)
  leaf-text: «`agent-run.sh boot` reads `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` (trim + regular-file check, fail closed) and loads `scripts/exit-witness.c`'s built library onto the app's one spawn command through `LD_PRELOAD`»
  master: .andromeda/architecture.md:259 «read by bare `scripts/agent-run.sh boot`, the window program's boot (`agent-run.ps1` does not read it, and `boot engine` never reads it)»
  now: The exit-witness arm is the window program's alone: "loaded into the window app's spawn alone through `LD_PRELOAD`"; under `boot engine` the spawn line "sets no preload even then" (arch:261). A second harness reader exists: `cargo xtask harness:engine-cycle` "reads it by value, only to pass it on into its children's cleared environment".
  replace-with: «bare `agent-run.sh boot` — the window program's boot alone; `boot engine` never reads it and sets no preload — reads `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` (trim + regular-file check, fail closed) and loads `scripts/exit-witness.c`'s built library onto the window app's one spawn command through `LD_PRELOAD` (`cargo xtask harness:engine-cycle` reads the variable by value only to pass it on to its children)»

- leaf: CLAUDE.md:81            class: stale   (pointer table; checked against the file it points at)
  leaf-text: «33-chunk route plan»
  master: docs/v0_2_0/pulse-v0_2_0-route.md:791 «**Total chunks:** 41 (#57 through #97), plus 2 pre-flight decisions (Pre-D1, Pre-D2). 33 original v2 chunks + 8 new consolidation chunks (#70-#77)»
  now: The route plan the row points at holds 41 chunks; 33 is the pre-consolidation count.
  replace-with: «41-chunk route plan (33 original + 8 consolidation)»

- leaf: CLAUDE.md:61            class: stale   (pointer table; section content)
  leaf-text: «Standard Contracts (`app_info` / `health` / `ready` envelopes; OTLP / MCP / IPC error schemas; the capability record's form another project reads) | `.andromeda/architecture.md` §Standard Contracts»
  master: .andromeda/architecture.md:86 «**Error response schema (Tauri IPC)**: every `#[taurpc::procedure]` returns `Result<T, AppError>`» (under `## Conventions`, arch:81; siblings at :87 OTLP and :88 MCP)
  now: §Standard Contracts (arch:100-171) holds the `app_info` / `health` / `ready` envelopes, the spec-conformant OTLP and MCP surfaces, the paginated-list envelope, the real-time push contract and the capability record. The three error schemas are in §Conventions, not there.
  replace-with: «Standard Contracts (`app_info` / `health` / `ready` envelopes; OTLP / MCP wire surfaces; paginated-list envelope; real-time push contract; the capability record's form another project reads) + error schemas (IPC `AppError` / OTLP / MCP) | `.andromeda/architecture.md` §Standard Contracts / §Conventions»

- leaf: CLAUDE.md:31            class: stale
  leaf-text: «since chunk #92) and, since the same chunk, one lifecycle row per incident status VALUE-change»
  master: .andromeda/architecture.md:220 «(2) `CorpusWriter::update_incident_status` (since `2026-08-30-diagnostics-un-muting-harness-truth-sweep`) records one lifecycle row per status VALUE-change»
  now: The second writer dates from chunk 2026-08-30-diagnostics-un-muting-harness-truth-sweep; as written, "the same chunk" reads as chunk #92.
  replace-with: «since chunk #92) and, since chunk 2026-08-30-diagnostics-un-muting-harness-truth-sweep, one lifecycle row per incident status VALUE-change»
```

One line each (lower consequence):
- CLAUDE.md:14 — omission — the Key-directories `xtask/` list says "agent-run harness" and names none of the console-engine verbs (`check:engine-log`, `harness:engine-settled`, `harness:engine-cycle`; arch:268). Same fact as the line-36 finding; fix both or neither.
- CLAUDE.md:7 — omission — the product paragraph describes a desktop dashboard only; arch:293 Product type now adds "a second program over the same engine: the console program `andromeda-pulse-engine`" (`run` | `version`, no window). Lines 13, 35 and 98 already say it, so this is only the opening sentence.
- CLAUDE.md:44 — omission (security comparer's line) — "`pre-push:linux`'s by-value read of `HOME` / `PATH`" is no longer the only one: arch:258 adds `harness:boot-series` (`PATH`, to find `xvfb-run`) and `harness:engine-cycle` (both, to build its children's cleared environment).
- CLAUDE.md:119-120 — stale against the tree, not a master — `testing.md` "(Rust source + tests)" and `observability.md` "(Rust source — tracing discipline)": both rules' `paths:` frontmatter also match `pulse-app/ui/**/*.{ts,tsx}`.

Checked and true as written:
- Line 15 (six jobs on `ubuntu-22.04`), line 98 (architecture block), line 50 (fault identity) and line 45 (TauRPC pin / capability policy) match the master.
- Every other pointer-table path and section exists: §Threat Model Summary, §Bootstrap phases, §Security Anti-Patterns, test-plan §3 / §6 / §10, obs-plan §3 / §10, a11y-plan §3, the three `*-contracts.toml` files plus the architecture one (4 rows, 4 key files), `cache/{rust,ts}/tree.db`, `scripts/code-graph.py` and its cookbook, the 5 summaries, the 5 core docs, the playbook, `docs/andromeda-improvements.md`, `docs/v0_2_0/`.
- Counts by `ls`: 14 files in `.claude/docs/services/`, 8 in `.claude/rules/`, 14 dirs in `crates/`; "100 chunks / 9 epochs" matches `.andromeda/route.md:19-20`.

Master-side notes (the leaf is right or follows another master; no leaf edit proposed):
- **Self-observation, CLAUDE.md:46.** The leaf says "`tracing` ecosystem only (no OTel SDK in self-runtime)". arch:71 and arch:281 still say "`opentelemetry-stdout` (or file exporter)" is the product's exporter. obs-plan:30, :72, :158 and :170 say tracing-only with no OTel SDK, and state that they strengthen the arch wording; no workspace manifest declares an `opentelemetry*` dependency. The leaf follows obs-plan; the arch text is the stale side.
- **Crate count.** arch:4 and arch:214 (canonical) say fourteen library crates, which the leaf follows. arch:295 and :297 still say "twelve Rust crates" / "twelve reserved crate names", arch:53, :59 and :66 say "12-module", and arch:83 lists 12 router crates.
- **Directory-structure key file.** `project-directory-structure.md` lists only 8 crates under `crates/` (no `curation`, `triage`, `corpus`, `security`, `interpretation`, `config-watcher`) and three workflow files. The tree has a fourth, `.github/workflows/secret-scan.yml`, which the arch master never names; CLAUDE.md:15 mirrors the master's three.
- **Likely origin of the stale leaf lines 27 and 33.** Code comments still carry the old wording: `crates/triage/src/lib.rs:6` ("Module skeletons at scaffold stage") and `crates/interpretation/src/lib.rs:7` / `broadcast.rs:5` ("Concrete mistralrs implementation…").

Closing lines:
- CLAUDE.md: generated body read whole (128 lines, 3-130; long lines 9, 31, 32, 35, 44, 50 read by window; 33 and 98 whole) · findings 9 in full + 4 one-line
- .andromeda/architecture.md: read whole · 328 lines · long lines read by window: yes (line 268 in three windows, 77 in two; 35, 47, 76, 78, 93, 186, 220, 221, 226, 229, 250, 286 printed whole, unclipped)
- .andromeda/registries/contracts/architecture/build-system.md: read whole · 5 lines · long lines read by window: no (none)
- .andromeda/registries/contracts/architecture/deployment-model.md: read whole · 4 lines · long lines read by window: no (none)
- .andromeda/registries/contracts/architecture/project-directory-structure.md: read whole · 59 lines · long lines read by window: no (none)
- .andromeda/registries/contracts/architecture/ci-cd-approach.md: read whole · 6 lines · long lines read by window: yes (line 3 in two windows)
- Not read whole: nothing in my assignment. The other masters (security-plan, test-plan, obs-plan, a11y-plan, route.md, the v0.2.0 route) were only grepped for headings and specific counts, to check pointer-table targets.
~~~


## architecture → docs/stack.md · docs/conventions.md · rules/frontend.md

finding blocks in full: 14 · chars 19985

~~~
## Findings — leaves vs `.andromeda/architecture.md` (most consequential first)

Every `leaf-text` below was checked with `grep -c -F` and occurs exactly once in its file. Where a passage exceeds 300 chars I give its start and end anchors (both verified unique) and mark the elision with `…`.

```
- leaf: .claude/docs/stack.md:38            class: contradicted
  leaf-text: «- **Implementation status:** chunk #82 trait surface + stub concrete `MistralRsInference` substrate landed (session 139 commit cf6686b); … positioned in route §Epoch 9 before chunk #84 fallback-tier work.»  (the whole of line 38; start and end anchors exact)
  master: .andromeda/architecture.md:76 «Chunk 2026-10-05-l4-runs-the-founder-s-pick-with-its-authors-settings ships that pick» · :242 «consumed by `pulse-app/src/llamacli_inference.rs` … (chunk #84)» · :212 «with no window, no display and no model runner» · :250 «UNSET SEATS NOTHING»
  now: The runtime swap is not "the next implementation work to plan": the llama-cli runner is shipped (`pulse-app/src/llamacli_inference.rs`, chunk #84) and runs the founder's pick. `MistralRsInference`, `cf6686b`, `0e37159` appear nowhere in architecture.md (history moved to `architecture-amendments.md`). The master also now states the runner is seated per program: the window app seats the canned or the llama-cli runner; the console program seats the canned runner only when `ANDROMEDA_PULSE_L4_DETERMINISTIC` is truthy, else nothing.
  replace-with: «- **Implementation status:** shipped — `LlamaCliInference` (`pulse-app/src/llamacli_inference.rs`, chunk #84) is the concrete runner behind the trait and runs the founder's pick (chunk 2026-10-05-l4-runs-the-founder-s-pick-with-its-authors-settings); the deterministic canned runner (`pulse-app/src/deterministic_inference.rs`) is selected by `ANDROMEDA_PULSE_L4_DETERMINISTIC`. Per program: the window app `pulse-app` seats the canned runner when the gate is truthy, else the llama-cli runner; the console program `andromeda-pulse-engine run` has no model runner — truthy seats the canned runner, unset seats nothing (no interpretation subscriber, no L4 heartbeat; cues and digests are produced and no incident forms), reported once per boot on `app.boot.engine`.»

- leaf: .claude/docs/stack.md:21            class: stale
  leaf-text: «8 name-dispatched tools incl. `query_traces` / `query_metrics` / `query_logs` / `generate_snapshot` (gated by `--features mcp-server`)»
  master: .andromeda/architecture.md:25 «9 named tools on the wire» · :60 «form the 9-tool wire surface»
  now: Nine tools: the four telemetry tools, the four chunk #94 corpus-backed tools, and `retrieve_incident_events` (chunk 2026-10-02-incident-events-readable-through-mcp). (`.claude/docs/conventions.md:23` already says 9.)
  replace-with: «9 name-dispatched tools — `query_traces` / `query_metrics` / `query_logs` / `generate_snapshot` + `query_incident_list` / `retrieve_report` / `retrieve_telemetry_slice` / `mark_incident_resolved` + `retrieve_incident_events` (gated by `--features mcp-server`)»

- leaf: .claude/docs/stack.md:77            class: stale
  leaf-text: «`pulse:default`, `pulse:tray`, `pulse:notification`, `pulse:updater`, `pulse:plugin-fs` — concrete JSON in `pulse-app/capabilities/`.»
  master: .andromeda/architecture.md:269 «`pulse:default`, `pulse:tray`, `pulse:notification`, `pulse:updater`, `pulse:plugin-fs`, `pulse:clipboard`»
  now: Six reserved capability identifiers, the sixth `pulse:clipboard`; :268 also pins «6 capability files» in `staged_gate::EXPECTED_GRANTS`.
  replace-with: «`pulse:default`, `pulse:tray`, `pulse:notification`, `pulse:updater`, `pulse:plugin-fs`, `pulse:clipboard` — concrete JSON in `pulse-app/capabilities/` (6 files, pinned by `staged_gate::EXPECTED_GRANTS`).»

- leaf: .claude/docs/stack.md:37            class: stale
  leaf-text: «rely on a user-set `ANDROMEDA_PULSE_LLAMA_CUDA_BIN` / `ANDROMEDA_PULSE_LLAMA_CPU_BIN` env var pair pointing at user-managed install (the current dev pattern from session 144 spike)»
  master: .andromeda/architecture.md:242 «`ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`» · :243 «`ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH`» · :76 «rely on the `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH`/`_CPU_BIN_PATH` env pair (current dev pattern)»
  now: Both variables end in `_PATH`; the names without it are registered nowhere. They are product-consumed by `pulse-app/src/llamacli_inference.rs`, guarded like `ANDROMEDA_PULSE_MODEL_PATH` (reject `..`, ≤ 4096 bytes, canonicalize, regular file, confinement under `ANDROMEDA_PULSE_L4_ALLOW_ROOT` when set).
  replace-with: «rely on the `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` / `ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH` env pair pointing at a user-managed install (the current dev pattern; both consumed by `pulse-app/src/llamacli_inference.rs`)»

- leaf: .claude/docs/stack.md:37            class: stale
  leaf-text: «**flagged-pending** — the runtime-swap chunk MUST decide whether»
  master: .andromeda/architecture.md:76 «**Open caveat (decide during any runtime chunk)**: prebuilt-binary distribution is unsettled»
  now: The runtime swap has landed; the distribution question stays open and is to be decided "during any runtime chunk", no longer by a pending runtime-swap chunk.
  replace-with: «**unsettled (open caveat — decide during any runtime chunk)**: whether»

- leaf: .claude/docs/stack.md:72            class: stale
  leaf-text: «- **Deployment target:** Cross-platform desktop app (Windows / macOS / Linux). NOT a web app, NOT a CLI fleet, NOT a microservice cluster.»
  master: .andromeda/architecture.md:293 «and — since chunk 2026-10-10-console-engine-entry-point — a second program over the same engine: the console program `andromeda-pulse-engine`»
  now: Product type is the desktop application «shipped as native bundles» plus the console program, «driven by a closed two-word command line (`run` | `version`) with no window and no display. Not a web app, not a microservice fleet.» The master no longer carries a "not a CLI" clause, and names what the console program is not yet: «a background service, a bundled release artifact».
  replace-with: «- **Deployment target:** Cross-platform desktop app (Windows / macOS / Linux) shipped as native bundles, and — since chunk 2026-10-10-console-engine-entry-point — a second program over the same engine: the console program `andromeda-pulse-engine` (`run` | `version`, no window, no display; not yet a background service or a bundled release artifact). Not a web app, not a microservice fleet.»

- leaf: .claude/docs/stack.md:10            class: stale
  leaf-text: «`scripts/agent-run.sh boot` preloads it into the app's spawn alone to record which call ended the process.»
  master: .andromeda/architecture.md:32 «bare `scripts/agent-run.sh boot` preloads into the window app's spawn alone (`boot engine` spawns the console engine with no preload, since chunk 2026-10-10-agent-harness-drives-the-console-engine)»
  now: The exit-witness arm is the window program's alone: only bare `boot` preloads; `boot engine` never reads the variable and sets no preload (:259–:261), and `harness:engine-cycle` reads a witness file in its data dir as a failure (:226).
  replace-with: «Bare `scripts/agent-run.sh boot` preloads it into the window app's spawn alone (`boot engine` spawns the console engine with no preload) to record which call ended the process.»

- leaf: .claude/docs/stack.md:35            class: stale
  leaf-text: «unchanged from chunk #82 — only the concrete impl swaps from `MistralRsInference` to a new `LlamaCliInference` sibling at the `pulse-app/` binary boundary»
  master: .andromeda/architecture.md:76 «is the swap boundary — only the concrete impl changes»
  now: The master names no `MistralRsInference`; `LlamaCliInference` is the existing concrete impl (:230, :246), not a new sibling to come.
  replace-with: «unchanged from chunk #82 — the swap boundary: only the concrete impl changes (today `LlamaCliInference`, `pulse-app/src/llamacli_inference.rs`, at the `pulse-app/` binary boundary)»

- leaf: .claude/docs/stack.md:28            class: retired
  leaf-text: «, swapped from the original `mistralrs = "=0.8.0"` choice per session 144 (2026-05-25) empirical invalidation: upstream issue … ~4.3 s for a complete schema-conformant L4 output).»  (from that comma to the end of line 28; start and end anchors exact)
  master: .andromeda/architecture.md:76 «*(History — the original `mistralrs = "=0.8.0"` choice and its empirical invalidation: see `architecture-amendments.md` § Decision-history 2026-05-25.)*»
  now: architecture.md no longer states the mistral.rs issue, the Windows MSVC deadlock or the tok/sec figures (28.2 / 231.2 / 122.3 are in `architecture-amendments.md` only). The retired passage also quotes a figure «under L4 `--json-schema-file` GBNF constraint», an argv the master says the product never passes (:76 «never `--json-schema-file`»).
  replace-with: «. (History — the original `mistralrs = "=0.8.0"` choice and its empirical invalidation: `.andromeda/architecture-amendments.md` § Decision-history 2026-05-25.)»

- leaf: .claude/docs/conventions.md:21            class: stale
  leaf-text: «`snapshot.generate`, `snapshot.list_recent`, `snapshot.copy_to_clipboard`, `plugins.list`, `plugins.reload`, `plugins.invoke`, `mcp.status`, `mcp.start`, `mcp.stop`, `workspace.detect`, `workspace.list`, `telemetry.frontend.record_frame_ms`»
  master: .andromeda/architecture.md:187 «(`snapshot.list_recent` and `snapshot.copy_to_clipboard` deferred — no runtime emitter as of chunk #74; only `snapshot.generate` implemented» · :191 «`workspace.list` deferred — no runtime emitter as of chunk #74; only `workspace.detect` implemented»
  now: Three of the names the leaf lists as per-crate router procedures are deferred with no runtime emitter (they are commented out of `EXPECTED_PROCEDURES`). The Conventions bullet itself (:91) gives five examples and says «the Convention defines the shape, not the enumeration».
  replace-with: «`snapshot.generate`, `plugins.list`, `plugins.reload`, `plugins.invoke`, `mcp.status`, `mcp.start`, `mcp.stop`, `workspace.detect`, `telemetry.frontend.record_frame_ms` (`snapshot.list_recent`, `snapshot.copy_to_clipboard` and `workspace.list` are deferred — no runtime emitter)»

- leaf: .claude/rules/frontend.md:30            class: omission
  leaf-text: «- Real-time push: subscribe to `pulse://stream/spans` / `pulse://stream/metrics` / `pulse://stream/logs` / `pulse://stream/snapshot-progress` / `pulse://stream/plugin-events` via `Channel<Uint8Array>` API; payloads are binary Arrow IPC.»
  master: .andromeda/architecture.md:207 «`pulse://stream/plugin-events` (deferred — no runtime emitter as of chunk #74; pending plugin invocation telemetry chunk)» · :184 «`streams.subscribe_spans`, `streams.subscribe_metrics`, `streams.subscribe_logs` — pulse-app crate (Tauri Channel<Vec<u8>> binding …)»
  now: The Channel bindings that exist are the three `streams.subscribe_*` procedures; `plugin-events` is a reserved name with no emitter, so a subscription to it receives nothing.
  replace-with: «- Real-time push: subscribe to `pulse://stream/spans` / `pulse://stream/metrics` / `pulse://stream/logs` through `streams.subscribe_spans` / `_metrics` / `_logs` (`Channel<Uint8Array>`; payloads are binary Arrow IPC); `pulse://stream/snapshot-progress` is the fourth contract event; `pulse://stream/plugin-events` is reserved but deferred — no runtime emitter.»

- leaf: .claude/rules/frontend.md:28            class: stale
  leaf-text: «- Generated bindings live alongside `pulse-app/ui/src/bindings/` (per crate router).»
  master: .andromeda/architecture.md:286 «the git-INDEX copy of `pulse-app/ui/src/bindings/index.ts`» · :268 «rewrites the tracked `pulse-app/ui/src/bindings/index.ts`»
  now: The generated bindings are one tracked file, `pulse-app/ui/src/bindings/index.ts`, not a file per crate router; the `test` stage rewrites it, and a pin edit must be staged together with it or `capability-drift` reds.
  replace-with: «- Generated bindings are the one tracked file `pulse-app/ui/src/bindings/index.ts` (all routers; regenerated by the test run — stage it together with any `EXPECTED_PROCEDURES` edit).»

- leaf: .claude/docs/stack.md:67            class: stale
  leaf-text: «13 → 15 at 2026-08-24-headful-mechanics-probe-race-disposition, 15 → 16 at 2026-08-27-report-window-copy-affordance, where `widget-close` stopped being terminal, 16 → 17»
  master: .andromeda/architecture.md:47 «13 → 15 at `2026-08-24-headful-mechanics-probe-race-disposition`, where `widget-close` stopped being terminal — `signpost-repeat` follows it deliberately»
  now: `widget-close` stopped being terminal at the 13 → 15 step (when `signpost-repeat` was added after it), not at 15 → 16, which added `report-copy`.
  replace-with: «13 → 15 at 2026-08-24-headful-mechanics-probe-race-disposition, where `widget-close` stopped being terminal, 15 → 16 at 2026-08-27-report-window-copy-affordance, 16 → 17»

- leaf: .claude/docs/stack.md:20            class: omission
  leaf-text: «`wasmtime` 25+ with WASM Component Model + WIT»
  master: .andromeda/architecture.md:22 «`wasmtime` 25+ family … Cargo.toml requirement `"48.0.4"`, lockfile-resolved **48.0.5** as of 2026-10-04 … 49.x is out of reach while the toolchain is pinned at 1.95»
  now: The leaf carries requirement and resolved version for `duckdb` (:41) and `rmcp` (:21) but gives wasmtime as "25+" alone, though the same leaf's line 6 rests the Rust floor on wasmtime 48.0.5.
  replace-with: «`wasmtime` 25+ family (Cargo.toml requirement `"48.0.4"`, lockfile-resolved 48.0.5 as of 2026-10-04; 49.x out of reach while the toolchain is pinned at 1.95) with WASM Component Model + WIT»
```

### Lower consequence, one line each

- `.claude/docs/stack.md:51` — omission — the five event names match architecture.md:169, but :207 marks `pulse://stream/plugin-events` «deferred — no runtime emitter»; same qualifier as the frontend.md:30 finding.
- `.claude/docs/stack.md:29` — retired — «, well under D1's ~6-10 calls/min capacity, and D1 releases RAM + VRAM between cadence ticks (local-first respect for the user's machine)»: architecture.md:76 now gives the D1-over-D2 reason as «the ~5s cold-start tax (~1–2s warm) hides behind no spinner and D2's permanent-lifecycle complexity buys throughput Pulse won't exercise»; no capacity figure and no RAM/VRAM clause in any `.andromeda` master. The «two unbounded-generation runaways during the spike work» clause on the same line is likewise absent.
- `.claude/docs/stack.md:31` — retired — « CUDA build matched to the host driver's max-supported toolkit (currently CUDA 13.1 against driver 596.36 on the spike host). Metal / Vulkan / SYCL / HIP available …»: stated in no master (the only host driver architecture.md names is nvidia-open 610.57.04 at :251).
- `.claude/docs/stack.md:30` — retired — the «Quantization formats supported: GGUF (2-8 bit) …» bullet: stated in no master.
- `.claude/docs/stack.md:34` — retired — the «Tokenizer» bullet (the `tokenizers` workspace dep, chunk #81): stated in no master; architecture.md holds only the build-script variable `ANDROMEDA_LLAMA3_TOKENIZER_PATH` (:247).
- `.claude/docs/stack.md:35` — stale — «`candle` if needed (original chunk #82 escape hatch remains valid)» → master :76 «`candle` (+ `outlines-rs`/`llguidance` for constrained sampling) if llama.cpp's maintenance posture changes».
- `.claude/docs/stack.md:6` — retired — the closing parenthetical «(Edition 2024 itself cannot parse below 1.85; security-positive defaults …)» is not in architecterure.md's Stack row (:14), which instead says «so no toolchain below 1.95.0 builds the product».
- `.claude/docs/stack.md:87` — stale — «resolve before tagging v0.1.0»: master :53 says «read both `Cargo.toml`s before tagging» with no version, and :305 adds the fallback (downgrade to `tonic` 0.13.x); «`cargo deny check bans` enforces» is not in architecture.md.
- `.claude/rules/frontend.md:27` — stale — «TauRPC-generated `.d.ts` bindings»: the Conventions sentence it restates (:83) says «TauRPC-generated `.ts` bindings», and the file is `index.ts`. (The Stack row :46 still says `.d.ts`, so the master is not uniform here.)

### Closing

- `.claude/docs/stack.md`: generated body read whole (98 lines) · findings 19 (11 in full, 8 one-line)
- `.claude/docs/conventions.md`: generated body read whole (96 lines) · findings 1
- `.claude/rules/frontend.md`: generated body read whole (lines 1–93; `## Session Additions` at line 94 not read for reporting) · findings 3 (2 in full, 1 one-line)

- `.andromeda/architecture.md`: read whole · 328 lines · long lines read by window: yes — 14 lines exceed 1 800 chars (35, 47, 76, 77, 78, 93, 186, 220, 221, 226, 229, 250, 268, 286); line 268 (42 291 chars) read in four contiguous windows (1–11 000, 11 001–22 000, 22 001–32 500, 32 501–end), line 77 (16 016) in two; the others printed uncut with `awk`; window seams checked for continuity.
- `.andromeda/registries/contracts/architecture/build-system.md`: read whole · 5 lines · long lines read by window: no (none)
- `.andromeda/registries/contracts/architecture/ci-cd-approach.md`: read whole · 6 lines · long lines read by window: line 3 (7 924 chars) printed uncut in one piece
- `.andromeda/registries/contracts/architecture/deployment-model.md`: read whole · 4 lines · long lines: none
- `.andromeda/registries/contracts/architecture/project-directory-structure.md`: read whole · 59 lines · long lines: none
  (These four were not named in the assignment; I read them because §Infrastructure Patterns is only a pointer to them and stack.md's "Development & CI" / "Infrastructure" sections restate them. No finding came from them: stack.md:61–62 match build-system.md.)

Nothing was left unread. Not judged, because the source is another master and not architecture.md (attributed by grep counts across the `.andromeda` masters, not by a whole read of those masters):
- stack.md:54–55, :57, part of :58 (tracing stack, "NO OTel SDK", sentry, `wgpu_backend`) — obs-plan.
- stack.md:63–65 (nextest 0.9.x, cargo-llvm-cov, proptest) — test-plan.
- stack.md:69, :74, :80 (cargo-audit / cargo-deny / cargo-auditable versions, Dependabot, Premium SKU, OIDC, Minisign, rustls) — security-plan.
- stack.md:22 and frontend.md Framework / State / Routing / Design-token / a11y / CSP sections — design, layout, a11y, security masters. frontend.md:64's «skeleton-pulse» is obs-plan's, not architecture.md's (whose `TelemetryApi` roster at :185–:188 is six methods, none of that name).
- conventions.md:80–85 (Logging conventions) — obs-plan §3. conventions.md:87–96 (Git / commit conventions, File modification discipline) — found in no `.andromeda` master I grepped (`Co-Authored-By`, `Conventional commits`: 0 hits in all eight files); the section header says "Extracted from architecture.md Conventions section", which does not hold them.

Three things in the master itself that the orchestrator may want to know before writing edits (not leaf findings):
1. architecture.md:71 and :281 name `opentelemetry-stdout` (or a file exporter) as the product's own telemetry exporter, while stack.md:55 (obs-plan-sourced) says «NO OTel SDK linked». The two masters disagree; I did not adjudicate.
2. architecture.md:76 says the L4 rate has «no user-action trigger» (stack.md:29 repeats it as «no user-action L4 trigger in the codebase»), while :204 registers `investigate.run_action`, which «runs a real `LlmInferenceRunner::generate_constrained` analysis» on a user action. The leaf matches :76 as written, so I reported nothing.
3. architecture.md:295 and :297 still say «twelve Rust crates» / «twelve reserved crate names» against «fourteen library crates (sixteen workspace members)» at :4 and sixteen names at :214. No leaf in this assignment states a twelve.

One typo of mine to correct when reading: in the one-liner for stack.md:6, "architecterure.md" is `architecture.md`.
~~~


## architecture + test-plan §3 → docs/commands.md · docs/gotchas.md · docs/workflow.md

finding blocks in full: 13 · chars 20327

~~~
## Findings (most consequential first)

None of the three leaves holds a `## Session Additions` line or a `USER:` block, so each file was compared whole.

Where a leaf line carries column-alignment spaces, `leaf-text` quotes only the comment part (unique in the file) and the command-column change is given separately.

```
- leaf: .claude/docs/commands.md:100            class: stale
  leaf-text: «# Real-process status verdict JSON {verdict,pid,ended,log_file_basename,last_write_age_seconds,stale_after_seconds}; exits 0/1/1/2»
  master: .andromeda/architecture.md:268 «`cargo xtask harness:status [--program window|console]` … `{verdict, pid, ended, program, log_file_basename, last_write_age_seconds, stale_after_seconds}` — seven members»
  now: Seven members (adds `program`), pinned by set equality; five arms `running-healthy` (exit 0) / `stale` (exit 1) / `wrong-program` (exit 1) / `not-running` (exit 1) / `cannot-evaluate` (exit 2); the verb takes `--program window|console`. Same in 5-command-implementation.md:17-30.
  replace-with: «# Real-process status verdict JSON {verdict,pid,ended,program,log_file_basename,last_write_age_seconds,stale_after_seconds}; program = the log family's last app.boot.engine record (window | console | unknown); with --program, a run of the other program, or of none, reads wrong-program; exit 0 running-healthy / 1 stale, wrong-program or not-running / 2 cannot-evaluate»
  also: command column on that line becomes `cargo xtask harness:status [--program window|console]`
```

```
- leaf: .claude/docs/commands.md:101            class: stale
  leaf-text: «# Boot readiness verdict JSON {verdict,pid,ended,otlp_grpc,otlp_http}: status running-healthy AND a TCP connection accepted on both OTLP ports; exit 0 ready / 1 not-ready or ended / 2 cannot-evaluate»
  master: .andromeda/architecture.md:268 «`cargo xtask harness:ready [--program window|console]` … `{verdict, pid, ended, program, otlp_grpc, otlp_http}` — six members»
  now: Six members (adds `program`), pinned by set equality; five arms `ready` (exit 0) / `not-ready` (exit 1) / `wrong-program` (exit 1) / `ended` (exit 1) / `cannot-evaluate` (exit 2); "`wrong-program` holds whatever the receivers answer, and a dead pid is `ended` whatever was asked". Same in 5-command-implementation.md:5.
  replace-with: «# Boot readiness verdict JSON {verdict,pid,ended,program,otlp_grpc,otlp_http}: status running-healthy for the program asked AND a TCP connection accepted on both OTLP ports; exit 0 ready / 1 not-ready, wrong-program or ended / 2 cannot-evaluate»
  also: command column on that line becomes `cargo xtask harness:ready [--program window|console]`
```

```
- leaf: .claude/docs/commands.md:88            class: stale
  leaf-text: «# cargo xtask harness:status — real-process verdict JSON, exits 0/1/1/2 (not-running is non-zero)»
  master: .andromeda/registries/contracts/test-plan/5-command-implementation.md:17 «the sh `status` takes one optional word — none passes `--program window`, `engine` passes `--program console`, any other word is usage, exit 2»
  now: The sh verb always asks for a program, and the verdict has five arms (0 / 1 `stale` / 1 `wrong-program` / 1 `not-running` / 2). "`agent-run.ps1 status` takes no word and passes no flag."
  replace-with: «# cargo xtask harness:status --program window (status engine, sh only: --program console) — real-process verdict JSON, exit 0 running-healthy / 1 stale, wrong-program or not-running / 2 cannot-evaluate»
```

```
- leaf: .claude/docs/commands.md:86            class: omission
  leaf-text: «# Start app, await ready via harness:ready verdict — status running-healthy AND both OTLP ports accepting (10s default; HARNESS_STATUS_TIMEOUT overrides)»
  master: .andromeda/registries/contracts/test-plan/5-command-implementation.md:4 «the sh `boot` takes one optional word — none is the window app, as above; `engine` is the console engine; any other word is usage, exit 2»
  now: `boot engine` pre-builds `cargo build --bin andromeda-pulse-engine --release` and xtask, spawns `target/release/andromeda-pulse-engine run` by path and polls `harness:ready --program console`; bare `boot` asks `--program window`. The word is sh only ("`agent-run.ps1` is unchanged, so the two scripts no longer take the same words").
  replace-with: «# Start the window app (boot engine, sh only: the console engine andromeda-pulse-engine run; any other word is usage, exit 2), await ready via harness:ready verdict — status running-healthy for the program asked AND both OTLP ports accepting (10s default; HARNESS_STATUS_TIMEOUT overrides)»
```

```
- leaf: .claude/docs/commands.md:103            class: omission
  leaf-text: «on the dev host set ANDROMEDA_PULSE_OTLP_GRPC_PORT / _HTTP_PORT off 4317 / 4318 first»
  master: .andromeda/architecture.md:268 «`cargo xtask check:engine-log` — the console engine's log check … `cargo xtask harness:engine-settled [--timeout-seconds N]` (default 60) … `cargo xtask harness:engine-cycle [--data-dir DIR] [--grpc-port N] [--http-port N]`»
  now: Three verbs are registered beside the four `harness:*` verbs this block already lists. Basis for calling it an omission: the file's line 3 claims "Full command surface", and the block carries every other `harness:*` verb; the heading itself claims nothing, so this is your call. Contracts as in the replacement; also in per-chunk-gate-discipline.md:54.
  replace-with: «on the dev host set ANDROMEDA_PULSE_OTLP_GRPC_PORT / _HTTP_PORT off 4317 / 4318 first
cargo xtask harness:engine-settled [--timeout-seconds N]       # Console engine settle verdict JSON {verdict,pid,program,ingest_ticks,buffer_ticks,connection_ticks,memory_samples_populated}: boot record reads console, pid alive, two records of each of ingest.tick / buffer.tick / connection.tick and one non-zero metric.buffer.memory_bytes sample; exit 0 settled / 1 ended, wrong-program or not-settled / 2 cannot-evaluate (default 60 s, refused below 20); writes no file; harness:settled stays the window's
cargo xtask check:engine-log                                   # Console engine log check, no argument (paths resolved as harness:status does): seven arms family, program, panic, heartbeat-gap (over 45 000 ms between consecutive ingest.tick / buffer.tick / connection.tick records), progress, process-end, budget (memory max 512 000 000 B); one line per arm, then engine-log: PASS | FAIL | cannot-evaluate; exit 0 / 1 / 2, a FAIL on any arm outranks a cannot-evaluate
cargo xtask harness:engine-cycle [--data-dir DIR] [--grpc-port N] [--http-port N]   # Linux: one whole run of the console engine — injector build, agent-run.sh boot engine, inject_demo --sustained --error-pct=0, harness:engine-settled, status engine, cleanup, check:engine-log — every child under a cleared environment; defaults target/engine-cycle/{UTC second} and harness-only ports 24317 / 24318 (4317 / 4318 refused: shared-port); verdict JSON {verdict,boot,settled,status,cleanup,check,error_records,witness_file}; exit 0 pass / 1 fail / 2 cannot-evaluate; CI: the boot job's Console engine cycle step»
```

```
- leaf: .claude/docs/workflow.md:49            class: stale
  leaf-text: «- Heartbeat-stall detection passes (no `{module}.tick` gap >45s during test run)»
  master: .andromeda/architecture.md:268 «`heartbeat-gap` (per target of `ingest.tick`, `buffer.tick`, `connection.tick`: … a gap over 45 000 ms between two consecutive records ⇒ FAIL; `viz.tick` and `plugins.tick` are never read)» and «`cargo xtask ci-gates` runs no grader and prints no perf-budget, frame or heartbeat line»
  now: The gap check is CI-enforced only for the console engine's log, by `cargo xtask check:engine-log`, the last step of `cargo xtask harness:engine-cycle` (the `boot` job's `Console engine cycle` step, ci-cd-approach.md:3). It covers three tick targets, not every module, and not a test run. Supporting text outside my assignment: obs-plan.md:559 "No CI step makes the gap check over the WINDOW app's log".
  replace-with: «- Console engine cycle passes (`cargo xtask harness:engine-cycle`, the `boot` job's `Console engine cycle` step): its `cargo xtask check:engine-log` grades the console engine's own log — no gap over 45 000 ms between consecutive `ingest.tick` / `buffer.tick` / `connection.tick` records (fewer than two is cannot-evaluate, never PASS), a family holding records with `rows_ingested` above 0, memory max ≤ 512 000 000 B, no `app.panic.fatal`, exactly one `app.exit` and it is last. No CI step makes the heartbeat-gap check over the window app's log»
```

```
- leaf: .claude/docs/workflow.md:50            class: stale
  leaf-text: «- Zero `app.panic.fatal` spans in test logs»
  master: .andromeda/architecture.md:268 «It reads the app's log family `agent-latest.jsonl*` under the resolved log dir in two arms: zero-spans (the family holds at least one record …) and zero-panic (no `app.panic.fatal` record at level ERROR)»
  now: The panic reading is `cargo xtask ci-gates`, run "by ci.yml's `boot` job after the smoke and the series" over the boot smoke's app log, not over test logs. It has a second arm (zero-spans), and "never a pass over an absent family" (exit 2). The record is a log record, not a span.
  replace-with: «- `cargo xtask ci-gates` passes over the `boot` job's app log family (`agent-latest.jsonl*`): zero-spans (the family holds at least one record) and zero-panic (no `app.panic.fatal` record at level ERROR); an absent family is cannot-evaluate (exit 2), never a pass»
```

```
- leaf: .claude/docs/gotchas.md:13-14            class: stale
  leaf-text: «### `rust-toolchain.toml` 1.84 → 1.85 bump» (line 13) and the whole of line 14, which begins «Architecture's Stack table specified `rustc 1.84+` initially, but Edition 2024 cannot parse below `1.85.0`. **Bump `rust-toolchain.toml` to `1.85.0` minimum**» (line 14 is about 480 chars; replace it whole)
  master: .andromeda/architecture.md:14 «toolchain pinned to rustc 1.95.0 (`rust-toolchain.toml`); the workspace declares `rust-version = "1.95"` once in `[workspace.package]` … so no toolchain below 1.95.0 builds the product»
  now: The Stack table no longer says 1.84+; the pin is 1.95.0, the declared floor equals it and is "set by the resolved dependency graph", held by the xtask test `declared_floor_equals_the_pinned_channel`; "the workspace's own code needs only rustc ≥ 1.89". Nothing is open, yet the entry sits under "Open reconciliations". Outside my assignment: `rust-toolchain.toml` reads `channel = "1.95.0"`; `CLAUDE.md` holds no "1.85" or "1.84" string, so the entry's last sentence is also unsupported; security-plan.md:473 still keeps "NEVER let the rust-toolchain drift below 1.85.0" as the Edition 2024 floor.
  replace-with: «### `rust-toolchain.toml` pin — CLOSED (1.95.0)
The toolchain is pinned to rustc 1.95.0 and the workspace declares `rust-version = "1.95"` once in `[workspace.package]`, equal to the pin and set by the resolved dependency graph (wasmtime 48.0.5 and its internal crates, cranelift 0.135.5, pulley 48.0.5 declare 1.95.0) — no toolchain below 1.95.0 builds the product, though the workspace's own code needs only rustc ≥ 1.89. The xtask test `declared_floor_equals_the_pinned_channel` holds the declared floor equal to the pin. Edition 2024 itself cannot parse below 1.85.0, so the pin never drifts below that; wasmtime 49.x is out of reach while the pin is 1.95 (it needs Rust 1.96).»
```

```
- leaf: .claude/docs/workflow.md:71-72            class: omission
  leaf-text: «# 5. Push + open PR
git push -u origin feat/my-task»
  master: .andromeda/registries/contracts/test-plan/per-chunk-gate-discipline.md:32 «`cargo xtask pre-push:linux` … is the **local Linux pre-push check**» and :36 «**The merge-base probe (operator leg, directly before the push).**»
  now: The pre-push verb runs six stages on the Linux dev host (exit 0 green / 1 red / 2 cannot-evaluate). The probe at :39 goes "directly before" a build-branch push; "A red is brought to the operator before any push; the pass never clears it by a fetch, a merge or a rebase of its own." Caveat: the master frames the probe as an entry of every chunk plan that lists a push, not of a generic dev loop. The leaf's step 3 says "(mirror CI)" with four commands and then pushes with neither.
  replace-with: «# 5. Pre-push check (Linux dev host, ci.yml's Node major first on PATH), the merge-base probe, then push + open PR
cargo xtask pre-push:linux                       # six stages; exit 0 green / 1 red / 2 cannot-evaluate
m="$(git ls-remote origin refs/heads/main | cut -f1)" && test -n "$m" && git merge-base --is-ancestor "$m" HEAD   # red: bring it to the operator; never clear it by a fetch, merge or rebase
git push -u origin feat/my-task»
```

```
- leaf: .claude/docs/commands.md:73-74            class: omission
  leaf-text: «# With MCP feature
cargo run --bin pulse-app --features mcp-server»
  master: .andromeda/architecture.md:212 «Console engine binary: `andromeda-pulse-engine` — the second `[[bin]]` of the `pulse-app` package … a closed two-word command line — `run` … `version`»
  now: "The product binaries are three: `pulse-app` (the window app), `andromeda-pulse-engine` (the console program), `andromeda-pulse-mcp`". `run` boots "with no window, no display and no model runner, and stays up until SIGTERM or SIGINT"; `version` prints one line and exits 0; any other command line exits 2. Path `target/{profile}/andromeda-pulse-engine` (architecture.md:210). Same basis as the three-verb omission: the "Full command surface" claim on line 3, with "Run locally" naming one of two runnable programs.
  replace-with: «# With MCP feature
cargo run --bin pulse-app --features mcp-server

# Console engine — the package's second program: same engine boot, no window, no display, no model runner
cargo build --bin andromeda-pulse-engine --release
target/release/andromeda-pulse-engine run        # stays up until SIGTERM / SIGINT; binds 4317 / 4318 and writes the same pid file as pulse-app
target/release/andromeda-pulse-engine version    # one line, exit 0; any other command line is usage, exit 2»
```

```
- leaf: .claude/docs/workflow.md:44            class: stale
  leaf-text: «the frame arm by `cargo xtask perf:frame-sample` on a GPU dev host, since hosted runners expose no WebGPU adapter»
  master: .andromeda/architecture.md:268 «`cargo xtask perf:frame-sample` — the dev-host frame-budget gate … Windows only; boots `target/release/pulse-app.exe` … `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS` (a SwiftShader WebGPU flag set)»
  now: The verb is Windows only and grades under a software (SwiftShader) adapter; not-Windows reads INCONCLUSIVE (exit 2). `commands.md:110` already says "Windows dev host only … software WebGPU adapter". Outside my assignment: obs-plan.md:546 says "Windows GPU dev host", so the leaf dropped "Windows". The same bullet also omits that the memory budget is graded a second time in CI, for the console engine's log (`check:engine-log` `budget` arm); that is covered if the workflow.md:49 replacement above is taken.
  replace-with: «the frame arm by `cargo xtask perf:frame-sample` on a Windows dev host (software WebGPU adapter), since hosted runners expose no WebGPU adapter»
```

```
- leaf: .claude/docs/gotchas.md:31            class: stale
  leaf-text: «the JSON file at `~/.andromeda-pulse/logs/agent-latest.jsonl` IS the agent surface»
  master: .andromeda/registries/contracts/test-plan/5-command-implementation.md:39 «the sink is `tracing_appender::rolling::daily(log_dir, "agent-latest.jsonl")`, which DATE-SUFFIXES every file (`agent-latest.jsonl.YYYY-MM-DD`), so a bare-name reader … silently finds nothing»
  now: The surface is the rotated family `agent-latest.jsonl*`; no file of the bare name exists. Low confidence on class: security-plan.md:165 and :352 (outside my assignment) still write the bare name.
  replace-with: «the JSON log family `~/.andromeda-pulse/logs/agent-latest.jsonl*` (date-suffixed daily, `agent-latest.jsonl.YYYY-MM-DD`; a bare-name read finds nothing) IS the agent surface»
```

```
- leaf: .claude/docs/commands.md:62            class: retired
  leaf-text: «cargo xtask changelog          # Generate release changelog»
  master: .andromeda/architecture.md:42 «CI task runner | `cargo-xtask` | Release / sign / notarize as Rust binaries inside the same workspace»
  now: No assigned master states a `changelog` task; architecture.md:42 and :319 name release / sign / notarize only. Outside my assignment: `xtask/src/main.rs` `enum Cmd` has no `Changelog` variant.
  replace-with: «» (delete the line)
```

### One line each (lower consequence, or not decidable from my masters)

- `.claude/docs/commands.md:59-61` — not a leaf finding: `cargo xtask release` / `sign` / `notarize` match architecture.md:42 and :319, but `xtask/src/main.rs` `enum Cmd` has no such variants (35 variants read). The master, not the leaf, is what disagrees with the code.
- `.claude/docs/commands.md:92` — true as written; if lines 86 and 88 gain `engine`, this line should say the ps1 script takes no `engine` word (architecture.md:268 "`agent-run.ps1` takes no such word").
- `.claude/docs/commands.md` (block at 94-119) — also absent, all defined in architecture.md:268 or per-chunk-gate-discipline.md: `ci-gates`, `quarantine-tracking`, `check:ingest-progress`, `capability-widening-check`, `smoke:gap-resume` / `external-resolve` / `hue-shift` / `discovery`, `perf:load-profiles`, `webview-drive`. An omission only on the strength of the line-3 "Full command surface" claim.
- `.claude/docs/workflow.md:27` — omission (weak): the merge-gate summary names no `boot` job gate (smoke, `harness:boot-series --count 7`, `ci-gates`, `Console engine cycle`), though ci-cd-approach.md:3 makes each a gating step.
- `.claude/docs/gotchas.md:46` — stale name (weak): the heading "rmcp stdio reservation" credits rmcp, while architecture.md:60 says the stdio protocol is the hand-rolled `jsonrpc.rs` layer and rmcp is an anchor dependency only; the body is true.
- `.claude/docs/gotchas.md:30-31` — not a leaf finding: "NO OTel SDK is linked" agrees with obs-plan.md:30 and security-plan.md:165, but architecture.md:71 and :281 still say `opentelemetry-stdout` "is the only exporter the product itself uses". The two masters disagree with each other.
- `.claude/docs/workflow.md:87` and `.claude/docs/commands.md:135-139` — not decidable from my masters: both speak of "living artifacts (dependency-tree.md + api-surface.md)" and `cargo-modules` / `cargo-public-api` for their reconcile, while `commands.md:171` says the code-graph "replaces the retired markdown living-trees". The leaves contradict each other; no assigned master speaks to it.

### Closing

- `.claude/docs/commands.md`: generated body read whole (183 lines) · findings 7
- `.claude/docs/gotchas.md`: generated body read whole (78 lines) · findings 2
- `.claude/docs/workflow.md`: generated body read whole (99 lines) · findings 4

- `.andromeda/architecture.md`: read whole · 328 lines · long lines read by window: yes (14 lines over 1 800 chars; line 268, 42 291 chars, in three windows 1-13000 / 13001-27000 / 27001-end; the other 13 printed whole, unclipped)
- `.andromeda/registries/contracts/architecture/ci-cd-approach.md`: read whole · 6 lines · long lines read by window: yes (line 3, 7 924 chars, printed whole)
- `.andromeda/registries/contracts/test-plan/5-command-implementation.md`: read whole · 42 lines · long lines read by window: yes (lines 4 and 5 printed whole)
- `.andromeda/registries/contracts/test-plan/pid-file.md`: read whole · 9 lines · long lines read by window: no (none over 1 800 chars)
- `.andromeda/registries/contracts/test-plan/per-chunk-gate-discipline.md`: read whole · 56 lines · long lines read by window: yes (three line-range windows 1-31 / 32-47 / 48-56, each printed unclipped)

Not read whole, by assignment: the rest of `.andromeda/test-plan.md`, `obs-plan.md`, `security-plan.md`. I read single lines of them (test-plan:501, obs-plan:30 / :546 / :559, security-plan:165 / :352 / :473) only to cross-check; each use is marked "outside my assignment" above. Leaf statements whose only authority is one of those plans (coverage thresholds, a11y thresholds, DuckDB encryption, CSP, Minisign, wasmtime advisories, tray Halo, OTLP HTTP mitigations) were not judged.
~~~


## architecture → docs/services/*.md (14)

finding blocks in full: 38 · chars 43242

~~~
# Leaf recompute: `.claude/docs/services/*.md` (14 notes) against `.andromeda/architecture.md`

All fourteen notes are wholly generated (none has a `## Session Additions` line). Twelve of them carry findings; `curation.md` and `security.md` are clean.

Two kinds of evidence decide the findings below, and I mark which:
- **master**: the architecture master, or the located security/obs rule, says otherwise.
- **tree**: the path or dependency does not exist on disk (`ls` / `find` / `grep`). In several of these the master itself still names the old owner; see "Master-vs-tree divergences" at the end.

## Findings in full, most consequential first

### Who boots / wires a module

```
- leaf: .claude/docs/services/config-watcher.md:21            class: contradicted
  leaf-text: «The caller spawns with ITS spawner: `tauri::async_runtime::spawn(task.run())` in production boot, `tokio::spawn(task.run())` in `#[tokio::test]`.»
  master: .andromeda/architecture.md:4 «Neither entry point wires an engine part itself, and the shared boot and the console module name no window framework»
  now: The watcher is started inside the shared engine boot, which names no window framework. Tree: `pulse-app/src/engine_boot.rs:877-885` calls `start_config_watcher` then `tokio::spawn(config_task.run())`; `engine_boot.rs` has 0 `tauri::async_runtime::spawn`.
  replace-with: «The caller spawns with ITS spawner: `tokio::spawn(config_task.run())` in production boot — inside the one shared engine boot `pulse_app::engine_boot::start`, which both programs (the window app and the console engine `andromeda-pulse-engine`) call with a tokio runtime already entered and which names no window framework — and `tokio::spawn(task.run())` in `#[tokio::test]`.»
```

```
- leaf: .claude/docs/services/config-watcher.md:36            class: stale
  leaf-text: «- **Boot wiring:** `pulse-app/src/main.rs` (spawns via `tauri::async_runtime::spawn`)»
  master: .andromeda/architecture.md:4 «wired by ONE shared engine boot, `pulse_app::engine_boot` (`init_process`, then `start`), which both entry points of the `pulse-app` package call»
  now: `main.rs` wires no engine part; the shared boot does, for both programs. Tree: `main.rs` only hands the boot's handle slot and status to `ConfigApiImpl::new` (main.rs:187).
  replace-with: «- **Boot wiring:** `pulse-app/src/engine_boot.rs` (`engine_boot::start` starts the watcher and spawns its task via `tokio::spawn`, for both programs; `main.rs` wires none itself — it only passes the returned handle slot + status to `ConfigApiImpl`)»
```

```
- leaf: .claude/docs/services/triage.md:44            class: stale
  leaf-text: «- **Boot wiring:** `pulse-app/src/main.rs` (constructs broadcasts, spawns heartbeats, threads Arc<dyn Trait> into resolvers)»
  master: .andromeda/architecture.md:249 «the shared engine boot (`engine_boot::start`, `pulse-app/src/engine_boot.rs`, for both programs since chunk 2026-10-10-console-engine-entry-point; before it, `main.rs`) hands to the baseline state it builds»
  now: The detectors, broadcasts and heartbeats are built by `engine_boot::start` for both programs; "Neither entry point wires an engine part itself" (arch:4). The console program mounts no TauRPC router (arch:288).
  replace-with: «- **Boot wiring:** `pulse-app/src/engine_boot.rs` (`engine_boot::start`, the one shared engine boot both programs call — constructs broadcasts, builds the baseline state, spawns heartbeats; `main.rs` wires no engine part itself and only threads the returned `Arc<dyn Trait>` handles into the window app's resolvers; the console program mounts no resolver)»
```

```
- leaf: .claude/docs/services/interpretation.md:44            class: omission
  leaf-text: «`ANDROMEDA_PULSE_L4_DETERMINISTIC=true` swaps in a canned-`L4Output` runner at boot (`pulse-app/src/deterministic_inference.rs`).»
  master: .andromeda/architecture.md:250 «with TWO readers since chunk 2026-10-10-console-engine-entry-point»
  now: The window app seats the canned runner when truthy, else the llama-cli runner. The console program `andromeda-pulse-engine run` seats the canned runner "at the tier for the unknown hardware profile" when truthy; "UNSET SEATS NOTHING" — no interpretation subscriber, no L4 heartbeat, cues and digests still produced, no incident forms. The arm is reported once per boot on `app.boot.engine`.
  replace-with: «`ANDROMEDA_PULSE_L4_DETERMINISTIC=true` swaps in a canned-`L4Output` runner at boot (`pulse-app/src/deterministic_inference.rs`), with TWO readers: the window app `pulse-app` (truthy seats the canned runner, else the llama-cli runner) and the console program `andromeda-pulse-engine run` (truthy seats the canned runner at the tier for the unknown hardware profile; UNSET SEATS NOTHING — `engine_boot::start` spawns no interpretation subscriber and no L4 heartbeat, cues and digests are still produced, no incident forms). The arm taken is reported once per boot on `app.boot.engine` (`program` · `interpretation` · `reason`), WARN when nothing is seated.»
```

```
- leaf: .claude/docs/services/interpretation.md:27            class: omission
  leaf-text: «The paths come from `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` / `_CPU_BIN_PATH`, consumed in `pulse-app/src/llamacli_inference.rs`.»
  master: .andromeda/security-plan.md:398 «the window app `pulse-app` alone among the three product binaries: the console program `andromeda-pulse-engine` reads none of them and no `ANDROMEDA_PULSE_L4_ALLOW_ROOT`, constructs no `LlamaCliInference` and emits no `interpretation.model.*` record»
  now: The model runner and its three path variables belong to the window program only.
  replace-with: «The paths come from `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` / `_CPU_BIN_PATH`, consumed in `pulse-app/src/llamacli_inference.rs` — by the window app `pulse-app` alone: the console program `andromeda-pulse-engine` reads none of the three L4 path variables and no `ANDROMEDA_PULSE_L4_ALLOW_ROOT`, constructs no `LlamaCliInference` and emits no `interpretation.model.*` record.»
```

```
- leaf: .claude/docs/services/interpretation.md:10            class: omission
  leaf-text: «- `ANDROMEDA_PULSE_HARDWARE_PROFILE` — env override for the detected profile (bounded parse).»
  master: .andromeda/architecture.md:244 «read once per `HardwareProfileDetector::new()` (the boot detector at `pulse-app/src/main.rs`)»
  now: The detector is the window app's. security-plan:138 lists `ANDROMEDA_PULSE_HARDWARE_PROFILE` among the variables the console program does not read. Tree: `console.rs` seats `UnknownHardwareProfile`.
  replace-with: «- `ANDROMEDA_PULSE_HARDWARE_PROFILE` — env override for the detected profile (bounded parse), read once per `HardwareProfileDetector::new()` — the window app's boot detector in `pulse-app/src/main.rs`; the console program `andromeda-pulse-engine` never reads it and runs on the unknown hardware profile.»
```

```
- leaf: .claude/docs/services/ingest.md:51            class: stale
  leaf-text: «- **Run locally:** `cargo run --bin pulse-app` (the receiver crate is library-only; the binary wires it).»
  master: .andromeda/architecture.md:178 «Both engine programs bind these two receivers and nothing else — the window app and the console program `andromeda-pulse-engine`, each through the shared `engine_boot::start`, both defaulting to 4317 / 4318»
  now: Two programs bind the receivers, both through the shared boot; neither entry point wires them itself.
  replace-with: «- **Run locally:** `cargo run --bin pulse-app` (window app) or `cargo run --bin andromeda-pulse-engine -- run` (console engine, no window). The receiver crate is library-only; the shared engine boot `pulse_app::engine_boot::start` wires and binds both receivers for either program, both defaulting to 4317 / 4318.»
```

```
- leaf: .claude/docs/services/viz.md:29            class: omission
  leaf-text: «- **`viz.tick` heartbeat every 15s** with `query_latency_ms`, `subscribers_active`.»
  master: .andromeda/obs-plan.md:101 «`viz.tick` and `plugins.tick` are the window app's alone — the console program `andromeda-pulse-engine` never emits them»
  now: The engine's three ticks (`ingest.tick`, `buffer.tick`, `connection.tick`) come from the shared boot in both programs; `viz.tick` is the window's. obs-plan:537: its absence in the console program "is by design, not a stall". Tree: `heartbeat::spawn_window_ticks`, called from `main.rs:382`.
  replace-with: «- **`viz.tick` heartbeat every 15s** with `query_latency_ms`, `subscribers_active` — the window app's alone (spawned by `heartbeat::spawn_window_ticks`); the console program `andromeda-pulse-engine` never emits it, and its absence there is by design, not a stall.»
```

```
- leaf: .claude/docs/services/plugins.md:4            class: omission
  leaf-text: «Hosts `plugins.{list,reload,invoke}` TauRPC routers.»
  master: .andromeda/security-plan.md:100 «the console program `andromeda-pulse-engine` (… with no window framework, no webview, no TauRPC bridge and no plugin host»
  now: The plugin host exists in the window program only; `plugins.tick` is the window app's alone (obs-plan:101). Tree: `main.rs:244-256` runs `discover_plugins` and builds `PluginsApiImpl`; `engine_boot.rs` names no plugin.
  replace-with: «Hosts `plugins.{list,reload,invoke}` TauRPC routers. Window app only: the console program `andromeda-pulse-engine` has no plugin host and no TauRPC bridge, and never emits `plugins.tick`.»
```

```
- leaf: .claude/docs/services/mcp-server.md:16            class: omission
  leaf-text: «- TauRPC routers (visible from app side): `mcp.status`, `mcp.start`, `mcp.stop`.»
  master: .andromeda/architecture.md:288 «The console program mounts no TauRPC router»
  now: Only the window app can start or stop the sidecar over IPC. The sidecar's cross-process input `run/workspace-key` is "published at boot by either engine program … through `engine_boot::publish_workspace_key_for_sidecar` inside the shared `engine_boot::start`" (arch:226).
  replace-with: «- TauRPC routers (visible from the window app only — the console program `andromeda-pulse-engine` mounts no TauRPC router): `mcp.status`, `mcp.start`, `mcp.stop`. The sidecar's `run/workspace-key` is published at boot by either engine program through the shared `engine_boot::start`.»
```

```
- leaf: .claude/docs/services/ui-bridge.md:9            class: omission
  leaf-text: «- All other library crates expose router modules mounted by `pulse-app` via `ui-bridge`'s contract.»
  master: .andromeda/architecture.md:288 «The console program mounts no TauRPC router: a test of it still injects over real OTLP … and reads state back from the program's own log records»
  now: The bridge is mounted by the window program alone; the console program has "no TauRPC bridge" (security-plan:100). Tree: the one `taurpc::Router` is built in `main.rs:275`.
  replace-with: «- Router modules are mounted by the window app alone (`pulse-app/src/main.rs` builds the one `taurpc::Router`); the console program `andromeda-pulse-engine` mounts no TauRPC router — its state is read from its log records.»
```

```
- leaf: .claude/docs/services/workspace-detector.md:4            class: stale
  leaf-text: «Hosts `workspace.detect` / `workspace.list` TauRPC routers. Used by `snapshot` for snapshot path resolution.»
  master: .andromeda/architecture.md:191 «`workspace.detect` — workspace-detector crate (`workspace.list` deferred — no runtime emitter as of chunk #74; only `workspace.detect` implemented at `crates/ui-bridge/src/workspace_ipc.rs:47`)»
  now: One procedure, implemented in ui-bridge; `workspace.list` is deferred. The crate's boot-time use is the workspace key: "published at boot by either engine program … inside the shared `engine_boot::start`" (arch:226). Tree: `crates/snapshot/Cargo.toml` has no workspace-detector dependency; snapshots go to `data_dir.join("snapshots")`.
  replace-with: «Backs the one implemented procedure `workspace.detect` (resolver at `crates/ui-bridge/src/workspace_ipc.rs`; `workspace.list` is deferred — no runtime emitter). Used by the shared engine boot (`engine_boot::start` detects the host workspace and publishes `run/workspace-key` for both programs) and read cross-process by the `andromeda-pulse-mcp` sidecar.»
```

### Counts, keys, member sets

```
- leaf: .claude/docs/services/buffer.md:26            class: contradicted
  leaf-text: «metric points + log records use `(timestamp, resource_hash, name)`.»
  master: .andromeda/architecture.md:93 «metric points use `(metric_name, ts_unix_nano, resource_hash, seq)`** and **log records use `(ts_unix_nano, resource_hash, severity_number, seq)`»
  now: A LogRecord "carries no spec-defined unique id and no `name` column". `seq` is "the one declared exception" to no-surrogate-keys, an internal per-table ordinal that is never an observable; `metrics_points.labels` is outside the key. Tree: `schema.rs:66,82` agree with the master.
  replace-with: «metric points use `(metric_name, ts_unix_nano, resource_hash, seq)` and log records `(ts_unix_nano, resource_hash, severity_number, seq)` — a LogRecord has no `name` column; `seq` is the one declared surrogate exception (an internal per-table ordinal, one block per batch, never an observable), and `metrics_points.labels` sits outside the key.»
```

```
- leaf: .claude/docs/services/buffer.md:40            class: stale
  leaf-text: «`crates/buffer/src/schema.rs` (DDL for the 7 reserved tables)»
  master: .andromeda/architecture.md:217 «FIVE, down from eight at chunk `2026-08-30-diagnostics-un-muting-harness-truth-sweep`»
  now: Five reserved tables: `spans`, `span_events`, `metrics_points`, `log_records`, `log_templates`. Line 24 of the same leaf already says so.
  replace-with: «`crates/buffer/src/schema.rs` (DDL for the 5 reserved tables)»
```

```
- leaf: .claude/docs/services/corpus.md:4            class: stale
  leaf-text: «Six schema tables per dist-arch v3 for baseline state, service registry, pipeline metrics, incidents, incident events, and digest archive.»
  master: .andromeda/architecture.md:220 «FIVE, down from six at chunk `2026-08-30-diagnostics-un-muting-harness-truth-sweep`: `baseline_state` was DROPPED as dead schema»
  now: Five tables, `SCHEMA_VERSION` 2. Line 26 of the same leaf already says so.
  replace-with: «Five schema tables (SCHEMA_VERSION 2) for service registry, pipeline metrics, incidents, incident events, and digest archive — `baseline_state` was dropped as dead schema.»
```

```
- leaf: .claude/docs/services/corpus.md:27            class: stale
  leaf-text: «first-launch creates all 6 tables; subsequent launches verify version + apply diffs.»
  master: .andromeda/architecture.md:220 «fresh database (0) → create the v2 schema directly; v1 → `DROP TABLE IF EXISTS baseline_state` then stamp 2; a version NEWER than 2 → mismatch error»
  now: A three-rung ladder on `PRAGMA user_version`, five tables.
  replace-with: «a fresh database (0) creates the v2 schema directly (five tables); v1 → `DROP TABLE IF EXISTS baseline_state` then stamp 2; a version newer than 2 → mismatch error.»
```

```
- leaf: .claude/docs/services/corpus.md:14            class: retired
  leaf-text: «- Future write surface (chunks #64 / #66 / #70+): `triage::baseline::persist_state` writes BaselineState snapshots; `triage::pattern::storm` writes fingerprint counts; chunk #70+ incident write surface.»
  master: .andromeda/architecture.md:78 «it is the ONLY choke point all SEVEN production writers traverse: six share `IncidentPersistence` … while the seventh — the cross-process `andromeda-pulse-mcp` sidecar … calls `CorpusWriter` directly»
  now: The write surface exists (`CorpusWriter`, with a monotonic last-writer guard returning `Applied` | `DeclinedStale`). The baseline writer is gone: its "only production writer was retired at chunk #72's file-based `baseline_persistence` migration" (arch:220).
  replace-with: «- Write surface (`CorpusWriter`): incident status writes are arbitrated at the `CorpusWriter` choke point by the monotonic last-writer guard (`Applied` | `DeclinedStale`), which all SEVEN production writers traverse — six through `triage::contract::IncidentPersistence` (the persist cycle, the auto-resolve observer, `incidents.acknowledge`, `incidents.mark_resolved`, both `inference_runtime` L4 writers) and the cross-process `andromeda-pulse-mcp` sidecar calling `CorpusWriter` directly. No baseline snapshot is written here: `baseline_state` is dropped.»
```

```
- leaf: .claude/docs/services/corpus.md:36            class: contradicted
  leaf-text: «no write methods exposed via TauRPC.»
  master: .andromeda/architecture.md:78 «six share `IncidentPersistence` (the persist cycle, the auto-resolve observer, `incidents.acknowledge`, `incidents.mark_resolved`, and both `inference_runtime` L4 writers)»
  now: Two TauRPC procedures write corpus rows. `storage.export_for_training` (arch:195) also sits on `StorageApiImpl` and writes a JSONL export outside the data dir.
  replace-with: «these two expose no write. The corpus IS written over TauRPC elsewhere: `incidents.acknowledge` / `incidents.mark_resolved` reach `CorpusWriter` through `IncidentPersistence`, and `storage.export_for_training` (chunk #95) reads all incidents and writes an anonymized JSONL export outside the data dir.»
```

```
- leaf: .claude/docs/services/corpus.md:28            class: omission
  leaf-text: «- **Encryption posture:** Phase 6 default = cell-level AES-256-GCM with OS-keychain-stored key (alternatives documented: SQLCipher whole-database, age full-file).»
  master: .andromeda/architecture.md:248 «`ANDROMEDA_PULSE_CORPUS_PASSPHRASE` — opt-in fallback passphrase for the corpus AES-256-GCM cell key, used only when the OS credential store is unreachable»
  now: The leaf names no fallback and never names this env var. arch:221: "On a host with neither a credential store nor a configured passphrase the corpus degrades to absent rather than inventing a key."
  replace-with: «- **Encryption posture:** cell-level AES-256-GCM with the key in the OS credential store, primary; opt-in fallback `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` (BLAKE3-derived with the service id folded in, nothing written to disk, once-per-boot WARN; engages only when configured, so a transient store failure never silently switches keys). With neither a store nor a passphrase the corpus degrades to absent, never an invented key (alternatives documented: SQLCipher whole-database, age full-file).»
```

```
- leaf: .claude/docs/services/corpus.md:13            class: stale
  leaf-text: «(chunk #68 quadruple binding: router + capability JSON + EXPECTED_PROCEDURES + emit_taurpc_bindings test)»
  master: .andromeda/architecture.md:286 «`pulse-app/capabilities/` holds **no per-procedure entries** … Adding a TauRPC procedure therefore requires a router registration in the owning crate, an entry in Occupied Resources, and its `EXPECTED_PROCEDURES` pin … **not** a capability-JSON edit»
  now: No capability-JSON leg exists for a procedure. `ui-bridge.md:31` already says this.
  replace-with: «(binding: router registration + Occupied Resources entry + `EXPECTED_PROCEDURES` pin, staged together with the regenerated `bindings/index.ts`; no per-procedure capability-JSON entry exists)»
```

```
- leaf: .claude/docs/services/triage.md:11            class: contradicted
  leaf-text: «- `corpus::CorpusReader` for baseline-state bootstrap on cold start + persistence on tick (chunks #61 / #64 / #66 / etc. write through corpus).»
  master: .andromeda/architecture.md:78 «`triage` sits below `corpus` in the dependency DAG, so the `pulse-app` adapter maps between the two»
  now: `triage` cannot consume a `corpus` type. `baseline_state` was dropped, its "only production writer … retired at chunk #72's file-based `baseline_persistence` migration" (arch:220).
  replace-with: «- Persistence ports declared in `triage::contract` and implemented at the `pulse-app` binary boundary over `CorpusWriter` (`IncidentPersistence`, `DurableActiveIncidents`, the lifecycle and storm persistence traits) — `triage` sits below `corpus` in the DAG and names no `corpus` type. Baseline state is not in the corpus: `baseline_state` was dropped after its only writer moved to the file-based `baseline_persistence` at chunk #72.»
```

```
- leaf: .claude/docs/services/triage.md:30            class: stale
  leaf-text: «  - `incident/` — empty skeleton (future chunks)»
  master: .andromeda/architecture.md:202 «`IncidentsApiImpl` resolver backed by `crates/triage::contract::IncidentRegistry` + `IncidentPersistence` + `IncidentLifecycleBroadcast`; chunk #78»
  now: The incident module is built (also `tier_effective_at`, arch:186), and a `cadence` module exists (arch:207, `crates/triage/src/cadence/broadcast.rs:9`). So line 24's "7 sub-modules" is eight, and line 29's `interpretation/` is the file `crates/triage/src/interpretation.rs`, still a skeleton. Tree: `incident/{broadcast,persistence,registry,state_machine,tier_effective}.rs`, `cadence/{broadcast,config,coordinator}.rs`.
  replace-with: «  - `incident/` — incident registry + status state machine + persistence port + lifecycle broadcast + `tier_effective_at` (chunk #78 onward)
  - `cadence/` — the cadence coordinator, its config and the `pulse://stream/cadence-events` broadcast (chunk #80)»
  (and line 24 «7 sub-modules per chunk #60 scaffold» → «8 sub-modules»; line 29 «`interpretation/`» → «`interpretation.rs`»)
```

```
- leaf: .claude/docs/services/triage.md:39            class: stale
  leaf-text: «Coordinate cadence per Cadence Coordinator design (chunk #72 future scope).»
  master: .andromeda/architecture.md:207 «`pulse://stream/cadence-events` (chunk #80 — cadence coordinator L6-visibility topic, `crates/triage/src/cadence/broadcast.rs:9`)»
  now: The coordinator is built; all of these loops are spawned by the shared engine boot.
  replace-with: «All are spawned from the shared engine boot `engine_boot::start` (both programs). The cadence coordinator is built (chunk #80, `crates/triage/src/cadence/`, topic `pulse://stream/cadence-events`).»
```

```
- leaf: .claude/docs/services/snapshot.md:4            class: stale
  leaf-text: «Hosts `snapshot.{generate,list_recent,copy_to_clipboard}` TauRPC routers.»
  master: .andromeda/architecture.md:187 «(`snapshot.list_recent` and `snapshot.copy_to_clipboard` deferred — no runtime emitter as of chunk #74; only `snapshot.generate` implemented at `pulse-app/src/snapshot_runtime.rs`)»
  now: One procedure, hosted at the binary boundary. The same correction applies to line 14.
  replace-with: «The one implemented TauRPC procedure is `snapshot.generate`, hosted at the binary boundary in `pulse-app/src/snapshot_runtime.rs` (window app only — the console program mounts no TauRPC router); `snapshot.list_recent` and `snapshot.copy_to_clipboard` are deferred, with no runtime emitter.»
```

```
- leaf: .claude/docs/services/snapshot.md:14            class: stale
  leaf-text: «- TauRPC routers: `snapshot.generate`, `snapshot.list_recent`, `snapshot.copy_to_clipboard`.»
  master: .andromeda/architecture.md:187 «only `snapshot.generate` implemented at `pulse-app/src/snapshot_runtime.rs`»
  now: as above.
  replace-with: «- TauRPC procedure: `snapshot.generate` (`pulse-app/src/snapshot_runtime.rs`); `snapshot.list_recent` and `snapshot.copy_to_clipboard` deferred — no runtime emitter.»
```

```
- leaf: .claude/docs/services/snapshot.md:11            class: contradicted
  leaf-text: «- `workspace-detector` for snapshot path resolution (`.andromeda/` marker → `.andromeda/pulse/{timestamp}.md`; else → `~/.cache/andromeda-pulse/snapshots/{timestamp}.md`).»
  master: .andromeda/architecture.md:226 «Subpaths under the resolved root: `config.toml` (user settings), `plugins/` …, `snapshots/` (generated snapshot markdown files)»
  now: Snapshots live under the data dir root, which `ANDROMEDA_PULSE_DATA_DIR` overrides. The master names neither `.andromeda/pulse/` nor `~/.cache/`. Tree: `snapshot_runtime.rs:228` is `data_dir.join("snapshots")`; the crate has no workspace-detector dependency. Line 59 repeats the `.andromeda/pulse/` claim.
  replace-with: «- Snapshot files land in `snapshots/` under the resolved data dir (`~/.andromeda-pulse/snapshots/` on Linux; `ANDROMEDA_PULSE_DATA_DIR` overrides the root), resolved in `pulse-app/src/snapshot_runtime.rs` — no `workspace-detector` call, no `.andromeda/pulse/` and no `~/.cache/` location.»
```

```
- leaf: .claude/docs/services/ui-bridge.md:16            class: stale
  leaf-text: «  - Routers (mounted from sibling crates): `traces.*`, `metrics.*`, `logs.*`, `snapshot.*`, `plugins.*`, `mcp.*`, `workspace.*`, `telemetry.frontend.*`»
  master: .andromeda/architecture.md:184 «`streams.subscribe_spans`, `streams.subscribe_metrics`, `streams.subscribe_logs` — pulse-app crate»
  now: arch:181-204 also lists `streams.*`, `connection.*`, `services.*`, `storage.*`, `diagnostics.*`, `config.*`, `incidents.*`, `model.*`, `investigate.*`; `EXPECTED_PROCEDURES` stands at 44 (arch:188). `telemetry.frontend.*` (six methods) and `workspace.detect` are ui-bridge's own (`telemetry.rs`, `workspace_ipc.rs`), not a sibling's.
  replace-with: «  - Own routers: `telemetry.frontend.*` (six methods, `telemetry.rs`), `workspace.detect` (`workspace_ipc.rs`)
  - Routers mounted beside them by the window app: `traces.*`, `metrics.*`, `logs.*`, `streams.*`, `snapshot.generate`, `plugins.*`, `mcp.*` (feature-gated), `connection.*`, `services.*`, `storage.*`, `diagnostics.*`, `config.*`, `incidents.*`, `model.*`, `investigate.*` — Occupied Resources is the canonical list (`EXPECTED_PROCEDURES` 44)»
```

```
- leaf: .claude/docs/services/ui-bridge.md:17            class: stale
  leaf-text: «- Auto-generated TypeScript bindings: `pulse-app/ui/src/bindings/*.d.ts` (per crate router).»
  master: .andromeda/architecture.md:286 «the git-INDEX copy of `pulse-app/ui/src/bindings/index.ts`»
  now: One tracked bindings file. Tree: the directory holds `index.ts` and `bindings.test.ts`, no `.d.ts`.
  replace-with: «- Auto-generated TypeScript bindings: `pulse-app/ui/src/bindings/index.ts` (one tracked file for the whole procedure surface).»
```

```
- leaf: .claude/docs/services/ui-bridge.md:42            class: contradicted
  leaf-text: «- **TauRPC type generation:** `pulse-app/build.rs` invokes taurpc generation for each router crate»
  master: .andromeda/architecture.md:268 «The `test` stage rewrites the tracked `pulse-app/ui/src/bindings/index.ts`»
  now: Bindings are emitted by a test run, not by the build. Tree: `pulse-app/build.rs` is `tauri_build::build()` only; the emitter is the test `emit_taurpc_bindings` (`main.rs:417`). Line 52 makes the same `cargo build` claim.
  replace-with: «- **TauRPC type generation:** the `emit_taurpc_bindings` test in `pulse-app/src/main.rs` rewrites the tracked `pulse-app/ui/src/bindings/index.ts` when the workspace tests run (`pulse-app/build.rs` is `tauri_build::build()` only)»
```

```
- leaf: .claude/docs/services/ui-bridge.md:43            class: stale
  leaf-text: «- **Capability JSON:** `pulse-app/capabilities/{pulse-default,pulse-tray,pulse-notification,pulse-updater,pulse-plugin-fs}.json`»
  master: .andromeda/architecture.md:269 «`pulse:default`, `pulse:tray`, `pulse:notification`, `pulse:updater`, `pulse:plugin-fs`, `pulse:clipboard` — concrete capability JSON files live in `pulse-app/capabilities/`»
  now: Six identifiers and "6 capability files" (arch:268). Tree: `clipboard.json default.json notification.json plugin-fs.json tray.json updater.json` — no `pulse-` prefix.
  replace-with: «- **Capability JSON:** `pulse-app/capabilities/{default,tray,notification,updater,plugin-fs,clipboard}.json` (six files; identifiers `pulse:default` … `pulse:clipboard`)»
```

```
- leaf: .claude/docs/services/viz.md:14            class: stale
  leaf-text: «- Tauri Channel API: `pulse://stream/spans`, `pulse://stream/metrics`, `pulse://stream/logs` (binary Arrow IPC payloads).»
  master: .andromeda/architecture.md:184 «`streams.subscribe_spans`, `streams.subscribe_metrics`, `streams.subscribe_logs` — pulse-app crate (Tauri Channel<Vec<u8>> binding to `buffer::BroadcastSenders` for binary Arrow IPC; chunk #23)»
  now: The channel binding belongs to the `pulse-app` crate, not viz. Tree: `pulse-app/src/streams.rs`; `crates/viz/Cargo.toml` has no `tauri`, `taurpc` or `arrow` dependency. Line 4's "emits real-time push streams" carries the same owner error.
  replace-with: «- The push streams `pulse://stream/spans` / `metrics` / `logs` (binary Arrow IPC) are bound by the `streams.subscribe_{spans,metrics,logs}` procedures of the `pulse-app` crate (`Channel<Vec<u8>>` over `buffer::BroadcastSenders`, `pulse-app/src/streams.rs`), not by this crate.»
```

```
- leaf: .claude/docs/services/plugins.md:28            class: contradicted
  leaf-text: «  - `max_wasm_http_fields_size` — set per April 2026 CVE-2026-27572»
  master: .andromeda/security-plan.md:199 «wasi-http header fields bounded by the `MAX_WASM_HTTP_FIELDS_SIZE_BYTES` const in `crates/plugins/src/engine.rs` per April 2026 CVE-2026-27572 (NOT a `wasmtime::Config` method — enforcement attaches via the wasi-http context, `WasiHttpCtxBuilder::max_field_size`, when wasi-http imports land at chunk #46+)»
  now: The leaf lists it under "`wasmtime::Config` settings (binding)"; the master says it is not a `Config` method. Line 50 of the same leaf already names the const.
  replace-with: «  - wasi-http header field size — bounded by the `MAX_WASM_HTTP_FIELDS_SIZE_BYTES` const in `engine.rs` per April 2026 CVE-2026-27572; NOT a `wasmtime::Config` method — enforcement attaches via the wasi-http context (`WasiHttpCtxBuilder::max_field_size`) when wasi-http imports land»
```

```
- leaf: .claude/docs/services/plugins.md:14            class: stale
  leaf-text: «- Tauri Channel: `pulse://stream/plugin-events` (plugin lifecycle events).»
  master: .andromeda/architecture.md:207 «`pulse://stream/plugin-events` (deferred — no runtime emitter as of chunk #74; pending plugin invocation telemetry chunk)»
  now: The topic is reserved; nothing emits it.
  replace-with: «- Tauri Channel: `pulse://stream/plugin-events` — reserved, deferred (no runtime emitter).»
```

```
- leaf: .claude/docs/services/workspace-detector.md:13            class: stale
  leaf-text: «- TauRPC routers: `workspace.detect`, `workspace.list`.»
  master: .andromeda/architecture.md:191 «`workspace.list` deferred — no runtime emitter as of chunk #74; only `workspace.detect` implemented at `crates/ui-bridge/src/workspace_ipc.rs:47`»
  now: as quoted.
  replace-with: «- TauRPC procedure: `workspace.detect` (resolver in `crates/ui-bridge/src/workspace_ipc.rs`); `workspace.list` deferred — no runtime emitter.»
```

### Paths the tree contradicts

```
- leaf: .claude/docs/services/mcp-server.md:46            class: stale (tree)
  leaf-text: «- **TauRPC router:** `crates/mcp-server/src/router.rs` (mcp.status / mcp.start / mcp.stop visible from main app)»
  master: tree — `crates/mcp-server/src/` holds `contract.rs feature_gate.rs jsonrpc.rs lib.rs tools.rs tracing_setup.rs bin/`; `pulse-app/src/mcp_router.rs` exists (`#[cfg(feature = "mcp-server")] pub mod mcp_router`, `McpApiImpl` built in `main.rs:268`). arch:190 still attributes `mcp.*` to the mcp-server crate.
  now: The router lives at the binary boundary and is mounted by the window app only. Line 4's "Hosts `mcp.{status,start,stop}` TauRPC routers" has the same owner error.
  replace-with: «- **TauRPC router:** `pulse-app/src/mcp_router.rs` (`McpApiImpl`: mcp.status / mcp.start / mcp.stop; compiled only with `--features mcp-server`, mounted by the window app's `main.rs`)»
```

```
- leaf: .claude/docs/services/workspace-detector.md:47            class: stale (tree)
  leaf-text: «- **TauRPC router:** `crates/workspace-detector/src/router.rs`»
  master: .andromeda/architecture.md:191 «only `workspace.detect` implemented at `crates/ui-bridge/src/workspace_ipc.rs:47`»
  now: No `router.rs`; the crate holds `contract.rs detect.rs lib.rs marker.rs vcs.rs`.
  replace-with: «- **TauRPC resolver:** `crates/ui-bridge/src/workspace_ipc.rs` (`WorkspaceApiImpl`, `workspace.detect`)»
```

```
- leaf: .claude/docs/services/snapshot.md:50            class: stale (tree)
  leaf-text: «- **TauRPC router:** `crates/snapshot/src/router.rs`»
  master: .andromeda/architecture.md:187 «only `snapshot.generate` implemented at `pulse-app/src/snapshot_runtime.rs`»
  now: `crates/snapshot/src/` holds only `attribute_filter.rs contract.rs lib.rs markdown.rs token_budget.rs`. Every path on lines 44-51 is absent.
  replace-with: «- **TauRPC resolver + runtime:** `pulse-app/src/snapshot_runtime.rs` (`SnapshotApiImpl`, `snapshot.generate`)»
```

```
- leaf: .claude/docs/services/viz.md:37            class: stale (tree)
  leaf-text: «- **Query routers:** `crates/viz/src/{traces,metrics,logs}.rs` (TauRPC procedure handlers)»
  master: tree — `crates/viz/src/` holds `contract.rs lib.rs query.rs state.rs`; `main.rs:35,278-280` takes `TracesApiImpl` / `MetricsApiImpl` / `LogsApiImpl` from `pulse_app::viz_routers`. arch:183 still attributes the routers to the viz crate.
  now: Handlers sit at the binary boundary; the crate holds the query layer and `VizState`.
  replace-with: «- **Query routers:** `pulse-app/src/viz_routers.rs` (`TracesApiImpl` / `MetricsApiImpl` / `LogsApiImpl`); the query layer they call is `crates/viz/src/query.rs` + `state.rs` (`VizState`)»
```

## Further findings, one line each

- `config-watcher.md:35` (and `:24`) — stale (tree) — `crates/config-watcher/src/contract.rs` does not exist; the public surface is `lib.rs` `pub use` re-exports of the private `event` / `partition` / `watcher` modules. arch:299 still describes the contract-module shape for every crate.
- `config-watcher.md:34` — stale (tree) — "`crates/config-watcher/src/` (`partition_changed_keys`)" → `crates/config-watcher/src/partition.rs`.
- `triage.md:43` — stale (tree) — `{baseline,pattern,cue,digest,interpretation,incident,lifecycle}/` → `{baseline,cadence,cue,digest,incident,lifecycle,pattern}/` plus the file `interpretation.rs`.
- `triage.md:14-16` — omission — the Publishes list stops at chunk #67; arch:207 also lists `pulse://stream/incidents` (#78, producer-only), `pulse://stream/cadence-events` (#80) and `pulse://stream/digests` (#81).
- `ingest.md:30` — omission — arch:178: the engine config holds two validated ports and `engine_boot::start` builds `127.0.0.1:{port}` itself; "a rejected port variable records the bind as failed (`reason = "invalid_port"`) and starts no receiver", never a default.
- `ingest.md:42` — stale (tree) — `crates/ingest/src/pipeline.rs` does not exist; the mpsc hand-off is `crates/ingest/src/channel.rs` (`build_channel`, `IngestSender`).
- `ingest.md:43` — stale (tree) — tests are not only colocated, and `pipeline.rs` is absent: `crates/ingest/tests/{backpressure,grpc_loopback,http_loopback,invariants,proptest_invariants,rate_limit}.rs` exist.
- `ingest.md:47` — stale (tree) — `MockTraceSpan::builder()` has 0 hits under `crates`, `pulse-app/src`, `pulse-app/tests`.
- `ingest.md:13` — stale (tree) — `OtlpBatch` has 0 hits in `crates/ingest/src` and `crates/buffer/src`.
- `buffer.md:44` — stale (tree) — `crates/buffer/src/conn.rs` does not exist; the `:memory:` connection is opened by the shared boot (`pulse-app/src/engine_boot.rs:273`), and shared state is `crates/buffer/src/state.rs`.
- `buffer.md:29` — stale — the leaf writes `INTERVAL '<retention> seconds'`, arch:92 writes `INTERVAL '<retention> minutes'` "sweeping only live tables". The code is neither: four prepared `DELETE FROM {table} WHERE ts_unix_nano < ?` (`retention.rs:22-26`), `log_templates` not swept.
- `corpus.md:24` — stale — "`schema.rs` (DDL for 6 tables …)" → 5 tables + the v1→v2 ladder; the layout also omits `disposition.rs` (arch:221, the boot-time inventory-then-purge of orphaned content).
- `corpus.md:18` — omission — arch:37: `keyring` 3.x must carry the explicit platform feature set (`apple-native` / `windows-native` / `sync-secret-service` / `crypto-rust`); a bare `keyring = "3"` "links no backend and silently yields a per-process key".
- `corpus.md:35` — omission — "no parallel writers" holds for one process only; arch:78 names the `andromeda-pulse-mcp` sidecar as a cross-process writer of the same file, and arch:229 has the window app, the console program and the sidecar all opening it through `Corpus::open`.
- `snapshot.md:59` — contradicted — repeats `.andromeda/pulse/{timestamp}.md`; see the `:11` finding.
- `snapshot.md:44-49,51` — stale (tree) — `pipeline.rs`, `curate/{dedup,anomaly,critical_path,aggregate}.rs`, `format.rs`, `budget.rs`, `path.rs`, `io.rs`, `tests/integration/snapshot/` are all absent. On disk: `crates/snapshot/src/{markdown,token_budget,attribute_filter,contract}.rs`; curation primitives in `crates/curation/src/{dedupe,anomaly,critical_path,aggregation}.rs`; tests `pulse-app/tests/{e2e_p2_snapshot_generate,unit_snapshot_runtime}.rs`. I did not check which function orchestrates the pipeline.
- `snapshot.md:20-25` — stale (tree) — `crates/snapshot/Cargo.toml` depends on `chrono`, `curation`, `serde`, `thiserror`, `tracing` only; no `duckdb`, `arrow`, `tauri`, `tauri-plugin-notification` or `tiktoken-rs`.
- `snapshot.md:10` — stale (tree) — "`viz` crate aggregation utilities": the crate has no `viz` dependency; percentiles come from `curation`.
- `ui-bridge.md:52` — contradicted — "`cargo build` triggers taurpc codegen"; see the `:42` finding.
- `ui-bridge.md:40` — stale (tree) — `crates/ui-bridge/src/error.rs` does not exist; `AppError` is at `crates/ui-bridge/src/contract.rs:16`.
- `ui-bridge.md:41` — stale (tree) — `{app_info,health,ready,settings}.rs`: only `health.rs` exists; the crate holds `contract.rs health.rs lib.rs telemetry.rs workspace_ipc.rs`.
- `ui-bridge.md:10` — stale — `health` subsystems per arch:121-126 are `otlp_grpc_receiver`, `otlp_http_receiver`, `buffer`, `ingest_channel`, not viz / plugins / mcp-server.
- `ui-bridge.md:11` — stale — `ready` has eight checks per arch:139-146; the leaf's five lack `rows_ingested`, `buffer_used_seconds`, `retention_seconds`.
- `viz.md:38-39` — stale (tree) — `crates/viz/src/aggregator.rs` and `stream.rs` do not exist; queries are in `query.rs`, the channel pipeline in `pulse-app/src/streams.rs`.
- `viz.md:19-21` — stale (tree) — `crates/viz/Cargo.toml` has no `arrow`, `tauri` or `taurpc` dependency (it has `duckdb`, `specta`, `tokio`, `chrono`, `serde`, `tracing`).
- `plugins.md:18` — omission — "`wasmtime` 25+" omits the pin in arch:22: requirement `"48.0.4"`, lockfile-resolved 48.0.5, 49.x out of reach while the toolchain is pinned at 1.95.
- `mcp-server.md:47` — stale (tree) — `tests/integration/mcp/` does not exist (no root `tests/`); the tests are `crates/mcp-server/tests/{sidecar_subprocess,incident_events_subprocess}.rs` and `pulse-app/tests/e2e_p3_mcp_*.rs`.
- `mcp-server.md:4` — stale (tree) — "Hosts `mcp.{status,start,stop}` TauRPC routers": hosted in `pulse-app/src/mcp_router.rs`; see the `:46` finding.
- `workspace-detector.md:48,56` — stale (tree) — `tests/fixtures/workspaces/` exists nowhere in the repository; the tests are colocated and build `TempDir` layouts.
- `workspace-detector.md:18` — stale (tree) — `git2` is not a dependency (`thiserror`, `serde`, `tracing` only).
- `workspace-detector.md:10,24` — stale (tree) — no `git rev-parse --show-toplevel` shell-out; `vcs.rs` walks ancestors for `.git/HEAD`.

## Master-vs-tree divergences (not leaf findings; for the orchestrator)

- arch:183 / :189 / :190 attribute `traces.*`, `plugins.*` and `mcp.*` to the viz, plugins and mcp-server crates; the handlers are in `pulse-app/src/{viz_routers,plugins_router,mcp_router}.rs`.
- arch:92 writes the retention sweep as `INTERVAL '<retention> minutes'`; the code runs four prepared `ts_unix_nano < ?` deletes.
- arch:221 names the Windows store "Windows Credential Manager"; security-plan:276 says `WindowsDpapi` → Windows DPAPI. `corpus.md:10` follows the security plan, so I left it.
- arch:4 says the shared boot wires all fourteen library crates and neither entry point wires an engine part. `main.rs` itself runs `discover_plugins`, builds `VizState` and mounts every router — the window-only parts, consistent with security-plan:100, but arch:4 reads as broader than that.
- arch:299 says every library crate exposes a `contract` module; `config-watcher` has none.

## Coverage

- buffer.md: generated body read whole (60 lines) · findings 4
- config-watcher.md: generated body read whole (47 lines) · findings 4
- corpus.md: generated body read whole (45 lines) · findings 9
- curation.md: generated body read whole (32 lines) · findings 0
- ingest.md: generated body read whole (59 lines) · findings 5
- interpretation.md: generated body read whole (59 lines) · findings 3
- mcp-server.md: generated body read whole (63 lines) · findings 4
- plugins.md: generated body read whole (73 lines) · findings 4
- security.md: generated body read whole (55 lines) · findings 0
- snapshot.md: generated body read whole (68 lines) · findings 8
- triage.md: generated body read whole (46 lines) · findings 6
- ui-bridge.md: generated body read whole (59 lines) · findings 10
- viz.md: generated body read whole (55 lines) · findings 6
- workspace-detector.md: generated body read whole (62 lines) · findings 6

- .andromeda/architecture.md: read whole · 328 lines · long lines read by window: yes (35, 47, 76, 77, 78, 93, 186, 220, 221, 226, 229, 250, 268 in three windows, 286)
- .andromeda/registries/contracts/architecture/project-directory-structure.md: read whole · 59 lines · long lines read by window: no (none)
- .andromeda/registries/contracts/architecture/build-system.md: read whole · 5 lines · long lines read by window: no (none)
- .andromeda/registries/contracts/architecture/deployment-model.md: read whole · 4 lines · long lines read by window: no (none)

Not read whole:
- `.andromeda/security-plan.md` (475 lines) — per the assignment, located rules only. Read whole: lines 14, 72, 84, 100, 126, 133, 137, 139, 191, 199, 200, 276, 352, 404, 432, 434, 436, 438, 440. Lines 138 and 398 were read only as excerpts around `engine_boot`.
- `.andromeda/obs-plan.md` (662 lines) — located rules only: lines 101, 127, 180, 330, 537, 559 read whole.
- `.andromeda/registries/contracts/architecture/ci-cd-approach.md` — not read (no service note restates a CI rule).
- `.andromeda/test-plan.md` — not in my assignment, so leaf claims citing it are unjudged: the `cargo tree -p wasmtime | grep -q cranelift` check (`plugins.md:47,63`) and the test-plan section references.

Left unjudged for lack of a master statement:
- `security.md:15`, `corpus.md:9`, `triage.md:21` — the "subscriber Layer scrubbing deferred to chunk #70+" notes. security-plan:434 names the log sink in the intended posture, but I found no line that settles whether it is built.
- `security.md:55`, `corpus.md:45` — the dated test counts (85 and 88).
- `workspace-detector.md:28-37` — the `Workspace` struct block; no master defines it, and my grep for `pub struct Workspace` in the crate found nothing.

Nothing was edited.
~~~
