use axum::extract::{Path, Query};
use axum::{routing::get, Router};
use domain::dto::catalog_dto::CatalogItemSearchRequest;
use domain::dto::{CatalogItemListFilter, CatalogItemResponse};
use domain::repositories::CatalogItemRepository;
use domain::services::CatalogService;
use foxtive::helpers::block;
use foxtive::http::QueryParams;
use foxtive::not_found;
use foxtive_axum::http::response::ext::ResponderExt;
use foxtive_axum::http::HttpResult;
use std::str::FromStr;
use uuid::Uuid;

pub fn controller() -> Router {
    Router::new()
        .route("/items", get(list))
        .route("/items/{id}", get(get_item_details))
        .route("/items/{id}/compatible", get(get_compatible_items))
}

#[utoipa::path(
    get,
    path = "/api/v1/catalog/items",
    tag = "catalog",
    params(
        QueryParams,
        CatalogItemSearchRequest
    ),
    responses(
        (status = 200, description = "Catalog items fetched successfully")
    )
)]
async fn list(
    query: Query<QueryParams>,
    lq: Query<CatalogItemSearchRequest>,
) -> HttpResult {
    block(move || {
        let lq = lq.0;
        CatalogItemRepository::list(CatalogItemListFilter {
            query: query.0,
            category: lq.category,
            supplier_id: lq.supplier_id,
            in_stock: lq.in_stock,
        })
    })
    .await
    .respond_msg("Catalog items fetched successfully")
}

#[utoipa::path(
    get,
    path = "/api/v1/catalog/items/{id}",
    tag = "catalog",
    params(
        ("id" = String, Path, description = "Catalog Item ID (UUID or secondary ID)")
    ),
    responses(
        (status = 200, description = "Item details retrieved")
    )
)]
async fn get_item_details(id_str: Path<String>) -> HttpResult {
    block(move || {
        let item: CatalogItemResponse = match Uuid::from_str(&id_str) {
            Ok(id) => CatalogItemRepository::find(id)?.into(),
            Err(_) => CatalogItemRepository::find_by_secondary_id(&id_str.0)?
                .ok_or(not_found!("Item not found"))?
                .into(),
        };

        Ok(item)
    })
    .await
    .respond_msg("Item details retrieved")
}

#[utoipa::path(
    get,
    path = "/api/v1/catalog/items/{id}/compatible",
    tag = "catalog",
    params(
        ("id" = Uuid, Path, description = "Catalog Item ID")
    ),
    responses(
        (status = 200, description = "Compatible items retrieved")
    )
)]
async fn get_compatible_items(id: Path<Uuid>) -> HttpResult {
    block(move || CatalogService::get_compatible_items(id.0))
        .await
        .respond_msg("Compatible items retrieved")
}
