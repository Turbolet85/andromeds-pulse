
## 2026-09-30-span-level-redaction — redactions_applied unit at the column sites
**Section:** §5 Metric Coverage → `redactions_applied` row
**Change:** Beside the labels unit (one increment per REDACTED LABEL PAIR, unchanged), the row now states the column-site unit: one increment per redacted VALUE however many spans it masked — `scrub_otlp_field` masks each secret in place (`mask_secret_spans`) and increments once when the value carried any (4 values carrying 10 masked spans → 4; live wire 0 → 4). The drain/template path was said to call `scrub_attribute` directly; it now masks through `mask_secret_spans`.
**Why:** span masking left the counter's unit at the column sites unstated; the unit is unchanged per value, so the five-cell canary and the counter's meaning hold. Ratified span extent: founder at P4 2026-10-01, «Ок давай по типу правила».
**Kept:** §1 and §8 list the field with no unit or mechanism — no change there.
**Ref:** .andromeda/runs/2026-10-01T11-19-47Z-wrap/
