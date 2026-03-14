use axum::Router;

pub mod audit_controller;
pub mod supplier_controller;
pub mod system_controller;

pub fn routes() -> Router {
    Router::new()
        .nest("/system", system_controller::controller())
        .nest("/suppliers", supplier_controller::controller())
        .nest("/audit", audit_controller::controller())
}
