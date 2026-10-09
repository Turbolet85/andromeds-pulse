# security extract

## Relevance
partial — the bulk of the chunk (presentational `EmptyState` component, design tokens, a11y role, render branch) is security-out-of-scope; the only security lens is "new webview visual surface" → CSP + copy-hygiene guardrails.

## Constraints
- Empty-state message + hint MUST be static first-party string literals; no OTLP attribute value, telemetry field, last-seen service name, or raw error detail may be interpolated into the copy — per security-plan §Logging & Monitoring (NEVER-log list) + §Data Protection (snapshot OTLP-attribute leakage class), which applies to any user-visible surface, not just logs/snapshots.
- The Observatory glyph + any font/asset the surface adds MUST resolve first-party under `'self'`/`data:`; introduce NO remote origin/CDN and do NOT weaken `pulse-app/tauri.conf.json` CSP (`script-src 'self'`; never add `'unsafe-inline'` to `script-src`) — per security-plan §API Security (CSP webview row) + its Cross-cutting "no remote content" note.
- No new TauRPC procedure / `pulse-app/capabilities/` entry may be introduced (scope asserts frontend-only, no IPC delta); the negative-default capability model + xtask capability-drift gate MUST stay unperturbed — per security-plan §API Security (TauRPC capability authorization).
- Naming the `:4318` / `:4317` loopback receiver ports in the hint copy is NOT a secret disclosure — they are spec-fixed non-secret loopback ports (absent from security-plan §Secret Management "What counts as secret"); no config/env/secret value beyond these fixed ports may appear in copy.

## Patterns to follow
- First-party-only asset discipline: reuse an existing bundled/inline SVG glyph (scope names `telescope`/`aperture`/`constellation-grid`) rather than a remote-loaded icon/font — matches security-plan §API Security CSP `'self'` origin-matching posture (no CDN).
- Keep loading / error / empty branches distinct (scope's three-state rule): the error branch is the one that already carries sanitized `AppError` copy per security-plan §Error Handling (TauRPC bridge); the new empty branch must render its own static copy, not inherit or echo an error string.
- Static actionable copy mirrors the §Logging NEVER-log stance — the surface names the next step (point an exporter at the port), never the telemetry it is waiting on.

## Anti-patterns to avoid
- Do NOT make the hint "smart" by echoing telemetry — no interpolating an observed service name, attribute value, SQL fragment, URL, or error detail into message/hint (security-plan §Logging & Monitoring NEVER-log list; §Data Protection).
- Do NOT load the glyph/font from a remote URL/CDN or add `'unsafe-inline'`/remote origins to any CSP directive (security-plan §API Security CSP row).

## Contract bindings
Copy-hygiene guardrail is the UI analog of the §Logging NEVER-log stance (obs §PII cross-cut) — design owns the literal port-naming wording, a11y owns the `role` decision (scope OQ#3); security only asserts no-telemetry-interpolation across them. No CI-security-gate, auth, or PII-fixture binding fires (no data on the empty branch, no new gate, no new IPC).

## Acceptance criteria contributions
- (security) Empty-state message + hint are constant first-party strings and the glyph is a bundled first-party asset; DOM/source assertion shows no OTLP attribute / telemetry field / service name / error detail interpolated (per §Logging NEVER-log + §Data Protection).
- (security) No remote origin introduced and `pulse-app/tauri.conf.json` CSP is unchanged — `script-src 'self'`, no `'unsafe-inline'` added, glyph resolves under `'self'`/`data:` (per §API Security CSP).
- (security) No `#[taurpc::procedure]` or `pulse-app/capabilities/` delta appears in the diff; xtask capability-drift check stays green (per §API Security TauRPC capability authorization).

## Relevant amendment history
(none) — no entry in security-plan-amendments.md touches the frontend/webview, CSP, or empty-state/UI-copy area; the nearest entries are backend input-boundary registrations (`window-geometry.json`, `ANDROMEDA_PULSE_L4_DETERMINISTIC`), both outside this chunk's render-branch scope.