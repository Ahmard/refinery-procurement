#![allow(dead_code)]

use database::enums::{UserRole, UserStatus};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, utoipa::ToSchema)]
pub struct AuthLoginForm {
    pub identifier: String,
    pub password: String,
}

#[derive(Clone, Debug)]
pub struct AuthId(pub Uuid);

#[derive(Clone, Debug)]
pub struct AuthSupplierId(pub Option<Uuid>);

#[derive(Clone)]
pub struct AuthIsAuthenticated;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtTokenClaims {
    /// Identifies the subject (user or entity) the token is about.
    pub sub: Uuid,
    /// Indicates when the token was issued. Useful for token freshness. (time in timestamp)
    pub iat: usize,
    /// Specifies when the token expires. Helps prevent token reuse.
    pub exp: usize,
    /// Identifies the entity that issued the token (e.g., authentication server).
    pub iss: String,
    /// A unique identifier for the token. Helps prevent replay attacks.
    pub jti: Uuid,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserCacheData {
    pub id: Uuid,
    pub supplier_id: Option<Uuid>,
    pub session_id: Option<Uuid>,
    pub jti: Option<Uuid>,
    pub name: String,
    pub email: String,
    pub status: UserStatus,
    pub role: UserRole,
}
