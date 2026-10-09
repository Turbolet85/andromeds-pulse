# Obs validation — route draft

## Insert
- Between "Tier1 incident-path reliability" and "Investigate actions functional" (Epoch 1): **"Heartbeat ticks for incident subsystems — queue depth and coalesce signal count emitted per tick cycle"**.
  Reason: per obs-plan §Heartbeat (Standard tier), long-running subsystems must emit 15s ticks for >45s stall detection; the new incident queue/coalesce are Tier1-critical and require this observability.

(Other obs bootstrap — structured log sink, service identity, trace propagation, error reporting, PII scrubbing, heartbeat for ingest/buffer/viz/plugins — already landed in v0.2.0; OTel SDK init N/A, project uses tracing 0.1 self-observation.)
