# Codebase Research — 2026-10-10-shared-telemetry-test-data

## Scope
- **Depth:** deep on the test trees · **Reads:** 14 · **Globs/Greps:** 21
- **Harness rules consulted:** none — no live leg in this chunk (no boot of a product program outside a test binary;
  the repeated run of the port-using binaries is a nextest run)
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg
- **External inputs:** `inputs#I1` — the operator's phase directive, seven items · `inputs#I2` — the invocation line
  naming it

## Files inspected
- `pulse-app/tests/integration_engine_boot.rs` (1-480) — `pick_port` at `:132` binds `127.0.0.1:0`, reads the number
  and drops the listener; `boot` at `:145` calls it twice (`:147`, `:148`) and hands the two numbers to
  `engine_boot::start` as `EngineConfig { grpc_port: Some(OtlpPort), http_port: Some(OtlpPort), .. }`. Four
  `#[tokio::test]` arms call `boot` (`:279`, `:319`, `:360`, `:401`); the two re-exec arms (`:531`, `:574`) take no
  port. `one_span_request` (`:205`) hand-builds one span stamped from the wall clock, 2 call sites.
- `pulse-app/tests/integration_console_engine.rs` (1-135, 270-340, 505-522) — its own `PortClaim` / `claim_port`
  (`:40-83`), 3 call sites; `spans_request` (`:283`) hand-builds five spans stamped from the wall clock. The bind at
  `:513` is a listener the test holds for its own census, not a release.
- `xtask/tests/common/mod.rs` (1-140) — the other `PortClaim` / `claim_port` (`:71-113`); it differs from the twin
  only in how it finds the repository root (`repo_root()` against an inline `env!("CARGO_MANIFEST_DIR")`).
- `xtask/src/harness_ready.rs` (770-786) and `xtask/src/engine_cycle.rs` (424-440) — two unit tests bind port 0,
  read the number, drop the listener and then assert that the released port refuses a connection.
- `crates/buffer/src/consumer.rs` (283-425) — test module from `:284`; `span_batch` (6 call sites),
  `span_batch_with_events` (2), `metrics_batch_with_credential_name` (3), all on fixed stamps.
- `crates/buffer/src/appender.rs` (930-942, 1618-1652, 1846-1890) — test module from `:931`; the metric-point helper
  group `gauge_point`, `gauge_metric`, `labelled_metric_batch` (17 uses) is the only place a test sets attributes
  on a metric point outside the dev examples.
- `pulse-app/tests/e2e_drain_template_assignment.rs` (1-135) — `make_log_record(body, ts)` (5 call sites), 100 log
  records with stated bodies and fixed stamps, service set on the resource.
- `pulse-app/tests/e2e_pii_span_masking_stored_field.rs` (40-290) — one log record, one span and one span event with
  attributes that carry canaries on purpose; service set; stamps from the wall clock.
- `crates/ingest/tests/http_loopback.rs` (1-125) — the receiver tests build requests with no resource, zero stamps
  and default records.
- `crates/buffer/src/retention.rs` (grep) — the sweep deletes `WHERE ts_unix_nano < ?` with the cutoff from
  `SystemTime::now()` (`:178-179`), and the spans table takes seven columns (`:222`).
- `crates/ingest/Cargo.toml`, `crates/buffer/Cargo.toml`, `xtask/Cargo.toml`, `pulse-app/Cargo.toml`, root
  `Cargo.toml` (members) — dependency edges as they stand.
- `.andromeda/registries/contracts/test-plan/test-data-bootstrap.md` (full) — the keyed contract names
  `MockTraceSpan::builder()` and `rstest` composition.

## How telemetry is built in tests today (inputs#I1 item 1)
Derivation: a line scan of every tracked `.rs` file (`git ls-files '*.rs'`) for the struct literals `Span {`,
`Event {`, `NumberDataPoint {`, `LogRecord {` and for the wrappers around them (`Export…ServiceRequest {`,
`Resource… {`, `Scope… {`, `Metric {`, `Resource {`), lines holding `->`, `fn`, `impl`, `use` or a leading `//` left
out so that a signature is not counted as a literal.
- **26 files hold a span, span-event, number-point or log-record literal:** 25 span, 11 span-event, 10 number-point
  and 14 log-record literals, inside 170 wrapper literals. Two more files (`crates/ingest/src/grpc.rs`,
  `crates/ingest/src/http.rs`) hold wrapper literals only. 28 files in all.
- **By place:** `crates/ingest` 14 files (6 under `tests/`, 5 examples, 3 unit-test modules under `src/`),
  `crates/buffer/src` 2 (`appender.rs`, `consumer.rs`), `pulse-app/tests` 11, `xtask/src` 1 (`smoke.rs`).
  **None sits in a crate that leaves in Epochs 2 and 3.**
- **Shapes:** every site that needs a request re-types the same four nested wrappers; 28 local helper functions do
  it (`make_span` ×5, `make_export_request` ×5, `make_request` ×2, `make_batch` ×2 and 14 one-offs; name sweep
  `fn (make|build|one|…)…(span|request|metric|log|record|point|batch)…(`).
- **Event time** (per file, the same scan's `SystemTime::now|now_ns()` and `1_7dd_ddd_ddd|_unix_nano: 0` columns
  over the 28 files): 16 files read the wall clock, 11 write a fixed or zero literal, 5 of them do both, and 6
  leave every stamp at its default. No site takes the instant as a parameter of a shared builder.
- **Service:** set by a hand-built `service.name` resource attribute in 19 of the 28 files; left unset
  (`resource: None`) in the ingest receiver tests and in `consumer.rs`'s span helpers.
- **Version:** no test sets a sender's `service.version` (`grep -rn '"service\.version"'`: 5 hits, all the
  product's own log identity). The product stores no version (seven span columns, `retention.rs:222`).
- **Attributes:** on span events and log records in the two PII files and `appender.rs`; on metric points in
  `appender.rs` and two dev examples only.
- **The three factories test-plan §3 names** (`MockTraceSpan`, `MockArrowBatch`, `MockMetricPoint`): 0 hits each at
  the chunk base; control `fn claim_port`: 2 hits.

## Port picking (the carry and what the sweep found beside it)
Sweep `\[127, 0, 0, 1\], 0\)|127\.0\.0\.1:0|"127\.0\.0\.1", 0\)|localhost:0|\(Ipv4Addr::LOCALHOST, 0\)` over
`crates pulse-app xtask`: 27 lines in 20 files · 1 a comment · 3 release a port they read · 23 keep the listener
they bound.
- **Release sites, 3:** `pulse-app/tests/integration_engine_boot.rs:133` (the carry: the number is handed to an
  engine that binds it later); `xtask/src/harness_ready.rs:777` and `xtask/src/engine_cycle.rs:430` (each asserts
  that the port it just released refuses). The carry's text and the take-up's sweep named the first only.
- **Keep sites, 23 in 17 files:** a listener bound on port 0 that the test itself serves on or holds (the ingest
  receiver tests, nine `pulse-app` end-to-end and perf binaries, the census arm at
  `integration_console_engine.rs:513`). Nothing is released, so nothing can be taken in between; they lie in the
  kernel's ephemeral range, outside 20000 to 31999.
- **The claim's terms as the two twins hold them:** a number from 20000 to 31999, 24317 and 24318 left out, an
  advisory lock on `target/test-ports/{number}` held for the life of the claim, the port shown bindable when
  claimed. The range cannot yield 4317 or 4318. The directory is found from the compile-time
  `CARGO_MANIFEST_DIR`; no environment variable is read at run time.
- **The measurement behind the carry:** the last chunk's report (`report.md:141-143`, `:267-269`) records one of
  seven hand runs failing with bind-and-release and 14 of 14 green after the claims, "a count, not an exclusion".
  Spot-checked as recorded; not re-run here.

## Graph impact
- **`claim_port` / `pick_port`** — 5 call edges: `spawn` and one arm in
  `pulse-app/tests/integration_console_engine.rs` (`:125`, `:577`), `boot` in
  `pulse-app/tests/integration_engine_boot.rs` (`:147`, `:148`), `new` in `xtask/tests/common/mod.rs` (`:253`).
  No caller outside the three files; the name sweep `claim_port|PortClaim|CLAIMED_PORTS|HARNESS_PORTS|test-ports|
  pick_port` over the tracked tree reads 26 hits in those three files and none elsewhere, so no test pins their
  text.
- **`ingest` crate edges** — used by `buffer`, `pulse-app`, `ui-bridge`, `xtask`; `ingest` uses no sibling.
- **Who reaches `ingest`'s surface from outside its `src/`** — 181 sites in `pulse-app/tests`, 110 in
  `crates/ingest/tests`, 48 in `crates/buffer/src`. The OTLP types the factories need are already public and are
  built from tests today.
- **Names** — `PortClaim` and `claim_port` exist twice (the twins); a module `telemetry` exists in `ui-bridge`; no
  symbol `test_support`, `SpanBuilder`, `MetricPointBuilder` or `LogRecordBuilder` exists.

## Patterns detected
- **A shared module inside one package** (`xtask/tests/common/mod.rs:1-9`): one file, two test binaries, no manifest
  change. It does not reach a second package.
- **No dev-only edge between workspace members exists today** (an awk over every member's `[dev-dependencies]`
  for `path =`: 0 lines).
- **The retention sweep reads the record's own stamp against the wall clock** (`crates/buffer/src/retention.rs:23-26`,
  `:178-179`): a record with a fixed past stamp is deleted by the first sweep of a running engine. A test that
  feeds a live engine states "now" itself; a test that seeds a store directly states a fixed instant.
- **A replayed span identity fails its whole batch** (obs-plan §10; `consumer.rs` holds the pin): default ids must
  differ inside one built request, and a test that wants a duplicate sets it.
- **Local helpers wrap the literals**: in every candidate file the tests call a local helper, so moving a file
  onto the factories changes helper bodies and their call sites, not what a test asserts.

## Conventions to follow
- **Builder form with `rstest` composition** (test-plan §3 Test data bootstrap → Mechanism): set what the test
  states, default the rest.
- **The instant is an argument** (test-plan §4, triage crate): no factory reads a clock.
- **Library crates keep their tests in a co-located `mod tests`** (test-plan §2); `pulse-app`'s live under
  `pulse-app/tests/`.
- **A new member inherits the workspace package fields** (`license.workspace`, `rust-version.workspace`): the xtask
  pins `license_check` and `declared_floor_equals_the_pinned_channel` read every member of the root manifest.
- **`check:english-sources` and `quarantine-tracking` read `crates/` whole** (`xtask/src/source_lint.rs:25`,
  `xtask/ci/quarantine-tracking-check.sh:20`): a home under `crates/` needs no new root.

## Where one set can live (the P4 question)
- **A new dev-only workspace member** under `crates/`, depending on `ingest` for the OTLP types and taken by its
  consumers as a dev-dependency. It changes no product source and is compiled once. It is a seventeenth workspace
  member (architecture's canonical list and every count word change), it adds dev-only edges between members
  (none exists today, and architecture's dependency-direction rule states no carve-out for them), and `Cargo.lock`
  gains one workspace entry and no registry package. A unit-test module inside `crates/ingest/src` cannot use it
  (the test build of `ingest` and the `ingest` the member links are two crates to the compiler); `ingest`'s
  integration tests and every other consumer can.
- **One source file per concern, included by path** from each consuming test target: no member, no manifest
  change, no edge; each includer compiles its own copy, and the include crosses package directories.
- **A module inside `ingest` behind a Cargo feature** is ruled out by architecture §Cross-cutting Patterns
  (no separate test-ingest feature flag) and by inputs#I1 item 4.
- The port claim can live in the same home; it then serves the three claiming binaries and the carry's file from
  one definition, and the two release-then-read unit tests in `xtask/src` can take a claimed number as well.

## Owners of the sites this chunk does not move (inputs#I1 item 5; proposed, for the P5 card)
- `crates/ingest/tests/*` (6 files) and `crates/ingest/src/{grpc,http,invariants}.rs` — `Network OTLP receiver
  behind the token`, `Receiver refusals`, `Per-sender bounds`.
- `crates/buffer/src/appender.rs`, what stays of it — `Span keeps its identity`, `Span and resource attributes
  kept`, `Histogram metrics keep their shape`, `Logs and metrics name their service`.
- `pulse-app/tests/e2e_storm_detection.rs` — `Detection baseline through the console engine`.
- `pulse-app/tests/e2e_p1_otlp_grpc_to_traces_query.rs` — `Detection parity after the removals` (critical paths
  restated). `pulse-app/tests/e2e_p6_channel_arrow_ipc.rs` — `Display-only computation retired`.
- `pulse-app/tests/e2e_pii_bare_credential_stored_field.rs` — `Span keeps its identity` (each field scrubbed
  before storage).
- `pulse-app/tests/perf_*.rs` (3) and `crates/ingest/examples/load_profiles.rs` — `Load profiles re-based on the
  engine`.
- `crates/ingest/examples/inject_*.rs` (4) — `Scenario legs driven against the console engine`.
- `xtask/src/smoke.rs` — `Desktop distribution retired`.

## New files to create
- `crates/test-support/Cargo.toml` — the dev-only member's manifest (the P4 answer, inputs#I3)
- `crates/test-support/src/lib.rs` — the two modules' declarations
- `crates/test-support/src/otlp.rs` — the factories for spans, span events, metric points and log records, with their own tests
- `crates/test-support/src/ports.rs` — the one port claim, with its own tests
- `xtask/src/test_support_edges.rs` — the committed pin of the dev-only edge (inputs#I3), a test-only module

## Files to modify
- `Cargo.toml` — the member list
- derived `Cargo.lock` by `cargo build --workspace --tests` — one workspace entry
- `crates/buffer/Cargo.toml` — a dev-dependency on the set
- `pulse-app/Cargo.toml` — a dev-dependency on the set
- `xtask/Cargo.toml` — a dev-dependency on the set
- `xtask/src/main.rs` — the test-only module's declaration, beside `license_check` and `rust_floor`
- `pulse-app/tests/integration_engine_boot.rs` — claimed ports for the four in-process arms; its span from the factories
- `pulse-app/tests/integration_console_engine.rs` — its claim twin removed; its spans from the factories
- `xtask/tests/common/mod.rs` — its claim twin removed
- `xtask/src/harness_ready.rs` — test module only: the released-port test takes a claimed number
- `xtask/src/engine_cycle.rs` — test module only: the released-port test takes a claimed number
- `pulse-app/tests/e2e_drain_template_assignment.rs` — its log records from the factories
- `pulse-app/tests/e2e_pii_span_masking_stored_field.rs` — its span, span event and log record from the factories
- `crates/buffer/src/consumer.rs` — test module only: its spans, span events and metric point from the factories
- `crates/buffer/src/appender.rs` — test module only: the metric-point helper group from the factories

## Open questions
- none. Both plan-decision questions were answered by the operator at P4 (inputs#I3): the set lives in a new
  dev-only workspace member `test-support`, its dev-only edge allowed on the operator's word and pinned by a
  committed test; the version setter is not built here and is carried at the wrap to `Span keeps its identity`.
  The file lists above are the answer's.
