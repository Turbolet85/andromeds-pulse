# Cascade dispositions — 2026-10-04-supply-chain-advisories-on-wasmtime-resolved

Amended this pass: architecture §Stack and Technologies Plugin runtime row (:22), §Established Decisions
[Plugin Runtime] heading (:52), §Inherited Defaults Plugin runtime bullet (:362); test-plan §5 plugins -> runtime row
(:413). Retired claims: requirement `"48.0.3"`, lockfile-resolved 48.0.3 as of 2026-09-29, Cranelift 0.135.3,
"49.0.1 needs Rust 1.96", "the bump from 46.0.3 closed RUSTSEC-2026-0316" as the latest closing bump.

## The search
`cascade.py sweep` over `cascade-patterns.toml` (5 patterns, every control fired on the pre-pass masters):
- `wt-4803` fixed `48.0.3` — new 1 · standing 1 · leaf 2
- `cl-01353` fixed `0.135.3` — 0 rows (control fired at architecture.md:22 pre-pass)
- `wt-4901-r196` fixed `49.0.1` — 0 rows (control fired at architecture.md:22 pre-pass)
- `resolved` regex `lockfile-resolved|resolved \*{0,2}4[0-9]\.[0-9]` — the requirement-versus-resolution phrasing however
  pinned — standing 12 · leaf 4
- `wt-sweep` regex `wasmtime[` ]+(v?4[0-9]|48\.x|25\+)` — every stated wasmtime version or floor — standing 11 · leaf 8 ·
  curation 6
Sections also read by hand: the arch-derived leaves `.claude/docs/{stack,gotchas,conventions,security-summary,
tests-summary}.md` and `.claude/docs/services/plugins.md` for any wasmtime / Cranelift / RUSTSEC-2026-03xx claim
(`grep -nE 'wasmtime|cranelift|Cranelift|RUSTSEC-2026-03'`): floors and Cranelift posture only, no pin.

## Rows
- architecture.md:22 `wt-4803` new — the new text's "the bump from 48.0.3 closed …" (history inside the current claim): amended, correct.
- architecture.md:22 / :52 / :362 `resolved` + `wt-sweep` edited — the amended lines; re-read: no duplicate of the retired values on any of them.
- test-plan.md:413 `resolved` + `wt-sweep` edited — amended line; re-read: 48.0.5 / 2026-10-04, the 48.x and 25+ statements kept.
- security-plan.md:219 `wt-4803` + `wt-sweep` standing — the dated record of the 2026-09-29 chunk's bumps that ended the cargo-audit deferral: a true historical statement, no change.
- architecture.md:20 / :25 / :49 / :55 / :355 / :361, security-plan.md:140 / :210 `resolved` standing — DuckDB 1.10505.0 and rmcp 3.1.4 requirement-versus-resolution statements: a true claim sharing the phrasing, no change.
- security-plan.md:64 / :133 / :197 / :400, test-plan.md:37 / :77 `wt-sweep` standing — the `wasmtime 25+` floor, still true at 48.0.5: no change.
- CLAUDE.md:9 `wt-4803` ×2 + `resolved` + `wt-sweep` leaf — RE-DERIVED (requirement `"48.0.4"`, resolved 48.0.5, closing RUSTSEC-2026-0325 / -0326 / -0327, 49.x needs Rust 1.96).
- .claude/rules/security.md:79 `wt-4803` + `wt-sweep` leaf — restates security-plan:219's dated history (the 2026-09-29 bumps): true, no change.
- CLAUDE.md:29, .claude/docs/stack.md:16, .claude/docs/security-summary.md:21, .claude/docs/services/plugins.md:4 / :18 / :70 `wt-sweep` leaf — the 25+ floor; recomputed from the amended arch row: these leaves carry the family floor and never the pin (by their own shape), so the recompute yields the same text. No change.
- .claude/docs/gotchas.md:11, .claude/docs/services/mcp-server.md:20, .claude/docs/stack.md:36 `resolved` leaf — rmcp / DuckDB, no change.
- .claude/docs/session-learnings.md:1198 / :1375 / :1377 / :1479 / :1481 / :1492 `wt-sweep` curation — dated historical learnings (wasmtime 43, the 25+ spec reading): preserve-verbatim and still true as history; nothing routed to P3.
- Judgment bases (playbook / drift-base): 0 rows.

Closure: every printed row dispositioned; one leaf re-derived (CLAUDE.md:9); no cross-master citation of the retired values remains.
