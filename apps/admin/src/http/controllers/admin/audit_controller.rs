use axum::extract::{Path, Query};
use axum::routing::get;
use axum::Router;
use domain::dto::AuditLogResponse;
use domain::services::AuditLogService;
use foxtive::helpers::block;
use foxtive_axum::http::response::ext::ResponderExt;
use foxtive_axum::http::HttpResult;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, utoipa::ToSchema, utoipa::IntoParams)]
pub struct TargetQuery {
    entity: String,
    id: String,
}

pub fn controller() -> Router {
    Router::new()
        .route("/", get(get_all_logs))
        .route("/user/{id}", get(get_logs_by_user))
        .route("/target", get(get_logs_by_target))
}

#[utoipa::path(
    get,
    path = "/api/v1/admin/audit",
    tag = "audit",
    responses(
        (status = 200, description = "Audit logs fetched successfully")
    )
)]
async fn get_all_logs() -> HttpResult {
    block(move || {
        let logs = AuditLogService::get_all_logs()?
            .into_iter()
            .map(AuditLogResponse::from)
            .collect::<Vec<_>>();
        Ok(logs)
    })
    .await
    .respond_msg("Audit logs fetched successfully")
}

#[utoipa::path(
    get,
    path = "/api/v1/admin/audit/user/{id}",
    tag = "audit",
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "Audit logs for user fetched successfully")
    )
)]
async fn get_logs_by_user(Path(user_id): Path<Uuid>) -> HttpResult {
    block(move || {
        let logs = AuditLogService::get_logs_by_user(user_id)?
            .into_iter()
            .map(AuditLogResponse::from)
            .collect::<Vec<_>>();
        Ok(logs)
    })
    .await
    .respond_msg("Audit logs for user fetched successfully")
}

#[utoipa::path(
    get,
    path = "/api/v1/admin/audit/target",
    tag = "audit",
    params(
        TargetQuery
    ),
    responses(
        (status = 200, description = "Audit logs for target fetched successfully")
    )
)]
async fn get_logs_by_target(Query(target): Query<TargetQuery>) -> HttpResult {
    block(move || {
        let logs = AuditLogService::get_logs_by_target(target.entity, target.id)?
            .into_iter()
            .map(AuditLogResponse::from)
            .collect::<Vec<_>>();
        Ok(logs)
    })
    .await
    .respond_msg("Audit logs for target fetched successfully")
}
