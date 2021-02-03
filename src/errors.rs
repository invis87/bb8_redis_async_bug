use bb8_redis::redis;
use std::convert::From;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub struct ApiError {
    err: anyhow::Error,
}

impl Debug for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_tuple("").field(&self.err).finish()
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({:#})", self.err)
    }
}

impl From<String> for ApiError {
    fn from(string: String) -> Self {
        let anyhow_err = anyhow::Error::msg(string);
        ApiError { err: anyhow_err }
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> ApiError {
        ApiError { err }
    }
}

impl From<std::io::Error> for ApiError {
    fn from(err: std::io::Error) -> ApiError {
        let msg = format!("io error: '{:#}'", err);
        let anyhow_err = anyhow::Error::msg(msg);
        ApiError { err: anyhow_err }
    }
}

impl std::convert::From<tonic::transport::Error> for ApiError {
    fn from(err: tonic::transport::Error) -> ApiError {
        let msg = format!("tonic transport error: '{:#}'", err);
        let anyhow_err = anyhow::Error::msg(msg);
        ApiError { err: anyhow_err }
    }
}

impl std::convert::From<std::net::AddrParseError> for ApiError {
    fn from(err: std::net::AddrParseError) -> ApiError {
        let msg = format!("address parse error: '{:#}'", err);
        let anyhow_err = anyhow::Error::msg(msg);
        ApiError { err: anyhow_err }
    }
}

impl std::convert::From<redis::RedisError> for ApiError {
    fn from(err: redis::RedisError) -> ApiError {
        let msg = format!("redis error: '{:#}'", err);
        let anyhow_err = anyhow::Error::msg(msg);
        ApiError { err: anyhow_err }
    }
}

impl std::convert::From<bb8_redis::bb8::RunError<redis::RedisError>> for ApiError {
    fn from(err: bb8_redis::bb8::RunError<redis::RedisError>) -> ApiError {
        let msg = format!("getting connection from reddis error: '{:#}'", err);
        let anyhow_err = anyhow::Error::msg(msg);
        ApiError { err: anyhow_err }
    }
}

// === === === === === === === === === === === === === === ===
impl From<ApiError> for tonic::Status {
    fn from(err: ApiError) -> tonic::Status {
        tonic::Status::new(tonic::Code::Internal, err.err.to_string())
    }
}
