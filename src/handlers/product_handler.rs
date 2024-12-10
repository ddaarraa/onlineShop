use actix_web::HttpResponse;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use crate::entities::product; 
use crate::DbPool; 
use serde::{Deserialize, Serialize}; 

// Define the claims structure
#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String, // Subject (user ID or username)
    exp: usize,  // Expiration time
}

// Function to get a product by ID with JWT authorization
pub async fn get_all_product(db: &DbPool, product_id: i32) -> Result<HttpResponse, HttpResponse> {
    // Fetch the product from the database
    let product = product::Entity::find()
        .filter(product::Column::Id.eq(product_id)) // Filter by product ID
        .one(db.as_ref())
        .await
        .map_err(|_| HttpResponse::InternalServerError())?
        .ok_or(HttpResponse::NotFound())?;

    // Return the product as JSON
    Ok(HttpResponse::Ok().json(product))
}