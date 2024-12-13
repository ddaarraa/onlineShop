use actix_web::{HttpResponse, error};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum AuthError {
    #[display("Missing token")]
    MissingToken,
    #[display("Expired token")]
    ExpiredToken,
    #[display("Failed to verify token")]
    TokenVerificationFailed,
    #[display("Internal Error")]
    InternalError,
}

impl error::ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .content_type("text/html")
            .body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            AuthError::MissingToken => actix_web::http::StatusCode::UNAUTHORIZED,
            AuthError::ExpiredToken => actix_web::http::StatusCode::UNAUTHORIZED,
            AuthError::TokenVerificationFailed => actix_web::http::StatusCode::UNAUTHORIZED,
            AuthError::InternalError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}