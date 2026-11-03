# Implement outcome — 2026-11-03-advisories-since-2026-10-12-closed

**Outcome: green + smoke.** Plan rev 4, the founder's four forks (`inputs#I2`). One firing of the block
(`.andromeda/runs/2026-11-03T20-59-06Z-implement/gate-2026-11-03-advisories-since-2026-10-12-closed.json`):
`entries 37 · green 33 · red 0 · recorded 0 · timeout 0 · not-run 4` — the four `leg = 'operator'` entries
(34 hygiene, 35 merge-base, 36 push, 37 CI read) are the operator pass's, not fired. No entry re-run, no fix
iteration, no deferral (Rust and npm both changed, every gate ran).

Next: the OPERATOR PASS — entries 34, 35, 36, 37 (hygiene, merge-base probe, the build-branch push after the
operator pre-CI commit, the CI read) — and only then `/andromeda-wrap-session`.

## Files changed (5, all in research's lists)
- `Cargo.toml` — wasmtime `"48.0.4"` → `"48.0.8"`, one dated provenance paragraph naming RUSTSEC-2026-0353 … -0359,
  -0361 … -0364, -0378, -0379, -0422 … -0424 (`>=48.0.8, <49.0.0`; 49.x needs rustc 1.96).
- `Cargo.lock` — measured moves: wasmtime / pulley 48.0.5 → 48.0.8, cranelift 0.135.5 → 0.135.8 (28 packages);
  rustls 0.23.46, rustls-webpki 0.103.16, quinn-proto 0.11.19 (+ chacha20 0.10.2, rand 0.10.3, rand_core 0.10.1,
  rand_pcg 0.10.2), webpki-roots 1.0.11, webpki-root-certs 1.0.11, pulldown-cmark 0.13.6, zstd 0.13.4 / zstd-safe
  7.4.0 / zstd-sys 2.1.2, tokio 1.54.1, event-listener 5.4.2; duckdb / libduckdb-sys 1.10506.1, zip 6.0.0 → 9.3.0
  (+ nix 0.31.3, typed-path 0.13.0); tauri-plugin-updater 2.14.1 with tauri 2.12.5, tauri-build / -codegen / -macros
  2.7.4, tao 0.37.2, wry 0.57.1, tray-icon 0.25.1, urlpattern 0.6.0 (unic-* ×5, phf 0.11 ×4, png 0.17, zip 4.6.1
  removed); notify-rust 4.18.2 (see deviation 1). Every forecast of steps 1–4 matched the applied update.
- `deny.toml` — pruned: the ten withdrawn GTK3 ignores and the five unic-* ignores; the `zip`, `phf`,
  `phf_generator`, `phf_macros`, `phf_shared`, `png` and `windows-link` skips. Added: RUSTSEC-2026-0370 / -0371 /
  -0372 under one provenance comment (holders, owner this marker, closing condition, the founder's word). No skip
  added. `multiple-versions = "deny"` and the tonic canary untouched.
- `pulse-app/ui/package-lock.json` — `npm update vite vitest @vitest/mocker undici follow-redirects qs seroval
  ip-address` (each patched release confirmed with `npm view` first): vite 7.3.7, vitest / @vitest/* 4.1.12,
  undici 6.29.1, follow-redirects 1.16.2, qs 6.17.1, seroval 1.7.7, ip-address 10.7.6, chai 6.3.0,
  tinyrainbow 3.2.0. No `--force`, no downgrade; `package.json` unchanged.
- `pulse-app/ui/npm-policy.json` — three `advisory_exceptions` (GHSA-g3hv-75wg-m249 minimatch, GHSA-3g3m-2fjr-6pw4
  glob, GHSA-mrhx-c7xc-m3w5 zod), owner this marker, added 2026-11-03; holders in each `reason` re-derived from the
  updated lockfile and the holders' latest releases (`npm view`).

## Key readings
- Entry 1 `cargo audit`: exit 0, `Loaded 1403 security advisories` (fetched, `~/.cargo/advisory-db`, `$CARGO_HOME`
  unset), `warning: 12 allowed warnings found`, no vulnerability. Distinct ids: RUSTSEC-2025-0141 (bincode),
  -2024-0436 (paste), -2024-0370 (proc-macro-error), -2026-0371 / -0370 / -0372 (zbus family, each at two
  versions), -2024-0429 (glib, unsound) — every one in the base set; no new id. Yanked: async-executor 1.14.0,
  clipboard-win 5.4.1 — unchanged by the updates.
- Entry 9 `cargo deny check bans licenses sources`: green, no `unnecessary-skip`.
- Entry 10 `cargo deny check advisories`: `advisories ok`, exit 0 (not asserted), no `advisory-not-detected`.
- Entry 11 `cargo xtask check:npm-supply-chain`: exit 0, `arm: green-with-dispositions`, `unexcepted: []`,
  licenses 883 checked / 0 violations. The residual before the exceptions (`npm audit`) was exactly the two
  extract-zip ids already excepted plus the three founder-ruled ones.
- Entries 30 / 31 / 32: the window boot series settled both boots on Tauri 2.12.5 (246.88 s); the console engine
  cycle passed with the door listening (48.9 s); `pre-push:linux` green, all stages ok (123.08 s); entry 33 read
  the cycle pid file gone and both harness ports refusing.
- Entries 23 / 24: the bindings regen ran last of the cargo-adjacent steps and equal the chunk base.

## Deviations from the plan
1. **Step 6 — no Windows-target skips; one extra in-range lockfile update instead.** The diagnostic
   `cargo deny check bans licenses sources` after steps 1–4 printed the predicted eight `error[duplicate]` rows
   (`windows`, `windows-collections`, `windows-core`, `windows-future`, `windows-numerics`, `windows-result`,
   `windows-strings`, `windows-threading`). Their second copy (0.61) was held by tauri-winrt-notification 0.7.3
   (pinned `0.7` by notify-rust 4.17.0) and iana-time-zone 0.1.65 (whose range is `>=0.56, <=0.62`).
   tauri-winrt-notification 0.8.1 requires windows 0.62, and notify-rust 4.18.2 lies inside
   tauri-plugin-notification 2.3.3's own requirement: `cargo update -p notify-rust` (notify-rust 4.17.0 → 4.18.2,
   tauri-winrt-notification 0.7.3 → 0.8.1, mac-notification-sys 0.6.12 → 0.6.15) removed the whole windows 0.61
   family and windows-link 0.1.3. A skip must state why its duplicate is not source-fixable
   (security-plan §Dependency Security, the carve-out shape), and this one was, so the update was taken and no skip
   was written; windows-link's skip, now on a single version, was pruned. zbus versions unmoved; no manifest line.
   Justification: Cargo.lock is a listed file, the update is in-range with no code change, and the plan's
   constraint against relaxing `multiple-versions` is honoured more strictly, not crossed.
   **Accepted** by the overseer (founder-delegated), `inputs#I3`: «The notify-rust 4.18.2 move (in its holder's
   range, lockfile only, no skips) is accepted as a technical step, mine.» The same word sends the
   `cargo deny check advisories` designed-red line (Surfaced, below) to the wrap.
2. **Step 6 forecast vs measurement:** `ctor` 0.8.0 → 1.0.13 moved under Tauri 2.12 but stays two versions
   (0.2.9 under specta, 1.0.13 under tauri-utils), so its skip stands.
- No scope-record line: every changed file is in research's lists (`gate.py scope`: `clean — changed 5 · listed 5`).

## Surfaced (for the wrap — nothing authored)
- `cargo deny check advisories` now exits 0 (`advisories ok`). rules/security.md (2026-08-17 Session Addition) and
  the plan's entry note describe it as designed-red (the standing GTK3 glib unsound id); at the base, P5 measured
  exit 1 with the ten `advisory-not-detected` rows. After the prune it reads green, so the "designed-red" sentence
  may no longer hold of this tree — the wrap's reconcile decides whether a master or a leaf names it.
- security-plan §Security Anti-Patterns → Input's sentence that zip is not in the dependency graph was already
  false (the plan's expected amendments); zip 9.3.0 now stands as the one version.

## Guard entries
- none in the plan.

## Process hygiene
| process | started by | final state |
|---|---|---|
| none started outside the gate tool's own entries | — | `gate.py census` (pulse-app, andromeda-pulse-engine, andromeda-pulse, Xvfb, xvfb-run, WebKitWebProcess; args target/boot-smoke, target/engine-cycle, target/pre-push): `census: none running` |

The boot series, engine cycle and pre-push entries started and ended their own processes; entry 33 read the
pre-push cycle's pid file gone and ports 24317 / 24318 refusing.

## Decisions & corrections
- Bash guard refused three calls, each re-issued once in the form it passes: two leading `cd` into another
  directory (re-issued as absolute paths / a subshell), one `cat` heredoc to a file (re-issued through Write).
- Two provenance sentences drafted before measuring were false and corrected by reads before any gate ran: "both
  windows 0.61 holders' latest releases still require 0.61" (tauri-winrt-notification 0.8.1 requires 0.62;
  iana-time-zone admits `<=0.62`) — which led to deviation 1; and zod held "under webdriverio 9" (the chains are
  pa11y-ci 4 / pa11y 9 / pa11y 10 / lighthouse 13 through puppeteer). Standing learning met: verify a holder claim
  from the lockfile and the registry, not recall.
- The overseer's word (founder-delegated) bound this run: each gate once, no extra runs; stop after the gates, the
  operator pass on his word. Diagnostic runs the plan's steps direct (`cargo deny` in step 6, `npm audit` for step
  7's residual) were made once each before the block; no gate entry was fired twice.
