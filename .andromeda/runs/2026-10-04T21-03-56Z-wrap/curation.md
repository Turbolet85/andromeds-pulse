CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   2 in-place extensions (below)
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 3 dup · 2 task-specific/below-threshold · 0 conflict · 0 deferred
  No-other-home: "grep the bare value with a word boundary, never its quoted form" · "never fire the mutation Edit and its test run in one parallel batch"
  Extended: T2/testing.md: "2026-09-30: SWEEP HAZARD — … grepping the test trees for the OLD VALUE's literal" + "grep the bare value with a word boundary, never its quoted form"
    Proof: research §Sweeps' `grep -rn '"v2\.3"\|"v1\.2-fallback"\|"v1\.2-reflection"'` reported 5 hits; the bare-token grep found 3 more, at `pulse-app/tests/unit_inference_runtime.rs:539/561/628` (`"prompt_version=v2.3"` etc.). Without them the workspace nextest went red until they were moved (report.md Spec claims disproved; scope-record companion line). Score 0.8: measured 0.4 + technical 0.2 + no-other-home 0.2 (not on the route, not in a master, not a matrix note).
  Extended: T2/testing.md: "2026-08-17: … MUTATION CHECK … [extended 2026-08-23] VERIFY THE MUTATION ACTUALLY APPLIED" + "never fire the mutation Edit and its test run in one parallel batch"
    Proof: implement Step 12(d) — the anchored Edit returned `String to replace not found` (the formatter had reflowed `names_trigger`). The test run fired in the same batch printed 2 PASS on the unmutated tree. Re-read, re-applied, grep-confirmed, then RED (`evidence/mutation-checks.md`). Also logged as `recurrence-despite-learning` (the matched entry records the defect). Score 0.8: measured 0.4 + technical 0.2 + no-other-home 0.2.
  CLAUDE.md size: 154/200 · T1 28.8 KB, 15 over 600 B (health check 1, pre-existing; operator promotes)
  Rejected:
    - "an every-arm composition pin surfaces a dormant panic in a dev-probe arm keyed on document structure" — 0.6 (measured 0.4 + technical 0.2); no-other-home does not fire, because test-plan §1 now carries the A5 identity (amended this wrap) → reject at exactly 0.6
    - "conditional-pin asymmetry: the elsewhere pin cannot discriminate title-first" — duplicate of testing.md 2026-08-17 [extended 2026-08-23] (the positive half of a conditional property never discriminates)
    - "re-derive a coordinate shift by grep, never from remembered edit sizes" — duplicate of CLAUDE.md 2026-08-21 verify-at-HEAD (a cited coordinate is a pointer)
    - "the PreToolUse hook blocks cat heredoc appends" — duplicate (the handoff Notes carry it)
    - "rstest is not a pulse-app dev-dependency; adding it moves Cargo.lock" — task-specific, one-off (−0.3)
