# Intent — acknowledge-cadence-events-broadcast

_Captured at Phase 1 of /andromeda-evolve --allow-arch-registry invocation
2026-05-23T12-10-53. Verbatim user-stated intent + dashboard context that
informed Phase 1c clarifying answers without using clarifying-question budget._

## Phase 1b — Sanity check (one-sentence intent)

User selected (via AskUserQuestion): "Register cadence-events broadcast"

Full option label: "Add pulse://stream/cadence-events broadcast topic to
arch §Occupied Resources Tauri IPC events sub-section — the textbook D3
closure suggested by new-session dashboard, mirroring chunks #62/#67/#78
precedent."

## Phase 1c — Deep intent (no clarifying questions needed)

All Q1-Q5 trigger conditions clearly unmet from dashboard context + flag
invocation:

- **Q1 (plan(s) not specified):** plan IS specified — `.andromeda/architecture.md`
  §Occupied Resources Tauri IPC events (broadcast channels) sub-section.
- **Q2 (change scope ambiguous):** change scope is concrete — ADD the
  identifier `pulse://stream/cadence-events` to the existing bulleted
  list. No modification of existing entries.
- **Q3 (why missing):** why is concrete and well-grounded — chunk #80
  Cadence coordinator implementation (session 126 wrap commit 9296fa3)
  introduced the broadcast topic at `crates/triage/src/cadence/broadcast.rs:9`
  via `pub const STREAM_NAME_CADENCE_EVENTS: &str = "pulse://stream/cadence-events"`.
  D3 capability-drift between code reality and arch §Occupied Resources
  declared in `state.yaml.drift_warnings`. Standard chunk-then-amendment
  Type 7 pattern.
- **Q4 (multiple plans):** only 1 plan implicated (arch.md only). No
  cross-plan reconciliation needed.
- **Q5 (refuse-pattern wording):** intent mentions arch.md modification
  (which would normally Refuse 1) but `--allow-arch-registry` flag was
  set explicitly to authorize the narrow Refuse 1 exception per
  refuse-taxonomy.md Refuse 1 Exception.

## Grounding

- **Code evidence:** `crates/triage/src/cadence/broadcast.rs:9` declares
  `STREAM_NAME_CADENCE_EVENTS = "pulse://stream/cadence-events"`. Wrapped
  in `CadenceEventBroadcast` struct (line 36+) used at boot-time spawn
  in `pulse-app/src/main.rs` per chunk #80 implementation.
- **Drift cite:** `state.yaml.drift_warnings[0]` (D3 severity warning;
  first_observed_session_count=126; last_observed_session_count=126).
- **Precedent:** chunks #62 (attention-cues), #63 (restart-events),
  #67 (service-lifecycle), #78 (incidents) all followed identical
  chunk-then-amendment Type 7 (Type 6 single-item) pattern; this is
  the 5th instance of the well-established single-item arch-registry
  broadcast topic amendment shape.

## Slug

`acknowledge-cadence-events-broadcast` (kebab-case; 4 words; mirrors
chunks #62 `acknowledge-attention-cues-broadcast` + #63
`acknowledge-restart-events-broadcast` precedent slug shape).
