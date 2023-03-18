use serde::Deserialize;

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password_hash: String
}

#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String
}
