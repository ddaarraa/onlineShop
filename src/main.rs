use actix_web::{ middleware::{self}, web, App, HttpServer};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

mod migrator;
mod entities;
mod handlers;
mod db;
mod routes;
mod models;
mod middlewares;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv::dotenv().ok();// load env variables
    
    let db: DbPool = db::config::database_connection().await; // Create a connection pool

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(routes::user_route::create_user)
            .service(routes::user_route::login_user)
            .service(
                web::scope("/products")
                .wrap(middleware::from_fn(middlewares::auth::auth))
                .service(routes::product_routes::get_all_product)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

type DbPool = Arc<DatabaseConnection>; 




