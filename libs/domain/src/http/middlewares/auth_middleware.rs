#![allow(dead_code)]

use crate::dto::auth_dto::{AuthId, UserCacheData};
use crate::dto::AuthSupplierId;
use crate::helpers::auth::{cleanup_bearer_token, get_cache_key, get_jwt_claims};
use crate::services::user_service::UserService;
use axum::extract::Request;
use axum::http;
use axum::middleware::Next;
use foxtive::prelude::AppStateExt;
use foxtive::FOXTIVE;
use foxtive_axum::http::HttpResult;
use std::sync::Arc;
use tracing::info;

pub async fn middleware(mut request: Request, next: Next) -> HttpResult {
    let token = cleanup_bearer_token(
        request
            .headers()
            .get(http::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok()),
    )?;

    let cache_key = get_cache_key(&token)?;

    let user = FOXTIVE
        .cache()
        .get_or_put::<UserCacheData, _, _>(&cache_key, move || async move {
            let claims = get_jwt_claims(&token)?;
            UserService::make_cache_data(claims.sub).await
        })
        .await?;

    info!("Authenticated user: {user:?}");

    request.extensions_mut().insert::<AuthId>(AuthId(user.id));

    request
        .extensions_mut()
        .insert::<AuthSupplierId>(AuthSupplierId(user.supplier_id));

    request
        .extensions_mut()
        .insert::<Arc<UserCacheData>>(Arc::new(user));

    Ok(next.run(request).await)
}
