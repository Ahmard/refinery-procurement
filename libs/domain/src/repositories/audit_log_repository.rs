use crate::impl_crud_repo;
use database::models::audit_log::{AuditLog, AuditLogInsertable};
use database::schema::audit_logs;
use foxtive::FOXTIVE;
use foxtive::prelude::{AppResult, AppStateExt};
use diesel::QueryDsl;

pub struct AuditLogRepository;

impl_crud_repo!(
    AuditLogRepository,
    audit_logs,
    AuditLog,
    AuditLogInsertable,
    "audit_log"
);

impl AuditLogRepository {
    pub fn find_by_user_id(user_id: uuid::Uuid) -> AppResult<Vec<AuditLog>> {
        use diesel::prelude::*;
        use foxtive::prelude::IntoAppResult;

        audit_logs::table
            .filter(audit_logs::user_id.eq(user_id))
            .load(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }

    pub fn find_by_target(
        target_entity: String,
        target_id: String,
    ) -> AppResult<Vec<AuditLog>> {
        use diesel::prelude::*;
        use foxtive::prelude::IntoAppResult;

        audit_logs::table
            .filter(audit_logs::target_entity.eq(target_entity))
            .filter(audit_logs::target_id.eq(target_id))
            .load(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }
}
