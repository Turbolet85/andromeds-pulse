# Fan-out results — 2026-08-29-halo-state-pulse-signature-deferred

7/7 extracts returned and validated. Per-extract checks (header · sections-in-order · anchors · in-domain · size 20–180 · entity escapes):

| specialty | relevance | validation | raw twin |
|---|---|---|---|
| arch | partial | clean | no (raw == stripped) |
| security | partial | **entity escapes** (`&amp;` ×5) → decoded on save | `.raw-security.md` |
| design | relevant | clean | no (raw == stripped) |
| layouts | relevant | **entity escapes** (`&lt;section&gt;` ×2) → decoded on save | `.raw-layouts.md` |
| tests | relevant | **entity escapes** (`&lt;name&gt;` ×1) → decoded on save | `.raw-tests.md` |
| obs | partial | clean | no (raw == stripped) |
| a11y | partial | **entity escapes** (`&lt;section&gt;`/`&amp;` ×5) → decoded on save | `.raw-a11y.md` |

Entity-decode remedy applied per fan-out.md check 6 (one decode on save, never a re-spawn) — 4 of 7 affected, consistent with the reference's own "measured ×4" note. 0 escapes remain in the seven decoded extracts.

**Aggregate:**
- **A** — all 7 present ✓
- **B** — 0 `No domain coverage` (3 relevant, 4 partial) ✓
- **C** — bindings reciprocate (design↔layouts on the co-owned defer sites; tests↔security on the audit pin; arch↔obs on no-new-target). **One cross-extract premise conflict, resolved in a11y's favor pending P3 confirmation:** the design extract inherited scope's `[inferred]` claim that a11y-plan (master) carries the SC 2.3.3 Halo wording; the a11y distiller MEASURED the master + sidecar at **zero "Halo" occurrences** (2026-08-29) — the halo wording lives in a11y LEAVES (`rules/a11y.md:42`, SR scripts, harness assets incl. the `tray-icon-halo` baseline bucket), and the master's SC 2.3.3 is token-bound + surface-agnostic. P3 re-derives first-hand and closes scope H3 accordingly (expected: a11y = cascade-leaf only, NO a11y master amendment).

**New candidate sites the extracts surfaced beyond scope's list (P3 verifies each at HEAD):**
- design-system §Self-Validation Protocol #3 (Signature Test — "mandatory per Brand Identity Coherence") — a FOURTH design-master halo site.
- design-system-amendments §Downstream Readiness "For obs specialist" (Halo pulse-frequency/color logging hooks) — sidecar index; check whether it needs a defer note or is history-only.
- obs-plan §8 `hue_update_ms` narrative — a restating site of the non-render claim; check-at-wrap either amended or recorded verified-still-true.
- layout-templates candidate line sets from the layouts distiller (webview 19, 21-23, 27, 28, 44-46, 65/68, 80, 98-121, 222, 238; native 246, 248-250, 254, 262/264/267, 299-301, 303-307, 309, 375-376) — verify at HEAD.
- a11y leaves + harness assets: `rules/a11y.md:42`, `docs/a11y-summary.md`, 3 SR scripts, `reduced-motion.spec.ts:27`, `p11-constellation-semantics.spec.ts:30`, `baselines/a11y-violations-summary.json:66` (`tray-icon-halo` bucket) — classify leaf-cascade vs untouched.
- tests extract flags: whether any of the three det-L4 legs (webview-drive, smoke:gap-resume, smoke:external-resolve) currently asserts over `ResolutionSummary`/`EvidenceRefs` surfaces; and the 2026-05-08 by-construction-no-test precedent as the counter-shape to argue against per pin.
