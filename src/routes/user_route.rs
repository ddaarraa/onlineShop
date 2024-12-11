
use actix_web::{post, web, HttpResponse};
use sea_orm::DatabaseConnection;
use crate::{errors::api_error::ApiError, handlers, models};
use std::sync::Arc;


type DbPool = Arc<DatabaseConnection>;


#[post("/register")]
pub async fn create_user(db: web::Data<DbPool>, new_user: web::Json<models::user::User>) -> Result<HttpResponse , ApiError> {
    let response = handlers::user_handler::insert_user(&db, new_user.into_inner()).await?;
    Ok(response)
}

#[post("/login")]
pub async fn login_user(db: web::Data<DbPool>, login_info: web::Json<models::user::User>) -> Result<HttpResponse , ApiError> { 
    let response = handlers::user_handler::login_user(&db, &login_info.username, &login_info.password).await?;
    Ok(response)
}