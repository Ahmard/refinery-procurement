use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::Serialize;
use uuid::Uuid;

use crate::enums::catalog_category::CatalogCategory;
use crate::schema::catalog_items;

#[derive(Serialize, Queryable, AsChangeset, Identifiable, Clone)]
#[diesel(table_name = catalog_items)]
pub struct CatalogItem {
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
    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = catalog_items)]
pub struct CatalogItemInsertable {
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
    pub created_by: Uuid,
}
