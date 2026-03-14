use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::Serialize;
use uuid::Uuid;

use crate::enums::purchase_order_status::PurchaseOrderStatus;
use crate::schema::purchase_order_status_history;

#[derive(Serialize, Queryable, AsChangeset, Identifiable, Clone)]
#[diesel(table_name = purchase_order_status_history)]
pub struct PurchaseOrderStatusHistory {
    pub id: Uuid,
    pub purchase_order_id: Uuid,
    pub status: PurchaseOrderStatus,
    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = purchase_order_status_history)]
pub struct PurchaseOrderStatusHistoryInsertable {
    pub purchase_order_id: Uuid,
    pub status: PurchaseOrderStatus,
    pub created_by: Uuid,
}
