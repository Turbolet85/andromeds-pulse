# Session Handoff

**Last Updated:** 2026-10-04T18:08:00Z
**Branch:** chore/migrate-pulse-to-v3 · 0 ahead of origin/chore/migrate-pulse-to-v3 as read at this wrap's Setup (the chunk commit follows it, then the push)
**Status:** clean
**Last Commit:** 2026-10-04-l4-hardware-probe-finds-cuda-on-arch-layout-hosts — chunk wrap (the L4 hardware probe finds CUDA on Arch-layout hosts)

## Position
- **Done:** `2026-10-04-l4-hardware-probe-finds-cuda-on-arch-layout-hosts`.
  - The Linux CUDA probe checks `libcuda.so` / `libcuda.so.1` under `/usr/lib/x86_64-linux-gnu`, `/usr/local/cuda/lib64`, `/usr/lib` and `/usr/lib64` (8 candidates, a fixed-path presence check behind a root-taking seam).
  - This host now reads GPU-present: `l4_decision_probe` prints `…_LLAMA_CUDA_BIN_PATH is unset` (was `…_CPU_…`).
  - CI ci#37220563721 on `7fc5fa2` is green (13/13).
- **Next:** "L4 interpretation names its triggering cue" — now `BLOCKED-ON:` a Linux llama.cpp CUDA binary (none on this host; a founder desk act). The GGUF model is restored (`AI-Model/`, sha256 matches the 2026-05-24 spike record). Phase may take the head up and fold the block into scope, or skip to the next doable entry. Then, in order:
  - "The declared Rust floor matches the code" (carries the skip-arm lock-file CARRY);
  - "pre-push:linux runs natively on Linux" (three CARRY blocks; closes Epoch 4).

## Work done
- 4 source files + the derived lock line. Workspace tests 2601 → 2614 (+13 `cuda_probe_` cases).
- Red-before-green: 8/13 failed on the base set. Mutation checks: (a) 4 RED · (b) 5 RED · (c) clippy dead-code RED — the arm-wiring guard holds (`evidence/`).
- Operator pass: hygiene, six native stages, the regen and base close after stage 5, the pre-CI commit, push `03fb097..7fc5fa2`, then CI green.

## Drift resolved
- **Amendments:** 3, all the plan's expected entries:
  - architecture §Occupied Resources → env vars registers `ANDROMEDA_PULSE_HARDWARE_PROFILE`;
  - security-plan §Input Validation's CLI / env var row lists the var and its closed parse;
  - test-plan §4 interpretation bullet gains the probe pins.
- **Escalations:** 0. Four docs returned `proposals: []`.
- **Leaves re-derived:**
  - `docs/services/interpretation.md`: the "dlopen-shaped" claim (disproved), a stale bare-key claim, the guard location, Dependencies;
  - `docs/security-summary.md` item 8 (`.andromeda/runs/2026-10-04T17-56-08Z-wrap/`).

## Notes
- **Walker-mark decision (overseer, founder-delegated, 2026-10-04):** `pulse-app/tests/unit_observability_allowlist_sweep.rs` stays unmarked. This is by the mark's design, not a deferral: the sweep reads `.rs` only, so Rust delta already voids the right gates.
- **Playbook rule proposed (awaiting approval):** registering a PRE-EXISTING product-consumed env var as registry completeness. `playbook.md:20` requires "a var the chunk added", so today only a plan's expected-amendment entry settles it.
- **Observation, not actioned:** obs-plan §8 has no row for `interpretation.hardware.detect`, although `observability.rs` registers its exact leaf (pre-existing).
- **Ports:** 4317/4318 are shared with conductor-builder. Ask the operator for the slot before any run that launches pulse-app, a window or the model.
- **Host:** Omarchy Linux.
  - `grep` is ugrep: a bounded-context `.{0,N}` pattern fails silently, and `grep -c` on a binary prints nothing (use python).
  - A PreToolUse hook blocks `cat >> file <<EOF`; use the Write/Edit tools.
- **Flycheck:** stop rust-analyzer's `cargo check` tree before heavy cargo steps. Select it by `comm == cargo` + `--message-format=json`; never `pgrep -f`. None was running at any check this session.
- **Founder rulings:** record by name and date, relayed by the pc overseer. Sidecars name the ruling and never quote it.
- **Epoch 4** is at 58 entries, and the no-split ruling holds. It closes at the wrap that completes "pre-push:linux runs natively on Linux", with the sidecar consolidation and the diagnose nudge then.
- **Hand-run pre-push:**
  - the `--features mcp-server` bindings regen must follow stage 5 (it clobbered again this chunk; the re-fired regen restored it);
  - stage 3 needs a fresh `PUPPETEER_CACHE_DIR` under `target/`;
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
- **Still open from prior wraps:**
  - `recurrence-despite-learning` (run-dir hygiene trip): a CHECK in the operator pass's hygiene step; it held this chunk (hygiene clean twice);
  - `recurrence-despite-learning` (bindings clobber): the operator-pass CHECK; it held again this chunk;
  - a writer census at the wrong layer;
  - targeted nextest `timeout` sizing from a measured cold build;
  - the `producer | grep -q` under pipefail plan-authoring CHECK;
  - the implement report-step CHECK;
  - the bindings-regen PIPELINE half;
  - macOS `SystemTime` µs ticks;
  - Windows `.ico` vs palette PNG;
  - the deferral-destination generalization;
  - `inject_demo --sustained` cannot form an incident.
