
## 2026-09-30-span-level-redaction — security crate covers span masking
**Section:** §4 What unit tests cover → security crate
**Change:** The security crate now also covers `mask_secret_spans` / `MaskedValue` with 31 co-located `span_mask` pins — one embedded canary per category (exact masked strings), verdict equality with `scrub_attribute` over both corpora and two proptests, the class-aware extent, catalog-order merges, single-token parity, byte-identical allowed values, and idempotence through the bounded fixpoint — mutation-checked (keyed extent · merge · span masking · fixpoint each redden their pins), with `proptest-regressions/scrubber.txt` replaying the two discriminating seeds. Crate suite was 54; now 85. "`scrub_attribute` and the `ScrubbedValue` contract are asserted unchanged" still holds.
**Why:** the new primitive's unit-tier coverage was unrecorded; it records the founder-ratified span extent (P4 2026-10-01, «Ок давай по типу правила»).
**Ref:** .andromeda/runs/2026-10-01T11-19-47Z-wrap/
