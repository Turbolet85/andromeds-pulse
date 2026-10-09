# layouts extract — phase-74

## No domain coverage

Chunk #77 ("Specialist plan reconciliation (security + tests) — manual security-plan rewrite + 5 PII vector tests + Drain golden corpus harness") is entirely out of the layouts domain. The chunk has three explicitly declared parts per the epoch context:

1. **Manual security-plan body rewrite** — touches `.andromeda/security-plan.md` sections (§Threat Model, §Data Protection §At rest, §Secret Management, §Anti-Pattern Logging). No layout / UI surface implication.
2. **Test plan rewrite + 5 PII vector tests + Drain golden corpus harness** — touches `.andromeda/test-plan.md` + `.claude/rules/testing.md` + materializes Rust-side tests (AppError sanitization, plugin path basename-only, MCP response body redaction, path env var canonicalization, capability widening static analysis) + LogHub Drain corpus fixture. All backend / test-harness work; zero webview surfaces.
3. **Wrap-session carry-over flag clearing** — META-level state.yaml / handoff bookkeeping.

The epoch context explicitly enumerates the specialist plan touches for this chunk: "security-plan (definitely), test-plan / `.claude/rules/testing.md` (definitely). **obs-plan / design-system / a11y-plan / layout-templates / architecture: no edits this chunk.**"

The `.andromeda/layout-templates.md` plan covers the three UI shells (compact widget glance surface, full dashboard expanded surface, tray icon menu surface) — none of these are modified, extended, or referenced by chunk #77's security-plan rewrite, PII vector tests, or Drain golden corpus harness work. No new modal primitive, no new layout density change, no new InvestigationModal / SettingsModal touch, no compact-widget Halo re-shape, no tray menu re-wiring. The 5 PII vector tests are Rust integration tests under `crates/*/tests/` or `pulse-app/tests/` (per the testing-rules `[lib] test = false` constraint), not webview surface tests.

Orchestrator should treat the layouts extract as empty for this chunk — Phase 2 merge has nothing layouts-side к reconcile against security / tests / arch cross-domain bindings for this META reconciliation chunk.
