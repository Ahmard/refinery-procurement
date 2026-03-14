use chrono::NaiveDateTime;
use database::models::audit_log::AuditLog;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLogResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub action: String,
    pub target_entity: String,
    pub target_id: String,
    pub changes: Option<serde_json::Value>,
    pub created_at: NaiveDateTime,
}

impl From<AuditLog> for AuditLogResponse {
    fn from(log: AuditLog) -> Self {
        Self {
            id: log.id,
            user_id: log.user_id,
            action: log.action,
            target_entity: log.target_entity,
            target_id: log.target_id,
            changes: log.changes,
            created_at: log.created_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AuditLogCreateRequest {
    pub user_id: Uuid,
    pub action: String,
    pub target_entity: String,
    pub target_id: String,
    pub changes: Option<serde_json::Value>,
}
