# Session Handoff

**Last Updated:** 2026-10-04T15:04:55Z
**Branch:** chore/migrate-pulse-to-v3 · 0 ahead of origin/chore/migrate-pulse-to-v3 as read at this wrap's Setup (the chunk commit follows it, then the push)
**Status:** clean
**Last Commit:** 2026-10-04-retry-storm-interpretation-names-its-cause — chunk wrap (an incident born of a retry storm names the retry in its title)

## Position
- **Done:** `2026-10-04-retry-storm-interpretation-names-its-cause`.
  - The producer titles every new incident `{Cause label}: {model title}`, using `triage::contract::cue_cause_label` and grounding before the scrub. It does so at creation, on the dedupe-refresh JSON and in the resolution JSON.
  - Identity, the prompt, the report projection and the bindings are unchanged.
  - CI ci#37209338065 on `2116c3c` is green.
- **Next:** "The L4 hardware probe finds CUDA on Arch-layout hosts". Then, in order:
  - **NEW** "L4 interpretation names its triggering cue" (founder ruling 2026-10-04, the prompt-framing half Conductor v3-09 needs). It needs the restored real L4 model, which is absent on this host and whose restoration is a founder desk act. Its causal premise is a hypothesis (n = 1), not measured;
  - "The declared Rust floor matches the code" (carries the skip-arm lock-file CARRY);
  - "pre-push:linux runs natively on Linux" (three CARRY blocks).

## Work done
- 4 source files changed. Workspace tests 2592 → 2601 (+1 label pin, +6 producer pins, +2 report pins).
- Red-before-green: 0/8 passed before the fix. Mutation check: 6 red / 5 green (`evidence/mutation-checks.md`).
- Operator pass: hygiene, six native stages, the regen and base close after stage 5, the pre-CI commit, push `e71dba5..2116c3c`, then CI green.

## Drift resolved
- **Amendments:** 2, both architecture, both the plan's expected entries: §Occupied Resources `ANDROMEDA_PULSE_L4_DETERMINISTIC` (a cue-grounded title in every mode) and §Established Decisions [Fault Identity] (the cue kind reaches the incident text, never its identity).
- **Escalations:** 0. Six docs returned `proposals: []`.
- **Leaves re-derived:** the CLAUDE.md warnings Fault-identity bullet, `docs/services/interpretation.md` and `docs/services/triage.md` (`.andromeda/runs/2026-10-04T14-54-42Z-wrap/`).

## Notes
- **Boot-smoke WATCH retired:** 3 green runs (ci#37200709989 · ci#37203184509 · ci#37209338065) and no recurrence since the `a073722` red.
- **npm audit (observation, not this chunk's):** stage 3's `npm ci` printed `10 high severity vulnerabilities`. No npm file changed, and CI's `supply-chain` job is green.
- **Conductor:**
  - A live deterministic harvest now reads `Retry storm: Deterministic verification incident`. The overseer measured that no live Conductor code compares the title, and Pulse edits nothing there.
  - v3-09 (the rank-1 hypothesis) is not moved; the new entry owns it.
- **Ports:** 4317/4318 are shared with conductor-builder. Ask the operator for the slot before any run that launches pulse-app, a window or the model.
- **Host:** Omarchy Linux.
  - `grep` is ugrep: a bounded-context `.{0,N}` pattern fails silently, and `grep -c` on a binary prints nothing (use python).
  - A PreToolUse hook blocks `cat >> file <<EOF`; use the Write/Edit tools.
- **Flycheck:** stop rust-analyzer's `cargo check` tree before heavy cargo steps. Select it by `comm == cargo` + `--message-format=json`; never `pgrep -f`. None was running at any check this session.
- **Founder rulings:** record by name and date, relayed by the pc overseer, never as "Viola". Sidecars name the ruling and never quote it.
- **Epoch 4** is at 58 entries, and the no-split ruling holds. It closes at the wrap that completes "pre-push:linux runs natively on Linux", with the sidecar consolidation and the diagnose nudge then.
- **Hand-run pre-push:**
  - the `--features mcp-server` bindings regen must follow stage 5;
  - stage 3 needs a fresh `PUPPETEER_CACHE_DIR` under `target/`;
  - stage 5 under `env -i` runs no credential-store leg;
  - phase's saved `gate.py run` captures trip `gate.py hygiene` (placeholder their `root` / `logs` header lines).
- **Test residue:** two empty test lock files in `/tmp` from the skip arm, owned by the Rust-floor entry's CARRY.
- **Pre-existing tool verdicts, not this chunk's:**
  - `route.py` UNPARSED/INDETERMINATE on frozen lines;
  - `registry.py contracts` NOT MIGRATED (arch · tests · obs · a11y);
  - matrix `P-072` UNPARSED (legacy notes placement);
  - architecture sidecar UNPARSED group headers `:15` / `:177`.
- **Not this wrap (founder's hand):** the `.gitattributes` re-checkout and the U35 door. PR #39 stays a draft.
- **Still open:** the `sidecar.py` Ref defect relayed to overseer1 at session 65.
- **Last failed command:** none.

## Deferred learnings
- **New this wrap:** `recurrence-despite-learning` (run-dir hygiene trip): the 2026-09-30 testing.md entry did not prevent a second trip, this time through `gate.py run` capture headers. It was extended in place; the remedy is a CHECK in the operator pass's hygiene step.
- **Still open from prior wraps:**
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

## Session End Status
Completed normally at 2026-10-04 19:16:45
