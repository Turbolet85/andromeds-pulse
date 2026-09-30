# Cascade dispositions — wrap 2026-09-30T06-23-56Z (2026-09-29-scrubber-path-false-positive)

## The search
- `cascade.py sweep --patterns-file cascade-patterns.toml` (cascade v1.1, baseline `a08ae29d`, the pre-CI parent). Four patterns, each with a fired control:
  - `recall-prec` — regex `recall[ -]over[ -]precision`, case-insensitive. The retired catalog-wide posture claim.
  - `credit-card` — regex `credit[_ ]card`, case-insensitive. Every statement about the arm.
  - `suite-count` — fixed `crate suite 14`. The stale count.
  - `no-arm-retuned` — fixed `no arm added`. The unscoped residual.
- A fifth pattern, `digits-13-19` (`13[-–]19 digit`), was REFUSED by the tool: its control never fired over the pre-pass masters. It was run by hand instead: `grep -rnE '13[-–]19|Luhn' CLAUDE.md .claude/rules .claude/docs docs/session-learnings.md` → 2 hits, both `.claude/docs/services/security.md` (`:29`, `:36`).
- Sections read whole: security-plan §Security Anti-Patterns → Logging (`:428`–`:444`); test-plan §4 security crate row (`:366`); `.claude/docs/services/security.md` (whole file); `.claude/rules/security.md` `:56`–`:57`; `.claude/docs/security-summary.md` headings plus `grep -i 'redact|PII'` (2 hits, both unrelated: the data-classification NONE line, and the logging-redaction-wire bootstrap phase); `.claude/docs/tests-summary.md` / `.claude/rules/testing.md` `grep -i 'security crate|crate suite|scrubber'` (3 unrelated Session-Additions hits, no count claim); CLAUDE.md `grep -i 'redact|scrub'` (3 hits).

## Rows
| row | disposition |
|---|---|
| `security-plan.md:428` recall-prec · credit-card ×3 | **amended** (S1): the posture is scoped to seven arms and the credit_card precision rule added. The ×3 credit-card hits are the catalog enumeration, the `provider_key` ordering clause (true) and the new paragraph |
| `security-plan.md:436` recall-prec · credit-card | **amended** (S2): scoped to the key-anchored arms |
| `security-plan.md:444` no-arm-retuned · credit-card | **amended** (S3): tied to its origin chunk, with the later retune noted |
| `test-plan.md:366` credit-card ×2 | **amended** (T1): count 54, plus the two corpora. The pre-existing `before credit_card` ordering clause stays true |
| suite-count: 0 rows after the pass (control fired at `test-plan.md:366`) | the stale count no longer stands anywhere |
| `.claude/rules/security.md:56` credit-card (leaf) | **re-derived**: a `credit_card` precision bullet was added after the catalog bullet. The catalog line itself stays true |
| `.claude/rules/security.md:57` recall-prec (leaf) | **re-derived**: "per the catalog's recall-over-precision posture" → "per the key-anchored arms' …" |
| `.claude/rules/security.md:161` recall-prec · credit-card (curation home — Session Additions 2026-08-23) | **no cascade edit**, preserved verbatim. Routed to P3 as an in-place extension of that entry, which states "per the catalog's own recall-over-precision posture" |
| `CLAUDE.md:32` credit-card (leaf, `GENERATED:setup:modules`) | **no change**: "8 P-047 categories … via OnceLock-cached compiled regex set" still holds (arch-derived; arch not amended) |
| `.claude/docs/services/security.md:29`, `:36` (hand-grep) | **re-derived**, whole catalog section recomputed from `crates/security/src/scrubber.rs` as read this session. Besides this chunk's claims, the leaf was stale on its own: 7 categories, no `provider_key`, wrong bearer/api-key/jwt shapes, a `Hash` derive the enum lacks, the test count 14. Corrected to current source |
| base rows (playbook / drift-base) | 0 across all patterns |
| S4 `security-plan.md:221` npm current state (hand sweep) | **amended**. Leaves: `grep -rn 'GHSA-ggr8\|green-with-dispositions\|dev-only bumps' CLAUDE.md .claude/rules .claude/docs` → 0 hits, so there is no leaf to re-derive |

## Binds
- test-plan §3 ↔ obs-plan §3: untouched (no harness/log-format change).
- a11y ↔ obs schema: untouched.
- Cross-master citations: the patterns ran over all seven masters. The only master hits are the amended sites; no other master restates the posture or the count.
