# Operator pass — 2026-09-29-ci-wall-time-and-round-trips

Fired by the builder on the overseer's instruction ("Run the operator pass now (entries 10-14)"), after the host
constraint was released.

## Entry 10 — hygiene
- run: `python -X utf8 {tools_dir}/gate.py hygiene`
- exit 0 · atom `contains hygiene: clean` held
- summary: `hygiene: clean — read 27 (runs 27 · evidence 0) · trails 12 not read · binary 0 not read by P1`

## Entry 11 — pre-CI commit + push
- commit `226554a` `chore(2026-09-29-ci-wall-time-and-round-trips): operator pre-CI commit` (40 files; staged
  bindings carried `"mcp":` ×1)
- run: `git diff --quiet && git diff --cached --quiet && git push origin chore/migrate-pulse-to-v3` · exit 0 ·
  `e98d838..226554a  chore/migrate-pulse-to-v3 -> chore/migrate-pulse-to-v3`

## Entry 12 — CI conclusion
- run: `python -X utf8 {tools_dir}/ci.py conclusion --sha HEAD --wait 5400`
- exit 0 · atom `contains verdict: green` held
- `226554ae6d28 verdict: green · checks 10/10 · wall 4853 s` · runs `secret-scan#36607192380` completed/success ·
  `ci#36607192309` completed/success (polled 157× over 4864 s)

## Entry 13 — per-job wall-clock (ci#36607192309, the first round after the key rename: cold by construction)
| job | conclusion | start → end (UTC) | minutes |
|---|---|---|---|
| lint / test (windows-latest) | success | 17:45:29 → 19:06:20 | 80.9 |
| lint / test (macos-latest) | success | 17:45:34 → 18:52:36 | 67.0 |
| lint / test (ubuntu-22.04) | success | 17:45:28 → 18:48:44 | 63.3 |
| boot smoke (ubuntu-22.04) | success | 17:45:28 → 18:24:48 | 39.3 |
| supply-chain | success | 17:45:28 → 18:21:40 | 36.2 |
| coverage gate | success | 17:45:29 → 18:12:49 | 27.3 |
| a11y (windows-latest) | success | 17:45:28 → 18:03:53 | 18.4 |
| a11y (ubuntu-22.04) | success | 17:45:28 → 18:00:34 | 15.1 |
| a11y (macos-latest) | success | 17:45:34 → 17:58:28 | 12.9 |

Round 80.9 min, set by Windows lint-test. The ≤27 min forecast is for a WARM round and is not measured here. The
a11y jobs ran 12.9–18.4 min against the ≤10 min a11y-plan budget — cold, their read-only lint-test cache was not
yet saved when they started.

Cache lines (`gh run view 36607192309 --log`): every job logged `No cache found.`; `Saving cache` in the five
saving jobs (lint-test ×3, boot, coverage); a11y ×3 and supply-chain restore-only, nothing saved.

## Entry 14 — Actions cache usage after the green round
- `{"active_caches_size_in_bytes":8576022877,"active_caches_count":6}` — 8.58 GB of the 10 GB cap
- keys: lint-test Windows 2.49 GiB · lint-test macOS 2.01 GiB · lint-test Linux 1.67 GiB · boot Linux 1.60 GiB ·
  coverage (registry only) 188.97 MiB · gitleaks 5.45 MiB. The old `*-cargo` keys are already gone.
