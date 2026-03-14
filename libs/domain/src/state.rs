use crate::APP;
use foxtive::helpers::env;
use foxtive::results::AppResult;

#[derive(Debug, Clone)]
pub struct AppState {
    pub rmq_queue_app: String,
    pub rmq_exchange_app: String,
}

pub fn boot(env_prefix: &str) -> AppResult<()> {
    let state = AppState {
        rmq_queue_app: env::var(env_prefix, "RMQ_QUEUE_APP")?,
        rmq_exchange_app: env::var(env_prefix, "RMQ_EXCHANGE_APP")?,
    };

    APP.get_or_init(move || state);

    Ok(())
}
