use sqlx::{FromRow, Row, postgres::PgRow};

use super::users::User;

#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub user: User,
    pub expires: i64
}

impl FromRow<'_, PgRow> for Session {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.try_get("session_id")?,
            user: User {
                id: row.try_get("user_id")?,
                email: row.try_get("email")?,
                password_hash: row.try_get("password_hash")?
            },
            expires: row.try_get("expires")?
        })
    }
}
