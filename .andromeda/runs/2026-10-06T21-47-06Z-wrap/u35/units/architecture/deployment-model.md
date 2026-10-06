**Deployment model.**
- Public OSS desktop app distributed through GitHub Releases; "deployment" is the release pipeline, not server hosting.
- No Docker, no Kubernetes, no serverless, no docker-compose.
- Runtime topology on the user's machine: one Tauri process hosting all fourteen library crates and the embedded webview; one optional `andromeda-pulse-mcp` sidecar process (only when `--features mcp-server` is enabled at build time and `ANDROMEDA_PULSE_MCP_ENABLED=true` at runtime).
