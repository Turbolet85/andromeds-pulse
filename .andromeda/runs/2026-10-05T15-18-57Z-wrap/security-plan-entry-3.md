
## 2026-10-05-l4-runs-the-founder-s-pick-with-its-authors-settings — the full-path never-log set names the L4 grammar temp file
**Section:** §Security Anti-Patterns → Logging (the "NEVER log a full product-consumed filesystem path" bullet) · §Logging & Monitoring → What NEVER to log (the full-path bullet)
**Change:** Both enumerations now include the per-spawn L4 grammar temp file passed as `--grammar-file` (`andromeda-pulse-llama-grammar-{pid}-{nanos}.gbnf` in `std::env::temp_dir()`), which is never logged at all. Measured: 0 occurrences of `andromeda-pulse-llama-grammar` and 0 full GGUF paths in the GREEN real-model leg's log.
**Why:** The file was renamed and re-purposed this chunk. The categorical rule already covered it; naming it keeps the enumerated set exact.
**Ref:** .andromeda/runs/2026-10-05T15-18-57Z-wrap/
