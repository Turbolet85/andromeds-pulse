# Operator pass — 2026-10-04-corpus-key-creation-is-race-free

Fired 2026-10-04 by the implementing session on the overseer's word ("Run the operator pass now, entries 18-28, with
a fresh per-run PUPPETEER_CACHE_DIR, then the bindings regen and the base check after stage 5, push, CI read"). The six
native pre-push stages are the founder deviation of 2026-10-04 (plan §Test Commands; `pre-push:linux` drives
`wsl.exe` and cannot evaluate on the Linux dev host). The rust-analyzer flycheck cargo tree was stopped before the
heavy cargo stages (rust-analyzer itself left running).

| plan entry | what | result |
|---|---|---|
| 18 | `gate.py hygiene` | `hygiene: clean — read 26 (runs 25 · evidence 1) · trails 12 not read · binary 0 not read by P1` |
| 19 | stage 1/6 script-modes | green — exit 0, `100755` |
| 20 | stage 2/6 source-lint | green — exit 0, `"verdict": "clean"`, 481 files, 0 hits |
| 21 | stage 3/6 npm, fresh `PUPPETEER_CACHE_DIR` = `target/pre-push/puppeteer-20261004T083419Z` (absolute, gitignored, `test ! -e` before use) | green — exit 0: `npm ci` added 800 packages; `npm run build` built (1867 modules) |
| 22 | stage 4/6 clippy | green — exit 0, 0 warning/error lines |
| 23 | stage 5/6 test | green — exit 0, `2580 tests run: 2580 passed, 0 skipped` (2575 + the 5 new `race_free` fns) |
| 24 | stage 6/6 ci-gates over fresh `target/pre-push/data-20261004T083629Z` (SEED_LOG verbatim) | green — exit 0: zero-spans PASS (3 records) · zero-panic PASS · heartbeat-gap PASS (max 15000 ms vs 45000 ms) · perf-budget NEUTRAL (frame cannot-evaluate, memory and snapshot NEUTRAL: the seed carries no samples) |
| 25 | `--features mcp-server` bindings regen, after stage 5 | green — exit 0, `1 test run: 1 passed`. Before it, `git diff --quiet 76d6cca -- bindings/index.ts` read 1: stage 5 had re-emitted the no-mcp shape, as the handoff predicted |
| 26 | base-identity close | green — exit 0, bindings byte-identical to `76d6cca` |
| 27 | pre-CI commit + push | see §CI below |
| 28 | `ci.py conclusion --sha HEAD --wait 2400` | see §CI below |

## Spec claim disproved by measurement (stage 5 arm)
The plan's Implementation notes and entry 23's note state that stage 5 runs under `env -i`, which drops
`XDG_RUNTIME_DIR`, "so the witness exercises the temp_dir arm there ... Both arms are live on this host". Measured
false: `env -i` also drops `DBUS_SESSION_BUS_ADDRESS`, so the Secret Service is unreachable and every credential-store
leg SKIPS in stage 5. A one-shot re-run of the corpus `race_free` fns plus `corpus_key_survives_a_real_process_boundary`
under the stage's exact `env -i` with `--success-output immediate` printed three `[skip] no OS credential store on this
host; …` lines (concurrent-creation, cross-process, fail-closed store-untouched); 6 passed. Entry 23's atom
(`passed, 0 skipped`) reads nextest's skip COUNT, not the `[skip]` lines (nextest captures passing output), so it
passes in both worlds and cannot show the arm.

The temp-dir arm WAS then measured live in a separate one-shot: the same selection under `env -u XDG_RUNTIME_DIR`
(session bus kept) → 6 passed, 0 `[skip]` lines; the witness and the sequential test removed their `/tmp` lock files.
So both directory arms are measured with a live store — the XDG arm by the listed entry 4, the temp-dir arm by this
one-shot — but not by stage 5 as the plan claimed.

## Test residue (skip arm)
On the skip arm, `corpus_key_survives_a_real_process_boundary` takes the lock (creating an empty 0600 lock file in the
temp dir) before the store call fails, then returns before its cleanup. Two such files remain in `/tmp` from the stage 5
run and the arm probe (`andromeda-pulse-corpus-key-9838ad066f9912a5.lock`, `…-6968b65906a08147.lock`), both pid-keyed
test services. Not fixed in this pass (a source change after the gates ran); a follow-up owner can move the lock-file
removal ahead of the skip return. The same applies on CI runners, which have no credential store.

## Deviations
- Stage 3 ran with a fresh per-invocation `PUPPETEER_CACHE_DIR` under `target/pre-push/` (founder ruling 2026-10-04);
  `~/.cache/puppeteer` was not touched.
- Stage 6 ran over a fresh per-invocation data dir under `target/pre-push/` (no `rm`), per the plan's form.
