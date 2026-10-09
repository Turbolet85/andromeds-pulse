# Requirements — andromeda-pulse v0.4.0

_Derived from `andromeda-pulse-0.4.0/intent.md` §5 findings (OBSERVED→EXPECT) and §7 by `/andromeda-route` Phase A on
2026-10-09. One numbered capability per finding, the three removals of Theme 0 included: a removal is a capability
like any other. Capability ids continue the project scheme after **P-082**, the highest id the 0.3.0 verification
matrix holds (its `requirements.md` stops at P-078; P-079…P-082 were minted into the ledger after route) → v0.4.0 =
**P-083…P-103**. Each capability's EXPECT is its acceptance shape; the OBSERVED clause is carried into
`verification-matrix.json` as `observed_gap`. Ids are reused verbatim by the verification matrix._

_Two capabilities do not come from a §5 finding: **P-102** is the intent's §7 definition of done (one end-to-end
scenario, delegated to the external harness), and **P-103** is a residual absorbed on the operator's word of
2026-10-09 (`.andromeda/residuals.md:21`). **P-088** and **P-093** carry controls the intent marks as proposed by the
overseer for the founder's review at this route's Phase 4 (F2b, F6b)._

## Theme 0 — What leaves
- P-083 · The window is gone — no crate depends on the window toolkit; the tree holds no webview interface; no gate boots a window or audits one; the masters say in plain words that this version has no interface and which later version brings a minimal one on another toolkit; the 0.3.0 capabilities that were about the window are recorded as retired with the surface, not as regressed (per intent R1, ruled).
- P-084 · The local model is gone from the engine — the engine starts, detects and reports with no model present and none configured; no gate needs a GPU; the words of a report come from the engine's own facts (P-099); a large model reads through the door (P-092) and is not part of the engine (per intent R2, ruled; + `.andromeda/residuals.md:19`, its unimplemented CPU-route retirement).
- P-085 · The corpus is stored unencrypted, for this first stage — the engine keeps its stores readable on its own node with no credential store; the secret scrubber at ingest stays as it is; the security master records the decision, its date and that it is to be looked at again (per intent R3, ruled).

## Theme 1 — The engine stands on its own node
- P-086 · A program without a window — the engine is a console program that runs in the background on a Linux server and is driven by commands; nothing in it needs a display, a GPU or a desktop session; everything 0.3.0 detected is still detected (per intent F1).
- P-087 · Telemetry arrives over a network — a service on another host sends OTLP to the engine with a token over an encrypted channel; a request without the token is refused; connecting a service takes an address and a token and installs nothing beside it (per intent F2, ruled: the engine is a separate node from the watched service).
- P-088 · What the network receiver refuses — a request with a missing or wrong token is refused before its body is read, and the refusal is recorded without the token's value; the token is made, shown once, replaced and revoked by an engine command and is never written to a log or a report; a sender that exceeds its rate is slowed or refused without starving another; the existing size and count invariants apply to a remote sender as they do today; a plain unencrypted connection from another host is refused; everything the scrubber removes today is still removed before anything is stored (per intent F2b; the token and the encrypted channel are ruled, the remaining controls are proposed for the founder's review).
- P-089 · The watcher counts by event time — every check reasons by the time stamped inside the record, with a stated allowance for lateness; a batch that arrives late is counted where it belongs, and a recorded stream can be played again and give the same result (per intent F3, ruled).
- P-090 · The node is small — the engine fits a 1 GB node under a long run; the figure is measured, and the database file does not grow without bound under constant insert and delete (per intent F4, ruled: 1 GB of memory, about $5 a month).

## Theme 2 — Memory on disk and a working door
- P-091 · What was received is still there tomorrow — raw telemetry is kept for days and survives a restart; what the watcher has learned survives a restart; a gap while the engine was down is not read as the service's silence (per intent F5).
- P-092 · The agent's door opens onto the real data — an agent on the developer's machine reaches the engine's tools and reads the telemetry the engine holds, filtered by service, time range, trace and text, with aggregates; it reaches one incident whole, with its evidence; the door lives inside the engine's process; the same door serves the test harness and the status widget (per intent F6; + `.andromeda/residuals.md:31`, the sidecar's telemetry tools read an empty database; + `.andromeda/residuals.md:11`, an incident's span and timestamp evidence is empty in every mode).
- P-093 · What the door may do — the door answers only a party the node's owner has admitted; it reads the engine's stores (telemetry, incidents, reports, the engine's own state) and nothing else on the node: not the token, not configuration secrets, not files outside those stores; every answer is bounded in size and time, so no question can exhaust the node; what it returns was scrubbed at ingest; the one write it keeps (resolving an incident) is named as a write and can be switched off (per intent F6b; proposed for the founder's review).

## Theme 3 — Keep what OpenTelemetry sends
- P-094 · A span keeps its meaning — the stored span keeps its name, kind and parent, the service's version and environment, and the attributes that say which route, which status code, which database or peer it touched, under both generations of the semantic-convention names; who calls whom can be answered from what is stored (per intent F7; + `.andromeda/residuals.md:33`).
- P-095 · Metrics and logs keep theirs — a percentile can be computed from stored metrics; a log line and a metric point name their service; exception type, message and stack are readable through the door (per intent F8; + `.andromeda/residuals.md:35`).

## Theme 4 — Comparisons that work from the first minute
- P-096 · A new kind of error is news — the first appearance of an error fingerprint for a service is a finding, and so is the return of one that had gone away (per intent F9).
- P-097 · A release is compared with the one before — when a new version of a service appears, the engine compares it with the previous one (error rate, new error kinds, latency) and says when the new one is worse, naming the version and the moment it appeared (per intent F10; + `.andromeda/residuals.md:29`, nothing today knows a release happened).
- P-098 · Silence is told apart from "nothing arrived" — "the service went quiet", "no data reached the engine" and "the check could not be evaluated" are three different states, and a recovery is announced (per intent F11).

## Theme 5 — The voice
- P-099 · The engine says it, in one line and in a report — a command of the engine prints the current state in one line a desktop panel can show; a change of state raises a system notification; the notification carries a short report (what, where, since when, after which version, which errors are new) written without a model (per intent F12, ruled: the first surface is a panel module fed by an engine command, on the founder's own desktop only; + `.andromeda/residuals.md:19`, the report on an event is assembled without a model).
- P-100 · The report is enough for a large model — given the report and the door, a large model names the cause that was planted; the report is narrow and names its evidence by identifier; "not enough data" is an honest answer the engine can give (per intent F13).
- P-101 · A real service is watched too — one real service of the founder's sends its telemetry to the engine for days; what the engine reported and what it stayed silent on is read against what actually happened to that service (per intent F14, ruled 2026-10-08; which project is not yet named — the founder is asked before this capability is taken up).

## Definition of done
- P-102 · The bad-version scenario, end to end (dynamic-external) — Conductor rolls out a "bad version" of one service in its simulated world, over the network, to an engine on another host; within minutes the panel module and a system notification carry a message naming the service, the version and the new error; a large model, given the report and the door, names the planted cause; the engine did this on a 1 GB node and stayed silent through the healthy hours before it; after each theme the external harness checks that theme's result from the input to the output (per intent §7 and §6 "Order", ruled; the scenario exercises P-087, P-090, P-092, P-096, P-097, P-099 and P-100 together, on an engine that P-083…P-086 have left headless and model-free).

## Absorbed residual
- P-103 · The local pre-push check runs on Linux — the check a developer runs before a push runs natively on the Linux dev host, over the stages that survive this version's removals, with no second operating system beside it (origin `.andromeda/residuals.md:21`, carried to this version by the founder's ruling of 2026-10-09; absorbed on the operator's word of 2026-10-09 — the intent omitted it).

## Notes for the reader (not capabilities)

**The 0.3.0 capabilities P-083 retires with the window, id by id** (the intent's R1 leaves the naming to the route;
read from `andromeda-pulse-0.3.0/verification-matrix.json`, 22 entries, all `verified`; this classification is the
route's and is the founder's to correct at Phase 4):
- wholly about the window (15): P-061 · P-062 · P-063 · P-064 · P-065 · P-066 · P-068 · P-069 · P-070 · P-071 · P-076 ·
  P-078 · P-080 · P-081 · P-082.
- about the window and the local model (1): P-072 (the Investigate buttons run a model analysis) — retired by P-083
  and P-084 together.
- about the local model alone (1): P-073 (the deterministic stand-in) — retired by P-084, not by P-083.
- partly (3): P-067 (the surface that showed only live services goes; the liveness truth of the service registry
  stays engine state) · P-075 (its four delegated timing bounds P-025 / P-027 / P-037 / P-045 each end at a paint in
  the window and go with it; its telemetry → incident → read-back half continues as P-102) · P-079 (the constellation
  half goes; the single-sourced workspace key stays).
- not about either (2): P-074 (one incident for a sustained storm) and P-077 (the demo telemetry injector) — both
  must still hold under P-086's "everything 0.3.0 detected is still detected".

**One OBSERVED clause of the intent, re-read at HEAD `60ef43c`:** F1 says "`tauri` is a dependency of the ingest
and triage crates as well as of the shell". Measured: `crates/ingest/Cargo.toml` and `crates/triage/Cargo.toml`
declare `specta` (the bindings-type derive of the window's IPC bridge) behind a default-on `taurpc-runtime` feature
and do not declare `tauri`; `tauri` itself is declared by `crates/ui-bridge` (optional) and `pulse-app`. The EXPECT
is unaffected — no crate depends on the window toolkit — and the matrix's `observed_gap` for P-086 states the
measured form.

## Carried residuals (NOT 0.4.0 capabilities — deliberately unnumbered, no matrix entry)

- **Layer-2 incident identity is revisitable** (`.andromeda/residuals.md:9`, origin
  `2026-08-17-incident-fingerprint-producer-repaired`) — per-fingerprint incident de-duplication is possible but
  declined; the decision stays coalesce-per-cue-identity. Stays visible because no intent finding re-opens it: P-096
  makes a new fingerprint a finding but does not rule on incident identity. An open design question, not pending work.
- **Quiet observations** (`.andromeda/residuals.md:17`, origin `2026-10-05-l4-model-chosen-by-pattern-discrimination`) —
  the founder's ruling of 2026-10-05 that a finding with no hard cue is shown as a quiet observation, never as a red
  incident, is not withdrawn. Stays visible because the intent's §8 keeps it as a ruling for the version that surfaces
  slow findings, and §6 puts the slow patterns out of 0.4.0.
- **Stale doc comment above `select_corpus_matches`** (`.andromeda/residuals.md:23`, origin
  `2026-10-07-l4-probe-reproduces-the-canary-history-miss`) — a comment-only fix, never a chunk of its own (the
  operator's word, 2026-10-09). Stays visible because no finding matches it; it rides whichever chunk next touches
  `crates/triage/src/digest/retrieval.rs`, or goes with that code if a chunk retires it.
- **The half-hour digest's rate** (`.andromeda/residuals.md:27`, origin `2026-10-09-0-pending-version-close`) — a
  reflection digest's request rate is divided by a literal 60 over an 1800 s window. Re-carried on the operator's word
  of 2026-10-09: it stays visible until a chunk retires or repairs the assembler code that computes it (the reflection
  digest is today the local model's prompt, which P-084 retires; the half-hour timeframe is out of 0.4.0 per intent §6).
- **Stored metrics and logs feed no detector** (the unmatched remainder of `.andromeda/residuals.md:35`, whose
  storage half is absorbed as P-095) — no triage query reads `metrics_points`, and the one triage read of
  `log_records` has its rows discarded. Stays visible because P-095 expects metrics and logs to be stored with their
  meaning and readable, and no finding expects a detector over them.
