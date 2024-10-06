use thiserror::Error;
/// 自定义错误类型
#[derive(Error, Debug)]
pub enum AppError {
    // {0}是应该格式化占位符，使用时将其替换为实际的错误消息。
    #[error("Request parameter error: {0}")]
    ReqParamError(String),
    #[error("Delete error: {0}")]
    ReqDeleteFail(String),
    #[error("IO error: {0}")]
    IOError(String),
    #[error("Register error: {0}")]
    RegisterError(String),
    #[error("Login error: {0}")]
    LoginError(String),
    #[error("Authenticate error: {0}")]
    AuthenticateError(String),
    #[error("Refresh token error: {0}")]
    RefreshTokenError(String),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("Other error: {0}")]
    OtherError(String),
}
// 基础设施层错误类型
#[derive(Error, Debug)]
pub enum InfraError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("Redis error: {0}")]
    RedisError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("User not found")]
    UserNotFound,
    #[error("User create error: {0}")]
    UserError(String),
    #[error("Insert error: {0}")]
    InsertError(String),
    #[error("Email mismatch")]
    EmailMismatch,
    #[error("Password hash error: {0}")]
    PasswordHashError(String),
    #[error("Password verify error: {0}")]
    PasswordVerifyError(String),
    #[error("Jwt encode error: {0}")]
    JwtEncodeError(String),
    #[error("Jwt decode error: {0}")]
    JwtDecodeError(String),
    #[error("Network timeout error: {0}")]
    NetworkTimeoutError(String),
    #[error("Network connection error: {0}")]
    NetworkConnectionError(String),
    #[error("Config load error: {0}")]
    ConfigLoadError(String),
    #[error("Invalid input format error: {0}")]
    InvalidInputFormatError(String),
    #[error("Missing required field error: {0}")]
    MissingRequiredFieldError(String),
    #[error("Other error: {0}")]
    OtherError(String),
}
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("IO error: {0}")]
    WrongCredentials(String),
    #[error("IO error: {0}")]
    MissingCredentials(String),
    #[error("IO error: {0}")]
    TokenCreation(String),
    #[error("IO error: {0}")]
    InvalidToken(String),
}
