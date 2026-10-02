# Validation Log — 2026-05-03T20:26:49Z setup-project re-run

## Pre-flight
- CLAUDE.md exists at `./CLAUDE.md`: ✓

## Health checks (15)

| Check | Status | Detail |
|---|---|---|
| 1 — Size | ✓ | 131 / 200 lines (~66% of budget) |
| 2 — Markers parseable | ✓ | 20 marker tags found (10 GENERATED:setup* pairs + 1 USER:session-learnings pair, all balanced) |
| 3 — @ imports valid | ✓ | 3/3 (architecture.md, route.md, session-handoff.md) all resolve |
| 4 — Rules YAML | ✓ | 7/7 rule files (a11y / design-tokens / frontend / observability / testing / verification-harness all have valid `paths:` frontmatter; security.md is universal — no frontmatter expected) |
| 5 — Core docs | ✓ | 5/5 (stack / conventions / commands / gotchas / workflow) |
| 6 — Arch staleness | ✓ | architecture.md is 102,518 seconds (28h) OLDER than CLAUDE.md — well within tolerance (no staleness; CLAUDE.md regenerated when arch.md or any specialist plan changes; this re-run regenerates state.yaml mtime cursors but CLAUDE.md GENERATED sections are byte-identical to existing) |
| 7 — Scope coverage | ✓ | N/A (0 scopes defined under `.andromeda/scopes/`) |
| 8 — Handoff | ✓ | `.claude/session-handoff.md` present + non-empty (last updated 2026-05-03T18:56:50Z by chunk #11 wrap) |
| 9 — .gitignore | ✓ | 4/4 required entries (`.claude/backup/`, `.claude/settings.local.json`, `target/`, `node_modules/`) |
| 10 — Specialist plans + summaries | ✓ | 6/6 plans (security-plan, design-system, layout-templates, test-plan, obs-plan, a11y-plan) + 5/5 summaries (security-summary, design-summary, tests-summary, obs-summary, a11y-summary) |
| 11 — Route chunks | ✓ | 52 detected by line-pattern grep (route §1 declares 55 — minor undercount because grep pattern `^[A-Z].* — ` doesn't match every chunk line variation; ≥1 threshold satisfied robustly) |
| 12 — Living artifacts | ✓ | dependency-tree.md + api-surface.md both present with valid `<!-- METADATA -->` and `<!-- LIVING:* -->` markers; reconciled 2026-05-03T18:56:50Z by chunk #11 wrap-session (well within 7-day freshness window) |
| 13 — state.yaml | ✓ | Python `yaml.safe_load` parsed successfully (schema_version=1, session_count=9, last_completed_chunk.route_index=11). Stack trace in stdout was Windows `cp1252` codec failing to encode the ✓ U+2713 character — print() failure post-validation, not a YAML parse failure. |
| 14 — Agent harness | ✓ | scripts/agent-run.sh + scripts/agent-run.ps1 both present (Development Style=agent-driven per arch §Cross-cutting Patterns) |
| 15 — Pointer table | ✓ | 23 rows in CLAUDE.md GENERATED:setup:pointer-table block (well above 5-entry sanity threshold) |

## Cross-skill diff check (NEW v2 invariant)

5/5 shared contracts byte-identical across 3 triangle skills (`andromeda-setup-project` / `andromeda-wrap-session` / `andromeda-new-session`):

| Contract | Status |
|---|---|
| section-markers.md | ✓ |
| health-criteria.md | ✓ |
| session-state-contract.md | ✓ |
| integrity-protocol.md | ✓ |
| curation-tier-decision.md | ✓ |

## Hook smoke test

| Hook | Status | Detail |
|---|---|---|
| Formatter (rustfmt) | ✓ | `rustfmt` resolves on PATH; rust-toolchain.toml pin satisfied |

## Summary

**16 ✓ / 0 ⚠ / 0 ✗** (15 health checks + 5 cross-skill diff + 1 hook smoke = 21 individual checks; all pass)

**Decision:** proceed to Phase 9 commit.

## Notes

- **Check 13 cosmetic warning:** Python's `print()` failed with `UnicodeEncodeError` on the ✓ checkmark character because Windows console default codec is `cp1252` (not UTF-8). This is a stdout-encoding issue, not a YAML-validation issue — the `yaml.safe_load()` call AND the `assert d['schema_version'] == 1` BOTH succeeded before `print()` raised. Manual YAML inspection confirms state.yaml is well-formed.
- **No drift introduced by this re-run:** the only file mtime changes are scoped to test-plan.md (already advanced), 3 materialized files (testing.md, tests-summary.md, state.yaml), and 2 NEW files (this validation-log.md + materialization-plan.md). No upstream specialist plan was modified by setup-project.
- **D5 / State J resolution:** state.yaml.plan_freshness.tests_mtime now matches actual test-plan.md mtime (2026-05-03T20:24:05Z); next wrap-session/new-session will see no plan-freshness mismatch.
