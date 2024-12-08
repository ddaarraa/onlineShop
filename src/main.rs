use actix_web::{ post, web, App, HttpServer, HttpResponse};
use sea_orm::{ sea_query::func, Database, DatabaseConnection};
use dotenv::dotenv;
use serde::Deserialize;
// use std::env;
use std::{env, sync::Arc};

mod migrator;
type DbPool = Arc<DatabaseConnection>; // Type alias for the database connection

mod entities;
mod handlers;
mod db;


#[post("/users")]
async fn create_user(db: web::Data<DbPool>, new_user: web::Json<handlers::user_handler::NewUser>) -> HttpResponse {
    match handlers::user_handler::insert_user(&db, new_user.into_inner()).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/login")]
async fn login_user(db: web::Data<DbPool>, login_info: web::Json<LoginInfo>) -> HttpResponse {
    match handlers::user_handler::login_user(&db, login_info.username.clone(), login_info.password.clone()).await {
        Ok(token) => HttpResponse::Ok().json(token), // Return the JWT token
        Err(_) => HttpResponse::Unauthorized().finish(), // Return 401 if login fails
    }
}

// Define a struct for login information
#[derive(Deserialize)]
struct LoginInfo {
    username: String,
    password: String,
}

#[actix_web::main]
// #[tokio::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok(); // Load environment variables from .env file
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // Create a connection pool
    let db: DbPool = Arc::new(Database::connect(&database_url).await.expect("Failed to connect to the database"));

    // Check for a command-line argument to determine if we should reset the database
    if let Err(err) = db::config::run(&db).await {
        panic!("{}", err);
    }

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone())) // Share the database connection pool
            .service(create_user)
            .service(login_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

    
}




