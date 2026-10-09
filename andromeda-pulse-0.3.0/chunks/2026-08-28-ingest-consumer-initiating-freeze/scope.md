# Scope — Ingest consumer initiating freeze

**Marker:** `2026-08-28-ingest-consumer-initiating-freeze`
**Version:** `andromeda-pulse-0.3.0` · **Epoch 4 — Polish & ship: verification**
**Working entry (verbatim outcome):** what stops the consumer under light load is identified, so the wedge
has a named cause and not only a survivable aftermath.

---

## 1. What this chunk is

A **forensic attribution** chunk, not a feature chunk. Two predecessors deliberately stopped short of this
and said so rather than implying coverage:

- `2026-08-26-ingest-consumer-stall-under-sustained-load` shipped the drain-progress observable
  (`rows_ingested_delta` · `last_append_age_seconds` · the `buffer.consumer.stalled` transition WARN ·
  `cargo xtask check:ingest-progress`) and **falsified its own prime hypothesis** — viz shared-connection
  contention is NOT the cause (a matched 15-minute RED leg at HEAD reproduced every stated precondition and
  stayed healthy at 47,898 rows with the ingest channel at 0.0 %).
- `2026-08-26-cadence-runaway-blocking-pool` bounded the **amplifier** (the cue→cadence fan-out that made
  recovery impossible) and states explicitly that the **initiating event stays unattributed**.

So the survivable-aftermath half is DONE. What is missing is the trigger: the single event at
**17:20:50.787** after which the consumer's `duckdb.append` never fired again.

## 2. Outcome this chunk owes

**The initiating freeze has a NAMED cause, stated with the evidence that supports it and the limits of that
evidence.** A named cause is the deliverable; a code fix is conditional on what the attribution finds
(§4).

Three honest terminal outcomes, all acceptable, in preference order:

1. **Attributed + fixed** — the mechanism is identified from the preserved record and/or the code, AND the
   fix is proportionate and pinnable in this chunk.
2. **Attributed, fix owned elsewhere** — the mechanism is identified but the repair is larger than this
   chunk (e.g. a runtime-configuration or architectural change); the cause is recorded and the fix gets its
   own route entry.
3. **Not attributable from the preserved record** — the forensic pass is exhausted without a mechanism that
   the evidence actually supports. This is a REPORTED result, not a failure: it must state what was ruled
   OUT, what evidence would settle it, and confirm the armed detector will capture that evidence on the next
   occurrence.
4. **Attributed, and ALREADY FIXED upstream** — the cause is identified and a landed predecessor already
   removes it, so the chunk's honest work is VERIFICATION (prove the mechanism, prove the existing bound
   closes it, pin it against regression) rather than repair.
   **[premise-corrected: outcomes 1–3 were authored at promotion as the full ladder; P3's timeline
   reconstruction made outcome 4 the leading candidate — the runaway that `2026-08-26-cadence-runaway-
   blocking-pool` bounded had been at full tier1 rate for ~55 s when the stall began, so the bound may
   already close the initiating freeze. Outcome 3 is correspondingly much less likely. This is a
   plan-decision blocker resolved with the operator before P4 synthesis.]**

**Explicitly NOT owed: a live reproduction.** The entry records that it does not reproduce on demand, and
its NOTE ratifies the wait-for-the-detector posture. A chunk that spends its budget trying to re-trigger the
wedge is working against its own stated placement rationale.

## 3. The evidence base (all coordinates re-verified first-hand at promotion)

- **Durable wedge log** — `D:/dev/evidence/pulse-l4run-171923/logs/agent-latest.jsonl.2026-08-25`,
  **131,324,444 bytes**. Path and size confirmed at promotion. The temp-scratchpad copy is NOT durable and
  must not be cited.
- **Signatures in that log** (as recorded by the predecessor): 2,683 `cadence.trigger` ·
  4,599 `triage.cue.emit` · 26,821 `metric.pipeline.l1a.query_count_total`. `[inferred]` — carried from the
  entry, re-derivable but NOT yet re-derived first-hand; P3 re-derives before any of them shapes a
  conclusion (per the verify-at-HEAD discipline: a relayed count is itself a coordinate).
- ~~**The pre-freeze window is LIGHT** — ~1.7 blocking queries/sec: 11 cadence cycles and 110 L1a queries
  across the 66 seconds before the freeze. The cue storm FOLLOWED the freeze by 32 seconds, which is what
  makes the runaway an amplifier and never the trigger.~~
  **[premise-corrected: 17:20:50 is the end of injector run 1, not the freeze — `ingest.tick.span_count`
  is flat at 2187 with `buffer_capacity_pct` 0.0 for 13 ticks (17:21:01–17:24:01), so the consumer had
  nothing to consume. The producer RESUMES at 17:24:16 and the channel then climbs 0→100 % while
  `rows_ingested` never leaves 2187 — that is the actual stall onset. Tier1 `service_went_silent` cadence
  triggers began 17:23:21, ~55 s BEFORE it. So load at freeze onset was HEAVY, the storm PRECEDED the
  stall, and the runaway is a live TRIGGER candidate rather than only an amplifier.]**
- **The two deaths** — both `spawn_blocking` users died ~3.5 minutes apart on **different mutexes**
  (`duckdb.append` 17:20:50, `viz.query.traces` 17:24:13) while every async task ran 16 minutes longer.
  Max append duration was 11 ms in both the wedge run and the healthy control, so DuckDB contention is
  already excluded.
- **A second durable record** — `D:/dev/evidence/pulse-l4run-20260827-064312/` (the 11h08m idle-burn arc).
  `[inferred]` — not named by the entry; noted as an available comparison corpus, since a HEALTHY long run
  is what makes a wedge signature discriminating rather than merely present.

## 4. Research direction the entry mandates

> *"Research should start from what BOTH `spawn_blocking` users dying 3.5 minutes apart on DIFFERENT mutexes
> implies about the shared blocking pool."*

Two users dying on different locks excludes any single-mutex explanation and points at the resource they
actually share: the tokio **blocking pool**. P3 must establish, first-hand:

- the real pool bound at HEAD (the entry says "512-thread default" — `[inferred]`, to be read off
  `pulse-app/src/main.rs`'s runtime builder, not assumed);
- every production `spawn_blocking` call site and what each holds while it runs;
- ~~**whether any path can leak a blocking thread permanently** — a `timeout` wrapped around `spawn_blocking`
  ABANDONS the future but cannot cancel the blocking closure, so an abandoned task keeps its thread AND its
  lock forever. `crates/triage/src/baseline/sql.rs:441` already carries a comment naming exactly this
  hazard.~~
  **[premise-corrected: that comment DOCUMENTS A LANDED REPAIR, not an open hazard —
  `run_q7_with_timeout` runs the primary on a dedicated `try_clone()`d connection and calls
  `interrupt_handle().interrupt()` on the timeout arm (`sql.rs:462`, `:482`). It is also the ONLY production
  `timeout`-over-`spawn_blocking` site in the workspace (every other `timeout(` hit is `#[cfg(test)]` or the
  llamacli subprocess guard), so this mechanism class is closed. Two facts additionally exclude the simplest
  alternatives: `buffer.retention.sweep` succeeded at 17:21:25 and 17:23:05 on the SHARED appender
  connection, so neither that mutex nor the blocking pool was exhausted then.]**

The discipline that governs the whole pass: **a mechanism is named only when the preserved evidence supports
it.** Two predecessors already had a prime hypothesis measure FALSE; the correct move on a falsified premise
here is a report entry, not a strained fit.

## 5. Boundaries

**IN scope**
- Forensic analysis of the preserved wedge log + the healthy comparison run.
- Code-graph / source analysis of blocking-pool usage and lifetime, in `crates/buffer`, `crates/triage`,
  `crates/viz` and `pulse-app`.
- A proportionate fix **iff** §2 outcome 1 holds — with a discriminating pin (a test that is RED without it).
- Recording what was ruled OUT, so the next occurrence starts from a narrower field.

**OUT of scope**
- Re-litigating or weakening the amplifier bound (`CueLatch`) or the drain-progress observable — both
  landed and are pinned; this chunk consumes them, never edits their semantics.
- Chasing a live reproduction of the wedge (§2).
- Any change to the ingest/OTLP receiving path itself — the receivers kept ACCEPTING throughout (27,675
  spans accepted and never persisted); the defect is on the consumer side of the channel.
- The `App-registry reconciliation` residual, which is the next entry and has its own owner.

## 6. Surfaces / contracts this chunk may touch

- **Observability** — any new field or target follows the exact-leaf discipline; a bare `buffer` /
  `interpretation` key resolves to a POPULATED sibling set and silently redacts every field, so a new target
  needs its OWN exact allowlist leaf plus a field-set pin. No target is un-muted or narrowed without an
  amendment.
- **`crates/buffer` consumer + retention** · **`crates/triage` baseline SQL** · **`crates/viz` query fns**
  (already isolated onto their own connections since the predecessor) · **`pulse-app` runtime builder**.
- **No new TauRPC procedure is anticipated**; if one becomes necessary it carries the full pin set
  (router registration + `EXPECTED_PROCEDURES` + the `emit_taurpc_bindings` merge). `[inferred]`
- **No DDL, no new corpus table, no new crate edge** anticipated. `[inferred]`

## 7. Folded annotations

**PREREQ — `cargo audit` standing deferral (pin #18).**
Origin `2026-08-15-corpus-key-persistence`; ratified at the `2026-08-16-fault-identity-semantics-decided`
wrap; re-pinned here from `2026-08-27-incident-persist-vs-resolve-write-race`, origin preserved.
- **Basis:** the upstream RustSec DB fails to load (`duplicate advisory ID: RUSTSEC-2026-0244`) — external
  decay, re-verified unchanged at the last wrap.
- **Overlap signal:** `cargo deny check advisories`, designed-red at the same **8 owned DISTINCT ids**
  0189/0190/0194/0195/0204/0222/0253/0258, with `bans licenses sources` exit 0 — seventh consecutive
  identical result.
- **Interval:** session 49 ran the probe full-form; sessions 50 and 51 were between-points. **Next interval
  point: 52.** `state.yaml` reads `session_count: 51`, so **this chunk's wrap is session 52 and owes the
  FULL-form probe** — not another recorded skip. Enumerate DISTINCT `RUSTSEC-` ids, never error BLOCKS.
- The pin rides this route entry; re-derive its number from the route line at wrap, never from memory.

**NOTE (placement).** Operator-directed 2026-08-26: a one-observation mystery whose detector is armed can
wait for that detector to fire again. This ratifies outcome 3 of §2 as legitimate — the chunk is not
obligated to manufacture a cause.
