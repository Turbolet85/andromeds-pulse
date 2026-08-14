# security extract — phase-65

## Chunk relevance

- **chunk #69 Phase A** — Drain Rust spike validation: minimal Drain prototype tested against LogHub corpus subset (Apache web logs + Linux syslog + HDFS application logs). Spike code lives in throwaway branch or `crates/triage-experimental/` (gitignored); does NOT commit to main. Deliverable: `.andromeda/decisions/pre-d2-drain-spike.md` findings doc with PROCEED/REVISE/SPLIT/DEFER decision.

Security domain has **limited but non-zero applicability**: Phase A is a research spike (gitignored, throwaway, no production code path). Most security constraints (Tauri capability gating, AppError sanitization, OTLP receiver hardening, PII scrubber integration, log redaction wiring) are **Phase B scope** and explicitly excluded. The security extract focuses on (a) constraints that apply even to gitignored experimental code given it runs against external corpus data, (b) dependency hygiene for any new crates pulled in by the spike, and (c) anti-patterns to keep visible so Phase A code does not accidentally encode patterns that would block Phase B reuse.

## Constraints

1. **Dependency audit gates apply to spike dependencies if any are added to workspace `Cargo.toml`** — per security plan §Dependency Security CI integration, `cargo audit` + `cargo deny check bans licenses sources` run in `ci.yml` on every PR. Even if `crates/triage-experimental/` is gitignored at the crate-content level, the workspace `Cargo.toml` `members = [...]` entry plus any Drain-related deps added to the workspace (e.g., regex / aho-corasick if used for masking patterns) flow into the lockfile and CI scans. Either (a) keep the spike on a throwaway branch with no workspace member registration, OR (b) any deps added must be SPDX-allowlisted + non-banned + non-duplicate per `deny.toml` policy. Preferred: throwaway branch (zero CI footprint).

2. **LogHub corpus handling — public datasets, no PII concerns from upstream, but no commit to repo** — LogHub is publicly published research corpus (Apache 2.0). Per security plan §Data Protection "data lifecycle" framing, snapshot files persist indefinitely on disk under user control; the same posture applies to spike inputs: LogHub samples used as spike fixtures MUST NOT commit into the repo (would inflate clone footprint + binds `cargo deny check sources` if registry-vendored). Document the LogHub fetch path in `.andromeda/decisions/pre-d2-drain-spike.md` rather than vendoring; spike harness loads from a local cache outside git.

3. **No OTLP attribute values in spike output / findings doc** — per security plan §Logging anti-patterns "NEVER log raw OTLP attribute values, span/log/metric content payloads, snapshot file contents, clipboard contents, or MCP tool response bodies." Phase A's measurements are LOC count, template assignment % match rate, per-event latency p99, template tree memory footprint — pure statistical aggregates. The findings doc at `.andromeda/decisions/pre-d2-drain-spike.md` MUST NOT include verbatim log lines from the spike corpus that could echo PII even from public datasets; if illustrative examples are needed, use synthesized log lines or LogHub-license-cleared excerpts only.

4. **Spike code MUST NOT mutate workspace `Cargo.toml` source list, dependabot.yml, or `deny.toml` allowlist** — per security plan §Dependency Security update policy + §Bootstrap phases `dep-audit-tooling-install` (deny.toml `[bans]` / `[licenses]` / `[sources]` sections are load-bearing). Any add to these files outside a throwaway branch would re-trigger §Dependency Security CI gates against the spike's dependency footprint; throwaway branch avoids the entire chain.

5. **Phase A spike MUST NOT introduce SQL-shaped patterns that could carry forward into Phase B production violating §Code Patterns** — per security plan §Input Validation row "DuckDB query parameters" + §Code Patterns ban "NEVER `format!(\"SELECT … WHERE service_name = '{}'\", user_input)\"". If the spike prototypes any DuckDB lookup (e.g., for diagnostics surface preview), it must use `Connection::prepare` + `?` placeholders even at spike level, so the pattern that lands in `.andromeda/decisions/pre-d2-drain-spike.md` "PROCEED" recommendation does not encode a banned shape into the Phase B production carry-over.

6. **Throwaway branch / gitignored crate semantics MUST be enforced via `.gitignore`** — per security plan §Secret Management `.gitignore` entries for `.env*`, `*.key`, etc. is the established discipline. If the chosen path is `crates/triage-experimental/`, add this directory to `.gitignore` BEFORE creating any spike files (avoid accidental commit of corpus samples or perf-trace outputs that might include incidentally-captured paths to user home directories).

## Patterns to follow

- **Throwaway research branch convention** — per CLAUDE.md "Spike code lives in throwaway branch or `crates/triage-experimental/` (gitignored); does NOT commit to main." Mirrors security plan §Secret Management posture on never committing transient artifacts.
- **Findings doc structure** — `.andromeda/decisions/` directory convention for explicit decision records (mirrors session-handoff.md "Last completed chunk" record pointing to `.andromeda/decisions/pre-d2-drain-spike.md`). Per security plan §Security Decisions Log append-only convention, the findings doc is an audit-trail artifact; once written, do not retroactively edit (correction goes in a follow-up entry).
- **Spike-then-production discipline** — when Phase B (production) lands, it MUST re-derive Drain logic from clean specifications informed by the spike findings rather than promoting spike code as-is. This mirrors security plan §Code Patterns hygiene where production paths require sanitization that throwaway code may skip.
- **Capability-drift gate sleeps during Phase A** — `xtask capability-drift` per security plan §API Security TauRPC capability authorization is irrelevant to Phase A because Phase A excludes TauRPC `diagnostics.template_distribution()` procedure introduction. Phase B will need the standard 4+1-place binding pattern (router registration + `pulse-app/capabilities/` JSON + `xtask EXPECTED_PROCEDURES` + `emit_taurpc_bindings` test merge + arch §Occupied Resources update via `/andromeda-evolve --allow-arch-registry` or `/andromeda-scope-arch`). Note for Phase B planning, not Phase A acceptance.

## Anti-patterns to avoid

1. **DO NOT commit LogHub corpus files into the repo** — public data but inflates clone footprint, may carry CC-BY-NC-SA-style restrictions in subsets, and creates `cargo deny check sources` noise if registry-vendored. Spike harness should fetch LogHub samples from local cache outside git or document a fetch script.

2. **DO NOT add Drain-spike dependencies to workspace `Cargo.toml` `[workspace.dependencies]` table** — Phase A is throwaway; deps in the workspace pollute the production dependency graph + trigger `cargo deny check bans` against the spike footprint. Spike crate keeps its own `Cargo.toml` with deps declared locally; throwaway branch avoids the entire issue.

3. **DO NOT include verbatim log-line excerpts in `.andromeda/decisions/pre-d2-drain-spike.md`** — per security plan §Logging "NEVER log raw OTLP attribute values" pattern extends to decision-record audit-trail documents. Findings doc is statistical aggregates + decision rationale; if illustrative examples needed, use sanitized synthetic log lines.

4. **DO NOT prototype any in-process IPC, network listener, or external process spawn in spike code** — Phase A is a Drain-algorithm validation only. Per security plan §Code Patterns "NEVER `tokio::process::Command::new(...).arg(user_input)`" + §Threat Model "Attack surface" classification, even a throwaway prototype that spawns child processes or opens sockets reintroduces attack surface analysis at Phase B promotion time. Keep Phase A as a pure in-memory CLI / criterion benchmark.

## Contract bindings

- **Binds to tests domain**: Phase A spike has no production tests gate (gitignored throwaway code); test coverage of Drain Phase B production code is tests-domain scope. Security gate `cargo deny` / `cargo audit` does NOT need to pass against spike if spike stays on throwaway branch — coordination point is "throwaway branch" decision, which security and tests both endorse.
- **Binds to obs domain**: spike-level tracing is optional (criterion benchmark output is sufficient for latency measurement). Per security plan §Logging redaction rules, IF the spike emits any tracing instrumentation, it follows the §Logging NEVER-log list (no raw log content, no full paths). Phase B production Drain integration into the L1c log-template-mining layer will need full obs+security PII-scrubbing-at-attribute layer integration via `crates/security`'s scrubber primitive (chunk #68 landed) — that's Phase B scope, not Phase A.
- **Binds to arch domain (Phase B only, not Phase A)**: when Phase B lands, the `triage` crate gains `triage::drain::*` module + the `log_templates` table in DuckDB schema (new reserved table) — arch §Occupied Resources "DuckDB database / schema names" requires `/andromeda-evolve --allow-arch-registry` amendment. Not Phase A scope; flagged for Phase B planning.

## Acceptance criteria contributions

1. **(security) Spike artifacts MUST NOT land on main branch via accidental commit** — verified by: (a) throwaway branch chosen OR `crates/triage-experimental/` added to `.gitignore` BEFORE any spike file written; (b) final review pre-Phase-B confirms `git ls-files | grep triage-experimental` returns empty on main.

2. **(security) No new workspace dependencies appear in `Cargo.lock` after Phase A completion** — verified by: `git diff main..HEAD -- Cargo.lock` shows no additions OR additions exist only on a throwaway branch never merged to main. `cargo deny check bans licenses sources` remains GREEN on main throughout Phase A duration.

3. **(security) `.andromeda/decisions/pre-d2-drain-spike.md` findings doc contains no verbatim log lines from LogHub corpus** — verified by: manual review of findings doc confirms only statistical aggregates (LOC count, latency p99 μs, template % match rate, memory footprint bytes) + decision rationale + sanitized synthetic examples (if any). No raw log-line content from Apache / Linux syslog / HDFS corpus quoted in the doc.
