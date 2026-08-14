# security extract

## Relevance
Partial — a frontend-only (webview) layout/a11y chunk that operates inside the `webview content` threat vector and renders OTLP-derived user-content, so a few preserve-invariant guardrails apply, but it adds zero new security surface (no new boundary/IPC/dependency/secret/log/error) and needs no new security implementation.

## Constraints
- Zero new trust boundary: all four deliverables live in `pulse-app/ui/**` with zero `.rs` delta, adding no OTLP/HTTP/TauRPC/config input surface — so no new §Input Validation boundary row and no serde/post-decode validation is introduced or required (per security-plan §Input Validation boundary table; §Threat Model Summary `webview content` vector).
- CSP preservation: D1 (`max-height`/`overflow-y`) and D3 (label positioning) styling must ship under the existing Tauri CSP — `style-src 'self' 'unsafe-inline'` already permits inline/positioning styles and `script-src` MUST stay `'self'`; CSS-only scroll/label work needs no CSP directive change (per §API Security, CSP row).
- No new capability/IPC: chunk asserts no new TauRPC procedure, `pulse-app/capabilities/` entry, or `pulse://` topic, so capability-authorization + the xtask drift check are preserved by construction (per §API Security, TauRPC capability authorization).
- No OTLP-content leakage: the table + constellation labels render OTLP-derived user-content (service names, span fields) that can incidentally contain secrets; D4's `announce(...)` and any new code MUST NOT emit that content to a log sink or the live region beyond existing UI-state announcements (per §Logging & Monitoring NEVER-log list; §Security Anti-Patterns §Logging).
- No new dependency: scope reuses existing design tokens/CSS ("no new token unless research shows a gap"); adding a crate/JS dep would trip the §Dependency Security cargo-audit/cargo-deny gates — not expected here (per §Dependency Security).

## Patterns to follow
- Style within the existing CSP envelope — reuse the already-accepted `style-src 'unsafe-inline'` for the scroll wrapper and label positioning; do not open `tauri.conf.json` CSP (per §API Security, CSP row).
- Keep the frontend/backend boundary intact — stay in `pulse-app/ui/**`, zero `.rs`, so no §Input Validation boundary and no §API Security capability entry gets registered (mirrors the scope Gate-note cargo-gate deferral).
- Treat OTLP-derived strings (service names in labels, span/row fields) as user-controlled content and render them as escaped text / canvas-text, never as markup, on the first-party webview surface (per §Threat Model Summary — "OTLP attributes are user-controlled content"; `webview content` trust boundary).

## Anti-patterns to avoid
- NEVER loosen `script-src` (no `'unsafe-inline'`), and never route untrusted OTLP content into a style context that would force replacing the accepted `style-src 'unsafe-inline'` with nonces (per §API Security, CSP row).
- NEVER log or announce raw OTLP attribute values / trace-row content / service-name label text — D4 announcements stay UI-state metadata only (sort direction, Errors-only toggle) (per §Security Anti-Patterns §Logging bullet 1; §Logging & Monitoring NEVER-log list).
- NEVER add a TauRPC procedure without a paired `pulse-app/capabilities/` entry — if this "frontend-only" chunk unexpectedly reaches for IPC, register the capability + run the xtask drift check rather than ship a silently-rejected call (per §Security Anti-Patterns §API).

## Contract bindings
- CI security gate ↔ tests §CI Integration: the scope Gate note defers the security gate (`cargo audit` + `cargo deny check bans licenses sources` + capability-drift xtask) for this zero-`.rs` frontend chunk; security owns the gate content, tests owns the one-workflow integration. Deferral is conditional — it re-engages (plus `bindings.ts` regen) if any `.rs` is touched (per §Dependency Security CI integration; §Bootstrap phases `dep-security-ci-gate`).
- No PII-scrubbing binding (no persistence/obs path touched) and no auth binding (auth approach = none).

## Acceptance criteria contributions
- (security) Diff is `pulse-app/ui/**`-only — zero `crates/**`, `pulse-app/capabilities/**`, or `tauri.conf.json` delta (git diff / grep verifies); a violation re-engages capability-drift + input-validation registration (§API Security; §Input Validation; Gate note).
- (security) Tauri CSP unchanged — `script-src` stays `'self'` with no `'unsafe-inline'` added; the new scroll/label styles ship under the existing `style-src` (§API Security, CSP row).
- (security) No OTLP attribute values / trace-row content / label text reach a log sink or the `StatusLiveRegion` beyond the existing UI-state announcements — D4 preserves sort/Errors-only announce semantics only (§Logging & Monitoring NEVER-log list; §Security Anti-Patterns §Logging).

## Relevant amendment history
(none) — no amendment touches the Traces-route webview UI / CSS / announce surface. The two nearest §Input Validation registry-completeness amendments (2026-06-28 `ANDROMEDA_PULSE_L4_DETERMINISTIC`; 2026-06-29 `window-geometry.json` boundary) both registered a NEW deserialized `.rs` input boundary — the exact surface this zero-`.rs` chunk does NOT add — so neither governs here; they only establish why a chunk that adds no new boundary needs no §Input Validation amendment.
