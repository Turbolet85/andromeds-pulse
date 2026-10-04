# Session Handoff

**Last Updated:** 2026-10-04T21:17:00Z
**Branch:** chore/migrate-pulse-to-v3 · 0 ahead of origin/chore/migrate-pulse-to-v3 as read at this wrap's Setup (the chunk commit follows it, then the push)
**Status:** clean
**Last Commit:** 2026-10-04-l4-interpretation-names-its-triggering-cue — chunk wrap (the digest names its trigger and frames corpus matches; the prompt tells the model to describe the trigger)

## Position
- **Done:** `2026-10-04-l4-interpretation-names-its-triggering-cue`, flipped `complete` for its doable part.
  - The digest renders `TRIGGER: {cue_cause_label}` (first cue) and a framing note under `CORPUS MATCHES:`.
  - `TRIGGER_FRAMING_INSTRUCTION` sits in all three tiers; prompt lineage is v2.4 / v1.3-fallback / v1.3-reflection.
  - The probe gained S4, the `nf` arm, `names_trigger`, `--min-rank1`, and `test = true` with 8 pins.
  - CI ci#37232849843 on `126788c` is green (13/13).
- **Next:** "L4 framing measured on the real model" (minted at this wrap on the operator's word). It is
  `BLOCKED-ON:` a Linux llama.cpp CUDA binary, re-measured STANDING at 21:14Z.
  - It carries the pre-registered series: `shipped --min-rank1 36` (rank-1 names the retry in ≥ 36/40), then `nf`
    record-only. Never recorded as passed.
  - Conductor's fourth v3-09 series waits on it.
  - Phase may take it up and fold the block, or skip to the next doable entry. Then:
    - "The declared Rust floor matches the code" (carries the skip-arm lock-file CARRY);
    - "pre-push:linux runs natively on Linux" (three CARRY blocks; closes Epoch 4).

## Work done
- 9 source files (6 listed + 3 recorded companions); workspace tests 2614 → 2628 (+14).
- Red-before-green: 5 of 7 pins red. Mutations: 4 of 4 red, with (d) reddening only the rank1 pin
  (`evidence/`).
- Probe A5 is now the identity: it had panicked since prompt v2.3.
- Operator pass: hygiene, the six native stages, the regen and base close after stage 5, the pre-CI commit,
  push `5d6e344..126788c`, CI green.

## Drift resolved
- **Amendments:** 6, all the plan's expected entries:
  - arch [Fault Identity]: the "own route entry" clause now records the landed framing, effect unmeasured;
  - security-plan ×2: the observed maximum is re-based from 6,932 to 7,185 B (~2.28×, ≈ 9.0 KiB), in lockstep;
  - test-plan §1: the probe trigger is partially discharged;
  - test-plan §4: the interpretation lineage and framing pin, and the triage digest framing pins.
- **Escalations:** 0. The two D-security-input proposals were routine by actual class (playbook
  escalate-outside-class rule).
- **Leaves re-derived:**
  - CLAUDE.md warnings (Fault Identity: the TRIGGER line is keyed on the identity cue);
  - `docs/services/triage.md` (the stale "digest/ empty skeleton" line recomputed; the dev-probe re-exports).

## Notes
- **Gated-arm gap (P5 halt, resolved).** Phase took the blocked head up "gated", but the plan claimed no matrix
  cap. The wrap's gated arm needs one: `route.py flip --to gated` and `matrix.py flip --gated` both refuse.
  - The operator (overseer, founder-delegated, 2026-10-04) chose `complete` + a new owned route entry. This
    realizes the founder ruling of 2026-10-02 (nothing moves to 0.4.0).
  - Recorded as a `contract.jointly-contradictory-instructions` evolve record, for the pipeline owners.
- **Conductor relay:** the moved `assembler.rs` coordinates (+20 inside `render_payload`, +13 at its head, +1 at
  the cue-ref build) are in this chunk's `report.md` → Cross-project claims.
- **Playbook rule proposed at the prior wrap (still awaiting approval):** registering a PRE-EXISTING
  product-consumed env var as registry completeness (`playbook.md:20` requires "a var the chunk added").
- **Observation, not actioned:** obs-plan §8 has no row for `interpretation.hardware.detect` (pre-existing).
- **Ports:** 4317/4318 are shared with conductor-builder. Ask the operator for the slot before any run that
  launches pulse-app, a window or the model.
- **Host:** Omarchy Linux.
  - `grep` is ugrep: a bounded-context `.{0,N}` pattern fails silently, and `grep -c` on a binary prints nothing
    (use python).
  - A PreToolUse hook blocks `cat >> file <<EOF`; use the Write/Edit tools.
- **Flycheck:** stop rust-analyzer's `cargo check` tree before heavy cargo steps. Select it by `comm == cargo` +
  `--message-format=json`; never `pgrep -f`. None was running at any check this session.
- **Founder rulings:** record by name and date, relayed by the pc overseer. Sidecars name the ruling and never
  quote it.
- **Epoch 4** is at 59 entries; the no-split ruling holds. It closes at the wrap that completes "pre-push:linux
  runs natively on Linux".
- **Hand-run pre-push:**
  - the `--features mcp-server` bindings regen must follow stage 5 (it clobbered again this chunk; the regen
    restored it);
  - stage 3 needs a fresh `PUPPETEER_CACHE_DIR` under `target/pre-push/`;
  - stage 5 under `env -i` runs no credential-store leg.
- **npm audit:** stage 3 still prints `10 high severity vulnerabilities`; CI's `supply-chain` job is green.
- **Test residue:** two empty test lock files in `/tmp` from the skip arm, owned by the Rust-floor entry's CARRY.
- **Pre-existing tool verdicts, not this chunk's:**
  - `route.py` UNPARSED/INDETERMINATE on frozen lines;
  - `registry.py contracts` NOT MIGRATED (arch · tests · obs · a11y);
  - matrix `P-072` UNPARSED (legacy notes placement).
- **Not this wrap (founder's hand):** the `.gitattributes` re-checkout and the U35 door. PR #39 stays a draft.
- **Still open:** the `sidecar.py` Ref defect relayed to overseer1 at session 65.
- **Last failed command:** none.

## Deferred learnings
- **New this wrap:**
  - `recurrence-despite-learning` (mutation applied?): testing.md 2026-08-17 [extended 2026-08-23]. The Edit
    errored loudly, but a test run in the same parallel batch read green on the unmutated tree. Extended in place;
    the remedy is a CHECK in implement's mutation step;
  - `recurrence-despite-learning` (sweep hazard): testing.md 2026-09-30. Research grepped the quoted literal and
    missed 3 unquoted `prompt_version=` pins. Extended in place; the remedy is a CHECK in phase research's sweep.
- **Still open from prior wraps:**
  - `recurrence-despite-learning` (run-dir hygiene trip): the operator-pass hygiene CHECK held this chunk;
  - `recurrence-despite-learning` (bindings clobber): the operator-pass CHECK held again;
  - a writer census at the wrong layer;
  - targeted nextest `timeout` sizing from a measured cold build;
  - the `producer | grep -q` under pipefail plan-authoring CHECK;
  - the implement report-step CHECK;
  - the bindings-regen PIPELINE half;
  - macOS `SystemTime` µs ticks;
  - Windows `.ico` vs palette PNG;
  - the deferral-destination generalization;
  - `inject_demo --sustained` cannot form an incident.
