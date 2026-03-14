use axum::{Router, middleware};
use domain::http::middlewares::auth_middleware;

pub mod catalog_controller;

fn api_routes() -> Router {
    Router::new()
        .nest("/catalog", catalog_controller::controller())
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
