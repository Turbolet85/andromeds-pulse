# Codebase Research — 2026-10-10-capability-record-re-based

## Scope
- **Depth:** moderate · **Reads:** 31 · **Globs/Greps:** 27
- **Harness rules consulted:** none — no live leg in this chunk (it boots nothing and drives no external process).
- **Platform issues consulted:** none — no runner-only bullet (the one CI read of Setup 5a is an open run, not a
  red) and no CI-reading entry outside the operator leg.
- **External inputs:**
  - `inputs#I1` — the operator directive for this phase (seven items).
  - `inputs#I2` — Conductor's route line 56, the answering entry `Accepted capability set re-based` (v4-06).
  - `inputs#I3` — Conductor's copy of the old gate record. Read at P3 against this tree: the two files differ in
    three `notes` strings only (`diff` of both through `python -m json.tool`: three changed lines, 420, 463, 789,
    each a "Dynamic-verification (Conductor)" note); every id, title, category, mode and scenario is equal.
  - `inputs#I4` — Conductor's copy of the capability spec, byte-equal to this tree's (`cmp`, exit 0).
  - `inputs#I5` — Conductor's pin, `contracts/pulse-capabilities.toml`: three keys, `sut_version = "v0.3.0"`,
    `captured_at = "2026-08-08"`, `capabilities = [...]`, a flat list of 82 ids P-001…P-082.
  - `inputs#I6` — the pin's reader, `crates/conductor-core/src/capability_manifest.rs`: the three fields, every id
    `P-NNN`-shaped and unique, `accepts(p_id)` is list membership. Its doc comment: re-aiming at a newer release
    "is an edit to that file, not a Rust change".
  - `inputs#I7` — Conductor's requirement v4-06: its accepted set, its classification and its coverage record are
    derived from the re-based record and name no retired capability; "where Pulse's record is not yet written,
    Conductor's stays unbuilt rather than guessed".

## Files inspected
- `xtask/src/main.rs` (240-247, 259-338, 760-972, 1021-1075) — the gate. `verify_capability_matrix` (`:774`)
  reads the fixed path `docs/v0_2_0/capability-verification-matrix.json`, demands exactly P-001…P-060 once each
  (`:917-923`), validates each entry's `verification_mode` and scenarios (seven kinds, eight known xtask gates),
  writes `target/capability-matrix/report.json`, prints one JSON event and the line
  `verify:capability-matrix: {state} ({n}/60 capabilities, {k} violation(s))` (`:958`). Exit 0 clean, 1 on a
  violation (`:967-971`); an absent or unparseable file is an `Err`, which `main` prints as `xtask error:` and
  maps to exit 1 (`:330-336`) — no arm of its own. The function has no test: `git grep` for
  `verify_capability_matrix` and `capability-matrix` under `xtask/src` finds only `main.rs` itself.
- `xtask/src/main.rs` (`:1021`, `mod empty_input_tests`) — the pattern for per-arm pins over a `tempfile::TempDir`
  root; `tempfile` and `serde_json` are already `xtask` dependencies (`xtask/Cargo.toml`), `serde` derive and
  `toml` are not.
- `.github/workflows/ci.yml` (`:105-106`) — the step, in `lint-test`, a plain `run:`; its display name reads
  "(chunk #99 — P-001..P-060 scenario mapping)".
- `pulse-app/tests/a11y_perf_workflow.rs` (`:146-153`) — `ci_workflow_invokes_xtask_verify_capability_matrix`
  asserts that `ci.yml` contains `cargo xtask verify:capability-matrix`; its message says "P-001..P-060".
- `pulse-app/tests/quality_gate_workflow.rs` (`:241-246`) — the soft-fail ban's listed commands; the verb is not
  among them (the generic no-`continue-on-error` arm still reads every step).
- `docs/v0_2_0/capability-verification-matrix.json` (whole, through a listing script) — 60 entries; keys
  `id · title · category · verification_mode · scenarios · notes`; scenario keys `kind · ref · contains`. Kinds
  across the file: 81 `nextest-file`, 11 `ui-test`, 3 `a11y-spec`, 2 `ci-script`, 2 `source-evidence`,
  1 `by-construction`.
- `docs/v0_2_0/pulse-capability-spec.md` (`:70-736`, statement, observable and boundary of each id) — the texts
  the dispositions below are read from.
- `andromeda-pulse-0.3.0/verification-matrix.json` (22 entries, requirement and `ref` of each) — P-061…P-082,
  all `verified`; the proof is a prose `ref`, not structured scenarios.
- `andromeda-pulse-0.4.0/requirements.md` (`:19-80`, `:84-103`) — the 47 capabilities of this version and the
  route's reading of the 22.
- `andromeda-pulse-0.4.0/working-route.md` (`:27-83`) — the entries of Epochs 1 to 3 that remove each surface.
- `pulse-app/ui/package.json` (scripts) and `pulse-app/ui/tests-a11y/axe/` — `test` is `vitest run`; `test:a11y`
  chains contrast, Playwright over the a11y config, Lighthouse, pa11y and the aggregator, and no vitest. 16 axe
  specs and one keyboard-focus spec stand.
- `pulse-app/src/incidents_router.rs` (`:171`, `:232`), `crates/triage/src/incident/registry.rs` (`:88`) —
  acknowledgment is reached only through the IPC route `incidents.acknowledge`; the sidecar has no such tool.
- `pulse-app/src/connection_router.rs` — the receiver state machine's reader is the IPC route
  `connection.current_state` (and the diagnostics snapshot).
- `.andromeda/residuals.md` (`:13`) — the glow layer's render half reads `dropped (the window it would render in is
  retired by P-083; the operator's word, founder-delegated, 2026-10-09)`.

## Graph impact (from the code-graph query)
- **verify_capability_matrix** — 1 caller: `main()` @ `xtask/src/main.rs:326` (rust plane, `rows: 1`). The
  change stays inside the `xtask` crate.
- **xtask** — `crate_edges`: one outbound row in the graph (`xtask → ingest`), no inbound row. A new module in
  `xtask` reaches no other crate.

## What was measured
- **The gate today:** `cargo xtask verify:capability-matrix` on the untouched tree at `279a477e` — exit 0,
  `verify:capability-matrix: clean (60/60 capabilities, 0 violation(s))`. Its report path is ignored by git
  (`git check-ignore -q target/capability-matrix/report.json`, exit 0).
- **Who runs the webview unit tests:** nobody. `git grep -n vitest -- xtask .github scripts` finds one line, a
  comment in `xtask/src/webview_drive.rs:82`; no workflow step, xtask verb or pre-push stage calls `npm test`.
  So a `ui-test` scenario of the old record, and every `*.test.ts(x)` proof of the 0.3.0 record, runs in no gate.
- **Who runs the headful and dev-host graders:** nobody in CI. `git grep` for `smoke:hue-shift`, `smoke:discovery`,
  `self-verify` and `webview-drive` under `.github` and `xtask/src/pre_push.rs`: 0 lines. `l4-latency-p99` has no
  caller outside its own two scripts.
- **Which a11y specs name the unguarded ids:** `git grep` for `P-066`, `P-068`, `P-081`, `Toggle dashboard`,
  `Errors only` under `pulse-app/ui/tests-a11y`: only three manual screen-reader scripts. No axe spec covers them.
- **CI on `279a477e`:** `ci#38046702090` read three times during this phase (Setup 5a and twice at P3), each
  **in progress** (2 checks running at the last read, the oldest the coverage gate at 925 s); `secret-scan#38046702082` success. No verdict.

## The 82 ids, read against the tree and the route
"Runs" below means a CI job runs the proof today: a `nextest-file` under `cargo xtask test` (job `lint-test`) or
an axe spec under `cargo xtask test:a11y` (job `a11y`). Route entries are named by title (`working-route.md`).

**Claimed, the reading not in doubt (24).** Each names the 0.4.0 requirement under which the engine keeps it.
- Detection, P-086 ("everything 0.3.0 detected is still detected"; the entry `Detection baseline through the
  console engine` names five cue families, one incident per storm, auto-resolve): P-005 (+P-096), P-006 (+P-095),
  P-007 (+P-096), P-008, P-009 (+P-091), P-010, P-011, P-012, P-013 (+P-091), P-014 (+P-098), P-015, P-016, P-017,
  P-018, P-057, P-074. The model's part in their texts (a severity hint the model may dismiss) leaves with P-084.
- Cue to incident, P-112 with P-086: P-021 (a cue forms the incident itself now), P-022 (auto-resolution; the
  resolution summary by a model leaves).
- P-041 Persistent incident corpus — P-112 (the engine's own record; a store written by 0.3.0 is not read).
- P-039 MCP delivery — P-092 and P-093 (the door: incident list, one incident whole, a telemetry slice, the one
  write). Its "Send to agent" button leaves with the window.
- P-047 PII scrubbing at ingestion — P-085 ("the secret scrubber at ingest stays as it is") and P-088.
- P-055 Configuration hot reload, P-056 Prospective threshold application — P-108 (the configuration becomes the
  engine's own). No route entry removes the `config-watcher` crate; `Cadence coordinator retired` removes only
  its cadence keys, `Window retired` its IPC route and its settings notice.
- P-077 Demo telemetry injector — claimed on the route's reading (`requirements.md:103`); no 0.4.0 requirement
  restates it, which the record says in its note.

**Retired with the window (26).** Removed by `Window retired` unless said otherwise. The tray, the updater and the
notification plugins leave with that entry too (`working-route.md:50`), so the tray is "the window" here.
- P-024 (runs: axe p5) · P-025 (no proof runs: one `ui-test`, grader `smoke:hue-shift` dev-host only; also
  `Display-only computation retired`) · P-026 (no proof runs: `ui-test` and source evidence) · P-028 (runs) ·
  P-029 (runs) · P-037 (runs: axe p9 and a nextest file) · P-038 (runs) · P-045 (runs) · P-051 (runs).
- P-023 Acknowledge cool-down (runs: `e2e_incidents_lifecycle.rs`). Its only trigger is the IPC route
  `incidents.acknowledge`; P-093 keeps one write, resolving. The cool-down code in `crates/triage` is named by no
  route entry and is P-121's to find.
- P-061, P-062, P-063 (run: nextest files under `pulse-app/tests`) · P-064, P-065, P-066, P-068 (no proof
  runs: vitest only) · P-081 (part runs: axe p4; the rest is vitest) · P-069 (runs: axe p11) · P-070 (part runs: `crates/ui-bridge/src/health.rs`; the rest is
  vitest) · P-071 (runs: axe p13) · P-080 (runs: axe p8) · P-082 (runs: axe p1).
- Gates that drive the window, removed by `Window's gates retired`: P-076 (no proof runs: `webview-drive` has no
  CI caller) · P-078 (part runs: the eight parser tests in `xtask/src/self_verify.rs`; the verb has no CI caller).
- P-072 Investigate actions — the window and the model (`Window retired`, `Local model retired`); runs.

**Retired with the model (13).** P-084's title is "the local model is gone from the engine, with everything that
served it", so the cadence and the digest are this surface.
- P-019, P-020, P-033, P-034, P-053, P-054, P-073 — `Local model retired`. All run as nextest files; P-020's real
  leg is env-gated and P-054's latency script has no caller.
- P-031, P-035 — the model's report (`Local model retired`, `Incident is the engine's own record`: "one report
  form exists, the one P-099 describes, and the old renderer is gone"); run.
- P-032 Project context — the intent's R11 lists "the project context" under the model; `Local model retired` and
  `Workspace detection retired`; runs.
- P-052, P-060 — `Cadence coordinator retired`; P-059 — `Digest retired`; run.

**None of the 82 is about the desktop's distribution.** Read over all 82 texts: no id names a bundle, an updater
channel, a package channel or an installer. The third surface P-117 names has no member in this record.

**In doubt, for P4 (19).**
- *A removal outside P-117's three surfaces (4):* P-043 Project-scoped memory (the workspace; P-105,
  `Workspace detection retired`) · P-046 Export for training (P-106, `Training export retired`) · P-049 Encryption
  at rest (P-085, `Corpus encryption at rest retired`; the security master states its control as current truth at
  six sites) · P-079 (the constellation half with the window, the workspace key with P-105). All run.
- *An engine half stays, the shown half leaves (11):* P-001, P-002, P-003, P-004 (receiver state, last-span
  tracker, failure surface, orthogonality — code in `crates/ingest/src/connection.rs`, shown as the connection
  dot; nearest 0.4.0 text P-098; no route entry names the state machine) · P-027, P-067 (service discovery and
  liveness; P-109 keeps "the service registry's liveness truth") · P-042 (incident state across restarts; read and
  acknowledged were the window's; P-112, P-091) · P-036, P-044 (similar past incidents; P-110 keeps the search,
  P-092 serves it; their reader was the model's report) · P-058 (pipeline self-observability, a window view with
  model metrics; P-122 and P-092 for the engine's own state) · P-075 (Conductor end to end: the read-back half
  continues as P-102, the four timing bounds end at a paint).
- *The fate turns on what the product is (4):* P-030 (no interrupting notification by default, "SHALL NOT be
  weakened in future versions", against P-099's system notification on a change of state) · P-040 (works fully
  without MCP, "SHALL NOT weaken", against P-092's door inside the engine from its first step) · P-048 (no raw
  OTLP attribute values stored, against P-094's kept attributes and P-091's seven days of raw telemetry) · P-050
  (fully local by default, against P-087's network receiver and P-093's door reachable from another host). P-030
  has no test (by-construction) and P-040 one `ui-test` that no gate runs.

**No proof runs today, among the ids retired in every reading (7):** P-025, P-026, P-064, P-065, P-066, P-068,
P-076. `[corrected at P4, after the dialog]` The dialog's fourth question named eight with none and two with a
part; the full `ref` of P-081 (cut at 520 characters in the first listing) names
`pulse-app/ui/tests-a11y/axe/p4-live-trace-list.spec.ts`, which the `a11y` job runs, so P-081 has a part. And read
by one rule for every retired id — a part when at least one automated proof the old record names runs in a CI job
and at least one does not — a part holds for fourteen: P-020 (its real-model leg is env-gated), P-024, P-028,
P-029 (a `ui-test` beside a proof that runs), P-054, P-060 (the latency script has no caller), P-063 (the
`self-verify` verb), P-069, P-070, P-071, P-080, P-081, P-082 (vitest beside an axe spec or a nextest file),
P-078 (the verb beside its parser tests). A one-time manual check is not counted as a guard on either side.

## Patterns detected
- **A gate's pure decision pinned per arm over constructed inputs** (`xtask/src/main.rs:1021`,
  `xtask/src/ci_gates.rs`, `xtask/src/perf_budget.rs`): the verb calls a function that takes its inputs and
  returns a verdict; the tests build real files under `tempfile::TempDir`.
- **Exit 0 pass · 1 findings · 2 cannot-evaluate, and no pass over an absent input** (`xtask/src/ci_gates.rs`,
  `xtask/src/staged_gate.rs`, `xtask/src/source_lint.rs`): the absent input is its own exit.
- **A verdict line that says what was read** (`xtask/src/main.rs:958`, `ci-gates: zero-spans PASS (N log records
  across M file(s))`): the count follows the input, never a constant.
- **A leaving thing names its route entry by title** (`architecture.md` §Stack, "Leaves with `Window retired`").

## Conventions to follow
- **A new xtask module is `snake_case.rs` beside `main.rs`, declared with `mod`** (`xtask/src/main.rs`, the
  `mod ci_gates;` family); no workspace member and no dependency is added.
- **Source text passes `cargo xtask check:english-sources`** — ASCII annotations in `.rs` files.
- **A workflow edit keeps the step a plain named `run:`**, adds no action and leaves `permissions:` alone.
- **`pulse-app` unit-shaped pins live in `pulse-app/tests/*.rs`**; the xtask pins stay co-located in `xtask/src`.

## New files to create
- `docs/capability-record.json` — the one current record: 82 entries, one disposition each
- `xtask/src/capability_record.rs` — the record's reader, its pure verdict and the per-arm pins

## Files to modify
- `xtask/src/main.rs` — the verb reads the record through the new module; its about string, verdict line and exits
- `.github/workflows/ci.yml` — the step's display name only
- `pulse-app/tests/a11y_perf_workflow.rs` — the pin's message only

## Open questions
- Which surface word does the record give an id retired by a removal that is none of the three P-117 names
  (P-043, P-046, P-049, the key half of P-079)? → blocks: plan-decision
- Are the eleven split ids claimed in a changed form or retired with the surface that showed them? → blocks:
  plan-decision
- What becomes of the four ids whose fate turns on what the product is (P-030, P-040, P-048, P-050)? → blocks:
  plan-decision (the founder's; an answer relayed or given in his stead is PROVISIONAL)
