# security extract

## Relevance
partial. The chunk touches four security surfaces: a self-observation log leaf (the redaction and allowlist discipline), a TauRPC argument struct plus a bindings shape change (IPC validation and the staged-artifacts gate), three GitHub workflow files (CI supply-chain posture), and the `cargo audit` / `cargo deny` pin discharge. It adds no new trust boundary, auth, secret or network surface.

## Constraints
- The hue-latency leaf and any sibling leaf must never carry an OTLP attribute value. The service name is OTLP user content, so a per-changed-service emission must not put a service identifier, name or hash on the record. Per security-plan §Logging & Monitoring ("What NEVER to log") and §Security Anti-Patterns → Logging (NEVER bullet 1). Whether the scope's "per-changed-service emission, no identifier" shape meets this is the design's job. Research confirms the emitted field set at the resolver in `crates/ui-bridge/src/telemetry.rs`.
- Any field added to or changed on the delegated-timing argument struct (`ConstellationHueLatencyInput`) must be validated at deserialization, with a bounded validator (the `validate_duration_ms` shape) or a closed enum (the `HueSeverityTier` shape). Each rejection arm must be unit-tested by name. Per security-plan §Input Validation, the TauRPC bridge row, which names the three `telemetry.frontend.*` procedures and their validators. Research answers whether the current struct already bounds a wall-clock-derived duration of this shape.
- Rejections crossing the bridge must be `AppError::Validation { field, reason }` or another sanitized `AppError` variant. They must not carry stack traces, file paths or struct names. Per security-plan §Error Handling (TauRPC bridge) and §Security Anti-Patterns → Logging (NEVER bullet 4).
- Adding a field to `ServiceListItem` changes the shape of the generated bindings but adds no procedure. Both `capability-drift` and `check:staged-artifacts` must hold against the STAGED (git-index) bindings, with `EXPECTED_PROCEDURES` left unchanged. Per security-plan §API Security, "TauRPC capability authorization" row, and §Dependency Security → CI integration (the staged-artifacts fail-condition roster).
- The workflow edits must keep the supply-chain posture:
  - workflow-level `permissions: { contents: read }` stays;
  - every third-party Action stays pinned by its 40-char SHA;
  - `harden-runner` stays the first step of every job;
  - release secrets stay scoped to the `production-release` environment.
  
  Per security-plan §Secret Management ("GitHub Environment scoping"), §Bootstrap phases (`dep-security-ci-gate`) and §Security Anti-Patterns → Secrets. The fix relocates `ANDROMEDA_PULSE_DATA_DIR` and must not restructure the jobs around it.
- `ANDROMEDA_PULSE_DATA_DIR` is a product-binary `*_DIR` var, and the product canonicalizes it on read. Moving it to job-level `env:` or to `$RUNNER_TEMP` must keep it an absolute runner-owned directory with the same meaning. Per security-plan §Input Validation, "CLI / env var inputs" row. Research checks whether the value that finally resolves is accepted by the product's canonicalize-and-confine path on each matrix OS.
- The supply-chain gates run as two invocations:
  - `cargo deny check bans licenses sources` is the pass/fail gate;
  - `cargo deny check advisories` is observed separately, with DISTINCT `RUSTSEC-` ids re-enumerated from scratch.
  
  The `cargo audit` pin is at a between-point: re-verify its basis (`duplicate advisory ID: RUSTSEC-2026-0244`) and record `probe skipped per ratified interval (next: 67)`, with no "Nth consecutive" ordinal. The chunk adds no new dependency. Per security-plan §Dependency Security → CI integration ("Standing deferral", counting rules (a)–(c)).

## Patterns to follow
- Delegated-timing IPC validation follows the existing `telemetry.frontend.*` shape: a pure bounded validator per numeric field plus a closed enum per categorical field, each with a rejection test named after it (security-plan §Input Validation, TauRPC bridge row, 2026-08-21 note).
- The exact-leaf allowlist discipline of `ui.ipc.rejection`:
  - the leaf admits only bounded, non-free-text fields (closed enum, bounded integer);
  - no bare parent-key allowlist entry exists;
  - a dedicated allowlist test pins the admitted set, the fallback discriminator and the banned fields.
  
  Carry this over when naming and admitting the carrying field on `metric.constellation.hue_update_ms`, or on a sibling leaf (security-plan §Security Anti-Patterns → Logging, the `ui.ipc.rejection` NO-SCRUB boundary paragraph).
- Staged-index verification: `capability-drift` folds any non-clean staged outcome into its failure exit. The commit gate reads the INDEX copy of `pulse-app/ui/src/bindings/index.ts`, not the worktree copy (security-plan §API Security, TauRPC capability authorization row).
- Visible advisory dispositions:
  - a finding the first real CI run surfaces that has a stated safe upgrade gets a named owner and is never ignore-listed;
  - only no-safe-upgrade findings take an ID-scoped ignore entry, with provenance and a closing condition.
  
  Per security-plan §Dependency Security → CI integration.

## Anti-patterns to avoid
- NEVER log a raw OTLP attribute value. Folding a service name or service identifier into the timing leaf "to make it per-service" is exactly this ban (security-plan §Security Anti-Patterns → Logging).
- NEVER reference a third-party Action by floating tag, and never grant `contents: write` at workflow level, while editing `ci.yml` / `release.yml` / `update-channels.yml` (security-plan §Security Anti-Patterns → Secrets).
- NEVER add a TauRPC procedure without its `EXPECTED_PROCEDURES` pin and a validated argument struct. This applies if the design moves from "reuse `record_constellation_hue_latency`" to a sibling procedure (security-plan §Security Anti-Patterns → API).

## Contract bindings
- security ↔ obs: the leaf's field set and its cardinality. "`service` is never a label" (obs-plan §5) and "never log OTLP attribute values" (security-plan §Logging & Monitoring) both close the service-identifier option. The allowlist entry at `pulse-app/src/observability.rs` is the shared enforcement point.
- security ↔ tests:
  - the allowlist pin test (`pulse-app/tests/unit_observability_allowlist_delegated_timing.rs`) must carry the renamed or added field and the banned-field checks;
  - validator rejection unit tests are required;
  - once the workflow-parse fix lands, CI actually runs the supply-chain job (`cargo deny`, `check:npm-supply-chain`, `capability-widening-check`, `check:staged-artifacts`) for the first time since 2026-07-25. Its findings are this chunk's findings to triage (security-plan §Bootstrap phases, `dep-security-ci-gate`).
- security ↔ arch: `ServiceListItem` is an IPC payload shape. The bindings regen and the staged-bindings gate bind to arch's §Occupied Resources route list, which stays unchanged because no procedure is added.

## Acceptance criteria contributions
- A wire or log read of the hue-timing leaf shows only allowlisted, bounded fields: the named duration field plus `severity_tier`. There is no service name, identifier, hash or other OTLP attribute value. The allowlist pin test asserts the admitted set and the banned fields (per security-plan §Security Anti-Patterns → Logging).
- Every new or changed field on the delegated-timing input struct has a bounded validator or a closed enum. Out-of-range input returns `AppError::Validation` and is covered by a rejection test named after it (per security-plan §Input Validation, TauRPC bridge row; §Error Handling).
- With `EXPECTED_PROCEDURES` unchanged, `cargo xtask capability-drift` exits 0 and `cargo xtask check:staged-artifacts` exits 0 against the committed index (per security-plan §API Security, TauRPC capability authorization row).
- The pass/fail gate `cargo deny check bans licenses sources` exits 0. `cargo deny check advisories` is recorded separately with DISTINCT ids, and the report carries the `cargo audit` between-point record. All three edited workflow files keep workflow-level `contents: read`, their SHA pins and `harden-runner` as the first step (per security-plan §Dependency Security → CI integration; §Security Anti-Patterns → Secrets).
