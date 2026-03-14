use axum::extract::{Path, Query};
use axum::{routing::get, routing::post, Router};
use domain::dto::purchase_order_dto::{
    IdempotencyKey, PurchaseOrderCreateRequest, PurchaseOrderItemRequest, PurchaseOrderListFilter,
    PurchaseOrderSearchRequest,
};
use domain::dto::AuthId;
use domain::repositories::PurchaseOrderRepository;
use domain::services::PurchaseOrderService;
use foxtive::helpers::block;
use foxtive::http::QueryParams;
use foxtive_axum::http::extractors::JsonBody;
use foxtive_axum::http::response::ext::ResponderExt;
use foxtive_axum::http::HttpResult;
use tracing::info;
use uuid::Uuid;
use validator::Validate;

/// Get all routes for the procurement controller
pub fn controller() -> Router {
    Router::new()
        .route("/purchase-orders", get(list))
        .route("/purchase-orders", post(create_purchase_order))
        .route("/purchase-orders/{id}/items", post(add_item_to_order))
        .route("/purchase-orders/{id}/submit", post(submit_order))
        .route("/purchase-orders/{id}", get(get_order_details))
}

/// List purchase orders with filters and pagination
#[utoipa::path(
    get,
    path = "/api/v1/procurement/purchase-orders",
    tag = "procurement",
    params(
        QueryParams,
        PurchaseOrderSearchRequest
    ),
    responses(
        (status = 200, description = "Purchase orders fetched successfully")
    )
)]
async fn list(query: Query<QueryParams>, lq: Query<PurchaseOrderSearchRequest>) -> HttpResult {
    block(move || {
        let lq = lq.0;
        PurchaseOrderRepository::list(PurchaseOrderListFilter {
            query: query.0,
            status: lq.status,
            supplier_id: lq.supplier_id,
            created_by: lq.created_by,
        })
    })
    .await
    .respond_msg("Purchase orders fetched successfully")
}

/// Create a new draft purchase order
#[utoipa::path(
    post,
    path = "/api/v1/procurement/purchase-orders",
    tag = "procurement",
    request_body = PurchaseOrderCreateRequest,
    responses(
        (status = 200, description = "Purchase order created successfully")
    )
)]
async fn create_purchase_order(
    auth_id: AuthId,
    idempotency_key: IdempotencyKey,
    form: JsonBody<PurchaseOrderCreateRequest>,
) -> HttpResult {
    info!("Handling create purchase order request");

    block(move || {
        PurchaseOrderService::create_draft_order(form.into_inner(), idempotency_key, auth_id.0)
    })
    .await
    .respond_msg("Purchase order created successfully")
}

/// Add an item to a draft purchase order
#[utoipa::path(
    post,
    path = "/api/v1/procurement/purchase-orders/{id}/items",
    tag = "procurement",
    request_body = PurchaseOrderItemRequest,
    params(
        ("id" = Uuid, Path, description = "Purchase Order ID")
    ),
    responses(
        (status = 200, description = "Item added to order successfully")
    )
)]
async fn add_item_to_order(
    id: Path<Uuid>,
    auth_id: AuthId,
    form: JsonBody<PurchaseOrderItemRequest>,
) -> HttpResult {
    info!(order_id = %id.0, "Handling add item to order request");

    form.validate()?;

    let order_id = id.0;
    let created_by = auth_id.0;

    block(move || PurchaseOrderService::add_item_to_order(order_id, form.into_inner(), created_by))
        .await
        .respond_msg("Item added to order successfully")
}

/// Submit a draft purchase order for approval
#[utoipa::path(
    post,
    path = "/api/v1/procurement/purchase-orders/{id}/submit",
    tag = "procurement",
    params(
        ("id" = Uuid, Path, description = "Purchase Order ID")
    ),
    responses(
        (status = 200, description = "Purchase order submitted successfully")
    )
)]
async fn submit_order(auth_id: AuthId, id: Path<Uuid>) -> HttpResult {
    info!(order_id = %id.0, "Handling submit order request");

    let order_id = id.0;

    block(move || PurchaseOrderService::submit_order(order_id, auth_id.0))
        .await
        .respond_msg("Purchase order submitted successfully")
}

/// Get complete purchase order details
#[utoipa::path(
    get,
    path = "/api/v1/procurement/purchase-orders/{id}",
    tag = "procurement",
    params(
        ("id" = Uuid, Path, description = "Purchase Order ID")
    ),
    responses(
        (status = 200, description = "Order details retrieved successfully")
    )
)]
async fn get_order_details(id: Path<Uuid>) -> HttpResult {
    info!(order_id = %id.0, "Handling get order details request");

    block(move || PurchaseOrderService::get_order_details(id.0))
        .await
        .respond_msg("Order details retrieved successfully")
}
