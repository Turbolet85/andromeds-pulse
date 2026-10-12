# 1A tree — andromeda-pulse-0.4.0 (second derivation)

_Phase 1A outline: epoch → sub-block → chunk candidate, each with its source. The first id on a line is the
requirement the chunk serves; "retires" / "replaces" / "builds on" names the master section the chunk touches, cited
only if `plans-not-opened.md` lists it as opened. Sub-blocks are dropped in 1C. What leaves with a retiring chunk is
listed under it — the 25-word route line cannot hold the whole list._

## Epoch 1 — Foundation

**Sub-block: base CI on Linux**
- CI on Linux alone (source: P-113, intent R7 "Windows and macOS legs in three CI jobs"; retires test-plan §9 pipeline
  table — the `lint-test` and `a11y` matrices' other legs and the whole `release` job, whose only legs are macOS and
  Windows — and its "Matrix builds" block; arch key `ci-cd-approach`). Fast feedback per synthesis-protocol
  §Placement: one run's wall-clock recorded.
- Supply-chain job same on push and pull request (source: P-120, intent F17; builds on security-plan §Dependency
  Security → CI integration and §Bootstrap phases `dep-security-ci-gate`. The plan names `actions-rust-lang/audit`;
  the workflow runs `rustsec/audit-check`, as read at `ci.yml:493-494`.) hypothesis: the witness needs a push event,
  and `ci.yml:3-6` runs on a push for `main` alone.
- Pre-push check native on Linux (source: P-103, intent F16; + P-113 for the `wsl.exe` hop; replaces arch §Occupied
  Resources "xtask CLI surfaces" `pre-push:linux` and test-plan key `per-chunk-gate-discipline` lines 5–18, the
  standard gate set whose webview and bindings lines leave in Epoch 2). Leaves with it: the distro clone, the
  cross-project Node dependency arch records there.

**Sub-block: the record removals write into**
- Capability record re-based (source: P-117, intent R11; replaces test-plan §9 "Capability verification matrix" — the
  gate over the 60-id file — and arch §Existing Scopes `pulse-v0_2_0-route`). Placed before the first removal: that
  gate fails on a dangling file reference, so a removal needs a record to write "retired" into (measured: the spec
  and the old file hold P-001…P-060; P-061…P-082 stand in the 0.3.0 ledger).

**Sub-block: a console engine that CI can run**
- Corpus encryption at rest retired (source: P-085, intent R3; retires security-plan §Data Protection → At rest
  "Persistent incident corpus" and "Corpus-key lock file", §Secret Management → Runtime, §Threat Model Summary data
  classification "persistent incident corpus", §Input Validation's `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` and
  `XDG_RUNTIME_DIR` rows, §Security Anti-Patterns → Input carve-out "corpus-key lock dir"; arch §Stack "Secret / key
  storage", §Occupied Resources "At-rest posture" and the lock-file location; test-plan §4 "corpus crate"; obs-plan §8
  corpus key-custody leaves). Leaves with it: `keyring` and `aes-gcm` dependencies and the `keyring`-internal
  duplicate carve-out (security-plan §Dependency Security), the keychain and encryption modules, the passphrase
  variable, the lock file, the orphan disposition. Kept: the scrubber at ingest. The security master records the
  decision, its date and the review.
- Console engine entry point (source: P-086, intent F1; replaces arch §Stack "Desktop shell" and §Standard Contracts
  `health` / `ready` as the liveness surface; builds on obs-plan key `tracing-init` — log sink, panic hook, the
  process-end record — and key `heartbeat-ticks`).
- One place on a node (source: P-118, intent F4b; + P-113 for the per-OS roots; replaces arch §Occupied Resources
  "Filesystem locations" and obs-plan key `log-file-location`; builds on security-plan §Security Anti-Patterns →
  Input, the canonicalize-and-confine rule for path variables).
- Agent harness drives the console engine (source: P-086; builds on test-plan §3 keys `5-command-implementation`,
  `pid-file`, `status-endpoint-shape`, `bootstrap-phases` items 2–5; obs-plan §10 CI gates — zero-panic, heartbeat
  gap, perf budget).
- Engine end-to-end gate reachable (source: P-086, P-090; [GATE_REACHABILITY] for the gates that grow with the code —
  the engine as a service under a memory cap, a sender on a second host; the clean-node service steps of P-107 land
  on the same pipe later; replaces test-plan §9 "Boot smoke (harness)" as the built-product gate).
- Shared telemetry test data (source: P-089, P-094; builds on test-plan §7 and key `bootstrap-phases` item 6 — three
  factories specified and unimplemented, test-plan §1 pending trigger `test-data-bootstrap-factories-unimplemented`).
- Detection baseline through the console engine (source: P-086 "everything 0.3.0 detected is still detected";
  builds on test-plan §1 Critical paths and §6 scenarios — P1 and P3 re-driven headless; P2 waits for the door; P4,
  P5, P6, P7 named for retirement by P-104, P-083, P-109, P-105).

## Epoch 2 — What leaves: the window, its bridge, its distribution and the other platforms

**Sub-block: the window**
- Window's gates retired (source: P-083, intent R1; retires test-plan §9 rows "A11y suite", "Boot smoke (harness)",
  "Staged artifacts", "E2E tests"; test-plan §6 driver row "desktop-webview" — the 17-stage headful drive; test-plan
  §4 "Framework (webview unit tests)"; test-plan §1 surfaces "desktop-webview", "desktop-native", "TauRPC IPC
  procedures", "Real-time IPC Channels"; security-plan §API Security "TauRPC capability authorization" — the
  capability-drift, widening and staged gates; arch §Occupied Resources "xtask CLI surfaces" `smoke:hue-shift`,
  `smoke:discovery`, `perf:frame-sample`, `check:staged-artifacts`; obs-plan §10 frame row; a11y-plan §1 and key
  `bootstrap-phases-derive-for-route-setup-project`). Leaves: the `a11y` and `boot` CI jobs, the webview suite, the
  `xtask` webview drive, self-verify, smoke, staged gate, hue shift, discovery, frame sampling, bundle format. The
  engine gate of Epoch 1 stands in their place.
- Window retired (source: P-083; retires arch §Stack rows "Desktop shell", "Visualization surface", "Tauri IPC
  bridge", "OS notifications", "Updater", "GUI verification harness"; arch §Established Decisions [Platform],
  [WebGPU Visualization Surface], [Tauri IPC Bridge]; arch §Standard Contracts IPC envelopes and real-time push;
  arch §Occupied Resources "Tauri IPC routes", "Tauri capability identifiers", `window-geometry.json`,
  `__NV_DISABLE_EXPLICIT_SYNC`; arch §Cross-cutting tray, notification and webview-IPC policies; security-plan
  §Threat Model Summary vectors "IPC", "in-app updater", "OS notification / tray", "webview content", §API Security
  CSP and updater rows, §Error Handling "TauRPC bridge"; obs-plan §1 surfaces desktop-webview and desktop-native,
  key `logging-stack` "Frontend bridge"; design-system §Brand Identity; layout-templates preamble; a11y-plan §1).
  The three interface masters state that this version has no interface and which later version brings a minimal one.
  The 0.3.0 capabilities about the window are recorded retired in the capability record, not regressed.
- Desktop distribution retired (source: P-114, intent R8; retires arch §Stack "Release pipeline", "Code signing",
  "Distribution channels", "Updater"; arch §Established Decisions [Deployment / Release Pipeline], [Code Signing],
  [Distribution Channels]; arch §Occupied Resources "Updater channel", "Bundle artifact names"; arch key
  `ci-cd-approach` `release.yml` and `update-channels.yml`; security-plan §Data Protection "Code-signing key
  custody", §Secret Management → Storage "Production (release pipeline)" and rotation, §Bootstrap phases
  `secret-management-init`, §Dependency Security "npm channel"; test-plan §9 "Supply chain" npm step). Leaves: the
  bundle workflow, channel publishing, two runbooks, the channel-manifest test, the npm tree's bot watch and
  supply-chain gate. The two external channel repositories (`turbolet85/homebrew-andromeda-pulse`,
  `turbolet85/scoop-andromeda-pulse`, as named in `update-channels.yml`) are the founder's to retire by hand.
  hypothesis: what precedes this chunk is the engine's Linux build in CI, not the documented service of P-107.

**Sub-block: what served the window inside the engine**
- Bridge leaves the engine's crates (source: P-108, intent R1b; retires arch §Conventions "Workspace API style" and
  "Error response schema (Tauri IPC)", arch §Cross-cutting "Cross-bridge data shape"; security-plan §Input
  Validation "TauRPC bridge" row; test-plan §4 "ui-bridge crate"). The configuration schema becomes the engine's
  own; its interface keys (theme, widget position, always-on-top, snapshot preset) leave.
- Display-only computation retired (source: P-109, intent R1c; retires arch §Occupied Resources "Tauri IPC events"
  and the delegated-timing observables, arch §Established Decisions [In-Process Channel Architecture] fan-out to UI
  subscribers; test-plan §1 critical path P6 and §6 scenario P6; obs-plan §4 scenario P6, §8 delegated timing leaves).
  Kept: the service registry's liveness truth.

**Sub-block: three more ruled removals**
- Plugin host retired (source: P-104, intent R4; retires arch §Stack "Plugin runtime", §Established Decisions
  [Plugin Runtime] and [Plugin Distribution Channel], arch §Occupied Resources `plugins.*` and
  `ANDROMEDA_PULSE_PLUGIN_DIR`; security-plan §Threat Model Summary vector "plugin host", §API Security "Plugin host
  capability sandbox", §Input Validation "Plugin host inputs"; test-plan §1 critical path P4, §6 scenario P4, §9
  "Cranelift-only WASM enforcement"; obs-plan §4 scenario P4, `plugins.tick` in §10 CI gates).
- Training export retired (source: P-106, intent R6; retires arch §Occupied Resources `storage.export_for_training`
  and "Out-of-data-dir egress sink"; security-plan §Data Protection, the three product-written locations outside the
  data dir — with P-085 and P-084 none of the three remains).
- Other operating systems retired from the code (source: P-113, intent R7; retires test-plan §1 coverage trigger
  "multi-platform-compat", test-plan §1 pending triggers that name the `.ps1` twins; arch §Occupied Resources
  `agent-run.ps1` half and `ANDROMEDA_PULSE_MSEDGEDRIVER_PATH`; obs-plan §1 trigger "multi-platform-exporter-compat").
  Leaves: seven PowerShell scripts, five PowerShell spawns in `xtask`, Windows path handling, three link hints, the
  linker override, the library-test workaround.

## Epoch 3 — What leaves: the local model, what fed and paced it, and the records of the old product

**Sub-block: the incident before the model leaves**
- Incident is the engine's own record (source: P-112, intent R2d; + P-099 for the plain report; replaces arch
  §Established Decisions [Fault Identity] — the title, the framing, the model-authored fields — and arch §Occupied
  Resources `ANDROMEDA_PULSE_L4_DETERMINISTIC`; builds on arch §Established Decisions [Corpus Write Arbitration];
  security-plan §Security Anti-Patterns → Logging "incident-summary corpus write boundary"). What replaces the
  model's answer is reachable before the model leaves. A store written by 0.3.0 is not read.

**Sub-block: the model and what served it**
- Local model retired (source: P-084, intent R2; retires arch §Stack "AI/ML serving" and "L4 grammar conversion
  check", arch §Established Decisions [LLM Inference Runtime], arch §Occupied Resources the model variables and the
  per-spawn grammar temp file; security-plan §Input Validation "L4 inference argv prompt" and the three model path
  variables, §Security Anti-Patterns → Input "NARROWED EXCEPTION" and → Code Patterns argv bound, §Dependency
  Security "Vendored test-only channel"; test-plan §4 "interpretation crate" and the Python requirement, test-plan §1
  pending triggers `l4-decision-probe-arg-parse-unit-coverage` and `l4-latency-p99-ps1-run-coverage`; obs-plan §10
  "L4 constrained inference" row, §8 `interpretation.*` leaves). `experiments/` is a git-ignored local directory
  (measured): no commit shows it gone.
- Digest retired (source: P-110, intent R2b; retires arch §Occupied Resources `digest_archive`,
  `pulse://stream/digests`, `ANDROMEDA_LLAMA3_TOKENIZER_PATH`; security-plan §Dependency Security carve-out (b)
  `ureq` — the build-time tokenizer fetch; arch §Established Decisions [Fault Identity] "canary-history readings and
  the corpus-selection remedy"). The similar-incident search is re-homed beside incidents; residual lines `:23`
  (its stale comment corrected there) and `:27` (the rate defect, closed by removal).
- Cadence coordinator retired (source: P-111, intent R2c; retires arch §Occupied Resources
  `pulse://stream/cadence-events`; obs-plan §5 "Cadence cycle rate", §8 `cadence.tick`; test-plan §10 Load profiles
  "high" — its assertion is worded over the seven queries at the coordinator's cadence). The emitter's handle to the
  cadence trigger channel leaves with it (measured: `crates/triage/src/cue/emitter.rs:399-402`).

**Sub-block: identity and the record of it**
- Workspace detection retired (source: P-105, intent R5; retires arch §Occupied Resources `workspace.detect`,
  `run/workspace-key`; security-plan §Threat Model Summary vector "filesystem reads", §Input Validation "Published
  workspace key"; test-plan §1 critical path P7, §6 scenario P7, pending trigger
  `workspace-key-cross-process-coverage`; obs-plan §4 scenario P7). The "two tokens, one incident" clause is proven
  in Epoch 4.
- Supply-chain gate re-based on the smaller graph (source: P-083, P-084, P-104, P-085 — intent §4's rule; builds on
  security-plan §Dependency Security "Duplicate-version carve-outs" and the advisory ignore discipline, §Bootstrap
  phases `dep-audit-tooling-install`; test-plan §9 "Cargo deny enforcement").
- Log allowlist describes this engine (source: P-115, intent R9; retires obs-plan §8 allowlist leaves for the window,
  the model, the digest, the tray and the plugins; builds on obs-plan §8 default-deny posture).
- Records say what the product is (source: P-116, intent R10; rewrites arch §Project Intent and §Design Philosophy,
  `.andromeda/input.md`, the product description, `project.yaml`, `CLAUDE.md`, the three webview-scoped rules; marks
  `docs/v0_2_0/` as history; arch §Existing Scopes). The always-loaded security rule's loopback line moves with the
  receiver in Epoch 4.

**Sub-block: the theme's check**
- Theme 0 checked (source: P-086 "everything 0.3.0 detected is still detected", P-102 and intent §6 "Order"; builds on
  test-plan §6 scenarios P1 and P3 and the baseline taken in Epoch 1).

## Epoch 4 — The engine stands on its own node

**Sub-block: the receiver and its token**
- Token lifecycle by engine command (source: P-088, intent F2b; builds on security-plan §Secret Management "What
  counts as secret" and §Logging & Monitoring "What NEVER to log"; replaces security-plan §Authentication &
  Authorization "N/A (deliberate)").
- Network OTLP receiver behind the token (source: P-087, intent F2; + P-088 for channel termination and key custody;
  + P-116 for the rule; replaces arch §Occupied Resources "bound on `127.0.0.1` only", security-plan §Threat Model
  Summary "Auth model: none" and the loopback attack-surface vector, §Data Protection → In transit "TLS: N/A for
  inbound", §API Security "Host header allowlist", §Security Anti-Patterns → API "NEVER bind the OTLP receivers to
  `0.0.0.0`"; test-plan §1 "Loopback-only OTLP binding" and its coverage trigger; `.claude/rules/security.md`
  §Loopback-only network surfaces).
- Receiver refusals and per-sender bounds (source: P-088; + P-105 for "a token is not an incident key"; builds on
  security-plan §API Security "Rate limiting" and "Request size limit", §Input Validation OTLP rows, §Logging &
  Monitoring "What to log" — OTLP request rejections; obs-plan §8 default-deny allowlist for the refusal record).
  With it: refusals recorded without the token's value; the size and count invariants hold for a remote sender; what
  the scrubber removes is still removed before storage.

**Sub-block: time and size**
- Checks reason by event time (source: P-089, intent F3; replaces arch §Conventions "Database entity naming" — the
  cutoff by the system clock; test-plan §7 "Time-sensitive data"). The engine's own liveness stays on the process
  clock (obs-plan key `heartbeat-ticks`).
- Recorded stream replays to the same result (source: P-089; builds on test-plan §2 agent-runnable invariants —
  deterministic).
- Engine memory measured (source: P-090, intent F4; replaces obs-plan §10 "Buffer memory bounded" — a row-count
  estimate, "no gate bounds the app's RSS" — and obs-plan §5 `metric.buffer.memory_bytes`).

**Sub-block: delivery and the theme's check**
- Engine delivered to a node (source: P-107, intent F14; replaces arch key `deployment-model`; builds on
  security-plan §Supply chain integrity `cargo-auditable` and test-plan §9 "Supply chain", the auditable Linux
  release build).
- Theme 1 checked by the external harness (source: P-102, intent §6 "Order").

## Epoch 5 — Memory on disk and a working door

**Sub-block: the stores**
- Telemetry store on disk (source: P-091, intent F5; replaces arch §Established Decisions [Telemetry Retention
  Surface] and [Database] "in-memory ring buffer", security-plan §Data Protection → At rest "DuckDB ring buffer" and
  → Data lifecycle "Telemetry retention"; builds on security-plan §Security Anti-Patterns → Data Protection — the
  store's own encryption stays off — and → Logging, redaction at the write boundary as the control).
- Learned state survives a restart (source: P-091; builds on test-plan §4 "triage crate", the bootstrap window).
- Disk store measured under load (source: P-090, P-091; builds on test-plan §10 Load profiles and obs-plan §10
  "DuckDB Connection Isolation" and its drain-progress signal).

**Sub-block: the door**
- Door inside the engine's process (source: P-092, intent F6; + P-093 for admission and transit; replaces arch
  §Established Decisions [MCP Server Surface], arch key `deployment-model` "one optional sidecar process",
  security-plan §Threat Model Summary vector "MCP stdio surface", §API Security "MCP feature double-gate",
  §Security Anti-Patterns → Code Patterns sidecar double gate; test-plan §9 "MCP-feature tests (`mcp-test` job)",
  test-plan §1 critical path P3; obs-plan key `snapshot-paste-to-ai-integration`). Residual `:31` (the tools read an
  empty database).
- Door admission lifecycle and bounds (source: P-093, intent F6b; builds on security-plan §Input Validation "MCP stdio
  inputs" and "DuckDB query parameters", §Logging & Monitoring — tool response bodies never logged; obs-plan §10
  reference connection pattern for a new reader).
- Door telemetry queries (source: P-092; builds on security-plan §Security Anti-Patterns → Input, prepared
  statements; obs-plan §8 query anonymizer).
- One incident whole through the door (source: P-092; residual `:11`, evidence references empty in every mode; test-plan
  §1 pending trigger `mcp-incident-read-back-cross-process-coverage`).
- Curated snapshot through the door (source: P-092; builds on arch §Established Decisions [Snapshot Curation
  Default], test-plan §1 critical path P2 and pending trigger `snapshot-resolver-level-coverage`, obs-plan §10
  snapshot row).
- Theme 2 checked by the external harness (source: P-102).

## Epoch 6 — Keep what OpenTelemetry sends

- Span keeps its identity (source: P-094, intent F7; replaces arch §Conventions "Nullable patterns" — a parent column
  the table does not have; builds on security-plan §Security Anti-Patterns → Logging intended posture). Residual `:33`.
- Span and resource attributes kept (source: P-094; builds on security-plan §Security Anti-Patterns → Logging, the
  joined key-and-value scrub shape for pair-shaped data).
- Who calls whom (source: P-094; + P-119 for the two parent-keyed aggregations).
- Histogram metrics keep their shape (source: P-095, intent F8). Residual `:35`.
- Logs and metrics name their service (source: P-095; builds on arch §Conventions "Primary key convention").
- Theme 3 checked by the external harness (source: P-102).

## Epoch 7 — Comparisons that work from the first minute

- New error kind is a finding (source: P-096, intent F9; + P-119, high-severity logs; builds on arch §Established
  Decisions [Fault Identity] L1 fingerprint; incident identity stays per check and service).
- Release noticed (source: P-097, intent F10). Residual `:29`.
- Release compared with the one before (source: P-097; + P-119, the per-operation aggregation).
- Silence told apart (source: P-098, intent F11; builds on obs-plan key `heartbeat-ticks`, liveness against progress).
- Every aggregation has a consumer (source: P-119, intent F8b).
- Theme 4 checked by the external harness (source: P-102).

## Epoch 8 — The voice

- One report form without a model (source: P-099, intent F12; + P-112, the record carries every field the report
  states).
- State in one line for a desktop panel (source: P-099; replaces arch §Cross-cutting "Tray icon policy" as the
  always-on state surface).
- System notification on a change of state (source: P-099; replaces arch §Cross-cutting "OS notification policy").
- Report names its evidence (source: P-100, intent F13).
- Large model names the planted cause (source: P-100).
- Theme 5 checked by the external harness (source: P-102).

## Epoch 9 — Polish & ship

- Real service watched for days (source: P-101, intent F15; touches security-plan §Threat Model Summary "Compliance
  triggers" and §Compliance Controls only once the founder names the service and its personal data).
- Bad-version scenario end to end (source: P-102, intent §7 point 1).
- No survivor (source: P-121, intent §7 point 2).
- Version close on Linux (source: P-117 and every id; test-plan §10 quality gates).
