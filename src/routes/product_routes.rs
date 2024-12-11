use actix_web::{get, web, HttpResponse};
use sea_orm::DatabaseConnection;
use crate::{handlers, models::{self, product::GetProductError}};
use std::sync::Arc;

type DbPool = Arc<DatabaseConnection>; 

#[get("/{id}")]
async fn get_all_product(db: web::Data<DbPool>, product_id: web::Path<models::product::ProductId>) -> Result<HttpResponse , GetProductError> {
    match handlers::product_handler::get_product(&db, product_id.id).await {
        Ok(response) => Ok(response), 
        Err(err) => {
            match err {
                models::product::GetProductError::ProductNotFound => {
                    Err(models::product::GetProductError::ProductNotFound)
                }
                models::product::GetProductError::DatabaseError => {
                    Err(models::product::GetProductError::DatabaseError)
                }
            }
        }
    }
}
