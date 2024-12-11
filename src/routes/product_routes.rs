use actix_web::{get, web, HttpResponse};
use sea_orm::DatabaseConnection;
use crate::{errors::api_error::ApiError, handlers, models};
use std::sync::Arc;

type DbPool = Arc<DatabaseConnection>; 

#[get("/{id}")]
async fn get_all_product(db: web::Data<DbPool>, product_id: web::Path<models::product::ProductId>) -> Result<HttpResponse , ApiError> {
    let response = handlers::product_handler::get_product(&db, product_id.id).await?;
    Ok(response)
}
