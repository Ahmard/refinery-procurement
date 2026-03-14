use chrono::NaiveDateTime;
use database::enums::{UserRole, UserStatus};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub status: UserStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub role: UserRole,
}

pub struct UserCreateDto {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
    pub status: Option<UserStatus>,
    pub created_by: Option<Uuid>,
}