use std::env;

use actix_web::{web, HttpResponse, Error};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use crate::entities::product; // Import the product entity
use crate::DbPool; // Import the DbPool type
use serde::{Deserialize, Serialize}; // Import for JWT claims

// Define the claims structure
#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String, // Subject (user ID or username)
    exp: usize,  // Expiration time
}

// Function to get a product by ID with JWT authorization
pub async fn get_product(db: &DbPool, product_id: i32, token: Option<String>) -> Result<HttpResponse, HttpResponse> {
    // Check for the token
    let token = token.ok_or(HttpResponse::Unauthorized())?;
    println!("Token: {}", token);
    // Decode the token
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    println!("JWT_SECRET: {}", secret_key); // Debug print
    let decoding_key = DecodingKey::from_secret(secret_key.as_ref()); // Use the secret key from the environment
    let validation = Validation::new(Algorithm::HS256); // Ensure the algorithm matches

    // let claims_result = decode::<Claims>(&token, &decoding_key, &validation);
    // match claims_result {
    //     Ok(decoded) => {
    //         let claims = decoded.claims;
    //         println!("Claims: {:?}", claims); // Log the claims for debugging
    //     },
    //     Err(err) => {
    //         println!("Token decode error: {:?}", err); // Log the error
    //         return Err(HttpResponse::Unauthorized().finish());
    //     }
    // }

    

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