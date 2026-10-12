# Working Route — andromeda-pulse-0.4.0

_Ordered WHAT-not-HOW chunk list for this version. Reorder = move up/down._
_No numbers, no per-chunk IDs/metadata. At promotion /andromeda-phase prefixes the chunk's line with_
_`[{marker}]` to freeze it (wrap's route-resolve edits only the markerless tail; once the chunk's master_
_record is complete, wrap P7 flip-compacts its line to `[{marker}] {title} — {scope hint}`, archiving the_
_verbatim line to route-archive.md); markerless lines stay mutable._
_Chunks separated by `   ↓` within an epoch; only `### Epoch K — {name}` headers are structural._

### Epoch 1 — Foundation: a console engine, its gates and its records before anything leaves
CI on Linux alone — Windows and macOS legs leave three jobs, the release-build job whole; one run's wall-clock recorded (P-113)
   ↓
Supply-chain job same on push and pull request — fails on a finding, never on its own reporting; repair witnessed on a push (P-120)
   ↓
Pre-push check native on Linux — surviving stages run on the dev host; the second-system hop and its distro clone leave (P-103, P-113)
   ↓
Capability record re-based — one current record for all 82 ids, each claimed or retired with its surface; the old gate reads it (P-117)
   ↓
Corpus encryption at rest retired — credential-store key, cell encryption, passphrase fallback, key lock file, orphan disposition leave; scrubber unchanged; decision dated for review (P-085)
   ↓
Console engine entry point — one program boots ingest, buffer, detectors, corpus with no display, driven by commands; log, identity, panic, heartbeat, process-end records kept (P-086)
   ↓
One place on a node — operator-set, owner-only locations for stores, log and pid, one documented service default, one resolver; other-system roots leave (P-118, P-113)
   ↓
Agent harness drives the console engine — five verbs target it, verdict arms pinned by tests; panic, heartbeat-gap, process-end, budget checks grade its log (P-086)
   ↓
Shared telemetry test data — spans, metric points and log records from one set of factories; event time, service, version and attributes settable (P-089, P-094)
   ↓
Engine end-to-end gate reachable — memory-capped engine in CI, loopback sender, finding read through stdio sidecar, log kept; second host reaches stub; recorded green (P-086, P-090)
   ↓
Detection baseline through the console engine — five cue families, one incident per storm and auto-resolve driven headless, recorded before window and model leave (P-086)

### Epoch 2 — What leaves: the window, its bridge, its distribution and the other platforms
Window's gates retired — a11y and boot CI jobs, webview suite, headful drive, self-verify, staged, bindings, hue, discovery, frame, bundle checks leave (P-083)
   ↓
Window retired — Tauri shell, webview interface, IPC routers, tray, updater, notification plugins leave; design, layout, a11y masters state no interface; critical path P5 retired (P-083)
   ↓
Desktop distribution retired — bundle workflow, channel publishing, release and updater-key runbooks, channel-manifest test, npm tree, its bot watch and supply-chain gate leave (P-114)
   ↓
Bridge leaves the engine's crates — bridge crate, its feature and type derives leave ingest, triage, viz; configuration becomes the engine's own, no interface key (P-108)
   ↓
Display-only computation retired — stream topics, per-batch encode, effective-tier state, constellation metrics, discovery observer leave; registry liveness, receiver-entry trace context stay; critical path P6 retired (P-109)
   ↓
Plugin host retired — plugin crate, WebAssembly runtime dependency, example plugins, plugin directory and its CI backend assertion leave; critical path P4 recorded retired (P-104)
   ↓
Training export retired — export module and its sink outside the stores leave; nothing the engine writes lands outside its own stores (P-106)
   ↓
Other operating systems retired from the code — PowerShell scripts and spawns, Windows path handling, link hints, linker override, library-test workaround leave (P-113)

### Epoch 3 — What leaves: the local model, what fed and paced it, and the records of the old product
Incident is the engine's own record — formed from its cue: check, service, since when, errors, evidence ids; plain report; 0.3.0 stores not read (P-112, P-099)
   ↓
Local model retired — interpretation crate, inference runtimes, model router, hardware profile, grammar, stand-in, decision probe, latency grader, model variables, vendored converter leave (P-084)
   ↓
Digest retired — prompt assembly, token budgets, build-time tokenizer fetch, digest queue, damper and archive table leave; similar-incident search re-homed beside incidents (P-110)
   ↓
Cadence coordinator retired — tiers, reflection cadence, hardware-profile gate and their configuration keys leave; nothing decides when a model runs (P-111)
   ↓
Workspace detection retired — detector crate, workspace incident key, filtered queries, published key file leave; incidents keyed by check and service; critical path P7 retired (P-105)
   ↓
Supply-chain gate re-based on the smaller graph — advisory ignores and duplicate carve-outs for departed crates pruned; advisory, licence, ban, secret-scan gates stay (P-083, P-084, P-104)
   ↓
Log allowlist describes this engine — every allowed target one the engine emits; window, model, digest, tray and plugin targets and their pins leave (P-115)
   ↓
Records say what the product is — brief, product description, project record, architecture intent, agent instructions, rules describe a console engine; 0.2–0.3 documents marked history (P-116)
   ↓
Detection parity after the removals — five cue families, one incident per storm, auto-resolve read against the baseline in the engine gate; critical paths restated (P-086)
   ↓
Theme 0 checked by the external harness — detection after the removals, read through the stdio sidecar on the engine's host; reading recorded (P-102)

### Epoch 4 — The engine stands on its own node
Security posture restated for a networked engine — tier, auth model, attack surface re-read against a token-gated network receiver and stores on disk (P-116, P-087, P-088)
   ↓
Token lifecycle by engine command — made, shown once, replaced, revoked; not recoverable from the node's stores; never written to a log or a report (P-088)
   ↓
Network OTLP receiver behind the token — another host sends over an encrypted channel; termination and key custody stated; replaces the loopback-only boundary (P-087, P-088, P-116)
   ↓
Receiver refusals — wrong or missing token refused before body read; plaintext from another host refused; refusals logged as bounded counts, never the token's value (P-088)
   ↓
Per-sender bounds — sender over its rate slowed or refused, starving no other; size and count invariants hold remotely; token never keys an incident (P-088, P-105)
   ↓
Checks reason by event time — windows, restart detection and retention read the record's own stamp; lateness allowance stated; late batches counted where they belong (P-089)
   ↓
Recorded stream replays to the same result — a captured synthetic stream played again gives identical findings (P-089)
   ↓
Engine memory measured — process memory under a long run inside 1 GB, the figure recorded; replaces the buffer's row-count estimate (P-090)
   ↓
Engine delivered to a node — one auditable Linux binary and a documented service; on a clean node it runs, returns after restart, receives telemetry (P-107)
   ↓
Theme 1 checked by the external harness — another host sends; findings read through the stdio sidecar on the engine's host; reading recorded, path pinned (P-102)

### Epoch 5 — Memory on disk and a working door
Telemetry store on disk — replaces the in-memory buffer, tick, stall signal kept; 7-day default term, configurable; oldest first at a size ceiling; survives restart (P-091)
   ↓
Learned state survives a restart — the baseline is kept; a gap while the engine was down is not read as the service's silence (P-091)
   ↓
Load profiles re-based on the engine — four profiles through the network receiver into the disk store; high profile's assertion restated; their gating run named (P-090)
   ↓
Disk store measured under load — week-on-disk reading sets the ceiling figure; file bounded under constant insert and delete; memory within the node's gigabyte (P-090, P-091)
   ↓
Door inside the engine's process — agent tools read the engine's stores; admitted parties only, encrypted in transit; replaces the stdio sidecar (P-092, P-093)
   ↓
Door admission lifecycle and bounds — party admitted and revoked by command; store-only reads; answers bounded in size and time; resolve write named, switchable (P-093)
   ↓
Door telemetry queries — spans, metrics and logs filtered by service, time range, trace and text, with aggregates (P-092)
   ↓
One incident whole through the door — the incident with its evidence, span and time references populated, and the incidents like it from the past (P-092)
   ↓
Curated snapshot through the door — a narrow, budgeted view of recent telemetry made for a large model; the snapshot code's one consumer (P-092)
   ↓
Theme 2 checked by the external harness — restart survival and the door's answers, input to output; reading recorded, path pinned in the engine gate (P-102)

### Epoch 6 — Keep what OpenTelemetry sends
Span keeps its identity — name, kind and parent stored; the service's version and environment kept; each scrubbed before storage (P-094)
   ↓
Span and resource attributes kept — route, status code, database and peer under both semantic-convention generations, scrubbed before storage (P-094)
   ↓
Who calls whom — the caller-and-callee question answered from stored spans; the caller–callee and critical-path aggregations read real parents (P-094, P-119)
   ↓
Histogram metrics keep their shape — histogram-family points stored so a percentile can be computed (P-095)
   ↓
Logs and metrics name their service — service identity on every row; exception type, message and stack readable through the door (P-095)
   ↓
Theme 3 checked by the external harness — what was sent is what the door returns; reading recorded, path pinned in the engine gate (P-102)

### Epoch 7 — Comparisons that work from the first minute
New error kind is a finding — first appearance per service in all three encodings, and a gone one's return; high-severity logs feed it (P-096, P-119)
   ↓
Release noticed — a new version of a service recorded with the moment it appeared (P-097)
   ↓
Release compared with the one before — error rate, new error kinds, latency, per operation where known; says when the new version is worse (P-097, P-119)
   ↓
Silence told apart — service went quiet, no data reached the engine, check not evaluable: three states; recovery announced (P-098)
   ↓
Every aggregation has a consumer — each of the seven feeds a named check, report or door answer, or is gone; none runs for nothing (P-119)
   ↓
Theme 4 checked by the external harness — a worse release and a quiet service each told correctly; reading recorded, path pinned in engine gate (P-102)

### Epoch 8 — The voice
One report form without a model — what, where, since when, after which version, which errors are new; incident record carries all of it (P-099, P-112)
   ↓
State in one line for a desktop panel — an engine command prints the current state; a panel module on the founder's desktop shows it (P-099)
   ↓
System notification on a change of state — carries the short report in a stated terse form; each raise leaves a record a check reads (P-099)
   ↓
Report names its evidence — evidence cited by identifier, the report narrow; "not enough data" an answer the engine gives (P-100)
   ↓
Large model names the planted cause — given the report and the door, measured over planted faults; a reading, never a gate (P-100)
   ↓
Theme 5 checked by the external harness — a planted fault reaches panel, notification and report; reading recorded, path pinned in the engine gate (P-102)

### Epoch 9 — Polish & ship
Real service watched for days — a founder's service, named first with its personal data; reports, silences, the engine's own log read against what happened (P-101)
   ↓
Bad-version scenario end to end — Conductor over network to an engine on a 1 GB node; panel, notification, cause named; silent through healthy hours (P-102)
   ↓
No survivor — the audit question asked again of the tree: nothing stands for a removed thing, nothing runs with no consumer (P-121)
   ↓
Version close on Linux — full gate green; every capability verified or deferred on a measured basis; capability record and external harness's accepted set agree (P-117)
