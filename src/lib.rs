pub mod client;
pub mod errors;

pub mod api {
    include!(concat!(env!("OUT_DIR"), "/v1.rs"));
}

use errors::ApiError;

pub type Result<T> = std::result::Result<T, ApiError>;
pub type StdResult<T, E> = std::result::Result<T, E>;
