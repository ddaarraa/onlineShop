use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

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