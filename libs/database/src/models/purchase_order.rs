use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::Serialize;
use uuid::Uuid;

use crate::enums::purchase_order_status::PurchaseOrderStatus;
use crate::schema::purchase_orders;

#[derive(Serialize, Queryable, AsChangeset, Identifiable, Clone)]
#[diesel(table_name = purchase_orders)]
pub struct PurchaseOrder {
    pub id: Uuid,
    pub po_number: Option<String>,
    pub supplier_id: Uuid,
    pub created_by: Uuid,
    pub requestor: Option<String>,
    pub cost_center: Option<String>,
    pub payment_terms: Option<String>,
    pub needed_by_date: Option<chrono::NaiveDate>,
    pub status: PurchaseOrderStatus,
    #[serde(skip_serializing)]
    pub idempotency_key: Option<String>,
    pub submitted_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub total_cost: BigDecimal,
}

#[derive(Insertable)]
#[diesel(table_name = purchase_orders)]
pub struct PurchaseOrderInsertable {
    pub po_number: Option<String>,
    pub supplier_id: Uuid,
    pub created_by: Uuid,
    pub requestor: Option<String>,
    pub cost_center: Option<String>,
    pub payment_terms: Option<String>,
    pub needed_by_date: Option<chrono::NaiveDate>,
    pub status: PurchaseOrderStatus,
    pub idempotency_key: Option<String>,
    pub submitted_at: Option<NaiveDateTime>,
    pub total_cost: BigDecimal,
}
