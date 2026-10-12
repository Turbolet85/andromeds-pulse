# 1A tree — andromeda-pulse-0.4.0

_Phase 1A decomposition. The sub-block tier is visible here and dropped at 1C. Each leaf: candidate title +
`(source: requirement id · intent finding · the master section it builds on, replaces or retires)`. This file is
where every chunk's upstream is cited in full; the draft's lines carry the requirement ids only, inside the
25-word form._

_The six plans describe the 0.3.0 system. They are cited below only where a chunk RETIRES or REPLACES what they
describe (marked `retires:` / `replaces:`), or builds on a part that stays (`builds on:`). No leaf installs, extends
or polishes a retired surface, and the bootstrap merging map yields no leaf for one._

## Post-merge changes (Phase 3 — the tree below is the Phase 1 state)

- MOVED to Epoch 1, first: Corpus encryption at rest retired (source unchanged; security Reorder, a verified dependency — `merge-decisions.md`).
- INSERTED, Epoch 1: Agent harness drives the headless engine (source: P-086 · intent F1 · builds on: test-plan §3 Test Harness Contract → 5-command implementation, PID file; obs-plan §9 Telemetry artifact handling and §10 CI gates; replaces: test-plan §9 "Boot smoke (harness)" as the producer of the graded log).
- INSERTED, Epoch 1: End-to-end paths re-driven through the engine (source: P-086 · intent F1, §3 · replaces: test-plan §6 "Drivers per surface", the TauRPC and Tauri-channel drivers of critical paths P1–P7).
- INSERTED, Epoch 1: Shared telemetry test data (source: P-089, P-094 · intent F3, F7 · builds on: test-plan §3 Bootstrap phases item 6, recorded unimplemented; test-plan §7 Test Data & Fixtures).
- INSERTED, Epoch 2: Supply-chain gate re-based on the smaller graph (source: P-083, P-085 · intent R1, R3 · builds on: security-plan §Dependency Security → CI integration and §Bootstrap phases `dep-security-ci-gate`).
- INSERTED, Epoch 4: Disk store measured under load (source: P-090, P-091 · intent F4, F5 · replaces: obs-plan §10 the buffer gauge and drain-progress invariant as defined against the in-memory ring; test-plan §10 "Load profiles (release / tag gate)" as stated against it).
- MOVED within Epoch 4: Door admission and bounds, ahead of Door queries.
- CORRECTED: test-plan §9 "Capability verification matrix" is NOT among the gates `Window's gates retired` retires (it checks product capabilities P-001…P-060); its entries that anchor into the window, the model and the encryption are pruned by each removal, and `Version close on Linux` states the outcome.
- Scope hints rewritten in place: 15 lines (`merge-decisions.md`).

## Epoch 1 — Foundation: a headless engine and its gates on Linux (P-086, P-103)

- Headless engine entry point (source: P-086 · intent F1 · replaces: architecture §Established Decisions [Platform] "Tauri 2 desktop shell" as the process that hosts the engine; builds on: architecture §Established Decisions [In-Process Channel Architecture], [Database], [Backend Framework — OTLP receiver])
- Engine end-to-end gate reachable (source: P-086, P-090 · intent F1, F4, §7 · `phase-1/synthesis-protocol.md` §Placement · builds on: test-plan §3 Test Harness Contract → 5-command implementation and Bootstrap phases; replaces, once the window's gates retire: test-plan §9 "Boot smoke (harness)")
- Windows and macOS CI legs retired; pre-push check native on Linux (source: P-103 · `.andromeda/residuals.md:21` · intent §8 "builds and tests on Linux alone" · retires: test-plan §9 "Matrix builds" Windows and macOS rows and the `release` job's macOS/Windows builds; replaces: architecture §Occupied Resources → xtask CLI surfaces `pre-push:linux`, the WSL verb; builds on: architecture §Infrastructure Patterns → CI/CD approach)

## Epoch 2 — What leaves (P-083, P-084, P-085)

### Sub-block: the window (R1)
- Window's gates retired (source: P-083 · intent R1 · retires: test-plan §9 "A11y suite", "Boot smoke (harness)", "Staged artifacts", the `supply-chain` job's npm channel gate, "Capability verification matrix"; test-plan §6 driver row desktop-webview and scenario P5; test-plan §1 surfaces desktop-webview, desktop-webview unit tests, desktop-webview a11y tier, desktop-native; a11y-plan §3 A11y Assertion Harness Contract and §9 CI Integration, whole; obs-plan §10 "WebGPU canvas frame" budget row; security-plan §Bootstrap phases `dep-security-ci-gate`, its capability-drift and npm halves)
- Window retired (source: P-083 · intent R1 · retires: architecture §Established Decisions [Platform], [WebGPU Visualization Surface], [Tauri IPC Bridge — Surface 2], [Deployment / Release Pipeline], [Code Signing], [Distribution Channels]; architecture §Standard Contracts, the three Tauri IPC envelopes, the paginated-list envelope and the real-time push contract; architecture §Occupied Resources → Tauri IPC routes, Tauri IPC events, Tauri capability identifiers, Updater channel, Bundle artifact names; architecture §Cross-cutting Patterns → OS notification policy, Tray icon policy, Webview IPC capability policy; design-system, layout-templates and a11y-plan, each whole; security-plan §API Security rows CSP, TauRPC capability authorization, Updater capability isolation; security-plan §Secret Management, the signing and updater key custody; obs-plan §2 the `telemetry.frontend.*` bridged observables)

### Sub-block: the local model (R2)
- Incident worded from the engine's own facts (source: P-084, P-099 · intent R2, F12 · `.andromeda/residuals.md:19` · replaces: architecture §Established Decisions [LLM Inference Runtime — L4 interpretation layer] as the author of an incident's text; builds on: architecture §Established Decisions [Fault Identity], the cue cause label that already grounds the title)
- Local model retired (source: P-084 · intent R2 · `.andromeda/residuals.md:19` · retires: architecture §Established Decisions [LLM Inference Runtime — L4 interpretation layer]; architecture §Stack rows "AI/ML serving" and "L4 grammar conversion check (test-only)"; architecture §Occupied Resources → the model, binary, allow-root, hardware-profile and deterministic-mode environment variables and the per-spawn grammar temp file; obs-plan §10 "L4 constrained inference" budget row and its dev-host grader; security-plan §Security Anti-Patterns → Input, the narrowed exception for the three model path variables; test-plan §1 pending triggers for the decision probe and the latency grader)

### Sub-block: the corpus's encryption, and what still holds (R3, F1)
- Corpus encryption at rest retired (source: P-085 · intent R3 · retires: security-plan §Data Protection → At rest, the corpus's cell encryption and the corpus-key lock file; security-plan §Secret Management → Runtime, the corpus encryption key; architecture §Stack row "Secret / key storage" and §Occupied Resources → Corpus SQLite → At-rest posture; builds on: security-plan §Security Anti-Patterns → Logging, the scrubber at the write boundary, unchanged)
- Detection parity after the removals (source: P-086 · intent F1 "everything 0.3.0 detected is still detected", §3 · 0.3.0 capabilities P-074 and P-077 · builds on: test-plan §5 Integration Test Strategy; architecture §Cross-cutting Patterns → Test-time telemetry injection)

## Epoch 3 — The engine stands on its own node (P-087, P-088, P-089, P-090)

### Sub-block: the network receiver (F2, F2b)
- Network OTLP receiver behind a token (source: P-087 · intent F2 · replaces: security-plan §Threat Model Summary "Auth model: none … Loopback-only OTLP binding is the de facto authorization boundary", §Authentication & Authorization "N/A (deliberate)", §Data Protection → In transit "TLS: N/A for inbound", §API Security row "Host header allowlist"; architecture §Occupied Resources → Network ports "bound on `127.0.0.1` only"; builds on: architecture §Established Decisions [Backend Framework — OTLP receiver], [API Style — OTLP Receiver Surface 1])
- Receiver refusals and per-sender bounds (source: P-088 · intent F2b, proposed for the founder's review · replaces: security-plan §API Security row "Rate limiting", the global cap with a per-source-port key; builds on: security-plan §API Security row "Request size limit", §Input Validation, the post-decode invariants; security-plan §Logging & Monitoring)
- Token lifecycle by engine command (source: P-088 · intent F2b · no plan section states a runtime credential the engine issues: security-plan §Secret Management lists release-pipeline keys and the corpus key only; builds on: security-plan §Secret Management "What counts as secret", the never-logged rule)

### Sub-block: event time (F3)
- Checks reason by event time (source: P-089 · intent F3, ruled · builds on: architecture §Conventions → Timestamp handling; replaces: the system-clock reading in windows, cadences, the restart detector and the retention cutoff that intent F3 observes)
- Recorded stream replays to the same result (source: P-089 · intent F3 · builds on: test-plan §2 "Deterministic — no flaky retries"; architecture §Cross-cutting Patterns → Test-time telemetry injection)

### Sub-block: the node's size, and the theme's check (F4, §6)
- Node size measured (source: P-090 · intent F4, ruled · replaces: obs-plan §10 "Buffer memory bounded" ≤ 512MB row, a row-count estimate that by its own text bounds no process memory)
- Theme 1 checked by the external harness (source: P-102 · intent §6 "after each, the external harness checks the result from the input to the output" · builds on: test-plan §1 pending trigger `mcp-incident-read-back-cross-process-coverage`)

## Epoch 4 — Memory on disk and a working door (P-091, P-092, P-093)

### Sub-block: memory on disk (F5)
- Telemetry store on disk (source: P-091, P-090 · intent F5, F4 · replaces: architecture §Established Decisions [Telemetry Retention Surface] "In-memory DuckDB ring buffer only (5–10 min)"; security-plan §Data Protection → At rest "DuckDB ring buffer: in-memory … encryption is N/A" and → Data lifecycle "Telemetry retention … No persistence"; builds on: architecture §Established Decisions [Database], [ORM / Migrations])
- Learned state survives a restart (source: P-091 · intent F5 · builds on: architecture §Occupied Resources → Corpus SQLite reserved tables; `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS`)

### Sub-block: the door (F6)
- The door inside the engine's process (source: P-092 · intent F6 · `.andromeda/residuals.md:31` · replaces: architecture §Established Decisions [MCP Server Surface] the stdio sidecar process and §Infrastructure Patterns → Deployment model "one optional `andromeda-pulse-mcp` sidecar process"; security-plan §API Security row "MCP feature double-gate"; builds on: architecture §Established Decisions [API Style — MCP Server Surface 3])
- Door queries (source: P-092 · intent F6 · builds on: architecture §Standard Contracts → MCP server, the telemetry tools; security-plan §Input Validation, prepared statements)
- One incident whole through the door (source: P-092 · intent F6 · `.andromeda/residuals.md:11` · builds on: architecture §Standard Contracts → MCP server, the incident and report tools; architecture §Established Decisions [Fault Identity])

### Sub-block: what the door may do, and the theme's check (F6b, §6)
- Door admission and bounds (source: P-093 · intent F6b, proposed for the founder's review · replaces: security-plan §Threat Model Summary vector "MCP stdio surface … No network exposure" trust boundary; builds on: security-plan §Error Handling, the JSON-RPC error object; architecture §Established Decisions [Corpus Write Arbitration])
- Theme 2 checked by the external harness (source: P-102 · intent §6)

## Epoch 5 — Keep what OpenTelemetry sends (P-094, P-095)

### Sub-block: spans (F7)
- Span keeps its identity (source: P-094 · intent F7 · `.andromeda/residuals.md:33` · builds on: architecture §Conventions → Database entity naming, Nullable patterns "nullable columns are reserved for OTLP-spec-optional fields (e.g., `parent_span_id`)")
- Span and resource attributes kept (source: P-094, P-088 · intent F7, F2b last clause · builds on: security-plan §Security Anti-Patterns → Logging, scrubber coverage of every column a producer can reach)
- Who calls whom (source: P-094 · intent F7 · `.andromeda/residuals.md:33`, the two aggregation queries that reference a parent column the table lacks)

### Sub-block: metrics and logs, and the theme's check (F8, §6)
- Histogram metrics keep their shape (source: P-095 · intent F8 · `.andromeda/residuals.md:35` · builds on: architecture §Conventions → Primary key convention for metric points)
- Logs and metrics name their service (source: P-095 · intent F8 · `.andromeda/residuals.md:35` · builds on: security-plan §Security Anti-Patterns → Logging, the `service_name` scrub at its choke point)
- Theme 3 checked by the external harness (source: P-102 · intent §6)

## Epoch 6 — Comparisons that work from the first minute (P-096, P-097, P-098)

- New error kind is a finding (source: P-096 · intent F9 · builds on: architecture §Established Decisions [Fault Identity], the L1 exception fingerprint)
- Release noticed (source: P-097 · intent F10 · `.andromeda/residuals.md:29` · builds on: the service version P-094 keeps)
- Release compared with the one before (source: P-097 · intent F10)
- Silence told apart (source: P-098 · intent F11 · builds on: the silence cue family of intent §3; obs-plan §3 Observability Harness Contract → Heartbeat ticks "liveness AND progress")
- Theme 4 checked by the external harness (source: P-102 · intent §6)

## Epoch 7 — The voice (P-099, P-100)

### Sub-block: one line and a notification (F12)
- Short report without a model (source: P-099 · intent F12 · `.andromeda/residuals.md:19` · builds on: architecture §Established Decisions [Snapshot Curation Default]; security-plan §Logging & Monitoring, no raw attribute value in a report beyond what was scrubbed)
- State in one line for a desktop panel (source: P-099 · intent F12, ruled: a panel module fed by an engine command, the founder's desktop only · replaces: architecture §Cross-cutting Patterns → Tray icon policy as the always-on state surface)
- System notification on a change of state (source: P-099 · intent F12 · replaces: architecture §Cross-cutting Patterns → OS notification policy; design-system §Surface: desktop-native → Notifications (OS-native))

### Sub-block: enough for a large model, and the theme's check (F13, §6)
- Report names its evidence (source: P-100 · intent F13)
- Large model names the planted cause (source: P-100 · intent F13 · replaces: the model-grading instruments P-084 retires)
- Theme 5 checked by the external harness (source: P-102 · intent §6)

## Epoch 8 — Polish & ship (P-101, P-102)

- Real service watched for days (source: P-101 · intent F14, ruled 2026-10-08; the founder names the service before this is taken up)
- Bad-version scenario end to end (source: P-102 · intent §7, ruled)
- Version close on Linux (source: P-083…P-103 · intent §7 "plus a check at every theme" · builds on: test-plan §10 Quality Gates & Coverage Targets)
