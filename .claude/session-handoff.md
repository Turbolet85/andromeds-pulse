# Session Handoff

**Last Updated:** 2026-10-04T12:42:41Z
**Branch:** chore/migrate-pulse-to-v3 · 0 ahead of origin/chore/migrate-pulse-to-v3 as read at this wrap's Setup (the chunk commit follows it, then the push)
**Status:** clean
**Last Commit:** 2026-10-04-linux-launch-stays-up-on-nvidia-wayland — chunk wrap (the default Linux launch survives on NVIDIA + native Wayland)

## Position
- Done: `2026-10-04-linux-launch-stays-up-on-nvidia-wayland`.
  - On Linux, `main()`'s first statement sets `__NV_DISABLE_EXPLICIT_SYNC=1` when unset (a preset is honoured); one boot record `app.boot.render.posture`.
  - Live legs on the dev display: GREEN alive 60 s (base died 3/3 at ~2 s); PRESET `=0` died as designed. CI `ci#37200709989` on `2099998` green.
- **Next: "Retry-storm interpretation names its cause"** (measure first; now carries the boot-smoke WATCH, 1 of 3). Then, in order:
  - **NEW** "The L4 hardware probe finds CUDA on Arch-layout hosts" (the `libcuda.so` side finding, placed on the overseer's proposal — the founder may move it);
  - "The declared Rust floor matches the code" (carries the skip-arm lock-file CARRY);
  - "pre-push:linux runs natively on Linux" (three CARRY blocks: PC22, stage ORDER, the `env -i` Secret Service skip).

## Work done
- New `pulse-app/src/render_posture.rs` + two test files (12 tests, re-exec `set_var` witness, exact-leaf guard); workspace 2592/2592; 3 mutation checks RED as predicted.
- Operator pass: hygiene (two phase-run `.rs` controls renamed `.rs.txt`), six native pre-push stages green, regen + base check after stage 5, pushed `ffb62f0..2099998`.

## Drift resolved
8 amendments (architecture 2 · security-plan 2 · obs-plan 2 · test-plan 2), 2 detector proposals rejected and re-raised by the orchestrator, 1 escalation resolved (`.andromeda/runs/2026-10-04T12-30-22Z-wrap/fanout-results.md`).
- **Boundary widening** (a presence-read, product-set system env var): applied as the founder's live ratification of 2026-10-04, confirmed by the overseer at this wrap; residual (a preset `0` still dies) stated in the bodies.
- **New test-plan trigger:** `render-posture-main-placement-coverage` — `main()`'s order is proven only by the gate-time probe and the live legs.
- **Leaves re-derived:** CLAUDE.md (`pulse-app` line), docs security-summary/obs-summary, rules observability/testing.

## Notes
- **Ports:** 4317/4318 are shared with conductor-builder. Ask the operator for the slot before any run that launches pulse-app, a window or the model.
- **Host:** Omarchy Linux. `grep` is ugrep: a bounded-context `.{0,N}` pattern fails silently, and `grep -c` on a binary prints nothing (use python). The code-graph refresh builds again (rust + ts, this wrap).
- **Flycheck:** stop rust-analyzer's `cargo check` tree before heavy cargo steps — select it by `comm == cargo` + `--message-format=json`; never `pgrep -f` (it matches your own shell).
- **Founder rulings:** record by name and date, relayed by the pc overseer, never as "Viola"; sidecars name the ruling, never quote it.
- **Epoch 4** is at 57 entries; the no-split ruling holds. It closes at the wrap that completes "pre-push:linux runs natively on Linux", with the sidecar consolidation and the diagnose nudge then.
- **Hand-run pre-push:** the `--features mcp-server` bindings regen must follow stage 5 (`cargo xtask test`); stage 3 needs a fresh `PUPPETEER_CACHE_DIR` under `target/`; stage 5 under `env -i` runs no credential-store leg (no session bus).
- **Mutation checks:** a workspace `cargo nextest` builds the bin target too — rebuild the binary a live leg launches after reverting a mutation (done this chunk; byte-string check confirmed).
- **Test residue:** two empty test lock files in `/tmp` from the skip arm (owned by the Rust-floor entry's CARRY); not deleted — unidentifiable without a BLAKE3 tool.
- **Pre-existing tool verdicts, not this chunk's:** `route.py` UNPARSED/INDETERMINATE on frozen lines; `registry.py contracts` NOT MIGRATED (arch · tests · obs · a11y); matrix `P-072` UNPARSED (legacy notes placement).
- **Not this wrap (founder's hand):** the `.gitattributes` re-checkout and the U35 door. PR #39 stays a draft.
- **Still open:** the `sidecar.py` Ref defect relayed to overseer1 at session 65.
- **Last failed command:** none.

## Deferred learnings
- Still open from prior wraps:
  - `recurrence-despite-learning` (bindings clobber) — the operator-pass CHECK (stat `bindings/index.ts` or run `check:staged-artifacts` before the push); held this chunk;
  - writer census at the wrong layer (grep the trait-level persist call in phase research);
  - targeted nextest `timeout` sizing from a measured cold build;
  - the `producer | grep -q` under pipefail plan-authoring CHECK;
  - the implement report-step CHECK (unit-only claims vs a longer live run);
  - the bindings-regen PIPELINE half;
  - macOS `SystemTime` µs ticks;
  - Windows `.ico` vs palette PNG;
  - the deferral-destination generalization;
  - `inject_demo --sustained` cannot form an incident.
