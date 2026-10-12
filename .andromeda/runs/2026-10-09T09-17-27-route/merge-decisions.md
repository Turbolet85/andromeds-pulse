# Merge decisions — andromeda-pulse-0.4.0

_Phase 3. One line per suggestion: `{validator} {Insert|Reorder|Rewrite|Remove} · {applied|adjusted|rejected|deferred} · {reason}`. 30 suggestions from 5 validators: 15 applied · 11 adjusted · 1 rejected · 3 deferred. No suggestion asked to install, extend or sequence work for a retired surface. The already-delivered check read `.andromeda/master-route.md` (83 `complete` records, all andromeda-pulse-0.3.0) and the masters for what earlier versions shipped._

## a11y — 0 suggestions

a11y No-suggestions · n/a · the validator reads all nine bootstrap items of its plan as retired with the window by P-083 and confirms the gate-before-surface order; nothing to judge.

## design — 2 suggestions

design Rewrite `Window retired` (+ "the deferred signature's fate") · rejected · misreads: the Halo State Pulse layer is a surface P-083 retires, and its residual (`.andromeda/residuals.md:13`) was dispositioned at Phase A — dropped on the operator's word of 2026-10-09 — so its fate is decided and the flip is Phase 6's; the chunk's "masters state no interface this version" already covers the deferred passages in both design masters.
design Rewrite `System notification on a change of state` (+ "its terse form stated") · applied · the notification is a new surface of P-099, and its only stated form rule (design-system §Surface: desktop-native → Notifications) leaves with the window five epochs earlier; the successor's form has to be stated somewhere.

## security — 10 suggestions

security Insert "Supply-chain gate re-based on the smaller graph" (Epoch 2) · applied · security-plan §Dependency Security → CI integration says a stale skip is pruned, not kept (its line 219, re-read); `deny.toml` still carries the Tauri GTK3 and unic advisory ignores (re-read). Placed after `Local model retired`, since its original anchor `Corpus encryption at rest retired` moved to Epoch 1.
security Insert "Channel key custody on the node" (Epoch 3) · deferred · a control for a new surface that no finding states: intent F2/F2b give a lifecycle to the token only and do not say who terminates the encrypted channel, so the chunk would presume the engine holds a private key. Raised at Phase 4 as a requirement the intent may be missing.
security Reorder `Corpus encryption at rest retired` before `Headless engine entry point` · adjusted · a real dependency, verified: no workflow, script or xtask source sets `ANDROMEDA_PULSE_CORPUS_PASSPHRASE` (0 hits), and architecture §Occupied Resources records that a host with neither a credential store nor a passphrase has no corpus — so the headless boot and its read-back gate could not hold an incident on a CI runner. Adjustment: the chunk crosses from Epoch 2 into Epoch 1, so both epoch titles change (Epoch 1 gains "readable stores"; Epoch 2 loses "the corpus's encryption"). The operator reviews this order at Phase 4.
security Reorder `Door admission and bounds` before `Door queries` · applied · admission precedes the two chunks that widen what the door returns; security-plan §Threat Model Summary records "no network exposure" as the sidecar's trust boundary, which P-092 replaces.
security Rewrite `Token lifecycle by engine command` (+ "not recoverable from the node's stores") · applied · restates F2b's "shown once" under P-085's unencrypted stores; security-plan §Secret Management "What counts as secret".
security Rewrite `Corpus encryption at rest retired` ("readable only by the engine's account") · applied · security-plan §Data Protection → At rest states access control by OS user permissions for each medium; once cell encryption leaves, that boundary is the corpus's only at-rest control — what must still hold without the retired thing.
security Rewrite `Telemetry store on disk` (+ "owner-only") · applied · same ground as the line above, for the store that P-091 puts on disk.
security Rewrite `Door admission and bounds` (+ "encrypted in transit") · deferred · a control for a new surface that no finding states: F6b lists admission, store-only reads, bounded answers and the switchable write, and F2's encrypted channel is stated for telemetry only. Raised at Phase 4 with the channel-key item.
security Rewrite `Span keeps its identity` (+ "each scrubbed before storage") · applied · the new client-controlled columns fall under P-088's last clause and security-plan §Security Anti-Patterns → Logging.
security Rewrite `Real service watched for days` (+ "with the personal data it carries") · deferred · it adds a question to what the founder is asked before P-101 is taken up, and touches the security master's "Compliance triggers: None", which rests on telemetry staying on the user's own machine. His to answer; raised at Phase 4.

## obs — 6 suggestions

obs Insert "Engine's own log from first boot" (Epoch 1) · adjusted · the log stack, service identity, heartbeat and panic hook were delivered by earlier versions (the masters record them present); what is new is that a console host needs its own sink drain and process-end record. Folded into `Headless engine entry point` ("own log, heartbeat, process-end record kept") instead of a separate chunk.
obs Insert "Engine gate's log kept and graded" (Epoch 1) · adjusted · same direction as tests' harness Insert; merged into one inserted chunk, `Agent harness drives the headless engine` ("panic, heartbeat-gap and budget checks grade its log"), which precedes the gate and so precedes the boot-smoke job's retirement.
obs Insert "Refusals visible without flooding the log" (Epoch 3) · adjusted · folded into `Receiver refusals and per-sender bounds` ("refusals logged as bounded counts"); one unit of work with the refusals themselves.
obs Insert "Engine's own liveness stays on the process clock" (Epoch 3) · adjusted · a constraint on the event-time chunk, not a unit of its own; folded into `Checks reason by event time`.
obs Insert "Disk store's size and drain progress in the engine's log" (Epoch 4) · adjusted · merged with tests' load-profile Insert into one chunk, `Disk store measured under load`.
obs Rewrite `Real service watched for days` (+ "the engine's own log") · adjusted · applied with a trim ("named first") to stay inside 25 words.

## tests — 12 suggestions

tests Insert "Agent harness drives the headless engine" (Epoch 1) · adjusted · applied; its scope hint also carries obs' log-grading Insert (see above), and "with no display server" is dropped because the entry-point chunk states it.
tests Insert "End-to-end paths re-driven through the engine" (Epoch 1) · applied · test-plan §6 drives six of the seven critical paths through the window's IPC; the detection scenarios need a headless driver before any removal, or `Detection parity after the removals` has nothing to compare against. "Named for retirement" for P2, P4 and P7 is a decision the intent does not make — raised at Phase 4.
tests Insert "Shared telemetry test data" (Epoch 1) · adjusted · applied with the requirement ids it serves (P-089, P-094). Not delivered by an earlier version: test-plan §3 Bootstrap phases item 6 records it unimplemented.
tests Insert "Load and fault profiles on the disk store" (Epoch 4) · adjusted · merged with obs' disk-store Insert into `Disk store measured under load`; "latency budgets restated" is dropped — restating a budget is the test master's amendment at that chunk's wrap.
tests Rewrite `Theme 1 checked by the external harness` (+ "path pinned in engine gate") · applied · a reading taken outside this repository guards nothing here; the engine gate reached in Foundation is where each theme's path gets its committed assertion.
tests Rewrite `Theme 2 checked by the external harness` · applied · as Theme 1.
tests Rewrite `Theme 3 checked by the external harness` · applied · as Theme 1.
tests Rewrite `Theme 4 checked by the external harness` · applied · as Theme 1.
tests Rewrite `Theme 5 checked by the external harness` · applied · as Theme 1; the panel and the notification themselves are on the founder's desktop, so what the gate can pin is the engine's side of the chain.
tests Rewrite `Recorded stream replays to the same result` ("a captured synthetic stream") · applied · test-plan §7 self-bootstrapping: a committed replay fixture is generated telemetry, never a capture from a watched service.
tests Rewrite `The door inside the engine's process` (+ "and test job") · applied · the `mcp-test` CI job's retirement becomes stated work.
tests Rewrite `Version close on Linux` (+ "held by the capability verification matrix") · adjusted · the validator's correction is taken: test-plan §9's capability verification matrix checks product capabilities P-001…P-060, not Tauri capabilities, so it is not one of the window's gates. Two edits instead of the proposed text: `Window's gates retired` now reads "Tauri-capability gates", and `Version close on Linux` states the outcome ("the older capability matrix gate carries no retired anchor"). Not taken: extending that file with this version's ids — the version's own `verification-matrix.json` is the ledger for P-083…P-103.
