# obs extract

## Relevance
partial — no new SLO, metric family or critical path; obs binds through (a) the agent-readable record the dismissal measurement grades, (b) the exact-leaf allowlist discipline for any `interpretation.*` field the fix adds or changes, (c) the PII ban on digest/model-output content in the log, and (d) the machine-readable-CI rule for the PREREQ lint output.

## Constraints
- The live measurement's evidence surface is the JSON-per-line self-observation log at `<data_dir>/logs/agent-latest.jsonl`; a per-decision outcome that only exists in chat, a window or an ad-hoc stdout dump is not an obs signal (per obs-plan §3 Log file location; §2 Agent-readable invariants). Whether the code ALREADY emits a record when L4 returns `Decision::Dismiss` (vs only on creation via `interpretation.incident.created`) is research's question.
- Every `interpretation.*` target must resolve to its OWN exact allowlist leaf whose field set EQUALS the emit site's field set in both directions; no bare `interpretation` key may exist (the `for_target("interpretation").is_none()` discriminator must stay green). Adding a field to an existing emit (e.g. a decision/dismiss label on `interpretation.incident.created`, or sampling params on `interpretation.model.load`) is a leaf COMPLETION, else the field redacts silently (per obs-plan §8 `interpretation.incident.created`, `interpretation.generation.damper`, `interpretation.model.load` entries; §8 muted-diagnostic backlog).
- New fields are aggregate counts or bounded static labels only — never incident identity, `scope_id`, workspace, title/detail, payload, digest text, prompt text or model-authored justification (per obs-plan §8 Default-deny posture and the `interpretation.incident.created` entry; §5 Metric label cardinality discipline).
- Emission rate: one record per generation / state transition at most — never per cue, per suppression or per digest-field; counters ride an existing heartbeat as tick-aggregated FIELDS (per obs-plan §11 Logs "NEVER log in hot path at info"; §5 Counter tick-aggregated-as-FIELDS rows; §8 damper "ONCE per damper state transition").
- The leaf-equality guard must live under `pulse-app/tests/` (the `[lib] test = false` rule — a src-level `mod tests` never runs) (per obs-plan §8 `interpretation.generation.damper` / l4_path_guard guard entries).
- CI-produced findings must be machine-readable, not dependent on terminal rendering; the lint-stage consumer convention is GitHub Actions annotations (`::error::`) (per obs-plan §9 Pipeline integration `fmt`/`clippy` row; §11 CI "NEVER rely on colored terminal output for CI parsing").

## Patterns to follow
- Damper discrimination: `interpretation.generation.damper` transitions plus the `generations_suppressed_total` / `generations_run_total` fields on `metric.pipeline.l4.backoff_remaining_seconds` let a run separate "storm digest never generated (suppressed)" from "generated and dismissed" — the live half should grade against these rather than infer from incident absence (per obs-plan §8 damper + backoff gauge entries; §5 tick-aggregated counters).
- `interpretation.incident.created` {`created`, `deduped`, `severity`, `priority_tier`} is the existing creation-side record; `incidents.list_active.request` `item_count` is the live-verified incident-formation count on a storm run (per obs-plan §8 Incident-path diagnostic leaves).
- Leaf guard shape: exact-resolve + field-set equality both directions + bare-prefix fallback discriminator, mutation-checked (leaf narrowed → set pin RED while resolve pin stays green) (per obs-plan §8 `triage.cue.tick` and `interpretation.generation.damper` entries).
- `interpretation.model.load` {`model_identity`, `tier`, `load_status`, `inference_mode`} is the leaf a sampling-argument change (`--seed` / `--temp`) would extend if those values are surfaced (per obs-plan §8 muted-backlog `interpretation.model.load` row).

## Anti-patterns to avoid
- NEVER log raw OTLP-derived content — the storm digest body, the composed prompt, or the model's title/justification text — to make a dismissal "debuggable"; record bounded decision labels and counts only (per obs-plan §11 Logs and §11 PII Scrubbing default-deny; §8 Default-deny posture).
- NEVER add an `interpretation.*` emit or field without its exact leaf, or re-introduce a bare `interpretation` key — the target still resolves and the new field redacts to `<redacted>` with every resolver probe green (per obs-plan §8 `interpretation.incident.created` bare-key mechanism lesson).
- NEVER emit per-cue or per-suppression records to count decisions (per obs-plan §11 Logs hot-path ban; §5 tick-aggregated counter convention).

## Contract bindings
- obs ↔ security: digest content and model output are scrubbed-OTLP-derived user content; the log-boundary ban here pairs with security-plan §Logging & Monitoring and the redaction-at-write control (per obs-plan §8 PII Scrubbing data classification row "Raw OTLP telemetry payloads").
- obs ↔ tests: any new/completed `interpretation.*` leaf carries a `pulse-app/tests/unit_observability_allowlist_*.rs` equality guard; the live re-measurement leg reads `agent-latest.jsonl` as its grader input (per obs-plan §8 guard entries; §3 Log format JSON schema — binding contract from tests §5).
- obs ↔ CI: the PREREQ Cyrillic step's output convention (machine-readable, annotation-shaped) per obs-plan §9 Pipeline integration.

## Acceptance criteria contributions
- (obs) Every `interpretation.*` target the chunk adds or changes resolves to an exact leaf whose field set equals its emit site's in both directions, guarded under `pulse-app/tests/`, with `for_target("interpretation").is_none()` still asserted (per obs-plan §8 `interpretation.incident.created` / `interpretation.generation.damper`).
- (obs) The live leg's log carries 0 `<redacted>` values on every target the measurement grades, and its per-generation outcome (generated vs damper-suppressed vs created vs dismissed) is countable from `agent-latest.jsonl` alone (per obs-plan §3 Log file location; §8 muted-diagnostic backlog CLOSED).
- (obs) A canary placed in storm-digest input appears 0× in the live leg's `agent-latest.jsonl` — no digest, prompt or model-justification text reaches the log (per obs-plan §8 Default-deny posture; §11 Logs).
- (obs) A Cyrillic-literal hit reaches CI output as a machine-readable finding naming file:line on every runner OS (per obs-plan §9 Pipeline integration; §11 CI).
