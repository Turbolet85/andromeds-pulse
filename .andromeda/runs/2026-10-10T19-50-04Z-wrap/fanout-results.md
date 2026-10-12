# Fan-out results — 2026-10-10-engine-harness-proofs-completed

Seven doc-agents, one batch. Returns copied by script from each agent's transcript (entity-decode probe: `entities=0` on all seven, no return changed by decoding). Proposals are numbered in the order returned.

## architecture
verdict: 24 proposal(s) · stripped: a five-line comment preamble (what was read; no key-file proposal; D-arch-decisions: one §Stack row incomplete; a note for the citation sweep on `scripts/agent-run.sh:310`, which the sweep had already re-pointed)

```yaml
proposals:
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → xtask CLI surfaces → `cargo xtask harness:ready` (the contract sentence: `ready` requires … / `otlp_grpc` / `otlp_http` read …)"
    change: >-
      `ready` requires the `harness:status` verdict `running-healthy` AND both receivers reading `accepting`; `otlp_grpc` / `otlp_http` read one of three closed labels `accepting` | `foreign` | `refusing` (was two; `null` on `cannot-evaluate`) — `accepting`: a loopback connection accepted on the resolved port AND the log family holding, after its last `app.boot.engine` record, an INFO record of that receiver's bind target (`app.boot.otlp.grpc.bind` / `app.boot.otlp.http.bind`) whose `bind_address` is `127.0.0.1:` and that port; `foreign`: the connection accepted and no such record (a failed-bind ERROR record, a record for another port or for the other receiver, a record before the last boot record, a log with no boot record); `refusing`: no connection accepted. The verb probes both ports, then reads the log family once and takes the program from `harness_status::program_of`. Verdict words, exits and the six members unchanged; holds for both programs; it still logs nothing and occupies no port.
    sidecar: "2026-10-10-engine-harness-proofs-completed: `harness:ready` reads `ready` only over the run's own bound records; receiver labels 2 → 3 (`foreign`)."
    rationale: >-
      Report Changes → Symbols / APIs ("The readiness decision, as built") and Schema / config ("two receiver members take one of three closed labels … was two"); Counts: receiver labels of `harness:ready` 2 → 3. Architecture still says `ready` needs only "a TCP connection accepted on `127.0.0.1` at each port" and that the members read `accepting` | `refusing`. Note for the applier: the same entry says the verb "reads the status through `harness_status::for_program`"; the report names `harness_status::program_of` as where the program now comes from and does not say whether `for_program` is still called, so that clause is to be re-read, not assumed.
    basis: "xtask/src/harness_ready.rs:172-227"
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Filesystem locations → Subpaths under the resolved root → `run/andromeda-pulse.pid` (the clause: the boot poll through `harness:ready`, which holds that verdict plus an accepted connection on both OTLP ports)"
    change: >-
      the boot poll through `harness:ready`, which holds that verdict plus both receivers reading `accepting` — a connection accepted on each OTLP port AND this run's own bound record for that port in the log family; a port another listener holds reads `foreign` and is not ready.
    sidecar: "2026-10-10-engine-harness-proofs-completed: pid-file bullet restates the `harness:ready` decision with the bound-record condition."
    rationale: >-
      Second occurrence of the retired claim "ready = status verdict + accepted connection on both ports". Report Changes → Symbols / APIs: a receiver reads `accepting` only with the INFO bind record after the last `app.boot.engine` record; "An accepted loopback connection does not say whose listener accepted it" is what the chunk closed.
    basis: "xtask/src/harness_ready.rs:200-227"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → xtask CLI surfaces → `scripts/agent-run.{sh,ps1}` entry (the sentence: so it reports ready only on the `running-healthy` status verdict for the program asked plus an accepted connection on both OTLP ports)"
    change: >-
      so it reports ready only on the `running-healthy` status verdict for the program asked plus both receivers reading `accepting` (a connection accepted on each OTLP port and this run's own bound record for it in the log family; `harness:ready`'s entry, above).
    sidecar: "2026-10-10-engine-harness-proofs-completed: `boot` readiness sentence follows the `harness:ready` decision as built."
    rationale: >-
      Third occurrence of the same retired claim, in the script's own entry. Report Changes → Harness / gate surface: "`cargo xtask harness:ready` — the decision above; one more label. Both scripts call it".
    basis: "xtask/src/harness_ready.rs:172-227"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Environment variables → `ANDROMEDA_PULSE_LOGFILE` (the clause: the same family yields the `program` member of both verdicts (its last `app.boot.engine` record, `read_program`))"
    change: >-
      the same family yields the `program` member of both verdicts from its last `app.boot.engine` record — `harness:status` through `harness_status::read_program` (its one caller, `harness_status::run`), `harness:ready` through `harness_status::program_of` over the one read of the family it makes itself — and, since chunk 2026-10-10-engine-harness-proofs-completed, `harness:ready` also reads from it the two bind targets' records (`app.boot.otlp.grpc.bind`, `app.boot.otlp.http.bind`) that stand after that boot record.
    sidecar: "2026-10-10-engine-harness-proofs-completed: log-family readers: `harness:ready` reads the bind records; `read_program` is `harness:status`'s alone."
    rationale: >-
      Report Changes → Symbols / APIs: "`run_ready` probes both ports, then reads the log family once (`smoke::read_jsonl_lines`) and takes the program from `harness_status::program_of`; `harness_status::read_program` keeps its one caller, `harness_status::run`." The registry line still attributes both verdicts' `program` to `read_program` and names no read of bind records.
    basis: "xtask/src/harness_ready.rs:200-227"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → xtask CLI surfaces → `scripts/agent-run.{sh,ps1}` entry (the sentence: On a failed readiness poll `boot` reports how the app ended — … followed, only when the last readiness verdict held a `refusing` label, by …)"
    change: >-
      … followed, when the last readiness verdict held a `refusing` label, by `the receivers never both accepted: OTLP gRPC 127.0.0.1:{port}, OTLP HTTP 127.0.0.1:{port}`, and, when it held `foreign`, by `  a receiver port accepts but this run's log holds no bound record for it: OTLP gRPC 127.0.0.1:{port}, OTLP HTTP 127.0.0.1:{port}` (sh only, since chunk 2026-10-10-engine-harness-proofs-completed; both lines can print) — and runs `bash "$0" cleanup`; `agent-run.ps1 boot` is not edited: it mirrors the poll and the `refusing` line alone through `Invoke-Ready`, and prints no `foreign` line.
    sidecar: "2026-10-10-engine-harness-proofs-completed: `agent-run.sh boot` gains the `foreign` failure line; the `.ps1` matches `refusing` alone."
    rationale: >-
      Report Changes → Symbols / APIs (`scripts/agent-run.sh` 196-200: "a second `case` in the readiness-failure branch … The `refusing` line beside it is unchanged; both can print") and Harness / gate surface ("`agent-run.ps1` is not edited and its readiness branch still matches `refusing` alone"). The new failure line is unregistered, and "mirrors the poll and the line" now overstates the `.ps1`.
    basis: "scripts/agent-run.sh:196-200"
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → xtask CLI surfaces → `cargo xtask harness:engine-cycle` (the sentence: Refused before anything starts — … on `not-linux` · `shared-port` … `injector-build-failed`)"
    change: >-
      The refusal list holds eight closed labels (was seven): add `port-taken` — either cycle port already accepting a loopback connection — read in `drive` after the existing refusal and before the `HOME` read, so 4317 and 4318 are refused (`shared-port`) before any port is probed; stderr `harness:engine-cycle: cannot-evaluate (port-taken)`, the verdict object with nulls, exit 2, nothing built or started, no data dir made (chunk 2026-10-10-engine-harness-proofs-completed). The verb gained no flag.
    sidecar: "2026-10-10-engine-harness-proofs-completed: `harness:engine-cycle` refusal labels 7 → 8 (`port-taken`)."
    rationale: >-
      Report Changes → Symbols / APIs (`fn taken_port_refusal` 122-127, called in `drive` 243-250; "New closed label `port-taken`") and Counts ("refusal labels of `harness:engine-cycle`: 7 → 8"); Reverted / negative API facts: "`harness:engine-cycle` gained no flag". The registered list ends at `injector-build-failed`.
    basis: "xtask/src/engine_cycle.rs:122-127"
  - detector: D-arch-resources
    severity: escalate
    section: "§Occupied Resources → xtask CLI surfaces → `cargo xtask pre-push:linux` (the sentence: It binds no port and starts no `pulse-app`; …)"
    change: >-
      Replace the no-port sentence: since chunk 2026-10-10-engine-harness-proofs-completed stage six (`ci-gates`) starts the console engine `andromeda-pulse-engine` and binds loopback 24317 and 24318 — in order: the stage data dir removed and created again; either cycle port already accepting stops the run `cannot-evaluate` / `engine-port-taken` with nothing started; `cargo xtask harness:engine-cycle --data-dir {that dir}` under the stage child set; then both ports and the pid of that data dir's pid file are read — one still accepting or alive runs the script's `cleanup` once on that data dir and the stage is red (`stage-failed:ci-gates`) whatever the cycle returned; else cycle exit 2 → `cannot-evaluate` / `engine-cycle-unevaluated`; exit 0 → `cargo xtask ci-gates` over the same data dir decides the stage; any other exit → red. Stage names, order and the six verdict members unchanged; a stage that stops `cannot-evaluate` is listed `"ok": false`; the product ports 4317 / 4318 are never touched. This is the operator's reading (the pc overseer, 2026-10-10), PROVISIONAL — the founder's own word is owed (Boundary widening).
    sidecar: "2026-10-10-engine-harness-proofs-completed: `pre-push:linux` stage six runs a real console engine cycle and binds 24317 / 24318 — operator's reading, PROVISIONAL, founder's word owed."
    rationale: >-
      Report Changes → Harness / gate surface: "**The check now starts `andromeda-pulse-engine` and binds loopback 24317 and 24318.** … This change is the operator's reading (the pc overseer, 2026-10-10, inputs#I3), **PROVISIONAL, the founder's own word owed** (playbook pattern: Boundary widening)", and inputs#I5 item 4 prescribes the amended sentence's three parts. Architecture states the opposite as a boundary ("It binds no port and starts no `pulse-app`"). Raised to escalate because the widening rests on a provisional reading and a founder ratification is outstanding; the wording itself is carried by the report.
    basis: "xtask/src/pre_push.rs:322-369"
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → xtask CLI surfaces → `cargo xtask pre-push:linux` (the sentence: Reasons, complete: `all-stages-ok` · … · `restore-failed:bindings`)"
    change: >-
      Reasons, complete — eleven (was nine): add `engine-port-taken` (a cycle port, 24317 or 24318, already accepting before stage six starts anything) and `engine-cycle-unevaluated` (the stage's engine cycle exited 2), both `cannot-evaluate`.
    sidecar: "2026-10-10-engine-harness-proofs-completed: `pre-push:linux` reasons 9 → 11 (`engine-port-taken`, `engine-cycle-unevaluated`)."
    rationale: >-
      Report Changes → Symbols / APIs ("Two new closed reasons: `engine-port-taken`, `engine-cycle-unevaluated`") and Counts ("reasons of `pre-push:linux`: 9 → 11"). The entry calls its nine-item list "complete".
    basis: "xtask/src/pre_push.rs:305-320"
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → xtask CLI surfaces → `cargo xtask pre-push:linux` (the sentence: Every probe and stage child is spawned with a CLEARED environment plus exactly `HOME` · `PATH` · `ANDROMEDA_PULSE_DATA_DIR` …)"
    change: >-
      Add to the child-set sentence: the stage child set is unchanged (`HOME`, `PATH`, `ANDROMEDA_PULSE_DATA_DIR`), with one more exception — the single `cleanup` child stage six runs when a cycle left a port accepting or its engine alive also carries the two cycle port variables `ANDROMEDA_PULSE_OTLP_GRPC_PORT` / `ANDROMEDA_PULSE_OTLP_HTTP_PORT` (24317 / 24318), so its probe never touches 4317 or 4318; the cycle the stage starts builds its own children's set (`harness:engine-cycle`'s entry), which holds no session bus and no runtime dir, so "no stage reaches an OS credential store" stands.
    sidecar: "2026-10-10-engine-harness-proofs-completed: `pre-push:linux`'s one cleanup child carries the two cycle port variables beyond the stage child set."
    rationale: >-
      Report Changes → Harness / gate surface: "The stage child set is unchanged (`HOME`, `PATH`, `ANDROMEDA_PULSE_DATA_DIR`); the one cleanup child also carries the two cycle port variables, so its probe never touches 4317 or 4318"; Deviations lists it as an unplanned addition. The entry says "exactly" and names only the `npm` and git exceptions.
    basis: "xtask/src/pre_push.rs:375-383"
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Environment variables → `PUPPETEER_CACHE_DIR` (the sentence: The same verb SETs `GIT_INDEX_FILE` … and `ANDROMEDA_PULSE_DATA_DIR` … for every probe and stage child)"
    change: >-
      Add: and, since chunk 2026-10-10-engine-harness-proofs-completed, `ANDROMEDA_PULSE_OTLP_GRPC_PORT` / `ANDROMEDA_PULSE_OTLP_HTTP_PORT` (24317 / 24318) into the one `cleanup` child of its sixth stage only — harness-only, no new name.
    sidecar: "2026-10-10-engine-harness-proofs-completed: env registry names the two port variables `pre-push:linux` sets into its cleanup child."
    rationale: >-
      Second occurrence of the "what `pre-push:linux` SETs into its children" claim, in the environment-variable registry. Report Changes → Harness / gate surface ("the one cleanup child also carries the two cycle port variables") and Symbols / APIs ("No new `ANDROMEDA_PULSE_*` name").
    basis: "xtask/src/pre_push.rs:375-383"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → xtask CLI surfaces → `cargo xtask ci-gates` (the sentence: Run by ci.yml's `boot` job after the smoke and the series, and by `pre-push:linux`'s sixth stage over its own seed log)"
    change: >-
      Run by ci.yml's `boot` job after the smoke and the series, and by `pre-push:linux`'s sixth stage over the log family of the real console engine cycle that stage ran, in the same data dir and only when the cycle exited 0 (the stage's seed log is removed, chunk 2026-10-10-engine-harness-proofs-completed).
    sidecar: "2026-10-10-engine-harness-proofs-completed: `ci-gates`' pre-push caller reads a real engine cycle's log family, not a seed."
    rationale: >-
      Occurrence of the retired pre-push claim in the `ci-gates` caller clause. Report Changes → Symbols / APIs ("`SEED_LOG` and `seed` are removed") and Counts ("what stage six of `pre-push:linux` reads: a seed of three records → the log family of a real console engine cycle (335 records in each of the four readings)").
    basis: "xtask/src/pre_push.rs:305-320"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → xtask CLI surfaces → `cargo xtask harness:engine-cycle` (the sentence: Its callers are the `Console engine cycle` step of ci.yml's `boot` job … and a chunk's gate block on the dev host)"
    change: >-
      Its callers are the `Console engine cycle` step of ci.yml's `boot` job (unchanged), a chunk's gate block on the dev host, and — since chunk 2026-10-10-engine-harness-proofs-completed — the sixth stage of `cargo xtask pre-push:linux` (`--data-dir` the stage data dir, default ports 24317 / 24318; operator's reading, PROVISIONAL).
    sidecar: "2026-10-10-engine-harness-proofs-completed: `harness:engine-cycle` gains a third caller, `pre-push:linux` stage six."
    rationale: >-
      Occurrence of the retired pre-push claim as a closed caller list. Report Changes → Harness / gate surface: "`cargo xtask harness:engine-cycle --data-dir {that dir}` under the stage child set".
    basis: "xtask/src/pre_push.rs:322-369"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Network ports → `:24317` / `:24318`"
    change: >-
      Add: since chunk 2026-10-10-engine-harness-proofs-completed the pair is also bound on every `cargo xtask pre-push:linux` run, whose sixth stage runs a cycle on the defaults (operator's reading, PROVISIONAL, the founder's word owed); a cycle refuses to start when either port already accepts (`port-taken`), and the pre-push stage stops `cannot-evaluate` / `engine-port-taken` on the same reading before starting anything.
    sidecar: "2026-10-10-engine-harness-proofs-completed: 24317 / 24318 are bound by `pre-push:linux` stage six too; a taken port is a refusal."
    rationale: >-
      The port registry names `harness:engine-cycle` as the pair's only binder. Report Changes → Harness / gate surface ("The check now starts `andromeda-pulse-engine` and binds loopback 24317 and 24318"; "either cycle port (24317, 24318) accepting stops the run `cannot-evaluate` / `engine-port-taken`") and Symbols / APIs (`port-taken`).
    basis: "xtask/src/pre_push.rs:322-369"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Filesystem locations → Subpaths under the resolved root → `run/andromeda-pulse.pid` (the reader list: read by `cargo xtask harness:status` … `harness:engine-settled` … and by `scripts/agent-run.{sh,ps1}`)"
    change: >-
      Add a reader: since chunk 2026-10-10-engine-harness-proofs-completed `cargo xtask pre-push:linux` reads the pid file of its stage data dir after the stage's engine cycle, to tell whether the engine it started is still alive (alive, or a cycle port still accepting, runs the script's `cleanup` once and makes the stage red).
    sidecar: "2026-10-10-engine-harness-proofs-completed: pid-file readers gain `pre-push:linux` stage six."
    rationale: >-
      The bullet enumerates the pid file's readers; the retired "pre-push starts nothing" claim is why pre-push was absent. Report Changes → Harness / gate surface: "then both ports and the pid of that data dir's pid file are read: one still accepting or alive → the script's `cleanup` runs once on that data dir and the stage is red".
    basis: "xtask/src/pre_push.rs:356-357"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → xtask CLI surfaces → `scripts/agent-run.{sh,ps1}` entry (the sentence: A second caller of the sh script since that chunk: `cargo xtask harness:engine-cycle` runs `boot engine`, `status engine` and `cleanup`)"
    change: >-
      Add after it: a third caller since chunk 2026-10-10-engine-harness-proofs-completed — the sixth stage of `cargo xtask pre-push:linux` runs `cleanup` once on its stage data dir, under the stage child set plus the two cycle port variables, when its cycle left a port accepting or its engine alive.
    sidecar: "2026-10-10-engine-harness-proofs-completed: `agent-run.sh cleanup` gains a caller, `pre-push:linux` stage six's teardown."
    rationale: >-
      Occurrence of the retired pre-push claim as a caller count of the script. Report Changes → Symbols / APIs (`fn end_cycle` … "its `cleanup` call is 375-383") and Harness / gate surface ("the script's `cleanup` runs once on that data dir").
    basis: "xtask/src/pre_push.rs:375-383"
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Network ports (the bullet: Both engine programs bind these two receivers and nothing else — … That no third socket listens is by construction and was not measured from the process's socket list)"
    change: >-
      Replace the last sentence: that no third socket listens is by construction and, since chunk 2026-10-10-engine-harness-proofs-completed, read from a spawned console engine's socket list under `/proc` — `pulse-app/tests/integration_console_engine.rs` asserts the listening sockets are exactly the two receivers on `127.0.0.1` and no socket beyond them, and that with a rejected gRPC port variable the bind record reads `invalid_port`, that receiver is not started and the HTTP receiver alone listens. Read on the console program; not read on the window app.
    sidecar: "2026-10-10-engine-harness-proofs-completed: the two-receivers-and-nothing-else claim is now a socket census on a spawned engine."
    rationale: >-
      Report Changes → Coverage of new surfaces ("the three spawned readings (rejected port, socket census, lock file's directory) → tests integ on a spawned engine, each read red first") and Expected amendments (site `third socket|no other socket|socket census` architecture body 1); Outcome 4: "Listening sockets are exactly the two receivers on `127.0.0.1`; a rejected variable reads `invalid_port` and the census the other alone". The registry still says "was not measured from the process's socket list". The arms spawn `integration_console_engine`'s console engine, hence the window-app limit.
    basis: "pulse-app/tests/integration_console_engine.rs:631-655"
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Filesystem locations → Out-of-data-dir lock file (corpus-key creation) (the clause: for the console program the file was read on a hand run, and its placement outside the data dir is not asserted by a test)"
    change: >-
      Replace the clause: for the console program its placement is asserted by a spawned-engine test since chunk 2026-10-10-engine-harness-proofs-completed (`the_corpus_key_lock_file_sits_in_the_directory_the_environment_gives`, `pulse-app/tests/integration_console_engine.rs`) in both environments, a runtime dir handed and not handed: the file sits in the directory the rule gives, zero bytes, mode `0o600`, nothing under the data dir, and its name and the lock directory's path appear 0 times in the run's log family. Add beside residual (2): the children of `cargo xtask harness:engine-cycle` — and so of `pre-push:linux`'s sixth stage — are handed no runtime dir, so a cycle's engine writes the lock file to the temp dir, where it stays.
    sidecar: "2026-10-10-engine-harness-proofs-completed: the console program's corpus-key lock-file placement is asserted by a spawned arm; a cycle's engine writes it to the temp dir."
    rationale: >-
      Report Changes → Coverage of new surfaces ("PII ✓ (the lock arm asserts the lock file's name and the lock directory's path, as given and as resolved, appear 0 times in the run's log family)"), Harness / gate surface ("The cycle's engine writes its corpus-key lock file to the temp dir (the cycle's child set holds no runtime dir)") and Expected amendments (site `andromeda-pulse-corpus-key-` body 1); Outcome 5: "zero bytes, mode `0o600`, nothing under the data dir, 0 times in the log — met, in both environments". The registry still says "not asserted by a test".
    basis: "pulse-app/tests/integration_console_engine.rs:657-751"
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Environment variables → the bullet `Set by cargo xtask harness:engine-cycle into its children only` (the clause: which key path the engine then took was not read from its log)"
    change: >-
      Replace the clause: handed a passphrase variable and no session bus, the engine of a cycle and of the spawned suite takes the passphrase fallback after opening its lock file — read on the spawned suite's engine, whose first arm asserts one `corpus.keychain.fallback` record and whose lock-file arm waits on that record (chunk 2026-10-10-engine-harness-proofs-completed).
    sidecar: "2026-10-10-engine-harness-proofs-completed: the key path of a harness-started engine is read (`corpus.keychain.fallback`) on the spawned suite."
    rationale: >-
      Report Changes → Expected amendments (the Network ports / Filesystem / Environment variables row): "the engine of a cycle and of the spawned suite is handed a passphrase variable and no session bus, so it takes the passphrase fallback after opening its lock file (the spawned suite's first arm asserts one `corpus.keychain.fallback` record; the lock-file arm waits on that record)". The registry still says the key path "was not read from its log". The report gives the record's read on the spawned suite; it reports no read of a cycle's own log for it.
    basis: "pulse-app/tests/integration_console_engine.rs:657-751"
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Filesystem locations (new bullet, beside `Harness-made data dirs outside the default data dir (the engine cycle)`)"
    change: >-
      **Test-made port-claim files outside every data dir:** `target/test-ports/` — one empty file per claimed port number, each held under an advisory lock (`File::try_lock`) for as long as a test uses the port; written by the three port-using test binaries `agent_run_shell`, `engine_cycle_refusals` (`xtask/tests/common/mod.rs`, `claim_port` / `PortClaim`) and `integration_console_engine`; git-ignored, never unlinked by a test (unlinking a lock file would let two later claimants lock different inodes under one name — reasoned, not measured), removed only by `cargo clean` (about 1 350 files after the chunk's runs; no owner named). Never product-written, so the THREE product-written exceptions are unchanged (chunk 2026-10-10-engine-harness-proofs-completed).
    sidecar: "2026-10-10-engine-harness-proofs-completed: registers `target/test-ports/`, the test suites' port-claim files."
    rationale: >-
      Report Changes → Harness / gate surface ("held by an advisory lock (`File::try_lock`) on a file under `target/test-ports/` (git-ignored, never unlinked: 1 350 empty files after implement's runs)") and Coverage of new surfaces ("test-port claims (`target/test-ports/`)"). `test-ports` has 0 hits in architecture; the registry already lists harness-made locations of this kind (`target/engine-cycle/{stamp}/`, the exit witness).
    basis: "xtask/tests/common/mod.rs:87-113"
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Network ports (new bullet, after `:24317` / `:24318`)"
    change: >-
      `:20000`–`:31999`, 24317 and 24318 left out — test-only loopback ports, since chunk 2026-10-10-engine-harness-proofs-completed: every port the three test binaries `agent_run_shell`, `engine_cycle_refusals` and `integration_console_engine` use is a number CLAIMED from this range (an advisory file lock under `target/test-ports/`, shown bindable when claimed), not bound to port 0 and released; 4317 and 4318 are bound by no test and no gate entry. `pulse-app/tests/integration_engine_boot.rs` still picks by bind-and-release, outside the claimed range. No port is added to the product.
    sidecar: "2026-10-10-engine-harness-proofs-completed: registers the test-only claimed port range 20000–31999 (24317 / 24318 excluded)."
    rationale: >-
      Report Changes → Harness / gate surface ("every port the three binaries … use is claimed, not bound-to-0-and-released: a number from 20000 to 31999 (24317 and 24318 left out)"), Insufficient fixes (`integration_engine_boot.rs` "still picks by bind-and-release") and Outcome 4 ("4317 and 4318 are bound by no test and no gate entry"); Symbols / APIs: "no product port". A port range tests occupy on the dev host and the runner is unregistered.
    basis: "xtask/tests/common/mod.rs:87-113"
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → xtask CLI surfaces → `cargo xtask check:english-sources` (the clause: scans `crates` · `pulse-app/src` · `pulse-app/tests` · `pulse-app/ui/src` · `xtask/src`)"
    change: >-
      scans six roots (was five): `crates` · `pulse-app/src` · `pulse-app/tests` · `pulse-app/ui/src` · `xtask/src` · `xtask/tests` (the last since chunk 2026-10-10-engine-harness-proofs-completed; an absent `xtask/tests` is `cannot-evaluate`, like any absent root).
    sidecar: "2026-10-10-engine-harness-proofs-completed: English-only lint roots 5 → 6 (`xtask/tests`)."
    rationale: >-
      Report Changes → Symbols / APIs ("`xtask/src/source_lint.rs` — `ROOTS` is `[&str; 6]` (lines 24, 30), gaining `xtask/tests`"), Counts ("English-only lint roots: 5 → 6") and Harness / gate surface ("A missing `xtask/tests` now fails both (a root absent is `cannot-evaluate` for the lint …)").
    basis: "xtask/src/source_lint.rs:24"
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → xtask CLI surfaces → `cargo xtask quarantine-tracking` (the clause: It scans the `.rs` files under `crates`, `pulse-app/src`, `pulse-app/tests` and `xtask/src`)"
    change: >-
      It scans the `.rs` files under five search dirs (was four): `crates`, `pulse-app/src`, `pulse-app/tests`, `xtask/src` and `xtask/tests` (the last since chunk 2026-10-10-engine-harness-proofs-completed, in the `.sh` and in the `.ps1` mirror; a missing `xtask/tests` fails the check like any missing search dir).
    sidecar: "2026-10-10-engine-harness-proofs-completed: quarantine search dirs 4 → 5 (`xtask/tests`)."
    rationale: >-
      Report Changes → Symbols / APIs ("`QUARANTINE_SEARCH_DIRS` is `[&str; 5]` (939-945)"), Counts ("quarantine search dirs: 4 → 5 (both gain `xtask/tests`)") and Harness / gate surface ("(the `.sh`; the `.ps1` mirror takes the same entry) read `xtask/tests` … a missing search dir fails the quarantine check"). The entry's "held by five pins that run it with `bash`" is left as it stands: the report adds one pin for the new directory in `xtask/src/main.rs` but does not say it is one of the bash-run pins.
    basis: "xtask/src/main.rs:939-945"
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Environment variables (new bullet, test-only, beside the harness-only SET entries)"
    change: >-
      Handed to a child by test code only, no new name (chunk 2026-10-10-engine-harness-proofs-completed): `TMPDIR` and `XDG_RUNTIME_DIR` — SYSTEM variables — to the spawned engine in the lock-file arm of `pulse-app/tests/integration_console_engine.rs` alone; `HOME`, `PATH`, `TMPDIR`, the data dir, the two port variables, the passphrase variable and `HARNESS_STATUS_TIMEOUT` to `scripts/agent-run.sh` by the `xtask/tests` suites; `LLVM_PROFILE_FILE` passed through by `xtask/tests/engine_cycle_refusals.rs` when the test process holds it. No `ANDROMEDA_PULSE_*` name is added and no product read changes.
    sidecar: "2026-10-10-engine-harness-proofs-completed: registers the variables the new tests hand a child (`TMPDIR`, `XDG_RUNTIME_DIR`, `LLVM_PROFILE_FILE` among them)."
    rationale: >-
      Report Changes → Symbols / APIs: "Variables the new tests hand a child (test code only): `TMPDIR` and `XDG_RUNTIME_DIR` to the spawned engine in the lock-file arm alone; `HOME`, `PATH`, `TMPDIR`, the data dir, the two port variables, the passphrase variable and `HARNESS_STATUS_TIMEOUT` to the script; `LLVM_PROFILE_FILE` passed through when the test process holds it." `TMPDIR` and `LLVM_PROFILE_FILE` have 0 hits in architecture. Lowest-confidence proposal of the set: the registry so far lists variables set by xtask verbs and scripts, none set by test code, so the orchestrator may decline it as below the registry's granularity.
    basis: "xtask/tests/engine_cycle_refusals.rs:38-40"
  - detector: D-arch-decisions
    severity: warning
    section: "§Stack and Technologies → Gate-script pins (test-only)"
    change: >-
      Extend the row: since chunk 2026-10-10-engine-harness-proofs-completed the `xtask` package has its first integration tests — two binaries in `xtask/tests/` (`agent_run_shell` 19 tests, `engine_cycle_refusals` 6) over the shared module `xtask/tests/common/mod.rs` — which run the committed `scripts/agent-run.sh` and the built `harness:engine-cycle` verb with `bash` on real console engines; with the three new spawned arms of `pulse-app/tests/integration_console_engine.rs` they need, on every host that runs the workspace tests, `bash`, a Linux `/proc`, an advisory file lock under `target/` and the engine built in the test profile (a `--workspace` run builds it). No Cargo dependency, no manifest or lockfile change (`tempfile` and `serde_json` were already dependencies of `xtask`; no `libc`); never in the shipped binary. They run in ci.yml's `lint / test`, `mcp-server tests` and `coverage gate` jobs.
    sidecar: "2026-10-10-engine-harness-proofs-completed: Gate-script pins row names the `xtask/tests` binaries and what a test host needs for them (`bash`, `/proc`, a file lock under `target/`, the test-profile engine)."
    rationale: >-
      Report Changes → Crates / modules ("The `xtask` package gains its first integration tests: two test binaries … and a shared module"), Dependencies ("none added or bumped … use `tempfile` and `serde_json`, already normal dependencies of the package, and no `libc`") and Harness / gate surface ("the new tests, which need `bash`, `/proc`, an advisory file lock under `target/` and the engine built in the test profile"); Expected amendments names this row (`xtask/tests` has 0 hits in the masters). No library or runtime is added and no locked decision is contradicted; the drift is that the §Stack row listing what a test host must hold names neither the new consumers of `bash` nor `/proc`.
    basis: "xtask/tests/common/mod.rs:51-64"
```

Dispositions:
- 1: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 2: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 3: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 4: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 5: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 6: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 7: escalate — check 1, playbook rule Boundary widening (never routine); RESOLVED on the operator's recorded direction: "The pre-push sixth stage is PROVISIONAL … the amended sentence says what the stage does now … that it is the operator's reading (the pc overseer, 2026-10-10), and that the founder's own word is owed" — the operator (the pc overseer), 2026-10-10, relayed verbatim in the wrap directive (inputs#I5 item 4; the P4 answer is inputs#I3). Applied in that PROVISIONAL form; listed at the route-resolve card
- 8: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 9: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 10: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 11: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 12: apply with its primary, in the PROVISIONAL form — a restating site of the stage-six widening (dependent group; see the primary); its caller clause itself is an accurate this-chunk addition
- 13: apply with its primary, in the PROVISIONAL form — a restating site of the stage-six widening (dependent group; see the primary)
- 14: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 15: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 16: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 17: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 18: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 19: apply — check 1, Accurate this-chunk addition: a new git-ignored location of a kind §Filesystem locations already enumerates (harness-made dirs); its missing owner goes to the route-resolve card
- 20: apply — check 1, Accurate this-chunk addition: a port range tests occupy on a host that shares ports with another project; a kind §Network ports enumerates (the harness-only pair)
- 21: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 22: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 23: reject — check 1, playbook rule Registry over-reach: §Environment variables registers what a product or harness PROGRAM reads or sets; the child environment of a test is below that grain, and the four spawned arms that already handed variables to an engine registered none. The detector rated it its lowest-confidence proposal
- 24: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)

## security-plan
verdict: 9 proposal(s) · stripped: a five-line comment tail (no drift on D-security-deps, D-security-auth, D-security-logging; line numbers are the report's)

```yaml
proposals:
  - detector: D-security-input
    severity: escalate
    section: "§Security Anti-Patterns → Input (the harness-class paragraph for `cargo xtask pre-push:linux`, chunk `2026-10-09-pre-push-check-native-on-linux`)"
    change: >-
      Add, dated chunk `2026-10-10-engine-harness-proofs-completed`: the verb's sixth stage now starts `andromeda-pulse-engine` and binds loopback 24317 and 24318 — it removes and re-creates the stage data dir, stops `cannot-evaluate` / `engine-port-taken` with nothing started when either cycle port already accepts, runs `cargo xtask harness:engine-cycle --data-dir {that dir}` under the unchanged stage child set (`HOME`, `PATH`, `ANDROMEDA_PULSE_DATA_DIR`), then reads both ports and the pid of that data dir's pid file (one still accepting or alive → the script's `cleanup` once and the stage red whatever the cycle returned; cycle exit 2 → `cannot-evaluate` / `engine-cycle-unevaluated`; exit 0 → `ci-gates` over the same data dir decides); item (b)'s "exactly" set gains one exception (the single cleanup child also carries the two cycle port variables, so its probe never touches 4317 / 4318); item (c)'s "writes only its per-run area" gains the cycle engine's content-free corpus-key lock file in the system temp dir (the cycle's child set holds no runtime dir) and the cycle's stderr printing the data dir through the boot verb's ready line; and the classification of THIS stage-six change is the operator's reading (the pc overseer, 2026-10-10), PROVISIONAL, the founder's own word owed — it is no longer covered by the 2026-10-09 "routine harness evidence, not a boundary widening" reading that heads the paragraph.
    sidecar: >-
      2026-10-10-engine-harness-proofs-completed — §Security Anti-Patterns → Input: `pre-push:linux` stage six starts the console engine and binds loopback 24317/24318 (two new closed reasons, cleanup child carries the cycle port variables, lock file in the temp dir); classification PROVISIONAL, founder's word owed.
    rationale: >-
      Report Changes → Harness / gate surface: "The check now starts `andromeda-pulse-engine` and binds loopback 24317 and 24318", "the one cleanup child also carries the two cycle port variables", "The cycle's engine writes its corpus-key lock file to the temp dir", and "This change is the operator's reading (the pc overseer, 2026-10-10, inputs#I3), PROVISIONAL, the founder's own word owed (playbook pattern: Boundary widening)"; Symbols / APIs names the two new closed reasons. The paragraph still states an exact child-environment set, "writes only its per-run area", and a not-a-boundary-widening reading. Validation is present per Coverage of new surfaces (`pre-push:linux` stage six → validation ✓: fixed ports, a taken port refused before anything starts); the drift is the doc's description and classification. The report's Expected amendments names this section.
    basis: "xtask/src/pre_push.rs:322-369 (read_engine_cycle), 305-320 (decide_ci_gates), 375-383 (the cleanup call)"

  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → CLI / env var inputs (the row's third harness-only reader, `cargo xtask pre-push:linux`)"
    change: >-
      Extend the `pre-push:linux` passage: since chunk `2026-10-10-engine-harness-proofs-completed` its sixth stage runs a real console engine cycle on the fixed harness-only ports 24317 / 24318 (never 4317 / 4318), a cycle port already accepting reads `cannot-evaluate` / `engine-port-taken` before anything starts, a cycle that could not be evaluated reads `cannot-evaluate` / `engine-cycle-unevaluated`, the stage reads both ports and the stage data dir's pid file after the cycle and ends what it started, the "pinned set" sentence notes the one cleanup child that also carries the two cycle port variables, and the verdict's six members still carry no value of an environment variable and no path; classification PROVISIONAL per §Security Anti-Patterns → Input.
    sidecar: >-
      2026-10-10-engine-harness-proofs-completed — §Input Validation CLI/env row: `pre-push:linux` stage six registered (real engine cycle on 24317/24318, closed reasons `engine-port-taken` and `engine-cycle-unevaluated`), PROVISIONAL.
    rationale: >-
      Duplicate occurrence of the `pre-push:linux` contract the primary amends (the report counts three `pre-push:linux` sites in the security-plan body). Report Changes → Symbols / APIs: "Two new closed reasons: `engine-port-taken`, `engine-cycle-unevaluated`"; Counts: reasons of `pre-push:linux` 9 → 11; Harness / gate surface gives the stage-six order. The row describes the verb as reading `HOME` / `PATH` only and says every probe and stage child gets "a cleared environment plus a pinned set", with no port, engine or pid-file read.
    basis: "xtask/src/pre_push.rs:288-303 (CycleReading), 305-320, 322-369"
    dependent-of: D-security-input

  - detector: D-security-input
    severity: escalate
    section: "§Threat Model Summary → Attack surface → CLI input (env vars + binary launch), Trust boundary"
    change: >-
      In the harness-only carve-out list, beside the `pre-push:linux` member (by-value read of `HOME` / `PATH`, per-run area under `target/pre-push/run/`), state that since chunk `2026-10-10-engine-harness-proofs-completed` its sixth stage starts the console engine and binds loopback 24317 / 24318, and that this part sits beside the carve-out with its classification PROVISIONAL (the operator's reading, the pc overseer, 2026-10-10; the founder's own word owed) — worded as the exit-witness arm's PROVISIONAL note already is in the same sentence run.
    sidecar: >-
      2026-10-10-engine-harness-proofs-completed — Threat Model CLI-input trust boundary: `pre-push:linux` stage six (engine started, 24317/24318 bound) noted as PROVISIONAL beside the harness carve-out.
    rationale: >-
      Third occurrence of the `pre-push:linux` member (report Expected amendments: "`pre-push:linux` security-plan body 3"). This site lists the member inside a carve-out that "stands per §Security Anti-Patterns → Input"; the report makes the stage-six change PROVISIONAL with the founder's word owed, so the restating site would otherwise carry the settled reading alone.
    basis: "report Changes → Harness / gate surface, the `cargo xtask pre-push:linux` bullet"
    dependent-of: D-security-input

  - detector: D-security-input
    severity: escalate
    section: "§Security Anti-Patterns → Input (the harness-class paragraph of chunk `2026-10-10-agent-harness-drives-the-console-engine`, item (c) of `cargo xtask harness:engine-cycle`)"
    change: >-
      Replace "it writes only its per-cycle data dir" with a statement that the verb itself writes only its per-cycle data dir, while the engine it starts also writes the content-free corpus-key lock file `andromeda-pulse-corpus-key-{h16}.lock` into the system temp dir — the cycle's child set holds no runtime dir, so the engine takes the `std::env::temp_dir()` branch of the lock-dir rule, and with a passphrase and no session bus it takes the passphrase fallback after opening that lock file; the lock file is never removed (the residual §Data Protection → At rest already records).
    sidecar: >-
      2026-10-10-engine-harness-proofs-completed — §Security Anti-Patterns → Input: `harness:engine-cycle` item (c) corrected — the cycle's engine writes the corpus-key lock file to the system temp dir, outside the per-cycle data dir.
    rationale: >-
      Same claim the primary retires ("writes only its own area"), stated a second time for the cycle verb. Report Changes → Harness / gate surface: "The cycle's engine writes its corpus-key lock file to the temp dir (the cycle's child set holds no runtime dir)"; Expected amendments: "the engine of a cycle and of the spawned suite is handed a passphrase variable and no session bus, so it takes the passphrase fallback after opening its lock file"; Process hygiene lists "the lock file in the system temp dir" as left on the host. The placement is pinned on a spawned engine in both environments (acceptance criterion 5).
    basis: "pulse-app/tests/integration_console_engine.rs:657-751 (the lock-file arm)"
    dependent-of: D-security-input

  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → CLI / env var inputs (the row's fifth harness-only boundary, `cargo xtask harness:engine-cycle` refusals)"
    change: >-
      The closed refusal list gains an eighth label, `port-taken`: a cycle port that already accepts a loopback connection is refused before anything starts — stderr `harness:engine-cycle: cannot-evaluate (port-taken)`, the verdict object with nulls, exit 2, nothing built or started, no data dir made; the check runs after the existing refusals (so 4317 and 4318 are refused as `shared-port` before any port is probed) and before the `HOME` read.
    sidecar: >-
      2026-10-10-engine-harness-proofs-completed — §Input Validation CLI/env row: `harness:engine-cycle` refusal labels 7 → 8 (`port-taken`).
    rationale: >-
      Report Changes → Symbols / APIs: "`fn taken_port_refusal(grpc_accepting, http_accepting)` (122-127), called in `drive` (243-250) … New closed label `port-taken`"; Counts: "refusal labels of `harness:engine-cycle`: 7 → 8". The row enumerates seven labels ending at `injector-build-failed`. Validation is present (Coverage: `harness:engine-cycle` refusing `port-taken` → validation ✓, unit 2 + integ); the drift is the stale enumeration. Sweep: the row is the only place the document enumerates the labels (report: `injector-build-failed` security-plan body 1); the Anti-Patterns → Input item (b) says only "4317 and 4318 refused", which still holds.
    basis: "xtask/src/engine_cycle.rs:122-127, 243-250"

  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → CLI / env var inputs (the row's `harness:ready` reader)"
    change: >-
      Register the verb's new input: since chunk `2026-10-10-engine-harness-proofs-completed` `harness:ready` probes both resolved ports and then reads the log family once as JSON lines (a line that is no JSON is skipped), and each receiver member is one of three closed labels — `accepting` (a loopback connection accepted on the resolved port AND, after the last `app.boot.engine` record, an INFO record of that receiver's bind target, `app.boot.otlp.grpc.bind` / `app.boot.otlp.http.bind`, whose `bind_address` is `127.0.0.1:` and that port), `foreign` (accepted, no such record: a failed-bind ERROR record, another port, the other receiver, a record before the last boot record, or no boot record at all), `refusing` (no connection accepted); `ready` only when the status verdict is `running-healthy` and both read `accepting`, for both programs; labels only — no record text, address or path leaves the verb.
    sidecar: >-
      2026-10-10-engine-harness-proofs-completed — §Input Validation CLI/env row: `harness:ready` reads the log family's bind records; receiver labels 2 → 3 (`foreign`).
    rationale: >-
      Report Changes → Symbols / APIs ("The readiness decision, as built") and Schema / config: "the `harness:ready` verdict's two receiver members take one of three closed labels (`accepting` · `foreign` · `refusing`), was two". The row gives the verb's inputs as the two port variables, the three system variables and the `program` label from the last `app.boot.engine` record; the bind-record read is a new deserialized input it does not name. Validation is present per Coverage (`harness:ready` reading `foreign` → validation ✓: the port from `resolve_port`, the log read as JSON lines, a non-JSON line skipped; PII n/a, labels only). The report's Expected amendments names "the readiness verb's read of the log" for this section.
    basis: "xtask/src/harness_ready.rs:172-180 (enum Receiver), 200-227 (bound_by_this_run)"

  - detector: D-security-input
    severity: escalate
    section: "§Security Anti-Patterns → Input (the harness-class paragraphs listing what `harness:ready` reads: item (a) of chunk `2026-10-09-boot-smoke-s-early-exit-found-and-closed` and item (a) of chunk `2026-10-10-agent-harness-drives-the-console-engine`)"
    change: >-
      Add to the readers of the class: since chunk `2026-10-10-engine-harness-proofs-completed` `cargo xtask harness:ready` also reads the receivers' bind records from the log family (after the last `app.boot.engine` record) and lets out one of three closed labels per receiver (`accepting` · `foreign` · `refusing`); `agent-run.sh boot` prints one more failure line on `foreign`, carrying the two loopback ports and nothing else, while `agent-run.ps1` is not edited and its readiness branch still matches `refusing` alone; harness-only, nothing product-consumed.
    sidecar: >-
      2026-10-10-engine-harness-proofs-completed — §Security Anti-Patterns → Input: `harness:ready`'s read of the log family's bind records and its third receiver label registered in the harness class; `agent-run.sh boot` foreign line noted, `.ps1` unchanged.
    rationale: >-
      Second place the document lists what `harness:ready` reads (the exit record through `read_ended`; `--program` with its three-label `program` member) — the same reader inventory the §Input Validation proposal extends (report: `harness:ready` security-plan body 3). Report Changes: `run_ready` "probes both ports, then reads the log family once (`smoke::read_jsonl_lines`)"; `scripts/agent-run.sh` 196-200 adds the foreign line; Harness / gate surface: "`agent-run.ps1` is not edited and its readiness branch still matches `refusing` alone". The report's Expected amendments names this section for the readiness verb's read of the log.
    basis: "xtask/src/harness_ready.rs:200-227; scripts/agent-run.sh:196-200"
    dependent-of: D-security-input

  - detector: D-security-input
    severity: escalate
    section: "§Input Validation → CLI / env var inputs (the row's fifth harness-only boundary, the `scripts/agent-run.sh` `boot` / `status` closed word set)"
    change: >-
      Replace "sh only; no committed shell-level test, test-plan §1 `harness-cleanup-verdict-and-boot-spawn-shell-coverage`" with: sh only; the word arms are pinned by committed shell-level tests in `xtask/tests/agent_run_shell.rs` (bare `boot` resolves the window program; `status engine` and bare `status`; any other word after `boot` or `status` is usage, exit 2, no data dir) since chunk `2026-10-10-engine-harness-proofs-completed`, which closes that row's word-arm clause; the `.ps1` leg is parsed, not run.
    sidecar: >-
      2026-10-10-engine-harness-proofs-completed — §Input Validation CLI/env row: the `agent-run.sh` closed word set is now pinned by `xtask/tests/agent_run_shell.rs`; the "no committed shell-level test" qualifier retired for the word arms.
    rationale: >-
      Report Outcome → The four pending rows: `harness-cleanup-verdict-and-boot-spawn-shell-coverage` — "(f) per word arm: closed"; acceptance criterion 1 lists "the word arms (bare `boot`; `status engine` and bare `status`; any other word)"; Crates / modules: the `xtask` package gains the `agent_run_shell` test binary. The row still says the word set has no committed shell-level test. The report's Expected amendments counts two security-plan sites for this row key.
    basis: "xtask/tests/agent_run_shell.rs:453-476 (any other word), 244-259 (bare boot), 428-451 (status)"

  - detector: D-security-input
    severity: escalate
    section: "§Security Anti-Patterns → Input (the exit-witness arm, item (a): \"the arm ships with no committed shell-level test\")"
    change: >-
      Keep the claim and scope it: the exit-witness branches of `agent-run.sh boot` still have no committed shell-level test — they are clause (d) of test-plan §1 `harness-cleanup-verdict-and-boot-spawn-shell-coverage`, which stays open on (c) the `.ps1` boot-cycle leg, (d) the exit-witness branches and (e) the boot series' `timed-out` path, while its `cleanup`, `boot` failure-reason and word-arm clauses closed at chunk `2026-10-10-engine-harness-proofs-completed`.
    sidecar: >-
      2026-10-10-engine-harness-proofs-completed — §Security Anti-Patterns → Input: the exit-witness arm's "no committed shell-level test" scoped to clause (d) of the shell-coverage row, the row's (a)/(b)/(f) clauses now closed.
    rationale: >-
      Second occurrence of the "no committed shell-level test (test-plan §1 `harness-cleanup-verdict-and-boot-spawn-shell-coverage`)" wording. Here it remains TRUE — report Outcome: "Open, with their owners on the route: (c) the `.ps1` boot-cycle leg (`working-route.md:64`); (d) the exit-witness branches and (e) the boot series' `timed-out` path (`working-route.md:50`)" — but the row it cites is no longer wholly open, so after the primary is applied an unscoped citation reads as if the whole script were still unpinned. A scoping edit, not a reversal; drop it if the orchestrator prefers to leave a still-true sentence untouched.
    basis: "report Outcome → The four pending rows, the `harness-cleanup-verdict-and-boot-spawn-shell-coverage` bullet"
    dependent-of: D-security-input
```

Dispositions:
- 1: escalate — check 1, playbook rule Boundary widening (never routine); RESOLVED on the operator's recorded direction: "The pre-push sixth stage is PROVISIONAL … the amended sentence says what the stage does now … that it is the operator's reading (the pc overseer, 2026-10-10), and that the founder's own word is owed" — the operator (the pc overseer), 2026-10-10, relayed verbatim in the wrap directive (inputs#I5 item 4; the P4 answer is inputs#I3). Applied in that PROVISIONAL form; listed at the route-resolve card. D-security-input's escalate severity also applies here by its own class
- 2: apply with its primary, in the PROVISIONAL form — a restating site of the stage-six widening (dependent group; see the primary)
- 3: apply with its primary, in the PROVISIONAL form — a restating site of the stage-six widening (dependent group; see the primary)
- 4: apply — check 1, Accurate this-chunk addition (a correction of `writes only its per-cycle data dir`: the cycle's engine writes the lock file to the temp dir, measured by the lock-file arm); escalate severity not governing: playbook rule of 2026-08-23 (an escalate-severity detector outside its class) — validation present and tested per Coverage
- 5: apply — check 1, the 2026-08-23 rule: registry completeness for a closed refusal label the report shows validated and tested (unit 2 + integ)
- 6: apply — check 1, the 2026-08-23 rule: the verb reads two more record kinds from a log family it already read, and lets out closed labels only (Coverage: validation ✓, PII n/a). Recorded with NO classification of its own; the plan's Expected amendments lists it under Boundary widening, and the directive (inputs#I5 item 7) asks that the changed decision be registered — brought to the card for the operator's reading of its class
- 7: apply with proposal 6 (dependent)
- 8: apply — check 1, Accurate this-chunk addition (the word arms are pinned; the row's clause (f) closed)
- 9: apply — check 1: a scoping edit of a still-true sentence, needed once proposal 8 lands (the row it cites is no longer wholly open)

## design-system
verdict: 0 proposal(s) · stripped: three comment lines (why neither detector fired) — raw twin kept

`proposals: []`

## layout-templates
verdict: 0 proposal(s) · stripped: four comment lines (why neither detector fired; files read) — raw twin kept

`proposals: []`

## test-plan
verdict: 23 proposal(s) · stripped: a five-line comment preamble (no drift on the runner, the status shape, the log format, the PID file; the obs-plan cross-read found no contradiction)

```yaml
proposals:
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 → 5-command implementation"
    change: "boot → Readiness signal: `harness:ready` reads `ready` only when the status verdict is `running-healthy` AND both receivers read `accepting`; a receiver reads `accepting` when a loopback connection is accepted on the resolved port AND the log family holds, after its last `app.boot.engine` record, an INFO record of that receiver's bind target (`app.boot.otlp.grpc.bind` / `app.boot.otlp.http.bind`) whose `bind_address` is `127.0.0.1:` and that port; `foreign` when the connection is accepted and no such record stands (a failed-bind ERROR record, a record for another port or the other receiver, a record before the last boot record, or a log with no boot record); `refusing` when no connection is accepted — each receiver label is `accepting` | `foreign` | `refusing` (was two); verdict words, exits and the six members unchanged; holds for both programs; `decide_ready` takes two `Receiver` values and carries 8 more pins; `agent-run.ps1` is not edited."
    sidecar: "2026-10-10-engine-harness-proofs-completed: §3 5-command implementation, boot Readiness signal — ready now needs each accepting port to be this run's own bound record in the log; third receiver label `foreign`."
    rationale: "Report Changes → Symbols / APIs (The readiness decision, as built), Schema / config (receiver labels 2 → 3) and Harness / gate surface (`cargo xtask harness:ready`). The key still says ready is `running-healthy` AND a TCP connection accepted at each port, with labels `accepting` | `refusing`. obs-plan §3 holds no statement of the readiness decision or the labels, so the two docs do not contradict; test-plan §3 alone is stale."
    basis: "xtask/src/harness_ready.rs:172-180 (enum Receiver), 182-198 (impl Receiver), 200-227 (bound_by_this_run)"
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 → 5-command implementation"
    change: "boot → Exit code: on the readiness failure path the sh verb prints, when the last readiness verdict held a `refusing` label, the unchanged `the receivers never both accepted: …` line, and, when it held a `foreign` label, a second line `a receiver port accepts but this run's log holds no bound record for it: OTLP gRPC 127.0.0.1:{port}, OTLP HTTP 127.0.0.1:{port}` — both can print; the lines are no longer driven-once-by-hand: three committed pins in `xtask/tests/agent_run_shell.rs` read the receivers line, neither line, and the foreign line (the foreign arm ends `cleanup: ports-lingering`, the test's listener still holding the port); `agent-run.ps1` is not edited and its readiness branch still matches `refusing` alone, so it prints no line for `foreign`. The exit-witness refusal's no-committed-test clause stays."
    sidecar: "2026-10-10-engine-harness-proofs-completed: §3 5-command implementation, boot Exit code — foreign failure line added (sh only); the receivers-line arms are committed pins."
    rationale: "Report Changes → Symbols / APIs (`scripts/agent-run.sh` 196-200) and Harness / gate surface (`scripts/agent-run.sh boot`: one more failure line; ps1 matches `refusing` alone); Outcome criterion 1 and The four pending rows (b). The label states the receivers line prints only on `refusing`, names one line, and calls it driven once by hand with no committed test."
    basis: "scripts/agent-run.sh:196-200; xtask/tests/agent_run_shell.rs:306-393"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§1 Test Scope Summary → Test harness requirements (the one-line harness summary)"
    change: "The `boot` clause reads: ready is the `cargo xtask harness:ready` verdict for the program asked — the status verdict `running-healthy` AND, on each resolved OTLP port (`:4317`/`:4318` by default), a TCP connection accepted that this run's own log shows bound (an accepting port with no bound record of this run reads `foreign`, not ready): §3 → 5-command implementation."
    sidecar: "2026-10-10-engine-harness-proofs-completed: §1 harness one-line summary — readiness restated with the own-bound-record condition."
    rationale: "Same claim as the Readiness signal label, restated in the §1 summary (`running-healthy` AND a TCP connection accepted on both resolved OTLP ports). Report Changes → Symbols / APIs (The readiness decision, as built): an accepted connection alone no longer reads ready."
    basis: "xtask/src/harness_ready.rs:200-227"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 → Per-chunk gate discipline"
    change: "`pre-push:linux` paragraph, sixth stage: `ci-gates` no longer reads a seed log it wrote itself (`SEED_LOG` and `seed` are removed); stage six, in order: the stage data dir removed and created again; either cycle port (24317, 24318) accepting stops the run `cannot-evaluate` / `engine-port-taken` with nothing started; `cargo xtask harness:engine-cycle --data-dir {that dir}` under the stage child set; then both ports and the pid of that data dir's pid file are read — one still accepting or alive → the script's `cleanup` runs once on that data dir and the stage is red (`stage-failed:ci-gates`) whatever the cycle returned; else cycle exit 2 → `cannot-evaluate` / `engine-cycle-unevaluated`; exit 0 → `cargo xtask ci-gates` over the same data dir decides the stage; any other exit → red. Stage names, order and the six verdict members unchanged. Readings added: four green on the warm dev host, 107.04 to 110.44 s with the engine already built in release, stage six 18.8 to 19.0 s over the log family of a real console engine cycle (335 records each); no cold release build timed. PROVISIONAL — the operator's reading (the pc overseer, 2026-10-10), the founder's own word owed."
    sidecar: "2026-10-10-engine-harness-proofs-completed: §3 Per-chunk gate discipline, pre-push:linux — stage six reads a real console engine cycle in place of a seed log (PROVISIONAL, founder's word owed)."
    rationale: "Report Changes → Harness / gate surface (`cargo xtask pre-push:linux`), Symbols / APIs (`xtask/src/pre_push.rs`: `SEED_LOG` and `seed` removed, `read_engine_cycle`, `decide_ci_gates`, `end_cycle`) and Counts / qualifiers moved (what stage six reads; wall-clock 89 s → 107.04–110.44 s). The key file still says the stage runs over a data dir holding only a seed log of non-metric records and reads a record it wrote itself. Report Decisions → Carried for the second window, Item 4, fixes the wording duty (what the stage does now, operator's reading, founder's word owed)."
    basis: "xtask/src/pre_push.rs:322-369 (read_engine_cycle), 305-320 (decide_ci_gates), 375-383 (the cleanup call)"
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 → Per-chunk gate discipline"
    change: "`pre-push:linux` paragraph, the sentence `it binds no port and starts no pulse-app` now reads: the verb installs nothing and prints no install command; it starts no `pulse-app`, and since chunk 2026-10-10-engine-harness-proofs-completed its sixth stage starts `andromeda-pulse-engine` and binds loopback 24317 and 24318 (never 4317 or 4318) — PROVISIONAL, the operator's reading (the pc overseer, 2026-10-10), the founder's own word owed."
    sidecar: "2026-10-10-engine-harness-proofs-completed: §3 Per-chunk gate discipline, pre-push:linux — the no-port / no-program sentence amended: stage six starts the console engine on 24317 / 24318 (PROVISIONAL)."
    rationale: "Report Changes → Harness / gate surface: `The check now starts andromeda-pulse-engine and binds loopback 24317 and 24318`, marked PROVISIONAL, the founder's own word owed (Boundary widening). The key file's sentence is the same retired claim as the seed-log clause, stated as a ban; the report's Expected amendments counts it (the no-port sentence, 1 hit in this key file)."
    basis: "xtask/src/pre_push.rs:322-369"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 → Per-chunk gate discipline"
    change: "`pre-push:linux` paragraph, `Reasons, complete:` lists eleven: `all-stages-ok` · `not-linux` · `pins-unreadable` · `home-unset` · `provisioning-missing` · `run-dir-unusable` · `tree-unreadable` · `stage-failed:{stage}` · `restore-failed:bindings` · `engine-port-taken` · `engine-cycle-unevaluated`; a stage that stops `cannot-evaluate` is listed with `ok` false."
    sidecar: "2026-10-10-engine-harness-proofs-completed: §3 Per-chunk gate discipline, pre-push:linux — closed reasons 9 → 11 (`engine-port-taken`, `engine-cycle-unevaluated`)."
    rationale: "Report Changes → Symbols / APIs (`xtask/src/pre_push.rs`: two new closed reasons) and Counts / qualifiers moved (reasons of `pre-push:linux`: 9 → 11). The key file's list is headed `complete` and holds nine."
    basis: "xtask/src/pre_push.rs:305-320"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 → Per-chunk gate discipline"
    change: "`pre-push:linux` paragraph, the child-environment sentence: every probe and stage child still gets a cleared environment plus exactly `HOME`, `PATH`, `ANDROMEDA_PULSE_DATA_DIR` (and `PUPPETEER_CACHE_DIR` for `npm`), with one exception — the sixth stage's single `cleanup` child also carries the two cycle port variables, so its probe never touches 4317 or 4318; the cycle's engine writes its corpus-key lock file to the temp dir (the cycle's child set holds no runtime dir)."
    sidecar: "2026-10-10-engine-harness-proofs-completed: §3 Per-chunk gate discipline, pre-push:linux — the stage-six cleanup child carries the two cycle port variables beyond the stage child set."
    rationale: "Report Changes → Harness / gate surface (`The stage child set is unchanged …; the one cleanup child also carries the two cycle port variables …; The cycle's engine writes its corpus-key lock file to the temp dir`) and Deviations (unplanned additions: the stage's cleanup child carrying the two cycle port variables beyond the stage child set). The key file says every child gets `exactly` the set."
    basis: "xtask/src/pre_push.rs:375-383"
    dependent-of: D-tests-obs-harness
  - detector: D-tests-obs-harness
    severity: warning
    section: "§3 → Per-chunk gate discipline"
    change: "Engine cycle gate form: the refusal list gains an eighth label — `port-taken`: either cycle port already accepting a connection, checked after the existing refusals and before the `HOME` read (so 4317 and 4318 are refused before any port is probed); stderr `harness:engine-cycle: cannot-evaluate (port-taken)`, the verdict object with nulls, exit 2, nothing built or started, no data dir made. The form is also run by `pre-push:linux`'s sixth stage (above), and `engine_cycle::drive` now runs under committed tests (`xtask/tests/engine_cycle_refusals.rs`: five refusal labels and one whole failing cycle on a real engine that ends the engine it started)."
    sidecar: "2026-10-10-engine-harness-proofs-completed: §3 Per-chunk gate discipline, Engine cycle gate form — refusal labels 7 → 8 (`port-taken`); the form gains pre-push stage six as a caller."
    rationale: "Report Changes → Symbols / APIs (`xtask/src/engine_cycle.rs`: `taken_port_refusal`, new closed label `port-taken`), Counts / qualifiers moved (refusal labels 7 → 8) and Harness / gate surface (`cargo xtask harness:engine-cycle`: one more refusal before anything starts). The form lists seven refusals and names its uses as the dev-host gate block and the `boot` job's step only."
    basis: "xtask/src/engine_cycle.rs:122-127 (taken_port_refusal), 243-250 (its call in drive); xtask/tests/engine_cycle_refusals.rs:133-155, 157-191"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Pending coverage triggers → row `engine-boot-rejected-port-and-socket-census-coverage`"
    change: "LANDED 2026-10-10-engine-harness-proofs-completed — no longer pending: all three owed readings are spawned arms of `pulse-app/tests/integration_console_engine.rs`, each read red first — (1) a rejected gRPC port variable reads the failed-bind record with `invalid_port`, a refused connection on that port and the HTTP receiver alone in the census; (2) a socket census from the process's own socket list reads exactly the two receivers on `127.0.0.1` and nothing else; (3) the corpus-key lock file sits in the directory the environment gives, in both environments (with and without a runtime dir), zero bytes, mode `0o600`, nothing under the data dir, its name and directory 0 times in the log family. The suite's arms go 4 → 7. Not covered by this row and not owned: `pulse-app/tests/integration_engine_boot.rs` drives no rejected port in process."
    sidecar: "2026-10-10-engine-harness-proofs-completed: §1 trigger `engine-boot-rejected-port-and-socket-census-coverage` — LANDED (three spawned arms)."
    rationale: "Report Outcome → The four pending rows (all three owed clauses closed, a spawned arm each), criteria 4 and 5, and Changes → Coverage of new surfaces (the three spawned readings, each read red first) and Counts (arms 4 → 7). The row still says the three readings are not committed."
    basis: "pulse-app/tests/integration_console_engine.rs:570-629, 631-655, 657-751"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Pending coverage triggers → row `harness-log-family-resolution-coverage`"
    change: "Shell half LANDED 2026-10-10-engine-harness-proofs-completed — no longer pending: three committed `logs` pins in `xtask/tests/agent_run_shell.rs` drive the script's verb — it prints a record of the date-suffixed member a real engine wrote, it reads the base the log-file variable names before the data dir, and with no base resolving it says so and exits 1. Nothing further owed on this row."
    sidecar: "2026-10-10-engine-harness-proofs-completed: §1 trigger `harness-log-family-resolution-coverage` — shell `logs` half LANDED (three pins, one on a real engine's family)."
    rationale: "Report Outcome → The four pending rows (`closed (three logs pins, one on a real engine's family)`) and criterion 1. The row still says the harness-level assertion for the shell `logs` verb is owed and its evidence is a one-time hand check."
    basis: "xtask/tests/agent_run_shell.rs:526-590"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Pending coverage triggers → row `engine-log-arm-mutation-and-engine-cycle-glue-coverage`"
    change: "Narrowed 2026-10-10-engine-harness-proofs-completed: (1) one mutation per arm of `check:engine-log` — closed: seven of seven, each one line, each reddening pins (6, 3, 7, 9, 7, 8, 8 of 45) and changing the arm's line over one real kept log (n = 1, the passing side of each arm; `evidence/mutations.md`), 45 of 45 after the reverts. (2) The refusal labels are eight (`port-taken` added) and every one is named by a test: three through `refusal`, five by `xtask/tests/engine_cycle_refusals.rs` (`home-unset`, `path-unset`, `data-dir-unusable`, `injector-build-failed`, `port-taken`); `engine_cycle::drive` runs under committed tests (those five refusals and one whole failing cycle on a real engine). Still owed: a runner reading of a red cycle when one occurs; `engine_log::run_check` and `run_settled` have no pin of their own beyond the cycle."
    sidecar: "2026-10-10-engine-harness-proofs-completed: §1 trigger `engine-log-arm-mutation-and-engine-cycle-glue-coverage` — mutations and refusal-label pins closed; open: a runner red cycle, `run_check` / `run_settled` pins."
    rationale: "Report Outcome → The four pending rows, Mutations and Limits (one mutation per arm, passing side, n = 1); Changes → Counts (refusal labels 7 → 8). The row says no mutation was run, `drive` has no pin, and of `the seven refusal labels` four are named by no test. The heartbeat pin that stayed green under the inverted comparison is the card's (found and not owned), not written into the row here."
    basis: "xtask/tests/engine_cycle_refusals.rs:79-155 (the five label pins), 157-191 (the failing cycle)"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Pending coverage triggers → row `harness-cleanup-verdict-and-boot-spawn-shell-coverage`"
    change: "Narrowed 2026-10-10-engine-harness-proofs-completed (19 committed pins in `xtask/tests/agent_run_shell.rs`, the committed sh script run by the real `bash` on a real console engine): closed — (a) a pin per `cleanup` token (`clean` twice over, `no-pid-ports-accepting`, `ports-lingering`, `app-survived`) plus `clean` after a real boot; (b) a pin per sh `boot` failure-reason arm (the engine build failing, the xtask build failing, the built binary absent, no spawn record, never ready with the receivers line / with neither line / with the foreign line, killed during the poll); (f) a pin per word arm (bare `boot`; `status engine` and bare `status`; any other word → usage, exit 2, no data dir). Still open, with their owners on the route: (c) the `.ps1` boot-cycle leg, `Invoke-Ready` and the absence of a `foreign` line in `agent-run.ps1` included (`Other operating systems retired from the code`, `working-route.md:64`); (d) the exit-witness branches and (e) the boot series' `timed-out` path (`Window's gates retired`, `working-route.md:50`). Limits: the sh pins boot the console engine; for the window program the `foreign` reading is pinned at the decision and not read live. Two behaviours are pinned as they stand, owner to be named at the card: `cleanup` reads an unreaped child as alive (`app-survived`) while `harness:status` reads such a pid as dead; `boot` with no spawn record exits 1 and leaves the engine it started running."
    sidecar: "2026-10-10-engine-harness-proofs-completed: §1 trigger `harness-cleanup-verdict-and-boot-spawn-shell-coverage` — clauses (a), (b), (f) closed by `xtask/tests/agent_run_shell.rs`; (c), (d), (e) stay open with route owners."
    rationale: "Report Outcome → The four pending rows and criterion 1 (the tests listed per arm), criterion 6 (window `foreign` pinned at the decision, not read live); Decisions → Found and not owned (the unreaped-child and no-spawn-record pins); Carried for the second window, Item 7 (the shell-coverage row stays open on its three clauses with their owners). The row still says the `cleanup` verdict, the `boot` arms, the receivers line and the word arms ship with NO committed test."
    basis: "xtask/tests/agent_run_shell.rs:107-161 (cleanup tokens), 163-194, 196-242, 261-426 (boot arms), 244-259 and 428-476 (word arms)"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → 5-command implementation"
    change: "boot → Command body: the clause `The word is sh only …, has no committed shell-level test (the same §1 trigger)` now reads: the word is sh only (`agent-run.ps1` is unchanged) and since chunk 2026-10-10-engine-harness-proofs-completed is pinned at shell level by `xtask/tests/agent_run_shell.rs` — `boot engine` reading ready and `cleanup` ending the engine clean, bare `boot` resolving the window program, `status engine` on a running engine, any other word after `boot` or `status` → usage, exit 2, no data dir — and is still read end to end by `cargo xtask harness:engine-cycle`. The exit-witness arm's and the ps1 leg's no-committed-test clauses stay (the §1 trigger's open clauses)."
    sidecar: "2026-10-10-engine-harness-proofs-completed: §3 5-command implementation, boot Command body — the `engine` word and `boot engine` spawn arm now carry committed shell-level pins."
    rationale: "Same retired claim as the §1 row (`no committed shell-level test` for the word and the `boot engine` arm), restated in the key file. Report Outcome → The four pending rows: (f) per word arm closed; criterion 1."
    basis: "xtask/tests/agent_run_shell.rs:163-194, 244-259, 428-476"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Pending coverage triggers → row `quarantine-tracking-ps1-mirror-coverage`"
    change: "Widened 2026-10-10-engine-harness-proofs-completed: the `.ps1` mirror gained the fifth search dir, `xtask/tests`, as the `.sh` did, again with NO committed test — parsed by `pwsh` (0 parse errors; its version not re-read), never run; on the `.sh` side the new entry is pinned (`quarantine_check_reads_the_xtask_integration_tests`, `xtask/src/main.rs`), and a missing `xtask/tests` now fails the check. Owed line unchanged."
    sidecar: "2026-10-10-engine-harness-proofs-completed: §1 trigger `quarantine-tracking-ps1-mirror-coverage` — widened by the `xtask/tests` search-dir entry (ps1 parsed, not run)."
    rationale: "Report Changes → Files (`xtask/ci/quarantine-tracking-check.ps1` modified), Dev-tool versions (`pwsh` parsed the `.ps1` mirror, 0 parse errors), Harness / gate surface (the `.ps1` mirror takes the same entry) and Outcome → Limits (`The .ps1 mirror was parsed, not run`). A new line of the mirror with no test at any tier; the row is where the doc carries that gap."
    basis: "xtask/ci/quarantine-tracking-check.ps1:14-15; xtask/src/main.rs:1021-1031"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Pending coverage triggers (new row `pre-push-engine-cycle-stage-glue-coverage`)"
    change: "NEW 2026-10-10-engine-harness-proofs-completed. `pre-push:linux`'s sixth stage is pinned at its pure decision (`decide_ci_gates`, 7 pins) and by one pin that runs the stage's own teardown against a real process (`end_cycle_ends_the_process_the_stage_data_dir_s_pid_file_names`); the glue `pre_push::read_engine_cycle` has no pin of its own and is read only by the check itself (four green readings on the dev host, not a committed test and not a CI step). Not read live and not pinned beyond the decision: a taken port on 24317 / 24318 (the `engine-port-taken` wiring of the stage's probe) and the stage's teardown after a cycle that left an engine. Owed: a committed reading of the stage over a taken cycle port and over a cycle that leaves its engine running."
    sidecar: "2026-10-10-engine-harness-proofs-completed: §1 new trigger `pre-push-engine-cycle-stage-glue-coverage` — `read_engine_cycle` glue pinned at the decision only."
    rationale: "Report Changes → Coverage of new surfaces (`pre-push:linux` stage six: `Not read live: a taken port on 24317 / 24318, and the stage's teardown after a cycle that left an engine`) and Outcome → Limits (`the stage's engine-port-taken wiring (read_engine_cycle's probe) is pinned at the decision only`). A new path whose integration-tier reading is not committed; the same class the glue row records for `engine_cycle::drive`. Proposed as its own row so it does not overlap the edit of the engine-cycle glue row."
    basis: "xtask/src/pre_push.rs:322-369 (read_engine_cycle), 305-320 (decide_ci_gates), 1091-1132 (the teardown pin)"
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Per-chunk gate discipline"
    change: "Spawned-program witness form: `two picked loopback ports` reads `two claimed loopback ports` — a number from 20000 to 31999 (24317 and 24318 left out), held for the test by an advisory lock (`File::try_lock`) on a file under the git-ignored `target/test-ports/` (never unlinked), shown bindable when claimed, never bound-to-0-and-released; what the engine is handed is unchanged in kind. The form's readings gain three, each a spawned arm: a rejected port variable (the failed-bind record `invalid_port`, a refused connection, the other receiver alone listening), a socket census of the process read from `/proc` (exactly the two loopback receivers), and the corpus-key lock file's directory, size and mode with 0 occurrences in the log family. `integration_console_engine.rs` holds 7 arms (was 4); 4317 and 4318 are bound by no test."
    sidecar: "2026-10-10-engine-harness-proofs-completed: §3 Per-chunk gate discipline, Spawned-program witness form — ports are claimed under an advisory lock, not picked; three readings added (rejected port, socket census, lock-file directory)."
    rationale: "Report Changes → Harness / gate surface (Test-side surface: every port … is claimed, not bound-to-0-and-released; `This changes how the four existing spawned arms take their ports`), Reverted / negative API facts (bind-to-0 picking replaced by `claim_port` / `PortClaim`), Counts (arms 4 → 7) and Coverage of new surfaces (the three spawned readings). The form describes two picked ports and lists none of the three readings."
    basis: "pulse-app/tests/integration_console_engine.rs:35-43 (PortClaim), 449-493 (socket_census), 570-629, 631-655, 657-751"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary → Surfaces under test → row `console engine program`"
    change: "Driver cell: `its own TempDir data dir and two picked loopback ports` reads `its own TempDir data dir and two claimed loopback ports (a number from 20000 to 31999 held under an advisory lock in target/test-ports/, 24317 and 24318 left out; §3 → Per-chunk gate discipline, Spawned-program witness form)`; Signal cell gains: the process's socket census (exactly the two loopback receivers), a rejected port variable's failed-bind record, and the corpus-key lock file's directory."
    sidecar: "2026-10-10-engine-harness-proofs-completed: §1 Surfaces under test, console engine program — ports claimed, not picked; census / rejected-port / lock-file signals named."
    rationale: "Same retired claim as the Spawned-program witness form (`two picked loopback ports`), restated in the §1 surface table. Report Changes → Harness / gate surface (Test-side surface) and Deviations (`Port claims replace bind-and-release picking in three test binaries, the four existing spawned arms included`)."
    basis: "pulse-app/tests/integration_console_engine.rs:35-43"
    dependent-of: D-tests-coverage
  - detector: D-tests-framework
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions → Directory pattern"
    change: "A second exception beside `pulse-app`: since chunk 2026-10-10-engine-harness-proofs-completed the `xtask` package has a `tests/` directory holding its first integration tests — two test binaries, `xtask/tests/agent_run_shell.rs` and `xtask/tests/engine_cycle_refusals.rs`, and a shared module `xtask/tests/common/mod.rs` (`#![allow(dead_code)]`, each binary using a part of it) — because they drive the committed script, the built `harness:engine-cycle` verb and the built engine as processes, which a co-located `mod tests` cannot; xtask's unit pins stay co-located in `xtask/src`. The directory is read by `cargo xtask check:english-sources` (a sixth root) and `cargo xtask quarantine-tracking` (a fifth search dir), and its absence fails both. Framework and runner are unchanged: libtest under `cargo nextest run --workspace --profile ci`, no dependency added (`tempfile` and `serde_json` were already normal dependencies of the package)."
    sidecar: "2026-10-10-engine-harness-proofs-completed: §2 Directory pattern — `xtask/tests/` is the second separate tests directory (two integration binaries + `common/mod.rs`)."
    rationale: "Report Changes → Crates / modules (`The xtask package gains its first integration tests: two test binaries … and a shared module`), Counts (xtask integration test binaries 0 → 2; lint roots 5 → 6; quarantine search dirs 4 → 5), Dependencies (none added). §2 states tests are co-located under each crate's `src/` with no separate `tests/` directory EXCEPT `pulse-app`. The runner the chunk used (the Outcome's gates) matches §3 `run` and §4; only the location convention is off-spec. The report's Expected amendments notes `xtask/tests` has 0 hits in the masters."
    basis: "xtask/src/source_lint.rs:24 and 30 (ROOTS); xtask/src/main.rs:939-945 (QUARANTINE_SEARCH_DIRS)"
  - detector: D-tests-framework
    severity: warning
    section: "§2 Test Strategy → Test directory + naming conventions → Test function naming"
    change: "…per-module test submodules live in `src/` for the library crates and for `xtask`'s unit pins, while pulse-app's are `#[test]` fns in `pulse-app/tests/*.rs` and xtask's process-level pins are `#[test]` fns in `xtask/tests/*.rs`."
    sidecar: "2026-10-10-engine-harness-proofs-completed: §2 Test function naming — `xtask/tests/*.rs` named beside `pulse-app/tests/*.rs`."
    rationale: "Same claim as the Directory pattern bullet (only pulse-app's tests live outside `src/`), restated one bullet below. Report Changes → Crates / modules."
    basis: "xtask/tests/agent_run_shell.rs (new file · 590 lines), xtask/tests/engine_cycle_refusals.rs (new file · 191 lines)"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§4 Unit Test Strategy → Conventions → Test file location → Rust"
    change: "`pulse-app` is no longer the one exception named: add that the `xtask` package keeps its unit pins co-located in `xtask/src` and, since chunk 2026-10-10-engine-harness-proofs-completed, holds its integration-tier pins of the script, the cycle verb's refusals and a failing cycle in `xtask/tests/` (§2 Directory pattern)."
    sidecar: "2026-10-10-engine-harness-proofs-completed: §4 Test file location — `xtask/tests/` named as the second out-of-`src` location."
    rationale: "Same claim as §2 Directory pattern (`pulse-app is the exception`), restated in §4's conventions. Report Changes → Crates / modules."
    basis: "xtask/tests/common/mod.rs (new file · 420 lines)"
    dependent-of: D-tests-framework
  - detector: D-tests-framework
    severity: warning
    section: "§9 CI Integration → Pipeline structure → row `Lint + tests (lint-test job)`"
    change: "Add to the row's what-the-tests-need run: since chunk 2026-10-10-engine-harness-proofs-completed the workspace run also needs `bash`, `/proc`, an advisory file lock under `target/` (`target/test-ports/`) and the engine built in the test profile (a `--workspace` run builds it) — the two `xtask/tests` binaries (25 tests) and the three new spawned arms of `integration_console_engine.rs`; no setup step and the `ci` workflow is not edited; the same tests run in the `mcp-server tests` and `coverage gate` jobs; green on `ci#38079305603` (the failing-cycle pin passed in `lint / test`, where only the test profile is built, by running the committed script from a scratch root that links the built engine)."
    sidecar: "2026-10-10-engine-harness-proofs-completed: §9 lint-test row — the workspace run now needs `bash`, `/proc`, a file lock under `target/` and the test-profile engine."
    rationale: "Report Changes → Harness / gate surface (`The ci workflow is not edited. Its lint / test, mcp-server tests and coverage gate jobs now run the new tests, which need bash, /proc, an advisory file lock under target/ and the engine built in the test profile`), Cross-project / external claims (`ci#38079305603` green) and Decisions (the pin passed in the `lint / test` job, 2.1 s). §9's row enumerates what the job's tests need on the runner and does not hold these."
    basis: "xtask/tests/common/mod.rs:59-64 (require_proc), 35-49 (engine_bin), 87-113 (claim_port), 334-350 (stand_in_bash)"
  - detector: D-tests-framework
    severity: warning
    section: "§9 CI Integration → Pipeline structure → row `Integration tests`"
    change: "Parallel cell: `partial (per-process isolation prevents port conflicts)` reads `partial — per-process isolation separates processes, not port numbers: a port bound to 0, read and released can be handed to another test starting at the same instant (as measured at chunk 2026-10-10-engine-harness-proofs-completed: one of seven hand runs of the three port-using binaries failed one arm on a picked port another process held; after the change 14 of 14 hand runs green — a count, not an exclusion); the binaries `agent_run_shell`, `engine_cycle_refusals` and `integration_console_engine` therefore claim each port under an advisory file lock in `target/test-ports/`; `pulse-app/tests/integration_engine_boot.rs` still picks by bind-and-release (owner: none named)`."
    sidecar: "2026-10-10-engine-harness-proofs-completed: §9 Integration tests row — per-process isolation does not prevent port conflicts; three binaries claim ports under a file lock."
    rationale: "Report Changes → Reverted / negative API facts (bind-to-0 picking written, then replaced: one of seven hand runs failed), Harness / gate surface (Test-side surface) and Insufficient fixes (`integration_engine_boot.rs` still picks by bind-and-release; owner none named); Decisions (measured: 20 000 binds, about 70 draws at one instant; 14 of 14 after). The row asserts the runner's per-process isolation prevents port conflicts, which the chunk measured false under that runner."
    basis: "xtask/tests/common/mod.rs:66-74 (PortClaim), 87-113 (claim_port); pulse-app/tests/integration_console_engine.rs:35-43"
  - detector: D-tests-framework
    severity: warning
    section: "§3 → Bootstrap phases"
    change: "Item 1 (test-runner-install): the parenthesis `provides per-process isolation for port-binding tests` gains: isolation of processes only — it does not keep two tests from being handed one port number; the port-using spawned suites claim their ports (§9 Integration tests row)."
    sidecar: "2026-10-10-engine-harness-proofs-completed: §3 Bootstrap phases item 1 — per-process isolation qualified: it does not allocate ports."
    rationale: "Softer restatement of the §9 claim (nextest's per-process isolation as the answer for port-binding tests). Report Changes → Reverted / negative API facts and Harness / gate surface (Test-side surface). Offered so the claim does not survive in the key file after the §9 row is amended; the orchestrator may judge the sentence still true as worded."
    basis: "xtask/tests/common/mod.rs:87-113"
    dependent-of: D-tests-framework
```

Dispositions:
- 1: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 2: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 3: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 4: apply with its primary, in the PROVISIONAL form — a restating site of the stage-six widening (dependent group; see the primary) (the stage-six order and its readings are this chunk's)
- 5: escalate — check 1, playbook rule Boundary widening (never routine); RESOLVED on the operator's recorded direction: "The pre-push sixth stage is PROVISIONAL … the amended sentence says what the stage does now … that it is the operator's reading (the pc overseer, 2026-10-10), and that the founder's own word is owed" — the operator (the pc overseer), 2026-10-10, relayed verbatim in the wrap directive (inputs#I5 item 4; the P4 answer is inputs#I3). Applied in that PROVISIONAL form; listed at the route-resolve card
- 6: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 7: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 8: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 9: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 10: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 11: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 12: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 13: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 14: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 15: apply — check 1, playbook rule of 2026-08-28 (recording a measured open gap as OPEN): the report's Coverage and Limits carry it; a new pending row needs an owner on the route, named at the route-resolve card
- 16: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 17: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 18: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 19: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 20: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 21: apply — check 1, playbook rule Accurate this-chunk addition (the named thing is in the report's Changes)
- 22: apply — check 1, playbook rule of 2026-08-14 (a doc claim a measurement disproved, the impl already correct for the three binaries) with the remainder recorded as open (`integration_engine_boot.rs`, owner at the card)
- 23: apply — check 1: a qualifier on a sentence that stays true as worded, so the §9 claim does not survive in the key file

## obs-plan
verdict: 0 proposal(s) · stripped: a comment block: why none of the four detectors fired, and TWO notes for the orchestrator bound by no detector — (1) §9's `xtask test` row and its triage paragraph say one test binary spawns the console program, where three now do; (2) the plan's Expected amendments for §10 are additive. Both are raised by the orchestrator below — raw twin kept

`proposals: []`

## a11y-plan
verdict: 0 proposal(s) · stripped: a comment block (why neither detector fired; the sweep for collateral claims, 0 real hits) — raw twin kept

`proposals: []`

## Raised by the orchestrator (check 5, Expected-amendments reconciliation; check 6)

- **obs-plan §10 CI gates** — the plan's entry "what the pre-push stage grades": no detector proposed it (additive).
  Raised routine-by-report, in the PROVISIONAL form of the stage-six widening (the same resolved escalation):
  `check:engine-log` and `ci-gates` are now also run by `pre-push:linux`'s sixth stage over a real cycle's log.
- **obs-plan §10 Load-profile constraints** — the plan's entry "the mutation readings per arm of `check:engine-log`":
  raised routine; the report's Outcome ("Mutations") substantiates it (seven of seven, n = 1 log, the passing side).
- **obs-plan §9 Pipeline integration (`xtask test` row) and §9 CI failure → artifact triage** — the obs detector's
  note 1: "one test binary of the run, `integration_console_engine`, spawns the console program" is stale (three
  binaries now start a console engine: report Crates / modules, Harness / gate surface). Raised routine, two sites.
- **architecture §Stack → Gate-script pins and §xtask CLI surfaces → `quarantine-tracking`** — "five pins" of the
  quarantine script: the report adds one pin for the new directory (`xtask/src/main.rs` 1021-1031); six pins as
  counted in `mod empty_input_tests` at this wrap. Raised routine with architecture proposals 22 and 24.
- **matrix#P-103 notes** — ledger-note form: dispositioned at P7.3, never by a proposal.
- **Spec claims disproved by measurement** (check 6): both entries are statements of this chunk's `plan.md`, a chunk
  artifact — a report entry and no edit; DISPOSED.
- **Citation sweep rows**: none dispositioned `claim false`.
- **Check 2** (cross-contradiction): no two proposals edit one section in opposing directions.
- **Check 3** (intent-consistency): the deviations are the report's own, accepted by the operator (inputs#I4); the
  scope record is empty (`gate.py scope` clean).
- **Check 4** (absence needs evidence): every "0 hits" a proposal cites is the report's own count
  (`scratchpad/master_hits.py`, one regex per row); the cascade sweep re-reads them after the apply.
- **Rejected for a source the report does not carry:** 0 — every coordinate in a `change`, `rationale` or `basis`
  was checked against the report's bullets and its "New text, by line" section.

Totals: 56 proposals · apply 55 · reject 1 (architecture 23, Registry over-reach) · escalated and resolved 1 group (the stage-six boundary widening: architecture 7, security-plan 1, test-plan 5 as primaries, with their dependents) · rejected for a source the report does not carry 0.
