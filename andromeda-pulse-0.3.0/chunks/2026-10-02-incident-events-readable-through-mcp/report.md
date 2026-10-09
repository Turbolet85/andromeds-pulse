# Report — 2026-10-02-incident-events-readable-through-mcp

**Chunk:** an incident's lifecycle events read back through the MCP sidecar
**Date:** 2026-10-03T23:46:09Z (wrap run `2026-10-03T23-46-09Z-wrap`)
**Commits:** since `a69030a` (the parent of the oldest pre-CI commit; `git log --format='%h %s' a69030a..HEAD`):
`4a26ad8 chore(…): operator pre-CI commit` (S, Windows host, superseded) · `cdb6c1e chore(…): operator pre-CI commit` (S2,
Linux host — the round's binary).

## Changes (structured — detectors read this)
- **Files** (`git diff --stat a69030a -- . ':!.andromeda/runs'` + the three uncommitted evidence files):
  - source: `crates/corpus/src/contract.rs` · `crates/mcp-server/src/{tools.rs, jsonrpc.rs, bin/andromeda-pulse-mcp.rs}` ·
    `crates/triage/src/contract.rs` · `pulse-app/src/inference_runtime.rs`
  - tests: `crates/mcp-server/tests/incident_events_subprocess.rs` (new) · `crates/mcp-server/tests/sidecar_subprocess.rs` ·
    `pulse-app/tests/e2e_p3_mcp_incident_events_content.rs` (new)
  - chunk folder: `plan.md`/`research.md`/`scope.md` (phase) · `scope-record.md` · `evidence/{mutation-checks, round-request,
    round-binary, round-result, operator-pass, premise-correction}.md` · this report
  - bookkeeping: `.andromeda/{friction-log.ndjson, master-route.md}` · `andromeda-pulse-0.3.0/working-route.md` (phase's
    promotion) · `.claude/session-handoff.md` (the session-end hook's one-line timestamp, committed at S2) · run dirs
- **Symbols / APIs:**
  - NEW MCP tool `retrieve_incident_events` (the NINTH; `crates/mcp-server/src/tools.rs` `TOOL_RETRIEVE_INCIDENT_EVENTS`, appended
    to `ALL_TOOL_NAMES`, dispatched through the existing single `dispatch_tool` emission site). Input `{incident_id: integer}`
    (required, `additionalProperties: false`). Response `{incident_id, events: [{event_kind, occurred_unix_nano}], total,
    truncated}`, oldest first, bounded at `INCIDENT_EVENTS_READ_LIMIT = 256` (`truncated` when a 257th row exists). Unknown id →
    JSON-RPC -32603 `incident not found` (the existing `load_incident_row` existence check). `event_kind` is COERCED on
    egress: only a member of `triage::contract::incident_event_kinds()` passes, anything else reads `unknown`.
    `result_type_label` → `incident_events`; `result_count_for` → the `events` length. No new tracing target, no allowlist
    change.
  - NEW `CorpusWriter::load_incident_events(incident_id: i64, limit: u32) -> Result<Vec<IncidentEventRow>, Error>` + row type
    `IncidentEventRow { event_kind: String, occurred_unix_nano: i64 }` (`crates/corpus/src/contract.rs`); one impl
    (`impl CorpusWriter for Corpus` — `grep -rn 'CorpusWriter for' crates pulse-app xtask`: 1 hit). Prepared statement
    `SELECT event_kind, occurred_unix_nano FROM incident_events WHERE incident_id = ?1 ORDER BY id ASC LIMIT ?2`; the
    `payload` column is never selected. Its doc names both writers: the producer's creation event via
    `save_incident_event` and one row per status VALUE change via `update_incident_status`.
  - NEW `triage::contract::INCIDENT_EVENT_CREATED: &str = "created"` + `incident_event_kinds() -> [&'static str; 4]` =
    `["created", "active", "acknowledged", "resolved"]` — the closed `incident_events.event_kind` vocabulary, written by the
    producer and read by the sidecar (founder's option A, 2026-10-03).
  - CHANGED `pulse-app/src/inference_runtime.rs::create_incident_from_l4_output` — writes `INCIDENT_EVENT_CREATED` where
    it wrote the literal `"created"`; behaviour byte-identical (the single production incident-creation path:
    `grep -rn 'save_new_incident(\|create_incident_from_l4_output(' crates pulse-app/src` → one caller each).
  - RENAMED `mcp_server::jsonrpc::tools_list_with_8_tools` → `tools_list_manifest` (count-free); refs at
    `crates/mcp-server/src/bin/andromeda-pulse-mcp.rs` (import + call) and its four in-file pins, all updated
    (`grep -rn 'tools_list_with_8_tools' crates pulse-app xtask scripts` → 0 hits after).
  - `tools/list` manifest: ninth entry; its description names `created` as the first event, written when the
    interpretation path opens the incident, then one per status change.
- **Crates / modules:** none added or removed; `corpus`, `mcp-server`, `triage`, `pulse-app` changed.
- **Dependencies:** none added or bumped (`Cargo.lock` byte-identical to `a69030a`: `git diff --quiet a69030a cdb6c1e --
  Cargo.lock` exit 0).
- **Schema / config:** none — no DDL, `SCHEMA_VERSION` 2 unchanged, no config key, no env var, no port. Redaction shape
  of the new egress: a coerced closed label + a timestamp; no payload, no ciphertext, no client text.
- **Spec-master edits:** none (no `.andromeda/` master changed; `git diff --name-only a69030a -- .andromeda/*.md` lists
  only `master-route.md`, the phase promotion).
- **Counts / qualifiers moved:**
  - MCP tool count 8 → 9 — stated in security-plan (`grep -cE 'Tools: 8|all 8 tools|8 tools|eight tools'`: 2 hits, `:68`
    §Threat Model MCP vector "Tools: 8" and `:140` §Input Validation "all 8 tools dispatched by name"), in CLAUDE.md
    §Modules `mcp-server` ("8 tools"), and as rosters in architecture (`query_incident_list` 4 hits) / security-plan (2) /
    test-plan (2); obs-plan names only the first four tools (`:236` MCP tools list, `:308` `method` value set) — 0
    `query_incident_list` hits there.
  - `incident_events` production WRITERS: documented as one (`update_incident_status`), measured TWO (see Spec claims
    disproved). Production READERS: 0 → 1 (the sidecar).
  - Workspace test count: Windows 2560 → 2571 (+11: 5 corpus + 6 dispatcher pins; the vocabulary pin and the `created`
    dispatcher pin were added after the S gate run and counted in the 2573 below); Linux default-features 2575 (2573 +
    the two Unix-only exit-cause arms). Feature-gated `package(mcp-server)` 86 → 87. Basis: gate logs
    `implement-2026-10-02T15-37-29Z` (2571 · 86), `implement-2026-10-03T09-17-05Z` (2573 · 87),
    `implement-2026-10-03T21-54-37Z` (2575 · 87).
- **Dev-tool versions:** HOST MOVE — the dev host changed from Windows (D: Dev Drive) to Omarchy Linux
  (`~/dev/projects/andromeda-pulse`, fresh clone, 2026-10-03, founder word via the overseer). On the Linux dev host:
  `cargo-nextest` (the test runner) INSTALLED prebuilt 0.9.146 (`cargo-nextest 0.9.146 (8af696ddc 2026-09-21)`, host
  `x86_64-unknown-linux-gnu`) — absent before; rustc/cargo 1.95.0 auto-installed by rustup on first use; node 26.8.2 /
  npm 11.19.1 read (mise). No lockfile-resolved crate is this line's subject.
- **Harness / gate surface:** no code change. `cargo xtask pre-push:linux` (WSL-only, `wsl.exe`) could not run on the
  Linux host; its six stages ran natively per a founder ruling (deviation below); porting the verb is its own route entry
  after P1 (founder 2026-10-03).
- **Cross-project / external claims:**
  - Conductor round on S2: commit `e6e1eefa6d0946b79e66695792abe1e3db18c09c` (Conductor `build/conductor-0.3.0`), CI
    `CI#37162108538` `completed success` on that sha (read with `gh run view`), `round-ledger.md` 7/7 PASS, sha256s
    re-measured equal to S2's — `evidence/round-result.md` (cited, never copied).
  - Pulse CI on S `4a26ad8`: `ci#37069724167` — 11/12 success, red `supply-chain`.
  - Pulse CI on S2 `cdb6c1e`: `ci#37157540938` (22:10:44Z → 22:29:05Z) — 11/12 success, red `supply-chain` only:
    RUSTSEC-2026-0325/0326/0327 on `wasmtime 48.0.3`, published 2026-10-02 (advisory DB `ef6173cbc5c5`, 1290 advisories).
  - Conductor launch finding (Pulse's): S2's default launch posture died ~1.5 s in on Gdk `Error 71 (Protocol error)
    dispatching to Wayland display` (NVIDIA GA102 · Hyprland · WebKitGTK 2.52.6 · native Wayland); the round ran with
    `WEBKIT_DISABLE_DMABUF_RENDERER=1` (`round-ledger.md` at `e6e1eef`, overseer's relay 2026-10-04).
- **Reverted / negative API facts:** S shipped `coerce_event_kind` passing only the three `IncidentStatus` labels, and a
  `tools/list` description stating "Creation records no event"; both were REPLACED at S2 — the producer's real
  `created` event read `unknown` through the S reader (relayed failure of assertion 7, 2026-10-03; reproduced RED on the
  Linux-bound tree, `evidence/mutation-checks.md`).
- **Insufficient fixes:** none.
- **Spec claims disproved by measurement:**
  1. security-plan `:440` (§Security Anti-Patterns → Logging, the `incident_events` NO-SCRUB paragraph) states `event_kind`
     is "a bounded status label from the closed `IncidentStatus` set" and "All seven production writers reach it through the
     one `update_incident_status` choke point". FALSE: `pulse-app/src/inference_runtime.rs::create_incident_from_l4_output`
     writes `event_kind = "created"` through `IncidentPersistence::save_incident_event` → `CorpusWriter::save_incident_event`
     (since chunk #92 `001a768`, 2026-05-31; pinned by `pulse-app/tests/unit_incident_producer.rs` `vec!["created"]`). The
     no-scrub conclusion still holds (both writers carry a bounded literal and an empty payload); the claim's mechanism and
     vocabulary do not.
  2. architecture `:210` (§Occupied Resources → Corpus SQLite database) states `incident_events` "gained its first
     production writer the same chunk" (2026-08-30, `update_incident_status`). FALSE as a writer census: the producer's
     `created` write predates it by three months and is a second writer.
  3. CLAUDE.md §Modules `corpus` restates claim 2 ("`incident_events` has a production writer since the same chunk: one
     lifecycle row per incident status VALUE-change") — a leaf of the same fact.
  4. obs-plan `:236` / `:308` list four MCP tools / four `method` values — stale since chunk #94 added four incident tools
     (pre-existing), now 9 with this chunk.
  5. CHUNK-ARTIFACT claims (plan Goal `:15-16`, Step 4 `:64-65`, Step 6 `:76`, Forks `:363`; scope §Boundaries "an incident's
     CREATION records NO event") — the phase premise "creation records no event" was true of `Corpus::save_incident` only;
     research mapped the corpus layer, never the `IncidentPersistence` writer one layer up. Recorded in
     `evidence/premise-correction.md`; no amendment owed (chunk artifacts have no writer at wrap).
  6. obs-plan's APP stderr-sink claims (the route CARRY): `pulse-app/src/observability.rs::init` builds the file layer only
     (research.md §Patterns detected, re-read at `a69030a`) — sites obs-plan `:67 :72 :76 :175 :198 :281 :437 :471`
     (`grep -n -i stderr .andromeda/obs-plan.md`); `:438` and `:740` (the SIDECAR's stderr) are true and stay.
- **Expected amendments (from plan):**
  - architecture (MCP server row · MCP Server Surface · External wire — MCP server · Standard Contracts) roster 8 → 9 —
    carried: Symbols/APIs (the ninth tool) + Counts (architecture `query_incident_list` 4 hits locate the roster sites).
  - architecture §Occupied Resources → `incident_events` first production READER (the sidecar) — carried: Symbols/APIs
    (`load_incident_events`) + Spec claims disproved #2 (the writer census is ALSO wrong: two writers).
  - security-plan §Threat Model "Tools: 8" + §Input Validation "all 8 tools" → 9 — carried: Counts (2 hits, `:68`, `:140`).
  - security-plan §Security Anti-Patterns → Logging `incident_events` NO-SCRUB paragraph (egress carries only the coerced
    closed label + timestamp) — carried: Symbols/APIs (coercion) + Spec claims disproved #1 (the paragraph's vocabulary and
    choke-point claims are false; the amendment must state the four-kind vocabulary and both writers).
  - obs-plan §4 MCP stdio row + Scenario P3 `method` set gains `retrieve_incident_events` — carried: Counts (obs-plan
    `:236`, `:308`, 0 `query_incident_list` hits — the row itself names no roster) + Spec claims disproved #4.
  - obs-plan app-sink correction (`:67 :72 :76 :175 :198 :281 :437 :471`) + leaves `.claude/rules/observability.md` `:20`
    `:38` (+ retire the `:151` `[correction]` entry) and `.claude/docs/obs-summary.md` `:14` `:56` — carried: Spec claims
    disproved #6 (no source changed; `grep -n -i stderr` hits counted above).
  - test-plan §1 `mcp-incident-read-back-cross-process-coverage` (`:125`) + §6 P3 Current residual (`:507`) — carried:
    Coverage (two cross-process legs committed, both ran on the Linux host; the existing `e2e_p3_mcp_resolve_content.rs`
    unchanged).
  - `matrix#P-075 notes — …` — `ledger-note — owner P7.3`: re-verified on S2 `cdb6c1e` by Conductor `e6e1eef`,
    `CI#37162108538`, 7/7 PASS, evidence `andromeda-pulse-0.3.0/chunks/2026-10-02-incident-events-readable-through-mcp/evidence/round-result.md`;
    acceptance and ref untouched. The note must name the round's `WEBKIT_DISABLE_DMABUF_RENDERER=1` posture.
  - The plan's wording "creation records none" in any amendment text is SUPERSEDED (premise-correction.md): every
    amendment states the four-kind vocabulary with `created` first.
- **Coverage of new surfaces:**
  - `retrieve_incident_events` (MCP tool) → validation serde `IncidentIdArgs` + `additionalProperties:false`✓ ·
    instrumentation `mcp.tools.call.response` / `metric.mcp.tool_call_duration_ms` / `mcp.tools.call.error` via
    `dispatch_tool`✓ · PII coerced label + timestamp only, response body never logged — stderr AND file-sink canary
    absence measured✓ · tests unit (7 dispatcher pins) + integ (mcp-server subprocess ×2) + e2e (real producer → real
    sidecar) · a11y n/a · tokens n/a
  - `CorpusWriter::load_incident_events` → validation prepared `?1`/`?2`✓ · instrumentation n/a (no log; errors →
    `QueryFailed`) · PII n/a (payload never selected) · tests unit (5 corpus pins) · a11y n/a · tokens n/a
  - `triage::contract::incident_event_kinds` / `INCIDENT_EVENT_CREATED` → validation n/a · instrumentation n/a · PII n/a ·
    tests unit (1 vocabulary pin) + the e2e leg · a11y n/a · tokens n/a

## Deviations from intent
- **Premise correction + scope widening (founder's option A, 2026-10-03, relayed by the overseer).** Assertion 7 failed on
  S: the producer's `created` event read `unknown`. The fix put one vocabulary in `triage::contract`, read by producer and
  sidecar, proven by the real producer through the real sidecar. Scope record — `gate.py scope` (P1, this wrap): `clean —
  changed 9 · listed 6 · recorded 3 (widening 3)`:
  - widening `crates/triage/src/contract.rs` · serves `crates/mcp-server/src/tools.rs` · word: "option A, the shared event
    vocabulary in triage::contract, … This is his approval of the scope widening (…)" — founder live word 2026-10-03,
    relayed by the overseer
  - widening `pulse-app/src/inference_runtime.rs` · serves `crates/mcp-server/src/tools.rs` · same word
  - widening `pulse-app/tests/e2e_p3_mcp_incident_events_content.rs` · serves
    `crates/mcp-server/tests/incident_events_subprocess.rs` · same word
- **Plan gate 6 red by construction** (`git diff --name-only a69030a -- … pulse-app …` prints
  `pulse-app/src/inference_runtime.rs`): superseded by the same founder word; recorded in `scope-record.md`.
- **Host move mid-chunk** (founder 2026-10-03): D: (Windows) settled by a move to Omarchy Linux; the fix-A tree was
  restored at `4a26ad8`, the full block re-run here. Two fresh-host preconditions were supplied on the overseer's go, no code
  change: `cargo-nextest` installed; `pulse-app/ui/dist` built (`npm ci` + `npm run build`, ci.yml `:72-78`; gitignored).
- **Plan entry 18 — DEVIATION (founder ruling 2026-10-03):** `pre-push:linux` needs `wsl.exe`; its six stages ran
  natively (env-isolated like the WSL form), xtask untouched; first run red at `npm` on a partial puppeteer browser cache
  (deleted by the founder: exactly the two `linux-148.0.7778.97` folders); re-run green. Named difference: `npm ci` in the
  session env reported install scripts as held back yet started puppeteer's download and exited 0 leaving it partial;
  under `env -i` the same install failed on that partial cache (`evidence/operator-pass.md`).
- **Plan entries 17 / 21 / 22 path remaps** (operator's word): tools at `~/.claude/skills/andromeda-tools/scripts/`,
  Conductor at `~/dev/projects/conductor`.
- **S superseded, never relayed:** the round ran ONCE, against S2 (founder 2026-10-03); `round-request.md` rewritten for S2.
- **RED-at-base form:** source edits stashed with tests kept, run against the base sidecar (the plan named the reading,
  not the form).
- **Mutation 2 (ORDER BY dropped) does not discriminate** — `incident_events` has no secondary index, so the rowid scan
  equals `ORDER BY id ASC`; an `ORDER BY id DESC` control reddened the 2 order pins; the clause stays as a contract.
- **Smoke:** the release sidecar driven directly (initialize · tools/list · tools/call) instead of `agent-run.sh`, which
  launches pulse-app on 4317/4318 (operator: ask before launching; this chunk touches no pulse-app boot path).

## Decisions & corrections
- Founder rulings (relayed by the pc overseer / overseer1, recorded by date — never as "Viola"):
  - 2026-10-02: the ninth-tool surface; creation-event = none as the ledger then read (FALSIFIED — see above); P-075
    recorded as this plan's criterion + a wrap notes line.
  - 2026-10-03: option A (shared vocabulary in `triage::contract`, producer + sidecar, the real-producer leg) — the
    widening approval; one round only, against S2; Conductor held its drive meanwhile.
  - 2026-10-03: entry 18's stages run natively, recorded as a deviation; porting `pre-push:linux` to native Linux is its
    own route entry after P1.
  - 2026-10-03 / 2026-10-04: wasmtime 48.0.3 → 48.0.4 is its OWN chunk, "Supply-chain advisories on wasmtime resolved"
    (WHAT-only, recorded pre-direction), placed right after this chunk and before the retry-storm entry, no advisory ignore;
    the supply-chain red on S2 is recorded with the founder's exact wording (Outcome).
- Overseer: the key-creation race and the Wayland launch finding are OWNER QUESTIONS for the founder at route-resolve —
  this wrap does not place them.
  - Key-creation race: `OsKeychainBackend::fetch_from_os_store` (`crates/corpus/src/keychain.rs`) gets, then on `NoEntry`
    generates + sets, with no compare-and-set; concurrent first-run processes each mint a key, last write wins. Measured on
    the fresh Linux credential store: entry 3 red with `incident not found` on seeded ids, one key `created = modified =
    21:54:52`, then green on the identical tree with the key present (`mutation-checks.md`). Pre-existing since chunk
    2026-08-15-corpus-key-persistence.
  - Wayland launch: S2 default posture died on Gdk Error 71 on NVIDIA + Hyprland + WebKitGTK 2.52.6; one host, one launch;
    no confining mechanism identified (`round-result.md`).
- Sweep hazards found this chunk:
  - A producer grep at the WRONG LAYER: research grepped corpus `INSERT … incident_events` / `save_incident` and missed the
    `IncidentPersistence::save_incident_event` writer one layer up; the census needs the trait-level writer grep
    (`grep -rn 'save_incident_event' crates pulse-app/src`).
  - `git check-ignore -v pulse-app/ui/dist` printed NOTHING for a not-yet-existing directory while `dist/` IS ignored; the
    file-path form `git check-ignore -v --no-index pulse-app/ui/dist/index.html` matched `.gitignore:42`.
  - `npm warn install-scripts … held back` did not mean the script did not run (the puppeteer download started and was left
    partial, `npm ci` exit 0).
  - A hygiene-gate host-path predicate fires on a QUOTED plan path (`C:/Users/…`, `/d/dev/…`) in an evidence file.

## Outcome
**Acceptance criteria** (plan §Acceptance Criteria, re-asserted against the diff):
- (corpus) `load_incident_events` ordered/exact/empty/excluded/limit/no-payload — MET (5 corpus pins, Linux gate 3).
- (arch) 9 tools listed; tools/call body + -32603 `incident not found`, no path — MET (Linux gates 3 + 5; release-sidecar
  smoke at /implement P3).
- (security) only `event_kind` from the closed set (unknown → `unknown`), `occurred_unix_nano`, `total`, `truncated`,
  `incident_id` — MET under the corrected vocabulary: the closed set is now FOUR kinds (`created` + three status labels),
  founder option A; the plan's "status-label set" wording is superseded (premise-correction.md).
- (obs/security) stderr + `agent-latest.jsonl*` carry tool name + `result_type`, 0 seeded timestamps / title canary — MET
  (mcp-server subprocess leg, Linux gate 3, no `[skip]`).
- (security) double gate — MET by construction (`sidecar_env_unset_exits_cleanly_without_stdio` green in gate 5).
- (tests) new pins collected by name, leg ran — MET (gate 4 collected 16; gate 3 zero `[skip]` lines).
- (arch/tests) no dependency/schema/capability/webview/TauRPC/**pulse-app source** change — **UNMET AS WRITTEN**:
  `pulse-app/src/inference_runtime.rs` changed (gate 6 red). Superseded by the founder's widening word (scope-record);
  unlinked to the matrix → for P2's attention as a recorded supersession, not a reopened question. Bindings byte-identical
  to the base (gate 15 green); dependency/schema/capability/webview/TauRPC halves hold.
- (tests) standard gate set green in §3 order — MET except gate 6 (above).
- (P-075) Conductor commits ONE deterministic round against S2 with 7/7 PASS — MET: `e6e1eef`, `CI#37162108538`, 7/7
  PASS, recorded in `round-result.md`; run under the `WEBKIT_DISABLE_DMABUF_RENDERER=1` launch-posture deviation.

**Gates** — final full block, Linux host, run `implement-2026-10-03T21-54-37Z` (warm target, after `ui/dist` and nextest):
- `cargo fmt --check` — green
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` — green
- `cargo nextest run --workspace --features mcp-server --profile ci --success-output immediate -E 'test(/incident_events/)'`
  — green (16/16, `lacks [skip] no OS credential store` held). First block in the same run dir red on this entry
  (`-32603 incident not found` — the key-creation race on an empty credential store, two-sided: green with the key present).
- `cargo nextest list --workspace --features mcp-server --profile ci -E 'test(/incident_events/)'` — green (16 collected)
- `cargo nextest run --workspace --features mcp-server --profile ci -E 'package(mcp-server)'` — green (87/87)
- `git diff --name-only a69030a… -- Cargo.lock deny.toml pulse-app crates/corpus/src/schema.rs crates/ui-bridge` —
  `red · no output` — prints `pulse-app/src/inference_runtime.rs`; superseded by the founder's widening (above)
- `cargo xtask check:english-sources` — green (`"verdict": "clean"`)
- `cargo xtask capability-widening-check` — green
- `cargo xtask check:ingest-progress` — green
- `cargo xtask check:staged-artifacts` — green
- `cargo xtask capability-drift` — green
- `cargo xtask verify:capability-matrix` — green
- `cargo nextest run --workspace --profile ci` — green (2575/2575)
- `cargo nextest run -p pulse-app --features mcp-server --bin pulse-app -E 'test(emit_taurpc_bindings)'` — green
- `git diff --quiet a69030a… -- pulse-app/ui/src/bindings/index.ts` — green
- `cargo build --workspace --release --features mcp-server` — green (re-run after the native `npm` stage; S2 artifacts
  `pulse-app` `23f6ef2b…6ada`, `andromeda-pulse-mcp` `e64f3688…e02e`)
- `python -X utf8 …/gate.py hygiene` (leg operator) — green, `hygiene: clean` (path remap)
- `cargo xtask pre-push:linux` (leg operator) — DEVIATION: six stages native, `verdict green` on the re-run (script-modes
  100755 · source-lint clean · npm 0/0 · clippy 0 · test 2575/2575 · ci-gates PASS, perf-budget NEUTRAL)
- `git rev-parse HEAD` (leg operator) — S2 `cdb6c1ed572761ae384597a7ed437222e3a1d1fc`
- `git diff --quiet && git diff --cached --quiet && git push origin chore/migrate-pulse-to-v3` (leg operator) — exit 0,
  `4a26ad8..cdb6c1e`
- `python -X utf8 …/ci.py conclusion --sha HEAD --wait 2400` (leg operator, path remap) — `red · contains verdict:
  green`: `ci#37157540938` 11/12, red `supply-chain`. **red — not this chunk's: Cargo.lock identical to a69030a,
  RUSTSEC-2026-0325/0326/0327 on wasmtime 48.0.3 published after it → "Supply-chain advisories on wasmtime resolved"; it
  does not block this chunk** (founder ruling 2026-10-04).
- `grep -rlF '<id>' …/conductor-0.3.0/chunks --include=*.md` (leg operator, `<id>` = S2, path remap) — green, 3 hits.
- Smoke: release sidecar direct drive at /implement P3 (Windows); the boot path is unchanged (no `main.rs` /
  `observability.rs` / ui-bridge / capability touch).

**Watches:** none folded.

**Outcome basis:** the operator pass ran — the verdicts rest on the Linux final state (S2 `cdb6c1e`, `ci#37157540938`,
`evidence/operator-pass.md` §Operator pass to S2) and the Conductor round (`evidence/round-result.md`); the Windows runs
(`implement-2026-10-02T15-37-29Z`, `implement-2026-10-03T09-17-05Z`) are superseded history in `mutation-checks.md`.
Overseer directives between implement and this report: the premise correction (option A), the host move, the native
entry 18, the supply-chain wording, the P1-only stop.

**Process hygiene:** cargo / nextest / sidecar subprocesses / the release sidecar smoke / native-stage children /
background monitors — started by this session's runs, all terminated (re-measured: `pgrep -c 'andromeda-pulse-|pulse-app'`
→ 0); no pulse-app launched by this session; Conductor's round processes are Conductor's (its ledger §Process census
records them terminated).
