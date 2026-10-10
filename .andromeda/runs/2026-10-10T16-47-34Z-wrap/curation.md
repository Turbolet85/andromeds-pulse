# Curation — 2026-10-10-agent-harness-drives-the-console-engine (wrap, second window)

Scope, stated: this wrap ran in two windows on the operator's directive (inputs#I4 item 8). The first window's
conversation (implement, the operator pass, the report) was cleared before this one, so a correction only that
conversation held is not curated here. The candidates are this window's own and the report's Decisions &
corrections section.

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):
    + verification-harness.md: "Every arm line of `cargo xtask check:engine-log` keeps the arm's name between the prefix and the state word … a gate atom `contains engine-log: PASS` is met by the verdict line alone"
      Proof: the report's Sweep hazards, third bullet; the gate entry's atom in plan.md's Test Commands; the line shape at `xtask/src/engine_log.rs` 111-118 (`fn line`, New text listing) and the verdict lines at 131-142.
    + verification-harness.md: "Read a count or a verdict out of a captured gate log by its own summary, never by a line grep …"
      Proof: the report's Sweep hazards, first and fourth bullets: `grep -c 'PASS .*{module}::'` read 90 for 45 tests in the gate tool's capture; a grep for `"name"` / `"ok"` over the pre-push log matched 850 kB of libtest-JSON lines.
  Tier 3 (.claude/docs/session-learnings.md): none
  Extended: T2/security.md: "2026-06-11 (session 183): CORRECTION to the §Tauri capability gating body block" + "the body bullet it corrects was recomputed from security-plan at this wrap; the two now agree"
      Proof: `.claude/rules/security.md` line 28 as rewritten at this wrap from security-plan §API Security (`staged_gate::EXPECTED_GRANTS`, six files, both directions) and §Security Anti-Patterns → API; the entry's own text had said the body "remains as-written until the next full re-derive".
  Filters: 0 dup · 1 task-specific (clippy's `manual_contains` on a whole-line compare: a lint's own message, not a project rule) · 0 conflict · 0 deferred
  Counts: T1 0 · T2 3 writes (2 new, 1 extension) · T3 0

Not curated, surfaced (preserve-verbatim homes the citation sweep listed; handoff):
- `.claude/rules/frontend.md:99` (2026-05-09): cites `xtask/src/main.rs:550` for `format!("{router}.{method}")`.
  That literal is in no file under `xtask/src` today (`grep -rn` at this wrap: 0 hits), so the claim was not
  re-verified and the entry was not corrected: a correction needs the measurement of what the parser does now.
- `.claude/rules/observability.md:149` (2026-08-21): `:1078`, `:1934` name lines of a dated measurement of bare
  allowlist keys, one of which (`interpretation`) was removed since; a historical reading, left as written.
- `.claude/rules/security.md:160` (2026-06-11): the entry's own extension already says the cited number is stale.
