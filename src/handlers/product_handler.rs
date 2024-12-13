use actix_web::HttpResponse;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use crate::entities::product ; 
use crate::DbPool;
use crate::errors::api_error::ApiError;
use serde::{Deserialize, Serialize}; 

// Define the claims structure
#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String, // Subject (user ID or username)
    exp: usize,  // Expiration time
}

// Function to get a product by ID with JWT authorization
pub async fn get_product(db: &DbPool, product_id: i32) -> Result<HttpResponse, ApiError> {
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
            return Err(ApiError::ObjectNotFoundError);
        }
        Err(err) => {
            return Err(ApiError::DatabaseError { db_err: (err) });
        }
    }  
}

pub async fn get_all_products(db: &DbPool) -> Result<HttpResponse, ApiError> {
    // Fetch all products from the database
    let products = product::Entity::find()
        .all(db.as_ref()) // Retrieve all products
        .await;

    match products {
        Ok(products) => {
            if products.is_empty() {
                return Err(ApiError::ObjectNotFoundError); // Return error if no products found
            }
            return Ok(HttpResponse::Ok().json(products)); // Return the list of products
        }
        Err(err) => {
            return Err(ApiError::DatabaseError { db_err: err });
        }
    }
}