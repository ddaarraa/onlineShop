use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::Display;
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
use serde_json;
use crate::middlewares::error::AuthError;

#[derive(Debug, Display)]
pub enum ApiError {
    #[display("Validation error")]
    ValidationError { details: Vec<ValidationFieldError> },
    
    #[display("Database error: {db_err}")]
    DatabaseError { db_err: DbErr },
    
    #[display("Authentication/Authorization error: {detail}")]
    AuthError { detail: String },

    #[display("Object not found")]
    ObjectNotFoundError,
    
    #[display("Internal server error: {detail}")]
    InternalServerError { detail: String },
}

// Struct for holding individual field validation errors
#[derive(Serialize, Deserialize, Debug)]
pub struct ValidationFieldError {
    pub field: String,
    pub message: String,
}

impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let response_body = match self {
            ApiError::ValidationError { details } => {
                // For validation errors, return each error field separately
                serde_json::json!({
                    "error": self.to_string(),
                    "details": details,
                })
            }
            ApiError::DatabaseError { db_err } => {
                // For database errors, include the error message
                serde_json::json!({
                    "error": self.to_string(),
                    "details": db_err.to_string(),
                })
            }
            ApiError::AuthError { detail } => {
                // For auth errors, include the error message
                serde_json::json!({
                    "error": self.to_string(),
                    "details": detail,
                })
            }
            ApiError::ObjectNotFoundError => {
                // For object not found, no details need to be included
                serde_json::json!({
                    "error": self.to_string(),
                })
            }
            ApiError::InternalServerError { detail } => {
                // For internal server errors, include the error message
                serde_json::json!({
                    "error": self.to_string(),
                    "details": detail,
                })
            }
        };

        HttpResponse::build(self.status_code())
            .content_type("application/json")
            .json(response_body)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            ApiError::DatabaseError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ObjectNotFoundError => StatusCode::NOT_FOUND,
            ApiError::AuthError { .. } => StatusCode::UNAUTHORIZED,
            ApiError::InternalServerError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<AuthError> for ApiError {
    fn from(auth_error: AuthError) -> Self {
        match auth_error {
            AuthError::MissingToken => ApiError::AuthError {
                detail: "Missing token".to_string(),
            },
            AuthError::ExpiredToken => ApiError::AuthError {
                detail: "Expired token".to_string(),
            },
            AuthError::TokenVerificationFailed => ApiError::AuthError {
                detail: "Failed to verify token".to_string(),
            },
            AuthError::InternalError => ApiError::InternalServerError {
                detail: "Internal error occurred while handling token".to_string(),
            },
        }
    }
}
