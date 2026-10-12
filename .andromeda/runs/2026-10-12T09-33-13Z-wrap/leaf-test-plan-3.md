group: test-plan-3
read: .claude/rules/verification-harness.md · 167 · «- 2026-10-10: Read a count or a verdict out of a captured ga»

- leaf: .claude/rules/verification-harness.md:22   class: omission
  leaf-text: «none within 5 s → `boot: the app did not start (no spawn record)`, exit 1;»
  source: .andromeda/test-plan.md:136 «and `boot` with no spawn record exits 1 and leaves the engine it started running»
  replace-with: «none within 5 s → `boot: the app did not start (no spawn record)`, exit 1, leaving the program it started running (pinned as it stands, not repaired; owned by the route entry `One place on a node`);»

- leaf: .claude/rules/verification-harness.md:25   class: omission
  leaf-text: «ps1 mirrored in lockstep — its live leg owed via test-plan §1 `harness-cleanup-verdict-and-boot-spawn-shell-coverage`)»
  source: .andromeda/test-plan.md:136 «Closed: (a) a pin per `cleanup` token — `clean` with no pid file and both ports refusing, `no-pid-ports-accepting`, `ports-lingering`, `app-survived` — and `clean` after a real boot»
  replace-with: «ps1 mirrored in lockstep — its live leg owed via test-plan §1 `harness-cleanup-verdict-and-boot-spawn-shell-coverage`, item (c), owned by the route entry `Other operating systems retired from the code`; each sh token, and `clean` after a real boot, pinned by `xtask/tests/agent_run_shell.rs` since chunk 2026-10-10-engine-harness-proofs-completed, where an unreaped child reads `app-survived` though `harness:status` reads such a pid as dead, pinned as it stands)»
