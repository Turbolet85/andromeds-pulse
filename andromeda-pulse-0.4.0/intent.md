# v0.4.0 Intent — andromeda-pulse

**Purpose & how to use this file.** This is the INTENT for version 0.4.0 — the single source of truth that
`/andromeda-route --version 0.4.0` derives `vision.md`, `requirements.md` and `working-route.md` from. Every
finding states what we OBSERVED and what we EXPECT. It says WHAT, never HOW: a transport, a file format or a
library named here is an example unless the line says "ruled".

_Third assembly, 2026-10-09, by the pc overseer. The first (committed at `39edd11`, adapted at `7f99c38`) gave a route
of 52 entries and 25 requirements; an audit of that route against the tree — "what of the old form would still stand
at the end, with no entry that removes it, replaces it or gives it a consumer?" — found fifteen such clusters, and the
founder's word the same day was «без полумер … если что то не вписывается в нашу новую парадигму исправляем а не
стараемся сгладить углы просто». The second assembly named every survivor and gave it an owner — the sixteen findings
of Theme 0; its route reached Phase 4 (run `2026-10-09T10-26-56-route`, 39 requirements, nothing committed). That
review showed three things the second assembly lacked. This one adds them and changes nothing else: (1) the agent's
door stands inside the engine from the engine's first step, as ruled on 2026-10-09 — the second route put it in its
fifth epoch, and five chunks before it would each have had to keep the old sidecar working (F6, F6b, §6); (2) six controls of the new
surfaces that no finding stated (F3, F4, F4b, F6b, F14, F15) and the security master's posture (R10); (3) the OBSERVED
figures which that run re-read and I read again (R1, R2, R2c, R7, R11, F4, F4b, F6). His word at that review, asked
whether to derive again: «если считаешь нужным переделывай, если нет то оставляем, лимит и время не проблема, главное
правильность». Sources: his rulings of 2026-10-08/09 (trail: `~/dev/projects/additional/pc-overseer/shaping-0.4.0.md`),
the limits recorded at the 0.3.0 close, the study in the incubator folder, and readings of the source at `18a872d`
and `7f99c38`. He reviews the derived requirements and route at the route's Phase 4._

**Ids.** The first derivation minted P-083…P-107; nothing was built on them. Each finding below that existed then
names the id it KEEPS, so that records which already cite those ids stay true; a finding marked `new` takes the next
free id from P-108. This assembly adds no finding and moves none, so the fourteen marked `new` take P-108…P-121 in
this file's order, as they did in the second derivation.

---

## 1. Context

Pulse 0.3.0 closed on 2026-10-09 at `0e45d58` (22 of 22 capabilities verified, the route empty) and was merged
into `main` (`60ef43c`). It is a Tauri desktop application: an OTLP receiver, an in-memory buffer, detectors, a
local model that writes an incident's interpretation, a window with a widget and a dashboard, and an MCP sidecar.

On 2026-10-08/09 the founder restated what Pulse is for and changed its form. His words:

> «основной тезис которым мы изначально характеризовывали пульс был тихий помощник который избавляет вайбкодера от
> настройки какой то коммерческой большой системы и оверхеда постоянно следить за дашбордом и пытаться понять, все ли
> у нас нормально»

> «наша задача исключить из этого уравнения специалиста который следит за этими дашбордами и соответственно за
> ненадобностью и самих дашбордов, такой себе хедлесс датадог»

In short: a quiet helper for one developer with one product — the measuring instruments of the mainstream
observability systems, with the specialist and the dashboards removed. It notices by itself, says in plain words
what broke, and hands a report to a large model that investigates.

## 2. The core problem (one sentence)

> Pulse measures a small part of what it is sent, forgets it within ten minutes, and can say what it found only
> through a window on the developer's own machine — so it can neither stand watch next to a deployed product nor
> give an investigating agent anything to dig in.

## 3. What WORKS today — preserve, build on (do NOT rebuild)

- OTLP ingest of traces, metrics and logs (gRPC and HTTP), its invariants and rate limits, the secret scrubber.
- The DuckDB buffer with its tables and its retention sweep, and concurrent reads beside the writer inside one process.
- Detection: the learned baseline and the cue emitter that reads it (error-rate spike, latency regression, retry
  storm, restart, silence), the latch, incident de-duplication, auto-resolve. measured: detection does not depend on
  the cadence coordinator (`crates/triage/src/cue/emitter.rs:399`).
- The exception fingerprint and the log-template mining.
- The incident corpus and the search for similar past incidents ("previously seen") — an engine fact, kept; today it
  is housed inside the digest module.
- The curated snapshot (markdown, token-budgeted) and the curation it stands on — kept, and given a consumer (F6).
- The telemetry queries the MCP tools use (`crates/viz/src/query.rs`) and configuration hot reload.

## 4. The rule of this version

Three things were ruled out of the product — the window, the local model inside the engine, the desktop it lived on —
and three more on 2026-10-09 (the plugin host, workspace detection, the training export). **A removal is complete
only when what existed solely to serve the removed thing is gone too, or has a new consumer named here.** Each
removal below is a finding of its own, so that it gets a capability id and a line in the verification matrix whose
acceptance states an outcome; a removal that is only prose is never checked.

## 5. Findings — OBSERVED vs EXPECT

Coordinates in OBSERVED were read from source at `18a872d` or `7f99c38`; sizes are line counts of the files named.

### Theme 0 — What leaves

**R1 — the window is gone.** (ruled · keeps P-083)
OBSERVED: a Tauri shell and a webview interface (`pulse-app/ui`, 243 tracked files), ten IPC router files
(`pulse-app/src/*_router.rs`), the tray, the window and its geometry, the updater and the notification plugin; the gates that exist for
it (`a11y` and `boot` CI jobs, the webview suite, and in `xtask` the webview drive, staged gate, smoke, npm gate, hue
shift, discovery, self-verify, frame sampling and bundle format); three masters whose subject it is (design-system,
layout-templates, a11y-plan). Most of 0.3.0's 22 verified capabilities are about it; the route names which, id by id.
EXPECT: no crate depends on the window toolkit; the tree holds no webview interface, no IPC router and no gate that
boots a window, audits one or samples its frames; the tests that drive them leave with them and the remaining gate is
green; the masters say in plain words that this version has no interface and which later version brings a minimal one
on another toolkit. The 0.3.0 capabilities that were about the window are recorded as retired with the surface, not
as regressed.

**R1b — the bridge leaves the engine's crates.** (new)
OBSERVED: `tauri` itself is declared only by `pulse-app` and optionally by `crates/ui-bridge` (5 files, about 4800
lines), but the bridge's types reach into the engine: the ingest and triage crates carry a default-on `taurpc-runtime`
feature with `specta` derives, `crates/viz` depends on `specta` unconditionally, and the engine's own configuration
schema (`Settings`) lives in the bridge crate, read by `crates/config-watcher`, with keys for a theme, a widget
position, always-on-top and a snapshot preset. R1's first clause would be met without touching any of this.
EXPECT: no engine crate carries a feature, a derive or a dependency that exists for an IPC bridge; the engine's
configuration is the engine's own and holds no key about an interface; the bridge crate is gone.

**R1c — what was computed only to be shown is gone.** (new)
OBSERVED: the buffer encodes every batch to Arrow IPC and broadcasts it on thirteen `pulse://stream/*` topics whose
only production subscriber is the window's channel (`pulse-app/src/streams.rs:36-48`); triage computes state for
display alone — the effective tier for the halo (`crates/triage/src/incident/tier_effective.rs`), the constellation
metrics, the discovery observer.
EXPECT: the engine does no work whose only reader was the window: no stream topic, no per-batch encode for a
subscriber that does not exist, no display state. The service registry's liveness truth stays engine state.

**R2 — the local model is gone from the engine, with everything that served it.** (ruled: «Уходит из движка» · keeps
P-084, widened)
OBSERVED: `crates/interpretation` (10 tracked files, about 3700 lines: runner trait, prompt builders, the `L4Output` schema,
degraded mode); in `pulse-app` the inference runtimes, the model router, the hardware profile, the grammar file; the
decision probe (three files under `pulse-app/examples/`, about 10,500 lines) and the latency grader; six
`ANDROMEDA_PULSE_*` environment variables for the model, one of which `xtask` still sets; `experiments/` (git-ignored, no tracked file: a directory on the dev host) and a
vendored `llama-cpp`.
EXPECT: the engine starts, detects and reports with no model present and none configured; no gate needs a GPU; no
crate, example, script, environment variable, vendored runtime or experiment that exists for a local model remains;
their tests leave with them; `experiments/` is gone from the dev host, shown by a listing because no commit can show it. The words of a report come from the engine's own facts (F12). A large model reads
through the door (F6) and is not part of the engine.

**R2b — the digest is gone.** (new)
OBSERVED: `crates/triage/src/digest/` (6 files, about 2800 lines) assembles a token-budgeted text — budgets of 500,
2000 and 3000 tokens, an embedded Llama-3 tokenizer, a last-write-wins queue, a damper against standing GPU load — and
broadcasts it. Its subscribers are the model and a persister that writes every digest to the corpus table
`digest_archive`, which nothing reads. The triage crate downloads that tokenizer from a third-party host at first
build (`crates/triage/build.rs`). The recorded defect "the half-hour digest's rate is divided by a literal 60"
(`.andromeda/residuals.md:27`) lives in this code.
EXPECT: the engine builds with no network access and no tokenizer; it assembles no prompt, counts no tokens and
archives no digest; the `digest_archive` table is gone. The search for similar past incidents is kept and lives where
incidents live. The rate defect is closed by the code's removal, not carried.

**R2c — the cadence coordinator is gone.** (new)
OBSERVED: `crates/triage/src/cadence/` (4 files, about 1600 lines) has four modes — three tiers and a 1800 s
"reflection" — and a gate on a GPU/CPU hardware profile. A cycle runs seven aggregation queries, keeps only whether
each succeeded (`coordinator.rs:190`), and sends a trigger for the digest. None of it serves detection, but the cue
emitter is handed its trigger channel (`crates/triage/src/cue/emitter.rs:399-402`).
EXPECT: nothing in the engine decides when a model should run; no hardware profile, tier or reflection cadence
remains, no configuration key for one, and no handle to one in the emitter. The ladder of timeframes of a later version is built on event time (F3),
not on this.

**R2d — an incident is the engine's own record.** (new)
OBSERVED: an incident's content is a field holding the model's `L4Output` as JSON (`crates/triage/src/contract.rs:462`);
its title and detail are the model's; the report an agent retrieves renders the model's sections — hypotheses,
investigation steps, project context, a resolution summary — and a `degraded_mode` flag that would read true for every
incident once no model runs (`crates/interpretation/src/markdown.rs`, `crates/mcp-server/src/tools.rs:366-392`).
EXPECT: an incident stores what the engine knows — which check found it, in which service, since when, after which
version, which errors, which evidence by identifier — and nothing shaped as a model's answer; one report form exists,
the one F12 describes, and the old renderer is gone. A store written by 0.3.0 is not read: the engine on a node
starts from its own empty stores.

**R3 — the corpus is stored unencrypted, for this first stage.** (ruled: «сразу все шифровать не стоит чтоб не усложнять
отлаживание … а потом будет видно» · keeps P-085)
OBSERVED: the incident corpus is encrypted per cell with a key from the desktop's credential store
(`crates/corpus/src/keychain.rs`, `encryption.rs`; the `keyring` and `aes-gcm` dependencies); a server has no such
store (a CI runner already read `KeyringUnavailable`); the buffer is not encrypted.
EXPECT: the engine keeps its stores readable on its own node with no credential store, and no code or dependency for
one remains; the secret scrubber at ingest stays as it is; the security master records the decision, its date and
that it is to be looked at again.

**R4 — the plugin host is gone.** (ruled 2026-10-09 · keeps P-104)
OBSERVED: `crates/plugins`, the `wasmtime` dependency, `plugins-examples/`, and the critical path that drives the
plugin lifecycle.
EXPECT: the workspace depends on no WebAssembly runtime; no plugin host or example is in the tree; that critical path
is recorded as retired, not regressed.

**R5 — workspace detection is gone.** (ruled 2026-10-09 · keeps P-105)
OBSERVED: `crates/workspace-detector` reads the directory the application was started in; incidents are keyed by the
workspace it detects (the registry's cooldown key, `incidents.workspace` and three `WHERE workspace` queries, a key
file published for the sidecar).
EXPECT: the engine reads no project folder. One engine watches one product: its incidents live in its one store and
are told apart by the check and the service alone; nothing takes the workspace key's place, and in this version a
sender's token is not an incident key — the same storm sent under two tokens forms one incident. Incidents are
formed, kept, listed and resolved the same from wherever the engine is started.

**R6 — the training export is gone.** (ruled 2026-10-09 · keeps P-106)
OBSERVED: `pulse-app/src/training_export.rs` and its route write incidents to the user's Downloads folder for model
training.
EXPECT: no export exists and nothing the engine writes lands outside its own stores.

**R7 — the other operating systems are gone from the code, not only from CI.** (new)
OBSERVED: seven `.ps1` files (754 lines); the `wsl.exe` hop in `xtask/src/pre_push.rs`; five `pwsh`/`powershell`
spawns in `xtask`; Windows path handling (`\\?\`) in `crates/config-watcher`; `rstrtmgr` link hints in three build
scripts; an MSVC linker override in `.cargo/config.toml`; `[lib] test = false` in `pulse-app/Cargo.toml`, a WebView2
workaround; data-directory names for Windows and macOS in five source files; Windows and macOS legs in three
CI jobs.
EXPECT: the tree builds, tests and runs on Linux alone and holds no branch, script, link hint or workaround for
another operating system; CI has no leg for one.

**R8 — the desktop's distribution is gone.** (new; the engine's own delivery is F14)
OBSERVED: `release.yml` builds desktop bundles for three systems; `update-channels.yml` (411 lines) publishes to a
Homebrew tap and a Scoop bucket; two runbooks describe the release environment and the updater's key rotation; a test
pins the channel manifests; the dependency bot watches an npm tree.
EXPECT: no workflow, runbook, test or bot configuration for a desktop bundle, an updater, a package channel or an npm
tree remains. (The two external channel repositories, `turbolet85/homebrew-andromeda-pulse` and
`turbolet85/scoop-andromeda-pulse`, are the founder's to retire by hand; name them on the card.)

**R9 — the log allowlist describes this engine.** (new)
OBSERVED: `pulse-app/src/observability.rs` allows 215 log targets by name; by their names at least 35 are the
window's and 42 the model's or the digest's; eighteen tests pin them.
EXPECT: every allowed target is one the engine can emit; none names a window, a model, a digest, a tray or a plugin.

**R10 — the records say what the product is.** (new)
OBSERVED: `.andromeda/input.md`, `andromeda-pulse-product-description.md`, `.andromeda/project.yaml` and architecture
§Project Intent call the product a cross-platform desktop dashboard — "Not a web app, not a CLI"; `CLAUDE.md` opens
"Cross-platform Tauri 2 desktop dashboard"; three `.claude/rules` files are scoped to the webview; the security rule
an agent always loads says the receivers "MUST bind to 127.0.0.1 only … no auth model", which contradicts F2;
`docs/v0_2_0/` (seven files, about 5200 lines) describes the local-model product as current. The security master
rests its tier, `Minimal (0)`, on "no persistent user data store", "no internet-exposed network surface" and "no user
authentication surface" (`.andromeda/security-plan.md:22-24`), and calls that tier the single source of truth for
how much rigor later work carries (`:109`); F2, F2b and F5 end all three.
EXPECT: every record a newcomer or an agent reads first describes a console engine on its own node, for one developer
with one product; no rule instructs what this version removes or forbids what it adds; what describes 0.2–0.3 is
either gone or marked as the history of those versions. The security master's tier, auth model and attack surface
are restated for a networked engine with stores on disk, no later than the chunk that opens the network receiver,
and what follows is built to the restated tier.

**R11 — the capability record is re-based.** (new)
OBSERVED: 82 capabilities stand as claimed, in two records: P-001…P-060 in `docs/v0_2_0/pulse-capability-spec.md`
with a gate over `capability-verification-matrix.json`, P-061…P-082 in `andromeda-pulse-0.3.0/verification-matrix.json`; many of the first sixty are about the window (the halo, the
constellation, the report's render surface, the findings counter) or the model (the severity tiers, the hypotheses,
the degraded mode, the project context). The external harness pins its accepted set to these 82.
EXPECT: one current record says, id by id for all 82, whether the engine still claims the capability or it was
retired with the window, the model or the desktop; the old gate carries no retired id; that record is what the
external harness syncs its accepted set to.

### Theme 1 — The engine stands on its own node

**F1 — a program without a window.** (keeps P-086)
OBSERVED: one application binary that is a Tauri process; no headless mode.
EXPECT: the engine is a console program that runs in the background on a Linux server and is driven by commands;
nothing in it needs a display, a GPU or a desktop session. Everything 0.3.0 detected is still detected.

**F2 — telemetry arrives over a network.** (ruled: the engine is a SEPARATE node from the watched service — «не все
захотят неизвестную штуку ставить рядом с сервисом» · keeps P-087)
OBSERVED: the receivers bind `127.0.0.1`, the HTTP one admits only localhost `Host` values
(`crates/ingest/src/http.rs:113-117`), and neither asks for a credential.
EXPECT: a service on another host sends OTLP to the engine with a token over an encrypted channel; a request
without the token is refused; connecting a service takes an address and a token and installs nothing beside it.

**F2b — what the network receiver refuses.** (the token and the channel are ruled; the rest is the overseer's,
PROVISIONAL until the founder's own word · keeps P-088)
OBSERVED: the receiver's only guards today are its localhost bind, a `Host` allow-list, size and count invariants and
a global rate limit; none of them assumes a sender it does not trust.
EXPECT: a request with a missing or wrong token is refused before its body is read, and the refusal is recorded
without the token's value; the token is made, shown once, replaced and revoked by an engine command and is never
written to a log or a report; it is stated who terminates the encrypted channel and where its private key lives —
never in a store the door reads, never in a log; a sender that exceeds its rate is slowed or refused without starving
another; the existing size and count invariants apply to a remote sender; a plain unencrypted connection from
another host is refused. Everything the scrubber removes today is still removed before anything is stored.

**F3 — the watcher counts by event time.** (ruled · keeps P-089)
OBSERVED: windows, the restart detector and the retention cutoff read the system clock; the restart cue keys on a gap
in ARRIVAL of more than 20 s.
EXPECT: every check reasons by the time stamped inside the record, with a stated allowance for lateness; a batch
that arrives late is counted where it belongs, and a recorded stream can be played again and give the same result.
A record stamped in the future beyond a stated allowance cannot move what the checks take for the present, and what
is done with such a record is stated.

**F4 — the node is small.** (ruled: 1 GB of memory, about $5 a month · keeps P-090)
OBSERVED: the engine's memory use with a database on disk has never been measured. The engine's log rotates daily
(`pulse-app/src/observability.rs:2645`) and nothing removes an old file.
EXPECT: the engine fits a 1 GB node under a long run; the figure is measured, and the database file does not grow
without bound under constant insert and delete. The engine's own log is bounded in total size, so neither can fill
the node.

**F4b — the engine has one place on a node.** (new; who may read it is the overseer's, PROVISIONAL until the founder's
own word)
OBSERVED: stores, log and pid resolve under the user's home or configuration directory, by a resolver written more
than once (five source files carry its directory names; the sidecar has its own,
`crates/mcp-server/src/bin/andromeda-pulse-mcp.rs:28`). Nothing states who may read them.
EXPECT: the operator says where the engine keeps its stores, its log and its pid, and there is one documented default
for a service; one piece of code resolves it. The stores, the log, the pid and any token material are readable by
the engine's owner alone.

**F14 — the engine reaches a node.** (the overseer's, PROVISIONAL until the founder's own word · keeps P-107)
OBSERVED: the only delivery is the desktop bundle R8 removes, and its signing goes with it.
EXPECT: the engine is delivered as one Linux binary with a documented way to run it as a service; on a clean node the
documented steps give an engine that runs, comes back after a restart and receives telemetry. The binary comes with
a checksum, and the documented steps verify it.

### Theme 2 — Memory on disk and a working door

**F5 — what was received is still there tomorrow.** (keeps P-091; the number is ruled: «7 дней»)
OBSERVED: raw telemetry lives 600 s in an in-memory database; only incidents persist; the learned baseline is
discarded at start when older than an hour.
EXPECT: raw telemetry is kept 7 days by default — the term configurable, a ceiling on the store's size beside it at
which the oldest goes first — and survives a restart; what the watcher has learned survives a restart; a gap while
the engine was down is not read as the service's silence. Whether a week fits the node's disk under a real stream is
measured, and the ceiling's figure is set from that reading.

**F6 — the agent's door opens onto the real data, from the engine's first step.** (ruled 2026-10-09: the test harness
reads the engine through the same door as the large model, so the door exists from the first step · keeps P-092,
widened)
OBSERVED: the MCP sidecar's four telemetry tools read their own empty in-memory database and accept only a time
window and a limit; the curated snapshot and the curation crates (about 3200 lines) have the window's toast and
that empty database as their only outlets. The sidecar is a second process built on what this version removes: it
opens the corpus through the desktop's credential store (`crates/mcp-server/src/bin/andromeda-pulse-mcp.rs:6`), reads
the published workspace key (`:78`), and renders an incident by parsing the model's answer with the model's renderer
(`crates/mcp-server/src/tools.rs:25-28`, `:380`). Until a door exists it is the only way anything outside the process
reads a finding.
EXPECT: the door lives inside the engine's process and replaces the stdio sidecar as soon as the console engine
exists — ahead of the removals the sidecar is built on (the credential store, the resolver, the incident it parses,
the model's renderer, the workspace key), so that those chunks change the door, which stays, and never the sidecar,
which goes. Every reading from outside the process — the engine's own end-to-end gate, the external harness at each
theme — goes through the door. An agent on the developer's machine reaches the engine's tools and reads the telemetry
the engine holds, filtered by service, time range, trace and text, with aggregates; it reaches one incident whole,
with its evidence, and the incidents like it from the past; it can ask for the curated snapshot — a narrow, budgeted
view of recent telemetry made for a large model — which is that code's consumer from now on. The same door serves the
test harness and the status widget (F12).

**F6b — what the door may do.** (the overseer's, PROVISIONAL until the founder's own word · keeps P-093)
OBSERVED: the sidecar's tools are local, unauthenticated, and one of them writes (it marks an incident resolved).
EXPECT: until the engine accepts connections from another host, the door answers on the engine's own host only. From
the moment it can be reached from another host it answers only a party the node's owner has admitted, and admission
has a lifecycle — how a party is admitted and how it is revoked, as the token has; it is stated where an admitted
party's credential lives on the developer's machine — never in a repository, never in a log; the door is encrypted
in transit; it reads the engine's stores —
telemetry, incidents, reports, the engine's own state — and nothing else on the node: not the token, not
configuration secrets, not files outside those stores; every answer is bounded in size and time, so no question can
exhaust the node; what it returns was scrubbed at ingest; the one write it keeps (resolving an incident) is named as
a write and can be switched off.

### Theme 3 — Keep what OpenTelemetry sends

**F7 — a span keeps its meaning.** (keeps P-094)
OBSERVED: a span is stored as seven fields; its name, kind, parent, status message, every attribute and every
resource attribute but `service.name` are dropped at ingest.
EXPECT: the stored span keeps its name, kind and parent, the service's version and environment, and the
attributes that say which route, which status code, which database or peer it touched — under both generations of
the semantic-convention names. Who calls whom can be answered from what is stored.

**F8 — metrics and logs keep theirs.** (keeps P-095)
OBSERVED: histogram-family points are stored as the value 0; a log row and a metric row carry no service
identity; stored exception text has no reader.
EXPECT: a percentile can be computed from stored metrics; a log line and a metric point name their service;
exception type, message and stack are readable through the door.

**F8b — every aggregation has a consumer.** (new)
OBSERVED: seven aggregation queries exist (`crates/triage/src/baseline/sql.rs`: per-service rate/errors/latency,
per-operation, exception fingerprints, caller→callee, cardinality, high-severity logs, critical path). Their only
production callers are the coordinator, which discards the rows, and the digest assembler, which uses two; two of
them reference a parent column the table does not have.
EXPECT: at the end of this version each of the seven either feeds a check, a report or a door answer named in this
intent — per-operation for F10, high-severity logs for F9, caller→callee and critical path for F7 — or is gone. None
runs for nothing.

### Theme 4 — Comparisons that work from the first minute

**F9 — a new kind of error is news.** (keeps P-096)
OBSERVED: the fingerprint feeds the retry-storm detector and the search for similar past incidents; nothing says
"this error was never seen before". An error reaches the engine in three encodings — a span's error status, an
exception on a span, a high-severity log — and only the second is fingerprinted.
EXPECT: the first appearance of an error for a service is a finding, in whichever of the three encodings it arrives,
and so is the return of one that had gone away.

**F10 — a release is compared with the one before.** (keeps P-097)
OBSERVED: the service version is dropped at ingest; nothing knows a release happened.
EXPECT: when a new version of a service appears, the engine compares it with the previous one — error rate, new
error kinds, latency, per operation where the operation is known — and says when the new one is worse, naming the
version and the moment it appeared.

**F11 — silence is told apart from "nothing arrived".** (keeps P-098)
OBSERVED: a vanished stream and an engine that could not receive look alike.
EXPECT: "the service went quiet", "no data reached the engine" and "the check could not be evaluated" are three
different states, and a recovery is announced.

### Theme 5 — The voice

**F12 — the engine says it, in one line and in a report.** (ruled: the first surface is a panel module fed by an
engine command, on the founder's own desktop (Omarchy, Waybar) only; phone channels and a web-hook are 0.5.0 · keeps
P-099)
OBSERVED: findings are visible only inside the window.
EXPECT: a command of the engine prints the current state in one line a desktop panel can show; a change of state
raises a system notification; the notification carries a short report — what, where, since when, after which
version, which errors are new — written without a model. This is the one report form (R2d).

**F13 — the report is enough for a large model.** (keeps P-100)
OBSERVED: no measurement exists of whether a large model can work from a report of the engine's.
EXPECT: given the report and the door, a large model names the cause that was planted. The report is narrow and
names its evidence by identifier; "not enough data" is an honest answer the engine can give.

**F15 — a real service is watched too.** (ruled 2026-10-08; which project is not yet named — ask the founder before
this is taken up · keeps P-101)
OBSERVED: every reading of Pulse so far was taken on telemetry a test tool composed.
EXPECT: one real service of the founder's sends its telemetry to the engine for days; what the engine reported and
what it stayed silent on is read against what actually happened to that service. He names the service together with
the personal data its telemetry carries, and before its first send each kind he names is either scrubbed before
storage or its keeping is decided and recorded (the overseer's, PROVISIONAL until his own word); until then the security master's "Compliance triggers: None" keeps its
present basis.

### Theme 6 — The build and its gates

**F16 — the check before a push runs on Linux.** (ruled carried from 0.3.0 · keeps P-103)
OBSERVED: `cargo xtask pre-push:linux` needs `wsl.exe`; on the Linux dev host its stages are run by hand.
EXPECT: the check a developer runs before a push runs natively on the Linux dev host over the stages that survive
this version's removals.

**F17 — a push to `main` reads green.** (new)
OBSERVED: CI run `37907730264` on `60ef43c`, the push the merge of PR #39 made, ended failure in the `supply-chain`
job: `rustsec/audit-check` is denied a check run under `permissions: contents: read` (`ci.yml:8-9`, `:493-496`); on a
pull request the same denial prints "Unable to publish audit check!" and passes, so a green pull-request run does not
witness a repair.
EXPECT: the supply-chain job ends the same on a push as on a pull request; it fails on a finding and never on its own
reporting; the repair is witnessed where the defect shows.

## 6. Scope, priorities, non-goals

- **Order (ruled):** what replaces a thing is reachable before the chunk that removes the thing; the door (F6) stands
  before the removals the sidecar is built on and is what every check from outside the process reads through; after each theme the
  external harness (Conductor) checks the result from the input to the output, through the door. Pulse and Conductor advance in pairs,
  one at a time.
- **Out of this version (ruled):** the ladder of timeframes (half an hour, two hours, ten, a day) and every slow
  pattern that needs it; the engine calling a large model itself; phone channels and a web-hook; any interface
  beyond the panel module; encryption at rest; a check over stored metric points — in this version their consumer is
  the door (F6, F8); hosting platforms where nothing can run beside the service are
  served by F2's network receiver and are not themselves a test target.
- **Not designed for:** a classifier model in the engine; a team product (on-call rotations, shared rights,
  months of retention); one engine watching several products.
- **Standing rulings that stay visible and are not this version's work:** a finding with no hard cue is shown quietly,
  never as a red incident (2026-10-05) — for the version that surfaces slow findings; per-fingerprint incident
  identity stays declined.
- **Verify before building on it:** thresholds and formulas quoted in the study came through a summarising fetch.

## 7. Definition of done

1. **One end-to-end scenario (ruled).** Conductor rolls out a "bad version" of one service in its simulated world,
   over the network, to an engine on another host. Within minutes the panel module and a system notification carry a
   message naming the service, the version and the new error. A large model, given the report and the door, names
   the planted cause. The engine did this on a 1 GB node, and stayed silent through the healthy hours before it.
   (keeps P-102)
2. **No survivor.** At the close, the question this assembly was built from is asked again of the tree — what stands
   that exists only for the window, the local model, the desktop, another operating system, the plugins, the workspace
   or the training export, and what runs with no consumer — and the answer is nothing. (new)

## 8. What we expect Andromeda to derive from this intent

- `vision.md` from §1, §2, §4 and §7; `requirements.md` with one numbered capability per EXPECT, every removal
  included, each keeping the id this file names for it.
- A route in which no removal is a title only: each chunk that retires a thing names what leaves with it. And one in
  which no chunk maintains a thing this version replaces.
- `.andromeda/residuals.md` holds no `open` line (the first derivation flipped all fifteen). Of its four lines
  re-carried to 0.4.0: the half-hour digest's rate (`:27`) is closed by R2b; the stale doc comment above
  `select_corpus_matches` (`:23`) is corrected when that code is re-homed by R2b; quiet observations (`:17`) and
  per-fingerprint identity (`:9`) stay visible as §6 says. A fifth, the remainder of `:35` (stored metrics feed no
  check), stays visible too: §6 names the metrics' consumer in this version.
