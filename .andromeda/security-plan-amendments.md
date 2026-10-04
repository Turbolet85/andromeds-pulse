# Security Plan — Amendment History (andromeda-pulse)

_Append-only v3 changelog sidecar for `security-plan.md`._

_**The plan body (`security-plan.md`) holds ONLY current truth.** This file holds the
externalized Security Decisions Log and superseded guidance, plus a pointer to where each
item's current-truth substance now lives in the body. Append new entries at the bottom;
never modify or delete historical entries._

---

## 2026-06-28 — v3 normalization: externalize the Security Decisions Log
**Section:** `## Security Decisions Log` (was the body's final section)
**Change:** passages moved verbatim to the archive — the Security Decisions Log: all five dated entries (`2026-05-02` … `2026-05-22`), the section intro and the "Subsequent entry format" template; the section heading and its append-template line are removed from the body.
- Folded into the body first (the one current-truth item only the log carried): WASM plugin signature verification is deferred post-v1 — third-party plugins run unverified in v1; capability-scoped WIT + `ResourceLimiter` mitigate impact, not provenance; accepted residual risk, surfaced in the plugin-install README. Now stated in §Data Protection (At rest — Plugins bullet), which previously only said "Documented as a residual risk in §Security Decisions Log".
**Why:** v3 shape — the Decisions Log is append-only history, not current truth; current truth lives in the body sections. Every other current-truth item in the log (tier Minimal, authorization model, code-signing key custody, OTLP `:4318` Host allowlist + CORS + body cap, plugin `ResourceLimiter` + `epoch_interruption(true)`, the `tonic` 0.14 ↔ 0.13 duplicate, `rmcp` reconciliation, `rust-toolchain` ≥ 1.85.0, snapshot/clipboard/MCP leakage handling, the Minisign key-rotation runbook) and the four `2026-05-22` (chunk #77) corpus.db rewrites were already in the body, so no control was removed.
**Ref:** NOT DERIVED

---

## 2026-06-28 — Fold + externalize: `opentelemetry-stdout` → tracing-only self-observation
**Section:** §Data Protection (Logs medium) · §Bootstrap phases (`logging-redaction-wire`) · §Logging & Monitoring (Log format) · §Error Handling (internal logging)
**Change:** At all four sites the body now states the current self-observation truth: `tracing` + `tracing-subscriber` JSON formatter writing to `~/.andromeda-pulse/logs/agent-latest.jsonl` — tracing-only, **no OTel SDK linked into the self-runtime**. Was `opentelemetry-stdout` / file exporter to `~/.andromeda-pulse/logs/`. The three `[DEPRECATED 2026-05-08]` blockquotes are removed, and the unannotated `opentelemetry-stdout` reference in §Error Handling is corrected too; the superseded text is preserved verbatim in the archive.
**Why:** obs-plan §12 Phase 3.5 pivot — the OTel SDK is dropped from the self-runtime; `tracing-subscriber` JSON is the canonical self-observation surface. Functionally equivalent (JSON-per-line at the same path); no security-posture change.
**Ref:** NOT DERIVED

---

## 2026-06-28 — Fold + externalize: `max_wasm_http_fields_size` correction (was `[AMENDED 2026-05-11]`)
**Section:** §Input Validation (Plugin host inputs row) · §API Security (Plugin host capability sandbox row)
**Change:** Both rows now state: the canonical wasi-http header field-size bound is the `MAX_WASM_HTTP_FIELDS_SIZE_BYTES` const in `crates/plugins/src/engine.rs` — **NOT** a `wasmtime::Config` method; enforcement attaches via the wasi-http context (`WasiHttpCtxBuilder::max_field_size`) when wasi-http imports are introduced at chunk #46+. Was `Config::max_wasm_http_fields_size` plus two inline `[AMENDED 2026-05-11 — see Decisions Log …]` markers (removed). The CVE-2026-27572 (April 2026 wasi-http header-explosion) anchor is preserved.
**Why:** spec↔reality drift fixed during `/andromeda-implement` chunk #45: the non-existent method call was removed from `engine.rs`; the const is the substrate-level bound.
**Ref:** NOT DERIVED

---

## 2026-06-28 — Trim: §Authentication & Authorization SKIPPED section → one-line N/A note
**Section:** §Authentication & Authorization
**Change:** The full `_[SKIPPED — …]_` block is reduced to a one-line deliberate-N/A note, keeping the load-bearing substance — Tauri capability gating (`pulse:default` / `pulse:tray` / `pulse:notification` / `pulse:updater` / `pulse:plugin-fs`) substitutes for runtime authorization — and the pointer to §API Security + §Security Anti-Patterns.
**Why:** v3 lean — a deliberate N/A is a one-line note, not a full section. No control removed.
**Ref:** NOT DERIVED

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
**Change:** `ANDROMEDA_PULSE_L4_DETERMINISTIC` added to the env-var inputs list; How column: `l4_deterministic: bool` via bounded truthy-parse (`1|true|yes`), default false, no unbounded string.
**Why:** The chunk (P-073) added the env var, code-validated by bounded truthy-parse and unit-tested, but it was absent from the §Input Validation boundary enumeration (registry completeness). Resolved with the user and codified as routine via a new playbook rule, so future code-validated env-var registrations do not re-escalate.
**Ref:** NOT DERIVED

---

## 2026-06-29-window-geometry-movable-shell — Register window-geometry.json input boundary in §Input Validation
**Section:** §Input Validation — boundary table (new row after Configuration values)
**Change:** New boundary row `<data_dir>/window-geometry.json`: integer x/y per window label via `serde` to `Position { x: i32, y: i32 }`; missing/corrupt → default (centered/snap) via `unwrap_or_default` (non-fatal); atomic `.tmp`+rename; no coordinate values logged.
**Why:** The chunk (P-061) added the persisted geometry file as a deserialized-at-boot input surface. It is code-validated (integer-only serde + graceful default) and unit-tested (roundtrip/missing/corrupt), so it matches the bounded-config-input playbook rule — routine registry completeness, not an unvalidated-boundary HALT — and was applied silently per that rule.
**Ref:** NOT DERIVED


## 2026-08-14-workspace-key-alignment — published-key input boundary + measured corpus key custody
**Section:** §Input Validation (new boundary row); §Threat Model Summary → Attack surface (MCP stdio vector + filesystem-reads vector); §Secret Management → Runtime / Development / Production-dev separation / What counts as secret; §Data Protection → At rest (persistent incident corpus); §Threat Model Summary → Data classification (corpus row)
**Change:**
- New §Input Validation row for `<data_dir>/run/workspace-key`: bounded ≤ `MAX_WORKSPACE_KEY_BYTES` (4096), UTF-8 checked, trailing newline trimmed, empty + control-character values rejected, canonicalize-and-confine on write; the value is consumed ONLY as an opaque filter string, so CWE-22 is closed by construction.
- Attack surface: the sidecar's stdio is no longer its only input; the key file joins the filesystem-reads entry-point list with its declared validation discipline.
- Corpus key custody corrected at all six restating sites — was "OS keychain primary OR passphrase-fallback-with-warning"; now measured reality: keyring resolves with no platform backend, so the non-persisting mock store yields a per-PROCESS key, no keychain entry is written, and the fallback warning never fires — historical encrypted cells are unreadable by any later process.
**Why:** The workspace-key rows register a genuinely new cross-process input boundary this chunk landed, following the `window-geometry.json` precedent (code-validated + unit-tested ⇒ routine registry completeness). The key-custody correction records measured truth (a spec claim disproved by measurement). The impl fix (keyring platform features + a migration) is owned by the "Corpus key persistence" working-route entry, not this chunk.
**Ref:** NOT DERIVED

## 2026-08-15-corpus-key-persistence — corpus key custody closed at 6 restating sites; passphrase + deny carve-outs + a ratified external-decay deferral registered
**Section:** §Threat Model Summary → Data classification (user-content corpus) · §Data Protection → At rest → Persistent incident corpus · §Secret Management → Storage → Runtime · → Development · → Production / dev separation · §Secret Management → What counts as secret · §Input Validation (CLI / env var row + its rule cell) · §Dependency Security → CI integration · §Bootstrap phases → dep-audit-tooling-install
**Change:**
- Corpus key custody, at all SIX sites the 2026-08-14 correction landed on: was a per-process ephemeral key from keyring's mock store; now the explicit platform feature set links a real OS credential store, the key persists across processes (0 `decryption_failed` where 13 defined the defect; Windows entry `corpus-key.com.andromeda.pulse` present), and the P-049 fallback has a named secret source.
- `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` registered in the §Input Validation env-var row: non-empty, ≤ 1024 bytes, unset ⇒ not-configured; KDF input only, never canonicalized, never a path, never logged. Added to "What counts as secret".
- ID-scoped `[bans] skip` carve-outs `core-foundation` + `security-framework` (duplicates internal to `keyring`); `multiple-versions = "deny"` unrelaxed; the `tonic` canary unskippable.
- A standing external-decay deferral for `cargo audit` ratified, with `cargo deny check advisories` as the named overlap signal.
**Why:** The chunk measured the previous claim false and closed it. The fallback is opt-in deliberately: it engages only when configured, so a transient store failure cannot silently switch keys — the failure mode that created the orphaned content. The deferral is bounded: `cargo audit` cannot LOAD the RustSec DB (upstream duplicate advisory ID, reproduced), the overlap runs every wrap, and its findings carry visible dispositions — 3 no-safe-upgrade IDs ignore-listed, 7 upgradeable ones (5 vulnerabilities) left RED under a named owner rather than accepted.
**Ref:** NOT DERIVED

## 2026-08-16 (0-pending route-adaptation wrap) — cargo-audit standing deferral re-ratified at pin #3 with a re-run interval, and migrated onto the route
**Section:** §Dependency Security → CI integration (Standing deferral — `cargo audit` unrunnable)
**Change:** The Probe clause was "re-run `cargo audit` once per wrap"; now a ratified INTERVAL of every 3rd wrap (ran at session 25, next at session 28). Between points: re-verify basis + overlap, and the absorbing chunk's report records `probe skipped per ratified interval (next: {point})` — never a silent skip. Records the pin-#3 re-ratification (age trigger: a third consecutive re-pin HALTs once into dialogue) and that the pin MIGRATED off `.claude/session-handoff.md` onto the working-route entry `Baseline-family reachability`, origin marker `2026-08-15-corpus-key-persistence` preserved. Basis strengthened: the DB-load failure reproduced byte-identical in this repo AND on the Conductor project the same day — one upstream event, two projects. The END condition is unchanged — the deferral ends the first time `cargo audit` loads.
**Why:** Route-resolve's age trigger fired and the operator ratified continuing with an interval rather than converting the deferral to a route entry or an escalation. An interval is permitted when the named overlap runs green every chunk anyway — `cargo deny check advisories` reads the same advisory data through its own loader and runs every chunk. Recording it in the body was mandatory: "once per wrap" in the spec beside "every 3rd wrap" on the route would ship a self-contradicting instruction.
**Ref:** NOT DERIVED

## 2026-08-16-baseline-family-reachability — register the baseline-bootstrap env boundary
**Section:** §Input Validation → CLI / env var inputs row
**Change:** `ANDROMEDA_PULSE_BASELINE_BOOTSTRAP_SECONDS` added to the row's name list and validation column: bounded `u64` parse, whitespace-trimmed, non-zero and strictly below `WINDOW_DURATION_SECONDS` (86400); unset / empty / unparseable / zero / out-of-range all fall back to the 3600s default, never panicking and never silently adopting an unintended bound. Not a path, never canonicalized, carries no user data.
**Why:** A new external-input boundary shipped unregistered. Ruled routine by the playbook's 2026-06-28 rule, which names this row and scopes the input detector's escalate severity to ACTUALLY-unvalidated boundaries — this one is validated (bounded parse) and unit-tested (reject-zero / reject-empty / reject-unparseable / reject-at-or-above-window / whitespace-trim / unset-identical-to-default). Recorded so the boundary enumeration stays the complete list it claims to be.
**Ref:** NOT DERIVED

## 2026-08-17-incident-fingerprint-producer-repaired — `cargo audit` standing deferral: interval point DISCHARGED, re-pinned
**Section:** §Dependency Security → Standing deferral — `cargo audit` unrunnable (external decay)
**Change:** The probe-interval clause records the session-28 interval point as **DISCHARGED by a real probe**: `cargo audit` RAN (exit 1), the basis reproduced byte-identical (`duplicate advisory ID: RUSTSEC-2026-0244`), and the overlap `cargo deny check advisories` reported exactly the 7 owned upgradeable IDs — no new finding. Next probe at **session 31**. Was "ran at session 25, next at session 28" (retired); the near-zero-information rationale now rests on three probe points rather than two.
**Why:** The chunk carried the pin as a folded PREREQ and discharged it. Keeps requirement (d) of the 2026-08-15 routine-BOUNDED-WAIT playbook rule — a probe re-run interval must exist and stay current; a spec naming a discharged point as pending would misdirect the next wrap. The security detector correctly declined to propose it (no dependency added or bumped — outside its deps invariant); the orchestrator raised it instead.
**Ref:** NOT DERIVED

## 2026-08-21-delegated-timing-observables — `telemetry.*` boundary registered; per-procedure capability claim retired at 6 sites
**Section:** §Threat Model Summary (IPC + webview vectors); §Input Validation (TauRPC row); §API Security (TauRPC capability authorization); §Bootstrap phases (`dep-security-ci-gate`); §Security Anti-Patterns (API)
**Change:**
- `telemetry.frontend.*` added to the TauRPC boundary row with its validators: `validate_duration_ms`, `validate_discovered_count` (bound 10_000), closed `HueSeverityTier`; each rejection unit-tested by name.
- Retired at every site: "capability JSON enumerates exactly these procedures" and the two-enforcement-pieces claim. Now: capability gating admits the webview to the IPC layer as a whole; per-procedure enforcement is the validated argument struct plus the `EXPECTED_PROCEDURES` drift gate.
- The §Anti-Patterns NEVER ban is RETARGETED, not deleted — it bans an unpinned procedure, plus a second clause banning an ungranted core API / `core:window:*` permission, which genuinely IS silently rejected at runtime.
**Why:** Measured at this chunk — three procedures added, capabilities untouched, drift clean. Operator-ratified with an explicit guardrail that the core-API half of the invariant must survive intact.
**Ref:** NOT DERIVED


## 2026-08-22-pii-scrubber-recall — catalog is eight categories; the `spans` coverage claim corrected as measured
**Section:** §Security Anti-Patterns → Logging (Uniform scrubber coverage paragraph) · §Data Protection Sensitivity note · §Persistent incident corpus
**Change:**
- The catalog's EIGHTH category `provider_key` recorded — the bare-credential recall arm (anchored-prefix + length floor), ordered after the keyed arms and before `credit_card`, with the recall-over-precision posture and its bounding false-positive corpus stated. No prior amendment had enumerated a scrubber pattern.
- APPLIED AS MEASURED: was coverage of "OTLP appender DuckDB writes for `spans` / `log_records` / `span_events`" — the `spans` half is false. The body now separates INTENDED posture from MEASURED reality, names the five client-controlled columns that never reach `scrub_attribute` (`spans.service_name` · `span_events.name` · `metrics_points.metric_name` · `log_records.severity_text` · `instrumentation_scopes.scope_name`/`.scope_version`), states that the DuckDB ring buffer is unencrypted so such values are stored and read in plaintext, and NAMES the owning route entry.
- The two same-master duplicates at §Data Protection and §Persistent incident corpus that restated "at all persistence boundaries" are qualified — the corpus paths ARE covered; the generalization was not.
**Why:** The chunk measured the recall gap RED before fixing it and, doing so, mapped the boundary precisely. Per the APPLY-AS-MEASURED rule a body stating measured truth plus an owner beats one asserting coverage that does not exist. The impl half is owned by the working-route entry "Ingestion scrub coverage — the five client-controlled columns that never reach `scrub_attribute`" — operator-decided as a standalone entry, not a CARRY on the diagnostics sweep, because a security-grade coverage hole should not inherit a housekeeping entry's priority.
**Ref:** NOT DERIVED

## 2026-08-23-ingestion-scrub-coverage — ring-buffer scrub coverage flipped from open gap to covered
**Section:** §Security Anti-Patterns → Logging (primary) · §Threat Model Summary → Data classification, Sensitivity note (restating site) · §Data Protection → At rest — per medium, Persistent incident corpus bullet (restating site)
**Change:**
- The MEASURED-reality clause was "the intended posture does NOT yet hold for five columns"; it now holds for every client-controlled ring-buffer column a producer can reach. The four live columns — `spans.service_name`, `span_events.name`, `metrics_points.metric_name`, `log_records.severity_text` — pass through `scrub_attribute` at the write boundary; "The `spans` table receives NO scrub at all" is retired.
- Count corrected from five to four-live-plus-one-vacuous: `instrumentation_scopes.scope_name`/`.scope_version` have no producer, so they cannot receive host data, leak, or be scrubbed.
- Records WHERE the `service_name` scrub sits and why — inside `extract_service_name`, the choke point for three consumers (the column, the storm `FingerprintObserver`, the baseline `SpanObserver` tap) — the uniform-redaction treatment with its rejected alternatives, and the accepted `metrics_points.metric_name` PK-collision risk as a loud rather than silent failure. Both restating sites updated in the same pass.
- PRESERVED unchanged: the ring buffer remains unencrypted (write-boundary redaction is the control, not confidentiality at rest); `span_events.exception_type` remains a deliberate exclusion; the 8-category catalog is untouched.
**Why:** The chunk closed the coverage half this section named itself the owner of, verified RED-before / GREEN-after on the real OTLP path with a mutation check proving the pins discriminate and a live wire run showing 0 canary leaks. The five-count was wrong at HEAD: `append_record_batch_to_table` is the only DuckDB write path in `crates/buffer` and fires for exactly four tables.
**Ref:** NOT DERIVED

## 2026-08-23-metrics-points-identity — the accepted LOUD collision failure is closed, and its residual recorded
**Section:** §Security Anti-Patterns → Logging (uniform scrubber coverage paragraph)
**Change:** Retired: the `metrics_points.metric_name` redaction collision as "an accepted, LOUD failure (ERROR at `flush()`), never a silent one". Now the key carries a `seq` ordinal, so two distinct credential-shaped names redacting identically no longer collide and both rows persist. Recorded in its place: the rejected alternative STILL stands — the discriminator is content-independent (a per-table monotonic counter never derived from the secret), so no stable identifier of the credential is minted; scrubbing `metric_name` remains mandatory; RESIDUAL — the two names still redact to one placeholder, so the rows land indistinguishable: redaction holds and nothing new leaks, but the whole-batch ERROR was the only signal a credential-shaped collision had occurred, and closing it removes that signal.
**Why:** The chunk deliberately removed the behaviour this section asserted (RED measured the ERROR at HEAD; GREEN persisted all rows with 0 ERROR); leaving the clause would describe a failure mode the code can no longer produce. No drift-base detector covered this section — a gap closed at the same wrap by the new `D-security-logging` detector.
**Ref:** NOT DERIVED

## 2026-08-23-metrics-points-labels — five scrubbed cells and a third scrub shape
**Section:** §Security Anti-Patterns → Logging (coverage paragraph · treatment paragraph · count-correction paragraph); §Threat Model Summary → Data classification → Sensitivity note; §Data Protection → At rest → Persistent incident corpus bullet
**Change:**
- Coverage is **FIVE** persisted client-controlled ring-buffer cells — the four columns plus `metrics_points.labels`.
- A **THIRD scrub SHAPE** beside choke-point and push-site: labels scrub the JOINED `key=value` form UNIONED with the bare value, because `secret_kv` and `api_key` are key-name-anchored and unreachable on a split value — a value-only scrub would store the entire keyed class verbatim. Label KEYS stored verbatim; only the VALUE becomes `[REDACTED:{category}]`.
- Failure mode stated: over-redaction when the KEY alone trips a match, accepted per the recall-over-precision posture; because the key survives, two distinct keys never collapse into one placeholder, so the redaction-collision class is NOT reintroduced.
- "Uniform redaction across all four" rescoped to the four COLUMN targets. Count-correction disambiguated: the RETIRED five was four live plus one vacuous; the CURRENT five is a genuine live count. Both restating sites moved four → five, their `instrumentation_scopes`-producer-less and still-UNENCRYPTED clauses preserved verbatim.
**Why:** The joined-form requirement was an operator correction at phase review and is load-bearing, not defensive — a mutation check stores `password=hunter2` verbatim when the label scrub is neutralised; wire-proven with 0 canary literals. `instrumentation_scopes.*` (producer-less) and `span_events.exception_type` (deliberate class-identifier exclusion) remain the stated non-coverage; the ring buffer remains unencrypted, so write-boundary redaction is still the control.
**Ref:** NOT DERIVED

## 2026-08-23-webview-self-verify — harness-only path carve-out + measured npm-channel gap
**Section:** §Input Validation → CLI / env var inputs · §Security Anti-Patterns → Input · §Bootstrap phases → input-validation-library-install · §Dependency Security → CI integration
**Change:**
- The categorical `ANDROMEDA_PULSE_*_PATH` / `*_DIR` canonicalize-and-confine ban is scoped to **product-binary** reads, carving out harness/xtask-only tool-locator vars that resolve an external tool living outside the data dir by design — guarded instead by trim + `is_file()` + clean skip. Applied at all THREE restating sites (the §Input ban, the §Input Validation controls column, the §Bootstrap `strict-path` bullet). `ANDROMEDA_PULSE_MSEDGEDRIVER_PATH` registered as its first member; the resolved value is never logged.
- MEASURED: the `pulse-app/ui` npm channel gets Dependabot version updates but **no advisory/license/ban scanning** — `cargo audit` / `cargo deny` / `cargo-auditable` stop at the Rust boundary and `ci.yml` runs `npm ci` but no `npm audit`. A PRE-EXISTING gap (27 → 29 direct devDeps) this chunk widened by +2 direct / +132 transitive plus a native `.exe` delivered via an npm optional dependency; the owning route entry is named.
**Why:** The carve-out was escalated and resolved with the operator: the exemption already existed by precedent — `_PIDFILE` / `_LOGFILE` are harness-only path vars long absent from the §Input Validation row — but no doc stated it, so the ban read as a standing self-violation. A proposal claiming no npm Dependabot ecosystem was REJECTED (absence needs evidence: `.github/dependabot.yml` configures `npm` at `/pulse-app/ui`); only the narrower verified gap — advisory scanning, not update coverage — is recorded, per APPLY-AS-MEASURED.
**Ref:** NOT DERIVED

## 2026-08-23-a11y-verification — owned upgradeable advisory count corrected 7 to 8
**Section:** §Dependency Security → CI integration (Standing deferral — `cargo audit` unrunnable, Probe clause)
**Change:** The probe clause records the interval points actually run (25 / 28 / 31 / 34 / 37) and the session-40 discharge (2026-08-23): probe RAN, exit 1, basis byte-identical (`duplicate advisory ID: RUSTSEC-2026-0244`), overlap at exactly **8** owned upgradeable IDs — RUSTSEC-2026-0189 / 0190 / 0194 / 0195 / 0204 / 0222 / 0253 / 0258; next point session 43. The former **7** is retired to history as the session-28 reading.
**Why:** The chunk discharged the standing `cargo audit` PREREQ in full form at its interval point and measured the overlap at eight, matching the route annotation against the plan body's stale seven. The deferral does NOT end — the RustSec DB still cannot load. The deps detector raised it at `escalate` severity, but a stale measured COUNT is not a banned or unvetted dependency (none added or bumped; `cargo deny check bans licenses sources` ok), so it was dispositioned routine-APPLY by actual class; that severity/class mismatch — the third across three escalate-severity detectors — was codified with the operator as a new playbook rule.
**Ref:** NOT DERIVED
---

## 2026-08-25 operator-adaptation wrap (0-pending) — interval point 43 discharged + two counting rules
**Section:** §Dependency Security → CI integration (Standing deferral — `cargo audit` unrunnable, Probe clause)
**Change:**
- Session-43 discharge (2026-08-25): probe RAN, exit read DIRECTLY (never through a pipe) = 1 under cargo-audit 0.22.2, basis byte-identical (`duplicate advisory ID: RUSTSEC-2026-0244`); overlap re-enumerated first-hand — `cargo deny check bans licenses sources` exit 0, `cargo deny check advisories` exit 1 at the SAME 8 owned IDs (0189 / 0190 / 0194 / 0195 / 0204 / 0222 / 0253 / 0258), set unchanged. Was "next at session 43"; now next point session 46.
- Counting rule (a): error BLOCKS are not the ID count — the run showed **10 blocks for 8 distinct IDs** because a crate present at two lockfile versions raises one block per version (quick-xml's 0194 / 0195 each twice); enumerate distinct `RUSTSEC-` ids, never report a block count as an owned-set size.
- Counting rule (b): an interval POINT belongs to the wrap that actually occurs — inserting a wrap moves the count, so a point owed "at the next entry's wrap" falls on the inserted wrap instead.
**Why:** The 0-pending operator-requested route-adaptation wrap; the operator directed the probe was owed here IN FULL FORM because the count moved when this wrap was inserted — rule (b) generalizes that. The body's "next at session 43" was falsified by this probe, so it is corrected to current truth. The deferral does NOT end — the RustSec DB still cannot load. Rule (a) came from re-enumerating first-hand: a naive read reports 10 owned IDs against 8 and manufactures a false "the set grew" alarm. The pin was re-pinned from the Halo entry onto the lifted Demo-injector entry (origin preserved).
**Ref:** NOT DERIVED
---

## 2026-08-25-demo-injector-formalized-api-surface-retire — the categorical path-confinement claim was false; recorded as a measured exception
**Section:** §Security Anti-Patterns → Input (the product-binary `*_PATH` / `*_DIR` ban + its carve-out) · §Input Validation (CLI / env var inputs row)
**Change:**
- Retired: "Everything the product reads stays under the categorical rule". Replaced by a MEASURED EXCEPTION block naming the three product-consumed path vars canonicalized but NOT confined — `ANDROMEDA_PULSE_MODEL_PATH` · `ANDROMEDA_PULSE_LLAMA_CUDA_BIN_PATH` · `ANDROMEDA_PULSE_LLAMA_CPU_BIN_PATH`, all read by the shipped binary via `pulse-app/src/llamacli_inference.rs`.
- The block separates what they DO enforce (trim + reject-empty, `canonicalize()`, assert regular-file — resolving `..` and symlink chains) from what they deliberately OMIT (the data-dir confinement assert; binary paths are intentionally user-managed per the chunk-#84 plan). Consequence stated: an actor who can set these vars can point the product at an arbitrary on-disk file — read as a model, or EXECUTED as the inference binary — a real escalation over the msedgedriver carve-out, whose var never reaches the shipped binary. Bounded to this tier's local-single-user trust boundary; owner named.
- The §Input Validation enumeration, which listed none of the three, now carries them plus the build-only `ANDROMEDA_LLAMA3_TOKENIZER_PATH`.
**Why:** The chunk's live real-L4 leg exercised these vars against the shipped binary, surfacing that the categorical ban had been false since chunk #84 — the harness-only carve-out covers only vars that never reach the product. Applied as-measured rather than widening the carve-out (which would BLESS the exception as policy and hide the CWE-22 consequence) or deferring (leaving the spec measurably false). Operator ruling: bundle the fix with the sibling argv-prompt residual (digest-derived OTLP-sourced prompt text passed as the `-p` argv value) under ONE route entry, "L4 runtime security residuals", since both trace to the same chunk-#84 subprocess design. The gap is a CATEGORICAL claim false about PRE-EXISTING product-consumed vars — a blind class for the input detector, which the Expected-amendments coverage floor exists to catch.
**Ref:** NOT DERIVED

## 2026-08-26-cadence-runaway-blocking-pool — cargo audit interval point 46 discharged
**Section:** §Dependency Security → CI integration → Standing deferral (`cargo audit` unrunnable)
**Change:** Interval point 46 recorded as DISCHARGED IN FULL FORM (2026-08-26): probe RAN under cargo-audit 0.22.2, true exit **1** read DIRECTLY (never through a pipe), basis byte-identical (`duplicate advisory ID: RUSTSEC-2026-0244`, DB still unloadable); overlap re-enumerated first-hand (`cargo deny check bans licenses sources` exit 0; `advisories` exit 1) at the SAME 8 distinct owned IDs (0189/0190/0194/0195/0204/0222/0253/0258), unchanged; counting rule (a) re-confirmed at 10 error blocks for those 8 ids. Was `next at session 46`; now `next at session 49`. The deferral does NOT end.
**Why:** Leaving "next at session 46" would name a discharged point as pending, breaking requirement (d) of the 2026-08-15 routine-BOUNDED-WAIT rule. Dispositioned routine-APPLY-BY-ACTUAL-CLASS (playbook 2026-08-23): the deps detector fired at escalate severity on a stale measured pin while no dependency was added or bumped and the bans/licenses/sources gate is green, so the escalate condition is affirmatively absent.
**Ref:** NOT DERIVED

## 2026-08-26-l4-runtime-security-residuals — L4 path guard lands; the exception narrows, the argv residual closes
**Section:** §Input Validation (CLI / env var row · How column · NEW L4-argv row) · §Security Anti-Patterns → Input (MEASURED→NARROWED exception paragraph, three clauses) · §Threat Model Summary → CLI input trust boundary · §Bootstrap phases → `input-validation-library-install` · §Security Anti-Patterns → Code Patterns · §Security Anti-Patterns → Logging · §Logging & Monitoring → What NEVER to log
**Change:** ELEVEN edits across seven sites, one drift family.
- §Input Validation CLI row: "canonicalized but NOT confined" retired for the three L4 path vars; now the landed guard — traversal-reject BEFORE canonicalize, 4096-byte bound, `canonicalize()`, regular-file assert, plus opt-in confinement under the newly enumerated `ANDROMEDA_PULSE_L4_ALLOW_ROOT`, fail-closed. How column: the `strict-path` mandate narrowed off this boundary — `std` both-sides-canonicalize on the `publish_workspace_key` precedent is what shipped.
- NEW row for the L4 `-p` argv boundary: `validate_prompt_bounded`'s 16 KiB ceiling + NUL/C0/C1 rejection with `\n\r\t` whitelisted, and the ceiling's measured derivation.
- §Anti-Patterns → Input: heading MEASURED→NARROWED; the OMIT clause separates what is now enforced from the data-dir assert still deliberately omitted, consequence SCOPED to hosts that leave the root unset; `Use strict-path` replaced by the primitive actually used; Owner clause DISCHARGED — the route entry landed and the sibling argv-prompt residual is closed.
- §Threat Model: "Path env vars have no declared canonicalization" retired.
- §Bootstrap: two carve-outs from the `strict-path` install mandate, including its declared-but-unused state (five manifests, zero `.rs` users).
- §Code Patterns: the `Command::arg(user_input)` ban names the one product path that does it and the bound admitting it — a SHAPE control, not a confidentiality fix, since the OTLP-derived substring is already scrubbed upstream.
- §Anti-Patterns → Logging + §Logging & Monitoring: basename-only broadened from plugin paths to every product-consumed filesystem path; the two pre-existing full-path boot records (`app.boot.tracing.init` `log_dir`, `app.boot.pid` `path`) are a named carried exception owned by the "Diagnostics un-muting + harness-truth sweep" entry.
**Why:** The chunk shipped the guard (all three arms measured live, including a legitimate out-of-tree model still loading under an enforced root), so every unconfined-convention site was stale. The `strict-path` narrowing follows the P4 primitive decision. The exception is NARROWED, not deleted: confinement is opt-in by design — a data-dir default would break every host that has run L4 — so the arbitrary-file consequence persists where the root is unset.
**Ref:** NOT DERIVED

## 2026-08-26-interpretation-brief-completeness — L4-argv row re-based; citable section joins the content set
**Section:** §Input Validation → "L4 inference argv prompt" row · §Security Anti-Patterns → Code Patterns (the `Command::arg` L4 `-p` bullet — the duplicate derivation site)
**Change:** Both sites now (1) list the `<CITABLE_EVIDENCE_IDS>` section (deduped order-stable `digest.attention_cues[].fingerprint` ids — blake3 full-hex, non-PII by construction, bounded by the cue set) among the `-p` operand's content sources beside `digest.payload_summary` + `project_context`, and (2) re-derive the `MAX_PROMPT_BYTES` (16384) ceiling from the post-citable measurement — prompts 6715..=6830 bytes, ~2.4× the observed maximum with ~9.5 KiB headroom. Was the 154-record 5947..=6297 / ~2.6× basis, retired at both sites. The guard is unchanged and still fires at `validate_prompt_bounded`'s single production caller; the shape-not-confidentiality note and the below-`CreateProcess`-32767 clause stand.
**Why:** The derivation basis moved, and the grown prompt was measured through the guard live (6,830 B max vs 16,384). Applied at both sites in one pass because the bans-section bullet mirrors the row verbatim — a single-site apply would leave the superseded figure standing.
**Ref:** NOT DERIVED

## 2026-08-26-interpretation-brief-completeness — incident-summary write boundary joins the scrub enumeration
**Section:** §Security Anti-Patterns → Logging → Uniform scrubber coverage (MEASURED-reality enumeration)
**Change:** The corpus-persist enumeration gains the incident-summary write boundary: `Incident.resolution_summary_text` holds a `scrub_attribute`-scrubbed JSON projection of the parsed `L4Output`, written at THREE paths — incident creation, dedupe re-generation refresh, and the resolution-summary final write (`scrubbed_l4_json` in `pulse-app/src/inference_runtime.rs`). `ScrubbedValue::Redacted` collapses to the category marker in place (honest-degraded: the report renders the pending notice, never fabricated content). Unchanged: the ring-buffer five-cell set, the 8-category catalog, and the `exception_type` / producer-less exclusions.
**Why:** the enumeration listed only Drain/Baseline/ServiceRegistry/RetryStorm persists + ring-buffer cells, so its coverage set was stale for a store boundary this chunk changed.
**Kept:** the restating sites (§Threat Model / §Data Protection → At rest) carry only the categorical every-corpus-write claim, which this extends rather than retires — no dependent edits owed there.
**Ref:** NOT DERIVED

## 2026-08-27-idle-observer-generation-damper — L4-argv measurement re-base + inference.error posture extension
**Section:** §Input Validation → L4 inference argv prompt row
**Change:**
- Prompt-byte measurement context re-based: the 6715..=6830 B figure is re-labelled the 2026-08-26 sustained-leg reading, joined by the 2026-08-27 readings (reflection 6,932 B — the new observed maximum; sparse-digest v2.2 legs 6,386–6,387 B), all stated as session measurement notes, never bounds; ~2.4× headroom now cited against 6,932 B (~9.2 KiB).
- `interpretation.inference.error` also carries the two formerly-SILENT `InferenceFailed` sites (`io_error` / `stdout_utf8_invalid`) as bounded categories with no output bytes.
**Why:** the chunk's live legs measured a reflection prompt above the documented range's upper end, and the two error sites were de-silenced — the row's redaction-posture claim was incomplete without them.
**Ref:** NOT DERIVED

## 2026-08-27-idle-observer-generation-damper — Code Patterns duplicate re-based in lockstep
**Section:** §Security Anti-Patterns → Code Patterns (the argv-ban L4 `-p` bound sentence)
**Change:** The duplicate prompt-byte claim re-based to the 6,932 B observed maximum (2026-08-27 reflection), keeping the 2026-08-26 sustained range as context, flagged as measurement notes.
**Why:** this sentence restated the §Input Validation row's retired wording verbatim (a dependent of the primary re-base); a single-site apply would have left the stale range alive in the bans section.
**Ref:** NOT DERIVED

## 2026-08-27-idle-observer-generation-damper — standing-deferral point 49 discharged
**Section:** §Dependency Security → Standing deferral — `cargo audit` unrunnable
**Change:** Recorded the session-49 discharge: probe RAN, exit read DIRECTLY = 1, basis byte-identical; overlap re-enumerated first-hand — `bans licenses sources` exit 0, `advisories` exit 1 at the SAME 8 DISTINCT ids (fifth consecutive unchanged probe). Next interval point 52.
**Why:** the interval clause said "next at session 49"; this wrap was session 49 and ran the full-form probe.
**Ref:** NOT DERIVED

## 2026-08-28-ingest-consumer-block-under-gap-resume — point 52 recorded, next point 55, and the running probe ordinal retired
**Section:** §Dependency Security → Standing deferral — `cargo audit` unrunnable (external decay)
**Change:**
- The interval trail gains the session-52 discharge (2026-08-28, chunk `2026-08-28-ingest-consumer-initiating-freeze`: probe RAN full-form, exit read DIRECTLY = 1 under cargo-audit 0.22.2, basis byte-identical, overlap re-enumerated first-hand at the same 8 DISTINCT ids). The trailing pointer was "next at session 52" (stale — naming a point already spent); now "next at session 55 (sessions 53 and 54 are between-points)".
- New **counting rule (c)** retires the running "Nth consecutive" ordinal from probe records; the existing phrasings "fourth consecutive probe" and "fifth consecutive probe" are replaced with "set identical to the prior enumeration".
**Why:** (1) a doc-only staleness correction with no impl half. (2) operator-ruled: two different cadences — the every-3rd-wrap PROBE and the every-wrap OVERLAP — were both narrated as "Nth consecutive", so the counts diverged and drifted; the replacement states the same fact without a number that must be maintained to stay true. This wrap (session 53) is a between-point: basis + overlap re-verified and `probe skipped per ratified interval (next: 55)` recorded in the chunk report, never a silent skip.
**Ref:** NOT DERIVED

## 2026-08-28-duplicate-span-replay-fails-loudly — ureq carve-out, stale skips pruned, session-54 between-point
**Section:** §Dependency Security (Duplicate-version carve-outs · Standing deferral — `cargo audit` unrunnable)
**Change:**
- Added the `ureq` duplicate-version carve-out with provenance and closing condition: `libduckdb-sys` 1.10505's build script pulls `ureq` 3.4.0 while `crates/triage/build.rs` uses 2.12.1; **both are `[build-dependencies]` only**, so nothing reaches a shipped binary; not source-fixable without migrating that build script to the breaking `ureq` 3 API.
- The same bump dropped the second copies of `windows-core`/`windows-result`/`windows-strings`; their skips were **pruned rather than kept** (a stale skip hides the duplicate's return).
- Session-54 between-point appended to the standing-deferral chain: basis + overlap re-verified first-hand, same 8 DISTINCT ids, `probe skipped per ratified interval (next: 55)`.
**Why:** `multiple-versions = "deny"` is never relaxed, so a live carve-out admitting a new transitive dependency must be enumerated where the section claims to enumerate them; the deferral chain must carry every interval point, never a silent skip.
**Ref:** NOT DERIVED

## 2026-08-29-app-registry-reconciliation — cargo-audit deferral, session-55 interval point discharged
**Section:** §Dependency Security → Standing deferral — `cargo audit` unrunnable (external decay)
**Change:** Records the session-55 full-form probe: `cargo audit` exit read DIRECTLY = 1 under cargo-audit 0.22.2, basis byte-identical (`parse error: duplicate advisory ID: RUSTSEC-2026-0244`); overlap re-enumerated first-hand — `cargo deny check bans licenses sources` exit 0, `cargo deny check advisories` exit 1 at the same 8 DISTINCT ids (0189/0190/0194/0195/0204/0222/0253/0258), set identical to the prior enumeration; counting rule (a) re-confirmed at 10 blocks for 8 distinct ids. Next interval point: session 58.
**Why:** session 55 was the owed interval point per the ratified every-3rd-wrap cadence. The deferral does not end — `cargo audit` still cannot load the database.
**Ref:** NOT DERIVED

## 2026-08-29-advisory-backlog — owned set 8 → 0 · strict-path mandate retired (DROP executed) · rmcp pin/mechanism corrected · session-58 probe discharged
**Section:** §Dependency Security → CI integration (standing-deferral clause) + → Pinning (item (b)) · §Input Validation (plugin-host row · CLI/env-var row · MCP stdio row) · §Bootstrap phases → input-validation-library-install · §Security Anti-Patterns → Input (NARROWED EXCEPTION tail) + → Code Patterns (sidecar ban naming) · §Threat Model Summary → MCP vector entry point
**Change:**
- Session-58 full-form probe: `cargo audit` exit read DIRECTLY = 1 under cargo-audit 0.22.2, basis byte-identical — the deferral does NOT end. Owned set re-enumerated FROM SCRATCH was 8, now ZERO owned-and-upgradeable ids (`cargo deny check advisories` exit 0, first green since 2026-08-15; `bans licenses sources` exit 0). The 8-id enumeration retired to history — each upgraded at the source: rmcp 3 · wasmtime 46 · quick-xml 0.41 single · crossbeam-epoch 0.9.20 · anyhow 1.0.104 · lru 0.18 · h2 0.4.19. Next interval point session 61.
- Pinning item (b) closed: rmcp pinned `"3"`, resolved 3.1.4.
- The strict-path install mandate is RETIRED as executed-DROP: `std` both-sides-canonicalize is the single product path primitive; the plugin-host and env-var rows lose their strict-path arms, carve-out (b) is closed, and the Anti-Patterns "intended primitive" clause is retired.
- The MCP row's mechanism corrected to the measured hand-rolled serde JSON-RPC 2.0 layer, with `rmcp` as the feature-gated anchor import; the Threat-Model entry point and the Code-Patterns ban renamed accordingly.
**Why:** the chunk upgraded all 8 owned findings, executed the operator-ratified strict-path DROP (workspace + five member manifests, zero `.rs` users), and measured the rmcp-macro mechanism as never implemented; its wrap was the owed session-58 interval point.
**Kept:** the 4-tool enumerations at the Threat-Model and Code-Patterns sites are left untouched — the pre-existing 4-vs-8 gap stays owned by the Diagnostics-un-muting route entry.
**Ref:** NOT DERIVED

## 2026-08-30-npm-advisory-coverage — npm-channel gate shipped; dev-only blast-radius claim corrected
**Section:** §Dependency Security → CI integration (npm-channel bullet REWRITTEN gap-closed · build-failure roster extended) · §Bootstrap phases → dep-security-ci-gate (roster extended)
**Change:**
- The MEASURED 2026-08-23 npm-gap bullet is rewritten to the shipped state: `cargo xtask check:npm-supply-chain` runs in ci.yml's `supply-chain` job (SHA-pinned setup-node, NO `npm ci` — lockfile-only license/class source at 906/909 coverage); exit contract 0/1/2; policy `pulse-app/ui/npm-policy.json` with mandatory-provenance GHSA-scoped exceptions, per-class license allowlists and a package-name denylist (never duplicate-version deny — measured non-portable at 43/830 multi-version by design). Current state green-with-dispositions: 2 advisory + 2 license exceptions, 0 unexcepted; residual roots GHSA-ggr8-5vv4-36mx + GHSA-jmr9-qjv8-65gv, both no-forward-fix. Prune-not-keep applied live (parse-cache-control); the owning route entry DISCHARGED.
- The whole-channel claim was "dev/harness-only, no runtime or bundle path"; now CORRECTED as measured: it holds for the 29 devDependencies only — 9 runtime `dependencies` (37 runtime-class lockfile packages) Vite-bundle into the shipped webview.
- The build-failure roster and the dep-security-ci-gate bootstrap roster both gain the npm gate, so neither reads Rust-boundary-only.
**Why:** the chunk shipped the gate this section owned as a measured gap, and measurement disproved the dev-only claim. Applied by actual class: no banned/unvetted dependency exists — deny gates green, zero Rust dep changes.
**Ref:** NOT DERIVED

## 2026-08-30-diagnostics-un-muting-harness-truth-sweep — full-path exception retired · dead-schema re-base (ring buffer + corpus) · incident_events no-scrub boundary · MCP tool roster 4→8
**Section:** §Security Anti-Patterns → Logging (full-path bullet · "Two different fives" · MEASURED-reality coverage spans · NEW `incident_events` paragraph) · §Logging & Monitoring → What NEVER to log · §Threat Model Summary (user-content Where + Sensitivity note · corpus Where/Volume · MCP vector Entry point) · §Data Protection (At rest Covers + ring-buffer parenthetical · Data lifecycle Corpus retention) · §Input Validation matrix (MCP stdio row)
**Change:**
- The KNOWN CARRIED EXCEPTION on the never-log-full-paths rule is DELETED at both sites: `app.boot.tracing.init` / `app.boot.pid` now emit basenames via `pulse_app::observability::log_basename` (`log_dir_basename` / `path_basename` / `run_dir_basename` / `data_dir_basename`); the rule is categorical; the boot smoke measured 0 full paths on the un-muted set.
- Producer-less-tables clauses re-based: the `span_links`/`resources`/`instrumentation_scopes` CREATEs are deleted (ring buffer 8→5 tables), so reserved == written and no scrub target can be vacuous by construction; the Threat-Model table enumeration trimmed to the shipped set.
- All `baseline_state` claims re-based to the corpus v2 drop: corpus Holds 5 tables; BaselineState removed from the encrypted-corpus Covers list, the coverage-span enumeration and the retention clause — it persists via the chunk-#72 file-based path.
- NEW: `incident_events` lifecycle writes recorded as a deliberate no-scrub boundary — `event_kind` is a bounded status label, payload empty-encrypted, one row per status value-change at the seven-writer choke point.
- MCP tool roster at the Threat-Model vector and the Input-Validation row corrected 4 → 8 (the chunk-#94 corpus-backed tools were missing at both sites).
**Why:** the chunk landed the basename emits, the schema drops (operator-ruled: DROP both dead-schema sets) and the lifecycle persistence (operator-ruled: PERSIST at the choke point); the carried exception named this chunk as its owner, so leaving it would preserve a discharged obligation. The 4-tool roster predated chunk #94 while every other master already listed 8.
**Ref:** NOT DERIVED

## 2026-08-30-staged-bindings-assertion — staged-artifacts gate joins the CI roster; capability-drift enforcement re-described
**Section:** §Dependency Security → CI integration (build-failure roster) · §Bootstrap phases → `dep-security-ci-gate` · §API Security → TauRPC capability authorization · §Security Anti-Patterns → API
**Change:**
- Build-failure roster gains `cargo xtask check:staged-artifacts` exit 1 (`staged-drift`: staged bindings/grants disagree with `EXPECTED_PROCEDURES`/`staged_gate::EXPECTED_GRANTS`, a staged subject deletion, or an unpinned capability file) and exit 2 (`cannot-evaluate`: git health probe fails); capability-drift folds any non-clean staged outcome into its own FAILURE exit — the roster now covers the commit's STAGED content.
- `dep-security-ci-gate` wires the verb as a plain named `run:` step after capability-drift (no third-party action — SHA-pin rule not triggered; workflow `permissions:` untouched) and re-points the capability-drift wiring sentence at the extended mechanism.
- The §API Security TauRPC row re-describes the shipped gate: worktree + git-index bindings diff PLUS the staged `capabilities/*.json` grants vs the `EXPECTED_GRANTS` semantic triple (both directions; unpinned file / staged deletion red) — a missing or silently widened `core:window:*` grant is caught mechanically in CI.
- The §Anti-Patterns API bullet's enforcement clause extended likewise: silent runtime rejection was the sole stated enforcement for core:window grants; now it is not.
**Why:** the chunk added one fail condition to the roster and the gate was live-proven both directions (fixture reds + real-repo green). Applied by actual class — no dependency added or bumped.
**Ref:** NOT DERIVED

## 2026-08-30-acl-rejection-logging — deferral pointer advanced · `ui.ipc.rejection` no-scrub boundary · clipboard ban de-scoped
**Section:** §Dependency Security → Standing deferral (trailing pointer) · §Security Anti-Patterns → Logging (new boundary paragraph) · §Logging & Monitoring → What NEVER to log (clipboard bullet)
**Change:**
- The standing-deferral record gains session 61's DISCHARGE (full-form probe at 2026-08-30-staged-bindings-assertion; basis byte-identical; overlap re-enumerated from scratch EMPTY, advisories exit 0); trailing pointer advances to session 64 (62/63 between-points).
- §Anti-Patterns → Logging gains the `ui.ipc.rejection` deliberate NO-SCRUB boundary paragraph beside the `incident_events` one: bounded-by-construction triple (closed enum · coerced 4+unknown label · u32 byte count), exact leaf load-bearing (no bare `ui` key), payload/command/raw-text excluded, wire-measured markdown 0× across 6,033 revoked-run lines, pinned ×4.
- The NEVER-log clipboard bullet was scoped to `snapshot.copy_to_clipboard` alone; now EVERY clipboard-payload path, naming the webview `copyMarkdown` failure route that now reports through `record_ipc_rejection` carrying only the bounded triple.
**Why:** the trailing pointer was measured stale (the stale-trailing-pointer class of the 2026-08-28 precedent); the chunk added a new log boundary and the clipboard ban had to reach it. Applied by actual class — no dependency changes.
**Ref:** NOT DERIVED

## 2026-08-30-dead-lib-src-test-migration — snapshot/clipboard no-log arm marked UNVERIFIED (owned)
**Section:** §Security Anti-Patterns → Logging (first NEVER bullet, verification-status note) · §Logging & Monitoring → What NEVER to log (Snapshot file contents + Clipboard contents bullets)
**Change:** The `snapshot.generate` / `snapshot.copy_to_clipboard` resolver arm of the no-log ban now carries an explicit UNVERIFIED qualifier at both restating sites: its only canary (`generate_does_not_log_snapshot_or_clipboard_or_workspace_path_canaries`) was dead-by-construction under `[lib] test = false` and was DELETED (any test binary linking the resolver aborts at load — `STATUS_ENTRYPOINT_NOT_FOUND` via the tauri clipboard/notification import chain), so executed coverage moved 0→0 and the gap is now VISIBLE. The clipboard bullet's wire measurement is scoped to the WEBVIEW `copyMarkdown` route it actually covered. The rule stands; verification is owed via test-plan §1 `snapshot-resolver-level-coverage` (the `mock_builder` lift, blocked by the Wry-typed `AppHandle<Wry>` OnceLock).
**Why:** every neighbouring boundary claim in the section carries an explicit wire measurement, so an unqualified snapshot arm overstated the evidence held. Record-as-open: it records the gap, names the owner, claims no fix.
**Ref:** NOT DERIVED

## 2026-08-30-agent-harness-teardown-truth — pin #22 session-64 FULL-FORM discharge recorded; pointer → session 67
**Section:** §Dependency Security → CI integration → Standing deferral (`cargo audit` unrunnable)
**Change:** Appended the session-64 discharge record (the owed INTERVAL POINT, session count confirmed 63 → 64 at the wrap): probe RAN full-form, exit read DIRECTLY = 1, basis byte-identical (`duplicate advisory ID: RUSTSEC-2026-0244`); overlap re-enumerated FROM SCRATCH — `cargo deny check advisories` exit 0 with the owned upgradeable set EMPTY, `bans licenses sources` exit 0. Trailing pointer was "Next interval point: session 64" (spent); now session 67 (65/66 between-points).
**Why:** the standing-deferral clause's own maintenance shape — a spent pointer left in place is exactly the stale-trailing-pointer failure class its history warns about.
**Ref:** NOT DERIVED

## 2026-09-29-p-025-hue-shift-observable-made-gradable — the `cargo audit` standing deferral ENDED (pin #22 discharged)
**Section:** §Dependency Security → CI integration → the standing-deferral bullet
**Change:** Was a standing deferral — `cargo audit` cannot load the RustSec DB (`duplicate advisory ID: RUSTSEC-2026-0244`), overlap `cargo deny check advisories`, probe every 3rd wrap, a running list of discharged interval points, next point session 67. Now the bullet records the deferral ENDED: `cargo audit` loads the DB (1273 advisories) and exits 0 after the rustls 0.23.45 / quinn-proto 0.11.15 / wasmtime 48.0.3 bumps (RUSTSEC-2026-0285 / -0185 / -0316); it is a plain pass/fail gate again. New: it reads the advisory-db under `$CARGO_HOME`, not `$HOME/.cargo/advisory-db` (a probe of the latter measured a copy 359 commits behind). Kept in the body: the distinct-ids counting rule, the no-ordinal rule, visible advisory dispositions. The probe-point history leaves the body; the entries below and the origin report hold it. Partial retirement of the 2026-08-16 re-ratification, the 2026-08-25 counting-rules and the 2026-08-28 ordinal entries: their rules stand, their probe points do not.
**Why:** the bullet's own terminating clause ("ends the first time `cargo audit` loads") fired at this chunk's gates; the operator relay directed the retirement.
**Supersedes:** 2026-08-17-incident-fingerprint-producer-repaired — `cargo audit` standing deferral: interval point DISCHARGED, re-pinned
2026-08-26-cadence-runaway-blocking-pool — cargo audit interval point 46 discharged
2026-08-27-idle-observer-generation-damper — standing-deferral point 49 discharged
2026-08-29-app-registry-reconciliation — cargo-audit deferral, session-55 interval point discharged
2026-08-30-agent-harness-teardown-truth — pin #22 session-64 FULL-FORM discharge recorded; pointer → session 67
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/

## 2026-09-29-p-025-hue-shift-observable-made-gradable — npm residuals: GHSA-ggr8 pruned, GHSA-7pqw accepted
**Section:** §Dependency Security → npm channel (`pulse-app/ui`) → current state
**Change:** Was residual roots GHSA-ggr8-5vv4-36mx (deepmerge-ts, "pinned <8 by the entire webdriverio 9 line") and GHSA-jmr9-qjv8-65gv (extract-zip). Now both advisory exceptions are extract-zip — GHSA-jmr9 and GHSA-7pqw-9j4j-h8q3 (range `*`, 2.0.1 the latest release, no fixed release anywhere; npm's remedy a rejected pa11y-ci downgrade; dev-only via the puppeteer chains) — and GHSA-7pqw is the one residual accepted at this chunk. GHSA-ggr8 was pruned: its closing condition fired (webdriverio 9.32.0 brings deepmerge-ts 8.0.2). The chunk's dev-only bumps are recorded: vitest 4.1.11 (GHSA-82fw), qs 6.16.0, undici 6.29.0, webdriverio 9.32.0. Exception counts unchanged (2 advisory + 2 license).
**Why:** the no-safe-upgrade class is exactly what the exception form exists for; the operator relay named GHSA-7pqw the one accepted residual.
**Ref:** .andromeda/runs/2026-09-29T15-21-19Z-wrap/

## 2026-09-29-scrubber-path-false-positive — credit_card arm Luhn-gated · npm current state re-read
**Section:** §Security Anti-Patterns → Logging (Uniform scrubber coverage · A THIRD scrub SHAPE · Residual) · §Dependency Security → npm channel → current state
**Change:**
- The `credit_card` arm was "13–19 digits with optional separators, no checksum" (`\b(?:\d[ \-]?){13,19}\b`) under a catalog-wide recall-over-precision posture. Now it is precision-gated: a candidate run of ASCII digit groups joined by single ` `/`-` matches only when some window of consecutive WHOLE groups carrying 13–19 digits passes Luhn.
- Recall over precision is scoped to the other seven arms, both in the catalog paragraph and in the labels failure-mode clause.
- Accepted recall trade: a Luhn-invalid (mistyped) card number, or one fused into a longer single digit group, no longer redacts.
- Closed: a date-time stamp such as the harness data-dir basename (path, `workspace=` key and digest `PROJECT:` forms) and a 19-digit nanosecond timestamp now return `Allowed`.
- Whole-value replacement (P-048) is unchanged.
- The Residual's "catalog unchanged, no arm retuned" is tied to `2026-08-23-ingestion-scrub-coverage`, and the later predicate retune is noted.
- npm current state re-read: `green-with-dispositions`, same four exceptions. Five advisories reported after the prior green run were closed by in-range bumps with no exception: brace-expansion 1.1.21 / 5.0.12, ip-address 10.7.2 (GHSA-6j4f, -qhr7, -q2hr, -h3mg, -j6r3).
**Why:** a date-stamped workspace key redacted as a card, wiping the digest the model read and the Report's Project Context. Every issued card number passes Luhn, so whole-group windows keep recall on real cards. Windows at arbitrary offsets would pass most long runs by chance. This is a boundary widening, ratified by the founder at /phase P4 (2026-09-30). The npm advisories had fixed releases, so the founder's no-deferral, no-exception-when-fixable ruling applied.
**Kept:** the whole-value replacement that blanks an entire digest on one true positive — a founder-ordered span-level redaction route entry owns it.
**Ref:** .andromeda/runs/2026-09-30T06-23-56Z-wrap/

## 2026-09-30-dual-license — cargo-deny license-checks the workspace's own crates
**Section:** §Dependency Security → CI integration
**Change:** The `cargo deny check bans licenses sources` bullet now states that the `licenses` check covers the workspace's OWN crates as well as its dependencies: `deny.toml` sets no `private` key, so cargo-deny's default (`private.ignore = false`) keeps every member in the checked set — as measured at this chunk (a scratch config without `"MIT"` in `[licenses] allow` rejected all 16 members, cargo-deny 0.20.2). The project's own expression `MIT OR Apache-2.0` passes because both `MIT` and `Apache-2.0` sit in `[licenses] allow`.
**Why:** the project moved to `MIT OR Apache-2.0`; the sibling project had to REMOVE a `private = { ignore = true }` exemption to get the same coverage, which this project never set — so the fact is recorded as measured rather than inferred, and `deny.toml` stayed untouched.
**Ref:** .andromeda/runs/2026-09-30T07-44-36Z-wrap/

## 2026-09-30-p-027-discovery-bound — the scrubbed tap name now also keys the lifecycle registry
**Section:** §Security Anti-Patterns → Logging (the `extract_service_name` choke-point paragraph)
**Change:** `extract_service_name` still has THREE consumers (the `spans` column, the storm `FingerprintObserver`, the baseline `SpanObserver` tap). New: the tap drives one `CompositeSpanObserver` fanning each span to baseline · restart · the first-sighting `DiscoveryObserverAdapter`, which registers a baseline-admitted service in the lifecycle `ServiceRegistry` at its first span — so the tap's scrubbed name now also keys that registry directly, where before only `tick_all` inserted it from the baseline's own map. The desync argument now covers the lifecycle registry as well as the baseline registry.
**Why:** the tap's downstream fan-out grew by one observer while the choke point's callers did not, so a consumer count read off the function's callers would miss the new write path.
**Kept:** not a boundary widening — the registry receives the same scrubbed names it already received from `tick_all`, only earlier; no new input class and no new crossing.
**Ref:** .andromeda/runs/2026-09-30T11-29-23Z-wrap/

## 2026-09-30-perf-budget-gate-reads-real-samples — the boot-recorder state files join the harness-only class
**Section:** §Security Anti-Patterns → Input (the harness/xtask-only carve-out) · §Threat Model Summary → CLI input trust boundary
**Change:** the carve-out gains a sibling class — STATE FILES, not env vars: `run/andromeda-pulse.spawn` (app pid) and `run/andromeda-pulse.exit` (one-line end), written by the `scripts/agent-run.{sh,ps1}` boot waiting wrapper (ps1 since this chunk) and read only by the harness — `boot` polls the spawn record ≤ 5 s, and `cargo xtask harness:status` reads the exit record through `read_ended`'s bounded grammar (one line, ≤ 48 printable ASCII). The product never reads either, so canonicalize-and-confine has nothing to guard. Was: the trust-boundary line said "the harness-only tool-locator carve-out is unchanged".
**Why:** the ps1 recorder made these records a cross-shell harness surface; the sh records had never been named in this plan. Validated at the wrap as routine by actual class — no product boundary crossed, the read is grammar-bounded.
**Ref:** .andromeda/runs/2026-09-30T15-36-30Z-wrap/

## 2026-09-30-perf-instruments-measure-their-budgets — record_webgpu_adapter validated input
**Section:** §Input Validation → TauRPC bridge row
**Change:** `telemetry.frontend.record_webgpu_adapter` validates `outcome` as the closed serde `snake_case` enum `WebgpuAdapterOutcome` (5 values; an unknown value is rejected at deserialization, pinned by name) and coerces `window_label` through `coerce_window_label` (4 + `unknown`); pinned in `EXPECTED_PROCEDURES` (44). The body quotes the founder's ratification at P4 2026-09-30, «Да, делай».
**Why:** a new webview → Rust input class; it is a Boundary widening, ratified by the founder at P4, and it is validated at the boundary.
**Ref:** .andromeda/runs/2026-09-30T19-50-57Z-wrap/

## 2026-09-30-perf-instruments-measure-their-budgets — ui.webgpu.adapter no-scrub log boundary
**Section:** §Security Anti-Patterns → Logging
**Change:** NEW deliberate NO-SCRUB webview-originated log boundary `ui.webgpu.adapter`: one record per canvas-mount adapter request (INFO on `obtained`, WARN otherwise) behind its own exact leaf `{outcome, window_label}` beside `ui.ipc.rejection`; nothing client-controlled or free-text crosses it (closed enum + coerced label; the raw adapter / error text excluded); still no bare `ui` key; pinned ×3; its only live witness the dev-host frame leg — no CI job witnesses it. The body quotes the founder's ratification at P4 2026-09-30, «Да, делай».
**Why:** the Logging section enumerates every deliberate no-scrub boundary; this is the second webview-originated one, a Boundary widening ratified by the founder at P4.
**Ref:** .andromeda/runs/2026-09-30T19-50-57Z-wrap/

## 2026-09-30-span-level-redaction — span-level masking replaces whole-value replacement
**Section:** §Security Anti-Patterns → Logging (catalog paragraph · INTENDED posture · MEASURED reality incident-summary clause · identity-column treatment + `metric_name` Residual) · §Threat Model Summary → corpus Sensitivity note · §Data Protection → At rest → Persistent incident corpus
**Change:** Was: whole-value replacement (P-048) — a true positive anywhere replaced the WHOLE value; every boundary passed through `scrub_attribute()`; a redaction collapsed `resolution_summary_text` to the category marker (pending notice). Now: every boundary but `metrics_points.labels` calls `mask_secret_spans`, masking each secret where it sits with a CLASS-AWARE extent — keyed arms (`bearer` · `api_key` · `secret_kv`) to the end of the line, bare arms (`jwt` · `provider_key` · `email` · `ssn`) the whole whitespace token, `credit_card` the Luhn-qualifying digit-group run; overlaps merge under the earliest arm in catalog order; text outside spans byte-identical; a single-token secret masks exactly as before. `scrub_attribute` stays the detection verdict (gates the mask; no locatable span → whole value, fail closed) and the one call at `encode_labels` (labels stay whole-value per label). Masking re-runs to a fixpoint (≤ 4 extra passes). `resolution_summary_text` is masked per STRING LEAF and stays parseable (the training export's `interpretation` likewise). Identity cells and the three persisted service keys are span-masked: a multi-word name keeps its non-secret words; the collision Residual narrows to names differing only inside a masked span (`seq` still separates the rows).
**Why:** one true positive no longer blanks a whole digest the model reads; the class-aware extent keeps never-weaken (a multi-word value after `password:` and a base64 bearer tail past `+` / `/` lie outside the regex match). A Boundary widening — what crosses the store/log boundaries widened from a marker to the value's non-secret remainder — ratified by the founder at P4 2026-10-01, «Ок давай по типу правила». The fixpoint answers a measured single-pass gap (a Luhn-valid run fused to a key word).
**Kept:** `metrics_points.labels` whole-value per label (a keyed match spans the key, so a span mask would erase it); the L1 fingerprint computed from the raw stacktrace; the eight arms and the Luhn gate (detection unchanged).
**Ref:** .andromeda/runs/2026-10-01T11-19-47Z-wrap/

## 2026-10-01-conductor-return — `app.exit` NO-SCRUB boundary and the exit-handler ban
**Section:** §Security Anti-Patterns → Logging · §Security Anti-Patterns → Universal
**Change:**
- Logging: `app.exit` is a deliberate NO-SCRUB product-originated log boundary — one record per process end behind its own exact leaf `{exit_class, exit_code, exit_code_known, signal}` (two closed labels, an `i32` 0 when unknown, a bool); a panic payload, an error `Display`, a native message and every path excluded; no bare `app` key; re-exec arms read 0 full paths and 0 planted canary. Stated failure mode: the ends it cannot see — SIGKILL, `_exit`, Windows `TerminateProcess`, a Rust `std::process::exit` on Windows (`ExitProcess`, no `atexit`), pre-sink failures.
- Universal: NEVER log, or run code needing thread-locals, from a C `atexit` or signal handler on the thread ending the process (glibc destroys its TLS before `atexit` handlers; a panic in an `extern "C"` handler aborts). The `libc` exit FFI hands off to a thread spawned at install with a bounded wait; the Unix signal path records from a runtime task, then restores `SIG_DFL` and re-raises the SAME signal — never an exit code standing in for it.
**Why:** the chunk adds a new log boundary and a new process-lifecycle FFI surface (`libc` as a direct dependency); the closed field set keeps the record bounded by construction, and the ban records the measured failure mode of logging at exit so the hand-off design is not "simplified" away.
**Ref:** .andromeda/runs/2026-10-02T12-54-57Z-wrap/

## 2026-10-02-incident-events-readable-through-mcp — the ninth MCP tool on the threat model and input-validation rows
**Section:** §Threat Model Summary → MCP stdio surface (Entry point) · §Input Validation → MCP stdio inputs row
**Change:** was "Tools: 8" / "all 8 tools dispatched by name"; now 9 — `retrieve_incident_events` (corpus-backed, read-only) joins the roster. Its input `{incident_id: integer}` deserializes through the serde `IncidentIdArgs` with `additionalProperties: false`; its corpus read is a prepared `?1`/`?2` statement bounded at `INCIDENT_EVENTS_READ_LIMIT` = 256 that never selects `payload`; it returns only a coerced closed event-kind label + a timestamp per event.
**Why:** a new crossing on the MCP stdio boundary — a Boundary widening, ratified by the founder at P4 (2026-10-02) for the read surface and by the founder's option A (2026-10-03) for the shared vocabulary it coerces against; recorded here as that ratification.
**Ref:** .andromeda/runs/2026-10-03T23-46-09Z-wrap/

## 2026-10-02-incident-events-readable-through-mcp — incident_events NO-SCRUB basis restated: four kinds, two writers, one reader
**Section:** §Security Anti-Patterns → Logging (the `incident_events` NO-SCRUB paragraph)
**Change:** was "`event_kind` — a bounded status label from the closed `IncidentStatus` set" and "All seven production writers reach it through the one `update_incident_status` choke point"; now `event_kind` comes from the closed four-kind vocabulary `triage::contract::incident_event_kinds()` = `created` / `active` / `acknowledged` / `resolved` (one shared definition, producer and sidecar), and TWO writer paths reach the table — the producer's creation event (`create_incident_from_l4_output` → `IncidentPersistence::save_incident_event` → `CorpusWriter::save_incident_event`) and `update_incident_status`, still the choke point for every STATUS writer incl. the sidecar's `mark_incident_resolved`. Both carry a bounded literal and an empty encrypted payload, so the deliberate NO-SCRUB conclusion stands. Added: the one production READER, the sidecar's `retrieve_incident_events` (`CorpusWriter::load_incident_events`) — `payload` never selected, `event_kind` coerced on egress (else `unknown`), only the label + `occurred_unix_nano` returned, the response body never logged (stderr and file-sink canary absence measured).
**Why:** the paragraph's vocabulary and single-choke-point claims were falsified by measurement — the producer's `created` row predates them (chunk #92) and read `unknown` through the first reader; the no-scrub conclusion holds on the corrected basis.
**Ref:** .andromeda/runs/2026-10-03T23-46-09Z-wrap/
