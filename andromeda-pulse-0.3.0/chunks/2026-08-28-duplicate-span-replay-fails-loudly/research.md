# Codebase Research — 2026-08-28-duplicate-span-replay-fails-loudly

## Scope
- **Depth:** moderate · **Reads:** 11 files · **Globs/Greps:** 12 · **Graph queries:** 2 (rust plane, `db_state` warm)

## Files inspected
- `crates/buffer/src/appender.rs` (632–652 + grep) — `append_record_batch_to_table` is the single write fn: `conn.appender(table)` → `append_record_batch()` → `flush()`, each mapping to `Error::Append`. The `Appender` is created fresh per call and dropped on scope exit, including on the `?` early-return.
- `crates/buffer/src/consumer.rs` (30–300) — the whole production path. `run_consumer` `spawn_blocking`s `dispatch_batch` per batch and **continues the `while let` loop after an `Err`**, logging `target: "duckdb.append", reject_reason = describe_error(&e)`. `dispatch_batch` takes `conn.lock()` ONCE and holds it across every append in the batch.
- `crates/buffer/src/schema.rs` (38/54 + 316–400) — `spans` PK `(trace_id, span_id)`; `span_events` PK `(trace_id, span_id, event_index)`. **The decisive find is the comment at :320–323** (see Graph/Patterns below).
- `crates/ingest/examples/inject_demo.rs` (224–331) — `seq = b * 100_000 + si*1_000 + oi*100 + k` with `let mut b: u64 = 0` at :277; `trace_id(seq)` :224, `span_id(seq)` :231, `error_roll(seq)` :242, `jitter = (seq * 7) % eff_base` :302. Zero `#[test]`s in the file.
- `xtask/src/gap_resume.rs` — `DEFAULT_OBSERVE_MINUTES = 9` (540 s) with the doc comment "Run 2 must outlast the stall announcement threshold (30 non-draining ticks = 450s)"; `judge()` at :176; three `LegArm`s.
- `xtask/src/ingest_progress.rs` — `evaluate()` :38 keys on `TARGET_STALLED = "buffer.consumer.stalled"`; `Verdict::{Pass, Fail, Neutral}`.
- `xtask/src/main.rs` (73, 198, 210) — `observe_minutes: Option<u64>`, defaulted via `unwrap_or(defaults.observe_minutes)`; **no lower-bound check**.
- `xtask/src/webview_drive.rs` (190–217, 678–728) — `cmd.env("PULSE_INJECTOR", injector)` at :728 and nothing else; the arch claim that no arguments are threaded holds at HEAD.
- `pulse-app/src/observability.rs` (228–245, 3140–3170) — the allowlist and its guard (premise 5).
- `Cargo.toml` :104 / `Cargo.lock` — `duckdb = "1.10500"` resolving to **1.10502.0**.
- `.andromeda/test-plan.md` :353 — records the duplicate-INSERT hang as the reason the PK contract is asserted via `information_schema`.

## Graph impact
- **`append_record_batch_to_table`** — **7 rows** (the trace's `rows` field is the authority; a `tail`-clipped view of this result under-counted it as 6 and mis-stated the first line as :673): 4 `#[cfg(test)]` wrappers in `appender.rs` (:668 / :692 / :717 / :742), one test at :1998, a module-level `use` at `consumer.rs:11` (an import, not a call site), and **exactly one production caller**: `consumer::append_table_traced()` at `crates/buffer/src/consumer.rs:188`. The security extract's "only DuckDB write path in `crates/buffer`" claim holds at HEAD, with `append_table_traced` as the single production wrapper.
- **`append_table_traced`** — 4 callers, all `consumer::dispatch_batch()` (`consumer.rs:111, 125, 135, 148`) — i.e. `spans`, `span_events`, `metrics_points`, `log_records`. One caller of `dispatch_batch`: `consumer::run_consumer()` at :62.
- Blast radius is therefore entirely inside `crates/buffer`; **no new crate edge is implied** by an appender-level repair.

## Patterns detected
- **The hang is a documented property of THIS DuckDB build, not an unexplained one** (`crates/buffer/src/schema.rs:320–323`): *"Runtime PK check via duplicate-INSERT path was observed to hang in libduckdb-sys 1.10502 on this build — schema introspection is the contract assertion, not behavioral PK enforcement."* Recorded at `2026-08-22-log-records-identity` and re-applied for `metrics_points`. The project has been routing AROUND this hang in unit tests since then; the gap-resume wedge is the same upstream behaviour reached through the production appender path.
- **But the two shapes differ, and the difference is load-bearing** (`consumer.rs:76-93` + the predecessor's measurement): a plain duplicate INSERT hangs *immediately*; the appender path's **first** violating flush returns `Err` cleanly (the ERROR is logged, so `spawn_blocking` joined and the `Appender` dropped), and the **next** flush hangs. So the first failure leaves residual state on the *connection*, not on the dropped `Appender`. Do not conflate the two shapes.
- **Failure is already non-fatal to the loop** (`consumer.rs:76-93`): `run_consumer` logs and continues after `Ok(Err(e))`. So if `flush()` merely *returned* on the second violation, the consumer would keep draining with no further change — the repair's whole job is making the second flush return.
- **The mutex is held BECAUSE flush hangs, not the reverse** (`consumer.rs:99`): `dispatch_batch` holds `guard` across all appends; a hang inside `flush()` never releases it. The isolation topology is not implicated (consistent with the scope §1 boundary).
- **`seq` drives THREE things in the injector, not one** (`inject_demo.rs:301-304`): identity (`trace_id`/`span_id`), the error pattern (`error_roll(seq) < eff_err`), and duration (`jitter = (seq * 7) % eff_base`). Perturbing `seq` itself would move the storm's error distribution and timings; folding a per-process nonce into **only** `trace_id()`/`span_id()` changes identity while leaving batch count, span count, error count and durations untouched.
- **`b` restarting at 0 is the only cross-restart collision source** (`inject_demo.rs:277`) — `si`/`oi`/`k` are positional within a batch.

## Conventions to follow
- **Error surface**: `crates/buffer/src/contract.rs:6` holds the `Error` enum; `consumer.rs::describe_error` maps `Error::Append { .. } → "append_failed"` — the bounded category the leg reads. A new failure category means a new arm there.
- **Hang-safe test form**: `appender::tests::append_logs_batch_same_tick_same_severity_keeps_both_records` runs on a worker thread under a 30 s `recv_timeout` so a hang FAILS rather than wedges the suite (test-plan §4). Any pin that deliberately provokes a constraint violation must use this shape.
- **Never a bare duplicate-INSERT probe** — the documented hang above is exactly why `schema.rs` asserts PKs via `information_schema.key_column_usage`.
- **Post-append fold** (`consumer.rs:113-118, 131, ...`): `state.record_redactions(redactions)` fires at each table's OWN site AFTER its append, so a rejected batch contributes zero. Any restructuring of the failure path must preserve this.

## Files to modify
- `crates/buffer/src/appender.rs` — `append_record_batch_to_table` (:632): the appender-state / connection-state repair, plus its `#[cfg(test)]` pins.
- `crates/buffer/src/consumer.rs` — only if the chosen fork needs the call site (a terminal/degraded announcement, or a new bounded category in `describe_error`).
- `crates/buffer/src/contract.rs` — only if a new `Error` variant is added for the terminal fork.
- `crates/ingest/examples/inject_demo.rs` — the process-unique identity fold, and (if P4 takes it) a deliberate replay flag + its `#[cfg(test)] mod tests`.
- `xtask/src/gap_resume.rs` — the observe-window guard; `xtask/src/main.rs` if the guard is enforced at CLI parse rather than in `run_gap_resume`.
- `pulse-app/src/observability.rs` — **only if** the repair adds a field to `duckdb.append` (see premise 5).

## Scope premise closure

1. **VERIFIED (with a fork-dependent nuance)** — the appender object lives in `append_record_batch_to_table` (`appender.rs:632`) and the mutex is taken by `dispatch_batch` (`consumer.rs:99`), so appender/connection-state repair belongs in `appender.rs`. The "terminal and reported" half of the fork would additionally need `consumer.rs` (a bounded category / once-per-transition announcement) and possibly `contract.rs`.
2. **VERIFIED** — `dispatch_batch`'s `Batch::Spans` arm appends `spans` (`consumer.rs:111`) *and* `span_events` (`consumer.rs:125`) on the same guard, both `?`-propagated. Replayed spans replay their events, and `span_events` PK `(trace_id, span_id, event_index)` collides identically. The repair must hold for every table `append_table_traced` serves, not only `spans`.
3. **VERIFIED as a real gap** — arm A's storm precondition comes from the *silence family* during the gap, not from duplicates. Once the injector stops self-colliding, arm A would exit 0 **because the collision is gone, not because the hang is fixed**, and the appender repair would be unexercised by the acceptance. Keeping the repair exercised is a genuine P4 decision.
4. **[premise-corrected: the hang is a documented `libduckdb-sys` 1.10502 behaviour recorded at `crates/buffer/src/schema.rs:320–323`, not an unexplained one]** — what remains genuinely open is narrower: why the *appender* path's first violating flush returns cleanly while the next hangs, and whether the residual is an open transaction / table lock the failed flush leaves on the connection. Two candidate mechanisms for /implement to discriminate by probe, not by inference: **(M1)** connection-level residual state (a leftover transaction or lock) that the next flush waits on — repairable in our code by resetting that state after a failed flush, satisfying the "connection stays usable" fork; **(M2)** an upstream lock the connection cannot clear, in which case only prevention (never letting a violating batch reach `flush()`) or the "terminal and reported" fork is reachable.
5. **VERIFIED** — the allowlist key is `"duckdb"` (resolved from `duckdb.append` via `split('.').next()`), `pulse-app/src/observability.rs:233–244`, permitting exactly `rows_appended` / `duration_ms` / `table_name` / `reject_reason`, guarded by `allowlist_for_target_resolves_duckdb_append_to_arrow_appender_fields` (:3142). `reject_reason` survives redaction today, so **no new leaf is owed for the existing fields**. **Caveat the premise did not state:** any NEW field the repair adds to `duckdb.append` is silently redacted unless added to that entry and its guard.

## Additional finding — the arch byte-identity clause (raised by the arch extract)
`architecture.md` §Stack requires the injector's arg-less default to emit the storm "byte-identically", and `webview_drive.rs:728` confirms only the PATH is threaded, so the headful leg always runs that default. **The historical verification of that clause was a COUNT equality** (`3540 = 3540` spans at `2026-08-25-demo-injector-formalized-api-surface-retire`), not a byte comparison. Because `seq` — and therefore `error_roll` and `jitter` — can be left untouched while only `trace_id()`/`span_id()` fold in a per-process nonce, the property as actually verified (batch count, span count, error count, durations) is preservable. Whether the literal word "byte-identically" then needs narrowing in §Stack is a wrap-time amendment question, not a blocker.

## Open questions
- **Which fork does the repair take — "connection left usable" vs "terminal and reported"?** The scope states it as an unresolved fork and M1/M2 above decide what is *reachable*, but the choice is a design decision with different blast radius (appender-only vs appender + consumer + contract). → blocks: **plan-decision** (P4 must resolve before synthesis).
- **How does arm A keep exercising the appender repair once the injector stops self-colliding?** Premise 3 confirms it otherwise passes for the wrong reason. → blocks: **plan-decision**.
- **Does the second flush hang under M1 or M2?** Discriminated by a probe in `crates/buffer` (worker thread + `recv_timeout`), not by reading. → blocks: **implementation-scope** — the file list above is provisional on M2, which would pull in `consumer.rs` + `contract.rs`.
