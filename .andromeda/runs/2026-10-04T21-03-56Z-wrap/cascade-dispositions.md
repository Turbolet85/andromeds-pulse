# Cascade dispositions — 2026-10-04-l4-interpretation-names-its-triggering-cue

**The search:** `cascade.py sweep --patterns-file cascade-patterns.toml` (`sweep.txt` in this run dir), run after
all six body amendments landed and before any sidecar entry. It covered the 13 patterns below, spanning the retired
claims' wording AND their mechanism. Every pattern's control fired on the pre-pass masters (baseline `5d6e344`).

| id | targets |
|---|---|
| `own-route-entry` · `prompt-remedy` · `sym-untouched` | the retired arch clause "a prompt-side remedy … is its own route entry" and its mechanism (the model-authored fields "untouched") |
| `b6932` · `x2.4` · `kib9.2` | the retired observed-maximum figure, ratio and headroom |
| `lineage-v23` · `v12-suffix` | the retired prompt lineage |
| `zero-tests` · `neither-test` · `a5-equality` | the retired §1 probe claims |
| `corpus-matches` · `decision-probe` | the new surface's tokens, for restatements elsewhere |

Sections read beyond the listing:
- `architecture.md:72` whole (Fault Identity);
- `security-plan.md:139` and `:461` around the figure;
- `test-plan.md:144`, `:384`, `:385`;
- leaf headers (provenance) of `.claude/docs/services/{triage,interpretation}.md`, `security-summary.md`,
  `tests-summary.md`;
- `CLAUDE.md` `GENERATED:setup:warnings` / `modules`.

## Rows
| row | disposition |
|---|---|
| `test-plan.md:328` own-route-entry standing | no change — "porting the verb to native Linux is its own route entry" (pre-push:linux), a true claim sharing the token |
| `test-plan.md:377` own-route-entry standing @c1027 | no change — window read: `crates/buffer/src/schema.rs:320–323` comment "owned by its own route entry"; unrelated |
| `.claude/docs/session-learnings.md:2551` own-route-entry curation | no change — "the producer repair became its own route entry"; a historical curation entry, preserve-verbatim |
| `.claude/docs/session-learnings.md:2617` own-route-entry curation @c1752 | no change — window read: the persist-vs-resolve race fix; unrelated, preserve-verbatim |
| `.andromeda/playbook.md:62` own-route-entry base | no change — a generic rule's wording ("it needs its own route entry"); not a quotation of the retired arch clause |
| `.claude/rules/verification-harness.md:95` own-route-entry leaf | no change — pre-push native port; unrelated |
| `.claude/docs/tests-summary.md:88` own-route-entry leaf | no change — pre-push native port; unrelated |
| `architecture.md:72` sym-untouched standing edited @c4939 | amended — the new text "leaves the model-authored symptom, timeline and ranked hypotheses as the model wrote them" is TRUE (the producer still does not rewrite them); the retired part ("its own route entry") is gone |
| `security-plan.md:139` b6932 standing edited | amended — 6,932 B kept as HISTORY ("the 2026-08-27 damper-chunk legs added a reflection prompt at 6,932 B"); the observed maximum is now 7,185 B, ~2.28×, 9,199 B ≈ 9.0 KiB; no `~2.4×` / `9.2 KiB` remains (patterns at 0 rows) |
| `security-plan.md:461` b6932 standing edited | amended — "before it 6,932 B, the 2026-08-27 reflection prompt" kept as history beside the 7,185 B maximum |
| `test-plan.md:144` lineage-v23 new | this pass's text — "since prompt v2.3" is history, true |
| `test-plan.md:385` lineage-v23 new | this pass's text — "v2.3 / v1.2-* at 2026-10-01-…" is history, true |
| `test-plan.md:144` a5-equality new | this pass's text — "its content-equality branch runs for no arm", true |
| `architecture.md:72` corpus-matches new | this pass's text |
| `architecture.md:237` corpus-matches standing @c3361 | no change — window read: the deterministic-mode title prefix reaching "the digest's CORPUS MATCHES lines"; still true (the header is unchanged; the note sits under it) |
| `test-plan.md:384` · `:385` corpus-matches new | this pass's text |
| `architecture.md:72` decision-probe new | this pass's text |
| `test-plan.md:144` decision-probe standing edited ×2 | the row's own id + the example path, both true |

Zero-row patterns (controls fired): `prompt-remedy`, `x2.4`, `kib9.2`, `v12-suffix`, `zero-tests`, `neither-test`.
The retired forms survive nowhere in the masters, registries, curation homes, judgment bases or leaves.

## Lateral binds
- `test-plan §3 ↔ obs-plan §3`: neither §3 was amended (§1 and §4 only). Bound pair unchanged.
- `a11y-plan schema ↔ obs-plan schema`: untouched.

## Leaves re-derived (step 3)
- `CLAUDE.md` `GENERATED:setup:warnings` (from arch §Established Decisions [Fault Identity]): the Fault Identity
  line gains "the digest's `TRIGGER:` line … is keyed on the SAME first cue the identity is taken from; it is
  framing, never identity". `GENERATED:setup:modules` was recomputed against arch: the `triage` / `interpretation`
  entries state no amended claim — no change.
- `.claude/docs/services/triage.md` (workspace/modules leaf of arch):
  - the `digest/` module line was a stale "empty skeleton". It is recomputed from `crates/triage/src/digest/mod.rs`
    at HEAD: assembler / `render_payload` incl. the TRIGGER line and the corpus framing note, retrieval, LWW queue,
    damper, broadcast;
  - the contract line now names the `cue_cause_label` → digest `TRIGGER:` use and the `#[doc(hidden)]` dev-probe
    re-exports.
- `.claude/docs/services/interpretation.md`: read; it states nothing about prompt composition or lineage. No
  amended claim, no change.
- `.claude/docs/security-summary.md` / `.claude/rules/security.md` (security-plan leaves): carry no prompt-byte
  figure (`b6932` leaf 0 rows; `grep prompt` read). No change.
- `.claude/docs/tests-summary.md` / `.claude/rules/testing.md` / `.claude/rules/verification-harness.md`
  (test-plan leaves): carry no §1 probe row, no §4 lineage (`decision-probe` / `lineage-v23` leaf 0 rows). No
  change.
