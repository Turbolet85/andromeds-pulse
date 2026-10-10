# Scope — Shared telemetry test data

**Marker:** `2026-10-10-shared-telemetry-test-data` · version `andromeda-pulse-0.4.0` · Epoch 1 — Foundation: base CI, the capability record, a console engine with its door, its gates
**Working entry (`working-route.md:35`):** "Shared telemetry test data — spans, metric points and log records from one
set of factories; event time, service, version and attributes settable (P-089, P-094)"
**Chunk base:** `50216d7e` (HEAD at take-up). Every diff-shaped probe of this chunk names this sha, never HEAD.

## Intent
Tests across the workspace build their spans, metric points and log records by hand, each site fixing its own time
stamp, service name and attributes. The entries that follow in this version (the door, the detection baseline,
event time, the kept span fields) all need telemetry whose event time, service, version and attributes a test can
set on purpose. This chunk gives the tests one set of factories for that data and moves a first group of existing
tests onto it, so that the set ships with a consumer. It is test data only: no product surface is built and no
product source changes for the factories' sake.

## What the chunk builds (the entry's own words)
- **Spans, metric points and log records from one set of factories.** One place a test asks for telemetry of the
  three kinds, in place of hand-built values.
- **Event time, service, version and attributes settable.** A test states the time stamped inside a record, the
  service that sent it, that service's version, and the attributes it carries; what it does not state takes a
  default.
- **It names P-089 and P-094.** The entry lists both ids. What of each this chunk can show is in "Capabilities"
  below.

## Folded freight (the entry's one annotation; `route.py pins` lists one block on line 35, 578 chars)
- **CARRY, closes first:** "pulse-app/tests/integration_engine_boot.rs still picks its ports by binding port 0,
  reading the number and releasing it (pick_port, four in-process arms), the way that failed one of seven hand runs
  of the three port-using test binaries before they took claimed ports (measured at
  2026-10-10-engine-harness-proofs-completed; the claim is claim_port in xtask/tests/common/mod.rs and its twin in
  pulse-app/tests/integration_console_engine.rs); pinned on this entry so that it closes first (the operator's word
  at that chunk's route-resolve card, the pc overseer, 2026-10-10)". The chunk moves that file's port picking to
  the claimed-port form. Coordinates re-read at the take-up: `fn pick_port` at
  `pulse-app/tests/integration_engine_boot.rs:132`, called at `:147` and `:148`; `pub fn claim_port` at
  `xtask/tests/common/mod.rs:87`; `fn claim_port` at `pulse-app/tests/integration_console_engine.rs:49`. No other
  `.rs` file outside `target/` names either function.
- `[premise-corrected: the take-up's sweep read one spelling; the full sweep reads 27 lines in 20 files]`
  **Two more tests read a port they have released.** The take-up wrote that no other file binds `127.0.0.1:0`.
  Research's sweep over every spelling of a bind to port 0 (research, "Port picking") finds three release sites:
  the carry's, and two unit tests in `xtask/src/harness_ready.rs` (`:777`) and `xtask/src/engine_cycle.rs`
  (`:430`), each of which drops its listener and then asserts that the port refuses. The chunk moves those two to
  a claimed number as well, so that no test reads a port it has released. The other 23 sites keep the listener
  they bound and release nothing; they are left as they are.
- **"four in-process arms"** — verified at P3: four `#[tokio::test]` arms call `boot`
  (`integration_engine_boot.rs:279`, `:319`, `:360`, `:401`), which calls `pick_port` twice; the two re-exec arms
  take no port.
- **"the way that failed one of seven hand runs … (measured at
  2026-10-10-engine-harness-proofs-completed)"** — verified at P3 against that chunk's record (`report.md:141-143`,
  `:267-269`: one of seven hand runs failed with bind-and-release, 14 of 14 green after the claims, "a count, not
  an exclusion"). Not re-run.

## What the directive adds (inputs#I1, named by inputs#I2 — each closed at P3, or at P4 where the bullet says so)
- **The set ships with its first consumer** (inputs#I1 item 1). Verified at P3 as reachable: research lists how
  spans, metric points and log records are built in tests today (28 files; 25 span, 11 span-event, 10 number-point
  and 14 log-record literals; where the time stamp, service, version and attributes are fixed). The plan names
  which existing tests move onto the set in this chunk, and which later entries are its next consumers. A factory
  nothing calls is not built here. One setter has no caller today: no test sets a sender's version and the
  product stores none, so whether the version setter is built now is the operator's at P4.
- **It can show neither named capability whole** (inputs#I1 item 2). Verified at P3: the retention sweep compares
  a record's stamp with the wall clock (`crates/buffer/src/retention.rs:178-179`), so no check reasons by event
  time yet (P-089); a stored span has seven columns and none for name, kind, parent or attributes (`:222`)
  (P-094). The expected reading is "advanced, not claimed" with a dated note; P5 says what of each acceptance
  this chunk shows and claims nothing that was not shown.
- **One shared port helper, if it can serve all three** (inputs#I1 item 3). Verified at P3 as reachable: the two
  twins differ only in how they find the repository root, and no test pins their text. What one helper costs
  depends on where the shared set lives (a new dev-only workspace member, or source files included by path);
  that is the operator's at P4, and the plan states the cost of the answer taken.
- **Test data is not product** (inputs#I1 item 4). Verified at P3: every OTLP type the factories build is already
  public in `ingest` and is built from tests today (181 reference sites in `pulse-app/tests`, 48 in
  `crates/buffer/src`); no product type needs a constructor or a field. No product source changes for the
  factories' sake; two of the files that move hold their tests in a co-located test module
  (`crates/buffer/src/consumer.rs`, `crates/buffer/src/appender.rs`), where only lines inside that module change.
- `[premise-corrected: research's census finds none of the 28 files in a crate that leaves]` **What the old form
  leaves behind** (inputs#I1 item 5). Every hand-built span, point or record the chunk replaces is removed in the
  same chunk. The ones it does not replace are counted in the report, each count beside the route entry that will
  move or remove them. The directive expected many of them to sit in crates that leave in Epochs 2 and 3; the
  census reads otherwise: all 28 files sit in `ingest` (14), `buffer` (2), `pulse-app/tests` (11) and `xtask`
  (1), none of which leaves. The unmoved sites are therefore counted by the route entry that owns the surface
  they test, not by a leaving crate (research, "Owners of the sites this chunk does not move").
- **Halts** (inputs#I1 item 6). A new listening socket, a new place on disk, a credential, a command word of the
  engine, or anything reachable from another host stops for the operator's word. Test legs that bind ports use
  claimed ports, never 4317 or 4318. The plan lists the merge-base probe directly before its push entry.
- **At P5** (inputs#I1 item 7, inputs#I2): say what `planlint` check 4 listed, print the plan card and stop for
  the operator's `yes`.

## What the operator answered at P4 (inputs#I3)
- **The set lives in a new dev-only workspace member, `test-support`.** The operator's word for the dependency
  rule: "a dev-only edge to a test-support member is allowed, and only that. Pin it: a committed test reddens if
  any member names test-support outside its dev-dependencies, or if test-support depends on anything but the types
  it builds. The amended architecture sentence states the carve-out in those words." The chunk builds that pin,
  and the architecture sentence is an expected amendment of the wrap.
- **The version setter is not built here.** "A setter nothing calls is a foundation without a consumer. The entry
  word version is recorded at the wrap as carried to Span keeps its identity, which builds the setter together
  with the stored column that reads it." Of the entry's four settable things this chunk delivers three: event
  time, service and attributes.

## CI read at Setup (step 5a)
- `50216d7e` (the last wrap's commit, the only commit from the last flip through HEAD): **verdict not yet
  available** — `ci#38084492828` in progress (7 of 7 checks listed, 6 running, oldest 147 s at the read);
  `secret-scan#38084492829` completed, success. Not folded as green. P3 read it twice more: still in progress (two
  checks running at the last read, the coverage job the oldest at 907 s). It stays unresolved here; P5 reads it
  once more before the card.

## Capabilities
- **P-089 (The watcher counts by event time)** and **P-094 (A span keeps its meaning)** are named by the entry.
  Six later entries name them too: `Checks reason by event time`, `Future-stamped records bounded` and `Recorded
  stream replays to the same result` for P-089; `Span keeps its identity`, `Span and resource attributes kept`
  and `Who calls whom` for P-094. This chunk makes neither fully verifiable (verified at P3, see "It can show
  neither named capability whole"): it supplies the data those entries' checks will be driven with. Expected at
  P5: no claim, one dated note per id.

## Boundaries
- No product source changes for the factories (see "Test data is not product").
- No new listening socket, place on disk, credential, engine command word or anything reachable from another host.
- No migration of hand-built test data in crates that leave in Epochs 2 and 3; those sites are counted.
- No check that reasons by event time and no stored span field: those are later entries'.
- Ports 4317 and 4318 are not bound by any leg of this chunk.
