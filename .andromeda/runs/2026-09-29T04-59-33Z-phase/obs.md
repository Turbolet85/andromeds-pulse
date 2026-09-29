# obs extract

## Relevance
relevant — §A changes what the P-025 delegated-timing metric measures, how often it fires, and what fields its allowlist leaf carries. §B moves `ANDROMEDA_PULSE_DATA_DIR` in CI, and the CI log-artifact path depends on that variable. The U05 reflow and the two PREREQ discharges are outside obs.

## Constraints
- **The leaf is exact and lists every field.** Per obs-plan §8 PII Scrubbing → "The three DELEGATED TIMING leaves", `metric.constellation.hue_update_ms` is an EXACT leaf that must carry every field its emit site emits, and nothing else.
  - A bare `metric` key exists and keeps only `value` / `unit` / `module`. So a carrying field that is added or renamed without being admitted to the leaf is redacted silently, and the target still resolves.
  - If the contract's sibling-leaf option is taken instead, the new target needs its own exact leaf. The old leaf must then be retired or documented as superseded (§8, same entry).
- **No service identity on the metric.** Per obs-plan §5 Metric Coverage → cardinality discipline and §8's `metric.constellation.discovery_ms` note, `service` is an OTLP resource attribute. It may never be a label on a delegated-timing leaf, and label fields stay bounded enums: `severity_tier` ∈ `HueSeverityTier` (`none | curious | suggested | autonomous`). This is why scope §A item 4 resolves to per-changed-service emission rather than a service-identifier field.
- **Emit raw per-event values; never aggregate in the emission path.** Per obs-plan §5 (Distribution conceptual type) and §11 Obs Anti-Patterns → Metrics, the agent computes p50/p95/p99 from the raw event stream with `jq`.
  - The current slowest-wins maximum per render pass (scope §A "Fire site") is an aggregation inside the emission path. §11 rules it out as the metric's shape, independent of the contract.
- **Emit only on a tier change.** Per obs-plan §11 Obs Anti-Patterns → Logs ("NEVER log in hot path at `info` level") and → Telemetry Strategy (no over-instrumenting hot paths), emission must key off an actual tier transition for a service. Emitting on every render pass or every unchanged dot at `info` is ruled out.
- **The frontend bridge is the decided mechanism.** Per obs-plan §3 Observability Harness Contract → Logging stack → Frontend bridge, the delegated timing observables flow webview → hand-rolled TauRPC `telemetry.frontend.*` → backend `tracing::info!(target: "metric.{name}", ...)`.
  - No browser OTel SDK.
  - Any sibling leaf follows the §2 Telemetry Strategy → naming convention `metric.{module}.{measure}` (here `metric.constellation.*`).
- **Spec text owed if the measured interval changes.** Obs-plan §8 describes the leaf as measuring "span-arrival → the severity-driven hue", and §2 → naming lists `metric.constellation.hue_update_ms` as P-025's observable. If the measured interval becomes paint instant − `tier_effective_at`, or the carrying field or leaf changes, that §8 text (and §2 if the leaf name changes) no longer matches. The chunk owes an obs-plan amendment for it. Whether the amendment goes through the chunk's own spec-amendment window is the plan's call.
- **The CI log artifact lives under the data dir.** Per obs-plan §9 CI Integration → Telemetry artifact handling and §3 → Log file location, CI uploads `<data_dir>/logs/agent-latest.jsonl` from every job, and `<data_dir>` is `ANDROMEDA_PULSE_DATA_DIR` when set.
  - §B relocates that variable from workflow-level `env:` to job-level `env:` or a step's `$RUNNER_TEMP`.
  - Every job and step that writes or uploads the log must still resolve to the same directory.
  - Whether any upload step references the variable is research's question.

## Patterns to follow
- **Allowlist guard in `pulse-app/tests/`.** Obs-plan §8 → DELEGATED TIMING leaves names the guard `pulse-app/tests/unit_observability_allowlist_delegated_timing.rs`, which lives under `pulse-app/tests/` (the `[lib] test = false` rule). It asserts an exact resolve plus field-set equality in both directions. Its fallback-discrimination test is mutation-checked: neutralize the leaf, see RED; restore it, see GREEN.
  - A carrying-field change updates this guard in the same commit.
  - The chunk re-runs the mutation check rather than assuming it still discriminates.
- **Numeric inputs bounded on both sides.** Obs-plan §8 → `metric.constellation.discovery_ms` precedent: clamped client-side, rejected above the bound by a server-side `validate_*` at the resolver. A duration derived from `tier_effective_at` should follow the same shape: non-negative and bounded before `tracing::info!`. Whether the existing resolver `record_constellation_hue_latency` already validates `duration_ms` is research's question.
- **Carrying field named in the event.** Per obs-plan §6 Log Coverage → Required fields, and §3 → Log format JSON schema extensions (`duration_ms` is a listed optional field), the duration is a structured field in `fields`. The contract requires its field name to be stated literally, because the external grader reads it from the JSON line.
- **Live wire proof.** Obs-plan §8 leaf entries use this precedent, e.g. `app.boot.window.navigation` and `triage.cue.tick`: records observed in `agent-latest.jsonl` carrying every field unredacted, alongside the unit guard.
- **NEUTRAL-tolerant check scripts.** Per obs-plan §10 SLO Invariants → WebGPU frame two-state posture and → Load-profile constraints, any new xtask/CI check over `agent-latest.jsonl` must report NEUTRAL when the webview is absent. An absent metric stream is not a failure. P-025 grading itself stays with Conductor's live leg and is never a CI gate (scope §Boundaries).

## Anti-patterns to avoid
- **No service identity on the leaf.** Never add a service name, `scope_id`, or incident id as a label or field on the hue leaf to disambiguate per-service records (obs-plan §11 → Metrics, unbounded label cardinality; §8 Default-deny posture).
- **No precomputed aggregate.** Never emit a max, quantile or other precomputed aggregate in place of raw per-event values (obs-plan §11 → Metrics, "no precomputed quantiles in the emission path").
- **No unadmitted carrying field.** Never ship a carrying field that is emitted but absent from the exact leaf. That produces a PARTLY redacted target: resolver-only probes pass while the field renders `<redacted>` (obs-plan §8 → `triage.cue.tick` and `triage.incident.persist` "partly redacted" lessons; → DELEGATED TIMING leaves bare-`metric` fallback note).

## Contract bindings
- **obs ↔ Conductor's P-025 measurement contract (external grader).** The carrying field's literal name, and the leaf target it rides, form the interface Conductor's live leg reads from `agent-latest.jsonl`. Renaming either is a contract change and must be stated literally (scope §A item 3).
- **obs ↔ tests.** The field set is pinned by `pulse-app/tests/unit_observability_allowlist_delegated_timing.rs` (obs-plan §8), a test-plan harness artifact; a field change must land in both in the same commit. `ServiceListItem` gaining `tier_effective_at` changes the bindings shape, which falls under the staged-bindings gate (tests/security domain, noted here only because the webview-side duration computation depends on it).
- **obs ↔ security.** The rule that `service` is never a label is shared between obs-plan §5 / §8 and security-plan §Logging & Monitoring (default-deny allowlist at the subscriber Layer).
- **obs ↔ CI (§B).** Per obs-plan §9 → Telemetry artifact handling, every CI job uploads `<data_dir>/logs/agent-latest.jsonl`. The `ANDROMEDA_PULSE_DATA_DIR` relocation must keep the writer's path and the uploader's path identical.

## Acceptance criteria contributions
- The delegated-timing allowlist guard passes with field-set equality in both directions, covering the named carrying field (and any sibling leaf, if one is chosen). A mutation that neutralizes the leaf turns its fallback discriminator RED (per obs-plan §8 PII Scrubbing → DELEGATED TIMING leaves).
- Live wire proof: on a booted run with a tier transition, `agent-latest.jsonl` holds hue-leaf records whose carrying duration field and `severity_tier` are both present and unredacted, with 0 `<redacted>` on that target. No record carries a service name, scope id or incident id (per obs-plan §8 Default-deny posture and §5 Metric Coverage → cardinality discipline).
- Emission cardinality: one record per service whose tier changed, per transition. There is no per-render-pass slowest-wins aggregate, and no record for dots whose tier did not change (per obs-plan §11 Obs Anti-Patterns → Metrics and → Logs hot-path ban).
- Obs-plan §8 (and §2 naming, if the leaf changes) describes the corrected interval (paint instant − `tier_effective_at`) and the named carrying field, rather than "span-arrival → hue" (per obs-plan §8 → DELEGATED TIMING leaves; §2 Telemetry Strategy → naming conventions).
