# Report — 2026-08-16-baseline-family-reachability

**Chunk:** Baseline-family reachability — the silence family becomes reachable for a fresh service inside a bounded, declared warm-up
**Date:** 2026-08-16T18:53:27Z
**Commits:** none yet for this chunk (this wrap commits it); prior commit on branch = `63316aa chore(route): operator-requested adaptation — 0-pending wrap`

## Changes (structured — detectors read this)

- **Files** (counts from `git diff --numstat`):
  - `crates/triage/src/cue/thresholds.rs` (+148/−0)
  - `crates/triage/src/baseline/mod.rs` (+122/−7)
  - `crates/triage/src/baseline/activity_floor.rs` (+62/−10)
  - `pulse-app/src/observability.rs` (+12/−0)
  - `pulse-app/src/main.rs` (+6/−1)
  - `pulse-app/tests/unit_observability_allowlist_bootstrap_window.rs` (NEW, 64 lines)
  - Source total: 5 modified + 1 new · +350/−18

- **Symbols / APIs:**
  - NEW env var `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` — production-consumed (changes the running
    app's cold-start gate), NOT harness-only. Bounded parse: non-zero integer strictly below
    `WINDOW_DURATION_SECONDS`; unset / empty / unparseable / zero / out-of-range ⇒ 3600s default.
  - NEW `pub const ENV_BASELINE_BOOTSTRAP_SECONDS` · `pub const TARGET_BOOTSTRAP_WINDOW_OVERRIDE` ·
    `pub fn resolve_bootstrap_window_seconds() -> u64` · `pub fn Thresholds::from_env() -> Self`
    (`crates/triage/src/cue/thresholds.rs`).
  - NEW tracing target `triage.baseline.bootstrap_window.override` (WARN, once per boot, only when the
    resolved bound ≠ default). Fields: `resolved_seconds` · `default_seconds` · `reason`
    (`env_override` | `env_rejected_out_of_range` | `env_rejected_unparseable`).
  - NEW `pub fn BaselineState::set_bootstrap_window_seconds(&mut self, u64)` +
    `pub fn BaselineState::bootstrap_window_seconds(&self) -> u64`.
  - CHANGED signature `ActivityFloor::bootstrap_state(&self, now_nanos: i64)` →
    `(&self, now_nanos: i64, window_seconds: u64)`.
  - CHANGED signature free fn `baseline::bootstrap_state(persistence, cap, now_nanos)` →
    `(persistence, cap, now_nanos, bootstrap_window_seconds)`.
  - UNCHANGED (deliberately): `iter_service_silence_snapshots(&self, now_nanos)` keeps its 1-arg form;
    `evaluate_service_went_silent` keeps `_thresholds`. See Deviations.
  - No new IPC method, endpoint, port, socket, or broadcast topic.

- **Crates / modules:** changed — `triage` (`baseline`, `cue`), `pulse-app` (`observability`, `main`). None
  added or removed.

- **Dependencies:** none added, none bumped. No `Cargo.toml` / `Cargo.lock` delta.

- **Schema / config:** one new config key (the env var above; env layer only — NOT surfaced through
  `Settings` / `config.toml`, so no TauRPC contract or hot-reload delta). `BaselineState` gains a
  `#[serde(skip, default = "default_bootstrap_window_seconds")] bootstrap_window_seconds: u64` — skipped, so
  the persisted bincode shape is UNCHANGED and no corpus migration is required; the explicit non-zero default
  exists because a deserialized `0` would mark every service `Ready` at once.

- **Spec-master edits:** none. `/andromeda-implement` authored no spec change; the four expected amendments
  are queued below for this wrap's fan-out.

- **Counts / qualifiers moved:** workspace test count 1775 (+1 skip) → **1790 (+1 skip)**, stated in
  `.claude/session-handoff.md`. `crates/triage` gains 12 tests; `pulse-app` gains 3 (the new guard file).

- **Dev-tool versions:** none.

- **Reverted / negative API facts:**
  - A 2-arg `iter_service_silence_snapshots(now_nanos, window)` was written and then REVERTED. It would have
    forced a signature change on `lifecycle/registry.rs::tick_all` — a **trait** method in a module the plan
    never scoped — plus its impl, mocks, both `lifecycle/mod.rs` call sites and its tests. Operator chose the
    BaselineState-held bound instead, so the file ended byte-identical to HEAD.
  - `crates/triage/src/cue/evaluate.rs` was edited and fully reverted; it carries ZERO delta.
  - The allowlist guard was first written into `pulse-app/src/observability.rs`'s `mod tests` and relocated —
    see Spec claims disproved.

- **Spec claims disproved by measurement:**
  1. **The route entry's stated mechanism was falsified** (already corrected in `scope.md` at /phase P3, and
     re-confirmed by the shipped fix). The entry asked that "the same value must reach BOTH mirrored
     constants". Measured: `Thresholds.bootstrap_window_seconds` was **inert** — defaulted, validated
     (rejects 0), documented as mirroring `baseline::BOOTSTRAP_WINDOW_SECONDS` "so config-path / hot-reload
     stay consistent" — yet read by **no** evaluation path, while `evaluate_service_went_silent` took
     `_thresholds` underscore-prefixed. Making the two constants agree would have been a silent no-op. The
     shipped fix makes the field load-bearing instead (boot resolves it; boot hands it to `BaselineState`).
  2. **Family count**: the entry named "the silence and activity-floor families" (two). Measured: the 3600s
     gate has exactly ONE consumer — the silence family at `cue/evaluate.rs:164`. *activity-floor* is its
     substrate (`ActivityFloor` owns `first_observed_unix_nanos` → `BootstrapState` **and** the
     quiet-duration t-digest). *error-baseline-spike* is gated by `min_ewma_samples = 10`
     (`cue/evaluate.rs:38`) and never touches the hour.
  3. **`agent-run.sh status` cannot report on a running app.** It delegates to `cargo xtask harness:status`,
     which constructs an in-process `tauri::test::mock_builder` app — so it describes a MOCK, structurally.
     Measured this wrap: exit 0, well-formed envelope, `pid 38332`, `uptime_ms 0`, `status "initialized"`,
     `last_tick_at null`, with **no pulse-app running at all**. This is the root cause of the
     "green about the wrong process" open question carried since 2026-08-15. Its `boot` verb additionally
     runs `cargo run --release`.
  4. **130+ `#[test]` fns in `pulse-app/src/observability.rs` never execute** — `[lib] test = false`
     (the Windows WebView2 workaround) means src-level tests there compile, pass clippy, and never run.
     Measured: `grep -c "#\[test\]"` = 132; zero `allowlist_for_target_resolves*` names appear in the nextest
     output. This chunk's own guard was initially written there and would have satisfied its acceptance
     criterion while proving nothing.
  5. **The corpus `baseline_state` TABLE is dead schema** (measured at /phase P5 per operator rider). Zero
     readers, zero production writers: created by DDL (`corpus/schema.rs:27,36`), listed in the
     orphan-disposition table enumerations (`disposition.rs:25,37`), and written by exactly one `INSERT` that
     sits under `#[cfg(test)]` (`disposition.rs:355`; `#[cfg(test)]` opens at `:308`). No `SELECT` against it
     exists. Real baseline persistence round-trips through `pipeline_metrics` under
     `metric_name = "baseline_state"` / `layer = "l1b"` (`pulse-app/src/baseline_persistence.rs:47,78,95`).
     Its 0-row reading across all measured legs is therefore the CORRECT value, not a broken persistence path.
  6. **`inject_demo` invoked via `cargo run` inside a timeout measures the compiler.** Measured: a 10s budget
     was spent entirely compiling `rustls` + `h2` after `cargo clean`; the injector was killed (rc=124,
     child exit 143) having sent **zero** spans, with `rows_ingested = 0` / `span_events_seen = 0`.

- **Coverage of new surfaces:**
  - `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` (env input boundary) → validation **bounded-parse ✓**
    (non-zero, `< WINDOW_DURATION_SECONDS`, fallback-never-panic; `serde`/`TryFrom` not applicable — plain
    `u64::parse` with explicit range, per §Validation Library "no validation library") · instrumentation
    **✓** (once-per-boot WARN naming the resolved + default value and a bounded reason) · PII **n/a** (two
    integers + a static label; no user data) · tests **unit ✓** (7 resolver tests incl. reject-zero /
    reject-empty / reject-unparseable / reject-at-or-above-window / whitespace-trim / unset-identical-to-
    default) · a11y **n/a** (no UI) · tokens **n/a**.
  - `triage.baseline.bootstrap_window.override` (self-observation target) → validation **n/a** ·
    instrumentation **✓ (is the instrumentation)** · PII **redacted-by-construction ✓** — EXACT allowlist leaf
    enumerating **all three** fields the emit site emits, no bare `triage` / `triage.baseline` prefix key ·
    tests **unit ✓ ×3, in `pulse-app/tests/` where they actually run** (field-set completeness · PII-ban ·
    no-bare-prefix) · a11y **n/a** · tokens **n/a**.
  - `BaselineState::{set_,}bootstrap_window_seconds` (in-process API) → validation **n/a** (internal) ·
    instrumentation **n/a** · PII **n/a** · tests **unit ✓** (short-window-reaches-gate behaviour guard ·
    resume-path-applies-window · deserialized-default-is-non-zero) · a11y **n/a** · tokens **n/a**.

## Deviations from intent

1. **Threading design changed from plan step 3/4** — *justified, operator-approved at an /implement fork.*
   Research derived the modify-set from a graph query on the **constant**, so it never enumerated the callers
   of the **function** whose signature the plan directed changing. Two production callers were missing:
   `cue/emitter.rs` (which produces the very `services_ready` counter the acceptance asserts on) and
   `lifecycle/registry.rs::tick_all`, a **trait** method in an unscoped module whose `Bootstrapping → Active`
   transition reads the same gate. Surfaced rather than silently expanded. The chosen design holds the bound
   on `BaselineState`, so the silence evaluator, the emitter counters and the lifecycle registry share ONE
   bound and cannot disagree — with zero unscoped files touched and no trait change.
2. **Plan step 4's letter not met** — *justified.* `evaluate_service_went_silent` keeps `_thresholds`. Its
   INTENT is met: `Thresholds.bootstrap_window_seconds` is populated by the resolver and is what boot hands
   to `BaselineState`, so it is genuinely load-bearing rather than a second spelling of `3_600`. Remove it and
   the gate loses its bound.
3. **Clock injected by parameter, not `tokio::time::pause()`** (plan step 6 named the tokio form) —
   *justified.* `bootstrap_state` / `iter_service_silence_snapshots` take `now_nanos` explicitly, so there is
   no ambient clock to control; parameter injection is the stronger form and matches the existing tests in
   both files. The test-plan §11 ban is on *un-injected* real time, which is honoured.
4. **Smoke driven by the direct-binary variant, not `agent-run.sh boot`** — *justified.* The harness `boot`
   verb runs `cargo run --release` (a full release rebuild, prohibitive immediately after `cargo clean`) and
   its `status` verb reports a mock (Spec claim 3). test-plan §3 sanctions the direct-binary variant for
   exactly this case.
5. **Guard test relocated** from `pulse-app/src/observability.rs` to `pulse-app/tests/` — *justified,
   corrective.* See Spec claim 4; a guard that cannot fail is not a guard.

## Decisions & corrections

- **Operator fork (/phase P4) — mechanism:** env-resolved bound + fix the inert field, over sample-count
  readiness / full Settings config-surfacing / shortening the global default. Production default stays 3600
  and the app is byte-identical when the var is unset.
- **Operator fork (/phase P4) — verification depth:** paused-clock units **and** a live proof, over either
  alone. The live leg is what caught that the first run's zero was a dead feed rather than a broken gate.
- **Operator fork (/implement P1) — threading:** bound on `BaselineState` (see Deviation 1).
- **Operator decision (/implement P2) — disk:** `cargo clean` to clear a 100%-full `D:` (7.4M free of 300G;
  `target/debug` 209G, of which 172G stale `deps`). Freed 225.6 GiB. A prior `rm -rf target/debug/incremental`
  was DENIED and was not retried.
- **Correction to a standing memory's scope:** `cargo clean` is recorded as pre-authorized, but scoped to
  *overnight-autonomy* sessions; this was an interactive session, so it was escalated rather than assumed.
- **Reusable discipline confirmed twice this session:** verify each asserted test NAME appears as `PASS` — a
  suite total cannot see a dead test file; and assert the feed advanced (`rows_ingested > 0`) BEFORE reading
  any zero on a downstream counter.

## Outcome

**Acceptance criteria: met.** Gates run (the plan's `## Test Commands`, all green):
- `cargo fmt --check` ✓ (after one `cargo fmt` reflow — the 2026-08-14 hook-vs-gate lesson)
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` ✓
- `cargo nextest run --workspace --profile ci` ✓ — **1790 passed, 1 skipped** (baseline 1775 + 1)
- `cargo xtask capability-drift` ✓ clean (0 missing, 0 extra) — after the documented `bindings.ts`
  regen-via-`--features mcp-server`; re-verified clean AFTER the smoke ran the production binary
- `cargo xtask capability-widening-check` ✓ clean (0 violations / 3 inspected)
- `bash scripts/agent-run.sh status` — ran, exit 0, **output not usable as evidence** (Spec claim 3)

**Smoke (P3, boot-path changed): PASS**, direct-binary variant on a fresh `ANDROMEDA_PULSE_DATA_DIR`:
- feed precondition asserted FIRST: `rows_ingested` max **1080** (seeded over the real OTLP gRPC receiver via
  the prebuilt `inject_demo`, no in-process bypass)
- **`services_ready` 0 → 5** — 16 ticks at 0, then 88 ticks at 5, inside a ~17s run under a 5s window. Under
  the 3600s default this stays 0 for a full hour.
- `triage.baseline.bootstrap_window.override` emitted **once**, un-redacted (proves the leaf works in
  production, not only in the guard)
- 0 `ERROR`, 0 `app.panic.fatal`, both OTLP ports bound then released, **zero orphan processes**
- Bounded-subprocess discipline: SIGTERM did not stop the app within a 10s grace window in either run;
  SIGKILL was required both times.

**Gate deferrals:** none. This chunk has real Rust delta, so every changed-surface and workspace gate ran; no
source-delta-proportional deferral was invoked.

**`cargo audit` PREREQ (absorbed from the working entry):** the ratified interval is every 3rd wrap — it ran
at session 25 and the next point is **session 28**. This wrap is session 26, so:
**probe skipped per ratified interval (next: 28)**. Basis re-verified unchanged (upstream RustSec DB cannot
load: `parse error: duplicate advisory ID: RUSTSEC-2026-0244`, first-hand at session 25 and byte-identical on
a second project the same day — one upstream event). Named overlap `cargo deny check advisories` re-observed
this wrap.

**Expected amendments (for this wrap's fan-out):**
- `architecture.md` §Occupied Resources → Environment variables — register
  `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` (production-consumed, bounded parse, 3600s default).
- `security-plan.md` §Input Validation → CLI / env var row — the new bounded input boundary.
- `obs-plan.md` §8 — the new EXACT leaf `triage.baseline.bootstrap_window.override` with all three fields.
- `test-plan.md` §2 / §4 — name `triage` in the three library-crate enumerations it is absent from
  (pyramid Unit row · "What unit tests cover" · test-file-location), per the `corpus` precedent.
