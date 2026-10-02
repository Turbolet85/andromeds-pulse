# Conductor arm-zero classification — NOT YET RUN (operator-gated)

**Status:** pending the operator's headful half. Recorded rather than silently skipped, per the plan's
`(evidence)` criterion and test-plan §6 P5 documented-residual shape.

## Why it did not run in this /implement session

The leg needs an external instrument driven from another repo plus a headful launch the agent cannot
schedule on its own. The phase directive is explicit: *"the leg needs the operator's headful half —
schedule it with them."*

## The recipe, ready to run

Conductor is the **instrument**, not a session:

1. Run Conductor's preflight binary **from its own repo root** (`D:\dev\projects\conductor`).
2. Launch `pulse-app` from **outside** that repo.
3. Propagate `ANDROMEDA_PULSE_DATA_DIR` to the sidecar (Conductor already mandates this — its
   `architecture.md:174`).
4. Land **all** output in this folder. **Never write into Conductor's tree.**

## What the counters will say on sight

The `buffer.tick` feed counters landed by `2026-08-14-fingerprint-feed-capture-repair` discriminate the
failure class without further investigation:

| Reading | Class |
|---|---|
| `span_events_seen = 0` | nothing reached the appender |
| `fingerprints_computed = 0` with `span_events_seen > 0` | attributes lost in ingest |
| `observer_invocations = 0` with `fingerprints_computed > 0` | wiring |
| all three non-zero | the feed is healthy; the arm's zero came from elsewhere |

A **non-reproducing arm is a result**, not a skip — record it verbatim if the arm now reads non-zero.

## Two findings from this chunk that change what the re-run will show

1. **The workspace-key mismatch is fixed.** Conductor's arm previously could not see incidents through
   `query_incident_list` because the sidecar filtered by `data_dir` while the app stamped the detected
   project root. The app now publishes its key to `{data_dir}/run/workspace-key` and the sidecar reads
   it.

2. **A second, independent defect still blocks the read-back** — the corpus encryption key does not
   survive a process boundary (`keyring 3.6.3` resolved with no platform credential-store backend, so
   its non-persisting mock store is in use). Until that is fixed, **any** cross-process incident read
   fails with `decryption failed`, for every workspace key. See
   `after-query-incident-list.json` §blocked_by. Expect the arm to reach decryption and fail there
   rather than returning an empty list — which is itself the confirmation that piece 1 landed.
