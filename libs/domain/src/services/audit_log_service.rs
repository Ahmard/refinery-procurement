use crate::dto::AuditLogCreateRequest;
use crate::repositories::AuditLogRepository;
use database::models::audit_log::{AuditLog, AuditLogInsertable};
use foxtive::prelude::AppResult;
use uuid::Uuid;

pub struct AuditLogService;

impl AuditLogService {
    pub fn record_activity(req: AuditLogCreateRequest) -> AppResult<AuditLog> {
        let new_log = AuditLogInsertable {
            user_id: req.user_id,
            action: req.action,
            target_entity: req.target_entity,
            target_id: req.target_id,
            changes: req.changes,
        };

        AuditLogRepository::create(new_log)
    }

    pub fn get_all_logs() -> AppResult<Vec<AuditLog>> {
        AuditLogRepository::all()
    }

    pub fn get_logs_by_user(user_id: Uuid) -> AppResult<Vec<AuditLog>> {
        AuditLogRepository::find_by_user_id(user_id)
    }

    pub fn get_logs_by_target(
        target_entity: String,
        target_id: String,
    ) -> AppResult<Vec<AuditLog>> {
        AuditLogRepository::find_by_target(target_entity, target_id)
    }
}
