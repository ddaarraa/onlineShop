use actix_web::{get, web, HttpResponse, HttpRequest};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use crate::handlers;
use std::sync::Arc;

type DbPool = Arc<DatabaseConnection>; 

#[derive(Deserialize)]
struct ProductId {
    id: i32,
}

#[get("/products/{id}")]
async fn get_all_product(db: web::Data<DbPool>, product_id: web::Path<ProductId>, req: HttpRequest) -> HttpResponse {
    let token = req.headers().get("Authorization").and_then(|h| h.to_str().ok()).map(|h| h.to_string());

    match handlers::product_handler::get_product(&db, product_id.id, token).await {
        Ok(response) => response,
        Err(response) => response,
    }
}