use sea_orm::{sqlx::types::chrono, ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, Set};
use crate::{entities, DbPool}; // Import your entities module
use serde::Deserialize; // Import Deserialize trait
use bcrypt::{hash, DEFAULT_COST}; // Add this import
use jsonwebtoken::{encode, EncodingKey, Header, Algorithm}; // Import JWT encoding
use serde::Serialize; // Import Serialize and Deserialize traits
// use crate::entities::user::Model; // Import the user model
use std::env; // Import env for accessing environment variables

#[derive(Deserialize)] // Derive Deserialize for NewUser
pub struct NewUser {
    pub username: String,
    pub password: String,
}

pub async fn insert_user(db: &DbPool, new_user: NewUser) -> Result<(), DbErr> {
    // Hash the password
    let hashed_password = hash(new_user.password, DEFAULT_COST).expect("Failed to hash password");

    let user = entities::user::ActiveModel {
        username: Set(new_user.username),
        password: Set(hashed_password), // Use the hashed password
        ..Default::default() // Fill in other fields if necessary
    };

    user.insert(db.as_ref()).await?;
    Ok(())
}

// Example function to verify password
// pub async fn verify_user_password(stored_hash: &str, password: &str) -> bool {
//     verify(password, stored_hash).unwrap_or(false)
// }

#[derive(Serialize)]
struct Claims {
    sub: String, // Subject (user ID or username)
    exp: usize,  // Expiration time
}

pub async fn login_user(db: &DbPool, username: String, password: String) -> Result<String, DbErr> {
    // Fetch the user from the database
    let user = entities::user::Entity::find()
        .filter(entities::user::Column::Username.eq(username))
        .one(db.as_ref())
        .await?
        .ok_or(DbErr::RecordNotFound("User not found".into()))?;

    // Verify the password
    if user.verify_password(&password) {
        // Create JWT claims
        let claims = Claims {
            sub: user.username.clone(),
            exp: (chrono::Utc::now().timestamp() + 3600) as usize, // 1 hour expiration
        };

        // Get the secret key from the environment variable
        let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        // println!("JWT_SECRET: {}", secret_key); // Debug print
        // Generate JWT
        let header = Header::new(Algorithm::HS256);
        let token = encode(&header, &claims, &EncodingKey::from_secret(secret_key.as_ref()))
            .map_err(|e| DbErr::Custom(format!("Failed to generate JWT: {:?}", e)))?;
        Ok(token)
    } else {
        Err(DbErr::Custom("Invalid password".into()))
    }
}