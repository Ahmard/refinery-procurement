use crate::impl_crud_repo;
use database::models::catalog_item_compatibility::{
    CatalogItemCompatibility, CatalogItemCompatibilityInsertable,
};
use database::schema::catalog_item_compatibility;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, RunQueryDsl};
use foxtive::prelude::{AppResult, AppStateExt, IntoAppResult};
use foxtive::FOXTIVE;

pub struct CatalogItemCompatibilityRepository;

impl_crud_repo!(
    CatalogItemCompatibilityRepository,
    catalog_item_compatibility,
    CatalogItemCompatibility,
    CatalogItemCompatibilityInsertable,
    "catalog_item_compatibility"
);

impl CatalogItemCompatibilityRepository {
    /// Find compatible items for a given item
    pub fn find_compatible_for(item_id: uuid::Uuid) -> AppResult<Vec<CatalogItemCompatibility>> {
        catalog_item_compatibility::table
            .filter(catalog_item_compatibility::item_id.eq(item_id))
            .get_results(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }

    /// Find items that are compatible with the given item (reverse lookup)
    pub fn find_compatible_with(item_id: uuid::Uuid) -> AppResult<Vec<CatalogItemCompatibility>> {
        catalog_item_compatibility::table
            .filter(catalog_item_compatibility::compatible_item_id.eq(item_id))
            .get_results(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }

    /// Check if two items are compatible
    pub fn are_compatible(item_id: uuid::Uuid, compatible_item_id: uuid::Uuid) -> AppResult<bool> {
        use diesel::dsl::exists;
        use foxtive::prelude::IntoAppResult;

        let query = catalog_item_compatibility::table.filter(
            catalog_item_compatibility::item_id
                .eq(item_id)
                .and(catalog_item_compatibility::compatible_item_id.eq(compatible_item_id)),
        );

        diesel::select(exists(query))
            .first(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }

    /// Find all compatibility relationships for an item (alias for find_compatible_for)
    pub fn find_by_item_id(item_id: uuid::Uuid) -> AppResult<Vec<CatalogItemCompatibility>> {
        Self::find_compatible_for(item_id)
    }
}
