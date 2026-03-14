use crate::impl_crud_repo;
use chrono::Utc;
use database::models::auth_session::{AuthSession, AuthSessionInsertable};
use database::schema::auth_sessions;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use foxtive::database::ext::OptionalResultExt;
use foxtive::prelude::{AppResult, AppStateExt, IntoAppResult};
use foxtive::FOXTIVE;

pub struct AuthSessionRepository;

impl_crud_repo!(
    AuthSessionRepository,
    auth_sessions,
    AuthSession,
    AuthSessionInsertable,
    "auth_session"
);

impl AuthSessionRepository {
    /// Find auth sessions by user ID
    pub fn find_by_user_id(user_id: uuid::Uuid) -> AppResult<Vec<AuthSession>> {
        auth_sessions::table
            .filter(auth_sessions::user_id.eq(user_id))
            .get_results(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }

    /// Find active auth session by token
    pub fn find_by_token(token: &str) -> AppResult<Option<AuthSession>> {
        auth_sessions::table
            .filter(auth_sessions::token.eq(token))
            .filter(auth_sessions::expires_at.gt(Utc::now().naive_utc()))
            .first(&mut FOXTIVE.db_conn()?)
            .optional()
    }

    /// Delete expired auth sessions
    pub fn delete_expired() -> AppResult<usize> {
        use database::schema::auth_sessions::dsl::*;
        diesel::delete(auth_sessions.filter(expires_at.lt(Utc::now().naive_utc())))
            .execute(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }
}
