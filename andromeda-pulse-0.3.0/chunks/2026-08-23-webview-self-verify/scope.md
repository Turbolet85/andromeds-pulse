# Scope — Webview self-verify on the Windows host

**Marker:** `2026-08-23-webview-self-verify`
**Version:** andromeda-pulse-0.3.0 · Epoch 4 — Polish & ship: verification
**Working entry:** "Webview self-verify on the Windows host — agent-driven affordance proof through the real Tauri window; operator legs reserved for judgment items"

## What this chunk builds

A **headful driver harness** that lets an agent attach to the *real* Tauri window on this Windows
host and **press a user-facing control**, then observe the effect. Today no harness in either repo
can do that — this chunk closes exactly that gap and nothing else.

The deliverable is the driver plus **one canonical proven leg**. It is a capability-enabling chunk:
the entries that follow it (notably the Integration UX e2e test, P-076) convert their UI legs from
OPERATOR legs to AGENT legs by consuming this driver.

## Why the gap exists (verified at HEAD, not taken from the entry on trust)

Both halves of the working entry's CONTEXT reproduced exactly:

- `cargo xtask self-verify` is **boot-quit by construction and never clicks** — verified verbatim at
  `.claude/rules/verification-harness.md:110`: it "proves the shell BOOTS (window shown + receivers +
  heartbeat + zero panics) but it NEVER CLICKS a user-facing control (it hides/quits programmatically)".
- The Playwright a11y suite **drives the SPA in Chromium against an http-server `dist`, never the app** —
  verified at `pulse-app/ui/playwright-a11y.config.ts`: `webServer.command` is
  `npx http-server dist -p 4173 --silent -P http://localhost:4173?`, `baseURL http://localhost:4173`,
  single project `browserName: "chromium"`.

So the gap is the DRIVER, not the intent. Confirmed.

## The two candidate driver shapes (research owns the pick — neither is decided here)

Both are **unmeasured on this host**. The chunk measures both before planning around either: one probe
per shape, attach to the real window, press one control.

**(a) tauri-driver + WebdriverIO.**
- Named by this project's own test-plan as the desktop-webview E2E stack: `.andromeda/test-plan.md:54`
  (§2 desktop-webview row — "tauri-driver (Tauri CLI test harness) with WebDriver protocol") and
  `:395` (§9 — "`tauri-driver` 2.x + `WebdriverIO` 9.x + `mocha`"); `test-plan-amendments.md:18`
  records the same pin.
- The companion Conductor repo **already carries it installed and working** — verified at
  `conductor/crates/conductor-tauri/ui/package.json`: `@crabnebula/tauri-driver ^2.0.9`,
  `webdriverio ^9.29.1`, `@wdio/{cli,globals,local-runner,mocha-framework,spec-reporter} ^9.29.1`,
  `@axe-core/webdriverio ^4.12.1`, with `node_modules/` populated, a `wdio.conf.ts`, and an
  `a11y = wdio run wdio.conf.ts` script. *(Note: this manifest lives at `crates/conductor-tauri/ui/`,
  NOT the repo-root `ui/` — a narrower glob returns a false negative.)*
- Windows support is via **msedgedriver matched to the installed WebView2 runtime**; macOS unsupported.

**(b) WebView2 remote debugging + Playwright `connectOverCDP`.**
- `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=N`, then attach Playwright over CDP.
- Reuses the stack Pulse already has — verified at `pulse-app/ui/package.json`:
  `@playwright/test ^1.49.0`, `@axe-core/playwright ^4.11.0`, `playwright.config.ts` +
  `playwright-a11y.config.ts` both present. **VERIFIED — no CDP-attach path exists at HEAD**: zero hits
  for `connectOverCDP` / `remote-debugging` / `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS` across
  `pulse-app/` + `xtask/` + `scripts/`. `playwright.config.ts` is moreover **inert** — `testDir:
  "./tests-e2e"` names a directory that does not exist, under `testIgnore: ["**/*"]`. A reserved-but-
  disabled E2E surface, which is where shape (b) would land.
- **Counter-weight found at P3, recorded here because it bears on the pick:** `test-plan.md:148`
  (§2 Agent-runnable invariants) bans "no Playwright headful mode" in the same clause as human-in-loop
  verification and manual screenshot comparison. A literal reading is adverse to shape (b), so (b)
  winning on measurement owes a test-plan amendment that (a) does not.

**Host facts measured this session** (relevant to both shapes):
- `tauri-driver` — **absent from PATH**.
- `msedgedriver` — **absent from PATH**.
- WebView2 Runtime installed: **151.0.4129.101** (also .78) at
  `C:\Program Files (x86)\Microsoft\EdgeWebView\Application`. Shape (a) needs an msedgedriver
  matching this version line.

Whichever shape lands **installs HOST-WIDE** — Conductor's Epoch-5 head is the same entry and reuses
it — so the chunk records the install recipe in its report as a **host fact, not a project one**.

## The canonical leg — and a premise correction the directive's framing needs

**The leg:** click the custom-titlebar **✕** and observe the effect. Not a synthetic button.

Verified live, so the leg is real:
- The control is a native `<button aria-label="Close to tray">` at
  `pulse-app/ui/src/components/WindowControls.tsx` (class `window-control--close`), calling
  `useWindowControls().close()` → `getCurrentWindow().close()` (`hooks/use-window-controls.ts:19`).
- It **has a production render site** (the 2026-08-21 orphan-check applied): `WindowControls` is mounted
  at `components/Titlebar.tsx:166`, and `Titlebar` is mounted by BOTH `dashboard/router.tsx:150` and
  `widget/CompactWidget.tsx:73`. Not an orphan like `HaloCanvas`.
- Platform gate passes here: `usePlatform()` is UA-based and returns `"windows"` on this host, so the
  three buttons render (macOS returns `null`).

**PREMISE CORRECTION — the ✕ is NOT dead at HEAD.** `verification-harness.md:110` records it as
matrix-`verified` yet doing NOTHING because `getCurrentWindow().close()` was silently rejected by the
negative-default ACL. That was true **at chunk `2026-06-29-window-size-constraints`**. It has since been
fixed: `pulse-app/capabilities/default.json` **now grants `core:window:allow-close`** (added 2026-06-29
per its own rationale text, alongside `allow-start-dragging` / `allow-minimize` / `allow-toggle-maximize`).

Consequence for this chunk — the outcome the directive names is unchanged, the mechanism is not:
- The leg is a **REGRESSION GUARD** (the ✕ works today and must keep working), **not** a live
  bug reproduction. The historical dead affordance is the *motivating evidence* for why the driver is
  needed, not a currently-red condition.
- Because it is green-on-arrival, the leg **MUST be mutation-checked to prove it discriminates**:
  revoke `core:window:allow-close`, rebuild, and confirm the leg goes RED — otherwise it is a leg that
  would pass against a dead affordance, which is precisely the failure class this chunk exists to end.
  This mutation check is not named by the working entry; it follows from the correction — and P3
  confirmed it is **load-bearing rather than merely prudent**: `capability_drift` diffs procedures
  from `bindings.ts` ARGS_MAP against `EXPECTED_PROCEDURES` and **never inspects grants**, while
  `capability_widening_check` covers only the three NEVER-widen caps and only in the widening
  direction — so a revoked `core:window:allow-close` left in the tree passes BOTH gates. The restore
  therefore needs an explicit byte-level assertion; no existing gate covers it.

**Observable precision** (the effect is a HIDE, not an exit — `pulse-app/src/window.rs:179`):
`CloseRequested` → `api.prevent_close()` → hide. Per `window.rs:141-142`, closing the **widget** takes the
app to the tray (hides both windows + emits `tray.signpost.shown`); closing the **dashboard** merely
collapses back to the widget (no extra hide, no signpost). The leg asserts the window becomes hidden —
never that the process exited.

**P3 sharpened this into the exact assertion, and it needs nothing new.** `handle_close_to_tray`
(`window.rs:119-129`) emits `target: "ui.layout.transition"` with `layout_mode_from` (the sanitized
window label), `layout_mode_to = "hidden"`, `tray_visible = hide_ok`, on EVERY close of either window;
the allowlist leaf at `observability.rs:975` permits exactly those fields, so all three survive
un-redacted. **No new tracing target and no new allowlist leaf are needed.** `layout_mode_from` also
names WHICH window closed, discharging the layouts requirement to record the attached surface for free.
The RED arm asserts the **ABSENCE** of that line: with the grant revoked, the ACL drops the IPC inside
the webview before any Rust boundary is reached, so `CloseRequested` never fires — and per
`verification-harness.md:110` the historical failure was silent (1719 nextest + the boot-quit
self-verify all passed against the dead ✕), so the RED leg must NOT be planned around an error line.
`tray.signpost.shown` (`window.rs:166`) is a usable secondary presence signal for the widget-close path
only — it has no allowlist leaf and `for_target` resolves `None` for it, but it emits zero fields, so
nothing is lost and target + message still reach the log.

## Boundaries

- **Claims no capability.** The driver has no capability of its own; **P-076 is its consumer and claims
  at ITS chunk**. The verification-matrix link step is a deliberate no-op here and the coverage gate
  no-ops — the fifth consecutive no-claim, which is correct.
- **Operator legs stay reserved for judgment items** ("does it LOOK right"), per the 2026-07-05 learning
  that an obs-log smoke is layout-blind and only a rendered-pixels check catches "it renders, but wrong".
  This chunk does not try to automate aesthetic judgment.
- **Not** the Integration UX e2e test itself (P-076), **not** the a11y verification entry, **not** the
  Halo canvas disposition — those are separate markerless entries downstream of this one.
- No production Rust/TS behaviour change is expected, and P3 found none indicated — no `crates/**` or
  `pulse-app/src/**` edit falls out of the research. If a probe nonetheless requires a change to make the
  app drivable (e.g. a test-only env gate), that surfaces at implement, not here — and arch bans a
  test-mode back-door, so such a change is argued, never assumed.

## Surfaces / contracts touched

- **VERIFIED — the extension point is `xtask/src/self_verify.rs`** (438 lines): it already launches the
  real dev binary under a fresh `ANDROMEDA_PULSE_DATA_DIR`, polls `:4317`/`:4318` for readiness, globs
  the date-suffixed log family, composes the a11y harness, and cleans up with a zero-orphan check. It
  has every piece except the press. Threading sites for a new subcommand are the `Cmd` enum and its
  `match` arm, both in `xtask/src/main.rs`. Whether the press itself lands JS-side (`pulse-app/ui/`
  devDeps + a driver config/spec) is probe-determined.
- Note `headless_skip_reason` (`self_verify.rs:279-292`) skips only on display-less **Linux** — a
  Windows host never skips, so the new leg runs for real here.
- Read-only against: `pulse-app/capabilities/default.json` (mutation-check target, restored after),
  `pulse-app/src/window.rs` (close semantics), the `agent-latest.jsonl*` date-suffixed obs-log family
  (per `verification-harness.md` 2026-06-29 — the glob is mandatory; a bare-name reader silently finds
  nothing).
- `.claude/rules/verification-harness.md` — the rule file whose :110 entry motivates the chunk;
  **VERIFIED** — its "never clicks" claim narrows once a driver exists. P3 found **two further spec
  sites in the same class**, so this is a cluster, not a one-liner, and all are wrap-time amendments
  rather than phase edits: (1) `test-plan.md:148` bans "no Playwright headful mode" and `:497` calls the
  headful tauri-driver suite "not agent-driven" — both encode the pre-2026-08-22 posture this entry's
  operator ruling supersedes, so an amendment is owed whichever shape lands; (2) `a11y-plan.md:228`
  (§1 P5), `.claude/rules/a11y.md:75` and `.claude/docs/a11y-summary.md:30` all name
  `button[aria-label="Minimize to tray"]`, a string with **zero occurrences anywhere in the codebase**
  — the shipped labels are `"Minimize"` and `"Close to tray"`. A three-site stale accessible name,
  directly relevant because this leg keys on one.

## Folded annotations

**PREREQ — `cargo audit` (pin #10).** Standing deferral since `2026-08-15-corpus-key-persistence`,
ratified 2026-08-16, every-3rd-wrap re-run INTERVAL; migrated onto this entry at the 2026-08-23
adaptation wrap (origin preserved). Probe points ran at sessions 25/28/31/34/**37**; **point 37 fired in
full form and DISCHARGED** (probe unchanged, 5th consecutive), so **the next point is 40 — this session
owes no probe**. Basis unchanged: the RustSec DB cannot LOAD (`parse error: duplicate advisory ID:
RUSTSEC-2026-0244`), upstream and unclearable by any repo change. Overlap `cargo deny check advisories`
stable at the eight owned IDs (0189/0190/0194/0195/0204/0222/0253/0258), owned by the Advisory backlog
entry and never ignore-listed; pass/fail half `cargo deny check bans licenses sources` ok. This chunk
carries the pin forward; it does not discharge it.

No `CARRY:` and no `BLOCKED-ON:` annotations on this entry.
