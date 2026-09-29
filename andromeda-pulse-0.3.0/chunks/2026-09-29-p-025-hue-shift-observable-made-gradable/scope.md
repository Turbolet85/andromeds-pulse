# Scope — 2026-09-29-p-025-hue-shift-observable-made-gradable

**Chunk:** P-025 hue-shift observable made gradable
**Working-route intent (verbatim outcome):** the emitted hue-shift timing measures the interval the ≤2 s budget
bounds, per Conductor's P-025 measurement contract.

**Provenance:** entry inserted on the founder's word 2026-09-28 (0-pending adaptation, run
`2026-09-28T21-45-38Z-wrap`). A second source is folded in: the operator directive
`D:/dev/projects/additional/pc-overseer/relays/pulse-phase-p025-2026-09-29.md` (pc overseer, founder-delegated;
the founder may overturn), which folds the CI workflow-parse fix into this chunk. Every coordinate the entry, the
contract and the directive name was re-verified at P1 against HEAD `f15536b` (promotion.md's fold-as-hypotheses
rule); results inline. Mechanism claims stay `[inferred]` until P3 closes them.

**Capability claim:** NONE. P-025 is a v0.2.0 id covered by P-075 (`andromeda-pulse-0.3.0/requirements.md:26`,
verified: "P-075 · Conductor e2e + delegated-timing verification … **P-025** (halo hue ≤2 s)"), which stays with the
Conductor-return entry. This chunk makes P-025 *gradable*; Conductor's live leg grades it.

## What it builds

### A. The P-025 observable (the entry's outcome)

Contract: `D:/dev/projects/conductor/contracts/pulse-p025-measurement-contract.md` (pinned at Pulse `83d4060`,
captured 2026-09-13). Its §What a complete implementation looks like is the acceptance shape:

1. **Expose the tier's effective instant** (`tier_effective_at`) on the payload the constellation canvas already
   receives (`ServiceListItem`, produced by `services.list_with_states`), per the contract's two-case rule:
   - **rise** — an incident opens above the current maximum → that incident's `opened_at_unix_nano`
     (verified: `crates/triage/src/contract.rs:418`, `pub opened_at_unix_nano: i64` on `Incident`);
   - **fall** — every incident holding the maximum leaves the active set → the instant the last one left.
     `[premise-corrected: the contract names the broadcast's transitioned_at_unix_nano (broadcast.rs:41); the persist-cycle reconciler resolves through IncidentRegistry::mark_resolved and emits NO lifecycle event, and the broadcast has no subscriber, so that source misses a leave path. Every resolution path (incidents_router.rs:300, incident_observer.rs:67, persistence.rs:272) goes through mark_resolved, which stamps Incident.resolved_at_unix_nano and KEEPS the row in the in-memory registry (no remove/retain anywhere) — the fall instant is that field]`
     `[premise-corrected: the contract says "by resolution or acknowledgement"; list_active excludes ONLY Resolved (incident/registry.rs:213-221), so an Acknowledged incident stays active and keeps contributing its tier — the maximum falls ONLY on resolution]`
2. **Compute the emitted duration as paint instant − `tier_effective_at`**, millisecond resolution.
3. **Name the carrying field explicitly and admit it to the leaf's allowlist entry** (the leaf's admitted set is
   `duration_ms` + `severity_tier`; verified: the test `pulse-app/tests/unit_observability_allowlist_delegated_timing.rs`
   exists; the allowlist entry sits at `pulse-app/src/observability.rs:990`).
4. **Emit per changed service**, not the slowest-wins maximum across the render pass — or carry a service
   identifier. The identifier option is closed by construction: `service` is an OTLP resource
   attribute and may never be a label (security-plan §Logging; obs-plan §5 cardinality — the canvas comment at
   `ConstellationCanvas.tsx:85-87` states this; `unit_observability_allowlist_delegated_timing.rs:74-93` bans
   the identifier fields on the leaf), so per-changed-service emission is the only admissible shape.

The contract's leaf choice: a corrected `duration_ms` on `metric.constellation.hue_update_ms` (preferred), or a
sibling leaf with the old one retired/documented superseded. The field that carries the duration is stated
literally either way.

**Coordinates re-verified at HEAD (P1):**
- Fire site: `pulse-app/ui/src/widget/ConstellationCanvas.tsx:112-155` — `elapsedMs = nowMs -
  item.last_seen_unix_nano / 1_000_000` (`:142`), slowest-wins across `dots` (`:143-146`), tagged with the slowest
  dot's tier, one `recordConstellationHueLatency` per pass. Contract terms 1 (staleness) and 2 (slowest-wins) hold
  as written.
- Emit site (a refinement the contract did not name): the leaf is emitted by the TauRPC resolver
  `record_constellation_hue_latency` at `crates/ui-bridge/src/telemetry.rs:272-282` (`target:
  "metric.constellation.hue_update_ms"`), input `ConstellationHueLatencyInput` (`:122`).
- Tier derivation: `pulse-app/src/services_router.rs` — `list_with_states` (`:93`), `tier_rank` (`:80`).
- Lifecycle tick: `crates/triage/src/lifecycle/mod.rs:52` `DEFAULT_LIFECYCLE_HEARTBEAT_INTERVAL = 15 s`.
- `crates/triage/src/baseline/activity_floor.rs:84` `last_observed_unix_nanos` exists (the rejected fix
  candidate's anchor — the contract rules it insufficient; this chunk does NOT take that path).

**Mechanism claims folded as hypotheses (P3 closes):**
- `[premise-corrected: "that site holds … both timestamps" holds for the RISE only — list_with_states reads list_active, which excludes the resolved incidents whose resolved_at is the fall instant; the resolver needs an all-statuses listing of the workspace's incidents from the registry (one IncidentRegistry implementor, so the trait extension threads nowhere else)]` The contract's claim "That site holds the full `Incident` records at the moment it decides the tier, so both timestamps the rule below needs are already in hand there" (contract §The window), kept verbatim for the record.
- "An incident's `priority_tier` is immutable after opening" — VERIFIED at HEAD (mechanism, not the contract's count
  of eight, which is not copied): the only `.priority_tier =` field assignment in `crates/` + `pulse-app/` is the
  derived `ServiceListItem` one (`services_router.rs:97`); the dedupe path (`inference_runtime.rs:808-836`) calls
  `observe_reemission` and never rewrites the tier; the corpus incident UPDATEs (`corpus/src/contract.rs:652`,
  `:682`) set `status, updated_unix_nano, resolved_unix_nano, payload` / `read_unix_nano` only.
- "`last_seen_unix_nano` has no ingest-path writer …" (contract term 3) — NOT re-derived: the plan removes the hue
  observable's dependence on `last_seen_unix_nano` entirely, so nothing rests on it.
- `ServiceListItem` gaining a field is a bindings shape change, NOT a new TauRPC procedure — VERIFIED: the struct
  is the item type of the existing `services.list_with_states` payload and is never bincode-persisted
  (`lifecycle/persistence.rs::services_to_entries` converts it to `ServiceRegistryEntry`); 3 Rust struct literals
  break (`lifecycle/registry.rs:189`, `lifecycle/persistence.rs:312,319`); TS fixtures are unaffected (the field
  renders optional).
- The P-027 discovery effect is a SEPARATE `useEffect` (`ConstellationCanvas.tsx:88-110`) — VERIFIED separable;
  it stays byte-untouched.
- NEW (research): only the WIDGET canvas (`pulse-app/ui/src/widget/ConstellationCanvas.tsx`) emits P-025 — the
  dashboard copy (`dashboard/routes/traces/ConstellationCanvas.tsx`) calls no `recordConstellation*`; the fire
  site stays on the widget.
- NEW (research): under `prefers-reduced-motion: reduce` the widget canvas paints ONCE at loop start
  (`frame-loop.ts:21-24`; the loop effect's deps are `[adapter, reducedMotion]`), so a later tier change is never
  repainted and no paint instant exists — the "paint" end of the window is undefined there (P4 fork).
- NEW (research): the webview polls `services.list_with_states` every 1 000 ms (`use-service-constellation.ts:22`),
  so the observable's floor includes ≤1 s of poll latency — part of the quantity the budget bounds, not an
  artifact.

### B. CI workflow-parse fix (operator directive, folded)

The directive (item 1) states CI has never started a job since at least 2026-07-25: three workflow files set a
WORKFLOW-level `env:` from `${{ runner.temp }}`, and the `runner` context is unavailable at workflow level (only
`github` `inputs` `secrets` `vars`). Verified at HEAD:
- `.github/workflows/ci.yml:12` `ANDROMEDA_PULSE_DATA_DIR: ${{ runner.temp }}/andromeda-pulse-ci-data`
- `.github/workflows/release.yml:35` `… ${{ runner.temp }}/andromeda-pulse-release-data`
- `.github/workflows/update-channels.yml:53` `… ${{ runner.temp }}/andromeda-pulse-update-channels-data`
- Control: `.github/workflows/secret-scan.yml` has no workflow-level `env:` (verified).

Fix direction per the directive: move the variable to where `runner` is available (job-level `env:`, or
`$RUNNER_TEMP` inside steps) — the HOW is research's.
`[premise-corrected: GitHub's context-availability table (fetched 2026-09-29) lists jobs.<job_id>.env → github, needs, strategy, matrix, vars, secrets, inputs — NO runner — so job-level env: fails parsing exactly as workflow-level does; runner is available only at step level (steps.env / run / with). The fix is a step writing ANDROMEDA_PULSE_DATA_DIR=$RUNNER_TEMP/… to $GITHUB_ENV right after harden-runner in each job, bash shell (the Windows matrix leg), so the existing ${{ env.ANDROMEDA_PULSE_DATA_DIR }} uses in later steps keep resolving and the boot-smoke's three verbs keep sharing one exported dir]` Acceptance: a push of this chunk's commit starts real jobs
(`ci.py conclusion` reads checks > 0 on that sha), which is a witness the parse failure cannot pass vacuously.
Further genuine failures the first real run surfaces are THIS chunk's findings, triaged by the plan's own rules,
not reasons to widen it (directive item 2).

Scope boundary on B: the fix touches only the `ANDROMEDA_PULSE_DATA_DIR` placement in the three files (VERIFIED:
its only other uses are `${{ env.ANDROMEDA_PULSE_DATA_DIR }}` in upload `with.path` at `ci.yml:160,208` and
`release.yml:181`, which the step-level `env` context serves; `quality_gate_workflow.rs:185-210` requires ci.yml's
workflow-level `env:` block to keep `CI_RUN_ID` / `GIT_COMMIT_SHA` / `DEPLOYMENT_ENVIRONMENT`, which stay).
`actionlint` is not installed on this host, so the committed local witness is a workflow self-lint test in
`pulse-app/tests/quality_gate_workflow.rs` (no workflow- or job-level `env:` value references a step-only
context); the decisive witness stays the pushed run starting jobs.

### C. CI verdicts Setup 5a read (the second fold source)

Base = last master flip `83d4060`; every sha through HEAD read via `ci.py conclusion`:

| sha | verdict | runs (id · path · conclusion · wall-clock) |
|---|---|---|
| `f15536b` | red · checks 0/0 | 36489885394 ci.yml · 36489884167 update-channels.yml · 36489882780 release.yml — all failure, started = updated (0 s) 2026-09-28T22:01:40-41Z |
| `8fc47f1` | none recorded | — |
| `99d2f9c` | none recorded | — |
| `ca5d7ee` | none recorded | — |
| `8b86529` | red · checks 0/0 | 36487767925 ci.yml · 36487766806 update-channels.yml · 36487765789 release.yml — all failure, 0 s, 2026-09-28T21:41:25-26Z |
| `83d4060` | red · checks 0/0 | 34535079754 update-channels.yml · 34535078976 ci.yml · 34535078246 release.yml — all failure, 0 s, 2026-09-10T21:59:04-06Z |

- Every red above has NO failing job — `checks 0/0`, 0 s wall-clock (started = updated), the run named by its
  PATH not its `name:` — the workflow-file-issue signature the directive names. VERIFIED against THE RECORDED RUNS
  (runner-only subject, closed against the runs, not HEAD): all nine run rows read that way at all three shas.
  The subject intersects B, so these reds are owned here: B's acceptance is what turns them.

## Folded annotations (the working entry's freight, all four blocks)

- **CONTEXT (U05 reflow):** the first step is U05's reflow from the setup upgrade `8b86529` — `cargo fmt --all`,
  with `cargo fmt --all -- --check` as a Test Command. (Verified: `rustfmt.toml` holds `edition = "2024"`.)
  `[premise-corrected: cargo fmt --all -- --check at HEAD f15536b exits 0 with zero "Diff in" lines (run 2026-09-29, scratchpad fmtcheck.txt) — U05's reflow is already applied, so the "first step" has nothing to do; cargo fmt --all -- --check stays as the fmt gate and the CRLF-noise question does not arise]`
  The directive's item 4 stands: ~203 worktree files are checked out CRLF (index LF) and the `.gitattributes`
  re-checkout is the founder's hand — do NOT run it.
- **CONTEXT (contract):** Conductor's contract (above, §A); every coordinate re-verified at HEAD at P1 — done,
  results inline in §A.
- **PREREQ: close rust gate deferral** — deferred since `2026-08-30-agent-harness-teardown-truth` (zero
  .rs/manifest/lockfile delta there). This chunk IS Rust-touching (§A items 1 and 3 at minimum; the U05 reflow
  touches every `.rs` file), so it discharges the deferral: `cargo clippy --workspace --all-targets
  --all-features -- -D warnings` + workspace `cargo nextest run --workspace --profile ci` owed in its gates.
- **PREREQ: re-check `cargo audit`** — standing deferral since `2026-08-15-corpus-key-persistence`, ratified pin
  #22, origin preserved. This chunk's wrap is session 66 — a BETWEEN-POINT (the FULL-FORM point is session 67;
  confirm the session count at the wrap, an inserted wrap moves it). Between points: re-verify basis
  (`duplicate advisory ID: RUSTSEC-2026-0244`) + overlap (`cargo deny check advisories`, DISTINCT `RUSTSEC-` ids
  re-enumerated from scratch; `bans licenses sources` as the pass/fail gate) and record `probe skipped per
  ratified interval (next: 67)` in the report. No "Nth consecutive" ordinal.

## P4 rulings (val-1: intent-incomplete, amended at P5)

The operator ruled three forks at P4 (overseer, founder-delegated). Each widens the scope above only where the
outcome needs it.
- **Reduced-motion repaint, IN scope.** Without it the paint instant does not exist under reduced motion, so the
  P-025 sample would be untrue in that mode. The fix also keeps a11y-plan §6.
- **Witnessed-only samples.** A sample is emitted only when `tier_effective_at` ≥ the canvas mount instant. A
  restored tier is not a latency, and Conductor drives telemetry into an already-open Pulse, so every change it
  grades is witnessed.
- **One draft PR as the CI carrier.** `ci.yml` triggers only on `pull_request` and `push: main`
  (`.github/workflows/ci.yml:3-6`, read at P4), so after the parse fix a push to this branch starts no job. The
  operator pass opens ONE draft PR `chore/migrate-pulse-to-v3 → main`. It re-fires CI on every later push, and no
  trigger config changes. It stays an operator entry, never merged or closed by the builder; its fate is the
  founder's. §B's acceptance reads that PR run's checks on the pushed sha.
- **Leaned without asking:** the live leg lives in xtask (`smoke:hue-shift`), following the
  `external_resolve.rs` / `gap_resume.rs` precedent. This adds `xtask/src/hue_shift.rs`, `xtask/src/main.rs` and
  `xtask/src/external_resolve.rs` to the modify-set.

## Boundaries

- IN: §A (observable), §B (CI parse fix), the U05 reflow, the two PREREQ discharges.
- OUT: grading P-025 (Conductor's live leg, operator-gated, never a CI gate); the P-027/P-026/P-030 sibling
  delegated-timing leaves beyond byte-preservation; the Halo State Pulse glow layer (deferred to the next version,
  ruling 2026-08-29); the `.gitattributes` re-checkout (founder's hand); any capability claim.
- No new TauRPC procedure, no new capability JSON grant, no new workspace dependency — VERIFIED by the file list in
  `research.md` (the `ServiceListItem` field rides the existing procedure; the hue input struct and its leaf stay
  unchanged — the contract's preferred option, a corrected `duration_ms`).
