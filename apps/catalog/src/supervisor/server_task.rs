use crate::http;
use foxtive::helpers::env;
use foxtive::results::AppResult;
use foxtive_axum::server::{Server, StaticFileConfig};
use foxtive_supervisor::contracts::SupervisedTask;
use tokio::sync::broadcast;
use tracing::{info, warn};
use domain::APP_CODE;
use domain::http::server_shared::{allowed_headers, allowed_methods, allowed_origins};

pub struct ServerTask {
    shutdown_tx: broadcast::Sender<()>,
}

impl ServerTask {
    pub fn create() -> ServerTask {
        let (shutdown_tx, _) = broadcast::channel(1);
        ServerTask { shutdown_tx }
    }
}

#[foxtive::async_trait]
impl SupervisedTask for ServerTask {
    fn id(&self) -> &'static str {
        "server-task"
    }

    fn name(&self) -> String {
        "Server Task".to_string()
    }

    async fn run(&self) -> AppResult<()> {
        let env_prefix = APP_CODE;
        // Create a shutdown receiver for this run
        let mut shutdown_rx = self.shutdown_tx.subscribe();

        let host = env::var(env_prefix, "SERVER_HOST")?;
        let port: u16 = env::var(env_prefix, "SERVER_PORT")?.parse()?;

        info!("Starting server at {}:{}", host, port);
        let foxtive_setup = domain::setup::foxtive::setup(env_prefix).await?;

        Server::new(foxtive_setup)
            .host(host)
            .port(port)
            .router(http::kernel::boot())
            .has_started_bootstrap(true)
            .template_directory("resources/templates/**/*.html")
            .static_config(StaticFileConfig {
                dir: "resources/static".to_string(),
                path: "/static".to_string(),
            })
            .allowed_origins(allowed_origins())
            .allowed_headers(allowed_headers())
            .allowed_methods(allowed_methods())
            .bootstrap(domain::setup::finish_setup)
            .on_started(async { info!("Server started successfully") })
            .shutdown_signal(async move {
                shutdown_rx.recv().await.ok();
                info!("Axum server received shutdown signal");
            })
            .run()
            .await
    }

    async fn should_restart(&self, _attempt: usize, error: &str) -> bool {
        // Don't restart if port is in use
        !error.contains("address already in use")
            && !error.contains("trace dispatcher has already been set")
            && !error.contains("assertion failed: path.starts_with('/')")
    }

    async fn on_shutdown(&self) {
        warn!("Shutting down Axum server...");
        // Send shutdown signal to axum
        let _ = self.shutdown_tx.send(());
    }
}
