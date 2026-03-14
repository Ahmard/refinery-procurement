use crate::impl_crud_repo;
use database::models::purchase_order_item::{PurchaseOrderItem, PurchaseOrderItemInsertable};
use database::schema::purchase_order_items;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use foxtive::prelude::{AppResult, AppStateExt, IntoAppResult};
use foxtive::FOXTIVE;

pub struct PurchaseOrderItemRepository;

impl_crud_repo!(
    PurchaseOrderItemRepository,
    purchase_order_items,
    PurchaseOrderItem,
    PurchaseOrderItemInsertable,
    "purchase_order_item"
);

impl PurchaseOrderItemRepository {
    /// Add a new item to a purchase order
    pub fn add_item(item: PurchaseOrderItemInsertable) -> AppResult<PurchaseOrderItem> {
        Self::create(item)
    }

    /// List all items for a purchase order
    pub fn list_by_order(purchase_order_id: uuid::Uuid) -> AppResult<Vec<PurchaseOrderItem>> {
        purchase_order_items::table
            .filter(purchase_order_items::purchase_order_id.eq(purchase_order_id))
            .get_results(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }

    /// Find all items for a catalog item
    pub fn find_by_catalog_item_id(
        catalog_item_id: uuid::Uuid,
    ) -> AppResult<Vec<PurchaseOrderItem>> {
        purchase_order_items::table
            .filter(purchase_order_items::catalog_item_id.eq(catalog_item_id))
            .get_results(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }

    /// Calculate total quantity ordered for a catalog item
    pub fn total_quantity_for_catalog_item(catalog_item_id: uuid::Uuid) -> AppResult<i64> {
        use diesel::dsl::sum;

        purchase_order_items::table
            .filter(purchase_order_items::catalog_item_id.eq(catalog_item_id))
            .select(sum(purchase_order_items::quantity))
            .first::<Option<i64>>(&mut FOXTIVE.db_conn()?)
            .into_app_result()
            .map(|opt| opt.unwrap_or(0))
    }
}
