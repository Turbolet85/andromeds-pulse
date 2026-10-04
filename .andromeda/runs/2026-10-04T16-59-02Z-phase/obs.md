# obs extract

## Relevance
partial. The chunk widens a filesystem-presence heuristic and its scope declares no new log target. obs-plan names no hardware-profile record at all (no `detect_gpu_present`, `HardwareProfile`, `profile_label` or `ANDROMEDA_PULSE_HARDWARE_PROFILE` mention anywhere in the plan). So obs adds only guard-rails: path hygiene, the default-deny allowlist (only if an emit is added), and the downstream `interpretation.model.load` `tier` field.

## Constraints
- Self-observation is `tracing`-only, JSON-per-line to the file sink. Any diagnostic the probe gains goes through `tracing`, never stderr text or an OTel SDK (per obs-plan §1 Obs Scope Summary "Architectural choice"; §11 Logs; §11 Universal).
- A probed library path is a filesystem path. obs-plan requires basename-only logging for every product-consumed path and bans full paths in logs and spans (per obs-plan §8 PII Scrubbing data-classification table; §11 Logs "full paths (Vector 3)"; §1 logging-sensitive row). The scope expects the probe to log no path at all. Whether `hardware.rs` logs anything today is research's question.
- The subscriber Layer is default-deny. Any new target, or any new field on an existing one, needs its OWN exact allowlist leaf naming every emitted field. No bare `interpretation` key may exist, or a sibling silently resolves to the wrong field set and redacts (per obs-plan §8 PII Scrubbing "Default-deny posture"; §8 `interpretation.incident.created` entry). The scope declares no new log target. If P4 adds one anyway, this rule binds in full.
- `interpretation.model.load` carries `tier` in its completed leaf `{model_identity, tier, load_status, inference_mode}` (per obs-plan §8 "Muted-diagnostic backlog", the `interpretation.model.load` bullet). If the detected profile feeds the tier, this host's emitted `tier` value changes while the field set must not. Whether `tier` derives from `HardwareProfileDetector` is research's question.
- Diagnostics stay off hot paths. A once-per-boot or once-per-transition record is the sanctioned shape, never a per-call or per-generation emit (per obs-plan §11 Logs "NEVER log in hot path at info level"; §6 Log Coverage `warn` row, the once-per-boot precedents). Whether `detect_gpu_present` runs once at boot or per generation is research's question.
- Label values must be bounded and enumerated in advance. A profile label is already a closed vocabulary, but a candidate path or a soname string as a field value would not be (per obs-plan §11 Metrics "unbounded label cardinality").

## Patterns to follow
- A hardware-state field must be truthful or removed. The hardcoded `gpu_available: false` on `app.boot.gpu.check` was deleted rather than kept, and a truthful record replaced it (per obs-plan §1 multi-platform-exporter-compat row). The analogue here: the GPU-present reading should reflect the host's real driver layout.
- The "indistinguishable boot" rationale: a once-per-boot record with a closed label exists whenever two boots would otherwise read alike (`app.boot.render.posture`, `interpretation.model.allow_root`, per obs-plan §6 Log Coverage `warn` row). The plan does not mandate such a record for the hardware profile, and the scope bars a new target. Mention it only as a P4 consideration if research finds the detected profile never reaches the log.
- Allowlist guards live under `pulse-app/tests/` (the `[lib] test = false` rule). They assert exact-resolve, field-set equality in both directions, and the `for_target("interpretation").is_none()` discriminator (per obs-plan §8 `interpretation.model.allow_root` / `.load.error` paragraph; §8 `interpretation.incident.created` entry). This applies only if an emit is added or changed.

## Anti-patterns to avoid
- Logging a full probed path (for example `/usr/lib/libcuda.so`) or a resolved symlink target as a field or message text (per obs-plan §11 Logs Vector 3; §11 Spans / Traces "NEVER log full canonicalized ... paths").
- Adding an `interpretation.*` emit without its own exact leaf, or registering a bare `interpretation` prefix key to "cover" it (per obs-plan §8 `interpretation.incident.created` entry; §8 `interpretation.generation.damper` entry).
- Emitting per-probe or per-generation records from the detector (per obs-plan §11 Logs; §11 Telemetry Strategy "NEVER over-instrument hot paths").

## Contract bindings
- obs ↔ security: basename-only path logging (per obs-plan §8 data-classification table "Environment variables (config)" + the path rows). The env-var NAME `ANDROMEDA_PULSE_HARDWARE_PROFILE` is OK to log as a name. The security rule "NEVER log a full product-consumed filesystem path" is the paired mandate.
- obs ↔ tests: if any emit is added or changed, the exact-leaf guard file under `pulse-app/tests/` is the test-side binding (per obs-plan §8 allowlist guard entries). Otherwise there is none for this chunk.

## Acceptance criteria contributions
- (obs) Any log family produced while proving the chunk (a probe run or test capture) holds 0 occurrences of a full probed library path such as `/usr/lib/libcuda.so`, `/usr/lib64/...` or `/usr/local/cuda/lib64/...`. If a path-derived value appears at all, it is a basename (per obs-plan §8 PII Scrubbing data-classification table; §11 Logs).
- (obs) The `interpretation.model.load` allowlist leaf's field set stays exactly `{model_identity, tier, load_status, inference_mode}`, and `for_target("interpretation").is_none()` still holds (per obs-plan §8 "Muted-diagnostic backlog" `interpretation.model.load` bullet; §8 `interpretation.incident.created` entry).
- (obs) If the chunk adds or changes any `tracing` emit, it has its own exact leaf whose field set equals the emit site's in both directions. The guard lives under `pulse-app/tests/`, and the emit fires off any hot path, at most once per boot or transition (per obs-plan §8 `interpretation.model.allow_root` / `.load.error` paragraph; §11 Logs).
