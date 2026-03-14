//! Data Transfer Objects for Catalog operations
//! 
//! These DTOs handle request/response mapping for catalog items.

use bigdecimal::BigDecimal;
use foxtive::http::QueryParams;
use database::enums::catalog_category::CatalogCategory;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Search filters for catalog items
#[derive(Debug, Deserialize, Default, utoipa::IntoParams, utoipa::ToSchema)]
pub struct CatalogItemSearchRequest {
    pub category: Option<String>,
    pub supplier_id: Option<Uuid>,
    pub in_stock: Option<bool>,
}

pub struct CatalogItemListFilter {
    pub query: QueryParams,
    pub category: Option<String>,
    pub supplier_id: Option<Uuid>,
    pub in_stock: Option<bool>,
}

/// Response for a single catalog item
#[derive(Debug, Serialize)]
pub struct CatalogItemResponse {
    pub id: Uuid,
    pub secondary_id: String,
    pub name: String,
    pub category: CatalogCategory,
    pub supplier_id: Uuid,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub price_usd: BigDecimal,
    pub lead_time_days: Option<i32>,
    pub in_stock: Option<bool>,
    pub specs: Option<serde_json::Value>,
}

impl From<database::models::catalog_item::CatalogItem> for CatalogItemResponse {
    fn from(item: database::models::catalog_item::CatalogItem) -> Self {
        Self {
            id: item.id,
            secondary_id: item.secondary_id,
            name: item.name,
            category: item.category,
            supplier_id: item.supplier_id,
            manufacturer: item.manufacturer,
            model: item.model,
            price_usd: item.price_usd,
            lead_time_days: item.lead_time_days,
            in_stock: item.in_stock,
            specs: item.specs,
        }
    }
}

/// Detailed response including compatibility information
#[derive(Debug, Serialize)]
pub struct CatalogItemDetailResponse {
    #[serde(flatten)]
    pub item: CatalogItemResponse,
    pub compatible_items: Vec<CatalogItemResponse>,
}

/// Request to get compatible items
#[derive(Debug, Deserialize)]
pub struct CompatibleItemsRequest {
    pub item_id: Uuid,
}
