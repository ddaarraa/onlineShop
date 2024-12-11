use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};
use actix_web::{HttpResponse, error};
use derive_more::Display;

#[derive(Serialize,Deserialize)]
pub struct Product {
    pub id: ProductId,
    pub name: String,
    pub description: Option<String>, // Optional field
    pub price: Decimal,
    pub stock: i32,
}
#[derive(Serialize,Deserialize)]
pub struct ProductId {
    pub id: i32,
}


#[derive(Debug, Display)]
pub enum getProductError {
    #[display("product not found")]
    ProductNotFound,
    #[display("Database error")]
    DatabaseError,
}

impl error::ResponseError for getProductError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .content_type("text/html")
            .body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            getProductError::ProductNotFound { .. } => actix_web::http::StatusCode::BAD_REQUEST,
            getProductError::DatabaseError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}