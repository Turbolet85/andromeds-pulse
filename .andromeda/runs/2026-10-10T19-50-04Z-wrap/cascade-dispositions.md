# Cascade dispositions — 2026-10-10-engine-harness-proofs-completed

## The search

`cascade.py sweep` (cascade v1.2) over `cascade-patterns.toml`, run after the last master amendment of this pass and
before any leaf was touched. Baseline `514fbe57` (the parent of the pre-CI commit `d30b7037`). Every pattern's control
fired on the pre-pass masters.

Copied from the listing: `total (22 patterns) · 184 rows over 26 files` ·
`per class · new 20/4 · standing 87/9 · leaf 66/14 · curation 9/4 · base 2/1`.

The 22 patterns, one per retired claim, its wording and its mechanism: `no-port` (the check binds no port and starts
no program) · `seed` (stage six reads a seed it wrote) · `ready-accept` (ready is the status verdict plus an accepted
connection) · `two-labels` (`accepting` | `refusing`) · `receivers-line` (one failure line, printed only on `refusing`)
· `cycle-labels` (seven refusal labels) · `prepush-reasons` (nine reasons) · `census` (no third socket, not measured)
· `lock-unasserted` (the lock file's placement not asserted) · `key-path` (the key path not read) · `five-pins` ·
`lint-roots` (five roots, four search dirs) · `picked-ports` · `no-shell-test` · `one-binary` (one test binary spawns
the engine) · `isolation` (per-process isolation prevents port conflicts) · the three names `prepush-name`,
`ready-name`, `cycle-name` · `pending-rows` (the five row ids) · `tests-dir` (no separate `tests/` directory but
`pulse-app`'s) · `warm-time` (89 s / 90 s).

What was NOT looked for: a restatement that carries none of these wordings and none of the three verb names; the
window program's readiness wording in `agent-run.ps1` (source, not a master); design-system, layout-templates and
a11y-plan beyond the sweep's own read of them (their detectors returned no proposal and no row stands in them).

Sections read whole for the apply: architecture §Stack → Gate-script pins; §Occupied Resources → Network ports,
→ Filesystem locations (the pid file, the lock file, the harness-made dirs), → Environment variables (`PIDFILE`,
`LOGFILE`, `HOME`/`PATH`, `PUPPETEER_CACHE_DIR`, the cycle's SET bullet), → xtask CLI surfaces (`harness:ready`,
`harness:engine-cycle`, `check:english-sources`, the `agent-run` entry, `pre-push:linux`, `ci-gates`,
`quarantine-tracking`); security-plan §Threat Model Summary (the CLI-input trust boundary), §Input Validation (the
CLI / env row), §Security Anti-Patterns → Input (the three harness-class paragraphs and the exit-witness arm);
test-plan §1 (the surfaces row, the harness summary, six pending rows), §2 and §4 conventions, §9 (two rows), and the
key files `5-command-implementation` (boot's Command body, Readiness signal, Exit code), `per-chunk-gate-discipline`
(the `pre-push:linux` paragraph, the Spawned-program witness form, the Engine cycle gate form), `bootstrap-phases`
(item 1); obs-plan §9 (the `xtask test` row, the triage bullet) and §10 (the heartbeat-stall gate, Load-profile
constraints).

**A second sweep, after the route-resolve card.** The card's answers added five master sentences (owners in
architecture §Network ports and §Filesystem locations and in three test-plan §1 rows and one §9 row; the class of
the readiness verb's read in security-plan) and their leaf lines in `.claude/rules/security.md` and
`.claude/docs/security-summary.md`. The same pattern file was run again after them and before their three sidecar
entries. Copied from that listing: `total (22 patterns) · 183 rows over 26 files` ·
`per class · new 20/4 · standing 87/9 · leaf 65/14 · curation 9/4 · base 2/1`. The leaf rows that still match a
retired claim's wording were each read: `.claude/rules/verification-harness.md` :22, :86, :89, :100,
`.claude/rules/observability.md` :98 and `.claude/docs/tests-summary.md` :12, :94 are the re-derived sentences
themselves (they name the retired state as past, or list the labels now eight and the roots now six);
`.claude/rules/verification-harness.md:22` and `.claude/docs/tests-summary.md:90` on `no-shell-test` are the
exit-witness arm, still without a shell-level test; `.claude/docs/services/ingest.md:48` is this wrap's own text.
The dispositions below are of the first listing; no master row changed class between the two.

## Master and key-file rows (new 20 · standing 87)

Every `new` row is this pass's own text, read where it stands: no change. Every `standing edited` row is a line this
pass amended; the swept text that still stands on it was re-read for an intra-line duplicate of the retired claim:

- `no-port` — architecture.md:270 ×2 and per-chunk-gate-discipline.md:32 ×2: both are this pass's sentences ("It
  starts no `pulse-app`", "until that chunk the verb bound no port and started no program"). Amended; no duplicate.
- `seed` — architecture.md:270 ×4, obs-plan.md:559 ×2, per-chunk-gate-discipline.md:32 ×2: this pass's "in place of
  a seed log … it wrote itself" and the `ci-gates` caller clause. Amended.
- `ready-accept` — architecture.md:270, 5-command-implementation.md:5: the `accepting` definition, now one of two
  conditions. Amended. (architecture.md:227 and test-plan.md:67 no longer match: amended.)
- `two-labels` — 0 rows: both sites now read three labels.
- `receivers-line` — architecture.md:270 and 5-command-implementation.md:6: amended (the `foreign` line added).
  test-plan.md:135 ×2: the dated "Widened 2026-10-09" paragraph of the row, history under its own date; the row's new
  "Narrowed" paragraph states the present. No change. 5-command-implementation.md:4: the ps1 mirror carrying the
  `refusing` line — still true. No change.
- `cycle-labels` — architecture.md:270, security-plan.md:138, per-chunk-gate-discipline.md:54: amended to eight.
  test-plan.md:155 ×3: the dated NEW paragraph ("of the seven refusal labels") and this pass's Narrowed paragraph. No
  further change.
- `prepush-reasons` — architecture.md:270 ×3, per-chunk-gate-discipline.md:32 ×3: amended to eleven; the other
  match on each line is the `test` stage's `restore-failed:bindings`, true.
- `census` — architecture.md:178 amended; test-plan.md:62 new; test-plan.md:154 ×4 the dated NEW paragraph and this
  pass's LANDED paragraph.
- `lock-unasserted` — test-plan.md:154: the dated NEW paragraph, with LANDED after it. architecture.md:230 no longer
  matches: amended.
- `key-path` — 0 rows: amended.
- `five-pins` — test-plan.md:496: amended on the line ("five … six since"). architecture.md:34 and test-plan.md:152
  no longer match as a present claim: amended to six.
- `lint-roots` — architecture.md:270 ×2: amended to six roots and five search dirs.
- `picked-ports` — architecture.md:180 ×2, test-plan.md:504: this pass's own text.
- `no-shell-test` — security-plan.md:396: the exit-witness arm, scoped to the row's clause (d), still true.
  test-plan.md:135 ×3: dated paragraphs, the Narrowed paragraph after them. 5-command-implementation.md:4 and :6: the
  exit-witness arm and its refusal, still without a committed test — true. security-plan.md:138 no longer matches.
- `one-binary` — 0 rows: both obs-plan §9 sites amended to three binaries.
- `isolation` — test-plan.md:504 and bootstrap-phases.md:3 amended. test-plan.md:194 (`cargo-nextest` 0.9.x
  (per-process isolation)): names the runner's property, claims nothing about ports. No change.
- `prepush-name` — architecture.md:179, :227, :230, :233, test-plan.md:156, obs-plan.md:559,
  per-chunk-gate-discipline.md:54: new. architecture.md:265, :270 ×5, security-plan.md:85 ×2, :138, :396,
  per-chunk-gate-discipline.md:32: amended. architecture.md:260 (the `HOME` / `PATH` read), test-plan.md:233 (the
  `test` stage's constructed environment), per-chunk-gate-discipline.md:42 (the merge-base probe: the check builds
  the tip alone): each read, each still true. No change.
- `ready-name` — architecture.md:227 ×4, :256 ×2, :270 ×4, security-plan.md:85, :138 ×3, :396 ×4, test-plan.md:67,
  :135, 5-command-implementation.md:4 ×2, :5: amended lines. architecture.md:255 (`PIDFILE` threaded into the
  child), 5-command-implementation.md:7 (the timeout readings), pid-file.md:5 (the exit record's readers): true. No
  change.
- `cycle-name` — architecture.md:34, :230, test-plan.md:182, :496, obs-plan.md:509,
  per-chunk-gate-discipline.md:32, :52: new. architecture.md:179, :227, :233, :256, :266, :270 ×7,
  security-plan.md:85 ×2, :138 ×2, :396 ×3, test-plan.md:62 ×2, :135, obs-plan.md:559,
  5-command-implementation.md:4, per-chunk-gate-discipline.md:54 ×2: amended lines. architecture.md:238, :255, :260,
  :261 (what the cycle sets and does not carry), test-plan.md:502, :515 and ci-cd-approach.md:3 (the CI step,
  unchanged: the workflow is not edited): true. No change.
- `pending-rows` — every row is a line this pass amended, but 5-command-implementation.md:35 (the ps1 `cleanup`
  live leg, still owed): true. No change.
- `tests-dir` — test-plan.md:182 ×5 and :204 ×3 amended (the second separate directory). test-plan.md:149, :153,
  :205, :228 ×2, :235: other "co-located" pins and the webview convention, untouched by this chunk. No change.
- `warm-time` — per-chunk-gate-discipline.md:32 ×2: the dated readings of 89 s and 90 s stand as readings; this
  pass added the four new ones after them.

## Judgment bases (base 2)

- playbook.md:26 (`tests-dir`): "It is the exception to …" — another rule's wording, no claim about test
  directories. No change.
- playbook.md:60 (`tests-dir`): the 2026-08-14 rule's provenance quotes the then-false claim "no separate `tests/`
  directory" as the case it was minted on. A quotation of a retired claim as history; no change, no proposal.

## Curation homes (curation 9) — never edited by the cascade

- `.claude/rules/testing.md:316` (`lint-roots`; the 2026-10-10 SWEEP HAZARD entry names four search dirs): stale by
  one directory → P3, an in-place extension.
- `.claude/rules/verification-harness.md:127` (`pending-rows`; "now tracked as the
  `harness-log-family-resolution-coverage` trigger"): the row landed at this chunk → P3, an in-place extension.
- `.claude/rules/testing.md:121`, `:240` (`isolation`): two entries on nextest's per-process isolation and tracing
  subscribers; neither claims anything about ports. No change.
- `.claude/rules/frontend.md:121`, `.claude/rules/testing.md:222` ×3, `.claude/docs/session-learnings.md:2112`,
  `:2120`, `:2222` (`tests-dir`): "co-located" webview and dead-code entries. No change.

## Leaves (leaf 66 over 14 files) — re-derived from the amended masters

- `.claude/rules/verification-harness.md` :22 (boot: the word's pins, the readiness decision, the `foreign` line),
  :86 (the cycle: eight labels, the third caller), :89 (six roots), :100 (pre-push: stage six, eleven reasons, the
  cleanup child, the PROVISIONAL sentence, the four readings) → re-derived. :25 (`cleanup`, the ps1 leg still owed)
  → no change, true.
- `.claude/rules/observability.md` :88 (the boot poll's decision), :98 (the gap check outside CI) → re-derived.
- `.claude/rules/testing.md` :17 (isolation separates processes, not ports), :22 (claimed ports; the `xtask/tests`
  binaries), :25 (the second tests directory), :64 (the readiness decision) → re-derived. :26 (webview), :91 (the
  smoke trigger) → no change, true.
- `.claude/rules/security.md` :17 (the pre-push stage as PROVISIONAL, the word set's pins, the readiness read,
  `port-taken`, the lock file) → re-derived in its body paragraph; its `## Session Additions` untouched.
- `.claude/docs/commands.md` :90, :105, :110, :111 → re-derived.
- `.claude/docs/obs-summary.md` :68 → re-derived.
- `.claude/docs/tests-summary.md` :12 (boot row), :94 (pre-push) → re-derived. :90 (the witness arm still without a
  shell-level test), :139 → no change, true.
- `.claude/docs/security-summary.md` :43 → re-derived (three places on the line).
- `.claude/docs/stack.md` :12 (six pins; the xtask integration tests and what a host needs) → re-derived. :63 → no
  change, true.
- `.claude/docs/workflow.md` :72 → re-derived. :49 → no change, true.
- `CLAUDE.md` :36 (the `xtask` module line) and :44 (the path-variable warning: the PROVISIONAL stage) → re-derived
  inside their `GENERATED:setup:*` blocks. :14, :88 → no change, true.
- `.claude/docs/services/ingest.md` :48 — this wrap's own corrected text (below).
- `.claude/docs/services/config-watcher.md` :40, `interpretation.md` :51 (`tests-dir`): "co-located" test notes,
  read, true. No change.

A leaf statement no master states, found by the sweep and corrected from architecture's sentence: the boot poll's
decision in `.claude/rules/observability.md:88` and `.claude/docs/obs-summary.md:68` (obs-plan itself holds no
statement of the readiness decision).

## The leaf layout list the last handoff owed (inputs#I5 item 3)

The handoff written at `514fbe57` named 19 groups of file-layout lines in nine service notes that the tree
contradicts and no master holds. Each was corrected at this wrap from a read of the source tree as it stands
(`find crates/{crate} -type f`, each crate's `Cargo.toml`, each `lib.rs`, and the files named below); line numbers
are the handoff's. None is carried again.

| # | Leaf, line(s) | What it said | What the tree shows, and what the note says now |
|---|---|---|---|
| 1 | config-watcher.md 35 | public contract in `src/contract.rs` | no such file; `lib.rs` re-exports from three private modules → the line names `lib.rs` |
| 2 | config-watcher.md 24 | "`contract` module is the ONLY `pub` surface" | no `contract` module; `mod event; mod partition; mod watcher;` + `pub use` → rewritten |
| 3 | config-watcher.md 34 | classification in `src/` | `partition_changed_keys` is in `src/partition.rs` → named; `event.rs` added |
| 4 | triage.md 43 (the per-module line) | seven dirs, `interpretation/` among them, no `cadence/` | seven dirs `baseline, cadence, cue, digest, incident, lifecycle, pattern` and one file `interpretation.rs` → rewritten |
| 5 | ingest.md 42 | hand-off in `src/pipeline.rs` | no such file; `src/channel.rs` holds `Batch`, `IngestSender`, `IngestReceiver` → rewritten |
| 6 | ingest.md 43 | tests colocated in four files, `pipeline.rs` among them | a `#[cfg(test)]` module in eight of nine `src` files, and six files under `crates/ingest/tests/` → rewritten |
| 7 | ingest.md 47 | `MockTraceSpan::builder()` for fixtures | 0 hits under `crates`, `pulse-app/src`, `pulse-app/tests`, `xtask/src` → said so |
| 8 | ingest.md 13 | decoded `OtlpBatch` records | no such type; `ingest::channel::Batch` (`Spans` / `Metrics` / `Logs`) → rewritten |
| 9 | buffer.md 44 | a connection pool in `src/conn.rs` | no such file, no pool; the caller opens the connection (`pulse-app/src/engine_boot.rs`), `schema.rs::create_schema` makes the tables → rewritten |
| 10 | snapshot.md 44-49, 51 | seven paths (`pipeline.rs`, `curate/`, `format.rs`, `budget.rs`, `path.rs`, `io.rs`, `tests/integration/snapshot/`) | none exists; the crate is `contract.rs`, `markdown.rs`, `token_budget.rs`, `attribute_filter.rs` → the section rewritten |
| 11 | snapshot.md 20-25 | deps `duckdb`, `arrow-rs`, `tauri`, the notification plugin, `tiktoken-rs` | `Cargo.toml`: `curation`, `chrono`, `serde`, `thiserror`, `tracing` → rewritten |
| 12 | snapshot.md 10 | consumes `viz` aggregation utilities | no `viz` dependency; percentiles come from `curation` → rewritten, with line 9 (the DuckDB read is `pulse-app/src/snapshot_runtime.rs`'s) |
| 13 | ui-bridge.md 40 | `AppError` in `src/error.rs` | no such file; `src/contract.rs` → rewritten |
| 14 | ui-bridge.md 41 | the envelope in `{app_info,health,ready,settings}.rs` | only `health.rs` exists and carries all five procedures → rewritten |
| 15 | viz.md 38-39 | `aggregator.rs`, `stream.rs` | neither exists; `query.rs` holds the three queries, the Channel is `pulse-app/src/streams.rs`'s → rewritten |
| 16 | viz.md 19-21 | deps `arrow-rs`, `tauri`, `taurpc` | `Cargo.toml` names none of them → rewritten |
| 17 | mcp-server.md 47 and 4 | tests under `tests/integration/mcp/`; the crate "hosts" the `mcp.*` router | two files under `crates/mcp-server/tests/` and the `e2e_p3_mcp_*` files of `pulse-app/tests`; the router is `pulse-app/src/mcp_router.rs` → both rewritten |
| 18 | workspace-detector.md 48, 56 | a committed fixture dir `tests/fixtures/workspaces/` | none; the tests build their trees in `tempfile` dirs → both rewritten |
| 19 | workspace-detector.md 18, 10, 24 | `git2` or a `git rev-parse` shell-out | no `git2`, no process: a walk up at most 32 ancestors for `.git/`, then `.git/HEAD` → all three rewritten |

Found in the same reads and corrected with them, each from the file named:

- config-watcher.md, the dependency line: `toml` and the workspace crate `ui-bridge` were missing (`Cargo.toml`).
- triage.md, the dependency lines: `duckdb` and `tokenizers` were missing, and "Internal: `crates/security`" named a
  dependency `Cargo.toml` does not hold.
- ingest.md, four places: `opentelemetry-proto` is not a dependency; the OTLP protos are vendored under
  `crates/ingest/proto/` and compiled by `build.rs`.
- ui-bridge.md, the dependency list: the four Tauri-side crates are optional, `serde_json` is a dev-dependency,
  `anyhow` is not a dependency, seven workspace crates are path dependencies (`Cargo.toml`).
- viz.md, the Responsibility paragraph and the first Publishes bullet: the crate hosts no router and emits no
  stream; `pulse-app/src/viz_routers.rs` and `pulse-app/src/streams.rs` do.
- workspace-detector.md, the struct block: the type is `WorkspaceContext` with a nested `VcsMetadata`
  (`src/contract.rs`), and the project name comes from the root's basename, not from a `Cargo.toml`.
- buffer.md, the tests line: `crates/buffer/tests/drain_golden_corpus.rs` and its fixtures.

Seen and left, with the reason:

- ingest.md and others: the version numbers of test tools (`axum-test` 18.7, `reqwest` 0.12, `tonic` 0.14.5) —
  not read from the lockfile at this wrap.
- ui-bridge.md 38 ("No `pub use` re-exports across crate boundaries"): `snapshot::contract` re-exports
  `curation::contract` items; which crates the sentence was meant to bind was not established — for the operator.
- workspace-detector.md 52 (the P7 integration assertion's field names) and viz.md 15 (the
  `metric.trace.latency_percentiles` event): each restates a master (test-plan §6, obs-plan §5); a master defect, if
  one, is not a leaf's to settle.
