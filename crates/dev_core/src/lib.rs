pub mod errors;
use crate::errors::AppError;
use actix_web::HttpResponse;
use color_eyre::Result;

pub type AppResult<T> = Result<T, AppError>;
pub type AppResponse = AppResult<HttpResponse>;
