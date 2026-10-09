# Report — 2026-08-15-tier-1-incident-path-investigation

**Chunk:** Tier-1 incident-path investigation — a detected storm produces an incident again: premise-check the in-window regression claim, locate the break in the storm→cue→incident seam, fix it, and un-mute the diagnostics that forced an out-of-band sqlite3 read
**Date:** 2026-08-15
**Commits:** (none since last_wrap — `3ac3d9d feat(2026-08-15-corpus-key-persistence)` is the last commit; this chunk's work is uncommitted at report time)

**Headline: the chunk's own premise was falsified by its first step.** A detected storm DOES
produce an incident on HEAD — measured twice on two fresh data dirs. No regression existed, so no
bisect was run and no incident-path fix was made. The obs repair the entry OWNS landed and is
proven live; the investigation's finding is the primary deliverable.

## Changes (structured — detectors read this)

- **Files:** 9 paths per `git status`. Source: `pulse-app/src/observability.rs` (+31),
  `crates/triage/src/cadence/coordinator.rs` (+86, test-only), `pulse-app/tests/unit_observability_allowlist_incident_diagnostics.rs` (NEW).
  Artifacts: `andromeda-pulse-0.3.0/chunks/2026-08-15-tier-1-incident-path-investigation/` (scope · research · plan · report · `evidence/premise-check.md`),
  `.andromeda/runs/2026-08-15T18-08-00Z-phase/` (7 extracts + graph trace).
  Bookkeeping: `.andromeda/master-route.md`, `andromeda-pulse-0.3.0/working-route.md`,
  `.claude/session-handoff.md`, `.andromeda/friction-log.ndjson`.
- **Symbols / APIs:** NO new public fn, TauRPC procedure, IPC method, endpoint, export, port, socket,
  or env var. One new test fn `cadence::coordinator::tests::start_cadence_coordinator_ignores_suggested_cue_on_the_attention_broadcast`
  (`crates/triage`). Six new test fns in the new `pulse-app/tests/` integration target.
- **Crates / modules:** none added · none removed · none restructured.
- **Dependencies:** none added · none bumped. `Cargo.toml` / `Cargo.lock` untouched.
- **Schema / config:** none. No corpus table, no DDL, no `SCHEMA_VERSION` bump, no config key.
- **Spec-master edits:** none applied in the chunk (implement is read-only on specs). ONE expected
  amendment queued for this wrap: `obs-plan.md §8` — the three muted-diagnostic backlog entries move
  from *measured-not-fixed* to *landed leaves* (this entry is their named owner).
- **Counts / qualifiers moved:**
  - workspace nextest **1768 + 1 skip → 1775 + 1 skip** (+7 = 6 allowlist guards + 1 coordinator proof-lock).
    Stated in the handoff; test-plan states no absolute count.
  - `obs-plan.md §8` muted-diagnostic backlog: **3 targets outstanding → 0 outstanding** (the five
    NEWLY discovered ones are a separate, un-owned set — see Deviations).
- **Dev-tool versions:** none installed or upgraded.
- **Reverted / negative API facts:** none. Nothing was written then removed. NOTE the *absence* that
  matters: `crates/triage/src/pattern/storm.rs`, `crates/triage/src/cadence/coordinator.rs`
  (production code), and `pulse-app/src/inference_runtime.rs` were candidate fix sites in the plan
  and were deliberately NOT modified — no break was located, and the plan barred manufacturing one.
- **Spec claims disproved by measurement** (needs disposition — Validate check 6):
  1. **The working-route entry's own premise.** It states the break "plausibly REGRESSED within the
     last two chunks' window". **Measured FALSE**: `evidence/premise-check.md` §3–§5 — a sustained
     storm on HEAD produces 2 active incident rows. The entry is `[{marker}]`-frozen, so this is a
     recorded correction, not an edit.
  2. **The `triage.cue.emit == 0` inference.** The entry reads that zero as evidence "the break sits
     in the storm→cue seam". **Disproved**: that target has exactly ONE writer
     (`crates/triage/src/cue/emitter.rs:195`), fed only by the BaselineState-derived emit cycle; the
     storm producer logs `triage.pattern.storm.detected` / `.emit` instead. Zero there is the
     EXPECTED reading for a storm-only run and carries no information about the storm path.
  3. **The P-074-era CARRY calling the `LwwQueue` path dead.** Measured NUANCE, not a clean
     disproof: `drain_all` remains dead (0 production callers, unchanged), but the queue's CAP path
     is **live** — `digest.lww.drop drop_reason=queue_cap_reached` fired on every run (tier1/tier2/tier3).
     Route-resolve nuances the annotation; it must not overclaim in either direction.
- **Coverage of new surfaces:**
  - `obs leaf incidents.list_active.request → item_count` → validation n/a · instrumentation ✓ (exact allowlist leaf, live-verified unredacted) · PII redacted✓ (aggregate count only) · tests unit✓ (`unit_observability_allowlist_incident_diagnostics`) · a11y n/a · tokens n/a
  - `obs leaf triage.incident.persist → incident_count, persist_kind, duration_ms` → validation n/a · instrumentation ✓ · PII redacted✓ (counts + `&'static str`-typed label, bounded by construction) · tests unit✓ · a11y n/a · tokens n/a
  - `obs leaf triage.incident.corpus_restore → kind, restored_incident_count` → validation n/a · instrumentation ✓ · PII redacted✓ (count + literal label) · tests unit✓ · a11y n/a · tokens n/a

## Deviations from intent

1. **Steps 2 (bisect) and 4 (fix) produced nothing** — Branch B. The plan's own Step 1 branch
   instruction ("skip Step 2, proceed to Step 3B") and Step 4's no-break clause. Bisecting a premise
   the measurement disproves would have burned the chunk.
2. **`triage.incident.persist` leaf carries a third field, `duration_ms`, beyond the plan's list.**
   The emit site (`crates/triage/src/incident/persistence.rs:176-182`) emits three fields;
   allowlisting the two named would have left the target PARTLY redacted — the same defect in
   miniature. Caught by reading the emit site rather than transcribing the plan. Live-verified:
   `{"duration_ms": 37, "incident_count": 2, "persist_kind": "incident"}`.
3. **Ground truth read via Python stdlib `sqlite3` (`mode=ro`), not the `sqlite3` CLI** — the CLI is
   not on this host's PATH. Same engine, same plaintext columns, keyless, equally independent of the
   app's decryption path.
4. **The plan's narrow gate `cargo nextest run -p pulse-app -E 'binary(...)'` failed** with
   rlib-format errors naming dep crates (`muda`, `wasmtime_internal_cache`, `libduckdb_sys`,
   `arrow_cast`) immediately after a SUCCESSFUL `cargo build --workspace --tests`. Cause:
   `--workspace` unifies features across members, `-p` resolves a narrower subgraph. Ran the plan's
   own `--workspace` gate instead — a strict superset, and it compiled and passed clean.
5. **The plan's a11y suite-health line is malformed** — `npx playwright test --list --prefix pulse-app/ui`
   uses `--prefix`, an npm flag, not an npx/playwright one. Ran the correct form from `pulse-app/ui`
   with `--config=playwright-a11y.config.ts`; suite reported 33 tests in 15 files (non-empty).
6. **Smoke driven by direct binary launch, not `agent-run.sh run`** — the verification needs a fresh
   `ANDROMEDA_PULSE_DATA_DIR` + `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` + a sustained `inject_demo`
   storm, which the harness `run` verb does not express; this direct-launch form is the project's own
   recorded boot-smoke procedure. `agent-run.sh status` still ran as the plan's listed harness gate.
7. **Five ADDITIONAL muted-diagnostic targets discovered and deliberately NOT fixed** —
   `metric.pipeline.l1a.query_count_total → query_name`; `metric.pipeline.l1a.query_latency_p99_milliseconds
   → duration_ms/query_name/row_count_returned` (100–110× each per run); `triage.cue.tick →
   bypass_triggered/cues_suppressed`; `triage.incident.auto_resolve.tick →
   duration_ms/evaluated_count/resolved_count`; `interpretation.model.load → inference_mode`. Same
   class as the three repaired, but named by no plan step or acceptance criterion. Implementing them
   would have silently widened the chunk; handed off instead.

## Decisions & corrections

- **Operator (phase invocation):** take this chunk out of working-route order; premise-check FIRST;
  rank the workspace-key suspect but do not assume it; keyless sqlite3 is ground truth; counters over
  gauges; every claim names its query; evidence lands in this chunk's folder.
- **Operator (P5 approval):** approved the decision-procedure shape, the conditional treatment of the
  `CadenceTriggerChannel` asymmetry (design change ≠ regression repair), and the explicit rejection of
  threshold-lowering. Supplied `CANARY_STORM_COUNT = 6` as a named Conductor-side constant — a fact no
  artifact reachable from this repo contained — converting research's arithmetic fit into a specific
  cross-project mismatch with a Conductor-side fix.
- **Operator (wrap invocation):** counts from `git status` as standing practice; nuance the LwwQueue
  CARRY rather than overclaim; the five new muted targets join the existing targeted-cleanup family;
  the `agent-run.sh status` pid defect gets an owner; the Conductor-visit queue rides the handoff
  explicitly as the critical path.
- **Correction carried from /implement:** the coordinator comment at
  `crates/triage/src/cadence/coordinator.rs:407-409` claims non-Autonomous cues "flow through their
  dedicated channels". True for BaselineState-derived cues (`emit_cue` forwards Suggested to
  `CadenceTriggerChannel`, `emitter.rs:187-190`); **false for storm-derived ones** —
  `observe_and_dispatch_storm` never forwards, so a Suggested storm is dropped outright. Recorded
  here; the comment fix rides a future chunk that touches the file.
- **Honest limit on an acceptance criterion.** "Locked by a test that would have failed against
  pre-fix code" is fully met by the 6 allowlist guards (the three keys were absent; `for_target(…)
  .expect(…)` would panic, and the pre-fix run shows the fields `<redacted>`). The coordinator test is
  a **drift guard on a contract that already held** — with an Autonomous positive control so it cannot
  pass on a dead coordinator — but it is NOT claimed as a regression lock, because there was no fix.

## Outcome

**Acceptance criteria:** met, with two conditional criteria correctly resolving to their no-break
branch.

- Premise-check ran on HEAD, fresh data dir, deterministic L4; outcome recorded at
  `evidence/premise-check.md` with the command or SQL beside every number ✓
- Incident count established by keyless read of `corpus.db` plaintext columns ✓ (2 rows, twice)
- Break located and named with evidence — **resolved as: no break exists**, with the arithmetic
  accounting for every measured zero (`CANARY_STORM_COUNT=6` vs `DEFAULT_AUTONOMOUS_THRESHOLD=10`) ✓
- "rows > 0 where it showed 0 before the fix" — **conditional, N/A**: no fix, so no before/after ✓
- Regression lock — met by the 6 allowlist guards; the coordinator test is a drift guard (see above) ✓
- Three obs leaves resolve EXACTLY and emit unredacted on a live run ✓ — `item_count` shows 0→1→2,
  `{"kind":"incident","restored_incident_count":0}`, `{"duration_ms":37,"incident_count":2,"persist_kind":"incident"}`
- No bare `triage` / `incidents` prefix key ✓ (asserted by test)
- Every added field is a count or bounded label; no evidence artifact carries decrypted payloads ✓
- No new crate / procedure / topic / env var / corpus table / capability ✓
- a11y: zero new violation tuples vs baseline ✓

**Gates green** (commands run): `cargo fmt --check` · `cargo build --workspace --tests --jobs 4` ·
`cargo nextest run --workspace --profile ci` → **1775 passed / 1 skipped** ·
`cargo clippy --workspace --all-targets --all-features -- -D warnings` · `cargo nextest run -p triage`
→ 421/421 · `cargo xtask capability-drift` → clean · `cargo xtask capability-widening-check` → clean ·
a11y chain → 33 playwright + Lighthouse 7×≥90 + pa11y 7/7 + **regression-detector 0 new** ·
`bash scripts/agent-run.sh status` → exit 0. No gate deferrals — this chunk has Rust delta, so the
workspace gates were mandatory and all ran.

**Smoke** (boot-path changed — `observability.rs` is boot-time obs init): TWO full boots on separate
fresh data dirs. `pc-run1` pre-edit on HEAD (the premise-check, 32,985 lines) and `pc-run2` post-edit
(the fix proof, 26,101 lines). Both: 0 `app.panic.fatal`, 0 `ERROR`, 3 benign
`digest.lww.drop queue_cap_reached` WARN, 2 active incident rows, clean `Stop-Process` shutdown with
`:4317`/`:4318` released and zero orphans.

**Verification matrix:** no capability claimed — correct no-op. This chunk UNBLOCKS P-075 (its canary
needs an incident to form) but does not verify it; P-075 stays pooled.
