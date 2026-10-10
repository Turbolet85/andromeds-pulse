# Working Route — andromeda-pulse-0.4.0

_Ordered WHAT-not-HOW chunk list for this version. Reorder = move up/down._
_No numbers, no per-chunk IDs/metadata. At promotion /andromeda-phase prefixes the chunk's line with_
_`[{marker}]` to freeze it (wrap's route-resolve edits only the markerless tail; once the chunk's master_
_record is complete, wrap P7 flip-compacts its line to `[{marker}] {title} — {scope hint}`, archiving the_
_verbatim line to route-archive.md); markerless lines stay mutable._
_Chunks separated by `   ↓` within an epoch; only `### Epoch K — {name}` headers are structural._

### Epoch 1 — Foundation: base CI, the capability record, a console engine with its door, its gates
[2026-10-09-ci-on-linux-alone] CI on Linux alone — Windows and macOS legs leave three jobs, the release-build job whole; one run's wall-clock recorded (P-113)
   ↓
[2026-10-09-supply-chain-job-same-on-push-and-pull-request] Supply-chain job same on push and pull request — fails on a finding, never on its own reporting; repair witnessed on a push (P-120)
   ↓
[2026-10-09-boot-smoke-s-early-exit-found-and-closed] Boot smoke's early exit found and closed — the cause of the app ending itself after ready is named; equal source reads the same (P-129)
   ↓
[2026-10-09-pre-push-check-native-on-linux] Pre-push check native on Linux — surviving stages run on the dev host; the second-system hop and its distro clone leave (P-103, P-113)
   ↓
[2026-10-10-boot-smoke-s-self-end-named-from-a-run] Boot smoke's self-end named from a run — the job keeps what names who ended the app; cause closed; equal source reads the same (P-129)
   ↓
[2026-10-10-boot-smoke-s-self-end-closed] Boot smoke's self-end closed — every self-end counted with its label; the cause behind the ending call closed; equal source reads the same (P-129)
   ↓
[2026-10-10-no-ci-step-reads-nothing] No CI step reads nothing — every comparison has a producer for its baseline or is gone; every upload finds its file or is gone (P-128)
   ↓
[2026-10-10-no-gate-stands-while-reading-nothing] No gate stands while reading nothing — coverage branch and empty-report arms, empty test selections, boot budget and heartbeat reads, zero-span check: each reads something or leaves (P-128)
   ↓
Capability record re-based — one current record for all 82 ids, each claimed or retired with its surface; the old gate reads it (P-117)
   ↓
Console engine entry point — one program boots ingest, buffer, detectors, corpus with no display, driven by commands; log, identity, panic, heartbeat, process-end records kept (P-086)
   ↓
Agent harness drives the console engine — five verbs target it, verdict arms pinned by tests; panic, heartbeat-gap, process-end, budget checks grade its log (P-086) · CARRY: two readings obs-plan §10 CI gates states as unmet belong to this entry (the operator's word, the pc overseer, 2026-10-10, at 2026-10-10-no-gate-stands-while-reading-nothing's route-resolve card): the heartbeat-gap check is made by no CI step since that chunk (cargo xtask ci-gates left it; xtask/ci/heartbeat-gap-check.sh's one caller is perf:load-profiles, which no workflow runs), and "a test run that produces zero spans fails the build" was never made by any step (cargo xtask test reads no log; measured at that chunk); this entry's plan rules for each whether the engine version of the check is built or the sentence retired, and nothing is retired before then · CARRY: the sixth stage of cargo xtask pre-push:linux runs ci-gates over one seed record the stage wrote itself and reads two lines over it since that chunk (the record count and the panic read; measured there, exit 0); it is not a step of a CI job, so P-128's sentence does not reach it; the seed's doc comment in xtask/src/pre_push.rs still says it drives the heartbeat arm and the grader's empty-arm reading, which the verb no longer runs · CARRY: cargo xtask check:ingest-progress, a standard gate entry and not a CI step, prints "NEUTRAL — no buffer.tick events in the log family" and exits 0 over a log family with no tick (seen at that chunk's gate block on the dev host, not measured further); the progress read over the engine's log is this entry's (the operator's word, as above)
   ↓
Shared telemetry test data — spans, metric points and log records from one set of factories; event time, service, version and attributes settable (P-089, P-094)
   ↓
Door inside the engine's process — tools read the engine's stores, owner only, own host, call records allowlisted; stdio sidecar, its double gate leave (P-092, P-093) · CARRY: this entry's owner-only is P-126's — until another host can reach the door, another account on the engine's host is refused
   ↓
Scenario legs driven against the console engine — gap-and-resume and external-resolve keep their verdicts, the external write going through the door (P-086, P-092)
   ↓
One place on a node — operator-set, owner-only locations for stores, log, pid; one documented service default, one resolver; other-system roots, shared-temp fallback leave (P-118, P-113)
   ↓
Corpus encryption at rest retired — credential-store key, cell encryption, passphrase fallback, key lock file, orphan disposition leave; scrubber unchanged; decision dated for review (P-085) · CARRY: the credential-store legs this entry removes are the ones cargo xtask pre-push:linux's test stage is stated to clean-skip, its constructed environment carrying no session bus (stated, not measured, at 2026-10-09-pre-push-check-native-on-linux); test-plan §4's corpus bullet and the pre-push:linux paragraph of §3 → Per-chunk gate discipline both say so and are corrected with the legs
   ↓
Engine end-to-end gate reachable — memory-capped engine in CI, loopback sender, finding read through door, log graded, kept; second host reaches stub; recorded green (P-086, P-090) · CARRY: obs-plan §11 bans losing the log file and the snapshot on a test failure ("NEVER lose telemetry artifacts (log file + snapshot on failure)", "NEVER skip snapshot generation on test failure"), and both stand unmet (measured at 2026-10-10-no-ci-step-reads-nothing: no step of ci.yml writes or uploads a snapshot when a test fails, and a failed test in the lint-test job uploads no app log); this entry, which keeps the engine's log in CI, owns both bans from now (the operator's word, the pc overseer, 2026-10-10, at that chunk's route-resolve card): each is met here, or retired here on the operator's word
   ↓
Detection baseline through the console engine — five cue families, one incident per storm, auto-resolve read through the door, recorded before window and model leave (P-086)

### Epoch 2 — What leaves: the window, its bridge, its distribution and the other platforms
Window's gates retired — a11y and boot CI jobs, webview suite, headful drive, self-verify, staged, bindings, hue, discovery, frame, bundle checks leave (P-083) · CARRY: the boot CI job this entry removes was the subject of P-129, closed at 2026-10-10-boot-smoke-s-self-end-closed (the app calls XInitThreads as the second statement of main; ci#38026637514, attempts 1 to 3, 24 boots, 0 self-ended, measured at that chunk's evidence/operator-pass.md); until the job leaves here its Boot series (equal source) step stays a gating step, and a self-end in it is a new reading, brought to the operator, never an expected red (the operator's word, the pc overseer, 2026-10-10, that chunk's inputs#I5 item 2) · CARRY: cargo xtask pre-push:linux keeps a script-modes stage whose only subject is CI's by-path run of scripts/agent-run.sh, and the only workflow lines that run it by path are the boot job's (measured at 2026-10-09-pre-push-check-native-on-linux's scope: 3 lines in ci.yml); when the boot job leaves here the stage has nothing left to guard · CARRY: built for the boot job at 2026-10-10-boot-smoke-s-self-end-named-from-a-run and leaving with it here, unless this entry's card names a later gate that takes one over: scripts/exit-witness.c; xtask/src/harness_witness.rs and the exit_witness member of harness:settled; xtask/src/harness_series.rs and the harness:boot-series verb; the boot verb's exit-witness arm in scripts/agent-run.sh (ANDROMEDA_PULSE_EXIT_WITNESS_LIB, ANDROMEDA_PULSE_EXIT_WITNESS_FILE, LD_PRELOAD on the spawn line); the two boot-job steps in ci.yml (Build the exit witness, Boot series (equal source)) with their three pins in pulse-app/tests/quality_gate_workflow.rs; the masters' rows for each (the operator's word, the pc overseer, 2026-10-10, that chunk's inputs#I4 item 5) · CARRY: the series' first boot builds the app again on the runner (boot 2 took 3 min 27 s and 3 min 36 s to reach ready on ci#38019133294 and ci#38022477393, and reached ready 3 min 3 s to 3 min 34 s after the smoke's ready line in each attempt of ci#38026637514; later boots about 8 s apart; measured at 2026-10-10-boot-smoke-s-self-end-closed's research.md and evidence/operator-pass.md); cause not measured (no series boot keeps its build.log; on the dev host a shell-launched build after four series compiled nothing); nothing is owed as work, the cost ends when the boot job leaves here, and it rides along if this entry's card keeps the series for a later gate (the operator's word, the pc overseer, 2026-10-10, inputs#I5 item 4) · CARRY: the a11y job's a11y-violations upload has three path members and its if-no-files-found: error fails the step only when none matches; it found 5 files on ci#38042949735 as on the two runs before it, which member each file matched was not read, and one member missing beside a matching one was never read on a runner (record: 2026-10-10-no-gate-stands-while-reading-nothing's evidence/gate-census.md); not repaired, since the job leaves here (the operator's answer at that chunk's plan dialog) · CARRY: the lint-test budget step prints "frame: cannot-evaluate: 0 samples, no adapter record in this log" beside its PASS, the frame arm not required there (measured on ci#38042949735; its two required arms read n=3 and n=50); the frame check and its arm leave with this entry (the operator's word, the pc overseer, 2026-10-10, at that chunk's route-resolve card)
   ↓
Window retired — Tauri shell, webview interface, IPC routers, tray, updater, notification plugins leave; design, layout, a11y masters state no interface; critical path P5 retired (P-083) · CARRY: no workflow runs an ESLint step (measured 2026-10-09; record: 2026-10-09-ci-on-linux-alone's report, Spec claims disproved 3), while a11y-plan §9 CI Integration lists a Lint stage as a pull-request check; the stage's subject leaves here, and that row is corrected when the master states no interface · CARRY: two stages of cargo xtask pre-push:linux exist for what this entry removes (record: 2026-10-09-pre-push-check-native-on-linux's report): its npm stage installs and builds the webview interface, and its test stage runs inside a restore that puts the bridge's generated bindings back as found (reason restore-failed:bindings); its source-lint stage loses pulse-app/ui/src, one of its five roots · CARRY: built at 2026-10-10-boot-smoke-s-self-end-closed and leaving with the window here: pulse-app/src/xlib_threads.rs, its call as the second statement of main (pulse-app/src/main.rs), its declaration in pulse-app/src/lib.rs, and pulse-app/tests/unit_xlib_threads.rs; with them architecture §Stack and Technologies' row for the call and the workspace tests' need for the host's libX11.so.6; that test also holds the only committed pin that the render-posture step is main's first statement (test-plan §1 render-posture-main-placement-coverage, its narrowed clause), so that half is unpinned again when the test leaves (the operator's word, the pc overseer, 2026-10-10, inputs#I5 item 3)
   ↓
Desktop distribution retired — bundle workflow, channel publishing, runbooks, manifest test, npm tree, its watch, gate leave; release environment, secret names listed for the founder (P-114) · CARRY: the card names the release environment and every secret name the two workflows read (P-127; measured 2026-10-09 at b3ac58a: production-release, 14 names in release.yml, two push tokens in update-channels.yml); nothing outside the tree is deleted · CARRY: the npm tree this entry removes is what cargo xtask pre-push:linux's npm stage installs, and what the check's Node pin (ci.yml's node-version major, a required piece of its provisioning check) and the PUPPETEER_CACHE_DIR it sets for that stage exist for (record: 2026-10-09-pre-push-check-native-on-linux's report; architecture §Occupied Resources → Environment variables holds the row)
   ↓
Bridge leaves the engine's crates — bridge crate, its feature and type derives leave ingest, triage, viz; configuration becomes the engine's own, no interface key (P-108)
   ↓
Display-only computation retired — stream topics, per-batch encode, effective-tier state, constellation metrics, discovery observer leave; registry liveness, receiver-entry trace context stay; critical path P6 retired (P-109)
   ↓
Plugin host retired — plugin crate, WebAssembly runtime dependency, example plugins, plugin directory and its CI backend assertion leave; critical path P4 recorded retired (P-104)
   ↓
Training export retired — export module and its sink outside the stores leave; the model's per-spawn temp file is the one write left outside them (P-106)
   ↓
Other operating systems retired from the code — PowerShell scripts and spawns, Windows path handling, link hints, linker override, library-test workaround leave (P-113) · CARRY: shipped unwitnessed at 2026-10-09-boot-smoke-s-early-exit-found-and-closed and leaving here: scripts/agent-run.ps1's Invoke-Ready readiness poll and its receivers line (parsed by pwsh with 0 parse errors, never run), and the non-Unix branch of the socket probe in xtask/src/harness_ready.rs (not compiled on the dev host) · CARRY: xtask/ci/quarantine-tracking-check.ps1 gained the absent-input arms of its .sh twin at 2026-10-10-no-gate-stands-while-reading-nothing (a missing search dir and a scan of zero .rs files exit 1; the PASS lines count files) and was parsed by pwsh with 0 parse errors, never run; no pin reads it (test-plan §1, the row quarantine-tracking-ps1-mirror-coverage); it leaves here

### Epoch 3 — What leaves: the local model, what fed and paced it, and the records of the old product
Incident is the engine's own record — formed from its cue: check, service, since when, errors, evidence ids; plain report; 0.3.0 stores not read (P-112, P-099)
   ↓
Local model retired — interpretation crate, inference runtimes, model router, hardware profile, grammar, stand-in, decision probe, latency grader, model variables, vendored converter, experiments directory leave (P-084)
   ↓
Digest retired — prompt assembly, token budgets, build-time tokenizer fetch, digest queue, damper and archive table leave; similar-incident search re-homed beside incidents (P-110)
   ↓
Cadence coordinator retired — tiers, reflection cadence, hardware-profile gate, their configuration keys, the emitter's trigger handle leave; nothing decides when a model runs (P-111)
   ↓
Workspace detection retired — detector crate, workspace incident key, filtered queries leave; incidents told apart by check and service; critical path P7 retired (P-105)
   ↓
Supply-chain gate re-based on the smaller graph — advisory ignores and duplicate carve-outs for departed crates pruned; advisory, licence, ban, secret-scan gates stay (P-083, P-084, P-104)
   ↓
Log allowlist describes this engine — every allowed target emitted, every emitted field allowed; window, model, digest, tray and plugin targets and their pins leave (P-115)
   ↓
Records say what the product is — brief, product description, project record, architecture intent, agent instructions, rules describe a console engine; 0.2–0.3 documents marked history (P-116)
   ↓
Detection parity after the removals — five cue families, one incident per storm, auto-resolve read against the baseline in the engine gate; critical paths restated (P-086)
   ↓
Theme 0 checked by the external harness — detection after the removals, read through the door; reading recorded (P-102)

### Epoch 4 — The engine is reached over a network
Security posture restated for a networked engine — tier, auth model, attack surface for token-gated receiver, admitted-party door, disk stores; later entries built to it (P-116)
   ↓
Token lifecycle by engine command — made, shown once, replaced, revoked; not recoverable from stores; never in a log or a report; material owner-only (P-088, P-118)
   ↓
Network OTLP receiver behind the token — another host sends over an encrypted channel; termination, key custody, renewal stated; replaces the loopback-only boundary (P-087, P-088, P-116) · CARRY: the renewal this entry states is P-124's — who renews the channel's private key and by what steps; receiving continues across a renewal
   ↓
Receiver refusals — wrong or missing token refused before body read; plaintext from another host refused; refusals logged as bounded counts, never the token's value (P-088)
   ↓
Per-sender bounds — sender over its rate slowed or refused, starving no other; size and count invariants hold remotely; token never keys an incident (P-088, P-105)
   ↓
Door admission lifecycle and bounds — party admitted and revoked by command; store-only reads; answers bounded in size and time; resolve write named, switchable (P-093)
   ↓
Door reachable from another host — admitted parties only, encrypted in transit; where a party's credential lives on the developer's machine stated (P-093, P-092)

### Epoch 5 — The engine lives on a small node
Checks reason by event time — windows, restart detection and retention read the record's own stamp; lateness allowance stated; late batches counted where they belong (P-089)
   ↓
Future-stamped records bounded — a stamp beyond a stated allowance cannot move what checks take for the present; what happens to such a record stated (P-089)
   ↓
Recorded stream replays to the same result — a captured synthetic stream played again gives identical findings (P-089)
   ↓
Engine memory measured — process memory under a long run inside 1 GB, the figure recorded; replaces the buffer's row-count estimate in the budget gate (P-090)
   ↓
Engine's own log bounded — total size capped, so neither the log nor the database file can fill the node (P-090)
   ↓
Engine delivered to a node — one auditable Linux binary, checksum verified by the documented steps; clean-node service runs, returns after restart, receives telemetry (P-107)
   ↓
Theme 1 checked by the external harness — another host sends and reads findings through the door; reading recorded, path pinned in the engine gate (P-102)

### Epoch 6 — Memory on disk and the door's full answers
Telemetry store on disk — replaces the in-memory buffer, tick, stall signal kept; 7-day default term, configurable; oldest first at a size ceiling; survives restart (P-091) · CARRY: a test comment in crates/buffer/src/schema.rs, above the metrics_points key test, still says the duplicate-INSERT probe hangs on this build, which was disproved 2026-08-28 (a violating flush returns an error at duckdb 1.10505); test-plan §4's buffer bullet states both comments as read 2026-10-09; the comment is corrected when this entry reworks the store's schema tests
   ↓
Disk store opened across builds — a telemetry store written by another build is recognised, carried forward or refused by a stated rule, never misread (P-091) · CARRY: this entry's rule is P-125's — P-091's text has none
   ↓
Learned state survives a restart — the baseline is kept; a gap while the engine was down is not read as the service's silence (P-091)
   ↓
Load profiles re-based on the engine — four profiles through the network receiver into the disk store; high profile's assertion restated; their gating run named (P-090) · CARRY: xtask/ci/heartbeat-gap-check.sh keeps two "trivially passes" arms reachable only through cargo xtask perf:load-profiles, which no workflow runs, and it finds a tick by a compact-JSON pattern, so a record spelled with a space after the colon is not read as a tick (measured at 2026-10-10-no-gate-stands-while-reading-nothing's research: two such ticks 50 s apart read no tick, exit 0; the app writes compact JSON) · CARRY: the perf:slo-load verb's about string in xtask/src/main.rs still claims post-test frame and memory gates it does not run (the test asserts its own throughput bound and emits no perf sample; seen at that chunk); it rides this entry, which names the gating run (the operator's word, the pc overseer, 2026-10-10, at that chunk's route-resolve card)
   ↓
Disk store measured under load — week-on-disk reading sets the ceiling figure; file bounded under constant insert and delete; memory within the node's gigabyte (P-090, P-091)
   ↓
Door telemetry queries — spans, metrics and logs filtered by service, time range, trace and text, with aggregates (P-092)
   ↓
One incident whole through the door — the incident with its evidence, span and time references populated, and the incidents like it from the past (P-092)
   ↓
Curated snapshot through the door — a narrow, budgeted view of recent telemetry made for a large model; the snapshot code's one consumer (P-092)
   ↓
Engine's own state through the door — current memory use and the size of each store answered read-only, bounded in size and time (P-122)
   ↓
Theme 2 checked by the external harness — restart survival and the door's answers, input to output; reading recorded, path pinned in the engine gate (P-102) · CARRY: the restart between the two readings is done on the engine's host by whoever runs the check; the harness starts and stops no process of the engine's (Conductor's conductor-0.4.0/requirements.md:59 says the same)

### Epoch 7 — Keep what OpenTelemetry sends
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

### Epoch 8 — Comparisons that work from the first minute
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

### Epoch 9 — The voice
One report form without a model — what, where, since when, after which version, which errors are new; incident record carries all of it (P-099, P-112)
   ↓
State in one line for a desktop panel — an engine command prints the state through the door; the founder's desktop panel shows it (P-099, P-092)
   ↓
System notification on a change of state — carries the short report in a stated terse form; each raise leaves a record a check reads (P-099) · CARRY: the record each raise leaves is read through the door (P-123); this entry's check reads it there
   ↓
Report names its evidence — evidence cited by identifier, the report narrow; "not enough data" an answer the engine gives (P-100)
   ↓
Large model names the planted cause — given the report and the door, measured over planted faults; a reading, never a gate (P-100)
   ↓
Theme 5 checked by the external harness — a planted fault reaches panel, notification and report; reading recorded, path pinned in the engine gate (P-102)

### Epoch 10 — Polish & ship
Named service's personal data settled — each kind the founder names scrubbed before storage, or its keeping decided and recorded, before the first send (P-101)
   ↓
Real service watched for days — the founder's named service; reports, silences, the engine's own log read against what happened (P-101)
   ↓
Bad-version scenario end to end — Conductor over network to an engine on a 1 GB node; panel, notification, cause named; silent through healthy hours (P-102)
   ↓
No survivor — the audit question asked again of the tree: nothing stands for a removed thing, nothing runs with no consumer (P-121)
   ↓
Version close on Linux — full gate green; every capability verified or deferred on a measured basis; capability record and external harness's accepted set agree (P-117)
