# Cascade dispositions — 2026-09-30-dual-license

Amendments of the pass: architecture §Established Decisions ([License], new) · §Design Philosophy :4 (crate counts) ·
§Infrastructure Patterns runtime topology :258 (crate count) · §Occupied Resources → xtask CLI surfaces :242 (the
capability-drift slot) · security-plan §Dependency Security → CI integration :216 (own-crate license check) ·
test-plan §3 Per-chunk gate discipline (gate-set block + ordering note) · playbook (approved supersession of :74 +
the appended gate-order rule — judgment base, the propose→approve channel, operator-approved this wrap).

## The search
`cascade.py sweep --patterns-file cascade-patterns.toml` (baseline `1dfca741`, the pre-CI parent), all four controls
fired on the pre-pass masters:
- `drift-last` regex `capability-drift[^|]{0,80}\bLAST\b` — the retired ordering, named form (control test-plan.md:304)
- `runs-last` fixed `runs LAST` — the retired ordering's verb phrase (control architecture.md:241)
- `crate-count` regex `twelve library crates|fourteen workspace members` — the stale count (control architecture.md:4)
- `module-12` fixed `12-module` — the count's rationale spelling (control architecture.md:48)

Beside the tool (patterns whose controls cannot fire — no master stated them pre-pass):
- by hand, masters, case-insensitive: `capability-drift.{0,100}(last|after every|final step|at the end)` and
  `(runs|run|placed|sequenced) last` over all seven masters + `.andromeda/registries/` (absent) → 0 hits post-pass.
- by hand, leaves: `licen|\bMIT\b|twelve|fourteen workspace|capability-drift` over CLAUDE.md, `docs/{stack,conventions,
  security-summary,tests-summary,commands,gotchas,workflow}.md`, `rules/{testing,verification-harness}.md`; and
  `\bLAST\b|runs last|last-and-alone` over CLAUDE.md + every `docs/*.md` / `rules/*.md` except session-learnings.
- the project's license value in the masters: `grep -c -w MIT` → 0 in all seven pre-pass (the new [License] entry is
  the first statement), so no master restated the old `MIT`.

## Rows
| row | disposition |
|---|---|
| `.claude/rules/testing.md:95` drift-last leaf | re-derived from the amended test-plan §3 (gate-set order, the regen + base-named close) |
| `.claude/docs/tests-summary.md:128` drift-last leaf | re-derived, same |
| `.claude/rules/testing.md:258` drift-last curation | Session Additions (preserve-verbatim) — routed to P3 as an in-place extension candidate; never a cascade edit |
| `.andromeda/playbook.md:76` drift-last/runs-last base ×2 | the superseded rule's own note, kept verbatim under its new `SUPERSEDED 2026-09-30` prefix (approved) — no change |
| `.andromeda/playbook.md:120` drift-last base | the appended rule's own provenance sentence ("the LAST ordering of the rule it supersedes") — true, no change |
| `.andromeda/architecture.md:242` runs-last standing @c4274 | AMENDED this pass (window read at 4074-4474: "rides the slot that runs LAST in every gate list") → the capability-drift slot, BEFORE the workspace nextest since 2026-09-30 |
| `.claude/docs/session-learnings.md:455`, `:457` ×3 crate-count curation | a 2026-05 learning narrating that past cascade ("eight → twelve library crates") — history, true as history, no change |
| `.andromeda/architecture.md:48`, `:54`, `:61` module-12 standing | decision-time rationale inside §Established Decisions entries ([Backend Framework] · [Tauri IPC Bridge] · [Module Boundaries]) stating the module count the decision weighed — the current count lives at :4 / §Occupied Resources :202; no change |

Leaf rows by the hand sweep:
- `CLAUDE.md:7` (`GENERATED:setup:overview`) "Public OSS (MIT)" → re-derived "Public OSS (MIT OR Apache-2.0)".
- `CLAUDE.md` architecture block — already states fourteen library crates / sixteen members: no change.
- `.claude/rules/security.md` §Supply chain + CI (body) and `.claude/docs/security-summary.md:32` — re-derived with the
  own-crate license-check fact from the amended security-plan :216.
- `.claude/docs/tests-summary.md:83` — the CI step order (ci.yml unchanged): no change.
- `.claude/docs/stack.md`, `conventions.md` (arch leaves by provenance, playbook :78) — no license, count or ordering
  claim: no change.
- `.claude/rules/security.md:112`, `:156` — Session Additions narrating the bindings-regen family's past ordering —
  curation homes, routed to P3 with `testing.md:258`.
