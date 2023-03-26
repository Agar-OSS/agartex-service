use sqlx;

use super::users::User;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Session {
    #[sqlx(rename = "session_id")]
    pub id: String,
    #[sqlx(flatten)]
    pub user: User,
    pub expires: i64
}
