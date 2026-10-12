# Curation — 2026-10-10-shared-telemetry-test-data

Scope: this wrap ran in two windows. The first window's conversation (the implement run, the operator pass and
Phase 1) was cleared before curation ran; what it held reached curation only through the report's "Decisions &
corrections" section, which the operator's directive asked to be written for this purpose (inputs#I8 item 2). The
second window's conversation was scanned whole.

```
CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):                   + host-linux.md: "This host's user git config sets diff.mnemonicPrefix … a probe that parses diff headers pins the two prefix flags, and the host's git config is never changed to make a probe pass" (confidence 1.0)
                                              + testing.md: extension of the 2026-10-10 YAML-scalar sweep-hazard entry (confidence 0.8)
  Tier 3 (.claude/docs/session-learnings.md): + "A gate entry that is wrong by its own text is corrected by an operator edit of the plan between runs" (confidence 0.8)
  Filters: 1 dup · 2 task-specific · 0 conflict (→ handoff) · 1 deferred (→ handoff) · 3 below threshold · 3 recurrence-despite-learning (→ handoff)
  No-other-home: "a probe's known-positive control runs in the shape the probe itself runs in" · "a gate entry wrong by its own text is corrected by an operator edit of the plan between runs"
  Extended: T2/testing.md: "2026-10-10: SWEEP HAZARD — an unquoted YAML scalar ends at ` #` … an atom of a new plan probe … cannot be vouched for by a baseline taken on the untouched tree" + "a probe's known-positive control runs in the shape the probe itself runs in"
```

## Applied

1. **Tier 2, `.claude/rules/host-linux.md`, new entry** — the host's diff prefixes.
   Signals: explicit operator correction (+0.4) · verified by measurement, a real gate failure (+0.4) · specific
   technical detail (+0.2) = 1.0. The file has no `paths:`, so the entry was held to one sentence (389 B).
   Proof: gate entry 4 of the chunk's plan read `red · exit 0 ✗ (exit 1)` at its first firing because its awk
   takes the file name from `+++ b/{path}` and a commit-to-work-tree diff prints `+++ w/{path}` on this host
   (`evidence/test-module-guard.md`); `git config --show-origin --get diff.mnemonicPrefix` reads true in the
   user-level config; after the operator's edit pinned `--src-prefix=a/ --dst-prefix=b/` the entry read green
   (`evidence/operator-edit-plan.md`, firing 2). The operator's word: inputs#I4 ("set no git config").

2. **Tier 2, `.claude/rules/testing.md`, in-place extension** of the 2026-10-10 entry that ends "cannot be
   vouched for by a baseline taken on the untouched tree".
   Signals: verified by measurement (+0.4) · specific technical detail (+0.2) = 0.6, then "reached no other
   durable home this wrap" (+0.2) = 0.8. Filter 1 read it as an additive facet (the control side of the same
   rule), so it is one sentence tagged `Extended 2026-10-10`.
   Proof: entry 4's control was a commit-to-commit diff and its baseline an empty diff; neither printed the
   header form the entry reads when it runs (the report, "Spec claims disproved by measurement"). After the edit
   the control was re-run as a commit against the work tree, `514fbe57` against the tree: 25 lines, each with its
   file name (the report, "Sweep hazards and host facts found").

3. **Tier 3, `.claude/docs/session-learnings.md`, new entry at the top** — the operator edit of a plan between
   runs.
   Signals: explicit operator correction (+0.4) · specific technical detail (+0.2) = 0.6, then "reached no other
   durable home this wrap" (+0.2) = 0.8. No rule file's `paths:` covers a chunk plan, so Tier 2 had no home and
   the entry went to Tier 3.
   Proof: inputs#I4 (the operator took none of the three offered options and named this course, "as at the
   capability-record chunk") and inputs#I5 (the edit, the re-fire, then the pass); the record is
   `evidence/operator-edit-plan.md`, with `plan.md` line 179 and lines 339-342 as edited. This is the second
   chunk that took the course; the first was `2026-10-10-capability-record-re-based`.

## Not applied

- **Deferred by the cap (→ handoff):** "a mandated factory that nothing calls is retired from the mandate, not
  kept as owed; an entry that needs one builds it with its consumer" (confidence 0.7: an explicit operator ruling
  +0.4, rule language +0.3). The fourth of four above the threshold. Its instance is written in test-plan §3
  item 6; the general rule has no curated home.
- **Duplicate (1):** "a test-support crate cannot serve unit modules of the crate it depends on" — this wrap's
  cascade wrote it into the generated body of `.claude/rules/testing.md` and into `docs/services/ingest.md`.
- **Task-specific (2):** an input snapped at the start of a step can move while the step runs (a fact about the
  pipeline's own input tool, recorded in the friction ledger, not a project learning); the evolve record's stamp
  passed through a shell variable (the same).
- **Below the threshold (3):** two identical default records in one batch both persist (0.2, and test-plan §7 now
  carries the default-batch reading); a second lock on a live claim's file is refused inside one process (0.2; the
  member's own test holds it); the allow comment's placement for the secret scanner (0.2; the run does not show
  which placement the scanner needs).
- **Recurrence despite a learning (3, → handoff):**
  - `CLAUDE.md` Tier 1, 2026-05-31 (map the orphan graph before a cleanup): plan step 4 named `random_word` for
    removal while `run_passphrase` in the same file calls it; /implement kept the function.
  - `.claude/rules/host-linux.md`, Transports (a `cat` heredoc with a file target): refused once more in the
    implement session, re-issued once as anchored edits.
  - `.claude/rules/observability.md` Session Additions, 2026-08-21 (a negative grep is trusted only after its
    known-present control fires): at Phase 1 a sweep for other diff-header parsers ran first with a fixed string
    that could not match the escaped spelling; it read 0 on the one file known to hold the token, and was re-run.
- **Positive, no learning:** the count arithmetic held (20 tests written, 3112 → 3132 in three places); the edge
  pin read red first; each mutation was read back before its run and each restore confirmed by checksum.

## Stale citations in preserve-verbatim homes (the citation sweep's six rows)

None is edited here. They ride the handoff, as the operator's word at earlier wraps has it for this class:
`.claude/rules/frontend.md:99` (`xtask/src/main.rs:550`, the map says 552) · `.claude/rules/security.md:160`
(`xtask/src/main.rs:813` twice; the entry already says the number is no longer the function's line) ·
`.claude/rules/observability.md:149` (two numbers the entry itself dates to 2026-08-26) · and one new to the
list, `.claude/docs/session-learnings.md:1324` (`Cargo.toml:42` for the `specta` workspace dependency, which
stands at line 48 in the work tree).

## For the card

`CLAUDE.md` Tier 1, 2026-05-20, says "a leaf crate never depends on a sibling crate". It was written of product
edges (the injected scrubber). `buffer` now names `test-support` under `[dev-dependencies]`, by the carve-out
architecture states as PROVISIONAL. Not extended here; lean: leave it until the founder's word on the carve-out.
