use crate::impl_crud_repo;
use database::models::user::{User, UserInsertable};
use database::schema::users;
use diesel::{QueryDsl, RunQueryDsl, ExpressionMethods};
use foxtive::database::ext::OptionalResultExt;
use foxtive::prelude::{AppResult, AppStateExt, IntoAppResult};
use foxtive::FOXTIVE;
use database::enums::{UserRole, UserStatus};

pub struct UserRepository;

impl_crud_repo!(UserRepository, users, User, UserInsertable, "user", soft_delete);

impl UserRepository {
    /// Find user by email
    pub fn find_by_email(email_addr: &str) -> AppResult<Option<User>> {
        use database::schema::users::dsl::*;
        
        users
            .filter(email.eq(email_addr))
            .first(&mut FOXTIVE.db_conn()?)
            .optional()
    }

    /// Find users by role
    pub fn find_by_role(role: UserRole) -> AppResult<Vec<User>> {
        users::table
            .filter(users::role.eq(role))
            .get_results(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }

    /// Find users by status
    pub fn find_by_status(status: UserStatus) -> AppResult<Vec<User>> {
        users::table
            .filter(users::status.eq(status))
            .get_results(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }

    /// Find active users
    pub fn find_active() -> AppResult<Vec<User>> {
        users::table
            .filter(users::status.eq(UserStatus::Active))
            .get_results(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }

    /// Check if email exists
    pub fn email_exists(email_addr: &str) -> AppResult<bool> {
        use diesel::dsl::exists;
        use diesel::prelude::*;
        use foxtive::prelude::IntoAppResult;
        use database::schema::users::dsl::*;

        let query = users.filter(email.eq(email_addr));
        
        diesel::select(exists(query))
            .first(&mut FOXTIVE.db_conn()?)
            .into_app_result()
    }
}
