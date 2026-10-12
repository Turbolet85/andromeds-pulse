# 1A tree — andromeda-pulse-0.4.0 (third pass, from the intent's third assembly)

_Phase 1A outline: epoch → sub-block → chunk candidate, each with its source. The first id on a line is the
requirement the chunk serves; "retires" / "replaces" / "builds on" names the master section the chunk touches, cited
only if `plans-not-opened.md` lists it as opened. Sub-blocks are dropped in 1C. What leaves with a retiring chunk is
listed under it — the 25-word route line cannot hold the whole list. The second pass's tree is `second/1a-tree.md`;
every line here was re-derived against the third assembly, and a line that reads as it did there was re-checked, not
copied blind._

## Epoch 1 — Foundation

**Sub-block: base CI on Linux**
- CI on Linux alone (source: P-113, intent R7 "Windows and macOS legs in three CI jobs"; retires test-plan §9 pipeline
  table — the `lint-test` and `a11y` matrices' other legs and the whole `release` job — and its "Matrix builds"
  block; arch key `ci-cd-approach`). One run's wall-clock recorded (synthesis-protocol §Placement, fast feedback).
- Supply-chain job same on push and pull request (source: P-120, intent F17; builds on security-plan §Dependency
  Security → CI integration and §Bootstrap phases `dep-security-ci-gate`). The witness is a push-event run; how this
  branch gets one is the chunk's to read from the workflow (the operator's word, 2026-10-09).
- Pre-push check native on Linux (source: P-103, intent F16; + P-113 for the `wsl.exe` hop; replaces arch §Occupied
  Resources "xtask CLI surfaces" `pre-push:linux` and test-plan key `per-chunk-gate-discipline`, its `pre-push:linux`
  paragraph — six stages, of which `npm` leaves with the window).

**Sub-block: the record removals write into**
- Capability record re-based (source: P-117, intent R11; replaces test-plan §9 "Capability verification matrix" and
  arch §Existing Scopes `pulse-v0_2_0-route`). Before the first removal and before the door replaces the sidecar:
  that gate fails on a dangling file reference.

**Sub-block: a console engine with its door**
- Console engine entry point (source: P-086, intent F1; replaces arch §Stack "Desktop shell" and §Standard Contracts
  `health` / `ready` as the liveness surface; builds on obs-plan keys `tracing-init`, `service-identity`,
  `heartbeat-ticks`).
- Door inside the engine's process (source: P-092, intent F6 "replaces the stdio sidecar as soon as the console
  engine exists, ahead of the removals the sidecar is built on"; + P-093 "until the engine accepts connections from
  another host, the door answers on the engine's own host only"; replaces arch §Established Decisions [MCP Server
  Surface], arch key `deployment-model` "one optional sidecar process", security-plan §Threat Model Summary vector
  "MCP stdio surface", §API Security "MCP feature double-gate", §Security Anti-Patterns → Code Patterns sidecar
  double gate; test-plan §9 "MCP-feature tests (`mcp-test` job)", §1 critical path P3 and §6 scenario P3; obs-plan
  key `snapshot-paste-to-ai-integration`, §4 scenario P3). Leaves with it, as measured at `7f99c38`: the sidecar
  binary, its own in-memory database, its resolver copy (`andromeda-pulse-mcp.rs:28`), its credential-store open
  (`:66-67`), the workspace key file published only for it and its read (`:78`), the compile-and-run double gate,
  the `mcp-test` CI job, the anchor dependency. In the process the telemetry answers read the buffer the engine
  holds: residual `:31` (the tools read an empty database) closes here. THE ONLY entry that touches the sidecar.
- Corpus encryption at rest retired (source: P-085, intent R3; retires security-plan §Data Protection → At rest
  "Persistent incident corpus" and "Corpus-key lock file", §Secret Management → Runtime, §Input Validation's
  passphrase and `XDG_RUNTIME_DIR` rows, §Security Anti-Patterns → Input "corpus-key lock dir"; arch §Stack "Secret /
  key storage", §Occupied Resources "At-rest posture"; test-plan §4 "corpus crate"; obs-plan §8 corpus key-custody
  leaves). Changes the door's corpus read, never a sidecar. Kept: the scrubber. The security master records the
  decision, its date and the review.
- One place on a node (source: P-118, intent F4b, with "readable by the engine's owner alone"; + P-113 for the per-OS
  roots; replaces arch §Occupied Resources "Filesystem locations", obs-plan key `log-file-location`; builds on
  security-plan §Data Protection → At rest and §Logging & Monitoring → Access controls).

**Sub-block: the gates of the console engine**
- Agent harness drives the console engine (source: P-086; builds on test-plan §3 keys `5-command-implementation`,
  `pid-file`, `status-endpoint-shape`, `bootstrap-phases` items 2–5; test-plan §1 pending triggers
  `harness-cleanup-verdict-and-boot-spawn-shell-coverage`, `harness-log-family-resolution-coverage`,
  `exit-hook-main-composition-coverage`; obs-plan §10 CI gates).
- Shared telemetry test data (source: P-089, P-094; builds on test-plan §7 and key `bootstrap-phases` item 6).
- Engine end-to-end gate reachable (source: P-086, P-090; reads through the door per P-092; [GATE_REACHABILITY]; a
  loopback sender, because the engine takes no sender from another host before the token — security-plan §Security
  Anti-Patterns → API; the second host reaches a stub; replaces test-plan §9 "Boot smoke (harness)"; obs-plan §9
  Telemetry artifact handling for the kept log).
- Detection baseline through the console engine (source: P-086; read through the door; builds on test-plan §1
  Critical paths, §6 scenarios P1 and P3).

## Epoch 2 — What leaves: the window, its bridge, its distribution and the other platforms

- Window's gates retired (source: P-083, intent R1; retires test-plan §9 rows "A11y suite", "Boot smoke (harness)",
  "Staged artifacts", "E2E tests"; test-plan §6 driver row "desktop-webview"; test-plan §4 "Framework (webview unit
  tests)"; test-plan key `per-chunk-gate-discipline` — the standard gate set's capability-widening, staged-artifacts,
  capability-drift, bindings-regen and webview lines, and its scenario legs `smoke:hue-shift`, `smoke:discovery`,
  `perf:frame-sample`; security-plan §API Security "TauRPC capability authorization"; obs-plan §10 frame row;
  a11y-plan §1 and key `bootstrap-phases-derive-for-route-setup-project`). Leaves: the `a11y` and `boot` CI jobs, the
  webview suite, the `xtask` webview drive, self-verify, smoke, staged gate, hue shift, discovery, frame sampling,
  bundle format.
- Window retired (source: P-083; retires arch §Stack rows "Desktop shell", "Visualization surface", "Tauri IPC
  bridge", "OS notifications", "Updater", "GUI verification harness"; arch §Established Decisions [Platform],
  [WebGPU Visualization Surface], [Tauri IPC Bridge]; arch §Standard Contracts IPC envelopes and real-time push;
  arch §Occupied Resources "Tauri IPC routes", "Tauri capability identifiers", `window-geometry.json`,
  `__NV_DISABLE_EXPLICIT_SYNC`; arch §Cross-cutting tray, notification and webview-IPC policies; security-plan
  §Threat Model Summary vectors "IPC", "in-app updater", "OS notification / tray", "webview content", §API Security
  CSP and updater rows; obs-plan §1 surfaces desktop-webview and desktop-native, key `logging-stack` "Frontend
  bridge"; test-plan §1 critical path P5 and obs-plan §4 scenario P5; design-system §Brand Identity;
  layout-templates preamble; a11y-plan §1). Ten router files, the tray, window geometry, the updater and
  notification plugins leave. The three interface masters state no interface and which later version brings one.
- Desktop distribution retired (source: P-114, intent R8; retires arch §Stack "Release pipeline", "Code signing",
  "Distribution channels"; arch §Established Decisions [Deployment / Release Pipeline], [Code Signing],
  [Distribution Channels]; arch §Occupied Resources "Updater channel", "Bundle artifact names"; arch key
  `ci-cd-approach`; security-plan §Data Protection "Code-signing key custody", §Secret Management → Storage
  "Production (release pipeline)", §Bootstrap phases `secret-management-init`, §Dependency Security "npm channel").
  The two channel repositories are named on the card.
- Bridge leaves the engine's crates (source: P-108, intent R1b; retires arch §Conventions "Workspace API style",
  arch §Cross-cutting "Cross-bridge data shape"; security-plan §Input Validation "TauRPC bridge" row).
- Display-only computation retired (source: P-109, intent R1c; retires arch §Occupied Resources "Tauri IPC events";
  test-plan §1 critical path P6, §6 scenario P6; obs-plan §4 scenario P6, §8 delegated timing leaves; keeps obs-plan
  key `trace-context-propagation`, the receiver-entry half).
- Plugin host retired (source: P-104, intent R4; retires arch §Stack "Plugin runtime", §Established Decisions
  [Plugin Runtime]; security-plan §API Security "Plugin host capability sandbox"; test-plan §1 critical path P4, §9
  "Cranelift-only WASM enforcement"; obs-plan §4 scenario P4).
- Training export retired (source: P-106, intent R6; retires arch §Occupied Resources `storage.export_for_training`
  and "Out-of-data-dir egress sink").
- Other operating systems retired from the code (source: P-113, intent R7: seven PowerShell scripts, five spawns,
  Windows path handling, three link hints, the linker override, the library-test workaround; retires test-plan §1
  trigger "multi-platform-compat"; obs-plan §1 trigger "multi-platform-exporter-compat").

## Epoch 3 — What leaves: the local model, what fed and paced it, and the records of the old product

- Incident is the engine's own record (source: P-112, intent R2d; + P-099 for the plain report; replaces arch
  §Established Decisions [Fault Identity] title and framing clauses, arch §Occupied Resources
  `ANDROMEDA_PULSE_L4_DETERMINISTIC`; builds on arch §Established Decisions [Corpus Write Arbitration]). The door's
  incident and report answers change with it; the door stays.
- Local model retired (source: P-084, intent R2; retires arch §Stack "AI/ML serving", §Established Decisions [LLM
  Inference Runtime], the model variables; security-plan §Input Validation "L4 inference argv prompt", §Dependency
  Security "Vendored test-only channel"; test-plan §4 "interpretation crate", §1 pending triggers
  `l4-decision-probe-arg-parse-unit-coverage` and `l4-latency-p99-ps1-run-coverage`; obs-plan §10 "L4 constrained
  inference" row). The experiments directory is shown gone by a listing on the card; its deletion on the dev host is
  asked of the operator.
- Digest retired (source: P-110, intent R2b; retires arch §Occupied Resources `digest_archive`,
  `pulse://stream/digests`, `ANDROMEDA_LLAMA3_TOKENIZER_PATH`; security-plan §Dependency Security carve-out (b)).
  Residual lines `:23` and `:27`.
- Cadence coordinator retired (source: P-111, intent R2c, with "no handle to one in the emitter"; retires arch
  §Occupied Resources `pulse://stream/cadence-events`; obs-plan §5 "Cadence cycle rate", §8 `cadence.tick`;
  test-plan §10 Load profiles "high").
- Workspace detection retired (source: P-105, intent R5; retires arch §Occupied Resources `workspace.detect`;
  security-plan §Threat Model Summary vector "filesystem reads"; test-plan §1 critical path P7, §6 scenario P7;
  obs-plan §4 scenario P7). The published key file already left with the sidecar.
- Supply-chain gate re-based on the smaller graph (source: P-083, P-084, P-104 — intent §4's rule; builds on
  security-plan §Dependency Security "Duplicate-version carve-outs", §Bootstrap phases `dep-audit-tooling-install`).
- Log allowlist describes this engine (source: P-115, intent R9; retires obs-plan §8 leaves for the window, the
  model, the digest, the tray and the plugins).
- Records say what the product is (source: P-116, intent R10; rewrites arch §Project Intent and §Design Philosophy,
  `.andromeda/input.md`, the product description, `project.yaml`, `CLAUDE.md`, the three webview-scoped rules; marks
  `docs/v0_2_0/` as history).
- Detection parity after the removals (source: P-086; test-plan §1 Critical paths restated).
- Theme 0 checked by the external harness (source: P-102, intent §6 "through the door").

## Epoch 4 — The engine is reached over a network

- Security posture restated for a networked engine (source: P-116, intent R10 "no later than the chunk that opens the
  network receiver, and what follows is built to the restated tier"; restates security-plan §Threat Model Summary —
  tier, auth model, attack surface — and §Authentication & Authorization).
- Token lifecycle by engine command (source: P-088, intent F2b; + P-118 "any token material readable by the owner
  alone"; builds on security-plan §Secret Management "What counts as secret", §Logging & Monitoring "What NEVER to
  log").
- Network OTLP receiver behind the token (source: P-087, intent F2; + P-088 channel termination and key custody;
  + P-116 the rule; replaces arch §Occupied Resources "bound on `127.0.0.1` only", security-plan §Data Protection →
  In transit, §API Security "Host header allowlist", §Security Anti-Patterns → API; test-plan §1 "Loopback-only OTLP
  binding"; `.claude/rules/security.md` §Loopback-only network surfaces).
- Receiver refusals (source: P-088; builds on security-plan §Logging & Monitoring "What to log", obs-plan §11 Logs
  and §5 tick-aggregated counters).
- Per-sender bounds (source: P-088; + P-105 "the same storm sent under two tokens forms one incident"; builds on
  security-plan §API Security "Rate limiting", "Request size limit").
- Door reachable from another host (source: P-093, intent F6b "from the moment it can be reached from another host";
  + P-092; builds on security-plan §Secret Management for the credential's custody).
- Door admission lifecycle and bounds (source: P-093; builds on security-plan §Input Validation "DuckDB query
  parameters", §Logging & Monitoring — answer bodies never logged; obs-plan §10 reference connection pattern).

## Epoch 5 — The engine lives on a small node

- Checks reason by event time (source: P-089, intent F3; replaces arch §Conventions "Database entity naming" cutoff;
  test-plan §7 "Time-sensitive data").
- Future-stamped records bounded (source: P-089, intent F3's added clause).
- Recorded stream replays to the same result (source: P-089; test-plan §2 deterministic invariant).
- Engine memory measured (source: P-090, intent F4; replaces obs-plan §10 "Buffer memory bounded").
- Engine's own log bounded (source: P-090, intent F4's added clause; builds on obs-plan key `log-file-location`,
  daily rotation; security-plan §Logging & Monitoring "Log retention").
- Engine delivered to a node (source: P-107, intent F14 with its checksum; replaces arch key `deployment-model`;
  builds on security-plan §Supply chain integrity `cargo-auditable`).
- Theme 1 checked by the external harness (source: P-102; read from the other host through the door).

## Epoch 6 — Memory on disk and the door's full answers

- Telemetry store on disk (source: P-091, intent F5; replaces arch §Established Decisions [Telemetry Retention
  Surface], security-plan §Data Protection → At rest "DuckDB ring buffer"; keeps obs-plan key `heartbeat-ticks`
  liveness and progress signals).
- Learned state survives a restart (source: P-091).
- Load profiles re-based on the engine (source: P-090; test-plan §10 Load profiles).
- Disk store measured under load (source: P-090, P-091; obs-plan §10 "DuckDB Connection Isolation").
- Door telemetry queries (source: P-092; security-plan §Security Anti-Patterns → Input, prepared statements).
- One incident whole through the door (source: P-092; residual `:11`; test-plan §1 pending trigger
  `mcp-incident-read-back-cross-process-coverage`, whose subject becomes the door).
- Curated snapshot through the door (source: P-092; arch §Established Decisions [Snapshot Curation Default];
  test-plan §1 critical path P2; obs-plan §10 snapshot row).
- Theme 2 checked by the external harness (source: P-102).

## Epoch 7 — Keep what OpenTelemetry sends

- Span keeps its identity (source: P-094, intent F7; residual `:33`).
- Span and resource attributes kept (source: P-094; security-plan §Security Anti-Patterns → Logging, the joined
  key-and-value scrub shape).
- Who calls whom (source: P-094; + P-119).
- Histogram metrics keep their shape (source: P-095, intent F8; residual `:35`).
- Logs and metrics name their service (source: P-095).
- Theme 3 checked by the external harness (source: P-102).

## Epoch 8 — Comparisons that work from the first minute

- New error kind is a finding (source: P-096, intent F9; + P-119).
- Release noticed (source: P-097, intent F10; residual `:29`).
- Release compared with the one before (source: P-097; + P-119).
- Silence told apart (source: P-098, intent F11).
- Every aggregation has a consumer (source: P-119, intent F8b).
- Theme 4 checked by the external harness (source: P-102).

## Epoch 9 — The voice

- One report form without a model (source: P-099, intent F12; + P-112).
- State in one line for a desktop panel (source: P-099; replaces arch §Cross-cutting "Tray icon policy").
- System notification on a change of state (source: P-099; replaces arch §Cross-cutting "OS notification policy").
- Report names its evidence (source: P-100, intent F13).
- Large model names the planted cause (source: P-100; test-plan §2 deterministic invariant: a reading, not a gate).
- Theme 5 checked by the external harness (source: P-102).

## Epoch 10 — Polish & ship

- Named service's personal data settled (source: P-101, intent F15's added clause; touches security-plan §Threat
  Model Summary "Compliance triggers", §Compliance Controls and §Security Anti-Patterns → Logging catalog).
- Real service watched for days (source: P-101).
- Bad-version scenario end to end (source: P-102, intent §7 point 1).
- No survivor (source: P-121, intent §7 point 2).
- Version close on Linux (source: P-117 and every id; test-plan §10 quality gates).
