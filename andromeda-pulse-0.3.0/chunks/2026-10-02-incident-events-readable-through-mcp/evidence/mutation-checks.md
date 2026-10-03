# One-shot controls — 2026-10-02-incident-events-readable-through-mcp

The plan's ONE-SHOT controls at /implement (plan §Test Commands preamble): the RED-at-base read of the new pins, and a
mutation check of Step 4's coercion and Step 1's ORDER BY. Each reading is quoted from the run that produced it; the
raw logs stayed in the session scratchpad (they carry host paths).

## RED at base (2026-10-02, /implement P2, before any green run)

**Form.** The chunk's SOURCE edits (`crates/corpus/src/contract.rs`, `crates/mcp-server/src/**`) were set aside with
`git stash push -- <those paths>`; the new and changed TEST files stayed in place. So the sidecar binary was the
base's, and the tests were this chunk's. Then:

`cargo nextest run -p mcp-server --features mcp-server --profile ci --success-output immediate --no-fail-fast -E 'test(/incident_events/) | test(/sidecar_tools_list/)'`

then `git stash pop` (the restored diff vs `a69030a` re-read identical: 5 files, +379 / -17).

**Reading.** Exit 100. `Summary [0.266s] 3 tests run: 0 passed, 3 failed, 77 skipped`.

| test | first diagnostic line |
|---|---|
| `incident_events_subprocess::incident_events_read_back_cross_process_with_body_and_clean_logs` | `no error expected: {"error":{"code":-32601,"message":"tool dispatch failed for `retrieve_incident_events`: unknown tool"},"id":950,"jsonrpc":"2.0"}` |
| `incident_events_subprocess::incident_events_unknown_id_returns_a_sanitized_not_found_error` | `left: Number(-32601)` / `right: -32603` |
| `sidecar_subprocess::sidecar_tools_list_returns_every_tool_by_name` | `left: 8` / `right: 9` |

**What it proves.** The base sidecar knows no `retrieve_incident_events` (-32601, `unknown tool` — the predicted
base reading) and lists 8 tools. The cross-process leg REACHED the sidecar rather than clean-skipping: the seeding
corpus opened through `OsKeychainBackend`, so this host's credential store is present and the leg's arms are live
here, not vacuous.

## Green (the same session, gate run `implement-2026-10-02T15-37-29Z`)
All 16 non-operator entries green on the first run, 0 red. The targeted entry collected and passed 13 pins by name (5
corpus `load_incident_events_*`, 6 dispatcher `retrieve_incident_events_*`, 2 subprocess `incident_events_*`), with no
`[skip]` line in its log — the subprocess leg ran. Workspace default suite 2571/2571 (2560 at the base + the 11 non-gated
pins); `package(mcp-server)` under the feature 86/86.

## Mutation 1 — Step 4's coercion made a pass-through
**Mutation.** `tools.rs:523` `"event_kind": coerce_event_kind(&event.event_kind),` →
`"event_kind": event.event_kind.as_str(),` (anchored Edit; the mutated line confirmed by `grep -n` before the run).
`cargo nextest run --workspace --features mcp-server --profile ci --no-fail-fast -E 'test(/retrieve_incident_events/)'`

**Reading.** Exit 100. `6 tests run: 5 passed, 1 failed`. The failing pin is
`tools::tests::retrieve_incident_events_coerces_an_out_of_set_kind_to_unknown`:
`left: [("client text <b>", 1700000000333), ("resolved", 1700000000444)]` /
`right: [("unknown", 1700000000333), ("resolved", 1700000000444)]`.

**What it proves.** The coercion pin discriminates — a pass-through reaches the egress with the raw stored text. The
five siblings stay green under the mutation, as they must: none stores an out-of-set kind, so the coercion pin alone
carries this guard. Restored by the inverse anchored Edit.

## Mutation 2 — Step 1's ORDER BY dropped
**Mutation.** `contract.rs:863` `WHERE incident_id = ?1 ORDER BY id ASC LIMIT ?2` → `WHERE incident_id = ?1 LIMIT ?2`.
`cargo nextest run -p corpus --profile ci --no-fail-fast -E 'test(/load_incident_events/)'`

**Reading.** Exit 0. `5 tests run: 5 passed`. **The pins do NOT discriminate a dropped ORDER BY.**

**Why, measured.** `incident_events` carries no secondary index (`crates/corpus/src/schema.rs:69-77`: the rowid primary
key only), so SQLite answers the unordered query by a rowid-order table scan — the same order the clause requests. The
dropped clause is therefore unobservable on this SQLite version and schema; no assertion over the result can see it.

**Control — the order assertions are live.** The same pins against `ORDER BY id DESC`: exit 100, `5 tests run: 3
passed, 2 failed` — `load_incident_events_reads_transitions_oldest_first_with_written_timestamps`
(`left: [resolved@3333, acknowledged@2222]` / `right: [acknowledged@2222, resolved@3333]`) and
`load_incident_events_honours_the_limit` (`left: [resolved@4000, active@3000]` / `right: [acknowledged@2000,
active@3000]`). So the pins pin the order itself; what they cannot see is a clause whose absence changes nothing today.
The clause stays: it makes the order a contract rather than a property of the current query plan (an index added later
on `incident_id` could change the scan order). Restored to `ORDER BY id ASC`; the source diff vs `a69030a` re-read
5 files, +379 / -17, identical to the gated state.

## RED at S's reader — the `created` event (2026-10-03, after the premise correction)
**Form.** The new and updated tests written first — the dispatcher pin
`retrieve_incident_events_reads_the_producers_created_event_as_created`, the mcp-server subprocess leg reseeded with the
creation event as the producer writes it, the real-producer leg `pulse-app/tests/e2e_p3_mcp_incident_events_content.rs`,
and the vocabulary pin — with the reader (`coerce_event_kind`) and the producer (`inference_runtime.rs`) still at S:
`cargo nextest run --workspace --features mcp-server --profile ci --success-output immediate --no-fail-fast -E 'test(/incident_events/)'`

**Reading.** Exit 100. `16 tests run: 13 passed, 3 failed` — every failure the predicted one:

| test | first diagnostic line |
|---|---|
| `tools::tests::retrieve_incident_events_reads_the_producers_created_event_as_created` | `left: [("unknown", 1700000000050), ("resolved", 1700000000555)]` / `right: [("created", …), ("resolved", …)]` |
| `e2e_p3_mcp_incident_events_content::real_producer_incident_events_read_back_through_the_sidecar` | `left: [("unknown", <opened_at>)]` / `right: [("created", <opened_at>)]` |
| `incident_events_subprocess::incident_events_read_back_cross_process_with_body_and_clean_logs` | the first event `"event_kind": String("unknown")` where `"created"` was expected |

**What it proves.** The real producer, through the real adapter, onto a real on-disk corpus, read by the real sidecar:
its own creation event came back `unknown` — the overseer's relayed failure, reproduced on this host before the fix. The
vocabulary pin (`incident_events_vocabulary_is_created_plus_each_status_label`) was green in the same run (the set is
new and additive); the 12 other pins stayed green.

## Green after the fix (gate run `implement-2026-10-03T09-17-05Z`)
`entries 22 · green 15 · red 1 (6) · recorded 0 · timeout 0 · not-run 6`. The targeted entry collected and passed 16
pins by name (`16 tests run: 16 passed`, no `[skip]` line — both cross-process legs ran); the collection probe listed 16;
`package(mcp-server)` under the feature 87/87; the default workspace suite 2573/2573 (2571 + the vocabulary pin and the
`created` dispatcher pin). Bindings byte-identical to `a69030a`; clippy, the ASCII source gate, the capability probes
and the release build green.

**Entry 6 red — superseded by the founder's word.** The scope guard
`git diff --name-only a69030a -- Cargo.lock deny.toml pulse-app crates/corpus/src/schema.rs crates/ui-bridge` printed
exactly one path, `pulse-app/src/inference_runtime.rs` — the producer now writing `INCIDENT_EVENT_CREATED`, one of the
three paths the founder approved as a scope widening on 2026-10-03 (relayed by the overseer; recorded in
`scope-record.md`). The new `pulse-app/tests/e2e_p3_mcp_incident_events_content.rs` is untracked, so `git diff` does not
list it. No dependency, lockfile, schema, capability or ui-bridge path changed.

## Re-verification on the Linux host (2026-10-03, run `implement-2026-10-03T21-54-37Z`)
The Windows verdict above was measured on a host that is gone (overseer, founder word 2026-10-03: D: settled by a move to
Omarchy Linux; fresh clone, the fix-A tree restored at HEAD `4a26ad8`, cold target). The full block was re-run here.

**Two environment preconditions the fresh host lacked**, each supplied on the overseer's go and never a code change:
1. `cargo-nextest` was not installed. Prebuilt 0.9.146 from `get.nexte.st` into `~/.cargo/bin`; it printed
   `cargo-nextest 0.9.146 (8af696ddc 2026-09-21)`, host `x86_64-unknown-linux-gnu`.
2. `pulse-app/ui/dist` did not exist, so `tauri::generate_context!()` panicked at compile time ("The `frontendDist`
   configuration is set to `"ui/dist"` but this path doesn't exist"), reddening every entry that compiles pulse-app
   (2, 3, 4, 5, 13, 14, 16 in run `implement-2026-10-03T21-42-17Z`). `npm ci` then `npm run build` in `pulse-app/ui`, exactly
   as `.github/workflows/ci.yml:72-78`; both outputs are gitignored (`.gitignore:41-42`) and no tracked file changed.

**A first-run red, and its mechanism.** The first block after those (same run dir) went red on entry 3 only: both
`incident_events_read_back_cross_process_with_body_and_clean_logs` and
`real_producer_incident_events_read_back_through_the_sidecar` RAN (no `[skip]` line) and got `-32603 incident not found`
for a seeded id. The Secret Service then held ONE corpus key, `created = modified = 2026-10-03 21:54:52` — minted during
that very run: the store was empty, and the concurrently-running seeding test processes each found `NoEntry` in
`OsKeychainBackend::fetch_from_os_store` (`crates/corpus/src/keychain.rs`), each generated and stored its own key, and the
last write won — a process whose rows were encrypted under an overwritten key hands the sidecar undecryptable rows,
which `load_incident_by_id` reports as absent. **Two-sided basis:** the identical tree, re-run with the key present
(`--only 3`), went green 16/16 and the key was not rewritten (`modified` unchanged). The defect is the get-then-set race in
first-ever key creation, pre-existing since chunk 2026-08-15-corpus-key-persistence — not this chunk's. It is surfaced for
an owner (a first-boot app + sidecar, or any two first-run processes, can split the key; the boot disposition then
treats the loser's rows as orphaned). It also makes every cross-process corpus leg order-sensitive on a FRESH credential
store, the existing `pulse-app/tests/e2e_p3_mcp_resolve_content.rs` included.

**Verdict, full block on a warm target:** `entries 22 · green 15 · red 1 (6) · recorded 0 · timeout 0 · not-run 6`.
- Entry 3: `16 tests run: 16 passed`, zero `[skip]` lines. The cross-process legs each RAN and passed:
  `incident_events_subprocess::incident_events_read_back_cross_process_with_body_and_clean_logs`,
  `incident_events_subprocess::incident_events_unknown_id_returns_a_sanitized_not_found_error`,
  `e2e_p3_mcp_incident_events_content::real_producer_incident_events_read_back_through_the_sidecar`.
- Entry 4 collected 16; entry 5 (`package(mcp-server)`) 87/87; entry 13 (default workspace) 2575/2575 — the Windows
  2573 plus the two Unix-only exit-cause arms.
- Entry 6 red with exactly `pulse-app/src/inference_runtime.rs` — the founder's widening (`scope-record.md`).
- Entry 15 (bindings byte-identical to `a69030a`) and entry 16 (the release build) green.

## Restoration confirmed
`cargo nextest run -p corpus -p mcp-server --profile ci -E 'test(/incident_events/)'` on the restored source: exit 0,
`11 tests run: 11 passed` (the 5 corpus and 6 dispatcher pins; the 2 subprocess pins are feature-gated out of this form
and were green in the gated run on the identical source).
