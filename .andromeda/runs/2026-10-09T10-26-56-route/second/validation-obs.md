# Obs validation — route draft

## Rewrite
- `Console engine entry point`: "log, heartbeat, process-end record kept" → "log, service identity, panic record, heartbeat, process-end record kept"
  Reason: per obs-plan §3 keys `tracing-init` (init order: panic hook after the sink) and `service-identity`, both live in the entry point this chunk replaces. The plan sources identity from the Tauri bundle identifier, which leaves in Epoch 2.

- `Engine end-to-end gate reachable`: "recorded green" → "engine log kept as a run artifact; recorded green"
  Reason: per obs-plan §9 Telemetry artifact handling, the `logs-boot` artifact (the log `ci-gates` grades) rides the boot job that "Window's gates retired" removes. The gate replacing it must keep the engine's log downloadable.

- `Window retired`: "design, layout and a11y masters state no interface" → "design, layout and a11y masters state no interface; critical path P5 recorded retired"
  Reason: per obs-plan §1 Critical paths and §4 scenario P5, its must-trace spans leave with the window and tray. The draft records only P4's retirement.

- `Display-only computation retired`: "registry liveness stays" → "registry liveness and receiver-entry trace context stay; critical path P6 recorded retired"
  Reason: per obs-plan §3 key `trace-context-propagation`, only the IPC-envelope and push-stream halves leave with the display (§4 scenario P6). Receiver-entry extraction remains a §10 Standard+ invariant.

- `Workspace detection retired`: "incidents told apart by check and service" → "incidents told apart by check and service; critical path P7 recorded retired"
  Reason: per obs-plan §1 Critical paths and §4 scenario P7, the `app.boot.workspace.*` span chain and the `app.boot.workspace_key` record leave with the detector.

- `Receiver refusals and per-sender bounds`: "token never an incident key" → "token never an incident key; a refusal flood cannot grow the log without bound"
  Reason: per obs-plan §11 Logs (no per-event records on a hot path) and the §5 tick-aggregated counter convention (`append_rejections` precedent). On a network-reachable receiver, one record per refusal lets a remote sender set the log's size.

- `Telemetry store on disk`: "replaces the in-memory buffer" → "replaces the in-memory buffer, keeping its tick, eviction count and stalled-consumer signal"
  Reason: per obs-plan §3 key `heartbeat-ticks` (liveness AND progress) and §10 Standard+ invariants, `buffer.tick` and `buffer.consumer.stalled` are sited on the buffer this chunk replaces. The later "Disk store measured under load" reads them.

- `Real service watched for days`: "reports and silences read against what happened" → "reports and silences read against what happened; engine's own log read for panics, tick gaps and stalls"
  Reason: per obs-plan §10 Error budget (zero unlogged panics, measured by parsing the log) and the heartbeat and drain-progress invariants. This is the route's only multi-day run, the span of time that budget is defined over.
