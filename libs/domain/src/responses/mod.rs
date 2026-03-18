use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;
use database::enums::purchase_order_status::PurchaseOrderStatus;
use database::models::purchase_order::PurchaseOrder;
use database::models::supplier::Supplier;
use database::models::user::User;

#[derive(Serialize)]
pub struct PurchaseOrderListResponse {
    pub id: Uuid,
    pub po_number: Option<String>,
    pub supplier_id: Uuid,
    pub created_by: Uuid,
    pub requestor: Option<String>,
    pub supplier_name: String,
    pub total_amount: f64,
    pub cost_center: Option<String>,
    pub payment_terms: Option<String>,
    pub needed_by_date: Option<chrono::NaiveDate>,
    pub status: PurchaseOrderStatus,
    pub submitted_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl PurchaseOrderListResponse {
    pub fn make((order, supplier, user): (PurchaseOrder, Supplier, User)) -> Self {
        Self {
            id: order.id,
            po_number: order.po_number,
            supplier_id: order.supplier_id,
            created_by: order.created_by,
            requestor: Some(user.name),
            cost_center: order.cost_center,
            payment_terms: order.payment_terms,
            needed_by_date: order.needed_by_date,
            total_amount: 0.0,
            status: order.status,
            supplier_name: supplier.name,
            submitted_at: order.submitted_at,
            created_at: order.created_at,
            updated_at: order.updated_at,
        }
    }
}