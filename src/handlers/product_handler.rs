use actix_web::HttpResponse;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use crate::entities::product; 
use crate::DbPool;
use crate::models;
use serde::{Deserialize, Serialize}; 

// Define the claims structure
#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String, // Subject (user ID or username)
    exp: usize,  // Expiration time
}

// Function to get a product by ID with JWT authorization
pub async fn get_product(db: &DbPool, product_id: i32) -> Result<HttpResponse, models::product::getProductError> {
    // Fetch the product from the database
    let product = product::Entity::find()
        .filter(product::Column::Id.eq(product_id)) // Filter by product ID
        .one(db.as_ref())
        .await;

    match product {
        Ok(Some(product)) => {
            return Ok(HttpResponse::Ok().json(product))
        }
        Ok(None) => { 
            return Err(models::product::getProductError::ProductNotFound);
        }
        Err(_) => {
            return Err(models::product::getProductError::DatabaseError);
        }
    }
    
    
}