// use crate::errors::{user_error, product_error};
use actix_web::{error, http::StatusCode, HttpResponse};
use derive_more::Display;
use sea_orm::DbErr;
use serde_json;

// #[derive(Debug, Display)]
// pub enum ApiErrors {
//     #[display("Inserting User Error")]
//     InsertUserError(user_error::InsertUserError),

//     #[display("Database error: {details}")]
//     DatabaseError { details: String },

// }

// impl AppError for ApiErrors {
//     fn status_code(&self) -> StatusCode {
//         match self {
//             ApiErrors::InsertUserError(err) => AppError::status_code(err),
//             ApiErrors::DatabaseError {..} => StatusCode::INTERNAL_SERVER_ERROR,
//         }
//     }

//     fn error_response(&self) -> HttpResponse {
//         match self {
//             ApiErrors::InsertUserError(err) => AppError::error_response(err),
//             ApiErrors::DatabaseError{..}  => HttpResponse::InternalServerError()
//                 .content_type("text/html")
//                 .body(self.to_string()),
//         }
//     }
// }



// pub trait AppError: std::fmt::Debug + std::fmt::Display {
//     fn status_code(&self) -> StatusCode;

//     fn details(&self) -> Option<String> {
//         None
//     }

//     fn error_response(&self) -> HttpResponse {
//         let mut response_builder = HttpResponse::build(self.status_code()); // Create a longer-lived builder
    
//         if let Some(details) = self.details() {
//             response_builder.json(serde_json::json!({
//                 "error": self.to_string(),
//                 "details": details
//             }))
//         } else {
//             response_builder.json(serde_json::json!({
//                 "error": self.to_string(),
//             }))
//         }
//     }
// }

// impl ResponseError for ApiErrors {
//     fn status_code(&self) -> StatusCode {
//         AppError::status_code(self) // Delegate to `AppError`
//     }
//     fn error_response(&self) -> HttpResponse {
//         AppError::error_response(self) // Delegate to `AppError`
//     }
// }


#[derive(Debug, Display)]
pub enum ApiError {
    #[display("Validation error: {field}")]
    ValidationError{field: String},
    
    #[display("Database error: {db_err}")]
    DatabaseError{db_err: DbErr},
    
    #[display("Authentication/Authorization error: {detail}")]
    AuthError{detail: String},

    #[display("Object not found")]
    ObjectNotFoundError,
    
    // #[display("Application logic error: {detail}")]
    // LogicError{detail: String},
    
    #[display("Internal server error: {detail}")]
    InternalServerError{detail: String},
}

impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .content_type("application/json")
            .body(serde_json::json!({ "error": self.to_string() }).to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::ValidationError{..} => StatusCode::BAD_REQUEST,
            ApiError::DatabaseError{..} => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::ObjectNotFoundError => StatusCode::NOT_FOUND,
            ApiError::AuthError{..} => StatusCode::UNAUTHORIZED,
            // ApiError::LogicError{..} => StatusCode::BAD_REQUEST,
            ApiError::InternalServerError{..} => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
