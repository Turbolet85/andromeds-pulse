# arch extract

## Relevance
Relevant — window management (close behavior, tray state), shell verification via IPC introspection, and agent-driven harness infrastructure all within arch's core desktop-shell + cross-cutting patterns domain.

## Constraints
1. Tauri 2.x is the locked desktop shell (per §Stack and Technologies + §Established Decisions); window-close event dispatch is a Tauri-level concern.
2. Window-close behavior MUST follow tray icon policy: "closing the main window minimizes to tray rather than terminating the process (process termination requires the explicit Quit menu item or OS-level kill)" (per §Cross-cutting Patterns § Tray icon policy).
3. OS notification capability (`pulse:notification`) must be reserved outbound-only; no widening beyond the 3 NEVER-widen caps allowed (per chunk scope §Non-goals row + arch §Cross-cutting Patterns § Tray icon policy).
4. If notification-based "still running" signpost is used, MUST honor `notifications_enabled` config opt-out (per §Cross-cutting Patterns § OS notification policy).
5. Self-verify harness uses existing TauRPC introspection endpoints (`app_info`, `health`, `ready`, window geometry from P-061 amendment) per §Standard Contracts; no new IPC procedures required.
6. Harness runs external to binary via script infrastructure per §Infrastructure Patterns CI/CD, matching agent-driven Development Style (per §Cross-cutting Patterns).

## Patterns to follow
1. TauRPC introspection envelope for shell-health assertion: `app_info` (identity), `health` (subsystem liveness), `ready` (ingest readiness) per §Standard Contracts.
2. Tauri window event handler dispatch (Tauri 2.x native, arch prescribes no specific pattern beyond "deterministic action on close").
3. Tray icon as the persistent "still running" surface (already reserved per §Cross-cutting Patterns § Tray icon policy).
4. `ANDROMEDA_PULSE_*` environment variable namespace for any harness-only or runtime override (consistent with chunk #74 amendment pattern for harness-only vars).
5. Script harness existing shape (`scripts/agent-run.{sh,ps1}` boot/run/status/cleanup/logs commands per §Infrastructure Patterns) extended with assert verbs.

## Anti-patterns to avoid
1. DO NOT add new procedures to the TauRPC envelope beyond the existing introspection contract; close behavior is a window event, not a bridged RPC.
2. DO NOT widen `pulse:notification`, `pulse:tray`, or `pulse:plugin-fs` capabilities; the 3 NEVER-widen caps remain outbound-only (per scope §Non-goals + xtask capability-widening-check must stay green).
3. DO NOT bypass notification opt-out configuration or add new capability grants without explicit arch amendment per §Occupied Resources amendment protocol (chunk #74 amendment 1-3 precedent).

## Contract bindings
- **Security**: if notification-based signpost is chosen, security-plan §Anti-Patterns must validate notification text cannot leak sensitive telemetry (standard IPC-bridge binding per amendment history pattern).
- **Tests**: self-verify harness reuses existing a11y/contrast assertion harness (per scope §Deliverable B); tests-plan must own that harness definition.
- **Design**: "still running" signpost UX (notification text if chosen, or tray tooltip persistence) is design-owned; tray menu glyph + locale strings already reserved by design per §Cross-cutting Patterns § Tray icon policy.
- **A11y**: tray menu keyboard navigation + screen-reader labels owned by a11y specialist per §Cross-cutting Patterns § Tray icon policy; self-verify harness must compose the existing a11y-plan §3 contrast/reduced-motion harness.

## Acceptance criteria contributions
1. (arch) Window `CloseRequested` event executes the configured action (hide-to-tray with running-state signpost OR quit, per scope open question #1 P4 decision) and genuinely closes the window without orphaning the process.
2. (arch) Tray→Quit terminates the process cleanly with zero-orphan verification possible via status check on exit code.
3. (arch) Self-verify harness queries IPC introspection (`app_info`, `health`, `ready`) + window-geometry read-back (P-061 amendment) to assert shell health before calling a11y-harness; exits with deterministic status code (0 = pass, nonzero = fail).
4. (arch) If `pulse:notification` capability is used for signpost, fires only once on first close (idempotent per user), respects `notifications_enabled` config, and remains within outbound-only 3-cap constraint per `xtask capability-widening-check`.

## Relevant amendment history
2026-06-29-window-geometry-movable-shell (P-061): registered `window-geometry.json` filesystem location for remembered window positions (used by P-063 chunk's self-verify harness to assert correct placement). No prior amendments to window-close behavior or tray policy (those are original §Cross-cutting Patterns entries, unchanged from genesis).