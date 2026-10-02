## Output Protocol

1. **Patches Only:** Output changes as patches (old → new), never full document reproduction. Format each as:
   ```
   ### Patch N: {brief description}
   **Old:** {exact text from document}
   **New:** {replacement text}
   ```

2. **Changelog Required:** For each iteration, provide a line in this format:
   ```
   [Iteration N] [substantive|cosmetic] {description}
   ```
   Use `substantive` for content changes that affect test correctness; `cosmetic` for formatting/style only.

3. **Prohibited Actions:**
   - DO NOT reproduce the entire document
   - DO NOT restructure sections without justification
   - DO NOT label cosmetic changes as substantive
   - DO NOT delete or rewrite sections wholesale

4. **No Issues Case:** If the document passes all analysis dimensions, respond with:
   ```
   [Iteration N] [cosmetic] No substantive issues detected. Document is faithful to Phase 1 outputs and tier-appropriate.
   ```

---

## Analysis Protocol

**Chain-of-thought instructions for iteration agent:**

1. **Read** the full document once without annotation to build working context.

2. **Cross-reference** sections against each other. Walk through these concrete checklist items (ordered by downstream impact):
   - **Phase 1 entity → Coverage Map / Sections 4–6:** Every entity in test-scope Section 1 testable table (19 entities) has at least one coverage assignment in Section 2 Test Strategy table, Section 4 Unit tests, Section 5 Integration tests, or Section 6 E2E scenarios. Untestable entities (none in this project per testability column) have boundary test plan, not invented coverage.
   - **Phase 1 surface → Section 6 Driver assignment:** Every surface in test-scope Section 2 (7 surfaces: desktop-webview, desktop-native, OTLP/gRPC, OTLP/HTTP, TauRPC IPC, Real-time Channels, MCP sidecar) has at least one row in Section 6 "Drivers per surface" table with concrete agent-runnable driver (name + version).
   - **Phase 1 critical path → Section 6 Scenario:** All 7 critical paths from test-scope Section 4 (P1–P7: ingest+visualize, curated snapshot, MCP query, plugin lifecycle, widget toggle, real-time push, workspace detection) map to explicit `#### Scenario: {Path name}` headings in Section 6 with Steps, Verification signal, and Cleanup populated.
   - **Phase 1 trigger → addressing section:** All 22 coverage triggers from test-scope Section 5 have at least one addressing section in test-plan Sections 2–11:
     - `security-vector-coverage:` → Section 11 Project-specific or Security subsection (negative test rules)
     - `performance-budget:` → Section 10 Performance Budgets table + Section 6 scenarios with perf assertions
     - `compliance-test:` → Section 10 Compliance Test Coverage (if triggered)
     - `property-based / chaos-test:` → Section 2 Test Strategy table row + Section 6 scenarios
     - `multi-platform-compat / load-test / contract-test / agent-driven-discipline:` → Sections 4–6 or Section 11 universal bans
   - **Tool/version mentions across 3 sites:** Plan body (Sections 4–8) ↔ Test Decisions Log Section 12 (rationale + version) ↔ test-research.md catalog. A tool named in Section 4/5/6 must appear in test-research.md with matching version. Rationale for tool choice must appear in Section 12.
   - **Coverage thresholds consistency:** Section 10 Quality Gates table row "Standard | ≥ 75% | ≥ 70% | ≥ 85%" must match declared test_tier=Standard exactly.
   - **`(See § X)` pointers validity:** All cross-references within test-plan point to actual sections; dangling pointers are substantive bugs. Test-plan does NOT cross-reference obs / a11y / route / setup-project (downstream consumers; do not exist at test-plan generation time).
   - **Section 3 Harness completeness:** All 5-command bodies (`boot`, `run`, `status`, `cleanup`, `logs`) are **concrete** (not placeholders, not `{…}` menus, not "TBD"). Each has: command body (shell/cargo invocation), exit code semantics, output format, timeout or readiness signal.
   - **Section 11 anti-pattern coverage:** Universal subsection has ≥5 agent-driven bans; Project-specific subsection lists ≥3 stack-specific bans grounded in THIS project's OTLP/WASM/TauRPC/DuckDB stack.
   - **Section 10 Performance Budgets table alignment:** If `performance-budget` triggers from test-scope Section 5 are present (they are), the Performance Budgets table must have concrete p50/p95/p99 values for all triggered operations. Not "TBD" or "TBD per perf testing".

3. **Check each dimension** (below) with the anchor examples in mind. If an anchor's condition is found, flag for patching.

4. **Out-of-scope discipline:** If a finding would require writing specific test code (`it()` / `test()` / `describe()` / `#[test]` blocks > 5 lines — tests-pass domain), threat models / auth flows / encryption configs (security's domain), design tokens / component patterns (design's domain), OTel span / metric schemas (obs' domain), ARIA attributes / WCAG rules (a11y's domain), or concrete observability / error-reporting platform picks — do NOT patch. Instead, verify the test-plan exposes the boundary requirement (the 'what test layer must enforce' rather than the 'how it's implemented'). Patch only if the boundary itself is unstated. Test-plan defines test strategy / coverage / harness / fixtures / quality gates; other specialists define their own concerns.

5. **Prioritize** findings by impact: Downstream-blocking (setup-project cannot materialize scripts; Phase 1 coverage incomplete; tools are fabricated) → Implementation-misleading (tier depth mismatch; thresholds misaligned) → Signal-diluting (generic anti-patterns; missing stack-specific bans).

---

## Analysis Dimensions

### 1. Test Scope Faithfulness [priority: high]

- Does every entity in Section 1 Coverage Scope table (19 entities) have explicit coverage assignment in Section 2 Test Strategy table OR Sections 4–6 (Unit/Integration/E2E test names)?
- Are all 7 critical paths (P1–P7 from Section 4 test-scope) present as explicit `#### Scenario: {Path name}` headings in Section 6 with Steps, Verification signal, and Cleanup?
- Do all coverage triggers from test-scope Section 5 (22 triggers across security-vector, performance-budget, compliance, chaos, multi-platform, load, contract, agent-driven discipline) have at least one addressing section in test-plan Sections 2–11?
- **Adversarial:** If an entity or critical path is listed in test-scope Section 1 or 4 but is missing from test-plan coverage map OR lacks a corresponding Section 6 scenario heading (for critical paths), does the iteration agent catch it and flag for addition?

**Anchor example:** Section 1 Coverage Scope table and Section 6 Critical path scenarios

> "**19 entities** (ingest-gRPC, ingest-HTTP, buffer, viz, ui-bridge, snapshot, workspace-detector, plugins, mcp-server, pulse-app, real-time channels, OTLP anti-patterns, Tauri capability gating, updater, loopback binding, path canonicalization, config.toml, DuckDB prepared statements, self-observation loop)" and "#### Scenario: P1 — Receive OTLP telemetry (gRPC), visualize in WebGPU dashboard" through "#### Scenario: P7 — Workspace detection"

**Why this matters:** Downstream iteration agents depend on test-plan coverage being exhaustive and traceable to Phase 1. Missing a critical path or entity means implementation will skip that coverage tier, leaving gaps in test quality.

---

### 2. Tier Calibration [priority: high]

- Does the depth of Sections 2, 4, 5, 6 (Test Strategy, Unit, Integration, E2E) match Standard tier expectations (all sections present; Integration + E2E + Mocking required; no Minimal-tier skips)?
- Are Section 10 Quality Gates thresholds exactly "line ≥ 75%, branch ≥ 70%, function ≥ 85%" per Standard tier defaults?
- Do Sections 3, 7, 8, 9 contain concrete Standard-tier detail (not Minimal stub; not Comprehensive expansion)?
- Does Section 10 Compliance Test Coverage section correctly state "Skipped for Standard tier + Minimal security tier" or list compliance controls if security_tier=Hardened?
- **Adversarial:** If Section 10 coverage thresholds are lowered without explicit justification in Section 12 Decisions Log (e.g., "line ≥ 60%" instead of "≥ 75%"), does the dimension catch over/under-engineering?

**Anchor example:** Section 10 Quality Gates coverage thresholds

> "| Tier | Line coverage | Branch coverage | Function coverage | | Standard | ≥ 75% | ≥ 70% | ≥ 85% |"

**Why this matters:** Tier miscalibration signals conflicting requirements to implementation team — missing Integration layer, or Comprehensive-depth sections in a Standard-tier project, mislead developers about scope.

---

### 3. Tool Anchoring [priority: high]

- Does every named tool/framework in Sections 4–8 (cargo-nextest 0.9, cargo-llvm-cov 0.8.5, tauri-driver 2.x, tonic 0.14.5, axum-test 18.7.0, reqwest 0.12.x, rstest 0.26.1, mockall 0.13.x, httpmock 0.7.x, arrow-rs 55.x, proptest 1.10.0, etc.) appear in test-research.md with matching version?
- For OTLP/gRPC receiver testing (Section 5), does the plan cite tonic 0.14.5 and explain why (native Rust, matches production receiver)?
- For E2E testing (Section 6), does the plan document tauri-driver 2.x as chosen driver for desktop-webview?
- Is every tool's agent-runnable mechanism honored (exit code 0/non-zero, structured test output, JSON-RPC response shape, jq-extractable fields)?
- **Adversarial:** If Section 5 Integration tests name a tool like "tonic-web" that doesn't appear in test-research.md catalog, or names "tauri-driver-pro" (fabricated) with version that cannot be verified, does the dimension flag as fabricated?

**Anchor example:** Section 5 Integration Test Strategy boundary types and Section 12 Decisions Log E2E drivers

> "| ingest → buffer (gRPC TraceService.Export → DuckDB rows) | Send valid gRPC message via `tonic` client; query DuckDB via `Connection::query_map` | `tonic` 0.14.5, `duckdb-rs` 1.5.x |" and "**E2E drivers:** `tauri-driver` 2.x + `WebdriverIO` 9.x for desktop-webview; ... `tonic` 0.14.5 for OTLP gRPC; ... `arrow-rs` 55.x for Real-time Channel Arrow IPC decode"

**Why this matters:** setup-project uses these tool names + versions to materialize CI pipelines and test harness scripts. Fabricated tools cause immediate build failure; version mismatches cascade to fixture incompatibility.

---

### 4. Anti-Pattern Relevance [priority: high]

- Are all anti-patterns in Section 11 grounded in THIS project's stack (OTLP/gRPC on tonic, HTTP on axum, WASM on wasmtime, DuckDB, TauRPC IPC, Tauri desktop app) and security constraints (Minimal tier, loopback-only, no persistent accounts)?
  - **Universal subsection:** Does it have ≥5 agent-driven specific bans (no Percy/Chromatic, no manual smoke, no Selenium IDE recorder, no real network, no real time)?
  - **Project-specific subsection:** Does it have ≥3 stack-relevant bans? Verify these are present: (a) OTLP self-dialing prevention (ANDROMEDA_OBSERVER_URL must not dial back to own `:4317`/`:4318`), (b) raw OTLP dump rejection (snapshot tests assert dedup + anomaly markers, not raw JSON), (c) post-prost invariant checks (span_id/trace_id length validation), (d) loopback-only binding enforcement (reject `0.0.0.0` binds)?
- **Adversarial:** If Section 11 includes a ban like "NEVER use XPath selectors" (irrelevant for IPC-only project) or "NEVER use persistence layer without ORM" (project uses DuckDB directly, ORM N/A), does the dimension catch signal dilution?

**Anchor example:** Section 11 Universal and Project-specific subsections

> "### Universal (agent-driven specific)\n\n- NEVER use Percy / Chromatic / Applitools (visual regression with human review)\n- NEVER include manual smoke step ...\n- NEVER use Selenium IDE / Playwright codegen-recorder ...\n- NEVER include 'human reviews canvas' ...\n- NEVER use real network calls ...\n- NEVER use real time ... without injection\n- NEVER expose secrets in test output\n\n### Project-specific (andromeda-pulse)\n\n- NEVER allow OTLP self-dialing (receiver instrumented with exporter pointing back to own `:4317`/`:4318`)\n- NEVER dump raw OTLP JSON as snapshot (defeats token-efficiency design goal)\n- NEVER skip post-`prost` OTLP invariant checks (span_id != 8 bytes, trace_id != 16 bytes)\n- NEVER bind OTLP receivers to `0.0.0.0` (breaks loopback-only security model)"

**Why this matters:** Anti-patterns guide both test author and code reviewer. Generic patterns dilute the signal; stack-specific bans ensure developers absorb constraints relevant to their architecture.

---

### 5. Downstream Readiness [priority: high]

- **For setup-project:** Can the setup specialist materialize `scripts/agent-run.{sh,ps1}` from Section 3 Test Harness Contract? All 5 commands must be concrete:
  - `boot`: command body (not "start the app"), readiness signal (not "wait for ready"), exit code semantics, timeout
  - `run`: command body (e.g., `cargo nextest run --workspace`), exit code semantics
  - `status`: command body (invoke TauRPC `health`), polled fields (JSON shape with status / subsystems / pid)
  - `cleanup`: command body (kill + port check), idempotency, exit code semantics
  - `logs`: location, format (JSON lines with timestamp/level/target/message), agent-parseable signals (jq / grep patterns)
- **For obs:** Can the observability specialist derive log format constraints from Section 3.3 (Log Format)? Must specify: stream command, JSON structure, agent-parseable signals (error detection, ingest confirmation, startup markers).
- **For route:** Can the routing specialist derive bootstrap items (test framework, harness scaffold, CI gate order) from Section 9 CI Integration (pipeline stages, matrix builds, quality gate conditions)?
- **Adversarial:** If Section 3 `boot` readiness signal is "Poll `health` every 500ms up to 10s timeout; assert `status == ok`" but doesn't specify what JSON fields must be present or what "ok" status looks like, does the dimension catch underspecification?

**Anchor example:** Section 3 Test Harness Contract — 5-command implementation

> "**Command 1: `boot`**\n- **Input:** None (or environment variables...)\n- **Starts:** Tauri desktop application...\n- **Readiness Signal:**\n  - `health` TauRPC command returns `status: 'ok'` and all subsystems report non-error state\n  - OR `ready` TauRPC command returns `ready: true`\n  - Log line: ... receiver bind confirmation logs appear (`listening on 127.0.0.1:4317 gRPC`, `listening on 127.0.0.1:4318 HTTP`)\n- **Exit Code Semantics:** 0 = success; non-zero = startup failure\n- **Observable Port Readiness:** Agent confirms `:4317` and `:4318` accept connections"

**Why this matters:** setup-project is the most dependent downstream skill. Underspecified Section 3 results in `TODO` comments in generated scripts and harness bootstrap failures.

---

### 6. Performance Budget Coverage [trigger: performance-budget triggers present in test-scope Section 5] [priority: high]

- Is Section 10 Performance Budgets table populated with concrete p50/p95/p99 values (not "TBD") for all perf-critical operations?
  - 6 rows minimum: OTLP gRPC RPC, OTLP HTTP RPC, TauRPC `traces.query`, Snapshot generation, DuckDB insert, WebGPU throughput
  - Each row must have concrete numeric targets (e.g., "p99: 100ms") or frame rate targets (e.g., "frame rate >= 30 fps")
- For each perf-critical path, are concrete test scenarios in Section 6 present with perf assertions?
  - Example: WebGPU throughput scenario should include "Inject 10,000 spans/sec for 10 seconds (100k total) → assert buffer ingests all without dropping → assert frame rate stays >= 30 fps"
  - Example: Snapshot token budget scenario should include "Generate snapshot from 5000+ spans with token_budget=25000 → assert response.token_count <= 25000 (strict)"
- Are perf-testing tools cited from test-research.md (criterion.rs for unit-level microbench, custom load-driver for throughput stress)?
- Is the budget assertion mechanism agent-runnable (exit code 0 on pass, p99 latency or frame rate in structured stdout) — not "developer reviews chart"?
- **Adversarial:** If Section 10 Snapshot token budget row says "p99: TBD" or if Section 6 P2 scenario doesn't assert `response.token_count <= 25000`, does the dimension flag as incomplete?

**Anchor example:** Section 10 Performance Budgets table and Section 6 Scenario P2

> "| **Snapshot generation (500 spans, 25k token budget)** | 100ms | 300ms | 500ms |" and "- Step 5: Assert response.token_count <= 25000 (strict)"

**Why this matters:** Perf budgets are contract boundaries between test-plan and implementation. TBD values defer decisions to implementation time, risking regressions. Agent-runnable assertions ensure CI gates catch perf drift automatically.

---

### 7. Cross-Surface Coordination Coverage [trigger: multiple surfaces (desktop-webview, desktop-native, OTLP/gRPC, OTLP/HTTP, TauRPC, Channels, MCP) in test-scope Section 2] [priority: high]

- For multi-surface projects (desktop-webview + TauRPC IPC + OTLP gRPC + OTLP HTTP + Real-time Channels + MCP sidecar), are cross-surface coordination tests explicitly covered in Section 5 Integration Test Strategy AND Section 6 E2E scenarios?
- Do the two cross-surface-coordination triggers from test-scope Section 5 appear as concrete test scenarios?
  - Trigger: "TauRPC ↔ IPC Channels consistency": Subscribe to `pulse://stream/spans` → send gRPC trace → call `traces.query` TauRPC → assert span appears in both (no desync)
  - Trigger: "Snapshot generation + Notification emit": Call `snapshot.generate` → await `pulse://stream/snapshot-progress` → assert notification emitted with token count
- Per surface, is the driver + cross-surface communication verification signal specified?
  - Example: tauri-driver webview + Channel API subscription + gRPC client
  - Example: TauRPC client + Channel async listener
- Are cross-surface anti-patterns present in Section 11 (e.g., "NEVER test desktop-webview in isolation when critical path spans webview + tray + OTLP receiver")?
- **Adversarial:** If Section 5 Integration tests only cover ingest → buffer (single surface) and Section 6 scenarios skip the two explicit coordination triggers, does the dimension catch coverage gaps?

**Anchor example:** Section 5 Integration Test Strategy and Section 6 E2E scenarios

> "| **Real-time IPC Channels (pulse://stream/spans, etc.)** | Tauri IPC Channel subscription in test client | Agent subscribes to channel, triggers ingest event (e.g., send gRPC span), receives binary Arrow IPC payload on channel; agent validates Arrow schema | `arrow-ipc::StreamReader::try_new(bytes, None)` for decoding |" and "#### Scenario: P6 — Real-time push of spans/metrics/logs via Tauri IPC Channel\n- **Steps:**\n  1. Boot app; confirm health OK\n  2. Subscribe to channel `pulse://stream/spans`\n  3. Send 50 synthetic gRPC trace spans\n  4. Await Channel event (timeout 5s); receive binary Arrow IPC payload\n  5. Decode payload; extract RecordBatch\n  6. Assert schema and row count"

**Why this matters:** Cross-surface desync (e.g., `traces.query` returns data but Channel never emits, or vice versa) is a category of bugs easily missed by single-surface unit tests. Explicit coordination tests catch these.

---

Read the document, analyze along all 7 dimensions, output patches and changelog.
