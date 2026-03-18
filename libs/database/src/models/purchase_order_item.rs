use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::Serialize;
use uuid::Uuid;

use crate::schema::purchase_order_items;

#[derive(Serialize, Queryable, AsChangeset, Identifiable, Clone)]
#[diesel(table_name = purchase_order_items)]
pub struct PurchaseOrderItem {
    pub id: Uuid,
    pub purchase_order_id: Uuid,
    pub catalog_item_id: Uuid,
    pub quantity: BigDecimal,
    pub snapshot_price: BigDecimal,
    pub snapshot_lead_time: Option<i32>,
    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = purchase_order_items)]
pub struct PurchaseOrderItemInsertable {
    pub purchase_order_id: Uuid,
    pub catalog_item_id: Uuid,
    pub quantity: BigDecimal,
    pub snapshot_price: BigDecimal,
    pub snapshot_lead_time: Option<i32>,
    pub created_by: Uuid,
}
