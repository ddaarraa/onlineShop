
use actix_web::{get, post, web, HttpResponse};
use sea_orm::DatabaseConnection;
use crate::{handlers,models};
use std::sync::Arc;


type DbPool = Arc<DatabaseConnection>;


#[post("/users")]
pub async fn create_user(db: web::Data<DbPool>, new_user: web::Json<models::user::User>) -> HttpResponse {
    match handlers::user_handler::insert_user(&db, new_user.into_inner()).await {
        Ok(response) => response, // Return the successful response
        Err(err) => {
            // Handle the error and return an appropriate HttpResponse
            match err {
                models::user::InsertUserError::ValidationError { field } => {
                    HttpResponse::BadRequest().body(format!("Validation error on field: {}", field))
                }
                models::user::InsertUserError::HashedpasswordError => {
                    HttpResponse::InternalServerError().body("Internal server error")
                }
                models::user::InsertUserError::DatabaseError => {
                    HttpResponse::InternalServerError().body("Database error")
                }
            }
        }
    }
}

#[post("/login")]
pub async fn login_user(db: web::Data<DbPool>, login_info: web::Json<models::user::User>) -> HttpResponse {  
    handlers::user_handler::login_user(&db, &login_info.username, &login_info.password).await
}