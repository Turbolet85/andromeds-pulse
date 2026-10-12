# Fan-out results - 2026-10-10-shared-telemetry-test-data

Seven doc-agents, one batch, the second window of this wrap. Each return was taken from the agent's final
hand-back by script and entity-decoded; the probe read `entities=0` on all seven before and after the decode.
Detectors sent: 19, one prompt per doc (architecture 2, security-plan 4, design-system 2, layout-templates 2,
test-plan 3, obs-plan 4, a11y-plan 2), equal to the 19 `doc:` names of the drift-base.

Proposals: 27 (architecture 8, security-plan 1, test-plan 18, the other four 0).
Rejected for a source the report does not carry: 0 - every coordinate a proposal cites is a line the report
states or a row or added range of its last section.

## architecture

verdict: proposals 8 - nothing stripped

### architecture proposal 1

```yaml
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Cargo workspace crate names"
    change: >-
      Add a seventeenth reserved name, `test-support`, after `xtask`, and say what it is: the workspace member `crates/test-support` (package `test-support`, `publish = false`, inheriting the workspace `version`, `edition`, `rust-version`, `license`, `repository`), a library crate with modules `otlp` (telemetry test factories) and `ports` (the test port claim), taken as a dev-dependency only — `buffer`, `pulse-app` and `xtask` name it under `[dev-dependencies]`, it names `ingest` under `[dependencies]` and has no other table, and no product target links it (chunk 2026-10-10-shared-telemetry-test-data). The closing sentence stays: all seventeen names are reserved at the workspace level.
    sidecar: >-
      chunk 2026-10-10-shared-telemetry-test-data — §Occupied Resources → Cargo workspace crate names: registered the seventeenth workspace member `test-support` (dev-only test-support library; modules `otlp`, `ports`).
    rationale: >-
      Report Changes → "Crates / modules": "added the workspace member `crates/test-support` … a library crate taken as a dev-dependency only"; "Counts / qualifiers moved": workspace members 16 → 17. The report's own site count reads `test-support`: architecture 0+0 (named nowhere), and the crate-names registry lists sixteen names. A landed workspace crate absent from the registry is the detector's drift; §Design Philosophy names this list the canonical workspace-member list.
    basis: "Cargo.toml:18 (the added member line; the report reads the member list at 17 entries) · crates/test-support/Cargo.toml (new file, 14 lines)"
```

disposition: apply - check 1: Accurate this-chunk addition (the member is the report's Crates / modules; the registry enumerates crate names) - check 5: the plan's first entry

### architecture proposal 2

```yaml
  - detector: D-arch-resources
    severity: warning
    section: "§Design Philosophy → Single-process modular monolith"
    change: >-
      The parenthesis should read seventeen workspace members total: the fourteen library crates the product is wired from + the `pulse-app` binary crate + the `xtask` task-runner crate + the dev-only test-support library `test-support`, which no product target links and the shared engine boot does not wire. "Fourteen library crates … are wired by ONE shared engine boot" stays as the count of the wired crates; `test-support` is a fifteenth library crate by form only.
    sidecar: >-
      chunk 2026-10-10-shared-telemetry-test-data — §Design Philosophy: member count sixteen → seventeen; "fourteen library crates" kept as the count of the crates the product is wired from, the dev-only `test-support` named beside them.
    rationale: >-
      Report "Counts / qualifiers moved": members 16 → 17, site `architecture.md:4`; "The new member is a fifteenth library crate by form and is not one the product is wired from: it is dev-only. Whether the count word moves, and to what, is the amendment's wording." The sentence's predicate is wiring by the engine boot, which the new member is outside of ("a crate no product target links"), so fourteen holds for the subject and only the total moves.
    basis: ".andromeda/architecture.md:4"
    dependent-of: D-arch-resources
```

disposition: apply - check 1: Accurate this-chunk addition - check 5: the plan's first entry. Wording decided here: "fourteen library crates" stays as the crates the shared engine boot wires; the total reads seventeen and names the dev-only member

### architecture proposal 3

```yaml
  - detector: D-arch-resources
    severity: warning
    section: "§Stack and Technologies → Primary language / runtime"
    change: >-
      "(all 16 members inherit it)" should read "(all 17 members inherit it)".
    sidecar: >-
      chunk 2026-10-10-shared-telemetry-test-data — §Stack and Technologies → Primary language / runtime: members inheriting the declared `rust-version` 16 → 17.
    rationale: >-
      Report "Counts / qualifiers moved" lists `architecture.md:14` as a site stating the member count as current; "Crates / modules" says the new member inherits the workspace `rust-version`. Same retired claim as the primary (sixteen members).
    basis: ".andromeda/architecture.md:14"
    dependent-of: D-arch-resources
```

disposition: apply - dependent of proposal 1, applied with it

### architecture proposal 4

```yaml
  - detector: D-arch-resources
    severity: warning
    section: "§Infrastructure Patterns → Deployment model"
    change: >-
      In the runtime-topology bullet, "the Tauri process hosting all fourteen library crates" should read "the Tauri process hosting the fourteen library crates the product is wired from" (the workspace's fifteenth library crate by form, the dev-only `test-support`, is in no product process).
    sidecar: >-
      chunk 2026-10-10-shared-telemetry-test-data — keyed contract Deployment model: "all fourteen library crates" qualified to the crates the product is wired from, now that a dev-only library member exists.
    rationale: >-
      Report "Counts / qualifiers moved" names `.andromeda/registries/contracts/architecture/deployment-model.md:4` among the "fourteen library crates" sites (architecture 1+1). The number stays true of what the process hosts; the word "all" no longer holds of the workspace's library crates by form. Lowest-weight proposal of the set: a qualifier, no count moves — the orchestrator may leave it if "library crates" is read as the product's.
    basis: ".andromeda/registries/contracts/architecture/deployment-model.md:4"
    dependent-of: D-arch-resources
```

disposition: reject - no claim is retired at this site: the key file says the Tauri process hosts all fourteen library crates, which stays true of the fourteen that Design Philosophy names; the retired claim (sixteen members) is not stated there, so this is not a surviving duplicate. The detector itself marks it optional

### architecture proposal 5

```yaml
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Network ports (the `:20000`–`:31999` test-only bullet)"
    change: >-
      Replace "the three test binaries … use" and the closing "`integration_engine_boot.rs` still picks by bind-and-release … (owned by the route entry `Shared telemetry test data`)" with the claimants as they stand since chunk 2026-10-10-shared-telemetry-test-data: one definition, `test_support::ports::claim_port` / `PortClaim` (`crates/test-support/src/ports.rs`), on the same terms (a number from 20000 to 31999, 24317 and 24318 left out, an advisory `File::try_lock` on `target/test-ports/{number}` held for the life of the claim, the port shown bindable at the claim, a panic when none can be claimed), serves the four port-using binaries `agent_run_shell`, `engine_cycle_refusals`, `integration_console_engine` and `integration_engine_boot` (its four in-process arms hand the engine two claimed ports each, held for the arm's life), the two xtask unit tests `harness_ready::tests::receiver_probe_tells_a_listening_loopback_port_from_a_closed_one` and `engine_cycle::tests::a_listener_on_a_cycle_port_is_read_as_taken_and_a_closed_one_is_not` (one each) and `test-support`'s own four tests. No test in `integration_engine_boot.rs`, `xtask/src/harness_ready.rs` or `xtask/src/engine_cycle.rs` binds port 0 any more, so the bind-and-release carry is closed; the 23 binds of port 0 that keep their listener are untouched. The measured history of the earlier chunk (one of seven, 14 of 14) stays as dated; no new port range, no port added to the product.
    sidecar: >-
      chunk 2026-10-10-shared-telemetry-test-data — §Occupied Resources → Network ports: test-range claimants three binaries → four port-using binaries + two xtask unit tests + `test-support`'s own tests, through the one `test_support::ports::claim_port`; the `integration_engine_boot` bind-and-release carry closed.
    rationale: >-
      Report "Symbols / APIs" → Ports / sockets: "No new listening socket and no new port range. New claimants of the registered test-only range: the four in-process arms of `integration_engine_boot.rs` … the two xtask unit tests … and the member's own four tests"; "No test in `integration_engine_boot.rs`, `harness_ready.rs` or `engine_cycle.rs` binds port 0 any more (… last line `0`)"; "Counts / qualifiers moved": "`architecture.md:180` names 'the three test binaries' by name". The registry's claimant set and its "still picks by bind-and-release" sentence are both false after the chunk. Note for the orchestrator: the report's Ports bullet says "five test binaries" while enumerating four named binaries plus the xtask unit-test binary plus `test-support`'s own (six by its own list), so this change names the claimants and carries no total.
    basis: ".andromeda/architecture.md:180 · crates/test-support/src/ports.rs:45-74 (`claim_port`), 17-21 (`PortClaim`)"
```

disposition: apply - check 1: Accurate this-chunk addition - check 5: the plan's third entry. The claimants are written by name; the number of claiming test binaries is six by the tree (callers of `claim_port`: six files serving six test binaries), not the report's "five"

### architecture proposal 6

```yaml
  - detector: D-arch-resources
    severity: warning
    section: "§Occupied Resources → Filesystem locations → Test-made port-claim files outside every data dir"
    change: >-
      Replace "written by the three port-using test binaries `agent_run_shell`, `engine_cycle_refusals` (`xtask/tests/common/mod.rs`, `claim_port` / `PortClaim`) and `integration_console_engine`" with: written by every test that calls the one claim, `test_support::ports::claim_port` / `PortClaim` (`crates/test-support/src/ports.rs`; since chunk 2026-10-10-shared-telemetry-test-data the claim is defined in neither `xtask/tests/common/mod.rs` nor `integration_console_engine.rs`) — the four port-using binaries `agent_run_shell`, `engine_cycle_refusals`, `integration_console_engine`, `integration_engine_boot`, two xtask unit tests and `test-support`'s own tests. The directory is found from the `test-support` crate's compile-time manifest directory, two levels under the repository root (`claims_dir`); nothing is read from the environment at run time. The rest stands (ignored by git, never unlinked, removed by `cargo clean`, never product-written); the "about 1 350" count stays the earlier chunk's dated reading.
    sidecar: >-
      chunk 2026-10-10-shared-telemetry-test-data — §Occupied Resources → Filesystem locations: `target/test-ports/` writers three binaries → every caller of the one `test_support::ports::claim_port`; the claim's home moved out of `xtask/tests/common/mod.rs`.
    rationale: >-
      Report "Counts / qualifiers moved": "`architecture.md:234` also says the files are 'written by the three port-using test binaries … (`xtask/tests/common/mod.rs`, `claim_port` / `PortClaim`) and `integration_console_engine`'; the claim is in neither of those two files now." "Symbols / APIs" → Removed: the struct, the function and the two constants left both files; one `.rs` file defines the claim (gate census `1`). "Places on disk: none new. `target/test-ports/` (registered) is reached from one more crate's manifest directory". Same retired claim as the Network-ports proposal (three claimants, the claim's home).
    basis: ".andromeda/architecture.md:234 · crates/test-support/src/ports.rs:27-37 (`claims_dir`), 45-74"
    dependent-of: D-arch-resources
```

disposition: apply - dependent of proposal 5, applied with it

### architecture proposal 7

```yaml
  - detector: D-arch-decisions
    severity: warning
    section: "§Stack and Technologies → Gate-script pins (test-only)"
    change: >-
      Append to the row: since chunk 2026-10-10-shared-telemetry-test-data the port claim is no longer defined in the shared module `xtask/tests/common/mod.rs` (which keeps `random_word` and `run_passphrase`); it and the `pulse-app` engine tests take `claim_port` / `PortClaim` from the workspace's dev-only member `test-support` (`test_support::ports`), named under `[dev-dependencies]` of `xtask`, `pulse-app` and `buffer` — the row's first manifest and lockfile change, so "Still no Cargo or npm dependency and no manifest or lockfile change" is qualified to "until that chunk": `Cargo.lock` gained one workspace entry (`test-support`, depending on `ingest`), no registry package and no npm dependency were added, `deny.toml` is unchanged. The advisory-lock need under `target/test-ports/` now also holds for the four in-process arms of `pulse-app/tests/integration_engine_boot.rs`, two xtask unit tests (in `harness_ready.rs`, `engine_cycle.rs`) and `test-support`'s own port tests. One more test-only xtask pin module, `test_support_edges` (five tests, declared `#[cfg(test)]` in `xtask/src/main.rs`), reads the workspace manifests as text, as `license_check` and `rust_floor` do, and needs no host tool; it runs in the workspace test job.
    sidecar: >-
      chunk 2026-10-10-shared-telemetry-test-data — §Stack → Gate-script pins (test-only): the port claim left `xtask/tests/common/mod.rs` for the dev-only member `test-support` (first manifest/lockfile change of the row, no registry package); new xtask pin module `test_support_edges`.
    rationale: >-
      Report "Dependencies": "no registry package added or bumped. `Cargo.lock` gained one workspace entry … `buffer`, `pulse-app` and `xtask` each name `test-support` under `[dev-dependencies]`" — against the row's present-tense "Still no Cargo or npm dependency and no manifest or lockfile change". "Harness / gate surface": "One more xtask pin module, `test_support_edges` … The port claim left `xtask/tests/common/mod.rs`; that file now takes it from `test_support::ports`." "Counts / qualifiers moved" names `architecture.md:34` as a site of `xtask/tests/common/mod.rs`. No new library or runtime: the stack itself is not contradicted, the row's dependency sentence and its host-need set are.
    basis: ".andromeda/architecture.md:34 · xtask/Cargo.toml:28-30 · xtask/tests/common/mod.rs:16-17 · xtask/src/main.rs:35-36 · Cargo.lock:7306-7312"
```

disposition: apply - check 1: Accurate this-chunk addition - check 5: the plan's third entry

### architecture proposal 8

```yaml
  - detector: D-arch-decisions
    severity: escalate
    section: "§Cross-cutting Patterns → Module dependency direction"
    change: >-
      Keep the three rules and add one carve-out in the operator's words (inputs#I3, inputs#I8 item 4; the report quotes them with an elision — the full sentence is taken from the input, not from this proposal): "a dev-only edge from a member to `test-support` is allowed, and only that; … pinned by a committed test". As built at chunk 2026-10-10-shared-telemetry-test-data, the first dev-only edges between workspace members: `buffer`, `pulse-app` and `xtask` name `test-support` under `[dev-dependencies]` and no member names it in any other table; `test-support` names `ingest` under `[dependencies]` and has no other table; `ingest` does not name `test-support`. The pin is the test-only xtask module `test_support_edges` (`no_member_names_test_support_outside_its_dev_dependencies`, `test_support_depends_on_ingest_alone`, `test_support_is_a_member_that_some_member_uses`). Qualify the closing sentence: the graph the product is built from stays a DAG with `pulse-app` as the only root — no product target links `test-support`.
    sidecar: >-
      chunk 2026-10-10-shared-telemetry-test-data — §Cross-cutting Patterns → Module dependency direction: carve-out added in the operator's words — a dev-only edge from a member to `test-support`, and only that, pinned by `xtask/src/test_support_edges.rs`.
    rationale: >-
      Report "Dependencies": "New edges between workspace members, the first dev-only ones in the workspace"; "Expected amendments": `architecture.md:289` "says 'no library crate depends on a sibling unless its declared contract requires it' and states no carve-out". The library crate `buffer` now names a sibling its declared contract does not require, and `test-support` is a member outside the pulse-app-rooted graph, so the pattern as written is contradicted by what landed. Why escalate and not warning: the amendment does not record a fact in a registry, it widens a locked boundary rule (a permitted edge class that did not exist), and the report carries the operator's instruction for exactly this reading — inputs#I8 item 4: if the sentence is read as a boundary widening, the second window halts in the ordinary form, in full, the operator's answer there provisional, "not to be written unhalted". Recommendation: accept with the operator's words verbatim; the widening is narrow (one target crate, dev-only), the member was named and accepted by the operator in the plan (Outcome, arch criterion "met"), and it is mechanically held — the five pin tests pass and both planned mutations read red (`evidence/mutations.md`). The pin is slightly wider than the plan's words (Deviation 7: it also refuses any other `dependencies` table header and any `features` table in the member's manifest), which tightens the carve-out, not loosens it. No other occurrence of the sibling rule was found in architecture or its four key files (the [Module Boundaries] decision, the Stack "Module boundary enforcement" row and the Inherited Defaults "Module boundaries" line state crate-per-module and graph enforcement, not the sibling rule).
    basis: ".andromeda/architecture.md:289 · xtask/src/test_support_edges.rs:98-109, 111-126, 128-146 · crates/buffer/Cargo.toml:53 · pulse-app/Cargo.toml:84 · xtask/Cargo.toml:28-30"
```

disposition: ESCALATE - check 1: a locked rule gains a permitted edge class; the Accurate-this-chunk-addition rule excludes a relaxation of a locked decision, and the Boundary widening rule is judged by subject. inputs#I8 item 4: halt in the ordinary form, the answer there PROVISIONAL

## security-plan

verdict: proposals 1 - stripped: five comment lines above the list, one verdict per detector (D-security-input holds, its one proposal carried from the plan; -auth, -deps, -logging hold; `security-plan.md:218` read as a dated measurement, no amendment)

### security-plan proposal 1

```yaml
  - detector: D-security-input
    severity: warning
    section: "§Security Anti-Patterns → Input"
    change: "In the harness-only carve-out of the product-binary path-variable ban, add a test-only member (chunk `2026-10-10-shared-telemetry-test-data`): the test port claim has ONE home, `test_support::ports::claim_port` / `PortClaim` in `crates/test-support/src/ports.rs` - the workspace member `test-support` (`publish = false`), a library crate named by `buffer`, `pulse-app` and `xtask` under `[dev-dependencies]` only and depending on `ingest` alone, both held by the xtask test module `test_support_edges`, so no product target links it. A claim is a number from 20000 to 31999 with 24317 and 24318 left out, held under an advisory lock (`File::try_lock`) on the empty file `target/test-ports/{number}` for the life of the claim and shown bindable at the claim; the file is never unlinked, and a panic follows when no port can be claimed. The directory is resolved from the crate's compile-time manifest directory, two levels under the root; nothing is read from the environment at run time and no variable is added. It is test-written and never product-written or product-read: no new place on disk, no new port range, no new listening socket, no line of product source changed. State the facts only - write no class reading (routine harness evidence / boundary widening) unless the operator gives one."
    sidecar: "chunk 2026-10-10-shared-telemetry-test-data - §Security Anti-Patterns → Input: named the test-only port claim as a harness-only member (one home `test_support::ports` in the dev-only member `test-support`; empty lock files under `target/test-ports/`; directory from the compile-time manifest dir, no run-time environment read); new text, no existing line corrected."
    rationale: "Report Changes → Expected amendments (from plan) lists `security-plan §Security Anti-Patterns → Input, the port claim's one home and its files under target/test-ports/` as CARRIED, with the site search `test-ports`: security-plan 0+0 and `claim_port`: security-plan 0+0 - the section does not name the claim today, so the fact is new text, not a correction. The facts are the report's: Symbols / APIs → `test_support::ports` (the terms of the claim, the compile-time directory, nothing read from the environment at run time), Removed (the two twin definitions gone; exactly one `.rs` file defines the claim, gate entry last line `1`), Ports / sockets (no new listening socket, no new port range), Env vars (none added, none read by the new code), Places on disk (none new; `target/test-ports/` reached from one more crate's manifest directory), Crates / modules and Dependencies (dev-dependency only, the edge pin). Severity is warning, not the detector's escalate: the detector's escalate condition (an unvalidated external-input boundary) is NOT met - the report's Coverage of new surfaces reads `no new external surface` and `validation n/a` for `claim_port`; this entry reaches security-plan through the plan's list, which no security detector's invariant covers. Every other member of that carve-out cites a named person's class reading; the report carries none for this member, so the change text asserts none. No dependent proposals: the change retires no existing claim, and the two sites that restate the carve-out's membership (§Threat Model Summary → Attack surface → CLI input; §Input Validation → the CLI / env var row) enumerate members that read a variable or a command line, which this one does not."
    basis: "crates/test-support/src/ports.rs:45-74 (claim_port); crates/test-support/src/ports.rs:27-37 (claims_dir); xtask/src/test_support_edges.rs:98-109 and 111-126 (the two edge pins)"
```

disposition: apply - check 1: Accurate this-chunk addition (new text for a fact the report's Symbols / APIs carries; escalate condition absent: no external input surface) - check 5: the plan's sixth entry. Written as facts, with no class reading

## design-system

verdict: proposals 0 - stripped: two comment lines after the empty list (D-design-tokens: no UI element, every `tokens` flag n/a; D-design-status-narrative: no status claim touched)

## layout-templates

verdict: proposals 0 - stripped: three comment lines after the empty list (no surface or region added; no status claim touched; the moved counts have no site in the doc)

## test-plan

verdict: proposals 18 - stripped: eleven comment lines above the list (D-tests-obs-harness no drift; evaluated and left: `test-plan.md:496` "3112 tests" a dated reading, `test-plan.md:182` names the shared module only, the key file's "three port-using binaries" a dated measurement)

### test-plan proposal 1

```yaml
  - detector: D-tests-framework
    severity: warning
    section: "§4 Unit Test Strategy — Fixture pattern at unit level"
    change: >-
      Re-base the sample and its lead sentence on the factories as built: telemetry test data comes from the dev-only member `crates/test-support`,
      module `test_support::otlp` — free functions that open a builder (`span_event()`, `span()`, `metric_point()`, `gauge()`, `log_record()`, `resource()`),
      setters `at(unix_nano)`, `ResourceBuilder::service`, `attribute(key, value)`, built on `ResourceBuilder` as `resource_spans()` / `resource_metrics()` /
      `resource_logs()` / `trace_request()` / `logs_request()`; e.g. `resource().service("default-service").span(span().trace_id([1u8; 16]).span_id([2u8; 8])).resource_spans()`.
      Defaults are fixed (`DEFAULT_TIME_UNIX_NANO`, `DEFAULT_SERVICE`), no clock, nothing random; there is no `MockTraceSpan`, no `duration_ms` setter, and the member
      has no `rstest` dependency (`rstest` 0.26.1 stays the fixture library of the crates that already use it).
    sidecar: >-
      2026-10-10-shared-telemetry-test-data — §4 Fixture pattern: the `MockTraceSpan::builder()` + `#[fixture]` sample replaced by the `test_support::otlp`
      factories as built (one dev-only crate, no rstest in it).
    rationale: >-
      Report Changes, "Symbols / APIs" and "Expected amendments": none of the three specified names exists; the set is `test_support::otlp`'s free functions and
      builders in one crate, with no Arrow-batch factory and no `rstest` dependency in the member. §4 still shows `MockTraceSpan::builder()….duration_ms(100).build()`
      as the unit fixture pattern, and "Not built" lists the span-duration setter.
    basis: "crates/test-support/src/otlp.rs:98-107"
```

disposition: apply - check 1: Accurate this-chunk addition - the sample re-derived from the report's Symbols / APIs

### test-plan proposal 2

```yaml
  - detector: D-tests-framework
    severity: escalate
    section: "§3 → Bootstrap phases"
    change: >-
      Item 6 (test-data-bootstrap-wire) reads as built, not as owed-and-unimplemented: since chunk 2026-10-10-shared-telemetry-test-data the factories are ONE set in the
      dev-only workspace member `crates/test-support` (`test_support::otlp`: spans with their events, metric points as integer points of a gauge, log records, one resource
      per built form), taken as a dev-dependency by `buffer`, `pulse-app` and `xtask` and depending on `ingest` alone — not `MockTraceSpan` in `ingest`, `MockArrowBatch` in
      `buffer`, `MockMetricPoint` in `viz`. Dropped from the item: the three `Mock*` names, the one-factory-per-crate placement, and the Arrow-batch factory (not built, no
      caller); also not built for lack of a caller: a metrics export request, a span-duration setter, a no-resource form; the version setter waits for the stored column
      that reads it (route entry `Span keeps its identity`). The `rstest` half stays as landed elsewhere; the member itself does not depend on `rstest`.
    sidecar: >-
      2026-10-10-shared-telemetry-test-data — §3 Bootstrap phases item 6: UNIMPLEMENTED mandate replaced by the factories as built (`test_support::otlp`, one dev-only
      crate); the per-crate `Mock*` trio and the Arrow-batch factory dropped from the item.
    rationale: >-
      Report "Expected amendments" names this site and "what of item 6 is dropped"; "Symbols / APIs → Not built" and Deviation 3 list what was left out. ESCALATED because
      the amendment retires a mandated factory (`MockArrowBatch`) rather than meeting it: the report's ground for not building is inputs#I1 item 1 ("a factory nothing calls
      is not built here"), which speaks of this chunk, and the trigger's own resolution clause makes dropping a matter of what "the project does not actually want" — the
      operator's word. Recommendation: record it as dropped, as the plan's expected amendment says; the alternative is to keep an Arrow-batch factory as owed in the item.
    basis: "crates/test-support/src/otlp.rs:333-446"
    dependent-of: D-tests-framework
```

disposition: ESCALATE - check 1: no rule matches and no recorded direction names the drop. The plan's entry leaves "what of item 6 is dropped" to the wrap; inputs#I1 item 1 ("a factory nothing calls is not built here") speaks of this chunk; the one home and the names are settled by inputs#I3, the Arrow-batch factory is not

### test-plan proposal 3

```yaml
  - detector: D-tests-framework
    severity: escalate
    section: "§1 Pending coverage triggers — `test-data-bootstrap-factories-unimplemented`"
    change: >-
      Append the landing in the row's own form: LANDED 2026-10-10-shared-telemetry-test-data by the row's second resolution arm — §3 item 6 amended to the set the project
      built. One set of factories (`test_support::otlp` in `crates/test-support`) builds spans with events, metric points and log records; six test files take their
      telemetry from it (`consumer.rs`, `appender.rs`, `integration_engine_boot.rs`, `integration_console_engine.rs`, `e2e_drain_template_assignment.rs`,
      `e2e_pii_span_masking_stored_field.rs`); none of the three `Mock*` names exists and no Arrow-batch factory was built. Residue, each with a route owner: hand-built
      literals read span 19, span event 7, number point 7, log record 12 after the chunk (25 / 11 / 10 / 14 at its base, `evidence/census.md`), and three
      `crates/ingest/src` files stay hand-built because `ingest`'s own unit modules cannot take a crate that depends on `ingest` (from the plan's notes, not measured by a compile).
    sidecar: >-
      2026-10-10-shared-telemetry-test-data — §1 trigger `test-data-bootstrap-factories-unimplemented`: landed in changed form (one `test-support` set, `Mock*` trio
      dropped); residue of hand-built literals stated with its census counts.
    rationale: >-
      Report "Expected amendments" names this trigger; "Counts / qualifiers moved" gives the census (25 → 19, 11 → 7, 10 → 7, 14 → 12) and "Decisions & corrections"
      the `ingest` limit. The row still says "none of the three exists … Resolution: implement the three factories, or amend §3 item 6". ESCALATED with the item-6
      proposal: closing the trigger rests on the same drop decision.
    dependent-of: D-tests-framework
```

disposition: ESCALATE - with proposal 2: closing the pending row rests on the same answer

### test-plan proposal 4

```yaml
  - detector: D-tests-framework
    severity: warning
    section: "§3 → Test data bootstrap"
    change: >-
      Mechanism reads: the `test_support::otlp` factories (dev-only member `crates/test-support`) — e.g. `resource().service("test-app").span(span().trace_id([0u8; 16]).span_id([0u8; 8]))`
      built as `trace_request()` / `logs_request()` (the export requests a test sends through the `tonic` client) or as `resource_spans()` / `resource_metrics()` /
      `resource_logs()` (the resource-level batches a test hands the buffer); unset event time and service are fixed constants, an unset span id is the span's position
      under its resource plus one; no `duration_ms` setter (a span ends `SPAN_DURATION_NANOS` after its start) and no metrics export request. The two `MockTraceSpan::builder()`
      lines go; `rstest` composition is no part of the member.
    sidecar: >-
      2026-10-10-shared-telemetry-test-data — §3 Test data bootstrap, Mechanism: `MockTraceSpan::builder()` examples replaced by the `test_support::otlp` forms as built.
    rationale: >-
      Report "Expected amendments" names "the keyed contract Test data bootstrap"; the key file's Mechanism label still shows `MockTraceSpan::builder()….duration_ms(50).build()`
      and a `#[fixture]` over it, neither of which exists ("Symbols / APIs", "Not built").
    basis: "crates/test-support/src/otlp.rs:433-438"
    dependent-of: D-tests-framework
```

disposition: apply - check 1: Accurate this-chunk addition - check 5: the plan's fourth entry (the keyed contract)

### test-plan proposal 5

```yaml
  - detector: D-tests-framework
    severity: warning
    section: "§5 Integration Test Strategy — Driver(s), Module ↔ DuckDB"
    change: >-
      The parenthetical reads: seeded by OTLP ingest through the live receiver, by direct in-memory inserts, or — in the crates that take `test-support` (`buffer`,
      `pulse-app`, `xtask`) — by resource-level batches from `test_support::otlp` (`resource_spans()` / `resource_metrics()` / `resource_logs()`); no Arrow-batch factory
      exists: the `MockArrowBatch` this row once named was dropped from §3 item 6 at chunk 2026-10-10-shared-telemetry-test-data.
    sidecar: >-
      2026-10-10-shared-telemetry-test-data — §5 Module ↔ DuckDB: "`MockArrowBatch` specified-but-unimplemented" replaced by the factories as built; no Arrow-batch factory.
    rationale: >-
      Duplicate occurrence of the retired "specified-but-unimplemented, see §3 item 6" claim; report "Expected amendments" lists "§5 (two rows)" and states no Arrow-batch
      factory was built; "Dependencies" names the three members that take `test-support`.
    dependent-of: D-tests-framework
```

disposition: held on proposal 2's answer - its second clause says the Arrow-batch factory was dropped

### test-plan proposal 6

```yaml
  - detector: D-tests-framework
    severity: warning
    section: "§5 Integration Test Strategy — Boundary types covered, buffer → viz row"
    change: >-
      The row's last sentence reads: no Arrow-batch factory exists (the `MockArrowBatch` of §3 item 6 was dropped at chunk 2026-10-10-shared-telemetry-test-data), and `viz`
      does not take `test-support` (a dev-dependency of `buffer`, `pulse-app` and `xtask` only) — seed the in-memory DuckDB directly or via OTLP ingest.
    sidecar: >-
      2026-10-10-shared-telemetry-test-data — §5 buffer → viz row: "`MockArrowBatch` is unimplemented — see §3 item 6" replaced by "no Arrow-batch factory; `viz` does not take `test-support`".
    rationale: >-
      Second of the "§5 (two rows)" the report's "Expected amendments" lists; "Dependencies": `buffer`, `pulse-app` and `xtask` each name `test-support` under
      `[dev-dependencies]`, and no other member does (pinned by `no_member_names_test_support_outside_its_dev_dependencies`).
    basis: "xtask/src/test_support_edges.rs:98-109"
    dependent-of: D-tests-framework
```

disposition: held on proposal 2's answer - the same clause

### test-plan proposal 7

```yaml
  - detector: D-tests-framework
    severity: warning
    section: "§6 E2E Test Strategy — Scenario P1, step 2"
    change: >-
      Step 2 reads: build 100 synthetic spans with the `test_support::otlp` factories (`resource().service("test-app").spans(…)`) and take the export request from
      `trace_request()` — no `MockTraceSpan::builder()`.
    sidecar: >-
      2026-10-10-shared-telemetry-test-data — §6 P1 step 2: `MockTraceSpan::builder()` replaced by `test_support::otlp` → `trace_request()`.
    rationale: >-
      Duplicate occurrence: the step names a builder that does not exist; the report gives `trace_request()` as the export request a test sends ("Built forms").
    basis: "crates/test-support/src/otlp.rs:433-438"
    dependent-of: D-tests-framework
```

disposition: apply - re-derived: the step names the builder form; it does not say the P1 test file uses it (that file is among the unmoved, by the census)

### test-plan proposal 8

```yaml
  - detector: D-tests-framework
    severity: warning
    section: "§7 Test Data & Fixtures — Seed strategies, Persistent data — DuckDB rows"
    change: >-
      Strategy cell reads: OTLP ingest via `tonic` gRPC client + the `test_support::otlp` factories (or their resource-level batches handed to the buffer).
    sidecar: >-
      2026-10-10-shared-telemetry-test-data — §7 Seed strategies, DuckDB rows: "`MockTraceSpan` builders" → "`test_support::otlp` factories".
    rationale: >-
      Duplicate occurrence of the `MockTraceSpan` claim in a table cell; report "Expected amendments" lists §7.
    dependent-of: D-tests-framework
```

disposition: apply - check 1: Accurate this-chunk addition - check 5: the plan's fourth entry (section 7)

### test-plan proposal 9

```yaml
  - detector: D-tests-framework
    severity: warning
    section: "§7 Test Data & Fixtures — Self-bootstrapping requirement, Builder factories bullet"
    change: >-
      The bullet reads: builder factories — one set, `test_support::otlp` in the dev-only member `crates/test-support` (`span_event()`, `span()`, `metric_point()`,
      `gauge()`, `log_record()`, `resource()`), implemented at chunk 2026-10-10-shared-telemetry-test-data; event time, service and string attributes are settable, every
      default is fixed and crosses the write boundary unmasked (`rows_ingested` 7, `redactions_applied` 0, `append_rejections` 0 on a default batch of every kind). The
      per-crate `MockTraceSpan` / `MockArrowBatch` / `MockMetricPoint` trio is not what was built and no Arrow-batch factory exists (§3 item 6). Tests the factories do
      not yet serve keep self-bootstrapping via OTLP ingest and per-test in-memory DuckDB seeding.
    sidecar: >-
      2026-10-10-shared-telemetry-test-data — §7 Self-bootstrapping: "all three SPECIFIED BUT NOT IMPLEMENTED" replaced by the one `test_support::otlp` set as built.
    rationale: >-
      Duplicate occurrence; the bullet still says all three are unimplemented "until they land". Report "Schema / config" gives the default-batch reading and
      "Symbols / APIs" the names and home.
    basis: "crates/buffer/src/consumer.rs:625-666"
    dependent-of: D-tests-framework
```

disposition: held on proposal 2's answer - its sentence on the trio and the Arrow-batch factory

### test-plan proposal 10

```yaml
  - detector: D-tests-framework
    severity: warning
    section: "§11 Test Anti-Patterns — Test Data"
    change: >-
      First bullet reads: NEVER use production telemetry dumps in tests — generate synthetic telemetry via the `test_support::otlp` factories (or OTLP ingest where a
      crate cannot take them).
    sidecar: >-
      2026-10-10-shared-telemetry-test-data — §11 Test Data ban: `MockTraceSpan::builder()` → the `test_support::otlp` factories.
    rationale: >-
      Duplicate occurrence in a ban: it prescribes a builder that does not exist (report "Expected amendments": `MockTraceSpan` test-plan 6+3, all six body sites covered
      by this set of proposals).
    dependent-of: D-tests-framework
```

disposition: apply - a duplicate occurrence of the retired builder name, in a ban

### test-plan proposal 11

```yaml
  - detector: D-tests-framework
    severity: warning
    section: "§4 Unit Test Strategy — Framework (Rust crates)"
    change: >-
      "inherited by all 16 members" reads "inherited by all 17 members" (the seventeenth, the dev-only `test-support`, since chunk 2026-10-10-shared-telemetry-test-data).
    sidecar: >-
      2026-10-10-shared-telemetry-test-data — §4 Framework: workspace member count 16 → 17 (`crates/test-support`, inheriting the workspace `rust-version`).
    rationale: >-
      Report "Counts / qualifiers moved": workspace members 16 → 17, site `test-plan.md:194`, stated as a current count; "Crates / modules": the new member inherits the
      workspace `version`, `edition`, `rust-version`, `license`, `repository`. The report's grep finds this one site in test-plan (1+0).
    basis: "Cargo.toml:18"
```

disposition: apply - check 1: Accurate this-chunk addition - check 5: the plan's fifth entry (the member count)

### test-plan proposal 12

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy — Test pyramid, Unit row"
    change: >-
      The per-crate list gains `test-support`: (per-crate tests: ingest, buffer, viz, ui-bridge, snapshot, workspace-detector, plugins, mcp-server, corpus, triage,
      security, interpretation, and the dev-only test-support).
    sidecar: >-
      2026-10-10-shared-telemetry-test-data — §2 Unit row: `test-support` added to the per-crate enumeration (14 co-located unit tests).
    rationale: >-
      Report "Crates / modules" adds the library member `crates/test-support`, and "Coverage of new surfaces" gives its unit tests (10 in `otlp`, 4 in `ports`). The
      invariant holds — the new paths are tested at the unit tier — but the row's enumeration of crates carrying that tier omits the new one ("Expected amendments":
      the new crate at the three library-crate enumerations).
    basis: "crates/test-support/src/otlp.rs:448-719"
```

disposition: apply - check 5: the plan's fifth entry (the first library-crate enumeration)

### test-plan proposal 13

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy — Conventions, Test file location (Rust)"
    change: >-
      The LIBRARY-crate list gains `test-support` (co-located `mod tests` in `crates/test-support/src/otlp.rs` and `ports.rs`), and the xtask sentence notes one more
      co-located pin module, `xtask/src/test_support_edges.rs` (five tests, `#[cfg(test)]`, declared in `xtask/src/main.rs`).
    sidecar: >-
      2026-10-10-shared-telemetry-test-data — §4 Test file location: `test-support` added to the co-located library-crate list; xtask pin module `test_support_edges` noted.
    rationale: >-
      Second of the three enumerations; report "Symbols / APIs" places the member's tests in co-located modules (448-719, 76-120) and the xtask pin at `xtask/src/main.rs:35-36`.
    basis: "crates/test-support/src/ports.rs:76-120"
    dependent-of: D-tests-coverage
```

disposition: apply - check 5: the second enumeration; the library list only, the xtask pin is named in proposal 14's bullet

### test-plan proposal 14

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: "§4 Unit Test Strategy — What unit tests cover"
    change: >-
      A new bullet: **test-support crate (dev-only):** the `otlp` factories — two builds from the same calls are equal, an unset event time is the one fixed instant and
      an unset service the one fixed name, ids have their wire lengths and unset span ids differ by position, what a test sets on a span and its event, a metric point
      and a log record is read back (10 tests); the `ports` claim — a claimed number lies in 20000–31999 and is no harness port, two live claims differ, the lock file
      of a live claim cannot be locked again, a claimed port binds on loopback (4 tests). Its edges are pinned from xtask (`test_support_edges`, 5 tests: no member names
      it outside `[dev-dependencies]`, it depends on `ingest` alone, it is a member some member uses), and the write-boundary reading of its defaults is `buffer`'s
      `consumer::tests::a_default_built_batch_of_every_kind_lands_whole_with_no_redaction`.
    sidecar: >-
      2026-10-10-shared-telemetry-test-data — §4 What unit tests cover: `test-support` bullet added (otlp 10, ports 4; xtask edge pin 5; buffer default-built pin).
    rationale: >-
      Third enumeration (the per-crate bullets); report "Symbols / APIs" and "Coverage of new surfaces" name every test listed; the 20 tests are the whole of the
      3112 → 3132 move ("Counts / qualifiers moved").
    basis: "xtask/src/test_support_edges.rs:98-146"
    dependent-of: D-tests-coverage
```

disposition: apply - check 5: the third enumeration

### test-plan proposal 15

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: "§9 CI Integration — Pipeline structure, Integration tests row"
    change: >-
      The closing sentences read: the four port-using binaries `integration_engine_boot`, `integration_console_engine`, `agent_run_shell` and `engine_cycle_refusals`
      claim each port under an advisory file lock in `target/test-ports/` through ONE definition, `test_support::ports::claim_port` (dev-only member `crates/test-support`),
      since chunk 2026-10-10-shared-telemetry-test-data; `pulse-app/tests/integration_engine_boot.rs` hands the engine two claimed ports in each of its four in-process arms
      and no longer picks by bind-and-release (the carry the route entry `Shared telemetry test data` owned is closed; no test in that file, `harness_ready.rs` or
      `engine_cycle.rs` binds port 0 and releases it). Two xtask unit tests and the member's own four tests claim too. As measured at that chunk, seven of seven hand
      runs of the four binaries were green (a count, not an exclusion). The dated reading of the three binaries (one of seven failed, then 14 of 14) stays as written.
    sidecar: >-
      2026-10-10-shared-telemetry-test-data — §9 Integration tests row: three claiming binaries → four through one `claim_port`; "`integration_engine_boot.rs` still
      picks by bind-and-release" retired, owner sentence closed.
    rationale: >-
      Report "Counts / qualifiers moved" names `test-plan.md:504` (three → four port-using binaries); "Symbols / APIs → Ports / sockets" and "Removed" give the one
      definition, the new claimants and the port-0 census (0); Outcome: "The carry is closed — met", "seven of seven — met, as a count". The row's last clause is now false.
    basis: "crates/test-support/src/ports.rs:45-74"
```

disposition: apply - check 1: Accurate this-chunk addition - check 5: the plan's fifth entry (section 9, the owner sentence closed)

### test-plan proposal 16

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Per-chunk gate discipline"
    change: >-
      Spawned-program witness form, after the claim's terms: since chunk 2026-10-10-shared-telemetry-test-data the claim is one definition — `test_support::ports::claim_port`
      / `PortClaim` in the dev-only member `crates/test-support`, on the same terms (the two twins in `integration_console_engine.rs` and `xtask/tests/common/mod.rs` are
      removed; the lock directory is found from the crate's compile-time manifest directory, nothing read from the environment at run time; a panic when no port can be
      claimed) — and it is taken by four port-using binaries (`integration_engine_boot`'s four in-process arms joined, two claimed ports each) and by two xtask unit
      tests; seven of seven hand runs of the four binaries green at that chunk (a count). "The three port-using binaries" stays as the dated measurement it is.
    sidecar: >-
      2026-10-10-shared-telemetry-test-data — §3 Per-chunk gate discipline, Spawned-program witness form: the port claim's one home (`test_support::ports`) and its
      fourth claiming binary added; dated three-binary reading kept.
    rationale: >-
      Report "Counts / qualifiers moved" names `per-chunk-gate-discipline.md:52`, and "Expected amendments" names this form ("four claiming binaries, the owner sentence
      closed"). Nothing in the key is false — the three-binary count is dated — so this is the addition that keeps the form's description of the claim current; the
      terms are quoted from "Symbols / APIs → `test_support::ports`".
    basis: "crates/test-support/src/ports.rs:17-21"
    dependent-of: D-tests-coverage
```

disposition: apply - check 5: the plan's fifth entry (the witness form); an addition, the dated three-binary reading kept

### test-plan proposal 17

```yaml
  - detector: D-tests-coverage
    severity: warning
    section: "§3 → Bootstrap phases"
    change: >-
      Item 1's closing clause reads: so the port-using suites — the spawned ones and, since chunk 2026-10-10-shared-telemetry-test-data, the in-process arms of
      `integration_engine_boot.rs` — claim their ports through `test_support::ports::claim_port`, §9 Integration tests row.
    sidecar: >-
      2026-10-10-shared-telemetry-test-data — §3 Bootstrap phases item 1: claiming is no longer the spawned suites' alone (in-process engine-boot arms claim too).
    rationale: >-
      Restates the §9 row's actor ("the port-using spawned suites claim their ports") without its tokens; report "Ports / sockets": the four in-process arms of
      `integration_engine_boot.rs` are new claimants. Narrower than the fact rather than false; lowest-priority of the set.
    dependent-of: D-tests-coverage
```

disposition: apply - a restatement of the section 9 row's actor, brought in line with it

### test-plan proposal 18

```yaml
  - detector: D-tests-coverage
    severity: escalate
    section: "§10 Quality Gates & Coverage Targets — Cumulative across workspace"
    change: >-
      A bullet states the measure as it stands: `crates/test-support` — test factories and the port claim, a dev-only member — is INSIDE the coverage measure since chunk
      2026-10-10-shared-telemetry-test-data, a stated exception to "Exclude … test fixtures, and mock implementations"; the `coverage` job of ci#38088083705 read
      Line 35070/38688 = 90.6% and Function 3695/4129 = 89.5% with it inside (34620/38214 and 3623/4056 on the previous chunk's run); `xtask/` stays outside.
    sidecar: >-
      2026-10-10-shared-telemetry-test-data — §10 Cumulative: `crates/test-support` recorded as inside the coverage measure (90.6% line / 89.5% function), an exception
      to the test-fixtures exclusion.
    rationale: >-
      Report "Harness / gate surface": "`crates/test-support` is inside the coverage measure … `xtask` stays outside"; "Found and not owned" item 5 leans "leave; the plan
      states it, and both thresholds read above their bars with it inside". §10's first Cumulative bullet says test fixtures are excluded from coverage, and the new
      crate is test fixtures by purpose. ESCALATED because the doc edit ratifies a departure from a stated exclusion rule; the other remedy (excluding the crate through
      `COVERAGE_IGNORE_FILENAME_REGEX`) is a code change outside a doc pass. Recommendation: ratify as measured, per the report's lean. No test-plan line number is
      given by the report for this bullet.
```

disposition: ESCALATE - the amendment would ratify a departure from a stated exclusion rule (test fixtures are excluded from coverage); the report's found-and-not-owned item 5; inputs#I8 item 8 sends it to the operator

## Escalations — resolved with the operator at the Phase 2 halt (inputs#I9)

The halt was in the ordinary form, in full: each proposal, why it escalated, the recommendation (inputs#I8 item 4).
The answers are kept verbatim in `relay-2.md` of this run dir, snapped as inputs#I9.

1. architecture proposal 8, the dependency carve-out → **applied, PROVISIONAL.** word: "Write it (Recommended)",
   with the note "PROVISIONAL, my reading as operator, the founder own word owed." — the operator (the pc
   overseer), 2026-10-11, given here. The body and the sidecar entry say PROVISIONAL; the founder's own word
   supersedes it. No playbook rule proposed: the class keeps escalating.
2. test-plan proposals 2 and 3, item 6 and its pending row → **applied.** word: "Drop them (Recommended)", with the
   note "a mandated factory nothing calls is retired from the item, not kept as owed; an entry that needs one
   builds it with its consumer." — the operator (the pc overseer), 2026-10-11, given here. Proposals 5, 6 and 9,
   held on this answer, applied in the dropped form.
3. test-plan proposal 18, the coverage measure → **applied in part; the rest after the route-resolve card.** word:
   "Record and carry", with the note "The rule says fixtures are outside the measure, so the measure is what moves:
   record the figures as measured now, and name at the route-resolve card the entry that takes the crate out of
   the measure." — the operator (the pc overseer), 2026-10-11, given here. The §10 bullet names its owner, so it
   is written once the card names the entry; the addendum of `cascade-dispositions.md` records it.

Totals: 27 proposals — applied 26 (19 at Validate; 3 held ones on answer 2; the 4 escalated ones on answers 1 to
3, test-plan proposal 18 of them after the card) · rejected 1 (architecture proposal 4) · rejected for a source the
report does not carry 0 · escalations 3, covering 4 proposals, all answered. The halt message said "20 apply";
the count is 19, the fourth escalated proposal (test-plan 3) having been counted there as an apply.

A slip of the report found at Validate and corrected in it: its Ports / sockets bullet said claims are made "from
five test binaries" while listing six; the tree reads six files calling `claim_port`, serving six test binaries.
The word now reads "six".

## obs-plan

verdict: proposals 0 - stripped: the comment block above the empty list (all four detectors hold; seen and left: section 9 Pipeline integration, the `xtask test` row, "three test binaries of the run start the console program" - read at the cascade sweep, see below)

## a11y-plan

verdict: proposals 0 - stripped: the comment block after the empty list (no interactive element; no schema change; a sweep of the moved counts and names read 0 hits in the doc and its ten key files)
