# obs-plan.md ## 3. Observability Harness Contract — the keyed contracts, one row per key (U35): key · file · bytes · labels. Read one file whole when the chunk turns on its key; never the set.

- Tracing init · .andromeda/registries/contracts/obs-plan/tracing-init.md · 2107 B · labels: Crate set, Init order, Init body sketch (≤ 5 lines), Why this approach
- Service identity · .andromeda/registries/contracts/obs-plan/service-identity.md · 780 B · labels: service.name, service.version, deployment.environment, Default subscriber fields
- Logging stack · .andromeda/registries/contracts/obs-plan/logging-stack.md · 1692 B · labels: Library, Format, Sink, Frontend bridge
- Log format JSON schema · .andromeda/registries/contracts/obs-plan/log-format-json-schema.md · 1103 B
- Log file location · .andromeda/registries/contracts/obs-plan/log-file-location.md · 655 B · labels: Path, Rotation
- Snapshot / paste-to-AI integration · .andromeda/registries/contracts/obs-plan/snapshot-paste-to-ai-integration.md · 2399 B · labels: External-OTLP snapshot, Path, Schema, Trigger, MCP tools (hand-rolled JSON-RPC 2.0 over stdio; rmcp 3.x declared-but-unused), Self-observation paste-to-AI, Format, No separate snapshot needed
- Trace context propagation · .andromeda/registries/contracts/obs-plan/trace-context-propagation.md · 1373 B · labels: HTTP boundaries (`:4318` OTLP/HTTP receiver on axum 0.8), gRPC boundaries (`:4317` OTLP/gRPC receiver on tonic 0.14), IPC boundaries (TauRPC), Internal async boundaries, Real-time push streams
- Heartbeat ticks · .andromeda/registries/contracts/obs-plan/heartbeat-ticks.md · 1891 B · labels: Tick interval, Tick event format, Stall detection (two signals — liveness AND progress), Complementarity with the `health` command
