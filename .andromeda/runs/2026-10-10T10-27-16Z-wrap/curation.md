CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):
    + testing.md: "a job log echoes each step's script, so a token count over job logs counts the script's own lines" (confidence 0.8)
      Proof: the plan's six-logs probe read `6 9` on `ci#38038281709`; three of the nine lines were the coverage step's own script text (its "trivially passes" echo and its `Branch:` echo) beside one output line, and two were the `mcp-test` step's echoed command. The same probe read `6 0` on `ci#38042949735` only because the tokens left the script text too (evidence/operator-pass.md, entry 30).
    + testing.md: "a fixture holding the ignore attribute must not start a source line with it under the trees the quarantine check scans" (confidence 0.8)
      Proof: the check greps `^[[:space:]]*#\[ignore` over `.rs` files under its four search dirs (xtask/ci/quarantine-tracking-check.sh); the five quarantine pins live in `xtask/src/main.rs`, inside that scope, and write their fixture as one-line string literals for that reason; `cargo xtask quarantine-tracking` read `PASS (0 quarantine(s) across 312 file(s))` with them in the tree (gate entry, /implement).
    ~ verification-harness.md, the 2026-05-14 entry: CORRECTED in place (exempt from the cap)
      Proof: the entry said a fresh empty data dir makes `cargo xtask ci-gates` "return NEUTRAL on empty logs"; the gate entry `ANDROMEDA_PULSE_DATA_DIR="$(mktemp -d)/absent" cargo xtask ci-gates` read exit 2 with `::error::ci-gates: cannot-evaluate (no agent-latest.jsonl* file in the log dir)` (/implement's gate block and this wrap's light gate).
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 2 dup · 2 below the threshold · 0 conflict · 0 deferred
    dup: the mutation rule (a one-line removal does not restore a defect a fix holds in two places) held as written in rules/testing.md, 2026-10-10 — applied, not a recurrence; "an imprecise report bullet propagates into proposals" is the report template's own rule.
    below the threshold: `cargo xtask test` takes nextest arguments only after `--` (one event, 0.3); a looped pin stops at its first failing case (0.4).
  No-other-home: both Tier 2 entries (each scored exactly 0.6 before the signal; neither fact is on the route, in a master, or in a playbook rule).
  Not curated, stated: the four stale line citations in preserve-verbatim homes (rules/frontend.md:99, rules/security.md:160, rules/observability.md:148 twice) each cite a subject this chunk did not make false; their numbers were stale before it and stay on the handoff, by the operator's earlier word.
