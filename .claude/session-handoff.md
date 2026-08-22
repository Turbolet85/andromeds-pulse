# Session Handoff

**Last Updated:** 2026-08-22T22:36:03Z
**Branch:** chore/migrate-pulse-to-v3 (tracks `origin/chore/migrate-pulse-to-v3`; 16 ahead before this commit — unpushed)
**Status:** clean (wrapped)
**Last Commit:** `2026-08-22-log-records-identity` — two log records in the same tick at the same severity both survive ingestion

## Position
- Done: `2026-08-22-log-records-identity` — `log_records` gains `seq BIGINT NOT NULL`; PK widens to 4 columns. **Measured RED first** (`PRIMARY KEY or UNIQUE constraint violation` at `flush()`, **zero rows landed**), GREEN after, on the real OTLP path. The route entry's "silently drops" premise was falsified in SHAPE: the loss is **whole-batch** and it IS logged at ERROR.
- Next (first markerless): **Ingestion scrub coverage** — the five client-controlled columns that never reach `scrub_attribute`. Carries the re-pinned `cargo audit` PREREQ (**pin #6**).
- **Tail grew 7 → 10.** Three new entries landed at operator direction; see Route below.

## Work done
6 source files (5 modified + 1 new dev-only producer example) + the regenerated bindings artifact; +222/−12. The `seq` ordinal threaded through `state` → `appender` → both `schema.rs` DDL representations, plus the two out-of-crate test fixtures (DDL **and** all three INSERTs, unconditionally — a PK column can never be NULL).

**Gates green:** fmt · clippy `-D warnings` · `-p buffer` **160/160** · nextest **1835** + 1 skip (1833 → +2, exactly the two new tests) · capability-drift clean · capability-widening 0/3 · `cargo deny check bans licenses sources` ok · advisories designed-RED at exactly the 8 owned IDs. **Smoke PASS and discriminating** — feed precondition `rows_ingested` **0 → 2** asserted first, `duckdb.append rows_appended=2` on `log_records`, 0 ERROR / 0 panics / 0 PK violations, both body canaries + the service label absent, **`seq` 0 times in the obs log** (it stayed an allocator, never an observable), 6 tick families, `buffer.tick` max gap 15.0s, clean shutdown both ports released.

## Drift resolved
**4 amendments across 2 masters + 1 leaf · 3 escalations resolved · drift = 0.** arch ×1 (§Primary key convention — the clause claimed log records key on "timestamp + resource hash + name" and **the table has no `name` column at all**, inaccurate at HEAD independent of this chunk) · test-plan ×3 (new `buffer-log-seq-allocator-unit-coverage` trigger; the redaction-trigger's basis clause corrected — `-p buffer` HAS now run, though that trigger stays OPEN; §4 buffer bullet now names TWO in-crate gaps). Cascade: `.claude/docs/conventions.md` §Primary keys. 5 of 7 detectors clean; 0 false positives; **0 proposal-level escalations** — all three escalations were orchestrator/operator-originated.

## Route
- **NEW** `metrics_points identity` (after Ingestion scrub coverage) — the direct sibling: PK at 3 sites, **no label column at all**, so same-metric points differing only by label set collide by construction AND lose their labels.
- **NEW** `Webview self-verify on the Windows host` (before A11y verification) — operator ruling 2026-08-22; the gap is the DRIVER (self-verify is boot-quit and "never clicks"; the Playwright suite drives `dist` in Chromium, not the app). Turns the UI legs below into agent legs.
- **NEW** `Staged-bindings assertion` (after the Diagnostics sweep) — see the recurrence note below.
- **CARRY** on `Integration UX e2e test` (P-076) — the det-L4 blind-spot family, folding intake #13 (`ResolutionSummary` unreachable; zero constructors, fixture pins `false`) with intake #2's degraded half (which was never pinned anywhere). Same class: an absence check over those surfaces passes for the wrong reason.
- **CARRY** on `Diagnostics un-muting + harness-truth sweep` — the `append_rejections` aggregate fold (`rows_appended` counts rows REQUESTED, computed before the append, so it can never witness a non-landing).
- **MATRIX** P-075 — PREMISE-CORRECTION in the **notes channel only**; acceptance untouched, cap stays pooled (`chunk:null`, `planned`). Its "P-025 ≤2s" clause was disproved Conductor-side at 7fd1608 (`hue_update_ms` measures staleness at the tier flip, not render latency).

## Curation
T1 0 · T2 0 · **T3 2** · **corrections 1** (cap-exempt). Filters: 1 duplicate · 1 `recurrence-despite-learning` · 1 minor recurrence. CLAUDE.md **153/200**.

## Notes

- **`recurrence-despite-learning` — the bindings defect, FIFTH occurrence.** `capability-drift` was RED at HEAD from commit `70344d5` while that chunk's handoff recorded it clean. My worktree was byte-identical to HEAD, so this chunk did not cause it. **Four** existing rule entries describe this exact defect (`rules/testing.md` 2026-05-13 / 05-17 / 2026-08-15, `rules/security.md` 2026-06-12) and it shipped anyway. Operator ruling: a check the wrap RUNS beats a discipline the wrap author must REMEMBER. Two-layer fix — an interim ORDERING rule in `.andromeda/playbook.md` (capability-drift LAST in P7, assert the **staged** copy via `git show :<path>`) + the new route entry for the mechanical xtask assertion. Not curated as a fifth prose entry, by design.

- **Cascade DAG under-enumeration (found, fixed, ruled).** `.claude/docs/conventions.md` declares itself arch-derived and restates §Conventions verbatim, but the amendment-flow cascade table names only `stack.md` (+ CLAUDE.md `GENERATED` blocks) as arch's leaves — so the arch amendment left it asserting the retired wording. The arch detector's own sweep was correct but scoped to arch; only the orchestrator's cross-master grep caught it. New playbook rule: **enumerate arch leaves by provenance header**, not by a hardcoded list (today: exactly `{conventions.md, stack.md}`).

- **CLAUDE.md `USER:session-learnings` corrected in place.** The 2026-08-21 external-relay entry claimed "a SECOND table at `:185` the relay did not name" — measured false: that is the same table's other representation, and `ddl_constants_match_concatenated_schema` already forces the pair to agree. Tagged `[corrected 2026-08-22: …]`, and it now records the two FURTHER unguarded copies (viz + triage fixtures) that both the relay and the entry missed. **First exercise of the cascade→curation routing path** for a preserve-verbatim home; it worked end to end.

- **INTAKE #8 — CLOSED as refuted** (operator directive). No `relative_magnitude` label exists anywhere in the code (one doc-comment occurrence; the live enum is `BypassReason::{Relative, Absolute}`). Dropped from the open list; no disposition owed.

- **INTAKE #13 — CLOSED**: text recovered from the `edb949f` handoff via `git show` and folded into the P-076 CARRY above. It is no longer an unquotable ghost obligation.

- **`cargo audit`:** **probe SKIPPED per ratified interval (next: 34)** — this wrap is session 33, the second consecutive between-points wrap. Never silent; recorded here and in the chunk report, and re-pinned in compact form (**pin #6**, origin `2026-08-15-corpus-key-persistence`, chain age preserved). Basis UNCHANGED and upstream. Overlap re-derived and **STABLE at eight** owned upgradeable IDs — no new finding.

- **A coverage gap in this chunk's own work, recorded not hidden:** `BufferState::reserve_log_seq_block` shipped with **indirect-only** coverage — exercised through the collision test, with no direct pin on its monotonicity / block-width contract, and exactly one caller, so a caller-side change would silently retire its only coverage. Filed as `buffer-log-seq-allocator-unit-coverage`. NOT the same as the prior chunk's gap: `buffer` now has real in-crate tests.

- **The duplicate-INSERT hang is NOT refuted.** Only the Appender path was driven (0.09s, no hang). `crates/buffer/src/schema.rs`'s caution about the INSERT path stands untested — do not read this chunk as retiring it.

- **Disk:** 85G free (72%) at wrap — the operator cleaned mid-session; the earlier 9.0G/98% reading is stale.

- Last failed command: none.
