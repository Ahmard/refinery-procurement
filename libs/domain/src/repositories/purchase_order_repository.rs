//! Repository for PurchaseOrder model

use crate::dto::purchase_order_dto::PurchaseOrderListFilter;
use crate::impl_crud_repo;
use chrono::NaiveDate;
use database::enums::purchase_order_status::PurchaseOrderStatus;
use database::models::purchase_order::{PurchaseOrder, PurchaseOrderInsertable};
use database::schema::{purchase_orders, suppliers, users};
use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl,
};
use foxtive::database::ext::{OptionalResultExt, PaginationResultExt};
use foxtive::database::pagination::Paginate;
use foxtive::prelude::{AppResult, AppStateExt, IntoAppResult};
use foxtive::results::AppPaginationResult;
use foxtive::FOXTIVE;
use uuid::Uuid;
use crate::responses::PurchaseOrderListResponse;

pub struct PurchaseOrderRepository;

impl_crud_repo!(
    PurchaseOrderRepository,
    purchase_orders,
    PurchaseOrder,
    PurchaseOrderInsertable,
    "purchase_order"
);

impl PurchaseOrderRepository {
    /// List purchase orders with filters and pagination
    pub fn list(filter: PurchaseOrderListFilter) -> AppPaginationResult<PurchaseOrderListResponse> {
        let mut builder = purchase_orders::table
            .inner_join(suppliers::table)
            .inner_join(users::table)
            .into_boxed();

        if filter.query.search().is_some() {
            builder = builder.filter(
                purchase_orders::po_number
                    .ilike(filter.query.search_query_like())
                    .or(purchase_orders::requestor.ilike(filter.query.search_query_like())),
            );
        }

        if let Some(status) = filter.status {
            builder = builder.filter(purchase_orders::status.eq(status));
        }

        if let Some(supplier_id) = filter.supplier_id {
            builder = builder.filter(purchase_orders::supplier_id.eq(supplier_id));
        }

        if let Some(created_by) = filter.created_by {
            builder = builder.filter(purchase_orders::created_by.eq(created_by));
        }

        builder
            .paginate(filter.query.curr_page())
            .per_page(filter.query.per_page())
            .load_and_count_pages(&mut *FOXTIVE.db_conn()?)
            .map_page_data(PurchaseOrderListResponse::make)
    }

    /// Find purchase order by PO number
    pub fn find_by_po_number(po_number: &str) -> AppResult<Option<PurchaseOrder>> {
        purchase_orders::table
            .filter(purchase_orders::po_number.eq(po_number))
            .first(&mut FOXTIVE.db_conn()?)
            .optional()
    }

    /// Find purchase orders by supplier ID
    pub fn find_by_supplier_id(supplier_id: uuid::Uuid) -> AppResult<Vec<PurchaseOrder>> {
        purchase_orders::table
            .filter(purchase_orders::supplier_id.eq(supplier_id))
            .get_results(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }

    /// Find purchase orders by status
    pub fn find_by_status(status: PurchaseOrderStatus) -> AppResult<Vec<PurchaseOrder>> {
        purchase_orders::table
            .filter(purchase_orders::status.eq(status))
            .get_results(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }

    /// Find purchase orders by requestor
    pub fn find_by_requestor(requestor: &str) -> AppResult<Vec<PurchaseOrder>> {
        purchase_orders::table
            .filter(purchase_orders::requestor.eq(requestor))
            .get_results(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }

    /// Find pending purchase orders (not yet submitted)
    pub fn find_pending() -> AppResult<Vec<PurchaseOrder>> {
        purchase_orders::table
            .filter(purchase_orders::status.eq(PurchaseOrderStatus::Draft))
            .get_results(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }

    /// Find purchase orders needing attention by date
    pub fn find_urgently_needed(before_date: NaiveDate) -> AppResult<Vec<PurchaseOrder>> {
        purchase_orders::table
            .filter(purchase_orders::needed_by_date.le(before_date))
            .filter(purchase_orders::needed_by_date.is_not_null())
            .get_results(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }

    /// Find purchase orders by creator user ID
    pub fn find_by_creator(created_by: Uuid) -> AppResult<Vec<PurchaseOrder>> {
        purchase_orders::table
            .filter(purchase_orders::created_by.eq(created_by))
            .get_results(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }

    /// Find purchase order by idempotency key
    pub fn find_by_idempotency_key(key: &str) -> AppResult<Option<PurchaseOrder>> {
        purchase_orders::table
            .filter(purchase_orders::idempotency_key.eq(key))
            .first(&mut FOXTIVE.db_conn()?)
            .optional()
    }
}
