# security extract

## Relevance
partial. The chunk is CI, harness and gate work. Its security surface is: (a) the `ci.yml` edits (the release-cache allocation, and possibly a new sample-producing leg or job), which the supply-chain workflow rules govern; (b) the log that the perf gate will read, which the self-observation never-log list governs; (c) the `agent-run.ps1` spawn/exit record files, which the harness path-var carve-out governs. No new IPC, OTLP, MCP or TauRPC surface is in scope.

## Constraints
- Every third-party GitHub Action that this chunk adds or re-parameterizes in `ci.yml` (the rust-cache step behind the `release`/`lint-test` keys, any artifact upload/download step, any step in a new perf-sample leg) must be pinned as `<owner>/<repo>@<40-char-SHA> # vX.Y.Z`, never by a floating tag (per security-plan §Security Anti-Patterns → Secrets; §Bootstrap phases `dep-security-ci-gate`).
- Any NEW job must have SHA-pinned `step-security/harden-runner` as its first step, in the job's current egress posture (per security-plan §Bootstrap phases `dep-security-ci-gate`).
- Workflow-level `permissions` must stay `contents: read`. The cache re-allocation and any perf leg must not elevate to `write`. Signing and notarization credentials stay inside the `production-release` GitHub Environment behind its manual approval gate. Whether the `ci.yml` `release` job touches any of those secrets is research's question (per security-plan §Secret Management → GitHub Environment scoping; §Security Anti-Patterns → Secrets).
- The sample-bearing log is self-observation output. Frame, memory and snapshot sample records may carry bounded numeric or label fields only. They may never carry OTLP attribute values, span/log/metric payloads, snapshot file contents, DuckDB query parameter values or a full product-consumed path. This matters most for the snapshot arm, because the `snapshot.generate` resolver arm of the ban is recorded as UNVERIFIED (per security-plan §Logging & Monitoring → What NEVER to log; §Security Anti-Patterns → Logging).
- Field redaction applies at the subscriber layer, not at call sites. If an emitter is added (scope's producer-existence clause), its fields must pass the existing allowlist and redaction layer rather than bypass it (per security-plan §Logging & Monitoring → Log format).
- The run that produces the samples must feed load from an external injector into loopback `127.0.0.1:4317/:4318`. The product must never self-export to its own ports, so its own metrics stay `tracing`-only (per security-plan §Security Anti-Patterns → API, last two bullets).
- The Windows spawn/exit record files that `agent-run.ps1` writes beside the pidfile are harness-only artifacts, in the same class as the `ANDROMEDA_PULSE_PIDFILE` / `_LOGFILE` carve-out. They take trim + existence check + clean skip, not data-dir confinement. This holds only while the shipped binary never consumes them. Whether `harness:status` resolves them through the pidfile var is research's question (per security-plan §Security Anti-Patterns → Input, carve-out clause).

## Patterns to follow
- Gate exit contract in which an infrastructure or cannot-evaluate outcome is its own arm and NEVER a pass. The npm supply-chain gate (0 green · 1 red · 2 cannot-evaluate, with registry-unreachable never counted as a findings pass) is the analog for the scope's vacuous-gate guard, where all-NEUTRAL or an absent log must not read as green (per security-plan §Dependency Security → npm channel; §Dependency Security → CI integration fail-condition roster).
- Emit a basename via `pulse_app::observability::log_basename` whenever a record names a log dir, run dir or data dir, following the `app.boot.tracing.init` / `app.boot.pid` precedent (per security-plan §Security Anti-Patterns → Logging, second bullet).
- Use an exact allowlist leaf with bounded fields for any new self-observation record. The `ui.ipc.rejection` bounded triple (closed enum + coerced label + bounded integer) is the precedent for a record that carries no client text (per security-plan §Security Anti-Patterns → Logging, `ui.ipc.rejection` paragraph).
- For a harness-only path, trim, run an `is_file()` check and skip cleanly on absence, as with `ANDROMEDA_PULSE_MSEDGEDRIVER_PATH` (per security-plan §Security Anti-Patterns → Input).

## Anti-patterns to avoid
- Referencing `actions/cache`, `Swatinem/rust-cache`, `actions/upload-artifact` or any other third-party Action by `@vN` or a floating tag in a cache or perf-leg edit (per security-plan §Security Anti-Patterns → Secrets).
- Writing canary or OTLP-derived attribute text, snapshot markdown, or full filesystem paths into the perf-sample log. The ban also applies if CI uploads that log as a run artifact, which widens where it lives (per security-plan §Security Anti-Patterns → Logging, first two bullets).
- Pointing any OTel exporter at the product's own `:4317`/`:4318` to manufacture samples (per security-plan §Security Anti-Patterns → API).

## Contract bindings
- security ↔ tests/CI: the perf leg and the cache change share `ci.yml` with the `supply-chain` job. SHA pinning, harden-runner-first and `contents: read` bind on every job the chunk adds or edits. The scope's CI critical-path constraint (1297 s at `fb93fca`) belongs to the operator and P4, not to security.
- security ↔ obs: obs-plan owns the sample record schema (§1/§10 perf gates). security-plan §Logging & Monitoring owns the never-log list and the subscriber-layer redaction those records must pass.
- security ↔ verification-harness: the `agent-run.ps1` spawn/exit mirror sits under the harness path-var carve-out, not under the product-binary canonicalize-and-confine rule.
- Release-job cache: security-plan states no cache-integrity rule for a cache that a PR-reachable job saves and the `release` job restores. This is flagged for P4 awareness only and is not a plan mandate.

## Acceptance criteria contributions
- Every `uses:` line added or changed in `.github/workflows/ci.yml` by this chunk is pinned by a 40-char SHA with a version comment, and every new job's first step is SHA-pinned `step-security/harden-runner`. A grep over the diff against `fb93fca` verifies this (per security-plan §Security Anti-Patterns → Secrets; §Bootstrap phases `dep-security-ci-gate`).
- Workflow-level `permissions` remain `contents: read`, and the diff introduces no new `secrets.*` reference outside the `production-release` environment (per security-plan §Secret Management → GitHub Environment scoping).
- The sample-bearing log that the gate reads contains 0 occurrences of the injector's canary attribute values or snapshot content and 0 full data-dir, run-dir or log-dir paths (per security-plan §Security Anti-Patterns → Logging).
- `cargo deny check bans licenses sources` passes and `cargo audit` exits 0 after the chunk, as two separate invocations (per security-plan §Dependency Security → CI integration).
