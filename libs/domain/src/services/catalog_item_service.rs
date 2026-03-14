use foxtive::results::AppResult;
use database::models::catalog_item::{CatalogItem, CatalogItemInsertable};
use crate::repositories::CatalogItemRepository;

pub struct CatalogItemService;

impl CatalogItemService {
    pub fn create(ins: CatalogItemInsertable) -> AppResult<CatalogItem> {
        CatalogItemRepository::create(ins)
    }
}