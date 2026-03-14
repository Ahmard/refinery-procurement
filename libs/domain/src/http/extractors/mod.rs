use crate::dto::auth_dto::AuthId;
use crate::dto::{AuthSupplierId, IdempotencyKey};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use foxtive::{bad_request, internal_server_error};
use foxtive_axum::error::HttpError;

impl<S> FromRequestParts<S> for AuthId
where
    S: Send + Sync,
{
    type Rejection = HttpError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthId>()
            .ok_or(HttpError::AppError(internal_server_error!(
                "Something went wrong"
            )))
            .cloned()
    }
}

impl<S> FromRequestParts<S> for AuthSupplierId
where
    S: Send + Sync,
{
    type Rejection = HttpError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthSupplierId>()
            .ok_or(HttpError::AppError(internal_server_error!(
                "Something went wrong"
            )))
            .cloned()
    }
}

impl<S> FromRequestParts<S> for IdempotencyKey
where
    S: Send + Sync,
{
    type Rejection = HttpError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let key = parts
            .headers
            .get("Idempotency-Key")
            .map(|h| h.to_str().unwrap().to_string())
            .ok_or(bad_request!("Idempotency-Key header is required"))?;

        Ok(IdempotencyKey(key))
    }
}
