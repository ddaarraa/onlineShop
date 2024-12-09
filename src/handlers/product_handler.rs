use std::{collections::BTreeMap, env};

use actix_web::{web, HttpResponse, Error};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
// use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use sea_orm::{sqlx::types::chrono, ColumnTrait, EntityTrait, QueryFilter};
use sha2::Sha256;
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
    // let trimmed_token = token.trim();
    let token = token.strip_prefix("Bearer ").ok_or(HttpResponse::Unauthorized())?;

    println!("Token: {}", token);
    // Decode the token
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    println!("JWT_SECRET: {}", secret_key); // Debug print
    let key: Hmac<Sha256> = Hmac::new_from_slice(secret_key.as_bytes()).expect("Invalid key length");
    // let decoding_key = DecodingKey::from_secret(secret_key.as_ref()); // Use the secret key from the environment
    // let validation = Validation::new(Algorithm::HS256); // Ensure the algorithm matches
    let claims_result: Result<BTreeMap<String, String>, _> = token.verify_with_key(&key);

    match claims_result {
        Ok(claims) => {
            println!("Token verified successfully!");
            
            // Access specific claims
            if let Some(sub) = claims.get("sub") {
                println!("Subject (sub): {}", sub);
            } else {
                println!("No 'sub' claim found");
            }

            if let Some(exp) = claims.get("exp") {
                println!("Expiration (exp): {}", exp);

                // Validate expiration
                if let Ok(exp_time) = exp.parse::<u64>() {
                    let now = chrono::Utc::now().timestamp() as u64;
                    if exp_time > now {
                        println!("Token is still valid.");
                    } else {
                        println!("Token has expired.");
                    }
                } else {
                    println!("Invalid 'exp' format.");
                }
            } else {
                println!("No 'exp' claim found");
            }
        }
        Err(err) => {
            println!("Failed to verify token: {:?}", err);
            return Err(HttpResponse::Unauthorized().body("Failed to verify token"));
        }
    }
    
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