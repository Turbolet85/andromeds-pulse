# Scope — 2026-06-29-predictable-close-self-verify

**Marker:** `2026-06-29-predictable-close-self-verify`
**Version:** andromeda-pulse-0.3.0 · Epoch 2 (Window & shell hygiene)
**Capabilities:** **P-063** (Predictable close + honest tray) + **~P-078** (NEW — agent-headful self-verify harness; authored this chunk)
**Source:** working-route Epoch-2 entry "Predictable close + honest tray … (P-063 · intent F3) · /phase to fold in a new agent-headful-self-verify cap (~P-078) when planning" → intent §4 F3 + the dogfood rationale.

---

## Why these two are ONE chunk
The close bug is the *cause* of the missing self-verify: the running app does not quit cleanly (X / taskbar-close do not terminate; only Tray→Quit or a kill stops it), so a headful boot→verify→quit harness would orphan the process on Windows. That is exactly why **boot-smoke is skipped every UI chunk** today (the Windows GUI-orphan hazard). Fixing the close behavior (P-063) is the prerequisite that *unblocks* a clean-quitting self-verify harness (~P-078). Landing both together, FIRST in the UI run, means every later Epoch-2/3 UI chunk can be autonomously boot→assert→quit verified instead of shipped visually unseen (v0.2.0 passed 1676 tests yet shipped illegible).

---

## Deliverable A — Predictable close + honest tray (P-063)

**What it builds:** a predictable, signposted window-close. Window-close (the titlebar close button, the OS taskbar close, Alt-F4) performs ONE configured, honest action; the dashboard window is genuinely closable; the tray is the single honest "still running" surface; tray→Quit is the one true terminate path.

- The architecture already mandates the default: arch §Tray icon policy — "closing the main window minimizes to tray rather than terminating the process (process termination requires the explicit Quit menu item or OS-level kill)." So the **expected default = minimize/hide-to-tray on close**, NOT terminate.
- F3's gap is that this is currently broken AND unsignposted: the window does not hide and there is **no clear indication it is still running**. The fix makes close actually hide-to-tray AND adds a clear "still running in the tray" signpost.
- The **prior chunk (P-061) granted the `allow-close` capability** (window close button fires) but explicitly deferred the close *behavior* to P-063 (per the P-061 matrix note + the 2026-06-29 capability learning). This chunk owns the behavior + the signpost, not the capability grant.

**Boundaries (A):** does not redesign the tray menu surface (arch §Tray policy already fixes its actions: open/focus, ingest summary, snapshot, quit); only ensures the tray honestly reflects running state and is reachable after close. No new tray *actions* beyond what arch already reserves.

## Deliverable B — Agent-headful self-verify harness (~P-078, NEW cap)

**What it builds:** a MINIMAL, agent-runnable, headful self-verification path: **boot the real app headful → assert the window is up + correctly placed/rendered + key interactions respond → run the existing a11y/contrast assertion harness → clean quit (zero orphan process)**. Machine-parseable pass/fail, deterministic exit code (agent-driven Development Style per arch §Cross-cutting Patterns). It composes the existing pieces rather than inventing a new test framework:
- the existing **5-command agent harness** `scripts/agent-run.{sh,ps1}` (boot/run/status/cleanup/logs) per arch §xtask + verification-harness rules;
- the existing **a11y/contrast harness** (a11y-plan §3 — axe-core / Lighthouse / pa11y / Playwright / colorjs.io; SC 2.3.3 reduced-motion);
- the existing IPC introspection envelope (`app_info` / `health` / `ready`) + window geometry (P-061's `window-geometry.json` + the centered/remembered logic) as the assert surface.

**Boundaries (B) — this is the MINIMAL self-verify, deliberately not the full e2e:**
- NOT the full integration-UX e2e (launch→telemetry→Traces→storm→incident→Investigate) — that is **P-076** (Epoch 4), which already CARRIES the headful drag-delta e2e (P-061 residual) + the deferred Investigate Playwright axe spec (P-072 residual).
- NOT a live titlebar drag-delta assertion (P-061's residual → P-076).
- The harness asserts *the shell is healthy and the app quits cleanly*, giving later chunks a reusable boot→verify→quit gate; it does not exercise the data/AI-debug path.

---

## Surfaces & contracts touched (WHAT, not HOW)
- `pulse-app/src/main.rs` — the Tauri window `CloseRequested` event handler (close → hide-to-tray + signpost; tray→Quit → terminate). This is the chunk's behavioral core.
- `pulse-app/capabilities/*.json` — `allow-close` already granted (P-061); the signpost may need `pulse:notification` (arch §OS notification policy reserves it). No widening of the 3 NEVER-widen caps (`pulse:notification`/`pulse:tray`/`pulse:plugin-fs` stay outbound-only) — `xtask capability-widening-check` must stay green.
- The tray surface (arch §Tray icon policy) — honest running-state reflection; Quit as the terminate path.
- The "still running" signpost — an explicit OS-notification decision (arch §OS notification policy: "no other subsystem emits notifications without an explicit decision") and/or a persistent tray title/tooltip. Honors the `notifications_enabled` opt-out.
- The self-verify harness — `scripts/agent-run.{sh,ps1}` and/or an `xtask` verb and/or a tauri-driver headful spec (mechanism is a P4 decision); reuses the a11y/contrast harness from a11y-plan §3.
- `verification-matrix.json` — link P-063 to this chunk; author the new P-078 entry (verification ledger, not a spec). requirements.md authoring of P-078 is a P5-surfaced decision.

## Non-goals
- Window size constraints / aspect lock (P-062 — next chunk).
- Browser-chrome / context-menu / canvas-image suppression (P-064/P-065).
- Widget→dashboard navigation (P-066).
- Real-model L4 judgment quality (explicitly out of scope per intent §5).
- The full integration-UX e2e + the carried drag-delta + Investigate axe specs (P-076 / Epoch 4).

## Open questions (resolve at P4 via AskUserQuestion)
1. **Close behavior** — minimize/hide-to-tray (arch default) with a first-close "still running" signpost · vs quit-on-close · vs a `Settings.close_to_tray` toggle (default hide-to-tray). Arch §Tray policy leans hide-to-tray; F3 permits either.
2. **"Still running" signpost form** — OS-notification toast on first close (pulse:notification, respects `notifications_enabled`) · vs a persistent tray tooltip/title · vs both.
3. **Self-verify harness mechanism + extent** — extend the `agent-run` script harness asserting via IPC/geometry · vs an `xtask self-verify` verb · vs a tauri-driver headful WebDriver spec; and how minimal (shell-health + clean-quit only, vs +basic render/key-interaction assertions). The a11y/contrast reuse is fixed; the driver is the open choice.

## Acceptance shape (high-level; refined in plan.md)
- Window-close performs the configured action (hide-to-tray with a visible/clear running indication, OR quit per the P4 decision); the dashboard window genuinely closes; tray→Quit terminates the process with no orphan.
- The agent-headful self-verify harness boots the real app, asserts shell health (window present + placed + rendered + a key interaction responds) + runs the a11y/contrast harness, and quits cleanly with a deterministic pass/fail exit — runnable by an agent unattended.
- No regression: `xtask capability-widening-check` + `xtask capability-drift` stay green; the 3 NEVER-widen caps unwidened.
