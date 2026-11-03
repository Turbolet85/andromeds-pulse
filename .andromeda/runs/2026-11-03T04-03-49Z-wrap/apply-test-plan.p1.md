# Apply — test-plan · part 1 of 2

## Verdicts
- fanout-test-plan.md 3 · applied · E1
- fanout-test-plan.md 4 · applied · E2
- fanout-test-plan.md 5 · applied · E3
- fanout-test-plan.md 6 · applied · E4
- fanout-test-plan.md 7 · applied · E5
- fanout-test-plan.md 8 · applied · E6
- fanout-test-plan.md 9 · applied · E7
- fanout-test-plan.md 10 · applied · E8
- fanout-test-plan.md 11 · applied · E9
- fanout-test-plan.md 12 · applied · E10
- fanout-test-plan.md 13 · applied · E11
- fanout-test-plan.md 14 · applied · E12
- fanout-test-plan.md 15 · applied · E13
- fanout-test-plan.md 16 · applied · E14
- fanout-test-plan.md 18 · applied · E15
- fanout-test-plan.md 19 · applied · E16
- fanout-test-plan.md 20 · applied · E17
- fanout-test-plan.md 21 · applied · E18
- fanout-test-plan.md 22 · applied · E19
- fanout-test-plan.md 23 · applied · E20
- fanout-test-plan.md 24 · applied · E21
- fanout-test-plan.md 25 · applied · E22
- fanout-test-plan.md 26 · applied · E23
- fanout-test-plan.md 27 · applied · E24
- fanout-test-plan.md 28 · applied · E25
- fanout-test-plan.md 29 · applied · E26

## Edits
@fact id=fct-rjbmbvd5 expect_rev=1
old: | **mcp-server (stdio sidecar)** | Architecture Stack → hand-rolled serde JSON-RPC 2.0 over stdio with `rmcp` as a feature-gated anchor dependency (per the 2026-08-29 [MCP Server Surface] amendment), Workspace/Modules,
new: | **mcp-server (the tools library the door serves)** | Architecture Stack → hand-rolled serde JSON-RPC 2.0, Workspace/Modules,
old: Feature-gated (`--features mcp-server`) and runtime-gated (`ANDROMEDA_PULSE_MCP_ENABLED=true`); JSON-RPC 2.0 over stdin/stdout testable via subprocess pipe; caller is LLM
new: No binary, no cargo feature and no env gate: the crate holds `contract`, `jsonrpc` and `tools`, and `pulse-app` names it as a plain dependency; caller is LLM
old: Both gates are the stdio sidecar's, and `rmcp` still enters only under the feature: since chunk 2026-10-10-door-inside-the-engine-s-process the crate itself is linked by `pulse-app` in every build (the dependency is no longer optional) and its dispatch is also reached, under neither gate, through the console engine's door — `mcp_server::tools::call_tool` (the dispatch with no log record; `dispatch_tool` keeps its signature and calls it) over `door::EngineTools` — testable
new: `mcp_server::tools::call_tool` (the dispatch with no log record) has one product caller, `door::EngineTools::call` — testable
old: alone, in the sidecar's `tools/list` answer too (pin
new: alone (pin

@fact id=fct-6c37jotm expect_rev=1
old: plugin sandbox + MCP sidecar)
new: plugin sandbox + the mcp-server tools served through the console engine door)

@fact id=fct-qlyoxiil expect_rev=1
old: Third product binary beside `pulse-app` and `andromeda-pulse-mcp`, since chunk 2026-10-10-console-engine-entry-point; no `stop` command;
new: One of the two product binaries, beside `pulse-app`; no `stop` command;

@fact id=fct-dxqwgkbv expect_rev=1
old: | **MCP stdio sidecar (rmcp on stdin/stdout)** | subprocess spawning with feature flag `--features mcp-server` + env `ANDROMEDA_PULSE_MCP_ENABLED=true` + JSON-RPC 2.0 client writing to stdin | JSON-RPC 2.0 response on stdout with `result` (success) or `error` (failure); agent observes via exit code (must be 0 on graceful shutdown) + stdout line parsing (jq-extractable `result.traces` / `result.metrics` / `result.logs` arrays); MCP tool call (e.g., `query_traces(time_range, service_filter, limit)`) asserted via call/result round-trip with structured output. | single-surface (stdio sidecar only; caller is external LLM agent, untestable in harness) | Feature-gated at compile time; both compile-time and runtime gates must be asserted in negative test (attempt to spawn without feature flag must fail). Cannot test external LLM behavior; only test that tool schema is valid and tool invocations return correct data shape. |
new: | **MCP stdio sidecar** | N/A — retired: no stdio sidecar binary, no compile-time or runtime gate and no subprocess test exist; an agent on stdio reaches the nine tools through the relay `andromeda-pulse-engine door` (the door row) | — | — | — |

@fact id=fct-ot5xbdcp expect_rev=1
old: | MCP stdio sidecar (feature-gated `--features mcp-server`) + buffer access + JSON-RPC 2.0 contract | Agent: spawns app with `ANDROMEDA_PULSE_MCP_ENABLED=true --features mcp-server` → pipes JSON-RPC 2.0 call to stdin: `{"jsonrpc": "2.0", "method": "tools/call", "params": {"name": "query_traces", "arguments": {"time_range": [now-5m, now], "service_filter": "my-app", "limit": 100}}}` → reads stdout for JSON-RPC result → asserts `result.traces` is array of trace objects with schema match (trace_id, span_name, duration_ms, etc.) →
new: | the console engine door (`run/door/sock`, or the stdio relay `andromeda-pulse-engine door`) + buffer access + JSON-RPC 2.0 contract | Agent: spawns `andromeda-pulse-engine run` → sends telemetry over OTLP → writes a JSON-RPC 2.0 line to the door: `{"jsonrpc": "2.0", "method": "tools/call", "params": {"name": "query_traces", …}}` → reads the answer line → asserts `result` and the tool's shape → reads one `door.call` record per answered request in the engine's log family (`pulse-app/tests/e2e_p3_mcp_subprocess_tools_call.rs`; no feature flag, no env gate) →

@fact id=fct-ft7bwy6m expect_rev=1
old: | Security Plan Vector 4: "gated by compile-time `--features mcp-server` AND runtime `ANDROMEDA_PULSE_MCP_ENABLED=true`" | Negative test: (1) attempt to spawn MCP sidecar without feature flag compile → build fails; (2) runtime env var unset, attempt to spawn → assert sidecar not spawned (cannot find binary or refused at startup). Positive test: both gates enabled → sidecar spawns and accepts JSON-RPC 2.0. Both gates are the stdio sidecar's: since chunk 2026-10-10-door-inside-the-engine-s-process the same nine tools also answer through the console engine's door under neither gate, admitted
new: | Security Plan Vector 4 | N/A — retired: no `mcp-server` feature, no runtime gate and no sidecar exist. The nine tools answer only through the console engine's door, admitted
old: that path's negative tests are the door's — another account refused at three layers and, since chunk 2026-10-12-stdio-agent-reaches-the-door, another account
new: its negative tests carry Vector 4 — another account refused at three layers and another account
old: also through the relay — per Surfaces
new: also through the relay, and an undeclared argument member refused with `-32602` for each of the nine tools (`declared_only`) — per Surfaces

@fact id=fct-7n6noty7 expect_rev=1
old: tool call routing; schema validation; feature-gate build verification
new: tool call routing (`call_tool`); schema validation

@fact id=fct-o6sm757u expect_rev=1
old: - **Module ↔ MCP sidecar:** `tokio::process::Command` spawning subprocess + JSON-RPC 2.0 client (`serde_json`)
new: - **Module ↔ engine door:** a `tokio::net::UnixStream` client writing JSON-RPC 2.0 lines (`serde_json`) to `run/door/sock`, on an engine booted in process by `engine_boot::start` or a spawned `andromeda-pulse-engine run`

@fact id=fct-6l7zuqld expect_rev=1
old: | **MCP stdio sidecar** | `tokio::process::Command` + JSON-RPC 2.0 client | direct subprocess | Spawn with feature flag + env var; write frames to stdin, parse stdout |
new: | **console engine door** | `tokio::net::UnixStream` client + JSON-RPC 2.0 lines (and the spawned stdio relay `andromeda-pulse-engine door`) | in process (`engine_boot::start`) or a spawned `andromeda-pulse-engine run` | No feature flag, no env var; write one request per line to `run/door/sock`, parse the answer line |

@fact id=fct-y4dwpurc expect_rev=1
old: MCP stdio sidecar (feature-gated), buffer access
new: the console engine door (socket or stdio relay), buffer access

@fact id=fct-c5f6426b expect_rev=1
old: 1. Boot app with `ANDROMEDA_PULSE_MCP_ENABLED=true --features mcp-server` via cargo build
new: 1. Spawn the built `andromeda-pulse-engine run` on its own data dir and two claimed loopback ports
old: 2. Spawn `./target/debug/andromeda-pulse --features mcp-server` via `tokio::process::Command` with stdin/stdout piped
new: 2. Send synthetic spans and logs to it over OTLP gRPC
old: 3. Populate buffer with 100 synthetic spans via OTLP ingest
new: 3. Ask through `run/door/sock` with a `tokio::net::UnixStream` client, one connection per request line
old: 4. Write JSON-RPC 2.0 frame to sidecar stdin: `{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"query_traces","arguments":{"time_range":["2026-05-02T16:10:00Z","2026-05-02T16:20:00Z"],"service_filter":"test-app","limit":100}}}`
new: 4. Write JSON-RPC 2.0 `tools/call` lines for `query_traces`, `query_metrics`, `query_logs` and `generate_snapshot`, and two more carrying an SQL-injection fragment and a sentinel as the cursor
old: 5. Parse stdout line-delimited JSON; extract `result.traces` array
new: 5. Parse each answer line; assert the four tool calls answer `result` (not `error`)
old: 6. Assert `result.traces.length > 0`; verify each row has trace_id / span_name / duration_ms fields
new: 6. Assert each tool's result shape; after the engine ends, read one `door.call` record per answered request, and the SQL-injection fragment and the sentinel cursor 0 times, in its log family (`pulse-app/tests/e2e_p3_mcp_subprocess_tools_call.rs`)

@fact id=fct-erazkrzq expect_rev=1
old: send SIGTERM to sidecar PID; verify exit code 0; verify no zombie process
new: end the spawned engine by SIGTERM and read its log family after the end (§3 → Per-chunk gate discipline, Spawned-program witness form)

@fact id=fct-m4xvcm7g expect_rev=1
old: - **Current residual (chunk 2026-08-17-conductor-e2e-verification-closure):** the subprocess steps above are committed ONLY for the 4 telemetry tools. The **incident**-tool read-back leg (`retrieve_telemetry_slice` / `retrieve_report` / `query_incident_list` / `mark_incident_resolved`) is committed only as **in-process `dispatch_tool`** assertions in `pulse-app/tests/e2e_p3_mcp_incident_tools.rs` — despite that file's `e2e_p3_` name, its own docstring points cross-process framing at `crates/mcp-server/tests/sidecar_subprocess.rs`, where the incident tools appear only as `tools/list` NAME assertions. The JSON-RPC subprocess form for the deterministic evidence read-back has been proven live as smoke evidence, not as a committed step. **Widened at chunk 2026-08-27-incident-persist-vs-resolve-write-race:** `mark_incident_resolved` gained an honest not-applied result for a write the corpus monotonic guard declines. **That half is DISCHARGED at chunk 2026-08-29-app-registry-reconciliation** — `pulse-app/tests/e2e_p3_mcp_resolve_content.rs` spawns the real sidecar subprocess and asserts `mark_incident_resolved`'s response CONTENT in both the applied and the guard-declined arms (feature-gated behind `--features mcp-server`, so outside the default workspace gate). The residual is therefore **re-scoped to the remaining three incident tools** — `retrieve_telemetry_slice` / `retrieve_report` / `query_incident_list` — which are still committed only as in-process `dispatch_tool` assertions. **The ninth tool, `retrieve_incident_events` (chunk 2026-10-02-incident-events-readable-through-mcp), shipped with the subprocess form COMMITTED** — `crates/mcp-server/tests/incident_events_subprocess.rs` and the real-producer → real-sidecar leg `pulse-app/tests/e2e_p3_mcp_incident_events_content.rs` (`--features mcp-server`) — so the residual stays exactly those three read-back tools. Tracked as `mcp-incident-read-back-cross-process-coverage` in §1.
new: - **Current residual:** the steps above are committed for the 4 telemetry tools. The incident tools are asserted in process through `call_tool` in `pulse-app/tests/e2e_p3_mcp_incident_tools.rs` and, for `mark_incident_resolved` and `retrieve_incident_events`, through the door socket of a console engine booted in process with `FakeKeychainBackend` (`pulse-app/tests/e2e_p3_mcp_resolve_content.rs`, `pulse-app/tests/e2e_p3_mcp_incident_events_content.rs`), all inside the default workspace run, on CI too; the declined resolve arm and what is still owed — the three read-back tools `retrieve_telemetry_slice` / `retrieve_report` / `query_incident_list` — stand at the §1 row `mcp-incident-read-back-cross-process-coverage`.

@fact id=fct-nsjvxbz7 expect_rev=1
old: | Process mocking (MCP sidecar) | `tokio::process::Command` (built-in) | spawn, pipe stdin/stdout, send JSON-RPC 2.0 frames |
new: | Process (spawned engine and door relay) | the built `andromeda-pulse-engine` spawned as a process (no mocking library) | spawn `run`, or the `door` relay, and write JSON-RPC 2.0 lines to the door |

@fact id=fct-js2thrtn expect_rev=1
old: Evidence section, both non-vacuous since chunk 2026-08-17-conductor-e2e-verification-closure — is guarded ONLY by in-process `dispatch_tool` assertions in `pulse-app/tests/e2e_p3_mcp_incident_tools.rs` (whose `e2e_p3_` name is misleading: its own docstring points cross-process framing at `crates/mcp-server/tests/sidecar_subprocess.rs`). **Verified by search, not inferred:** the committed subprocess tests `tools/call` only the 4 telemetry tools (`query_traces` / `query_metrics` / `query_logs` / `generate_snapshot`); the 4 incident tools appear cross-process ONLY as `tools/list` NAME assertions (`sidecar_subprocess.rs:122-124`), never with a content assertion. The real subprocess chain WAS proven live at that chunk's P3 smoke, but smoke evidence is not a regression guard — a future chunk needs a new test file spawning the sidecar and asserting incident-tool response CONTENT. Chunk 2026-08-17-conductor-e2e-verification-closure. **The owed content WIDENED at chunk 2026-08-27-incident-persist-vs-resolve-write-race:** `mark_incident_resolved` no longer returns `{"resolved": true}` unconditionally — a write the corpus monotonic guard DECLINES now returns an honest not-applied result — and that new response shape was covered ONLY by the in-process signature/compile path, so a revert to the always-true form would have passed every committed cross-process assertion. **NARROWED at chunk 2026-08-29-app-registry-reconciliation — the widened half is DISCHARGED, and the "ONLY as `tools/list` NAME assertions" claim above no longer holds for all four tools.** `pulse-app/tests/e2e_p3_mcp_resolve_content.rs` spawns a real `andromeda-pulse-mcp` subprocess against a shared on-disk encrypted corpus and asserts `mark_incident_resolved`'s response CONTENT in BOTH arms — the applied shape (`resolved: true` + the id it resolved) and the guard-declined shape (no success result; the error message reports the resolution was not applied, driven by seeding a row whose `updated_unix_nano` is newer than the sidecar's `now`). 2/2, and neither SKIPPED: both open the corpus with `OsKeychainBackend::new("com.andromeda.pulse")`, the same backend + service id the sidecar uses, with a clean skip only where no credential store exists. **It is FEATURE-GATED** (`cargo test --test e2e_p3_mcp_resolve_content -p pulse-app --features mcp-server`) and therefore sits OUTSIDE the default `--workspace` gate, like the other `--features mcp-server` pulse-app tests. **NARROWED again at chunk 2026-10-02-incident-events-readable-through-mcp — the ninth tool shipped WITH its cross-process content guard.** `retrieve_incident_events` (a fifth incident tool) has two committed legs: `crates/mcp-server/tests/incident_events_subprocess.rs` spawns the real sidecar against seeded rows and asserts the response CONTENT plus stderr and file-sink canary absence; `pulse-app/tests/e2e_p3_mcp_incident_events_content.rs` (`--features mcp-server`) drives the REAL producer `create_incident_from_l4_output` into the real sidecar and asserts the four-kind vocabulary with `created` first. Both clean-skip without an OS credential store (every CI runner) and both ran on the Linux dev host (`lacks [skip]` held, 16/16).
new: Evidence section — is guarded by in-process `call_tool` assertions in `pulse-app/tests/e2e_p3_mcp_incident_tools.rs`, under default features. Two incident tools carry content tests through the console engine door: `pulse-app/tests/e2e_p3_mcp_resolve_content.rs` asserts `mark_incident_resolved`'s answer in both arms — the declined arm reads no `result`, an error answer carrying the door's fixed message, the row still `active` and listed — and `pulse-app/tests/e2e_p3_mcp_incident_events_content.rs` asserts `retrieve_incident_events`' content; both boot the console engine in process with `FakeKeychainBackend` and read through the door socket inside the default workspace run, on CI too, with no feature gate and no credential-store skip. `crates/mcp-server` holds no `tests/` dir.

@fact id=fct-i5odf4oq expect_rev=1
old: | Call `query_traces` via MCP → assert `result_content` NOT in `agent-latest.jsonl`, only `result_type` + `result_count` metadata.
new: — **LANDED, no longer pending** | `pulse-app/tests/security_mcp_response_body_redaction.rs` calls `query_traces` through the door of a spawned console engine over spans whose `service.name` resource attribute carries a marker (a trace row answers no other attribute) and asserts the marker 0 times in the engine's log family.
old: (vector 4) |
new: (vector 4) Row retained for audit trail. |

@fact id=fct-p2dhprbq expect_rev=1
old: and the `<data_dir>/run/workspace-key` publish/read round-trip (app writes at boot, `andromeda-pulse-mcp` sidecar reads) is exercised at the filesystem boundary.
new: and the engine stamps incidents with `incident_workspace_key`; neither program writes `<data_dir>/run/workspace-key` (`publish_workspace_key` / `read_published_workspace_key` stay in the library with no product caller).

@fact id=fct-5c3xpzsh expect_rev=1
old: | The app→sidecar key handoff is proven in-process (byte-equality of published vs read key, plus the absent-file fallback) and by `cargo build -p mcp-server --features mcp-server`, but no test spawns the sidecar as a subprocess against a data dir holding a published key and asserts the key-scoped query returns the app's rows. The blocker is CLEARED as of chunk 2026-08-15-corpus-key-persistence (cross-process encrypted-payload reads now succeed — a second process decrypted its predecessor's rows with 0 `decryption_failed`), so the leg is UNBLOCKED but still unimplemented. Qualifier: it presumes a host with a credential store OR a configured `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` — the fallback is opt-in, not automatic. **MEASURED ONCE at smoke level** (chunk 2026-08-17-conductor-e2e-verification-closure): a real `andromeda-pulse-mcp` subprocess read the live AES-256-GCM corpus under the published `run/workspace-key` and returned the key-scoped row's `fingerprint_refs`, with the 2 persisted rows' `workspace` matching the published key byte-for-byte, 0 `corpus.read.undecryptable` and no keychain fallback — so the leg is now known-good in practice while STILL having no committed test. Chunks 2026-08-14-workspace-key-alignment (opened) + 2026-08-15-corpus-key-persistence (unblocked) + 2026-08-17-conductor-e2e-verification-closure (measured, uncommitted). |
new: | **RETIRED — its subject no longer exists.** Neither program writes `run/workspace-key` and no stdio sidecar exists to read it; `publish_workspace_key` / `read_published_workspace_key` stay in `workspace-detector` with no product caller, their removal owned by the route entry «Workspace detection retired». Row retained for audit trail. |

@fact id=fct-rnb7vczj expect_rev=1
old: | Publish the app's resolved key (atomic `.tmp`+rename), read it back through the sidecar's read path; assert byte-equality with the app's filter key, and that an absent/invalid file falls back to `data_dir` on both ends | `tempfile` 3.x; `tokio::process::Command` for the sidecar-subprocess leg per Module ↔ MCP sidecar |
new: | N/A — retired: no program publishes the key and no sidecar reads it | — |

@fact id=fct-mbichyuo expect_rev=1
old: | Linux: `cargo nextest run --workspace --features mcp-server --profile ci --no-tests=fail` — a selection that matches no test fails the job (since chunk 2026-10-10-no-gate-stands-while-reading-nothing; pinned by `ci_workflow_nextest_runs_fail_on_an_empty_selection`; the runner's cargo-nextest 0.9.133 took the flag and ran 2946 tests, as measured on `ci#38042949735`) | own job | restore-only `lint-test-Linux` |
new: | N/A — retired: `ci.yml` holds no `mcp-test` job and no `mcp-server` feature, pinned absent by `ci_workflow_holds_no_mcp_test_job_and_no_mcp_server_feature` (`pulse-app/tests/quality_gate_workflow.rs`) | — | — |

@fact id=fct-ep45valw expect_rev=1
old: the same tests run in the `mcp-test` and `coverage` jobs;
new: the same tests run in the `coverage` job;
old: the key the `mcp-test` and `a11y` jobs restore read-only
new: the key the `a11y` job restores read-only

@fact id=fct-p7fl7gmb expect_rev=1
old: Each of the six jobs of `ci.yml` declares
new: Each of the five jobs of `ci.yml` (lint-test · a11y · boot · supply-chain · coverage) declares

@fact id=fct-b5tpgm6t expect_rev=1
old: `cargo build --workspace --release --features mcp-server` → `Build the exit witness`
new: `cargo build --workspace --release` → `Build the exit witness`

@fact id=fct-gnnl6mzk expect_rev=1
old: the `boot` job's `cargo build --workspace --release --features mcp-server`
new: the `boot` job's `cargo build --workspace --release`

@fact id=fct-n45lk2to expect_rev=1
old: (113 top-level `pulse-app/tests/*.rs` files as of chunk 2026-10-12-stdio-agent-reaches-the-door — the one new `unit_door_bridge.rs` — 112 at its base;
new: (112 top-level `pulse-app/tests/*.rs` files, as measured by `ls pulse-app/tests/*.rs | wc -l` at the work tree of chunk 2026-11-03-stdio-sidecar-retired;

@fact id=fct-mqmrzd2o expect_rev=1
old: (`pulse-app/tests/unit_door.rs`, 19 tests;
new: (`pulse-app/tests/unit_door.rs`, 20 tests;

## Patterns
[[pattern]]
id = "tp1-01"
fixed = "as a feature-gated anchor dependency"

[[pattern]]
id = "tp1-02"
fixed = "Both gates are the stdio sidecar's"

[[pattern]]
id = "tp1-03"
fixed = "plugin sandbox + MCP sidecar"

[[pattern]]
id = "tp1-04"
fixed = "Third product binary beside"

[[pattern]]
id = "tp1-05"
fixed = "subprocess spawning with feature flag"

[[pattern]]
id = "tp1-06"
fixed = "pipes JSON-RPC 2.0 call to stdin"

[[pattern]]
id = "tp1-07"
fixed = "attempt to spawn MCP sidecar without feature flag"

[[pattern]]
id = "tp1-08"
fixed = "feature-gate build verification"

[[pattern]]
id = "tp1-09"
fixed = "Module ↔ MCP sidecar"

[[pattern]]
id = "tp1-10"
fixed = "Spawn with feature flag + env var"

[[pattern]]
id = "tp1-11"
fixed = "MCP stdio sidecar (feature-gated)"

[[pattern]]
id = "tp1-12"
fixed = "Write JSON-RPC 2.0 frame to sidecar stdin"

[[pattern]]
id = "tp1-13"
fixed = "send SIGTERM to sidecar PID"

[[pattern]]
id = "tp1-14"
fixed = "spawns the real sidecar subprocess and asserts"

[[pattern]]
id = "tp1-15"
fixed = "Process mocking (MCP sidecar)"

[[pattern]]
id = "tp1-16"
fixed = "is guarded ONLY by in-process `dispatch_tool`"

[[pattern]]
id = "tp1-17"
fixed = "only `result_type` + `result_count` metadata"

[[pattern]]
id = "tp1-18"
fixed = "sidecar reads) is exercised at the filesystem boundary"

[[pattern]]
id = "tp1-19"
fixed = "The app→sidecar key handoff is proven in-process"

[[pattern]]
id = "tp1-20"
fixed = "read it back through the sidecar's read path"

[[pattern]]
id = "tp1-21"
fixed = "--workspace --features mcp-server --profile ci"

[[pattern]]
id = "tp1-22"
fixed = "run in the `mcp-test` and `coverage` jobs"

[[pattern]]
id = "tp1-23"
fixed = "the `mcp-test` and `a11y` jobs restore read-only"

[[pattern]]
id = "tp1-24"
fixed = "Each of the six jobs of `ci.yml`"

[[pattern]]
id = "tp1-25"
fixed = "cargo build --workspace --release --features mcp-server"

[[pattern]]
id = "tp1-26"
fixed = "113 top-level `pulse-app/tests/*.rs` files"

[[pattern]]
id = "tp1-27"
fixed = "`pulse-app/tests/unit_door.rs`, 19 tests"
