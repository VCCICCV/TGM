use argon2::{Argon2, PasswordHash};
use common::error::InfraError;
use domain::model::user::User;
use password_hash::{
    rand_core::OsRng,
    {PasswordHasher, PasswordVerifier, SaltString},
};

// 加盐加密
pub async fn hash_password(password: &str) -> Result<String, InfraError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| InfraError::OtherError("Failed to hash password".to_owned()))
        .map(|hash| hash.to_string())
}
// 验证密码
pub async fn verify_password(user: &User, password: &str) -> Result<bool, InfraError> {
    let hashed_password = &user.password;
    match PasswordHash::new(&hashed_password) {
        Ok(parsed_hash) => {
            let argon2 = Argon2::default();
            argon2
                .verify_password(password.as_bytes(), &parsed_hash)
                .map_err(|e| InfraError::OtherError(format!("Failed to verify password: {}", e)))
                .map(|_| true)
        }
        Err(_) => Err(InfraError::OtherError("Invalid hashed password".to_owned())),
    }
}
