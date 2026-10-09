CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  1 correction, in place — the second seed bullet ("every new TauRPC
    procedure needs both router registration AND a `pulse-app/capabilities/` JSON entry") now reads: router
    registration AND the `EXPECTED_PROCEDURES` pin in `xtask/src/main.rs`; no per-procedure capability entry exists;
    a missing core-API / `core:window:*` grant IS silently rejected. Tagged `[corrected 2026-10-06: …]`.
    Proof: measured at this wrap — `pulse-app/capabilities/` holds six files (clipboard · default · notification ·
      plugin-fs · tray · updater) whose permission strings are `core:default`, nine `core:window:allow-*`,
      `updater:default`, four `notification:allow-*` and `clipboard-manager:allow-write-text` — 0 procedure names;
      the per-procedure pin is `EXPECTED_PROCEDURES` at `xtask/src/main.rs:1228`. The same statement already stands
      in the Critical Warnings block and in `.claude/rules/security.md` §Tauri capability gating.
  Tier 2 (.claude/rules/security.md `## Session Additions`): 3 entries corrected in place, 5 clauses —
    - the first 2026-05-09 entry: "(a) `pulse-app/capabilities/` JSON registration … router-level" → "(a) its
      `EXPECTED_PROCEDURES` pins in `xtask/src/main.rs`", tagged;
    - the first 2026-05-12 entry: "the documented triple (router registration + `pulse-app/capabilities/` JSON +
      …EXPECTED_PROCEDURES)" → "the documented pair (router registration + …EXPECTED_PROCEDURES)", tagged; and
      "router + capability JSON + EXPECTED_PROCEDURES" → "router + EXPECTED_PROCEDURES" in its two later
      sentences (the tag names all three places and that the entry's "FOURTH" / "4th" ordinals still count the
      phantom member);
    - the 2026-05-25 entry: "(alongside router registration + capability JSON + xtask EXPECTED_PROCEDURES + …)"
      → the list without "capability JSON", tagged (its "6th" still counts the phantom member).
    Proof: the same measurement as above, and the operator's own — word: "measured by me, the six
      pulse-app/capabilities files hold only core, clipboard-manager, notification and updater permissions, no
      procedure name" — the overseer, founder-delegated, in this wrap's conversation, 2026-10-06. The directive
      counted four sibling clauses; the confirming read found a fifth restatement inside the same 2026-05-12
      entry (its closing sentence) and corrected it under that entry's one tag.
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 0 task-specific · 0 conflict · 0 deferred
  CLAUDE.md size: 154/200 · T1 29.1 KB, 15 over 600 B (the corrected bullet stays under the cap)

Source of the Tier-1 candidate: the overseer's relay (`relay-1.md` §4), which names it as this session's own setup
re-run finding (`.andromeda/runs/2026-10-06T21-41-29Z-setup-project/materialization-plan.md`, Tier 1 observation).
Corrections are exempt from the max-3 cap (curation-guide §Corrections).

Left standing, named for the next reader:
- `security.md` Session Additions, the second 2026-05-09 entry ("…no new `pulse-app/capabilities/` JSON entry…"):
  true as written (the Settings path needs none), though it lists the entry among costs a new namespace would pay.
- `security.md` generated body, the "Static analysis test gap" paragraph ("`xtask capability-drift` … verifies
  TauRPC ↔ capability JSON sync only"): a generated body is corrected at its source, and the 2026-06-11 Session
  Additions entry already supersedes that paragraph.

No other candidate: the session carried no user correction of the work, no new dependency used in code, no
repeated command and no "from now on" convention.
