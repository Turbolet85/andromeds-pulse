# Session Handoff

**Last Updated:** 2026-10-04T22:48:00Z
**Branch:** chore/migrate-pulse-to-v3 · 0 ahead of origin/chore/migrate-pulse-to-v3 as read at this wrap's Setup (the chunk commit follows it, then the push)
**Status:** clean
**Last Commit:** 2026-10-04-declared-rust-floor-matches-the-code — chunk wrap (the declared Rust floor is 1.95, equal to the pin, witnessed; the skip arm leaves no lock file)

## Position
- **Done:** `2026-10-04-declared-rust-floor-matches-the-code`, flipped `complete`.
  - `rust-version = "1.95"`, set by the dependency graph (wasmtime / cranelift / pulley declare 1.95.0). The
    xtask test `declared_floor_equals_the_pinned_channel` holds it equal to the pin.
  - The 20 MSRV-gated clippy sites were fixed in place, with no suppression.
  - The corpus skip arm removes its own lock file; witness `corpus_key_skip_arm_leaves_no_lock_file`.
  - CI ci#37238688476 on `e9ec090` is green (13/13), including lint-test on macOS and Windows.
- **Next:** "L4 framing measured on the real model" (`working-route.md:164`). Its block is CLEARED: this wrap
  removed the BLOCKED-ON on the overseer's directive (measured 2026-10-05 ~00:40) and re-verified it read-only.
  - The binary is `~/dev/tools/llama.cpp-b9305/build/bin/llama-cli` (b9305, 63248fc, CUDA 13.3).
  - The leg env is one file: `. ~/dev/projects/additional/pc-overseer/l4-env.sh`.
  - It carries the pre-registered series: `shipped --min-rank1 36`, then `nf` record-only. Never recorded as
    passed.
  - Model slot: ports 4317/4318 are shared with conductor-builder.
  - Then: "pre-push:linux runs natively on Linux" (three CARRY blocks; it closes Epoch 4).

## Work done
- 16 source/manifest files (15 listed + 1 new); workspace tests 2628 → 2630; corpus 87 → 88.
- RED before green: `(true, 1)`. Mutations (a), (b), (c) each went RED. Readings are in `evidence/`.
- The Xvfb boot smoke was green (`posture=applied`, recorded, not asserted).
- The `/tmp` lock residue was removed and recorded. The post-stage-5 census found no lock of any name, so its
  origin is undetermined.
- Operator pass: hygiene, the six native stages, the regen and base close, pre-CI commit `e9ec090`, the push,
  CI green.

## Drift resolved
- **Amendments:** 7, all the plan's expected entries, 0 escalations.
  - arch ×4: §Stack, [Primary Language], §Inherited Defaults, and the `app_info` example `"1.95"`;
  - security-plan ×1: §Universal, the 1.85.0 Edition minimum kept with the build floor beside it;
  - test-plan ×2: §4 Framework, and the corpus row's skip-arm witness.
- **Leaves re-derived:** CLAUDE.md Stack line, `docs/stack.md`, `rules/testing.md`, `rules/security.md` §Rust
  toolchain, `docs/security-summary.md`, `docs/services/corpus.md`.

## Notes
- **Plan gate 8 is a plan defect.** Under the gate tool's pipefail, an empty `grep` exits 1, so the
  `grep … | wc -l` entry expecting exit 0 reads red exactly when its property holds. The light gate skipped it on
  the overseer's directive; its predicate (0 `incompatible_msrv`) is measured true.
- **Plan gate 10 is a plan defect too.** The scope guard's exclusion pathspec omitted the chunk's own new file
  `xtask/src/rust_floor.rs`, which the pre-CI commit made visible. It was skipped on the overseer's ruling;
  `gate.py scope` reads 16 changed · 16 listed.
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
  - the `--features mcp-server` bindings regen must follow stage 5 (it clobbered again: −22 lines, restored by
    the regen);
  - stage 3 needs a fresh `PUPPETEER_CACHE_DIR` under `target/pre-push/` (green first time this chunk);
  - stage 5 under `env -i` runs no credential-store leg;
  - re-run hygiene after any evidence edit.
- **npm audit:** stage 3 still prints `10 high severity vulnerabilities`; CI's `supply-chain` job is green.
- **Pre-existing tool verdicts, not this chunk's:**
  - `route.py` UNPARSED/INDETERMINATE on frozen lines;
  - `registry.py contracts` NOT MIGRATED (arch · tests · obs · a11y);
  - matrix `P-072` UNPARSED (legacy notes placement).
- **Still open, carried:**
  - Playbook rule proposed at an earlier wrap, still awaiting approval: registering a PRE-EXISTING
    product-consumed env var as registry completeness.
  - obs-plan §8 has no row for `interpretation.hardware.detect`.
  - The `contract.jointly-contradictory-instructions` evolve record, for the pipeline owners.
  - The `sidecar.py` Ref defect relayed to overseer1 at session 65.
- **Not this wrap (founder's hand):** the `.gitattributes` re-checkout and the U35 door. PR #39 stays a draft.
- **Last failed command:** none.

## Deferred learnings
- **New this wrap:**
  - `recurrence-despite-learning` (pipefail pipeline exit): testing.md 2026-06-05 [extended 2026-08-30]. A plan
    gate again mis-predicted `grep | wc -l` under pipefail. Extended in place; the open plan-authoring CHECK below
    is its remedy.
  - `recurrence-despite-learning` (host path in committed evidence): testing.md 2026-09-30. A hand-written
    temp-path transcript tripped hygiene. Extended in place.
  - `recurrence-despite-learning` (scope guard omits new files): testing.md 2026-10-04 (diff-shaped probes). The
    plan's scope-guard pathspec omitted the chunk's new file, so it went red after the pre-CI commit. The remedy is
    a CHECK in phase's Test Commands authoring.
  - `recurrence-despite-learning` (clean-skip invisible): testing.md 2026-10-04. The residue record first stated
    stage 5's skip arm as observed, then was corrected to an inference.
- **Still open from prior wraps:**
  - the `producer | grep -q` under pipefail plan-authoring CHECK (recurred, above);
  - mutation applied? (implement's mutation-step CHECK; held this chunk);
  - the sweep hazard (phase research's bare-value CHECK);
  - run-dir hygiene trip (operator-pass CHECK);
  - bindings clobber (operator-pass CHECK; held);
  - a writer census at the wrong layer;
  - targeted nextest `timeout` sizing from a measured cold build;
  - the implement report-step CHECK;
  - the bindings-regen PIPELINE half;
  - macOS `SystemTime` µs ticks;
  - Windows `.ico` vs palette PNG;
  - the deferral-destination generalization;
  - `inject_demo --sustained` cannot form an incident.
