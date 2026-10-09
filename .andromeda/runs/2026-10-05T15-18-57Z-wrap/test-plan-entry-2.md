
## 2026-10-05-l4-runs-the-founder-s-pick-with-its-authors-settings — the grammar-mismatch emission and the unrun .ps1 grader become owed rows
**Section:** §1 Pending coverage triggers → `llamacli-inference-error-emission-coverage` (widened) · §1 → `l4-latency-p99-ps1-run-coverage` (new)
**Change:**
- `llamacli-inference-error-emission-coverage` is widened. `generate_constrained` also WARNs `error_category = "grammar_schema_mismatch"` (`recovery_action = "skip_digest"`) when `grammar_for_schema` refuses a non-L4 schema. Only the helper's two return arms are pinned, and no live leg can fire the emission, since both production callers pass the L4 schema. The owed per-category assertion now covers `io_error` / `stdout_utf8_invalid` / `grammar_schema_mismatch`.
- New row `l4-latency-p99-ps1-run-coverage`. The dev-host grader `xtask/ci/l4-latency-p99.{sh,ps1}` was changed in lockstep: budget 10000 ms, nearest rank, and an unlabeled record exits 1. Only the `.sh` has run, against the three `xtask/ci/fixtures/l4-latency-*.jsonl` fixtures and the live GREEN leg. The `.ps1` has never run (no `pwsh` on the dev host). Owed: run it over the same fixtures.
**Why:** Both are new paths this chunk added with no executed test at their tier. The `.ps1` run is the overseer's named owed item.
**Ref:** .andromeda/runs/2026-10-05T15-18-57Z-wrap/
