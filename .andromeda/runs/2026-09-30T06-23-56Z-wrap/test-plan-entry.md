
## 2026-09-29-scrubber-path-false-positive — §4 security crate row re-synced
**Section:** §4 Unit Test Strategy → security crate
**Change:** Was "crate suite 14 → 23" (stale since before this chunk). Now: crate suite 54, 39 → 54 this chunk. It names the `credit_card` arm's two new corpora: `scrubber_redacts_luhn_valid_card_forms` (8 Luhn-valid forms, including a card behind `key=` and beside an unrelated digit group) and `scrubber_allows_non_card_digit_runs` (7 byte-identical `Allowed` digit runs: date-time stamps in path / key / `PROJECT:` forms, a 19-digit nanosecond timestamp, the Luhn-invalid `4532-1234-5678-9010`). Both are mutation-checked so each rejection branch discriminates.
**Why:** the card arm is now Luhn over whole-group windows; its recall and precision halves are each pinned, and the count is re-derived by measurement.
**Ref:** .andromeda/runs/2026-09-30T06-23-56Z-wrap/
