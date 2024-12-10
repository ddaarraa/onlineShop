
use actix_web::{HttpResponse, error};
use serde::{Deserialize, Serialize};
use derive_more::Display;
#[derive(Serialize,Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}


#[derive(Debug, Display)]
pub enum InsertUserError {
    #[display("Validation error on field: {field}")]
    ValidationError { field: String },
    #[display("Database error")]
    DatabaseError,
    #[display("Hashedpassword error")]
    HashedpasswordError,
}

impl error::ResponseError for InsertUserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .content_type("text/html")
            .body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            InsertUserError::ValidationError { .. } => actix_web::http::StatusCode::BAD_REQUEST,
            InsertUserError::DatabaseError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            InsertUserError::HashedpasswordError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Debug, Display)]
pub enum LoginUserError {
    #[display("User not found")]
    UserNotFound,
    #[display("Invalid password")]
    InvalidPassword,
    #[display("Database error")]
    DatabaseError,
}

impl error::ResponseError for LoginUserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .content_type("text/html")
            .body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            LoginUserError::UserNotFound => actix_web::http::StatusCode::UNAUTHORIZED,
            LoginUserError::InvalidPassword => actix_web::http::StatusCode::UNAUTHORIZED,
            LoginUserError::DatabaseError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}