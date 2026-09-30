# design extract

## No domain coverage
The chunk narrows the `credit_card` regex arm in `crates/security/src/scrubber.rs` and adds scrubber test pins. It adds or changes no rendered surface, token, typography, motion, iconography or component pattern. design-system.md never mentions redaction or scrub-placeholder rendering, so a changed `[redacted: …]` string reaching a webview would be a text-content change, not a design one.
