use std::fmt::{Display, Formatter};
use foxtive::enums::AppMessage;

mod event;

pub use event::AppEvent;

#[derive(Debug, PartialEq)]
pub enum EnumError {
    InvalidVariant(String),
}

impl From<EnumError> for foxtive::Error {
    fn from(value: EnumError) -> Self {
        match value {
            EnumError::InvalidVariant(e) => {
                println!("Invalid Event: {e}");
                AppMessage::internal_server_error("Invalid Quarkaxis Event Variant").into_anyhow()
            }
        }
    }
}

impl Display for EnumError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Event")
    }
}