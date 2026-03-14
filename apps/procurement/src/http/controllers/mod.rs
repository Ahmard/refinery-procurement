use axum::{middleware, Router};
use domain::http::middlewares::auth_middleware;

pub mod procurement_controller;

fn api_routes() -> Router {
    Router::new()
        .nest("/procurement", procurement_controller::controller())
        .layer(middleware::from_fn(auth_middleware::middleware))
    // Middleware doesn't apply to routes added after it
}

fn web_routes() -> Router {
    Router::new()
}

pub fn routes() -> Router {
    Router::new()
        .nest("/api/v1", api_routes())
        .merge(web_routes())
}
