## Output Protocol

Rules for iteration agents:

1. **Output:** patches (old → new) + changelog. NOT full document reproduction.
2. **Patch format:**
   - `### Patch N: <short description>`
   - `**Old:**` followed by the EXACT current text from the draft (verbatim, including whitespace and markdown).
   - `**New:**` followed by the replacement text.
   - Use `### Patch N (insert after "<anchor text>"):` for pure additions.
   - Use `### Patch N (delete):` for pure removals.
3. **Changelog:** every patch gets one line at the bottom of the response in the form:
   `[Iteration N] [substantive|cosmetic] <description>`
   where `substantive` = changes coverage, controls, anti-patterns, tools, versions, justifications, or downstream-derivable content; `cosmetic` = whitespace, prose polish, typo fixes, heading rename without semantic shift.
4. **PROHIBITED:**
   - Reproducing the full document or whole sections that did not change.
   - Restructuring section order, renaming top-level headings, or rewriting the Threat Model Summary verbatim block without a stated defect.
   - Cosmetic changes labeled `substantive` (or vice versa).
   - Editing the `Security Decisions Log` initial entry's historical text — append new dated entries instead.
   - Inventing tools, versions, or CVE IDs not already grounded in the draft or `security-research.md`.
   - Adding content owned by other specialists: no test cases, no design tokens, no OTel span/metric schemas, no ARIA rules, no observability platform picks (Datadog/Sentry/Splunk).
5. **If no issues found:** output the single line `No patches` plus one cosmetic changelog entry stating which dimensions were checked clean.

## Analysis Protocol

Before writing patches, the iteration agent MUST execute these five phases in order. Skipping a phase produces low-value patches that consume the 8-patch budget without raising plan rigor.

### Phase A — Read once, note nothing

Read the entire `security-plan-draft.md` end-to-end. Do NOT mark issues yet. The goal is to load the full document into working memory so the cross-references in Phase B are accurate, not to start patching.

### Phase B — Walk this fixed cross-reference checklist

Go through each item below in order. Each one is a concrete pair of locations in the draft that MUST agree. Treat any mismatch as a candidate finding.

1. **Every Threat Model Summary attack-surface vector ↔ at least one mitigation.** All 9 vectors (loopback OTLP `:4317`/`:4318`, TauRPC bridge, WASM plugin host, MCP stdio, filesystem reads, in-app updater, OS notification/tray, CLI/env vars, webview content) must trace to at least one row in §Input Validation OR one entry in §API Security OR one ban in §Security Anti-Patterns. A vector with no concrete mitigation is dangling and should be patched.
2. **Every data classification ↔ NEVER-log entry.** Each classification entry in TMS Sec 1 (telemetry user-content in DuckDB ring buffer + snapshots, config in `config.toml`, signing/release credentials, third-party WASM plugin binaries) must have its raw form covered in the §Logging & Monitoring "What NEVER to log" list OR the §Secret Management "What counts as secret" list.
3. **Tool/version mentions across 3 sites.** Plan body ↔ TMS verbatim copy block ↔ `security-research.md` catalog. A tool pinned at one site as `(latest)` while another site claims a specific version is a substantive inconsistency. Surface area to walk: `tonic`, `axum`, `prost`, `tower_governor`, `tower_http`, `wasmtime`, `cargo-audit`, `cargo-deny`, `cargo-auditable`, `gitleaks`, `strict-path`, `garde`, `rmcp`, `step-security/harden-runner`, `EmbarkStudios/cargo-deny-action`, `actions-rust-lang/audit`, `tracing` + `tracing-subscriber`, `opentelemetry-stdout`, `tauri-plugin-updater`, `duckdb`, `serde`, `thiserror`.
4. **`(See § X)` cross-references ↔ actual subsection name.** The footers `(See \`## Security Anti-Patterns\` § X for Y bans.)` after Input Validation, Data Protection, API Security, Dependency Security, Secret Management, Error Handling, and Logging & Monitoring must each name a subsection that exists. Dangling pointers are substantive bugs.
5. **Decisions Log open-question N ↔ section that surfaces it.** The 6 open questions in the Decisions Log must each be surfaced (or reflected) in the relevant section body. An open question with no section-body trace is dangling.
6. **Bootstrap-phase ordering ↔ recommended ordering line.** The 9 phases listed in §Bootstrap phases vs the typical ordering line at the bottom of that section. SKIP phases must be omitted from ordering; non-SKIP phases must each appear.
7. **Anti-Pattern CVE anchor ↔ stack relevance.** Every CVE/GHSA referenced in §Security Anti-Patterns must apply to a crate or surface this stack actually uses. A CVE on `zip` is irrelevant if the project does not depend on `zip`.

### Phase C — Score each Analysis Dimension

For each dimension below, scan the relevant section using the dimension's questions and anchor example. Anchor examples show the iteration agent the EXACT class of issue to look for in this document.

### Phase D — Out-of-scope discipline (non-negotiable)

If a finding would require any of the following, do NOT patch — the security plan defines WHAT to protect / log / verify; other specialists define HOW:
- Writing test cases (`it()`, `test()`, "test that X happens") → tests' domain
- Naming OTel span / metric / trace schemas (span attributes, metric names, trace fields) → obs' domain
- Naming `aria-` attribute names or specific WCAG conformance claims → a11y's domain
- Naming design tokens, component patterns, typography picks → design's domain
- Naming a concrete observability / error-reporting platform pick (Datadog, Sentry, Splunk, Bugsnag, Rollbar) → obs' domain

The security plan is allowed to state the **boundary requirement** ("PII must be scrubbed before transmission to external sinks", "non-suppressible toast on clipboard write"). It is NOT allowed to state the implementation ("scrub via `obs::PiiLayer::new(...)` with field list X", "render via `<Toast severity='warning'>`"). If the boundary is unstated, patch it. If the boundary is stated but the implementation is missing, do NOT patch — that's downstream.

### Phase E — Prioritize by patch budget

Patch budget rules within the iteration's 8-patch limit:
- **Bucket 1 (downstream-blocking):** patches that fix dimensions tagged `[priority: high]` and dimension #5 (Downstream Readiness). Spend budget here first.
- **Bucket 2 (implementation-misleading):** patches that fix tier calibration drift, tool fabrication, or supply-chain coherence. Spend after bucket 1 is empty.
- **Bucket 3 (signal-diluting):** patches that fix anti-pattern relevance or generic configs. Only spend if the patch is one-line and undeferrable.

Cosmetic patches (whitespace, typo) only land if they fit in a remaining slot AND no substantive issue exists.

## Analysis Dimensions

### 1. Threat Model Faithfulness [priority: high]

- Does every attack vector listed under **Attack surface** in Threat Model Summary (loopback OTLP `:4317`/`:4318`, TauRPC bridge, WASM plugin host, MCP stdio, filesystem reads, in-app updater, OS notification/tray, CLI/env vars, webview content) have at least one matching control in §Input Validation, §API Security, §Data Protection, or §Secret Management — and conversely, does any control reference a vector that is NOT in the Phase 1 vector list (paraphrase drift)?
- Does every Data classification entry (telemetry user-content in DuckDB ring buffer + snapshots, config in `config.toml`, signing/release credentials, third-party WASM plugin binaries) get matching protection in §Data Protection plus a matching NEVER-log entry in §Logging & Monitoring (e.g., snapshot file contents, OTLP attribute values, plugin file paths, DuckDB query parameter values)?
- Does the Threat Model Summary block copy Phase 1 fields verbatim (tier label `Minimal (0)`, justification, data classifications, all 9 attack vectors, Auth model, Infrastructure, Compliance triggers) — or has the sub-agent paraphrased / dropped the negative-vector entry that establishes the local-only posture?

**Anchor example:** Threat Model Summary, Attack surface, "OS notification / tray surfaces" vector

> "- **Vector:** OS notification / tray surfaces
>   - **Entry point:** `tauri-plugin-notification` (capability `pulse:notification`); tray icon (capability `pulse:tray`). Outbound user-facing only.
>   - **Trust boundary:** Outbound surfaces; not an input vector. Capability-gated."

**Issue:** TMS lists this vector but neither §Input Validation, §API Security, nor §Security Anti-Patterns mentions `pulse:notification` or `pulse:tray` capabilities. Search evidence: `grep -n "pulse:notification\|pulse:tray"` against the draft yields only TMS Sec 2 (lines 73–74) — zero hits in §API Security (lines 170–187), zero in §Security Anti-Patterns (lines 362–425), zero in §Secret Management. The "outbound only / capability-gated" framing is correct but the capability hardening (e.g., NEVER-grant rule for these two capabilities mirroring the `pulse:updater` rule on line 393) is not codified anywhere mitigation-side, leaving the vector dangling per the Phase B item 1 cross-reference.

**Why this matters:** `setup-project` reads the security plan to materialize capability JSON defaults; a vector that lives only in TMS prose with no anti-pattern entry produces a capability file that does not enforce the outbound-only constraint, and a future webview-exposed `pulse:notification` becomes a phishing surface.

**Adversarial:** If a future iteration patches the TMS block to "compress" the outbound vectors line into the webview vector, what does `route` derive — that the OS notification surface no longer exists, or that it was merged? Without a mitigation row anywhere downstream, the surface silently becomes invisible to the bootstrap phaser.

### 2. Tool Anchoring (Catalog ↔ Plan ↔ Pinning) [priority: high]

- Does every named tool/library/middleware in the plan (e.g., `tonic` 0.14.x, `axum` 0.8.x, `prost` 0.14, `tower_governor`, `tower_http`, `wasmtime` 25+, `cargo-audit` 0.22.1, `cargo-deny` 0.19.4, `cargo-auditable` 0.7.4, `gitleaks`, `strict-path`, `garde` 0.20+, `rmcp` 0.3.x, `step-security/harden-runner`, `EmbarkStudios/cargo-deny-action`, `actions-rust-lang/audit`, `tracing` + `tracing-subscriber`, `opentelemetry-stdout`, `tauri-plugin-updater` 2.x, `duckdb` crate, `serde`, `thiserror` 2.x) appear in `security-research.md` catalog with the SAME version pin? Flag any tool referenced without a research-catalog cross-reference, and any version drift between the plan and the catalog.
- For categories with multiple candidates (e.g., `gitleaks` vs `trufflehog` for secret scanning, `garde` vs `validator` for cross-field validation, `cargo-vet` vs `cargo-crev` for review-graph trust), does the plan pick one and document the trade-off, or does any pick appear without rationale?
- Does any GitHub Action reference use a floating tag (`@v2`, `@main`, `@latest`) anywhere in the plan body, even when the surrounding prose claims "pinned by SHA"? A floating-tag literal is the live exposure regardless of the surrounding claim.

**Anchor example:** §Bootstrap phases, `dep-security-ci-gate` bullet (line 239)

> "Wire `step-security/harden-runner@v2` (pinned by SHA) as the first step of every job in `ci.yml`/`release.yml`/`update-channels.yml` with `egress-policy: audit` initially per security-research §StepSecurity harden-runner; promote to `block` with allowlist after a clean audit window. Pin all third-party Actions by 40-char commit SHA per security-research §Pin GitHub Actions by SHA + Dependabot."

**Issue:** The literal `step-security/harden-runner@v2` is a floating major-version tag in the same sentence that claims "(pinned by SHA)" and ends with "Pin all third-party Actions by 40-char commit SHA". The §Secrets ban on line 401 ("NEVER reference third-party GitHub Actions by `@v2` / floating tag — pin by 40-char commit SHA per security-research §Pin GitHub Actions by SHA + Dependabot. The tj-actions/changed-files CVE-2025-30066 (23k repos) demonstrated the retroactive-tag-rewrite attack class.") makes this a self-contradiction within the same plan: the prose forbids `@v2` but the example uses `@v2`. Search evidence: `grep -n "step-security\|harden-runner"` returns lines 239 and 271 only; line 271 says "`harden-runner` egress logging" with no version and is fine; line 239 is the violation.

**Why this matters:** `setup-project` materializes the bootstrap script verbatim from this bullet — copy/pasting `@v2` into `ci.yml` reproduces the exact attack class the same plan bans. A reader following the plan as a recipe ships the vulnerability the anti-pattern forbids.

**Adversarial:** If the iteration agent "fixes" this by rewriting the prose to "(SHA pin TBD)" without supplying a real 40-char SHA, has the patch resolved the bug or just downgraded a substantive defect into a `[TBD]` placeholder that will never get filled? The patch must either supply a pinning convention pattern (e.g., `step-security/harden-runner@<40-char SHA> # v2.10.4`) or escalate to the Decisions Log as an open question — silent removal of `@v2` without a replacement is worse than the current state.

### 3. Tier Calibration & SKIP-Marker Honoring [priority: high]

- Tier is `Minimal (0)`. The plan correctly SKIPs §Authentication & Authorization, §Compliance Controls, full §Supply chain integrity, and §Tamper evidence — does any iteration patch accidentally re-introduce Standard/Hardened content (e.g., user-account auth flows, GDPR-as-controller obligations, full sigstore SBOM pipeline) that would inflate tier rigor inappropriately?
- The plan explicitly INCLUDES §Logging & Monitoring as a focused subset for Minimal tier (justified by Self-observation discipline, snapshot/clipboard/MCP leakage paths, and `logging-redaction-wire` bootstrap). Does the §Logging section stay within that focused scope (NEVER-log list + snapshot/clipboard/MCP hygiene + retention + access controls) or does it drift into Standard-tier security-event audit infrastructure (SIEM integration, tamper-evident log chain, full security-event taxonomy)?
- Several Standard-tier-coded items are explicitly retained at Minimal because they are load-bearing for distribution (Minisign updater signing, Azure Key Vault HSM, `cargo-auditable`). Does each retained item carry an explicit "load-bearing even at Minimal tier" justification, or do any newly added retained items lack that rationale?

**Anchor example:** §Data Protection section header (line 139)

> "_[Light for Minimal — `HTTPS enforced` + `no sensitive data stored` may suffice;
> full for Standard / Hardened]_"

**Issue:** This is the generic Minimal-tier hint copied from the template, but the section that follows is NOT light — it includes `tauri-plugin-updater` Minisign verification posture, Azure Key Vault HSM custody, and a per-medium at-rest table for 5 mediums. The hint contradicts the actual content. Search evidence: lines 142–166 contain ~25 lines of substantive Data Protection material, far beyond `HTTPS enforced` + `no sensitive data stored`. The retained content is correct (load-bearing for distribution channel) but the section header still claims it "may suffice" with two trivial bullets, which signals to a future re-runner or reader that the section is overbuilt and could be trimmed — risking accidental deletion of the load-bearing pieces.

**Why this matters:** A future iteration agent reading the `[Light for Minimal — … may suffice]` hint may try to "right-size" by deleting the per-medium at-rest detail or the Code-signing key custody paragraph (line 164–166), regressing the Minimal-tier-retained controls flagged as load-bearing. The header should be patched to acknowledge "Minimal-tier-retained for distribution-channel load-bearing detail" so the marker reflects reality.

**Adversarial:** If a future Standard-tier re-tier happens, does the hint's "full for Standard / Hardened" actually communicate what's missing — backups encryption, log-chain tamper evidence, KMS rotation logs — or is it so generic that the re-tier produces a Standard-tier section with the same content as Minimal? The hint should either be specific about what the Standard delta would add or acknowledge that the Minimal version is already substantively populated.

### 4. Anti-Pattern Stack Relevance [priority: medium]

- Each `### {Domain}` block under Security Anti-Patterns must be grounded in THIS Tauri 2 / Rust / `wasmtime` / OTLP / DuckDB / GitHub Actions stack. Are bans tied to specific 2025–2026 CVEs that apply to crates this stack actually uses, or do any CVEs reference crates not in `Cargo.lock` (i.e., the CVE is real but irrelevant)?
- The §Authentication ban subsection is intentionally empty because auth approach=none and capability-system bans cover the substituting authorization model. Does any iteration accidentally insert auth-token-rotation / JWT-secret-handling / OAuth-callback bans that don't apply to a local-first single-user app with no user accounts?
- Are bans appropriately centralized in `## Security Anti-Patterns` and only cross-referenced from section bodies, or has any iteration inlined a duplicate ban inside a section body, creating two places to maintain it?

**Anchor example:** §Security Anti-Patterns, §Input, line 375

> "- NEVER read `ANDROMEDA_PULSE_*_PATH` / `*_DIR` env vars without `std::path::Path::canonicalize()` + an assertion that the canonical path starts under the resolved per-platform data dir (or under the explicit override base if `ANDROMEDA_PULSE_DATA_DIR` is set). CVE-2025-29787 (zip crate symlink-chain extraction) and CVE-2025-68705 (RustFS path traversal) demonstrate this is a live class."

**Issue:** CVE-2025-29787 is anchored to the `zip` crate, but this project's stack does not use the `zip` crate. Search evidence: `grep -ni "\\bzip\\b\\| zip-rs" /dev_projects/andromeda-pulse/.andromeda/runs/2026-05-02T08-52-16-security/security-plan-draft.md` returns zero matches. The project's distribution path is `tauri-action` → `.msi`/`.dmg`/`.AppImage`/`.deb` (line 96) plus Tauri updater Minisign — none of which depend on the `zip` crate. The CVE is real and the underlying class (symlink-chain path traversal) is correctly identified, but the anchor CVE is irrelevant for THIS stack. CVE-2025-68705 (RustFS path traversal) is also a non-stack crate, but it's the second anchor; if both anchors are non-stack, the ban becomes a pattern-only assertion that "trust me, this is a real class" rather than a CVE-grounded warning the iteration agent can cross-reference.

**Why this matters:** Anti-Patterns are read by `tests` for test-scope derivation and by `setup-project` for capability rule materialization. CVE anchors give those skills the "why this specific check matters" hook. A non-stack CVE anchor either misleads `tests` into writing a test against the wrong attack vector, or signals to a reader that the ban is over-cautious / ignorable. The fix is to swap to a `walkdir` / `tokio::fs::canonicalize` / `tauri-plugin-fs` symlink-chain CVE that's actually in the dependency graph, or to drop the CVE anchor and replace with the underlying CWE (CWE-22 path traversal) so the ban is honest about being class-based, not CVE-grounded.

**Adversarial:** If `setup-project` materializes a `.claude/rules/` rule that says "block symlink chains because zip crate CVE-2025-29787" but the codebase has no `zip` dep, does a developer reading the rule conclude "this rule was copy/pasted from a template, the actual codebase isn't affected, I can ignore it" — and disable the canonicalization the rule was meant to enforce?

### 5. Downstream Readiness [priority: high]

Per D26 cross-cutting read chain, the security plan is read by `route` (bootstrap phase ordering), `setup-project` (materializing `@` imports + secure defaults), `tests` (test SCOPE — not cases), `obs` (PII scrub + redaction constraints), `design` (secure UX requirements), `a11y` (secure form patterns), `security-pass` (per-phase enrichment), and `combine` + `/implement`.

- Can `route` derive bootstrap phase ordering directly from §Bootstrap phases? Verify each non-SKIP phase has a concrete actionable scope (named libraries + workflow files + concrete steps) and that no phase is a placeholder.
- Can `setup-project` materialize `@` imports + secure defaults? Specifically, can it derive concrete rule files (e.g., a `.claude/rules/` rule on `strict-path` canonicalization for `*_PATH`/`*_DIR` env vars, on `wasmtime::ResourceLimiter` + `epoch_interruption` config, on TauRPC `AppError` enum collapse, on `tower_governor` per-source-port keying, on Host-header allowlist middleware, on `pulse-app/capabilities/` xtask drift check) — verify the Bootstrap phases or relevant section bodies provide enough specificity for each rule.
- Can `obs` derive its PII scrub list and structured log format constraints from §Logging & Monitoring NEVER-log + §Error Handling Internal logging? Verify the NEVER-log list is complete enough to pin obs's redaction layer — and that the plan does NOT prescribe span schema / metric names / trace fields (those belong to obs).
- Can `tests` derive test SCOPE from Anti-Patterns + Threat Model Summary + Bootstrap phases without the plan itself enumerating test cases?
- Can `design` and `a11y` derive what they need? Auth approach=none means no MFA/recovery UX surfaces; verify the plan does NOT inadvertently inject design/a11y obligations beyond the user-facing snapshot warning, the non-suppressible "X bytes copied" toast on `snapshot.copy_to_clipboard`, and the README plugin-install warning.

**Anchor example:** §Bootstrap phases, recommended ordering line (lines 242–245)

> "route uses this list to plan phase ordering (typical ordering for this Minimal-tier Tauri stack:
> input-validation-library-install → dep-audit-tooling-install →
> secret-management-init → secret-scanning-ci-gate → error-sanitization-wire →
> logging-redaction-wire → dep-security-ci-gate). setup-project uses this list
> to materialize each phase's bootstrap script + dependency list + CI workflow
> snippet."

**Issue:** The recommended ordering correctly omits `auth-scaffolding-baseline` and `supply-chain-signing-init` because both are SKIP. However, `route` reads this line to plan bootstrap phasing and the line gives 7 phases in a single arrow chain — but the bullet list above (lines 232–240) defines 9 phases including the 2 SKIPs. There is no explicit annotation that the omissions are intentional SKIPs vs simply missing from the recommended ordering. Search evidence: `grep -n "ordering\|SKIP for Minimal\|auth-scaffolding-baseline" lines 224–248` shows the SKIP markers live only on the bullet definitions, not on the recommended-ordering line. A future iteration agent reading just the recommended line and not the bullets above could conclude "auth-scaffolding-baseline got dropped, let me add it back." The fix is a one-line annotation: "(auth-scaffolding-baseline + supply-chain-signing-init are SKIP per their bullet markers and intentionally omitted from this ordering)."

**Why this matters:** `route` derives phase ordering from this line; an inconsistency between the bullet list (9 phases, 2 marked SKIP) and the recommended-ordering line (7 phases, no SKIP annotation) makes route's derivation ambiguous — does route re-add the SKIPs, treat them as deferred, or error on the mismatch? Each path produces a different bootstrap plan downstream.

**Adversarial:** If the project later upgrades to Standard tier, what triggers a re-population of the SKIP'd phases — a re-run of `/andromeda-security`, a manual edit, or `route` itself? If the recommended-ordering line is the only place `route` looks, will `route` ever re-add the SKIP'd phases on a tier change, or will it forever follow the 7-phase Minimal ordering even after the bullets are upgraded?

### 6. API Hardening Completeness [priority: medium] [trigger: API surface present — loopback OTLP receivers + TauRPC bridge + MCP stdio per Threat Model Summary Sec 2]

- Does the §API Security controls table cover every relevant control for THIS API surface (rate limiting via `tower_governor` on `:4317` + `:4318`, CORS deny-by-default on `:4318`, Host-header allowlist on `:4318`, CSP for webview, request size limit for both OTLP transports, TauRPC capability authorization, updater capability isolation, plugin host capability sandbox, MCP feature double-gate)? Flag any missing control for a vector listed in TMS.
- Are configurations stack-specific and named (e.g., `tower_governor` `tower::Layer` install, `axum` `DefaultBodyLimit::max(8 * 1024 * 1024)`, `tonic` `.max_decoding_message_size(8 * 1024 * 1024)`, `tower_http::cors::CorsLayer::new()` with no `allow_origin` calls, `axum::middleware::from_fn` for Host header, `wasmtime::ResourceLimiter` per-`Store`, `tauri.conf.json` `app.security.csp` literal string) — not generic "configure rate limiting"?
- For non-REST surfaces (gRPC `:4317`, TauRPC, MCP stdio per arch.md), is the adapter note acknowledged with surface-specific control mapping, and is the CSRF row correctly N/A (no cookies, no browser session for OTLP; capability-based for TauRPC) without iteration patches "fixing" this by inserting CSRF middleware that doesn't apply?

**Anchor example:** §API Security, CSP row (line 179)

> "| CSP (webview content) | Recommended `tauri.conf.json` `app.security.csp`: `\"default-src 'self' ipc: http://ipc.localhost; img-src 'self' data:; style-src 'self' 'unsafe-inline'; script-src 'self'; connect-src 'self' ipc: http://ipc.localhost; worker-src 'self' blob:\"` — `worker-src blob:` required for WebGPU compute pipelines. NO remote content per Cross-cutting Patterns (no CDN). | Tauri 2 framework-level CSP; local scripts get hashed, styles + external scripts use cryptographic nonce. Configured in `pulse-app/tauri.conf.json`. |"

**Issue:** The CSP literal `script-src 'self'` and `style-src 'self' 'unsafe-inline'` are inconsistent with the Stack-reference column's claim "local scripts get hashed, styles + external scripts use cryptographic nonce." Search evidence: the CSP literal contains `'self'` for script-src and `'self' 'unsafe-inline'` for style-src — neither contains a `'nonce-...'` placeholder, a `'sha256-...'` hash, or a `'strict-dynamic'` directive that would activate hash-or-nonce semantics. Either the literal needs to be patched to include `'nonce-{NONCE}'` placeholders that the Tauri build pipeline substitutes, or the Stack-reference prose needs to drop the "hashed / nonce" claim and acknowledge that this CSP relies on `'self'` origin matching plus `'unsafe-inline'` for styles. As written, `/implement` materializing this `tauri.conf.json` would ship `'unsafe-inline'` for styles (a real exposure) while the prose claims nonces are in use.

**Why this matters:** `/implement` reads this row verbatim into `pulse-app/tauri.conf.json`. Shipping `'unsafe-inline'` for styles is a real CSP weakness the prose claims is mitigated; the gap between literal and claim becomes a shipped bug. Either the literal must reflect nonces (and Tauri's CSP nonce-injection mechanism must be wired in `setup-project`), or the prose must reflect that `'unsafe-inline'` is the conscious choice with a stated risk acceptance.

**Adversarial:** If a Tauri 2 minor version changes the CSP nonce-injection mechanism (e.g., from build-time substitution to runtime injection), does this row still produce a valid CSP, or does the literal break silently because the `'self'` script-src no longer matches the nonced scripts the build now emits? The literal needs either a Tauri-version pin or a "regenerate via `tauri build` rather than copy/paste" annotation.

### 7. Supply Chain Coherence (Minimal-tier-retained) [priority: high] [trigger: distributable signed binaries via Tauri updater + Homebrew tap + Scoop manifest, even at Minimal tier]

Although the tier is Minimal and the full Standard-tier `### Supply chain integrity` block is correctly SKIPPED, the project ships signed binaries through three channels (Tauri updater, Homebrew tap, Scoop manifest). The Minimal-tier-retained controls below MUST stay coherent across §Data Protection (in transit), §Secret Management, §Dependency Security, and §Bootstrap phases.

- Does the plan keep `cargo-auditable` 0.7.4 wired into `tauri-action` build (via `RUSTC_WRAPPER=cargo-auditable` or `cargo auditable build --release`) so post-release `cargo audit bin <bundle>` works against shipped `.msi`/`.dmg`/`.AppImage`/`.deb` artifacts? Does it justify why `cargo-vet` and `cargo-geiger` are deferred (per security-research right-sizing) instead of silently dropped?
- Does §Secret Management correctly centralize code-signing key custody (Windows EV via Azure Key Vault HSM-RSA, Tauri updater Minisign Ed25519, Apple Developer ID) under Azure Key Vault Premium SKU + GitHub OIDC federation — with no long-lived service principal secret in GitHub Actions? Verify the rotation cadence covers each key (Minisign keypair runbook, EV cert per CA policy, Apple Developer ID per Apple lifecycle, Azure OIDC federation trust, GITHUB_TOKEN scoping) and the `production-release` Environment manual-approval gate appears in the access path.
- Does §Bootstrap phases retain `step-security/harden-runner` (pinned by 40-char SHA, NOT `@v2`) on every job in `ci.yml`/`release.yml`/`update-channels.yml` with `egress-policy: audit` and a documented promotion path to `block`? Does it retain the `cargo deny check bans licenses sources` gate (load-bearing for the documented `tonic` 0.14 ↔ `opentelemetry-otlp` 0.31 / `tonic` 0.13 duplicate-version risk, and for the Homebrew/Scoop license-downgrade incident class)?
- Critical CVE response SLA (72h public-advisory → Dependabot PR merge → tagged patch release → updater + Homebrew/Scoop pipelines) is the realistic distribution-channel SLA. Verify it stays consistent with the rotation cadences and is not contradicted elsewhere.

**Anchor example:** §Dependency Security, Pinning paragraph (line 197)

> "**Pinning:** `Cargo.lock` checked into the repository (standard for binary crates). Workspace pins per Stack table. Open reconciliation items from Established Decisions: (a) `tonic` 0.14 vs `opentelemetry-otlp` 0.31 (which still pins `tonic` 0.13) — must be resolved before tagging v0.1.0 per the Inherited Defaults open question; (b) `rmcp` \"1.5.0\" reference vs published 0.3.x line — reconcile per the same Inherited Defaults open question."

**Issue:** This paragraph correctly surfaces the two open reconciliations and ties them to `cargo deny check bans` enforcement (line 203 + Universal ban line 425). However, the §Input Validation table row for MCP stdio (line 130) writes `rmcp` (architecture-pinned 0.3.x or higher per Established Decisions caveat)` while the Decisions Log open question (line 444) writes `rmcp` "1.5.0" vs published 0.3.x line` — three different version representations across three sites: "1.5.0" reference, "0.3.x line", and "0.3.x or higher". Search evidence: `grep -n "rmcp"` returns lines 61, 130, 197, 338, 417, 444 — and the version string differs between line 130 ("0.3.x or higher") and line 197 ("1.5.0" reference vs "0.3.x line"). For the Phase B item 3 cross-reference (tool/version mentions across 3 sites must agree), this is a divergence. The reconciliation is correctly flagged as an open question, but the plan body itself uses inconsistent version strings — and `setup-project` reading line 130 will install `rmcp = "0.3"` while a maintainer reading line 444 will think the answer is "pick between 1.5.0 and 0.3.x".

**Why this matters:** `setup-project` materializes Cargo.toml deps from §Input Validation row Stack column. If line 130 says "0.3.x or higher" and line 444 says "1.5.0 vs 0.3.x", `setup-project` picks `0.3.x` and the open-question reconciliation never gets resolved against the published 1.5.0 candidate — it gets pre-emptively closed by setup-project's pick. The fix is to align the three site references on one version representation and surface the choice (e.g., "pinned to `rmcp = \"0.3\"` pending Inherited Defaults reconciliation; published 1.5.0 line is incompatible per security-research §rmcp").

**Adversarial:** If `cargo deny check bans` is the enforcement mechanism for both open reconciliations (a) and (b), and the iteration agent never patches the `tonic` 0.14 ↔ 0.13 duplicate, does CI actually fail at v0.1.0 tag — or does `deny.toml` need explicit `[[bans.deny]]` entries for the duplicate, and does the plan specify those entries? The bullet on line 234 says `Add deny.toml at workspace root with bans, licenses, sources sections` but does not specify what's IN the bans section. `cargo deny` will not catch the duplicate unless the duplicate is explicitly banned.

Read the document, walk Phase A through E, output patches and changelog.
