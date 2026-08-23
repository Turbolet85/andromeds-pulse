# Session Handoff

**Last Updated:** 2026-08-23T14:00:00Z
**Branch:** chore/migrate-pulse-to-v3 (tracks `origin/chore/migrate-pulse-to-v3`; 21 ahead before this commit — unpushed)
**Status:** clean (wrapped)
**Last Commit:** `feat(2026-08-23-webview-self-verify): an agent presses a real control in the live Tauri window`

## Position
- Done: **2026-08-23-webview-self-verify** — the headful driver gap closed. `cargo xtask webview-drive` presses the production custom-titlebar ✕ in the live Tauri window and asserts the app's own `ui.layout.transition {compact-widget → hidden}`; a RED arm with the capability revoked proves the leg discriminates a dead affordance.
- Next (first markerless): **Integration UX e2e test** (P-076) — now carries the `cargo audit` PREREQ as **pin #11** plus its own 13 CARRYs. Probe point 37 discharged; **next point is 40**, so session 39 owes no probe either.
- Then: A11y verification · Halo canvas disposition · Advisory backlog · **npm advisory coverage** (new this wrap) · Diagnostics un-muting · Staged-bindings assertion (+1 new CARRY) · Metrics label surface · Demo injector.

## Work done
Two new files (`xtask/src/webview_drive.rs` with 9 unit tests · `pulse-app/ui/tests-e2e/webview-drive.mjs`), four modified (`xtask/src/main.rs` · `xtask/src/self_verify.rs` · `pulse-app/ui/package.json`+lock · `pulse-app/ui/eslint.config.mjs`).

**Both driver shapes were probed before either was built, per the directive.** Shape (a) tauri-driver + WebdriverIO WON on measurement and owes no amendment. Shape (b) CDP + Playwright also attaches (all four windows enumerable as page targets) but has no CDP path at HEAD and runs against `test-plan.md:148` — probed, recorded, not adopted; `playwright.config.ts` left untouched and still inert.

**The directive's framing needed one correction that changed what the leg proves.** `core:window:allow-close` is granted today, so the ✕ works — the leg is a **regression guard, not a bug repro**, which is exactly why the mutation check was mandatory. The assertion needed no new obs target and no new allowlist leaf: `ui.layout.transition` already existed with a leaf permitting every field, and `layout_mode_from` names which window closed for free.

**Gates all green:** fmt · clippy `--workspace --all-targets --all-features` · **nextest 1886/1886 + 1 skip** (1877 + 9) · capability-drift clean · capability-widening-check clean · `cargo deny check bans licenses sources` ok · advisories at the 8 owned IDs unchanged · webview lint/typecheck/**801 vitest** · playwright `--list` 33/15 · **self-verify PASS**. Boot smoke fired as the Direct-binary variant four times on separate fresh data dirs.

## Drift resolved
**16 proposals · 15 applied · 1 rejected · 4 escalations resolved · drift = 0 on exit.**
- **arch** — registered `ANDROMEDA_PULSE_MSEDGEDRIVER_PATH` (harness-only class) + a §Stack row for the dev-only GUI harness.
- **security** — narrowed the categorical `ANDROMEDA_PULSE_*_PATH` canonicalize-and-confine ban to **product-binary** reads across all 3 restating sites, carving out harness-only tool locators (operator-approved; the exemption already existed by precedent via `_PIDFILE`/`_LOGFILE` but no doc stated it). Recorded the measured npm advisory-scanning gap.
- **test-plan** — headful agent-driven GUI automation is now IN scope (§2), P5 moved DEFERRED → PARTIALLY DISCHARGED, the shipped runner recorded (no mocha, no `@wdio/*`), §9 applied-as-measured (**not yet CI-wired**), +2 pending-coverage triggers.
- **a11y-plan** — the phantom `button[aria-label="Minimize to tray"]` (**zero occurrences in the codebase**) replaced with the shipped `Minimize` + `Close to tray`, at both restating sites.
- **1 REJECTED on validate check 4:** the detector claimed no npm Dependabot ecosystem; `.github/dependabot.yml` demonstrably configures npm at `/pulse-app/ui`. Only the verified residue was applied.
- Cascade also caught two test-plan sites quoting the security ban over *all* path vars, and correctly left `obs-plan.md:124` alone (it scopes to the product config-load path).

## Notes

- **The RED mutation arm is MANUAL — this is a discipline, not a gate.** Only `--expect-absent` is code; the revoke → rebuild → press → restore cycle was by hand. **No shipped gate catches a left-revoked capability grant** (`capability-drift` parses procedures from `bindings.ts` and never reads grants; `capability-widening-check` covers only the 3 NEVER-widen caps, widening direction only). Recorded three ways so it cannot go silent: test-plan trigger `webview-drive-mutation-arm-not-gated`, a CARRY on the Staged-bindings assertion entry, and a `rules/verification-harness.md` Session Addition.
- **`resolve_msedgedriver` ships with no unit test** — both branches were observed manually but neither is pinned. Trigger `webview-drive-msedgedriver-resolver-unit-coverage`.
- **The version pairing will decay.** msedgedriver **151.0.4129.101** ↔ WebView2 Runtime ↔ Edge, confirmed by build-hash equality with the CDP-reported `WebKit-Version`. The first Windows update that moves the runtime breaks it silently (the driver just refuses the session) and red-flags **both** projects — Conductor's Epoch-5 head reuses this exact install. Recipe + diagnosis in `docs/session-learnings.md`.
- **Self-caught during implement:** the first driver used fixed `sleep(N)` (violating this chunk's own acceptance criterion and racing teardown against the click's IPC) → replaced with bounded polls; and setting the node CWD under the repo made the spawned app emit a stray `pulse-app/ui/ui/src/bindings/index.ts`, since TauRPC dev-mode `export_types()` writes relative to CWD. One level higher would have **clobbered** the real bindings.
- **Disk:** D: hit 0.1 GB of 300 GB mid-gate, surfacing as `rust-lld` exit 1 (the documented signature). `cargo clean -p pulse-app` freed 97.6 GiB. No gate result was accepted from a disk-failed run.
- Curation: T1 ×1 (in-place extension — companion-repo config ≠ host capability) · T2 ×1 · T3 ×1. CLAUDE.md **153/200**.
- Last failed command: none.

## Deferred learnings
None.
