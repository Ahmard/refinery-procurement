//! Data Transfer Objects for Purchase Order operations
//! 
//! These DTOs handle request/response mapping for purchase orders and their items.

use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use database::enums::purchase_order_status::PurchaseOrderStatus;
use foxtive::http::QueryParams;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, OpenApi, ToSchema};
use uuid::Uuid;
use validator::Validate;

/// Search filters for purchase orders
#[derive(Debug, Deserialize, Validate, IntoParams, ToSchema)]
pub struct PurchaseOrderSearchRequest {
    pub status: Option<String>,
    pub supplier_id: Option<Uuid>,
    pub created_by: Option<Uuid>,
}

pub struct PurchaseOrderListFilter {
    pub query: QueryParams,
    pub status: Option<String>,
    pub supplier_id: Option<Uuid>,
    pub created_by: Option<Uuid>,
}

/// Request to create a new purchase order
#[derive(Debug, Deserialize, Validate, OpenApi, ToSchema)]
pub struct PurchaseOrderCreateRequest {
    pub supplier_id: Uuid,
    pub requestor: Option<String>,
    pub cost_center: Option<String>,
    pub payment_terms: Option<String>,
    pub needed_by_date: Option<NaiveDate>,
}

/// Request to add an item to a purchase order
#[derive(Debug, Deserialize, Validate, OpenApi, ToSchema)]
pub struct PurchaseOrderItemRequest {
    #[validate(length(min = 1))]
    pub item_id: String, // catalog secondary_id
    #[validate(range(min = 1))]
    pub quantity: i32,
}

/// Response for a purchase order line item
#[derive(Debug, Serialize)]
pub struct PurchaseOrderItemResponse {
    pub id: Uuid,
    pub catalog_item_id: String,
    pub item_name: String,
    pub quantity: BigDecimal,
    pub unit_price: BigDecimal,
    pub total_price: BigDecimal,
    pub snapshot_lead_time: Option<i32>,
}

/// Status history entry for audit trail
#[derive(Debug, Serialize)]
pub struct StatusHistoryEntry {
    pub status: PurchaseOrderStatus,
    pub created_at: NaiveDateTime,
    pub created_by: Uuid,
}

/// Complete purchase order response
#[derive(Debug, Serialize)]
pub struct PurchaseOrderResponse {
    pub id: Uuid,
    pub po_number: String,
    pub supplier_id: Uuid,
    pub supplier_name: String,
    pub status: PurchaseOrderStatus,
    pub items: Vec<PurchaseOrderItemResponse>,
    pub total_amount: BigDecimal,
    pub requestor: Option<String>,
    pub cost_center: Option<String>,
    pub payment_terms: Option<String>,
    pub needed_by_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub submitted_at: Option<NaiveDateTime>,
    pub status_history: Vec<StatusHistoryEntry>,
}

/// Request to get purchase order details
#[derive(Debug, Deserialize)]
pub struct PurchaseOrderDetailsRequest {
    pub order_id: Uuid,
}

/// Idempotency header handling
#[derive(Debug, Clone)]
pub struct IdempotencyKey(pub String);

impl IdempotencyKey {
    pub fn from_header(header: &str) -> Self {
        Self(header.to_string())
    }
}

