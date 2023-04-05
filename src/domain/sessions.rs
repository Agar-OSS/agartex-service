use sqlx;

use super::users::User;

#[derive(sqlx::FromRow, Debug, Clone, PartialEq)]
pub struct Session {
    #[sqlx(rename = "session_id")]
    pub id: String,
    #[sqlx(flatten)]
    pub user: User,
    pub expires: i64
}
