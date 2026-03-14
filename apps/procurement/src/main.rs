use crate::supervisor::server_task::ServerTask;
use chrono::{SecondsFormat, Utc};
use foxtive::results::AppResult;
use foxtive_supervisor::Supervisor;
use tracing::{error, info};

mod http;
mod supervisor;

#[tokio::main]
async fn main() -> AppResult<()> {
    println!(
        "\n  {}  [BOOT] Setting up environment...\n",
        Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true)
    );

    domain::setup::init_setup("procurement")?;

    info!("[BOOT] Environment setup complete.");
    info!("[BOOT] Starting application...");
    let mut supervisor = Supervisor::new().add(ServerTask::create()).start().await?;

    // Wait for SIGTERM or SIGINT
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Shutting down gracefully...");
            supervisor.shutdown().await;
        }
        result = supervisor.wait_any() => {
            error!(
                "Critical task '{}' terminated unexpectedly: {:?}",
                result.task_name,
                result.final_status
            );
            // Shutdown remaining tasks
            supervisor.shutdown().await;
            std::process::exit(1);
        }
    }

    Ok(())
}
