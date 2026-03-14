use foxtive::FOXTIVE;
use foxtive::prelude::{AppResult, AppStateExt, RabbitMQ};
use foxtive::rabbitmq::{ExchangeKind, QueueDeclareOptions};
use tracing::info;
use crate::APP;
use crate::ext::LocalAppStateExt;

pub async fn setup() -> AppResult<()> {
    FOXTIVE
        .rabbitmq()
        .lock()
        .await
        .setup_fn(|r| Box::pin(rabbitmq_setup_function(r)))
        .await;

    Ok(())
}

pub async fn rabbitmq_setup_function(mut rabbitmq: RabbitMQ) -> AppResult<()> {
    info!("preparing rabbitmq messaging...");

    // EXCHANGES
    rabbitmq
        .declare_exchange(&APP.state().rmq_exchange_app, ExchangeKind::Topic)
        .await?;

    // QUEUES
    rabbitmq
        .declare_queue(
            &APP.state().rmq_queue_app,
            QueueDeclareOptions {
                passive: false,
                durable: true, // Ensures queue persists
                exclusive: false,
                auto_delete: false,
                nowait: false,
            },
            Default::default(),
        )
        .await?;

    // BINDINGS
    rabbitmq
        .bind_queue(
            &APP.state().rmq_queue_app,
            &APP.state().rmq_exchange_app,
            "proc.#",
            Default::default(),
            Default::default(),
        )
        .await?;

    info!("rabbitmq messaging prepared");
    Ok(())
}