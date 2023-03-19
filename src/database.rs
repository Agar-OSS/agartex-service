use sqlx::{Error, postgres::PgPool};

use crate::constants;


pub async fn get_conn_pool() -> Result<PgPool, Error> {
    let pool = PgPool::connect(constants::DB_URL).await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}