use axum::{middleware, Router};
use domain::http::controllers::auth_controller;
use domain::http::middlewares::auth_middleware;

pub mod admin;

fn api_routes() -> Router {
    Router::new()
        .nest("/admin", admin::routes())
        .layer(middleware::from_fn(auth_middleware::middleware))
        // Middleware doesn't apply to routes added after it
        .nest("/auth", auth_controller::controller())
}

fn web_routes() -> Router {
    Router::new()
}

pub fn routes() -> Router {
    Router::new()
        .nest("/api/v1", api_routes())
        .merge(web_routes())
}
