# Vision — andromeda-pulse v0.4.0

_Derived from `andromeda-pulse-0.4.0/intent.md` §1 (context), §2 (the core problem) and §7 (definition of done) by
`/andromeda-route` Phase A on 2026-10-09. The intent file is the authority on what this version delivers; the seven
masters under `.andromeda/` still describe the 0.3.0 system and change only at the wrap of the chunk that changes
each thing._

## The problem

Pulse measures a small part of what it is sent, forgets it within ten minutes, and can say what it found only
through a window on the developer's own machine. So it can neither stand watch next to a deployed product nor give
an investigating agent anything to dig in (intent §2).

## Who it is for, and why

One developer with one product. On 2026-10-08/09 the founder restated what Pulse is for: a quiet helper that spares
that developer both the set-up of a large commercial observability system and the standing job of watching its
dashboards. It keeps the measuring instruments of the mainstream systems and removes the specialist and the
dashboards: it notices by itself, says in plain words what broke, and hands a report to a large model that
investigates (intent §1).

## What this version changes

- **It removes** the window (the Tauri shell and the webview interface, with everything that exists only to serve
  it), the local model inside the engine, and — for this first stage — the incident corpus's encryption at rest
  (intent §4, Theme 0). Each removal is a capability with its own id and its own line in the verification matrix.
- **It keeps and builds on** what works today: OTLP ingest of traces, metrics and logs with its invariants, rate
  limits and secret scrubber; the buffer's tables, retention sweep and aggregation queries; the learned baseline,
  the five cue families, the latch, incident de-duplication, auto-resolve, the incident corpus and the search for
  similar past incidents; the exception fingerprint and log-template mining; the curated snapshot and the MCP tool
  surface for incidents and reports (intent §3).
- **It adds**, in the founder's ruled order (intent §6): an engine that stands on its own small Linux node and takes
  telemetry over a network behind a token; memory on disk and an agent's door onto the real data; what
  OpenTelemetry sends kept with its meaning; comparisons that work from the first minute (a new kind of error, a
  release against the one before, silence told apart from "nothing arrived"); and a voice — one line a desktop
  panel shows, a system notification, a report written without a model.

## Out of this version

The ladder of timeframes (half an hour, two hours, ten, a day) and every slow pattern that needs it; the engine
calling a large model itself; phone channels and a web-hook; any interface beyond the panel module; encryption at
rest; hosting platforms where nothing can run beside the service as a test target. Not designed for: a classifier
model in the engine; a team product (intent §6).

## What "0.4.0 done" means

One end-to-end scenario, plus a check at every theme (intent §7, ruled). Conductor rolls out a "bad version" of one
service in its simulated world, over the network, to an engine on another host. Within minutes the panel module and
a system notification carry a message naming the service, the version and the new error. A large model, given the
report and the door, names the planted cause. The engine did this on a 1 GB node, and stayed silent through the
healthy hours before it. After each of the five themes the external harness checks that theme's result from the
input to the output; Pulse and Conductor advance in pairs, one at a time.
