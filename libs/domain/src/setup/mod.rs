use crate::{state, APP_CODE};
use ::foxtive::helpers::env;
use ::foxtive::prelude::AppResult;
use ::foxtive::setup::trace::{OutputFormat, Tracing};
use foxtive_axum::server::Server;
use foxtive_axum::FoxtiveAxumState;
use std::str::FromStr;
use std::sync::Arc;
use tracing::Level;

pub mod foxtive;
pub mod logger;
pub mod rabbitmq;

pub fn init_setup(service: &str) -> AppResult<()> {
    ::foxtive::setup::load_environment_variables(service);

    let env_prefix = APP_CODE;
    let log_level = env::var(env_prefix, "LOG_LEVEL").unwrap_or("debug".to_string());
    let log_with_target = env::var(env_prefix, "LOG_WITH_TARGET")
        .unwrap_or("false".to_string())
        .parse()?;
    let log_with_file_name = env::var(env_prefix, "LOG_WITH_FILE_NAME")
        .unwrap_or("false".to_string())
        .parse()?;
    let log_with_line_number = env::var(env_prefix, "LOG_WITH_LINE_NUMBER")
        .unwrap_or("false".to_string())
        .parse()?;
    let log_with_thread_names = env::var(env_prefix, "LOG_WITH_THREAD_NAMES")
        .unwrap_or("false".to_string())
        .parse()?;

    let log_format = OutputFormat::from_env(&format!("{env_prefix}_LOG_OUTPUT_FORMAT"))?;

    let tracing_config = Tracing::default()
        .with_level(Level::from_str(&log_level)?)
        .with_output_format(log_format)
        .with_include_target(log_with_target)
        .with_include_file(log_with_file_name)
        .with_include_line_number(log_with_line_number)
        .with_include_thread_names(log_with_thread_names)
        .with_logger_event_callback(move |e| {
            logger::on_event(e);
        });

    Server::init_bootstrap(service, tracing_config).expect("init bootstrap");

    Ok(())
}

pub async fn finish_setup(_fx_axum: Arc<FoxtiveAxumState>) -> AppResult<()> {
    state::boot(APP_CODE)?;
    rabbitmq::setup().await?;
    Ok(())
}
