# Codebase Research — 2026-09-30-p-027-discovery-bound

## Scope
- **Depth:** deep (mature codebase; the fix crosses ingest → buffer tap → triage registry → pulse-app boot wiring → webview poll) · **Reads:** 19 · **Globs/Greps:** 24 · **Graph queries:** 3 (rust ×2, ts ×1; trace `.andromeda/runs/2026-09-30T08-05-31Z-phase/tree-query-2026-09-30-p-027-discovery-bound.json`)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full (55 587 B, 149 lines; body + all 21 Session Additions). Applied: §Scenario legs (NOT gates) family contract; 2026-08-23 invoke-the-injector-by-path, one exported data dir per leg, glob the `agent-latest.jsonl*` family; 2026-08-28 the run window must outlast the threshold the verdict reads; 2026-08-28 a leg proves its own preconditions (INCONCLUSIVE, never PASS); 2026-08-29 an absence verdict names only what was observed. `.claude/rules/testing.md` is path-scoped to Rust source and loads when /implement touches it.
- **Platform issues consulted:** none. No runner-only bullet and no CI-reading entry outside the operator leg. The Setup CI read of `71f3369` was `in progress`, not red, so nothing was folded.

## Files inspected
- `crates/triage/src/lifecycle/registry.rs` (30-380, 409-430, 468-520) — `ServiceRegistry` trait (:74), `InMemoryServiceRegistry` (:155), `list_all` (:189), `tick_all` (:313), `next_natural_state` (:409). An entry is created ONLY by `tick_all`'s `or_insert` (:327-335), `set_state_on_restart` (:238-251), `set_manual_override` (:205-217) and `set_state_on_corpus_restore` (:282-293). `tick_all` stamps `last_seen_unix_nano = now_unix_nano` at TICK time (:333-341), never the span's arrival.
- `crates/triage/src/lifecycle/mod.rs` (40-235) — `DEFAULT_LIFECYCLE_HEARTBEAT_INTERVAL` = 15 s (:52); `start_lifecycle_heartbeat` (:87) skips the immediate first tick (:96), so the first `tick_all` runs at +15 s from spawn; `emit_tick_observability` (:155) buckets the tick's events by (from, to) into `triage.lifecycle.transition {from_state, to_state, count}` (:201-217).
- `crates/triage/src/lifecycle/state_machine.rs` (1-60) — the `Unknown` variant "exists for services seen via prior corpus but not yet observed in the current session". A service observed this session is therefore not Unknown.
- `crates/triage/src/baseline/mod.rs` (270-330, 374, 86) — `observe_span` (:279) admits a new service only under `ACTIVITY_FLOOR_SERVICE_CAP` (:86, enforced :292-298). `error_rate(&self, service_name) -> Option<f64>` (:374) is the only existing per-service lookup: `None` = untracked.
- `crates/buffer/src/consumer.rs` (95-125, 225-265) — `observe_spans_for_baseline` (:236) derives `now_nanos` once per batch and passes the SCRUBBED name (`extract_service_name(…, None)`, :245) to the `SpanObserver`. The `spans` append emits `duckdb.append` with `table_name = "spans"` (:114).
- `crates/ingest/src/observer.rs` (15-60) — the `SpanObserver` trait (:25), invoked once per decoded span.
- `pulse-app/src/baseline_observer.rs` (full), `pulse-app/src/restart_observer.rs` (full) — the adapter pattern (`Arc<…>` wrapper implementing `SpanObserver` at the binary boundary) and `CompositeSpanObserver` (:61), which fans out in `Vec` order.
- `pulse-app/src/main.rs` (555-600, 670-730, 1465-1495) — the composite is built at :572-581 (baseline, then restart). `lifecycle_registry` is built LATER at :677, and the composite is first consumed at :1177 (the buffer consumer spawn), so the composite can move below :677 without reordering the consumer. The heartbeat spawn is at :1483.
- `pulse-app/src/lib.rs` (1-39) — `pub mod` list. The adapters are `pub` modules so `pulse-app/tests/` can reach them.
- `pulse-app/tests/unit_span_observers.rs` (1-40) — home of the adapter pins (migrated there because `[lib] test = false`).
- `pulse-app/ui/src/hooks/use-service-constellation.ts` (full) — `POLL_INTERVAL_MS = 1000`, pull-only, plus a poll on focus. Consumers (ts graph): `CompactWidget.tsx:39`, `TracesRoute.tsx:24`, `ConnectionStatusLine.tsx:39`.
- `pulse-app/ui/src/widget/ConstellationCanvas.tsx` (70-130) — the P-027 mark (:90-116): `elapsed = Date.now() − item.last_seen_unix_nano / 1e6`, taking the slowest of the fresh batch. The P-025 effect (:118+) calls `hueShiftSamples`.
- `pulse-app/ui/src/widget/constellation-types.ts` (27-31, 67-78, 165-187, 244-276) — `isServiceLive` (60 s recency on `last_seen`); `visibleDots` filters out only `archived` and non-live entries; `unknown` brightness 0.5 and `bootstrapping` 0.85, both visible; `hueShiftSamples` skips a first-seen dot with no tier and samples tier changes anchored on `tier_effective_at_unix_nano`.
- `xtask/src/hue_shift.rs` (full) — the scenario-leg template: `judge_hue_shift` (pure, unit-pinned), anchor = `record ts − duration_ms`, `ANCHOR_TOLERANCE_MS` = 1000, INCONCLUSIVE on an unmet precondition, and helpers reused from `external_resolve` (`locate_app_binary`, `build_injector`, `tempdir`, `spawn_app`, `wait_for_receiver`, `shutdown`, `wait_ports_released`, `log_family`, `LegOutcome`).
- `xtask/src/main.rs` (88-100, 264) — verb registration shape (`#[command(name = "smoke:hue-shift", about = …)]` + dispatch arm).
- `crates/ingest/examples/inject_demo.rs` (95-130, 320-345) — five services, EVERY batch carries all five (:333), so the first `spans` append of a fresh-boot run is every service's first sighting.
- `pulse-app/src/observability.rs` (242-248, 736-740, 1653-1690) — exact leaves `duckdb.append {rows_appended, duration_ms, table_name}`, `triage.lifecycle.tick` (8 fields), `triage.lifecycle.transition {from_state, to_state, count}`, `services.list_with_states.request {item_count, traceparent}`.
- `docs/v0_2_0/capability-verification-matrix.json` (P-027 entry) — scenarios anchor `state_machine.rs` (contains "P-027"), `e2e_service_lifecycle.rs` ("lifecycle"), `ConstellationCanvas.test.tsx` ("dot"). Its notes state "span-arrival-to-dot latency is now emitted at `metric.constellation.discovery_ms`".
- `andromeda-pulse-0.3.0/chunks/2026-09-29-p-025-hue-shift-observable-made-gradable/evidence/green-leg.md` — the measured timeline (poll `item_count=0` every ~1 s until the first `triage.lifecycle.tick` at +9.4 s; discovery record 520 ms after the tick).

## Graph impact (rust + ts planes, built; trace rows are authoritative)
- **`tick_all` / `list_all` / `set_state_on_restart`** (rust `calls`, 23 rows) — production callers: `start_lifecycle_heartbeat` (`crates/triage/src/lifecycle/mod.rs:105`, `:118`), `reevaluate_now` (`mod.rs:139`), `run_lifecycle_persist_cycle` (`persistence.rs:105`), `list_with_states` (`pulse-app/src/services_router.rs:96`); the rest are co-located registry tests. A new trait method adds no caller to these.
- **`ServiceRegistry`** (rust `refs`, 41 rows) — ONE implementer (`InMemoryServiceRegistry`, `registry.rs:182`, confirmed by `grep -rn 'impl ServiceRegistry for' crates pulse-app` → 1 hit). All test uses (`unit_services_router.rs`, `e2e_service_lifecycle.rs`, `unit_config_watcher.rs`, two diagnostics tests) hold `Arc<dyn ServiceRegistry>` and construct `InMemoryServiceRegistry`, so a required trait method breaks no test double.
- **`CompositeSpanObserver` / `BaselineObserverAdapter`** (rust `refs`) — production use only at `pulse-app/src/main.rs:573` / `:578`; pins in `pulse-app/tests/unit_span_observers.rs`.
- **`recordConstellationDiscoveryLatency` / `useServiceConstellation` / `hueShiftSamples`** (ts `refs`, 34 rows) — the mark and the hue sampler have a single production site each (`ConstellationCanvas.tsx:113`, `:125`); the poll hook has three consumers. The chunk changes no webview code.
- Crate edges: the fix adds a `triage` trait method and a pulse-app adapter. `pulse-app` already depends on `ingest` + `triage`, so no new edge (arch §Module dependency direction).

## Patterns detected
- **SpanObserver adapter at the binary boundary** (`pulse-app/src/baseline_observer.rs:30`, `restart_observer.rs:44`): an `Arc`-wrapping struct implementing `ingest::observer::SpanObserver` and delegating to a `triage` API. Composed by `CompositeSpanObserver` in `Vec` order (`restart_observer.rs:80-98`).
- **Registry mutation returns the lifecycle event, and the caller broadcasts it** (`set_state_on_restart` → `broadcast.sender().send(event)`, `mod.rs:121-124`; the corpus restore at `main.rs:687-692`).
- **Tick-folded aggregates, never per-span emission** (`emit_tick_observability`, `mod.rs:155-217`; obs §5 / §11).
- **Scenario leg = pure judge + thin runner** (`xtask/src/hue_shift.rs:113` `judge_hue_shift` unit-pinned over synthetic `json!` lines; the runner at :280 prints `smoke:{leg}: …` lines and exits 0/1/2).

## Conventions to follow
- **pulse-app pins live in `pulse-app/tests/`** — `[lib] test = false`, and the lib-src ratchet reds any `#[test]` outside `main.rs` (`unit_span_observers.rs:1-6`).
- **triage tests are co-located** with the clock injected as a parameter (`tick_all(now_unix_nano, …)`, `registry.rs:136`), not `tokio::time::pause()`.
- **Service identity arrives scrubbed** — consume the name the tap passes (`consumer.rs:245`), never re-extract.
- **Empty service names are dropped** (`registry.rs:230`, `:278`, `:326`).
- **Leg output lines** take the `smoke:{leg}: …` prefix and end in a `PASS` / `FAIL — …` / `INCONCLUSIVE — …` line (atom from `xtask/src/hue_shift.rs:380-395`).

## Mechanism re-derivation (the equality the design needs)
- **Defect, re-derived at HEAD (was `measured at` P-025).** For a service first seen at instant S on a fresh boot, the earliest instant it can be listed by `services.list_with_states` is the first `tick_all` after S, because no other path inserts on span arrival (`registry.rs:205-345`). The first tick fires 15 s after heartbeat spawn (`mod.rs:52`, `:96`). The dot then follows within one 1 s poll (`use-service-constellation.ts:22`). So first-seen-to-dot = (next tick − S) + ≤ 1 s, up to ~16 s, measured 9 986 ms. VERIFIED.
- **The equality the fix needs.** For every span whose scrubbed service name is non-empty and admitted by the baseline under `ACTIVITY_FLOOR_SERVICE_CAP`, the registry holds an entry for that name BEFORE `observe_span` returns. The observer runs in the buffer consumer on the batch's `now_nanos`, so the entry exists within milliseconds of S. `list_all` then lists it on the next poll, giving first-seen-to-dot ≤ ~1 s + paint.
- **The state it registers.** Per `state_machine.rs` the entry is `Bootstrapping`, the same state `tick_all` reaches on first observation (`next_natural_state(Unknown) = Bootstrapping`, `registry.rs:424`; pinned by `tick_all_first_observation_creates_bootstrapping_entry`). The registration yields the same `Unknown → Bootstrapping` / `Activity` event the tick would have emitted, with `first_seen = last_seen = last_transition = S`. The next tick finds the entry at `Bootstrapping`, and `from == to` under `Learning`, so no duplicate event is emitted (`registry.rs:350-352`).
- **The P-027 observable, after the fix.** The mark's anchor is `last_seen_unix_nano` (`ConstellationCanvas.tsx:106`). For a brand-new service's first appearance it equals S, until the first tick refreshes it (`registry.rs:338-340`, only when `current_quiet_duration_seconds == 0`). So the existing mark then measures first-seen-to-dot for the case this chunk grades, with no field and no bindings change. At HEAD the same anchor is the tick stamp, which is why P-025's run recorded a ~0.5 s discovery sample against a ~10 s true interval.
- **P-025 unaffected.** `hueShiftSamples` samples only tier changes anchored on `tier_effective_at_unix_nano` and skips a first-seen dot with no tier (`constellation-types.ts:258-269`). An earlier dot turns the fresh-boot rise from a first-appearance-with-tier sample into a witnessed none → tier change with the same anchor. The rise value shortens; the anchor, the emission rule and `smoke:hue-shift`'s ANCHOR verdict are untouched.
- **The leg's independent anchor.** On a fresh data dir with one injector, the first `duckdb.append {table_name: "spans"}` record is every service's first-sighting instant (the injector's first batch carries all five, `inject_demo.rs:333`). The first `metric.constellation.discovery_ms` record's timestamp is the paint instant. Both exist at HEAD, so the leg reads RED at the chunk base by the interval (≈ 10 s) and by the mark's anchor error (≈ 9 s). The webview's `services.list_with_states.request` record preceding that first append is the precondition that the webview was already polling.

## New files to create
- `pulse-app/src/discovery_observer.rs` — the first-sighting `SpanObserver` adapter over the registry, the baseline and the lifecycle broadcast
- `xtask/src/discovery.rs` — the `smoke:discovery` scenario leg: a pure judge plus the runner

## Files to modify
- `crates/triage/src/lifecycle/registry.rs` — the `ServiceRegistry` first-sighting method, its `InMemoryServiceRegistry` impl, and co-located tests
- `crates/triage/src/lifecycle/mod.rs` — fold first-sighting transitions into the tick's `triage.lifecycle.transition` aggregate, and re-export
- `crates/triage/src/baseline/mod.rs` — a per-service tracked query for the cap check, if `error_rate` is not reused
- `crates/triage/src/contract.rs` — re-export any new public item
- `pulse-app/src/lib.rs` — `pub mod discovery_observer`
- `pulse-app/src/main.rs` — compose the adapter third in the `CompositeSpanObserver`, built after `lifecycle_registry`
- `pulse-app/tests/unit_span_observers.rs` — adapter pins
- `xtask/src/main.rs` — register `smoke:discovery` and its dispatch arm
- `docs/v0_2_0/capability-verification-matrix.json` — the P-027 notes line on what the observable anchors

## Open questions
- none. The two forks research surfaced are decided by artifacts, and the leans are recorded for P4: the registration state is `Bootstrapping` per `state_machine.rs`'s `Unknown` definition; the discovery anchor stays `last_seen` because it equals the first-sighting instant for the graded case, and a `ServiceListItem` field would change the bindings shape, whose close test-plan §3 leaves "not yet specified".
