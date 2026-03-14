#![allow(dead_code)]

use crate::dto::auth_dto::{AuthLoginForm, JwtTokenClaims, UserCacheData};
use crate::repositories::user_repository::UserRepository;
use chrono::{Duration, Utc};
use foxtive::helpers::jwt::AuthTokenData;
use foxtive::helpers::run_async;
use foxtive::prelude::AppStateExt;
use foxtive::results::AppResult;
use foxtive::{FOXTIVE, bad_request};
use tracing::info;
use uuid::Uuid;
use database::enums::UserStatus;
use crate::APP_CODE;
use crate::repositories::SupplierRepository;

pub struct AuthService;

impl AuthService {
    pub fn login(form: AuthLoginForm) -> AppResult<AuthTokenData> {
        info!("[{}] Finding user...", form.identifier);
        let user = match UserRepository::find_by_email(&form.identifier)? {
            Some(user) => user,
            None => return Err(bad_request!("Invalid email or password")),
        };

        if user.status.eq(&UserStatus::Inactive) {
            return Err(bad_request!("Your account has been temporarily suspended"));
        }

        if user.status.eq(&UserStatus::Suspended) {
            return Err(bad_request!("Your account has been suspended"));
        }

        // Capture other generic non-active statuses we have misses
        if user.status.ne(&UserStatus::Active) {
            return Err(bad_request!("Your account is currently not activated"));
        }

        info!("[{}] Verifying password...", form.identifier);
        if !FOXTIVE
            .helpers()
            .password
            .verify(&user.password_hash, &form.password)?
        {
            return Err(bad_request!("Invalid email or password"));
        }

        let claims = Self::make_jwt_claim(user.id);
        let jti = claims.jti;
        let token = FOXTIVE
            .app()
            .helpers
            .jwt
            .generate(claims)?;

        info!("[{}] Making cache data...", form.identifier);
        let auth_data = UserCacheData {
            jti: Some(jti),
            id: user.id,
            supplier_id: SupplierRepository::fetch_id_by_user_id(user.id)?,
            session_id: None,
            name: user.name,
            email: user.email,
            status: user.status,
            role: user.role,
        };

        run_async(
            FOXTIVE
                .cache()
                .put(&Self::cache_key(auth_data.id), &auth_data),
        )?;

        Ok(token)
    }

    fn make_jwt_claim(user_id: Uuid) -> JwtTokenClaims {
        let token_lifetime_in_minutes = FOXTIVE.app().jwt_token_lifetime;

        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + Duration::minutes(token_lifetime_in_minutes)).timestamp() as usize;

        JwtTokenClaims {
            iat,
            exp,
            jti: Uuid::new_v4(),
            iss: APP_CODE.to_string(),
            sub: user_id,
        }
    }

    pub fn cache_key(id: Uuid) -> String {
        format!("USER::{id}")
    }
}
