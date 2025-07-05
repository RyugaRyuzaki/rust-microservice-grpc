use crate::errors::{AppError, AppErrorCode};
use dev_proto::generated::error_proto::ProtoError;

impl From<AppError> for ProtoError {
    fn from(err: AppError) -> Self {
        ProtoError {
            message: err.message,
            code: err.code.0,
            service_name: env!("CARGO_PKG_NAME").to_string(),
            error_type: match err.code {
                AppErrorCode(1001) => "INTERNAL_ERROR",
                AppErrorCode(2001) => "INVALID_INPUT",
                AppErrorCode(3001) => "NOT_AUTHORIZED",
                AppErrorCode(4001) => "NOT_FOUND",
                AppErrorCode(5001) => "CACHE",
                AppErrorCode(6001) => "DATABASE_ERROR",
                AppErrorCode(7001) => "CONFLICT",
                AppErrorCode(8001) => "BAD_REQUEST",
                AppErrorCode(9001) => "PERMISSION_DENIED",
                _ => "UNKNOWN",
            }
            .to_string(),
            details: vec![],
            metadata: std::collections::HashMap::new(),
        }
    }
}

impl From<ProtoError> for AppError {
    fn from(proto: ProtoError) -> Self {
        AppError {
            message: proto.message,
            code: AppErrorCode(proto.code),
        }
    }
}

pub trait ErrorProtoExt {
    fn to_proto(&self) -> ProtoError;
    fn from_proto(proto: ProtoError) -> Self;
}

impl From<&AppError> for ProtoError {
    fn from(err: &AppError) -> Self {
        ProtoError {
            message: err.message.clone(),
            code: err.code.0,
            service_name: env!("CARGO_PKG_NAME").to_string(),
            error_type: match err.code {
                AppErrorCode(1001) => "INTERNAL_ERROR",
                AppErrorCode(2001) => "INVALID_INPUT",
                AppErrorCode(3001) => "NOT_AUTHORIZED",
                AppErrorCode(4001) => "NOT_FOUND",
                AppErrorCode(5001) => "CACHE",
                AppErrorCode(6001) => "DATABASE_ERROR",
                AppErrorCode(7001) => "CONFLICT",
                AppErrorCode(8001) => "BAD_REQUEST",
                AppErrorCode(9001) => "PERMISSION_DENIED",
                _ => "UNKNOWN",
            }
            .to_string(),
            details: vec![],
            metadata: std::collections::HashMap::new(),
        }
    }
}

impl ErrorProtoExt for AppError {
    fn to_proto(&self) -> ProtoError {
        self.into()
    }

    fn from_proto(proto: ProtoError) -> Self {
        proto.into()
    }
}
