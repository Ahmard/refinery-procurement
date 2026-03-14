use chrono::{SecondsFormat, Utc};
use foxtive::results::AppResult;
use foxtive_supervisor::Supervisor;
use tracing::{error, info};
use domain::{state, APP_CODE};
use domain::setup::rabbitmq;
use crate::supervisor::rabbitmq_consumer_task::RabbitmqConsumerTask;

mod supervisor;

#[tokio::main]
async fn main() -> AppResult<()> {
    println!(
        "\n  {}  [BOOT] Setting up environment...\n",
        Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true)
    );

    setup().await?;

    info!("[BOOT] Environment setup complete.");
    info!("[BOOT] Starting application...");
    let mut supervisor = Supervisor::new()
        .add(RabbitmqConsumerTask).start().await?;

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

async fn setup() -> AppResult<()> {
    domain::setup::init_setup("worker")?;
    let fx_setup = domain::setup::foxtive::setup(APP_CODE).await?;
    foxtive::setup::make_state(fx_setup).await?;
    state::boot(APP_CODE)?;
    rabbitmq::setup().await?;

    Ok(())
}