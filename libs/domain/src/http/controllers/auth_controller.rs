use axum::routing::{get, post};
use axum::Router;
use crate::dto::auth_dto::{AuthId, AuthLoginForm};
use crate::http::middlewares::auth_middleware;
use crate::services::auth_service::AuthService;
use crate::services::UserService;
use foxtive::helpers::block;
use foxtive_axum::http::extractors::JsonBody;
use foxtive_axum::http::response::ext::ResponderExt;
use foxtive_axum::http::HttpResult;

pub fn controller() -> Router {
    Router::new().route("/login", post(login)).route(
        "/me",
        get(me).layer(axum::middleware::from_fn(auth_middleware::middleware)),
    )
}

/// POST /api/v1/login
/// Login with email or phone number and password
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "auth",
    request_body = AuthLoginForm,
    responses(
        (status = 200, description = "Login successful")
    )
)]
async fn login(payload: JsonBody<AuthLoginForm>) -> HttpResult {
    block(move || AuthService::login(payload.into_inner()))
        .await
        .respond_msg("Login successful")
}

/// GET /api/v1/me
/// Get user profile
#[utoipa::path(
    get,
    path = "/api/v1/auth/me",
    tag = "auth",
    responses(
        (status = 200, description = "Profile fetched successfully")
    )
)]
async fn me(id: AuthId) -> HttpResult {
    block(move || UserService::make_profile(id.0))
        .await
        .respond_msg("Profile fetched successfully")
}
