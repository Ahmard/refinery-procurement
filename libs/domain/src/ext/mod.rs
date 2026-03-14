use std::sync::OnceLock;
use crate::APP;
use crate::state::AppState;

pub trait LocalAppStateExt {
    fn state(&self) -> &AppState;
}

impl LocalAppStateExt for OnceLock<AppState> {
    fn state(&self) -> &AppState {
        APP.get().expect("AppState not initialized")
    }
}
