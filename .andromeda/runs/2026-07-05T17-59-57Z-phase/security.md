# security extract

## Relevance
partial — the chunk's built surfaces (labels/layout, health palette, non-color cue) are design/a11y (out of security domain), but it renders untrusted OTLP-derived service-name strings in the webview and may conditionally add a TauRPC field.

## Constraints
- Service names shown on/near dots are OTLP `resource` attribute values = user-controlled, untrusted content that can carry secrets/URLs/IDs; treat the label string as untrusted at the render site (per security-plan §Threat Model Summary — "OTLP attributes are user-controlled content").
- The webview CSP must not be loosened for labels or the non-color cue: `script-src 'self'` (no `'unsafe-inline'`), no remote content (no CDN fonts/icons); any icon is a first-party asset or a `data:` URI only (`img-src 'self' data:`); `worker-src blob:` already covers the WebGPU canvas (per security-plan §API Security, CSP row).
- If the DOM/HTML-overlay label path is chosen (open question #2), the untrusted service name MUST reach the DOM only as escaped React text — never via `innerHTML`/`dangerouslySetInnerHTML` or interpolation into a style/attribute context; canvas `fillText` is inherently non-HTML-parsed and safe (per security-plan §API Security CSP `'unsafe-inline'` conscious-risk note + §Security Anti-Patterns § Code Patterns).
- No new logging of the service-name string or any raw OTLP attribute value from the renderer, `use-service-constellation.ts`, or any touched Rust handler (per security-plan §Logging & Monitoring "What NEVER to log" + §Security Anti-Patterns § Logging bullet 1).
- Default path adds NO new TauRPC namespace/field — reuse existing `services.list_with_states` / `ServiceListItem`; IF /implement proves the health field absent and a field/procedure is added, it MUST derive `serde`, use a smart enum for the severity field, and get a matching `pulse-app/capabilities/` entry verified by the xtask capability-drift check (per security-plan §Input Validation TauRPC row + §API Security TauRPC capability authorization).

## Patterns to follow
- No-new-namespace consumption: read the already-bridge-validated `services.list_with_states` → `ServiceListPayload`/`ServiceListItem` surface (serde + `AppError::Validation` at the bridge); derive dot health from the existing `state`/`priority_tier` fields rather than re-parsing raw OTLP attributes in the webview (per security-plan §Input Validation TauRPC row; scope §Data source).
- Rely on React default text-escaping as the XSS control for the untrusted service-name label — keep it (no raw-HTML sink) rather than adding sanitization (per security-plan §Security Anti-Patterns § Code Patterns).
- CSP is a framework-level literal in `pulse-app/tauri.conf.json`; if label/icon rendering ever needs a CSP change, regenerate via `tauri build` rather than hand-widening (per security-plan §API Security CSP row).

## Anti-patterns to avoid
- NEVER route the untrusted service-name string into `innerHTML`/`dangerouslySetInnerHTML`, a style/attribute interpolation, or a `Command` arg (per security-plan §Security Anti-Patterns § Code Patterns + § API CSP script ban).
- NEVER add `'unsafe-inline'` to `script-src` or introduce remote content (CDN icon/font) to render labels or the non-color cue (per security-plan §Security Anti-Patterns § API; §API Security CSP row "NO remote content").
- NEVER log raw OTLP attribute values / service-name strings from the new render or resolver path (per security-plan §Security Anti-Patterns § Logging bullet 1).

## Contract bindings
- obs: attribute-value logging discipline cross-cuts — service-name value must not become a high-cardinality log field in any new instrumentation (security-plan §Logging & Monitoring ↔ obs PII-scrubbing coverage).
- design/a11y: health/severity palette (design) and the SC 1.4.1 non-color cue (a11y) own those decisions; security only attaches the untrusted-string render + CSP discipline to whatever surface they pick.
- arch/tests (conditional): a new health-field TauRPC procedure binds to arch (capability decision) + tests (xtask capability-drift check as the CI enforcement).

## Acceptance criteria contributions
- (security) The service-name label renders via escaped text (React text node or canvas `fillText`) with no `innerHTML`/`dangerouslySetInnerHTML`/style-interpolation sink on the label path — grep of touched files confirms none.
- (security) No new or loosened CSP directive in `pulse-app/tauri.conf.json`; no remote-origin icon/font added for the label or non-color cue (first-party or `data:` only).
- (security) No raw OTLP attribute value / service-name string is logged from the renderer, `use-service-constellation.ts`, or any Rust handler touched (diff log-site grep).
- (security, conditional) IF a new TauRPC field/procedure is added: `serde`-derived struct + smart enum for the severity field + matching `pulse-app/capabilities/` entry passing the xtask capability-drift check.

## Relevant amendment history
(none) — no amendment touches the constellation renderer, service-name rendering, webview CSP, or the `services.list_with_states` IPC surface; the nearest webview-adjacent entry (2026-06-29-window-geometry-movable-shell) concerns a persisted geometry-JSON input boundary, a different surface, not label/service-name rendering.
