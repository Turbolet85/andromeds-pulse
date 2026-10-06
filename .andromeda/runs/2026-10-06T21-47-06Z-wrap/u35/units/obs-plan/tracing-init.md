### Tracing init

- **Crate set:** `tracing` 0.1 (macro-driven instrumentation; `#[instrument]` attribute on router/handler methods) + `tracing-subscriber` 0.3 (JSON formatter + Layer composition + `EnvFilter`) + `tracing-appender` 0.2 (daily-rolling file sink, non-blocking writer) + `tracing-error` 0.2 (SpanTrace for sanitized error chains)
- **Init order:** Before Tauri app spawn; before HTTP/gRPC server bind. Sequence: (1) Load env vars + config → validate log level via `EnvFilter::from_env("ANDROMEDA_PULSE_LOG_LEVEL")` with fallback `RUST_LOG`, (2) Build the `tracing_subscriber::registry()` composing the non-blocking file appender layer (no stderr layer), (3) Install panic hook calling `tracing::error!(target: "app.panic.fatal", ...)` with `tracing-error` SpanTrace, (4) Spawn Tauri app + bind OTLP receivers (tonic + axum). Between (3) and (4) `main` hands the `WorkerGuard` `init` returns to a process-wide slot (`hold_log_guard`), installs the at-exit hook (`install_exit_hook`: a `pulse-exit-reporter` thread + `libc::atexit`) and, on Unix, the SIGTERM/SIGINT listener (`install_signal_listener`). The app runs through `App::run_return`; its exit code goes to `exit_after_event_loop` (record `app.exit` → `flush_log_sink` drops the guard → `std::process::exit` with the SAME code). `WorkerGuard::drop` is the non-blocking file sink's only drain, so every exit path that can log drains it after its one record (§6 `app.exit`).
- **Init body sketch (≤ 5 lines):**
```rust
let (file_writer, _guard) = tracing_appender::non_blocking(rolling::daily(log_dir, "agent-latest.jsonl"));
tracing_subscriber::registry()
  .with(fmt::layer().json().with_writer(file_writer))
  .with(EnvFilter::from_env("ANDROMEDA_PULSE_LOG_LEVEL"))
  .with(ErrorLayer::default()).init();
std::panic::set_hook(Box::new(panic_to_tracing_error));
```
- **Why this approach:** by NOT linking an OTel SDK into the self-runtime, the "OTel monitoring an OTel monitor" recursion concern disappears by construction — there is no exporter to point anywhere, and the file-sink JSON line IS the agent-readable surface.
