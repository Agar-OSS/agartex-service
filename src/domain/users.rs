use serde::Deserialize;
use validator::{Validate, ValidationError};

use crate::constants;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct User {
    #[sqlx(rename = "user_id")]
    pub id: i32,
    pub email: String,
    pub password_hash: String
}

#[derive(Debug, Deserialize, Validate)]
pub struct Credentials {
    #[validate(email)]
    pub email: String,
    #[validate(
        length(min = 8, max = 32), 
        regex = "constants::PASSWORD_REGEX", 
        custom = "contains_uppercase",
        custom = "contains_lowercase",
        custom = "contains_digit",
        custom = "contains_special"
    )]
    pub password: String
}

fn contains_uppercase(password: &str) -> Result<(), ValidationError> {
    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        return Err(ValidationError::new("lowercase_character"))
    }
    Ok(())
}

fn contains_lowercase(password: &str) -> Result<(), ValidationError> {
    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        return Err(ValidationError::new("uppercase_character"))
    }
    Ok(())
}

fn contains_digit(password: &str) -> Result<(), ValidationError> {
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(ValidationError::new("digit_character"))
    } 
    Ok(())
}

fn contains_special(password: &str) -> Result<(), ValidationError> {
    if !password.chars().any(|c| constants::PASSWORD_SPECIAL_CHARS.contains(c)) {
        return Err(ValidationError::new("special_character"))
    }
    Ok(())
}
