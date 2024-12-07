use sea_orm::{ActiveModelTrait, DbErr, Set};
use crate::{entities, DbPool}; // Import your entities module
use serde::Deserialize; // Import Deserialize trait
use bcrypt::{hash, DEFAULT_COST}; // Add this import

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