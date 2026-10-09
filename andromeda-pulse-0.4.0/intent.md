# v0.4.0 Intent — andromeda-pulse

**Purpose & how to use this file.** This is the INTENT for version 0.4.0 — the single source of truth that
`/andromeda-route --version 0.4.0` derives `vision.md`, `requirements.md` and `working-route.md` from. Every
finding states what we OBSERVED and what we EXPECT. It says WHAT, never HOW: a transport, a file format or a
library named here is an example unless the line says "ruled".

_Assembled 2026-10-09 by the pc overseer from the founder's rulings of 2026-10-08 and 2026-10-09 (given by
dialog; the full trail with his words verbatim is `~/dev/projects/additional/pc-overseer/shaping-0.4.0.md`), from
the measured limits recorded at the 0.3.0 close (`.andromeda/residuals.md`, the eleven lines appended at `0e45d58`),
and from the study in this folder (`industry-instruments-and-engine-placement-2026-10-08.md`). The founder read its
Russian summary (`pc-overseer/intent-0.4.0-note.md`) and ruled the scope on 2026-10-09; he reviews the derived
requirements and route at the route's Phase 4._

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
- The DuckDB buffer with its tables, the retention sweep and the SQL aggregation queries, and concurrent reads
  beside the writer inside one process.
- The learned baseline, the cue families (error-rate spike, latency regression, retry storm, restart, silence),
  the latch, incident de-duplication, auto-resolve, the incident corpus and the search for similar past incidents.
- The exception fingerprint and the log-template mining.
- The curated snapshot (markdown, token-budgeted) and the MCP tool surface for incidents and reports.

## 4. What this version REMOVES (ruled by the founder, 2026-10-09)

Three things leave. Each is written below as a finding of its own (Theme 0), so that it gets a capability id and a
line in the verification matrix like anything that is added: a removal that is only prose is never checked.

## 5. Findings — OBSERVED vs EXPECT

Coordinates in OBSERVED were read from source at `18a872d`; `.andromeda/residuals.md` carries each in full.

### Theme 0 — What leaves

**R1 — the window is gone.** (ruled)
OBSERVED: a Tauri shell, a webview interface of about 24 thousand lines, two CI jobs that exist for it (the window's
boot smoke and the accessibility audit), and three masters whose subject it is (design-system, layout-templates,
a11y-plan). Most of 0.3.0's 22 verified capabilities are about it (the window, the widget, the constellation, the
tables, the Investigate buttons); the route names which, id by id.
EXPECT: no crate depends on the window toolkit; the tree holds no webview interface; no gate boots a window or audits
one; the masters say in plain words that this version has no interface and which later version brings a minimal
one on another toolkit. The 0.3.0 capabilities that were about the window are recorded as retired with the surface,
not as regressed.

**R2 — the local model is gone from the engine.** (ruled: «Уходит из движка»)
OBSERVED: an inference runtime that spawns a local model per generation, its grammar, its hardware probe, its
deterministic stand-in, and a decision probe that grades it; the incident's interpretation is the model's text.
EXPECT: the engine starts, detects and reports with no model present and none configured; no gate needs a GPU; the
words of a report come from the engine's own facts (F12). A large model reads through the door (F6) and is not part
of the engine.

**R3 — the corpus is stored unencrypted, for this first stage.** (ruled: «сразу все шифровать не стоит чтоб не усложнять
отлаживание … а потом будет видно»)
OBSERVED: the incident corpus is encrypted per cell with a key from the desktop's credential store; a server has no
such store (a CI runner already read `KeyringUnavailable`); the buffer is not encrypted.
EXPECT: the engine keeps its stores readable on its own node with no credential store; the secret scrubber at
ingest stays as it is; the security master records the decision, its date and that it is to be looked at again.

### Theme 1 — The engine stands on its own node

**F1 — a program without a window.**
OBSERVED: one application binary that is a Tauri process; no headless mode. `tauri` itself is declared by
`pulse-app` and, optionally, by `crates/ui-bridge`; the ingest and triage crates declare only an optional `specta`
derive behind a default-on `taurpc-runtime` feature and build without it. [corrected 2026-10-09 by the pc overseer,
after the route's Phase A re-read the manifests at `60ef43c`: this clause said `tauri` was a dependency of the
ingest and triage crates — a feature name had been read as a dependency.]
EXPECT: the engine is a console program that runs in the background on a Linux server and is driven by commands;
nothing in it needs a display, a GPU or a desktop session. Everything 0.3.0 detected is still detected.

**F2 — telemetry arrives over a network.** (ruled: the engine is a SEPARATE node from the watched service —
«не все захотят неизвестную штуку ставить рядом с сервисом».)
OBSERVED: the receivers bind `127.0.0.1`, the HTTP one admits only localhost `Host` values
(`crates/ingest/src/http.rs:113-117`), and neither asks for a credential.
EXPECT: a service on another host sends OTLP to the engine with a token over an encrypted channel; a request
without the token is refused; connecting a service takes an address and a token and installs nothing beside it.

**F2b — what the network receiver refuses.** (the token and the encrypted channel are ruled; the controls below are
proposed by the overseer for the founder's review at the route's Phase 4 — no specialist plan states them yet)
OBSERVED: the receiver's only guards today are its localhost bind, a `Host` allow-list, size and count invariants and
a global rate limit; none of them assumes a sender it does not trust.
EXPECT: a request with a missing or wrong token is refused before its body is read, and the refusal is recorded
without the token's value; the token is made, shown once, replaced and revoked by an engine command and is never
written to a log or a report; a sender that exceeds its rate is slowed or refused without starving another; the
existing size and count invariants apply to a remote sender as they do today; a plain unencrypted connection from
another host is refused. Everything the scrubber removes today is still removed before anything is stored.

**F3 — the watcher counts by event time.** (ruled)
OBSERVED: windows, cadences, the restart detector and the retention cutoff read the system clock; the restart cue
keys on a gap in ARRIVAL of more than 20 s.
EXPECT: every check reasons by the time stamped inside the record, with a stated allowance for lateness; a batch
that arrives late is counted where it belongs, and a recorded stream can be played again and give the same result.

**F4 — the node is small.** (ruled: 1 GB of memory, about $5 a month)
OBSERVED: the engine's memory use with a database on disk has never been measured.
EXPECT: the engine fits a 1 GB node under a long run; the figure is measured, and the database file does not grow
without bound under constant insert and delete.

### Theme 2 — Memory on disk and a working door

**F5 — what was received is still there tomorrow.**
OBSERVED: raw telemetry lives 600 s in an in-memory database; only incidents persist; the learned baseline is
discarded at start when older than an hour.
EXPECT: raw telemetry is kept for days and survives a restart; what the watcher has learned survives a restart; a
gap while the engine was down is not read as the service's silence.

**F6 — the agent's door opens onto the real data.**
OBSERVED: the MCP sidecar's four telemetry tools read their own empty in-memory database; they accept only a
time window and a limit.
EXPECT: an agent on the developer's machine reaches the engine's tools and reads the telemetry the engine holds,
filtered by service, time range, trace and text, with aggregates; it reaches one incident whole, with its
evidence. The door lives inside the engine's process. The same door serves the test harness and the status
widget (F12).

**F6b — what the door may do.** (proposed by the overseer for the founder's review at the route's Phase 4)
OBSERVED: the sidecar's tools are local, unauthenticated, and one of them writes (it marks an incident resolved).
EXPECT: the door answers only a party the node's owner has admitted; it reads the engine's stores — telemetry,
incidents, reports, the engine's own state — and nothing else on the node: not the token, not configuration secrets,
not files outside those stores; every answer is bounded in size and time, so no question can exhaust the node; what
it returns was scrubbed at ingest; the one write it keeps (resolving an incident) is named as a write and can be
switched off.

### Theme 3 — Keep what OpenTelemetry sends

**F7 — a span keeps its meaning.**
OBSERVED: a span is stored as seven fields; its name, kind, parent, status message, every attribute and every
resource attribute but `service.name` are dropped at ingest; two aggregation queries reference a parent column the
table does not have, and their results are discarded.
EXPECT: the stored span keeps its name, kind and parent, the service's version and environment, and the
attributes that say which route, which status code, which database or peer it touched — under both generations of
the semantic-convention names. Who calls whom can be answered from what is stored.

**F8 — metrics and logs keep theirs.**
OBSERVED: histogram-family points are stored as the value 0; a log row and a metric row carry no service
identity; stored exception text has no reader.
EXPECT: a percentile can be computed from stored metrics; a log line and a metric point name their service;
exception type, message and stack are readable through the door.

### Theme 4 — Comparisons that work from the first minute

**F9 — a new kind of error is news.**
OBSERVED: the fingerprint feeds the retry-storm detector and the search for similar past incidents; nothing says
"this error was never seen before".
EXPECT: the first appearance of an error fingerprint for a service is a finding, and so is the return of one that
had gone away.

**F10 — a release is compared with the one before.**
OBSERVED: the service version is dropped at ingest; the digest has never carried a commit; nothing knows a
release happened.
EXPECT: when a new version of a service appears, the engine compares it with the previous one — error rate, new
error kinds, latency — and says when the new one is worse, naming the version and the moment it appeared.

**F11 — silence is told apart from "nothing arrived".**
OBSERVED: a vanished stream and an engine that could not receive look alike.
EXPECT: "the service went quiet", "no data reached the engine" and "the check could not be evaluated" are three
different states, and a recovery is announced.

### Theme 5 — The voice

**F12 — the engine says it, in one line and in a report.** (ruled: the first surface is a panel module fed by an
engine command, on the founder's own desktop (Omarchy, Waybar) only; phone channels and a web-hook are 0.5.0.)
OBSERVED: findings are visible only inside the window.
EXPECT: a command of the engine prints the current state in one line a desktop panel can show; a change of state
raises a system notification; the notification carries a short report — what, where, since when, after which
version, which errors are new — written without a model.

**F13 — the report is enough for a large model.**
OBSERVED: the incident's interpretation was written by the local model; no measurement exists of whether a large
model can work from it.
EXPECT: given the report and the door, a large model names the cause that was planted. The report is narrow and
names its evidence by identifier; "not enough data" is an honest answer the engine can give.

**F14 — a real service is watched too.** (ruled 2026-10-08: a real stream beside the synthetic one, from one of the
founder's own local projects; which project is not yet named — ask him before this finding is taken up.)
OBSERVED: every reading of Pulse so far was taken on telemetry a test tool composed.
EXPECT: one real service of the founder's sends its telemetry to the engine for days; what the engine reported and
what it stayed silent on is read against what actually happened to that service.

## 6. Scope, priorities, non-goals

- **Order (ruled):** the five themes in the order above; after each, the external harness (Conductor) checks the
  result from the input to the output. Pulse and Conductor advance in pairs, one at a time.
- **Out of this version (ruled):** the ladder of timeframes (half an hour, two hours, ten, a day) and every slow
  pattern that needs it; the engine calling a large model itself; phone channels and a web-hook; any interface
  beyond the panel module; encryption at rest; hosting platforms where nothing can run beside the service are
  served by F2's network receiver and are not themselves a test target.
- **Not designed for:** a classifier model in the engine; a team product (on-call rotations, shared rights,
  months of retention).
- **Verify before building on it:** thresholds and formulas quoted in the study came through a summarising fetch.

## 7. Definition of done (ruled: ONE end-to-end scenario, plus a check at every theme)

Conductor rolls out a "bad version" of one service in its simulated world, over the network, to an engine on
another host. Within minutes the panel module and a system notification carry a message naming the service, the
version and the new error. A large model, given the report and the door, names the planted cause. The engine did
this on a 1 GB node, and stayed silent through the healthy hours before it.

## 8. What we expect Andromeda to derive from this intent

- `vision.md` from §1, §2 and §7; `requirements.md` with one numbered capability per EXPECT, the three removals
  of Theme 0 included (ids continue after `P-082`).
- The eleven `open` lines of `.andromeda/residuals.md` dispositioned against these findings: the limits recorded
  at the 0.3.0 close match F5–F10 and are absorbed there; "report without a model" is F12; "quiet observations"
  stays a ruling for the version that surfaces slow findings.
- A route whose first entries remove the window and the local model, so that every later chunk builds and tests on
  Linux alone.
