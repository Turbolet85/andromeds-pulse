# Report — 2026-10-10-shared-telemetry-test-data

**Chunk:** spans, metric points and log records from one set of test factories; event time, service and attributes
settable; engine-boot tests on claimed ports
**Date:** 2026-10-10T22:06:22Z (the clock at authoring; wrap run `.andromeda/runs/2026-10-10T22-04-24Z-wrap/`)
**Commits:** `d9651126 chore(2026-10-10-shared-telemetry-test-data): operator pre-CI commit` — the one commit of
`50216d7e..HEAD` (`git log --format='%h %s' 50216d7e..HEAD`). The chunk base is `50216d7e`, its parent.
**Window:** this report is the first window of a two-window wrap (inputs#I8 item 2, inputs#I7). Everything
curation needs from the implement and operator-pass conversation is under "Decisions & corrections".
**The wrap directive moved during P1.** It was snapped as inputs#I6 and its source was corrected by the operator
seconds later; the corrected text is inputs#I8 and is the one this report rests on. What moved is item 4 alone
(see Cross-project / external claims). Where this report cites the directive it cites inputs#I8.

## Changes (structured — detectors read this)

- **Files:** 20 source and manifest files, `git diff --stat 50216d7e -- . ':!.andromeda' ':!andromeda-pulse-0.4.0'
  ':!.claude'`: 1226 insertions, 496 deletions.
  - New (5): `crates/test-support/Cargo.toml` (14 lines) · `crates/test-support/src/lib.rs` (8) ·
    `crates/test-support/src/otlp.rs` (719) · `crates/test-support/src/ports.rs` (120) ·
    `xtask/src/test_support_edges.rs` (178).
  - Modified (15): `Cargo.toml` · `Cargo.lock` · `crates/buffer/Cargo.toml` · `pulse-app/Cargo.toml` ·
    `xtask/Cargo.toml` · `xtask/src/main.rs` · `crates/buffer/src/consumer.rs` · `crates/buffer/src/appender.rs` ·
    `xtask/src/harness_ready.rs` · `xtask/src/engine_cycle.rs` · `xtask/tests/common/mod.rs` ·
    `pulse-app/tests/integration_engine_boot.rs` · `pulse-app/tests/integration_console_engine.rs` ·
    `pulse-app/tests/e2e_drain_template_assignment.rs` · `pulse-app/tests/e2e_pii_span_masking_stored_field.rs`.
  - In the four files that hold their tests in a co-located module (`consumer.rs`, `appender.rs`,
    `harness_ready.rs`, `engine_cycle.rs`) every changed hunk starts inside that module (gate entry
    `git diff -U0 --src-prefix=a/ --dst-prefix=b/ 50216d7e… | awk …`, green: exit 0, no output). No line of
    product source changed. `pulse-app/ui/src/bindings/index.ts` equals the chunk base.
  - Not source: `plan.md` (two lines, the operator edit below) and six files under `evidence/`.
- **Symbols / APIs:** all new symbols are test support, in a crate no product target links. Coordinates are
  copied from "New text, by line".
  - `test_support::otlp` (`crates/test-support/src/otlp.rs`): constants `DEFAULT_TIME_UNIX_NANO`
    (`1_700_000_000_000_000_000`), `DEFAULT_SERVICE` (`"test-service"`), `SPAN_DURATION_NANOS` (`1_000_000`);
    six free functions that open a builder — `span_event()` 52-58, `span()` 98-107, `metric_point()` 174-180,
    `gauge()` 218-223, `log_record()` 268-276, `resource()` 324-331; six builder types `SpanEventBuilder`
    44-50, `SpanBuilder` 87-96, `MetricPointBuilder` 166-172, `GaugeBuilder` 211-216, `LogRecordBuilder` 258-266,
    `ResourceBuilder` 315-322.
    - Setters. Event time: `at(unix_nano: u64)` on the span event, the span, the metric point and the log record.
      Service: `ResourceBuilder::service`. Attributes (string values): `attribute(key, value)` on the same four.
      Also `SpanBuilder::{trace_id([u8; 16]), span_id([u8; 8]), name, event, events}`,
      `SpanEventBuilder::name`, `MetricPointBuilder::value(i64)`, `GaugeBuilder::{name, point, points}`,
      `LogRecordBuilder::{body, severity(number, text)}`, `ResourceBuilder::{span, spans, gauge, gauges,
      log_record, log_records}`.
    - Built forms, all on `ResourceBuilder` (333-446): `resource_spans()`, `resource_metrics()`,
      `resource_logs()` (the resource-level `Vec<Resource…>` the buffer takes) and `trace_request()`,
      `logs_request()` (the export requests a test sends). One resource per built form.
    - Behaviour the module's own tests hold (448-719, ten tests): no clock is read and nothing is random, so two
      builds from the same calls are equal; an unset event time is the fixed constant, an unset service the fixed
      name; a trace id is 16 bytes, a span id 8; an unset span id is the span's position under its resource plus
      one, big-endian, so unset ids differ; a span ends `SPAN_DURATION_NANOS` after its start; a metric point is
      an integer point of a gauge; a log record defaults to severity 9 / `INFO`, observed time 0.
    - Not built: a version setter (inputs#I3); a metrics export request, a span-duration setter and a
      no-resource form (no caller among the first consumers).
  - `test_support::ports` (`crates/test-support/src/ports.rs`): `pub struct PortClaim { pub number: u16, … }`
    17-21 and `pub fn claim_port() -> PortClaim` 45-74, with the terms the two twins held: a number from 20000 to
    31999, 24317 and 24318 left out, an advisory lock (`File::try_lock`) on `target/test-ports/{number}` held for
    the life of the claim, the port shown bindable at the claim, the file never unlinked, a panic when no port can
    be claimed. The directory is found from the crate's compile-time manifest directory, two levels under the
    root (`claims_dir` 27-37); nothing is read from the environment at run time. Four tests (76-120).
  - `xtask/src/test_support_edges.rs`: a `#[cfg(test)]` module declared at `xtask/src/main.rs:35-36`, five tests
    — `no_member_names_test_support_outside_its_dev_dependencies` 98-109,
    `test_support_depends_on_ingest_alone` 111-126, `test_support_is_a_member_that_some_member_uses` 128-146, and
    two that read the module's own parsing (148-167, 169-178). It reads manifests as text, as `license_check`
    and `rust_floor` do.
  - `crates/buffer/src/consumer.rs`: new test
    `consumer::tests::a_default_built_batch_of_every_kind_lands_whole_with_no_redaction` (625-666).
  - Removed: `fn pick_port` in `pulse-app/tests/integration_engine_boot.rs`; the `PortClaim` struct, the
    `claim_port` function and the constants `CLAIMED_PORTS` / `HARNESS_PORTS` in both
    `pulse-app/tests/integration_console_engine.rs` and `xtask/tests/common/mod.rs`. After the chunk exactly one
    `.rs` file under `crates pulse-app xtask` defines `claim_port`, `PortClaim` or `pick_port` (gate entry
    `grep -rlE … | wc -l`, last line `1`). `xtask/tests/common/mod.rs` keeps `random_word`: `run_passphrase`
    there calls it.
  - Ports / sockets: no new listening socket and no new port range. New claimants of the registered test-only
    range: the four in-process arms of `integration_engine_boot.rs` (two claimed ports each, held in `Booted` for
    the arm's life), the two xtask unit tests
    `harness_ready::tests::receiver_probe_tells_a_listening_loopback_port_from_a_closed_one` and
    `engine_cycle::tests::a_listener_on_a_cycle_port_is_read_as_taken_and_a_closed_one_is_not` (one each), and the
    member's own four tests. Claims are now made from six test binaries through one definition:
    `integration_engine_boot`, `integration_console_engine`, `agent_run_shell`, `engine_cycle_refusals` (the four
    port-using binaries the plan names), plus the xtask unit-test binary and `test-support`'s own.
    No test in `integration_engine_boot.rs`, `harness_ready.rs` or `engine_cycle.rs` binds port 0 any more (gate
    entry `grep -nE 'bind\(\("127\.0\.0\.1", 0\)\)|…' … | wc -l`, last line `0` at exit 1). The 23 binds of port
    0 that keep their listener are untouched.
  - Env vars: none added, none read by the new code.
  - Places on disk: none new. `target/test-ports/` (registered) is reached from one more crate's manifest
    directory and holds one more empty file per newly drawn number.
- **Crates / modules:** added the workspace member `crates/test-support` (package `test-support`, `publish =
  false`, inheriting the workspace `version`, `edition`, `rust-version`, `license`, `repository`), with modules
  `otlp` and `ports`. It is a library crate taken as a dev-dependency only. Added the test-only xtask module
  `test_support_edges`. Removed none.
- **Dependencies:** no registry package added or bumped. `Cargo.lock` gained one workspace entry
  (`test-support`, depending on `ingest`) and one `"test-support"` line in each of the `buffer`, `pulse-app` and
  `xtask` entries (`git diff 50216d7e -- Cargo.lock`: 10 insertions, 0 deletions). New edges between workspace
  members, the first dev-only ones in the workspace: `buffer`, `pulse-app` and `xtask` each name `test-support`
  under `[dev-dependencies]`; `test-support` names `ingest` under `[dependencies]` and has no other table.
  `ingest` does not name `test-support`. `cargo deny check bans licenses sources` and `cargo audit` exit 0 with
  `deny.toml` unchanged.
- **Schema / config:** none. No migration, config key, violation schema or scrub shape changed. What the chunk
  shows about an existing shape: every factory default crosses the write boundary unmasked (a batch of two
  default spans with two events, two default points and three default log records reads `rows_ingested` 7,
  `redactions_applied` 0, `append_rejections` 0), and identical default points and identical default log records
  in one batch all persist.
- **Spec-master edits:** none. No master, key file, sidecar or leaf was edited by the chunk.
- **Counts / qualifiers moved:**
  - Workspace members 16 → 17 (basis: the root `Cargo.toml` member list, 17 entries). Sites, by
    `grep -rnE '[Ss]ixteen workspace members|sixteen members|16 members|16 workspace'` over the seven masters,
    `.andromeda/registries/`, `CLAUDE.md`, `.claude/rules`, `.claude/docs`: 7 hits — `architecture.md:4`,
    `architecture.md:14`, `security-plan.md:218`, `test-plan.md:194`, `CLAUDE.md:98`, `.claude/rules/security.md:78`,
    `.claude/docs/stack.md:6`. Read: `security-plan.md:218` and `rules/security.md:78` state a measurement of
    2026-09-30 ("rejected all 16 members"), not a current count; the other five state the count as current.
  - Library crates: the masters say "fourteen library crates" / "14 library crates"
    (`grep -rnE '[Ff]ourteen library crates|14 library crates'`: 5 hits — `architecture.md:4`,
    `.andromeda/registries/contracts/architecture/deployment-model.md:4`, `CLAUDE.md:9`, `CLAUDE.md:12`,
    `CLAUDE.md:98`). The new member is a fifteenth library crate by form and is not one the product is wired from:
    it is dev-only. Whether the count word moves, and to what, is the amendment's wording.
  - Test binaries that claim ports: three → four named port-using binaries, plus two unit tests of the xtask
    binary and the member's own tests. Sites, by `grep -rnE 'three (claiming|port-using)'`: `architecture.md:234`,
    `test-plan.md:504`, `.andromeda/registries/contracts/test-plan/per-chunk-gate-discipline.md:52`;
    `architecture.md:180` names "the three test binaries" by name. `architecture.md:234` also says the files are
    "written by the three port-using test binaries `agent_run_shell`, `engine_cycle_refusals`
    (`xtask/tests/common/mod.rs`, `claim_port` / `PortClaim`) and `integration_console_engine`"; the claim is in
    neither of those two files now. `xtask/tests/common/mod.rs` is also named at `architecture.md:34` (Gate-script
    pins) and `test-plan.md:182` (`grep -n -F 'xtask/tests/common/mod.rs'`: 3 hits in the two bodies).
  - Workspace test count 3112 → 3132 (basis: the `Summary` line of the workspace run on the dev host, in the
    pre-push check, and in the `lint / test` and `coverage` jobs of `ci#38088083705`; the 20 added are the
    member's 14, the edge pin's 5, the `default_built` test). Site: `test-plan.md:496` ("the workspace run counted
    3112 tests") — a dated reading of the last chunk's verdict run, `grep -rn '3112'`: 1 count hit; the two other
    hits of that pattern are commit shas.
  - Hand-built telemetry literals in tests: span 25 → 19, span event 11 → 7, number point 10 → 7, log record
    14 → 12 (basis: `evidence/census.md`, a line scan on the base and on the tree; each kind also stands once in
    the factory). No master states these counts.
- **Dev-tool versions:** none — no host tool was installed, upgraded or read changed. `gitleaks` is not on the dev
  host (`which gitleaks`: not found), unchanged.
- **Harness / gate surface:** no `agent-run` verb, xtask verb, CI step or verdict shape changed. What moved:
  - One more xtask pin module, `test_support_edges`, run by the workspace test job.
  - The port claim left `xtask/tests/common/mod.rs`; that file now takes it from `test_support::ports`.
  - `crates/test-support` is inside the coverage measure (the `coverage` job read `Line: 35070/38688 = 90.6%`,
    `Function: 3695/4129 = 89.5%`; the last chunk's run read 34620/38214 and 3623/4056). `xtask` stays outside it.
  - The plan's gate entry 4 was corrected by the operator between runs (see Deviations): its `git diff` pins
    `--src-prefix=a/ --dst-prefix=b/`.
- **Cross-project / external claims:**
  - CI: `ci#38088083705` (event `pull_request`, attempt 1) on `d9651126f00614c5c10825b292c17cfe4be94277`:
    `verdict: green · checks 7/7`; with `secret-scan#38088083692` green. All six `ci` jobs `completed/success`; in
    the boot job the smoke, the series, `ci-gates` and `Console engine cycle` steps each read `success`. Record:
    `evidence/operator-pass.md`. The verdict was taken on the pre-CI commit; this wrap's own commit adds to it.
  - The dev host's user-level git config sets `diff.mnemonicPrefix = true` (`git config --show-origin --get
    diff.mnemonicPrefix`). Ground truth outside the repository; it is why gate entry 4 read red as first written.
  - **DRIFT — inputs#I6, the wrap directive's source moved after its snapshot.** `inputs.py verify`, re-run
    after this report was written, read `I6 drifted — the source's bytes moved`. The source
    (`../additional/pc-overseer/relays/pulse-wrap-shared-telemetry-test-data-2026-10-10.md`) was rewritten at
    2026-10-10T22:05:01Z, after the snapshot. `diff` of the kept copy against the source: lines 14-16, item 4,
    and nothing else. Before: "STOP ON ONE CARD that cites this word and lists the proposals it covers — one
    confirm, no re-argument; do not write it unhalted because this directive holds the sentence." After: "HALT IN
    THE ORDINARY FORM, in full (each proposal, why it escalated, your recommendation); my answer there is
    PROVISIONAL. Do not write it unhalted because this directive holds the sentence. (Corrected 2026-10-11 after
    the send: the first wording said "one card".)" The corrected text was snapped as inputs#I8. The invocation
    line (inputs#I7, a message, sent before the correction) still says "stops on ONE CARD"; the corrected relay is
    the later word and says of itself that it replaces the first wording. The second window follows inputs#I8:
    the ordinary halt, in full, the answer there provisional.
  - Inputs, from `inputs.py verify` (8 entries; before this report: unchanged 2 · n/a 5 of 7; after it and the
    second snap: I6 drifted, I8 unchanged):
    - I1 · `../additional/pc-overseer/relays/pulse-phase-shared-telemetry-test-data-2026-10-10.md` · copy ·
      unchanged
    - I2 · message, the operator in the /andromeda-phase invocation · copy · n/a
    - I3 · message, the operator answering the two P4 questions · copy · n/a
    - I4 · message, the operator answering the implement question on gate entry 4 · copy · n/a — cited here
      (inputs#I4) and in `evidence/operator-edit-plan.md`; `verify` printed it `UNCITED` before this report
    - I5 · message, the operator after the implement P4 report · copy · n/a — cited here (inputs#I5) and in
      `evidence/operator-edit-plan.md`, `evidence/operator-pass.md`; printed `UNCITED` before this report
    - I6 · `../additional/pc-overseer/relays/pulse-wrap-shared-telemetry-test-data-2026-10-10.md` · copy ·
      **drifted** — snapped at this wrap before the operator's correction; kept as the first wording, cited
      here (inputs#I6); what moved is named in the DRIFT bullet above
    - I7 · message, the operator in the /andromeda-wrap-session invocation · copy · n/a — snapped at this wrap,
      cited here (inputs#I7)
    - I8 · `../additional/pc-overseer/relays/pulse-wrap-shared-telemetry-test-data-2026-10-10.md` · copy ·
      unchanged — the corrected directive, snapped at this wrap after the drift was read, cited here
      (inputs#I8)
- **Reverted / negative API facts:** none shipped and reverted. Three one-line mutations were applied and
  reverted during /implement (`evidence/mutations.md`), each restore confirmed by checksum; none is in the tree.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - No spec-master claim was measured false.
  - A plan entry, not a spec: gate entry 4 as written at P5 could not read its subject on this host. Its awk takes
    the file name from `+++ b/{path}`; a commit-to-work-tree diff prints `+++ w/{path}` here. The first firing
    read `red · exit 0 ✗ (exit 1)` on five test-module hunks of `consumer.rs` (lines 288 to 355, below the awk's
    fall-through line 360), and the same form would read green on a product hunk between line 360 and a module
    line above it. Its baseline was an empty diff and its control a commit-to-commit diff, which prints `b/`.
    Evidence: `evidence/test-module-guard.md`. Disposed by the operator edit (`evidence/operator-edit-plan.md`);
    the entry's `baseline` text still describes the P5 reading.
  - A research figure not reproduced: research.md ("inside 170 wrapper literals") and the plan's Goal restate a
    wrapper-literal count of 170. The re-run reads 180 at the base under a re-derived pattern; five variants read
    180, 180, 169, 161 and 148. The four record-kind counts beside it reproduce exactly (25, 11, 10, 14 in 26
    files, 28 in all). Research's wrapper pattern is not recorded, so this is "not reproduced", not "shown
    false". research.md stays as written; `evidence/census.md` states the re-run's own reading.
- **Expected amendments (from plan):** the plan lists seven entries and one route carry. Sites were located with
  `grep -c` per master body plus that master's key files under `.andromeda/registries/` (body+key).
  - architecture §Occupied Resources → Cargo workspace crate names, a seventeenth member `test-support`, dev-only;
    architecture §Design Philosophy, the member count words — **carried**: "Crates / modules" and "Counts /
    qualifiers moved". `test-support`: architecture 0+0 (not yet named anywhere); the count pattern: architecture
    2+0 (`:4`, `:14`), and "fourteen library crates": architecture 1+1.
  - architecture §Cross-cutting Patterns → Module dependency direction, the carve-out in the operator's words —
    **carried**: "Dependencies" (the new dev-only edges) and "Symbols / APIs" (the pin). `Module dependency
    direction`: architecture 1+0 (`:289`, which says "no library crate depends on a sibling unless its declared
    contract requires it" and states no carve-out). The words are the operator's (inputs#I3, inputs#I8 item 4):
    "a dev-only edge from a member to `test-support` is allowed, and only that; … pinned by a committed test".
    inputs#I8 item 4: if the playbook reads that sentence as a boundary widening, the second window halts in the
    ordinary form, in full (each proposal, why it escalated, the recommendation), and the operator's answer there
    is provisional; it is not to be written unhalted. The first wording of that item (inputs#I6) and the
    invocation line (inputs#I7) said one card; the corrected relay replaces it.
  - architecture §Occupied Resources → Network ports and → Filesystem locations (the carry's file claims its
    ports; four binaries and two unit tests share one claim); architecture §Stack and Technologies → Gate-script
    pins (test-only) (the claim left `xtask/tests/common/mod.rs`; the new xtask pin) — **carried**: "Symbols /
    APIs" → Ports / sockets and Removed, "Harness / gate surface". `claim_port`: architecture 1+0 (`:234`);
    `test-ports`: architecture 3+0, test-plan 3+1; `xtask/tests/common/mod.rs`: architecture 2+0, test-plan 1+0;
    `Gate-script pins`: architecture 1+0; `license_check`: architecture 1+0.
  - test-plan §3 Bootstrap phases (item 6), §1 Pending coverage triggers
    (`test-data-bootstrap-factories-unimplemented`), §5 (two rows), §7 and the keyed contract Test data bootstrap —
    the factories as built: their names, their one home, what of item 6 is dropped — **carried**: "Symbols /
    APIs" (names and home) and its "Not built" line. `MockTraceSpan`: test-plan 6+3; `MockArrowBatch`: test-plan
    4+1; `MockMetricPoint`: test-plan 2+1; `test-data-bootstrap-factories-unimplemented`: test-plan 2+1; key
    files `contracts/test-plan/test-data-bootstrap.md` and `contracts/test-plan/bootstrap-phases.md`. As built,
    none of the three specified names exists: the set is `test_support::otlp`'s free functions and builders, in
    one crate (not one per `ingest` / `buffer` / `viz`), with no Arrow-batch factory and no `rstest` dependency
    in the member.
  - test-plan §9 Pipeline structure (Integration tests row) and §3 Per-chunk gate discipline → Spawned-program
    witness form (four claiming binaries, the owner sentence closed); test-plan §2 and §4 (the new crate at the
    three library-crate enumerations and the member count) — **carried**: "Counts / qualifiers moved" and
    "Symbols / APIs" → Ports / sockets. `integration_engine_boot`: test-plan 7+1; `bind-and-release`: test-plan
    1+0, architecture 1+0; the member-count pattern: test-plan 1+0 (`:194`).
  - security-plan §Security Anti-Patterns → Input, the port claim's one home and its files under
    `target/test-ports/` — **carried**: "Symbols / APIs" → `test_support::ports` and Places on disk.
    `test-ports`: security-plan 0+0, `claim_port`: security-plan 0+0 — the section does not name the claim today,
    as the plan says; the fact is new text for it, not a correction of a line.
  - Route, owed at the wrap: a `CARRY:` on `Span keeps its identity` for the entry's word "version" — **carried**
    under "Symbols / APIs" → Not built; it is P5's (route-resolve), not a master amendment (inputs#I3, inputs#I8
    item 5).
  - Ledger notes: P-089 and P-094 each take a dated note at this wrap (inputs#I8 item 3) — **ledger-note —
    owner P7.3**. Both entries read `planned`, unclaimed, and already hold the phase P5 note of 2026-10-10. What
    the data can now state: of P-089, a test states the instant stamped inside a span, a span event, a metric
    point and a log record (`at`), and no check reasons by it yet; of P-094, a test states a span's attributes,
    its events' attributes and the sending service, and no stored span field keeps them yet.
- **Coverage of new surfaces** (no new external surface, product operation or UI element; three test-only
  surfaces):
  - `test_support::otlp` factories → validation n/a · instrumentation n/a · PII defaults-unmasked✓ (the
    `default_built` test; a set canary still redacts in `dispatch_folds_redactions_after_a_successful_append` and
    in `e2e_pii_span_masking_stored_field`) · tests unit (10 in the member) + six consuming files · a11y n/a ·
    tokens n/a
  - `test_support::ports::claim_port` → validation n/a · instrumentation n/a · PII n/a · tests unit (4 in the
    member) + the four port-using binaries, 7 runs of 7 · a11y n/a · tokens n/a
  - xtask pin `test_support_edges` → validation n/a · instrumentation n/a · PII n/a · tests unit (5), red-first
    and two mutations red · a11y n/a · tokens n/a

## Deviations from intent

**Scope record: none — `gate.py scope` clean, 0 recorded** (`scope: clean — changed 20 · listed 20 · recorded 0 ·
absorbed 0 · excluded 63`, base `50216d7e`, at this wrap's P1).

1. **`random_word` kept in `xtask/tests/common/mod.rs`.** Plan step 4 lists it among what the file removes;
   `run_passphrase` in the same file calls it. The claim, its struct and its two constants were removed.
2. **A fourth site in `consumer.rs` moved.** Step 6 names three helpers. The fingerprint test's inline span and
   two events moved too, so the file holds no record literal. The plan's own prediction (span literals 25 → 19)
   counts that site.
3. **Forms of the factories not built for lack of a caller:** a metrics export request, a span-duration setter, a
   no-resource form. Step 5 asks for "the request and resource-level forms the tests send or hand to the buffer";
   no moved test sends a metrics request. inputs#I1 item 1: a factory nothing calls is not built here.
4. **Data the moved tests now leave to defaults**, with no assertion changed: trace and span ids in the two engine
   files and the PII file; the service of `consumer.rs`'s span helpers (they built `resource: None`, now the
   default service); event names and times in `span_batch_with_events`; the PII log record's observed time (was
   the wall clock, now 0 — `grep -rn observed_time_unix_nano crates/buffer/src crates/ingest/src` outside the
   generated stubs: 9 hits, all test literals writing 0, no product reader).
5. **The attribute setters on the span and on the log record have no caller outside the member's own tests.** The
   plan's step 5 lists all four attribute setters; the outside callers are the span event (PII file, `consumer.rs`)
   and the metric point (`appender.rs`). The plan's Provenance says each of the three setters (time, service,
   attributes) has an outside caller, which holds per setter kind, not per record kind.
6. **One mutation beyond the plan's two.** A factory default changed to trip a scrubber arm
   (`password=hunter2` in the default log body) turned the `default_built` test red (`left: 3`, `right: 0`);
   restored, checksum equal. It shows that pin reads the defaults.
7. **The edge pin's reading of (b)** is slightly wider than the plan's words: beside "exactly `ingest`" it refuses
   any table header of the member's manifest that contains `dependencies` other than `[dependencies]` itself, and
   any that contains `features` — so a `[target.….dependencies]` table reddens it too.
8. **The canary in `consumer.rs` moved into a constant** (`CREDENTIAL_NAME`) with the allow comment on its own
   line; the base had the comment on the line above the literal.
9. **The operator edit of the plan between runs** (inputs#I4, inputs#I5): entry 4's `git diff` takes
   `--src-prefix=a/ --dst-prefix=b/` (`plan.md:179`) and the acceptance line that restates the command follows
   (`plan.md:339-342`). Typed by the implement agent as the operator's hands; /implement edits no plan on its own.
   Record: `evidence/operator-edit-plan.md`.
10. **The census's wrapper column is this re-run's own** (180 at the base), not research's 170; see Changes.

## Decisions & corrections

What curation needs from the conversation of the implement run and the operator pass. Each item gives the fact,
where it was measured, and what is already written elsewhere.

**Operator words and corrections**

- **The answer on gate entry 4 took none of the three options offered** (inputs#I4). Offered: revise the plan
  through /andromeda-phase · set a repo-local `git config diff.mnemonicPrefix false` · report and stop. The
  operator's own text: "Finish the block and report this run as surfaced; set no git config. After the run ends I
  give you an operator edit of the plan between runs (the two prefix flags on entry 4, recorded in evidence as
  mine), and you re-fire that entry - as at the capability-record chunk." Two things in it: host git config is not
  changed to make a probe pass; and a gate entry defective by its own text is corrected by an operator edit of the
  plan between runs, recorded in `evidence/operator-edit-plan.md` with before and after, then re-fired. This is
  the second chunk that took that course (the first: `2026-10-10-capability-record-re-based`).
- **The directive for the pass** (inputs#I5): "If the boot job, the cycle step or a test is red on the runner,
  stop and report what the job kept; fix nothing on top. Start no skill - the wrap is mine to call." Nothing read
  red; the stop did not apply.
- **The wrap directive** (inputs#I8, inputs#I7): two windows; stops after P1, at any widening card, at the
  route-resolve card, and before the flip and the commit with `git status --short`.
- **The directive was corrected by the operator while P1 ran.** Item 4 first said the dependency carve-out stops
  on one card with one confirm; the corrected relay says it halts in the ordinary form, in full, and that the
  operator's answer there is provisional (inputs#I8; the first wording is kept as inputs#I6). Found by the second
  `inputs.py verify`, which read `drifted`; the copy and the source were then diffed. For curation: an input
  snapped at the start of a step can move while the step runs; the verify after the report is what caught it, and
  the first verify (seconds after the snap) could not have.

**Sweep hazards and host facts found**

- **A probe that parses `git diff` header lines reads `+++ w/` on this host.** The user git config sets
  `diff.mnemonicPrefix = true`: a commit against the work tree prints `c/` and `w/`, two commits print `a/` and
  `b/`. A probe keyed on `+++ b/` then never sets its file name. The entry was wrong in both directions (a false
  red on a hunk below its fall-through line, a possible false green above it). Remedy as applied: pin
  `--src-prefix=a/ --dst-prefix=b/` on the `git diff`. Nothing in the masters or leaves states this
  (`grep -rnF 'diff.mnemonicPrefix'` over masters, registries, `CLAUDE.md`, rules, docs: 0 hits). No other
  committed parser carries the form: `grep -rnE '\\?\+\\?\+\\?\+ [bw]\\?/'` (the header token, bare or
  regex-escaped) over `xtask scripts .github crates pulse-app/src pulse-app/tests .config`
  (`.rs .sh .py .yml .ps1 .mjs .toml`): 0 hits; over every chunk plan of 0.3.0 and 0.4.0: 1 file, this chunk's
  own, which is the pattern's control (1 line). A first sweep with the fixed string `+++ b` read 0 everywhere,
  this plan included: the awk spells the token escaped, so that sweep could not have found it.
- **A probe's known-positive control must run in the probe's own shape.** Entry 4's control was a
  commit-to-commit diff and its baseline an empty diff; neither printed the header form the entry reads when it
  runs. `.claude/rules/testing.md` already holds the neighbouring rule (2026-10-10: an atom that prints only once
  the chunk's edit exists cannot be vouched for by a baseline on the untouched tree); this is the control-side
  case of it. After the edit the control was re-run as a commit against the work tree (`514fbe57` against the
  tree: 25 lines, each with its file name).
- **A plan step that names a function for removal can name one with a second caller.** `random_word`: step 4's
  list against `run_passphrase`. `grep` for callers of each named removal before removing it. The Tier 1 rule on
  orphan-graph mapping before a cleanup covers the class.
- **The count arithmetic held.** 20 tests written, workspace count 3112 → 3132 on the dev host, in the pre-push
  check and on the runner. The rule that asks for it (`rules/testing.md`, 2026-08-29) worked as written.
- **Two identical default records in one batch both persist** (two default points of one gauge at one instant;
  three default log records): the `seq` ordinals hold them apart. A default-built batch needs no per-record
  variation to land whole.
- **A second lock on a live claim's file is refused inside one process** (`File::try_lock` on a second open of
  `target/test-ports/{number}` returns an error while the claim lives, and succeeds after it drops): the member's
  test reads it directly.
- **A test-support crate cannot serve unit-test modules of the crate it depends on.** `crates/ingest/src` test
  modules cannot take `test-support`: the test build of `ingest` and the `ingest` the member links are two crates
  to the compiler. Three files of the census stay hand-built for that reason (`crates/ingest/src/{grpc,http,
  invariants}.rs`). From the plan's notes; not measured by a compile here.
- **`gitleaks` passed with the allow comment on the literal's own line.** The base had it on the line above, and
  the base also passed its scans, so this run does not show which placement the scanner needs. The scanner is not
  on the dev host.

**Slips of the agent in this session**

- One Bash-guard refusal: a `cat` heredoc with a file target (writing an edit script). Re-issued once as anchored
  Edit calls. The handoff carries this as `recurrence-despite-learning: host-linux.md, Transports`; this is one
  more occurrence. No leading-`cd` refusal occurred.
- The smoke-step evolve record's `ts` and `id` were passed through a shell variable read from `date -u` in the
  same call, where the evolve reference asks for the stamp written literally by the author. The value was echoed
  into the window before the append and the append landed (`ids 2026-10-10T21:28:26Z-a`).
- The first draft of the acceptance-line edit split a code span across two lines; corrected in the next edit
  before anything read it.
- At this wrap's P1 the sweep for other diff-header parsers was first run with a fixed string that could not
  match the escaped spelling the awk uses; it read 0 on the one file known to hold the token. Re-run with a
  pattern that matches both spellings and with that file as its control before the count was written above. The
  rule it broke is already in `.claude/rules/observability.md` (2026-08-21, extended 2026-08-26: verify a
  prescribed grep returns a known-present control before trusting a negative).

**Things that went as designed (no learning, recorded so they are not re-derived)**

- Red first: the edge pin read 4 passed, 1 failed before any member named the set.
- Each mutation was read back as present before its run, and each restore was confirmed by checksum.
- The operator's word for the pass was snapped (inputs#I5) before the pre-CI commit; the two entries fired between
  the commit and the push ran with no run dir; the tree was clean at the push guard.
- No `REFUSED id:` or `REFUSED ts:` in any trail of this chunk's phase, implement or wrap run dirs
  (`grep -l REFUSED` over the three dirs' `*.json`: no file). For the main overseer (inputs#I8 item 9).

**Found and not owned** (inputs#I8 item 8 — for the card in the second window, each with a lean)

1. Entry 4's `baseline` key in `plan.md` still describes its P5 reading (an empty diff, a commit-to-commit
   control). Lean: leave; the plan is a record and `evidence/operator-edit-plan.md` says what the entry reads now.
2. research.md's wrapper count 170 is not reproduced (180). Lean: no edit (a chunk artifact gets a report entry);
   nothing consumes the figure.
3. Nothing durable states the host's diff-prefix setting. Lean: one Tier 2 entry in `.claude/rules/host-linux.md`
   (it serves the host), or an extension of the 2026-10-10 baseline entry in `rules/testing.md`; curation decides.
4. The attribute setters on the span and the log record have no outside caller yet. Lean: none owed; `Span and
   resource attributes kept` and `Logs and metrics name their service` are their next consumers by the route.
5. `crates/test-support` is test support and sits inside the coverage measure. Lean: leave; the plan states it,
   and both thresholds read above their bars with it inside.
6. `target/test-ports/` grows: the member's own tests draw 98 numbers per run of its test binary (64, 32, 1 and
   1 across its four tests), and no
   file is ever unlinked. The last handoff counted about 1 400 files there and records the operator's word that
   nothing is owed for them. Lean: leave; `cargo clean` removes them.
7. The plan's Provenance sentence on outside callers holds per setter kind only (Deviation 5). Lean: leave.

**What the last handoff left to this wrap's cascade** (inputs#I8 item 7): the handoff at `50216d7e` names nothing
as owed to the next wrap's cascade. Read: `git show 50216d7e:.claude/session-handoff.md`, lines matching
`cascade|owed|next wrap` — four hits. Line 33-35: the leaf layout list is closed, "Nothing of it is carried".
Line 49: "Nothing owed, the operator's word at the card". Line 68: about 12 leaf statements "stand from the wrap
before". Its other standing items are written as left or carried on the operator's word, not as owed to a
cascade: the leaf statements "seen and left" (`ui-bridge.md:38`, the test-tool version numbers in the service
notes, `workspace-detector.md:52`, `viz.md:15`), the stale line citations in preserve-verbatim homes, and the
"Carried, not re-read this wrap" list.

**The carries the plan owes, by owner** (inputs#I8 item 5 — the table for the route-resolve card; counts from
`evidence/census.md`, unmoved hand-built literals as span · span event · number point · log record)

| route entry (line on the working route) | what it would carry | files | literals |
|---|---|---|---|
| `Span keeps its identity` (139) | the version setter, built with the stored column that reads it (inputs#I3); `e2e_pii_bare_credential_stored_field.rs`; a share of `appender.rs` | 1 + a share | 1 · 1 · 0 · 1, and with the three entries below the 1 · 1 · 5 · 9 of `appender.rs` |
| `Span and resource attributes kept` (141) · `Histogram metrics keep their shape` (145) · `Logs and metrics name their service` (147) | the rest of `appender.rs`'s test module | a share | (shared, above) |
| `Network OTLP receiver behind the token` (92) · `Receiver refusals` (94) · `Per-sender bounds` (96) | six `crates/ingest/tests` files; three `crates/ingest/src` files that cannot take the member | 9 | 7 · 3 · 0 · 0 |
| `Scenario legs driven against the console engine` (39) | four `crates/ingest/examples/inject_*.rs` | 4 | 2 · 1 · 2 · 2 |
| `Load profiles re-based on the engine` (124) | three `pulse-app/tests/perf_*.rs`, `examples/load_profiles.rs` | 4 | 4 · 0 · 0 · 0 |
| `Detection baseline through the console engine` (47) | `e2e_storm_detection.rs` | 1 | 1 · 1 · 0 · 0 |
| `Detection parity after the removals` (83) | `e2e_p1_otlp_grpc_to_traces_query.rs` | 1 | 1 · 0 · 0 · 0 |
| `Display-only computation retired` (58) | `e2e_p6_channel_arrow_ipc.rs` | 1 | 1 · 0 · 0 · 0 |
| `Desktop distribution retired` (54) | `xtask/src/smoke.rs` | 1 | 1 · 0 · 0 · 0 |

Sums: 19 · 7 · 7 · 12, the hand-built totals. Said in advance: `Span keeps its identity` would take two blocks
from this wrap (the version setter; its share of the unmoved sites), and the three `ingest` owners and the four
`appender.rs` owners each share one set of files that the table does not split. What each entry already carries
was not read at P1 (`route.py pins` is P5's); whether an entry would hold more than it can is said at the card.
The owners are research's proposal as shown on the P5 card of the phase; none of those entries has been planned.

## Outcome

**Acceptance criteria, each re-asserted against the diff**

- (tests) One set of factories builds spans with their events, metric points and log records; set values read
  back; defaults fixed; no clock, no randomness — **met**. `crates/test-support/src/otlp.rs` holds no call of a
  clock or a random source; its ten tests and the entry
  `cargo nextest run … -E 'package(test-support) | (package(buffer) & test(default_built)) | (package(xtask) &
  test(test_support_edges))'` are green (20 of 20). Event time, service and attributes are settable; the version
  setter is not built, by the operator's answer (inputs#I3).
- (obs) A default-built batch reads `rows_ingested` equal to the rows sent, `redactions_applied` 0,
  `append_rejections` 0 — **met** (`consumer.rs` 625-666).
- (security) Every factory default passes the write boundary unmasked; a set canary still reads redacted —
  **met**: the same test; `dispatch_folds_redactions_after_a_successful_append` (one redaction) and
  `e2e_pii_span_masking_stored_field` (four) pass unchanged in their assertions.
- (arch) One added member, named and accepted by the operator; no member names it outside `[dev-dependencies]`;
  it depends on `ingest` alone — **met**: the five pin tests pass; both planned mutations read red
  (`evidence/mutations.md`).
- (arch) No product line changes for the factories — **met** against the diff: the scope guard prints nothing;
  the test-module-only guard, with its prefixes pinned by the operator edit, prints nothing. As the plan first
  wrote that guard it read red on this host for a reason that is not a product line (Changes, Deviation 9).
- (tests) The carry is closed — **met**: `integration_engine_boot.rs` hands the engine two claimed ports in each
  in-process arm; the port-0 census reads 0.
- (tests) One definition of the port claim serves the four binaries and the two unit tests — **met**: the census
  reads 1.
- (tests) The four port-using binaries read green seven of seven — **met**, as a count
  (`evidence/repeated-run.md`).
- (tests) Six test files take their telemetry from the factories and hold none of the literals they replaced;
  the census names an owner for every unmoved file — **met**. Five of the six hold no record literal; `appender.rs`
  holds none in the moved helper group and keeps what its other tests build by hand, as the plan states.
- (security) `cargo deny check bans licenses sources` and `cargo audit` exit 0 with no `deny.toml` change;
  `Cargo.lock` gained one workspace entry and no registry package — **met**.
- (obs) No `tracing` emit site, target or allowlist leaf added in product source; no OTel SDK crate in the graph
  — **met**: no product source line changed; the lockfile delta names `test-support` alone.
- (tests) The standard gate set passes in order; the bindings close reads exit 0; the local pre-push check reads
  green with its teardown probe — **met**, entry 4 after the operator edit.
- (tests) The pull-request run of the pushed tip reads `verdict: green`, its `coverage` job included — **met**:
  `ci#38088083705` on `d9651126`.
- **Capabilities: none claimed** — holds. `matrix.py show --chunk`: claimed 0. P-089 and P-094 read `planned`,
  unclaimed.

**Gates** (the plan's entries by `run`, in order; verdicts from the gate tool, gate v1.14)

- `cargo fmt --check` — green · exit 0
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — green · exit 0
- `git diff --name-only 50216d7e… -- crates pulse-app xtask …` (the scope guard) — green · exit 0 · no output
- `git diff -U0 … | awk …` (the test-module-only guard) — first firing as P5 wrote it: `red · exit 0 ✗ (exit 1)`,
  25 B of output; after the operator edit, `git diff -U0 --src-prefix=a/ --dst-prefix=b/ …`: green · exit 0 · no
  output (firing 2, `4.2.log`). The red was the entry's, not the tree's.
- `cargo nextest run … -E 'package(test-support) | …'` — green · exit 0 · 20 of 20
- `grep -rlE … 'fn claim_port|struct PortClaim|fn pick_port' … | wc -l` — green · exit 0 · last line `1`
- `grep -nE 'bind\(\("127\.0\.0\.1", 0\)\)|…' … | wc -l` — green · exit 1 · last line `0`
- `for i in 1 2 3 4 5 6 7; do cargo nextest run … ; done` — green · exit 0 · 118.98 s · seven `Summary` lines of
  38 passed
- `cargo deny check bans licenses sources` — green · exit 0 (`bans ok, licenses ok, sources ok`)
- `cargo audit` — green · exit 0
- `cargo xtask check:english-sources` — green · `"verdict": "clean"`, 511 files
- `cargo xtask quarantine-tracking` · `capability-widening-check` · `check:ingest-progress` ·
  `check:staged-artifacts` · `capability-drift` · `verify:capability-matrix` — each green · exit 0
- `cargo nextest run --workspace --profile ci --no-tests=fail` — green · 3132 of 3132
- `cargo nextest run -p pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'` — green
- `git diff --quiet 50216d7e… -- pulse-app/ui/src/bindings/index.ts` — green · exit 0
- `d="$(mise where node@24)" && PATH="$d/bin:$PATH" cargo xtask pre-push:linux` — green at /implement (153.62 s)
  and again on the committed tree in the operator pass (108.0 s, `"head": "d9651126…"`): `"verdict": "green"`,
  `all-stages-ok`, `engine-log: PASS`, `cleanup: clean`
- `test ! -e target/pre-push/run/data/run/andromeda-pulse.pid && …` (the teardown probe) — green both times
- `gate.py hygiene` (`leg = 'operator'`) — `hygiene: clean — read 53`, before the pre-CI commit
- the merge-base probe (`leg = 'operator'`) — green · `history: unmoved`
- the push (`leg = 'operator'`) — green · `history moved: refs/remotes/origin/build/andromeda-pulse-0.4.0
  50216d7e→d9651126`, the intended move
- `ci.py conclusion --sha HEAD --wait 2700` (`leg = 'operator'`) — exit 0 · `verdict: green · checks 7/7`
- No `defer`, no skip, no timeout, no `INDETERMINATE:` reading. No smoke: no boot-path or UI-surface file was
  touched and the plan lists no smoke entry.

**Watches:** none folded.

**Outcome basis.** The operator pass ran, so the gate verdicts rest on its final state: the commit list of Setup
(`d9651126`, alone) and the CI run of that HEAD recorded in `evidence/operator-pass.md`. Between /implement's P4
report (outcome surfaced, on entry 4) and this report stand the operator's directive inputs#I5 (the plan edit, the
re-fire, the pass) and six evidence files: `mutations.md`, `census.md`, `repeated-run.md`,
`test-module-guard.md`, `operator-edit-plan.md`, `operator-pass.md`. The implement conversation is present in
this window; the friction ledger was not read for this report.

**Process hygiene.** From /implement's census and the pass, re-measured at this wrap
(`ps -eo pid,comm,args` for `andromeda-pulse-engine`, `pulse-app`, `inject_demo`, `gate.py`, `ci.py`: none; ports
24317 and 24318 refuse):

| process | started by | final state |
|---|---|---|
| the console engine and the injector of the pre-push check's sixth stage, twice | gate entry 21 at /implement and in the pass | terminated |
| engines spawned by the port-using test binaries | gate entries 8, 18, 21 | terminated |
| the code-graph refresh | this wrap's Setup | ended, exit 0 (`rust ok 56s 10786/52188` · `ts ok 4s 4164/7755`) |

**Limits of this report**

- One CI run, attempt 1, on the pre-CI commit; this wrap's commit is not covered by it.
- The repeated run is a count of seven, taken once on the dev host; the runner ran the binaries once under the
  test job and once under coverage.
- The boot job's log and the run's two artifacts were not read beyond the step conclusions.
- The site counts per master are `grep -c` line counts of a token; a line that names a token twice counts once,
  and a site worded without the token is not counted.
- Line coordinates of new text come from the section below, read on the tree at this wrap's P1.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 50216d7e (the parent of the oldest pre-CI commit d9651126) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### Cargo.lock — added 10 line(s) in 4 range(s)
added: 881 · 5400 · 7306-7312 · 9746
- 7309-7311 «dependencies = [»
### Cargo.toml — added 1 line(s) in 1 range(s)
added: 18
### crates/buffer/Cargo.toml — added 1 line(s) in 1 range(s)
added: 53
### crates/buffer/src/appender.rs — added 26 line(s) in 7 range(s)
added: 943 · 1594-1598 · 1602-1603 · 1606-1608 · 1612-1615 · 1799-1805 · 1807-1810
### crates/buffer/src/consumer.rs — added 82 line(s) in 8 range(s)
added: 289 · 309 · 311-317 · 321-325 · 329-338 · 625-667 · 698-707 · 709-713
  - 625-666 @630 «async fn a_default_built_batch_of_every_kind_lands_whole_with_no_redaction() {»
### crates/test-support/Cargo.toml — new file · 14 line(s)
### crates/test-support/src/lib.rs — new file · 8 line(s)
### crates/test-support/src/otlp.rs — new file · 719 line(s)
- 19-21 «use ingest::grpc::proto::opentelemetry::proto::metrics::v1::{»
- 35-42 «fn string_attribute(key: &str, value: &str) -> KeyValue {»
  - 36-41 «KeyValue {»
- 44-50 @46 «pub struct SpanEventBuilder {»
- 52-58 «pub fn span_event() -> SpanEventBuilder {»
  - 53-57 «SpanEventBuilder {»
- 60-85 «impl SpanEventBuilder {»
  - 61-64 «pub fn name(mut self, name: &str) -> Self {»
  - 66-70 @67 «pub fn at(mut self, unix_nano: u64) -> Self {»
  - 72-75 «pub fn attribute(mut self, key: &str, value: &str) -> Self {»
  - 77-84 «fn build(self) -> span::Event {»
- 87-96 @89 «pub struct SpanBuilder {»
- 98-107 «pub fn span() -> SpanBuilder {»
  - 99-106 «SpanBuilder {»
- 109-164 «impl SpanBuilder {»
  - 110-113 «pub fn trace_id(mut self, trace_id: [u8; 16]) -> Self {»
  - 115-118 «pub fn span_id(mut self, span_id: [u8; 8]) -> Self {»
  - 120-123 «pub fn name(mut self, name: &str) -> Self {»
  - 125-130 @127 «pub fn at(mut self, unix_nano: u64) -> Self {»
  - 132-135 «pub fn attribute(mut self, key: &str, value: &str) -> Self {»
  - 137-140 «pub fn event(mut self, event: SpanEventBuilder) -> Self {»
  - 142-145 «pub fn events(mut self, events: impl IntoIterator<Item = SpanEventBuilder>) -> Self {»
  - 147-163 «fn build(self, position: usize) -> Span {»
- 166-172 @168 «pub struct MetricPointBuilder {»
- 174-180 «pub fn metric_point() -> MetricPointBuilder {»
  - 175-179 «MetricPointBuilder {»
- 182-209 «impl MetricPointBuilder {»
  - 183-187 @184 «pub fn at(mut self, unix_nano: u64) -> Self {»
  - 189-192 «pub fn value(mut self, value: i64) -> Self {»
  - 194-197 «pub fn attribute(mut self, key: &str, value: &str) -> Self {»
  - 199-208 «fn build(self) -> NumberDataPoint {»
- 211-216 @213 «pub struct GaugeBuilder {»
- 218-223 «pub fn gauge() -> GaugeBuilder {»
  - 219-222 «GaugeBuilder {»
- 225-256 «impl GaugeBuilder {»
  - 226-229 «pub fn name(mut self, name: &str) -> Self {»
  - 231-234 «pub fn point(mut self, point: MetricPointBuilder) -> Self {»
  - 236-239 «pub fn points(mut self, points: impl IntoIterator<Item = MetricPointBuilder>) -> Self {»
  - 241-255 «fn build(self) -> Metric {»
- 258-266 @260 «pub struct LogRecordBuilder {»
- 268-276 «pub fn log_record() -> LogRecordBuilder {»
  - 269-275 «LogRecordBuilder {»
- 278-313 «impl LogRecordBuilder {»
  - 279-283 @280 «pub fn at(mut self, unix_nano: u64) -> Self {»
  - 285-288 «pub fn body(mut self, body: &str) -> Self {»
  - 290-294 «pub fn severity(mut self, number: i32, text: &str) -> Self {»
  - 296-299 «pub fn attribute(mut self, key: &str, value: &str) -> Self {»
  - 301-312 «fn build(self) -> LogRecord {»
- 315-322 @317 «pub struct ResourceBuilder {»
- 324-331 «pub fn resource() -> ResourceBuilder {»
  - 325-330 «ResourceBuilder {»
- 333-446 «impl ResourceBuilder {»
  - 334-338 @335 «pub fn service(mut self, service: &str) -> Self {»
  - 340-343 «pub fn span(mut self, span: SpanBuilder) -> Self {»
  - 345-348 «pub fn spans(mut self, spans: impl IntoIterator<Item = SpanBuilder>) -> Self {»
  - 350-353 «pub fn gauge(mut self, gauge: GaugeBuilder) -> Self {»
  - 355-358 «pub fn gauges(mut self, gauges: impl IntoIterator<Item = GaugeBuilder>) -> Self {»
  - 360-363 «pub fn log_record(mut self, log_record: LogRecordBuilder) -> Self {»
  - 365-368 «pub fn log_records(mut self, log_records: impl IntoIterator<Item = LogRecordBuilder>) -> Self {»
  - 370-375 «fn build_resource(&self) -> Resource {»
  - 377-395 @379 «pub fn resource_spans(&self) -> Vec<ResourceSpans> {»
  - 397-413 @398 «pub fn resource_metrics(&self) -> Vec<ResourceMetrics> {»
  - 415-431 @416 «pub fn resource_logs(&self) -> Vec<ResourceLogs> {»
  - 433-438 @434 «pub fn trace_request(&self) -> ExportTraceServiceRequest {»
  - 440-445 @441 «pub fn logs_request(&self) -> ExportLogsServiceRequest {»
- 448-719 @449 «mod tests {»
  - 454-462 «fn string_value(attribute: &KeyValue) -> (&str, &str) {»
  - 464-472 «fn service_of(resource: &Option<Resource>) -> &str {»
  - 474-481 «fn built_spans(resource: &ResourceBuilder) -> Vec<Span> {»
  - 483-495 «fn built_points(resource: &ResourceBuilder) -> Vec<NumberDataPoint> {»
  - 497-504 «fn built_log_records(resource: &ResourceBuilder) -> Vec<LogRecord> {»
  - 506-512 «fn one_of_each() -> ResourceBuilder {»
  - 514-523 @515 «fn two_builds_from_the_same_calls_are_equal() {»
  - 525-546 @526 «fn an_unset_event_time_is_the_one_fixed_instant() {»
  - 548-563 @549 «fn an_unset_service_is_the_one_fixed_name_on_every_form() {»
  - 565-582 @566 «fn a_set_service_is_read_back_on_every_form() {»
  - 584-599 @585 «fn ids_have_their_wire_lengths_and_unset_span_ids_differ_by_position() {»
  - 601-611 @602 «fn a_set_id_is_kept_and_an_unset_neighbour_keeps_its_position() {»
  - 613-643 @614 «fn what_a_test_sets_on_a_span_and_its_event_is_read_back() {»
  - 645-667 @646 «fn what_a_test_sets_on_a_metric_point_is_read_back() {»
  - 669-698 @670 «fn what_a_test_sets_on_a_log_record_is_read_back() {»
  - 700-718 @701 «fn a_resource_with_nothing_added_builds_empty_forms_under_its_service() {»
### crates/test-support/src/ports.rs — new file · 120 line(s)
- 17-21 @18 «pub struct PortClaim {»
- 27-37 @30 «fn claims_dir() -> PathBuf {»
  - 31-36 «Path::new(env!("CARGO_MANIFEST_DIR"))»
- 39-43 «fn random_word() -> u64 {»
  - 40-42 «std::collections::hash_map::RandomState::new()»
- 45-74 @48 «pub fn claim_port() -> PortClaim {»
  - 52-72 «for _ in 0..10_000 {»
- 76-120 @77 «mod tests {»
  - 80-87 @81 «fn a_claimed_number_lies_in_the_range_and_is_no_harness_port() {»
  - 89-96 @90 «fn two_live_claims_differ() {»
  - 98-113 @99 «fn the_lock_file_of_a_live_claim_cannot_be_locked_again() {»
  - 115-119 @116 «fn a_claimed_port_binds_on_loopback() {»
### pulse-app/Cargo.toml — added 1 line(s) in 1 range(s)
added: 84
### pulse-app/tests/e2e_drain_template_assignment.rs — added 8 line(s) in 4 range(s)
added: 36 · 41-42 · 57 · 79-82
  - 79-82 «resource()»
### pulse-app/tests/e2e_pii_span_masking_stored_field.rs — added 15 line(s) in 3 range(s)
added: 23 · 182-185 · 193-202
  - 182-185 «let logs = resource()»
  - 193-198 «let event = span_event()»
  - 199-202 «let traces = resource()»
### pulse-app/tests/integration_console_engine.rs — added 8 line(s) in 3 range(s)
added: 23-24 · 232-233 · 236-239
  - 236-239 «resource()»
### pulse-app/tests/integration_engine_boot.rs — added 18 line(s) in 9 range(s)
added: 5 · 18 · 34-35 · 127-129 · 133-134 · 146-147 · 166 · 205-206 · 208-211
  - 208-211 «resource()»
### xtask/Cargo.toml — added 3 line(s) in 1 range(s)
added: 28-30
### xtask/src/engine_cycle.rs — added 5 line(s) in 1 range(s)
added: 430-434
### xtask/src/harness_ready.rs — added 5 line(s) in 1 range(s)
added: 777-781
### xtask/src/main.rs — added 2 line(s) in 1 range(s)
added: 35-36
### xtask/src/test_support_edges.rs — new file · 178 line(s)
- 16-21 «fn workspace_root() -> PathBuf {»
  - 17-20 «Path::new(env!("CARGO_MANIFEST_DIR"))»
- 23-25 «fn read(rel: &str) -> String {»
- 27-35 @28 «fn section_lines<'a>(text: &'a str, header: &str) -> Vec<&'a str> {»
  - 29-34 «text.lines()»
- 37-51 «fn workspace_members(cargo_toml: &str) -> Vec<String> {»
  - 39-41 «let Some(start) = workspace.find("members") else {»
  - 43-45 «let (Some(open), Some(close)) = (rest.find('['), rest.find(']')) else {»
  - 46-50 «rest[open + 1..close]»
- 53-59 «fn table_headers(manifest: &str) -> Vec<&str> {»
  - 54-58 «manifest»
- 61-79 @64 «fn lines_naming_the_member(manifest: &str) -> Vec<(String, String)> {»
  - 67-77 «for line in manifest.lines().map(str::trim) {»
- 81-90 @83 «fn declared_names(manifest: &str, header: &str) -> Vec<String> {»
  - 84-89 «section_lines(manifest, header)»
- 92-96 «fn other_members() -> Vec<String> {»
- 98-109 @99 «fn no_member_names_test_support_outside_its_dev_dependencies() {»
  - 100-108 «for member in other_members() {»
- 111-126 @112 «fn test_support_depends_on_ingest_alone() {»
  - 114-118 «assert_eq!(»
  - 119-125 «for header in table_headers(&manifest) {»
- 128-146 @129 «fn test_support_is_a_member_that_some_member_uses() {»
  - 130-133 «assert!(»
  - 134-141 «let users: Vec<String> = other_members()»
  - 142-145 «assert!(»
- 148-167 @149 «fn a_line_naming_the_member_is_listed_with_its_table() {»
  - 150-153 «let manifest = "[package]\nname = \"x\"\n# test-support in a comment\n[dependencies]\n\»
  - 154-157 «let tables: Vec<String> = lines_naming_the_member(manifest)»
  - 158-166 «assert_eq!(»
- 169-178 @170 «fn declared_names_reads_both_spellings_of_a_dependency_line() {»
  - 171-172 «let manifest = "[dependencies]\n# a comment\ningest = { path = \"../ingest\" }\n\n\»
  - 173-176 «assert_eq!(»
### xtask/tests/common/mod.rs — added 2 line(s) in 1 range(s)
added: 16-17
