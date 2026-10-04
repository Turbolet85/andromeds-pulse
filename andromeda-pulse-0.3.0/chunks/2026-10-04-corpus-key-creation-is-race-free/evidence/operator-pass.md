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

## CI
- Entry 27: pre-CI commit `a073722124b5d3a4263eb563c1cb4b08bc8028e1` (`chore(2026-10-04-corpus-key-creation-is-race-free):
  operator pre-CI commit, for the run this chunk's verdict reads`; source delta `crates/corpus/src/keychain.rs` only),
  after `hygiene: clean` (re-run with this file: read 27), the regen above, `check:staged-artifacts` staged-clean and
  `capability-drift` green. Pushed with the plan's clean-tree guard: `76d6cca..a073722  chore/migrate-pulse-to-v3`, exit 0.
- Entry 28: `ci.py conclusion --sha HEAD --wait 2400` -> exit 0, `a073722124b5 verdict: red · checks 13/13 · first-fail
  +653 s boot smoke (ubuntu-22.04)`; runs `ci#37189514735` (pull_request, in progress at the read) and
  `secret-scan#37189514733` (success). Polled 23 times over 682 s. Afterwards 11 jobs read success, `coverage gate`
  still in progress, `boot smoke (ubuntu-22.04)` failure (job 111398586536).
- The red, as read from the job log and its `logs-boot-Linux` artifact: `agent-run.sh boot` reached `boot: ready`
  (PID 7141) after about 4 min; `agent-run.sh status` then read `"ended": "exit 1"`, `"verdict": "not-running"`. The
  app log (54 lines) carries NO `app.exit` record and no `app.panic.fatal`; its last record is
  `ui.webgpu.adapter {outcome: no_navigator_gpu, window_label: main}` at 08:48:08.620Z, so the webview had started
  issuing IPC. `corpus.open.error {error_kind: KeyringUnavailable}` fired at 08:48:07.030Z, beside the boot records,
  1.6 s before the end, and the app kept running after it.
- Control (the base run `ci#37171298288` on `76d6cca`, green): the same `corpus.open.error {KeyringUnavailable}` (the
  runner has no credential store), 41 lines, no `ui.webgpu.adapter` record, and the app ended on the harness's
  SIGTERM (`app.exit {exit_class: signal, signal: sigterm}`).
- History: `boot smoke (ubuntu-22.04)` read success on the 24 CI runs before `a073722`; this is the first failure in
  that window.
- Basis: NOT established. The only source change reaches boot through `Corpus::open` -> the locked fetch, which
  failed into the same `KeyringUnavailable` the base run shows and did not block (same-millisecond timestamps). An
  end with exit 1 and no `app.exit` record is one of the ends that record cannot see (`_exit`, a kill, a pre-sink
  failure; rules/observability.md `app.exit`). Not reproduced here: the host shares ports 4317/4318 with
  conductor-builder and the overseer directed no pulse-app run. A re-run of the failed job (one-sided) or a two-sided
  probe beside a control minted from `76d6cca` is the operator's call.
- Attempt 2 (the overseer re-ran the failed job; gh-verified by the overseer and again at the wrap): `boot smoke
  (ubuntu-22.04)` job 111403021533 success; the run `ci#37189514735` attempt 2 completed/success. Entry 28 re-read at the
  wrap: `ci.py conclusion --sha HEAD --wait 60` -> exit 0, `a073722124b5 verdict: green · checks 13/13 · wall 2279 s`;
  `ci#37189514735` and `secret-scan#37189514733` completed/success.
- Disposition (overseer, 2026-10-04): the attempt-1 red is recorded with its cause NOT established — the first red in
  25 runs, and a one-sided re-run proves nothing about the chunk either way — and kept as a WATCHED residual.

## For the wrap (rulings received after the pass; not acted on here)
- Overseer 2026-10-04: keep the function-scoped `#[allow(clippy::incompatible_msrv)]` in this chunk (Cargo.toml is
  out of its scope); at the wrap, record the stale workspace `rust-version = "1.85"` as a finding (true floor at least
  1.89 for `File::lock`; HEAD's let-chains already needed 1.88).
- Founder ruling 2026-10-04, relayed live by the overseer, for route-resolve: the stale floor gets its own WHAT-only
  entry, "The declared Rust floor matches the code" (raise `rust-version` to the true floor and drop the
  `incompatible_msrv` allow), placed directly before "pre-push:linux runs natively on Linux", so Epoch 4 still closes
  at that entry's wrap (placement by the overseer).
- The boot-smoke red above is being re-run by the operator before the wrap.

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
