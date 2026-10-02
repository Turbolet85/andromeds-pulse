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

## Restoration confirmed
`cargo nextest run -p corpus -p mcp-server --profile ci -E 'test(/incident_events/)'` on the restored source: exit 0,
`11 tests run: 11 passed` (the 5 corpus and 6 dispatcher pins; the 2 subprocess pins are feature-gated out of this form
and were green in the gated run on the identical source).
