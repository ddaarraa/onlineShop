
use actix_web::{get, post, web, HttpResponse};
use sea_orm::DatabaseConnection;
use crate::{handlers,models};
use std::sync::Arc;


type DbPool = Arc<DatabaseConnection>;


#[post("/users")]
pub async fn create_user(db: web::Data<DbPool>, new_user: web::Json<models::user::User>) -> HttpResponse {
    match handlers::user_handler::insert_user(&db, new_user.into_inner()).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/login")]
pub async fn login_user(db: web::Data<DbPool>, login_info: web::Json<models::user::User>) -> HttpResponse {
    match handlers::user_handler::login_user(&db, &login_info.username, &login_info.password).await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(_) => HttpResponse::Unauthorized().finish(), 
    }
}