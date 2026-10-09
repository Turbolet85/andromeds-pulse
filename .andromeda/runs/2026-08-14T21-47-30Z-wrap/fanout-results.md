# Fan-out results — 2026-08-14-fingerprint-feed-capture-repair

7 Explore doc-agents, one parallel batch. Verdicts below; the full returned proposal text is
reproduced here rather than in per-doc `.raw-fanout-*.md` twins — every return was well-formed YAML
(three carried trailing explanatory prose, stripped) and this file carries their complete content,
so it is the sanctioned audit artifact for all seven.

**Harness note:** four returns (arch, design-system, layout-templates, a11y-plan) tripped the
subagent output guard on an instruction-shaped pattern (`settings-json`) — an artifact of the report
naming `.claude/settings.json`. Inspected: no directive-shaped text in any return; proposals unaffected.

| doc | verdict | proposals |
|---|---|---|
| arch | drift | 1 (D-arch-resources) |
| security-plan | clean | 0 |
| design-system | clean | 0 |
| layout-templates | clean | 0 |
| test-plan | drift | 4 (3× D-tests-framework as one dependent-of group, 1× D-tests-coverage) |
| obs-plan | drift | 4 (2 primaries + 2 dependent-of, all D-obs-instrumentation) |
| a11y-plan | clean | 0 |

Every escalate-severity detector (`D-security-input`, `D-security-deps`, `D-obs-pii`) returned clean.

## Proposals + disposition

### arch — D-arch-resources (warning) → APPLIED routine
§Occupied Resources → Environment variables: register `PYTHONUTF8` + `PYTHONIOENCODING` as
harness-only entries (set by `agent-run.{sh,ps1}` + mirrored in `.claude/settings.json`, not consumed
by the production binary). Rationale cited the report's Harness + Schema/config bullets and the
existing `ANDROMEDA_PULSE_PIDFILE` / `_LOGFILE` / `_DATA_DIR_KEEP` harness-only precedent.
Playbook: line-50 routine-APPLY (accurate this-chunk addition inside a documented structure).

### test-plan — D-tests-framework ×3 (warning, dependent-of group) → ESCALATED, resolved APPLY
Primary §2 directory-pattern + dependents §2 test-function-naming and §4 Rust test-file location:
document that `pulse-app`'s `[lib] test = false` makes co-located `mod tests` dead code, so its
probes live in `pulse-app/tests/*.rs` via `pub` + `#[doc(hidden)]`.
**Why escalated:** two playbook rules conflicted — line-50 routine-APPLY (accurate correction inside
a documented structure; the chunk did add a file under the exception) vs the 2026-06-29
routine-HANDOFF (a real but PRE-EXISTING inconsistency the chunk merely exposed — 58 of the 59
files in `pulse-app/tests/` predate it).
**Resolution (operator):** APPLY. The HANDOFF rule exists to stop a one-artifact fix from creating a
worse spec↔impl mismatch; here there is no impl half to fix — the impl IS the exception, only the
doc was wrong, so the fix completes in one artifact and strictly improves alignment.

### test-plan — D-tests-coverage (warning) → APPLIED routine
§1 Pending coverage triggers: add `harness-encoding-relay-coverage` — the UTF-8 relay ships with no
behavioural assertion (report Coverage flags it `unrunnable-here`). Playbook: line-50 routine-APPLY.

### obs-plan — D-obs-instrumentation ×4 (warning; 2 primaries + 2 dependent-of) → APPLIED routine
§5 Metric Coverage (new tick-aggregated counter row for the three feed counters) · §6 Log Coverage
(the `app.boot.buffer.degraded` WARN target) · §1 Heartbeat ticks (dependent-of: extend the
`buffer.tick` field enumeration) · §8 PII allowlist (dependent-of: extend the `buffer` per-module
list + add the boot-target leaf entry). All are this-chunk introductions landing inside already
documented structures. Playbook: line-50 routine-APPLY.

## Validation checks

1. **Playbook** — every proposal matched line-50 routine-APPLY except the D-tests-framework group,
   which matched two rules in opposing directions → escalated.
2. **Cross-contradiction** — none; no two proposals edit the same section in opposing directions.
3. **Intent-consistency** — the report diverges from chunk intent (the premise falsification). A
   *justified* divergence, so intent was incomplete → disposed under check 6.
4. **Absence-needs-evidence** — the absence claims (no such target documented, no other occurrence)
   each cite the report plus the agent's own grep; the orchestrator re-ran the cross-master grep
   independently at cascade step 2 and confirmed.
5. **Expected-amendments reconciliation** — the plan's list named obs-plan "§3 Required tick fields
   per module"; the buffer.tick enumeration actually lives at §1 line 101 (§3's Heartbeat-ticks
   subsection carries only a generic format example, verified — no duplicate site missed). Section
   number corrected; coverage floor MET (§1-as-§3 ✓, §6 ✓, §8 ✓), nothing under-ran.
6. **Disproved-claims disposition** — the dead-region-at-HEAD premise is disposed as
   orchestrator-raised routine: the master record's completion text is rewritten to describe actuals
   (capture + instrumentation + proof, no repair needed) at the P7 flip, and the falsification is
   recorded in scope/plan via the report rather than silently dropped.

## Cascade

- **Lateral binds:** test-plan §3 ↔ obs-plan §3 untouched (neither §3 amended; the 5-command
  discipline, status shape, PID path, and log sink are unchanged). a11y ↔ obs violation schema
  untouched.
- **Cross-master citation grep:** the amended wordings have no other occurrence in the seven masters
  (the two "no separate" hits in arch §321 / obs §332 are unrelated phrases; `route.md` is v2
  forensic history, not a master).
- **Preserve-verbatim homes:** one hit in `.claude/rules/testing.md:25` — in the GENERATED body, not
  `## Session Additions`, so it is a leaf re-derive rather than a curation route. `observability.md`'s
  other matches all sit in Session Additions and were left verbatim.
- **Leaf re-derives:** `.claude/rules/testing.md` (File placement) + `.claude/rules/observability.md`
  (Required tick fields). arch's leaves are a no-op — CLAUDE.md's pointer table points AT arch for
  env vars rather than duplicating them, and `stack.md` carries no env-var enumeration. Both
  `*-summary.md` docs: no matches, no-op.
