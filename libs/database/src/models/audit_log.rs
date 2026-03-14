use crate::schema::audit_logs;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize, Queryable, Identifiable, Selectable, AsChangeset)]
#[diesel(table_name = audit_logs)]
pub struct AuditLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub action: String,
    pub target_entity: String,
    pub target_id: String,
    pub changes: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = audit_logs)]
pub struct AuditLogInsertable {
    pub user_id: Uuid,
    pub action: String,
    pub target_entity: String,
    pub target_id: String,
    pub changes: Option<serde_json::Value>,
}
