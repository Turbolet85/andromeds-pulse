# Fan-out results — 2026-10-09-boot-smoke-s-early-exit-found-and-closed

Seven doc-agents, one batch. 19 detectors over 7 prompts (asserted before sending: 2 + 4 + 2 + 2 + 3 + 4 + 2).
Returns were stripped of commentary, entity-decoded (no entity found in any return) and parsed. Each proposal below
is the orchestrator's condensed line of the returned `change`; the applied text was re-derived from the report, never
pasted. Dispositions carry the Validate check that decided them.

## Verdicts
- architecture — 13 proposals (4 primaries, 9 dependents), all `D-arch-resources`; `D-arch-decisions` no drift.
- security-plan — 4 proposals (1 `D-security-logging`; 3 `D-security-input` at the detector's `escalate` severity);
  `D-security-auth`, `D-security-deps` no drift.
- design-system — `proposals: []`. Commentary stripped; raw twin `.raw-fanout-design-system.md`.
- layout-templates — `proposals: []`. Commentary stripped; raw twin `.raw-fanout-layout-templates.md`.
- test-plan — 12 proposals (11 `D-tests-obs-harness`, 1 `D-tests-coverage`); `D-tests-framework` no drift.
- obs-plan — 5 proposals, all `D-obs-defect-narrative`; `D-obs-instrumentation`, `D-obs-stack`, `D-obs-pii` no drift.
- a11y-plan — `proposals: []`. Commentary stripped; raw twin `.raw-fanout-a11y-plan.md`.

## architecture
| # | Section | Change (condensed) | Disposition |
|---|---|---|---|
| A1 | §Occupied Resources → xtask CLI surfaces | register `cargo xtask harness:ready`: object, four arms, exit codes, the both-ports rule | apply — check 1: Accurate this-chunk addition; the registry enumerates xtask verbs (`harness:status` is one), so it is not Registry over-reach; check 5: expected amendment 3 |
| A2 | same | register `cargo xtask harness:settled`: arms, exit codes, field set, the kept file | apply — as A1 |
| A3 | §Occupied Resources → Environment variables | a harness-only entry for `DISPLAY` · `DBUS_SESSION_BUS_ADDRESS` · `XDG_RUNTIME_DIR` | apply — check 1: New env var registration (read by presence, parsers pinned); the operator's P4 classification (inputs#I3): routine harness evidence, not a boundary widening |
| A4 | §Occupied Resources → Filesystem locations, the `logs/` item | the two harness-written files; the artifact's five files | apply — check 1: Accurate this-chunk addition; inputs#I3 as A3 |
| A5 | xtask CLI surfaces, the `agent-run` entry (dependent) | the boot poll is `harness:ready`; the conditional failure line; the ps1 mirror unwitnessed | apply — atomically with A1 |
| A6 | same entry (dependent) | "the three invocations" → four in one `xvfb-run -e`, cleanup unconditional | apply — with A2 |
| A7 | the `harness:status` entry (dependent) | callers: the `status` verb; the boot poll moved to `harness:ready` | apply — with A1 |
| A8 | Filesystem locations, the pid-file item (dependent) | two more xtask readers; the boot poll reaches the status verdict through `harness:ready` | apply — with A1 |
| A9 | Filesystem locations, the exit-record item (dependent) | `harness:ready` and `harness:settled` also report the record | apply — with A1 |
| A10 | Environment variables, `ANDROMEDA_PULSE_PIDFILE` (dependent) | also threaded into the `harness:ready` child; drop the script line cites | apply the threading half; **reject the cite-dropping half** — the citation sweep read those numbers and left them as proved (`unmoved`); a cite is the sweep's, not a proposal's |
| A11 | Environment variables, `ANDROMEDA_PULSE_LOGFILE` (dependent) | as A10 | as A10 |
| A12 | Tauri IPC routes, `record_webgpu_adapter` (dependent) | retire "some runs only / four of seven / 0.57 s to 1.40 s" | apply — check 6: disproved claims 2 and 3 |
| A13 | the `agent-run` entry (dependent) | the `/dev/null` streams are the wrapper's own; the app's go to `logs/boot.log` | apply — check 6: disproved claim 4 |

## security-plan
| # | Section | Change (condensed) | Disposition |
|---|---|---|---|
| S1 | §Security Anti-Patterns → Logging, the `ui.webgpu.adapter` paragraph | retire the some-runs-only witness; the smoke reads past the adapter request | apply — check 1: Accurate this-chunk addition; check 5: expected amendment 5; check 6: claims 2 and 3 |
| S2 | §Security Anti-Patterns → Input, the harness-only state-file class | new readers, the two harness-written files, the three system variables | apply — check 1: the `escalate` severity fired outside its class (no unvalidated boundary: Coverage reads validation ✓ on both verbs) → the escalate-outside-class rule, routine; the subject was put to the operator at P4 as a possible boundary widening and classified routine harness evidence (inputs#I3, the pc overseer, founder-delegated — a delegate's answer, so named at the route-resolve card); the body names that classification and the operator's standing stop |
| S3 | §Input Validation, the CLI / env var row (dependent) | the xtask-only reads and their bounded parses | apply — with S2 |
| S4 | §Threat Model Summary, CLI input trust boundary (dependent) | the restated carve-out enumeration gains the new members | apply — with S2 |

## test-plan
| # | Section | Change (condensed) | Disposition |
|---|---|---|---|
| T1 | §3 → 5-command implementation, Readiness signal | poll `harness:ready`; the OPEN gap closed with its measurements; what stays with P-129 | apply — check 1: Accurate this-chunk addition; check 5: expected amendment 1; check 6: claim 1 |
| T2 | same key, Exit code (dependent) | failure is no `ready` verdict; the conditional line | apply — with T1 |
| T3 | same key, Command body, the ps1 sentence (dependent) | `Invoke-Ready`; parsed, never run | apply — with T1 |
| T4 | same key, Timeout (dependent) | readings under `harness:ready` beside the dated 1.953 s | apply — with T1 |
| T5 | §1 harness one-line summary (dependent) | ready is the `harness:ready` verdict; the "measured gap, open" clause retired | apply — with T1 |
| T6 | same key, Command body, the waiting-subshell sentence | the `/dev/null` streams are the wrapper's own | apply — check 6: claim 4 |
| T7 | §9 Pipeline structure, the Boot smoke row | the four-invocation step, `harness:settled`'s contract, the five files, the four pins | apply — check 5: expected amendment 2 |
| T8 | §9 Build failure conditions (dependent) | the cycle also fails on a non-zero `harness:settled`; cleanup regardless | apply — with T7 |
| T9 | §1 the WebGPU throughput trigger row (dependent) | the CI frame line; the four-of-seven split retired | apply — check 6: claim 3 |
| T10 | §1 `ipc-rejection-wire-coverage` (dependent) | the CI boot log holds the adapter record on every settled run, still unasserted | apply — check 6: claims 2 and 3 |
| T11 | §1 `perf-slo-check-arm-coverage`, discharged row (dependent) | the old CI reading dated; the current one added | apply — check 6: claim 3 |
| T12 | §1 `harness-cleanup-verdict-and-boot-spawn-shell-coverage` | widened: the new line and the ps1 edit have no committed test | apply — check 5: expected amendment 2; `D-tests-coverage` |

## obs-plan
| # | Section | Change (condensed) | Disposition |
|---|---|---|---|
| O1 | §10 Performance budgets, the frame row | the old step order as history; the current order and the `ci#37979648967` reading | apply — check 5: expected amendment 4; check 6: claims 2 and 3 |
| O2 | §10 CI gates, the perf-budget bullet (dependent) | the boot-smoke frame line is no longer run-dependent | apply — with O1 |
| O3 | §1 Telemetry triggers, the multi-platform row (dependent) | the CI witness no longer depends on how long the app lives | apply — with O1 |
| O4 | §8 the `ui.webgpu.adapter` bullet (dependent) | the WARN arm's CI tally gains the ninth reading | apply — with O1 |
| O5 | §9 Telemetry artifact handling, the log-file row | what the boot job's artifact holds | apply — check 5: expected amendment 4; carried under the narrative detector as the agent says; inputs#I3 as A3 |

## Raised by the orchestrator (no detector binds them)
| # | Section | Change | Basis |
|---|---|---|---|
| X1 | architecture §Infrastructure Patterns → CI/CD approach | a `pull_request` run builds the pull request's merge ref; what "equal source" means across such runs | the operator's direction (inputs#I4 item 3: measure it and write it where a later plan's "equal source" will read it); the report's Cross-project bullet carries the measurement |
| X2 | obs-plan §9 CI subscriber default fields, `git.commit.sha` | on a pull-request run the field names the merge commit | the same direction and measurement |

## Counts
- Proposed 34 · applied 34 (two with a rejected sub-part: A10, A11) · rejected whole 0 · escalated 0.
- Rejected before the checks for a source the report does not carry: 0.
- Raised by the orchestrator: 2 (X1, X2), both applied.
- Check 2 (cross-contradiction): none; the two obs-plan and three test-plan lines edited twice were edited in one direction.
- Check 3 (intent-consistency): the report's deviations are justified (the conditional line; the open cases); the scope
  record holds no line.
- Check 6: disproved claims 1 to 4 matched by proposals; claim 5 is a chunk-artifact claim, disposed by its report entry.
