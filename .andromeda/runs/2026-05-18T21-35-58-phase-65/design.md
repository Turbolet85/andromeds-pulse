# design extract — phase-65

## No domain coverage

Chunk 69 (Phase A spike scope only) is out-of-domain for design-system. Reason: Phase A is a pure backend Rust research spike — minimal Drain prototype tested against LogHub corpus subset with measurements captured in `.andromeda/decisions/pre-d2-drain-spike.md`. The chunk explicitly EXCLUDES the Settings → Diagnostics "Template Distribution" panel UI and any user-facing visual surface (deferred to Phase B). Spike code lives in throwaway branch or gitignored `crates/triage-experimental/`. No visual rendering, no design tokens, no typography / motion / iconography / spacing / depth / radius / component patterns apply — design-system.md authority does not bind to research-spike findings docs.
