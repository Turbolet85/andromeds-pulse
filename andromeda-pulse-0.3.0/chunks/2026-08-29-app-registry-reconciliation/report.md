# Report — 2026-08-29-app-registry-reconciliation

**Chunk:** App-registry reconciliation with externally-resolved rows — an incident resolved outside the app
stops showing active until restart
**Date:** 2026-08-29
**Commits:** none yet this session (the two since `last_wrap` — `ba86147` drift-base detector, `416d7df` the
predecessor chunk — belong to the prior wrap)

## Changes (structured — detectors read this)

- **Files:**
  - `crates/triage/src/incident/persistence.rs` — the read port, the reconcile step, the fail-safe, the counter fold, a module-local clock helper, 6 new in-crate tests
  - `crates/triage/src/incident/mod.rs` · `crates/triage/src/contract.rs` — re-exports for the new trait + const
  - `pulse-app/src/incident_persistence.rs` — the adapter impl (ids only)
  - `pulse-app/src/main.rs` — N-trait-from-one-`Arc` wiring at the persist-loop spawn site
  - `pulse-app/src/observability.rs` — allowlist leaf completion
  - `pulse-app/tests/unit_observability_allowlist_incident_diagnostics.rs` — leaf guard + fallback discriminator
  - `pulse-app/tests/integration_incident_write_guard.rs` — call sites updated; one pre-existing test STRENGTHENED
  - `pulse-app/tests/e2e_p3_mcp_resolve_content.rs` **(new)** — cross-process CONTENT assertion
  - `xtask/src/external_resolve.rs` **(new)** · `xtask/src/main.rs` — the `smoke:external-resolve` leg

- **Symbols / APIs:**
  - NEW `triage::contract::DurableActiveIncidents` (trait; one method `active_incident_ids(&str) -> Result<Vec<i64>, IncidentError>`), re-exported through `incident::mod` and `contract`
  - NEW `triage::contract::INCIDENT_RECONCILE_KIND = "incident_reconcile"` — a `persist_kind` discriminator joining the existing bounded set
  - NEW private `triage::incident::persistence::reconcile_externally_resolved`
  - CHANGED `run_incident_persist_cycle` — +2 params (`durable: &dyn DurableActiveIncidents`, `now_unix_nano: i64`). **Remaining-caller fact:** 3 call sites total, ALL updated — `run_incident_persist_loop` (production), the in-crate test, and `pulse-app/tests/integration_incident_write_guard.rs` (×2). Not a sole-caller change.
  - CHANGED `run_incident_persist_loop` — +1 param (`durable: Arc<dyn DurableActiveIncidents>`); ONE production caller (`pulse-app/src/main.rs`), updated
  - NEW `impl DurableActiveIncidents for CorpusIncidentPersistence` (pulse-app) — a second trait view over the same `Arc<dyn CorpusWriter>`
  - NEW xtask subcommand `smoke:external-resolve` (`--bootstrap-seconds`, `--observe-seconds`)
  - **No TauRPC procedure added**; no IPC route, no event topic, no port, no env var.

- **Crates / modules:** none added or removed. New module `xtask/src/external_resolve.rs`. **No new crate edge** — the reconciler is `pulse-app`-sited and `pulse-app → corpus` already existed; `triage → corpus` still does not exist (graph-confirmed: `corpus` has exactly two inbound edges, `mcp-server` and `pulse-app`).

- **Dependencies:** none added, none bumped. `Cargo.toml` / `Cargo.lock` untouched.

- **Schema / config:** none. No DDL, no `SCHEMA_VERSION` bump, no `config.toml` key, no `Settings` field.

- **Spec-master edits:** none applied by implement (spec bodies are read-only there). Expected amendments carried into this wrap — see *Deviations* and the plan's Implementation notes.

- **Counts / qualifiers moved:**
  - `triage.incident.persist` allowlist leaf **4 → 5 fields** (adds `reconciled_count`). Stated in `obs-plan.md` §8 (Incident-path diagnostic leaves) and restated in `.claude/rules/observability.md`.
  - Workspace test count **2056 → 2074** (+18, all in the default gate) plus 2 mcp-gated tests that run under `--features mcp-server`.
  - `xtask smoke:gap-resume`-family leg count: a new sibling leg joins (`smoke:external-resolve`) — `test-plan.md` §3 and `.claude/rules/verification-harness.md` enumerate the scenario legs.

- **Dev-tool versions:** none.

- **Reverted / negative API facts:** a mutation neutralising `reconcile_externally_resolved` was applied and **fully reverted** (verified: 0 `MUTATION-CHECK` residue). It existed only to produce the RED arm.

- **Spec claims disproved by measurement:**
  - **The chunk PLAN's second acceptance half does not discriminate.** `plan.md` asserts `item_count` dropping as one of "two independent halves". Measured across the mutation pair: `item_count` went **2 → 0 in BOTH arms** — the finite storm's auto-resolve fires 120 s after last re-emission and the 170 s observation window covers it. A verdict keyed on the drop would have passed in both worlds. This is a **chunk-artifact** claim, not a spec-master one, so per the 2026-08-26 rule its disposition is THIS report entry plus the corrected leg — **no amendment is owed and none is pending**. The shipped verdict keys on `reconciled_count` (1 vs 0) and `declined_count` (0 vs 2), with both measured shapes pinned as a unit test.
  - `arch.md` §Established Decisions [Corpus Write Arbitration] **Accepted residual** states the running app shows an externally-resolved incident active until restart and its persist cycle declines that row every cycle. That was true when written; **this chunk closes it** — an amendment target, not a falsification.

- **Coverage of new surfaces:**
  - `triage.incident.persist → reconciled_count` (obs field on an existing target) → validation n/a · instrumentation ✓ (aggregate-only, once per cycle, never per row) · PII redacted✓ (a bare `u64`; no identity / `scope_id` / workspace / title / payload / SQL) · tests ✓ (leaf set-equality both directions + a fallback discriminator asserting no `triage`/`triage.incident` prefix key carries the persist fields) · a11y n/a · tokens n/a
  - `triage.incident.persist.error` + `persist_kind = "incident_reconcile"` (failure path of the durable read) → validation n/a · instrumentation ✓ (bounded static token; `error_category` only) · PII redacted✓ · tests ✓ (fail-safe unit pin) · a11y n/a · tokens n/a
  - `DurableActiveIncidents::active_incident_ids` (new internal trait boundary; **not** an external input) → validation n/a · instrumentation n/a (its failure surfaces on the error target above) · PII redacted✓ (returns row **ids only** — no payload is decrypted or decoded in `triage`) · tests ✓ (4 in-crate pins incl. the fail-safe) · a11y n/a · tokens n/a
  - `cargo xtask smoke:external-resolve` (dev-only harness leg, no runtime/bundle impact) → validation n/a · instrumentation n/a · PII n/a · tests ✓ (10 in-crate verdict pins incl. both measured live shapes) · a11y n/a · tokens n/a
  - `mark_incident_resolved` cross-process response CONTENT (existing MCP surface, newly asserted) → validation n/a · instrumentation n/a · PII n/a · tests ✓ (real subprocess, both applied and guard-declined arms) · a11y n/a · tokens n/a
  - **No UI surface touched** — no `pulse-app/ui/**` path in the modify-set; the existing ~1 s webview poll carries the effect with no frontend change.

## Deviations from intent

1. **The plan's `item_count` acceptance half was replaced.** Justification: the mutation check measured it non-discriminating (2 → 0 in both arms, via auto-resolve). Keeping it would have shipped a decorative "independent half". Replaced with `declined_total == 0`, which measured 0 (GREEN) vs 2 (RED). Both shapes are now pinned.
2. **`--sustained` could not drive the leg.** Justification: it holds a constant error rate, so the cue evaluator's short/long EWMA ratio converges and never trips the gate — measured as 0 incidents in 240 s. The leg backgrounds the arg-less finite storm instead, which also keeps the resolve inside the 120 s auto-resolve window. (`rules/testing.md` records this dead end for a different leg; it recurred here.)
3. **Boot-smoke used the direct-binary variant, not `npx @tauri-apps/cli dev`.** Justification: test-plan §3 sanctions the direct-binary form where the check needs runtime state; the leg boots the real binary three times with real OTLP ingest and clean shutdown, and it avoids the documented Windows GUI-orphan hazard.
4. **The plan's smoke was authored as one leg; it ran as a RED/GREEN pair.** Justification: the plan asked for a RED measured before the fix; implementation had already landed, so a mutation check (stronger — it also proves the pins discriminate) produced the RED arm. The mutation was verified to have landed (dead-code warning) and fully reverted.

## Decisions & corrections

- **Operator fork (P4):** reconciliation folds into the existing 60 s persist cycle (over a separate task or the 30 s auto-resolve tick); the cross-process test debt is discharged in-chunk.
- **Verdict design:** `item_count` is evidence, not a verdict half — recorded in the leg's own module doc so a later reader does not re-add it.
- **A harness typo produced a false product claim.** `query_incident_list` returns `items[].incident_id`, not `items[].id`. Reading the wrong name yielded `None` and the leg printed *"the storm did not form"* while the corpus held 2 `active` rows and the log showed 3 `incident.created`. Caught by calling the sidecar by hand and reading its real response. The INCONCLUSIVE text was reworded to stop asserting a cause it cannot distinguish.
- **Disk exhaustion twice** (`ENOSPC` on a file write, then `cl.exe : fatal error C1085`): `target/debug` reached 301.6 GB on a 300 GB drive. `cargo clean -p pulse-app` freed 97.4 GiB; the `--features mcp-server` graph needs a second full DuckDB C++ compile, so the pre-authorized full `cargo clean` (310.1 GiB) was taken. PowerShell was authoritative for free space — the ReFS reclaim lagged the delete by ~20 s both times.
- **Bindings were clobbered and regenerated**, exactly as the standing discipline predicts: `grep -c '"mcp":'` read 0 before regen and 1 after; `capability-drift` ran LAST and clean.

## Outcome

**Acceptance criteria: MET**, with deviation 1 above (one criterion replaced by a stronger one on measurement).

Gates, all run and green:
- `cargo fmt --check` · `cargo clippy --workspace --all-targets --all-features -- -D warnings` (0)
- `cargo nextest run --workspace --profile ci` — **2074 passed / 2074 run + 1 skip**; +18 vs the 2056 baseline, matching the tests added exactly, and each new test confirmed **collected by name** via `cargo nextest list`
- `cargo test --test e2e_p3_mcp_resolve_content -p pulse-app --features mcp-server` — 2/2, and **neither skipped** (no `[skip] no OS credential store` line: both drove a real sidecar subprocess against the shared encrypted corpus)
- `cargo xtask capability-widening-check` — clean (0 violations / 3 inspected)
- `cargo xtask check:ingest-progress` — PASS (128 buffer ticks, longest zero-delta run 0)
- `cargo xtask capability-drift` — clean (0 missing / 0 extra), run LAST after the bindings regen
- **1 fix-loop iteration** (two of my own xtask fixtures omitted the post-resolve `list` record the ~1 s webview poll always emits, so `judge` correctly stopped at its `saw_list_after` precondition — a fixture bug, not production logic)

Smoke (boot-path changed — `pulse-app/src/main.rs`): `cargo xtask smoke:external-resolve`, direct-binary variant, three full boots.

```
GREEN  item_count 2 → 0 · reconciled 1 · declined 0 · 2 persist cycles · 0 panics · 51,263 log lines · PASS
RED    item_count 2 → 0 · reconciled 0 · declined 2 · 2 persist cycles · 0 panics · 51,263 log lines · FAIL
```

A real `andromeda-pulse-mcp` subprocess resolved incident id=1 (`applied=true`); the app's registry reconciled
it within two persist cycles **without a restart**. `reconciled_count` went positive for the first time (the
predecessor recorded `declined_count` as plumbing-proven / arithmetic-test-only; this chunk drives both above
and back to zero respectively). The RED arm came from a mutation whose application was confirmed by the
resulting dead-code warning, and which was then fully reverted.

**Surfaced, not fixed:** `pulse://stream/incidents` is a registered event topic with **producers but no
consumer** — `STREAM_NAME_INCIDENTS` is defined and self-asserted, the auto-resolve observer and the
`incidents.*` router `send()` into it, and nothing subscribes or bridges it to the webview. This is why the
reconciler deliberately emits no lifecycle event: adding one would be dead plumbing that reads as if it did
something. Needs an owner; out of scope here.
