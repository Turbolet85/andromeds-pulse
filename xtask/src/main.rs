use std::process::ExitCode;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "xtask", about = "andromeda-pulse task runner")]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    #[command(name = "harness:status")]
    HarnessStatus,
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Cmd::HarnessStatus => match harness_status().await {
            Ok(code) => code,
            Err(e) => {
                eprintln!("xtask error: {e:?}");
                ExitCode::FAILURE
            }
        },
    }
}

async fn harness_status() -> Result<ExitCode> {
    // ui-bridge built without taurpc-runtime feature here — xtask is non-IPC
    // and Tauri runtime DLLs aren't available on Windows without WebView2.
    // current_health() returns the same envelope that the TauRPC resolver
    // would emit; full IPC roundtrip lands at the integration-test chunk
    // when actual subsystems (chunks #15-#18) exist to hit.
    let envelope = ui_bridge::health::current_health();

    let json = serde_json::to_string_pretty(&envelope)?;
    println!("{json}");

    let exit = match envelope.status {
        ui_bridge::health::HealthStatus::Ok => ExitCode::SUCCESS,
        ui_bridge::health::HealthStatus::Degraded => ExitCode::FAILURE,
    };
    Ok(exit)
}
