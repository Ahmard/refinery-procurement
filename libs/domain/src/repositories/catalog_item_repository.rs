use crate::dto::CatalogItemListFilter;
use crate::impl_crud_repo;
use database::models::catalog_item::{CatalogItem, CatalogItemInsertable};
use database::schema::catalog_items;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods, QueryDsl, RunQueryDsl,
};
use foxtive::database::ext::OptionalResultExt;
use foxtive::database::pagination::Paginate;
use foxtive::prelude::{AppResult, AppStateExt, IntoAppResult};
use foxtive::results::AppPaginationResult;
use foxtive::FOXTIVE;

pub struct CatalogItemRepository;

impl_crud_repo!(
    CatalogItemRepository,
    catalog_items,
    CatalogItem,
    CatalogItemInsertable,
    "catalog item"
);

impl CatalogItemRepository {
    pub fn list(filter: CatalogItemListFilter) -> AppPaginationResult<CatalogItem> {
        let mut builder = catalog_items::table
            .filter(catalog_items::deleted_at.is_null())
            .into_boxed();

        if filter.query.search().is_some() {
            builder = builder.filter(
                catalog_items::name
                    .ilike(filter.query.search_query_like())
                    .or(catalog_items::name.ilike(filter.query.search_query_like()))
                    .or(catalog_items::category.ilike(filter.query.search_query_like())),
            );
        }

        if let Some(state) = filter.in_stock {
            builder = builder.filter(catalog_items::in_stock.eq(state));
        }

        if let Some(id) = filter.supplier_id {
            builder = builder.filter(catalog_items::supplier_id.eq(id));
        }

        builder
            .paginate(filter.query.curr_page())
            .per_page(filter.query.per_page())
            .load_and_count_pages(&mut *FOXTIVE.db_conn()?)
    }
    /// Find catalog item by secondary ID
    pub fn find_by_secondary_id(secondary_id: &str) -> AppResult<Option<CatalogItem>> {
        catalog_items::table
            .filter(catalog_items::secondary_id.eq(secondary_id))
            .first(&mut FOXTIVE.db_conn()?)
            .optional()
    }

    /// Find catalog items by supplier ID
    pub fn find_by_supplier_id(supplier_id: uuid::Uuid) -> AppResult<Vec<CatalogItem>> {
        catalog_items::table
            .filter(catalog_items::supplier_id.eq(supplier_id))
            .get_results(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }

    /// Find catalog items by category
    pub fn find_by_category(
        category: database::enums::catalog_category::CatalogCategory,
    ) -> AppResult<Vec<CatalogItem>> {
        catalog_items::table
            .filter(catalog_items::category.eq(category))
            .get_results(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }

    /// Search catalog items by name (case-insensitive partial match)
    pub fn search_by_name(search_term: &str) -> AppResult<Vec<CatalogItem>> {
        let pattern = format!("%{}%", search_term);
        catalog_items::table
            .filter(catalog_items::name.ilike(pattern))
            .get_results(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }

    /// Find in-stock items
    pub fn find_in_stock() -> AppResult<Vec<CatalogItem>> {
        catalog_items::table
            .filter(catalog_items::in_stock.eq(Some(true)))
            .get_results(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }
}
