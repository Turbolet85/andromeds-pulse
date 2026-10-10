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

## 2026-09-30-p-027-discovery-bound — smoke:discovery registered; the P-027 discovery_ms anchor stated
**Section:** §Occupied Resources → xtask CLI surfaces · §Occupied Resources → Tauri IPC routes (the delegated-timing entry, P-027)
**Change:**
- xtask CLI surfaces: new `cargo xtask smoke:discovery` (`xtask/src/discovery.rs`), the P-027 scenario leg: boot the release binary on a fresh data dir, wait (≤ 60 s) for `services.list_with_states.request`, start `inject_demo --sustained --error-pct=0`, grade the first `metric.constellation.discovery_ms` at or after the first `duckdb.append {table_name: spans}` in a 30 s window — PASS iff interval ≤ 5000 ms, anchor within 1000 ms of that append, 0 `app.panic.fatal`, 0 ERROR; exit 0 PASS · 1 FAIL · 2 INCONCLUSIVE; artifact `target/discovery/`; dev-host only, not CI-wired. Was "the two sibling scenario legs" (gap-resume, external-resolve); now "the two remaining sibling scenario legs".
- Delegated timing: was a bare `→ metric.constellation.discovery_ms (P-027)`; now its anchor — `duration_ms` = paint instant − `ServiceListItem.last_seen_unix_nano`, stamped at a brand-new service's FIRST SIGHTING by `DiscoveryObserverAdapter` (composed after the baseline adapter) via `ServiceRegistry::register_first_sighting`, before the heartbeat's first 15 s tick, so the first-appearance sample measures first-seen-to-dot; a service re-entering liveness later is anchored on a tick-refreshed `last_seen` and is not a first discovery.
**Why:** the chunk registered a service at its first sighting instead of at the 15 s registry tick, and its measurement showed the old anchor hid the wait (a 435 ms sample over a 15 219 ms true interval at the base; 177 ms, anchor error 6 ms after).
**Ref:** .andromeda/runs/2026-09-30T11-29-23Z-wrap/

## 2026-09-30-perf-budget-gate-reads-real-samples — perf:budget and perf:frame-sample; the frame leg's env var; release owns its cache
**Section:** §Occupied Resources → xtask CLI surfaces · §Occupied Resources → Environment variables · §Infrastructure Patterns → CI/CD approach
**Change:**
- Registered `cargo xtask perf:budget --data-dir <DIR> --require <arm,arm>` (grades every `agent-latest.jsonl*` member; nearest-rank p99; Unreadable or required-empty fails; exit 0/1/2; a non-required empty frame arm prints the named cannot-evaluate line). `ci-gates` and `perf:load-profiles` use it in-process with no arm required; the `perf-slo-check` scripts are deleted.
- Registered `cargo xtask perf:frame-sample` — Windows dev-host frame gate, exit 0 PASS / 1 FAIL / 2 INCONCLUSIVE, artifact `target/perf-frame/`, NOT CI-wired.
- Env var `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS`: harness-only, SET on the frame leg's app child only, never read by the product, never in product config.
- CI/CD: lint-test Linux adds the perf-samples producer → `perf:budget --require memory,snapshot` → upload; `release` owns `release-{os}` (was a read-only restore of lint-test's key) and has no frame boot step; 8 cache entries, 10 605 172 169 of the 10 737 418 240 B cap (a watch).
**Why:** new CLI verbs and an env var are registry resources; the frame leg left CI after the hosted runner exposed no WebGPU adapter (operator decision). Release was going cold whenever lint-test re-saved without release dependencies.
**Ref:** .andromeda/runs/2026-09-30T15-36-30Z-wrap/

## 2026-09-30-perf-budget-gate-reads-real-samples — agent-run.ps1 writes the boot end-status records
**Section:** §Occupied Resources → xtask CLI surfaces (`scripts/agent-run.{sh,ps1}`, `harness:status` `ended`) · §Occupied Resources → Filesystem locations (`run/andromeda-pulse.spawn` + `.exit`)
**Change:** was "`agent-run.ps1` does not mirror the recorder" and "`ended` null under `agent-run.ps1`"; now the ps1 `boot` hidden `powershell -EncodedCommand` wrapper writes the app pid to `.spawn` and `exit N` (ASCII, no BOM) to `.exit`, with the ≤ 5 s spawn poll, the no-spawn-record exit 1 and the ended / still-running diagnosis; `ended` is real under both scripts (`exit -1` after `Stop-Process -Force`). Both files remain harness-written, never by the product.
**Why:** the chunk discharged the CARRY that left Windows `harness:status` unable to say how the app ended.
**Kept:** the five verbs, exit semantics and status/cleanup field set.
**Ref:** .andromeda/runs/2026-09-30T15-36-30Z-wrap/

## 2026-09-30-perf-instruments-measure-their-budgets — telemetry.frontend.record_webgpu_adapter registered
**Section:** §Occupied Resources → Tauri IPC routes
**Change:** NEW `telemetry.frontend.record_webgpu_adapter` — the 6th `TelemetryApi` method (roster 5 → 6): `WebgpuAdapterInput { outcome: WebgpuAdapterOutcome (closed serde enum of 5; unknown rejected), window_label (coerced 4 + unknown) }` → `ui.webgpu.adapter` {outcome, window_label} behind its own exact leaf; no capability-JSON change; `EXPECTED_PROCEDURES` 43 → 44; its only live witness the dev-host frame leg. The body quotes the founder's ratification at P4 2026-09-30, «Да, делай».
**Why:** every added TauRPC procedure needs its Occupied Resources entry; this one is a Boundary widening, ratified by the founder at P4.
**Ref:** .andromeda/runs/2026-09-30T19-50-57Z-wrap/

## 2026-09-30-perf-instruments-measure-their-budgets — perf-budget frame line names a cause
**Section:** §Occupied Resources → xtask CLI surfaces (`perf:budget` · `perf:frame-sample`)
**Change:** An empty frame arm names its cause via `xtask::perf_budget::frame_cause` over the same log's `ui.webgpu.adapter` records: unrequired → `frame: cannot-evaluate: 0 samples, {cause}`; `--require`d → `frame NEUTRAL — {cause} (required) FAIL`; `perf:frame-sample` 0-sample → `frame: 0 samples — {cause}`. Exit codes and the nearest-rank rule unchanged. Was: the fixed `frame: cannot-evaluate: 0 samples, no WebGPU adapter in this run`.
**Why:** the registered CLI contract stated retired output text.
**Ref:** .andromeda/runs/2026-09-30T19-50-57Z-wrap/

## 2026-09-30-perf-instruments-measure-their-budgets — release cache saves on failure
**Section:** §Infrastructure Patterns → CI/CD (Rust cache budget)
**Change:** `release-{os}` saves on failure too (`cache-on-failure: true`), like the `lint-test` and `boot` owning keys. Re-read after the change on `ci#36765040464`: byte-identical, 8 entries, 132 246 071 B (1.23 %) headroom, no key evicted — still a watch.
**Why:** a red release round had saved nothing, so the next rebuilt cold; the same key re-saves and no lockfile moved, so the change adds no cache growth.
**Ref:** .andromeda/runs/2026-09-30T19-50-57Z-wrap/

## 2026-09-30-span-level-redaction — span-masked identity and egress
**Section:** §Conventions → Primary key convention · §Occupied Resources → Out-of-data-dir egress sink (training export)
**Change:** Primary key convention — was: since the metric name is scrubbed at ingestion, two DISTINCT credential-shaped names redact to one placeholder and collide; now: the metric name is span-masked (`mask_secret_spans`), so two names collide only when they differ solely inside a masked span (a single-token credential-shaped name still masks whole; a multi-word name keeps its non-secret words); `seq` stays the disambiguator. Training-export egress — was: PII scrubbed via `scrub_attribute`; now: span-masked via `mask_secret_spans`, the `interpretation` field (a serialized `L4Output`) masked per JSON string leaf so it stays parseable, non-JSON text masked whole, and `count_redactions` counting every field CONTAINING a `[redacted:` placeholder.
**Why:** both sites named the retired whole-value mechanism. A Boundary widening ratified by the founder at P4 2026-10-01, «Ок давай по типу правила».
**Kept:** §Established Decisions [Fault Identity] — it states no derivation of `scope_id` from the scrubbed service name, so it carries nothing to amend; the L1 fingerprint stays computed from the raw stacktrace.
**Ref:** .andromeda/runs/2026-10-01T11-19-47Z-wrap/

## 2026-10-01-real-model-incident-surfacing — the English-only source gate registered; pre-push gains its sixth stage
**Section:** §Occupied Resources → xtask CLI surfaces (dev/CI gates) — the `check:english-sources` entry (new) and the `pre-push:linux` stage list
**Change:**
- Registered `cargo xtask check:english-sources` (`xtask/src/source_lint.rs`), a formalized CLI contract:
  - scans `crates` · `pulse-app/src` · `pulse-app/tests` · `pulse-app/ui/src` · `xtask/src` (`.rs`/`.ts`/`.tsx`) for U+0400–U+04FF;
  - one `::error file=…,line=…::` annotation per hit, every non-ASCII character rendered `\u{XXXX}`, so the whole stdout is ASCII;
  - then a `{verdict, hits, files_scanned}` JSON verdict with a twin `target/english-sources/report.json`;
  - exit 0 `clean` · 1 `findings` · 2 `cannot-evaluate` (never a pass);
  - wired as ci.yml `lint-test`'s "No Cyrillic in sources" step on all three OSes and as `pre-push:linux`'s `source-lint` stage.
- `pre-push:linux` stage list: was five (`script-modes` · `npm` · `clippy` · `test` · `ci-gates`); now six, with `source-lint` second after `script-modes`.
**Why:** the CI check had been an inline `shell: python` step that crashed with `UnicodeEncodeError` under the Windows runner's ANSI code-page pipe whenever it had a hit to print, and the local pre-push gate never ran it — so a Cyrillic literal passed pre-push green and failed CI on every `lint / test` job. One xtask verb now serves both, ASCII by construction.
**Ref:** .andromeda/runs/2026-10-01T18-16-18Z-wrap/

## 2026-10-02-incident-events-readable-through-mcp — MCP roster grows to nine with retrieve_incident_events
**Section:** §Stack and Technologies (MCP server row) · §Established Decisions → [MCP Server Surface] · §Conventions → External wire — MCP server · §Standard Contracts → MCP server — spec-conformant
**Change:** was an 8-tool wire surface; now 9 — `retrieve_incident_events` joins the four telemetry and four chunk #94 incident tools, all dispatched through the single `dispatch_tool` site. Its contract: input `{incident_id: integer}`, `additionalProperties: false`; response `{incident_id, events: [{event_kind, occurred_unix_nano}], total, truncated}`, oldest first, bounded at `INCIDENT_EVENTS_READ_LIMIT` = 256 with `truncated` past it; `event_kind` coerced on egress to `triage::contract::incident_event_kinds()`, any other value reading `unknown`; an unknown id → JSON-RPC -32603 `incident not found`. The hand-rolled JSON-RPC decision itself is unchanged.
**Why:** the chunk made the `incident_events` lifecycle table readable through MCP so the P-075 round's content fidelity covers it; the new read surface is a boundary widening the founder ratified at P4 (2026-10-02), and the shared event vocabulary the founder's option A (2026-10-03).
**Ref:** .andromeda/runs/2026-10-03T23-46-09Z-wrap/

## 2026-10-02-incident-events-readable-through-mcp — incident_events writer census corrected and first reader registered
**Section:** §Occupied Resources → Corpus SQLite database → Reserved tables (`incident_events`)
**Change:** was "`incident_events` gained its first production writer" at `2026-08-30-diagnostics-un-muting-harness-truth-sweep` (`update_incident_status` alone, `event_kind` = the new status label); now TWO production writers — the producer's creation event (`create_incident_from_l4_output` writes `INCIDENT_EVENT_CREATED` = `created` through `IncidentPersistence::save_incident_event` → `CorpusWriter::save_incident_event`, since chunk #92) and `update_incident_status` (one row per status VALUE-change; same-status refresh and `DeclinedStale` record nothing) — over the closed four-kind vocabulary `incident_event_kinds()` = `created` / `active` / `acknowledged` / `resolved`, empty encrypted payload; ONE production reader — the sidecar's `retrieve_incident_events` via `CorpusWriter::load_incident_events(incident_id, limit)` → `IncidentEventRow { event_kind, occurred_unix_nano }`, a prepared `?1`/`?2` `ORDER BY id ASC LIMIT` read never selecting `payload`. No DDL; `SCHEMA_VERSION` 2.
**Why:** the earlier census was taken at the corpus layer and missed the trait-level writer one layer up; the chunk's first read-back exposed it when the producer's `created` row read `unknown`. A writer census needs the trait-level grep (`save_incident_event`), not only the SQL one.
**Ref:** .andromeda/runs/2026-10-03T23-46-09Z-wrap/

## 2026-10-04-supply-chain-advisories-on-wasmtime-resolved — wasmtime requirement 48.0.3 → 48.0.4, resolved 48.0.5
**Section:** §Stack and Technologies → Plugin runtime · §Established Decisions → [Plugin Runtime] · §Inherited Defaults → Plugin runtime
**Change:** Was requirement `"48.0.3"`, lockfile-resolved 48.0.3 as of 2026-09-29 (Cranelift 0.135.3), with "49.0.1 needs Rust 1.96" (per the 2026-09-29-p-025-hue-shift-observable-made-gradable — wasmtime requirement 46 → 48.0.3 entry, whose RUSTSEC-2026-0316 closure still stands as the earlier bump); now requirement `"48.0.4"`, lockfile-resolved 48.0.5 as of 2026-10-04 (Cranelift 0.135.5 at `Cargo.lock:1393`) at all three sites. The Stack row adds that the bump from 48.0.3 closed RUSTSEC-2026-0325 / -0326 / -0327 with no advisory ignore, and that 49.x stays out of reach while the toolchain is pinned at 1.95 (49.0.2, the first patched 49.x, needs Rust 1.96). The 25+ family floor and the [Plugin Runtime] decision are unchanged.
**Why:** the three advisories against 48.0.3 turned the CI supply-chain job red; the founder ruled a source fix with no ignore. The requirement records the advisories' stated floor and the lockfile takes the newest compatible patch by the ordinary `cargo update -p` (founder ruling at plan review, 2026-10-04) — the two numbers differ, so both are stated.
**Ref:** .andromeda/runs/2026-10-04T02-20-23Z-wrap/

## 2026-10-04-corpus-key-creation-is-race-free — locked corpus-key creation; the lock file is the second data-dir escape
**Section:** §Occupied Resources → Corpus SQLite → At-rest posture · §Occupied Resources → Filesystem locations (new "Out-of-data-dir lock file (corpus-key creation)" bullet; the training-export sink bullet)
**Change:**
- At-rest posture: concurrent first creation converges on one key — `OsKeychainBackend::fetch_from_os_store` is a locked create-or-read (exclusive `std::fs::File::lock` on the entry's lock file, then get → only on `NoEntry` generate → set → read back → return the read-back key → unlock); no unlocked generate-and-set path remains. Measured: 8 re-exec processes → 1 distinct key (8 distinct, 7 not stored on the prior code). A lock failure maps to `KeychainError::Unavailable` / `Failed` → `Error::KeyringUnavailable`, degrading like an unreachable store.
- Filesystem locations: registers `andromeda-pulse-corpus-key-{h16}.lock` (first 16 hex of BLAKE3 over service ‖ 0x00 ‖ account) — per-user, content-free, mode 0600, never deleted, written by the app and the MCP sidecar through `Corpus::open`; in `$XDG_RUNTIME_DIR` on Linux, else `std::env::temp_dir()`; a set-but-unusable `XDG_RUNTIME_DIR` fails closed. Four residuals stated in the body (XDG disagreement, shared-`/tmp` denial, passphrase degrade, no lock timeout).
- The training-export sink was "the ONE deliberate exception" to data-dir confinement; now "one of TWO", the other being the lock file.
**Why:** concurrent first-run processes each minted a key and the last write won, so a losing process's rows were undecryptable to its peers. The lock is keyed on the credential entry, not the data dir, because racing processes may resolve different data dirs. A boundary widening the founder ratified live on 2026-10-04 (relayed by the overseer), confirmed by the operator at this wrap.
**Ref:** .andromeda/runs/2026-10-04T09-16-41Z-wrap/

## 2026-10-04-corpus-key-creation-is-race-free — the stated Rust floor matches the code
**Section:** §Stack and Technologies → Primary language / runtime · §Established Decisions → [Primary Language] · §Infrastructure Patterns → directory tree (`rust-toolchain.toml` comment) · §Inherited Defaults → Language / runtime
**Change:** was "Rust 2024 edition (rustc 1.84+)" at all four sites; now: toolchain pinned to rustc 1.95.0 (`rust-toolchain.toml`), code floor rustc ≥ 1.89 (`std::fs::File::lock`; let-chains already needed 1.88), the workspace's declared `rust-version = "1.85"` stale and owned by the route entry "The declared Rust floor matches the code".
**Why:** clippy `incompatible_msrv` measured the declared floor false at this chunk's first gate run; the founder ruled the Cargo.toml raise its own route entry (2026-10-04), so the masters state the truth and its owner meanwhile.
**Kept:** §Standard Contracts `app_info` example `"rust_version": "1.84.0"` — illustrative; what `app_info` reports was not measured, so the value question rides the same route entry.
**Ref:** .andromeda/runs/2026-10-04T09-16-41Z-wrap/

## 2026-10-04-linux-launch-stays-up-on-nvidia-wayland — the Linux NVIDIA launch-posture default registered
**Section:** §Occupied Resources → Environment variables (a new `__NV_DISABLE_EXPLICIT_SYNC` entry; the `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS` entry)
**Change:** registered `__NV_DISABLE_EXPLICIT_SYNC` — a SYSTEM variable read only by NVIDIA's driver, not an `ANDROMEDA_PULSE_*` input: on Linux the product presence-reads it and, when absent, sets it to `1` from `render_posture::apply_linux_default()`, the first statement of `main()`, ahead of the tokio multi-thread runtime build (`unsafe set_var` under a single-thread SAFETY precondition); the value is never parsed or logged; a preset value (empty included) is honoured, never overwritten; macOS / Windows untouched; NVIDIA-scoped by construction, no detection probe; WebKitGTK children inherit it; reported once per boot on `app.boot.render.posture {posture, lever}`; residual — a host presetting `0` dies as before. The `WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS` entry keeps its harness-only status and its out-of-product-config ban, now stating that this Linux default is the product's one sanctioned launch-posture env default and licenses no WebView2 or other GPU flag in product code or `tauri.conf.json`.
**Why:** the default Linux launch died ~2.0 s into boot on NVIDIA + native Wayland (GDK `Error 71` → `_exit(1)`, 3/3) and stayed up 60 s with this default applied, measured on one host; the four surviving levers shared one factor, removing NVIDIA's EGL explicit-sync path. A boundary widening the founder ratified live on 2026-10-04 (remedy option 1; the DMA-BUF-renderer lever and a detection probe rejected), recorded as such at this wrap by the overseer.
**Ref:** .andromeda/runs/2026-10-04T12-30-22Z-wrap/

## 2026-10-04-retry-storm-interpretation-names-its-cause — the incident title is cue-grounded in every L4 mode
**Section:** §Occupied Resources → `ANDROMEDA_PULSE_L4_DETERMINISTIC`
**Change:** The entry now states that the incident title is cue-grounded in EVERY mode, this one included.
- The producer prefixes the triggering cue kind's closed ASCII cause label (`triage::contract::cue_cause_label` — `Error-rate spike` · `Latency regression` · `Restart event` · `Service went silent` · `Retry storm` · `Reflection trend`) onto the model title BEFORE the scrub.
- So `Incident.title` and the `title` inside `resolution_summary_text` read `{Cause label}: {model title}` at creation, on the dedupe refresh (the JSON only; a deduped incident's own title is not rewritten) and in the resolution final write.
- Under this mode a storm incident reads `Retry storm: Deterministic verification incident` on the report header, Findings rows, MCP `query_incident_list` / `retrieve_report` and the digest's CORPUS MATCHES lines.
- Rows written before the chunk stay unprefixed (no migration).
- The canned rank-1 hypothesis names a retry storm for every incident, so only the title discriminates the cause in this mode.
**Why:** The model can drop the triggering cause (Conductor's d3 interpretation named no retry for a retry-storm-born incident), and the product had no deterministic field naming it. The founder ruled on 2026-10-04 (relayed by the overseer): ship the deterministic cause, in the title, at the producer, with existing rows left unprefixed. External harvests (Conductor) now observe the prefixed string; the overseer measured that no live Conductor code compares the title for equality.
**Ref:** .andromeda/runs/2026-10-04T14-54-42Z-wrap/

## 2026-10-04-retry-storm-interpretation-names-its-cause — Fault Identity: the cue kind reaches the incident text, never its identity
**Section:** §Established Decisions → [Fault Identity — what makes two faults ONE fault]
**Change:**
- Added: the cue KIND, not only its fingerprint, now reaches the incident TEXT. The producer writes `Incident.title`, and the `title` inside the persisted L4 JSON, as `{cue_cause_label(kind)}: {model title}`, a closed ASCII label grounded before the scrub (a reflection incident's synthetic `ReflectionTrend` included).
- Identity is unchanged: the tuple `(kind, scope, scope_id)`, the coalesce predicate, severity, priority tier and the evidence union all stand.
- A deduped incident keeps its creation title, which is coherent because `kind` is part of the identity key.
- The model-authored symptom, timeline and ranked hypotheses are untouched.
**Why:** An incident must name its trigger on every surface whatever the model wrote (founder ruling 2026-10-04, relayed by the overseer). Text and identity are kept separate so that the title never becomes a dedupe key.
**Kept:** Fingerprint stays out of the identity key (possible-but-declined). The model layer's remedy (a prompt that names the triggering cue and frames corpus matches as past/other incidents) is its own route entry, not this decision.
**Ref:** .andromeda/runs/2026-10-04T14-54-42Z-wrap/

## 2026-10-04-l4-hardware-probe-finds-cuda-on-arch-layout-hosts — the hardware-profile override registered
**Section:** §Occupied Resources → Environment variables (reserved at arch level)
**Change:** registers `ANDROMEDA_PULSE_HARDWARE_PROFILE`, product-consumed by `crates/interpretation/src/hardware.rs` (`ENV_HARDWARE_PROFILE_OVERRIDE`), read once per `HardwareProfileDetector::new()`: trim + lowercase, then a closed match over `gpu_primary` · `gpu_fallback` · `cpu_primary` · `cpu_fallback` (snake or kebab); any other value → one WARN on `interpretation.hardware.detect` and real detection; not a path, never logged by value; the only route to `GpuFallback`. The bullet also states real detection's Linux arm: `libcuda.so` or `libcuda.so.1` under `/usr/lib/x86_64-linux-gnu`, `/usr/local/cuda/lib64`, `/usr/lib` or `/usr/lib64` (was the first two dirs and `libcuda.so` only), a fixed-path presence check.
**Why:** registry completeness — a product-consumed input absent from the list since chunk #82; the var is unchanged. Applied on the plan's recorded expected amendment, the env-var registration rule not governing (its precondition, a var the chunk added, is false).
**Ref:** .andromeda/runs/2026-10-04T17-56-08Z-wrap/

## 2026-10-04-l4-interpretation-names-its-triggering-cue — Fault Identity: the model layer is framed at prompt composition
**Section:** §Established Decisions → [Fault Identity — what makes two faults ONE fault]
**Change:** The clause "the model-authored symptom, timeline and ranked hypotheses are untouched (a prompt-side remedy for the model layer is its own route entry)" was retired, as was that remedy's deferral in the `Kept` of "2026-10-04-retry-storm-interpretation-names-its-cause — Fault Identity: the cue kind reaches the incident text, never its identity". The body now says:
- The producer still leaves those fields as the model wrote them.
- The model layer is framed at prompt composition. `triage::digest::render_payload` renders a `TRIGGER: {cue_cause_label(kind)}` line, keyed on `cues.first()` (the cue the identity is taken from), directly after `OVERALL:`, and none without a cue.
- When corpus matches exist, a static note under the unchanged `CORPUS MATCHES:` header frames them as other or past incidents, context only.
- `interpretation::prompt::TRIGGER_FRAMING_INSTRUCTION` (ASCII) sits after `CITING_INSTRUCTION` in all three tiers' Output Instructions. It says the title, symptom and first hypothesis describe the TRIGGER signal, never a corpus match.
- Prompt lineage is v2.4 / v1.3-fallback / v1.3-reflection.
- This is framing, not identity: the tuple, the coalesce predicate and the title grounding are unchanged.
- Its effect on the rank-1 hypothesis is UNMEASURED. The pre-registered series (`l4_decision_probe --min-rank1 36`: rank-1 names the retry in ≥ 36/40, `nf` recorded) is gated on a Linux llama.cpp CUDA binary.
**Why:** This chunk is the route entry the clause deferred to. The framing lives in shared composition, never in a runner impl. Applied on the plan's recorded expected amendment, with the scope ruled by the overseer (founder-delegated, 2026-10-04): the TRIGGER line carries the kind label only, never `scope_id`.
**Kept:** No `triggering_cue` builder parameter (~50 call sites). The dead `# Corpus Retrieval` prompt sites are untouched, because production passes `""` there.
**Ref:** .andromeda/runs/2026-10-04T21-03-56Z-wrap/

## 2026-10-04-declared-rust-floor-matches-the-code — the declared Rust floor is 1.95, equal to the pin
**Section:** §Stack and Technologies → Primary language / runtime · §Established Decisions → [Primary Language] · §Inherited Defaults → Language / runtime · §Standard Contracts → `app_info` example
**Change:**
- Was "the workspace's declared `rust-version = "1.85"` stale and owned by the route entry 'The declared Rust floor matches the code'", per "2026-10-04-corpus-key-creation-is-race-free — the stated Rust floor matches the code"; now the workspace declares `rust-version = "1.95"` once in `[workspace.package]`, all 16 members inherit it.
- The floor is set by the resolved dependency graph: 28 packages declare `rust-version = "1.95.0"` (wasmtime 48.0.5 and its internal crates, cranelift 0.135.5, pulley 48.0.5), so no toolchain below 1.95.0 builds the product. rustc ≥ 1.89 (`std::fs::File::lock`; let-chains 1.88) stays, restated as the workspace's OWN-code bound, never the floor.
- The xtask test `declared_floor_equals_the_pinned_channel` holds the declared floor equal to the pinned channel at major.minor, so every pinned-toolchain build proves it.
- `app_info` example `"rust_version": "1.84.0"` → `"1.95"` — the field is `env!("CARGO_PKG_RUST_VERSION")`, the declared string verbatim; this settles the value question that entry's Kept left to this route entry.
**Why:** This chunk is the route entry those clauses deferred to. The floor witness's strength ("declared == pinned channel", over "≥ every dependency" and "none") was chosen by the overseer (founder-delegated, 2026-10-04) at P4: an overstated floor costs nobody, because Pulse ships an app, not a published crate.
**Kept:** `rust-toolchain.toml` `channel = "1.95.0"` and every dependency unchanged (wasmtime 49 needs Rust 1.96).
**Ref:** .andromeda/runs/2026-10-04T22-34-15Z-wrap/

## 2026-10-04-l4-framing-measured-on-the-real-model — Fault Identity: the trigger framing measured, FAIL 30/40
**Section:** §Established Decisions → [Fault Identity — what makes two faults ONE fault]
**Change:** Was: the framing's effect on the model's rank-1 hypothesis was "UNMEASURED", with the pre-registered series "gated on a Linux llama.cpp CUDA binary", per "2026-10-04-l4-interpretation-names-its-triggering-cue — Fault Identity: the model layer is framed at prompt composition". Now the body says the effect FAILS the pre-registered bar, measured on Llama-3.2-3B-Instruct-Q4_K_M through llama-cli b9305 CUDA `-ngl 99`, routed by detection, n = 10 per shape over the four retry-storm shapes S1–S4 (S4 alone carries a corpus match).
- shipped `--min-rank1 36`: `FAIL · rank1 30/40` (S1 9 · S2 6 · S3 5 · S4 10; elsewhere 5 · none 5 · unparsed 0).
- no-framing `nf`: 18/40 (S1 6 · S2 6 · S3 1 · S4 5).
- Each arm ran once and was never re-run or re-thresholded.
- The framing stays shipped. The shortfall lies in S2 and S3, which carry no corpus match, and is owned by its own 0.3.0 route entry.
- The tuple, the coalesce predicate and the title grounding are unchanged.
**Why:** The block cleared and the pre-registered series ran. Under the P4 ruling (overseer, founder-delegated, 2026-10-04), a FAIL completes the measuring chunk, recorded as measured and never as passed. The remedy stays inside 0.3.0 under the founder ruling of 2026-10-02. Standing trap: S4 reading 10/10 means the corpus-match restatement hypothesis does not account for this FAIL. A remedy aimed only at corpus-match framing would miss the measured shortfall.
**Kept:** The framing itself, because it raised rank-1 retry naming from 18 to 30 of 40 and the corpus-match shape from 5 to 10 of 10. The probe's still-owed flag-parse pins (test-plan §1) are left to that row.
**Ref:** .andromeda/runs/2026-10-04T23-17-01Z-wrap/

## 2026-10-04-l4-rank-1-hypothesis-names-the-retry-on-every-storm-shape — Fault Identity: the reworded framing measured, FAIL 34/40
**Section:** §Established Decisions → [Fault Identity — what makes two faults ONE fault] (the framing clause and its measured effect)
**Change:** `TRIGGER_FRAMING_INSTRUCTION` (ASCII, kind-generic, conditional on a TRIGGER line) now also obliges the first hypothesis statement to name the TRIGGER line's signal in that line's own words, and makes any other abnormal metric on the same service a cause or effect of that signal, never a separate first hypothesis. Prompt lineage was v2.4 / v1.3-fallback / v1.3-reflection; now v2.5 / v1.4-fallback / v1.4-reflection. The measured effect was `FAIL · rank1 30/40` with the shortfall in S2 and S3, owned by its own 0.3.0 route entry (per "2026-10-04-l4-framing-measured-on-the-real-model — Fault Identity: the trigger framing measured, FAIL 30/40", kept as the predecessor's history); now the two-slot series on the probe's synthetic storms:
- selection over S1–S4: rank1 v2.4 28/40 · reworded framing 37 · conventions sentence 36 · schema-description sentence 33 — the reworded framing shipped as the first qualifier in least-blast-radius order, no combination run;
- confirmation on the fixed tree: `FAIL · rank1 34/40` (S1 9 · S2 9 · S3 7 · S4 9), held-out S5/S6 20/20;
- record-only stem reading 29/40 on the v2.4 arm and 36/40 at confirmation: the grader accounts for 1 of 40 on v2.4;
- S3 stays the weakest shape; the remainder is owned by the L4 model-replacement route entry.
**Why:** The plan's decision rule, fixed before any run, selected the lever by measurement, and the confirmation slot is the acceptance; a FAIL is recorded as measured, never as passed (the 2026-10-04 P4 ruling, overseer, founder-delegated). The remainder's owner follows the founder ruling of 2026-10-05, relayed by the overseer, to replace the L4 model with a small current one chosen by measurement. Every candidate text stays kind-generic, so a remedy is never a word plant the grader rewards.
**Kept:** The identity tuple, the coalesce predicate, the title grounding and the kind-label-only TRIGGER line; the verdict grader (the substring `retry`), frozen after a seen FAIL — the stem reading is record-only.
**Ref:** .andromeda/runs/2026-10-05T06-01-47Z-wrap/

## 2026-10-05-l4-runs-on-a-small-current-model-chosen-by-measurement — LLM runtime: argv constants, the model, the grammar trap, the CPU route retired
**Section:** §Established Decisions → [LLM Inference Runtime — L4 interpretation layer] · §Stack → AI/ML serving row · §Occupied Resources → env vars → `ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH` · §Inherited Defaults → LLM inference runtime
**Change:**
- The argv was `-n` / `-st` / `-ngl` plus the outer timeout; now every invocation also carries `-c 8192` (`LLAMA_CLI_CTX_SIZE`) and `-rea off` (`LLAMA_CLI_REASONING`) right after the `-ngl` pair, first-party constants (`build_llama_cli_args`, signature unchanged).
- The entry named no model; now the shipped GGUF is Llama-3.2-3B-Instruct-Q4_K_M, and the naming series confirmed Qwen3.5-2B-Q4_K_M at `PASS · rank1 37/40` (bar 36, held-out 19/20; gemma-4-E2B and Nemotron 3 Nano 4B record-only 40/40 + 20/20) through a `--grammar-file` GBNF with per-model authors' sampling. The pattern-discrimination route entry chooses the model; the entry after it ships the founder's pick with its authors' sampling, a GBNF and thinking off — not automatically Qwen3.5-2B.
- The b9305 grammar-prefill trap: a `--json-schema-file` grammar is prefilled with a thinking template's generation prompt, so qwen35 / gemma4 GGUFs print `Failed to initialize samplers` on stdout, exit 0 and emit no JSON; `-rea off` does not prevent it; a `--grammar-file` grammar is never prefilled.
- The CPU route (CPU-primary / CPU-fallback → CPU build + `-ngl 0`) was a supported L4 tier; now it is founder-retired — every model over 30 s per generation on it (Llama max 33,968 ms; two candidates timed out at 60 s) — and still routed in code until the programmatic-L4 route entry; without a GPU, L4 analysis is programmatic and hands off as markdown or over MCP. Stated at all four routing sites.
**Why:** The chunk shipped the two constants and measured the candidates. The CPU route's disproval was ruled by the founder (2026-10-05, relayed by the overseer; restated by the founder at this wrap) and is recorded as measured with its owner, its implementation half being its own entry. Trap for later chunks: a thinking-template GGUF under the json-schema argv yields no interpretation and no visible error.
**Kept:** The D1 choice, the b9305 pin, the hardware-profile detection and its labels, and the json-schema mechanism for the shipped Llama template; `CUDA_VISIBLE_DEVICES` (set only inside operator-leg commands) stays out of the env-var registry.
**Ref:** .andromeda/runs/2026-10-05T09-41-57Z-wrap/

## 2026-10-05-l4-runs-on-a-small-current-model-chosen-by-measurement — Fault Identity: the rank-1 remainder rides the model
**Section:** §Established Decisions → [Fault Identity — what makes two faults ONE fault] (the remainder's owner)
**Change:** The remainder of the framing's FAIL (34/40 on Llama) was owned by the L4 model-replacement route entry; now it rides the L4 model, not the framing: the replacement chunk re-ran the same S1–S4 shapes and strict grader on candidate models under v2.5 and confirmed Qwen3.5-2B-Q4_K_M at `PASS · rank1 37/40` (bar 36, held-out 19/20) through a `--grammar-file` GBNF with per-model authors' sampling, while the shipped model stays Llama-3.2-3B. The remainder is owned by the pattern-discrimination route entry, which chooses the model, and the entry after it, which ships the founder's pick.
**Why:** The model-replacement entry ran and its verdict is recorded as measured. The founder ruled on 2026-10-05, at this wrap, that the model is chosen by pattern discrimination before any swap, because the naming series saturates on retry storms Pulse already catches; the identity tuple and the framing are untouched.
**Ref:** .andromeda/runs/2026-10-05T09-41-57Z-wrap/

## 2026-10-05-l4-model-chosen-by-pattern-discrimination — the L4 model is chosen: the founder's pick, gemma-4-E4B-it-Q4_K_M
**Section:** §Established Decisions → [LLM Inference Runtime — L4 interpretation layer] (the **Model** clause) · §Established Decisions → [Fault Identity] (the closing remainder clause)
**Change:** The Model clause no longer says the model "is chosen by the pattern-discrimination route entry": the series ran (six small models over pre-registered A / B / C shapes, n = 10 per shape-render, the `gb` arm, each model's authors' sampling, a blind 10 % audit 9/96 `agree`) and its rule recommended gemma-4-E4B-it-Q4_K_M (A-cause 49 %, a lower bound; B false alarm 0 %), also the heaviest candidate with GPU p50 above the 5000 ms gpu-primary budget; the founder picked the unsloth gemma-4-E4B-it-Q4_K_M (sha256 `85a896a0…ab87`) over Google's QAT q4_0 build after a record-only face-to-face read the two within run-to-run noise. The next route entry ships the pick; the shipped GGUF stays Llama-3.2-3B-Instruct-Q4_K_M until it lands. The [Fault Identity] remainder clause, which restated the same future choice, now says the model is chosen and the remainder is owned by the entry that ships the pick.
**Why:** The plan's recorded expected amendment; the founder's pick (2026-10-05, relayed verbatim by the overseer) retires the future-tense claim at both sites.
**Ref:** .andromeda/runs/2026-10-05T13-31-12Z-wrap/

## 2026-10-05-l4-runs-the-founder-s-pick-with-its-authors-settings — L4 output constrained by a committed GBNF via --grammar-file, with the authors' sampling
**Section:** §Established Decisions → [LLM Inference Runtime — L4 interpretation layer] (the constraint mechanism, the argv, the prefill-trap note) · §Stack and Technologies (AI/ML serving row) · §Inherited Defaults (LLM inference runtime line)
**Change:** Was "native JSON-schema-constrained GBNF sampling (`--json-schema-file`)"; now a committed GBNF, `pulse-app/src/l4-output.gbnf` (`L4_OUTPUT_GBNF`, 44 rules), passed per spawn with `--grammar-file` and never `--json-schema-file`. The grammar is the b9305 `json_schema_to_grammar.py` output for `L4_OUTPUT_JSON_SCHEMA`, byte for byte, pinned by `pulse-app/tests/unit_l4_grammar.rs`, which runs the converter vendored at `pulse-app/vendor/llama-cpp/`. `grammar_for_schema` maps any other schema to `InferenceFailed { grammar_schema_mismatch }` before anything is written or spawned, and a per-spawn RAII temp file in `std::env::temp_dir()` carries the grammar. After `-n` the argv carries the shipped model's published sampling `--temp 1.0 --top-p 0.95 --top-k 64 --min-p 0` (`LLAMA_CLI_TEMP` / `_TOP_P` / `_TOP_K` / `_MIN_P`), then `--grammar-file {path}`; `-p {prompt}` is last and the only OTLP-derived operand. The b9305 grammar-prefill trap now stands as the reason the product passes `--grammar-file`, re-measured on the product (chunk-base argv 4/4 `json_parse_failed`).
**Why:** A boundary widening (the subprocess boundary gains the `--grammar-file` crossing and four sampling operands), ratified by the founder's own word — at phase (inputs#I1 ruling 1) and live at this wrap's escalation on 2026-10-05, the grammar crossing and all four sampling operands, relayed verbatim by the overseer. The json-schema form cannot run gemma4 on b9305: an output-format grammar is prefilled and fails at sampler init, a user grammar never is.
**Ref:** .andromeda/runs/2026-10-05T15-18-57Z-wrap/

## 2026-10-05-l4-runs-the-founder-s-pick-with-its-authors-settings — the shipped L4 model is the founder's pick; the gpu-primary budget raised
**Section:** §Established Decisions → [LLM Inference Runtime — L4 interpretation layer] (the **Model** clause, the gpu-primary comparison, the next-entry sentence) · §Established Decisions → [Fault Identity] (the closing remainder clause)
**Change:** Was "the shipped GGUF is Llama-3.2-3B-Instruct-Q4_K_M … the next route entry ships that pick; until it lands the shipped GGUF stays Llama" (the forward half of 2026-10-05-l4-model-chosen-by-pattern-discrimination — the L4 model is chosen: the founder's pick, gemma-4-E4B-it-Q4_K_M; its record of the series and the pick stands); now the shipped GGUF is the founder's pick, the unsloth gemma-4-E4B-it-Q4_K_M (sha256 `85a896a0…ab87`), selected by `ANDROMEDA_PULSE_MODEL_PATH`, never committed, its file stem the identity `interpretation.model.load` reports. On the CUDA route it parsed 7/7 product generations at a live latency p99 of 6716 ms (n = 7), where the chunk-base `--json-schema-file` argv parsed 0/4; Llama-3.2-3B is recorded as the previous shipped GGUF. The gpu-primary comparison now reads that the pick's GPU p50 (5346 ms; 5187 ms re-run) sat above the then 5000 ms budget, which the founder raised to p99 ≤ 10000 ms by nearest rank, graded by the dev-host `xtask/ci/l4-latency-p99.{sh,ps1}` over samples labelled by `LlmInferenceRunner::hardware_profile`. [Fault Identity] no longer says the shipped model stays Llama or that the pick is owed to a later entry: this chunk ships it; the historical Llama series stands.
**Why:** The model-shipping entry ran. The founder ruled on 2026-10-05 (inputs#I1 ruling 2) that the L4 GPU SLO is raised; the phase proposed 10000 ms p99 from the measured tail (pattern-series max 7285 ms, fresh re-run max 7.5 s).
**Ref:** .andromeda/runs/2026-10-05T15-18-57Z-wrap/

## 2026-10-05-l4-runs-the-founder-s-pick-with-its-authors-settings — the per-spawn L4 grammar temp file registered as the third out-of-data-dir write
**Section:** §Occupied Resources → Filesystem locations (a new bullet; the export-sink bullet; the lock-file bullet)
**Change:** A new bullet registers `andromeda-pulse-llama-grammar-{pid}-{nanos}.gbnf` in `std::env::temp_dir()`, written by `LlamaCliInference::generate_constrained` through the RAII `GrammarTempFile` before each spawn and removed after the wait; it holds only `L4_OUTPUT_GBNF`, is passed as `--grammar-file`, is never logged, and a write failure is `grammar_tempfile_write_failed`. The location predates the lock file — since chunk #84 it held the JSON schema (`SchemaTempFile`); this chunk changed its content and name, not its location. Was "one of TWO deliberate exceptions" (export sink) and "the SECOND deliberate exception" (lock file); now three, each bullet naming the other two.
**Why:** A pre-existing reality the masters missed: the measured count of product-written locations outside the data dir is three, not two. The implementation already embodies it, so the doc alone was wrong.
**Ref:** .andromeda/runs/2026-10-05T15-18-57Z-wrap/

## 2026-10-05-l4-runs-the-founder-s-pick-with-its-authors-settings — §Stack gains the test-only L4 grammar conversion check
**Section:** §Stack and Technologies (a new row after AI/ML serving)
**Change:** A new row "L4 grammar conversion check (test-only)": llama.cpp b9305 (`63248fc3`) `examples/json_schema_to_grammar.py`, vendored byte-identical at `pulse-app/vendor/llama-cpp/` (MIT, its `LICENSE` beside it; stdlib-only Python 3), plus a Python 3 interpreter (`python3` or `python`) on every host that runs the workspace tests. `pulse-app/tests/unit_l4_grammar.rs` runs it on `L4_OUTPUT_JSON_SCHEMA` and asserts the committed `L4_OUTPUT_GBNF` equals its output; a missing interpreter fails the test and never skips it. It is never in the shipped binary and sits outside cargo-deny's view.
**Why:** The P4 fork answered by the overseer (founder-delegated): the founder's ruling requires the test to PERFORM the conversion, so the converter is vendored and Python becomes a test-time runtime.
**Ref:** .andromeda/runs/2026-10-05T15-18-57Z-wrap/

## 2026-10-06-l4-first-hypothesis-names-the-triggering-service — [Fault Identity]: the scope obligation, the v2.6 lineage, the reading, the kind-label-only line as standing
**Section:** §Established Decisions → [Fault Identity — what makes two faults ONE fault]
**Change:**
- The digest's `TRIGGER:` line is stated as carrying the kind label ONLY, never the `scope_id` or a service name (overseer ruling 2026-10-04, standing). A service-bearing line exists only as two harness-only probe renders; the question of superseding the ruling was made moot by the reading.
- `TRIGGER_FRAMING_INSTRUCTION` gains a fourth obligation: when the cue line under ATTENTION CUES carries a `scope_id`, the first hypothesis statement names that value exactly as written and attributes the signal to nothing else, a service whose name merely contains it included. One static sentence, conditional; it names no service and names the `scope_id` by reference. The instruction is 787 B, was 538 B.
- The prompt lineage was `v2.5 / v1.4-fallback / v1.4-reflection`; now `v2.6 / v1.5-fallback / v1.5-reflection`.
- The scope obligation's reading is recorded as measured: gemma-4-E4B-it-Q4_K_M, llama-cli CUDA, fired once 2026-10-07, 240/240 parsed, n = 10 per shape, the `identifies` grader over the first hypothesis statement (the `scope_id` as a whole word AND a retry token), bar ≥ 36/40 over S1–S4 and ≥ 19/20 over S7 + S8. `shipped` 40/40 · 20/20, the v2.5 baseline `ns` 38/40 · 20/20, harness-only `L` and `LI` 40/40 · 20/20; selection `shipped`, PASS, regression guard HOLDS.
- Its limit is stated beside it: the baseline met the bar too, so the probe had no known-positive for the miss Conductor's fourth series hit; the measured effect is two generations on S3; whether the sentence fixes that case is unmeasured, and Conductor's fifth series is the only end-to-end reading.
**Why:** The chunk shipped the smallest lever against the fourth series' NOT MET (the storm attributed to the hyphenated sibling) and proved it with one pre-registered reading; the body's record of what the framing layer says, and of what has been measured of it, has to match. The verdict stands as pre-registered and its caveat stands with it, confirmed by the overseer (founder-delegated) after his own recount. Standing trap: a bar whose baseline arm cannot miss measures the bar, not the fix.
**Kept:** The dated Llama-3.2-3B readings on the substring `retry` grader ("still FAILS the pre-registered bar of 36/40") — another model and grader, true as dated. The decision that the TRIGGER line is framing, never identity.
**Ref:** .andromeda/runs/2026-10-07T06-47-12Z-wrap/

## 2026-10-07-l4-probe-reproduces-the-canary-history-miss — [Fault Identity]: corpus selection narrowed to the triggering scope; the canary-history readings
**Section:** §Established Decisions [Fault Identity]
**Change:**
- The scope obligation's reading said "whether it fixes that case is unmeasured — Conductor's fifth series is the only end-to-end reading of it"; it now says the case was unmeasured at that chunk and that the next chunk read `shipped` 9 of 20 `both` on one replayed prompt of three.
- New clause, the readings: synthetic S7–S16 at n = 20 read 199 of 200 `both`, NOT REPRODUCED (the corpus block alone, in eight variations, does not carry the miss); three captured product prompts replayed, REPRODUCED on d2 (`shipped` missed in 11 of 20; d1 1, d3 0); remedy reading on d2 at n = 20: `shipped` 11 `both`, `CR` 16, `CO` 19, `CC` 14, `CX` 20, guards 20 of 20, the order `CR`, `CO`, `CC`, `CX` selected `CO`; a further n = 40 reading: service misses `CO` 5 of 120, `CX` 0 of 120.
- The founder chose `CX`, and it ships: `select_corpus_matches` takes `triggering_scope: Option<&str>` before `limit`, and `assemble` passes the cue's `scope_id`. With `Some(scope)` the scope arm keeps a candidate only when its `scope_id` equals that scope and is an active scope; the fingerprint arm is unchanged; the narrowing runs before the five-line cap and only removes; `None` selects as before. `render_payload`, the framing note, the `TRIGGER:` line, the cap and `DIGEST_CORPUS_RETRIEVAL_LIMIT` are unchanged. Selection of prompt context, not identity.
- "Incidents with no triggering cue … match on scope only" is qualified: under a cue carrying a `scope_id` the scope arm keeps that scope's candidates only.
- Not shown: `CX` as the whole remedy (on d3, 13 of 40 first hypotheses still name the sibling beside the right service; an own line with a model-authored title is kept; a fingerprint-kept line of another service was never read; no generation ran on the shipped tree). The probe keeps no durable known-positive (`S17` read clean).
**Why:** The chunk changed the mechanism this decision describes, and its readings retired the statement that the scope sentence's effect on the canary case was unmeasured. The remedy in the product is the founder's choice (the founder, 2026-10-07, by dialog, relayed by the pc overseer), not the candidate the pre-registered order selected; both stand in the body. Trap for later chunks: the readings rest on captured prompts that are deleted, so they cannot be run again and the probe is no guard for this remedy.
**Kept:** The kind-label-only `TRIGGER:` line and the 2026-10-04 ruling behind it. The sentence that the probe had no known-positive at chunk 2026-10-06, still true of the tree.
**Ref:** .andromeda/runs/2026-10-07T20-29-42Z-wrap/

## 2026-10-09T08-07-22Z-wrap — the CPU-route retirement is not implemented in 0.3.0; its owner left the route for the residuals
**Section:** §Stack AI/ML row · §Established Decisions [LLM Inference Runtime] · §Occupied Resources `ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH` · §Inherited Defaults
**Change:**
- All four sites said the CPU route stays routed, or the variable consumed, "until the programmatic-L4 route entry" lands. Now each says the retirement is not implemented in 0.3.0 and was carried to the next version at the 2026-10-09 version close; three of the four point at `.andromeda/residuals.md` (§Inherited Defaults states the carry without the pointer).
- [LLM Inference Runtime] names the removed entry by its title, "Without a GPU, L4 analysis is programmatic", says it was removed unbuilt on the founder's ruling, and states the new form it was carried in: the report on an event is assembled without a model.
- Unchanged: the 2026-10-05 retirement itself, its measured basis, and the statement that the CPU tiers still reach the CPU build.
- A partial retirement of `2026-10-05-l4-runs-on-a-small-current-model-chosen-by-measurement — LLM runtime: argv constants, the model, the grammar trap, the CPU route retired`: its claim that a route entry owns the implementation no longer stands; the rest of that entry does.
**Why:** The founder closed 0.3.0 as it stands on 2026-10-09 (his own pick among the options put to him, relayed verbatim by the pc overseer; this wrap was invoked on that relay), and the option he picked carries this entry to the next version in a new form. The route entry the four sites named as owner no longer exists, so they named an owner that is not there. Standing fact for later versions: nothing in 0.3.0 removes the CPU route or stops consuming the CPU binary path.
**Kept:** The four routing sites the removed entry's wrap was to amend are left describing the CPU route as it ships; they change when the retirement is implemented, not before.
**Ref:** .andromeda/runs/2026-10-09T08-07-22Z-wrap/

## 2026-10-09-ci-on-linux-alone — CI/CD approach: six jobs on Linux alone; the `release` job and both matrices leave
**Section:** §Infrastructure Patterns → CI/CD approach (the key file `registries/contracts/architecture/ci-cd-approach.md`) · §Occupied Resources → xtask CLI surfaces (`check:english-sources`)
**Change:**
- CI/CD approach, the `ci.yml` bullet. Was: seven independent jobs, `lint-test` and `a11y` on a Linux/macOS/Windows matrix, a macOS/Windows `release` job owning `release-{os}` with `cache-on-failure`, three perf steps "on Linux". Now: six jobs (`lint-test` · `mcp-test` · `a11y` · `boot` · `supply-chain` · `coverage`), each on `ubuntu-22.04`, no matrix, no `needs:` edge; the perf steps are unconditional; no CI job compiles or tests on Windows or macOS.
- The workspace release build keeps two CI witnesses, `supply-chain`'s `cargo auditable build --workspace --release` and `boot`'s `cargo build --workspace --release --features mcp-server`, pinned by `ci_workflow_keeps_the_linux_release_build_witnesses`; `ci_workflow_runs_on_linux_only` pins that the workflow names no other system.
- Cache keys: `lint-test-{os}` (owned by `lint-test`, restored read-only by `mcp-test` and `a11y`), `boot-Linux` (owned by `boot`, restored read-only by `supply-chain`), `coverage` (registry only). `{os}` resolves to `Linux` alone; no job writes `release-{os}` or a Windows or macOS `lint-test-{os}` key.
- A third dated cache reading beside the two of 2026-09-29/30: 2026-10-09T15:13:15Z, 10 entries, 12 208 662 121 B, which is 1 471 243 881 B over the 10 737 418 240 B cap; six entries (8 531 028 085 B) sit on keys with no writer; the four that stay sum 3 677 634 036 B.
- xtask CLI surfaces: the `check:english-sources` step is wired in `lint-test` on `ubuntu-22.04` (was "on all three OSes").
**Why:** Chunk 2026-10-09-ci-on-linux-alone removed the Windows and macOS runners from the `ci` workflow, the CI clause of P-113. The reading records the cache as measured and names no remedy: the wrap deletes nothing outside the tree, and who owns the six entries is a route matter.
**Kept:** The two dated cache measurements of 2026-09-29/30 with their "a watch" wording; the `release.yml` and `update-channels.yml` bullets (both workflows are unchanged); the dated Windows-runner facts in the xtask entry.
**Ref:** .andromeda/runs/2026-10-09T15-11-06Z-wrap/

## 2026-10-09-supply-chain-job-same-on-push-and-pull-request — CI/CD approach: the audit step, the trigger pin, the fourth cache reading
**Section:** §Infrastructure Patterns → CI/CD approach (key file `registries/contracts/architecture/ci-cd-approach.md`)
**Change:** The six-job sentence's `supply-chain` clause named only the auditable build; it now also names `cargo audit` as a plain `run:` step (no action, no token, nothing published, so it ends the same on a pull-request run and on a push run; exit 0 clean · 1 a vulnerability · 2 could not evaluate, on the runner image's own `cargo-audit`, which no step installs; pinned by `ci_workflow_audit_step_is_a_plain_run_step`). New sentence: the trigger block (`pull_request`, and `push` on `main`) is pinned as it stands by `ci_workflow_triggers_are_pull_request_and_push_on_main`. Fourth dated cache reading appended (2026-10-09T17:49:51Z): 10 entries, 12 208 662 121 B, the same count and bytes, none evicted, none created; "keys no job writes" holds for the build branch's workflow only — `main`'s workflow keeps seven jobs and its three-system matrix until the version reaches it, and its push run `ci#37964887106` restored the four orphaned entries on `refs/heads/main` (`last_accessed` 13:05Z → 17:15Z); the two on `refs/pull/39/merge` still read 08:24Z, so least-recently-used names those two alone.
**Why:** The step changed in this chunk (its record is security-plan's entry of the same marker), and the cache reading was this entry's carried duty. The reading no longer tests whether GitHub evicts the six orphaned entries first: two rounds on `main`'s old workflow touched four of them. A later reading that wants to test eviction waits until `main` runs the Linux-only workflow.
**Kept:** The third reading's sentence is left as written; the fourth qualifies it. Nothing outside the tree was deleted: the cache entries stand.
**Ref:** .andromeda/runs/2026-10-09T17-46-44Z-wrap/

## 2026-10-09-supply-chain-job-same-on-push-and-pull-request — record_webgpu_adapter: the CI boot job is a witness on some runs
**Section:** §Occupied Resources → Tauri IPC routes → `telemetry.frontend.record_webgpu_adapter`
**Change:** Was "its only live witness is the dev-host `perf:frame-sample` leg (the CI boot job stops the app before the webview issues any IPC)". Now: the dev-host `perf:frame-sample` leg is the live witness for `obtained`; the CI boot job's log holds the record on some runs only — `outcome: no_navigator_gpu` in four of seven boot-job logs of 2026-10-09 and no webview-originated record in three, by how long the app lives before the smoke's `cleanup` stops it (0.57 s to 1.40 s; as measured).
**Why:** Measured false as a rule while reading the boot smoke's application logs; the same correction lands in obs-plan, test-plan and security-plan under this marker. The procedure, its input type and the procedure count are unchanged.
**Ref:** .andromeda/runs/2026-10-09T17-46-44Z-wrap/

## 2026-10-09-boot-smoke-s-early-exit-found-and-closed — two harness verdict verbs registered; boot polls readiness
**Section:** §Occupied Resources → xtask CLI surfaces (`cargo xtask harness:status`; `cargo xtask harness:ready`, new; `cargo xtask harness:settled`, new; `scripts/agent-run.{sh,ps1}`) · → Filesystem locations (`run/andromeda-pulse.pid`; `run/andromeda-pulse.spawn` + `run/andromeda-pulse.exit`) · → Environment variables (`ANDROMEDA_PULSE_PIDFILE`, `ANDROMEDA_PULSE_LOGFILE`)
**Change:**
- `cargo xtask harness:ready` registered: object `{verdict, pid, ended, otlp_grpc, otlp_http}`; arms `ready` (exit 0) / `not-ready` (1) / `ended` (1) / `cannot-evaluate` (2); `ready` is the `harness:status` verdict `running-healthy` AND a TCP connection accepted on `127.0.0.1` at both resolved OTLP ports (defaults 4317 / 4318), one second per attempt; labels `accepting` | `refusing`; `ended` needs a pid; a port variable outside 1 to 65535 is `cannot-evaluate`.
- `cargo xtask harness:settled [--timeout-seconds N]` registered (default 30, below 8 refused, 200 ms poll): `settled` (exit 0: a navigation record for each of the four window labels and a live pid) / `ended` (1) / `not-settled` (1) / `cannot-evaluate` (2); object `{verdict, pid, ended, app_exit_record, windows_settled, display, session_bus}`, also written to `logs/harness-settled.json` when `logs/` exists. `not-settled` is pinned and never read live; the non-Unix socket branch was not compiled on the dev host.
- `boot` polls `harness:ready`. Was: "Callers are exactly the two harness legs (the boot poll + the `status` verb)" of `harness:status`; now the `status` verb is its one script caller. A failed poll prints `app ended:` or `app still running …`, and `the receivers never both accepted: …` only when the last verdict held a `refusing` label. The ps1 verb mirrors it through `Invoke-Ready`: parsed, never run.
- The pid file and the exit record gain two xtask readers; both variables are also threaded into the `harness:ready` child.
- The waiting subshell's `/dev/null` streams are its own; the app's stdout and stderr go to `logs/boot.log`.
**Why:** the chunk closed the readiness gap: `boot` had reported ready before the receivers bound. The registry enumerates xtask verbs, so a new verb registers.
**Kept:** the script line citations on the two variable entries (the citation sweep proved them); `harness:status`'s object, arms and exit codes.
**Ref:** .andromeda/runs/2026-10-09T19-51-29Z-wrap/

## 2026-10-09-boot-smoke-s-early-exit-found-and-closed — the CI smoke step, its kept files, the harness's system variables, what a pull-request run builds
**Section:** §Occupied Resources → xtask CLI surfaces (`scripts/agent-run.{sh,ps1}`, the smoke step) · → Filesystem locations (`logs/`) · → Environment variables (`DISPLAY` · `DBUS_SESSION_BUS_ADDRESS` · `XDG_RUNTIME_DIR`, new entry) · → Tauri IPC routes (`telemetry.frontend.record_webgpu_adapter`) · §Infrastructure Patterns → CI/CD approach
**Change:**
- The smoke step: was "the three invocations run inside one `xvfb-run`"; now it creates `logs/` and runs four inside one `xvfb-run -e "$ANDROMEDA_PULSE_DATA_DIR/logs/xvfb.log"` — `boot || exit 1`, `cargo xtask harness:settled; a=$?`, `status; b=$?`, `cleanup; c=$?`, `test "$a$b$c" = 000`; no `set -e`, so `cleanup` runs whatever the two before it returned; four workflow pins.
- `logs/` also holds two harness-written files the product neither writes nor reads, `logs/harness-settled.json` and `logs/xvfb.log`; the boot artifact `logs-boot-Linux` holds five files; `run/` is not uploaded.
- New harness-only entry: three SYSTEM variables read by `harness:settled` alone, labels `reachable` | `gone` | `unset` | `unknown`; a local display `:N` / `:N.S` probed as its X socket, a `unix:path=` bus address else the `bus` socket under the runtime dir by a Unix-socket connect; a `%`-escaped path reads `unknown`; no value printed or written; the product's own `XDG_RUNTIME_DIR` read unchanged.
- `record_webgpu_adapter`: was "the CI boot job's log holds the record on some runs only" (four of seven logs, the app alive 0.57 s to 1.40 s), per the entry "record_webgpu_adapter: the CI boot job is a witness on some runs"; now on every run that reaches the settle verdict (one run measured, `ci#37979648967`: 2 records, both `no_navigator_gpu`, the app alive 5.494 s); the old count stays as history.
- CI/CD approach: a `pull_request` run builds the pull request's merge ref, not the pushed tip (measured: `88d5ed30` is `e2931127` merged into `main`'s `178ebac5`); its log's `git.commit.sha` names the merge commit; "equal source" across such runs means the branch tip AND `main`'s tip both unchanged; the merged tree equalled the tip's on that run (0 files differ), which holds only while `main` holds nothing the branch lacks.
**Why:** the smoke now reads the app past the point where both red runs ended and keeps what a run that ends needs to show. The files and the variable read were classified routine harness evidence, not a boundary widening, by the pc overseer (founder-delegated) at the chunk's plan review, with a standing stop if the display output carries a watched service's telemetry. The merge-ref fact was measured and recorded on the operator's direction at the wrap: a later plan's "equal source" reads it here.
**Ref:** .andromeda/runs/2026-10-09T19-51-29Z-wrap/

## 2026-10-09-pre-push-check-native-on-linux — the pre-push:linux row describes the native check
**Section:** §Occupied Resources → xtask CLI surfaces, the `cargo xtask pre-push:linux` row
**Change:** The row describes a verb that runs on the Linux dev host itself; was "the WSL Linux pre-push verb … Windows host only", every call going through `wsl.exe` into a distro clone. Now:
- Linux alone; any other system reads `cannot-evaluate` / `not-linux`.
- The verdict document has six members, `{verdict, reason, head, tree, stages[{name, ok, ms}], missing[]}`, pinned by set equality, none carrying an environment value or a path; was eight, with `remediation` and `cache{bytes, cap, cleaned}`.
- Reasons, complete: `all-stages-ok` · `not-linux` · `pins-unreadable` · `home-unset` · `provisioning-missing` · `run-dir-unusable` · `tree-unreadable` · `stage-failed:{stage}` · `restore-failed:bindings`; `sync-mismatch` is gone.
- Order of a run: host guard → two pins from the repo (the Rust channel, ci.yml's Node major) → `HOME` → provisioning probes → per-run area → `head` and `tree` → six stages in the working tree, output on stderr.
- Every probe and stage child: a cleared environment plus `HOME`, `PATH`, `ANDROMEDA_PULSE_DATA_DIR`, `PUPPETEER_CACHE_DIR` (the `npm` stage only), `GIT_INDEX_FILE` (the tree-id git calls only); no session bus, runtime dir or display variable.
- `missing[]` names: `rust:{channel}`, `rust:clippy`, `cargo-nextest` (the last two probed only when the channel is listed), `node:{pin} (found {version})` with `none` / `unreadable`, `tool:npm`, `tool:git`, `tool:cc`, `tool:python3`. The apt-list pin and the one `sudo apt-get install` line are gone; the verb installs nothing.
- The per-run area `target/pre-push/run/` is reset once provisioning has passed, so a `provisioning-missing` run rewrites only the report twin. The clone, its 40 GiB cap and the binary-patch sync are gone.
- The `test` stage's rewrite of the tracked bindings is put back as found; `restore-failed:bindings` when it cannot; the restore does not run on a signal.
- "reads no new env var" is retired: the verb reads `HOME` and `PATH` by value and prints or writes neither.
- Node is the first `node` on the caller's PATH; on the dev host a user-level `mise` Node 24 (v24.21.0, npm 11.19.0, as measured there) beside the default Node 26. The other project's Node install is no longer read.
**Why:** The chunk rebuilt the verb natively and measured it green on the dev host twice. The reset-after-provisioning order, the `home-unset` reason and the `missing[]` names are implement's deviations, accepted for these rows by the operator (the pc overseer, 2026-10-10).
**Kept:** "Dev-host only — not wired into CI"; the three exits; the six stage names and their order; the `check:english-sources` row's mention of the `source-lint` stage.
**Ref:** .andromeda/runs/2026-10-10T01-19-03Z-wrap/

## 2026-10-09-pre-push-check-native-on-linux — HOME, PATH and PUPPETEER_CACHE_DIR registered as harness-only
**Section:** §Occupied Resources → Environment variables (two new rows)
**Change:** Two rows are new.
- `HOME` · `PATH`: SYSTEM variables, not `ANDROMEDA_PULSE_*` inputs, read harness-only and by value by `cargo xtask pre-push:linux` to build the environment its stage children receive. `HOME` unset or empty reads `cannot-evaluate` / `home-unset`. `PATH` is searched, absolute entries only, for the first directory holding a `node` file, and that directory alone is carried into the stage PATH between `{HOME}/.cargo/bin` and `/usr/local/bin:/usr/bin:/bin`. No value of either is printed or written.
- `PUPPETEER_CACHE_DIR`: harness-only, SET by the verb into its `npm` stage's children only, as the absolute `target/pre-push/run/puppeteer`, so the stage's browser download never touches `~/.cache/puppeteer`; read by no product code. The same row records that the verb SETs `GIT_INDEX_FILE` (`target/pre-push/run/index`) for the three git calls that compute `head` and `tree`, and `ANDROMEDA_PULSE_DATA_DIR` (`target/pre-push/run/data`) for every probe and stage child.
**Why:** The native verb reads two system variables and sets one variable no row held; the registry lists every harness-only variable beside the product's own.
**Ref:** .andromeda/runs/2026-10-10T01-19-03Z-wrap/

## 2026-10-10-boot-smoke-s-self-end-named-from-a-run — the boot series verb, the settle verdict's eighth member, the boot verb's witness arm
**Section:** §Occupied Resources → xtask CLI surfaces; §Infrastructure Patterns → CI/CD approach (key file)
**Change:**
- `cargo xtask harness:boot-series --count N` registered: N more boots after the smoke (ordinals 2 to N+1), each one cycle `boot` → `harness:settled` → `status` → `cleanup` under its own `xvfb-run` on its own data dir `series/boot-{ordinal}/`, `XDG_DATA_HOME` / `XDG_CACHE_HOME` set and `ANDROMEDA_PULSE_PIDFILE` / `_LOGFILE` removed per boot, a cycle bounded at 1200 s; verdict `{verdict, boots, settled, ended, other, per_boot}` with a twin at `logs/boot-series.json`; exit 0 `all-settled` · 1 `self-ended` · 1 `not-all-settled` · 2 `cannot-evaluate`; the closed `cycle` labels; the measured limit (a boot that ends before ready is counted `other`).
- `harness:settled`'s object: was seven members; now eight, the eighth `exit_witness` (`unset` · `unreadable` · `loaded` · `exit-call` · `runtime-exit` · `no-record`), read under a bound of 64 lines of 4096 bytes. Its caller: was the smoke step alone; now the smoke step and each series cycle.
- `scripts/agent-run.{sh,ps1}`: was "sh+ps1 in lockstep"; now in lockstep but for the sh-only exit-witness arm of `boot` (`ANDROMEDA_PULSE_EXIT_WITNESS_LIB` → `LD_PRELOAD` and `ANDROMEDA_PULSE_EXIT_WITNESS_FILE` on the one spawn command; exit 1 `boot: exit witness library not found` before the pre-build). The smoke step's two neighbours named, with three workflow pins.
- CI/CD approach: the `boot` job was release build → boot smoke → `ci-gates`; now release build → `Build the exit witness` → boot smoke → `Boot series (equal source)` (`--count 7`, `if: always()`) → `ci-gates`, skipped when the smoke or the series fails; the reading of `ci#38019133294` recorded (seven of eight boots ended, the series step 4 min 27 s).
**Why:** The chunk built what makes a CI run name who ended the app, and one run now reads eight boots. A later chunk reading a red `boot` job reads it by its per-boot verdicts; the cause is not closed here.
**Ref:** .andromeda/runs/2026-10-10T03-28-29Z-wrap/

## 2026-10-10-boot-smoke-s-self-end-named-from-a-run — the witness variables, the kept files and the stack row
**Section:** §Occupied Resources → Environment variables; §Occupied Resources → Filesystem locations; §Stack and Technologies
**Change:**
- Environment variables, new rows: `ANDROMEDA_PULSE_EXIT_WITNESS_LIB` (read by `agent-run.sh boot` alone; trim, regular-file check, fail closed) · `ANDROMEDA_PULSE_EXIT_WITNESS_FILE` (set on the spawn line, read by the preloaded library inside the app's process, which removes it and `LD_PRELOAD` at load and checks nothing of the path) · `LD_PRELOAD` (set on the spawn line alone; the one place harness code runs inside the app's process) · `XDG_DATA_HOME` · `XDG_CACHE_HOME` (set by the series on each of its boots). `PATH` gained a second harness-only by-value reader (the series, to find `xvfb-run`); the `ANDROMEDA_PULSE_PIDFILE` and `_LOGFILE` rows say the series removes them per boot.
- Filesystem locations: the harness-written set under `logs/` was two files; now also `logs/exit-witness.jsonl`, `logs/boot-series.json` and `logs/series/boot-{ordinal}/`; the artifact's contents as read on `ci#38019133294`. New subpath `series/boot-{ordinal}/` (per-boot data dirs, not uploaded). New harness-only entry outside the data dir: `scripts/exit-witness.c` and its two build outputs; the three product-written exceptions unchanged.
- §Stack: a harness-only row for the C exit-witness library built by the host's `cc` (dev host GCC 16.2.1 20260810, as measured 2026-10-10; the runner's version not read); the xtask controls on the built library fail on a missing `cc`.
**Why:** Each is a resource this chunk introduced. The classification of the library that runs inside the app's process is security-plan's and is PROVISIONAL there.
**Kept:** the stack row was shown to the operator beside the escalation and applied on the operator's word (the pc overseer, 2026-10-10).
**Ref:** .andromeda/runs/2026-10-10T03-28-29Z-wrap/
