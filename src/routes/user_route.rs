
use actix_web::{get, post, web, HttpResponse, HttpRequest};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use crate::handlers;
use std::sync::Arc;

type DbPool = Arc<DatabaseConnection>;

#[derive(Deserialize)]
struct LoginInfo {
    username: String,
    password: String,
}
#[post("/users")]
pub async fn create_user(db: web::Data<DbPool>, new_user: web::Json<handlers::user_handler::NewUser>) -> HttpResponse {
    match handlers::user_handler::insert_user(&db, new_user.into_inner()).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/login")]
pub async fn login_user(db: web::Data<DbPool>, login_info: web::Json<LoginInfo>) -> HttpResponse {
    match handlers::user_handler::login_user(&db, &login_info.username, &login_info.password).await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(_) => HttpResponse::Unauthorized().finish(), 
    }
}