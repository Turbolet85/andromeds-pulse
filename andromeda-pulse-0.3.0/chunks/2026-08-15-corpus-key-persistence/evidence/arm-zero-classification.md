# Conductor arm-zero classification — RUN 2026-08-15

**Status:** RUN (operator's headful window). Discharges the criterion recorded as unmet at
`2026-08-14-workspace-key-alignment` and carried as the FIRST STEP of this chunk.

Executed **pre-fix in intent** (the classification is keyring-independent — it reads log-side counters
that need no decryption) and, as it happened, on the post-fix binaries. That does not weaken it: the
discriminator is the `buffer.tick` feed counter triple, which the key fix does not touch. Where the fix
DID change the picture is recorded separately in §4.

## 1. Conditions

| | |
|---|---|
| Instrument | `D:\dev\projects\conductor` — `target/debug/conductor.exe preflight --json`, run from Conductor's own repo root |
| SUT | `target/debug/pulse-app.exe`, launched from the andromeda-pulse repo root (OUTSIDE Conductor's tree) |
| Data dir | `<scratchpad>/armzero-datadir` — one dir shared by app and sidecar |
| Sidecar | `target/debug/andromeda-pulse-mcp.exe`, resolved from `PATH`, spawned by Conductor |
| Env gates | `ANDROMEDA_PULSE_DATA_DIR` (propagated) · `ANDROMEDA_PULSE_MCP_ENABLED=true` · `ANDROMEDA_PULSE_L4_DETERMINISTIC=true` (P-073) |
| Published key | `{data_dir}/run/workspace-key` = `\\?\D:\dev\projects\andromeda-pulse` (the detected root, not the data dir) |

**Conductor's tree was not written to by this capture.** Conductor persists its own run journal under its
own `runs/` by design (`CONDUCTOR_RUNS_DIR` is traversal-guarded to its repo root, so it cannot be
redirected outward); that is the instrument recording itself. No file in Conductor's tree was authored
or modified here, and every artifact of this capture lives in this folder.

## 2. Preflight progression (three attempts, each unblocking one precondition)

1. `env_disabled` — the sidecar's runtime half of the MCP double-gate was unset. Declared
   `ANDROMEDA_PULSE_MCP_ENABLED=true`.
2. `unmet run-contract terms: [l4-deterministic]` — Conductor reads only its OWN environment and cannot
   inspect a process it did not launch, so the term must be declared on its side too. Declared it.
3. **Reached the canary gate**: protocol `2024-11-05` negotiated, all four required tools `present`,
   `canary_round_trip: "failed"`.

## 3. THE CLASSIFICATION — the `buffer.tick` discriminator

Read from the SUT's own log (`{data_dir}/logs/agent-latest.jsonl.2026-08-15`), 12 ticks:

```
span_events_seen: 6   fingerprints_computed: 6   observer_invocations: 6
```

Against the recipe's table this is the **fourth row**:

| Reading | Class |
|---|---|
| `span_events_seen = 0` | nothing reached the appender |
| `fingerprints_computed = 0` with `span_events_seen > 0` | attributes lost in ingest |
| `observer_invocations = 0` with `fingerprints_computed > 0` | wiring |
| **all three non-zero** | **the feed is healthy; the arm's zero came from elsewhere** ← THIS |

**Verdict: the fingerprint feed is NOT the arm's zero.** All six of Conductor's canary span-events
reached the appender, all six were fingerprinted, all six reached the observer — a 1:1:1 ratio with no
loss at any hop. This independently re-confirms the `2026-08-14-fingerprint-feed-capture-repair`
finding on a different driver (Conductor's canary rather than `inject_demo`).

## 4. Where the zero actually originates (narrowing past the discriminator)

Measured facts from the same run, in pipeline order:

| Stage | Observation | Source |
|---|---|---|
| Feed | 6 / 6 / 6 | `buffer.tick` |
| Storm detector | `storms_detected_total` 0 → **1** | `triage.pattern.storm.tick` @ 16:32:07.397Z |
| Cue emission | **0** — no `triage.cue.emit` in the entire run | log grep (`triage.cue.evaluate` ran 192×, `triage.cue.tick` 192×) |
| Incident rows | **0** in `incidents`, 0 in `incident_events` | independent read-only sqlite3 of `corpus.db` (plaintext columns; no key needed) |
| MCP read-back | `tool dispatch ok`, `{"items": [], "total": 0}` | direct JSON-RPC `query_incident_list` against the live sidecar |

**The zero originates at or before cue emission** — a storm IS detected and nothing is emitted
downstream of it, so no incident is created, nothing is persisted, and the read-back's zero is
TRUTHFUL rather than a read failure.

Stated conservatively: `triage.cue.emit` is the chunk-#62 cue target, and the 2026-06-28 Tier-1 work
introduced a separate internal carrier (`DigestTriggerBroadcast`) for the storm→digest path, so the
absence of that one target does not by itself prove no cue was carried. What IS proven without
inference: **a storm was detected and zero incident rows exist.** Locating the exact link belongs to
the route entries that own it, not to this chunk.

## 5. What THIS chunk's fix changed — the read-back failure mode

The comparison that matters, against the same tool call in the predecessor chunk's evidence:

| Capture | `query_incident_list` result |
|---|---|
| `2026-08-14-workspace-key-alignment` §after-fix | `error: "tool dispatch failed for query_incident_list: decryption failed"` |
| **this capture (post key-persistence fix)** | **`tool dispatch ok` — `{"items": [], "total": 0}`** |

The decryption barrier is gone. The read-back now completes and reports the corpus's real content. That
is the end-to-end confirmation of this chunk's fix through the Conductor path, and it also means the
`total: 0` here is a genuine downstream finding rather than an artifact of an unreadable corpus.

## 6. Collateral observation — muted diagnostics

Three fields that would have made §4 immediate are redacted by the default-deny allowlist:
`incidents.list_active.request → item_count`, `triage.incident.persist → incident_count` /
`persist_kind`, and (seen in the two-boot smoke) `triage.incident.corpus_restore → kind` /
`restored_incident_count`. Same muted-diagnostic class this chunk repaired for `corpus.open.error`,
on `triage.*` / `incidents.*` targets whose intended field sets are not this chunk's to choose.
Handed to the wrap.

## 7. Result disposition

A non-reproducing arm is a result, not a skip — recorded verbatim. The arm did not reproduce a feed
zero; it produced a **healthy feed with a downstream zero**, and the classification names that class.
