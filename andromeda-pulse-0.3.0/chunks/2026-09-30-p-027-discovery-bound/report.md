# Report — 2026-09-30-p-027-discovery-bound

**Chunk:** P-027 discovery bound — a new service's first dot within 5 s of first sighting, not at the 15 s registry tick
**Date:** 2026-09-30T11:35Z
**Commits:** `87fe658 chore(2026-09-30-p-027-discovery-bound): operator pre-CI commit` (the only commit since `last_wrap`
2026-09-30T07:59:50Z; `git log --format='%h %s' 71f3369..HEAD`)

## Changes (structured — detectors read this)
- **Files:** base `71f3369` (the parent of the pre-CI commit) → working tree; `gate.py scope`: clean — changed 10 ·
  listed 10 · recorded 0.
  - new: `pulse-app/src/discovery_observer.rs` · `xtask/src/discovery.rs`
  - modified: `crates/triage/src/lifecycle/registry.rs` · `crates/triage/src/lifecycle/mod.rs` ·
    `crates/triage/src/baseline/mod.rs` · `pulse-app/src/lib.rs` · `pulse-app/src/main.rs` ·
    `pulse-app/tests/unit_span_observers.rs` · `xtask/src/main.rs` · `docs/v0_2_0/capability-verification-matrix.json`
  - chunk folder: `plan.md` · `research.md` · `scope.md` · `report.md` · `evidence/{red-leg-at-base,green-leg,hue-shift-after,operator-pass}.md`
  - `crates/triage/src/contract.rs` (listed "re-export only if a public item is added") — NOT changed: no new public
    item; `ServiceRegistry` / `ServiceLifecycleBroadcast` / `BaselineState` were already on `triage::contract`.
- **Symbols / APIs:**
  - `triage::ServiceRegistry` gains two REQUIRED trait methods (`registry.rs`):
    - `register_first_sighting(&self, service: &str, now_unix_nano: i64) -> Option<ServiceLifecycleEvent>`: an empty
      name or an existing entry → `None` (an existing entry is not touched). Otherwise it inserts through DashMap's
      vacant-entry arm (race-safe against `tick_all`'s `or_insert`) an entry
      `{state: Bootstrapping, first_seen = last_seen = last_transition = now, manual_override: None}`, increments a
      registry-held `AtomicU64`, and returns `{from: Unknown, to: Bootstrapping, trigger: Activity, transitioned_at: now}`.
    - `take_first_sightings(&self) -> u64` swaps that counter to 0.
    - One implementer (`InMemoryServiceRegistry`, which gained the `first_sightings: AtomicU64` field; `new()` is now
      `Self::default()`; `from_entries` initialises the counter to 0). Basis: `grep -rn 'impl ServiceRegistry for'
      crates pulse-app` → 1 hit. Every other `ServiceRegistry` holder uses `Arc<dyn …>` over `InMemoryServiceRegistry`,
      so no test double broke (the workspace nextest built and passed).
  - `triage::lifecycle::emit_tick_observability` folds `registry.take_first_sightings()` into the `(unknown,
    bootstrapping)` bucket of the existing `triage.lifecycle.transition {from_state, to_state, count}` aggregate when > 0.
    Callers are unchanged: `start_lifecycle_heartbeat` (15 s tick) and `reevaluate_now` — whichever runs first drains
    the counter.
  - `triage::BaselineState::tracks_service(&self, service_name: &str) -> bool`
    (`self.services.contains_key(service_name)`, the map the `ACTIVITY_FLOOR_SERVICE_CAP` guard reads).
  - `pulse_app::discovery_observer::DiscoveryObserverAdapter {registry, baseline, broadcast}` implements
    `ingest::observer::SpanObserver`. Per span it returns early if the name is empty, the registry already lists the
    service (`current_state(name).is_some()`, one DashMap read) or `!baseline.tracks_service(name)`. Otherwise it calls
    `register_first_sighting(name, now_nanos)` and sends `Some(event)` on `ServiceLifecycleBroadcast::sender()`,
    ignoring a send error. It emits no tracing.
  - `pulse-app/src/main.rs`: the `CompositeSpanObserver` construction MOVED from before `lifecycle_registry` to after
    it, and now composes `vec![baseline_adapter, restart_adapter, discovery_adapter]`, discovery LAST. Its sole
    consumer is still the buffer consumer spawn. The heartbeat spawn and `DEFAULT_LIFECYCLE_HEARTBEAT_INTERVAL` (15 s)
    are untouched.
  - No TauRPC procedure, no `ServiceListItem` field, no bindings change: `git diff --quiet 71f3369 --
    pulse-app/ui/src/bindings/index.ts` exit 0.
  - No port, no env var, no IPC route added.
  - Residual, recorded rather than fixed (plan §Implementation notes; scope.md): the P-027 mark anchors on
    `ServiceListItem.last_seen_unix_nano`. For a brand-new service's first appearance that equals the first-sighting
    instant, until the first tick refreshes it. A service that goes non-live (> 60 s) and later returns is
    "discovered" again by the mark, anchored on a tick-refreshed `last_seen`. That is not a first discovery and is
    outside P-027's graded case.
- **Crates / modules:** changed `triage` (lifecycle registry + tick emitter; baseline query) · `pulse-app` (new pub
  module `discovery_observer`, boot composition) · `xtask` (new module `discovery`). No crate added or removed.
  `triage`'s manifest is unchanged (the W182 probe over `Cargo.lock Cargo.toml crates/triage/Cargo.toml
  pulse-app/Cargo.toml xtask/Cargo.toml pulse-app/ui` printed nothing), so it has no `ingest` / `pulse-app` edge.
- **Dependencies:** none (same probe; `cargo deny check bans licenses sources` green).
- **Schema / config:** none. No config key and no Settings field; the lifecycle heartbeat cadence and its
  hot-reload classification are unchanged. Scrub/redaction shapes are unchanged. The new adapter consumes the name
  `observe_spans_for_baseline` already passes, the SCRUBBED output of `extract_service_name(rs.resource, None)`
  (`crates/buffer/src/consumer.rs:245`).
  - `extract_service_name` production call sites are UNCHANGED at 3: `grep -rn 'extract_service_name(' crates
    pulse-app --include=*.rs` → `appender.rs:47` (spans column) · `appender.rs:493` (storm FingerprintObserver feed) ·
    `consumer.rs:245` (the baseline SpanObserver tap), plus `appender.rs:1208` in a test module.
  - What changed is the TAP's downstream fan-out: the one `SpanObserver` it drives is the `CompositeSpanObserver`,
    which now reaches THREE observers (baseline, restart, first-sighting) instead of two. The scrubbed tap name
    therefore now also becomes a `ServiceRegistry` DashMap key at first sighting. Before, `tick_all` inserted the same
    name from the baseline's own map, which is keyed by that same scrubbed name, only later.
- **Spec-master edits:** none (implement authors none).
- **Counts / qualifiers moved:**
  - Workspace nextest 2433 → 2447 (+14, exactly the 14 new tests; the targeted selector reported `14 tests run: 14
    passed, 2433 skipped`, and the workspace run `2447 tests run: 2447 passed`).
  - No master or leaf states that count: `grep -rnE '\b2433\b|\b2447\b' .andromeda/*.md CLAUDE.md .claude/docs
    .claude/rules` → 0 hits.
  - The number of scenario legs moved from three to four (`smoke:gap-resume` · `smoke:external-resolve` ·
    `smoke:hue-shift` → + `smoke:discovery`). Stated at `test-plan.md:318` ("the **third SCENARIO leg**"), in
    `architecture.md:242`'s xtask CLI surface list, and in the `.claude/rules/verification-harness.md` §Scenario legs
    list.
- **Dev-tool versions:** none — no host tool installed, upgraded or read changed.
- **Harness / gate surface:** new xtask verb `cargo xtask smoke:discovery` (`xtask/src/discovery.rs`, registered as
  `Cmd::SmokeDiscovery` in `xtask/src/main.rs`).
  - A dev-host-only SCENARIO leg, not CI-wired and not in the standard gate set.
  - Runner: boot the release binary on a fresh data dir; wait (≤ 60 s) for a `services.list_with_states.request`
    record; start `inject_demo --sustained --error-pct=0`; poll for the first `metric.constellation.discovery_ms`
    record at or after the first `duckdb.append {table_name: spans}`, within `OBSERVE_WINDOW` = 30 s (+3 s flush
    margin); stop; preserve the log family.
  - Pure judge `judge_discovery`: A = the first spans append, P = the first poll record, D = the first discovery
    record ≥ A. No A → INCONCLUSIVE. No P, or P > A → INCONCLUSIVE. No D within 30 s → FAIL. PASS iff `interval_ms`
    ≤ 5000 AND `anchor_error_ms` = |(D.ts − D.duration_ms) − A| ≤ 1000 AND 0 `app.panic.fatal` AND 0
    `level == "ERROR"`.
  - `observe_window_supports_verdict(s)` refuses a window below 15 + 1 + 5 = 21 s (INCONCLUSIVE).
  - Extra precondition: INCONCLUSIVE when the shared tempdir helper's `target/external-resolve/run-{pid}` dir
    already holds a log family (pid reuse).
  - Artifact: `target/discovery/{UTC stamp}/`. Exit 0 PASS · 1 FAIL · 2 INCONCLUSIVE.
  - Printed lines: `smoke:discovery: first-sighting→dot interval_ms=… anchor_error_ms=… discovered_count=…` ·
    `… within the 5000 ms bound: yes|no` · `… log family preserved at …` · a last `PASS` / `FAIL — …` /
    `INCONCLUSIVE — …`.
  - Reuses `external_resolve` helpers (`locate_app_binary` release-first, `binary_age`, `build_injector`, `tempdir`,
    `spawn_app`, `wait_for_receiver`, `shutdown`, `wait_ports_released`, `log_family`, `LegOutcome`). The shared
    `build_injector` still prints its `smoke:external-resolve:` prefix.
  - No change to agent-run scripts, status shapes or ci.yml.
- **Cross-project / external claims:**
  - CI on `87fe658` (the pre-CI commit carrying all the chunk's code): `ci#36706243490` pull_request
    completed/success, 13/13 checks (the operator reports 12/12 jobs, overseer-verified with `gh`), and
    `secret-scan#36706243474` success; read by `ci.py conclusion` (polled 47× / 1 432 s). This wrap's own commit adds
    only the chunk's records, route and ledger files on top of that tree.
  - The chunk base `71f3369`'s own CI read green at /implement Setup (`ci#36687259840` success, `gh run list`).
  - **GREEN-leg binary, for Conductor :63 evidence (operator request):** `target/release/pulse-app.exe` sha256
    `9e51d1d92e80fdc0b998fe5e1c65fbbd9c5eef4dd9e6b7c4883c5ccad1bf9ab4` (`sha256sum`, read at this wrap's Setup),
    mtime 2026-09-30 12:26:44 +0200.
    - Built by /implement gate entry 4 (`cargo build -p pulse-app --release`).
    - Its product sources are identical to those committed at `87fe658`: every product file it compiles
      (`registry.rs` 12:08:35, `discovery_observer.rs` 12:09:06, `main.rs` 12:09:28 and the rest) was last written
      before the build.
    - The only later source edit is the one-line lifetime elision in `xtask/src/discovery.rs`, which is not linked
      into `pulse-app`.
    - The rebuild at /implement's full-block re-run was a 0.82 s no-op.
    - This binary produced `evidence/green-leg.md` and `evidence/hue-shift-after.md`, and Conductor's :63 evidence
      used it.
- **Reverted / negative API facts:** none.
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:** none new.
  - The scope's P3 premise correction (the P-027 mark anchors on the tick stamp at HEAD) was recorded in
    `scope.md` at /phase and is now MEASURED: RED `discovery_ms` sample 435 ms against a true 15 219 ms interval
    (`evidence/red-leg-at-base.md`).
  - After the fix the same mark measures the first-sighting interval for a new service (GREEN sample 182.7 ms,
    interval 177 ms). Its disposition is the P-027 notes sentence already written to
    `docs/v0_2_0/capability-verification-matrix.json` and the expected amendments below.
- **Expected amendments (from plan):**
  - arch §Occupied Resources → xtask CLI surfaces — register `smoke:discovery` → carried (Harness / gate surface).
    Site: `grep -c 'smoke:hue-shift' architecture.md` = 1, at `:242`.
  - arch §Occupied Resources → Tauri IPC routes, delegated-timing entry — the P-027 anchor text → carried (Symbols /
    APIs: `last_seen_unix_nano` is now stamped at first sighting for a brand-new service).
    - Site: `grep -c 'discovery_ms' architecture.md` = 1, at `:177`. That entry states P-025's anchor in full but
      only `→ metric.constellation.discovery_ms (P-027)` for P-027, so the anchor text is ADDED there.
  - test-plan §3 scenario legs — `smoke:discovery` as the fourth → carried (Harness / gate surface; Counts). Site:
    `grep -c 'smoke:hue-shift' test-plan.md` = 1, at `:318` ("third SCENARIO leg").
  - test-plan §1 pending coverage triggers — `discovery-observer-wiring-coverage` → carried.
    - Fact: the `main.rs` composition ORDER (discovery after baseline) is pinned only by the shape test
      `discovery_adapter_composed_after_baseline_lists_service_on_first_span`, which builds its own composite, and
      live by the leg; nothing pins `main.rs` itself.
    - Site: `grep -cE 'Pending coverage' test-plan.md` = 1 section header, at `:111`.
  - obs-plan §8 delegated-timing leaves — the P-027 anchor sentence → carried (Symbols / APIs). Site: `grep -c
    'discovery_ms' obs-plan.md` = 2, of which `:543` is the §8 leaf entry (it states the fields, not the anchor).
  - security-plan §Security Anti-Patterns → Logging — the `extract_service_name` consumer enumeration → carried,
    with a CORRECTED fact (Schema / config).
    - `extract_service_name`'s three consumers are UNCHANGED. The first-sighting adapter consumes the baseline
      TAP's already-scrubbed output as a third observer behind the composite, so the tap's scrubbed name now also
      keys the lifecycle registry directly.
    - Site: `grep -c extract_service_name security-plan.md` = 2, at `:434` (the THREE-consumer sentence) and `:436`
      (the scrub-shape sentence; no count stated there).
  - `docs/v0_2_0/capability-verification-matrix.json` P-027 notes — NOT a wrap amendment. It was written at
    /implement plan step 8: one dated `ANCHOR 2026-09-30` sentence appended to the P-027 `notes`; ids, scenarios and
    delegation unchanged; `verify:capability-matrix` green.
- **Coverage of new surfaces:**
  - `DiscoveryObserverAdapter` (span-observer hot path, per span) → validation {empty-name drop + registry-listed
    early return + baseline cap check ✓} · instrumentation {no per-span record by design (obs-plan §11 hot-path);
    counted on the tick's `triage.lifecycle.transition` aggregate ✓} · PII {consumes the scrubbed tap name; no
    tracing carries a service name ✓} · tests {unit ×3 in `pulse-app/tests/unit_span_observers.rs`; e2e live via
    `smoke:discovery`} · a11y n/a · tokens n/a
  - `ServiceRegistry::register_first_sighting` / `take_first_sightings` → validation {empty name → None ✓} ·
    instrumentation {folded into the existing transition aggregate ✓; live: first tick `unknown → bootstrapping
    count 5` in the hue-shift run's log} · PII n/a (no new record) · tests {unit ×5 co-located:
    `first_sighting_registers_bootstrapping_entry_listed_before_any_tick` · `first_sighting_returns_the_activity_event_once`
    · `first_sighting_drops_empty_service` · `first_sighting_then_tick_emits_no_duplicate_event` ·
    `first_sighting_transitions_fold_into_tick_aggregate`} · a11y n/a · tokens n/a
  - `BaselineState::tracks_service` → validation n/a (read-only query) · instrumentation n/a · PII n/a · tests
    {exercised by the adapter pins} · a11y n/a · tokens n/a
  - `cargo xtask smoke:discovery` → validation {self-proving preconditions + window guard ✓} · instrumentation {reads
    the app's log} · PII n/a · tests {unit ×6 `discovery::tests::*`; live RED + GREEN} · a11y n/a · tokens n/a

## Deviations from intent
- **Added guard (not in the plan):** `smoke:discovery` reports INCONCLUSIVE when the shared
  `external_resolve::tempdir` dir (named by pid) already holds a log family. Justification: a reused pid would mix a
  prior run's records into a fresh-boot verdict; the leg must prove its own precondition (verification-harness
  2026-08-28).
- **Clippy red, fixed:** `clippy::needless_lifetimes` on `discovery_at_or_after`, one iteration; subset then full
  block re-run green.
- **Port-slot sequencing (operator directive):** ports 4317/4318 were shared with a parallel builder.
  - RED ran in a first operator-granted slot. The gate block ran with `--skip 5` (self-verify). `self-verify` and
    both legs ran in a second slot (`--only 5,6`, `--only 8`, legs by hand). The operator then held all activity
    for a quiet-desktop window.
  - RED at the base was taken from a release binary built BEFORE any product edit (sha256 prefix `8e382bb2`; tree
    differed from `71f3369` only in `xtask/`, which is not linked into `pulse-app`), instead of rebuilding at step 1
    after the leg alone. Same basis, different order.
- **Operator pass fired by this session on the operator's word** (normally the operator's): hygiene · `pre-push:linux`
  · the pre-CI commit `87fe658` · push · the CI read. Recorded in `evidence/operator-pass.md`.
- scope record: none — `gate.py scope` clean, 0 recorded (P1 read at this wrap: changed 10 · listed 10).

## Decisions & corrections
- **Claim corrected in-session.** I first reported the tick fold as "unit-only", reading the discovery leg's log
  alone: it ended ~8 s after boot, before the first tick. The hue-shift leg's log from the same slot shows it live:
  first sighting at 10:35:03.8, first tick at 10:35:18.6 carrying `unknown → bootstrapping count 5`, with no
  per-service tick event. Recorded as a `contract.narrow-basis-claim` friction.
- **Operator protocol for a shared port slot:** STOP and ask before any live leg that binds 4317/4318, and release
  promptly. The operator also asked for a hold with no windows while a parallel builder ran NVDA legs.
- **Sweep hazard:** `grep -rn 'extract_service_name('` finds the choke point's three production call sites
  unchanged, while the real change sits DOWNSTREAM of one of them (the tap's composite). Counting consumers of the
  scrubbed name by that function's callers misses a new observer.
- **Hygiene of the evolve tool:** a hand-typed `ts` a few seconds ahead of `date -u` is refused (exit 3). Re-read
  the clock and re-letter the ids.

## Outcome
- **Acceptance criteria, re-asserted against the diff:**
  - RED at the base / GREEN after, each on its own fresh data dir → MET. RED exit 1, `within the 5000 ms bound: no`,
    interval 15 219 ms, anchor error 14 784 ms. GREEN exit 0, interval 177 ms, anchor error 6 ms, 0 panic, 0 ERROR.
    A second fresh boot (hue-shift run) read 294 ms.
  - `smoke:hue-shift` exit 0 `PASS` → MET: rise 644 ms / anchor error 327 ms, fall 468 ms / anchor error 235 ms,
    beside the prior 9 986 / 510 ms.
  - Targeted selector exactly 14 new tests → MET (`14 tests run: 14 passed`).
  - Gate order → MET: capability-drift before the workspace nextest, the regen last, and the chunk-base
    `git diff --quiet` close exit 0.
  - `triage` manifest gains no `ingest` / `pulse-app` edge; the wiring is at the pulse-app boundary → MET (the W182
    probe printed nothing).
  - Dot sourced via `services.list_with_states` → `list_all`; no procedure or bridge field; bindings byte-identical
    to the base → MET.
  - The security criterion (scrubbed identity, baseline-admitted only, no tracing record carrying a service name) →
    MET. The adapter has no `tracing::` call.
  - No new target, no allowlist change; first sightings still surface as `unknown → bootstrapping` on the tick;
    `hue_update_ms` field set and anchor unchanged → MET (live-read, as above).
  - No `pulse-app/ui/**` change → MET (W182 probe).
  - Workspace nextest → MET, 2447/2447.
- **Gates** (/implement, final block run plus the slot runs):
  - `cargo fmt --check` green
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings` green (red once, `needless_lifetimes`,
    fixed)
  - the targeted `cargo nextest … -E 'test(/first_sighting/) | test(/discovery_adapter/) | test(/^discovery::/)'`
    green (`contains 14 tests run: 14 passed` held)
  - `cargo build -p pulse-app --release` green
  - `cargo xtask self-verify` green (`self-verify: PASS`, in the port slot)
  - both port-refusal probes green
  - `cargo xtask smoke:discovery`: leg live, driven by hand in the slot — PASS, exit 0 (artifact
    `target/discovery/2026-09-30T10-34-38Z` fresh)
  - `cargo xtask smoke:hue-shift`: leg live, driven by hand — PASS, exit 0 (artifact
    `target/hue-shift/2026-09-30T10-35-01Z`)
  - `cargo xtask verify:capability-matrix` green
  - `cargo deny check bans licenses sources` green
  - W182 manifests/ui probe green (no output)
  - W182 roster probe `recorded`: the 9 tracked chunk files + `.claude/session-handoff.md`, matching the
    touchpoints
  - `capability-widening-check` · `check:ingest-progress` · `check:staged-artifacts` · `capability-drift` all green
  - `cargo nextest run --workspace --profile ci` green, 2447/2447
  - the mcp-server bindings regen green
  - `git diff --quiet 71f3369… -- pulse-app/ui/src/bindings/index.ts` green
- **Operator entries** (fired on the operator's word; results in `evidence/operator-pass.md`):
  - `gate.py hygiene`: `hygiene: clean`
  - `cargo xtask pre-push:linux`: exit 0, `"verdict": "green"`
  - the push: `71f3369..87fe658`
  - `ci.py conclusion --sha HEAD --wait 2400`: exit 0, `verdict: green`, 13/13
- **Smoke:** the boot path changed (`main.rs`). The `role = 'smoke'` self-verify ran as a P2 gate (PASS), plus both
  direct-binary legs.
- **Watches:** none.
- **Outcome basis:** the operator pass ran.
  - Gate verdicts rest on the final state: pre-CI commit `87fe658` (the only commit since the base), whose CI run
    `ci#36706243490` is green (`evidence/operator-pass.md`).
  - Implement's P4 report (this conversation) is the basis for the slot runs, which the CI does not run.
  - The operator directive between implement and this report: the operator directed this session to fire the
    operator entries itself.
- **Process hygiene:** the implement census, re-measured at the end of the operator pass:

  | process | started by | final state |
  |---|---|---|
  | `pulse-app` × 5 | the RED leg, self-verify, the GREEN leg, the hue-shift leg | terminated; `Get-Process` count 0; :4317/:4318 refusing |
  | `inject_demo` × 3 | the legs | terminated (count 0) |
  | WSL `Ubuntu` pre-push stages | operator entry 22 | exited 0; WSL VM process list unmeasured — the operator can see it |
