# Mutation check — Step 6 (one-shot, recorded 2026-09-30 by /andromeda-implement)

All runs: `cargo nextest run -p security` (the mutations also used `--no-fail-fast`), each log alongside this file.

## RED-before — `red-before.log`
The new pins ran against the UNTOUCHED arm (`\b(?:\d[ \-]?){13,19}\b`, no Luhn), with only the test edits applied.
- exit 100 · `15 tests run: 8 passed, 7 failed`
- FAIL: `scrubber_allows_non_card_digit_runs::case_1..case_7`, all 7 cases. Every failure reads `was redacted as credit_card` (7×).
- PASS: `scrubber_redacts_luhn_valid_card_forms::case_1..case_8`, which the old arm already redacted.

## GREEN-after
- `cargo nextest run -p security`: exit 0 · `54 tests run: 54 passed`.

## (a) Luhn neutralized — `mutation-a-luhn-neutralized.log`
Mutation: `luhn_valid(&digits)` → `(luhn_valid(&digits) || true)` in `has_luhn_valid_window`.
- exit 100 · `54 tests run: 47 passed, 7 failed`
- FAIL: `scrubber_allows_non_card_digit_runs::case_1..case_7`, and nothing else.
- So the Luhn decision is what frees the measured value. Reverted with the Edit tool; a grep for the mutation token reads 0.

## (b) Windowing collapsed — `mutation-b-windowing-collapsed.log`
Mutation: `card_number_matches` runs Luhn over ALL the candidate's digits (13–19 total) instead of the whole-group window walk.
- exit 100 · `54 tests run: 52 passed, 2 failed`
- FAIL: `scrubber_redacts_luhn_valid_card_forms::case_7` (`ref 12 4111111111111111`) and `::case_8` (`4111 1111 1111 1111 22`), exactly the two adjacent-group cases.
- So the window walk carries recall. Reverted with the Edit tool; a grep for the mutation token reads 0.
