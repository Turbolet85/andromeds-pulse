# Tests validation — route draft

## Rewrite
- "Integration UX e2e test": Old: "the real assembled-product path (launch, telemetry, Traces render, storm, incident, Investigate)" → New: "incident path: L4-deterministic launch, OTLP ingest, real-time push, Traces visualization, storm, incident, MCP Investigate".
  Reason: real-time push (P6 critical path) and the deterministic L4 mode must be explicit for incident-path verification per test-plan §6 / intent F15.
