# Vision — andromeda-pulse v0.4.0

_Derived from `andromeda-pulse-0.4.0/intent.md` (its third assembly, 2026-10-09) §1 (context), §2 (the core
problem), §4 (the rule of this version), §6 (the ruled order) and §7 (definition of done) by `/andromeda-route`
Phase A on 2026-10-09, in run `2026-10-09T10-26-56-route`; it supersedes the vision committed at `39edd11` and the
second derivation's text of the same run. The intent file is the authority on what this version delivers; the seven
masters under `.andromeda/` still describe the 0.3.0 system and change only at the wrap of the chunk that changes
each thing._

## The problem

Pulse measures a small part of what it is sent, forgets it within ten minutes, and can say what it found only
through a window on the developer's own machine. So it can neither stand watch next to a deployed product nor give
an investigating agent anything to dig in (intent §2).

## Who it is for, and why

One developer with one product. On 2026-10-08/09 the founder restated what Pulse is for: a quiet helper that spares
that developer both the set-up of a large commercial observability system and the standing job of watching its
dashboards — in his words, a headless Datadog. It keeps the measuring instruments of the mainstream systems and
removes the specialist and the dashboards: it notices by itself, says in plain words what broke, and hands a report
to a large model that investigates (intent §1).

## The rule of this version

Six things were ruled out of the product: the window, the local model inside the engine, the desktop it lived on,
the plugin host, workspace detection and the training export. **A removal is complete only when what existed solely
to serve the removed thing is gone too, or has a new consumer the intent names.** The founder's word on how the
version is done: nothing that does not fit the new form is smoothed over. So every removal is a capability with its
own id and its own line in the verification matrix, whose acceptance states an outcome (intent §4).

## What this version changes

- **It removes** (intent Theme 0, sixteen findings): the window, the bridge's types inside the engine's crates and
  everything computed only to be shown; the local model with the digest, the cadence coordinator and the incident
  stored as the model's answer; the corpus's encryption at rest, for this first stage; the plugin host, workspace
  detection and the training export; the other operating systems in the code; the desktop's distribution; the log
  targets, the records and the capability record that describe the old product.
- **It keeps and builds on** (intent §3): OTLP ingest of traces, metrics and logs with its invariants, rate limits
  and the secret scrubber; the buffer's tables and retention sweep; the learned baseline, the cue emitter, the latch,
  incident de-duplication and auto-resolve; the exception fingerprint and log-template mining; the incident corpus
  and the search for similar past incidents; the curated snapshot and its curation; the telemetry queries behind the
  agent's tools; configuration hot reload.
- **It adds**, in the founder's ruled order — what replaces a thing is reachable before the chunk that removes the
  thing (intent §6): a console engine on its own small Linux node, with one place on that node, readable by its owner
  alone, and a way to reach it; an agent's door inside the engine from the engine's first step — it replaces the old
  sidecar ahead of the removals the sidecar is built on, and every reading from outside the process goes through it;
  telemetry over a network behind a token; checks that reason by event time; memory on disk and the door's answers
  over the real data; what OpenTelemetry sends kept with its meaning; comparisons that work from the first minute (a
  new kind of error, a release against the one before, silence told apart from "nothing arrived"); a voice — one
  line a desktop panel shows, a system notification, one report form written without a model; a pre-push check and
  a `main` that read green on Linux.

## Out of this version

The ladder of timeframes (half an hour, two hours, ten, a day) and every slow pattern that needs it; the engine
calling a large model itself; phone channels and a web-hook; any interface beyond the panel module; encryption at
rest; a check over stored metric points (in this version their consumer is the door); hosting platforms where
nothing can run beside the service, as a test target. Not designed for: a classifier
model in the engine; a team product; one engine watching several products (intent §6).

## What "0.4.0 done" means

Two things, both ruled or stated in intent §7.

1. **One end-to-end scenario.** Conductor rolls out a "bad version" of one service in its simulated world, over the
   network, to an engine on another host. Within minutes the panel module and a system notification carry a message
   naming the service, the version and the new error. A large model, given the report and the door, names the
   planted cause. The engine did this on a 1 GB node, and stayed silent through the healthy hours before it. After
   each theme the external harness checks that theme's result from the input to the output, through the door; Pulse
   and Conductor advance in pairs, one at a time.
2. **No survivor.** At the close the question the intent was assembled from is asked again of the tree — what stands
   that exists only for the window, the local model, the desktop, another operating system, the plugins, the
   workspace or the training export, and what runs with no consumer — and the answer is nothing.
