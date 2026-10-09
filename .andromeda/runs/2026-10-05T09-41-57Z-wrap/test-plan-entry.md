
## 2026-10-05-l4-runs-on-a-small-current-model-chosen-by-measurement — probe pins 16 → 29
**Section:** §1 Pending coverage triggers → `l4-decision-probe-arg-parse-unit-coverage`
**Change:** The probe was 16 collected pins; now 29 (13 new; `ARMS` 14 → 16): 2 on `--sampling` (the allowlist `--temp` / `--top-p` / `--top-k` / `--min-p` / `--presence-penalty` / `--chat-template-kwargs`, each value validated) and 9 on the per-row readings `thinking` / `elapsed_ms` / `peak_rss_kib` / `peak_vram_mib`, the `--footprint` summary, `--gbnf FILE` and the arms `nr` (drops `-rea off`) and `gb` (`--grammar-file` in place of `--json-schema-file`), through pure fns and `parse_args_from`. The owed flag parse was every flag but `--shapes`; now every flag but `--shapes` / `--sampling` / `--gbnf` / `--footprint`.
**Why:** The probe gained the arms, flags and readings the model-selection series measured with; each new pin was mutation-checked where the plan named one.
**Kept:** §4 takes no row for the two `unit_llamacli_inference` argv pins — the one file hit (§1 security-vector row) enumerates the guard branches, which they leave true.
**Ref:** .andromeda/runs/2026-10-05T09-41-57Z-wrap/
