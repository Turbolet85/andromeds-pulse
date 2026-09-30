# Curation — wrap 2026-09-30T06-23-56Z (2026-09-29-scrubber-path-false-positive)

**Tier 1** (CLAUDE.md `USER:session-learnings`): none.

**Tier 2** (`.claude/rules/*`):
- `testing.md`, NEW: "A test input echoed into a committed evidence log can trip the run-dir hygiene check." Score 0.8: verified by a real gate refusal +0.4, specific technical detail +0.2, no-other-home +0.2.
  - Proof: `gate.py hygiene` refused `evidence/red-before.log:88` and `evidence/mutation-a-luhn-neutralized.log:45` (P1 drive) on the synthetic `case_3` literal. It read `hygiene: clean` after the placeholder replacement (operator pass, `evidence/operator-pass.md` §11).
- `security.md`, CORRECTION (in place, `[corrected 2026-09-30]`; exempt from the cap): the 2026-08-23 composed-form entry said "per the catalog's own recall-over-precision posture". It now reads "per the key-anchored arms' …".
  - Proof: `crates/security/src/scrubber.rs` `credit_card` arm (Luhn over whole-group windows); `scrubber_allows_non_card_digit_runs` 7/7; mutation (a) reds exactly those 7 (`evidence/mutation-check.md`).
- `security.md`, EXTENDED: the 2026-08-30 "`npm audit`'s `fixAvailable` is resolver output" entry gained the targeted `npm update {pkg}` form for an in-range transitive fix. Score 0.8: verified +0.4, detail +0.2, no-other-home +0.2 (the technique is on no route and in no master; the master records only the bumps).
  - Proof: `npm view` listed brace-expansion 1.1.21 / 5.0.12 and ip-address 10.7.2. `npm update brace-expansion ip-address` changed exactly three lockfile entries (`git diff --stat`: 9+/9-, `package.json` untouched). `check:npm-supply-chain` then read `green-with-dispositions`, and ci#36675962820 went 13/13.

**Tier 3**: none.

**Filters:** 2 dup (the Luhn precision rule, already written into the rules body by this wrap's cascade; the bindings clobber, see below) · 0 task-specific · 0 conflict · 0 deferred.

**Recurrence-despite-learning** (handoff): the bindings-regen family (`testing.md` 2026-05-13 / 2026-05-17 / 2026-08-15; `security.md` 2026-06-12). /implement's `capability-drift` read the worktree `bindings/index.ts` right after the plan's own default-features workspace nextest, which the plan's gate order places ahead of it, and went red on 3 missing `mcp.*` procedures. The corpus states the remedy correctly; the plan's gate order still reproduces the failure every chunk.

Load-bearing: none. No-other-home: "evidence-log hygiene hit on echoed test inputs"; "targeted npm update for in-range transitive fixes".
