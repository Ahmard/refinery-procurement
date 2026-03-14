use crate::state::AppState;
use std::sync::OnceLock;

pub mod contracts;
pub mod enums;
pub mod dto;
pub mod event;
pub mod ext;
pub mod macros;
pub mod repositories;
pub mod services;
pub mod setup;
pub mod state;
pub mod http;
pub mod helpers;

pub static APP: OnceLock<AppState> = OnceLock::new();
pub static APP_CODE: &str = "PROC";
