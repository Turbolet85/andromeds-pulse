# Report — 2026-08-28-duplicate-span-replay-fails-loudly

**Chunk:** Duplicate-span replay fails loudly — a constraint-violating flush never wedges ingest, and the injector stops colliding with itself across restarts
**Date:** 2026-08-28
**Commits:** none since `last_wrap` (2026-08-28T20:50:00Z) — this wrap authors the chunk's first commit

## Changes (structured — detectors read this)

- **Files:**
  - `Cargo.lock` — duckdb + libduckdb-sys bumped (lockfile only)
  - `crates/buffer/src/appender.rs` — **test-only** (+45): one `#[cfg(test)]` pin; production fn byte-identical to HEAD
  - `crates/ingest/Cargo.toml` — explicit `[[example]]` declaration with `test = true`
  - `crates/ingest/examples/inject_demo.rs` — identity salt, `--replay` flag, first `#[cfg(test)] mod tests`
  - `deny.toml` — three stale skips pruned, one reasoned build-only skip added
  - `xtask/src/gap_resume.rs` — replay wiring, two-half gap-arm verdict, observe-window guard

- **Symbols / APIs:**
  - `crates/ingest/examples/inject_demo.rs` (all example-private, **no library surface**): `trace_id(seq)` → `trace_id(seq, salt)` and `span_id(seq)` → `span_id(seq, salt)` — **sole caller** is the batch loop in the same file, changed with them; new `fresh_run_salt() -> u64`, `const REPLAY_RUN_SALT: u64`, `Profile::run_salt()`, `Profile.replay_salt: Option<u64>`
  - `xtask/src/gap_resume.rs`: new `pub fn observe_window_supports_verdict(observe_minutes) -> Result<(), String>`, new `const STALL_ANNOUNCEMENT_SECONDS: u64 = 450`, new `StormEvidence.append_rejections: usize` field. `StormEvidence` is a `pub struct` with 7 in-crate literal construction sites (all in the same file's `mod tests`), **all updated**; `judge()` signature unchanged.
  - **CLI surface:** `inject_demo` gains `--replay` (arg-less default unchanged; unknown args still exit 2). Flag set 3 → 4.
  - **No TauRPC procedure, IPC method, endpoint, port, socket, or env var added or changed.** `capability-drift` clean.

- **Crates / modules:** none added, none removed. Modules changed: `buffer::appender` (tests only), `ingest` example target, `xtask::gap_resume`.

- **Dependencies:**
  - **Bumped:** `duckdb` + `libduckdb-sys` `1.10502.0 → 1.10505.0`. **`Cargo.toml` untouched** — the existing `version = "1.10500"` caret requirement already permitted it; this was a stale lockfile, not a pin change. `arrow` stays 58.2.0, matching its `arrow = "58"` pin.
  - **Transitively added:** `ureq` 3.4.0 (+ `ureq-proto`, `utf8-zero`) — `libduckdb-sys` **build-dependency only**.
  - **Transitively removed:** `rkyv`, `rkyv_derive`, `rend`, `rust_decimal`, `bitvec`/`radium`/`tap`/`wyz`, `seahash`, `windows-core`, `windows-result`, `windows-strings`. Net lockfile −241/+65.
  - **No shipped-binary dependency added.**

- **Schema / config:**
  - `deny.toml [bans] skip` — removed `windows-core` / `windows-result` / `windows-strings` (the bump dropped their second copies, so cargo-deny flagged them `unnecessary-skip`); added `{ crate = "ureq" }` with provenance, build-only rationale, and a stated closing condition. `multiple-versions = "deny"` untouched; the `tonic` canary untouched.
  - `crates/ingest/Cargo.toml` — `[[example]] name = "inject_demo" … test = true`.
  - **No DuckDB schema change. No PK altered.** `spans` stays `(trace_id, span_id)`; `span_events` stays `(trace_id, span_id, event_index)`. No surrogate key added.
  - No `config.toml` key, no migration, no violation-schema change.

- **Spec-master edits:** none applied by /implement (correctly — implement authors none). Expected amendments listed under Outcome.

- **Counts / qualifiers moved:**
  - Workspace nextest **2044 → 2056** (+1 skip unchanged) — stated in the handoff.
  - `inject_demo` CLI flag count **3 → 4** — stated in `architecture.md` §Stack GUI-verification-harness row.
  - `inject_demo` test count **0 → 8** — stated in `test-plan.md` §1 (`inject-demo-arg-parse-unit-coverage`: "the injector ships zero `#[test]`s and is live-exercised only").
  - duckdb version **1.10502 → 1.10505** — `architecture.md` §Stack + §Inherited Defaults state "DuckDB 1.5.x via `duckdb` crate 1.10500.x"; the root `Cargo.toml` comment says "`duckdb` crate 1.4.x" (already stale before this chunk).
  - `buffer` crate in-crate pin list — `test-plan.md` §4 enumerates it and asserts "No tracked in-crate `buffer` gap remains".
  - `smoke:gap-resume` arm count unchanged at **3**.

- **Dev-tool versions:** none — no external CLI installed or upgraded.

- **Reverted / negative API facts:** A repair to `append_record_batch_to_table` was **written as a probe and fully reverted**. Temporary `eprintln!` step markers inside the production fn (used to separate `flush()` from `Appender::drop`) and a throwaway plain-INSERT probe test were both removed; `git diff` confirms the production function is byte-identical to HEAD. **No appender code shipped** — the defect was fixed upstream by the dependency bump.

- **Spec claims disproved by measurement:** three.
  1. **"A duplicate-INSERT probe hangs on libduckdb-sys 1.10502."** Stated at `crates/buffer/src/schema.rs:320–323` and cited by `test-plan.md` §4 as the reason three PK contract tests assert via `information_schema` rather than behaviourally. **Measured FALSE:** a plain prepared duplicate INSERT returns `Constraint Error: Duplicate key … violates primary key constraint` in 0.05 s. The hang was specific to the Arrow `Appender::flush()` path. (First probe attempt was inconclusive — both inserts died on a `NOT NULL service_name` before reaching PK enforcement; only supplying every NOT NULL column exercised it.)
  2. **"The first replayed batch fails LOUDLY … and the NEXT one hangs."** Stated in `obs-plan.md` §10 defect 4, `.claude/rules/observability.md` §DuckDB connection isolation, and this chunk's own working-route entry / `scope.md` / `research.md`. **Measured FALSE at 1.10502:** a step-marker probe showed `flush:enter` on the FIRST violating batch with no matching return and no error ever produced — the first flush blocks, there is no loud first failure. (This is why the plan's selected fork, a reset hooked onto a returning flush error, was unimplementable and the run soft-exited before the operator widened scope.)
  3. **"A `[[example]]` target can host `#[cfg(test)] mod tests` and run under nextest."** Stated in `test-plan.md` §1 (`inject-demo-arg-parse-unit-coverage`). **True only with an explicit declaration** — Cargo defaults an auto-discovered example to `test = false`, so `cargo nextest run --workspace` collected **zero** of the 8 new tests until `[[example]] … test = true` was declared (workspace count moved +4, not +12).

- **Coverage of new surfaces:**
  - `inject_demo --replay` (dev-only CLI flag) → validation {n/a — bounded flag, unknown args exit 2} · instrumentation {n/a — dev tool, not the product} · PII {n/a — emits synthetic spans only} · tests {unit ✓ — 8 pins incl. parse arms, default-preservation, salt injectivity} · a11y {n/a} · tokens {n/a}
  - `observe_window_supports_verdict` (xtask harness guard) → validation {✓ — refuses a window below the threshold its verdict reads} · instrumentation {n/a — harness, prints its own verdict} · PII {n/a} · tests {unit ✓ — boundary pinned at 7m/8m/default} · a11y {n/a} · tokens {n/a}
  - `StormEvidence.append_rejections` (harness verdict field) → validation {n/a} · instrumentation {n/a — reads the app's existing `duckdb.append` record} · PII {✓ — counts records, reads no field value beyond the bounded `reject_reason` label} · tests {unit ✓ — counted-from-field pin + discriminating inconclusive pin} · a11y {n/a} · tokens {n/a}
  - **Store/log boundary failure mode CHANGED (detector-relevant):** a constraint-violating `Appender::flush()` now **returns `Err`** where it previously blocked unbounded. The `duckdb.append` ERROR + `reject_reason: "append_failed"` record is unchanged in shape and now actually fires per rejected batch. → PII {✓ — bounded category only; measured 120 ERROR records carrying `reject_reason: "append_failed"` and nothing else} · instrumentation {✓ — existing target, no new field, no new allowlist leaf} · tests {unit ✓ + e2e ✓}

## Deviations from intent

1. **Step 2 (the appender repair) was delivered by a dependency bump, not by code.** The plan selected the "recover + keep ingesting" fork implemented as a reset hooked onto a returning `flush()` error. Measurement showed `flush()` never returns on the first violation, so that fork had nothing to hook onto. *Justification:* operator-directed after an explicit SURFACE — option 2 ("widen scope to the dependency, try a version bump") then option 1 (re-plan) if it failed. The bump worked, so option 1 was not needed. Red→green on one unchanged test at 1.10502 vs 1.10505 is the attribution.
2. **`Cargo.lock` edited — outside the plan's modify-set.** *Justification:* the operator-authorized scope widening. Notably `Cargo.toml` needed no change at all.
3. **`crates/ingest/Cargo.toml` edited — outside the modify-set.** *Justification:* without it the 8 injector tests were collected zero times by the workspace gate — a dead-test shape that would have discharged nothing and silently misreported coverage. Making a listed file's tests actually run is the manifest counterpart of the listed change.
4. **`deny.toml` edited — outside the modify-set.** *Justification:* direct consequence of the authorized bump — it both stale-ed three skips and introduced one new build-only duplicate, failing `cargo deny check bans`. Keeping the supply-chain gate green is part of landing the bump; `multiple-versions = "deny"` was not relaxed.
5. **The M2 fallback branch was never built.** *Justification:* the plan gated it on the Step 1 probe and required surfacing before building. The bump removed the need entirely — `consumer.rs`, `contract.rs`, and `observability.rs` were correctly left untouched, as the plan's conditional touchpoints instructed.

## Decisions & corrections

- **Operator:** "ok lets try 2 then 1" — bump first as the cheapest decisive probe, re-plan only if it failed. It succeeded.
- **Measure-first held and paid.** The Step 1 probe was authored as the permanent pin, so its failure at HEAD *is* the recorded measurement (30 s timeout), and the same test passing at 0.06 s after the bump is the discriminator. No throwaway probe, no test written after the fix.
- **A probe that fails to reach its target is inconclusive, not a result.** The first plain-INSERT probe died on a NOT NULL column before touching PK enforcement and printed a confident "does not hang" line. Supplying every NOT NULL column was what turned it into evidence.
- **Salt identity, never `seq`.** `seq` drives `error_roll` and the latency jitter as well as the ids, so the nonce enters only `trace_id`/`span_id`. Batch count, span count, error count and durations are unchanged — which is what preserves the headful leg's storm budget.
- **A test count that moves by the wrong amount is a finding.** +4 instead of +12 is what exposed the dead example tests; every gate was green in both worlds.
- **`| tail` masked a `cargo deny` exit code again** — `exit=$?` read `tail`'s 0 while `bans FAILED`. The text, not the code, was the real signal.

## Outcome

**Acceptance MET.** `cargo xtask smoke:gap-resume` arm A exits 0 with **both** precondition halves satisfied:

```
PASS — gap arm: storm formed (15 cues / 5 tier1 triggers),
       120 of 2270 appends rejected on replayed identities,
       and the consumer kept draining across 54 buffer ticks
```

The rejection count is self-checking: run 1 emits 120 batches, run 2 emits 1080 under the same pinned salt, so exactly the first 120 replay run 1's keys — and exactly **120** appends were rejected. Run 2 then persisted 29,160 spans; before the fix ingest froze at 3,240 with the channel at 100 %.

- **Arm B** (`--sustained`) PASS — no storm as designed, consumer drained, 38 ticks.
- **Arm C** (`--reconnect-only`) PASS — 2380 appends, no storm, consumer drained, 42 ticks.
- **Log invariants (arm A):** 249,514 lines · **0 panics** · ERROR set is *exactly* the 120 by-design `duckdb.append` / `append_failed` records, no other target.

**Gates (the plan's `## Test Commands`, all run):** `cargo fmt --check` ✓ · `cargo clippy --workspace --all-targets --all-features -- -D warnings` ✓ · `cargo nextest run --workspace --profile ci` **2056 passed / 1 skipped** (+12 = exactly the tests added) ✓ · `cargo xtask capability-widening-check` clean (0 violations / 3 inspected) ✓ · `cargo xtask check:ingest-progress` PASS ✓ · `cargo build -p pulse-app --release` ✓ · the three `smoke:gap-resume` arms ✓ · `cargo deny check bans licenses sources` **ok** (real exit 0) ✓ · `cargo deny check advisories` designed-red, 8 distinct owned ids · `cargo xtask capability-drift` clean **LAST** after bindings regen ✓. **2 fix-loop iterations. No gate deferred.**

**Smoke:** the three gap-resume arms ARE the boot smoke — each spawns the real release binary against a fresh data dir. Release built first (`xtask` resolves `target/release/` before `target/debug/`); the leg's own age line confirmed `built 0m ago`.

**Audit PREREQ (pin #20, session 54 — a between-point):** basis re-verified first-hand — `cargo audit` still cannot load the RustSec DB (`parse error: duplicate advisory ID: RUSTSEC-2026-0244`, read directly under cargo-audit 0.22.2). Overlap re-enumerated first-hand: `cargo deny check advisories` exit 1 at **8 DISTINCT** ids — 0189 / 0190 / 0194 / 0195 / 0204 / 0222 / 0253 / 0258 — **set identical to the prior enumeration**; `bans licenses sources` exit 0. Recorded as `probe skipped per ratified interval (next: 55)` — the full-form probe was not owed at this point, though it was in fact run. No running "Nth consecutive" ordinal.

**Capabilities claimed: 0.** Version coverage unchanged at 21/22 verified, P-075 pooled (Conductor's).

**Expected amendments (wrap):**
- `obs-plan.md` §10 defect 4 — OPEN → **closed**, with the mechanism corrected (the first flush hung; the stated loud-first-failure never existed) and the fix attributed to the dependency bump.
- `.claude/rules/observability.md` §DuckDB connection isolation — mirrors that narrative.
- `test-plan.md` §3 — arm A red-by-design → **gate-grade**; the observe-window known gap **closed**; §4 buffer pin list extended; §1 `inject-demo-arg-parse-unit-coverage` **discharged** (and its "runs under nextest" claim qualified per disproof 3).
- `test-plan.md` §4 + `crates/buffer/src/schema.rs:320–323` — the duplicate-INSERT-hangs claim is false (disproof 1); the `information_schema` route may stay on other grounds, but not on that one.
- `architecture.md` §Stack GUI-verification-harness row — register `--replay`; duckdb pin 1.10500.x → 1.10505; and the "byte-identically" clause needs narrowing now that span identity is deliberately per-run (the historical verification was a COUNT equality, 3540 = 3540, not a byte comparison).
- `security-plan.md` §Dependency Security — session-54 between-point discharge; pointer stays at 55.
