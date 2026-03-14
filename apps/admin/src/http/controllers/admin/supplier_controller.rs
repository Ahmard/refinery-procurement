use axum::extract::{Path, Query};
use axum::{routing::get, Router};
use axum::routing::post;
use domain::dto::auth_dto::AuthId;
use domain::dto::supplier_dto::{SupplierCreateForm, SupplierDto};
use domain::repositories::SupplierRepository;
use domain::services::SupplierService;
use foxtive::helpers::block;
use foxtive::http::QueryParams;
use foxtive_axum::http::extractors::JsonBody;
use foxtive_axum::http::response::ext::ResponderExt;
use foxtive_axum::http::HttpResult;
use uuid::Uuid;
use validator::Validate;

pub fn controller() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/", post(store))
        .route("/{id}", get(show))
}

#[utoipa::path(
    get,
    path = "/api/v1/admin/suppliers",
    tag = "suppliers",
    params(QueryParams),
    responses(
        (status = 200, description = "Suppliers fetched successfully")
    )
)]
async fn index(query: Query<QueryParams>) -> HttpResult {
    block(move || SupplierRepository::list(query.0))
        .await
        .respond_msg("Suppliers fetched successfully")
}

#[utoipa::path(
    get,
    path = "/api/v1/admin/suppliers/{id}",
    tag = "suppliers",
    params(
        ("id" = Uuid, Path, description = "Supplier ID")
    ),
    responses(
        (status = 200, description = "Supplier info fetched successfully")
    )
)]
async fn show(id: Path<Uuid>) -> HttpResult {
    block(move || SupplierRepository::find(id.0))
        .await
        .respond_msg("Supplier info fetched successfully")
}

#[utoipa::path(
    post,
    path = "/api/v1/admin/suppliers",
    tag = "suppliers",
    request_body = SupplierCreateForm,
    responses(
        (status = 200, description = "Supplier created successfully")
    )
)]
async fn store(auth_id: AuthId, form: JsonBody<SupplierCreateForm>) -> HttpResult {
    form.validate()?;
    block(move || {
        SupplierService::create(SupplierDto {
            form: form.into_inner(),
            created_by: auth_id.0,
        })
    })
    .await
    .respond_msg("Supplier created successfully")
}
