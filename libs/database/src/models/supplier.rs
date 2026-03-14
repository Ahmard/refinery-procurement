use crate::enums::supplier_enums::SupplierStatus;
use crate::schema::suppliers;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Queryable, AsChangeset, Identifiable, Clone)]
#[diesel(table_name = suppliers)]
pub struct Supplier {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_by: Uuid,
    pub name: String,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub address: Option<String>,
    pub status: SupplierStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = suppliers)]
pub struct SupplierInsertable {
    pub user_id: Uuid,
    pub created_by: Uuid,
    pub name: String,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub address: Option<String>,
    pub status: SupplierStatus,
}
