# architecture.md — Amendments

Append-only history of amendments to `architecture.md`. **The body holds only current truth; this sidecar holds the
change history** (also in git). Two kinds of entry:

- **Registry-closure** — arch-registry acknowledgments (this was the in-body `§Architecture Registry Updates`
  section under v2): code that landed via `/andromeda-implement` chunks before arch could update `§Occupied
  Resources`. The items themselves are current truth in `§Occupied Resources`; these entries record when/why/where
  each was added. Each `Marker:` path under `.andromeda/runs/` holds the full original amendment record.
- **Decision-history** — the evolution narrative behind an `§Established Decisions` entry, kept out of the body so
  the body states only the current decision + its current constraints.

---

## Registry-closure amendments

### 2026-05-09 — Acknowledge `streams.*` namespace
**Section:** §Occupied Resources Tauri IPC routes.
**Change:** the Tauri IPC routes registry now lists `streams.subscribe_spans` / `streams.subscribe_metrics` / `streams.subscribe_logs` (chunk #23).
**Why:** closes the D3 stale drift for the chunk #23 TauRPC procedures.
**Ref:** NOT DERIVED

### 2026-05-09 — Acknowledge `telemetry.frontend.*` namespace
**Section:** §Occupied Resources Tauri IPC routes.
**Change:** the Tauri IPC routes registry now lists `telemetry.frontend.record_frame_ms` (chunk #29).
**Why:** closes the D3 capability drift for the chunk #29 frontend telemetry resolver.
**Ref:** NOT DERIVED

### 2026-05-11 — Acknowledge `pulse:clipboard` capability
**Section:** §Occupied Resources Tauri capability identifiers.
**Change:** the Tauri capability identifiers registry now lists `pulse:clipboard` (`pulse-app/capabilities/clipboard.json`, chunk #43) — the clipboard-manager write-only capability.
**Why:** closes the D3 capability drift for chunk #43; clipboard read stays excluded per security plan §Anti-Patterns API row 6.
**Ref:** NOT DERIVED

### 2026-05-16 — Acknowledge `curation` crate
**Section:** §Occupied Resources Cargo workspace crate names.
**Change:** the Cargo workspace crate names registry now lists `curation` (chunk #58, Epoch 9 Foundation v0.2.0) — the curation primitives extracted from snapshot.
**Why:** registers the crate chunk #58 added to the workspace.
**Ref:** NOT DERIVED

### 2026-05-16 — Acknowledge `connection.current_state` + `pulse://stream/connection-state`
**Section:** §Occupied Resources Tauri IPC routes + Tauri IPC events.
**Change:** the registries now list the IPC route `connection.current_state` and the IPC event `pulse://stream/connection-state` (chunk #59 connection FSM).
**Why:** registers the route and event chunk #59 added.
**Ref:** NOT DERIVED

### 2026-05-16 — Acknowledge `triage` crate + register `pulse-v0_2_0-route` scope
**Section:** §Occupied Resources Cargo workspace crate names + §Existing Scopes.
**Change:** the crate names registry now lists `triage` (chunk #60); §Existing Scopes registers `pulse-v0_2_0-route` (`docs/v0_2_0/pulse-v0_2_0-route.md`) — the first registered scope.
**Why:** registers the crate chunk #60 added and the v0.2.0 route scope.
**Ref:** NOT DERIVED

### 2026-05-17 — Acknowledge `pulse://stream/attention-cues`
**Section:** §Occupied Resources Tauri IPC events.
**Change:** the Tauri IPC events registry now lists `pulse://stream/attention-cues` (chunk #62).
**Why:** registers the event chunk #62 added.
**Ref:** NOT DERIVED

### 2026-05-17 — Acknowledge `pulse://stream/restart-events`
**Section:** §Occupied Resources Tauri IPC events.
**Change:** the Tauri IPC events registry now lists `pulse://stream/restart-events` (chunk #63).
**Why:** registers the event chunk #63 added.
**Ref:** NOT DERIVED

### 2026-05-18 — Acknowledge `services.list_with_states` + `pulse://stream/service-lifecycle`
**Section:** §Occupied Resources Tauri IPC routes + Tauri IPC events.
**Change:** the registries now list the IPC route `services.list_with_states` and the IPC event `pulse://stream/service-lifecycle` (chunk #67 service registry + lifecycle FSM).
**Why:** registers the route and event chunk #67 added.
**Ref:** NOT DERIVED

### 2026-05-18 — Acknowledge `corpus` + `security` crates + `storage.{inspect,path}` + `corpus/corpus.db`
**Section:** §Occupied Resources Cargo workspace crate names + Tauri IPC routes + Filesystem locations.
**Change:** the registries now list the `corpus` + `security` crates, the IPC routes `storage.inspect` / `storage.path`, and the `corpus/corpus.db` filesystem subpath (all chunk #68).
**Why:** closes the D3 capability drift for chunk #68 (persistent incident corpus + PII scrubber + storage router).
**Ref:** NOT DERIVED

### 2026-05-19 — Acknowledge `diagnostics.template_distribution`
**Section:** §Occupied Resources Tauri IPC routes.
**Change:** the Tauri IPC routes registry now lists `diagnostics.template_distribution` (chunk #69 Drain template-profiling).
**Why:** registers the route chunk #69 added.
**Ref:** NOT DERIVED

### 2026-05-21 — Acknowledge `log_templates` DuckDB table + Corpus SQLite schema sub-section
**Section:** §Occupied Resources DuckDB schema names + new Corpus SQLite schema names sub-section.
**Change:**
- the DuckDB schema names registry now lists the reserved table `log_templates` (chunk #69 Phase B);
- a new "Corpus SQLite database / schema names" sub-section lists `baseline_state` / `service_registry` / `pipeline_metrics` / `incidents` / `incident_events` / `digest_archive`.
**Why:** chunk #74 consolidation — registers the Drain template table and the corpus schema the registry did not carry.
**Ref:** NOT DERIVED

### 2026-05-21 — Tag-defer forward-promise entries
**Section:** §Occupied Resources Tauri IPC routes + Tauri IPC events + Environment variables.
**Change:** `snapshot.list_recent` + `snapshot.copy_to_clipboard` + `workspace.list` + `pulse://stream/plugin-events` + `ANDROMEDA_PULSE_CONFIG_PATH` are tagged "(deferred — no runtime emitter/consumer as of chunk #74)" — tag-deferred, NOT hard-deleted.
**Why:** closes D3 forward-promise drift (zero runtime references). Establishes the cleanup-amendment convention: a forward-promise entry with no runtime emitter/consumer is tagged deferred rather than deleted, preserving the audit trail.
**Ref:** NOT DERIVED

### 2026-05-21 — Acknowledge harness-only env vars + `run/andromeda-pulse.pid` subpath
**Section:** §Occupied Resources Environment variables + Filesystem locations.
**Change:** the Environment variables registry now lists `ANDROMEDA_PULSE_PIDFILE` / `ANDROMEDA_PULSE_LOGFILE` / `ANDROMEDA_PULSE_DATA_DIR_KEEP` as harness-only (consumed by `scripts/agent-run.{sh,ps1}`, NOT by the production binary); Filesystem locations lists the `run/andromeda-pulse.pid` subpath (written by the production binary, consumed by the harness for status/cleanup).
**Why:** chunk #74 consolidation. Harness-only env vars are excluded from the §Anti-Pattern Input row 4 canonicalization requirement.
**Ref:** NOT DERIVED

### 2026-05-23 — Acknowledge `incidents.*` namespace + `pulse://stream/incidents`
**Section:** §Occupied Resources Tauri IPC routes + Tauri IPC events.
**Change:** the registries now list the IPC routes `incidents.list_active` / `incidents.acknowledge` / `incidents.mark_resolved` and the IPC event `pulse://stream/incidents` (chunk #78).
**Why:** registers the namespace and event chunk #78 added.
**Ref:** NOT DERIVED

### 2026-05-23 — Acknowledge `pulse://stream/cadence-events`
**Section:** §Occupied Resources Tauri IPC events.
**Change:** the Tauri IPC events registry now lists `pulse://stream/cadence-events` — the chunk #80 cadence coordinator's L6-visibility topic.
**Why:** registers the event chunk #80 added.
**Ref:** NOT DERIVED

### 2026-05-23 — Acknowledge `pulse://stream/digests`
**Section:** §Occupied Resources Tauri IPC events.
**Change:** the Tauri IPC events registry now lists `pulse://stream/digests` — the chunk #81 L3 digest assembler topic.
**Why:** registers the event chunk #81 added.
**Ref:** NOT DERIVED

### 2026-05-24 — Acknowledge `interpretation` crate + `model.current_profile` + `pulse://stream/model-status`
**Section:** §Occupied Resources Cargo workspace crate names + Tauri IPC routes + Tauri IPC events.
**Change:** the registries now list the `interpretation` crate, the IPC route `model.current_profile` and the IPC event `pulse://stream/model-status` (all chunk #82).
**Why:** closes the D3 capability drift for chunk #82 — hardware profile detection + model loading + tokenizer substrate, the start of the L4 LLM interpretation pipeline.
**Ref:** NOT DERIVED

### 2026-05-25 — Acknowledge `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` + `ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH`
**Section:** §Occupied Resources Environment variables.
**Change:** the Environment variables registry now lists `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` + `ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH`, resolving the prebuilt `llama-cli.exe` per `HardwareProfileSource` tier routing; each is canonicalized + `is_file()`-asserted before spawn.
**Why:** registers the env vars of the chunk #84 L4 runtime swap (mistralrs → llama.cpp subprocess D1).
**Ref:** NOT DERIVED

### 2026-05-25 — Acknowledge `diagnostics.retry_interpretation`
**Section:** §Occupied Resources Tauri IPC routes.
**Change:** the Tauri IPC routes registry now lists `diagnostics.retry_interpretation` — the L4 degraded-mode FSM manual override (chunk #86 JSON-parse-failure handling + backoff; P-020 graceful degradation).
**Why:** registers the route chunk #86 added.
**Ref:** NOT DERIVED

### 2026-05-26 — Acknowledge `incidents.mark_all_read`
**Section:** §Occupied Resources Tauri IPC routes.
**Change:** the Tauri IPC routes registry now lists `incidents.mark_all_read` (chunk #87 Findings counter + dropdown; P-028/P-029/P-030).
**Why:** registers the route chunk #87 added.
**Ref:** NOT DERIVED

### 2026-05-27 — Acknowledge `incidents.get_report`
**Section:** §Occupied Resources Tauri IPC routes.
**Change:** the Tauri IPC routes registry now lists `incidents.get_report` (chunk #88 Diagnostic Report generation; P-031 + P-035–P-038).
**Why:** registers the route chunk #88 added.
**Ref:** NOT DERIVED

### 2026-06-03 — Acknowledge `storage.export_for_training` + `~/Downloads` egress exception
**Section:** §Occupied Resources Tauri IPC routes + Filesystem locations.
**Change:**
- the IPC routes registry now lists `storage.export_for_training` (chunk #95) — an anonymized JSONL incident corpus dump (P-046 community-training export, no auto-submission);
- Filesystem locations lists `<home>/Downloads/pulse-corpus-export-{ts}.jsonl` as an out-of-data-dir egress sink — the ONE documented exception to under-data-dir confinement, guarded by `..`-rejection + parent validation + egress PII scrubbing.
**Why:** registers the export route and its sink; the sink is out-of-data-dir by design, so it is named as the single confinement exception rather than left implicit.
**Ref:** NOT DERIVED

### 2026-06-04 — Acknowledge `config-watcher` crate + `config.{reload,status}` + `diagnostics.reevaluate_recent_window` + `pulse://stream/config-events`
**Section:** §Occupied Resources Cargo workspace crate names + Tauri IPC routes + Tauri IPC events.
**Change:** the registries now list:
- the `config-watcher` crate (`notify` 8.x FS watcher on `<data_dir>/config.toml` + `tokio::sync::watch` fan-out);
- the IPC routes `config.reload` / `config.status` and `diagnostics.reevaluate_recent_window`;
- the IPC event `pulse://stream/config-events` — aggregate-only, no raw config values or paths.
**Why:** closes the D3 capability drift for chunk #96 configuration hot reload + prospective threshold application (P-055 / P-056).
**Kept:** §Design Philosophy unmodified — its word-form counts ("twelve library crates" → 14, "fourteen workspace members" → 16) were surfaced informationally only.
**Ref:** NOT DERIVED

### 2026-06-05 — Acknowledge `diagnostics.snapshot` + `diagnostics.history`
**Section:** §Occupied Resources Tauri IPC routes.
**Change:** the Tauri IPC routes registry now lists `diagnostics.snapshot` + `diagnostics.history` (chunk #97 Settings → Diagnostics view; L6 self-observability, HYBRID-RENDER).
**Why:** registers the routes chunk #97 added.
**Ref:** NOT DERIVED

---

## Decision-history

### 2026-05-25 — LLM inference runtime: mistralrs → llama.cpp subprocess (D1), session 144 validation
**Section:** LLM inference runtime decision (L4).
**Change:** the L4 runtime was `mistralrs = "=0.8.0"` in-process (chunk #82/#83 Pre-D1 decision, chosen for its native JSON-constrained generation); now prebuilt `llama.cpp` (build b9305) invoked as a **subprocess** (D1 spawn-per-generation). The body carries the defense-in-depth bounds discipline `-n {max_tokens}` + `-st` + an outer wall-clock timeout. Only the concrete impl swapped (`MistralRsInference` → `LlamaCliInference`); `pub trait LlmInferenceRunner: Send + Sync` is unchanged. Rejected alternatives stay documented sibling-impl swap paths through that trait: in-process `llama-cpp-2` bindings (`=0.1.146` exposes `json_schema_to_grammar()` and fits the trait via `spawn_blocking`) and `candle` (the original chunk #82 escape hatch).
**Why:** `mistralrs` CPU inference is broken on Windows MSVC hosts (upstream issue #1134, open across 0.7.0 → 0.8.1+): sampling threads deadlock with zero token output. `llama.cpp` on the same GGUF and host generated cleanly — 122.3 tok/sec under `--json-schema-file` GBNF constraint with complete schema-conformant JSON (28.2 tok/sec CPU, 231.2 tok/sec unconstrained CUDA). `llama-cpp-2` was rejected for its Windows build-toolchain cost (`libclang` + `cmake` + MSVC), not justified for v0.2.0. Two runaway generations from missing bounds motivated the bounds discipline. The trait was designed to anticipate exactly this staged swap as the bus-factor mitigation pattern.
**Ref:** NOT DERIVED

---

## 2026-06-28-deterministic-env-gated-l4-mode — Register ANDROMEDA_PULSE_L4_DETERMINISTIC env var
**Section:** §Occupied Resources — Environment variables (reserved at arch level)
**Change:** the Environment variables registry now lists `ANDROMEDA_PULSE_L4_DETERMINISTIC` (truthy gate, default false), which selects the deterministic L4 runner (canned `L4Output`, no GPU/model) at pulse-app boot, alongside the existing `_LLAMA_{CUDA,CPU}_BIN_PATH` L4 env vars.
**Why:** the chunk (P-073) introduced the env-gated deterministic L4 mode, and every new env var is registered in §Occupied Resources.
**Kept:** §Established Decisions [LLM Inference Runtime] unchanged — the deterministic runner is a third impl behind the unchanged `LlmInferenceRunner` trait.
**Ref:** NOT DERIVED

---

## 2026-06-28-investigate-actions-functional — Register investigate.run_action TauRPC procedure
**Section:** §Occupied Resources — Tauri IPC routes (TauRPC procedures)
**Change:** the Tauri IPC routes registry now lists `investigate.run_action` (`InvestigateApiImpl`) — runs a real `LlmInferenceRunner::generate_constrained` analysis of the curated telemetry for one of the 4 bounded Investigate actions, reusing the incident `L4Output` schema, and returns a TRANSIENT scrubbed `InvestigateResultDto`; no incident is created or persisted, no broadcast; deterministic-L4-aware (P-073).
**Why:** the chunk (P-072) wired the 4 dead Investigate buttons to a real L4-backed analysis, and every new IPC procedure is registered in §Occupied Resources.
**Kept:** §Established Decisions [LLM Inference Runtime] + §Stack unchanged — the procedure is an additive second consumer of the unchanged `generate_constrained` trait method; no new crate, no new dep.
**Ref:** NOT DERIVED

---

## 2026-06-29-window-geometry-movable-shell — Register window-geometry.json filesystem location
**Section:** §Occupied Resources — Filesystem locations (subpaths under the resolved data dir root)
**Change:** Filesystem locations now lists `window-geometry.json` — remembered per-window positions: a Rust-owned JSON map of window label → integer x/y, written atomically (`.tmp` + rename) by `pulse-app/src/window_geometry.rs` on window-move and restored at boot.
**Why:** the chunk (P-061) added a new persisted data-dir file. New occupied filesystem resources are registered in §Filesystem locations (precedents: `corpus/corpus.db` chunk #68, the Downloads export sink chunk #95). The resources drift detector's literal scope is IPC/port/env-var/crate and does not flag filesystem additions, so this registration was by judgment; extending the detector to §Filesystem locations is a candidate if it recurs.
**Ref:** NOT DERIVED

---

## 2026-07-07-plain-language-connection-status — ready envelope: ReadyChecks +3 fields
**Section:** §Standard Contracts — `ready` command
**Change:** the `ready` envelope's `checks` object now carries `rows_ingested` / `buffer_used_seconds` / `retention_seconds` (u64) alongside the existing 5 checks; the §Standard Contracts example matches the live `ReadyChecks` shape.
**Why:** the chunk (P-070) exposes ingest rate + buffer fill on the EXISTING `ready` procedure — no new procedure, namespace or capability — to back the plain-language connection-status line. Fields added to an existing method's envelope were applied with the user as current-truth registry completeness.
**Ref:** NOT DERIVED

## 2026-07-10-incidents-floating-window-disclosure — Cross-window UI-coordination events registered
**Section:** §Occupied Resources → Tauri IPC events (broadcast channels)
**Change:** the IPC events registry now lists the first-party webview↔webview coordination events `findings:dismissed` / `report:open` (`{incidentId}`) / `report:closed` (`@tauri-apps/api/event`, gated by `core:event` in `pulse:default`) as a DISTINCT class from the `pulse://stream/*` Rust→webview broadcast/telemetry channels; the registry previously enumerated only the `pulse://stream/*` topics.
**Why:** the chunk added a separate `findings` + `report` floating-window pair that coordinates dismiss / open-report / cross-window focus-restore via named frontend events, whose contracts were unregistered.
**Kept:** the capability grants (`allow-set-position` / `allow-set-size`) and window labels stay out of §Occupied Resources, per the 2026-06-30 core-perm registry rule.
**Ref:** NOT DERIVED

## 2026-08-14-fingerprint-feed-capture-repair — Register the harness UTF-8 relay env vars
**Section:** §Occupied Resources → Environment variables (reserved at arch level)
**Change:** the Environment variables registry now lists `PYTHONUTF8` (`=1`) + `PYTHONIOENCODING` (`=utf-8`) as harness-only entries — SET (not read) by `scripts/agent-run.{sh,ps1}` and mirrored in the `.claude/settings.json` `env` block; the PowerShell half also sets `[Console]::OutputEncoding` / `InputEncoding`. Marked NOT consumed by the production binary, on the `ANDROMEDA_PULSE_PIDFILE` / `_LOGFILE` / `_DATA_DIR_KEEP` precedent.
**Why:** the chunk landed the UTF-8 relay in both harness scripts and settings.json. The registry is canonical for env vars the harness sets, not only those it reads, and already carries harness-only entries.
**Ref:** NOT DERIVED

## 2026-08-14-workspace-key-alignment — published workspace key + measured corpus key-custody reality
**Section:** §Occupied Resources → Filesystem locations; §Occupied Resources → Corpus SQLite database / schema names → At-rest posture
**Change:**
- Filesystem locations registers `run/workspace-key` — the resolved incident workspace key, published by the app at boot (atomic `.tmp` + rename, canonicalize-and-confine guard) and read cross-process by the `andromeda-pulse-mcp` sidecar; bounded ≤4096 bytes on read, consumed as an opaque filter string, never joined as a path.
- Corpus at-rest posture: was a posture measurement disproved; now OS-keychain sourcing is INTENDED but UNIMPLEMENTED — `keyring 3.6.3` resolves with `[log, zeroize]` only, no platform credential-store backend is linked, so the cell key is per-process ephemeral and historical encrypted content is orphaned until a backend fix plus migration lands.
- The "OS-keychain-encrypted" wording is dropped from the duplicate `corpus/corpus.db` subpath entry.
**Why:** the workspace-key registration follows the 2026-06-29 `window-geometry.json` registration precedent. The at-rest correction records measured truth: a second process could not decrypt its predecessor's rows and the OS credential store held zero entries. The defect is pre-existing, unmasked rather than caused by this chunk; the impl fix is owned by the "Corpus key persistence" working-route entry.
**Ref:** NOT DERIVED

## 2026-08-15-corpus-key-persistence — corpus key custody landed; two resources + a Stack row registered
**Section:** §Occupied Resources → Corpus SQLite → At-rest posture · §Occupied Resources → Filesystem locations · §Occupied Resources → Environment variables · §Stack and Technologies
**Change:**
- At-rest posture: was key sourcing INTENDED-but-UNIMPLEMENTED with a per-process ephemeral key; now `keyring` 3 carries the explicit platform feature set, so the OS credential store is the primary source and the key persists across processes. The opt-in `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` fallback and the boot-time inventory-then-purge disposition of pre-fix orphaned content are described; "Windows DPAPI" is corrected to "Windows Credential Manager".
- The duplicate "per-process ephemeral / no OS-keychain backend linked" wording on the `corpus/corpus.db` subpath entry is retired.
- Registered the env var `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` (bounded parse, secret-class, never logged) and the data-dir path `corpus/orphaned-inventory-{unix_nano}.md`.
- §Stack gains a "Secret / key storage" row — the credential-store runtime had never been registered as a stack layer, and the feature set is load-bearing because keyring 3.x has no `default` feature.
**Why:** the chunk measured the previous claim false and closed it — a second boot decrypted its predecessor's rows with zero decryption failures, the OS credential-store entry present. This is the inverse of the 2026-08-14 amendment, which recorded the gap and named this chunk as its owner.
**Ref:** NOT DERIVED

## 2026-08-16-baseline-family-reachability — register the baseline cold-start window override
**Section:** §Occupied Resources → Environment variables (reserved at arch level)
**Change:** the Environment variables registry now lists `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` — production-consumed (it moves the running app's silence gate; NOT the harness-only class); bounded parse (non-zero, strictly below `WINDOW_DURATION_SECONDS`, whitespace-trimmed, every rejection falling back to the 3600s default without a panic); resolved once at boot and handed from `Thresholds::bootstrap_window_seconds` to `BaselineState::set_bootstrap_window_seconds` so the silence evaluator, the emitter counters and the lifecycle registry share ONE bound. Env layer only — no `Settings` / `config.toml` surface, so no TauRPC contract and no hot-reload delta.
**Why:** the chunk landed a new production-consumed env var the registry did not list. Registration is routine per the 2026-06-28 new-env-var rule: the input is code-validated and unit-tested, so it is registry completeness rather than an unvalidated boundary.
**Kept:** env-layer-only scoping, deliberately — a `Settings` field was rejected because it trips the boot-smoke trigger and regenerates the TauRPC bindings.
**Ref:** NOT DERIVED

## 2026-08-16-fault-identity-semantics-decided — Fault identity decided at both layers
**Section:** §Established Decisions
**Change:** new entry **[Fault Identity — what makes two faults ONE fault]** recording both decided semantics:
- L1 fingerprint normalization is TOKEN-LEADING — an absolute marker must START a token; relative path structure stays identity-significant, so `src/a.rs` and `src/b/c.rs` are distinct fingerprints.
- L2 incident identity is COALESCE-PER-CUE-IDENTITY on `(kind, scope, scope_id)`.
The entry also carries both rejected alternatives, the deferred `AttentionCue` threading path, and the known `Incident.fingerprint` producer defect that leaves the corpus-retrieval `fingerprint_match` arm correct-per-contract but starved.
**Why:** operator-decided at the chunk's phase review, both recommendations accepted. Arch previously carried NO fault-identity, fingerprint or incident-coalescing entry, so a later chunk reading arch alone could have re-keyed coalescing on `L4Output.fingerprint` or re-greedied the normalizer — both measured wrong here. L1's docs and pre-existing test had encoded absolute-only all along; only the implementation was greedy (any `/` started a path, and the skip swallowed the rest of the token). Switching cost: the external Conductor verification harness had just aligned to the greedy behaviour and re-aligns in that repo; Conductor is read-only from here — cited, never copied.
**Kept:** purely additive — no existing arch wording was retired.
**Ref:** NOT DERIVED

## 2026-08-17-incident-fingerprint-producer-repaired — [Fault Identity]: deferral SHIPPED, producer repaired, retrieval arm FED
**Section:** §Established Decisions → [Fault Identity — what makes two faults ONE fault]
**Change:** four clauses of the entry restated to shipped reality, all within its single bullet (no second site restates the retired wording):
1. was the **DEFERRED** `AttentionCue` → `DigestCueRef` → producer threading; now **SHIPPED 2026-08-17** — both carriers named (`Option<String>`, `#[serde(default)]`, full 32-char lowercase hex; `None` for baseline-derived families).
2. was "Known producer defect, owned elsewhere"; now **"Producer repaired"**: `Incident.fingerprint` is produced from the threaded L1 fingerprint via the crate-internal `triage::contract::hex_lower` — the encoder the assembler applies to Q3 rows, explicitly NOT the 4-byte `fingerprint_to_hex_prefix`. The corpus-retrieval `fingerprint_match` arm is **FED**, not STARVED; its do-not-simplify guard now rests on the arm being live.
3. the L1 blast-radius sentence no longer says "confined to storm grouping": the value flows onward into `Incident.fingerprint` (persisted, read by the retrieval arm and assembler). No viz / MCP / UI consumer exists — the field reaches no rendered surface — and `span_events.fingerprint` is still never SELECTed.
4. the L2 coalesce-key rationale: was "because the cue does not carry one" (now false); now the key rests on every downstream surface already being N-safe. Per-fingerprint dedupe is **possible-but-declined** and REVISITABLE; the predicate itself is unchanged.
**Why:** the chunk shipped the exact threading the entry had recorded as deferred and fed the arm it described as starved; the same two now-false L2 reasons were corrected in-code in the same chunk.
**Kept:** the rejected alternative (keying on `L4Output.fingerprint`) is untouched and still stands.
**Ref:** NOT DERIVED

## 2026-08-17-conductor-e2e-verification-closure — deterministic-L4 fixture posture: populated arrays, and the two fields that stay empty
**Section:** §Occupied Resources → Environment variables → `ANDROMEDA_PULSE_L4_DETERMINISTIC`
**Change:** the entry now records:
- the canned `L4Output`'s arrays are POPULATED (three `evidence_refs`, two `hypotheses`, two `investigation_steps`), and this is load-bearing — the value flows through the single producer join into `Incident.evidence_refs.fingerprint_hashes`, the MCP `retrieve_telemetry_slice.fingerprint_refs` response and the Report's Evidence section; while the arrays were empty every payload-identity assertion there compared nothing and an absence check passed for the wrong reason;
- `trace_id` / `span_ids` / `timestamps_unix_nano` are hardcoded empty at the same producer in EVERY mode, so `span_refs` and `timestamps_unix_nano` are permanently empty in production;
- `degraded_mode` is not a fixture field but `Report.degraded_mode = parsed_l4.is_none()`, driven by Resolved-only persistence.
**Why:** registry completeness for a state the chunk changed (the `CANNED_L4_OUTPUT_JSON` value), plus two premises the chunk measured false — the always-empty sibling fields confirmed live by a real subprocess read. The prior wording was incomplete rather than false.
**Ref:** NOT DERIVED

## 2026-08-21-delegated-timing-observables — three delegated-timing IPC procedures + the capability-granularity correction
**Section:** §Occupied Resources → Tauri IPC routes; §Cross-cutting Patterns → Webview IPC capability policy
**Change:**
- the IPC routes registry now lists `telemetry.frontend.record_constellation_hue_latency` / `record_constellation_discovery_latency` / `record_findings_counter_refresh` with their input types, bounds and `metric.constellation.hue_update_ms` / `metric.constellation.discovery_ms` / `metric.findings.counter_refresh_ms` targets; P-025 measures the constellation DOT hue (the Halo canvas has no render site).
- capability policy: was per-procedure `pulse-app/capabilities/` entries; now Tauri capabilities stay negative-default at the IPC-LAYER granularity — TauRPC dispatches all methods through one invoke handler, so no per-procedure entries exist and the per-procedure gate is the `EXPECTED_PROCEDURES` pin. Core APIs (`fs` / `shell` / `dialog` / `http`, each `core:window:*`) STILL require an explicit grant and ARE silently rejected when missing.
**Why:** the chunk landed three procedures with `pulse-app/capabilities/` untouched while `xtask capability-drift` stayed clean — measuring the per-procedure-entry requirement false. The security rules had flagged this tension on 2026-05-03; the arch body still asserted it.
**Ref:** NOT DERIVED


## 2026-08-22-log-records-identity — log-record primary key restated; the `name` component it claimed never existed
**Section:** §Conventions → Primary key convention
**Change:** the conflated metric/log clause is split.
- Metric points keep OTLP-native identity, now stated by column (`metric_name` + `ts_unix_nano` + `resource_hash`).
- Log records: was "timestamp + resource hash + name"; now `(ts_unix_nano, resource_hash, severity_number, seq)`, with `seq` a monotonic in-process ordinal allocated per batch by `BufferState::reserve_log_seq_block` — recorded as the ONE declared exception to the no-surrogate-keys rule: an internal disambiguating ordinal, never an observable.
- Adds the rejection of a per-batch `span_events.event_index`-style ordinal and why it does not transfer, plus the note that the Arrow Appender enforces this key at `flush()` and a colliding pair fails the entire batch.
**Why:** `log_records` has **no `name` column at all**, so the retired wording was inaccurate independent of this chunk, which then changed the key itself. Doc-only: the impl already embodies the new convention. The `event_index` rejection is recorded because "follow the in-repo ordinal precedent" reads as endorsing it, and a literal application ships a half-fix (cross-batch collisions survive) — the operator chose the monotonic column for exactly that reason.
**Ref:** NOT DERIVED

## 2026-08-23-ingestion-scrub-coverage — three reserved DuckDB tables recorded as declared-but-never-written
**Section:** §Occupied Resources → DuckDB database / schema names → Reserved tables (primary) · §Conventions → Database entity naming (restating site)
**Change:** the reserved-table enumeration no longer reads as seven live write targets. `append_record_batch_to_table` is the only DuckDB write path in `crates/buffer` and fires for exactly FOUR — `spans`, `metrics_points`, `log_records`, `span_events`. **`resources`, `instrumentation_scopes` and `span_links` have no producer**: all three are declared in the schema and swept by retention, so three of the seven retention DELETEs sweep permanently-empty tables. The §Conventions restatement (the same seven names unqualified, plus the periodic `DELETE FROM <table>` cutoff task) is qualified in the same pass. Consequences recorded: none of the three can carry host data, so none can leak nor be scrubbed (security-plan §Logging records the same fact from the PII side, where only `instrumentation_scopes` has a client-controlled text column); a claim keyed on "the reserved tables" must not be read as "the written tables".
**Why:** the chunk measured `instrumentation_scopes` producer-less while establishing its target set, and the adjacent sweep found `resources` and `span_links` in the same state. Applied AS MEASURED: the impl half — a producer lands, or the dead schema and its dead DELETEs are retired — is owned by the working-route entry "Diagnostics un-muting + harness-truth sweep". This entry records the measurement, not the cleanup.
**Ref:** NOT DERIVED

## 2026-08-23-metrics-points-identity — metric-point key widened to four columns; the `seq` exception now spans two tables
**Section:** §Conventions → Primary key convention
**Change:**
- Metric points: was the three OTLP-native columns; now `(metric_name, ts_unix_nano, resource_hash, seq)`. The clause states WHY for both tables — `metrics_points` stores no attributes column, so two points of one metric differing only by label set are identical on every native column, and the ingestion scrub makes two distinct credential-shaped names redact to one placeholder and collide there.
- `seq` remains the one declared no-surrogate-key exception but now spans BOTH `log_records` and `metrics_points`, allocated per table from separate buffer-global counters (`reserve_log_seq_block` / the new `reserve_metric_seq_block`). The non-observability list gains `SELECT_METRICS` / `COUNT_METRICS` / the `metrics.*` IPC response.
- The rejected per-batch-ordinal rationale now covers both tables; the measured note extends to 2026-08-23 (`rows_ingested` 0 pre-fix → 5 post-fix).
**Why:** the chunk landed `metrics_points.seq BIGINT NOT NULL` and the 3→4 column PK in all three DDL representations plus the public `BufferState::reserve_metric_seq_block` boundary; the OTLP-native-only metric key was measured insufficient. No other arch site carries `metrics_points` key wording.
**Kept:** §Occupied Resources → DuckDB reserved tables enumerates TABLE names, not columns, so the new `seq` column needs no entry there.
**Ref:** NOT DERIVED

## 2026-08-23-metrics-points-labels — metrics_points gains a scrubbed non-key labels column
**Section:** §Conventions → Primary key convention
**Change:** was the present-tense premise that `metrics_points` stores **no attributes column at all**; now the table carries a scrubbed `labels VARCHAR NOT NULL DEFAULT ''` column (all three DDL representations), deliberately OUTSIDE the key — the PK stays exactly `(metric_name, ts_unix_nano, resource_hash, seq)`, so two data points of one metric differing only by label set remain identical on every KEY column and are separated by `seq` alone; labels carry dimension for read-back, never identity. The retired premise is kept as an explicit past-tense parenthetical, because it is the reason the `seq` exception exists.
**Why:** the chunk added the column, falsifying the section's claim while the `seq`-exception rationale it supports survives intact. The retired wording appeared nowhere else in the master.
**Kept:** §Occupied Resources → DuckDB reserved tables enumerates TABLE names, not columns (the same ruling the predecessor chunk recorded), so the new column needs no entry there; §Conventions → Database entity naming makes a table-level naming claim only, also unchanged.
**Ref:** NOT DERIVED

## 2026-08-23-webview-self-verify — GUI verification harness registered (env var + Stack row)
**Section:** §Occupied Resources → Environment variables · §Stack and Technologies
**Change:**
- the harness-only env-var class now lists `ANDROMEDA_PULSE_MSEDGEDRIVER_PATH` — NOT consumed by the production binary; whitespace-trimmed + `is_file()`-guarded; unset / not-a-file ⇒ clean SKIP printing the fetch recipe; deliberately NOT canonicalize-and-confined because the driver lives outside the data dir by design (cross-referencing the security-plan carve-out).
- §Stack gains a row for the dev-only GUI verification harness — `@crabnebula/tauri-driver` 2.x + `webdriverio` 9.x as `pulse-app/ui` devDependencies, win32 native driver via the napi optional dep, driven by `cargo xtask webview-drive`; no Rust dependency and no runtime/bundle impact.
**Why:** the chunk landed both. The env var joins its harness-only peers (`_PIDFILE` / `_LOGFILE` / `_DATA_DIR_KEEP`), so leaving it unregistered was registry drift. §Stack carried no GUI/e2e driver technology while the chunk added two npm devDeps plus a native driver binary; §Stack already hosts dev tooling (Code quality, CI task runner), so the row belongs there.
**Kept:** no workspace crate, TauRPC procedure or product-bound port changed — `:4444` / `:4445` are harness-owned, loopback and released at teardown, so no port reservation is implied.
**Ref:** NOT DERIVED

## 2026-08-23-integration-ux-e2e-test — harness row re-scoped to the 7-stage path; xtask→node relay set registered
**Section:** §Stack and Technologies → GUI verification harness (dev-only) row · §Occupied Resources → Environment variables
**Change:**
- the §Stack harness row's Role: was the one-press leg; now the **assembled 7-stage path** (launch → traces-empty → traces-populate → storm-incident → investigate → empty-states → widget-close) driven by `cargo xtask webview-drive [--expect-absent <STAGE>] [--no-inject]` under the deterministic-L4 gate the driver sets into the *spawned app's* environment, asserting per stage against the obs log and/or the driver's DOM report; `--no-inject --expect-absent <STAGE>` is the code-driven RED mutation arm, and telemetry enters only over real OTLP via `crates/ingest/examples/inject_demo`. The technology cell is unchanged — no dependency added or bumped.
- the harness-only **xtask→node relay set** `PULSE_BIN` · `PULSE_DATA_DIR` · `MSEDGEDRIVER_PATH` · `PULSE_INJECTOR` is registered as one entry, none consumed by the production binary; `PULSE_INJECTOR` is omitted by `--no-inject`.
**Why:** the Role cell — arch's only statement of what the sanctioned harness does — contradicted the shipped staged shape. The relay registration was applied wider than proposed: only `PULSE_INJECTOR` was flagged, but all four names were unregistered, and registering one of four would leave the registry lopsided and misleading. `PULSE_*` is a claimed prefix where a collision would be silent, so the set is registered together; the three pre-existing names are retroactive completeness from `2026-08-23-webview-self-verify` (doc-only, impl already correct).
**Kept:** deliberately not registered — `:4444` (harness-owned), `ANDROMEDA_PULSE_L4_DETERMINISTIC` (a new consumer, not a new variable — already registered), and the `msedgedriver` install location (a machine-local operator fact; arch already states the driver lives outside the repo).
**Ref:** NOT DERIVED

## 2026-08-23-headful-leg-extension — GUI harness Role cell: assembled path 7 → 13 stages
**Section:** §Stack and Technologies → GUI verification harness (dev-only), Role cell
**Change:** the assembled path enumeration was 7 stages; now 13 (adds traces-scroll · connection-status · findings-window · report-window · dashboard-toggle · dashboard-close); each id is `--expect-absent`-eligible; CLI shape unchanged.
**Why:** the chunk extended the harness `STAGES` to 13 and the GREEN leg drove all 13; this is the same cell both predecessor chunks re-scoped.
**Ref:** NOT DERIVED

## 2026-08-24-headful-mechanics-probe-race-disposition — headful leg 13 → 15 stages
**Section:** §Stack and Technologies → "GUI verification harness (dev-only)" row, Role cell
**Change:** the assembled path was 13 stages; now **15**, with the full chain re-enumerated: `native-menu-suppressed` inserted after `traces-scroll` and `signpost-repeat` after `widget-close`, which consequently is no longer terminal — `signpost-repeat` follows it deliberately, restoring the widget so a second real close press proves the P-063 signpost fires on every close. Growth lineage recorded as 7 → 13 → 15 with its chunk markers. The `--expect-absent`-eligible / CLI-shape-unchanged clause is preserved verbatim.
**Why:** the chunk landed two stages and DECLINED two others on a measured driver probe. Arch §Stack is the only site in the document stating the stage count.
**Ref:** NOT DERIVED

## 2026-08-25-demo-injector-formalized-api-surface-retire — injector CLI contract + two unregistered env vars
**Section:** §Stack and Technologies (GUI verification harness row) · §Occupied Resources → Environment variables (`PULSE_INJECTOR` entry; two new entries)
**Change:**
- The §Stack harness row's injector clause records `inject_demo`'s formalized CLI contract: arg-less default emits byte-identically to the prior finite ~600-batch 100%-error storm; `--sustained` runs unbounded at a moderate rate; `--minutes=N` bounds a run; `--error-pct=0..100` is range-checked; an unknown arg is rejected with exit 2; sustained mode retries a transient export failure under a consecutive-failure ceiling.
- The `PULSE_INJECTOR` entry states that `xtask::webview_drive` passes the PATH only and threads NO arguments, so the headful leg always runs the arg-less default, and the arg-less default's byte-identity is what keeps the leg's storm-stage budget stable.
- Registered two env vars absent from every registry section: `ANDROMEDA_PULSE_MODEL_PATH` (product-consumed by `llamacli_inference.rs`; previously named only in §Established Decisions [LLM Inference Runtime] prose as an open co-design item, while its two `_LLAMA_*_BIN_PATH` siblings were registered at chunk #84) and `ANDROMEDA_LLAMA3_TOKENIZER_PATH` (build-script only, `crates/triage/build.rs`; non-`ANDROMEDA_PULSE_` prefix).
**Why:** the chunk formalized the injector into a supported dev/test tool and exercised the model/binary env pair in its live real-L4 leg. Arch already registers the sibling harness CLI shape (`cargo xtask webview-drive [--expect-absent <STAGE>] [--no-inject]`) in full and has precedent for retroactive harness-only registration "for namespace completeness" (the `PULSE_*` relay set), so a formalized flag contract on an already-registered harness resource counts as an unregistered resource. The two env-var registrations close pre-existing registry gaps the chunk surfaced — no env var was added or changed by the chunk. The confinement posture of the model/binary paths belongs in security-plan §Security Anti-Patterns → Input, not here.
**Ref:** NOT DERIVED

## 2026-08-26-l4-runtime-security-residuals — L4 path guard, opt-in confinement root, dev-vs-bundled binary names
**Section:** §Occupied Resources → Environment variables (new `ANDROMEDA_PULSE_L4_ALLOW_ROOT` entry; `ANDROMEDA_PULSE_MODEL_PATH` entry) · §Occupied Resources → Process / service identity
**Change:**
- Registered `ANDROMEDA_PULSE_L4_ALLOW_ROOT` — the opt-in confinement root for the three product-consumed L4 path inputs: bounded trimmed parse; FAIL-CLOSED — set-but-unresolvable rejects every candidate rather than degrading to unconfined; opt-in rather than data-dir-default because the GGUF and `llama-cli.exe` live outside the data dir by design (a default would reject every shipped configuration); env-layer only; drives two observables.
- `ANDROMEDA_PULSE_MODEL_PATH`: was "Like them it is canonicalized but deliberately NOT confined under the data dir"; now the landed guard posture for all three vars: always-on traversal-reject-before-canonicalize (canonicalization erases `..`, so a post-check can never see it) + 4096-byte bound + `canonicalize()` + regular-file assert, PLUS confinement when the root resolves; unconfined only when the root is unset, then announced once per boot.
- §Process / service identity: the single "Binary name" line is split into the dev-build binary (`target/{profile}/pulse-app.exe`, governed by Cargo's `[[bin]] name`) and the bundled artifact (`andromeda-pulse.exe`, produced by the Tauri bundler); `target/{profile}/andromeda-pulse` exists on no host; the sidecar is the one legitimately `andromeda-pulse*`-named build output.
**Why:** the chunk landed the guard, so the registry's unconfined claim and its missing env var were stale against shipped reality. A typo must not disable the guard the operator asked for, hence fail-closed. The binary-name split was operator-directed; its stated recurrence basis ("cost two consecutive plans") was re-derived to ONE measured instance, so the amendment rests on the corrected ground: the registry entry INVITES the error by naming only the bundled form.
**Ref:** NOT DERIVED

## 2026-08-26-interpretation-brief-completeness — degraded_mode mechanism replaced (creation + dedupe attach)
**Section:** §Occupied Resources → Environment variables → `ANDROMEDA_PULSE_L4_DETERMINISTIC`
**Change:**
- Was "driven by Resolved-only `resolution_summary_text` persistence, so an Active incident renders degraded in real-model mode too"; now: `resolution_summary_text` is written AT CREATION (scrubbed `L4Output` JSON) and REFRESHED on every dedupe re-generation, so a LIVE (Active/Acknowledged) incident with a clean parse renders the full six-section brief (`degraded_mode: false`) in every mode; the degraded notice remains only for genuinely-unparsed/absent interpretation; the resolution-summary generation (when it fires) is the final write.
- Producer join restated: `Incident.evidence_refs.fingerprint_hashes` is the order-preserving dedup UNION of the parsed refs and the triggering cue's real L1 fingerprint — parsed refs FIRST, so the three synthetic P-073 refs still lead and contains-pins still ride them.
- Sibling residual unchanged: `trace_id` / `span_ids` / `timestamps_unix_nano` hardcoded empty in every mode.
**Why:** the chunk's fix made the old clause false — measured live, an Active incident renders `degraded_mode: false` cross-process, with the real model copying the citable cue fingerprint. `DigestKind::ResolutionSummary` has zero non-test constructors, so the retired clause's "Resolved-with-L4" class was production-empty and is subsumed here.
**Ref:** NOT DERIVED

## 2026-08-26-interpretation-brief-completeness — [Fault Identity] L1 blast radius extended
**Section:** §Established Decisions → [Fault Identity] → L1 blast-radius clause
**Change:** Was "No viz / MCP / UI consumer exists (the field reaches no rendered surface …)"; now the L1 fingerprint DOES reach a rendered surface — threaded as a citable evidence id into the L4 prompt (`citable_evidence_ids`, deduped order-stable from `digest.attention_cues[].fingerprint`) and, via the producer union, into the MCP `retrieve_telemetry_slice.fingerprint_refs` response and the Diagnostic Report's Evidence section — while the `Incident.fingerprint` FIELD itself still has no viz / MCP / UI consumer and `span_events.fingerprint` is still never SELECTed.
**Why:** the chunk's Deliverable B renders real fingerprints in the live brief. The prior wording was the staleness the 2026-08-17 producer-repair entry predicted this change would cause.
**Ref:** NOT DERIVED

## 2026-08-27-report-window-copy-affordance — headful leg 15 → 16 stages
**Section:** §Stack and Technologies → GUI verification harness (dev-only) Role cell
**Change:** Was 15-stage; the assembled path now reads **16-stage** and enumerates `report-copy` between `report-window` and `investigate`; the growth lineage gains `15 → 16 at 2026-08-27-report-window-copy-affordance`, the new stage pressing the report's real Copy control and reading its settled `data-copy-state` plus the live-region text, DOM-only by construction like its `report-window` / `findings-window` neighbours. Trailing clauses (per-stage `--expect-absent` eligibility, CLI shape unchanged, injector contract, msedgedriver / napi notes) untouched.
**Why:** the chunk appended one `STAGES` entry and the leg now reports 16/16. arch §Stack is the only site in this document stating the stage count, matching the precedent recorded at the 2026-08-24 entry.
**Ref:** NOT DERIVED

## 2026-08-27-incident-persist-vs-resolve-write-race — corpus write arbitration decided
**Section:** §Established Decisions (NEW entry, appended after [Fault Identity])
**Change:** Added **[Corpus Write Arbitration — which of two writers to the same incident row wins] Monotonic last-writer guard at the `CorpusWriter` choke point**:
- A bound `AND updated_unix_nano <= ?2` predicate on the existing `incidents` UPDATE plus a bound existence probe classifying the zero-row result; both `update_incident_status` signatures return `Result<IncidentWriteOutcome, _>` (`Applied` | `DeclinedStale`), so a stale write DECLINES rather than failing.
- Placement is the corpus layer — the only choke point all SEVEN production writers traverse (the seventh is the cross-process MCP sidecar, which bypasses the `IncidentPersistence` trait).
- REJECTED alternatives: a status-only predicate (disproved — `attach_resolution_summary` requires `status == Resolved`); delta-persistence (does not reach the cross-process path).
- `triage::contract::IncidentWriteOutcome` is a parallel type, not a re-export (DAG direction).
- Records the accepted app-registry-staleness residual with its owner.
**Why:** the chunk landed a concurrency-control pattern §Stack / §Established Decisions did not cover — no write-arbitration / optimistic-locking / transaction-scope entry existed, so this is an addition, not a correction, and retires nothing. Operator decision (P4 fork 1).
**Kept:** the §Fault Identity "resolution-summary generation is the final write" clause — preserved, and precisely the reason the status-only predicate was rejected.
**Ref:** NOT DERIVED

## 2026-08-28-duplicate-span-replay-fails-loudly — injector `--replay`, budget-not-byte identity, duckdb pin
**Section:** §Stack and Technologies (GUI verification harness row · Storage engine row) · §Established Decisions [Database] · §Inherited Defaults (Database) · §Occupied Resources (Environment variables → `PULSE_INJECTOR`)
**Change:**
- Registered the injector's fourth flag `--replay` (flag set 3 → 4): pins the identity salt so a restart deliberately re-emits the prior run's `trace_id`/`span_id` — the retrying-exporter shape `smoke:gap-resume` arm A drives.
- Arg-less default: was "byte-identically"; now **budget-identical** at BOTH sites (§Stack row and the `PULSE_INJECTOR` entry) — batch/span/error counts and durations unchanged, but each run draws a `fresh_run_salt()` into `trace_id`/`span_id`, so span identity is deliberately per-run and NOT byte-stable; the clause's historical verification was a COUNT equality (3540 = 3540), never a byte comparison.
- duckdb: was `1.10500.x` at all THREE sites; now requirement-plus-resolved — `^1.10500` (Cargo.toml, untouched) resolving to 1.10505.0.
**Why:** the chunk salts span identity so a producer restart cannot collide with itself on the `spans` primary key, and fixed the consumer-wedge defect by bumping the duckdb lockfile — leaving three sites naming a version family the lockfile no longer resolves and two naming a byte-identity the injector no longer provides.
**Ref:** NOT DERIVED

## 2026-08-29-app-registry-reconciliation — [Corpus Write Arbitration] Accepted residual CLOSED
**Section:** §Established Decisions → [Corpus Write Arbitration]
**Change:** Was an "Accepted residual" (the guard makes the corpus correct but not the app; after an external MCP resolve the running app shows the incident active until restart and its persist cycle declines that row every cycle); now a CLOSED record:
- The 60 s persist cycle reconciles first via the new `triage::contract::DurableActiveIncidents::active_incident_ids` read port — implemented at the `pulse-app` boundary over the same `Arc<dyn CorpusWriter>`, ids only, no new crate edge — resolving registry rows absent from the durable active set through the existing `IncidentRegistry::mark_resolved`, before the write loop.
- A durable-read `Err` reconciles nothing and warns with `persist_kind = "incident_reconcile"`.
- The reconciler deliberately emits no lifecycle event: `pulse://stream/incidents` has producers but no consumer.
**Why:** the residual named its own working-route owner, which is this chunk. Measured live: a real `andromeda-pulse-mcp` subprocess resolved a row and the running app dropped it within two persist cycles with no restart (`reconciled_count` 1 / `declined_count` 0, against 0 / 2 under a mutation removing reconciliation).
**Kept:** the monotonic predicate, its existence probe, the `Applied`/`DeclinedStale` semantics and `load_active_incidents`'s boot-only caller — unchanged.
**Ref:** NOT DERIVED

## 2026-08-29-advisory-backlog — rmcp mechanism recorded as measured + wasmtime requirement-plus-resolved restate
**Section:** §Stack (MCP server row · Plugin runtime row) · §Established Decisions ([MCP Server Surface] · [API Style — MCP Server Surface 3] · [Plugin Runtime]) · §Inherited Defaults (API style — external MCP · Plugin runtime) · §Occupied Resources (MCP stdio surface line · sidecar-binary label) · §Infrastructure Patterns (directory-tree comment · deployment-topology line)
**Change:**
- MCP mechanism amended to measured reality: the canonical surface is the hand-rolled serde JSON-RPC 2.0 layer at `crates/mcp-server/src/jsonrpc.rs` (MCP protocol `2024-11-05`, name-dispatch in `tools.rs`); `rmcp` is a feature-gated ANCHOR dependency — requirement `"3"`, lockfile-resolved 3.1.4 as of 2026-08-29, sole source contact `use rmcp as _;`. The "rmcp 1.5.0 vs published 0.3.x" reconciliation caveat is CLOSED at both its sites.
- wasmtime at all three sites takes the requirement-plus-resolved shape (duckdb precedent): family floor 25+ unchanged and still true, requirement `"46"`, lockfile-resolved 46.0.3 (Cranelift 0.133.3).
- "rmcp sidecar" naming leftovers renamed to the `andromeda-pulse-mcp` sidecar at three registry/topology sites.
**Why:** the chunk bumped rmcp 0.6.4 → 3.1.4 and wasmtime 43.0.2 → 46.0.3 to close RUSTSEC-2026-0189/0222, and its research measured that the rmcp-macro mechanism this doc described was never implemented. Operator resolved the wording escalation to amend-to-measured-reality.
**Kept:** the 8-tool wire surface, the `--features mcp-server` gate and the never-a-custom-envelope ban — unchanged (wire conformance proven by the real-subprocess legs).
**Ref:** NOT DERIVED

## 2026-08-30-npm-advisory-coverage — xtask npm-supply-chain gate registered
**Section:** §Occupied Resources (new "xtask CLI surfaces (dev/CI gates)" entry before §Tauri capability identifiers)
**Change:** Registered `cargo xtask check:npm-supply-chain` (`xtask/src/npm_gate.rs`) with its formalized contract — exit 0 green · 1 findings/policy red · 2 cannot-evaluate; six named verdict arms; one pretty-JSON verdict object on stdout — its policy input `pulse-app/ui/npm-policy.json` (per-class license allowlists + GHSA-scoped advisory exceptions with mandatory provenance + package-name denylist), the lockfile-only design (no `npm ci`, no node_modules), and its ci.yml `supply-chain`-job wiring beside the existing `Cmd::Audit`/`Cmd::DenyBans` verbs.
**Why:** the chunk landed a new formalized xtask CLI contract, which is an arch registry item per the 2026-08-25 formalized-CLI-contract rule.
**Ref:** NOT DERIVED

## 2026-08-30-diagnostics-un-muting-harness-truth-sweep — harness:status registered · dead schema dropped (DuckDB 8→5, corpus 6→5/v2) · 17-stage leg · incidents topic producer-only
**Section:** §Occupied Resources (xtask CLI surfaces · Filesystem `run/andromeda-pulse.pid` · Environment `ANDROMEDA_PULSE_PIDFILE`/`_LOGFILE` · DuckDB Reserved tables · Corpus Reserved tables · Tauri IPC events `pulse://stream/incidents`) · §Conventions (Database entity naming) · §Stack (GUI verification harness row)
**Change:**
- Registered `cargo xtask harness:status` (`xtask/src/harness_status.rs`): verdict JSON `{verdict, pid, log_file_basename, last_write_age_seconds, stale_after_seconds}`, four arms running-healthy/stale/not-running/cannot-evaluate at exits 0/1/1/2, derived out-of-process from the PID file + log-family mtime; it REPLACES the exit-0-unconditional in-process `current_health()` envelope. The pid-file entry and both harness env vars gain the xtask verb as a second reader.
- DuckDB reserved tables 8 → 5: the `span_links`/`resources`/`instrumentation_scopes` CREATEs deleted as dead schema; retention DELETEs 7 → 4; the 2026-08-23 CARRY disposition discharged; §Conventions Database-entity naming re-synced (producerless/permanently-empty-sweep wording retired).
- Corpus reserved tables 6 → 5: `baseline_state` dropped; `SCHEMA_VERSION` 2 with the ladder migration (fresh→v2 · v1→DROP+stamp · newer→mismatch); `incident_events` gains its first production writer (one lifecycle row per status value-change inside the guarded-update transaction).
- §Stack harness row: 16 → 17 stages, `boot-geometry` after `launch` — the driver records outer position/size + monitor and the Rust verdict re-derives the app's snap formula (`wx == mx + mw − round(480×scale) − 24 && wy == my + 24`).
- `pulse://stream/incidents` tagged **producer-only** at the broadcast-channel registry — producers exist, no consumer subscribes, the webview polls `incidents.list_active`.
**Why:** operator forks ruled DROP-both-dead-schema-sets, PERSIST-lifecycle-at-the-choke-point and RECORD-producer-only (2026-08-30); harness:status is an arch registry item per the 2026-08-25 formalized-CLI-contract rule; the 17-stage count and the 5-table sets were verified live.
**Ref:** NOT DERIVED

## 2026-08-30-staged-bindings-assertion — staged-artifacts gate registered; capability-drift's behavior re-pointed
**Section:** §Occupied Resources → xtask CLI surfaces (dev/CI gates) · §Cross-cutting Patterns → Webview IPC capability policy
**Change:**
- Registered `cargo xtask check:staged-artifacts` (xtask CLI surfaces 2 → 3; `xtask/src/staged_gate.rs`): asserts the git-INDEX copies of `pulse-app/ui/src/bindings/index.ts` + all `pulse-app/capabilities/*.json` (a `git ls-files --stage` health probe, then `git show :<path>` per subject) against `EXPECTED_PROCEDURES` and the in-code `staged_gate::EXPECTED_GRANTS` semantic-triple pin `{identifier, windows, permissions}` (6 files; `description`/`$schema`/`local` outside the pin; set equality BOTH directions; staged deletion + unpinned capability file are findings-RED, no neutral arm). Contract: one pretty-JSON verdict on stdout, arms `staged-clean`/`staged-drift`/`cannot-evaluate` at exit 0/1/2, report twin `target/staged-artifacts/report.json`, ci.yml plain named `run:` step directly after capability-drift. `capability-drift` itself runs the staged assertion after its unchanged worktree diff and folds any non-clean staged outcome into its existing FAILURE exit (0/1 contract preserved).
- §Webview IPC capability policy: was a worktree-only diff; now the `EXPECTED_PROCEDURES` pin is diffed against BOTH the worktree and the staged bindings, with staged grants riding `EXPECTED_GRANTS` — a pin edit must be STAGED with the regenerated bindings.
**Why:** the chunk shipped the verb and the `capability_drift()` extension. The policy clause restated the worktree-only mechanism and would have survived a single-site apply, so both sites move together. Live-proven: real repo `staged-clean` exit 0; a clobbered-worktree run exit 1 with the staged half green.
**Ref:** NOT DERIVED

## 2026-08-30-acl-rejection-logging — `telemetry.frontend.record_ipc_rejection` + `ui.ipc.rejection` leaf registered
**Section:** §Occupied Resources → Tauri IPC routes (TauRPC procedures)
**Change:** Registered `telemetry.frontend.record_ipc_rejection` (ui-bridge crate; the 5th `TelemetryApi` method — family roster 4 → 5), the capability-rejected-IPC record security-plan §Logging & Monitoring mandates. Bounded input triple (`IpcRejectionCategory` closed enum `acl_rejected`|`other`, classified webview-side; `coerce_window_label` third bounded copy; `validate_payload_bytes` bound 100_000_000) → target `ui.ipc.rejection` (WARN, once per rejection, no hot path) with its OWN exact allowlist leaf — no bare `ui` key exists. NO capability-JSON change; `EXPECTED_PROCEDURES` +1.
**Why:** the chunk landed the procedure, target and leaf, moving the roster count this registry entry carries; no duplicate roster site exists in arch.
**Ref:** NOT DERIVED

## 2026-08-30-agent-harness-teardown-truth — agent-run CLI contract registered + pid-file entry clause narrowed
**Section:** §Occupied Resources → xtask CLI surfaces (dev/CI gates) · §Occupied Resources → Filesystem locations (`run/andromeda-pulse.pid` entry)
**Change:**
- Registered `scripts/agent-run.{sh,ps1}` as a sibling formalized CLI contract: 5 unchanged verbs; `boot` pre-builds under its own exported env (app release + xtask, absorbing the env-fingerprint relink outside the timed window) and spawns the dev binary BY PATH (no `cargo run` wrapper; 10s default honest, measured 1.953s); `cleanup`'s four bounded verdict tokens with 0/1 exits from independent pid+port probes; the ci.yml Linux-only smoke step now GATING (`continue-on-error` dropped).
- Pid-file entry: was "the scripts reach status only THROUGH that xtask verb"; now narrowed to the status verdict (boot poll + `status` verb) — `cleanup` judges liveness independently, with pidfile CONTENT canonical (the app's `write_pid_file` overwrites the provisional spawn pid; msys ≠ Windows pid space).
**Why:** the chunk shipped the new verdict/exit contract and removed the wrapper; CI gating was operator-approved at P4; the verdict shape follows the harness:status precedent. Routine per the 2026-08-25 formalized-CLI-contract rule and the 2026-07-08 accurate-addition rule; both sites applied atomically.
**Ref:** NOT DERIVED

## 2026-09-29-p-025-hue-shift-observable-made-gradable — wasmtime requirement 46 → 48.0.3
**Section:** §Stack and Technologies → Plugin runtime · §Established Decisions → [Plugin Runtime] · §Inherited Defaults → Plugin runtime
**Change:** Was requirement `"46"`, lockfile-resolved 46.0.3 (Cranelift 0.133.3); now requirement `"48.0.3"`, lockfile-resolved 48.0.3 as of 2026-09-29 (Cranelift 0.135.3, read from the lockfile at this wrap). The Stack row adds that the bump closed RUSTSEC-2026-0316 and that 49.x is out of reach while the toolchain is pinned at 1.95 (49.0.1 needs Rust 1.96). The 25+ family floor is unchanged.
**Why:** the chunk upgraded wasmtime in-chunk to clear its first real CI run's advisory (founder ruling, via the overseer); three body sites restated the old pin.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/

## 2026-09-29-p-025-hue-shift-observable-made-gradable — P-025 hue interval stated at the IPC route
**Section:** §Occupied Resources → Tauri IPC routes → `telemetry.frontend.record_*` delegated-timing entry
**Change:** The `metric.constellation.hue_update_ms` clause now states its `duration_ms`: the paint instant minus the service's `ServiceListItem.tier_effective_at_unix_nano` (replayed by `triage::contract::tier_effective_at` — rise = the opening incident's `opened_at_unix_nano`, fall = the last max-tier holder's `resolved_at_unix_nano`), one record per service whose tier changed, emitted by `hueShiftSamples` only for changes witnessed after mount, no service id. Leaf name and fields unchanged.
**Why:** the chunk re-anchored the observable to the interval the P-025 budget bounds, per Conductor's measurement contract.
**Kept:** the `services.list_with_states` entry — the new `tier_effective_at_unix_nano` payload field is a field inside an already-registered procedure, which this registry does not enumerate (chunk #91's `priority_tier` join was never registered either).
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/

## 2026-09-29-p-025-hue-shift-observable-made-gradable — harness:status derives liveness from the pid
**Section:** §Occupied Resources → xtask CLI surfaces → `cargo xtask harness:status` · §Occupied Resources → Filesystem locations → `run/andromeda-pulse.pid`
**Change:** Was "derived from the pidfile plus the log family's mtime"; now from the pidfile, the liveness of the pid it holds, and the log family's mtime — a dead pid is `not-running` whatever the log says (`ps -o stat=` on Unix, a zombie counts as dead; `tasklist` on Windows). Before it, a crashed app read `running-healthy` for up to 60 s. The verdict JSON and its four arms are unchanged.
**Why:** measured on a CI boot smoke whose app panicked in 12 ms and still read healthy; fixed in-chunk on a founder ruling.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/

## 2026-09-29-p-025-hue-shift-observable-made-gradable — agent-run CI env export + boot failure path; smoke:hue-shift registered
**Section:** §Occupied Resources → xtask CLI surfaces → `scripts/agent-run.{sh,ps1}` · `cargo xtask smoke:hue-shift` (new)
**Change:**
- agent-run: was "the three invocations share the workflow-level `ANDROMEDA_PULSE_DATA_DIR`, ci.yml:12"; now they run inside one `xvfb-run` and share the variable every job exports to `$GITHUB_ENV` right after harden-runner, because `runner.*` is unavailable in workflow- and job-level `env:`. On a failed readiness poll `boot` reports how the app ended (signal name, exit status, or still running) and runs `bash "$0" cleanup`.
- New registration: `cargo xtask smoke:hue-shift` (`xtask/src/hue_shift.rs`), the P-025 scenario leg — grades the rise against `interpretation.incident.created` and the fall against the first `triage.incident.auto_resolve.tick` with `resolved_count ≥ 1`, each on anchor error ≤ 1000 ms; exit 0 PASS · 1 FAIL · 2 INCONCLUSIVE; artifact under `target/hue-shift/`; dev-host only, not CI-wired.
**Why:** the workflow-level form never parsed, so CI had not run for two months; the leg is a new formalized xtask CLI contract, a registry item per the 2026-08-25 rule.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/

## 2026-09-29-ci-wall-time-and-round-trips — seven-job CI, the pre-push verb, the boot end-status recorder
**Section:** §Infrastructure Patterns → CI/CD approach · §Occupied Resources → xtask CLI surfaces · §Occupied Resources → Filesystem locations
**Change:**
- CI/CD: was "matrix over Linux/macOS/Windows; fmt → clippy → xtask test → `cargo build --workspace` (release profile smoke)"; now seven independent jobs — `lint-test` (3-OS matrix; `perf:slo-load` on Linux) · `release` (macOS/Windows) · `mcp-test` (Linux) · `a11y` (3-OS) · `boot` (Linux: mcp-feature release build, boot smoke, `ci-gates`) · `supply-chain` (its auditable build is the Linux release smoke) · `coverage`. Rust caches budgeted against the 10 GB cap: `lint-test` / `boot` own the `lint-test-{os}` / `boot-Linux` target caches and save on failure; `release`, `mcp-test`, `a11y`, `supply-chain` restore read-only; `coverage` caches the registry only.
- xtask CLI surfaces: `cargo xtask pre-push:linux` registered (exit 0 green · 1 red · 2 cannot-evaluate; one JSON verdict `{verdict, reason, head, tree, stages[], missing[], remediation, cache}` + `target/pre-push/report.json`; Windows host, WSL `Ubuntu` via `env -i`; pins read from the repo; missing provisioning → `cannot-evaluate` with one apt line; tree-id-verified clone sync; stages `script-modes` · `npm` · `clippy` · `test` · `ci-gates`; no port, no `pulse-app`, no new env var; cross-project fragility: Node 24 from the Viola repo's distro install). `smoke:gap-resume` (three arms) and `smoke:external-resolve` (`reconciled_count` verdict) registered beside `smoke:hue-shift`, dev-host only. `harness:status` JSON gains `ended` (the `run/andromeda-pulse.exit` record, `exit N` / `signal N (NAME)`, ≤ 48 printable ASCII, only when not `running-healthy`, null under ps1); arms and exit codes unchanged. `agent-run.sh boot` runs the app under a waiting subshell writing `run/andromeda-pulse.{spawn,exit}`, takes the app pid from the spawn record (none in 5 s → exit 1), and its failure path prints `app ended: {record}` (was: the spawn pid IS the app, and a `wait` named the signal); `agent-run.ps1` does not mirror the recorder; the boot smoke is the `boot` job's step.
- Filesystem: `run/andromeda-pulse.spawn` + `run/andromeda-pulse.exit` registered as harness-written (never the product binary); the pidfile passage's provisional pid now comes from the spawn record.
**Why:** CI wall time was the build loop's bottleneck (warm 44.8 min); after a seven-job split and a fix to xtask's child-cargo environment the warm round measured 25.5 min. The recorder exists because an app that dies after `boot: ready` is an orphan whose end status nothing else can read; it is the Linux boot watch's instrument. A proposed §Stack row for the WSL distro was rejected as registry over-reach — the fragility rides the verb's registration.
**Ref:** .andromeda/runs/2026-09-29T21-44-34Z-wrap/

## 2026-09-30-dual-license — [License] decision; crate counts; the capability-drift slot
**Section:** §Established Decisions → [License] (new) · §Design Philosophy → Single-process modular monolith · §Infrastructure Patterns → runtime topology · §Occupied Resources → xtask CLI surfaces (the staged-assertion sentence)
**Change:**
- New [License] decision: `MIT OR Apache-2.0`, holder Turbolet85. `LICENSE-MIT` + `LICENSE-APACHE` at the root, LF. Set once at `Cargo.toml [workspace.package] license`; every member inherits via `license.workspace = true`, none overrides. The npm manifest and its lock root carry it. Homebrew `license any_of: ["MIT", "Apache-2.0"]`, Scoop `"license": "MIT|Apache-2.0"`. Tauri `bundle.license` inherits the Cargo value. cargo-deny checks own crates. Pinned by the test-only `xtask` module `license_check`. Out of the decision: per-file headers, a NOTICE bundle, `authors`.
- Counts: was "twelve library crates … fourteen workspace members total"; now fourteen library crates, sixteen members (§Occupied Resources stays canonical). Runtime topology: was "all twelve library crates"; now fourteen.
- xtask CLI surfaces: was "rides the slot that runs LAST in every gate list"; now the `capability-drift` slot, which runs BEFORE the default-features workspace nextest since 2026-09-30.
**Why:** the founder's license decision of 2026-09-29 (the repository public that day), in the Conductor `cdb7082` shape; the counts had gone stale against the 16-member list; the gate order moved by the overseer's founder-delegated directive (test-plan §3).
**Kept:** "12-module" inside the [Backend Framework] / [Tauri IPC Bridge] / [Module Boundaries] rationales — decision-time reasoning, not a current count.
**Ref:** .andromeda/runs/2026-09-30T07-44-36Z-wrap/
