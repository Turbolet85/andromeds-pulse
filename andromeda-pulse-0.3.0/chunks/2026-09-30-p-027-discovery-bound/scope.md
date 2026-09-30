# Scope — 2026-09-30-p-027-discovery-bound

## Source

Working-route entry (`andromeda-pulse-0.3.0/working-route.md:138`, taken up 2026-09-30), verbatim:

> P-027 discovery bound — a new service's first constellation dot appears within the 5 s bound, not at the
> registry's first lifecycle tick · CONTEXT: measured at 2026-09-29-p-025-hue-shift-observable-made-gradable — first
> rise 9986 ms: a new service's first dot appeared only at the registry's first 15 s lifecycle tick, and the paint
> followed 4 ms after the dot existed; P-027 bounds discovery at ≤ 5 s and sits inside P-075, which Conductor return
> verifies, so this precedes that entry

Operator directive at take-up (2026-09-30, `/andromeda-phase` arguments), verbatim:

> P-027 discovery bound (:138). Measured in P-025: a new service first dot appears only at the registry first 15 s
> lifecycle tick (first rise 9986 ms), against P-027 discovery <= 5 s inside P-075. Fix the cause in the discovery
> path (not the budget), prove it with a live leg that measures first-seen-to-dot for a brand-new service on a fresh
> boot, and keep P-025 hue timing unaffected. capability-drift before the workspace nextest; diff-shaped probes name
> the chunk base (W182).

Freight folded (route.py pins, row 138): one `CONTEXT:` block (346 chars), quoted above in full. No `CARRY` / `PREREQ`
/ `BLOCKED-ON` / `WATCH` on this entry.

## What this chunk builds

1. **The discovery-path fix.** A service that sends its first span on a freshly booted app gets a constellation dot
   within P-027's ≤ 5 s bound. The fix goes in the discovery path. The 5 s budget is not relaxed, and the P-027
   observable is not re-anchored to make the number smaller.
2. **A live leg** that measures first-seen-to-dot for a brand-new service on a fresh boot of the real binary. It
   grades the interval from the first `duckdb.append {table_name: "spans"}` record (the first-sighting instant) to
   the first `metric.constellation.discovery_ms` record (the paint), and it checks that the discovery record's own
   anchor (`timestamp − duration_ms`) lands on that first-sighting instant. It has a RED reading at the chunk base,
   where it must show the ~10 s latency, and a GREEN reading after the fix, which must be within 5 s (amended at the
   P3 premise closure: at HEAD the discovery record anchors on the tick, so it cannot grade itself; see below).
3. **P-025 hue timing unaffected.** The hue observable's semantics stay as they are: the
   `tier_effective_at_unix_nano` anchor, the one-sample-per-witnessed-change rule, and the fall bounded by the 1 s
   poll. `cargo xtask smoke:hue-shift` still PASSes after the fix.

## Boundaries (non-goals)

- The 5 s P-027 budget and the 2 s P-025 budget are not changed.
- No Conductor-side work. P-075 (Conductor e2e + the four delegated timing caps) stays with the "Conductor return"
  entry; this chunk makes the Pulse side meet the bound Conductor will assert.
- The perf-budget gate, span-level redaction and real-model incident surfacing are separate route entries, not
  folded here.

## Surfaces and contracts it touches (P3 premise closure, 2026-09-30)

- The mechanism, as measured at P-025 (`evidence/green-leg.md` of
  2026-09-29-p-025-hue-shift-observable-made-gradable, marker text verbatim): *"The service registry lists a service
  only after its first lifecycle heartbeat (`DEFAULT_LIFECYCLE_HEARTBEAT_INTERVAL` = 15 s). So on a freshly booted
  app the dot, and therefore its hue, cannot exist until that tick. Paint followed the first non-empty poll by
  4 ms."* VERIFIED at HEAD. Registry entries are created only by `tick_all`, `set_state_on_restart`,
  `set_manual_override` and `set_state_on_corpus_restore`, never on span arrival
  (`crates/triage/src/lifecycle/registry.rs:205-345`). The heartbeat skips its immediate first tick
  (`lifecycle/mod.rs:96`), and the 1 s webview poll (`use-service-constellation.ts:22`) is the only other step
  between span arrival and dot.
- Sites (corrected by research): the fix site is the span-observer composition (`pulse-app/src/main.rs:572-581`,
  built after `lifecycle_registry` at `:677`) plus a first-sighting method on the `ServiceRegistry` trait. The
  heartbeat, its 15 s cadence and the `services.list_with_states` resolver are NOT changed. `ConstellationCanvas.tsx`
  is NOT changed.
- The fix's shape (decided at P3): event-driven registration at first sighting, through a third `SpanObserver`
  adapter at the pulse-app boundary. It consumes the scrubbed name the tap already passes
  (`crates/buffer/src/consumer.rs:245`), registers only services the baseline admitted under
  `ACTIVITY_FLOOR_SERVICE_CAP`, and registers them as `Bootstrapping`, the state `tick_all` reaches on first
  observation. `state_machine.rs` defines `Unknown` as "seen via prior corpus but not yet observed in the current
  session". Tick-shortening is rejected: it would change a Settings-sourced, hot-reload-classified cadence contract for
  every transition.
- [premise-corrected: `registry.rs:333-341` stamps `last_seen_unix_nano` with the TICK instant, not span arrival] The
  P-027 observable's anchor at HEAD is the tick stamp, so the discovery record cannot see the gap it bounds (P-025's
  run recorded ~0.5 s against a ~10 s true interval). After the fix, a brand-new service's first appearance has
  `last_seen` = its first-sighting instant, so the existing mark measures first-seen-to-dot for the case this chunk
  grades. No `ServiceListItem` field, no bindings change, no allowlist change. The live leg grades against an
  independent backend anchor, the first `duckdb.append {table_name: "spans"}` record of a fresh-boot run, which
  exists at the chunk base. Residual (recorded, not fixed): a service that re-enters liveness later in a session is
  "discovered" again by the mark, anchored on a tick-refreshed `last_seen`. That is not a first discovery and is not
  graded here.
- P-025 interaction, VERIFIED (`constellation-types.ts:258-269`). `hueShiftSamples` skips a first-seen dot with no
  tier and samples tier changes anchored on `tier_effective_at_unix_nano`. With the dot present before the incident
  opens, the fresh-boot rise becomes a witnessed none → tier change with the same anchor, and its value shortens.
  The hue observable's anchor, its emission rule and the smoke:hue-shift ANCHOR verdict are unchanged.

## Gate directives (operator, 2026-09-30)

- `cargo xtask capability-drift` runs BEFORE the default-features workspace nextest (test-plan §3 gate order, the
  playbook's superseding rule from 2026-09-30-dual-license).
- Diff-shaped probes name the chunk base explicitly, `71f3369976b1c3acb54d5012256e94d892e9b57b` (HEAD at take-up), per
  W182, never a relative ref.

## CI verdict read at Setup (5a)

- `71f3369` (2026-09-30-dual-license wrap commit, the only sha since the last flip): **in progress**, so no verdict
  was available at read time. `ci#36687259840` (pull_request) was running 12 of 13 checks; the oldest was
  `lint / test (windows-latest)` at 179 s. `secret-scan#36687259806` completed, success. Not a red and not a green. It
  is recorded unresolved and re-read at /implement or wrap.

## Verification matrix

- `P-027` is not an id in this version's matrix (`matrix.py show --id P-027` → no such capability id). It is a
  v0.2.0 id (`docs/v0_2_0/capability-verification-matrix.json`), delegated into this version through P-075.
- [inferred] P-075 (dynamic-external, the only unclaimed entry) stays pooled. Its acceptance needs a Conductor
  assertion round, which no Pulse-side chunk performs. P5 confirms and records the non-claim.
