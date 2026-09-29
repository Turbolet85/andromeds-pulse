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

# Operator pass 2 — after the rebuild-thrash fix and the seven-job layout

Overseer's warm re-run of ci#36607192309 (attempt 2, relayed): 47.7 min, forecast 27 missed; perf:slo-load 17.0
min on ubuntu (rebuilt), criterion 5.9–7.6 min per OS. Instruction: find the rebuild, parallelise the tail, keep
cache keys stable, operator pass again.

## Entry 10 — hygiene
- first read refused 1 file (P1 drive: a host path this evidence file quoted at line 18); both `C:/…` run paths
  rewritten to `{tools_dir}/`; re-read: `hygiene: clean — read 2 (runs 1 · evidence 1)` · exit 0

## Entry 11 — pre-CI commit + push
- commit `4502d5d` (staged bindings `"mcp":` ×1) · guarded push exit 0 · `226554a..4502d5d`

## Entry 12 — CI conclusion
- exit 0 · atom `contains verdict: green` FAILED — `4502d5dcaf0d verdict: red · checks 13/13 · first-fail +736 s
  boot smoke (ubuntu-22.04)`; ci#36625507595 concluded `failure` (every other job success)
- the red: `boot: ready (PID=6847)` at 20:31:28.18, then `harness:status` `"verdict": "not-running"` 0.7 s later;
  `logs-boot-Linux`: 40 records, last at 20:31:27.957 (boot binds + first ticks), no `app.panic.fatal`, the only
  ERROR `KeyringUnavailable` (CI has no credential store); boot.log holds one AT-SPI dbind warning. The silent-death
  signature of the Linux boot watch (546d3f0) — its recurrence. No pulse-app source changed in this chunk.

## Entry 13 — per-job wall-clock (ci#36625507595, warm: every job `full match: true`)
| job | conclusion | minutes |
|---|---|---|
| coverage gate | success | 26.6 |
| lint / test (windows-latest) | success | 19.9 |
| lint / test (macos-latest) | success | 18.2 |
| release build (windows-latest) | success | 13.5 |
| a11y (windows-latest) | success | 12.1 |
| boot smoke (ubuntu-22.04) | failure | 11.3 |
| supply-chain | success | 6.1 |
| lint / test (ubuntu-22.04) | success | 6.1 |
| release build (macos-latest) | success | 5.8 |
| mcp-server tests (ubuntu-22.04) | success | 5.3 |
| a11y (ubuntu-22.04) | success | 5.3 |
| a11y (macos-latest) | success | 4.7 |

Round 27.6 min (first start → last end), set by coverage — its instrumented build is uncached by design (registry
only). Cache lines: every rust-cache step logged `Restored from cache key … full match: true`; the saving jobs
logged `Cache up-to-date`. a11y ran 4.7–12.1 min against the ≤10 min budget (Windows over).

## Entry 14 — Actions cache usage
- `{"active_caches_size_in_bytes":8576022877,"active_caches_count":6}` — unchanged, no new keys
