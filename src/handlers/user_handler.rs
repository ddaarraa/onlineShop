use sea_orm::{sqlx::types::chrono, ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use actix_web::HttpResponse;
use crate::{entities, DbPool}; // Import your entities module
use bcrypt::{hash, DEFAULT_COST};
use jwt::SignWithKey; // Import JWT encoding
use serde::Serialize; // Import Serialize and Deserialize traits
use std::env; // Import env for accessing environment variables
use std::collections::BTreeMap;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use crate::models;
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

pub async fn insert_user(db: &DbPool, new_user: models::user::User) -> Result<HttpResponse, models::user::InsertUserError> {
    // Validate the username and password
    if new_user.username.is_empty() {
        return Err(models::user::InsertUserError::ValidationError { field: "username".to_string() });
    }
    if new_user.password.is_empty() {
        return Err(models::user::InsertUserError::ValidationError { field: "password".to_string() });
    }

    let clone_user_name = new_user.username.clone();

    // Hash the password
    let hashed_password = hash(new_user.password, DEFAULT_COST)
        .map_err(|_| models::user::InsertUserError::HashedpasswordError)?;

    let user = entities::user::ActiveModel {
        username: Set(new_user.username),
        password: Set(hashed_password),
        ..Default::default()
    };
    let cloned_user = user.clone();

    // Attempt to insert the user into the database
    cloned_user.insert(db.as_ref()).await.map_err(|_| models::user::InsertUserError::DatabaseError)?;

    Ok(HttpResponse::Created().json(format!("User {} created successfully", clone_user_name)))
}


pub async fn login_user(db: &DbPool, username: &str, password: &str) -> Result<HttpResponse, models::user::LoginUserError> {
    // Fetch the user from the database
    let user = entities::user::Entity::find()
        .filter(entities::user::Column::Username.eq(username))
        .one(db.as_ref())
        .await;

    match user {
        Ok(Some(user)) => {

            // Verify the password
            if user.verify_password(password) {
                let token_generator = move || -> String {
                    
                    let claims = Claims {
                        sub: user.username.clone(),
                        exp: (chrono::Utc::now().timestamp() + 3600) as usize, // 1 hour expiration
                    };

                    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
                    let key: Hmac<Sha256> = Hmac::new_from_slice(secret_key.as_bytes()).expect("Invalid key length");

                    let mut claims_map = BTreeMap::new();
                    claims_map.insert("sub", claims.sub);
                    claims_map.insert("exp", claims.exp.to_string());

                    return claims_map.sign_with_key(&key).expect("Failed to sign token");
                };
                
                let token = token_generator();

                let response = Response{
                    token : token.to_string(),
                    body : "login successfully".to_string()
                };
                return Ok(HttpResponse::Ok().json(response));
            }else{
                return Err(models::user::LoginUserError::InvalidPassword);
            }
        }
        Ok(None) => return Err(models::user::LoginUserError::UserNotFound),
        Err(_) => return Err(models::user::LoginUserError::InternalError),
    }
}