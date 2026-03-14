use crate::impl_crud_repo;
use database::models::purchase_order_status_history::{
    PurchaseOrderStatusHistory, PurchaseOrderStatusHistoryInsertable,
};
use database::schema::purchase_order_status_history;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use foxtive::database::ext::OptionalResultExt;
use foxtive::prelude::{AppResult, AppStateExt, IntoAppResult};
use foxtive::FOXTIVE;

pub struct PurchaseOrderStatusHistoryRepository;

impl_crud_repo!(
    PurchaseOrderStatusHistoryRepository,
    purchase_order_status_history,
    PurchaseOrderStatusHistory,
    PurchaseOrderStatusHistoryInsertable,
    "purchase order status history"
);

impl PurchaseOrderStatusHistoryRepository {
    /// Find status history for a purchase order (ordered by creation date)
    pub fn find_by_purchase_order_id(
        po_id: uuid::Uuid,
    ) -> AppResult<Vec<PurchaseOrderStatusHistory>> {
        use database::schema::purchase_order_status_history::dsl::*;
        use diesel::prelude::*;

        purchase_order_status_history
            .filter(purchase_order_id.eq(po_id))
            .order(created_at.asc())
            .get_results(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }

    /// Get the latest status history entry for a purchase order
    pub fn find_latest_for_purchase_order(
        purchase_order_id: uuid::Uuid,
    ) -> AppResult<Option<PurchaseOrderStatusHistory>> {
        purchase_order_status_history::table
            .filter(purchase_order_status_history::purchase_order_id.eq(purchase_order_id))
            .order(purchase_order_status_history::created_at.desc())
            .first(&mut FOXTIVE.db_conn()?)
            .optional()
    }

    /// Find status changes by a specific user
    pub fn find_by_created_by(
        created_by: uuid::Uuid,
    ) -> AppResult<Vec<PurchaseOrderStatusHistory>> {
        purchase_order_status_history::table
            .filter(purchase_order_status_history::created_by.eq(created_by))
            .get_results(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }

    /// Find status history for a purchase order (alias for find_by_purchase_order_id)
    pub fn find_by_order_id(po_id: uuid::Uuid) -> AppResult<Vec<PurchaseOrderStatusHistory>> {
        Self::find_by_purchase_order_id(po_id)
    }
}
