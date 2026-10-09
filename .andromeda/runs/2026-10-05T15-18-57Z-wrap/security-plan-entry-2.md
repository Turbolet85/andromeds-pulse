
## 2026-10-05-l4-runs-the-founder-s-pick-with-its-authors-settings — three product-written locations outside the data dir, not two
**Section:** §Data Protection → At rest (the corpus-key lock-file bullet) · §Security Anti-Patterns → Input (the product-binary path-env-var ban, its `XDG_RUNTIME_DIR` carve-out)
**Change:** Was "the second product-written artifact there after the `~/Downloads` training-export sink" and "one of the two product-written locations outside the data dir"; now one of THREE, with the export sink and the per-spawn L4 grammar temp file `andromeda-pulse-llama-grammar-{pid}-{nanos}.gbnf` in `std::env::temp_dir()`. That file has been the constraint document since chunk #84 (the JSON schema then, the committed first-party GBNF now), RAII-removed after each wait, its path never logged.
**Why:** A pre-existing reality the plan missed, measured at that chunk's research: the temp-file write predates the lock file, and this chunk changed only its content and name. The implementation is correct; the doc alone was wrong.
**Ref:** .andromeda/runs/2026-10-05T15-18-57Z-wrap/
