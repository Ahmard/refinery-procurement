use crate::state::AppState;
use foxtive::helpers::env;
use std::sync::OnceLock;

pub mod contracts;
pub mod dto;
pub mod enums;
pub mod event;
pub mod ext;
pub mod helpers;
pub mod http;
pub mod macros;
pub mod repositories;
pub mod services;
pub mod setup;
pub mod state;
pub mod responses;

pub static APP: OnceLock<AppState> = OnceLock::new();
pub static APP_CODE: &str = "PROC";

pub fn is_live() -> bool {
    env::var(APP_CODE, "APP_ENVIRONMENT")
        .map(|val| val == "production")
        .unwrap_or(false)
}
