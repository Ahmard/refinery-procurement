#![allow(dead_code)]

use crate::dto::auth_dto::{JwtTokenClaims};
use foxtive::helpers::jwt::{Algorithm, Validation};
use foxtive::prelude::{AppResult, AppStateExt};
use foxtive::{FOXTIVE, unauthorized};
use tracing::debug;
use crate::APP_CODE;

pub fn cleanup_bearer_token(token: Option<&str>) -> AppResult<String> {
    token
        .and_then(|h| {
            let h = h.trim(); // Handle leading/trailing whitespace
            let (scheme, rest) = h.split_once(char::is_whitespace)?;
            (scheme.eq_ignore_ascii_case("bearer"))
                .then(|| rest.trim())
                .filter(|t| !t.is_empty()) // Reject empty tokens
        })
        .map(|t| t.to_string())
        .ok_or(unauthorized!("Invalid or missing Bearer token"))
}

pub fn get_cache_key(token: &str) -> AppResult<String> {
    let is_pat = token.starts_with("pat_");
    let is_sk = token.starts_with("live_sk_") || token.starts_with("test_sk_");

    match is_pat || is_sk {
        // pat or src secret key
        true => Ok(token.to_owned()),
        false => {
            if token.starts_with("ey") {
                // jwt
                Ok(get_jwt_claims(token)?.sub.to_string())
            } else {
                let spl_token: Vec<&str> = token.split(':').collect();
                let client_id = spl_token
                    .first()
                    .ok_or(unauthorized!("Invalid auth token"))?;

                Ok(client_id.to_string())
            }
        }
    }
}

pub fn get_jwt_claims(token: &str) -> AppResult<JwtTokenClaims> {
    let jwt = FOXTIVE.app().helpers.jwt.clone();

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&[APP_CODE]);
    validation.set_audience(&[APP_CODE]);

    let result = jwt.decode::<JwtTokenClaims>(token, &validation);
    let claims = match result {
        Ok(decoded) => decoded.claims,
        Err(err) => {
            debug!("invalid token({token}): {err:?}");

            return Err(unauthorized!("Invalid token, please provide valid token"));
        }
    };

    Ok(claims)
}

pub fn make_cache_key(app_code: &str, client_id: &str) -> String {
    format!("{}::{}", app_code.to_uppercase(), client_id)
}
