use actix_web::{get, web, HttpResponse, HttpRequest};
use sea_orm::DatabaseConnection;
use crate::{handlers, models};
use std::sync::Arc;

type DbPool = Arc<DatabaseConnection>; 

#[get("/products/{id}")]
async fn get_all_product(db: web::Data<DbPool>, product_id: web::Path<models::product::ProductId>, req: HttpRequest) -> HttpResponse {
    let token = req.headers().get("Authorization").and_then(|h| h.to_str().ok()).map(|h| h.to_string());

    match handlers::product_handler::get_all_product(&db, product_id.id, token).await {
        Ok(response) => response,
        Err(response) => response,
    }
}