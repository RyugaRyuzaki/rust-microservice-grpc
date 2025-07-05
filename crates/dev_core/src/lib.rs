pub mod errors;
use crate::errors::AppError;
pub mod error_proto;
use actix_web::HttpResponse;
use color_eyre::Result;

pub type AppResult<T> = Result<T, AppError>;
pub type AppResponse = AppResult<HttpResponse>;
pub use error_proto::ErrorProtoExt;
