# Cascade dispositions — 2026-10-04-declared-rust-floor-matches-the-code

## The search
`cascade.py sweep --patterns-file cascade-patterns.toml` (cascade v1.1; baseline `46b600a7`, the pre-CI parent), run
AFTER all 7 body amendments of this pass. Patterns, each with its control fired on the pre-pass masters:
- `decl-185` `rust-version = "1.85"` — the retired declared value (control arch:14)
- `any-185` `1\.85` — every restatement of 1.85, true or stale (control arch:14)
- `route-owner` `The declared Rust floor matches the code` — the retired owner pointer (control arch:14)
- `code-floor` `code floor` (i) — the retired phrasing "code floor ≥ 1.89" (control arch:47)
- `ge-189` `(≥|>=) ?(rustc )?1\.89` — the 1.89 bound, read for "as THE floor" vs "own-code bound" (control arch:14)
- `stale-decl` `rust-version…stale` — the retired verb (control arch:14)
- `appinfo-184` `1.84.0` — the old `app_info` example value (control arch:103)
- `suite-87` `→ ?87` — the moved corpus suite count (control test-plan:383)
- `msrv` `MSRV|incompatible_msrv` — the removed allow and its reason (control test-plan:627)
Sections read beyond the rows: arch §Stack, §Established Decisions [Primary Language], §Standard Contracts `app_info`,
§Inherited Defaults; security-plan §Security Anti-Patterns → Universal and §Data Protection "Corpus-key lock file"
(:167, never-deleted contract — unchanged, the new unlink is test-only); test-plan §4 Framework and corpus row.
Step-3 leaves checked by provenance beyond the rows: `.claude/docs/conventions.md` (`app_info` named, no
`rust_version` value — no change), `.claude/docs/commands.md` (no `app_info` example — no change),
`.claude/docs/tests-summary.md` (0 floor / corpus-count hits — no change), CLAUDE.md `GENERATED:setup:warnings` /
`modules` (no floor or allow statement — no change).

## Rows (41 printed; 9 count lines × 2 excluded)
Masters:
- `security-plan.md:470` any-185 standing edited mixed ×3 — amended line; the three 1.85 hits are the Edition-2024
  minimum (true) — no change beyond the amendment.
- `architecture.md:14`, `:47`, `:355` ge-189 — this pass's new text; 1.89 stated as the workspace's OWN-code bound beside
  the declared 1.95 — true, no change.
- `test-plan.md:344` ge-189 + msrv (new) — this pass's new text, true — no change.
- `test-plan.md:383` suite-87 standing edited @c2728 — "crate suite 82 → 87" is the race-free chunk's own historical
  step; this pass added "87 → 88" before it — true, no change.
- `test-plan.md:627` msrv ×2 — httpmock MSRV, unrelated — no change.
- `appinfo-184` — 0 rows after the pass (control fired): `architecture.md:103` amended to `"1.95"`.
Curation homes (never edited by the cascade):
- `.claude/rules/security.md:122` (Session Additions 2026-05-23, Rust 1.85 native async fn) — true, unrelated; no route.
- `.claude/docs/session-learnings.md:2402,:2404,:2406,:2408` — historical 1.84 → 1.85 Edition-minimum entries, true as
  history; no route.
Leaves:
- `CLAUDE.md:9` decl-185 · any-185 · route-owner · ge-189 · stale-decl — **re-derived** (Stack line recomputed from arch §Stack).
- `.claude/docs/stack.md:6` decl-185 · any-185 · route-owner · ge-189 · stale-decl — **re-derived** (Primary language).
- `.claude/docs/stack.md:84` decl-185 · any-185 · route-owner · ge-189 — **re-derived** (the "open half" closed).
- `.claude/rules/testing.md:17` code-floor · ge-189 — **re-derived** (Framework line, from test-plan §4).
- `.claude/rules/security.md:87` any-185 ×2 — **re-derived** in lockstep with security-plan :470 (minimum kept, floor added).
- `.claude/docs/security-summary.md:50` any-185 — **re-derived** (minimum kept, floor added).
- `.claude/docs/services/corpus.md:29` ge-189 · msrv — **re-derived** (the allow sentence removed); the same leaf's
  `:34` (skip-arm behaviour added) and `:45` (87 → 88 corpus tests) recomputed from test-plan §4's corpus row.
- `.claude/docs/gotchas.md:13`, `:14` any-185 — the historical "1.84 → 1.85 bump" gotcha: the Edition-2024 minimum it
  states is still true and not amended this pass — no change.

## Lateral binds
test-plan §3 ↔ obs-plan §3: neither section amended — holds. a11y ↔ obs schema: neither amended — holds.
Judgment bases (`playbook.md`, `drift-base.md`): 0 rows.
