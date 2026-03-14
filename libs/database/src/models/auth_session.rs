use chrono::NaiveDateTime;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::Serialize;
use uuid::Uuid;

use crate::schema::auth_sessions;

#[derive(Serialize, Queryable, AsChangeset, Identifiable, Clone)]
#[diesel(table_name = auth_sessions)]
pub struct AuthSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: NaiveDateTime,
    pub created_by: Option<Uuid>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = auth_sessions)]
pub struct AuthSessionInsertable {
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: NaiveDateTime,
    pub created_by: Option<Uuid>,
}
