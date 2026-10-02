# design extract

## No domain coverage

Chunk is Rust runtime/security only — path-input confinement and argv-prompt bounding in `pulse-app/src/llamacli_inference.rs` plus five security spec artifacts; it renders no surface, so no token, typography, motion, iconography, or component-pattern mandate from `design-system.md` applies (the rejection's observability lands on the `interpretation.model.load` tracing target, which is obs's domain, and the scope's Boundaries explicitly exclude the L4 runtime's behaviour and any UI file).
