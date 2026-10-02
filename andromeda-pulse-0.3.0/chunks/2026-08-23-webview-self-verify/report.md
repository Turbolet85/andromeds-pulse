# Report — 2026-08-23-webview-self-verify

**Chunk:** Webview self-verify on the Windows host — an agent-driven driver presses a real control in the live Tauri window; the custom-titlebar ✕ is the canonical leg
**Date:** 2026-08-23
**Commits:** (none yet — this wrap's commit is the chunk's first)

## Changes (structured — detectors read this)

- **Files:**
  - NEW `xtask/src/webview_drive.rs` — the leg; owns the verdict by reading the app's own obs log. 9 colocated unit tests.
  - NEW `pulse-app/ui/tests-e2e/webview-drive.mjs` — tauri-driver + WebdriverIO driver; presses the control, asserts nothing.
  - MOD `xtask/src/main.rs` — `mod webview_drive;` + `Cmd::WebviewDrive { expect_absent }` + its dispatch arm.
  - MOD `xtask/src/self_verify.rs` — four helpers widened `fn` → `pub(crate) fn` (no body change).
  - MOD `pulse-app/ui/package.json` + `package-lock.json` — two devDeps.
  - MOD `pulse-app/ui/eslint.config.mjs` — `tests-e2e/**/*.{js,mjs}` added to the existing Node-globals path list.

- **Symbols / APIs:**
  - NEW xtask subcommand `webview-drive` (`--expect-absent` flag) → `webview_drive::run_webview_drive(bool)`. xtask subcommands 18 → 19. **No TauRPC procedure added** — `EXPECTED_PROCEDURES` unchanged, `capability-drift` clean.
  - NEW crate-internal fns in `xtask::webview_drive`: `run_webview_drive` · `run_driver` · `is_widget_hidden_transition` · `unhealthy_records` · `report` · `resolve_msedgedriver`.
  - CHANGED visibility in `xtask::self_verify`: `headless_skip_reason` · `workspace_root` · `locate_pulse_binary` · `read_log_lines` are now `pub(crate)`. **Remaining callers kept:** `run_self_verify` still calls all four; `webview_drive` is an ADDITIONAL caller, not a replacement. Visibility widened within the `xtask` binary crate only — no external API surface exists to change.
  - NEW env var **`ANDROMEDA_PULSE_MSEDGEDRIVER_PATH`** — harness-only, **NOT consumed by the production binary**; read solely by `xtask::webview_drive::resolve_msedgedriver`. Unset / not-a-file ⇒ the leg SKIPs clean (exit 0) printing the fetch recipe; never a hard failure. **Not registered in arch §Occupied Resources → Environment variables** (expected amendment, below).
  - **Ports:** the product binds nothing new. `tauri-driver` transiently binds `127.0.0.1:4444` and msedgedriver `:4445` for the duration of a leg; both are harness-owned, loopback, and released at teardown (verified). No arch port reservation is implied.

- **Crates / modules:** `xtask` gains one module (`webview_drive`). **No workspace crate added / removed** — the reserved 16-member list is unchanged.

- **Dependencies:**
  - ADDED (npm devDependencies, `pulse-app/ui`): `@crabnebula/tauri-driver ^2.0.9`, `webdriverio ^9.31.2` (+132 transitive, devDeps only — no runtime/bundle impact; `ui/dist` unchanged).
  - The win32 native driver binary arrives via `@crabnebula/tauri-driver`'s napi **optional dependency** `@crabnebula/tauri-driver-win32-x64-msvc` — no `cargo install tauri-driver`, no Rust dep.
  - **No Rust dependency added or bumped** — `Cargo.toml` / `Cargo.lock` untouched.

- **Schema / config:** none. No migration, no config key, no violation-schema change. The eslint change is a path-list entry, not a rule change.

- **Spec-master edits:** **none** — implement authored no spec change (correctly; wrap's amendment flow owns them). Three are EXPECTED, listed under *Spec claims disproved by measurement*.

- **Counts / qualifiers moved:**
  - workspace nextest **1877 → 1886** (+9: 9 new `xtask::webview_drive::tests`).
  - xtask subcommands **18 → 19** (`about` strings in `xtask/src/main.rs` are the only enumeration).
  - No count stated in any `.andromeda/` spec master moved.

- **Dev-tool versions:** `msedgedriver` **151.0.4129.101** obtained for this host (matches the installed WebView2 Runtime **and** Edge; its build hash `cc1d9f4080fd9140611a9600b8d1615db310105d` matches the `WebKit-Version` the WebView2 CDP endpoint reports). It lives OUTSIDE the repo (scratchpad) and is referenced by env var — **nothing was committed and no repo path depends on it**.

- **Reverted / negative API facts:**
  - Driver **shape (b)** — WebView2 remote debugging (`WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--remote-debugging-port=N`) + Playwright `connectOverCDP` — was probed, WORKS (CDP answered, all four windows enumerable as page targets), and was **deliberately not adopted**. `pulse-app/ui/playwright.config.ts` is therefore **untouched and still inert** (`testDir: "./tests-e2e"` under `testIgnore: ["**/*"]` — it does not pick up the new `.mjs`, confirmed by `playwright test --list` still reporting 33 tests / 15 files from the a11y config).
  - An early driver revision synchronized with fixed `sleep(3000)`/`sleep(2000)`; **removed** before gating — it violated test-plan §11's `sleep(N)` ban and raced session teardown against the click's IPC. Replaced with bounded polls on explicit signals.
  - An early revision set the node subprocess CWD to `pulse-app/ui`; **removed** — see *Deviations*.

- **Spec claims disproved by measurement:**
  1. **`a11y-plan.md:228` (§1 Critical paths P5)** names the tray-minimize affordance `button[aria-label="Minimize to tray"]`. That string has **zero occurrences anywhere in the codebase** (`grep -rn 'Minimize to tray' pulse-app/ui/src crates/ pulse-app/src` → no hits). The shipped controls are `aria-label="Minimize"` and `aria-label="Close to tray"` (`pulse-app/ui/src/components/WindowControls.tsx`). Cascade sites restating it: `.claude/rules/a11y.md:75`, `.claude/docs/a11y-summary.md:30`.
  2. **`test-plan.md:148` (§2 Agent-runnable invariants)** bans "no Playwright headful mode" among human-in-loop prohibitions, and **`test-plan.md:497` (§6 P5)** calls the headful tauri-driver suite "not agent-driven … documented separately". Both encode the pre-2026-08-22 posture; the operator ruling this chunk implements makes agent-driven GUI verification the default, and the §2/§6 desktop-webview driver row is now **LIVE on this host**, not a declared intent. Measured: a real tauri-driver + WebdriverIO session pressed the production control and the app logged the effect.
  3. **`.claude/rules/verification-harness.md:110`** states the boot-quit self-verify "NEVER CLICKS a user-facing control". Still true *of self-verify*, but the surrounding claim that no harness can therefore prove a live affordance is now narrowed. **This is a rule file, not one of the seven spec masters** — its correction belongs to curation, not the amendment flow.
  4. (plan-level, non-spec) `plan.md` §Constraints cites "the working Conductor precedent". Conductor's own `crates/conductor-tauri/ui/wdio.conf.ts` header declares the suite **"DISPLAY-GATED … runs ONLY on Linux + xvfb … does NOT run on the Windows dev host"** — it evidences the stack is installed and configured, never that it runs here. The shape-(a) lean survived on first-hand measurement instead.

- **Coverage of new surfaces:**
  - `cargo xtask webview-drive [--expect-absent]` (dev/CI harness command; no product surface) → validation `n/a` (no untrusted input) · instrumentation `n/a` (prints to stdout; emits no product telemetry — deliberately, per obs-plan §11's ban on a second observation channel) · PII `n/a` (reads only its own per-run temp log; prints no log content, no paths beyond the binary/data-dir it created) · tests `unit✓` (9 colocated: predicate discrimination incl. boot-geometry and dashboard-hide negatives, `unhealthy_records` both ways, all four `report()` arms) **+ the live leg itself, run in both arms** · a11y `n/a` · tokens `n/a`
  - `ANDROMEDA_PULSE_MSEDGEDRIVER_PATH` (harness-only env boundary) → validation `✓` (trimmed, `is_file()` guarded, absent ⇒ clean skip with recipe — never panics, never a hard fail) · instrumentation `n/a` · PII `✓` (the skip message names the VARIABLE and the fetch URL, never the resolved path; the value is not logged) · tests `✗` — `resolve_msedgedriver` has **no unit test**; it is exercised only by the live leg (both the set and unset paths were observed manually this session, but not pinned) · a11y `n/a` · tokens `n/a`
  - `pulse-app/ui/tests-e2e/webview-drive.mjs` (test harness script) → validation `n/a` · instrumentation `n/a` · PII `n/a` · tests `n/a` (it IS the test driver; its correctness is proven by the two-arm result) · a11y `n/a` · tokens `n/a`
  - **No product UI element, no external product surface, and no hot-path operation was added** — the cross-cutting design/layout/a11y/obs detectors have no new subject in this chunk.

## Deviations from intent

1. **`pulse-app/ui/eslint.config.mjs` edited — not in the plan's Files-to-modify.** Justification: the new Node driver script tripped 17 `no-undef` errors because the config scopes Node globals to an explicit path list (`scripts/**`, `tests-a11y/**`, `*.config.*`). `codebase-research.md` classes a code-side lint allowlist required by an in-scope new file as a **boundary member**, not a gray-area call, so this is in scope. The edit adds `tests-e2e/**/*.{js,mjs}` beside its existing siblings — no rule change.
2. **`pulse-app/ui/playwright.config.ts` NOT edited** — the plan listed it for shape (b), which lost the probe. Leaving the inert config untouched is the correct no-op; recorded so the unused touchpoint is not read as an omission.
3. **New-files paths were probe-determined, as the plan intended.** The plan deliberately left them open pending measurement; they resolved to `xtask/src/webview_drive.rs` + `pulse-app/ui/tests-e2e/webview-drive.mjs`.
4. **Self-imposed acceptance violation caught and fixed mid-implement.** The first driver used fixed `sleep(N)` synchronization, contradicting this chunk's own acceptance criterion ("terminates on an explicit signal … no `sleep(N)`") and racing teardown against the click's IPC. Replaced with a bounded port-poll for driver readiness and a bounded log-poll for the transition.
5. **A stray bindings emission was produced, then removed.** Setting the node subprocess CWD to `pulse-app/ui` meant the app tauri-driver spawns inherited it, and TauRPC dev-mode `export_types()` writes bindings **relative to CWD** — emitting `pulse-app/ui/ui/src/bindings/index.ts`. No gate caught it (`capability-drift` reads the real path and stayed clean); it was visible only as an untracked directory. CWD now points at the per-run temp data dir so the throwaway emission dies with it; the stray tree was deleted and non-recurrence confirmed. Had CWD been one level higher it would have **clobbered** the real bindings instead — the documented recurring hazard.

## Decisions & corrections

- **Driver shape decided by measurement, not by the lean: (a) tauri-driver + WebdriverIO.** Both shapes were probed as the directive required. (a) is the stack both plans pin (`test-plan.md:54/:395`, amendments:18), needs no amendment, and pressed the real control end-to-end. (b) attaches fine but has no CDP path at HEAD and runs against `test-plan.md:148`, so adopting it would owe an amendment (a) does not.
- **The directive's framing needed one correction, which changed what the leg proves.** `core:window:allow-close` **is granted today** (added 2026-06-29), so the ✕ works — the leg is a **regression guard, not a bug reproduction**. That is precisely why the mutation check is mandatory rather than prudent: a green-on-arrival leg that cannot go RED guards nothing.
- **THE MUTATION ARM IS MANUAL — recorded as discipline, not as a gate.** `cargo xtask webview-drive` owns the GREEN leg and runs per-invocation. Only the `--expect-absent` *assertion* is code; the cycle around it (revoke `core:window:allow-close` → `cargo build -p pulse-app` → press → expect absent → restore → rebuild) was performed **by hand this session**. So "`capabilities/default.json` byte-identical after restore" is a **discipline today, not an enforced gate** — and no shipped gate would catch a left-revoked grant (`capability-drift` diffs procedures from `bindings.ts` and never reads grants; `capability-widening-check` covers only the 3 NEVER-widen caps and only in the widening direction). The RED arm is a **one-time discrimination proof**, not a per-run gate.
- **Assertion sharpened by the probe:** boot also emits `ui.layout.transition` (`layout_mode_from: "boot_default"` → `layout_mode_to: "top-right"`), so matching the target alone would pass on a record the press did not cause. The predicate requires `layout_mode_to == "hidden"` **and** `layout_mode_from == "compact-widget"`.
- **The RED arm cannot assert an error line.** The obs extract asked for one; the ACL drops the IPC inside the webview before any Rust boundary is reached, and the historical failure was silent (1719 nextest + boot-quit self-verify all passed against the dead ✕). The RED arm asserts **absence** of the transition. Confirmed live: the button was still found and still pressed successfully — only the effect was missing.
- **No new obs target and no new allowlist leaf were needed** — `ui.layout.transition` already exists with a leaf permitting every field the leg asserts (`observability.rs:975`).
- **`cargo audit` PREREQ (pin #10) carried, not discharged.** Next probe point is session 40; basis re-verified this session (zero dependency delta on the Rust side — `Cargo.toml`/`Cargo.lock` untouched).
- **Host fact for reuse (Conductor's Epoch-5 head reuses this install):** `@crabnebula/tauri-driver` ships the win32 native binary via its napi optional dep — no `cargo install`. msedgedriver must match the installed WebView2 Runtime; fetch `https://msedgedriver.microsoft.com/<version>/edgedriver_win64.zip` and point `ANDROMEDA_PULSE_MSEDGEDRIVER_PATH` at `msedgedriver.exe`. The first Windows update that moves the WebView2 runtime invalidates the pairing for **both** projects.

## Outcome

**Acceptance criteria: met.** The affordance criterion is satisfied by a real headful press of the production control selected by role + accessible name, and the discrimination criterion by the two-arm mutation result.

Two-arm evidence:
```
GREEN  press "Close to tray" on compact-widget →
       ui.layout.transition {layout_mode_from: compact-widget, layout_mode_to: hidden, tray_visible: true}
       clean log — 0 ERROR, 0 app.panic.fatal                                      PASS
RED    core:window:allow-close revoked + rebuilt → control still found, still pressed,
       20s bounded poll → transition ABSENT                                        PASS (discriminates)
RESTORE → byte-identical to HEAD (grant + rationale intact) → GREEN re-confirmed
```

**Gates green** (the plan's `## Test Commands`, all run):
`cargo fmt --check` · `cargo clippy --workspace --all-targets --all-features -- -D warnings` · `cargo nextest run --workspace --profile ci` **1886/1886 + 1 pre-existing skip** · `cargo xtask capability-drift` clean · `cargo xtask capability-widening-check` clean (0 violations / 3 inspected) · `cargo deny check bans licenses sources` **ok** · `cargo deny check advisories` observed separately, **exactly the 8 owned IDs** (0189/0190/0194/0195/0204/0222/0253/0258), unchanged · `npm run lint|typecheck|test --prefix pulse-app/ui` (**801 vitest across 77 files**) · `npx playwright test --list --config=playwright-a11y.config.ts` (33 tests / 15 files — suite-health intact) · `cargo xtask self-verify` **PASS** (shell health + axe 33 + Lighthouse 7 surfaces ≥90 + pa11y 7/7 + regression-detector 0 new violations vs baseline).

**Smoke: fired, and stronger than the standard form.** `pulse-app/capabilities/*.json` is on the boot-smoke trigger list verbatim (`test-plan.md:292`), so the gate was owed; it was satisfied by the Direct-binary variant (`test-plan.md:294`) run **four times** on separate fresh `ANDROMEDA_PULSE_DATA_DIR`s, each a real app boot with a real control press. The GREEN arm carries the 0-ERROR / 0-`app.panic.fatal` clean-log assertion; the RED arm records its expected absence as evidence, per the codified RED-leg carve-out. `bash scripts/agent-run.sh status` ran as the listed harness gate and behaved per its documented caveat (exit 0, `uptime_ms: 0` against a dead system — it evidences the 5-command contract, never that this run was live). Zero orphan processes; `:4317`/`:4318`/`:4444`/`:4445`/`:9222` all released.

**One environmental interruption, resolved:** the D: dev drive hit 0.1 GB free of 300 GB mid-gate, surfacing as `rust-lld.exe` exit 1 across a dozen pulse-app test targets — a linker error carrying no disk wording until `rustc-LLVM ERROR: IO failure on output stream: no space on device` was grepped out. `cargo clean -p pulse-app` freed **97.6 GiB** (14,571 files) and every gate then completed. No gate result was accepted from a disk-failed run.
