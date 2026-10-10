# Tests validation — route draft

## Insert
- Between `Agent harness drives the console engine` and `Engine end-to-end gate reachable`: **"Harness verbs pinned by committed tests — each cleanup verdict, each boot failure reason and the rotated log family asserted on every run"** (epoch: `Epoch 1`)
  Reason: Per test-plan §1 Pending coverage triggers `harness-cleanup-verdict-and-boot-spawn-shell-coverage` and `harness-log-family-resolution-coverage`, these arms are proven only by one-time live legs; re-pointing the verbs at a new process rewrites exactly those arms, and every later gate in the route reads through them.

- Between `Supply-chain gate re-based on the smaller graph` and `Log allowlist describes this engine`: **"Coverage gate re-based on the smaller workspace — line, branch and function readings taken after the removals; xtask's temporary exclusion decided"** (epoch: `Epoch 3`)
  Reason: Per test-plan §10 "Cumulative across workspace" (and §9 rows "Coverage report" / "Quality gates"), the 75/70/85 gate passes on function coverage only with xtask excluded "TEMPORARILY" (87.80 % against 85 %), and no chunk re-reads it after Epochs 2–3 remove whole crates and their test targets.

- Between `Disk store measured under load` and `Door inside the engine's process`: **"Load profiles re-based on the engine — four profiles through the network receiver into the disk store; the high profile's assertion restated; their gating run named"** (epoch: `Epoch 5`)
  Reason: Per test-plan §10 "Load profiles (release / tag gate)" and §1 triggers `load-test: OTLP receiver saturation` / `performance-budget`, the "high" assertion leaves with the cadence coordinator in Epoch 3 and the suite's release/tag gate leaves with the desktop distribution in Epoch 2. The disk-store chunk reads file size and memory, not drops, recovery or export latency.

## Reorder
- Move `Shared telemetry test data` before `Engine end-to-end gate reachable`
  Reason: Per test-plan §7 "Self-bootstrapping requirement" and §3 key `bootstrap-phases` item 6, fixtures precede their first consumer; the gate's second-host sender is the first producer of synthetic telemetry for the engine and would otherwise pin a sender the factories then duplicate.

## Rewrite
- `Agent harness drives the console engine`: "panic, heartbeat-gap and budget checks grade its log" → "panic, heartbeat-gap, process-end and budget checks grade its log"
  Reason: Per test-plan §1 pending trigger `exit-hook-main-composition-coverage`, the process-end record's production composition has no committed witness because the exit could not be induced headless; a console engine stopped by the harness makes it gradable on every run.

- `Theme 0 checked`: "reading recorded" → "reading recorded; critical paths restated"
  Reason: Per test-plan §1 Critical paths and §2 E2E row ("all 7 critical paths"), after Epochs 2–3 only P4's retirement is recorded in a chunk, while P5, P6 and P7 leave unrecorded and P1/P3 lose their IPC verification signals.

- `Large model names the planted cause`: "measured over planted faults" → "measured over planted faults; a reading, never a gate"
  Reason: Per test-plan §2 agent-runnable invariants (deterministic) and §10 zero-flakiness budget, plus the plan's standing ruling that a real-model generation cannot give a deterministic verdict, no gate may depend on model output.

- `System notification on a change of state`: "carrying the short report" → "carrying the short report; each raise leaves a record a check reads"
  Reason: Per the test-plan preamble's agent-driven invariant and the §1 `desktop-native` surface row (notification verified by a captured event, never by eye), a desktop notification is not observable in CI, so the Theme 5 path pinned in the engine gate needs a machine-readable signal.
