# Obs validation — route draft

## Insert
- Between `Headless engine entry point` and `Engine end-to-end gate reachable`: **"Engine's own log from first boot — structured, scrubbed log file with service identity, heartbeat ticks, and a record of every panic and process end"** (epoch: `Epoch 1`)
  Reason: Per obs-plan §3 Tracing init / Service identity / Heartbeat ticks and §7 Panic hook + Process-end cause, the sink drain and the `app.exit` record run through the Tauri event-loop exit in `pulse-app/src/main.rs`, so a console host has neither until a chunk states it. §10 CI gates grade this log, so it must precede the engine gate.

- Between `Engine end-to-end gate reachable` and `Windows and macOS CI legs retired; pre-push check native on Linux`: **"Engine gate's log kept and graded — the CI run's engine log uploaded; panic, heartbeat-gap and budget checks read it"** (epoch: `Epoch 1`)
  Reason: Per obs-plan §9 Telemetry artifact handling and §10 CI gates, the log those checks read today is the boot-smoke job's (`logs-boot-*`), which `Window's gates retired` removes in Epoch 2, so its replacement producer must exist first.

- Between `Receiver refusals and per-sender bounds` and `Token lifecycle by engine command`: **"Refusals visible without flooding the log — refused and slowed senders readable as counts in the engine's log; a refusal flood cannot grow it without bound"** (epoch: `Epoch 3`)
  Reason: Per obs-plan §11 Logs (no hot-path logging) and the §5 tick-aggregated counter convention, the receiver's rejection sites emit one WARN per refused request (`crates/ingest/src/http.rs`, `grpc.rs`), which on a network-reachable receiver makes log volume sender-controlled.

- Between `Checks reason by event time` and `Recorded stream replays to the same result`: **"Engine's own liveness stays on the process clock — heartbeat ticks and the stall signal hold under silence and under a replayed stream's old stamps"** (epoch: `Epoch 3`)
  Reason: Per obs-plan §3 Heartbeat ticks (liveness AND progress) and §10 Standard+ invariants, stall detection is a missing-tick gap on the process clock, and `cadence.tick` rides the cadence loop this chunk moves to event time, which does not advance on a quiet or replayed stream.

- Between `Telemetry store on disk` and `Learned state survives a restart`: **"Disk store's size and drain progress in the engine's log — bytes on disk, rows kept and evicted, and the stalled-consumer signal, each read true"** (epoch: `Epoch 4`)
  Reason: Per obs-plan §1 Telemetry triggers (perf-budget-instruments, buffer retention) and the §10 drain-progress invariant, the buffer gauge and `buffer.consumer.stalled` are defined against the in-memory ring this chunk replaces. `Node size measured` replaces only the process-memory half of the §10 "Buffer memory bounded" row.

## Rewrite
- `Real service watched for days`: "reports and silences read against what happened" → "reports, silences and the engine's own log read against what happened"
  Reason: Per obs-plan §10 always-required invariant (zero unlogged panics), Error budget and the >45 s heartbeat stall rule, the days-long run is the version's only window long enough to grade them, and no Polish chunk names the engine's own log.
