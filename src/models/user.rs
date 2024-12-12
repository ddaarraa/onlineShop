
use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Serialize,Validate,Deserialize)]
pub struct User {
    #[validate(length(min = 1, message = "Username cannot be empty"))]
    pub username: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
}


