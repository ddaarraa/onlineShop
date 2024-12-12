use sea_orm::{sqlx::types::chrono, ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use actix_web::HttpResponse;
use crate::{entities, errors::api_error::ApiError, models, DbPool, config}; // Import your entities module
use bcrypt::{hash, DEFAULT_COST};
use jwt::SignWithKey; // Import JWT encoding
use serde::Serialize; // Import Serialize and Deserialize traits
use std::collections::BTreeMap;
use hmac::{Hmac, Mac};
use sha2::Sha256;

#[derive(Serialize)]
struct Claims {
    sub: String, // Subject (user ID or username)
    exp: usize,  // Expiration time
}
#[derive(Serialize)]
struct Response {
    token: String,
    body: String,
}

pub async fn insert_user(db: &DbPool, new_user: models::user::User) -> Result<HttpResponse, ApiError > {
    
    if new_user.username.is_empty() {
        return Err(ApiError::ValidationError { field: "username".to_string() });
    }
    if new_user.password.is_empty() {
        return Err(ApiError::ValidationError { field: "password".to_string() });
    }

    let clone_user_name = new_user.username.clone();

    // Hash the password
    let hashed_password = hash(new_user.password, DEFAULT_COST)
        .map_err(|err| ApiError::InternalServerError { detail: err.to_string()})?;

    let user = entities::user::ActiveModel {
        username: Set(new_user.username),
        password: Set(hashed_password),
        ..Default::default()
    };

    let cloned_user = user.clone();

    // Attempt to insert the user into the database
    cloned_user.insert(db.as_ref()).await.map_err(|err| ApiError::DatabaseError{db_err: err})?;

    Ok(HttpResponse::Created().json(format!("User {} created successfully", clone_user_name)))
}


pub async fn login_user(db: &DbPool, username: &str, password: &str) -> Result<HttpResponse, ApiError> {
    // Fetch the user from the database
    let user = entities::user::Entity::find()
        .filter(entities::user::Column::Username.eq(username))
        .one(db.as_ref())
        .await;

    match user {
        Ok(Some(user)) => {
            // Verify the password
            if user.verify_password(password) {
                
                let token_generator = move || -> Result<String, ApiError> {
                    
                    let claims = Claims {
                        sub: user.username.clone(),
                        exp: (chrono::Utc::now().timestamp() + 3600) as usize, // 1 hour expiration
                    };

                    let secret_key = config::env_config::get_jwt_secret_from_config();

                    let secret_key = match secret_key {
                        Ok(secret_key) => secret_key,
                        Err(err) => return Err(ApiError::InternalServerError { detail: err.to_string()})
                    };
                    
                    let key = Hmac::<Sha256>::new_from_slice(secret_key.as_bytes())
                        .map_err(|_| ApiError::InternalServerError { detail: "Invalid key length for HMAC initialization".to_string() })?;

                    let mut claims_map = BTreeMap::new();
                    claims_map.insert("sub", claims.sub);
                    claims_map.insert("exp", claims.exp.to_string());

                    claims_map
                        .sign_with_key(&key)
                        .map_err(|_| ApiError::InternalServerError { detail: "Failed to sign token".to_string() })
                };
                
                match token_generator() {
                    Ok(token) => {
                        let response = Response {
                            token: token.to_string(),
                            body: "login successfully".to_string(),
                        };
                        Ok(HttpResponse::Ok().json(response))
                    }
                    Err(err) => Err(err),
                }
                
            }else{
                return Err(ApiError::AuthError { detail: "InvalidPassword".to_string()});
            }
        }
        Ok(None) => return Err(ApiError::AuthError { detail: "UserNotFound".to_string() }),
        Err(err) => return Err(ApiError::InternalServerError { detail: err.to_string()}),
    }
}