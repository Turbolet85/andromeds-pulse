# Working Route — andromeda-pulse-0.4.0

_Ordered WHAT-not-HOW chunk list for this version. Reorder = move up/down._
_No numbers, no per-chunk IDs/metadata. At promotion /andromeda-phase prefixes the chunk's line with_
_`[{marker}]` to freeze it (wrap's route-resolve edits only the markerless tail; once the chunk's master_
_record is complete, wrap P7 flip-compacts its line to `[{marker}] {title} — {scope hint}`, archiving the_
_verbatim line to route-archive.md); markerless lines stay mutable._
_Chunks separated by `   ↓` within an epoch; only `### Epoch K — {name}` headers are structural._

### Epoch 1 — Foundation: readable stores, a headless engine and its gates on Linux
Corpus encryption at rest retired — stores readable only by the engine's account, no credential store; scrubber unchanged; decision dated and marked for review (P-085)
   ↓
Headless engine entry point — a console program boots ingest, buffer, detectors, corpus with no display, driven by commands; own log, heartbeat, process-end record kept (P-086)
   ↓
Agent harness drives the headless engine — boot, run, status, cleanup and logs target the console program; panic, heartbeat-gap and budget checks grade its log (P-086)
   ↓
Engine end-to-end gate reachable — CI boots the headless engine under a 1 GB cap, sends OTLP, reads a finding back; recorded green verdict (P-086, P-090)
   ↓
End-to-end paths re-driven through the engine — detection scenarios run headless before any removal; each IPC-driven critical path re-driven or named for retirement (P-086)
   ↓
Windows and macOS CI legs retired; pre-push check native on Linux — surviving stages run on the dev host with no second operating system (P-103)
   ↓
Shared telemetry test data — spans, metric points and log records from one set of factories, with settable event time, service and attributes (P-089, P-094)

### Epoch 2 — What leaves: the window and the local model
Window's gates retired — a11y and boot-smoke CI jobs, webview suite, headful self-verify, UX e2e, bindings and Tauri-capability gates, npm channel gate (P-083)
   ↓
Window retired — Tauri shell, webview interface, IPC bridge, tray, updater and desktop bundles leave; design, layout and a11y masters state no interface this version (P-083)
   ↓
Incident worded from the engine's own facts — an incident forms from its cue and reads plainly with no model in its path (P-084, P-099)
   ↓
Local model retired — inference runtime, grammar, hardware probe, deterministic stand-in, decision probe and latency grader leave; no gate needs a GPU (P-084)
   ↓
Supply-chain gate re-based on the smaller graph — advisory, licence, ban and secret-scan gates stay; ignores and carve-outs for departed crates pruned (P-083, P-085)
   ↓
Detection parity after the removals — everything 0.3.0 detected is still detected: five cue families, one incident per storm, auto-resolve (P-086)

### Epoch 3 — The engine stands on its own node
Network OTLP receiver behind a token — a service on another host sends over an encrypted channel; replaces the loopback-only boundary (P-087)
   ↓
Receiver refusals and per-sender bounds — bad or missing token refused before body read; plaintext refused; no sender starves another; refusals logged as bounded counts (P-088)
   ↓
Token lifecycle by engine command — made, shown once, replaced, revoked; not recoverable from the node's stores; never written to a log or a report (P-088)
   ↓
Checks reason by event time — windows, cadences, restart detection, retention read record stamps, lateness allowance stated; engine's own liveness stays on the process clock (P-089)
   ↓
Recorded stream replays to the same result — a captured synthetic stream played again gives identical findings; late batches counted where they belong (P-089)
   ↓
Node size measured — the engine's process memory over a long run inside 1 GB; replaces the buffer's row-count memory estimate (P-090)
   ↓
Theme 1 checked by the external harness — telemetry from another host in, findings out; reading recorded, path pinned in engine gate (P-102)

### Epoch 4 — Memory on disk and a working door
Telemetry store on disk — raw telemetry kept for days, owner-only, surviving restart, bounded under constant insert and delete; replaces the in-memory ring buffer (P-091, P-090)
   ↓
Learned state survives a restart — the baseline is kept; a gap while the engine was down is not read as the service's silence (P-091)
   ↓
Disk store measured under load — size, drain progress, stall signal in the engine's log; load profiles and kill mid-write hold inside 1 GB (P-090, P-091)
   ↓
The door inside the engine's process — agent tools read the store the engine holds; replaces the stdio sidecar, its empty database and test job (P-092)
   ↓
Door admission and bounds — admitted parties only; store-only reads; answers bounded in size and time; the resolve write named and switchable (P-093)
   ↓
Door queries — telemetry filtered by service, time range, trace and text, with aggregates (P-092)
   ↓
One incident whole through the door — the incident with its evidence; span and timestamp references populated (P-092)
   ↓
Theme 2 checked by the external harness — restart survival and the door's answers, input to output; reading recorded, path pinned in engine gate (P-102)

### Epoch 5 — Keep what OpenTelemetry sends
Span keeps its identity — name, kind, parent and status message stored; the service's version and environment kept, each scrubbed before storage (P-094)
   ↓
Span and resource attributes kept — route, status code, database and peer under both semantic-convention generations, scrubbed before storage (P-094)
   ↓
Who calls whom — the caller-and-callee question answered from stored spans; the two parent-keyed aggregations read real data (P-094)
   ↓
Histogram metrics keep their shape — histogram-family points stored so a percentile can be computed (P-095)
   ↓
Logs and metrics name their service — service identity on every row; exception type, message and stack readable through the door (P-095)
   ↓
Theme 3 checked by the external harness — what was sent is what the door returns; reading recorded, path pinned in engine gate (P-102)

### Epoch 6 — Comparisons that work from the first minute
New error kind is a finding — the first appearance of an error fingerprint for a service, and the return of one that had gone (P-096)
   ↓
Release noticed — a new version of a service is recorded with the moment it appeared (P-097)
   ↓
Release compared with the one before — error rate, new error kinds and latency; the engine says when the new version is worse (P-097)
   ↓
Silence told apart — service went quiet, no data reached the engine, check not evaluable: three states; recovery announced (P-098)
   ↓
Theme 4 checked by the external harness — a worse release and a quiet service, each told correctly; reading recorded, path pinned in engine gate (P-102)

### Epoch 7 — The voice
Short report without a model — what, where, since when, after which version, which errors are new (P-099)
   ↓
State in one line for a desktop panel — an engine command prints the current state; a panel module on the founder's desktop shows it (P-099)
   ↓
System notification on a change of state — raised by the engine, carrying the short report, its terse form stated; replaces the window's snapshot-ready toast (P-099)
   ↓
Report names its evidence — evidence cited by identifier, the report narrow; "not enough data" is an answer the engine gives (P-100)
   ↓
Large model names the planted cause — given the report and the door, measured over planted faults (P-100)
   ↓
Theme 5 checked by the external harness — a planted fault reaches panel, notification and report; reading recorded, path pinned in engine gate (P-102)

### Epoch 8 — Polish & ship
Real service watched for days — one of the founder's services, named first; reports, silences and the engine's own log read against what happened (P-101)
   ↓
Bad-version scenario end to end — Conductor over the network to an engine on another host, a 1 GB node, silent through the healthy hours (P-102)
   ↓
Version close on Linux — full gate green; every capability verified or deferred on a measured basis; the older capability matrix gate carries no retired anchor
