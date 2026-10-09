# obs extract

## Relevance
partial — the chunk adds no product instrumentation (scope item 4: no product code changes); obs governs only what the measurement run and its committed evidence may carry, and which existing self-observation records can attest the real-mode routing.

## Constraints
- Model text is never a log or evidence field: per obs-plan §8 PII Scrubbing (default-deny posture; the `interpretation.incident.skipped` leaf names "prompt or model text" among the never-emitted content), the run's committed `evidence/` may carry only bounded labels, counts and the verdict line. Whether `l4_decision_probe` itself writes only bounded labels to stdout and `runs.json` is research's question (scope item 4 and Boundaries assert it; obs cannot attest it).
- Full product-consumed paths are never emitted: per obs-plan §8 Data classification (the plugin-paths row, basename-only) and §2 logging-sensitive row (the L4 path guard emits `env_var` + `path_basename` on `interpretation.model.load.error` and `root_basename` on `interpretation.model.allow_root`), the leg env `inputs#I2` model/binary paths may appear in evidence and logs only as a basename or variable NAME. Env var NAMES are OK to log per obs-plan §8 Data classification (environment-variables row); none of the three L4 vars carries a redacted suffix.
- Per obs-plan §6 Log Coverage (`warn` row), `interpretation.model.allow_root` fires EXACTLY ONCE per boot and WARNs on `confinement = "unconfined"`. Leaving `ANDROMEDA_PULSE_L4_ALLOW_ROOT` unset (the Boundaries keep the guard unchanged) therefore leaves an expected WARN, not a fault, if the probe boots the product subscriber. Whether the example binary installs that subscriber is research's question.
- Per obs-plan §8 PII Scrubbing (the completed `interpretation.model.load` leaf `{model_identity, tier, load_status, inference_mode}`), `inference_mode` is the designated field that attests which inference mode loaded. If the run produces self-observation output, this is the obs-side witness for the scope's `[inferred]` CUDA-routing claim. Whether the probe path emits `interpretation.model.load` at all is research's question.
- Per obs-plan §11 Anti-Patterns → SLO ("NEVER add retry-once policies … masks real failures"), the measurement itself must not be retried. This aligns with scope item 5: one run each, and a FAIL is recorded as measured.

## Patterns to follow
- Bounded-label records: every `interpretation.*` leaf in obs-plan §8 PII Scrubbing carries only closed static labels plus aggregate numerics, never identity, title or payload. The evidence reading of `names_trigger` distributions (scope item 3) follows the same shape, as counts per label per arm.
- Basename-only path spelling via `path_basename` / `root_basename`: the obs-plan §2 logging-sensitive row and §8 Data classification precedent, applied to any path the evidence names.
- Agent-readable, machine-parseable artifacts per obs-plan §11 Anti-Patterns → CI and Universal: the structured `runs.json` is the primary record, and stdout verdict lines are the secondary record.

## Anti-patterns to avoid
- NEVER log or commit raw model-generated content or prompt text (obs-plan §11 Anti-Patterns → PII Scrubbing default-deny; §8 `interpretation.incident.skipped` exclusion list).
- NEVER log full paths (obs-plan §11 Anti-Patterns → Logs, Vector 3). This covers the GGUF, `llama-cli` and `libggml-cuda.so` locations in the leg env.
- NEVER rely on colored or unstructured terminal output as the parsed record (obs-plan §11 Anti-Patterns → CI and Logs). The verdict must be read from the probe's stated line and `runs.json`, not from terminal prose.

## Contract bindings
- obs ↔ security: the basename-only and no-model-text evidence rules bind to security-plan §Security Anti-Patterns → Logging (L4 path vars, NEVER-log list). Committed-evidence hygiene (no full host path) is the shared enforcement point.
- obs ↔ tests: the bounded-label `runs.json` and the `names-trigger verdict` line are the acceptance artifacts the tests domain grades (the pre-registered `--min-rank1 36` rule). obs contributes only the constraint that they hold bounded labels.

## Acceptance criteria contributions
- (obs) Committed `evidence/` (probe stdout plus `runs.json` for both arms) contains zero model-generated text and zero prompt text. Only bounded labels, counts and the verdict line are allowed (per obs-plan §8 PII Scrubbing).
- (obs) Committed `evidence/` contains no full host path for the model, the `llama-cli` binary or the CUDA library: basenames or `~`-relative spellings only (per obs-plan §11 Anti-Patterns → Logs, Vector 3).
- (obs) If the real-mode run emits self-observation records, the `interpretation.model.load` record's `inference_mode` field is read and recorded as the routing witness. If it does not emit one, that is recorded as "routing not attested by self-observation" rather than inferred (per obs-plan §8 PII Scrubbing, the `interpretation.model.load` leaf).
