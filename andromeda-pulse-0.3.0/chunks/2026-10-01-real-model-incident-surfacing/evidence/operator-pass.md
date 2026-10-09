# Operator pass (plan entries 25-28), on the overseer's word 2026-10-01

Fired in plan order by /implement on the operator's explicit instruction ("the OPERATOR PASS on
my word, in plan order"); each entry's exact `run`, its exit and its atoms.

## Entry 25 - `python -X utf8 {tools}/gate.py hygiene`
- exit 0; expect `exit 0` + `contains hygiene: clean` -> held.
- `hygiene: clean — read 31 (runs 26 · evidence 5) · trails 13 not read · binary 0 not read by P1`

## Entry 26 - `cargo xtask pre-push:linux`
- exit 0; expect `exit 0` + `contains "verdict": "green"` -> held.
- head `09d08091d392745c38062f0400c2b4bd32a06179` + working tree, synced tree `ac1f63d7421f9a70dbcbf2fd28911a07fc4c282f`.
- stages, all ok: script-modes 88 ms · **source-lint 7358 ms** · npm 18518 ms · clippy 11762 ms ·
  test 63183 ms · ci-gates 1693 ms; `reason: all-stages-ok`, `missing: []`.
- The new sixth stage ran `cargo xtask check:english-sources` in the distro clone and read clean.

## Pre-CI commit
`chore(2026-10-01-real-model-incident-surfacing): operator pre-CI commit` - the whole tree (`git add -A`).

## Entry 27 - `git diff --quiet && git diff --cached --quiet && git push origin chore/migrate-pulse-to-v3`
- exit 0; `09d0809..69f0b93  chore/migrate-pulse-to-v3 -> chore/migrate-pulse-to-v3`.

## Entry 28 - `python -X utf8 {tools}/ci.py conclusion --sha HEAD --wait 2400`
- exit 0; expect `contains verdict: green` -> **did NOT hold**: `69f0b930cf89 verdict: red · checks 13/13
  · first-fail +622 s supply-chain (audit + deny + auditable)`. The run was then followed to completion:
  `ci#36893004900` conclusion **failure**, 11 success / 2 failure.
- Success: lint / test x3 (windows-latest, ubuntu-22.04, macos-latest) · release build x2 · a11y x3 ·
  mcp-server tests · coverage gate. The windows-latest lint/test log carries the verb:
  `check:english-sources: clean (exit 0)`, `"files_scanned": 477`, `"verdict": "clean"`; no
  `UnicodeEncodeError`.

### Red 1 - `supply-chain`, step `cargo xtask check:npm-supply-chain` (not this chunk's)
- Finding: `GHSA-c475-qrg2-pj4r` (`basic-ftp`, high, quadratic-time CPU DoS in `Client.list()`), arm
  `findings-red`, unexcepted; transitive via `get-uri`; lockfile pin `basic-ftp` 5.3.1, vulnerable range
  `<=6.2.0`, a 6.2.1 release exists (a major); `npm audit`'s `fixAvailable` names a webdriverio major
  downgrade (resolver output, not guidance).
- Two-sided basis: `pulse-app/ui/package.json`, `package-lock.json` and `npm-policy.json` are
  byte-identical between the chunk base `09d0809` and `69f0b93` (`git diff --stat` empty); the base's own
  `supply-chain` job (`ci#36858849215`) read **success** at 12:07Z over that same lockfile; the gate reproduces
  locally on this tree (exit 1, same GHSA). The difference is the advisory database's time, not the tree.
- Owner: the wrap's pin (not this chunk's to dispose).

### Red 2 - `boot smoke (ubuntu-22.04)` (runner-only; basis UNOBTAINABLE from this host)
- `agent-run.sh boot` reached `boot: ready (PID=27718)`; `status` then read
  `{"verdict": "not-running", "ended": "exit 1"}` and the step exited 1.
- The app's log (artifact `logs-boot-Linux`): 40 records over 0.27 s, 0 `app.panic.fatal`, ERROR only
  `corpus.open.error {error_kind: KeyringUnavailable}`; it never reached the webview's first IPC.
- Control - the base commit's boot (`ci#36858849215`, success): the same keyring ERROR and the same
  AT-SPI warning, 50 records over 2.2 s until cleanup. The records the base wrote and this run did not
  are all webview-originated (`ui-bridge.ready`, `ui.webgpu.adapter`, `services.list_with_states.request`,
  `connection.current_state.request`, `viz.query.traces`); this run wrote none the base lacked.
- So the process ended during window / WebKitGTK initialisation on xvfb, before the frontend ran. This
  chunk touched no `pulse-app/ui/**` path, no window or capability code, and the bindings are
  byte-identical to the base. The same code booted on the Windows dev host in Slot 2 and ran > 5 min
  with 0 ERROR / 0 panics. `pre-push:linux` carries no boot stage, so no local instrument reproduces the
  runner; the basis is UNOBTAINABLE here and is NOT claimed as "not this chunk's". A single re-run of the
  failed job is the cheap discriminator - the operator's call.

## Both reds solved in-chunk, on the overseer's word ("both reds solved now, not routed")

### Red 1 - the npm advisory, fixed by an override
- Dependency path (`npm ls basic-ftp --all`, lockfile classes): **dev-only**, one chain -
  `lighthouse@13.4.1` -> `puppeteer-core@25.9.0` -> `@puppeteer/browsers@3.2.1` -> `proxy-agent@6.5.0` ->
  `pac-proxy-agent@7.2.0` -> `get-uri@6.0.5` -> `basic-ftp@5.3.1` (declared `^5.0.2`).
- No upgrade path reaches a patched version: the newest `get-uri` (8.0.1) still declares
  `basic-ftp ^5.3.1`, and the advisory covers every 5.x (`<=6.2.0`); the patched release is 6.2.1.
- Fix: `pulse-app/ui/package.json` `"overrides": { "basic-ftp": "^6.2.1" }`, then `npm install` (npm 11.8.0):
  the lockfile delta is the one `basic-ftp` entry, 5.3.1 -> 6.2.1 (6 lines), LF kept.
- Compatibility read before forcing the major: `get-uri`'s FTP path calls `access` / `lastMod` / `list` /
  `downloadTo` / `close`, all present with the same signatures in `basic-ftp@6.2.1/dist/Client.d.ts`;
  6.2.1 has no dependencies and `engines.node >=10`.
- Proof: `npm audit` no longer lists `basic-ftp` (10 high remain, all the pre-existing excepted
  `extract-zip` chain); `cargo xtask check:npm-supply-chain` exit 0, `arm: green-with-dispositions`,
  `distinct: 2` (GHSA-7pqw-9j4j-h8q3, GHSA-jmr9-qjv8-65gv - the standing exceptions).
- Webview gates, now applicable (a `pulse-app/ui/**` path is touched): `npm run lint` 0 · `typecheck` 0 ·
  `test` 0 (82 files / 863 tests) · `build` 0.
- Scope record: `pulse-app/ui/package.json` widening with the overseer's word; the lockfile mechanical.

### Red 2 - boot smoke, the two readings
- Reading 1 (the failed attempt, job 110472991644): the harness:status verdict in the job log is the
  boot-death recorder's line - `"ended": "exit 1"`, `"verdict": "not-running"` (the app's own exit
  code 1, not a signal); artifact `boot.log` holds only the AT-SPI dbind warning; the app log stops
  0.27 s after the first record, before any webview IPC.
- Reading 2 (`gh run rerun 36893004900 --job 110472991644`, one re-run, same sha `69f0b93`; job
  110496296816, 17:31:01Z -> 17:47:18Z): **success** - `boot: ready (PID=7062)`,
  `"ended": null`, `"verdict": "running-healthy"`, `cleanup: clean`.
- What the pair supports: the same commit booted healthy on the re-run, so the exit 1 did not reproduce
  (1 failure in 2 attempts at one sha). It does not establish the cause of the first exit, which stays
  unexplained; no confining mechanism is identified.

## The fix push
- hygiene clean; `cargo xtask pre-push:linux` exit 0, `verdict: green`, all six stages ok (tree
  `3c4317155c11f49d24b5eb735d5c6cce704180ba`; its npm stage ran `npm ci` on the new lockfile).
- second operator pre-CI commit `f37cd3e`; pushed `69f0b93..f37cd3e`.
- Entry 28 re-read: `f37cd3e75978 verdict: green · checks 13/13 · wall 1227 s` -
  `ci#36902837947` success, `secret-scan#36902838296` success.
