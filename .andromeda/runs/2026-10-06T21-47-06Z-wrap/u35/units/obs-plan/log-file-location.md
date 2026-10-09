### Log file location

- **Path:** `<data_dir>/logs/agent-latest.jsonl` — `<data_dir>` from `resolve_data_dir()`: `ANDROMEDA_PULSE_DATA_DIR` when set, else `%APPDATA%\andromeda-pulse\` (Windows), `~/Library/Application Support/com.andromeda.pulse/` (macOS), `$XDG_CONFIG_HOME/andromeda-pulse/` else `~/.andromeda-pulse/` (Linux) — per arch §Filesystem locations (Windows + Linux halves corrected to `resolve_data_dir()` at chunk 2026-08-30-agent-harness-teardown-truth; macOS unchanged)
- **Rotation:** Daily rotation via `tracing_appender::rolling::daily(log_dir, "agent-latest.jsonl")`; previous day's file read by agent if current not yet created
