use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error,
};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sea_orm::sqlx::types::chrono;
use sha2::Sha256;
use std::collections::BTreeMap;
use crate::{config, middlewares::error::AuthError};

pub async fn auth(req: ServiceRequest, next: Next<impl MessageBody>) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // Extract the Authorization header
    let token = req.headers().get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .map(|h| h.to_string());

    if let Some(token) = token {
        // Verify the token
        let secret_key = config::env_config::get_jwt_secret_from_config();

        let secret_key = match secret_key {
            Ok(secret_key) => secret_key,
            Err(_) => return Err(AuthError::InternalError.into())
        };
        let key: Hmac<Sha256> = Hmac::new_from_slice(secret_key.as_bytes()).expect("Invalid key length");

        // Verify the token and extract claims
        let claims_result: Result<BTreeMap<String, String>, _> = token.verify_with_key(&key);

        match claims_result {
            Ok(claims) => {
    
                if let Some(exp) = claims.get("exp") {

                    if let Ok(exp_time) = exp.parse::<u64>() {
                        let now = chrono::Utc::now().timestamp() as u64;
                        if exp_time > now {
                            next.call(req).await 
                        } else {
                            return Err(AuthError::ExpiredToken.into());
                        }
                    } else {
                        return Err(AuthError::InternalError.into());
                    }
                } else {
                    return Err(AuthError::InternalError.into())
                }
            }
            Err(_) => {
                return Err(AuthError::TokenVerificationFailed.into());
            }
        }

    } else {
        // No token provided
        Err(AuthError::MissingToken.into())
    }
}



