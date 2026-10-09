# obs extract

## Relevance
partial — the chunk changes L4 input text (prompt / digest rendering) and runs a real-model measurement; obs applies to the self-observation records on the L4 path, the "model text never logged" discipline, and any tracing target the remedy might add. It adds no span, metric or heartbeat by its stated scope.

## Constraints
- Obs tier is Standard (per obs-plan §1 Obs Scope Summary). Self-observation stays `tracing`-only, written as JSON-per-line to the file sink (per obs-plan §3 Logging stack / §11 Universal). The real-model series must not add an OTel exporter or a stdout/stderr text channel to the product.
- Any NEW or changed `interpretation.*` tracing target needs its own EXACT allowlist leaf. That leaf must name every field the emit site emits, and no bare `interpretation` key may exist: `for_target("interpretation").is_none()` must hold (per obs-plan §8 PII Scrubbing, the `interpretation.incident.created` / `interpretation.incident.skipped` / `interpretation.generation.damper` entries). Whether the remedy adds or changes any emit site is research's question. The scope suggests it does not.
- The L4 records must never carry digest, prompt or model text, or a title, symptom, hypothesis, `scope_id` or incident identity. Only bounded labels and aggregate counts are allowed (per obs-plan §8, the `interpretation.incident.skipped` entry: "Never `scope_id`, title, symptom, `payload_summary`, incident identity, digest, prompt or model text"). A changed TRIGGER line or framing instruction is prompt text, so the same ban covers it.
- Paths in logs are basename only. That covers the model GGUF path, the llama-cli binary path and the allow-root (per obs-plan §8, the `interpretation.model.allow_root` / `interpretation.model.load.error` entries). The real-model leg's log family and the committed evidence must carry no full host path.
- If `interpretation.model.load`'s `model_identity` or `inference_mode` is used to attest which runtime and model served the series, read those fields from the log rather than inferring them (per obs-plan §8, Muted-diagnostic backlog, the `interpretation.model.load` leaf `{model_identity, tier, load_status, inference_mode}`).
- The zero-unlogged-panics invariant applies to every run of the series. A run that panics is a failure, never retried away (per obs-plan §10 Always-required SLO invariant; §11 SLO, the retry-once ban).
- Any per-generation diagnostic folds into an existing per-cycle or transition record. Never log per digest or per suppression at INFO (per obs-plan §11 Logs, the hot-path ban; §8, the `interpretation.generation.damper` entry: emitted per state transition, never per decision).

## Patterns to follow
- Exact-leaf plus field-set-equality guard. Put it under `pulse-app/tests/` (the `[lib] test = false` rule), with a no-bare-prefix discriminator, and mutation-check it (per obs-plan §8, the `unit_observability_allowlist_*.rs` guards, e.g. `..._incident_skip.rs`, `..._generation_damper.rs`). This applies only if the remedy touches an emit site.
- Wire-read the live leg: count `<redacted>` fields on the `interpretation.*` records the run produces and expect 0 (per obs-plan §8, the `interpretation.incident.skipped` entry: "wire-read at the chunk's live storm: 5 records, 0 `<redacted>` fields").
- Bounded closed-label vocabularies for any outcome field. The skip record's `skip_reason` / `decision` / `severity` / `digest_kind` is the model to follow (per obs-plan §8).
- Durable evidence states its own limits. Per-shape tallies at n = 10 are a measurement, not an exclusion. This mirrors the session-learnings discipline; on the obs side, record values exactly as the log or probe emitted them (per obs-plan §10 Error budget: measurement parsed from the log, never asserted).

## Anti-patterns to avoid
- NEVER log model output, prompt or digest text, or raw OTLP attribute values carried into the digest (per obs-plan §11 Logs; §8 Data classification, raw OTLP telemetry is High / scrub-required).
- NEVER add a high-cardinality field (a service name as a label, a hypothesis string, a fingerprint) to an `interpretation.*` or `metric.*` record (per obs-plan §11 Metrics, the unbounded-label ban).
- NEVER let a bare `interpretation` (or bare `metric`) fallback silently serve a new target. A resolve-only probe passes while every field redacts (per obs-plan §8, the `interpretation.incident.created` entry's mechanism note; the bare-`metric` note under the delegated timing leaves).

## Contract bindings
- obs ↔ security: never log the model, prompt or digest text, and log paths as basenames (obs-plan §8 ↔ security.md §Logging & redaction). The scope's "model output is never logged or committed as text" boundary is the evidence-side twin of this ban.
- obs ↔ tests: the `interpretation.*` allowlist guards in `pulse-app/tests/` (`unit_observability_allowlist_{incident_skip,generation_damper,l4_path_guard,sweep}.rs`, `observability_pins.rs`) and the producer pins in `unit_incident_producer.rs` must stay green under the prompt or digest change (obs-plan §8 ↔ test-plan §3). Whether any of them pins a prompt-version literal or digest text is research's question (the sweep hazard the scope names).
- obs ↔ tests harness: the real-model leg's log family is the JSON-per-line surface (obs-plan §3 Log format JSON schema / §6) that any `<redacted>` or path count reads.

## Acceptance criteria contributions
- (obs) Two cases, depending on the remedy:
  - No new or changed tracing emit site: every existing `interpretation.*` allowlist guard and the `for_target("interpretation").is_none()` discriminator stay green.
  - A new or changed site: it ships an exact leaf whose field set equals the emit site's in both directions, with a mutation-checked guard under `pulse-app/tests/`.
  (per obs-plan §8 PII Scrubbing, the `interpretation.*` leaf entries)
- (obs) Across the real-model series' log family and the committed `evidence/`, the following count 0: a full host path (model / binary / allow-root / data dir), a `<redacted>` field on an `interpretation.*` record, and any title, symptom, hypothesis, prompt or digest text (per obs-plan §8, the `interpretation.incident.skipped` and `interpretation.model.allow_root` entries; §11 Logs).
- (obs) 0 `app.panic.fatal` records across every series run's log family, and no run is repeated to clear a failure (per obs-plan §10 Always-required SLO invariant; §11 SLO).
