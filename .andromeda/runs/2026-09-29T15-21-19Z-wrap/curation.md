# Curation — 2026-09-29-p-025-hue-shift-observable-made-gradable (wrap P3)

Scope: this window's conversation + the report's *Decisions & corrections* (the P1 window's conversation is gone —
resumed wrap; a correction only that window held is not curated here).

CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):
    + security.md: "CI's Node/npm MAJOR must match the host that writes package-lock.json — npm 10 vs 11 disagree on unmet optional peers"
      Proof: CI red round fixed by commit `edfd8b3` (setup-node 22 → 24); report §Dev-tool versions + §Decisions sweep hazards. Signals: measured +0.4 · detail +0.2 · load-bearing +0.2 = 0.8.
    + testing.md: "`cargo llvm-cov` function % counts each compiled copy of a function"
      Proof: the coverage gate read function 83.95 % on CI run e904cb1's lcov (report §Counts); report §Decisions sweep hazards. Signals: measured +0.4 · detail +0.2 · no-other-home +0.2 = 0.8.
    + frontend.md: "under vitest 4, `vi.restoreAllMocks()` no longer clears a `vi.fn()`'s call history"
      Proof: the vitest 3.2.7 → 4.1.11 upgrade (report §Dependencies) broke call-count assertions, fixed in `use-findings.test.tsx` (scope-record widening); report §Decisions sweep hazards. Signals: measured +0.4 · detail +0.2 · no-other-home +0.2 = 0.8.
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 1 dup (cargo audit reads `$CARGO_HOME`'s advisory-db — now in the security.md body via the cascade) ·
    0 task-specific · 0 conflict · 2 at 0.6 rejected (zombie pid reads alive under `ps`/`kill -0` — amended into
    test-plan §3 this wrap; `grep | sort` under `pipefail` dies on an empty stream — amended into test-plan §1 this
    wrap) · 2 deferred (→ handoff, max-3 cap)
  Load-bearing: "CI's Node/npm major must match the lock-writing host" → CI wall time and round-trips
  No-other-home: "llvm-cov function % counts each compiled copy"; "vitest 4 restoreAllMocks keeps vi.fn() history"
  CLAUDE.md size: 154/200 · T1 28.8 KB, 15 over 600 B

Deferred (max-3 cap, confidence 0.8 each):
- macOS `SystemTime` ticks in whole µs, so a wall clock is never a uniqueness source — mint with a process counter.
- Windows builds embed the `.ico`, so a palette (non-RGBA) PNG icon fails only on macOS/Linux `generate_context!`.
