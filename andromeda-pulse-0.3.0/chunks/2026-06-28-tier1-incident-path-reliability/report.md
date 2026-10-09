# Report — 2026-06-28-tier1-incident-path-reliability

**Chunk:** Tier1 incident-path reliability — coalesce identical hard-signals + (premise-corrected) thread the triggering cue → one storm incident
**Date:** 2026-06-28
**Commits:** (none yet — wrap authors the chunk commit)

## Changes (structured — detectors read this)
- **Files:**
  - `crates/triage/src/cadence/broadcast.rs` — new `DigestTrigger` + `DigestTriggerBroadcast` (internal cue carrier) + 2 unit tests
  - `crates/triage/src/cadence/coordinator.rs` — `start_cadence_coordinator` emits `DigestTrigger` (full cue) at all 4 cycle sites; +`digest_trigger` param; `#[allow(clippy::too_many_arguments)]`; enriched Tier-1 test + 5 test call-sites updated
  - `crates/triage/src/cadence/mod.rs`, `crates/triage/src/contract.rs` — re-export the two new types
  - `pulse-app/src/digest_runtime.rs` — `spawn_cadence_subscriber` consumes `DigestTriggerBroadcast`, passes `Some(cue)` to `assemble`
  - `pulse-app/src/main.rs` — construct `cadence_digest_trigger` + wire to coordinator + subscriber (L6 `CadenceEventBroadcast` retained, still emitted)
  - `pulse-app/src/observability.rs` — `digest.runtime.cadence_tick` allowlist: `cue_kind`/`cue_priority` → `cue_present`
  - `pulse-app/tests/integration_tier1_storm_one_incident.rs` — NEW (3 tests)
  - `andromeda-pulse-0.3.0/verification-matrix.json` — P-074 → `implemented` (ledger)
- **Symbols / APIs:** new `triage::cadence::{DigestTrigger, DigestTriggerBroadcast}` (re-exported via `triage::contract`). `start_cadence_coordinator` gained an `Arc<DigestTriggerBroadcast>` param; `spawn_cadence_subscriber`'s first param changed `Arc<CadenceEventBroadcast>` → `Arc<DigestTriggerBroadcast>`. NO new TauRPC procedure, NO new HTTP/gRPC endpoint, NO new env var, NO new workspace crate.
- **Broadcast channels:** `DigestTriggerBroadcast` is an **INTERNAL in-process carrier** (coordinator → digest assembler), **NOT a `pulse://stream/*` topic** — no `STREAM_NAME`, no webview/Tauri-event emission. It follows the `cue::CadenceTriggerChannel` precedent (internal channels are NOT arch §Occupied-Resources broadcast topics). The L6 topic `pulse://stream/cadence-events` (`CadenceEventBroadcast`) is unchanged + still emitted → no §Occupied Resources change required.
- **Crates / modules:** none added/removed; changes confined to existing `triage` + `pulse-app`.
- **Dependencies:** none added, none bumped.
- **Schema / config:** none (no migration, no config key, no DuckDB/corpus table).
- **Coverage of new surfaces:**
  - `DigestTrigger` internal cue carrier (coordinator→assembler) → validation n/a (internal; not deserialized from an untrusted boundary) · instrumentation ✓ (existing `cadence.trigger` + `digest.runtime.cadence_tick` with `cue_present`) · **PII: carries `scope_id` but it is NEVER logged** — `cadence_tick` emits only `cue_present` (bool); the L6 `CadenceEvent` stays label-only/PII-free (asserted by chunk #80 `…does_not_leak_pii_canary…` + the new Tier-1 carrier test) · tests unit+integration ✓ · a11y n/a · tokens n/a

## Deviations from intent
- **Premise correction (research-corrects-intent, recorded WITH the user at /phase P4):** intent F13b / the original P-074 framed the fix as "coalesce + elastic queue with heartbeat ticks; TIER1_QUEUE_CAP=3 drops 75%". Phase-3 research falsified that mechanism: the `LwwQueue` is unused on the L4 path (`drain_all` = 0 production callers; the L4 feed is the broadcast), and the real defect was the un-threaded cue (`assemble(mode, None, …)`) making incident creation return early. The OUTCOME (one reliable storm incident) is unchanged; only the falsified mechanism was dropped. scope.md + verification-matrix#P-074 (requirement/observed_gap/acceptance + `notes`) corrected; intent.md + the working-route line left as the human-authored historical source.
- **No elastic queue / heartbeat-drain built; LwwQueue not revived** (per the user directive). The Tier-1 `LwwQueue` path is flagged a removal candidate for a future cleanup chunk (handoff note); not actioned here.
- **`observability.rs` touched as a field-alignment, not a new target** — the plan said "allowlist a new `metric.pipeline.l3.*` target only if added"; instead the existing `digest.runtime.cadence_tick` entry was updated because the tick's fields changed (`cue_present` replaces the old `CadenceEvent`-label fields). In-scope (listed touchpoint).
- **`#[allow(clippy::too_many_arguments)]`** added to `start_cadence_coordinator` (8th arg) — codebase-idiomatic (matches `render_payload`); a param struct would be over-engineering.
- **`inference_runtime.rs` / `assembler.rs` not modified** — plan flagged them "confirm only"; dedup + the `triggering_cue → DigestCueRef` mapping were already correct (proven by the storm test). No-op reconciliation.

## Decisions & corrections
- User directive (at /phase P4): fix the real bug (thread the cue), do NOT revive the dead queue or add a heartbeat-drain, confirm dedup fully coalesces, and amend the falsified acceptance to "exactly ONE incident, reproducibly" (not "0 dropped-by-cap"). Recorded as a conscious amendment.
- Mechanism: the cue (incl. `scope_id`) is delivered to the assembler on a NEW non-L6 `DigestTrigger` carrier so the PII-free `pulse://stream/cadence-events` topic is never widened — a hard constraint surfaced by the chunk #80 canary test.
- Threading `scope_id` to the incident also lights the chunk #91 per-service constellation join (free secondary benefit).
- **Carry-forward:** dead `LwwQueue` Tier-1 path (`drain_all` = 0 production callers) → removal candidate for a future cleanup chunk.

## Outcome
- **Acceptance MET** (verification-matrix#P-074): `integration_tier1_storm_one_incident.rs` — a ≥150-event identical-fingerprint storm under deterministic-L4 yields exactly ONE incident, reproducibly; a cue-less digest yields none (proving the cue-passthrough is the fix).
- **Gates green:** `cargo nextest -p triage` 417/417 · `integration_tier1_storm_one_incident` 3/3 · `cargo fmt --check` clean · `cargo clippy --workspace --all-targets --all-features -- -D warnings` clean · `cargo nextest run --workspace --profile ci` 1686 pass + 1 skip (exit 0) · bindings regenerated (mcp present) · `cargo xtask capability-drift` clean (0 missing, 0 extra).
- **Smoke:** skipped-for-cause — backend-only chunk (zero `pulse-app/ui/**`); the full Tauri GUI boot smoke carries the documented Windows orphan-process risk (verification-harness learning 2026-05-19) that can block builds; the non-panicking boot wiring is compile-verified (clippy `--all-targets` built the bin) and its behavior covered by the integration test. Mirrors the P-073 precedent.
