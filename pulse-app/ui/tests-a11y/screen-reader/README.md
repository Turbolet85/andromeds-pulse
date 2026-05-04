# Screen Reader Manual-Pass Test Spec Scaffold

Per-platform fixture specifications + structured JSON output schema for
manual screen-reader passes. Scaffold landed at chunk #14; full
activation deferred to chunk #25 (webview shell) + chunk #46 (CI gate
+ violation-tuple regression diff).

## Scope

Manual-pass fixtures for three SR/platform pairings:

- `a11y-sr-nvda.md` — NVDA 2025.3 + Chrome stable on Windows
- `a11y-sr-voiceover.md` — VoiceOver + Safari on macOS 15.3+
- `a11y-sr-orca.md` — Orca 48.x + Firefox on Linux

Each fixture covers all 7 Critical Paths P1–P7 (per `a11y-plan.md` §3
Critical paths table) with the canonical 9-step manual pass spec
sequence (boot → activate SR → navigate via SR commands → assert
label/heading/role announcement → verify error message ARIA → verify
toast aria-live → verify modal title + close button → verify interactive
role+state → verify WebGPU canvas region label).

## Supplemental, not sole gate

Manual SR passes are SUPPLEMENTAL evidence. Automated `axe-core` /
`Lighthouse` / `pa11y` (chunk #13 install + chunk #46 CI gate) remain
the PRIMARY a11y CI gates. Per `a11y-plan.md` §11 Universal
anti-patterns: NEVER use manual screen reader / keyboard / contrast as
ONLY method of verification. SR fixtures verify runtime announcement
quality per trigger that automated tooling cannot reach (e.g., the lived
order of spoken phrases through a screen reader), and feed the per-surface
violation-tuple regression diff that chunk #46 enforces.

## Surface inventory (9 targets, layouts vocabulary verbatim)

Surface IDs match `layout-templates.md` §Surface vocabulary exactly — no
abbreviation, no pluralization changes.

**desktop-webview** (6 sub-surfaces, React + WebGPU canvas):
- `desktop-webview/compact-widget`
- `desktop-webview/full-dashboard/traces`
- `desktop-webview/full-dashboard/metrics`
- `desktop-webview/full-dashboard/logs`
- `desktop-webview/full-dashboard/snapshots`
- `desktop-webview/full-dashboard/settings`

**desktop-native** (3 sub-surfaces, OS-native — no React webview):
- `desktop-native/tray-icon` (NSStatusItem / NotifyIcon / AppIndicator)
- `desktop-native/tray-menu` (OS context menu)
- `desktop-native/notifications` (OS-native toasts)

Modals are exercised within their parent surface fixture sections per
`layout-templates.md` §Component sections:
- Investigation modal → exercised within `full-dashboard/traces` (P2)
- Settings modal → exercised within `full-dashboard/settings` (P3 + P7)

Desktop-native surfaces exercise OS accessibility APIs, NOT webview
ARIA; fixture metadata records `surface_kind: "os-native"` versus
`surface_kind: "webview"` per `schema.json`.

## Output

Each manual pass emits one JSON record per assertion to a JSONL file:

```
pulse-app/ui/test-results/a11y-sr/<screen_reader>-<surface_slug>-<ISO-date>.jsonl
```

Output dir is gitignored (chunk #14 extends `.gitignore` "Test
artifacts" section with `pulse-app/ui/test-results/a11y-sr/`).

`<surface_slug>` = surface ID with `/` replaced by `_`
(e.g., `desktop-webview_full-dashboard_traces`).
`<ISO-date>` = local date of the pass run (e.g., `2026-05-15`).

JSON record shape per `schema.json` (draft 2020-12) — aligned to
`obs-plan.md` §3 Log format JSON schema + §6 Required fields binding
contract:

```json
{
  "timestamp": "2026-05-15T10:23:14.572Z",
  "level": "info",
  "target": "a11y.screen_reader.pass.compact_widget",
  "message": "P5 widget Esc -> tray restore: button label announcement verified",
  "fields": {
    "surface": "desktop-webview/compact-widget",
    "surface_kind": "webview",
    "screen_reader": "NVDA",
    "navigation_mode": "browse_mode",
    "wcag_sc": "SC 4.1.2",
    "critical_path": "P5",
    "status": "pass",
    "tool": "manual",
    "sr_announcement_expected": "Minimize to tray, button",
    "sr_announcement_actual": "Minimize to tray, button"
  }
}
```

`target` follows `{module}.{operation}` convention per
`.claude/rules/observability.md` §Span discipline — surface suffix is
snake_case enumerated; high-cardinality fields (per-tester-name,
per-run-UUID) MUST live in nested `fields.metadata.*`, never at the
top-level `target`.

## Schema

`schema.json` documents the JSON Schema (draft 2020-12) for output
records. **Documentation-only** — no runtime validator dependency
installed at chunk #14. Downstream chunk #46 may add a runtime validator
(`ajv` / equivalent) if the regression-diff CI gate requires programmatic
validation; chunk #14 deliberately keeps the surface lean.

## PII / fixture hygiene (security + obs binding)

Per `security-plan.md` §Logging & Monitoring + `obs-plan.md` §8 PII
Scrubbing (Vectors 1–6):

- Fixtures use SYNTHETIC example data only — no real OTLP attribute
  values harvested from any host application.
- `sr_announcement_expected` / `sr_announcement_actual` MUST be
  single-line strings — multi-line stack traces or narratives forbidden
  (use `\n` escapes if needed per `obs-plan.md` §11 Logs).
- Use `service.name = "demo-service"` and similar synthetic
  identifiers in example announcements.
- NEVER embed real plugin paths, real DuckDB query text, or real
  per-user home directory paths from any of the three platforms in
  fixture text or pass narrative.
- Output files (under gitignored `pulse-app/ui/test-results/a11y-sr/`)
  may capture observed strings during a real pass; those files MUST
  stay out of git.

## Activation status

| Chunk | Status | Scope |
|-------|--------|-------|
| #13   | shipped | a11y dev stack install (axe-core/Lighthouse/pa11y/react-aria-components/focus-trap-react/tabbable/eslint-plugin-jsx-a11y) |
| #14   | this scaffold | SR manual-pass fixture spec + JSON schema + .gitignore + harness-message updates |
| #25   | deferred | Webview shell — surfaces start to exist for manual pass runs to target |
| #28   | deferred | Compact widget shell |
| #33   | deferred | Full dashboard shell + per-tab views |
| #36   | deferred | Tray icon |
| #37   | deferred | Investigation modal |
| #38   | deferred | Settings modal |
| #46   | deferred | A11y CI gate + per-surface violation-tuple regression — consumes JSONL output from manual passes |

Until chunks #25+ land, the fixtures are AUTHORED but not RUNNABLE
end-to-end (the surfaces don't yet exist). Manual passes against
not-yet-shipped surfaces should record `status: "blocked"` in JSON
output. Spec discipline is established now; per-surface pass discipline
activates as surfaces land.

## Source authorities

- `.andromeda/a11y-plan.md` §3 (Screen Reader Test Pattern + Critical
  paths table) + §3.5 (Bootstrap phase `screen-reader-test-spec-setup`)
  + §4 (Per-component pattern catalog) + §7 (Per-surface SR test
  pattern table) + §11 (Anti-patterns)
- `.andromeda/obs-plan.md` §3 (Log format JSON schema) + §6 (Required
  fields) + §8 (PII Scrubbing default-deny posture) + §11 (Logs
  anti-patterns: JSON-per-line, single-line strings)
- `.andromeda/layout-templates.md` §Surface vocabulary (desktop-webview
  / desktop-native + Primary screens)
- `.andromeda/design-system.md` §Iconography (custom Icon registry
  glyph names: `aperture`, `telescope`, `constellation-grid`, `star`,
  `circular-pulse`) + §Motion (Halo State Pulse exemption +
  reduced-motion override)
- `.andromeda/security-plan.md` §Logging & Monitoring (Vectors 1–6
  redaction) + §Secret Management (.gitignore discipline)
- `.claude/rules/a11y.md` (canonical structured violation JSON schema
  example) + `.claude/rules/observability.md` (target naming
  convention) + `.claude/rules/frontend.md` (WebGPU canvas a11y
  wrapper pattern)
