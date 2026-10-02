# Scope — 2026-08-30-staged-bindings-assertion

## Intent (working entry, surface)
Staged-bindings assertion — the generated TauRPC bindings a commit actually carries match the
procedure pin, checked mechanically rather than remembered. Minted from the FIFTH occurrence of the
bindings-clobber defect (`capability-drift` RED at HEAD from commit 70344d5 while that chunk's
handoff recorded it clean), against FOUR existing prose rule entries describing the identical defect
(`rules/testing.md` 2026-05-13 / 05-17 / 2026-08-15 · `rules/security.md` 2026-06-12). The operator's
ruling: a check the wrap RUNS beats a discipline the wrap author must REMEMBER.

## What this chunk builds
- **A staged-copy assertion in xtask** that FAILS when the git-INDEX copy of
  `pulse-app/ui/src/bindings/index.ts` lacks the `mcp` namespace — subject read via
  `git show :<path>`, never the worktree. Grounded by the SIXTH occurrence (measured
  2026-08-23-ingestion-scrub-coverage): HEAD and the index both carried the correct mcp-bearing
  bindings while only the worktree was clobbered, so a worktree-reading check would have reported a
  false RED where the staged read reported the truth.
- **Unconditional trigger coverage** (CARRY folded — the fix must cover BOTH triggers or it closes
  one and ships the other): trigger (a) = a chunk that changes TauRPC and regenerates late;
  trigger (b) = ANY chunk whose gate set includes a default-features workspace test run — which is
  every chunk, because the plan-mandated `cargo nextest run --workspace --profile ci` is itself a
  default-features run and running the gates is what rewrites the file via `taurpc`'s
  `export_types()`. The assertion therefore runs unconditionally, never on a TauRPC-touched
  predicate.
- **Unskippable wiring** — the entry demands "wired so it cannot be skipped". VERIFIED (P3):
  `capability-drift` runs as its own ci.yml step (:103) AND last in every test-plan §3 gate list AND
  in the wrap's P7 slot — so the staged assertion folds INTO `capability_drift()` (any non-clean
  staged outcome fails it), making every existing invocation transitively run the check, plus an own
  verb + own ci.yml step for direct invocation and a named failure surface. The two consumers see
  different subjects by construction: locally the index is the about-to-be-committed copy; on a
  fresh CI checkout the index equals HEAD, so the same read validates the commit itself.
- **Extension to `pulse-app/capabilities/*.json`** (CARRY folded): the same staged-copy mechanism
  asserts the committed capability grants — identical class (a committed file a gate can silently
  alter). Origin `2026-08-23-webview-self-verify`: its RED mutation arm revokes
  `core:window:allow-close`, rebuilds, then restores BY HAND — only `--expect-absent` is code, so
  "byte-identical after restore" is a DISCIPLINE today and **no shipped gate catches a left-revoked
  grant** (`capability-drift` parses procedures from bindings.ts and never reads grants;
  `capability-widening-check` covers only the 3 NEVER-widen caps and only in the widening
  direction). Tracked test-plan-side as pending trigger `webview-drive-mutation-arm-not-gated`.
  VERIFIED (P3): the baseline is an xtask-side `EXPECTED_GRANTS` pin (sibling of
  `EXPECTED_PROCEDURES`, xtask/src/main.rs:1116; name measured free in the graph) over the 6
  capability files' `{identifier, windows, permissions}` — `description` fields deliberately
  excluded (default.json carries ~9.5 KB of prose whose edits must not red a security gate) — with
  set equality asserted in BOTH directions, so a revoked AND an added grant each red.
- **Interim ordering-rule retirement**: `.andromeda/playbook.md` carries the interim ORDERING rule
  codified at the 2026-08-22 wrap that "holds until this lands". VERIFIED (P3): playbook.md:74–76
  states verbatim "until it lands, this rule is the interim" — the wrap re-points the rule at the
  shipped gate (playbook is operational, not a spec master).
- **RED/GREEN proof discipline** (VERIFIED-concretized at P3): the assertion's discrimination is
  proven by mutation — a staged no-mcp copy (and a staged revoked-grant capability file) reds the
  gate; the restored copies green it — in runtime-built TempDir FIXTURE repos (`git init` + write +
  `git add`, per test-plan §7), never by mutating the real tree. Both real copies measured healthy
  at HEAD (worktree + staged `"mcp":` count 1 each), so the gate lands GREEN and the §3 boot-smoke
  conditional trigger does not attach.

## Hypotheses — ALL VERIFIED at P3 (2026-08-30; coordinates in research.md §Files inspected)
- VERIFIED — `capability_drift()` reads the WORKTREE (`fs::read_to_string`, xtask/src/main.rs:1173–1176);
  nothing in shipped code reads the git index (zero `"git"` invocations exist in xtask).
- VERIFIED — the 2026-06-12 `rules/security.md` staged-copy entry exists as cited; prose only, no
  code implements it. STRENGTHENING noted: the natural staged assertion diffs the FULL procedure
  set against `EXPECTED_PROCEDURES` (which already lists `mcp.status/start/stop` active), so
  mcp-absence is caught as a strict subset (3 missing) alongside any other staged drift.
- VERIFIED — the interim ordering rule is at `.andromeda/playbook.md:74–76`, text as cited, with
  "until it lands, this rule is the interim".
- VERIFIED — test-plan §1 trigger `webview-drive-mutation-arm-not-gated` (test-plan.md:131) names
  exactly this class and CARRYs the capabilities half onto this entry.
- VERIFIED — `pulse-app/capabilities/` holds SIX files (default, clipboard, notification, tray,
  plugin-fs, updater); full grant inventory enumerated in research.md.
- VERIFIED — all four prose entries exist (`rules/testing.md` :169 / :179 / :258 ·
  `rules/security.md` 2026-06-12).
- VERIFIED — `70344d5` exists: `feat(2026-08-22-pii-scrubber-recall)`, consistent with the playbook's
  "codified 2026-08-22 … after commit 70344d5 shipped bindings in the no-mcp shape".

## PREREQ folded (audit pin #22 — session arithmetic re-derived at promotion)
`state.yaml` `session_count: 60` → this chunk's wrap is session **61 = the owed FULL-FORM
`cargo audit` interval point** (session-60 between-point discharged at the diagnostics sweep wrap:
basis byte-identical exit 1 — `duplicate advisory ID: RUSTSEC-2026-0244` — advisories exit 0 with
the owned set EMPTY from scratch, bans licenses sources exit 0). Owed at this chunk's wrap and
recorded in its report — never a silent skip:
- run the probe FULL-FORM: `cargo audit` true exit read directly (no interval skip at this point);
- re-verify the overlap (`cargo deny check advisories`), re-enumerating DISTINCT `RUSTSEC-` ids from
  scratch — a NEW finding gets a visible disposition, never a quiet return to red;
- no "Nth consecutive" ordinal; end the deferral the first time `cargo audit` loads.

## Boundaries (not in scope)
- Not fixing the regen mechanism itself — `taurpc`'s dev-mode `export_types()` rewrite stays; this
  chunk gates the ARTIFACT a commit carries, not the generator.
- No production `.rs` or webview code changes — xtask + tests + CI wiring only (VERIFIED at P3:
  all touchpoints are `xtask/src/*` + `.github/workflows/ci.yml`).
- The RED mutation arm's manual restore stays manual — the gate catches a LEFT-REVOKED grant at
  commit/CI time; it does not automate the restore.
- Not the adjacent tail entries (ACL-rejection logging · Dead lib-src test migration) — adjacent
  harness work stays theirs.
