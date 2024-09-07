use thiserror::Error;
#[derive(Error, Debug)]
pub enum CustomError {
    #[error("Request parameter error: {0}")]
    ReqParamError(String),
    #[error("Delete error: {0}")]
    ReqDeleteFail(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("IO error: {0}")]
    IOError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Other error: {0}")]
    OtherError(String),
}