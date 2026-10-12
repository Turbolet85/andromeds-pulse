# Requirements — andromeda-pulse v0.4.0

_Derived from `andromeda-pulse-0.4.0/intent.md` (its second assembly, 2026-10-09) §5 findings (OBSERVED→EXPECT) and
§7 by `/andromeda-route` Phase A on 2026-10-09 — the SECOND derivation; it supersedes, as a whole, the requirements
committed at `39edd11` and adapted at `7f99c38` (25 capabilities, none claimed, none built on). One numbered
capability per EXPECT, every removal included: a removal is a capability like any other. **39 capabilities.** Ids
follow the intent finding by finding: the 25 findings that existed in the first derivation KEEP their ids
(P-083…P-107, subjects unchanged; P-084 and P-092 are widened), and the 14 findings the intent marks `new` take
**P-108…P-121** in the intent's order (the ID rule's own scan: the highest id in any prior `requirements.md` is
P-107). Each capability's EXPECT is its acceptance shape; the OBSERVED clause is carried into
`verification-matrix.json` as `observed_gap`. Ids are reused verbatim by the verification matrix._

_Marked by the intent as PROVISIONAL until the founder's own word: **P-088** beyond its token and its channel (F2b),
**P-093** (F6b) and **P-107** (F14). **P-101** waits on the founder naming the service (F15)._

## Theme 0 — What leaves
- P-083 · The window is gone — no crate depends on the window toolkit; the tree holds no webview interface, no IPC router and no gate that boots a window, audits one or samples its frames; the tests that drive them leave with them and the remaining gate is green; the masters say in plain words that this version has no interface and which later version brings a minimal one on another toolkit; the 0.3.0 capabilities that were about the window are recorded as retired with the surface, not as regressed (per intent R1, ruled).
- P-108 · The bridge leaves the engine's crates — no engine crate carries a feature, a derive or a dependency that exists for an IPC bridge; the engine's configuration is the engine's own and holds no key about an interface; the bridge crate is gone (per intent R1b).
- P-109 · What was computed only to be shown is gone — the engine does no work whose only reader was the window: no stream topic, no per-batch encode for a subscriber that does not exist, no display state; the service registry's liveness truth stays engine state (per intent R1c).
- P-084 · The local model is gone from the engine, with everything that served it — the engine starts, detects and reports with no model present and none configured; no gate needs a GPU; no crate, example, script, environment variable, vendored runtime or experiment that exists for a local model remains, and their tests leave with them; the words of a report come from the engine's own facts (P-099); a large model reads through the door (P-092) and is not part of the engine (per intent R2, ruled, widened; + `.andromeda/residuals.md:19`).
- P-110 · The digest is gone — the engine builds with no network access and no tokenizer; it assembles no prompt, counts no tokens and archives no digest; the digest archive table is gone; the search for similar past incidents is kept and lives where incidents live; the half-hour rate defect is closed by the code's removal, not carried (per intent R2b; closes `.andromeda/residuals.md:27`).
- P-111 · The cadence coordinator is gone — nothing in the engine decides when a model should run; no hardware profile, tier or reflection cadence remains, and no configuration key for one (per intent R2c).
- P-112 · An incident is the engine's own record — an incident stores what the engine knows (which check found it, in which service, since when, after which version, which errors, which evidence by identifier) and nothing shaped as a model's answer; one report form exists, the one P-099 describes, and the old renderer is gone; a store written by 0.3.0 is not read: the engine on a node starts from its own empty stores (per intent R2d).
- P-085 · The corpus is stored unencrypted, for this first stage — the engine keeps its stores readable on its own node with no credential store, and no code or dependency for one remains; the secret scrubber at ingest stays as it is; the security master records the decision, its date and that it is to be looked at again (per intent R3, ruled).
- P-104 · The plugin host is gone — the workspace depends on no WebAssembly runtime; no plugin host or example is in the tree; the critical path that drove the plugin lifecycle is recorded as retired, not regressed (per intent R4, ruled 2026-10-09).
- P-105 · Workspace detection is gone — the engine reads no project folder; one engine watches one product: its incidents live in its one store and are told apart by the check and the service alone; nothing takes the workspace key's place, and in this version a sender's token is not an incident key — the same storm sent under two tokens forms one incident; incidents are formed, kept, listed and resolved the same from wherever the engine is started (per intent R5, ruled 2026-10-09).
- P-106 · The training export is gone — no export exists and nothing the engine writes lands outside its own stores (per intent R6, ruled 2026-10-09).
- P-113 · The other operating systems are gone from the code — the tree builds, tests and runs on Linux alone and holds no branch, script, link hint or workaround for another operating system; CI has no leg for one (per intent R7).
- P-114 · The desktop's distribution is gone — no workflow, runbook, test or bot configuration for a desktop bundle, an updater, a package channel or an npm tree remains; the two external channel repositories are the founder's to retire by hand and are named on the card (per intent R8; the engine's own delivery is P-107).
- P-115 · The log allowlist describes this engine — every allowed log target is one the engine can emit; none names a window, a model, a digest, a tray or a plugin (per intent R9).
- P-116 · The records say what the product is — every record a newcomer or an agent reads first describes a console engine on its own node, for one developer with one product; no rule instructs what this version removes or forbids what it adds; what describes 0.2–0.3 is either gone or marked as the history of those versions (per intent R10).
- P-117 · The capability record is re-based — one current record says, id by id for all 82, whether the engine still claims the capability or it was retired with the window, the model or the desktop; the old gate carries no retired id; that record is what the external harness syncs its accepted set to (per intent R11).

## Theme 1 — The engine stands on its own node
- P-086 · A program without a window — the engine is a console program that runs in the background on a Linux server and is driven by commands; nothing in it needs a display, a GPU or a desktop session; everything 0.3.0 detected is still detected (per intent F1).
- P-087 · Telemetry arrives over a network — a service on another host sends OTLP to the engine with a token over an encrypted channel; a request without the token is refused; connecting a service takes an address and a token and installs nothing beside it (per intent F2, ruled: the engine is a separate node from the watched service).
- P-088 · What the network receiver refuses — a request with a missing or wrong token is refused before its body is read, and the refusal is recorded without the token's value; the token is made, shown once, replaced and revoked by an engine command and is never written to a log or a report; it is stated who terminates the encrypted channel and where its private key lives — never in a store the door reads, never in a log; a sender that exceeds its rate is slowed or refused without starving another; the existing size and count invariants apply to a remote sender; a plain unencrypted connection from another host is refused; everything the scrubber removes today is still removed before anything is stored (per intent F2b; the token and the channel are ruled, the rest PROVISIONAL until the founder's own word).
- P-089 · The watcher counts by event time — every check reasons by the time stamped inside the record, with a stated allowance for lateness; a batch that arrives late is counted where it belongs, and a recorded stream can be played again and give the same result (per intent F3, ruled).
- P-090 · The node is small — the engine fits a 1 GB node under a long run; the figure is measured, and the database file does not grow without bound under constant insert and delete (per intent F4, ruled: 1 GB of memory, about $5 a month).
- P-118 · The engine has one place on a node — the operator says where the engine keeps its stores, its log and its pid, and there is one documented default for a service; one piece of code resolves it (per intent F4b).
- P-107 · The engine reaches a node — the engine is delivered as one Linux binary with a documented way to run it as a service; on a clean node the documented steps give an engine that runs, comes back after a restart and receives telemetry (per intent F14; PROVISIONAL until the founder's own word).

## Theme 2 — Memory on disk and a working door
- P-091 · What was received is still there tomorrow — raw telemetry is kept 7 days by default, the term configurable, with a ceiling on the store's size beside it at which the oldest goes first, and survives a restart; what the watcher has learned survives a restart; a gap while the engine was down is not read as the service's silence; whether a week fits the node's disk under a real stream is measured, and the ceiling's figure is set from that reading (per intent F5; the 7 days are ruled).
- P-092 · The agent's door opens onto the real data — an agent on the developer's machine reaches the engine's tools and reads the telemetry the engine holds, filtered by service, time range, trace and text, with aggregates; it reaches one incident whole, with its evidence, and the incidents like it from the past; it can ask for the curated snapshot — a narrow, budgeted view of recent telemetry made for a large model — which is that code's consumer from now on; the door lives inside the engine's process; the same door serves the test harness and the status widget (per intent F6, widened; + `.andromeda/residuals.md:31`; + `.andromeda/residuals.md:11`).
- P-093 · What the door may do — the door answers only a party the node's owner has admitted, and admission has a lifecycle: how a party is admitted and how it is revoked, as the token has; the door is encrypted in transit; it reads the engine's stores (telemetry, incidents, reports, the engine's own state) and nothing else on the node: not the token, not configuration secrets, not files outside those stores; every answer is bounded in size and time, so no question can exhaust the node; what it returns was scrubbed at ingest; the one write it keeps (resolving an incident) is named as a write and can be switched off (per intent F6b; PROVISIONAL until the founder's own word).

## Theme 3 — Keep what OpenTelemetry sends
- P-094 · A span keeps its meaning — the stored span keeps its name, kind and parent, the service's version and environment, and the attributes that say which route, which status code, which database or peer it touched, under both generations of the semantic-convention names; who calls whom can be answered from what is stored (per intent F7; + `.andromeda/residuals.md:33`).
- P-095 · Metrics and logs keep theirs — a percentile can be computed from stored metrics; a log line and a metric point name their service; exception type, message and stack are readable through the door (per intent F8; + `.andromeda/residuals.md:35`).
- P-119 · Every aggregation has a consumer — at the end of this version each of the seven aggregation queries either feeds a check, a report or a door answer the intent names (per-operation for P-097, high-severity logs for P-096, caller→callee and critical path for P-094) or is gone; none runs for nothing (per intent F8b).

## Theme 4 — Comparisons that work from the first minute
- P-096 · A new kind of error is news — the first appearance of an error for a service is a finding, in whichever of the three encodings it arrives (a span's error status, an exception on a span, a high-severity log), and so is the return of one that had gone away (per intent F9).
- P-097 · A release is compared with the one before — when a new version of a service appears, the engine compares it with the previous one (error rate, new error kinds, latency, per operation where the operation is known) and says when the new one is worse, naming the version and the moment it appeared (per intent F10; + `.andromeda/residuals.md:29`).
- P-098 · Silence is told apart from "nothing arrived" — "the service went quiet", "no data reached the engine" and "the check could not be evaluated" are three different states, and a recovery is announced (per intent F11).

## Theme 5 — The voice
- P-099 · The engine says it, in one line and in a report — a command of the engine prints the current state in one line a desktop panel can show; a change of state raises a system notification; the notification carries a short report (what, where, since when, after which version, which errors are new) written without a model; this is the one report form (P-112) (per intent F12, ruled: the first surface is a panel module fed by an engine command, on the founder's own desktop only; + `.andromeda/residuals.md:19`).
- P-100 · The report is enough for a large model — given the report and the door, a large model names the cause that was planted; the report is narrow and names its evidence by identifier; "not enough data" is an honest answer the engine can give (per intent F13).
- P-101 · A real service is watched too — one real service of the founder's sends its telemetry to the engine for days; what the engine reported and what it stayed silent on is read against what actually happened to that service; he names the service together with the personal data its telemetry carries, and until then the security master's "Compliance triggers: None" keeps its present basis (per intent F15, ruled 2026-10-08; which project is not yet named — the founder is asked before this capability is taken up).

## Theme 6 — The build and its gates
- P-103 · The check before a push runs on Linux — the check a developer runs before a push runs natively on the Linux dev host over the stages that survive this version's removals (per intent F16, carried from 0.3.0 by ruling; + `.andromeda/residuals.md:21`).
- P-120 · A push to `main` reads green — the supply-chain job ends the same on a push as on a pull request; it fails on a finding and never on its own reporting; the repair is witnessed where the defect shows (per intent F17).

## Definition of done
- P-102 · The bad-version scenario, end to end (dynamic-external) — Conductor rolls out a "bad version" of one service in its simulated world, over the network, to an engine on another host; within minutes the panel module and a system notification carry a message naming the service, the version and the new error; a large model, given the report and the door, names the planted cause; the engine did this on a 1 GB node and stayed silent through the healthy hours before it (per intent §7 point 1, ruled; after each theme the external harness checks that theme's result from the input to the output, per intent §6 "Order"; the scenario exercises P-086, P-087, P-090, P-092, P-096, P-097, P-099 and P-100 together).
- P-121 · No survivor — at the close, the question the intent was assembled from is asked again of the tree (what stands that exists only for the window, the local model, the desktop, another operating system, the plugins, the workspace or the training export, and what runs with no consumer) and the answer is nothing (per intent §7 point 2).

## Notes for the reader (not capabilities)

**The 0.3.0 capabilities and what retires each, id by id** (intent R1 leaves the naming to the route; read from
`andromeda-pulse-0.3.0/verification-matrix.json`, 22 entries P-061…P-082, all `verified`. This classification is the
route's reading and is the founder's to correct at Phase 4; the id-by-id record for all 82 is P-117's work, not this
note's):
- wholly about the window, retired by P-083 (13): P-061 · P-062 · P-063 · P-064 · P-065 · P-066 · P-068 · P-069 ·
  P-070 · P-071 · P-080 · P-081 · P-082.
- gates that drive the window, retired by P-083 (2): P-076 (the integration UX end-to-end test, which also drives the
  model's Investigate actions — P-084) · P-078 (the headful self-verify harness).
- about the window and the local model (1): P-072 (the Investigate actions run a model analysis) — retired by P-083
  and P-084 together.
- about the local model alone (1): P-073 (the deterministic stand-in) — retired by P-084.
- about the window and the workspace key (1): P-079 — its constellation half goes with P-083, its single-sourced
  workspace key with P-105.
- partly (2): P-067 (the surface that showed only live services goes; the service registry's liveness truth stays
  engine state, P-109) · P-075 (its four delegated timing bounds each end at a paint in the window and go with
  P-083, its stand-in model with P-084; its telemetry → incident → read-back half continues as P-102).
- still claimed (2): P-074 (one incident for a sustained storm — the outcome holds under P-086's "everything 0.3.0
  detected is still detected"; the cadence → digest → model chain its text names leaves with P-084, P-110 and P-111)
  · P-077 (the demo telemetry injector).

**OBSERVED clauses of the intent, re-read at HEAD `7f99c38` on 2026-10-09** (the tree differs from HEAD only in
`intent.md` and the handoff). The EXPECT of each finding is unaffected; the matrix's `observed_gap` states the
measured form where it differs. Confirmed as written: R1's 243 tracked files under `pulse-app/ui` and its nine named
`xtask` modules; all of R1b; R1c's thirteen stream topics and the display-state file; R2's six model environment
variables (two `xtask` sites set one of them) and the vendored converter; R2b (6 files, 2820 lines, the tokenizer
fetched from a third-party host at build); R2c (4 files, 1628 lines, the seven queries kept only as a success count);
R2d's two coordinates; R3; R4; R5's cooldown key and three `WHERE workspace` queries; R6; R7's seven `.ps1` files
(754 lines), the `wsl.exe` hop, the three link hints, the linker override, `[lib] test = false` and the three CI
jobs with other-OS legs; R8; F2's `Host` allow-list; F5's 600 s and one-hour figures; F6's 3196 lines; F8b's seven
queries and the absent parent column; F16; F17's two workflow coordinates. Differing or not reproduced:
- R1 "eleven IPC routers": 10 files named `*_router.rs` under `pulse-app/src`, and 18 procedures traits in 16 source
  files (17 with a path, one root); neither count gives eleven. "About 23 thousand lines": the counting basis was not
  reproduced.
- R2 "`crates/interpretation` (8 files)": 9 tracked files (8 Rust sources and the schema document), 3684 lines. The
  decision probe is three files, 10507 lines; its main file alone is 6566. `experiments/` holds no tracked file — it
  is a git-ignored local directory, so no commit can show it gone.
- R7 "four `pwsh`/`powershell.exe` spawns": five (four `pwsh` in `xtask/src/main.rs`, one `powershell` in
  `xtask/src/perf_frame.rs`). "A data-directory resolver … in three copies": five source files carry the Windows or
  macOS data-directory names; which of them are whole resolvers was not read.
- R11 "82 capabilities … in `docs/v0_2_0/pulse-capability-spec.md` and in a gate over
  `capability-verification-matrix.json`": that spec and that file hold P-001…P-060 (60 ids); P-061…P-082 stand in
  `andromeda-pulse-0.3.0/verification-matrix.json` (22). The 82 are spread over two records. That the external
  harness pins its accepted set to 82 was not read here (another repository).
- Not re-read: R9's 215 log targets and their split; F3's 20 s arrival gap.
- One coupling the intent's §3 does not state: `start_emitter` (`crates/triage/src/cue/emitter.rs:399`) takes a
  handle to the cadence trigger channel as a parameter. Detection does not wait on the coordinator, but the emitter
  is wired to its channel — a fact for the chunk that retires the coordinator (P-111).

**Theme 0 holds sixteen findings** (R1, R1b, R1c, R2, R2b, R2c, R2d, R3…R11), counted from the intent's headings; the
relayed directive for this run says fifteen. All sixteen have a capability above.

## Carried residuals (NOT 0.4.0 capabilities — deliberately unnumbered, no matrix entry)

_`.andromeda/residuals.md` holds no `open` line (15 entries: 7 absorbed, 4 dropped, 4 re-carried to 0.4.0 — all
flipped by the first derivation), the 0.3.0 matrix holds no `deferred` entry, and the 0.3.0 `requirements.md` has no
carried-residuals section: the letter's three sources collect nothing. The four lines re-carried to 0.4.0 are
dispositioned below by the intent's §8; the letters give no flip from `re-carried:0.4.0`, so each line keeps that
status in `.andromeda/residuals.md` and its disposition is recorded here._

- **Layer-2 incident identity is revisitable** (`.andromeda/residuals.md:9`, origin
  `2026-08-17-incident-fingerprint-producer-repaired`) — per-fingerprint incident de-duplication is possible and
  stays declined; incidents coalesce per check and service. Stays visible because intent §6 names it a standing
  ruling that is not this version's work: P-096 makes a new error a finding and does not re-open incident identity.
- **Quiet observations** (`.andromeda/residuals.md:17`, origin `2026-10-05-l4-model-chosen-by-pattern-discrimination`)
  — the founder's ruling of 2026-10-05 that a finding with no hard cue is shown quietly, never as a red incident, is
  not withdrawn. Stays visible because intent §6 keeps it for the version that surfaces slow findings, and the slow
  patterns are out of 0.4.0.
- **Stale doc comment above the similar-incident search** (`.andromeda/residuals.md:23`, origin
  `2026-10-07-l4-probe-reproduces-the-canary-history-miss`) — owed: the comment corrected to the wiring as it is, its
  do-not-simplify guard kept. Not a capability and never a chunk of its own; by intent §8 it is corrected by the
  chunk that re-homes that code under P-110. Stays visible here until that chunk's wrap.
- **The half-hour digest's rate** (`.andromeda/residuals.md:27`, origin `2026-10-09-0-pending-version-close`) — a
  reflection digest's request rate is divided by a literal 60 over an 1800 s window. By intent §8 it is closed by
  P-110: the code that computes it is removed, not repaired. Stays visible here until that chunk's wrap, because the
  residuals line cannot be flipped to `absorbed:P-110` by any step of the letters.
- **Stored metrics feed no check** (the remainder of `.andromeda/residuals.md:35`, a line absorbed whole as P-095) —
  the line's text says no triage query reads the stored metric points and the one read of stored logs is discarded.
  The log half now has an owner (the high-severity-log query feeds P-096 under P-119). The metric half has none:
  P-095 expects a percentile to be computable from stored metrics and no finding expects a check over them. In 0.4.0
  the stored metrics' consumer is the door's metric query (P-092, P-095); no finding of the intent expects a check
  over metrics. Not one of the four lines intent §8 dispositions; kept visible by the letter's default (unmatched →
  re-carried), and kept so on the operator's word at this route's Phase 4 (2026-10-09).
