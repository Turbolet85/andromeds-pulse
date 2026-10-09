# obs extract

## Relevance
partial — the chunk adds a CI supply-chain gate, not product telemetry; obs contributes the gate's output/artifact shape (§9, §11 CI, §10 CI gates) and a negative check (no new `tracing` target ⇒ no §8 leaf owed).

## Constraints
- Gate output consumed by CI must be structured and machine-parseable; obs-plan §11 → CI bans relying on colored terminal output for CI parsing and bans human-review-gated analysis without a machine-parseable export. This is the obs-side reading of the scope's "deterministic, machine-parseable invocation."
- A gate must be hard-enforcing: obs-plan §11 → SLO requires that formal budgets trigger build failure and bans soft/no-enforcement budgets; obs-plan §10 → CI gates states every gate as an explicit exit-code assertion (`… or exit 1`). An advisory-only npm step would violate this class.
- The NEUTRAL-tolerance rule does **not** transfer here. obs-plan §10 → Load-profile constraints scopes it to "any future xtask check script over `agent-latest.jsonl`" (absent log stream ≠ failure). An npm gate reads a manifest/lockfile that is always present, so absence of findings must be GREEN-with-evidence, never NEUTRAL-by-default.
- obs-plan §9 → Pipeline integration enumerates CI stages against the telemetry each produces and its consumer; adding a per-PR scanning stage puts a §9 row in the amendment-expectation set for this chunk.
- obs-plan §8 → Default-deny posture requires that ANY new `tracing` target/field ship an EXACT allowlist leaf enumerating every field its emit site emits, guarded under `pulse-app/tests/`. The chunk's own Boundaries expect no `.rs` production changes, so the expected count is zero — whether the implementation stays at zero new emissions is research's question.
- obs-plan §1 (surfaces table) and §3 → Frontend bridge require the frontend telemetry bridge to be `web-vitals` 5.x → TauRPC `telemetry.frontend.*` → backend `tracing`, i.e. an obs mandate that lives **inside** the npm channel this chunk gates. Any upgrade/removal/pin disposition must not break it. Note: the scope's re-derived enumeration of the 9 runtime `dependencies` does not list `web-vitals`, so whether it is present at all (among the 29 devDeps, or absent, leaving §3's bridge unsatisfied) is research's question.
- obs-plan §11 → Project-specific bans conflating the product's self-observation surface with other surfaces; scan output must not be routed into `~/.andromeda-pulse/logs/agent-latest.jsonl`, and §11 → Logs / §8 Vector 6 basename-only path discipline governs that file (npm tooling emits full `node_modules` paths).

## Patterns to follow
- `xtask/ci/` check-script surface as the CI-gate shape — obs-plan §10 → CI gates names `xtask/ci/heartbeat-gap-check.sh` as the concrete precedent (post-step parse → assert → `exit 1`). This is the obs-side support for the scope's `[inferred]` "project precedent is the `cargo xtask` task surface."
- JSON-in / `jq`-assert / exit-code-out gate composition (obs-plan §10 → CI gates, all five bullets) — the established way a gate is both agent-readable and enforcing at once.
- CI artifact + `gh run download` triage flow (obs-plan §9 → Telemetry artifact handling / CI failure → artifact triage) — if the scan report is retained for later reading, this is the established consumption path rather than job-log scrollback.
- Guard-lives-where-it-runs: per the 2026-08-16 / 2026-08-22 amendment line, assertions belong under `pulse-app/tests/` because `[lib] test = false` makes src-level `mod tests` dead. Applies only if this chunk adds a Rust-side assertion.

## Anti-patterns to avoid
- Human-readable-only / color-dependent gate output, or a "review the output" step with no machine-parseable export (obs-plan §11 → CI).
- A soft gate: findings printed but the job stays green, or the pass/fail signal mixed into a designed-red observed stream (obs-plan §11 → SLO; consistent with the scope's two-invocation lesson).
- Emitting scan results into the product's self-observation log, or adding any `tracing` target/field without its §8 exact leaf (obs-plan §11 → Project-specific; §8 → Default-deny posture).

## Contract bindings
- obs ↔ tests: obs-plan §3 Observability Harness Contract + §9/§10 CI gates bind to the test plan's harness/invocation ownership — tests own *how* the gate is invoked in CI, obs owns that its output is structured and its failure is exit-code-enforced.
- obs ↔ security: the chunk is owned by security-plan §Dependency Security / §Supply chain + CI (disposition discipline, exception provenance). obs adds only output shape and the no-new-telemetry-target check; the ignore-list/exception policy is security's, not obs's.
- obs ↔ frontend (npm channel): obs-plan §3 → Frontend bridge makes an obs mandate a *consumer* of the very dependency channel being gated — a disposition that drops or downgrades the telemetry bridge package would break an obs requirement.
- The folded `cargo audit` PREREQ (probe/interval recording at wrap) is security/tests-owned; obs contributes no requirement to it.

## Acceptance criteria contributions
- The npm gate emits structured, machine-parseable output and signals via exit code — no color-dependent or human-review-gated parsing (per obs-plan §11 Obs Anti-Patterns → CI).
- An undispositioned finding fails the build (non-zero exit); no advisory-only/soft outcome, and any designed-red observed signal is a separate invocation from the pass/fail gate (per obs-plan §11 → SLO and §10 → CI gates).
- The chunk introduces zero new `tracing` targets/fields; if any is introduced, it carries an EXACT §8 allowlist leaf enumerating every emitted field with a guard under `pulse-app/tests/` (per obs-plan §8 → Default-deny posture).
- No scan output (which carries full `node_modules` paths) is written into `~/.andromeda-pulse/logs/agent-latest.jsonl`, keeping the CI supply-chain signal and the self-observation surface separate (per obs-plan §11 → Project-specific + §8 Vector 6).

## Relevant amendment history
- **2026-08-29-advisory-backlog** (the immediately preceding supply-chain chunk, same route family) — a dependency bump (rmcp 0.6.4 → 3.1.4) forced label corrections at eight obs-plan sites (§1 ×2, §3 ×3, §4 ×2, §6, §11), because the plan named the dependency version and a falsified mechanism claim rode along with it. Directly relevant: obs-plan §1/§3 likewise name npm-channel versions (`web-vitals` 5.x, React 19 + Tailwind v4 in the surfaces table), so an upgrade disposition landed by this chunk that moves one of those carries the same plan-label amendment obligation. Precedent resolution was amend-to-measured-reality.
- **2026-06-10 (chunk #99 tag gate)** and **2026-08-26 / 2026-08-28 (ingest-consumer entries)** — codified CI check scripts living under `xtask/ci/`, and the NEUTRAL-tolerance rule together with its scope (`agent-latest.jsonl` streams only). Relevant as the script-shape precedent, and as the boundary that stops NEUTRAL-tolerance from being mis-ported into a manifest-scanning gate.
- **2026-08-16 (`triage.baseline.bootstrap_window.override`)**, **2026-08-22 (`redactions_applied`)**, **2026-08-29 (incident-persist leaf 4 → 5)** — the recurring class where a new/extended emission shipped ahead of its §8 leaf, leaving fields silently redacted, plus the "guard where it actually runs" rule. Relevant here only as the negative check: this chunk is expected to add no emission, so no leaf should be owed.