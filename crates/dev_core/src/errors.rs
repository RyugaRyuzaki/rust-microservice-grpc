use actix_web::error::ResponseError;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use color_eyre::Report;
use redis::ErrorKind;
use serde::{Serialize, Serializer};
use serde_json::Error as SerdeJsonError;
use sqlx::Error as SqlxError;
use std::{convert::From, error::Error as StdError, fmt::Formatter};
use tracing::error;
use validator::ValidationErrors;

#[derive(Debug, Serialize)]
pub struct AppError {
    pub message: String,    // The error message
    pub code: AppErrorCode, // The error code that classifies the type of error
}

impl AppError {
    // Predefined error codes
    pub const INTERNAL_ERROR: AppErrorCode = AppErrorCode(1001);
    pub const INVALID_INPUT: AppErrorCode = AppErrorCode(2001);
    pub const NOT_AUTHORIZED: AppErrorCode = AppErrorCode(3001);
    pub const NOT_FOUND: AppErrorCode = AppErrorCode(4001);
    pub const CACHE: AppErrorCode = AppErrorCode(5001);
    pub const DATABASE_ERROR: AppErrorCode = AppErrorCode(6001); // Error code for bcrypt errors
    pub const CONFLICT: AppErrorCode = AppErrorCode(7001);
    pub const BAD_REQUEST: AppErrorCode = AppErrorCode(8001);
    pub const PERMISSION_DENIED: AppErrorCode = AppErrorCode(9001);
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AppErrorCode(pub i32);

impl AppErrorCode {
    // Create an AppError from an AppErrorCode with a custom message
    pub fn message(&self, message: String) -> AppError {
        AppError {
            message,
            code: self.clone(),
        }
    }

    // Return a default AppError based on the AppErrorCode
    pub fn default(self) -> AppError {
        let message = match self {
            AppErrorCode(2001) => "Invalid input.",
            AppErrorCode(3001) => "Invalid username or password provided",
            AppErrorCode(3002) => "Not authorized.",
            AppErrorCode(4001) => "Item not found.",
            AppErrorCode(5001) => "Bcrypt error.",
            AppErrorCode(6001) => "Bcrypt error.",
            AppErrorCode(9001) => "You don't have permission, ask the administrator",
            _ => "An unexpected error has occurred or token was expired", // Default message for unexpected errors
        };
        AppError {
            message: message.to_string(),
            code: self,
        }
    }
}

// implement std
impl StdError for AppError {}

// Implement From trait for converting AppErrorCode to AppError
impl From<AppErrorCode> for AppError {
    fn from(error: AppErrorCode) -> Self {
        error.default()
    }
}

// Serialize AppErrorCode to an integer for easier transmission over HTTP
impl Serialize for AppErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.0)
    }
}

// Convert color_eyre Report errors into AppError
impl From<Report> for AppError {
    fn from(e: Report) -> Self {
        error!("{:?}", e); // Log the Report error
        Self::INTERNAL_ERROR.message("An unexpected error occurred.".to_string())
        // Return a generic internal error message
    }
}

// Implement ResponseError trait for AppError to map it to HTTP status codes
impl ResponseError for AppError {
    // Map AppErrorCode to appropriate HTTP status codes
    fn status_code(&self) -> StatusCode {
        match self.code {
            AppErrorCode(2001) => StatusCode::BAD_REQUEST, // Map invalid input error to BAD_REQUEST (400)
            AppErrorCode(4001) => StatusCode::NOT_FOUND,   // Map not found error to NOT_FOUND (404)
            AppErrorCode(3001) => StatusCode::UNAUTHORIZED, // Map invalid credentials error to UNAUTHORIZED (401)
            AppErrorCode(3002) => StatusCode::UNAUTHORIZED, // Map not authorized error to UNAUTHORIZED (401)
            AppErrorCode(5001) => StatusCode::INTERNAL_SERVER_ERROR, // Map bcrypt error to INTERNAL_SERVER_ERROR (500)
            _ => StatusCode::INTERNAL_SERVER_ERROR, // Default to internal server error for any unknown error codes
        }
    }

    // Build an HTTP response with the appropriate status code and error message
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self) // Respond with the JSON representation of the error
    }
}

// Implement Display trait to provide a string representation of the AppError
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self) // Display the Debug representation of the AppError
    }
}

// Convert SQLx errors into AppError
impl From<SqlxError> for AppError {
    fn from(err: SqlxError) -> Self {
        error!("SQLx Error: {:?}", err); // Log the SQLx error
        AppError::INTERNAL_ERROR.message(format!("Database error: {:?}", err)) // Return an internal error with the database error message
    }
}
// Convert SQLx errors into AppError
impl From<ErrorKind> for AppError {
    fn from(err: ErrorKind) -> Self {
        error!("SQLx Error: {:?}", err); // Log the SQLx error
        AppError::INTERNAL_ERROR.message(format!("Database error: {:?}", err)) // Return an internal error with the database error message
    }
}
// Convert actix_web into AppError
impl From<actix_web::Error> for AppError {
    fn from(err: actix_web::Error) -> Self {
        AppError::INVALID_INPUT.message(format!("INVALID_INPUT: {:?}", err)) // Return an internal error with the database error message
    }
}

impl From<ValidationErrors> for AppError {
    fn from(errors: ValidationErrors) -> Self {
        let mut error_messages = Vec::new();

        for (field, error) in errors.field_errors() {
            let error_detail = format!("Field '{}' is invalid: {:?}", field, error);
            error_messages.push(error_detail);
        }

        AppError::INVALID_INPUT.message(error_messages.join("; "))
    }
}
impl From<SerdeJsonError> for AppError {
    fn from(err: SerdeJsonError) -> Self {
        error!("JSON Error: {:?}", err);
        AppError::INTERNAL_ERROR.message(format!("JSON serialization error: {:?}", err))
    }
}

impl AppError {
    pub fn invalid_uuid(id: &str) -> AppError {
        AppError::BAD_REQUEST.message(format!("Invalid UUID: {:?}", id))
    }

    pub fn required_field(field: &str) -> AppError {
        AppError::BAD_REQUEST.message(format!("Require Field: {:?}", field))
    }
    pub fn already_exists(field: String) -> AppError {
        AppError::BAD_REQUEST.message(format!("Item already exists: {:?}", field))
    }
}
