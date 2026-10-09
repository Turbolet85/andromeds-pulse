# architecture — archived amendment originals
Writer = wrap P7 only · read by NO loop skill · cold history, never cited for current truth; each run's originals under its own heading.

# Consolidated at the 2026-09-28T21-45-38Z-wrap 0-pending wrap — 61 re-worded · 0 pruned

### 2026-05-09 — Acknowledge `streams.*` namespace
**Section:** §Occupied Resources Tauri IPC routes.
**Added:** `streams.subscribe_spans` / `streams.subscribe_metrics` / `streams.subscribe_logs` (`pulse-app/src/streams.rs:16-18`, chunk #23).
**Why:** D3 stale-drift closure for chunk #23 TauRPC procedures.
**Marker:** `.andromeda/runs/2026-05-09T11-45-00-spec-amendment-legitimize-streams-namespace/amendment.md`

### 2026-05-09 — Acknowledge `telemetry.frontend.*` namespace
**Section:** §Occupied Resources Tauri IPC routes.
**Added:** `telemetry.frontend.record_frame_ms` (`crates/ui-bridge/src/telemetry.rs:99`, chunk #29).
**Why:** D3 capability-drift closure for chunk #29 frontend telemetry resolver.
**Marker:** `.andromeda/runs/2026-05-09T11-45-00-spec-amendment-legitimize-telemetry-namespace/amendment.md`

### 2026-05-11 — Acknowledge `pulse:clipboard` capability
**Section:** §Occupied Resources Tauri capability identifiers.
**Added:** `pulse:clipboard` (`pulse-app/capabilities/clipboard.json`, chunk #43 partial commit `6e2d398`).
**Why:** D3 capability-drift closure for chunk #43 clipboard-manager write-only capability (security plan §Anti-Patterns API row 6 — clipboard read excluded).
**Marker:** `.andromeda/runs/2026-05-11T00-15-00-spec-amendment-acknowledge-pulse-clipboard-capability/amendment.md`

### 2026-05-16 — Acknowledge `curation` crate
**Section:** §Occupied Resources Cargo workspace crate names.
**Added:** `curation` (`crates/curation/Cargo.toml`, chunk #58 Epoch 9 Foundation v0.2.0) — curation primitives extracted from snapshot.
**Marker:** `.andromeda/runs/2026-05-16T16-15-00-spec-amendment-acknowledge-curation-crate/amendment.md`

### 2026-05-16 — Acknowledge `connection.current_state` + `pulse://stream/connection-state`
**Section:** §Occupied Resources Tauri IPC routes + Tauri IPC events.
**Added:** `connection.current_state` (`pulse-app/src/connection_router.rs:46`) + `pulse://stream/connection-state` (`crates/ingest/src/connection.rs:25`), chunk #59 connection FSM.
**Marker:** `.andromeda/runs/2026-05-16T22-08-32-spec-amendment-acknowledge-connection-namespace/amendment.md`

### 2026-05-16 — Acknowledge `triage` crate + register `pulse-v0_2_0-route` scope
**Section:** §Occupied Resources Cargo workspace crate names + §Existing Scopes.
**Added:** `triage` (`crates/triage/Cargo.toml`, chunk #60 commit `589225f`) + `pulse-v0_2_0-route` (first registered scope, `docs/v0_2_0/pulse-v0_2_0-route.md`).
**Marker:** `.andromeda/runs/2026-05-16T23-39-39-spec-amendment-acknowledge-triage-crate-and-scope/amendment.md`

### 2026-05-17 — Acknowledge `pulse://stream/attention-cues`
**Section:** §Occupied Resources Tauri IPC events.
**Added:** `pulse://stream/attention-cues` (`crates/triage/src/cue/broadcast.rs:8`, chunk #62 commit `aeb4d7d`).
**Marker:** `.andromeda/runs/2026-05-17T10-34-52-spec-amendment-acknowledge-attention-cues-broadcast/amendment.md`

### 2026-05-17 — Acknowledge `pulse://stream/restart-events`
**Section:** §Occupied Resources Tauri IPC events.
**Added:** `pulse://stream/restart-events` (`crates/triage/src/pattern/broadcast.rs:8`, chunk #63 commit `61ca564`).
**Marker:** `.andromeda/runs/2026-05-17T14-15-00-spec-amendment-acknowledge-restart-events-broadcast/amendment.md`

### 2026-05-18 — Acknowledge `services.list_with_states` + `pulse://stream/service-lifecycle`
**Section:** §Occupied Resources Tauri IPC routes + Tauri IPC events.
**Added:** `services.list_with_states` (`pulse-app/src/services_router.rs:61`) + `pulse://stream/service-lifecycle` (`crates/triage/src/lifecycle/broadcast.rs:16`), chunk #67 service registry + lifecycle FSM.
**Marker:** `.andromeda/runs/2026-05-18T16-53-11-spec-amendment-acknowledge-services-namespace/amendment.md`

### 2026-05-18 — Acknowledge `corpus` + `security` crates + `storage.{inspect,path}` + `corpus/corpus.db`
**Section:** §Occupied Resources Cargo workspace crate names + Tauri IPC routes + Filesystem locations.
**Added:** `corpus` + `security` crates (chunk #68); `storage.inspect` / `storage.path` (`pulse-app/src/storage_router.rs:50`); `corpus/corpus.db` subpath (`pulse-app/src/main.rs:353`).
**Why:** D3 capability-drift closure for chunk #68 persistent incident corpus + PII scrubber + storage router.
**Marker:** `.andromeda/runs/2026-05-18T19-55-24-spec-amendment-acknowledge-chunk-68-corpus-additions/amendment.md`

### 2026-05-19 — Acknowledge `diagnostics.template_distribution`
**Section:** §Occupied Resources Tauri IPC routes.
**Added:** `diagnostics.template_distribution` (`pulse-app/src/diagnostics_router.rs:59`, chunk #69 Drain template-profiling).
**Marker:** `.andromeda/runs/2026-05-19T20-54-03-spec-amendment-acknowledge-diagnostics-namespace/amendment.md`

### 2026-05-21 — Acknowledge `log_templates` DuckDB table + Corpus SQLite schema sub-section
**Section:** §Occupied Resources DuckDB schema names + new Corpus SQLite schema names sub-section.
**Added:** `log_templates` DuckDB reserved table (`crates/buffer/src/schema.rs:18`, chunk #69 Phase B); new "Corpus SQLite database / schema names" sub-section listing `baseline_state` / `service_registry` / `pipeline_metrics` / `incidents` / `incident_events` / `digest_archive` (`crates/corpus/src/schema.rs:26-33`).
**Why:** chunk #74 v3 Phase 6 Consolidation amendment 1 of 3.
**Marker:** `.andromeda/runs/2026-05-21T12-08-11-spec-amendment-acknowledge-log-templates-and-corpus-schema/amendment.md`

### 2026-05-21 — Tag-defer forward-promise entries
**Section:** §Occupied Resources Tauri IPC routes + Tauri IPC events + Environment variables.
**Change:** tag-deferred (NOT hard-deleted — preserves audit trail; establishes the cleanup-amendment convention): `snapshot.list_recent` + `snapshot.copy_to_clipboard` + `workspace.list` + `pulse://stream/plugin-events` + `ANDROMEDA_PULSE_CONFIG_PATH` all tagged "(deferred — no runtime emitter/consumer as of chunk #74)".
**Why:** D3 forward-promise drift closure per audit Section 1.F + 2.I (chunk #74 amendment 2 of 3). Cross-document grep verified zero runtime references.
**Marker:** `.andromeda/runs/2026-05-21T12-13-44-spec-amendment-tag-defer-forward-promises/amendment.md`

### 2026-05-21 — Acknowledge harness-only env vars + `run/andromeda-pulse.pid` subpath
**Section:** §Occupied Resources Environment variables + Filesystem locations.
**Added:** `ANDROMEDA_PULSE_PIDFILE` / `ANDROMEDA_PULSE_LOGFILE` / `ANDROMEDA_PULSE_DATA_DIR_KEEP` (harness-only, NOT consumed by production binary; `scripts/agent-run.{sh,ps1}`); `run/andromeda-pulse.pid` subpath (production binary `pulse-app/src/main.rs:196`, consumed by harness for status/cleanup).
**Why:** chunk #74 v3 Phase 6 Consolidation amendment 3 of 3. Harness-only env vars excluded from §Anti-Pattern Input row 4 canonicalization requirement.
**Marker:** `.andromeda/runs/2026-05-21T12-15-47-spec-amendment-acknowledge-harness-env-and-pid-subpath/amendment.md`

### 2026-05-23 — Acknowledge `incidents.*` namespace + `pulse://stream/incidents`
**Section:** §Occupied Resources Tauri IPC routes + Tauri IPC events.
**Added:** `incidents.list_active` / `incidents.acknowledge` / `incidents.mark_resolved` (`pulse-app/src/incidents_router.rs:83-85`) + `pulse://stream/incidents` (`crates/triage/src/incident/broadcast.rs:17`), chunk #78.
**Marker:** `.andromeda/runs/2026-05-23T07-49-08-spec-amendment-acknowledge-incidents-namespace/amendment.md`

### 2026-05-23 — Acknowledge `pulse://stream/cadence-events`
**Section:** §Occupied Resources Tauri IPC events.
**Added:** `pulse://stream/cadence-events` (`crates/triage/src/cadence/broadcast.rs:9`, chunk #80 cadence coordinator L6-visibility topic).
**Marker:** `.andromeda/runs/2026-05-23T12-10-53-spec-amendment-acknowledge-cadence-events-broadcast/amendment.md`

### 2026-05-23 — Acknowledge `pulse://stream/digests`
**Section:** §Occupied Resources Tauri IPC events.
**Added:** `pulse://stream/digests` (`crates/triage/src/digest/broadcast.rs:15`, chunk #81 L3 digest assembler topic).
**Marker:** `.andromeda/runs/2026-05-23T21-46-00-spec-amendment-acknowledge-digests-broadcast/amendment.md`

### 2026-05-24 — Acknowledge `interpretation` crate + `model.current_profile` + `pulse://stream/model-status`
**Section:** §Occupied Resources Cargo workspace crate names + Tauri IPC routes + Tauri IPC events.
**Added:** `interpretation` crate (chunk #82); `model.current_profile` (`pulse-app/src/model_router.rs:48`); `pulse://stream/model-status` (`crates/interpretation/src/broadcast.rs:14`).
**Why:** D3 capability-drift closure for chunk #82 Hardware profile detection + model loading + tokenizer substrate (L4 LLM interpretation pipeline initiation).
**Marker:** `.andromeda/runs/2026-05-24T15-58-15-spec-amendment-acknowledge-chunk-82-additions/amendment.md`

### 2026-05-25 — Acknowledge `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` + `ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH`
**Section:** §Occupied Resources Environment variables.
**Added:** the two `_LLAMA_*_BIN_PATH` env vars resolving prebuilt `llama-cli.exe` per `HardwareProfileSource` tier routing (`pulse-app/src/llamacli_inference.rs`, chunk #84 L4 runtime swap mistralrs → llama.cpp subprocess D1). Canonicalized + `is_file()`-asserted before spawn.
**Marker:** `.andromeda/runs/2026-05-25T12-34-46-spec-amendment-acknowledge-chunk-84-llama-bin-paths/amendment.md`

### 2026-05-25 — Acknowledge `diagnostics.retry_interpretation`
**Section:** §Occupied Resources Tauri IPC routes.
**Added:** `diagnostics.retry_interpretation` (`pulse-app/src/diagnostics_router.rs:60-155`, chunk #86 JSON-parse-failure handling + backoff; L4 degraded-mode FSM manual-override; P-020 graceful degradation).
**Marker:** `.andromeda/runs/2026-05-25T18-49-17-spec-amendment-acknowledge-chunk-86-diagnostics-retry-interpretation/amendment.md`

### 2026-05-26 — Acknowledge `incidents.mark_all_read`
**Section:** §Occupied Resources Tauri IPC routes.
**Added:** `incidents.mark_all_read` (`pulse-app/src/incidents_router.rs:97`+`:292`, chunk #87 Findings counter + dropdown; P-028/P-029/P-030).
**Marker:** `.andromeda/runs/2026-05-26T16-25-48-spec-amendment-acknowledge-chunk-87-incidents-mark-all-read/amendment.md`

### 2026-05-27 — Acknowledge `incidents.get_report`
**Section:** §Occupied Resources Tauri IPC routes.
**Added:** `incidents.get_report` (`pulse-app/src/incidents_router.rs:173`+`:424`, chunk #88 Diagnostic Report generation; P-031 + P-035–P-038).
**Marker:** `.andromeda/runs/2026-05-27T17-09-46-spec-amendment-acknowledge-chunk-88-incidents-get-report/amendment.md`

### 2026-06-03 — Acknowledge `storage.export_for_training` + `~/Downloads` egress exception
**Section:** §Occupied Resources Tauri IPC routes + Filesystem locations.
**Added:** `storage.export_for_training` (`pulse-app/src/storage_router.rs:67`+`:157`, chunk #95) — anonymized JSONL incident corpus dump via `pulse-app/src/training_export.rs`; `<home>/Downloads/pulse-corpus-export-{ts}.jsonl` out-of-data-dir egress sink (the one documented exception to under-data-dir confinement; `..`-rejected + parent-validated + egress-PII-scrubbed). P-046 community-training export, no auto-submission.
**Marker:** `.andromeda/runs/2026-06-03T17-26-46-spec-amendment-acknowledge-chunk-95-export-for-training/amendment.md`

### 2026-06-04 — Acknowledge `config-watcher` crate + `config.{reload,status}` + `diagnostics.reevaluate_recent_window` + `pulse://stream/config-events`
**Section:** §Occupied Resources Cargo workspace crate names + Tauri IPC routes + Tauri IPC events.
**Added:** `config-watcher` crate (`notify` 8.x FS watcher on `<data_dir>/config.toml` + `tokio::sync::watch` fan-out, chunk #96); `config.reload` / `config.status` (`pulse-app/src/config_router.rs:70-73`); `diagnostics.reevaluate_recent_window` (`pulse-app/src/diagnostics_router.rs:104`+`:182`); `pulse://stream/config-events` (`crates/config-watcher/src/event.rs:13`, aggregate-only — no raw config values/paths).
**Why:** D3 capability-drift closure for chunk #96 Configuration hot reload + prospective threshold application (P-055 / P-056). Note: word-form narrative ("twelve library crates" → 14 / "fourteen workspace members" → 16) surfaced informationally; structural §Design Philosophy not modified.
**Marker:** `.andromeda/runs/2026-06-04T17-05-12-spec-amendment-acknowledge-chunk-96-config-hot-reload/amendment.md`

### 2026-06-05 — Acknowledge `diagnostics.snapshot` + `diagnostics.history`
**Section:** §Occupied Resources Tauri IPC routes.
**Added:** `diagnostics.snapshot` (`pulse-app/src/diagnostics_router.rs:193`+`:309`) + `diagnostics.history` (`:194`+`:383`), chunk #97 Settings → Diagnostics view (L6 self-observability, HYBRID-RENDER).
**Marker:** `.andromeda/runs/2026-06-05T17-56-29-spec-amendment-acknowledge-chunk-97-diagnostics-snapshot-history/amendment.md`

### 2026-05-25 — LLM inference runtime: mistralrs → llama.cpp subprocess (D1), session 144 validation
The v0.2.0 chunk #82/#83 Pre-D1 decision originally chose `mistralrs = "=0.8.0"` for its native first-class
JSON-constrained generation surface. Empirical validation in session 144 (2026-05-25) invalidated that choice on
the CPU code path: `mistralrs` CPU inference is broken on Windows MSVC hosts per upstream issue
[EricLBuehler/mistral.rs#1134](https://github.com/EricLBuehler/mistral.rs/issues/1134) ("Endless inferencing with
cpu", OPEN since 2025-02-12, spans 0.7.0 → 0.8.1+) — load completes, "Dummy run completed" logs, then sampling
worker threads deadlock in futex/sched_yield spin with zero token output. A cross-check using prebuilt `llama.cpp`
(build b9305) against the EXACT same GGUF on the EXACT same host (Ryzen 9 5950X / RTX 3090) generated tokens
cleanly: 28.2 tok/sec CPU, 231.2 tok/sec unconstrained CUDA (`-ngl 99`), 122.3 tok/sec under `--json-schema-file`
GBNF constraint with complete schema-conformant JSON in ~4.3s — versus 60+ minutes of mistralrs CPU saturation
producing zero tokens. The runtime was therefore swapped from in-process `mistralrs` to `llama.cpp` invoked as a
**subprocess** (D1 spawn-per-generation). Rejected alternatives, both still documented sibling-impl swap paths
through the unchanged `LlmInferenceRunner` trait: in-process `llama-cpp-2` bindings (Windows `libclang` + `cmake` +
MSVC build-toolchain cost — the spike confirmed `llama-cpp-2 = "=0.1.146"` exposes `json_schema_to_grammar()` and
fits the trait via `spawn_blocking`, but the build stack cost was not justified for v0.2.0); `candle` (the original
chunk #82 escape hatch). Two runaway generations during the spike (both from missing bounds) motivated the
defense-in-depth `-n {max_tokens}` + `-st` + outer wall-clock-timeout discipline now in the body. The chunk #82
`pub trait LlmInferenceRunner: Send + Sync` was specifically designed to anticipate this staged swap as the
bus-factor mitigation pattern — only the concrete impl swapped (`MistralRsInference` → `LlamaCliInference`).

## 2026-06-28-deterministic-env-gated-l4-mode — Register ANDROMEDA_PULSE_L4_DETERMINISTIC env var
**Section:** §Occupied Resources — Environment variables (reserved at arch level)
**Change:** Added `ANDROMEDA_PULSE_L4_DETERMINISTIC` (truthy gate, default false) selecting the deterministic L4 runner (canned `L4Output`, no GPU/model) at pulse-app boot, alongside the existing `_LLAMA_{CUDA,CPU}_BIN_PATH` L4 env vars.
**Why:** Chunk 2026-06-28-deterministic-env-gated-l4-mode (P-073) introduced the env-gated deterministic L4 mode; arch §Occupied Resources must register every new env var (D-arch-resources). The runner is a third impl behind the unchanged `LlmInferenceRunner` trait — §Established Decisions [LLM Inference Runtime] unchanged.

## 2026-06-28-investigate-actions-functional — Register investigate.run_action TauRPC procedure
**Section:** §Occupied Resources — Tauri IPC routes (TauRPC procedures)
**Change:** Added `investigate.run_action` (pulse-app `InvestigateApiImpl`) — runs a real `LlmInferenceRunner::generate_constrained` analysis of the curated telemetry for one of the 4 bounded Investigate actions, reusing the incident `L4Output` schema, and returns a TRANSIENT scrubbed `InvestigateResultDto`; no incident created/persisted, no broadcast; deterministic-L4-aware (P-073).
**Why:** Chunk 2026-06-28-investigate-actions-functional (P-072, intent F12) wired the 4 dead Investigate buttons to a real L4-backed analysis; arch §Occupied Resources must register every new IPC procedure (D-arch-resources). The procedure is an additive second consumer of the unchanged `generate_constrained` trait method — §Established Decisions [LLM Inference Runtime] + §Stack unchanged (no new crate, no new dep).

## 2026-06-29-window-geometry-movable-shell — Register window-geometry.json filesystem location
**Section:** §Occupied Resources — Filesystem locations (subpaths under the resolved data dir root)
**Change:** Added `window-geometry.json` (remembered per-window positions — Rust-owned JSON map of window label → integer x/y; atomic `.tmp`+rename; written by `pulse-app/src/window_geometry.rs` on window-move, restored at boot).
**Why:** Chunk 2026-06-29-window-geometry-movable-shell (P-061, intent F1) added a new persisted data-dir file for remembered window position. New occupied filesystem resources are registered in §Occupied Resources §Filesystem locations (precedent: `corpus/corpus.db` chunk #68, the Downloads export sink chunk #95). The D-arch-resources detector's literal scope is IPC/port/env-var/crate (it did not flag the filesystem file); registered as orchestrator-judged registry completeness. Candidate detector-growth: extend D-arch-resources to cover §Filesystem-locations additions if this recurs.

## 2026-07-07-plain-language-connection-status — ready envelope: ReadyChecks +3 fields
**Section:** §Standard Contracts — `ready` command
**Change:** Added `rows_ingested` / `buffer_used_seconds` / `retention_seconds` (u64) to the `ready` envelope's `checks` object, alongside the existing 5 checks.
**Why:** Chunk 2026-07-07-plain-language-connection-status (P-070) exposes ingest rate + buffer fill on the EXISTING `ready` procedure (no new procedure/namespace/capability) to back the plain-language connection-status line; the §Standard Contracts example now matches the live `ReadyChecks` shape. D-arch-resources flagged the §Standard-Contracts envelope shape (fields on an existing method, not a new resource) — applied WITH the user as current-truth registry completeness.

## 2026-07-10-incidents-floating-window-disclosure — Cross-window UI-coordination events registered
**Section:** §Occupied Resources → Tauri IPC events (broadcast channels)
**Change:** Registered the chunk's new first-party webview↔webview coordination events `findings:dismissed` / `report:open` (`{incidentId}`) / `report:closed` (`@tauri-apps/api/event`, gated by `core:event` in `pulse:default`) as a DISTINCT class from the `pulse://stream/*` Rust→webview broadcast/telemetry channels.
**Why:** The chunk added a separate `findings` + `report` floating-window pair that coordinate dismiss / open-report / cross-window focus-restore via named frontend events; arch's event registry previously enumerated only the `pulse://stream/*` broadcast topics, so the new event contracts were unregistered (D-arch-resources). Capability grants (`allow-set-position`/`allow-set-size`) + window labels stay out of arch §Occupied Resources per the 2026-06-30 core-perm registry rule.

## 2026-08-14-fingerprint-feed-capture-repair — Register the harness UTF-8 relay env vars
**Section:** §Occupied Resources → Environment variables (reserved at arch level)
**Change:** Added `PYTHONUTF8` (`=1`) + `PYTHONIOENCODING` (`=utf-8`) as harness-only entries — SET (not read) by `scripts/agent-run.{sh,ps1}` and mirrored in the `.claude/settings.json` `env` block; the PowerShell half also sets `[Console]::OutputEncoding`/`InputEncoding`. Marked NOT consumed by the production binary, following the `ANDROMEDA_PULSE_PIDFILE` / `_LOGFILE` / `_DATA_DIR_KEEP` precedent.
**Why:** The chunk's commit lands the UTF-8 relay in both harness scripts and settings.json (report §Changes → Harness + log-format, §Schema/config). The registry is canonical for env vars the harness sets, and already carries harness-only entries; two new ones were unregistered.

## 2026-08-14-workspace-key-alignment — published workspace key + measured corpus key-custody reality
**Section:** §Occupied Resources → Filesystem locations; §Occupied Resources → Corpus SQLite database / schema names → At-rest posture
**Change:** (1) Registered `run/workspace-key` as a data-dir subpath — the resolved incident workspace key, published by the app at boot (atomic `.tmp`+rename, canonicalize-and-confine guard) and read cross-process by the `andromeda-pulse-mcp` sidecar; bounded ≤4096 bytes on read, consumed as an opaque filter string, never joined as a path. (2) Corrected the Corpus at-rest posture: OS-keychain key sourcing is INTENDED but UNIMPLEMENTED — `keyring 3.6.3` resolves with `[log, zeroize]` only, no platform credential-store backend linked, so the cell key is per-process ephemeral and historical encrypted content is orphaned until a backend fix plus migration lands. (3) Dropped the now-false "OS-keychain-encrypted" wording from the duplicate `corpus/corpus.db` subpath entry.
**Why:** (1) was the chunk's planned Expected amendment, following the 2026-06-29 `window-geometry.json` registration precedent. (2)+(3) record MEASURED truth per the chunk report's "Spec claims disproved by measurement" #1 — the registry asserted a posture that measurement disproved (first process 0 decryption failures; second process 13 `decryption_failed` warnings against its own predecessor's rows; `cmdkey /list` zero entries). Pre-existing defect, unmasked not caused by this chunk; the impl fix is owned by the new "Corpus key persistence" working-route entry.

## 2026-08-15-corpus-key-persistence — corpus key custody landed; two resources + a Stack row registered
**Section:** §Occupied Resources → Corpus SQLite → At-rest posture · §Occupied Resources → Filesystem locations · §Occupied Resources → Environment variables · §Stack and Technologies
**Change:** The At-rest posture no longer says key sourcing is INTENDED-but-UNIMPLEMENTED with a per-process
ephemeral key: `keyring` 3 now carries the explicit platform feature set, so the OS credential store is the
primary source and the key persists across processes; the opt-in `ANDROMEDA_PULSE_CORPUS_PASSPHRASE`
fallback and the boot-time inventory-then-purge disposition of pre-fix orphaned content are described, and
"Windows DPAPI" is corrected to "Windows Credential Manager". The duplicate "per-process ephemeral / no
OS-keychain backend linked" wording on the `corpus/corpus.db` subpath entry is retired with it. Registered
two new resources: the env var `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` (bounded parse, secret-class, never
logged) and the data-dir path `corpus/orphaned-inventory-{unix_nano}.md`. Added a §Stack "Secret / key
storage" row — the credential-store runtime had never been registered as a stack layer, and the feature set
is load-bearing because keyring 3.x has no `default` feature.
**Why:** The chunk measured the previous claim false and closed it — a second boot decrypted its
predecessor's rows with 0 `decryption_failed` against the 13 that defined the defect, with the Windows
entry `corpus-key.com.andromeda.pulse` present. This is the inverse of the 2026-08-14 APPLY-AS-MEASURED
amendment, which recorded the gap and named this chunk as its owner.

## 2026-08-16-baseline-family-reachability — register the baseline cold-start window override
**Section:** §Occupied Resources → Environment variables (reserved at arch level)
**Change:** Registered `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` — production-consumed (it moves the
running app's silence gate; NOT the harness-only class), bounded parse (non-zero, strictly below
`WINDOW_DURATION_SECONDS`, whitespace-trimmed, every rejection falling back to the 3600s default without a
panic), resolved once at boot and handed from `Thresholds::bootstrap_window_seconds` to
`BaselineState::set_bootstrap_window_seconds` so the silence evaluator, the emitter counters and the
lifecycle registry share ONE bound. Env layer only — no `Settings` / `config.toml` surface, so no TauRPC
contract and no hot-reload delta.
**Why:** The chunk landed a new production-consumed env var and the registry did not list it (D-arch-resources).
Registration is routine per the playbook's 2026-06-28 new-env-var rule: the report shows the input
code-validated and unit-tested (7 resolver tests), so it is registry completeness rather than an unvalidated
boundary. The env-layer-only scoping is recorded deliberately — the alternative (a `Settings` field) was
rejected at /phase because it trips the boot-smoke trigger and regenerates the TauRPC bindings.

## 2026-08-16-fault-identity-semantics-decided — Fault identity decided at both layers
**Section:** §Established Decisions
**Change:** New entry **[Fault Identity — what makes two faults ONE fault]** recording both decided semantics: L1 fingerprint normalization is TOKEN-LEADING (an absolute marker must START a token; relative path structure stays identity-significant, so `src/a.rs` and `src/b/c.rs` are distinct fingerprints), and L2 incident identity is COALESCE-PER-CUE-IDENTITY on `(kind, scope, scope_id)`. The entry also carries both rejected alternatives, the deferred `AttentionCue` threading path, and the known `Incident.fingerprint` producer defect that leaves the corpus-retrieval `fingerprint_match` arm correct-per-contract but starved.
**Why:** Operator-decided at the chunk's /andromeda-phase P4 review, both recommendations accepted. Before this amendment arch carried NO fault-identity, fingerprint or incident-coalescing entry at all (verified: zero grep hits), so a later chunk reading arch alone could legitimately have re-keyed coalescing on `L4Output.fingerprint` or re-greedied the normalizer — both of which this chunk measured as wrong. **Decision-history:** L1's docs and its pre-existing test had encoded absolute-only all along; only the implementation was greedy (`is_absolute_path_start` fired on any `/`, and because `is_path_char` admits `/`, the skip swallowed the rest of the token). **Switching cost recorded:** the external Conductor verification harness had just aligned to the greedy behaviour — its P-017 spec clause, its `FingerprintVariant` tests and `scenarios/fingerprint-storm.toml` — and re-aligns a third time, executed in that repo; Conductor is read-only from here and is cited, never copied. Purely additive: no existing arch wording was retired, so no duplicate-occurrence amendment follows.

## 2026-08-17-incident-fingerprint-producer-repaired — [Fault Identity]: deferral SHIPPED, producer repaired, retrieval arm FED

**Section:** §Established Decisions → [Fault Identity — what makes two faults ONE fault]
**Change:** Four clauses of the entry restated to shipped reality, all within its single bullet (verified: `fingerprint` occurs on exactly one line of this document, so there is no second site restating the retired wording).
1. The **DEFERRED** `AttentionCue` → `DigestCueRef` → producer threading is now **SHIPPED 2026-08-17** — both carriers named (`Option<String>`, `#[serde(default)]`, full 32-char lowercase hex; `None` for baseline-derived families).
2. The **"Known producer defect, owned elsewhere"** paragraph becomes **"Producer repaired"**: `Incident.fingerprint` is produced from the threaded L1 fingerprint via the crate-internal `triage::contract::hex_lower` — the same encoder the assembler applies to Q3 rows, explicitly NOT the 4-byte `fingerprint_to_hex_prefix`. The corpus-retrieval `fingerprint_match` arm is **FED**, not STARVED; its do-not-simplify guard now rests on the arm being live.
3. The **L1 blast-radius** sentence no longer says "confined to storm grouping": the value now flows onward into `Incident.fingerprint` (persisted, read by the retrieval arm and assembler). No viz / MCP / UI consumer exists — the field reaches no rendered surface — and `span_events.fingerprint` is still never SELECTed.
4. The **L2 coalesce-key rationale** is restated on the ground that survives. "Because the cue does not carry one" became false, so the key now rests on every downstream surface already being N-safe. Per-fingerprint dedupe is **possible-but-declined** and the decision REVISITABLE; the predicate itself is unchanged.

**Why:** chunk `2026-08-17-incident-fingerprint-producer-repaired` shipped the exact threading this entry had recorded as the deferred path, and fed the arm the entry described as starved. Report §Changes → Symbols/APIs lands both new fields; §Spec-master edits names both amendments as due; §Decisions & corrections records that the same two now-false L2 reasons were corrected in-code at `pulse-app/src/inference_runtime.rs` in the same chunk. Rejected alternatives (keying on `L4Output.fingerprint`) are untouched and still stand.

## 2026-08-17-conductor-e2e-verification-closure — deterministic-L4 fixture posture: populated arrays, and the two fields that stay empty
**Section:** §Occupied Resources → Environment variables → `ANDROMEDA_PULSE_L4_DETERMINISTIC`
**Change:** The entry now records that the canned `L4Output`'s arrays are POPULATED (three `evidence_refs`, two `hypotheses`, two `investigation_steps`) and why that is load-bearing — the value flows through the single producer join into `Incident.evidence_refs.fingerprint_hashes` and on to the MCP `retrieve_telemetry_slice.fingerprint_refs` response and the Report's Evidence section, so while the arrays were empty every payload-identity assertion there compared nothing and an absence check passed for the wrong reason. It also records the measured negative: `trace_id` / `span_ids` / `timestamps_unix_nano` are hardcoded empty at the same producer in EVERY mode, so `span_refs` and `timestamps_unix_nano` are permanently empty in production; and that `degraded_mode` is not a fixture field at all but `Report.degraded_mode = parsed_l4.is_none()` driven by Resolved-only persistence.
**Why:** Registry-completeness for a state the chunk genuinely changed (the report's Changes bullet records the `CANNED_L4_OUTPUT_JSON` value change), plus the two disproved premises the chunk measured — one of which (the always-empty sibling fields) was confirmed live by a real subprocess read rather than derived. Raised by the orchestrator at Validate check 5; no detector proposed it because the prior wording was incomplete rather than false.

## 2026-08-21-delegated-timing-observables — three delegated-timing IPC procedures + the capability-granularity correction
**Section:** §Occupied Resources → Tauri IPC routes; §Cross-cutting Patterns → Webview IPC capability policy
**Change:** Registered `telemetry.frontend.record_constellation_hue_latency` / `record_constellation_discovery_latency` / `record_findings_counter_refresh` with their input types, bounds and `metric.constellation.hue_update_ms` / `metric.constellation.discovery_ms` / `metric.findings.counter_refresh_ms` targets, noting P-025 measures the constellation DOT hue (the Halo canvas has no render site). Separately corrected the capability policy: Tauri capabilities stay negative-default, but the granularity is the IPC LAYER — TauRPC dispatches all methods through one invoke handler, so no per-procedure `pulse-app/capabilities/` entries exist; the per-procedure gate is the `EXPECTED_PROCEDURES` pin. Core APIs (`fs`/`shell`/`dialog`/`http`, each `core:window:*`) STILL require an explicit grant and ARE silently rejected when missing.
**Why:** The chunk landed three procedures with `pulse-app/capabilities/` untouched while `xtask capability-drift` stayed clean — measuring the per-procedure-entry requirement false. `.claude/rules/security.md` had flagged this exact tension on 2026-05-03; the arch body still asserted it.

## 2026-08-22-log-records-identity — log-record primary key restated; the `name` component it claimed never existed
**Section:** §Conventions → Primary key convention
**Change:** Split the conflated metric/log clause. Metric points keep OTLP-native identity, now stated by column (`metric_name` + `ts_unix_nano` + `resource_hash`). Log records are restated as `(ts_unix_nano, resource_hash, severity_number, seq)`, with `seq` a monotonic in-process ordinal allocated per batch by `BufferState::reserve_log_seq_block` and recorded as the ONE declared exception to the no-surrogate-keys rule — an internal disambiguating ordinal, never an observable. Added the rejection of a per-batch `span_events.event_index`-style ordinal and why it does not transfer, plus the measured note that the Arrow Appender enforces this key at `flush()` and a colliding pair failed the entire batch.
**Why:** The retired wording claimed log records key on "timestamp + resource hash + name", but `log_records` has **no `name` column at all** — that clause was inaccurate at HEAD independent of this chunk (measured, `crates/buffer/src/schema.rs:82-93`). The chunk then changed the key itself, so the body was doubly stale. Doc-only fix: the impl already embodies the new convention, so this is the 2026-08-14 routine-APPLY shape, not a deferred handoff. The `event_index` rejection is recorded because the arch extract's "follow the in-repo ordinal precedent" reads as endorsing it, and a literal application would ship a half-fix (cross-batch collisions survive) — the operator chose the monotonic column at the P4 dialogue for exactly that reason.

## 2026-08-23-ingestion-scrub-coverage — three reserved DuckDB tables recorded as declared-but-never-written

**Section:** §Occupied Resources → DuckDB database / schema names → Reserved tables (primary) · §Conventions → Database entity naming (restating site)

**Change:** The reserved-table enumeration no longer reads as seven live write targets. `append_record_batch_to_table` is the only DuckDB write path in `crates/buffer` and fires for exactly FOUR — `spans`, `metrics_points`, `log_records`, `span_events`. **`resources`, `instrumentation_scopes` and `span_links` have no producer**: all three are declared in `crates/buffer/src/schema.rs` and swept by `crates/buffer/src/retention.rs`, so three of the seven retention DELETEs sweep permanently-empty tables. The §Conventions restatement, which listed the same seven names unqualified AND asserted the periodic `DELETE FROM <table>` cutoff task, was qualified in the same pass. Recorded consequences: none of the three can carry host data, so none can leak nor be scrubbed (security-plan §Logging records the same fact from the PII side, where only `instrumentation_scopes` was in frame because it is the only one of the three with a client-controlled text column), and a claim keyed on "the reserved tables" must not be read as "the written tables".

**Why:** chunk `2026-08-23-ingestion-scrub-coverage` measured `instrumentation_scopes` producer-less while establishing its own target set (the route entry and security-plan both said five columns; four are live), and the adjacent sweep found `resources` and `span_links` in the same state. Applied AS MEASURED per the 2026-08-15 playbook rule: the impl half — either a producer lands or the dead schema and its dead DELETEs are retired — is too large to ride this chunk and is owned by the working-route entry "Diagnostics un-muting + harness-truth sweep" (CARRY added at this wrap). This entry records the measurement, not the cleanup.

## 2026-08-23-metrics-points-identity — metric-point key widened to four columns; the `seq` exception now spans two tables
**Section:** §Conventions → Primary key convention
**Change:** Metric points are keyed `(metric_name, ts_unix_nano, resource_hash, seq)` rather than the three OTLP-native columns. The clause now states WHY for both tables — `metrics_points` stores no attributes column, so two points of one metric differing only by label set are identical on every native column, and the ingestion scrub makes two distinct credential-shaped names redact to one placeholder and collide there. `seq` remains the one declared no-surrogate-key exception but now spans BOTH `log_records` and `metrics_points`, allocated per table from separate buffer-global counters (`reserve_log_seq_block` / the new `reserve_metric_seq_block`), and the non-observability list gained `SELECT_METRICS` / `COUNT_METRICS` / the `metrics.*` IPC response. The rejected per-batch-ordinal rationale now covers both tables. Measured note extended to 2026-08-23 (`rows_ingested` 0 pre-fix → 5 post-fix).
**Why:** the chunk landed `metrics_points.seq BIGINT NOT NULL` and the 3→4 column PK in all three DDL representations plus the new public `BufferState::reserve_metric_seq_block` boundary; the pre-existing sentence stated the metric key as OTLP-native-only, which the chunk measured insufficient and changed. Verified single-site: `metrics_points` at architecture.md:84 and :203 carries no key wording, so no restating site survived.
**Not amended (verified):** §Occupied Resources → DuckDB reserved tables enumerates TABLE names, not columns, so the new `seq` column needs no entry there.

## 2026-08-23-metrics-points-labels — metrics_points gains a scrubbed non-key labels column
**Section:** §Conventions → Primary key convention
**Change:** Retired the present-tense premise that `metrics_points` stores **no attributes column at all**. The table now carries a scrubbed `labels VARCHAR NOT NULL DEFAULT ''` column (all three DDL representations), deliberately OUTSIDE the key — the PK stays exactly `(metric_name, ts_unix_nano, resource_hash, seq)`, so two data points of one metric differing only by label set remain identical on every KEY column and are separated by `seq` alone; labels carry dimension for read-back, never identity. The retired premise is preserved as an explicit past-tense parenthetical, because it is the reason the `seq` exception exists.
**Why:** Chunk `2026-08-23-metrics-points-labels` added the column; the section's claim was falsified while the `seq`-exception rationale it supports survives intact. **Not amended (verified):** §Occupied Resources → DuckDB reserved tables enumerates TABLE names, not columns — the same ruling the predecessor chunk recorded — so the new column needs no entry there. §Conventions → Database entity naming makes a table-level naming claim only, also unchanged. Single-site verified by grep: the retired wording appeared nowhere else in this master (its occurrences in `architecture-amendments.md` and `master-route.md` are append-only history and an immutable predecessor record).

## 2026-08-23-webview-self-verify — GUI verification harness registered (env var + Stack row)
**Section:** §Occupied Resources → Environment variables · §Stack and Technologies
**Change:** Registered `ANDROMEDA_PULSE_MSEDGEDRIVER_PATH` in the harness-only env-var class (NOT consumed by the production binary; whitespace-trimmed + `is_file()`-guarded; unset / not-a-file ⇒ clean SKIP printing the fetch recipe; deliberately NOT canonicalize-and-confined because the driver lives outside the data dir by design, cross-referencing the security-plan carve-out). Added a §Stack row for the dev-only GUI verification harness — `@crabnebula/tauri-driver` 2.x + `webdriverio` 9.x as `pulse-app/ui` devDependencies, win32 native driver via the napi optional dep, driven by `cargo xtask webview-drive`, no Rust dependency and no runtime/bundle impact.
**Why:** Chunk `2026-08-23-webview-self-verify` landed both. The env var joins the existing harness-only peers (`_PIDFILE` / `_LOGFILE` / `_DATA_DIR_KEEP`), so an unregistered sibling was registry drift (D-arch-resources). §Stack carried no GUI/e2e driver technology at all while the chunk added two npm devDeps plus a native driver binary (D-arch-decisions); §Stack already hosts dev tooling (Code quality, CI task runner), so the row belongs there. No workspace crate, no TauRPC procedure and no product-bound port changed — `:4444`/`:4445` are harness-owned, loopback and released at teardown, so no port reservation was implied.

## 2026-08-23-integration-ux-e2e-test — harness row re-scoped to the 7-stage path; xtask→node relay set registered
**Section:** §Stack and Technologies → GUI verification harness (dev-only) row · §Occupied Resources → Environment variables
**Change:** (1) The §Stack harness row's Role now describes the **assembled 7-stage path** (launch → traces-empty → traces-populate → storm-incident → investigate → empty-states → widget-close) driven by `cargo xtask webview-drive [--expect-absent <STAGE>] [--no-inject]` under the deterministic-L4 gate the driver sets into the *spawned app's* environment, asserting per stage against the obs log and/or the driver's DOM report, with `--no-inject --expect-absent <STAGE>` as the code-driven RED mutation arm and telemetry entering only over real OTLP via `crates/ingest/examples/inject_demo`. The technology cell is unchanged — no dependency was added or bumped. (2) Registered the harness-only **xtask→node relay set** `PULSE_BIN` · `PULSE_DATA_DIR` · `MSEDGEDRIVER_PATH` · `PULSE_INJECTOR` as one entry, none consumed by the production binary, with `PULSE_INJECTOR` omitted by `--no-inject`.
**Why:** Chunk `2026-08-23-integration-ux-e2e-test` turned the one-press leg into a staged path (report §Counts: "the headful leg 1 press → 7 stages"; §CLI surface gives `--expect-absent` a STAGE value and adds `--no-inject`), so the Role cell — arch's only statement of what the sanctioned harness does — contradicted the shipped shape (D-arch-decisions). The relay registration is D-arch-resources, but applied WIDER than proposed: the detector proposed `PULSE_INJECTOR` alone, and measurement showed all four relay names unregistered (0 occurrences each), so registering one of four would have left the registry lopsided and actively misleading. `PULSE_*` is a claimed prefix where a collision would be silent, so the set is registered together; the three pre-existing names are retroactive completeness from `2026-08-23-webview-self-verify` (doc-only fix, impl already correct — playbook 2026-08-14). Deliberately NOT proposed: `:4444` (harness-owned per the entry above), `ANDROMEDA_PULSE_L4_DETERMINISTIC` (a new consumer, not a new variable — already registered), and the `msedgedriver` relocation to `D:\dev\tools\edgedriver\` (a machine-local operator fact; arch already states the driver lives outside the repo).

## 2026-08-23-headful-leg-extension — GUI harness Role cell: assembled path 7 → 13 stages
**Section:** §Stack and Technologies → GUI verification harness (dev-only), Role cell
**Change:** the assembled path enumeration grew 7 → 13 (adds traces-scroll · connection-status · findings-window · report-window · dashboard-toggle · dashboard-close); each id is `--expect-absent`-eligible; CLI shape unchanged.
**Why:** the chunk extended `STAGES` in `xtask/src/webview_drive.rs` to 13 and the GREEN leg drove all 13 (report §Changes/§Outcome); this is the same cell both predecessor chunks re-scoped.

## 2026-08-24-headful-mechanics-probe-race-disposition — headful leg 13 → 15 stages
**Section:** §Stack and Technologies → "GUI verification harness (dev-only)" row, Role cell
**Change:** The assembled path is now **15 stages**, with the full chain re-enumerated: `native-menu-suppressed` inserted after `traces-scroll` and `signpost-repeat` after `widget-close`, which consequently STOPPED being terminal — `signpost-repeat` follows it deliberately, restoring the widget so a second real close press proves the P-063 signpost fires on every close. Growth lineage recorded as 7 → 13 → 15 with its chunk markers. The `--expect-absent`-eligible / CLI-shape-unchanged clause is preserved verbatim.
**Why:** The chunk landed two stages and DECLINED two others on a measured driver probe; arch §Stack is the only site in this document stating the stage count (verified — the other `13` hits are `tonic` 0.13, `cudart64_13.dll`, and a decryption count).

## 2026-08-25-demo-injector-formalized-api-surface-retire — injector CLI contract + two unregistered env vars
**Section:** §Stack and Technologies (GUI verification harness row) · §Occupied Resources → Environment variables (`PULSE_INJECTOR` entry; two new entries)
**Change:** (1) The §Stack harness row's injector clause now records `inject_demo`'s formalized CLI contract — arg-less default emits byte-identically to the prior finite ~600-batch 100%-error storm, `--sustained` runs unbounded at a moderate rate, `--minutes=N` bounds a run, `--error-pct=0..100` is range-checked, an unknown arg is rejected with exit 2, and sustained mode retries a transient export failure under a consecutive-failure ceiling. (2) The `PULSE_INJECTOR` entry now states that `xtask::webview_drive` passes the PATH only and threads NO arguments, so the headful leg always runs the arg-less default and the arg-less default's byte-identity is what keeps the leg's storm-stage budget stable. (3) Registered two env vars that appeared in no registry section: `ANDROMEDA_PULSE_MODEL_PATH` (product-consumed by `llamacli_inference.rs`; previously named only in the §Established Decisions [LLM Inference Runtime] prose as an open co-design item, while its two `_LLAMA_*_BIN_PATH` siblings were registered at chunk #84) and `ANDROMEDA_LLAMA3_TOKENIZER_PATH` (build-script only, `crates/triage/build.rs`; note the non-`ANDROMEDA_PULSE_` prefix).
**Why:** the chunk formalized the injector into a supported dev/test tool and exercised the model/binary env pair in its live real-L4 leg. Arch already registers the sibling harness CLI shape (`cargo xtask webview-drive [--expect-absent <STAGE>] [--no-inject]`) in full and has precedent for retroactive harness-only registration "for namespace completeness" (the `PULSE_*` relay set), so a formalized flag contract on an already-registered harness resource is an unregistered resource. The two env-var registrations are pre-existing registry gaps the chunk surfaced, not additions it made — the report's Dependencies/env bullet correctly reads "no env var added or changed". The confinement posture of the model/binary paths is recorded in security-plan §Security Anti-Patterns → Input, not here.

## 2026-08-26-l4-runtime-security-residuals — L4 path guard, opt-in confinement root, dev-vs-bundled binary names
**Section:** §Occupied Resources → Environment variables (new `ANDROMEDA_PULSE_L4_ALLOW_ROOT` entry; `ANDROMEDA_PULSE_MODEL_PATH` entry) · §Occupied Resources → Process / service identity
**Change:** (1) Registered `ANDROMEDA_PULSE_L4_ALLOW_ROOT` — the opt-in confinement root for the three product-consumed L4 path inputs, with its bounded trimmed parse and, load-bearingly, its FAIL-CLOSED disposition: set-but-unresolvable rejects every candidate rather than degrading silently to unconfined, because a typo must not disable the guard the operator asked for. Records why it is opt-in rather than data-dir-default (the GGUF and `llama-cli.exe` live outside the data dir by design, so a default would reject every shipped configuration), that it is env-layer only, and the two observables it drives. (2) The `ANDROMEDA_PULSE_MODEL_PATH` entry's closing clause — "Like them it is canonicalized but deliberately NOT confined under the data dir" — is RETIRED and replaced with the landed guard posture, stated for all three vars at once: always-on traversal-reject-before-canonicalize (canonicalization erases `..`, so a post-check can never see it) + 4096-byte bound + `canonicalize()` + regular-file assert, PLUS confinement when the root resolves; unconfined only when the root is unset, and then announced once per boot. (3) §Process / service identity's single "Binary name" line is SPLIT into the dev-build binary (`target/{profile}/pulse-app.exe`, governed by Cargo's `[[bin]] name`) and the bundled artifact (`andromeda-pulse.exe`, produced by the Tauri bundler), noting that `target/{profile}/andromeda-pulse` exists on no host and that the sidecar is the one legitimately `andromeda-pulse*`-named build output.
**Why:** the chunk landed the guard, so the registry's unconfined claim and its missing env var are both stale against shipped reality (report §Changes → Schema/config, §Symbols/APIs, and smoke legs A/B/C). The binary-name split was operator-directed at this wrap, and its stated basis was CORRECTED before landing here: the directive said the confusion "has now cost two consecutive plans (P-077's deviation 1)", but re-derivation at HEAD found the predecessor's `plan.md:117` names `./target/debug/pulse-app.exe` correctly and its deviation 1 concerns flat config — ONE measured instance (this chunk's plan), not two. The amendment therefore rests on the corrected basis: the registry entry INVITES the error by naming only the bundled form, which is worth fixing on its own merit rather than on a recurrence count that did not hold.

## 2026-08-26-interpretation-brief-completeness — degraded_mode mechanism replaced (creation + dedupe attach)
**Section:** §Occupied Resources → Environment variables → `ANDROMEDA_PULSE_L4_DETERMINISTIC`
**Change:** The trailing clause "driven by Resolved-only `resolution_summary_text` persistence, so an Active incident renders degraded in real-model mode too" is RETIRED. The entry now records the landed mechanism: `resolution_summary_text` is written AT CREATION (scrubbed `L4Output` JSON) and REFRESHED on every dedupe re-generation, so a LIVE (Active/Acknowledged) incident with a clean parse renders the full six-section brief (`degraded_mode: false`) in every mode; the degraded notice remains only for genuinely-unparsed/absent interpretation, and the resolution-summary generation (when it fires) is the final write. The producer-join sentence is restated: `Incident.evidence_refs.fingerprint_hashes` is now the order-preserving dedup UNION of the parsed refs and the triggering cue's real L1 fingerprint (parsed refs FIRST, so the three synthetic P-073 refs still lead and contains-pins still ride them). The sibling residual (`trace_id` / `span_ids` / `timestamps_unix_nano` hardcoded empty in every mode) stands unchanged.
**Why:** the chunk's fix made the old clause false (report §Spec claims disproved by measurement; §Changes → BEHAVIOR). Measured live: an Active incident renders `degraded_mode: false` cross-process, with the real model COPYING the citable cue fingerprint (`fingerprint_refs` == exactly the cited id). Research also re-verified `DigestKind::ResolutionSummary` has zero non-test constructors at HEAD, so the retired clause's "Resolved-with-L4" class was production-empty — subsumed here.

## 2026-08-26-interpretation-brief-completeness — [Fault Identity] L1 blast radius extended
**Section:** §Established Decisions → [Fault Identity] → L1 blast-radius clause
**Change:** "No viz / MCP / UI consumer exists (the field reaches no rendered surface …)" is RETIRED. The clause now records that the L1 fingerprint DOES reach a rendered surface — threaded as a citable evidence id into the L4 prompt (`citable_evidence_ids`, deduped order-stable from `digest.attention_cues[].fingerprint`) and, via the producer union, into the MCP `retrieve_telemetry_slice.fingerprint_refs` response and the Diagnostic Report's Evidence section — while the `Incident.fingerprint` FIELD itself still has no viz / MCP / UI consumer and `span_events.fingerprint` is still never SELECTed.
**Why:** the chunk's Deliverable B renders real fingerprints (report §Spec-master edits floor; §Symbols/APIs; the operator judgment leg read the copied fingerprint in the live brief). The prior wording was the exact staleness the 2026-08-17 producer-repair entry predicted this change would cause.

## 2026-08-27-report-window-copy-affordance — headful leg 15 → 16 stages
**Section:** §Stack and Technologies → GUI verification harness (dev-only) Role cell
**Change:** the assembled path reads **16-stage** and enumerates `report-copy` between `report-window` and `investigate`; the growth lineage gains `15 → 16 at 2026-08-27-report-window-copy-affordance`, describing the new stage as pressing the report's real Copy control and reading its settled `data-copy-state` plus the live-region text, DOM-only by construction like its `report-window` / `findings-window` neighbours. The trailing clauses (per-stage `--expect-absent` eligibility, CLI shape unchanged, injector contract, msedgedriver / napi notes) are untouched.
**Why:** the chunk appended one `STAGES` entry and the leg now reports 16/16 (report §Counts / qualifiers moved; §Symbols / APIs; §Outcome). arch §Stack is the only site in this document stating the stage count — re-confirmed by grep this wrap, matching the precedent recorded at the 2026-08-24 entry.

## 2026-08-27-incident-persist-vs-resolve-write-race — corpus write arbitration decided
**Section:** §Established Decisions (NEW entry, appended after [Fault Identity])
**Change:** added **[Corpus Write Arbitration — which of two writers to the same incident row wins] Monotonic last-writer guard at the `CorpusWriter` choke point** — a bound `AND updated_unix_nano <= ?2` predicate on the existing `incidents` UPDATE plus a bound existence probe classifying the zero-row result, so both `update_incident_status` signatures return `Result<IncidentWriteOutcome, _>` (`Applied` | `DeclinedStale`) and a stale write DECLINES rather than failing. Records why placement is the corpus layer (the only choke point all SEVEN production writers traverse — the seventh is the cross-process MCP sidecar, which bypasses the `IncidentPersistence` trait), both REJECTED alternatives (a status-only predicate, disproved because `attach_resolution_summary` requires `status == Resolved`; delta-persistence, which does not reach the cross-process path), why `triage::contract::IncidentWriteOutcome` is a parallel type rather than a re-export (DAG direction), and the accepted app-registry-staleness residual with its owner.
**Why:** the chunk landed a concurrency-control pattern §Stack / §Established Decisions did not cover. Both the phase arch extract and the wrap fan-out grep-verified independently that no write-arbitration / optimistic-locking / transaction-scope entry existed — so this is an addition, not a correction, and nothing in arch is retired by it (the nearby §Fault Identity "resolution-summary generation is the final write" clause is PRESERVED, and is precisely the reason the status-only predicate was rejected). Report §Decisions & corrections records it as the operator's P4 fork-1 decision; §Expected amendments named this section as owner.

## 2026-08-28-duplicate-span-replay-fails-loudly — injector `--replay`, budget-not-byte identity, duckdb pin
**Section:** §Stack and Technologies (GUI verification harness row · Storage engine row) · §Established Decisions [Database] · §Inherited Defaults (Database) · §Occupied Resources (Environment variables → `PULSE_INJECTOR`)
**Change:** Registered the injector's fourth flag `--replay` (pins the identity salt so a restart deliberately re-emits the prior run's `trace_id`/`span_id` — the retrying-exporter shape `smoke:gap-resume` arm A drives; flag set 3 → 4). Narrowed the arg-less-default claim from "byte-identically" to **budget-identical** at BOTH sites that stated it (§Stack row and the `PULSE_INJECTOR` entry): batch/span/error counts and durations are unchanged, but each run now draws a `fresh_run_salt()` into `trace_id`/`span_id`, so span identity is deliberately per-run and NOT byte-stable — and the clause's historical verification was a COUNT equality (3540 = 3540), never a byte comparison. Re-stated the duckdb qualifier at all THREE sites carrying `1.10500.x` as requirement-plus-resolved: `^1.10500` (Cargo.toml, untouched) resolving to 1.10505.0.
**Why:** the chunk salts span identity so a producer restart cannot collide with itself on the `spans` primary key, and fixed the consumer-wedge defect by bumping the duckdb lockfile — leaving three docs sites naming a version family the lockfile no longer resolves and two naming a byte-identity the injector no longer provides.

## 2026-08-29-app-registry-reconciliation — [Corpus Write Arbitration] Accepted residual CLOSED
**Section:** §Established Decisions → [Corpus Write Arbitration]
**Change:** The "Accepted residual" clause (the guard makes the corpus correct but not the app; after an external MCP resolve the running app shows the incident active until restart and its persist cycle declines that row every cycle) is replaced by a CLOSED record. The 60 s persist cycle now reconciles first via the new `triage::contract::DurableActiveIncidents::active_incident_ids` read port — implemented at the `pulse-app` boundary over the same `Arc<dyn CorpusWriter>`, ids only, no new crate edge — resolving registry rows absent from the durable active set through the existing `IncidentRegistry::mark_resolved`, before the write loop. A durable-read `Err` reconciles nothing and warns with `persist_kind = "incident_reconcile"`. The monotonic predicate, its existence probe, the `Applied`/`DeclinedStale` semantics and `load_active_incidents`'s boot-only caller are all unchanged; the reconciler deliberately emits no lifecycle event because `pulse://stream/incidents` has producers but no consumer.
**Why:** The residual named its own owner ("reconciliation is owned by its own working-route entry") and that entry is this chunk. Measured live: a real `andromeda-pulse-mcp` subprocess resolved a row and the running app dropped it within two persist cycles with no restart — `reconciled_count` 1 / `declined_count` 0, against 0 / 2 under a mutation with reconciliation removed.

## 2026-08-29-advisory-backlog — rmcp mechanism recorded as measured + wasmtime requirement-plus-resolved restate
**Section:** §Stack (MCP server row · Plugin runtime row) · §Established Decisions ([MCP Server Surface] · [API Style — MCP Server Surface 3] · [Plugin Runtime]) · §Inherited Defaults (API style — external MCP · Plugin runtime) · §Occupied Resources (MCP stdio surface line · sidecar-binary label) · §Infrastructure Patterns (directory-tree comment · deployment-topology line)
**Change:** The MCP surface's mechanism wording is amended to the measured reality (operator-resolved escalation, this wrap): the canonical surface is the hand-rolled serde JSON-RPC 2.0 layer at `crates/mcp-server/src/jsonrpc.rs` (MCP protocol `2024-11-05`, name-dispatch in `tools.rs`); `rmcp` is recorded as a feature-gated ANCHOR dependency — requirement `"3"`, lockfile-resolved 3.1.4 as of 2026-08-29, sole source contact `use rmcp as _;` (bin :20) — and the "rmcp 1.5.0 vs published 0.3.x" reconciliation caveat is CLOSED by measurement at both its sites. The 8-tool wire surface, the `--features mcp-server` gate, and the never-a-custom-envelope ban are unchanged (wire conformance proven by the 9/9 real-subprocess legs). wasmtime rows at all three sites take the requirement-plus-resolved shape on the duckdb precedent — family floor 25+ unchanged and still true, requirement `"46"`, lockfile-resolved 46.0.3 (Cranelift 0.133.3). "rmcp sidecar" naming leftovers renamed to the `andromeda-pulse-mcp` sidecar at three registry/topology sites.
**Why:** Chunk `2026-08-29-advisory-backlog` bumped rmcp 0.6.4 → 3.1.4 and wasmtime 43.0.2 → 46.0.3 to close RUSTSEC-2026-0189/0222, and its research measured the rmcp-macro mechanism this doc described as never implemented. The operator resolved the wording fork to amend-to-measured-reality.

## 2026-08-30-npm-advisory-coverage — xtask npm-supply-chain gate registered
**Section:** §Occupied Resources (new "xtask CLI surfaces (dev/CI gates)" entry before §Tauri capability identifiers)
**Change:** Registered `cargo xtask check:npm-supply-chain` (`xtask/src/npm_gate.rs`) with its formalized contract — exit 0 green · 1 findings/policy red · 2 cannot-evaluate; six named verdict arms; one pretty-JSON verdict object on stdout — its policy input `pulse-app/ui/npm-policy.json` (per-class license allowlists + GHSA-scoped advisory exceptions with mandatory provenance + package-name denylist), the lockfile-only design (no `npm ci`, no node_modules), and its ci.yml `supply-chain`-job wiring beside the existing `Cmd::Audit`/`Cmd::DenyBans` verbs.
**Why:** The chunk landed a new formalized xtask CLI contract, which is an arch registry item per the 2026-08-25 formalized-CLI-contract rule (D-arch-resources proposal; report §Symbols/APIs + §Counts/qualifiers moved (4)).

## 2026-08-30-diagnostics-un-muting-harness-truth-sweep — harness:status registered · dead schema dropped (DuckDB 8→5, corpus 6→5/v2) · 17-stage leg · incidents topic producer-only
**Section:** §Occupied Resources (xtask CLI surfaces · Filesystem `run/andromeda-pulse.pid` · Environment `ANDROMEDA_PULSE_PIDFILE`/`_LOGFILE` · DuckDB Reserved tables · Corpus Reserved tables · Tauri IPC events `pulse://stream/incidents`) · §Conventions (Database entity naming) · §Stack (GUI verification harness row)
**Change:** (1) Registered `cargo xtask harness:status` (`xtask/src/harness_status.rs`) with its verdict-JSON contract `{verdict, pid, log_file_basename, last_write_age_seconds, stale_after_seconds}`, four arms running-healthy/stale/not-running/cannot-evaluate at exits 0/1/1/2, derived out-of-process from the PID file + log-family mtime; it REPLACES the exit-0-unconditional in-process `current_health()` envelope. The pid-file entry and both harness env vars gain the xtask verb as a second reader. (2) DuckDB reserved tables 8→5 — the `span_links`/`resources`/`instrumentation_scopes` CREATEs deleted as dead schema, retention DELETEs 7→4, the 2026-08-23 CARRY disposition discharged; §Conventions Database-entity naming re-synced (producerless/permanently-empty-sweep wording retired). (3) Corpus reserved tables 6→5 — `baseline_state` dropped, `SCHEMA_VERSION` 2 with the ladder migration recorded (fresh→v2 · v1→DROP+stamp · newer→mismatch), and `incident_events` gains its first production writer (one lifecycle row per status value-change inside the guarded-update transaction). (4) §Stack harness row: 16 → 17 stages with `boot-geometry` after `launch` — the driver records outer position/size + monitor and the Rust verdict re-derives the app's own snap formula (`wx == mx + mw − round(480×scale) − 24 && wy == my + 24`). (5) `pulse://stream/incidents` tagged **producer-only** at the broadcast-channel registry (operator ruling 2026-08-30) — producers exist, no consumer subscribes, the webview polls `incidents.list_active`.
**Why:** The chunk's operator forks ruled DROP-both-dead-schema-sets, PERSIST-lifecycle-at-the-choke-point, and RECORD-producer-only; the harness:status contract is an arch registry item per the 2026-08-25 formalized-CLI-contract rule; the 17th stage and the 5-table sets are report §Counts-moved facts verified live this session (webview-drive 17/17; corpus user_version=2 with 5 tables; nextest 2239/2239).

## 2026-08-30-staged-bindings-assertion — staged-artifacts gate registered; capability-drift's behavior re-pointed
**Section:** §Occupied Resources → xtask CLI surfaces (dev/CI gates) · §Cross-cutting Patterns → Webview IPC capability policy
**Change:** (1) Registered `cargo xtask check:staged-artifacts` (xtask CLI surfaces 2 → 3; `xtask/src/staged_gate.rs`): asserts the git-INDEX copies of `pulse-app/ui/src/bindings/index.ts` + all `pulse-app/capabilities/*.json` (via a `git ls-files --stage` health probe then `git show :<path>` per subject) against `EXPECTED_PROCEDURES` and the in-code `staged_gate::EXPECTED_GRANTS` semantic-triple pin `{identifier, windows, permissions}` (6 files; `description`/`$schema`/`local` outside the pin; set equality BOTH directions; staged deletion + unpinned capability file findings-RED, no neutral arm); contract = one pretty-JSON verdict on stdout, arms `staged-clean`/`staged-drift`/`cannot-evaluate` at exit 0/1/2, report twin `target/staged-artifacts/report.json`, ci.yml plain named `run:` step directly after capability-drift; `capability-drift` itself runs the staged assertion after its unchanged worktree diff and folds any non-clean staged outcome into its existing FAILURE exit (0/1 contract preserved). (2) The §Webview IPC capability policy clause re-pointed: the `EXPECTED_PROCEDURES` pin is diffed against BOTH the worktree and the staged bindings, and the staged grants ride `EXPECTED_GRANTS` — a pin edit must be STAGED with the regenerated bindings.
**Why:** Report §Symbols/APIs ships the verb + the `capability_drift()` extension; §Counts moved states the registry 2 → 3; the second site (line 332) restated the worktree-only mechanism and would have survived a single-site apply (D-arch-resources primary + dependent). Live-proven: real repo `staged-clean` exit 0; clobbered-worktree run exit 1 with the staged half green.

## 2026-08-30-acl-rejection-logging — `telemetry.frontend.record_ipc_rejection` + `ui.ipc.rejection` leaf registered
**Section:** §Occupied Resources → Tauri IPC routes (TauRPC procedures)
**Change:** Registered `telemetry.frontend.record_ipc_rejection` (ui-bridge crate; the 5th `TelemetryApi` method — family roster 4 → 5): the capability-rejected-IPC record security-plan §Logging & Monitoring mandates. Bounded input triple (`IpcRejectionCategory` closed enum `acl_rejected`|`other` classified webview-side; `coerce_window_label` third bounded copy; `validate_payload_bytes` bound 100_000_000) → target `ui.ipc.rejection` (WARN, once per rejection, no hot path) with its OWN exact allowlist leaf — no bare `ui` key exists (measured); NO capability-JSON change; `EXPECTED_PROCEDURES` +1.
**Why:** Report §Symbols/APIs lands the procedure + target + leaf; §Counts moved records the roster 4 → 5 naming this registry entry; the Expected-amendments floor listed it. D-arch-resources; no duplicate roster site exists in arch.

## 2026-08-30-agent-harness-teardown-truth — agent-run CLI contract registered + pid-file entry clause narrowed
**Section:** §Occupied Resources → xtask CLI surfaces (dev/CI gates) · §Occupied Resources → Filesystem locations (`run/andromeda-pulse.pid` entry)
**Change:** (1) Registered `scripts/agent-run.{sh,ps1}` as a sibling formalized CLI contract: 5 unchanged verbs; `boot` pre-builds under its own exported env (app release + xtask, absorbing the env-fingerprint relink outside the timed window) and spawns the dev binary BY PATH (no `cargo run` wrapper; 10s default honest, measured 1.953s); `cleanup`'s four bounded verdict tokens with 0/1 exits from independent pid+port probes; the ci.yml Linux-only smoke step now GATING (`continue-on-error` dropped). (2) The pid-file entry's absolute "the scripts reach status only THROUGH that xtask verb" narrowed to the status verdict (boot poll + `status` verb) — `cleanup` judges liveness independently, with pidfile CONTENT canonical (the app's `write_pid_file` overwrites the provisional spawn pid; measured 59828 vs `$!` 128125, msys ≠ Windows pid space).
**Why:** Report §Symbols/APIs (the new verdict/exit contract + wrapper removal) + §Schema/config (CI gating, operator-approved at P4) + §Decisions (the harness:status-precedent verdict shape; the identity insight). D-arch-resources primary + dependent, applied atomically; routine per the 2026-08-25 formalized-CLI-contract rule + the 2026-07-08 accurate-addition rule.

## Registry migration (U35) — 2026-10-06

<!-- U35 · architecture.md · ## Infrastructure Patterns · sha256 5e7eceb1b1f245473d762649ff8911dd572359449e3e8afe66fe3bf3bf96fedd -->

## Infrastructure Patterns

**Build system.**
- Cargo workspace; package manager is `cargo`.
- Lint: `cargo fmt --check` + `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
- Typecheck: implicit in `cargo check --workspace --all-targets`; webview TypeScript bindings emitted by TauRPC are typechecked with `tsc --noEmit`.
- Build: `cargo tauri build` (invoked by `tauri-action`); release profile is workspace default with `lto = "thin"`, `codegen-units = 1`, `strip = true` for distribution bundles.

**Deployment model.**
- Public OSS desktop app distributed through GitHub Releases; "deployment" is the release pipeline, not server hosting.
- No Docker, no Kubernetes, no serverless, no docker-compose.
- Runtime topology on the user's machine: one Tauri process hosting all fourteen library crates and the embedded webview; one optional `andromeda-pulse-mcp` sidecar process (only when `--features mcp-server` is enabled at build time and `ANDROMEDA_PULSE_MCP_ENABLED=true` at runtime).

**Project directory structure.**

```
andromeda-pulse/
├── Cargo.toml                      # workspace manifest
├── Cargo.lock
├── rust-toolchain.toml             # pin rustc 1.95.0
├── .github/
│   └── workflows/
│       ├── release.yml             # tauri-action: build + sign + notarize + publish
│       ├── ci.yml                  # fmt + clippy + cargo-xtask test
│       └── update-channels.yml     # Homebrew tap + Scoop manifest jobs
├── crates/
│   ├── ingest/                     # OTLP receivers (tonic + axum)
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── buffer/                     # DuckDB ring buffer + Arrow appender
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── viz/                        # query layer feeding webview WebGPU charts
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── ui-bridge/                  # TauRPC routers
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── snapshot/                   # curated markdown generator
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── workspace-detector/         # detect host project context
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── plugins/                    # wasmtime Component Model host
│   │   ├── Cargo.toml
│   │   ├── wit/                    # WIT interface definitions
│   │   └── src/
│   └── mcp-server/                 # MCP stdio sidecar (feature-gated; hand-rolled JSON-RPC 2.0)
│       ├── Cargo.toml
│       └── src/
├── pulse-app/                      # Tauri binary crate that wires the workspace
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── capabilities/               # Tauri 2 capability JSON files
│   ├── icons/
│   ├── src/
│   │   ├── main.rs
│   │   └── lib.rs
│   └── ui/                         # webview source root (frontend tooling owned by design specialist)
│       └── src/
├── xtask/                          # cargo-xtask: release/sign/notarize/changelog tasks
│   ├── Cargo.toml
│   └── src/
├── plugins-examples/               # built-in plugin templates shipped with v1
│   └── README.md
├── docs/
└── README.md
```

**CI/CD approach.**
- Platform: GitHub Actions.
- `ci.yml` runs on every PR and push to main as seven independent jobs, so a round lasts as long as its longest job: `lint-test` (matrix Linux/macOS/Windows: `cargo fmt --check` → `cargo clippy ... -D warnings` → the xtask gates → `cargo xtask test`, plus on Linux `perf:slo-load`, then `cargo nextest run --workspace --profile perf-samples` (the `pulse-app/tests/perf_budget_samples.rs` producer, excluded from the default nextest filter), `cargo xtask perf:budget --data-dir target/tmp/perf-budget-samples --require memory,snapshot`, and an `if: always()` `logs-perf-samples-{os}` upload) · `release` (macOS/Windows: `cargo build --workspace --release`; no frame boot step — the frame budget is the dev-host `perf:frame-sample`) · `mcp-test` (Linux: nextest with `--features mcp-server`) · `a11y` (matrix: `cargo xtask test:a11y`) · `boot` (Linux: the mcp-feature release build, the agent-harness boot smoke, then `cargo xtask ci-gates`) · `supply-chain` (its `cargo auditable build --workspace --release` is the Linux release smoke) · `coverage`. Rust caches are budgeted against the 10 GB repository cap: `lint-test` and `boot` own target caches keyed `lint-test-{os}` / `boot-Linux` and save on failure too; `release` owns `release-{os}` (macOS, Windows; since chunk 2026-09-30-perf-budget-gate-reads-real-samples — a `lint-test` re-save after a `Cargo.lock` change drops the release dependencies, which left the job cold) and saves on failure too (`cache-on-failure: true`, chunk 2026-09-30-perf-instruments-measure-their-budgets — a red release round had saved nothing, so the next rebuilt cold); `mcp-test`, `a11y` and `supply-chain` restore one read-only; `coverage` caches the registry only (chunk 2026-09-29-ci-wall-time-and-round-trips). Measured after that chunk: 8 entries, 10 605 172 169 of the 10 737 418 240 B cap (≈ 1.2 % headroom — a watch until the next `Cargo.lock`-driven `lint-test` re-save sheds its release dependencies). Re-read after the `release` key gained `cache-on-failure` (`ci#36765040464`, chunk 2026-09-30-perf-instruments-measure-their-budgets): byte-identical, 8 entries, 132 246 071 B (1.23 %) headroom, no key evicted — still a watch.
- `release.yml` runs on tag push (`v*`): `tauri-action` builds `.msi` / `.dmg` / `.AppImage` / `.deb` per OS in matrix (macOS `.app` bundle is built then wrapped into `.dmg`, not published standalone); signs Windows artifacts via Azure Key Vault EV cert; notarizes macOS artifacts via Apple Developer ID; uploads bundles + `latest.json` to GitHub Releases.
- `update-channels.yml` runs on completion of `release.yml`: updates Homebrew tap and Scoop manifest with new version + sha256.
- All shared CI logic that needs Rust lives in the `xtask` crate so contributors can run identical commands locally with `cargo xtask <task>`.
