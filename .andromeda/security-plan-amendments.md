# Security Plan — Amendment History (andromeda-pulse)

_Append-only v3 changelog sidecar for `security-plan.md`._

_**The plan body (`security-plan.md`) holds ONLY current truth.** This file holds the
externalized Security Decisions Log and superseded guidance, plus a pointer to where each
item's current-truth substance now lives in the body. Append new entries at the bottom;
never modify or delete historical entries._

---

## 2026-06-28 — v3 normalization: externalize the Security Decisions Log

**Section:** `## Security Decisions Log` (was the body's final section)
**Change:** Moved all five dated entries (`2026-05-02` … `2026-05-22`) plus the section intro and
the "Subsequent entry format" template out of the body and into this sidecar (preserved verbatim
under _Externalized Security Decisions Log_ below). Removed the section from the body.
**Why:** v3 shape — the Decisions Log is append-only history, not current truth; current truth lives
in the body sections, this changelog lives here.
**Marker:** removed the `## Security Decisions Log` heading + the `(Append new entries at the bottom; do not modify historical entries.)` template from the body.

**Info-loss cross-check (every current-truth item in the 2026-05-02 entry was already in the body, except one — folded first):**
- Tier = Minimal (0) + justification → already in body §Threat Model Summary.
- Authorization model (Tauri capability gating + loopback OTLP + wasmtime WIT) → already in body §Threat Model (Auth model) + §API Security + §Data Protection.
- Code-signing key custody (Azure Key Vault Premium + GitHub OIDC; updater Minisign key) → already in body §Secret Management + §Data Protection (Code-signing key custody).
- OTLP `:4318` Host-header allowlist + CORS deny-by-default + body-size cap → already in body §API Security.
- Plugin host `ResourceLimiter` + `epoch_interruption(true)` + wasi-http field bound → already in body §API Security + §Input Validation.
- `tonic` 0.14 ↔ `opentelemetry-otlp` 0.31 / `tonic` 0.13 duplicate → already in body §Dependency Security + §Anti-Patterns §Universal + §Bootstrap.
- `rmcp` "1.5.0" vs 0.3.x reconciliation → already in body §Dependency Security + §Input Validation.
- `rust-toolchain` ≥ 1.85.0 (Edition 2024) → control already in body §Anti-Patterns §Universal ("NEVER let the rust-toolchain drift below 1.85.0").
- Snapshot / clipboard / MCP OTLP-attribute leakage handled by warnings, not sanitization → already in body §Logging & Monitoring + §Data Protection.
- Tauri updater Minisign key-rotation runbook (transitional dual-key release) → already in body §Secret Management (Rotation cadence).
- **FOLDED (was only in the log):** WASM plugin signature verification deferred post-v1 → third-party plugins run unverified in v1; capability-scoped WIT + `ResourceLimiter` mitigate impact, not provenance; accepted residual risk, surfaced in the plugin-install README. Folded into body §Data Protection (At rest — Plugins bullet) before externalizing (the body bullet previously only said "Documented as a residual risk in §Security Decisions Log").

**Note:** the `2026-05-22` entry (chunk #77) records four body rewrites that are ALL already present
in the current body — §Threat Model corpus.db data-type, §Data Protection corpus.db at-rest row,
§Secret Management runtime corpus key + "What counts as secret" entry, §Anti-Patterns §Logging
uniform-scrubber framing. No re-fold needed; the entry is pure history of changes already in the body.

---

## 2026-06-28 — Fold + externalize: `opentelemetry-stdout` → tracing-only self-observation

**Section:** §Data Protection (Logs medium) · §Bootstrap phases (`logging-redaction-wire`) · §Logging & Monitoring (Log format) · §Error Handling (internal logging)
**Change:** Replaced the superseded `opentelemetry-stdout` / file-exporter references in the body with
the current self-observation truth: `tracing` + `tracing-subscriber` JSON formatter writing to
`~/.andromeda-pulse/logs/agent-latest.jsonl` (tracing-only; **no OTel SDK linked into the
self-runtime**). Removed the three `[DEPRECATED 2026-05-08]` blockquotes. Also corrected the
**unannotated** `opentelemetry-stdout` reference in §Error Handling (it never carried a deprecation
blockquote but was the same superseded term). Superseded text preserved verbatim below.
**Why:** obs-plan §12 Phase 3.5 pivot — drop the OTel SDK from the self-runtime; `tracing-subscriber`
JSON is the canonical self-observation surface. Functionally equivalent (JSON-per-line at the same
path); no security-posture change. (See the `2026-05-08` Decisions Log entries below for the original
reconciliation + annotation history.)
**Marker:** removed 3× `> **DEPRECATED (2026-05-08):** …` blockquotes; current truth is now stated
inline at each of the four sites.

Superseded body text, preserved for audit trail:

- §Data Protection (Logs medium) blockquote + body:
  > **DEPRECATED (2026-05-08):** see Decisions Log entry "2026-05-08 — Annotate body deprecation: §Data Protection / §Bootstrap / §Logging opentelemetry-stdout refs". The `opentelemetry-stdout` reference below is superseded by obs-plan §12 Phase 3.5 pivot (no OTel SDK in self-runtime); `tracing-subscriber` JSON formatter at `~/.andromeda-pulse/logs/agent-latest.jsonl` is the canonical self-observation surface. Body preserved for audit trail.

  `opentelemetry-stdout`/file exporter destination per Cross-cutting Patterns Self-observation; redaction rules in §Logging & Monitoring apply.

- §Bootstrap phases (`logging-redaction-wire`) blockquote + body:
  > **DEPRECATED (2026-05-08):** see Decisions Log entry "2026-05-08 — Annotate body deprecation: §Data Protection / §Bootstrap / §Logging opentelemetry-stdout refs". The `opentelemetry-stdout` reference below is superseded by obs-plan §12 Phase 3.5 pivot; `tracing-subscriber` JSON formatter is the canonical self-observation primitive. Body preserved for audit trail.

  Wire logger redact paths per §Logging & Monitoring. Self-observation uses `opentelemetry-stdout` to `~/.andromeda-pulse/logs/` per Cross-cutting Patterns. Snapshot/clipboard/MCP-tool-response paths must apply attribute-value redaction for incidentally captured secrets per the snapshot/clipboard hygiene note.

- §Logging & Monitoring (Log format) blockquote + body:
  > **DEPRECATED (2026-05-08):** see Decisions Log entry "2026-05-08 — Annotate body deprecation: §Data Protection / §Bootstrap / §Logging opentelemetry-stdout refs". The `opentelemetry-stdout` reference in the paragraph below is superseded by obs-plan §12 Phase 3.5 pivot; `tracing-subscriber` is the sole self-observation primitive (no OTel SDK linked). Body preserved for audit trail.

  **Log format:** structured (JSON) via `tracing` + `tracing-subscriber` (the Rust ecosystem standard that pairs with `opentelemetry-stdout` exporter per Cross-cutting Patterns); consistent fields per the obs plan's eventual schema. Field redaction is applied at the subscriber layer, not at log call sites.

- §Error Handling (internal logging) — original superseded phrase (no blockquote existed here):
  "full error details to `opentelemetry-stdout` / file exporter at `~/.andromeda-pulse/logs/` per Cross-cutting Patterns Self-observation".

---

## 2026-06-28 — Fold + externalize: `max_wasm_http_fields_size` correction (was `[AMENDED 2026-05-11]`)

**Section:** §Input Validation (Plugin host inputs row) · §API Security (Plugin host capability sandbox row)
**Change:** Replaced the body's `Config::max_wasm_http_fields_size` mentions and the two inline
`[AMENDED 2026-05-11 — see Decisions Log …]` markers with the corrected current truth: the canonical
bound is the `MAX_WASM_HTTP_FIELDS_SIZE_BYTES` const in `crates/plugins/src/engine.rs` (this is **NOT**
a `wasmtime::Config` method); enforcement attaches via the wasi-http context
(`WasiHttpCtxBuilder::max_field_size`) when wasi-http imports are introduced at chunk #46+. The
CVE-2026-27572 (April 2026 wasi-http header-explosion) anchor is preserved.
**Why:** spec↔reality drift fixed during `/andromeda-implement` chunk #45 (full `2026-05-11` Decisions
Log entry preserved below). The non-existent method call was removed from `engine.rs`; the const is the
substrate-level bound.
**Marker:** removed 2× inline `**[AMENDED 2026-05-11 — see Decisions Log: …]**` markers; corrected
truth is now stated inline in both table rows.

---

## 2026-06-28 — Trim: §Authentication & Authorization SKIPPED section → one-line N/A note

**Section:** §Authentication & Authorization
**Change:** Reduced the full `_[SKIPPED — …]_` block to a one-line deliberate-N/A note. Kept the
load-bearing substance (Tauri capability gating — `pulse:default`/`pulse:tray`/`pulse:notification`/`pulse:updater`/`pulse:plugin-fs` — substitutes for runtime authorization) and the pointer to §API Security + §Security Anti-Patterns.
**Why:** v3 lean — a deliberate N/A is a one-line note, not a full section. No control removed.
**Marker:** n/a.

---

## Externalized superseded guidance — reference

The verbatim superseded `opentelemetry-stdout` text is reproduced inline in the
`2026-06-28 — Fold + externalize: opentelemetry-stdout` entry above.

---

## Externalized Security Decisions Log (verbatim, formerly the body's final section)

_Reproduced exactly as it stood in `security-plan.md` immediately before the 2026-06-28
externalization. The current-truth substance of these entries now lives in the body sections
named in the changelog entries above; this block is the preserved append-only history._

_Records key decisions during plan generation + manual additions between phase loops._

**Initial entry:**

`2026-05-02` — Initial security plan generated by `/andromeda-security`
- **Tier:** Minimal (0) — justified by: local-first, zero-infrastructure single-user desktop app; no user accounts; in-memory DuckDB ring buffer with 5–10 min retention (no persistent user data store); no internet-exposed network surface (OTLP receivers bound to `127.0.0.1` only); no compliance-regulated data classifications. Dominant risks per Phase 1 Sec 6: loopback OTLP receivers, untrusted third-party WASM plugins with deferred signature verification, code-signing key custody, in-app updater consuming `latest.json`.
- **Key decision:** Authorization model is Tauri 2 capability gating (`pulse:default` enumerates exactly the procedures listed in Occupied Resources) plus loopback-only OTLP receiver binding plus `wasmtime` capability-scoped WIT imports — three orthogonal capability systems substituting for user-authentication runtime authorization. Reasoning: no user accounts per Project Intent + standards-track Component Model + Tauri's negative-default capability model gives auditable trust boundaries without inventing an auth framework.
- **Key decision:** Code-signing key custody centralizes in Azure Key Vault Premium SKU + GitHub OIDC federation — eliminates the long-lived Azure service principal secret from GitHub Actions per security-research §Azure Key Vault OIDC. The Tauri updater Minisign Ed25519 private key sits next to the Windows EV cert in the same Vault; the public key bakes into `tauri.conf.json`. Reasoning: this is the single highest-value secret in the project (loss locks every installed instance out of updates per security-research §Tauri 2 Updater Signature Verification) and Azure Key Vault HSM-RSA is the documented best-practice path for OSS desktop apps in 2026.
- **Key decision:** OTLP HTTP `:4318` receives Host-header allowlist + CORS deny-by-default + body-size cap as a coordinated three-layer mitigation against the documented localhost-MCP DNS-rebinding chat-exfil class (Coder Agent API CVE 2025-09-19, CVE-2025-66414 MCP TypeScript SDK Dec 2025). Reasoning: Phase 1 Open consideration explicitly flagged "browser-origin `fetch('http://localhost:4318/v1/traces', ...)` from arbitrary websites is a real exposure" and security-research §Host header validation confirms with the lesson "localhost is not a security boundary."
- **Key decision:** Plugin host gets `wasmtime::ResourceLimiter` + `epoch_interruption(true)` + `max_wasm_http_fields_size` per security-research §wasmtime ResourceLimiter, on top of WIT capability gating. Reasoning: April 2026 advisory cluster (CVE-2026-27572 wasi-http header explosion + 6 others) requires resource bounds orthogonal to capability scoping; Cranelift backend on x86_64 is the unaffected configuration for the two Critical sandbox escapes — verified as the default build config.
- **Open questions / residual risks deferred to Decisions Log additions:**
  - **WASM plugin signature verification deferred post-v1** per Established Decisions Plugin Distribution Channel — third-party plugins run unverified in v1. Capability-scoped WIT + `ResourceLimiter` mitigate impact, not provenance. Document the limitation in the user-facing plugin install README.
  - **`tonic` 0.14 vs `opentelemetry-otlp` 0.31 / `tonic` 0.13 duplicate** must be reconciled before locking versions per Inherited Defaults open question; `cargo deny check bans` enforces.
  - **`rmcp` "1.5.0" vs published 0.3.x line** must be reconciled per Inherited Defaults open question; pinning will follow the resolution.
  - **`rust-toolchain.toml` minimum `1.84` predates Edition 2024 (1.85.0)** per security-research §Rust 2024 edition; bump to `1.85.0` minimum to align with `Cargo.toml` `edition = "2024"`.
  - **Snapshot / clipboard / MCP tool response surfaces are documented OTLP-attribute leakage paths** — addressed via user-facing warnings + visible clipboard-write event + README documentation rather than attempted sanitization (the data is what the user instrumented). Surfaced as documented behavior, not a defect.
  - **No hot key rotation path for the Tauri updater Minisign keypair without a transitional release** — runbook required before v0.1.0 ships.

`2026-05-08` — Reconcile self-observation references with obs Phase 3.5 pivot
- **Decision:** This security plan still references `opentelemetry-stdout` exporter destination in §Data Protection (logs medium) and §Bootstrap phases `logging-redaction-wire` bullet. Per obs-plan.md Decisions Log entry `2026-05-02 — User review (Phase 3.5, iteration 1) — pivot to tracing-only self-observation, drop OTel SDK from self-runtime`, the product no longer links an OTel SDK into self-observation runtime. Self-observation is `tracing` 0.1 + `tracing-subscriber` 0.3 JSON formatter writing to `~/.andromeda-pulse/logs/agent-latest.jsonl`. The legacy `opentelemetry-stdout` references in this plan are obsolete but functionally equivalent (both produce JSON-per-line at the same path) and do not change the security posture. Marked here for audit trail; downstream phase loop should treat `tracing-subscriber` JSON formatter as the canonical self-observation surface.
- **Rationale:** Cross-plan rot caught during cross-plan review. Skill discipline says specialist plans evolve through Decisions Log; this entry serves as the cross-reference rather than rewriting §Data Protection / §Bootstrap phases bodies (those rewrites belong in `/andromeda-security` re-run if/when needed).
- **Impact:** No behavioral change. §Logging redaction rules apply unchanged to the `tracing` JSON output. PII vectors 1-6 enforcement unchanged.
- **By:** Manual edit, cross-plan rot reconciliation
- **Amendment record:** `.andromeda/runs/2026-05-08T17-28-25-spec-amendment-reconcile-otel-stdout-references/amendment.md`

`2026-05-08` — Annotate body deprecation: §Data Protection / §Bootstrap / §Logging opentelemetry-stdout refs

- **Decision:** Add inline `> **DEPRECATED (2026-05-08):**` blockquote annotations inside the 3 deprecated body sites referencing `opentelemetry-stdout`:
  - §Data Protection logs medium row (line 154) — annotation as indented sub-paragraph inside the bullet
  - §Bootstrap phases logging-redaction-wire bullet (line 239) — same pattern
  - §Logging & Monitoring "Log format" paragraph (line 330) — standalone blockquote ABOVE the paragraph
- **Rationale:** Extends prior `2026-05-08T17-28-25Z-reconcile-otel-stdout-references` amendment (Decisions Log entry only) by adding visible body deprecation markers. Without annotation, future readers — and grep-based audit tools — see stale guidance with no inline cue. Body content preserved verbatim for audit trail; deprecation annotation is purely additive. Body content rewrite to align with current reality (text replacement of `opentelemetry-stdout` → `tracing-subscriber`) is reserved for a future `/andromeda-security` re-run, which has full authority to regenerate plan body content; /andromeda-evolve operates only at annotation level (per output-templates.md anti-pattern: "DO NOT modify the plan body content for Type 1/2/3/4 amendments. Only Type 5 deprecation is allowed to add a body annotation").
- **Impact:** No behavioral change. §Data Protection / §Bootstrap / §Logging body content preserved; new readers see deprecation notice at each site. PII vectors 1-6 enforcement unchanged. Tier 2/3 distillations need re-derivation (`/andromeda-setup-project --delta` will regenerate `.claude/rules/security.md`, `.claude/rules/observability.md`, `.claude/docs/security-summary.md`, `.claude/docs/obs-summary.md`). architecture.md §Established Decisions [Self-Observation] (line 64) + §Cross-cutting Patterns Self-observation discipline (line 283) ALSO have stale refs but are out of /andromeda-evolve scope; tracked for separate `/andromeda-arch` follow-up.
- **By:** /andromeda-evolve (user-driven Type 5 deprecation annotation, Path 1 uniform scope)
- **Amendment record:** `.andromeda/runs/2026-05-08T21-00-00-spec-amendment-obs-pivot-security-bodies/amendment.md`

`2026-05-11` — Reconcile `max_wasm_http_fields_size` reference: not a wasmtime::Config method

- **Decision:** §Input Validation row "Plugin host inputs" (line 126) + §API Security row "Plugin host capability sandbox" (line 189) + §Security Decisions Log 2026-05-02 (line 451) all reference `wasmtime::Config::max_wasm_http_fields_size(...)` as a wasmtime Config method. This method does NOT exist on `wasmtime::Config` in wasmtime 25.x — verified via `cargo check -p plugins --all-targets` E0599 compile error at `crates/plugins/src/engine.rs:58`. The actual wasi-http header field-size enforcement seam in wasmtime is via `wasmtime_wasi_http::WasiHttpCtxBuilder::max_field_size` (or equivalent on the wasi-http context), not the core `Config` struct. Amendment: the canonical bound for wasi-http header fields is defined as the `MAX_WASM_HTTP_FIELDS_SIZE_BYTES` const in `crates/plugins/src/engine.rs` at substrate level (chunk #45); actual enforcement attaches via the wasi-http context construction site when wasi-http imports are introduced in subsequent chunks (#46+). Chunk #45 substrate has zero wasi-http imports declared in the 3 plugin categories (custom-dashboard / data-transform / snapshot-template are pure Component Model without WASI), so the bound is moot in practice at this chunk; the const + future-enforcement-seam discipline is the correct substrate posture.
- **Rationale:** Spec ↔ reality drift surfaced by `/andromeda-implement` chunk #45 Phase 2 fix-loop. Path A (this amendment) chosen over Path A' (add `wasmtime-wasi-http` workspace dep + construct `WasiHttpCtxBuilder` at substrate) because chunk #45 substrate has no wasi-http imports — adding the dep at substrate level adds significant dep weight for zero actual security benefit. CVE-2026-27572 (April 2026 advisory cluster — wasi-http header explosion) anchor is preserved: the bound exists at substrate level, enforcement attaches at the chunk that introduces wasi-http imports.
- **Impact:** No behavioral change at chunk #45 substrate (no wasi-http imports declared; bound is moot). Body annotations added at §Input Validation row + §API Security row pointing to this Decisions Log entry. §Security Decisions Log 2026-05-02 historical entry preserved verbatim (Decisions Log is append-only). Implementation file `crates/plugins/src/engine.rs` removed the non-existent `config.max_wasm_http_fields_size(...)` method call; `MAX_WASM_HTTP_FIELDS_SIZE_BYTES` const declaration preserved as the canonical substrate-level bound. Tier 2/3 distillations need re-derivation (`/andromeda-setup-project --delta` will regenerate `.claude/rules/security.md` + `.claude/docs/security-summary.md` if they enumerate the wasmtime Config method by name; verify via grep at delta-rerun time). Phase 42 plan acceptance criterion security #3 is partially-superseded by this amendment — original criterion text required `Config::max_wasm_http_fields_size` set on Engine Config; the criterion is now satisfied as `Config::epoch_interruption(true)` set + `MAX_WASM_HTTP_FIELDS_SIZE_BYTES` const defined in engine.rs (used as span field name `max_wasm_http_fields_size_bytes` per obs instrumentation discipline). Phase artifacts at `.andromeda/phases/phase-42/` are immutable per /andromeda-phase contract; this Decisions Log entry is the canonical record of the criterion-supersession.
- **By:** /andromeda-implement Phase 2 Trigger 4 dialogue (spec-drift-protocol.md; user approved Path A)
- **Amendment record:** `.andromeda/runs/2026-05-11T17-50-00-spec-amendment-reconcile-max-wasm-http-fields-size/amendment.md`

**Subsequent entry format (for manual additions or re-runs):**

`{YYYY-MM-DD}` — {short title of decision}
- **Decision:** {what was decided}
- **Rationale:** {why — reference threat model / research / org constraint}
- **Impact:** {which sections affected; downstream skills affected}
- **By:** {`/andromeda-security` re-run / manual edit by {who}}

(Append new entries at the bottom; do not modify historical entries.)

---

`2026-05-22` — Specialist plan reconciliation — §Threat Model + §Data Protection §At rest + §Secret Management + §Anti-Pattern Logging body rewrites (chunk #77)

- **Decision:** Manual body rewrite of four specialist plan sections to reconcile pre-Consolidation-Phase-6 plan body with post-Consolidation-Phase-6 implementation reality (chunks #68-#73): (1) §Threat Model Data classification — added 5th Type for persistent incident corpus (corpus.db cell-level AES-256-GCM + OS keychain key custody per chunk #68; eliminates obsolete flat-file `baseline-corpus.bin` artifact post-chunk-#70 migration). (2) §Data Protection §At rest — added corpus.db row documenting single canonical encrypted persistence covering BaselineState (chunk #70) + ServiceRegistry (chunk #71) + RetryStormState (chunk #71); cross-references arch §Occupied Resources Corpus SQLite §At-rest posture as the canonical locked-schema reference. (3) §Secret Management — added Storage Runtime subsection documenting corpus encryption key custody flow (OS keychain primary via `keyring` crate per `crates/corpus/src/keychain.rs::OsKeychainBackend`; **passphrase-fallback-with-warning** posture per chunk #73 P-049 decision verified via `BackendKind::PassphraseFallback` enum variant at `crates/corpus/src/keychain.rs:33`); extended "What counts as secret" list with corpus encryption key entry. (4) §Anti-Pattern Logging — prepended uniform-scrubber-coverage framing paragraph documenting post-#72 reality (scrubber invoked at all persistence paths: OTLP appender DuckDB writes + Drain corpus persist + BaselineState / ServiceRegistry / RetryStormState corpus persist; replaces pre-#72 single-site framing identified by audit Section 1.B); preserved all 5 existing NEVER bullets intact.
- **Rationale:** Closes audit Section 2.H (security plan staleness — pre-Consolidation-Phase-6 body did not reflect corpus persistence layer + uniform scrubber coverage + P-049 fallback posture). Specialist plan re-derivation as a first-class operation is deferred to Andromeda v3 (where specialist plans become living artifacts with continuous evolution + a dedicated re-derivation skill); chunk #77 explicitly declares security-plan in its "Specialist plan touches" metadata per pulse-v0_2_0-route §77 Mechanism note as the within-lifecycle path for manual body rewrites (D4 drift detection fires only for specialist plan edits OUTSIDE declared chunk scope). Building interim re-derivation machinery in v2 is not worthwhile given the v3 redesign. Path A' (fix impl to match existing spec) was NOT applicable — the existing plan body was demonstrably stale relative to chunks #68-#73 implementation; impl is already correct per its acceptance criteria.
- **Impact:** §Threat Model Summary > Data classification (5th Type added); §Data Protection §At rest (corpus.db row added; Data lifecycle subsection extended); §Secret Management (Storage Runtime subsection added; "What counts as secret" list extended; Development paragraph clarified — corpus key active at developer runtime even though signing keys are N/A); §Security Anti-Patterns §Logging (uniform-scrubber-coverage framing paragraph prepended; 5 existing NEVER bullets preserved verbatim). DEPRECATED blockquotes at §Data Protection (line ~159 pre-rewrite) + §Bootstrap phases (line ~254 pre-rewrite) + §Logging "Log format" paragraph preserved verbatim per 2026-05-08 audit-trail discipline. Downstream Tier 2/3 distillations may need re-derivation via `/andromeda-setup-project --delta` if `.claude/rules/security.md` + `.claude/docs/security-summary.md` enumerate the rewritten content (verify via grep at delta-rerun time post-chunk-#77 commit). No changes to §Authentication / §Input Validation / §API Security / §Dependency Security / §Bootstrap phases / §Error Handling / §Logging & Monitoring / §Compliance Controls / §Security Anti-Patterns §Authentication-Input-DataProtection-API-Secrets-CodePatterns-Universal sub-sections (out of declared chunk #77 scope).
- **By:** `/andromeda-implement` chunk #77 (manual body rewrites within declared Specialist plan touches scope per pulse-v0_2_0-route §77 Mechanism note; user-approved chunk-scoped-exception path per /andromeda-implement Phase 1 dialogue)

---

## 2026-06-28-deterministic-env-gated-l4-mode — Register ANDROMEDA_PULSE_L4_DETERMINISTIC in §Input Validation
**Section:** §Input Validation — CLI / env var inputs row
**Change:** Added `ANDROMEDA_PULSE_L4_DETERMINISTIC` to the env-var inputs list + the How column (l4_deterministic: bool via bounded truthy-parse, `1|true|yes`, default false, no unbounded string).
**Why:** Chunk 2026-06-28-deterministic-env-gated-l4-mode (P-073) added the env var; it is code-validated (bounded truthy-parse, unit-tested `env_gate_truthy_parse_table`) but was absent from the §Input Validation boundary enumeration (D-security-input registry-completeness). Resolved with the user; codified as routine via a new playbook rule so future code-validated env-var registrations do not re-escalate.

---

## 2026-06-29-window-geometry-movable-shell — Register window-geometry.json input boundary in §Input Validation
**Section:** §Input Validation — boundary table (new row after Configuration values)
**Change:** Added the `<data_dir>/window-geometry.json` boundary row: integer x/y per window label via `serde` to `Position { x: i32, y: i32 }`; missing/corrupt → default (centered/snap) via `unwrap_or_default` (non-fatal); atomic `.tmp`+rename; no coordinate values logged.
**Why:** Chunk 2026-06-29-window-geometry-movable-shell (P-061) added the new persisted geometry file as a deserialized-at-boot input surface. D-security-input flagged it (escalate severity); the report shows it code-validated (integer-only serde + graceful default) + unit-tested (`unit_window_geometry.rs` roundtrip/missing/corrupt), so it matches the bounded-config-input playbook rule (routine registry-completeness, not an unvalidated-boundary HALT). Applied silently per that rule.


## 2026-08-14-workspace-key-alignment — published-key input boundary + measured corpus key custody
**Section:** §Input Validation (new boundary row); §Threat Model Summary → Attack surface (MCP stdio vector + filesystem-reads vector); §Secret Management → Runtime / Development / Production-dev separation / What counts as secret; §Data Protection → At rest (persistent incident corpus); §Threat Model Summary → Data classification (corpus row)
**Change:** (1) Added an §Input Validation row for `<data_dir>/run/workspace-key` — bounded ≤ `MAX_WORKSPACE_KEY_BYTES` (4096), UTF-8 checked, trailing newline trimmed, empty + control-character values rejected, canonicalize-and-confine on write, and the value consumed ONLY as an opaque filter string so CWE-22 is closed by construction. (2) Recorded that the sidecar's stdio is no longer its only input, and added the key file to the filesystem-reads entry-point list with its now-declared validation discipline. (3) Corrected the corpus key-custody claim at all six restating sites from "OS keychain primary OR passphrase-fallback-with-warning" to measured reality: keyring resolves with no platform backend, so the non-persisting mock store yields a per-PROCESS key, no keychain entry is written, and the fallback warning never fires — with the data-loss consequence stated (historical encrypted cells unreadable by any later process).
**Why:** (1)+(2) register a genuinely new cross-process input boundary this chunk landed, following the 2026-06-29 `window-geometry.json` precedent row (code-validated + unit-tested ⇒ routine registry completeness). (3) records measured truth per the chunk report's "Spec claims disproved by measurement" #1, which names §Secret Management explicitly. The impl fix (keyring platform features + a migration) is owned by the new "Corpus key persistence" working-route entry, not by this chunk.

## 2026-08-15-corpus-key-persistence — corpus key custody closed at 6 restating sites; passphrase + deny carve-outs + a ratified external-decay deferral registered
**Section:** §Threat Model Summary → Data classification (user-content corpus) · §Data Protection → At rest → Persistent incident corpus · §Secret Management → Storage → Runtime · → Development · → Production / dev separation · §Secret Management → What counts as secret · §Input Validation (CLI / env var row + its rule cell) · §Dependency Security → CI integration · §Bootstrap phases → dep-audit-tooling-install
**Change:** The 2026-08-14 correction is itself corrected at all SIX sites it landed on: the corpus cell key
is no longer per-process ephemeral from keyring's mock store — the explicit platform feature set links a real
OS credential store, the key persists across processes (measured: 0 `decryption_failed` where 13 defined the
defect; Windows entry `corpus-key.com.andromeda.pulse` present), and the P-049 fallback now has a named
secret source. Registered `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` in the §Input Validation env-var row with its
bounded-parse rule (non-empty, ≤ 1024 bytes, unset ⇒ not-configured; KDF input only, never canonicalized,
never a path, never logged) and added it to "What counts as secret". Recorded the ID-scoped `[bans] skip`
carve-out discipline (`core-foundation` + `security-framework`, duplicates internal to `keyring`) with
`multiple-versions = "deny"` unrelaxed and the `tonic` canary unskippable. Ratified a standing external-decay
deferral for `cargo audit` with `cargo deny check advisories` as the named overlap signal.
**Why:** The chunk measured the previous claim false and closed it. The fallback's opt-in semantics are
recorded deliberately: it engages only when configured, so a transient store failure cannot silently switch
keys — the failure mode that created the orphaned content in the first place. The deferral is bounded, not
open-ended: `cargo audit` cannot LOAD the RustSec DB (upstream duplicate advisory ID, reproduced), the
overlap signal still runs every wrap, and the overlap's own findings carry visible dispositions — 3
no-safe-upgrade IDs ignore-listed, 7 upgradeable ones (5 vulnerabilities) left RED under a named owner
rather than accepted.

## 2026-08-16 (0-pending route-adaptation wrap) — cargo-audit standing deferral re-ratified at pin #3 with a re-run interval, and migrated onto the route
**Section:** §Dependency Security → CI integration (Standing deferral — `cargo audit` unrunnable)
**Change:** The deferral's Probe clause changes from "re-run `cargo audit` once per wrap" to a ratified
INTERVAL of every 3rd wrap (ran at session 25, next at session 28), with the between-points discipline
stated explicitly: re-verify basis + overlap, and the absorbing chunk's report records
`probe skipped per ratified interval (next: {point})` — never a silent skip. Recorded the pin-#3
re-ratification (age trigger: third consecutive re-pin HALTs once into dialogue) and the fact that the pin
MIGRATED off `.claude/session-handoff.md`, where it had been floating, onto the working-route entry
`Baseline-family reachability` with its origin marker `2026-08-15-corpus-key-persistence` preserved. Basis
re-verified first-hand this wrap and strengthened: the DB-load failure reproduced byte-identical on
2026-08-16 in this repo AND on the Conductor project the same day — one upstream event, two projects.
**Why:** Route-resolve's age trigger fired (§Deferred-gate closure) and the operator ratified continuing
with an interval rather than converting the deferral to a route entry or an escalation. The reference
explicitly permits setting a re-run interval "when the named overlap runs green every chunk anyway", which
is the case here — `cargo deny check advisories` reads the same advisory data through its own loader and
runs every chunk. Recording it in the body was mandatory rather than optional: leaving "once per wrap" in
the spec while the route pin said "every 3rd wrap" would have shipped a self-contradicting instruction, the
same truth-in-diagnostics failure class this version's sweep entry already owns. The deferral's END
condition is unchanged — it ends the first time `cargo audit` loads.

## 2026-08-16-baseline-family-reachability — register the baseline-bootstrap env boundary
**Section:** §Input Validation → CLI / env var inputs row
**Change:** Added `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` to the row's name list and its validation
column: bounded `u64` parse, whitespace-trimmed, non-zero and strictly below `WINDOW_DURATION_SECONDS`
(86400); unset / empty / unparseable / zero / out-of-range all fall back to the 3600s default, never
panicking and never silently adopting an unintended bound. It is not a path, is never canonicalized, and
carries no user data.
**Why:** A new external-input boundary shipped unregistered (D-security-input, escalate severity). Ruled
routine by the playbook's 2026-06-28 rule, which names this row explicitly and scopes D-security-input's
escalate severity to ACTUALLY-unvalidated boundaries — the report shows this one validated (bounded parse)
and unit-tested (reject-zero / reject-empty / reject-unparseable / reject-at-or-above-window / whitespace-trim
/ unset-identical-to-default). Recorded here so the boundary enumeration stays the complete list it claims
to be.

## 2026-08-17-incident-fingerprint-producer-repaired — `cargo audit` standing deferral: interval point DISCHARGED, re-pinned

**Section:** §Dependency Security → Standing deferral — `cargo audit` unrunnable (external decay)
**Change:** The probe-interval clause now records the session-28 interval point as **DISCHARGED by a real probe** rather than naming it as merely upcoming: `cargo audit` RAN (exit 1), the basis reproduced byte-identical (`error loading advisory database: parse error: duplicate advisory ID: RUSTSEC-2026-0244`), and the named overlap `cargo deny check advisories` was re-observed reporting exactly the 7 owned upgradeable IDs — no new finding. Interval reset: next probe at **session 31**. The stale "ran at session 25, next at session 28" reading is retired; the near-zero-information rationale is restated over three probe points rather than two.
**Why:** chunk `2026-08-17-incident-fingerprint-producer-repaired` carried the pin as a folded PREREQ and discharged it. This maintains requirement (d) of the 2026-08-15 routine-BOUNDED-WAIT playbook rule (a probe re-run interval must exist and stay current); a spec still naming a discharged point as pending would misdirect the next wrap into treating session 28 as unmet. Raised by the orchestrator at Validate check 5 — the security fan-out detector surfaced the staleness but correctly declined to propose it, since no dependency was added or bumped and it falls outside D-security-deps' invariant. Report §Decisions & corrections carries the full provenance.

## 2026-08-21-delegated-timing-observables — `telemetry.*` boundary registered; per-procedure capability claim retired at 6 sites
**Section:** §Threat Model Summary (IPC + webview vectors); §Input Validation (TauRPC row); §API Security (TauRPC capability authorization); §Bootstrap phases (`dep-security-ci-gate`); §Security Anti-Patterns (API)
**Change:** Added `telemetry.frontend.*` to the TauRPC boundary row with its validators (`validate_duration_ms`, `validate_discovered_count` bound 10_000, closed `HueSeverityTier`, each rejection unit-tested by name). Retired "capability JSON enumerates exactly these procedures" and the two-enforcement-pieces claim at every site: capability gating admits the webview to the IPC layer as a whole, and the per-procedure enforcement is the validated argument struct plus the `EXPECTED_PROCEDURES` drift gate. The §Anti-Patterns NEVER ban was RETARGETED, not deleted — it now bans an unpinned procedure, and gains a second clause banning an ungranted core API / `core:window:*` permission, which genuinely IS silently rejected at runtime.
**Why:** Measured at this chunk (three procedures added, capabilities untouched, drift clean). Operator-ratified with an explicit guardrail that the core-API half of the invariant must survive intact.

