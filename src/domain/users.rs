use serde::Deserialize;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct User {
    #[sqlx(rename = "user_id")]
    pub id: i32,
    pub email: String,
    pub password_hash: String
}

#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String
}
