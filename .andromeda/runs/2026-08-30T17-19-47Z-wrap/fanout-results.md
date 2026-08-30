# Fan-out results — 2026-08-30-acl-rejection-logging wrap

7 doc-agents, one per spec source. Proposal-carrying returns have raw twins (`.raw-fanout-{doc}.md`);
clean-and-empty returns are recorded here as the sanctioned audit artifact.

| doc | proposals | verdict |
|---|---|---|
| arch | 1 (D-arch-resources) | register `telemetry.frontend.record_ipc_rejection` + `ui.ipc.rejection` leaf; roster 4→5. D-arch-decisions clean (deps affirmatively none; all mechanisms inside locked decisions). |
| security-plan | 3 (D-security-deps ×1 · D-security-logging ×2 incl. 1 dependent) | pointer advance (61 discharged → next 64); `ui.ipc.rejection` no-scrub boundary bullet; clipboard NEVER-log bullet widened. D-security-input / D-security-auth clean. |
| design-system | 2 (D-design-status-narrative primary + dependent) | accent-as-error-text completion re-dated 2026-08-23 → 2026-08-30 with the fourth site added, at both restating sites. D-design-tokens clean. |
| layout-templates | 0 | both detectors clean — no new surface/region; no layout-stated status touched (checked §Primary screens Report-window entry + §Empty/error state "Applies to" scoping). |
| test-plan | 5 (D-tests-coverage ×1 · D-tests-obs-harness ×4 incl. 2 dependents) | new `ipc-rejection-wire-coverage` §1 trigger; boot over-run cause widening; cleanup wrong-reason-pass trio (§3 cleanup · §1 summary · §3 PID lifecycle). D-tests-framework clean. |
| obs-plan | 2 (D-obs-instrumentation §6 · D-obs-pii §8) | dual-site registration of the `ui.ipc.rejection` WARN + exact leaf. D-obs-stack / D-obs-defect-narrative clean. |
| a11y-plan | 0 | both detectors clean; out-of-frame flag routed to the cascade grep: three a11y-plan sites word the accent pair's use context as "error state text" (:180 :348 :485) — inspected at cascade step 2. |

## Validation outcome (main)
- **Routine-apply (10):** arch#1 (2026-07-08 apply-within-structure) · sec-deps#2 (2026-08-23
  APPLY-BY-ACTUAL-CLASS — escalate condition affirmatively absent, actual class = stale measured pointer,
  2026-08-14 doc-only apply) · sec-logging#3+#4 (2026-07-08; NOT the 2026-08-25 reject shape — this adds a
  boundary bullet beside the `incident_events` precedent, it does not qualify the scrub gate) ·
  design#5+#6 (2026-07-08) · tests-coverage#7 (the standing `*-wiring-coverage` trigger class) ·
  tests-harness#8 NARROWED (the "measured 180s default" premise traced to a report imprecision — 180 was
  this session's `HARNESS_STATUS_TIMEOUT` override, report corrected; the amendment applies the measured
  CAUSE (env-triggered release re-fingerprint of a warm binary) + the override lever only, and does not
  restate the default) · obs#12 §6 + obs#13 §8 (2026-08-16 D-obs-pii routine rule — both load-bearing
  conditions hold: full 3-field set asserted both ways; guards under `pulse-app/tests/` where they run).
- **Escalated (3, one family):** tests-harness#9 (§3 cleanup Verification/Exit wrong-reason-pass) +
  dependents #10 (§1 summary) + #11 (§3 PID lifecycle) — a real, measured, PRE-EXISTING harness defect
  (cleanup kills the wrapper PID; app child survives; exit 0 not proof of teardown) whose impl half (fix
  `agent-run.{sh,ps1}` pidfile/kill) has NO owning route entry — the 2026-08-28 playbook rule applies
  as-open amendments only with a NAMED owner and escalates ownerless ones. Resolution recorded below
  after the operator dialogue.
- Checks 2–6: no cross-contradictions · intent-consistent · absence claims cite greps (one enforced
  narrowing, #8) · Expected-amendments floor 4/4 covered by detectors (0 orchestrator-raised) ·
  all 3 disproved-claims entries disposed (2 recorded-in-report per the closed-artifact rule; 1 → #2).
