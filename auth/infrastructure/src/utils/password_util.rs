use argon2::Argon2;
use common::error::InfraError;
use domain::model::dto::user_dto::LoginUserDTO;
use password_hash::{ PasswordHasher, SaltString };

// 加盐加密
pub async fn hash_password(salt: SaltString, password: &str) -> Result<String, InfraError> {
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| InfraError::OtherError("Failed to hash password".to_owned()))
        .map(|hash| hash.to_string())
}
// 验证密码
pub async fn verify_password(
    salt: SaltString,
    hash_password: &str,
    login_user_dto: LoginUserDTO,
) -> Result<bool, InfraError> {
    let input_password = login_user_dto.password;
    let argon2 = Argon2::default();
    let calculated_hash = argon2
      .hash_password(input_password.as_bytes(), &salt)
      .map_err(|_| InfraError::OtherError("Failed to recalculate hash".to_owned()))?;
    Ok(calculated_hash.to_string() == hash_password)
}

// let salt = SaltString::generate(&mut OsRng);
