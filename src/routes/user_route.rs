
use actix_web::{get, post, web, HttpResponse};
use sea_orm::DatabaseConnection;
use crate::{handlers,models::{self, user::{InsertUserError, LoginUserError}}};
use std::sync::Arc;


type DbPool = Arc<DatabaseConnection>;


#[post("/users")]
pub async fn create_user(db: web::Data<DbPool>, new_user: web::Json<models::user::User>) -> Result<HttpResponse , InsertUserError> {
    match handlers::user_handler::insert_user(&db, new_user.into_inner()).await {
        Ok(response) => Ok(response), // Return the successful response
        Err(err) => {
            match err {
                models::user::InsertUserError::ValidationError { field } => {
                    Err(models::user::InsertUserError::ValidationError { field /* value */ })
                }
                models::user::InsertUserError::HashedpasswordError => {
                    Err(models::user::InsertUserError::HashedpasswordError)
                }
                models::user::InsertUserError::DatabaseError => {
                    Err(models::user::InsertUserError::DatabaseError)
                }
            }
        }
    }
}

#[post("/login")]
pub async fn login_user(db: web::Data<DbPool>, login_info: web::Json<models::user::User>) -> Result<HttpResponse , LoginUserError> { 

    match handlers::user_handler::login_user(&db, &login_info.username, &login_info.password).await {
        Ok(response) => Ok(response),
        Err(err) => {
            match err {
                models::user::LoginUserError::UserNotFound => {
                    Err(models::user::LoginUserError::UserNotFound)
                }
                models::user::LoginUserError::InvalidPassword => {
                    Err(models::user::LoginUserError::InvalidPassword)
                }
                models::user::LoginUserError::InternalError => {
                    Err(models::user::LoginUserError::InternalError)
                }
            }
        }
    }   
}