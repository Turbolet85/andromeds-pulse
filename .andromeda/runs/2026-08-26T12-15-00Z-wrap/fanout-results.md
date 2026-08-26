# Fan-out results — 2026-08-26-cadence-runaway-blocking-pool

7 Explore doc-agents, one per spec source, one parallel batch. Report read:
`andromeda-pulse-0.3.0/chunks/2026-08-26-cadence-runaway-blocking-pool/report.md`.

| doc | detectors | verdict |
|---|---|---|
| architecture.md | D-arch-resources · D-arch-decisions | `proposals: []` — clean |
| security-plan.md | D-security-input · D-security-auth · D-security-deps · D-security-logging | 1 proposal (D-security-deps) |
| design-system.md | D-design-tokens | `proposals: []` — clean |
| layout-templates.md | D-layout-surface | `proposals: []` — clean |
| test-plan.md | D-tests-coverage · D-tests-framework · D-tests-obs-harness | 3 proposals |
| obs-plan.md | D-obs-instrumentation · D-obs-stack · D-obs-pii | 3 proposals |
| a11y-plan.md | D-a11y-surface · D-a11y-obs-schema | `proposals: []` — clean |

**Total: 7 proposals across 3 docs; 4 docs clean.** No return needed stripping beyond the fenced
YAML; no raw twin was warranted.

## Clean returns — reasoning recorded

- **architecture.md** — no IPC/procedure, endpoint, port, socket, env var, or crate added; the two env
  vars the smoke used are already registered (`ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS`,
  `ANDROMEDA_PULSE_L4_DETERMINISTIC`). Dependencies "none added, none bumped". The agent explicitly
  declined to raise the pre-existing "twelve library crates" count-word mismatch as this chunk's drift —
  correct, and consistent with the prior wrap's ruling on the same line.
- **design-system.md** / **layout-templates.md** / **a11y-plan.md** — zero `pulse-app/ui/**` paths; all
  three Coverage rows read `a11y n/a` / `tokens n/a`; no rendered surface, region, or interactive
  element. a11y additionally verified the violation-schema bind is untouched (the chunk moved emit-site
  FIELD sets, not the `timestamp`/`level`/`target`/`message`/`fields` envelope).

## Orchestrator's own verification (Validate checks 4 + 5)

The obs-plan backlog proposal asserted the census mechanism was "unverified for the remaining four".
Check 4 (absence needs evidence) required measuring rather than asserting. **Two wrong bases were used
before the correct one** — a raw string grep (which the 2026-08-21 rule bans, since test-module literals
pollute it) and a single-line `by_target.insert("key",` grep (the registrations are multi-line). Measured
on the registration basis (`by_target.insert(` + next line):

| §8 backlog target | resolves to | actual mechanism |
|---|---|---|
| `metric.pipeline.l1a.query_count_total` | bare `metric` key (`value`/`unit`/`module`) | PARTIAL — keeps `value`, loses `query_name` |
| `metric.pipeline.l1a.query_latency_p99_milliseconds` | bare `metric` key | PARTIAL — keeps `value`, loses its three labels |
| `triage.cue.tick` | exact leaf, 5 of 7 fields | PARTIAL — **fixed this chunk** (now 9 of 9) |
| `triage.incident.auto_resolve.tick` | nothing (no `triage`, no `triage.incident`) | TOTAL — matches the census claim |
| `interpretation.model.load` | exact leaf `[model_identity, tier, file_size_bytes, load_status]` | PARTIAL — `inference_mode` absent |

So the census's stated mechanism ("resolve to no allowlist entry") is accurate for **1 of 5**.

**Separate finding, outside this chunk's scope:** a **bare `interpretation` key EXISTS** at
`pulse-app/src/observability.rs:1934` carrying a populated set (`model_profile`, `model_tier`, …), while
obs-plan §8 and `.claude/rules/observability.md` both state as an invariant that *"no bare `interpretation`
prefix key may exist"*. The doc is not wrong — the CODE violates a documented invariant. Escalated.

Check 5 (expected-amendments floor) — the plan's list reconciled:
- "obs-plan §6 + §8 — `triage.cue.tick`'s exact leaf (dual-site)" → **§8 only**. §6's per-target
  enumeration is its WARN row; `triage.cue.tick` is an INFO heartbeat covered by §6's generic `info` row
  ("heartbeat ticks"), so no §6 entry is owed. Verified, not assumed.
- "obs-plan §1 + §5 + §8 — only if a tick GAINS fields (triple-site rule)" → **§5 + §8**. §1's tick
  enumeration lists the four subsystem heartbeats (`ingest`/`buffer`/`viz`/`plugins`); neither
  `triage.cue.tick` nor `cadence.tick` has ever been in it, so the triple-site premise does not hold for
  them. Adding them would document pre-existing targets under this marker.
- `cadence.tick` §8 leaf — **raised by the orchestrator**, no detector proposed it; the report's
  "Counts / qualifiers moved" substantiates it (7 → 8 fields, enumerated nowhere in obs-plan).
