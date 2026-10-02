# Cascade dispositions — 2026-09-30-span-level-redaction

**Search:** `cascade.py sweep --patterns-file cascade-patterns.toml` (5 patterns, written after the last body edit;
every control fired on the pre-pass masters): `whole-value` `(?i)whole[ -]value|replaces? the WHOLE` · `scrub-attr`
fixed `scrub_attribute` (the retired actor at 12 of 13 boundaries) · `collapse-marker`
`(?i)collaps\w* (to|into) (the|a|one) (bounded )?(category )?(marker|placeholder)` (the retired incident-summary
failure mode) · `one-placeholder` `(?i)redact\w* (to )?ONE placeholder|redact\w* identically` (the retired collision
prose) · `pending-notice` `(?i)pending notice` (the retired honest-degraded consequence). Phrasings read beyond the
tokens: "replaces the value", "blanks the whole digest" (pre-pass master hits 0 — a leaf phrase, read by hand in
`.claude/rules/security.md` and `.claude/docs/services/security.md`), "per redacted value" (0 pre-pass hits — the new
unit, absent before). Sections read: security-plan §Threat Model Summary · §Data Protection → At rest · §Security
Anti-Patterns → Logging (whole); architecture §Conventions → Primary key convention · §Occupied Resources (egress
sink) · §Established Decisions [Fault Identity]; test-plan §4 security / buffer crates; obs-plan §1 · §5 · §8.

## Rows (40 lines listed; every row)
| Row | Disposition |
|---|---|
| security-plan:428 whole-value ×3 (@c2425, 3559, 3882) | amended — this pass's text; each occurrence states the new posture accurately (span masking replaces whole-value; single-token parity; whole-value survives only at labels) |
| security-plan:430 whole-value (new) | amended — labels keep whole-value per label (true) |
| test-plan:373 whole-value (new) | amended — "single-token parity with whole-value replacement" (true) |
| .claude/rules/security.md:58 whole-value (leaf) | **stale** — "A redaction still replaces the WHOLE value (P-048), so one true positive anywhere in a digest blanks the whole digest" → re-derived |
| .claude/docs/services/security.md:38 whole-value (leaf) | **stale** — "A redaction replaces the WHOLE value (P-048 …)" → re-derived |
| security-plan:48, :166 scrub-attr (edited) | amended — "gated by `scrub_attribute`'s verdict" (true) |
| security-plan:428 scrub-attr ×3 (new) | amended — detection verdict / only caller `encode_labels` (true) |
| security-plan:430 scrub-attr ×2 (edited) | amended — verdict gate + labels whole-value (true) |
| security-plan:436 scrub-attr (standing) | no change — the third scrub shape: `encode_labels` still calls `scrub_attribute`, which still takes a single `&str` (true) |
| test-plan:373 scrub-attr ×2 (edited) | no change at those offsets — `ScrubbedValue::Allowed(_)` / "`scrub_attribute` and the `ScrubbedValue` contract are asserted unchanged" both still true |
| CLAUDE.md:142 scrub-attr (curation, Tier 1 2026-05-20) | no change — a historical example of the closure-injection pattern ("verified at chunk #72"); the pattern stands (the closure now wraps `mask_secret_spans`); preserve-verbatim |
| .claude/rules/security.md:147 scrub-attr ×4 (curation, 2026-05-26) | no change — history of the resolver-boundary defense-in-depth discipline; the discipline stands (the resolver now masks through the same scrubber) |
| .claude/rules/security.md:163 scrub-attr (curation, 2026-08-23) | no change — the composed-form rule is about `encode_labels`, which keeps `scrub_attribute` (true) |
| .andromeda/playbook.md:86 scrub-attr (base) | no change — the rule quotes "any **attribute value** crossing into persistent storage OR the self-observation log sink", wording the amended INTENDED-posture sentence keeps; its reasoning (bounded first-party label ≠ attribute value) is unaffected |
| CLAUDE.md:32 scrub-attr (leaf, GENERATED modules block) | **stale** — security module describes `scrub_attribute()` as the primitive → re-derived (adds `mask_secret_spans`) |
| .claude/docs/services/corpus.md:9 scrub-attr (leaf) | **stale** — "`scrub_attribute` for PII scrubbing at ingestion" → re-derived |
| .claude/docs/services/security.md:22, :32, :34 scrub-attr (leaf) | **stale as a surface list** — no `mask_secret_spans` / `MaskedValue` → re-derived |
| security-plan:436 collapse-marker (standing) | no change — "two distinct keys can never collapse into one placeholder" (labels; true) |
| .claude/rules/security.md:163 collapse-marker (curation) | no change — same labels claim (true) |
| .claude/rules/security.md:59 collapse-marker @c976 (leaf) | no change at that offset — labels claim (true) |
| security-plan:434 one-placeholder @c1969 (edited line) | no change at that offset — "two DISTINCT credential-shaped names redacting identically" describes the collision the `seq` key closed (history; single-token names still mask identically) |
| .claude/rules/security.md:59 one-placeholder ×2 @c2097, 2403 (leaf) | **stale** at @c2403 ("the two names still redact to one placeholder") → re-derived; @c2097 the closed-collision history (true) |
| .claude/docs/conventions.md:32 one-placeholder (leaf) | **stale** — "credential-shaped names redact to one placeholder and collide there" → re-derived from arch §Conventions |
| security-plan:432 pending-notice (edited) | amended — "no longer … sends the report to the pending notice" (true) |

## Leaves re-derived (step 3)
Table floor: security-plan → `.claude/docs/security-summary.md` · `.claude/rules/security.md` (body) · CLAUDE.md
`GENERATED:setup:warnings`; architecture (§Conventions, §Occupied Resources) → CLAUDE.md `GENERATED:setup:*` ·
`.claude/docs/conventions.md`; test-plan → `.claude/docs/tests-summary.md` · `.claude/rules/testing.md` (body);
obs-plan → `.claude/docs/obs-summary.md` · `.claude/rules/observability.md` (body). Plus the sweep's leaf rows:
`.claude/docs/services/{security,corpus}.md`. Each recorded below by the step-3 pass.
- `.claude/rules/security.md` (body) — re-derived: the credit_card bullet's whole-value sentence retired; a NEW body bullet "Span-level masking, not whole-value replacement" (class-aware extent · verdict gate · fail closed · fixpoint · per-caller placeholders · JSON per leaf · count anywhere); the coverage bullet's Residual narrowed. `## Session Additions` untouched.
- `.claude/docs/services/security.md` — re-derived whole (public surface + `mask_secret_spans` / `MaskedValue` / `ArmClass`; the WHOLE-value gotcha retired; lib.rs layout corrected to `pub mod scrubber;` — measured, the prior "re-exports" line was already false; tests 54 → 85).
- `.claude/docs/services/corpus.md:9` — re-derived (`mask_secret_spans` at the write boundary).
- `.claude/docs/conventions.md:32` — re-derived from arch §Conventions (collision only inside a masked span).
- `CLAUDE.md` `GENERATED:setup:modules` security line — re-derived (adds `mask_secret_spans()` → `MaskedValue`, span masking). Other GENERATED blocks: no statement on the amended arch sections (overview / warnings / pointer-table read — no scrub, redaction or PK-collision claim).
- `.claude/rules/observability.md` (body, heartbeat bullet) — re-derived: the per-VALUE unit at the column sites added beside the label-pair unit.
- `.claude/docs/security-summary.md` · `.claude/docs/obs-summary.md` · `.claude/docs/tests-summary.md` · `.claude/rules/testing.md` (body) — recomputed, no change: none states the scrub mechanism, the whole-value posture, the counter unit or the security suite count (`grep -inE 'scrub|redact|P-047'` hits read: unrelated — wire-log redaction rows, allowlist leaves, security-vector triggers).
