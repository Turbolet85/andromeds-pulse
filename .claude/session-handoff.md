# Session Handoff

**Last Updated:** 2026-10-04T09:31:55Z
**Branch:** chore/migrate-pulse-to-v3 · 0 ahead of origin/chore/migrate-pulse-to-v3 as read at this wrap's Setup (the chunk commit follows it, then the push)
**Status:** clean
**Last Commit:** 2026-10-04-corpus-key-creation-is-race-free — chunk wrap (corpus key creation is a locked create-or-read; concurrent first-run processes converge on one key)

## Position
- Done: `2026-10-04-corpus-key-creation-is-race-free`.
  - Only `crates/corpus/src/keychain.rs` changed; witness: 8 processes → 1 key (8 distinct at base, mutation-checked).
  - CI `ci#37189514735` on `a073722`: attempt 1 red on boot smoke (ubuntu-22.04), the re-run green — now a WATCH.
- **Next: "Linux launch stays up on NVIDIA + Wayland"** (measure the cause first; carries the boot-smoke WATCH). Then, in order:
  - "Retry-storm interpretation names its cause";
  - **NEW** "The declared Rust floor matches the code" (founder ruling 2026-10-04) — raise `rust-version`, drop the clippy allow; carries the skip-arm lock-file CARRY;
  - "pre-push:linux runs natively on Linux" — now three CARRY blocks (PC22, stage ORDER, the `env -i` Secret Service skip).

## Work done
- Locked create-or-read (`std::fs::File::lock` on a content-free per-entry lock file in `$XDG_RUNTIME_DIR` / the temp dir). Workspace 2580/2580; corpus 87/87.
- Operator pass: native stages green with a fresh per-run `PUPPETEER_CACHE_DIR`; regen + base check after stage 5; pushed `76d6cca..a073722`.

## Drift resolved
20 amendments (architecture 7 · security-plan 11 · test-plan 2), 1 rejected, 1 escalation resolved (`.andromeda/runs/2026-10-04T09-16-41Z-wrap/fanout-results.md`).
- **Boundary widening** (the lock file outside the data dir + the `XDG_RUNTIME_DIR` input): applied as ratified live by the founder 2026-10-04, confirmed by the operator at this wrap; four residuals stated in the bodies.
- **Rust floor:** the masters now say pinned 1.95.0, code ≥ 1.89, declared 1.85 stale → the new route entry.
- **Leaves re-derived:** CLAUDE.md (Stack · corpus · warnings), docs stack/commands/security-summary/services/corpus, rules testing/security.

## Notes
- **Ports:** 4317/4318 are shared with conductor-builder. Ask the operator for the slot before any run that launches pulse-app, a window or the model.
- **Host:** Omarchy Linux. `grep` is ugrep: a bounded-context `.{0,N}` pattern fails silently (use python). Python `duckdb` is not installed, so the code-graph refresh reads STALE.
- **Flycheck:** stop rust-analyzer's `cargo check` tree before heavy cargo steps — select it by `comm == cargo` + `--message-format=json`; never `pgrep -f` (it matches your own shell).
- **Founder rulings:** record by name and date, relayed by the pc overseer, never as "Viola"; sidecars name the ruling, never quote it.
- **Epoch 4** is at 56 entries; the no-split ruling holds. It closes at the wrap that completes "pre-push:linux runs natively on Linux", with the sidecar consolidation and the diagnose nudge then.
- **Hand-run pre-push:** the `--features mcp-server` bindings regen must follow stage 5 (`cargo xtask test`); stage 3 needs a fresh `PUPPETEER_CACHE_DIR` under `target/`; stage 5 under `env -i` runs no credential-store leg (no session bus).
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

## Session End Status
Completed normally at 2026-10-04 13:46:31
