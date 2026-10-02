## Output Protocol

Rules for iteration agents:
1. Output: patches (old → new) + changelog. NOT full document reproduction.
2. Patch format: `### Patch N: <description>`, `**Old:**` (exact text quoted from the document), `**New:**` (replacement text). Patches must be applicable verbatim — copy `**Old:**` byte-for-byte from `architecture-draft.md` (including punctuation, backticks, and surrounding markdown). If the snippet appears more than once in the document, expand the quote with one line of context above and below until it is unique.
3. Changelog entry shape: `[Iteration N] [substantive|cosmetic] <description>`
4. PROHIBITED:
   - Reproducing the full architecture-draft.md as your output.
   - Restructuring sections without a stated reason tied to a real defect.
   - Labeling whitespace, punctuation, or wording polish as `substantive`.
   - Inventing facts not derivable from existing decisions, conventions, or occupied resources.
5. **Patch budget:** at most 8 patches per iteration. Spend them in priority order — bucket 1 (downstream-blocking, `[priority: high]`) first, bucket 2 (implementation-misleading) only if budget remains, bucket 3 (signal-diluting, `[priority: medium]`) only if the fix is one line and undeferrable. A high-priority patch always preempts a medium-priority patch.
6. **Substantive vs cosmetic:** label a patch `substantive` only if it (a) resolves a contradiction across two sections, (b) elevates information a downstream specialist needs into the section they will look at, (c) closes an open question with a concrete value, or (d) removes content that belongs to a downstream specialist. Otherwise label `cosmetic`. Re-ordering bullets, polishing prose, and renaming for stylistic preference are `cosmetic`.
7. If you find no issues worth fixing: output exactly `No patches` and a single cosmetic changelog entry stating which dimensions you walked, what you cross-referenced, and why nothing fired.

## Analysis Protocol

Do NOT scan-and-patch. Reason in this order:

1. **Whole-document read first.** Read `architecture-draft.md` end-to-end without writing a single patch. Only after the full read may you start dimension analysis. The file is ~324 lines — one targeted Read of the whole document is cheap.

2. **Walk the cross-reference matrix.** For each row below, locate every site where the item appears and confirm the values agree. Disagreements are substantive patches.
   - **Tool/version pins:** Stack and Technologies table ↔ Established Decisions ↔ Inherited Defaults. Spot-check `tonic`, `axum`, `wasmtime`, `duckdb` crate, `tauri-plugin-notification`, `tauri-plugin-updater`, `rmcp` — every pin must match at every site, and every "Caveat to verify / Caveat to reconcile / Documented upgrade path / Deferred" annotation in Decisions must either be acknowledged in Inherited Defaults or be tagged with a concrete trigger condition (a measurable signal, not "if it gets complex").
   - **Workspace crate names:** Design Philosophy crate-count claim ↔ Conventions ("Workspace API style" router list) ↔ Infrastructure Patterns directory tree ↔ Occupied Resources reserved-crate list ↔ Project Intent growth-model claim. The same set of names must appear at every site, and the count word ("eight", "nine") must be arithmetically correct against the list.
   - **TauRPC procedure naming:** the Conventions "Endpoint naming" rule for IPC commands ↔ the Occupied Resources TauRPC procedures list ↔ the Standard Contracts examples (`app_info`, `health`, `ready`). One naming scheme must apply to all of them; if dot-namespacing and bare `snake_case` verbs coexist, the Convention must explicitly authorize both.
   - **DuckDB tables:** Conventions "Database entity naming" example list ↔ Occupied Resources reserved-tables list. Identical names; the example list does not need to enumerate every reserved table, but a name in one list must not contradict a name in the other.
   - **Env vars:** Occupied Resources env-var table ↔ Cross-cutting Patterns "Config management" precedence ↔ each consumer subsystem (which crate honors `ANDROMEDA_PULSE_PLUGIN_DIR` etc.). Every reserved env var should be traceable to one consumer crate; if the consumer is implicit, that is a candidate substantive patch.
   - **Capability identifiers:** Occupied Resources `pulse:*` capability list ↔ Stack/Decisions/Cross-cutting mention of the corresponding feature. A reserved capability with no behavioral spec elsewhere is a substantive gap.
   - **Caveats and upgrade paths:** every "Caveat to verify before locking" / "Caveat to reconcile" / "Documented upgrade path" / "Deferred" marker in Established Decisions must either (a) state a concrete trigger (numeric threshold, named signal, or measurable event) or (b) be acknowledged in Inherited Defaults as a known unresolved item.

3. **Out-of-scope discipline.** If a finding would require writing test cases (tests' domain), OTel span/metric/trace schemas or RUST_LOG format defaults (obs' domain), `aria-` attribute names or specific WCAG conformance claims (a11y's domain), design tokens / component patterns / typography picks / icon set choice (design's domain), threat-model tier or compliance frameworks (security's domain), or naming a concrete frontend framework (React/Vue/Svelte/Solid/Lit) / CSS tool (Tailwind/vanilla-extract/Emotion) / component library (shadcn/Radix/MUI) / test framework (Vitest/Jest/Playwright/Cypress) / logger config beyond the `opentelemetry-stdout` choice already made / auth library — do NOT patch the implementation. Instead, verify the document exposes the *boundary requirement* (the "what must hold" rather than the "how it's wired"). Patch only if the boundary itself is unstated. WebGPU / WGSL / `<canvas>` are arch-appropriate (they bind the visualization runtime) and are NOT specialist territory. Mobile framework is correctly N/A here.

4. **Walk each Analysis Dimension below with the anchor examples in mind.** The anchors are real defects already verified in the document — they are your calibration set for "what kind of issue counts." If you find no issue at the level of the anchor, that dimension does not need a patch this iteration.

5. **Prioritize before patching.** Sort candidate patches into the three buckets in the Output Protocol. Spend the budget top-down. A medium-priority patch loses its slot to a high-priority patch every time.

## Analysis Dimensions

### 1. Internal Consistency [priority: high]

- Does every version pin (`tonic`, `axum`, `wasmtime`, `duckdb` crate, `rmcp`, Tauri 2.x plugins) report the same value at every site (Stack table, Established Decisions, Inherited Defaults)? Where a pin is intentionally absent at one site (e.g., Stack lists `rmcp` without a number), is the absence consistent with the Decision's "Caveat to verify" note?
- Does the count of crates ("eight", "nine") in narrative bullets reconcile arithmetically with the workspace member list in Occupied Resources? Library crates plus the binary crate must add up.
- Does the IPC command naming rule in Conventions reconcile with the IPC procedure list in Occupied Resources, or do two naming schemes coexist without authorization?
- **Adversarial:** if the iteration agent fixes the count word in one place but not another, the next iteration sees fresh drift. Which single sentence is authoritative? Identify the canonical statement and route every other mention to match it; if there is no canonical statement, that itself is the substantive patch.

**Anchor example:** Conventions, "Endpoint naming" rule

> "**Endpoint naming**: OTLP endpoints are spec-fixed (`/v1/traces`, `/v1/metrics`, `/v1/logs`); Tauri IPC commands are `snake_case` verbs (`get_recent_traces`, `generate_snapshot`, `list_plugins`)."

**Issue:** Occupied Resources lists TauRPC procedures using a dotted-namespace scheme — `traces.*`, `metrics.*`, `logs.*`, `snapshot.generate`, `snapshot.list_recent`, `snapshot.copy_to_clipboard`, `plugins.list`, `plugins.reload`, `plugins.invoke`, `mcp.status`, `mcp.start`, `mcp.stop`, `workspace.detect`, `workspace.list`. The Convention example (`get_recent_traces`, `list_plugins`) is a flat `snake_case` verb; the reserved procedures use `<router>.<verb>` with a dot. Two schemes coexist without the Convention authorizing both.

**Why this matters:** `/andromeda-scope-arch` Phase 0 templates IPC procedures from the Convention rule; specialist plans for tests/obs/security copy procedure names from Occupied Resources. With two schemes, the first downstream caller picks one and the second silently picks the other — `taurpc` codegen produces TypeScript that does not match the Rust router definitions, and the bridge fails at runtime with a "method not found" that the agent loop will spend an iteration trying to debug.

### 2. Cross-reference Integrity [priority: high]

- Does every technology in the Stack and Technologies table appear in at least one downstream section (Established Decisions, Conventions, Standard Contracts, Occupied Resources, Cross-cutting Patterns) with a behavioral commitment, not just a name? A Stack row that names a tool but no later section says how/when it is used is a leak — either elevate the policy or mark the row as "name only, contract owned by specialist."
- Does every reserved env var in Occupied Resources have a traceable consumer subsystem (named crate or named subsystem) somewhere in the document? `ANDROMEDA_PULSE_PLUGIN_DIR` should map to `plugins`; `ANDROMEDA_PULSE_OTLP_GRPC_PORT` to `ingest`; `ANDROMEDA_PULSE_MCP_ENABLED` to `mcp-server`. Untraceable env vars are dead reservations.
- Does every reserved Tauri capability identifier (`pulse:default`, `pulse:tray`, `pulse:notification`, `pulse:updater`, `pulse:plugin-fs`) correspond to a behavioral spec (decision, convention, or cross-cutting pattern) elsewhere? A reserved capability with no policy is a placeholder.
- **Adversarial:** if a reserved env var has no consumer, the implementation phase will either (a) silently ignore it — user sets it expecting an effect, gets none — or (b) two crates both claim it and apply different semantics. Which failure mode is currently latent?

**Anchor example:** Stack and Technologies, OS notifications row

> "| OS notifications | `tauri-plugin-notification` 2.x | Native \"Snapshot ready\" toasts (Notification Center / Action Center / freedesktop) |"

**Issue:** `tauri-plugin-notification` and the example "Snapshot ready" toast appear ONLY in the Stack table. Searched the document with three keyword approaches (`notification`, `Notification`, `notify`); no Established Decision rationalizes when notifications fire, no Convention defines the toast text shape or the user opt-out behavior, no Cross-cutting Pattern defines the notification policy, and the reserved capability `pulse:notification` (Occupied Resources line 197) has no behavioral counterpart elsewhere. The Stack row commits to the dependency; nothing else commits to a contract. Cf. `tauri-plugin-updater`, which is acknowledged in Established Decisions ("auto-uploads `latest.json` for the Tauri updater plugin"), Occupied Resources ("Updater channel"), and Inherited Defaults — that is the cross-reference shape `tauri-plugin-notification` is missing.

**Why this matters:** the design specialist will look in Cross-cutting Patterns for "when does the OS notification fire"; security specialist will look for the consent / opt-out model in Occupied Resources or Cross-cutting; a11y specialist will look for the role/announce shape. All three find nothing and either (a) re-derive a policy that conflicts with what the snapshot generator implementer assumed or (b) escalate back to arch as a missing handoff. One sentence in Cross-cutting Patterns ("OS notifications fire on `pulse://stream/snapshot-progress` completion via `tauri-plugin-notification`; user opt-out via `notifications_enabled` in `config.toml`; capability gated by `pulse:notification`") closes the loop.

### 3. Decision Completeness & Open Questions [priority: high]

- For every "Caveat to verify before locking" / "Caveat to reconcile before locking versions" marker in Established Decisions, is the caveat acknowledged in Inherited Defaults (so a downstream reader of the TL;DR sees the open question), and is the trigger for resolution concrete (a `Cargo.toml` to read, a release to track) or hand-wavy?
- For every "Documented upgrade path" marker (e.g., `miette` for plugin diagnostics, `garde` for plugin manifest validation, native `wgpu` for the compact widget), is the activation trigger a measurable signal (numeric threshold, named UX failure, profiling result) or vague ("if it gets complex")?
- For every "Deferred" marker (e.g., `cargo-deny` `bans` rules), is the trigger for un-deferral concrete (when a boundary is breached, named by what evidence)?
- Where Established Decisions splits a behavior across build-time and runtime gates (e.g., `--features mcp-server` plus `ANDROMEDA_PULSE_MCP_ENABLED`), is the failure mode when the runtime flag is set but the build flag was not enabled documented anywhere, or does the user just get silent inaction?
- **Adversarial:** if `tonic 0.14` and `opentelemetry-otlp 0.31` cannot reconcile in `Cargo.toml`, who decides whether to downgrade `tonic`, fork `opentelemetry-otlp`, or wait for the upstream pin to update? Is the decision owner identified, or will the implementation agent pick whichever path it sees first?

**Anchor example:** Established Decisions, [Backend Framework — OTLP receiver]

> "**Caveat to reconcile before locking versions**: `tonic` 0.14 vs `opentelemetry-otlp` 0.31 (which still pins `tonic` 0.13 in some feature combinations) — read both `Cargo.toml`s before tagging."

**Issue:** the caveat is concrete about the verification step ("read both `Cargo.toml`s before tagging") but does not state the resolution policy if reconciliation fails — does the project pin `tonic` 0.13 (matching `opentelemetry-otlp`) and update the Stack table, fork `opentelemetry-otlp` to bump its pin, or wait for upstream? Inherited Defaults line 302 lists `tonic` 0.14.x with no acknowledgement of this open question. Same shape applies to the rmcp 1.5.0 vs 0.3.x caveat in the MCP Server Surface decision (line 52), which is also absent from Inherited Defaults' rmcp mention (line 309).

**Why this matters:** scope-arch will template "implement OTLP gRPC receiver" against Inherited Defaults — which says `tonic` 0.14.x with no caveat — and the implementation agent will encounter a Cargo resolver error mid-phase. The fix at that point requires re-opening arch to resolve the version, costing one full Andromeda cycle. Surfacing the open question in Inherited Defaults plus stating a default fallback ("if reconciliation fails at lock time, downgrade Stack to `tonic` 0.13.x to match `opentelemetry-otlp`") makes the implementation phase resilient.

### 4. Conventions Specificity [priority: high]

- For each error-response schema convention (Tauri IPC `AppError`, OTLP receiver `Status` proto, MCP JSON-RPC error object), are the variants/codes enumerated concretely enough that a downstream consumer can pattern-match without inventing names?
- Does the database-entity naming convention's example list reconcile with the reserved-tables list in Occupied Resources, including any naming differences (singular vs plural, suffix patterns)?
- Does the timestamp-handling convention specify which tables get the sibling `BIGINT ts_unix_nano` column (all tables? span tables only? a stated criterion?), or is "when nanosecond precision is required for spec round-tripping" the only filter?
- Are feature-flag names listed in the Conventions section the canonical set, and do they match the feature-flag names listed in Established Decisions and Cross-cutting Patterns?
- **Adversarial:** if the spans table appears in Occupied Resources as reserved but does not have a column-list sketch anywhere, two scope agents implementing two queries against `spans` will pick different column names. Is the schema sketch present, or is "ring-buffer schema is small" the only specification?

**Anchor example:** Conventions, "Database entity naming"

> "**Database entity naming**: DuckDB tables use plural `snake_case` (`spans`, `metrics_points`, `log_records`, `resource_attributes`); columns are `snake_case`; ring-buffer cutoff is enforced via a periodic `DELETE FROM <table> WHERE ts < now() - INTERVAL '<retention> minutes'` task."

**Issue:** the example list names four tables (`spans`, `metrics_points`, `log_records`, `resource_attributes`). Occupied Resources line 180 reserves seven tables: `spans`, `span_events`, `span_links`, `metrics_points`, `log_records`, `resources`, `instrumentation_scopes`. Two contradictions: (a) the Convention example uses `resource_attributes`; the reservation uses `resources` — different name for what should be the same table; (b) `span_events`, `span_links`, `instrumentation_scopes` are reserved but absent from the Convention example, leaving their naming pattern (plural? `_log` suffix? domain-prefix?) ambiguous to anyone reading only the Convention.

**Why this matters:** the buffer-crate implementation will pick one set of names; the viz-crate query implementation will pick the other. SQL queries written against `resource_attributes` will fail at runtime against a table actually named `resources`. The fix is one of: (a) reconcile the names so both lists agree, or (b) explicitly state the Convention example is illustrative and Occupied Resources is the canonical name list (with a forward pointer).

### 5. Downstream Specialist Handoff [priority: high]

For each downstream specialist plan that will read this arch.md, are the facts that specialist needs elevated where they will look:

- **design specialist:** Design Philosophy bullets currently emphasize developer-tool architecture ("token-efficient curation as a first-class output", "agent-driven development unambiguous"). Is there a brand/visual hint bullet (e.g., "developer-tool aesthetic, dense data-vis, dark-mode default") that gives design specialist a constraint to work against, or is the visual identity entirely greenfield such that design will produce something incompatible with the WebGPU canvas + Tauri tray + OS-notification surfaces already committed?
- **security specialist:** capture surface (`127.0.0.1`-only OTLP binding) is in Occupied Resources; plugin sandbox is in the Plugin Runtime decision; code signing is in Established Decisions. Is the IPC trust model (TauRPC bridge — who can invoke commands, capability JSON enumeration) elevated as a security-facing summary, or is it spread across `pulse:default` / `pulse:tray` capability IDs in Occupied Resources, the `Cross-bridge data shape` in Cross-cutting, and the `Serialize` constraint in the Tauri IPC Bridge decision?
- **tests specialist:** is there an explicit synthetic-telemetry injection point (a TauRPC procedure, a feature flag, a test-mode env var) for end-to-end harness tests, or must tests speak OTLP to `:4317`/`:4318` from outside the process? "Speak OTLP" is a valid choice — but it must be stated, not implicit.
- **obs specialist:** `opentelemetry-stdout` to `~/.andromeda-pulse/logs/` is the self-observation rule and `RUST_LOG` is honored as fallback. Is the log format (JSON vs text) explicit, and is the per-platform log path uniform across Windows/macOS/Linux or is `~/.andromeda-pulse/logs/` a Linux-style path that needs platform-specific resolution (Windows `%APPDATA%\andromeda-pulse\logs`, macOS `~/Library/Application Support/andromeda-pulse/logs`)?
- **a11y specialist:** input modalities (mouse/keyboard via webview, tray menu, OS notifications) are implicit in the Tauri choice. Is "primary input modality is keyboard + mouse via webview, plus tray menu and OS notifications; touch is N/A on the supported desktop OSes" stated, or must a11y infer from the Stack table?
- **Adversarial:** if any one of these handoff facts is absent, that specialist will either escalate back to arch (consuming one cycle) or invent a default that conflicts with another specialist's invention. Pick the specialist with the largest gap and patch only that one elevation per iteration.

**Anchor example:** Design Philosophy section (lines 1–7), all bullets

> "- **Local-first, zero-infrastructure** — every byte of telemetry stays on the developer's machine; no Docker, no collector cluster, no cloud backend means the install-to-first-trace loop is one binary launch, not a stack.
> - **Single-process modular monolith** — eight crates wired into one Tauri binary share memory via tokio channels rather than network hops, so ingest→buffer→viz latency is measured in microseconds and the OS only sees one process.
> - **Standards-track at the edges, opinionated in the middle** — OTLP at `:4317`/`:4318`, MCP over stdio, and WASM Component Model plugins are all spec-conformant so external tooling Just Works; internal contracts (TauRPC bridge, Arrow zero-copy hand-off) are tightly opinionated to keep agent-driven development unambiguous.
> - **Token-efficient curation as a first-class output** — the snapshot generator is treated as a peer of the visualization surface, not a side feature, because turning local telemetry into LLM context is the differentiator.
> - **Capability-scoped extensibility** — WASM Component Model plugins receive only the host imports they declare in WIT, so third-party plugins cannot escalate beyond explicitly granted resources, and the security posture remains auditable."

**Issue:** searched the entire document with eight keyword approaches (`design specialist`, `brand`, `aesthetic`, `visual`, `typography`, `color`, `dark mode`, `dense`). The only matches for "design specialist" are boundary disclaimers — line 77 ("Webview source files follow the design specialist's convention (out of scope here)") and line 261 ("frontend tooling owned by design specialist"). No bullet in Design Philosophy and no Cross-cutting pattern offers a visual-identity constraint. Yet the architecture commits to a WebGPU canvas inside a webview, a tray icon, OS notifications, and a "dense data-vis" implication via the snapshot/visualization differentiator — all of which constrain visual identity. Design specialist will read this draft and find nothing to anchor a token system against.

**Why this matters:** the design plan will produce a token system (color, motion, typography, density) ungrounded in arch. When `/implement` later wires the WebGPU canvas, the design tokens may assume light-mode defaults (incompatible with developer-tool dark-mode expectations), serif typography (incompatible with dense data-vis), or sparse spacing (incompatible with multi-pane dashboard density). One bullet in Design Philosophy ("**Developer-tool surface** — dense data-vis, dark-mode-default, low-chrome chart-first layout; visual identity is owned by the design specialist but must respect this density and color-scheme constraint") gives design specialist the boundary they need without naming a specific token.

### 6. Specialist Content Boundary [priority: high]

- Search the document for any specific frontend framework name (React/Vue/Svelte/Solid/Lit/Preact), CSS tool (Tailwind/vanilla-extract/Emotion/styled-components), component library (shadcn/Radix/MUI/Chakra/HeadlessUI), test framework (Vitest/Jest/Playwright/Cypress/WebdriverIO), logger config beyond the already-decided `opentelemetry-stdout` rule (Pino/Winston/`tracing-subscriber`-fmt-layer-picks), specific OTel span-attribute schema or metric instrument enumeration, specific `aria-*` attribute name, specific WCAG conformance level claim, specific design-token color or typography family, or auth library (NextAuth/Clerk/Auth0/Lucia). Any hit is a candidate substantive removal — those belong to the design / tests / obs / security / a11y specialist plan, not arch.
- The document already declares the boundary at lines 77 ("Webview source files follow the design specialist's convention (out of scope here)") and 261 ("frontend tooling owned by design specialist"). Verify the boundary is honored everywhere — including in any decision rationale or cross-cutting pattern bullet.
- WebGPU / WGSL / `<canvas>` / `navigator.gpu` are arch-appropriate (they bind the visualization runtime). DO NOT flag those.
- Mobile framework is N/A here (desktop only) — verify no mobile framework slipped in by accident.
- **Adversarial:** if a specialist tool name is in arch.md, the specialist plan now has two sources of truth (arch + its own plan). Which wins, and which iteration's patch is the correct one to remove?

**Anchor example:** Conventions, "File naming"

> "**File naming**: Rust files are `snake_case.rs`; one library crate per module under `crates/<module-name>/`; binary crate is `pulse-app/`. WIT interface files are `kebab-case.wit`. Webview source files follow the design specialist's convention (out of scope here)."

**Issue:** this is the **correctly drawn** boundary — it names the Rust + WIT conventions (arch's domain) and explicitly defers webview file naming to the design specialist with an "out of scope here" tag. Use this bullet as the calibration shape: any other place in the document that names a webview-tool / test framework / OTel-attribute-schema / specific design-token / specific a11y attribute should look like this (state the *boundary*, defer the *implementation*) — or be removed entirely. If a search of the document for the framework / tool / library names listed in the dimension question returns zero hits, no patch is required and you can record that in the changelog.

**Why this matters:** arch is the upstream contract. If a specialist tool name leaks into arch, every specialist plan that consumes arch will treat the leak as an architectural commitment, even if the specialist has independently picked a different tool — producing a contradiction the specialist must escalate back to arch to resolve.

### 7. Complexity & Scope Calibration [priority: medium]

- The document states "Scale intent: startup. Single-machine, single-user; no tenancy, no orchestration, no clustering" in Project Intent. Does the heavy machinery (eight library crates + WASM Component Model plugin host + dual OTLP transports + MCP sidecar + WebGPU canvas + EV signing + Apple notarization + three distribution channels) each map to a stated differentiator (token-efficient curation, capability-scoped extensibility, 10k spans/sec throughput, "ship on three OSes with one codebase"), or is some of it cargo-culted from "production OSS desktop" templates without a load-bearing decision?
- The CI matrix (Linux/macOS/Windows × fmt + clippy + xtask test + release smoke) plus three distribution channels (GitHub Releases + Homebrew + Scoop) plus EV signing + Apple notarization is a heavy v0.1.0 release pipeline. Is each piece justified by a decision rationale, or is the pipeline aspirational?
- **Adversarial:** which single piece of infrastructure, if removed, would NOT change any user-visible behavior at v0.1.0? If the answer is "none, every piece is load-bearing," accept the calibration. If the answer is "the Homebrew tap, because no Mac developer asked for it yet," that is a candidate cosmetic deferral — but only patch if it can be a one-line "deferred to v0.2.0" annotation, not a structural rework.

**Anchor example:** Established Decisions, [Distribution Channels]

> "- **[Distribution Channels] GitHub Releases (primary) + Homebrew tap + Scoop manifest**: three channels driven from the same workflow via `taiki-e/upload-rust-binary-action`-style patterns."

**Issue:** the rationale ("three channels driven from the same workflow") explains the *implementation cost* (low, because one workflow file produces all three) but not the *user demand* — there is no stated evidence that Homebrew + Scoop are required for v0.1.0 vs deferred to v0.2.0 once the GitHub Releases channel proves real users exist on each OS. For a "single-user local-first startup-tier app at v0.1.0," GitHub Releases alone is the minimum viable distribution. Conversely, if the differentiator is "agent-driven developer tool" and developers expect `brew install andromeda-pulse` / `scoop install andromeda-pulse`, the rationale should say so. Currently it says neither.

**Why this matters:** if Homebrew + Scoop are aspirational, the release pipeline carries setup cost (Homebrew tap repository, Scoop bucket, sha256 propagation) that delays the first release without buying user reach. If they are load-bearing for the agent-driven-developer-tool brand, that justification belongs in the rationale so it survives a future "let's drop Homebrew" review. One-line patch: append the demand evidence ("matches developer-tool install norms in 2026; lowest-friction install for the agent-driven target user") or add a deferral note ("Homebrew tap and Scoop manifest can be deferred to v0.2.0 if the release-pipeline setup blocks v0.1.0 ship").

---

Read the document, walk the Analysis Protocol, then walk each Analysis Dimension with its anchor in mind. Output patches and changelog per the Output Protocol; respect the 8-patch budget and the priority bucket order.
