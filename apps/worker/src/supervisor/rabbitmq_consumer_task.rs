use domain::ext::LocalAppStateExt;
use domain::APP;
use foxtive::prelude::{AppResult, RabbitMQ};
use foxtive::rabbitmq::Message;
use foxtive_supervisor::contracts::SupervisedTask;
use tracing::{info, warn};

pub struct RabbitmqConsumerTask;

#[foxtive::async_trait]
impl SupervisedTask for RabbitmqConsumerTask {
    fn id(&self) -> &'static str {
        "rabbitmq-task"
    }
    fn name(&self) -> String {
        "RabbitMQ Task".to_string()
    }

    async fn run(&self) -> AppResult<()> {
        RabbitMQ::new_from_foxtive()
            .await?
            .requeue_on_failure(true)
            .setup_fn(|r| Box::pin(domain::setup::rabbitmq::rabbitmq_setup_function(r)))
            .await
            .consume_forever(&APP.state().rmq_queue_app, "qax updates", handle)
            .await
    }

    async fn on_shutdown(&self) {
        warn!("Shutting down rabbitmq task");
    }
}

async fn handle(msg: Message) -> AppResult<()> {
    let rk = msg.routing_key().clone();
    let raw_event_name = rk.as_str();
    let event_data = msg.str()?;

    info!("[{raw_event_name}] {event_data}");

    msg.ack().await
}
