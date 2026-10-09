# Amendment Record — 2026-05-03T21-30-00Z-lift-accent

_Generated retroactively by user after session 11 implemented the
amendment ad-hoc but before /andromeda-implement spec-drift-protocol.md
existed. Backfilled per `spec-amendment-protocol.md` Part A schema as
the first use case of the new protocol. Read by /andromeda-wrap-session
Phase 6 (drift classification) and /andromeda-setup-project Phase 0
(delta scope)._

## Schema version: 1

## Identity

- **Amendment ID:** 2026-05-03T21-30-00Z-lift-accent
- **Trigger:** chunk #12 phase #9 via `npm run verify:contrast`
- **Authority resolution:**
  - **Winning plan/tier:** a11y-plan.md — tier=Standard (full WCAG 2.1 AA)
  - **Losing plan/concern:** design-system.md — palette aesthetic (Alert Burgundy hex specificity)
  - **Rationale:** WCAG SC 1.4.11 3:1 non-text minimum trumps Alert Burgundy hex specificity. a11y-tier=Standard takes precedence over design palette aesthetic per project's tier configuration. Burgundy semantic preserved (hex shifted dusty rose-ward but still recognizably anomaly accent).

## Plans amended

### Plan: `.andromeda/design-system.md`
- **Sections:**
  - Color Palette Core Colors (Accent row hex)
  - Color Palette Semantic Colors (Warning + Error rows hex)
  - Color Palette Border Progression / Component Patterns (rgba derivative)
  - Surface: desktop-webview Tokens block (--color-accent in @theme)
  - Anti-Patterns Universal Bans (palette enumeration)
  - Downstream Readiness obs / a11y refs (palette enumeration)
  - Design Decisions Log (NEW entry appended)
- **Decisions Log entry:** "2026-05-03 — Lift `--color-accent` from `#8B2E3B` to `#C7556A`"
- **Before → After:**
  - `--color-accent`: `#8B2E3B` → `#C7556A`
  - rgba derivative: `rgba(139, 46, 59, X)` → `rgba(199, 85, 106, X)` (X varies by usage)
  - accent/base contrast: 2.05:1 (FAIL) → 3.96:1 (PASS at SC 1.4.11 non-text 3:1 minimum)

## Implementation files synced

- `pulse-app/ui/src/styles/tokens.css` — `--color-accent` hex updated to `#C7556A`
- `pulse-app/ui/src/contrast/pairs.mjs` — `accent/base` reclassified: target_ratio 4.5 → 3.0, wcag_criterion `SC 1.4.3` → `SC 1.4.11`, usage `normal-text-or-non-text` → `non-text` (formal acknowledgement that accent is a non-text token)
- `pulse-app/ui/dist/tokens.css` — Vite/Tailwind regenerated with new hex (gitignored build artifact)
- `pulse-app/ui/dist/contrast-report.json` — harness re-run output flipped FAIL → PASS (gitignored build artifact)

## Expected downstream propagation

(Hard-coded baseline per `delta-rerun-protocol.md` plan→file mapping table for design-system.md amendment.)

- Tier 3: `.claude/docs/design-summary.md` — palette enumeration + Anti-Patterns hex listing + Decisions Log mention
- CLAUDE.md `<!-- GENERATED:setup:warnings -->` — if Alert Burgundy hex appears in top-10 universal warnings (verify by re-running setup-project --delta materialization)
- state.yaml: `plan_freshness.design_mtime` — acknowledge mtime advance (will refresh on next wrap-session Phase 8)

## Lifecycle status

- [x] Applied 2026-05-03T21:30:00Z — by `/andromeda-implement` (ad-hoc; backfilled retroactively to this format)
- [x] Noted 2026-05-03T23:22:08Z — by `/andromeda-wrap-session` (session 11; Phase 6 D5 amendment-aware classification recognized this as info-pending; Phase 8 lifecycle progression set `noted_at` ISO timestamp per v2.1 schema rename — was `noted_by_run` path until 2026-05-04 v2.1 migration)
- [x] Propagated 2026-05-03T23:35:00Z — by `/andromeda-setup-project --delta` (`.andromeda/runs/2026-05-03T23-35-00-setup-project-delta/`) (Phase 0 expanded delta scope to 3 files: design-summary.md per marker + design-tokens.md + a11y.md per Setup grep-discovery; Phase 9 set `propagated_by_run` path)
- [ ] Archived {pending — set by next /andromeda-wrap-session as `archived_at` ISO timestamp per v2.1 schema}

## Verification

- **Orphan grep:** `git grep -- "#8B2E3B"` returned 1 match
  - Acceptable matches:
    - `.andromeda/design-system.md` Decisions Log entry (cites old value as part of "before → after"; intentional)
  - Verified clean at: 2026-05-04T01:00:00Z (during retroactive backfill)
  - Status: clean
- **Orphan grep:** `git grep -- "139, 46, 59"` returned 0 matches → clean
- **Harness re-run:** `cd pulse-app/ui && npm run verify:contrast` exit 0 (12/12 pairs pass; accent/base now 3.96:1 ≥ 3.0:1 non-text)
- **Tests baseline:** `npm run test` 64/64 pass; `cargo nextest run --workspace` 36/36 pass

## Cross-references

- Specialist plan Decisions Log: `.andromeda/design-system.md` §Design Decisions Log entry dated 2026-05-03 — "Lift `--color-accent` from `#8B2E3B` to `#C7556A`"
- Harness output: `pulse-app/ui/dist/contrast-report.json` records `accent/base` flipped FAIL (2.05:1 < 4.5:1) → PASS (3.96:1 ≥ 3.0:1 non-text)
- state.yaml.spec_amendments.active[] entry: `2026-05-03T21-30-00Z-lift-accent` (added retroactively with backfill)

## Backfill note

This amendment was applied ad-hoc during session 11 (see session-handoff entries for chunk #12 wrap) BEFORE the spec-amendment-protocol formal contracts existed. The amendment record is retroactively backfilled on 2026-05-04T01:00:00Z to make chunk #12's amendment the first use case of the new protocol AND to ensure state.yaml accurately reflects history. The actual spec + implementation changes were committed during session 11's `/andromeda-implement` run; this record is a forensic reconstruction.
