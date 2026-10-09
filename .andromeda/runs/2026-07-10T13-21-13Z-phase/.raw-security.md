# security extract

## Relevance
Partial — security governs the second window's `core:window` / `core:webview` capability grants and the capability CI gates; the window rendering, positioning/clamping, cross-window focus, and disclosure a11y are out of the security domain (design/a11y/frontend).

## Constraints — domain rules that apply
- The Findings window's capability additions MUST be negative-default and minimal: grant exactly the `core:window:allow-*` (+ any `core:webview:allow-create-webview-window`) operations this window needs, each with a stated rationale, label-scoped where possible; unlisted IPC is silently rejected at the capability layer (per security-plan.md §API Security "TauRPC capability authorization" — negative-default, exactly-enumerated surface).
- MUST NOT grant Tauri core APIs (`fs`, `shell`, `dialog`, `http`) to the new window's capability without an explicit per-feature capability entry with stated rationale (per §Security Anti-Patterns §API; §Threat Model Summary "webview content" trust boundary — "no fs/shell/dialog/http core APIs granted").
- MUST NOT widen the 3 never-widen caps `pulse:notification` / `pulse:tray` / `pulse:plugin-fs`, and MUST NOT expose `pulse:updater` to the new window's webview JS (per §Security Anti-Patterns §API, lines 404-406) — the chunk restates this explicitly.
- Two-piece enforcement is required: the grant in `pulse-app/capabilities/*.json` AND the window/IPC registration in code; a grant present in code but absent from the capability JSON is a silent runtime rejection (per §API Security "TauRPC capability authorization"; §Bootstrap `dep-security-ci-gate` xtask drift check).
- The new window is a first-party label-render branch loading the existing bundle — the framework-level `tauri.conf.json` `app.security.csp` governs it as-is; NEVER relax `script-src` or add remote content for it (per §API Security CSP row; §Anti-Patterns §API).
- Conditional: if the Findings window persists its own geometry (a new window-label entry in `<data_dir>/window-geometry.json`), the geometry-input boundary applies — integer x/y via `serde`, missing/corrupt → graceful default (non-fatal `unwrap_or_default`), atomic `.tmp`+rename, and NO coordinate values logged (per §Input Validation "Persisted window geometry" row).

## Patterns to follow
- Two-piece capability discipline mirrored from the existing widget/dashboard windows: crate-side registration + matching `pulse-app/capabilities/` JSON, verified by the xtask drift check (§API Security "TauRPC capability authorization").
- Negative-default `core:window` model — `core:default` grants only read-only getters + internal toggle-maximize; every mutation (create/position/show/hide) needs an explicit `core:window:allow-*` grant (chunk §5; consistent with §API Security capability discipline).
- Label-scope the new grants to the Findings window rather than widening a global/shared capability (§API Security exactly-enumerated-surface principle).
- Consume `pulse://stream/incidents` (CARRY subscription) and `incidents.*` as existing first-party outbound Channel surfaces — outbound-only, no new inbound validation boundary introduced (§Input Validation "Tauri IPC `Channel` API streaming payloads" row).

## Anti-patterns to avoid
- NEVER widen `pulse:notification` / `pulse:tray` / `pulse:plugin-fs` beyond their current outbound/loader-only scope (§Anti-Patterns §API, lines 405-406).
- NEVER add a window/IPC capability grant in code without the matching `pulse-app/capabilities/` JSON entry — silent runtime rejection and a hard-to-diagnose UX bug; the xtask drift check is the enforcement (§Anti-Patterns §API, line 402).
- NEVER grant `fs`/`shell`/`dialog`/`http` core APIs (or `pulse:updater`) to the new window's webview without an explicit per-feature capability + rationale (§Anti-Patterns §API, lines 403-404).

## Contract bindings
- Capability CI gate ↔ tests harness: `xtask capability-drift` + `capability-widening-check` (+ `cargo build` ACL compile-embed) run as the security gate job in the single CI workflow (§API Security "TauRPC capability authorization"; §Bootstrap `dep-security-ci-gate`); binds to tests §CI Integration. The chunk's own verification shape already names these gates.
- PII-scrubbing binding: (none new) — incident/corpus content this window re-renders is already PII-scrubbed at persistence per chunk #72 uniform-scrubber coverage (§Data Protection corpus row; §Logging uniform-scrubber framing); this chunk only changes the display container and adds no new disk/clipboard/MCP disclosure path, so no scrubbing site is added.

## Acceptance criteria contributions
- (security) `pulse-app/capabilities/` grants exactly the `core:window` / `core:webview` operations the Findings window requires, each with a stated-rationale comment, scoped to the window label where possible.
- (security) `xtask capability-drift` + `capability-widening-check` pass and `cargo build` (ACL compile-embed) succeeds; no grant exists in code without a capability-JSON entry.
- (security) none of `pulse:notification` / `pulse:tray` / `pulse:plugin-fs` are widened; no `fs`/`shell`/`dialog`/`http` core API and no `pulse:updater` exposure is granted to the new window's webview.
- (security) if a new window-label geometry entry is persisted, a corrupt/missing file falls back to default without panic and no coordinate values are logged.

## Relevant amendment history
- 2026-06-29-window-geometry-movable-shell (P-061) — registered the `<data_dir>/window-geometry.json` input boundary in §Input Validation (integer x/y `serde`, graceful default, atomic write, no coordinate logging). Relevant because this chunk adds a second positioned window in the same movable-shell family and may add a third window-label geometry entry; the bounded-config-input discipline applies verbatim if the Findings window's position is persisted.
- No amendment has altered the Tauri capability-grant rules — the never-widen posture and two-piece enforcement this chunk must honor are original §API Security / §Anti-Patterns §API body truth (the 2026-06-28 v3-normalization entry confirms the capability model as current truth, not a change).