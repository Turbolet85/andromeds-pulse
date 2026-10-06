### Screen reader test pattern

**Per-surface test spec:** NVDA 2025.3 Windows / VoiceOver macOS 15.3+ / Orca 48.x Linux

**Manual pass spec format:** structured per-screen text + expected SR output (announces form labels / heading hierarchy / landmark roles / state changes via aria-live). Emitted as `a11y-sr-results.jsonl` with `{path, sr_announcement_expected, sr_announcement_actual, status}` records aligned to obs Section 6 schema.

**Supplemental to automated:** SR pass spec NEVER sole verification; axe-core / pa11y / Lighthouse cover SC 4.1.2 Name/Role/Value programmatically; SR pass verifies runtime announcement quality per trigger.
