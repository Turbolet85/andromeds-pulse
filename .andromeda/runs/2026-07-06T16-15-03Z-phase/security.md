# security extract

## Relevance
Partial (thin / preserve-only) — a frontend-only webview styling/positioning fix that touches the CSP-governed webview surface and renders PII-scrubbed, telemetry-derived findings data, but crosses **no new** security boundary (zero backend, zero new IPC/dependency/secret/logging per scope §Boundaries). Coverage is guardrail-only: don't regress existing controls.

## Constraints
- **CSP stays first-party static; no `script-src` relaxation.** The bounded-popover positioning/background may use first-party static inline styles (within the existing `style-src 'self' 'unsafe-inline'` conscious risk acceptance) but MUST NOT introduce dynamically-generated styles from untrusted input, and MUST NEVER add `'unsafe-inline'` to `script-src`. A layout bug does not warrant any `pulse-app/tauri.conf.json` CSP edit (per security-plan.md §API Security, CSP webview-content row).
- **Webview stays input-free for this surface.** No new TauRPC procedure and no `pulse-app/capabilities/` change may ride along with the fix — matches scope; enforced as a domain ban (per security-plan.md §Security Anti-Patterns §API "NEVER add a TauRPC procedure … without a matching `pulse-app/capabilities/` entry"; §Threat Model webview-content trust boundary = Tauri capability model gates IPC).
- **Preserve the PII-scrubbing boundary at the DOM.** The Findings dropdown renders findings that are PII-scrubbed projections of telemetry; the layout fix MUST NOT newly surface raw/unscrubbed OTLP attribute content in the DOM (e.g., a `title`/tooltip/`aria-label` added to fix overflow that dumps full attribute text) — consistent with scope's "no change to what the dropdown lists" (per security-plan.md §Data Protection corpus row + §Logging uniform-scrubber coverage).
- **No new webview core-API reach.** A CSS/DOM positioning fix must not pull in or widen `fs`/`shell`/`dialog`/`http` core APIs on `pulse:default` (per security-plan.md §Security Anti-Patterns §API "NEVER grant Tauri core APIs … to `pulse:default`").

## Patterns to follow
- **First-party static styling within the accepted CSP** — the panel's existing base style already uses design tokens + static inline positioning (`background: var(--color-raised-2)`, `position: absolute`); implement the bounded/overflow fix with that same first-party-static approach, not untrusted-input-derived CSS (per security-plan.md §API Security CSP row).
- **Negative-default webview trust boundary** — the webview reaches Rust only via the enumerated `pulse:default` procedures; keeping the fix DOM/CSS-only leaves the capability enumeration complete and drift-free (per security-plan.md §Threat Model webview-content trust boundary + §API Security TauRPC capability authorization).
- **Scrub-at-the-source, display-only downstream** — findings arrive already scrubbed via `security::scrubber::scrub_attribute` at persistence boundaries; the UI consumes the scrubbed projection and keeps treating it as display-only text (per security-plan.md §Logging uniform-scrubber coverage).

## Anti-patterns to avoid
- **NEVER relax CSP to make the popover render** — no `'unsafe-inline'` (or any loosening) on `script-src`, and no untrusted-input-derived inline styles to achieve positioning (per security-plan.md §API Security CSP row).
- **NEVER add a TauRPC procedure / `pulse-app/capabilities/` entry** as a side effect of the layout fix (per security-plan.md §Security Anti-Patterns §API) — scope already forbids backend changes.
- **NEVER route raw OTLP attribute / findings content into a logged, clipboard, or tooltip dump** while re-anchoring the popover (per security-plan.md §Security Anti-Patterns §Logging NEVER-log list).

## Contract bindings
- **PII scrubbing ↔ tests (no real PII in fixtures):** if the chunk touches `pulse-app/ui/src/widget/*.test.tsx` or `tests-a11y/axe/p8-findings-dropdown.spec.ts`, findings fixtures MUST be synthetic, not real telemetry (focus-guide cross-domain binding: PII scrubbing binds to obs §PII Scrubbing + tests).
- **CSP background token ↔ design:** the scope's `--color-inset` vs `--color-raised-2` reconciliation is a design decision; security's only tie is that whichever token wins stays a first-party static style (no untrusted-input CSS).
- CI security gate / auth flow bindings: (none) — not touched by this chunk.

## Acceptance criteria contributions
- (security) `pulse-app/tauri.conf.json` CSP is unchanged and no `'unsafe-inline'` is added to `script-src`; popover styling uses only first-party static/design-token styles (diff/grep verifies CSP untouched).
- (security) Diff is confined to `pulse-app/ui/**`: no `pulse-app/capabilities/` change and no new TauRPC procedure (per scope §Boundaries + §API anti-pattern).
- (security) The fix introduces no new `title`/`aria-label`/tooltip (or other DOM sink) that surfaces raw/unscrubbed OTLP attribute values; rendered findings remain the existing scrubbed projection.
- (security) Any dropdown a11y/axe or `*.test.tsx` fixtures use synthetic findings data, not real telemetry.

## Relevant amendment history
(none) — no amendment in `security-plan-amendments.md` touches this chunk's area (webview popover layout, CSP, or the Findings/incidents UI surface). The nearest widget-adjacent entry, `2026-06-29-window-geometry-movable-shell` (P-061, registering the `window-geometry.json` input boundary), is a Rust-side deserialized-file boundary on the movable-window shell, not the frontend popover/CSP surface this chunk changes — out of this chunk's security scope.
