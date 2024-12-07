use sea_orm::{ActiveModelTrait, DbErr, Set};
use crate::{entities, DbPool}; // Import your entities module
use serde::Deserialize; // Import Deserialize trait

#[derive(Deserialize)] // Derive Deserialize for NewUser
pub struct NewUser {
    pub username: String,
    pub password: String,
}

pub async fn insert_user(db: &DbPool, new_user: NewUser) -> Result<(), DbErr> {
    let user = entities::user::ActiveModel {
        username: Set(new_user.username),
        password: Set(new_user.password),
        ..Default::default() // Fill in other fields if necessary
    };

    user.insert(db.as_ref()).await?;
    Ok(())
}