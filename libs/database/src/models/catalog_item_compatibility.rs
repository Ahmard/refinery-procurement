use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::Serialize;
use uuid::Uuid;

use crate::schema::catalog_item_compatibility;

#[derive(Serialize, Queryable, AsChangeset, Identifiable, Clone)]
#[diesel(table_name = catalog_item_compatibility)]
pub struct CatalogItemCompatibility {
    pub id: Uuid,
    pub item_id: Uuid,
    pub compatible_item_id: Uuid,
    pub created_by: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = catalog_item_compatibility)]
pub struct CatalogItemCompatibilityInsertable {
    pub item_id: Uuid,
    pub compatible_item_id: Uuid,
    pub created_by: Uuid,
}
