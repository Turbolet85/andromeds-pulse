# User Intent — acknowledge-restart-events-broadcast

**Generated:** 2026-05-17T14:15:00Z
**Skill:** `/andromeda-evolve --allow-arch-registry`
**Slug:** `acknowledge-restart-events-broadcast`

## User-stated intent (inferred from session 81 handoff + D3 drift_warning)

Acknowledge the `pulse://stream/restart-events` Tauri broadcast topic in arch.md §Occupied Resources Tauri IPC events (broadcast channels) sub-section. The topic was introduced by chunk #63 (Restart event detector + dual-condition bypass — commit `61ca564`, session 81) and is declared at `crates/triage/src/pattern/broadcast.rs:8` as:

```rust
pub const STREAM_NAME_RESTART_EVENTS: &str = "pulse://stream/restart-events";
```

arch §Occupied Resources Tauri IPC events sub-section has not yet acknowledged this topic — D3 capability-drift class first observed at session_count 81 per `state.yaml.drift_warnings`. This amendment closes the drift gap via the standard Type 6 (`--allow-arch-registry`) pattern.

## Source signals

- **state.yaml.drift_warnings (session 81):**
  ```yaml
  - drift_id: D3
    description: "arch.md §Occupied Resources Tauri IPC events (broadcast channels) sub-section does not yet acknowledge `pulse://stream/restart-events` introduced this session at `crates/triage/src/pattern/broadcast.rs:8` (chunk #63 impl)"
    severity: warning
    remediation_hint: "Run /andromeda-evolve --allow-arch-registry to land Type 6 amendment (mirrors chunk #62 pulse://stream/attention-cues 2026-05-17 precedent at arch §Architecture Registry Updates)"
    first_observed_session_count: 81
    last_observed_session_count: 81
  ```
- **session-handoff.md Next Recommended Action:** `/andromeda-evolve --allow-arch-registry` to land the Type 6 amendment.
- **Precedent:** 2026-05-17 chunk #62 `pulse://stream/attention-cues` Type 6 entry at arch.md §Architecture Registry Updates (same shape, same registry section, same authorization flow).

## Phase 1b sanity check result

- Fast pattern match: Refuse 1 (arch.md body intent) — normally fast-path refuse, but `--allow-arch-registry` flag opens narrow Refuse 1 Exception per refuse-taxonomy.md §Refuse 1 Exception → §Phase 1b sanity-check interaction.
- Proceeds to Phase 1c (deep dialogue) — full Check 7 verification runs at Phase 3.

## Phase 1c clarifying questions used

0 of 4 (intent fully resolved from session context per autonomous-execution directive; no follow-up dialogue needed).
